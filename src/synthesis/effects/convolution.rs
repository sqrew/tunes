//! Convolution reverb using FFT-based overlap-add algorithm
//!
//! Convolution reverb applies the acoustic characteristics of a real space
//! to your audio by convolving it with an impulse response (IR).
//!
//! # How It Works
//!
//! 1. **Impulse Response (IR)**: A recording or simulation of how a space responds to sound
//! 2. **Convolution**: Mathematically applies that response to your audio
//! 3. **FFT Optimization**: Uses FFT to make convolution fast enough for real-time
//!
//! # Usage
//!
//! ```no_run
//! use tunes::synthesis::effects::{Convolution, convolution};
//!
//! // From preset (synthetic IR generated on-the-fly)
//! let reverb = convolution::presets::cathedral(0.5)?;
//!
//! // From file (real recorded IR)
//! let reverb = Convolution::from_file("cathedral.wav", 0.5)?;
//!
//! // Custom parameters
//! use tunes::synthesis::effects::IRParams;
//! let reverb = Convolution::from_params(IRParams::cathedral(), 0.5)?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use crate::error::{Result, TunesError};
use crate::synthesis::sample::Sample;
use crate::track::PRIORITY_SPATIAL;
use rustfft::num_complex::Complex;
use rustfft::{Fft, FftPlanner};
use std::collections::VecDeque;
use std::sync::Arc;

/// Convolution reverb effect using FFT-based processing
///
/// Applies the acoustic characteristics of a space to audio through convolution.
/// Uses overlap-add FFT algorithm for efficient real-time processing.
#[derive(Clone)]
pub struct ConvolutionReverb {
    /// Pre-computed impulse response in frequency domain
    ir_fft: Vec<Complex<f32>>,

    /// FFT size (power of 2, large enough for IR + block)
    fft_size: usize,

    /// Processing block size (samples per FFT block)
    block_size: usize,

    /// Hop size for overlap-add (typically block_size / 2)
    hop_size: usize,

    /// Input accumulation buffer
    input_buffer: Vec<f32>,

    /// Output buffer (stores processed samples)
    output_buffer: VecDeque<f32>,

    /// Overlap buffer for overlap-add algorithm
    overlap_buffer: Vec<f32>,

    /// Forward FFT planner
    fft: Arc<dyn Fft<f32>>,

    /// Inverse FFT planner
    ifft: Arc<dyn Fft<f32>>,

    /// Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub mix: f32,

    /// Processing priority (lower = earlier in signal chain)
    pub priority: u8,

    /// Sample counter for processing state
    sample_count: u64,
}

impl ConvolutionReverb {
    /// Create convolution reverb from impulse response samples
    ///
    /// # Arguments
    /// * `ir` - Impulse response samples (mono)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    /// * `block_size` - FFT block size (None = auto-select based on IR length)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::synthesis::effects::ConvolutionReverb;
    /// let ir = vec![1.0, 0.5, 0.25, 0.1];  // Simple IR
    /// let reverb = ConvolutionReverb::from_samples(&ir, 0.5, None)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn from_samples(ir: &[f32], mix: f32, block_size: Option<usize>) -> Result<Self> {
        if ir.is_empty() {
            return Err(TunesError::AudioEngineError(
                "Impulse response cannot be empty".to_string(),
            ));
        }

        // Auto-select block size based on IR length
        let block_size = block_size.unwrap_or_else(|| {
            if ir.len() < 4096 {
                2048
            } else if ir.len() < 16384 {
                4096
            } else {
                8192
            }
        });

        // FFT size must be large enough for IR + block (use next power of 2)
        let fft_size = (ir.len() + block_size).next_power_of_two();

