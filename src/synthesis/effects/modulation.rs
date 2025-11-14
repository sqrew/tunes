use crate::synthesis::automation::Automation;
use crate::track::PRIORITY_MODULATION;

/// Standard audio sample rate
const DEFAULT_SAMPLE_RATE: f32 = 44100.0;

#[derive(Debug, Clone)]
struct AllPassFilter {
    z1: f32,
}

impl AllPassFilter {
    fn new() -> Self {
        Self { z1: 0.0 }
    }

    fn process(&mut self, input: f32, delay_samples: f32) -> f32 {
        let coefficient = (1.0 - delay_samples) / (1.0 + delay_samples);
        let output = -input + self.z1;
        self.z1 = input + coefficient * output;
        output
    }
}

/// Chorus - creates thickness by layering detuned copies
#[derive(Debug, Clone)]
pub struct Chorus {
    pub rate: f32,    // LFO rate in Hz (typical: 0.5 to 3.0)
    pub depth: f32,   // Modulation depth in milliseconds (typical: 2.0 to 10.0)
    pub mix: f32,     // Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub priority: u8, // Processing priority (lower = earlier in signal chain)
    buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,

    // Automation (optional)
    rate_automation: Option<Automation>,
    depth_automation: Option<Automation>,
    mix_automation: Option<Automation>,
}

impl Chorus {
    /// Create a new chorus effect
    ///
    /// # Arguments
    /// * `rate` - LFO speed in Hz (0.1 to 5.0, typical: 1.0)
    /// * `depth` - Modulation depth in milliseconds (1.0 to 20.0, typical: 5.0)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub fn new(rate: f32, depth: f32, mix: f32) -> Self {
        Self::with_sample_rate(rate, depth, mix, DEFAULT_SAMPLE_RATE)
    }

    /// Create a new chorus effect with custom sample rate
    pub fn with_sample_rate(rate: f32, depth: f32, mix: f32, sample_rate: f32) -> Self {
        // Buffer size needs to accommodate maximum delay
        let max_delay_samples = ((depth * 2.0) * sample_rate / 1000.0) as usize;
        let buffer_size = max_delay_samples.max(1);

        Self {
            rate: rate.clamp(0.1, 10.0),
            depth: depth.clamp(0.5, 50.0),
            mix: mix.clamp(0.0, 1.0),
            priority: PRIORITY_MODULATION, // Modulation effects in middle-late position
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            lfo_phase: 0.0,
            rate_automation: None,
            depth_automation: None,
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

    /// Add automation for the rate parameter
    pub fn with_rate_automation(mut self, automation: Automation) -> Self {
        self.rate_automation = Some(automation);
        self
    }

    /// Add automation for the depth parameter
    pub fn with_depth_automation(mut self, automation: Automation) -> Self {
        self.depth_automation = Some(automation);
        self
    }

    /// Process a single sample at given sample rate
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, sample_rate: f32, time: f32, sample_count: u64) -> f32 {
        // Quantized automation lookups (every 64 samples = 1.45ms @ 44.1kHz)
        // Use bitwise AND instead of modulo for power-of-2
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.mix_automation {
                self.mix = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.rate_automation {
                self.rate = auto.value_at(time).clamp(0.1, 10.0);
            }
            if let Some(auto) = &self.depth_automation {
                self.depth = auto.value_at(time).clamp(0.5, 50.0);
            }
        }

        if self.mix < 0.0001 {
            return input;
        }

        // Write input to buffer
        self.buffer[self.write_pos] = input;

        // Calculate modulated delay time using sine LFO
        let lfo = (self.lfo_phase * 2.0 * std::f32::consts::PI).sin();
        let delay_ms = self.depth.mul_add(0.5 + 0.5 * lfo, 0.0);
        let delay_samples = ((delay_ms * sample_rate / 1000.0) as usize).min(self.buffer.len() - 1);

        // Read from delayed position
        let read_pos = (self.write_pos + self.buffer.len() - delay_samples) % self.buffer.len();
        let delayed = self.buffer[read_pos];

        // Advance LFO phase
        self.lfo_phase += self.rate / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        // Advance write position
        self.write_pos = (self.write_pos + 1) % self.buffer.len();

        // Mix dry and wet using FMA
        input.mul_add(1.0 - self.mix, delayed * self.mix)
    }

    /// Process a block of samples
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], sample_rate: f32, time: f32, sample_count: u64) {
        let time_delta = 1.0 / sample_rate;
        for (i, sample) in buffer.iter_mut().enumerate() {
            let current_time = time + (i as f32 * time_delta);
            let current_sample_count = sample_count + i as u64;
            *sample = self.process(*sample, sample_rate, current_time, current_sample_count);
        }
    }

    /// Reset the chorus state
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }

    // ========== PRESETS ==========

    /// Subtle chorus - gentle thickening
    pub fn subtle() -> Self {
        Self::new(0.5, 3.0, 0.3)
    }

    /// Classic chorus - 80s style chorus effect
    pub fn classic() -> Self {
        Self::new(1.5, 5.0, 0.5)
    }

    /// Wide chorus - expansive stereo spread
    pub fn wide() -> Self {
        Self::new(0.8, 8.0, 0.6)
    }

    /// Vibrato - 100% wet for pitch modulation
    pub fn vibrato() -> Self {
        Self::new(5.0, 3.0, 1.0)
    }

    /// Thick - dense, layered sound
    pub fn thick() -> Self {
        Self::new(2.0, 7.0, 0.7)
    }
}

