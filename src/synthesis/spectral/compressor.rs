//! Spectral compressor for frequency-selective dynamic range compression

use super::*;
use rustfft::num_complex::Complex;

/// Spectral compressor for frequency-selective dynamic range compression
///
/// SpectralCompressor applies independent compression to each frequency bin in the spectrum,
/// enabling multiband compression at extreme resolution (1024+ bands vs traditional 3-5 bands).
/// This allows for surgical dynamic control of specific frequency ranges.
///
/// # How It Works
///
/// 1. **STFT Analysis**: Decomposes audio into frequency bins via Short-Time Fourier Transform
/// 2. **Per-Bin Compression**: Each bin gets independent threshold comparison and gain reduction
/// 3. **Soft Knee**: Smooth transition into compression for natural sound
/// 4. **Attack/Release**: Exponential smoothing per bin to avoid artifacts
/// 5. **STFT Synthesis**: Reconstructs audio with compressed spectrum
///
/// # Use Cases
///
/// - **Multiband Mastering**: Extreme-resolution multiband compression (1024 bands!)
/// - **De-essing**: Compress harsh sibilance (6-8 kHz) without affecting rest of vocal
/// - **Taming Resonances**: Control specific problem frequencies
/// - **Creative Effects**: Per-frequency dynamics for unique textures
///
/// # Performance
///
/// - **Latency**: ~23ms @ 44.1kHz (2048 FFT, 512 hop)
/// - **CPU**: ~100-200x more expensive than regular compressor
/// - Uses SIMD for magnitude calculation
///
/// # Example
///
/// ```
/// use tunes::synthesis::spectral::{SpectralCompressor, WindowType};
///
/// // Create compressor with default settings
/// let mut comp = SpectralCompressor::new(2048, 512, WindowType::Hann, 44100.0);
///
/// // Configure for de-essing
/// comp.set_threshold(-20.0);  // Compress above -20 dB
/// comp.set_ratio(4.0);         // 4:1 ratio
/// comp.set_attack(1.0);        // 1ms attack
/// comp.set_release(50.0);      // 50ms release
/// comp.set_knee(6.0);          // 6 dB soft knee
///
/// // Process audio
/// let input = vec![0.0; 1024];
/// let mut output = vec![0.0; 1024];
/// comp.process(&mut output, &input);
/// ```
#[derive(Clone, Debug)]
pub struct SpectralCompressor {
    stft: STFT,
    #[allow(dead_code)]
    fft_size: usize,
    sample_rate: f32,

    // Compression parameters
    threshold_db: f32, // Threshold in dB
    ratio: f32,        // Compression ratio (e.g., 4.0 = 4:1)
    attack: f32,       // Attack time in ms
    release: f32,      // Release time in ms
    knee: f32,         // Soft knee width in dB

    // Pre-calculated coefficients for performance
    attack_coeff: f32,
    release_coeff: f32,

    // Per-bin envelope state (for attack/release smoothing)
    envelope: Vec<f32>,

    enabled: bool,
}

impl SpectralCompressor {
    /// Create a new spectral compressor
    ///
    /// # Arguments
    ///
    /// * `fft_size` - FFT size (must be power of 2, typically 2048)
    /// * `hop_size` - Hop size between FFT frames (typically fft_size/4)
    /// * `window_type` - Window function (Hann recommended)
    /// * `sample_rate` - Sample rate in Hz
    ///
    /// # Panics
    ///
    /// Panics if fft_size is not a power of 2, hop_size > fft_size, or sample_rate <= 0
    pub fn new(
        fft_size: usize,
        hop_size: usize,
        window_type: WindowType,
        sample_rate: f32,
    ) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(hop_size <= fft_size, "Hop size must be <= FFT size");
        assert!(sample_rate > 0.0, "Sample rate must be positive");

        // Default parameters
        let threshold_db = -20.0;
        let ratio = 4.0;
        let attack = 5.0; // 5ms attack
        let release = 50.0; // 50ms release
        let knee = 6.0; // 6 dB soft knee

        // Calculate attack/release coefficients
        let hop_time = hop_size as f32 / sample_rate;
        let attack_coeff = Self::calculate_coeff(attack, hop_time);
        let release_coeff = Self::calculate_coeff(release, hop_time);