        // Create FFT planners
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_size);
        let ifft = planner.plan_fft_inverse(fft_size);

        // Pre-compute IR FFT (do this once at creation)
        let ir_fft = {
            let mut ir_complex = vec![Complex::new(0.0, 0.0); fft_size];

            // Copy IR samples to complex buffer
            for (i, &sample) in ir.iter().enumerate() {
                ir_complex[i] = Complex::new(sample, 0.0);
            }

            // Transform to frequency domain
            fft.process(&mut ir_complex);
            ir_complex
        };

        Ok(Self {
            ir_fft,
            fft_size,
            block_size,
            hop_size: block_size / 2,
            input_buffer: Vec::with_capacity(block_size),
            output_buffer: VecDeque::with_capacity(fft_size),
            overlap_buffer: vec![0.0; fft_size],
            fft,
            ifft,
            mix: mix.clamp(0.0, 1.0),
            priority: PRIORITY_SPATIAL, // Convolution reverb typically comes last
            sample_count: 0,
        })
    }

    /// Create convolution reverb from IR file
    ///
    /// Loads a WAV file containing an impulse response and creates a convolution reverb.
    ///
    /// # Arguments
    /// * `ir_path` - Path to impulse response WAV file
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    /// * `block_size` - Optional FFT block size
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::synthesis::effects::ConvolutionReverb;
    /// let reverb = ConvolutionReverb::from_ir("cathedral.wav", 0.5, None)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn from_ir(ir_path: &str, mix: f32, block_size: Option<usize>) -> Result<Self> {
        // Load IR from file
        let ir_sample = Sample::from_file(ir_path)?;

        // Convert to mono if stereo (average channels)
        let ir_mono = if ir_sample.channels == 2 {
            ir_sample
                .data
                .chunks(2)
                .map(|chunk| (chunk[0] + chunk[1]) / 2.0)
                .collect::<Vec<f32>>()
        } else {
            ir_sample.data.as_ref().clone()
        };

        Self::from_samples(&ir_mono, mix, block_size)
    }

    /// Process a single audio sample through convolution
    ///
    /// Uses overlap-add FFT convolution for efficient processing.
    ///
    /// # Arguments
    /// * `input` - Input sample
    ///
    /// # Returns
    /// Processed output sample (wet/dry mixed)
    pub fn process(&mut self, input: f32) -> f32 {
        // Accumulate input samples
        self.input_buffer.push(input);

        // Process block when we have enough samples
        if self.input_buffer.len() >= self.block_size {
            self.process_block();
        }

        // Get output sample
        let output = self.output_buffer.pop_front().unwrap_or(0.0);

        self.sample_count += 1;

        // Apply wet/dry mix
        input * (1.0 - self.mix) + output * self.mix
    }

    /// Process accumulated input block with FFT convolution
    fn process_block(&mut self) {
        // Prepare input block (zero-padded to FFT size)
        let mut input_complex = vec![Complex::new(0.0, 0.0); self.fft_size];

        for (i, &sample) in self.input_buffer.iter().enumerate().take(self.block_size) {
            input_complex[i] = Complex::new(sample, 0.0);
        }

        // FFT the input block
        self.fft.process(&mut input_complex);

        // Multiply in frequency domain (complex multiplication = convolution in time domain)
        for i in 0..self.fft_size {
            input_complex[i] *= self.ir_fft[i];
        }

        // IFFT back to time domain
        self.ifft.process(&mut input_complex);

        // Normalize (rustfft doesn't auto-normalize IFFT)
        let scale = 1.0 / (self.fft_size as f32);

        // Overlap-add with previous block
        for i in 0..self.fft_size {
            let sample = input_complex[i].re * scale;

            // Add to overlap buffer and output
            let output_sample = sample + self.overlap_buffer[i];
            self.output_buffer.push_back(output_sample);

            // Update overlap buffer for next block
            self.overlap_buffer[i] = if i < self.fft_size - self.block_size {
                input_complex[i + self.block_size].re * scale
            } else {
                0.0
            };
        }

        // Keep hop_size samples in input buffer for overlap
        self.input_buffer.drain(0..self.hop_size);
    }

    /// Process a block of samples at once (more efficient than sample-by-sample)
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to process in-place
    pub fn process_block_direct(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Reset the convolution state
    ///
    /// Clears all internal buffers. Useful when stopping/starting playback.
    pub fn reset(&mut self) {
        self.input_buffer.clear();
        self.output_buffer.clear();
        self.overlap_buffer.fill(0.0);
        self.sample_count = 0;
    }

    /// Get the wet/dry mix amount
    pub fn mix(&self) -> f32 {
        self.mix
    }

    /// Set the wet/dry mix amount
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }
}

impl std::fmt::Debug for ConvolutionReverb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConvolutionReverb")
            .field("fft_size", &self.fft_size)
            .field("block_size", &self.block_size)
            .field("mix", &self.mix)
            .finish()
    }
}

/// Parameters for synthetic impulse response generation
///
/// These parameters define the characteristics of a simulated acoustic space.
#[derive(Debug, Clone)]
pub struct IRParams {
    /// Room dimensions in meters (length, width, height)
    pub room_dimensions: (f32, f32, f32),