/// Phaser - creates sweeping notches in the frequency spectrum
#[derive(Debug, Clone)]
pub struct Phaser {
    pub rate: f32,     // LFO rate in Hz (typical: 0.1 to 5.0)
    pub depth: f32,    // Modulation depth (0.0 to 1.0)
    pub feedback: f32, // Feedback amount (0.0 to 0.95)
    pub mix: f32,      // Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub stages: usize, // Number of all-pass filter stages (2, 4, 6, or 8)
    pub priority: u8,  // Processing priority (lower = earlier in signal chain)
    allpass_states: Vec<AllPassFilter>,
    lfo_phase: f32,

    // Automation (optional)
    rate_automation: Option<Automation>,
    depth_automation: Option<Automation>,
    feedback_automation: Option<Automation>,
    mix_automation: Option<Automation>,
}

impl Phaser {
    /// Create a new phaser effect
    ///
    /// # Arguments
    /// * `rate` - LFO speed in Hz (0.1 to 5.0, typical: 0.5)
    /// * `depth` - Modulation depth (0.0 to 1.0, typical: 0.7)
    /// * `feedback` - Feedback amount (0.0 to 0.95, typical: 0.5)
    /// * `stages` - Number of stages (2, 4, 6, or 8, typical: 4)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet, typical: 0.5)
    pub fn new(rate: f32, depth: f32, feedback: f32, stages: usize, mix: f32) -> Self {
        Self::with_sample_rate(rate, depth, feedback, stages, mix, DEFAULT_SAMPLE_RATE)
    }

