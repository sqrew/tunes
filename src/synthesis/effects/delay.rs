use crate::synthesis::automation::Automation;
use crate::track::PRIORITY_TIME_BASED;

/// Standard audio sample rate
const DEFAULT_SAMPLE_RATE: f32 = 44100.0;

/// Delay effect with feedback
#[derive(Debug, Clone)]
pub struct Delay {
    pub delay_time: f32, // Delay time in seconds
    pub feedback: f32,   // Feedback amount (0.0 to 0.99)
    pub mix: f32,        // Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub priority: u8,    // Processing priority (lower = earlier in signal chain)
    buffer: Vec<f32>,
    write_pos: usize,

    // Automation (optional)
    delay_time_automation: Option<Automation>,
    feedback_automation: Option<Automation>,
    mix_automation: Option<Automation>,
}

impl Delay {
    /// Create a new delay effect with default sample rate (44100 Hz)
    pub fn new(delay_time: f32, feedback: f32, mix: f32) -> Self {
        Self::with_sample_rate(delay_time, feedback, mix, DEFAULT_SAMPLE_RATE)
    }

    /// Create a new delay effect with custom sample rate
    pub fn with_sample_rate(delay_time: f32, feedback: f32, mix: f32, sample_rate: f32) -> Self {
        let buffer_size = (delay_time * sample_rate) as usize;
        Self {
            delay_time,
            feedback: feedback.clamp(0.0, 0.99),
            mix: mix.clamp(0.0, 1.0),
            priority: PRIORITY_TIME_BASED, // Time-based effects typically come late in chain
            buffer: vec![0.0; buffer_size.max(1)],
            write_pos: 0,
            delay_time_automation: None,
            feedback_automation: None,
            mix_automation: None,
        }
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the mix parameter
    pub fn with_mix_automation(mut self, automation: Automation) -> Self {
        self.mix_automation = Some(automation);
        self
    }

    /// Add automation for the feedback parameter
    pub fn with_feedback_automation(mut self, automation: Automation) -> Self {
        self.feedback_automation = Some(automation);
        self
    }

    /// Add automation for the delay time parameter
    pub fn with_delay_time_automation(mut self, automation: Automation) -> Self {
        self.delay_time_automation = Some(automation);
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
        // Use bitwise AND instead of modulo for power-of-2
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.mix_automation {
                self.mix = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.feedback_automation {
                self.feedback = auto.value_at(time).clamp(0.0, 0.99);
            }
            if let Some(auto) = &self.delay_time_automation {
                self.delay_time = auto.value_at(time).clamp(0.001, 10.0);
            }
        }

        // Early exit for bypassed effect
        if self.mix < 0.0001 {
            return input;
        }

        // Read from delay buffer
        let delayed = self.buffer[self.write_pos];

        // Write input + feedback to buffer using FMA
        self.buffer[self.write_pos] = delayed.mul_add(self.feedback, input);

        // Advance write position using bitwise AND (assumes power-of-2 buffer size)
        let buffer_len = self.buffer.len();
        self.write_pos = (self.write_pos + 1) % buffer_len;

        // Mix dry and wet signals using FMA
        input.mul_add(1.0 - self.mix, delayed * self.mix)
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

    /// Reset the delay buffer
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }

    // ========== PRESETS ==========

    /// Eighth note delay (125ms at 120 BPM) with subtle feedback
    pub fn eighth_note() -> Self {
        Self::new(0.125, 0.35, 0.3)
    }

    /// Quarter note delay (250ms at 120 BPM) with moderate feedback
    pub fn quarter_note() -> Self {
        Self::new(0.25, 0.4, 0.35)
    }

    /// Dotted eighth delay (187.5ms at 120 BPM) - classic U2/Edge sound
    pub fn dotted_eighth() -> Self {
        Self::new(0.1875, 0.35, 0.3)
    }

    /// Half note delay (500ms at 120 BPM) with spacious feedback
    pub fn half_note() -> Self {
        Self::new(0.5, 0.45, 0.4)
    }

    /// Slapback delay (80ms) with no feedback - classic rockabilly/country sound
    pub fn slapback() -> Self {
        Self::new(0.08, 0.0, 0.3)
    }

    /// Ping-pong style delay with higher feedback for multiple repeats
    pub fn ping_pong() -> Self {
        Self::new(0.375, 0.5, 0.4)
    }

    /// Subtle doubling effect (30ms) for thickening vocals/instruments
    pub fn doubling() -> Self {
        Self::new(0.03, 0.0, 0.2)
    }

    /// Long ambient delay (1 second) with high feedback for soundscapes
    pub fn ambient() -> Self {
        Self::new(1.0, 0.6, 0.5)
    }
}
