use crate::synthesis::automation::Automation;
use crate::track::PRIORITY_SPATIAL;

/// Standard audio sample rate
const DEFAULT_SAMPLE_RATE: f32 = 44100.0;

/// Simple reverb using multiple comb filters
#[derive(Debug, Clone)]
pub struct Reverb {
    pub room_size: f32, // Room size (0.0 to 1.0)
    pub damping: f32,   // High frequency damping (0.0 to 1.0)
    pub mix: f32,       // Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub priority: u8,   // Processing priority (lower = earlier in signal chain)
    comb_buffers: Vec<Vec<f32>>,
    comb_positions: Vec<usize>,
    filter_state: Vec<f32>,

    // Automation (optional)
    mix_automation: Option<Automation>,
    room_size_automation: Option<Automation>,
    damping_automation: Option<Automation>,
}

impl Reverb {
    /// Create a new reverb effect with default sample rate (44100 Hz)
    pub fn new(room_size: f32, damping: f32, mix: f32) -> Self {
        Self::with_sample_rate(room_size, damping, mix, DEFAULT_SAMPLE_RATE)
    }

    /// Create a new reverb effect with custom sample rate
    pub fn with_sample_rate(room_size: f32, damping: f32, mix: f32, sample_rate: f32) -> Self {
        // Prime numbers for comb filter delays (in samples) - scaled by room size
        let base_delays = [1557, 1617, 1491, 1422, 1277, 1356, 1188, 1116];
        let scale = 1.0 + room_size * 2.0;

        let comb_buffers: Vec<Vec<f32>> = base_delays
            .iter()
            .map(|&delay| {
                let size = ((delay as f32 * scale * sample_rate) / 44100.0) as usize;
                vec![0.0; size.max(1)]
            })
            .collect();

        Self {
            room_size: room_size.clamp(0.0, 1.0),
            damping: damping.clamp(0.0, 1.0),
            mix: mix.clamp(0.0, 1.0),
            priority: PRIORITY_SPATIAL, // Reverb typically comes last in chain
            comb_positions: vec![0; comb_buffers.len()],
            filter_state: vec![0.0; comb_buffers.len()],
            comb_buffers,
            mix_automation: None,
            room_size_automation: None,
            damping_automation: None,
        }
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the mix parameter
    ///
    /// # Example
    /// ```no_run
    /// use tunes::prelude::*;
    ///
    /// let reverb = Reverb::new(0.7, 0.6, 0.0)
    ///     .with_mix_automation(Automation::linear(&[
    ///         (0.0, 0.0),   // Start dry
    ///         (4.0, 0.8),   // Fade in over 4 seconds
    ///     ]));
    /// ```
    pub fn with_mix_automation(mut self, automation: Automation) -> Self {
        self.mix_automation = Some(automation);
        self
    }

    /// Add automation for the room size parameter
    pub fn with_room_size_automation(mut self, automation: Automation) -> Self {
        self.room_size_automation = Some(automation);
        self
    }

    /// Add automation for the damping parameter
    pub fn with_damping_automation(mut self, automation: Automation) -> Self {
        self.damping_automation = Some(automation);
        self
    }

    /// Process a single sample
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, time: f32, sample_count: u64) -> f32 {
        // Quantized automation lookups (every 64 samples = 1.45ms @ 44.1kHz)
        // This reduces automation overhead by 64x with no perceptible quality loss
        // Use bitwise AND instead of modulo for power-of-2
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.mix_automation {
                self.mix = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.room_size_automation {
                self.room_size = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.damping_automation {
                self.damping = auto.value_at(time).clamp(0.0, 1.0);
            }
        }

        if self.mix < 0.0001 {
            return input;
        }

        let mut output = 0.0;
        let feedback = self.room_size.mul_add(0.48, 0.5);
        let inv_damping = 1.0 - self.damping;

        // Process through all comb filters
        for i in 0..self.comb_buffers.len() {
            let buffer = &mut self.comb_buffers[i];
            let pos = self.comb_positions[i];

            // Read from buffer
            let delayed = buffer[pos];

            // Apply damping filter (simple lowpass) using FMA
            self.filter_state[i] =
                delayed.mul_add(inv_damping, self.filter_state[i] * self.damping);

            // Write to buffer with feedback using FMA
            buffer[pos] = self.filter_state[i].mul_add(feedback, input);

            // Advance position
            self.comb_positions[i] = (pos + 1) % buffer.len();

            // Accumulate output
            output += delayed;
        }

        // Average and mix using FMA
        output /= self.comb_buffers.len() as f32;
        input.mul_add(1.0 - self.mix, output * self.mix)
    }

    /// Process a block of samples
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    /// * `sample_rate` - Sample rate in Hz (for time advancement)
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], time: f32, sample_count: u64, sample_rate: f32) {
        let time_delta = 1.0 / sample_rate;
        for (i, sample) in buffer.iter_mut().enumerate() {
            let current_time = time + (i as f32 * time_delta);
            let current_sample_count = sample_count + i as u64;
            *sample = self.process(*sample, current_time, current_sample_count);
        }
    }

    /// Reset the reverb state
    pub fn reset(&mut self) {
        for buffer in &mut self.comb_buffers {
            buffer.fill(0.0);
        }
        self.comb_positions.fill(0);
        self.filter_state.fill(0.0);
    }

    // ========== PRESETS ==========

    /// Small room reverb - intimate, subtle space
    pub fn room() -> Self {
        Self::new(0.3, 0.7, 0.2)
    }

    /// Large concert hall - spacious, balanced
    pub fn hall() -> Self {
        Self::new(0.8, 0.5, 0.3)
    }

    /// Plate reverb - bright, dense reflections
    pub fn plate() -> Self {
        Self::new(0.5, 0.3, 0.25)
    }

    /// Chamber reverb - medium space with warmth
    pub fn chamber() -> Self {
        Self::new(0.6, 0.6, 0.25)
    }

    /// Cathedral/church - huge, long decay
    pub fn cathedral() -> Self {
        Self::new(0.95, 0.4, 0.4)
    }

    /// Ambient soundscape - massive space, lush
    pub fn ambient() -> Self {
        Self::new(0.9, 0.4, 0.5)
    }

    /// Subtle room presence - barely noticeable
    pub fn subtle() -> Self {
        Self::new(0.4, 0.6, 0.15)
    }

    /// Spring reverb - vintage, characteristic boing
    pub fn spring() -> Self {
        Self::new(0.4, 0.8, 0.3)
    }
}