    /// Create a new phaser effect
    ///
    /// Note: sample_rate parameter is ignored (kept for API compatibility).
    /// The actual sample rate is provided during processing.
    pub fn with_sample_rate(
        rate: f32,
        depth: f32,
        feedback: f32,
        stages: usize,
        mix: f32,
        _sample_rate: f32,
    ) -> Self {
        let stages = stages.clamp(2, 8);
        Self {
            rate,
            depth: depth.clamp(0.0, 1.0),
            feedback: feedback.clamp(0.0, 0.95),
            mix: mix.clamp(0.0, 1.0),
            stages,
            priority: PRIORITY_MODULATION, // Modulation effects in middle-late position
            allpass_states: vec![AllPassFilter::new(); stages],
            lfo_phase: 0.0,
            rate_automation: None,
            depth_automation: None,
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

    /// Add automation for the rate parameter
    pub fn with_rate_automation(mut self, automation: Automation) -> Self {
        self.rate_automation = Some(automation);
        self
    }

    /// Add automation for the depth parameter
    pub fn with_depth_automation(mut self, automation: Automation) -> Self {
        self.depth_automation = Some(automation);
        self
    }

    /// Add automation for the feedback parameter
    pub fn with_feedback_automation(mut self, automation: Automation) -> Self {
        self.feedback_automation = Some(automation);
        self
    }

    /// Process a single sample
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, sample_rate: f32, time: f32, sample_count: u64) -> f32 {
        // Quantized automation lookups (every 64 samples = 1.45ms @ 44.1kHz)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.mix_automation {
                self.mix = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.rate_automation {
                self.rate = auto.value_at(time).clamp(0.1, 10.0);
            }
            if let Some(auto) = &self.depth_automation {
                self.depth = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.feedback_automation {
                self.feedback = auto.value_at(time).clamp(0.0, 0.95);
            }
        }

        if self.mix < 0.0001 || self.depth < 0.0001 {
            return input;
        }

        // Generate LFO
        let lfo = (self.lfo_phase * 2.0 * std::f32::consts::PI).sin();

        // Map LFO to delay range (affects frequency of notches) using FMA
        let min_delay = 0.5;
        let max_delay = 5.0;
        let delay = (0.5 + 0.5 * lfo * self.depth).mul_add(max_delay - min_delay, min_delay);

        // Process through all-pass filter stages
        let mut output = input;
        for filter in &mut self.allpass_states {
            output = filter.process(output, delay);
        }

        // Apply feedback using FMA
        let feedback_sample = output * self.feedback;
        output = input + feedback_sample;

        // Advance LFO phase
        self.lfo_phase += self.rate / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        // Mix dry and wet using FMA
        input.mul_add(1.0 - self.mix, output * self.mix)
    }

    /// Process a block of samples
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], sample_rate: f32, time: f32, sample_count: u64) {
        let time_delta = 1.0 / sample_rate;
        for (i, sample) in buffer.iter_mut().enumerate() {
            let current_time = time + (i as f32 * time_delta);
            let current_sample_count = sample_count + i as u64;
            *sample = self.process(*sample, sample_rate, current_time, current_sample_count);
        }
    }

    /// Reset the phaser state
    pub fn reset(&mut self) {
        self.allpass_states = vec![AllPassFilter::new(); self.stages];
        self.lfo_phase = 0.0;
    }

    // ========== PRESETS ==========

    /// Slow phaser - gentle sweep (0.3 Hz)
    pub fn slow() -> Self {
        Self::new(0.3, 0.7, 0.5, 4, 0.5)
    }

    /// Classic phaser - 70s style phasing (0.5 Hz, 4 stages)
    pub fn classic() -> Self {
        Self::new(0.5, 0.8, 0.6, 4, 0.6)
    }

    /// Fast phaser - intense modulation (2.0 Hz, 6 stages)
    pub fn fast() -> Self {
        Self::new(2.0, 0.9, 0.7, 6, 0.7)
    }

    /// Subtle phaser - barely-there swoosh (0.4 Hz, mild depth)
    pub fn subtle() -> Self {
        Self::new(0.4, 0.5, 0.3, 4, 0.4)
    }

    /// Deep phaser - thick, resonant sweep (0.6 Hz, 8 stages)
    pub fn deep() -> Self {
        Self::new(0.6, 1.0, 0.8, 8, 0.8)
    }
}

/// Flanger - creates jet-plane/swoosh effects with very short delays
#[derive(Debug, Clone)]
pub struct Flanger {
    pub rate: f32,     // LFO rate in Hz (typical: 0.1 to 2.0)
    pub depth: f32,    // Modulation depth in milliseconds (typical: 1.0 to 5.0)
    pub feedback: f32, // Feedback amount (0.0 to 0.95)
    pub mix: f32,      // Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub priority: u8,  // Processing priority (lower = earlier in signal chain)
    buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,

    // Automation (optional)
    rate_automation: Option<Automation>,
    depth_automation: Option<Automation>,
    feedback_automation: Option<Automation>,
    mix_automation: Option<Automation>,
}

impl Flanger {
    /// Create a new flanger effect
    ///
    /// # Arguments
    /// * `rate` - LFO speed in Hz (0.1 to 2.0, typical: 0.5)
    /// * `depth` - Modulation depth in milliseconds (1.0 to 10.0, typical: 3.0)
    /// * `feedback` - Feedback amount (0.0 to 0.95, typical: 0.6)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet, typical: 0.5)
    pub fn new(rate: f32, depth: f32, feedback: f32, mix: f32) -> Self {
        Self::with_sample_rate(rate, depth, feedback, mix, DEFAULT_SAMPLE_RATE)
    }