    /// Reverb time (RT60) in seconds - time for 60dB decay
    pub rt60: f32,

    /// High frequency damping (0.0 = bright/reflective, 1.0 = dark/absorptive)
    pub damping: f32,

    /// Early reflection density (0.0 = sparse, 1.0 = dense)
    pub early_density: f32,

    /// Sample rate for IR generation
    pub sample_rate: f32,
}

impl IRParams {
    /// Small room preset (tight, intimate space)
    ///
    /// Good for: vocals, drums, close-mic'd instruments
    pub fn small_room() -> Self {
        Self {
            room_dimensions: (4.0, 5.0, 2.5),
            rt60: 0.3,
            damping: 0.6,
            early_density: 0.7,
            sample_rate: 44100.0,
        }
    }

    /// Concert hall preset (large, spacious venue)
    ///
    /// Good for: orchestral, ambient, classical music
    pub fn concert_hall() -> Self {
        Self {
            room_dimensions: (40.0, 30.0, 15.0),
            rt60: 2.5,
            damping: 0.4,
            early_density: 0.9,
            sample_rate: 44100.0,
        }
    }

    /// Cathedral preset (massive, long decay)
    ///
    /// Good for: pads, atmospheric sounds, experimental music
    pub fn cathedral() -> Self {
        Self {
            room_dimensions: (60.0, 40.0, 25.0),
            rt60: 4.5,
            damping: 0.5,
            early_density: 0.8,
            sample_rate: 44100.0,
        }
    }

    /// Plate reverb preset (vintage hardware simulation)
    ///
    /// Good for: vocals, drums, classic production
    pub fn plate() -> Self {
        Self {
            room_dimensions: (2.0, 1.5, 0.01),
            rt60: 2.0,
            damping: 0.2,
            early_density: 1.0,
            sample_rate: 44100.0,
        }
    }

    /// Spring reverb preset (vintage spring tank simulation)
    ///
    /// Good for: guitars, retro effects, surf music
    pub fn spring() -> Self {
        Self {
            room_dimensions: (0.5, 0.1, 0.1),
            rt60: 1.0,
            damping: 0.3,
            early_density: 0.6,
            sample_rate: 44100.0,
        }
    }
}

impl ConvolutionReverb {
    /// Create convolution reverb from synthetic IR parameters
    ///
    /// Generates an impulse response on-the-fly based on room characteristics.
    ///
    /// # Arguments
    /// * `params` - IR generation parameters
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::synthesis::effects::{ConvolutionReverb, IRParams};
    /// let reverb = ConvolutionReverb::from_params(IRParams::cathedral(), 0.5)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn from_params(params: IRParams, mix: f32) -> Result<Self> {
        let ir = generate_ir(&params);
        Self::from_samples(&ir, mix, None)
    }
}

/// Generate a synthetic impulse response from parameters
///
/// Creates a simulated room impulse response using:
/// - Early reflections based on room geometry
/// - Diffuse reverb tail with exponential decay
/// - Frequency-dependent damping
pub fn generate_ir(params: &IRParams) -> Vec<f32> {
    let duration = params.rt60 * 1.5; // Generate 1.5x RT60 for full decay
    let num_samples = (duration * params.sample_rate) as usize;

    let mut ir = vec![0.0; num_samples];

    // 1. Initial impulse (delta function)
    ir[0] = 1.0;

    // 2. Add early reflections (geometric room model)
    add_early_reflections(&mut ir, params);

    // 3. Add diffuse reverb tail (exponential decay)
    add_diffuse_tail(&mut ir, params);

    // 4. Normalize to prevent clipping
    let max = ir.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
    if max > 0.0 {
        for sample in &mut ir {
            *sample /= max;
        }
    }

    ir
}