        Self {
            stft: STFT::new(fft_size, hop_size, window_type),
            fft_size,
            sample_rate,
            threshold_db,
            ratio,
            attack,
            release,
            knee,
            attack_coeff,
            release_coeff,
            envelope: vec![1.0; fft_size], // Start at unity gain
            enabled: true,
        }
    }

    /// Calculate exponential coefficient for attack/release
    ///
    /// Formula: exp(-hop_time / time_constant)
    #[inline]
    fn calculate_coeff(time_ms: f32, hop_time: f32) -> f32 {
        let time_sec = time_ms / 1000.0;
        (-hop_time / time_sec).exp()
    }

    /// Set compression threshold in dB
    ///
    /// Frequencies with magnitude above this threshold will be compressed.
    ///
    /// # Arguments
    ///
    /// * `threshold_db` - Threshold in dB (typically -40.0 to 0.0)
    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.threshold_db = threshold_db;
    }

    /// Set compression ratio
    ///
    /// # Arguments
    ///
    /// * `ratio` - Compression ratio (e.g., 4.0 = 4:1). Must be >= 1.0.
    ///   - 1.0 = no compression
    ///   - 2.0 = 2:1 (gentle)
    ///   - 4.0 = 4:1 (moderate)
    ///   - 10.0 = 10:1 (heavy)
    ///   - 100.0+ = limiting
    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio.max(1.0);
    }

    /// Set attack time in milliseconds
    ///
    /// How quickly compression engages when signal exceeds threshold.
    /// Faster attack (1-5ms) = tighter control, slower attack (10-30ms) = more transient punch.
    pub fn set_attack(&mut self, attack_ms: f32) {
        self.attack = attack_ms.max(0.1);
        let hop_time = self.stft.hop_size as f32 / self.sample_rate;
        self.attack_coeff = Self::calculate_coeff(self.attack, hop_time);
    }

    /// Set release time in milliseconds
    ///
    /// How quickly compression releases when signal falls below threshold.
    /// Faster release (20-50ms) = more pumping, slower release (100-300ms) = smoother.
    pub fn set_release(&mut self, release_ms: f32) {
        self.release = release_ms.max(1.0);
        let hop_time = self.stft.hop_size as f32 / self.sample_rate;
        self.release_coeff = Self::calculate_coeff(self.release, hop_time);
    }

    /// Set soft knee width in dB
    ///
    /// Creates a smooth transition into compression.
    ///
    /// # Arguments
    ///
    /// * `knee_db` - Knee width in dB (0.0 = hard knee, 6.0-12.0 typical for soft knee)
    pub fn set_knee(&mut self, knee_db: f32) {
        self.knee = knee_db.max(0.0);
    }

    /// Process audio with spectral compression
    ///
    /// # Arguments
    ///
    /// * `output` - Output buffer to write processed audio
    /// * `input` - Input audio buffer
    pub fn process(&mut self, output: &mut [f32], input: &[f32]) {
        if !self.enabled {
            output.copy_from_slice(input);
            return;
        }

        // Add input to STFT buffer
        self.stft.add_input(input);

        let threshold_db = self.threshold_db;
        let ratio = self.ratio;
        let knee = self.knee;
        let attack_coeff = self.attack_coeff;
        let release_coeff = self.release_coeff;
        let envelope = &mut self.envelope;

        self.stft.process(output, |spectrum| {
            Self::apply_compression_static(
                spectrum,
                envelope,
                threshold_db,
                ratio,
                knee,
                attack_coeff,
                release_coeff,
            );
        });
    }

    /// Get current threshold in dB
    pub fn threshold(&self) -> f32 {
        self.threshold_db
    }

    /// Get current ratio
    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    /// Get current attack time in ms
    pub fn attack(&self) -> f32 {
        self.attack
    }

    /// Get current release time in ms
    pub fn release(&self) -> f32 {
        self.release
    }

    /// Get current knee width in dB
    pub fn knee(&self) -> f32 {
        self.knee
    }

    /// Apply compression to a spectrum (static version for STFT callback)
    ///
    /// This is the core compression algorithm that runs per FFT frame.
    #[inline]
    fn apply_compression_static(
        spectrum: &mut [Complex<f32>],
        envelope: &mut [f32],
        threshold_db: f32,
        ratio: f32,
        knee_db: f32,
        attack_coeff: f32,
        release_coeff: f32,
    ) {
        let len = spectrum.len();

        // Calculate magnitudes using SIMD
        let mut magnitudes = vec![0.0; len];
        ComplexOps::magnitude(&mut magnitudes, spectrum);

        // Process each bin with compression and attack/release
        for i in 0..len {
            // Convert magnitude to dB (with floor to avoid log(0))
            let mag_db = if magnitudes[i] > 1e-10 {
                20.0 * magnitudes[i].log10()
            } else {
                -100.0 // Floor at -100 dB
            };

            // Calculate gain reduction with soft knee
            let gain = if knee_db > 0.0 {
                // Soft knee compression
                let knee_lower = threshold_db - knee_db / 2.0;
                let knee_upper = threshold_db + knee_db / 2.0;

                if mag_db < knee_lower {
                    // Below knee: no compression
                    1.0
                } else if mag_db > knee_upper {
                    // Above knee: full compression
                    let over_db = mag_db - threshold_db;
                    let gain_reduction_db = over_db * (1.0 - 1.0 / ratio);
                    10.0_f32.powf(-gain_reduction_db / 20.0)
                } else {
                    // Inside knee: smooth transition
                    let knee_position = (mag_db - knee_lower) / knee_db; // 0..1
                    let over_db = mag_db - threshold_db;
                    let gain_reduction_db = over_db * (1.0 - 1.0 / ratio) * knee_position;
                    10.0_f32.powf(-gain_reduction_db / 20.0)
                }
            } else {
                // Hard knee compression
                if mag_db >= threshold_db {
                    let over_db = mag_db - threshold_db;
                    let gain_reduction_db = over_db * (1.0 - 1.0 / ratio);
                    10.0_f32.powf(-gain_reduction_db / 20.0)
                } else {
                    1.0
                }
            };

            // Apply attack/release smoothing
            let current_env = envelope[i];
            let coeff = if gain < current_env {
                attack_coeff // Compressing (reducing gain)
            } else {
                release_coeff // Releasing (increasing gain)
            };

            // Exponential smoothing: env = target + coeff * (current - target)
            envelope[i] = gain + coeff * (current_env - gain);

            // Apply gain to complex spectrum (both real and imaginary parts)
            spectrum[i].re *= envelope[i];
            spectrum[i].im *= envelope[i];
        }
    }

    /// Reset internal state (clear envelope memory)
    pub fn reset(&mut self) {
        self.stft.reset();
        self.envelope.fill(1.0); // Reset to unity gain
    }

    /// Enable or disable the compressor
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if compressor is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_compressor_creation() {
        let comp = SpectralCompressor::new(2048, 512, WindowType::Hann, 44100.0);
        assert!(comp.is_enabled());
    }

    #[test]
    #[should_panic(expected = "FFT size must be power of 2")]
    fn test_spectral_compressor_requires_power_of_two() {
        SpectralCompressor::new(1000, 250, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Hop size must be <= FFT size")]
    fn test_spectral_compressor_hop_validation() {
        SpectralCompressor::new(512, 1024, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Sample rate must be positive")]
    fn test_spectral_compressor_sample_rate_validation() {
        SpectralCompressor::new(512, 128, WindowType::Hann, 0.0);
    }

    #[test]
    fn test_spectral_compressor_set_threshold() {
        let mut comp = SpectralCompressor::new(512, 128, WindowType::Hann, 44100.0);
        comp.set_threshold(-30.0);
        assert_eq!(comp.threshold(), -30.0);
    }

    #[test]
    fn test_spectral_compressor_set_ratio() {
        let mut comp = SpectralCompressor::new(512, 128, WindowType::Hann, 44100.0);
        comp.set_ratio(8.0);
        assert_eq!(comp.ratio(), 8.0);

        // Test clamping to minimum 1.0
        comp.set_ratio(0.5);
        assert_eq!(comp.ratio(), 1.0);
    }

    #[test]
    fn test_spectral_compressor_set_attack() {
        let mut comp = SpectralCompressor::new(512, 128, WindowType::Hann, 44100.0);
        comp.set_attack(10.0);
        assert_eq!(comp.attack(), 10.0);
    }

    #[test]
    fn test_spectral_compressor_set_release() {
        let mut comp = SpectralCompressor::new(512, 128, WindowType::Hann, 44100.0);
        comp.set_release(100.0);
        assert_eq!(comp.release(), 100.0);
    }

    #[test]
    fn test_spectral_compressor_set_knee() {
        let mut comp = SpectralCompressor::new(512, 128, WindowType::Hann, 44100.0);
        comp.set_knee(12.0);
        assert_eq!(comp.knee(), 12.0);
    }

    #[test]
    fn test_spectral_compressor_process_silent() {
        let mut comp = SpectralCompressor::new(512, 128, WindowType::Hann, 44100.0);

        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];

        comp.process(&mut output, &input);

        // Silent input should produce silent output
        for &sample in &output {
            assert!(sample.abs() < 1e-6);
        }
    }

    #[test]
    fn test_spectral_compressor_process_with_compression() {
        let mut comp = SpectralCompressor::new(2048, 512, WindowType::Hann, 44100.0);
        comp.set_threshold(-40.0);
        comp.set_ratio(4.0);

        // Create a test signal with known frequency
        let mut input = vec![0.0; 2048];
        for i in 0..2048 {
            let t = i as f32 / 44100.0;
            input[i] = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.5;
        }

        let mut output = vec![0.0; 2048];
        comp.process(&mut output, &input);

        // Output should be non-zero (signal was compressed, not gated)
        let output_energy: f32 = output.iter().map(|x| x * x).sum();
        assert!(output_energy > 0.0);
    }

    #[test]
    fn test_spectral_compressor_reset() {
        let mut comp = SpectralCompressor::new(512, 128, WindowType::Hann, 44100.0);

        // Process some audio
        let input = vec![0.5; 256];
        let mut output = vec![0.0; 256];
        comp.process(&mut output, &input);

        // Reset
        comp.reset();

        // Envelope should be reset to unity gain (verify by checking some samples)
        // We can't access envelope directly, so just verify reset doesn't crash
        comp.process(&mut output, &input);
    }

    #[test]
    fn test_spectral_compressor_disabled() {
        let mut comp = SpectralCompressor::new(512, 128, WindowType::Hann, 44100.0);
        comp.set_enabled(false);

        let input = vec![0.5; 256];
        let mut output = vec![0.0; 256];

        comp.process(&mut output, &input);

        // When disabled, output should equal input
        for i in 0..256 {
            assert_eq!(output[i], input[i]);
        }
    }

    #[test]
    fn test_spectral_compressor_all_window_types() {
        for window_type in [WindowType::Hann, WindowType::Hamming, WindowType::Blackman, WindowType::Rectangular] {
            let mut comp = SpectralCompressor::new(512, 128, window_type, 44100.0);

            let input = vec![0.0; 256];
            let mut output = vec![0.0; 256];

            comp.process(&mut output, &input);
            assert_eq!(output.len(), 256);
        }
    }

    #[test]
    fn test_spectral_compressor_various_fft_sizes() {
        for fft_size in [512, 1024, 2048, 4096] {
            let hop_size = fft_size / 4;
            let mut comp = SpectralCompressor::new(fft_size, hop_size, WindowType::Hann, 44100.0);

            let input = vec![0.0; 512];
            let mut output = vec![0.0; 512];

            comp.process(&mut output, &input);
            assert_eq!(output.len(), 512);
        }
    }

    #[test]
    fn test_spectral_compressor_enable_disable() {
        let mut comp = SpectralCompressor::new(512, 128, WindowType::Hann, 44100.0);

        assert!(comp.is_enabled());

        comp.set_enabled(false);
        assert!(!comp.is_enabled());

        comp.set_enabled(true);
        assert!(comp.is_enabled());
    }

    #[test]
    fn test_spectral_compressor_soft_knee() {
        let mut comp = SpectralCompressor::new(2048, 512, WindowType::Hann, 44100.0);
        comp.set_threshold(-20.0);
        comp.set_ratio(4.0);
        comp.set_knee(6.0); // Soft knee

        // Create a test signal
        let mut input = vec![0.0; 2048];
        for i in 0..2048 {
            let t = i as f32 / 44100.0;
            input[i] = (2.0 * std::f32::consts::PI * 1000.0 * t).sin() * 0.3;
        }

        let mut output = vec![0.0; 2048];
        comp.process(&mut output, &input);

        // Output should be non-zero
        let output_energy: f32 = output.iter().map(|x| x * x).sum();
        assert!(output_energy > 0.0);
    }

    #[test]
    fn test_spectral_compressor_hard_knee() {
        let mut comp = SpectralCompressor::new(2048, 512, WindowType::Hann, 44100.0);
        comp.set_threshold(-20.0);
        comp.set_ratio(4.0);
        comp.set_knee(0.0); // Hard knee

        // Create a test signal
        let mut input = vec![0.0; 2048];
        for i in 0..2048 {
            let t = i as f32 / 44100.0;
            input[i] = (2.0 * std::f32::consts::PI * 1000.0 * t).sin() * 0.3;
        }

        let mut output = vec![0.0; 2048];
        comp.process(&mut output, &input);

        // Output should be non-zero
        let output_energy: f32 = output.iter().map(|x| x * x).sum();
        assert!(output_energy > 0.0);
    }
}