    /// Create a new flanger effect
    ///
    /// Note: sample_rate parameter is used only to estimate buffer size.
    /// The actual sample rate for processing is provided during process().
    pub fn with_sample_rate(
        rate: f32,
        depth: f32,
        feedback: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        // Buffer size needs to accommodate maximum delay (in samples)
        // Use provided sample_rate for initial buffer sizing
        let max_delay_samples = ((depth * 2.0) * sample_rate / 1000.0) as usize;
        let buffer_size = max_delay_samples.max(1);

        Self {
            rate,
            depth,
            feedback: feedback.clamp(0.0, 0.95),
            mix: mix.clamp(0.0, 1.0),
            priority: PRIORITY_MODULATION, // Modulation effects in middle-late position
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            lfo_phase: 0.0,
            rate_automation: None,
            depth_automation: None,
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

    /// Add automation for the rate parameter
    pub fn with_rate_automation(mut self, automation: Automation) -> Self {
        self.rate_automation = Some(automation);
        self
    }

    /// Add automation for the depth parameter
    pub fn with_depth_automation(mut self, automation: Automation) -> Self {
        self.depth_automation = Some(automation);
        self
    }

    /// Add automation for the feedback parameter
    pub fn with_feedback_automation(mut self, automation: Automation) -> Self {
        self.feedback_automation = Some(automation);
        self
    }

    /// Process a single sample
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, sample_rate: f32, time: f32, sample_count: u64) -> f32 {
        // Quantized automation lookups (every 64 samples = 1.45ms @ 44.1kHz)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.mix_automation {
                self.mix = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.rate_automation {
                self.rate = auto.value_at(time).clamp(0.1, 10.0);
            }
            if let Some(auto) = &self.depth_automation {
                self.depth = auto.value_at(time).clamp(0.5, 50.0);
            }
            if let Some(auto) = &self.feedback_automation {
                self.feedback = auto.value_at(time).clamp(0.0, 0.95);
            }
        }

        // Safety check: if buffer is empty, just pass through
        if self.buffer.is_empty() || self.mix < 0.0001 {
            return input;
        }

        // Calculate modulated delay time using sine LFO with FMA
        let lfo = (self.lfo_phase * 2.0 * std::f32::consts::PI).sin();
        let delay_ms = self.depth.mul_add(0.5 + 0.5 * lfo, 0.0); // 0 to depth milliseconds
        let delay_samples =
            ((delay_ms * sample_rate / 1000.0) as usize).min(self.buffer.len() - 1);

        // Read from delayed position
        let read_pos = if self.write_pos >= delay_samples {
            self.write_pos - delay_samples
        } else {
            self.buffer.len() - (delay_samples - self.write_pos)
        };
        let delayed = self.buffer[read_pos];

        // Write to buffer with feedback using FMA
        self.buffer[self.write_pos] = delayed.mul_add(self.feedback, input);

        // Advance LFO phase
        self.lfo_phase += self.rate / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        // Advance write position
        self.write_pos = (self.write_pos + 1) % self.buffer.len();

        // Mix dry and wet using FMA
        input.mul_add(1.0 - self.mix, delayed * self.mix)
    }

    /// Process a block of samples
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], sample_rate: f32, time: f32, sample_count: u64) {
        let time_delta = 1.0 / sample_rate;
        for (i, sample) in buffer.iter_mut().enumerate() {
            let current_time = time + (i as f32 * time_delta);
            let current_sample_count = sample_count + i as u64;
            *sample = self.process(*sample, sample_rate, current_time, current_sample_count);
        }
    }

    /// Reset the flanger state
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }

    // ========== PRESETS ==========

    /// Subtle flanger - gentle swoosh (0.5 Hz)
    pub fn subtle() -> Self {
        Self::new(0.5, 2.0, 0.3, 0.4)
    }

    /// Classic flanger - balanced flanging effect (1.0 Hz)
    pub fn classic() -> Self {
        Self::new(1.0, 3.0, 0.5, 0.6)
    }

    /// Jet flanger - dramatic jet-plane effect (2.0 Hz, high feedback)
    pub fn jet() -> Self {
        Self::new(2.0, 5.0, 0.8, 0.8)
    }

    /// Through-zero flanger - authentic through-zero sound (1.0 Hz)
    pub fn through_zero() -> Self {
        Self::new(1.0, 4.0, 0.9, 1.0)
    }

    /// Metallic flanger - sharp, resonant (1.5 Hz, high feedback)
    pub fn metallic() -> Self {
        Self::new(1.5, 4.5, 0.85, 0.85)
    }
}