/// Add early reflections based on room geometry
fn add_early_reflections(ir: &mut [f32], params: &IRParams) {
    let (length, width, height) = params.room_dimensions;
    let speed_of_sound = 343.0; // m/s

    // First-order reflections (6 surfaces: walls, floor, ceiling)
    let reflections = [
        (length / speed_of_sound, 0.8),              // Front wall
        (width / speed_of_sound, 0.8),               // Side wall
        (height / speed_of_sound, 0.7),              // Floor
        (length * 1.5 / speed_of_sound, 0.6),        // Back wall reflection
        (width * 1.5 / speed_of_sound, 0.6),         // Opposite side
        (height * 2.0 / speed_of_sound, 0.5),        // Ceiling reflection
    ];

    for (delay_seconds, amplitude) in reflections {
        let delay_samples = (delay_seconds * params.sample_rate) as usize;
        if delay_samples < ir.len() {
            ir[delay_samples] += amplitude;
        }
    }

    // Add additional random early reflections based on density
    if params.early_density > 0.5 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let num_extra = (params.early_density * 20.0) as usize;

        for _ in 0..num_extra {
            let delay = rng.gen_range(0.01..0.1); // 10-100ms
            let delay_samples = (delay * params.sample_rate) as usize;
            let amplitude = rng.gen_range(0.1..0.5);

            if delay_samples < ir.len() {
                ir[delay_samples] += amplitude;
            }
        }
    }
}

/// Add diffuse reverb tail with exponential decay
fn add_diffuse_tail(ir: &mut [f32], params: &IRParams) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Start diffuse tail after early reflections (~50ms)
    let start_sample = (0.05 * params.sample_rate) as usize;

    // Decay rate based on RT60
    // RT60 = time for 60dB decay (amplitude drops to 0.001)
    let decay_rate = (-60.0 / params.rt60) / params.sample_rate;
    let decay_coefficient = 10.0f32.powf(decay_rate / 20.0);

    // Simple one-pole lowpass for frequency-dependent damping
    let damping_coeff = 1.0 - params.damping;
    let mut lowpass_state = 0.0;

    for i in start_sample..ir.len() {
        // Generate random noise
        let noise = rng.gen_range(-1.0..1.0);

        // Apply exponential decay
        let decay = decay_coefficient.powf((i - start_sample) as f32);

        // Apply lowpass filter (simulates high-frequency absorption)
        lowpass_state = lowpass_state * params.damping + noise * damping_coeff;

        // Add to IR with decay
        ir[i] += lowpass_state * decay * 0.3;
    }
}

/// Type-safe convolution reverb namespace
///
/// Provides compile-time checked methods for creating convolution reverb.
///
/// # Example
/// ```no_run
/// use tunes::synthesis::effects::{Convolution, convolution};
///
/// // Type-safe preset (IDE autocomplete works!)
/// let reverb = convolution::presets::cathedral(0.5)?;
///
/// // From file
/// let reverb = Convolution::from_file("real_hall.wav", 0.5)?;
///
/// // Custom parameters
/// use tunes::synthesis::effects::IRParams;
/// let reverb = Convolution::from_params(IRParams::cathedral(), 0.5)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct Convolution;

impl Convolution {
    /// Create convolution reverb from impulse response file
    ///
    /// Loads a WAV file containing a real or recorded impulse response.
    ///
    /// # Arguments
    /// * `ir_path` - Path to WAV file
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    ///
    /// # Example
    /// ```no_run
    /// use tunes::synthesis::effects::Convolution;
    ///
    /// let reverb = Convolution::from_file("cathedral.wav", 0.5)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn from_file(ir_path: &str, mix: f32) -> Result<ConvolutionReverb> {
        ConvolutionReverb::from_ir(ir_path, mix, None)
    }

    /// Create convolution reverb from custom IR parameters
    ///
    /// Generates a synthetic IR based on room characteristics.
    ///
    /// # Arguments
    /// * `params` - Room parameters
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    ///
    /// # Example
    /// ```no_run
    /// use tunes::synthesis::effects::{Convolution, IRParams};
    ///
    /// let params = IRParams {
    ///     room_dimensions: (10.0, 8.0, 4.0),
    ///     rt60: 1.5,
    ///     damping: 0.4,
    ///     early_density: 0.8,
    ///     sample_rate: 44100.0,
    /// };
    /// let reverb = Convolution::from_params(params, 0.5)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn from_params(params: IRParams, mix: f32) -> Result<ConvolutionReverb> {
        ConvolutionReverb::from_params(params, mix)
    }
}

/// Built-in preset impulse responses
///
/// Type-safe presets with compile-time checking and IDE autocomplete.
///
/// # Example
/// ```no_run
/// use tunes::synthesis::effects::convolution;
///
/// let reverb = convolution::presets::cathedral(0.5)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub mod presets {
    use super::*;

    /// Small room reverb (0.3s RT60)
    ///
    /// Tight, intimate space.
    ///
    /// **Good for:** Vocals, drums, close-mic'd instruments
    ///
    /// # Example
    /// ```no_run
    /// use tunes::synthesis::effects::convolution;
    ///
    /// let reverb = convolution::presets::small_room(0.5)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn small_room(mix: f32) -> Result<ConvolutionReverb> {
        ConvolutionReverb::from_params(IRParams::small_room(), mix)
    }

    /// Concert hall reverb (2.5s RT60)
    ///
    /// Large, spacious venue with balanced reflections.
    ///
    /// **Good for:** Orchestral, ambient, classical music
    ///
    /// # Example
    /// ```no_run
    /// use tunes::synthesis::effects::convolution;
    ///
    /// let reverb = convolution::presets::concert_hall(0.5)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn concert_hall(mix: f32) -> Result<ConvolutionReverb> {
        ConvolutionReverb::from_params(IRParams::concert_hall(), mix)
    }

    /// Cathedral reverb (4.5s RT60)
    ///
    /// Massive space with very long decay time.
    ///
    /// **Good for:** Pads, atmospheric sounds, experimental music
    ///
    /// # Example
    /// ```no_run
    /// use tunes::synthesis::effects::convolution;
    ///
    /// let reverb = convolution::presets::cathedral(0.5)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn cathedral(mix: f32) -> Result<ConvolutionReverb> {
        ConvolutionReverb::from_params(IRParams::cathedral(), mix)
    }

    /// Plate reverb (2.0s RT60)
    ///
    /// Bright, dense reflections simulating vintage plate reverb hardware.
    ///
    /// **Good for:** Vocals, drums, classic production sound
    ///
    /// # Example
    /// ```no_run
    /// use tunes::synthesis::effects::convolution;
    ///
    /// let reverb = convolution::presets::plate(0.5)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn plate(mix: f32) -> Result<ConvolutionReverb> {
        ConvolutionReverb::from_params(IRParams::plate(), mix)
    }

    /// Spring reverb (1.0s RT60)
    ///
    /// Characteristic "boing" sound of vintage spring reverb tanks.
    ///
    /// **Good for:** Guitars, retro effects, surf music
    ///
    /// # Example
    /// ```no_run
    /// use tunes::synthesis::effects::convolution;
    ///
    /// let reverb = convolution::presets::spring(0.5)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn spring(mix: f32) -> Result<ConvolutionReverb> {
        ConvolutionReverb::from_params(IRParams::spring(), mix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_from_samples() {
        let ir = vec![1.0, 0.5, 0.25, 0.1];
        let reverb = ConvolutionReverb::from_samples(&ir, 0.5, None);
        assert!(reverb.is_ok());
    }

    #[test]
    fn test_process_sample() {
        let ir = vec![1.0, 0.5, 0.25];
        let mut reverb = ConvolutionReverb::from_samples(&ir, 0.5, Some(256)).unwrap();

        // Process some samples
        for _ in 0..1000 {
            let output = reverb.process(1.0);
            assert!(output.is_finite());
        }
    }

    #[test]
    fn test_empty_ir_fails() {
        let ir: Vec<f32> = vec![];
        let result = ConvolutionReverb::from_samples(&ir, 0.5, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_reset() {
        let ir = vec![1.0, 0.5, 0.25];
        let mut reverb = ConvolutionReverb::from_samples(&ir, 0.5, None).unwrap();

        // Process some samples
        for _ in 0..100 {
            reverb.process(1.0);
        }

        // Reset
        reverb.reset();

        // Buffers should be empty
        assert_eq!(reverb.sample_count, 0);
    }

    #[test]
    fn test_generate_ir() {
        let params = IRParams::small_room();
        let ir = generate_ir(&params);

        assert!(!ir.is_empty());
        assert!(ir[0].abs() > 0.0); // Should have initial impulse
    }

    #[test]
    fn test_from_params() {
        let reverb = ConvolutionReverb::from_params(IRParams::cathedral(), 0.5);
        assert!(reverb.is_ok());
    }

    #[test]
    fn test_ir_presets() {
        // Test all presets generate valid IRs
        for params in [
            IRParams::small_room(),
            IRParams::concert_hall(),
            IRParams::cathedral(),
            IRParams::plate(),
            IRParams::spring(),
        ] {
            let ir = generate_ir(&params);
            assert!(!ir.is_empty());
            assert!(ir.iter().all(|x| x.is_finite()));
        }
    }
}