/// Ring Modulator - creates metallic/robotic inharmonic tones
#[derive(Debug, Clone)]
pub struct RingModulator {
    pub carrier_freq: f32, // Carrier frequency in Hz (typical: 50 to 5000)
    pub mix: f32,          // Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub priority: u8,      // Processing priority (lower = earlier in signal chain)
    phase: f32,

    // Automation (optional)
    carrier_freq_automation: Option<Automation>,
    mix_automation: Option<Automation>,
}

impl RingModulator {
    /// Create a new ring modulator effect
    ///
    /// # Arguments
    /// * `carrier_freq` - Carrier frequency in Hz (50 to 5000, typical: 440)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet, typical: 0.7)
    pub fn new(carrier_freq: f32, mix: f32) -> Self {
        Self::with_sample_rate(carrier_freq, mix, DEFAULT_SAMPLE_RATE)
    }

    /// Create a new ring modulator effect with custom sample rate
    /// Note: sample_rate parameter is kept for API compatibility but ignored.
    /// The actual sample rate is provided at runtime during processing.
    pub fn with_sample_rate(carrier_freq: f32, mix: f32, _sample_rate: f32) -> Self {
        Self {
            carrier_freq,
            mix: mix.clamp(0.0, 1.0),
            priority: PRIORITY_MODULATION, // Modulation effects in middle-late position
            phase: 0.0,
            carrier_freq_automation: None,
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

    /// Add automation for the carrier frequency parameter
    pub fn with_carrier_freq_automation(mut self, automation: Automation) -> Self {
        self.carrier_freq_automation = Some(automation);
        self
    }

    /// Process a single sample
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `sample_rate` - Audio sample rate in Hz (provided by the engine at runtime)
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, sample_rate: f32, time: f32, sample_count: u64) -> f32 {
        // Quantized automation lookups (every 64 samples = 1.45ms @ 44.1kHz)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.mix_automation {
                self.mix = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.carrier_freq_automation {
                self.carrier_freq = auto.value_at(time).clamp(20.0, 10000.0);
            }
        }

        if self.mix < 0.0001 {
            return input;
        }

        // Generate carrier sine wave using fast wavetable lookup
        let carrier = crate::synthesis::wavetable::WAVETABLE.sample(self.phase);

        // Ring modulation = multiplication
        let modulated = input * carrier;

        // Advance phase
        self.phase += self.carrier_freq / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        // Mix dry and wet using FMA
        input.mul_add(1.0 - self.mix, modulated * self.mix)
    }

    /// Process a block of samples with SIMD acceleration
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], sample_rate: f32, time: f32, sample_count: u64) {
        use crate::synthesis::simd::{SimdWidth, SIMD};

        // Update automation params if needed
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.mix_automation {
                self.mix = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.carrier_freq_automation {
                self.carrier_freq = auto.value_at(time).clamp(20.0, 10000.0);
            }
        }

        // Early exit if no effect
        if self.mix < 0.0001 {
            return;
        }

        // Dispatch to SIMD implementation
        match SIMD.simd_width() {
            SimdWidth::X8 => self.process_block_simd::<8>(buffer, sample_rate),
            SimdWidth::X4 => self.process_block_simd::<4>(buffer, sample_rate),
            SimdWidth::Scalar => self.process_block_scalar(buffer, sample_rate),
        }
    }

    /// SIMD implementation - processes N samples at once
    #[inline(always)]
    fn process_block_simd<const N: usize>(&mut self, buffer: &mut [f32], sample_rate: f32) {
        let phase_increment = self.carrier_freq / sample_rate;
        let mix = self.mix;
        let one_minus_mix = 1.0 - mix;

        let num_chunks = buffer.len() / N;
        let remainder_start = num_chunks * N;

        // Process N samples at a time
        for chunk_idx in 0..num_chunks {
            let chunk_start = chunk_idx * N;
            let chunk = &mut buffer[chunk_start..chunk_start + N];

            // Generate N carrier phases
            let mut phases = [0.0f32; 8]; // Max size for N=8
            for i in 0..N {
                phases[i] = self.phase + (i as f32) * phase_increment;
                if phases[i] >= 1.0 {
                    phases[i] -= 1.0;
                }
            }

            // Process N samples
            for i in 0..N {
                let carrier = crate::synthesis::wavetable::WAVETABLE.sample(phases[i]);
                let input = chunk[i];
                let modulated = input * carrier;
                chunk[i] = input.mul_add(one_minus_mix, modulated * mix);
            }

            self.phase += (N as f32) * phase_increment;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }

        // Handle remainder with scalar
        for i in remainder_start..buffer.len() {
            let carrier = crate::synthesis::wavetable::WAVETABLE.sample(self.phase);
            let input = buffer[i];
            let modulated = input * carrier;
            buffer[i] = input.mul_add(one_minus_mix, modulated * mix);

            self.phase += phase_increment;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }
    }

    /// Scalar fallback
    #[inline(always)]
    fn process_block_scalar(&mut self, buffer: &mut [f32], sample_rate: f32) {
        let phase_increment = self.carrier_freq / sample_rate;
        let mix = self.mix;
        let one_minus_mix = 1.0 - mix;

        for sample in buffer.iter_mut() {
            let carrier = crate::synthesis::wavetable::WAVETABLE.sample(self.phase);
            let input = *sample;
            let modulated = input * carrier;
            *sample = input.mul_add(one_minus_mix, modulated * mix);

            self.phase += phase_increment;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }
    }

    /// Reset the ring modulator state
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    // ========== PRESETS ==========

    /// Metallic - classic ring mod tone (440 Hz carrier)
    pub fn metallic() -> Self {
        Self::new(440.0, 0.7)
    }

    /// Robotic - mid-range carrier for voice effects (220 Hz)
    pub fn robotic() -> Self {
        Self::new(220.0, 0.8)
    }

    /// Bell-like - high carrier for bell tones (880 Hz)
    pub fn bell() -> Self {
        Self::new(880.0, 0.6)
    }

    /// Deep - low carrier for sub-bass effects (110 Hz)
    pub fn deep() -> Self {
        Self::new(110.0, 0.75)
    }

    /// Harsh - high carrier for aggressive tones (1760 Hz)
    pub fn harsh() -> Self {
        Self::new(1760.0, 0.9)
    }

    /// Subtle - gentle inharmonic color (330 Hz)
    pub fn subtle() -> Self {
        Self::new(330.0, 0.4)
    }
}

/// Tremolo - rhythmic amplitude modulation
///
/// Creates periodic volume changes, adding rhythmic movement to the signal.
/// Lower frequency rates (< 10 Hz) create a pulsing effect, while higher rates
/// can create vibrato-like timbral changes.
#[derive(Debug, Clone)]
pub struct Tremolo {
    pub rate: f32,    // LFO rate in Hz (typically 1-20 Hz)
    pub depth: f32,   // Modulation depth 0.0 to 1.0
    pub priority: u8, // Processing priority
    phase: f32,       // LFO phase (0.0 to 1.0)

    // Automation (optional)
    rate_automation: Option<Automation>,
    depth_automation: Option<Automation>,
}

impl Tremolo {
    /// Create a new tremolo effect
    ///
    /// # Arguments
    /// * `rate` - LFO frequency in Hz (typically 1-20 Hz)
    /// * `depth` - Modulation depth 0.0 (no effect) to 1.0 (full tremolo)
    /// * `sample_rate` - Audio sample rate in Hz (kept for API compatibility but ignored)
    ///
    /// **Note:** The actual sample rate is provided at runtime during processing.
    pub fn with_sample_rate(rate: f32, depth: f32, _sample_rate: f32) -> Self {
        Self {
            rate: rate.max(0.01),
            depth: depth.clamp(0.0, 1.0),
            priority: PRIORITY_MODULATION,
            phase: 0.0,
            rate_automation: None,
            depth_automation: None,
        }
    }

    /// Create a tremolo with default sample rate (44100 Hz)
    pub fn new(rate: f32, depth: f32) -> Self {
        Self::with_sample_rate(rate, depth, DEFAULT_SAMPLE_RATE)
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the rate parameter
    pub fn with_rate_automation(mut self, automation: Automation) -> Self {
        self.rate_automation = Some(automation);
        self
    }

    /// Add automation for the depth parameter
    pub fn with_depth_automation(mut self, automation: Automation) -> Self {
        self.depth_automation = Some(automation);
        self
    }

    /// Process a single sample
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `sample_rate` - Audio sample rate in Hz (provided by the engine at runtime)
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, sample_rate: f32, time: f32, sample_count: u64) -> f32 {
        // Quantized automation lookups (every 64 samples)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.rate_automation {
                self.rate = auto.value_at(time).max(0.01);
            }
            if let Some(auto) = &self.depth_automation {
                self.depth = auto.value_at(time).clamp(0.0, 1.0);
            }
        }

        // Early out if no modulation
        if self.depth < 0.0001 {
            return input;
        }

        // Generate LFO (sine wave)
        let lfo = (self.phase * 2.0 * std::f32::consts::PI).sin();

        // Convert bipolar LFO (-1 to 1) to unipolar modulation
        // depth controls how much the volume varies
        let modulation = 1.0 - (self.depth * (1.0 - lfo) * 0.5);

        // Advance phase
        self.phase += self.rate / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        input * modulation
    }

    /// Process a block of samples with SIMD acceleration
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], sample_rate: f32, time: f32, sample_count: u64) {
        use crate::synthesis::simd::{SimdWidth, SIMD};

        // Update automation params if needed
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.rate_automation {
                self.rate = auto.value_at(time).clamp(0.1, 100.0);
            }
            if let Some(auto) = &self.depth_automation {
                self.depth = auto.value_at(time).clamp(0.0, 1.0);
            }
        }

        // Early exit if no effect
        if self.depth < 0.0001 {
            return;
        }

        // Dispatch to SIMD implementation
        match SIMD.simd_width() {
            SimdWidth::X8 => self.process_block_simd::<8>(buffer, sample_rate),
            SimdWidth::X4 => self.process_block_simd::<4>(buffer, sample_rate),
            SimdWidth::Scalar => self.process_block_scalar(buffer, sample_rate),
        }
    }

    /// SIMD implementation - processes N samples at once
    #[inline(always)]
    fn process_block_simd<const N: usize>(&mut self, buffer: &mut [f32], sample_rate: f32) {
        use std::f32::consts::PI;
        let phase_increment = self.rate / sample_rate;
        let depth = self.depth;
        let two_pi = 2.0 * PI;

        let num_chunks = buffer.len() / N;
        let remainder_start = num_chunks * N;

        // Process N samples at a time
        for chunk_idx in 0..num_chunks {
            let chunk_start = chunk_idx * N;
            let chunk = &mut buffer[chunk_start..chunk_start + N];

            // Generate N phases
            let mut phases = [0.0f32; 8]; // Max size for N=8
            for i in 0..N {
                phases[i] = self.phase + (i as f32) * phase_increment;
                if phases[i] >= 1.0 {
                    phases[i] -= 1.0;
                }
            }

            // Process N samples
            for i in 0..N {
                let lfo = (phases[i] * two_pi).sin();
                let modulation = 1.0 - (depth * (1.0 - lfo) * 0.5);
                chunk[i] *= modulation;
            }

            self.phase += (N as f32) * phase_increment;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }

        // Handle remainder with scalar
        for i in remainder_start..buffer.len() {
            let lfo = (self.phase * two_pi).sin();
            let modulation = 1.0 - (depth * (1.0 - lfo) * 0.5);
            buffer[i] *= modulation;

            self.phase += phase_increment;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }
    }

    /// Scalar fallback
    #[inline(always)]
    fn process_block_scalar(&mut self, buffer: &mut [f32], sample_rate: f32) {
        use std::f32::consts::PI;
        let phase_increment = self.rate / sample_rate;
        let depth = self.depth;
        let two_pi = 2.0 * PI;

        for sample in buffer.iter_mut() {
            let lfo = (self.phase * two_pi).sin();
            let modulation = 1.0 - (depth * (1.0 - lfo) * 0.5);
            *sample *= modulation;

            self.phase += phase_increment;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }
    }

    /// Reset the tremolo state
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    // ========== PRESETS ==========

    /// Slow tremolo - gentle, pulsing effect (2 Hz)
    pub fn slow() -> Self {
        Self::new(2.0, 0.5)
    }

    /// Classic tremolo - standard rock/blues tremolo (4 Hz)
    pub fn classic() -> Self {
        Self::new(4.0, 0.6)
    }

    /// Fast tremolo - intense modulation (8 Hz)
    pub fn fast() -> Self {
        Self::new(8.0, 0.7)
    }

    /// Subtle tremolo - barely noticeable pulse (3 Hz)
    pub fn subtle() -> Self {
        Self::new(3.0, 0.3)
    }

    /// Helicopter - extreme, rhythmic chop (12 Hz)
    pub fn helicopter() -> Self {
        Self::new(12.0, 0.9)
    }
}
