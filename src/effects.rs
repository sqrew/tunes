use crate::automation::Automation;
use crate::track::{
    PRIORITY_EARLY, PRIORITY_LAST, PRIORITY_MODULATION, PRIORITY_NORMAL, PRIORITY_SPATIAL,
    PRIORITY_TIME_BASED,
};

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

    /// Reset the delay buffer
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }
}

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

    /// Reset the reverb state
    pub fn reset(&mut self) {
        for buffer in &mut self.comb_buffers {
            buffer.fill(0.0);
        }
        self.comb_positions.fill(0);
        self.filter_state.fill(0.0);
    }
}

/// Distortion/overdrive effect
#[derive(Debug, Clone)]
pub struct Distortion {
    pub drive: f32,   // Drive amount (1.0 = no distortion, higher = more)
    pub mix: f32,     // Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub priority: u8, // Processing priority (lower = earlier in signal chain)

    // Automation (optional)
    drive_automation: Option<Automation>,
    mix_automation: Option<Automation>,
}

impl Distortion {
    /// Create a new distortion effect
    pub fn new(drive: f32, mix: f32) -> Self {
        Self {
            drive: drive.max(1.0),
            mix: mix.clamp(0.0, 1.0),
            priority: PRIORITY_NORMAL, // Distortion in normal/middle position
            drive_automation: None,
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

    /// Add automation for the drive parameter
    pub fn with_drive_automation(mut self, automation: Automation) -> Self {
        self.drive_automation = Some(automation);
        self
    }

    /// Process a single sample using soft clipping
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
            if let Some(auto) = &self.drive_automation {
                self.drive = auto.value_at(time).max(1.0);
            }
        }

        if self.mix < 0.0001 {
            return input;
        }

        let amplified = input * self.drive;

        // Soft clipping using tanh
        let distorted = amplified.tanh();

        // Compensate for gain increase
        let normalized = distorted / self.drive.sqrt();

        // Mix dry and wet using FMA
        input.mul_add(1.0 - self.mix, normalized * self.mix)
    }
}

/// Bit crusher - lo-fi digital degradation effect
#[derive(Debug, Clone)]
pub struct BitCrusher {
    pub bit_depth: f32,             // Bit depth (1.0 to 16.0, lower = more crushing)
    pub sample_rate_reduction: f32, // Sample rate divisor (1.0 = no reduction, higher = more lo-fi)
    pub mix: f32,                   // Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub priority: u8,               // Processing priority (lower = earlier in signal chain)
    hold_sample: f32,
    sample_counter: f32,

    // Automation (optional)
    bit_depth_automation: Option<Automation>,
    sample_rate_reduction_automation: Option<Automation>,
    mix_automation: Option<Automation>,
}

impl BitCrusher {
    /// Create a new bit crusher effect
    ///
    /// # Arguments
    /// * `bit_depth` - Bit depth (1.0 to 16.0, typical: 4.0-8.0 for lo-fi)
    /// * `sample_rate_reduction` - Downsample factor (1.0 = original, 4.0 = quarter rate)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub fn new(bit_depth: f32, sample_rate_reduction: f32, mix: f32) -> Self {
        Self {
            bit_depth: bit_depth.clamp(1.0, 16.0),
            sample_rate_reduction: sample_rate_reduction.max(1.0),
            mix: mix.clamp(0.0, 1.0),
            priority: PRIORITY_NORMAL, // BitCrusher in normal position
            hold_sample: 0.0,
            sample_counter: 0.0,
            bit_depth_automation: None,
            sample_rate_reduction_automation: None,
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

    /// Add automation for the bit depth parameter
    pub fn with_bit_depth_automation(mut self, automation: Automation) -> Self {
        self.bit_depth_automation = Some(automation);
        self
    }

    /// Add automation for the sample rate reduction parameter
    pub fn with_sample_rate_reduction_automation(mut self, automation: Automation) -> Self {
        self.sample_rate_reduction_automation = Some(automation);
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
            if let Some(auto) = &self.bit_depth_automation {
                self.bit_depth = auto.value_at(time).clamp(1.0, 16.0);
            }
            if let Some(auto) = &self.sample_rate_reduction_automation {
                self.sample_rate_reduction = auto.value_at(time).max(1.0);
            }
        }

        // Sample rate reduction (sample & hold)
        self.sample_counter += 1.0;
        if self.sample_counter >= self.sample_rate_reduction {
            self.hold_sample = input.clamp(-2.0, 2.0);
            self.sample_counter = 0.0;
        }

        // Bit depth reduction (quantization)
        // Use exp2 instead of powf for 2^x (much faster)
        let levels = self.bit_depth.exp2();
        let quantized = (self.hold_sample * levels).round() / levels;

        // Mix dry and wet using FMA, clamp output
        let output = input.mul_add(1.0 - self.mix, quantized * self.mix);
        output.clamp(-2.0, 2.0)
    }

    /// Reset the bit crusher state
    pub fn reset(&mut self) {
        self.hold_sample = 0.0;
        self.sample_counter = 0.0;
    }
}

/// Compressor - dynamic range control
#[derive(Debug, Clone)]
pub struct Compressor {
    pub threshold: f32, // Threshold in amplitude 0.0-1.0 (NOT dB! 0.3 ≈ -10dB, 0.5 ≈ -6dB)
    pub ratio: f32,     // Compression ratio (1.0 = no compression, 10.0 = heavy)
    pub attack: f32,    // Attack time in seconds
    pub release: f32,   // Release time in seconds
    pub makeup_gain: f32, // Makeup gain to compensate for volume reduction
    pub priority: u8,   // Processing priority (lower = earlier in signal chain)
    envelope: f32,

    // Automation (optional)
    threshold_automation: Option<Automation>,
    ratio_automation: Option<Automation>,
    attack_automation: Option<Automation>,
    release_automation: Option<Automation>,
    makeup_gain_automation: Option<Automation>,
}

impl Compressor {
    /// Create a new compressor
    ///
    /// # Arguments
    /// * `threshold` - Level above which compression starts in amplitude (0.0 to 1.0, NOT dB!)
    ///   Typical values: 0.5 = gentle, 0.3 = moderate, 0.2 = aggressive
    /// * `ratio` - Compression ratio (2.0 = gentle, 10.0 = heavy limiting)
    /// * `attack` - Attack time in seconds (typical: 0.001 to 0.1)
    /// * `release` - Release time in seconds (typical: 0.05 to 0.5)
    /// * `makeup_gain` - Output gain multiplier (1.0 to 4.0)
    pub fn new(threshold: f32, ratio: f32, attack: f32, release: f32, makeup_gain: f32) -> Self {
        Self {
            threshold: threshold.clamp(0.0, 1.0),
            ratio: ratio.max(1.0),
            attack: attack.max(0.001),
            release: release.max(0.001),
            makeup_gain: makeup_gain.max(0.1),
            priority: PRIORITY_EARLY, // Compressor typically early in chain
            envelope: 0.0,
            threshold_automation: None,
            ratio_automation: None,
            attack_automation: None,
            release_automation: None,
            makeup_gain_automation: None,
        }
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the threshold parameter
    pub fn with_threshold_automation(mut self, automation: Automation) -> Self {
        self.threshold_automation = Some(automation);
        self
    }

    /// Add automation for the ratio parameter
    pub fn with_ratio_automation(mut self, automation: Automation) -> Self {
        self.ratio_automation = Some(automation);
        self
    }

    /// Add automation for the attack parameter
    pub fn with_attack_automation(mut self, automation: Automation) -> Self {
        self.attack_automation = Some(automation);
        self
    }

    /// Add automation for the release parameter
    pub fn with_release_automation(mut self, automation: Automation) -> Self {
        self.release_automation = Some(automation);
        self
    }

    /// Add automation for the makeup gain parameter
    pub fn with_makeup_gain_automation(mut self, automation: Automation) -> Self {
        self.makeup_gain_automation = Some(automation);
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
            if let Some(auto) = &self.threshold_automation {
                self.threshold = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.ratio_automation {
                self.ratio = auto.value_at(time).max(1.0);
            }
            if let Some(auto) = &self.attack_automation {
                self.attack = auto.value_at(time).max(0.001);
            }
            if let Some(auto) = &self.release_automation {
                self.release = auto.value_at(time).max(0.001);
            }
            if let Some(auto) = &self.makeup_gain_automation {
                self.makeup_gain = auto.value_at(time).max(0.1);
            }
        }

        let input_level = input.abs();

        // Envelope follower with pre-computed coefficients
        let attack_coeff = (-1.0 / (self.attack * sample_rate)).exp();
        let release_coeff = (-1.0 / (self.release * sample_rate)).exp();

        // Use FMA for envelope calculation
        let coeff = if input_level > self.envelope {
            attack_coeff
        } else {
            release_coeff
        };
        self.envelope = self.envelope.mul_add(coeff, input_level * (1.0 - coeff));

        // Clamp envelope to prevent runaway values
        self.envelope = self.envelope.clamp(0.0, 2.0);

        // Calculate gain reduction
        let gain = if self.envelope > self.threshold {
            let over_threshold = self.envelope / self.threshold.max(0.001); // Prevent division by zero
            let compressed = over_threshold.powf(1.0 / self.ratio);
            (compressed * self.threshold / self.envelope).clamp(0.0, 1.0)
        } else {
            1.0
        };

        // Apply compression and makeup gain using FMA, clamp output to prevent clipping
        let output = input * gain * self.makeup_gain;
        output.clamp(-2.0, 2.0)
    }

    /// Reset the compressor state
    pub fn reset(&mut self) {
        self.envelope = 0.0;
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
        let delay_samples = (delay_ms * sample_rate / 1000.0) as usize;

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

    /// Reset the chorus state
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }
}

/// Parametric EQ - 3-band equalizer
#[derive(Debug, Clone)]
pub struct EQ {
    pub low_gain: f32,  // Low frequency gain (0.0 to 2.0, 1.0 = unity)
    pub mid_gain: f32,  // Mid frequency gain (0.0 to 2.0, 1.0 = unity)
    pub high_gain: f32, // High frequency gain (0.0 to 2.0, 1.0 = unity)
    pub low_freq: f32,  // Low band center frequency (Hz)
    pub high_freq: f32, // High band center frequency (Hz)
    pub priority: u8,   // Processing priority (lower = earlier in signal chain)
    // State variables for filters
    low_state: [f32; 2],
    mid_state: [f32; 2],
    high_state: [f32; 2],

    // Automation (optional)
    low_gain_automation: Option<Automation>,
    mid_gain_automation: Option<Automation>,
    high_gain_automation: Option<Automation>,
}

impl EQ {
    /// Create a new 3-band parametric EQ
    ///
    /// # Arguments
    /// * `low_gain` - Low frequency gain (0.5 = -6dB, 1.0 = 0dB, 2.0 = +6dB)
    /// * `mid_gain` - Mid frequency gain
    /// * `high_gain` - High frequency gain
    /// * `low_freq` - Low/mid crossover frequency (typical: 250 Hz)
    /// * `high_freq` - Mid/high crossover frequency (typical: 4000 Hz)
    pub fn new(
        low_gain: f32,
        mid_gain: f32,
        high_gain: f32,
        low_freq: f32,
        high_freq: f32,
    ) -> Self {
        Self {
            low_gain: low_gain.clamp(0.0, 4.0),
            mid_gain: mid_gain.clamp(0.0, 4.0),
            high_gain: high_gain.clamp(0.0, 4.0),
            low_freq: low_freq.clamp(20.0, 20000.0),
            high_freq: high_freq.clamp(20.0, 20000.0),
            priority: PRIORITY_EARLY, // EQ typically early in chain
            low_state: [0.0; 2],
            mid_state: [0.0; 2],
            high_state: [0.0; 2],
            low_gain_automation: None,
            mid_gain_automation: None,
            high_gain_automation: None,
        }
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the low gain parameter
    pub fn with_low_gain_automation(mut self, automation: Automation) -> Self {
        self.low_gain_automation = Some(automation);
        self
    }

    /// Add automation for the mid gain parameter
    pub fn with_mid_gain_automation(mut self, automation: Automation) -> Self {
        self.mid_gain_automation = Some(automation);
        self
    }

    /// Add automation for the high gain parameter
    pub fn with_high_gain_automation(mut self, automation: Automation) -> Self {
        self.high_gain_automation = Some(automation);
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
            if let Some(auto) = &self.low_gain_automation {
                self.low_gain = auto.value_at(time).clamp(0.0, 4.0);
            }
            if let Some(auto) = &self.mid_gain_automation {
                self.mid_gain = auto.value_at(time).clamp(0.0, 4.0);
            }
            if let Some(auto) = &self.high_gain_automation {
                self.high_gain = auto.value_at(time).clamp(0.0, 4.0);
            }
        }

        // Early exit if all gains are unity (no EQ needed)
        if (self.low_gain - 1.0).abs() < 0.01
            && (self.mid_gain - 1.0).abs() < 0.01
            && (self.high_gain - 1.0).abs() < 0.01
        {
            return input;
        }

        // Simple biquad filter approximations
        let low_coeff = (2.0 * std::f32::consts::PI * self.low_freq / sample_rate).min(0.9);
        let high_coeff = (2.0 * std::f32::consts::PI * self.high_freq / sample_rate).min(0.9);

        // Low shelf (one-pole lowpass) using FMA
        let diff_low = input - self.low_state[0];
        self.low_state[0] = self.low_state[0].mul_add(1.0, low_coeff * diff_low);
        let low = self.low_state[0] * self.low_gain;

        // High shelf (one-pole highpass) using FMA
        let diff_high = input - self.high_state[0];
        self.high_state[0] = self.high_state[0].mul_add(1.0, high_coeff * diff_high);
        let high = diff_high * self.high_gain;

        // Mid (bandpass - what's left)
        let mid = (input - self.low_state[0] - diff_high) * self.mid_gain;

        low + mid + high
    }

    /// Reset the EQ state
    pub fn reset(&mut self) {
        self.low_state = [0.0; 2];
        self.mid_state = [0.0; 2];
        self.high_state = [0.0; 2];
    }
}

/// Saturation - warm analog-style clipping
#[derive(Debug, Clone)]
pub struct Saturation {
    pub drive: f32,     // Drive amount (1.0 to 10.0)
    pub character: f32, // Saturation character (0.0 = soft, 1.0 = hard)
    pub mix: f32,       // Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub priority: u8,   // Processing priority (lower = earlier in signal chain)

    // Automation (optional)
    drive_automation: Option<Automation>,
    character_automation: Option<Automation>,
    mix_automation: Option<Automation>,
}

impl Saturation {
    /// Create a new saturation effect
    ///
    /// # Arguments
    /// * `drive` - Input gain (1.0 to 10.0, typical: 2.0-4.0)
    /// * `character` - Hardness (0.0 = soft/warm, 1.0 = hard/aggressive)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub fn new(drive: f32, character: f32, mix: f32) -> Self {
        Self {
            drive: drive.clamp(1.0, 20.0),
            character: character.clamp(0.0, 1.0),
            mix: mix.clamp(0.0, 1.0),
            priority: PRIORITY_NORMAL, // Saturation in normal position
            drive_automation: None,
            character_automation: None,
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

    /// Add automation for the drive parameter
    pub fn with_drive_automation(mut self, automation: Automation) -> Self {
        self.drive_automation = Some(automation);
        self
    }

    /// Add automation for the character parameter
    pub fn with_character_automation(mut self, automation: Automation) -> Self {
        self.character_automation = Some(automation);
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
            if let Some(auto) = &self.drive_automation {
                self.drive = auto.value_at(time).clamp(1.0, 20.0);
            }
            if let Some(auto) = &self.character_automation {
                self.character = auto.value_at(time).clamp(0.0, 1.0);
            }
        }

        if self.mix < 0.0001 {
            return input;
        }

        let amplified = input * self.drive;

        // Blend between soft (tanh) and hard (cubic) saturation
        let soft = amplified.tanh();
        let hard = if amplified.abs() <= 1.0 {
            amplified.mul_add(1.5, -0.5 * amplified * amplified.abs())
        } else {
            amplified.signum()
        };

        let saturated = soft.mul_add(1.0 - self.character, hard * self.character);

        // Compensate for gain and mix using FMA
        let normalized = saturated / self.drive.sqrt();
        input.mul_add(1.0 - self.mix, normalized * self.mix)
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
    sample_rate: f32,

    // Automation (optional)
    rate_automation: Option<Automation>,
    depth_automation: Option<Automation>,
    feedback_automation: Option<Automation>,
    mix_automation: Option<Automation>,
}

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

impl Phaser {
    /// Create a new phaser effect
    ///
    /// # Arguments
    /// * `rate` - LFO speed in Hz (0.1 to 5.0, typical: 0.5)
    /// * `depth` - Modulation depth (0.0 to 1.0, typical: 0.7)
    /// * `feedback` - Feedback amount (0.0 to 0.95, typical: 0.5)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet, typical: 0.5)
    /// * `stages` - Number of stages (2, 4, 6, or 8, typical: 4)
    pub fn new(rate: f32, depth: f32, feedback: f32, mix: f32, stages: usize) -> Self {
        Self::with_sample_rate(rate, depth, feedback, mix, stages, DEFAULT_SAMPLE_RATE)
    }

    /// Create a new phaser effect with custom sample rate
    pub fn with_sample_rate(
        rate: f32,
        depth: f32,
        feedback: f32,
        mix: f32,
        stages: usize,
        sample_rate: f32,
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
            sample_rate,
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
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, time: f32, sample_count: u64) -> f32 {
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
        self.lfo_phase += self.rate / self.sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        // Mix dry and wet using FMA
        input.mul_add(1.0 - self.mix, output * self.mix)
    }

    /// Reset the phaser state
    pub fn reset(&mut self) {
        self.allpass_states = vec![AllPassFilter::new(); self.stages];
        self.lfo_phase = 0.0;
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
    sample_rate: f32,

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

    /// Create a new flanger effect with custom sample rate
    pub fn with_sample_rate(
        rate: f32,
        depth: f32,
        feedback: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        // Buffer size needs to accommodate maximum delay (in samples)
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
            sample_rate,
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
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, time: f32, sample_count: u64) -> f32 {
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
            ((delay_ms * self.sample_rate / 1000.0) as usize).min(self.buffer.len() - 1);

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
        self.lfo_phase += self.rate / self.sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        // Advance write position
        self.write_pos = (self.write_pos + 1) % self.buffer.len();

        // Mix dry and wet using FMA
        input.mul_add(1.0 - self.mix, delayed * self.mix)
    }

    /// Reset the flanger state
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }
}

/// Ring Modulator - creates metallic/robotic inharmonic tones
#[derive(Debug, Clone)]
pub struct RingModulator {
    pub carrier_freq: f32, // Carrier frequency in Hz (typical: 50 to 5000)
    pub mix: f32,          // Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub priority: u8,      // Processing priority (lower = earlier in signal chain)
    phase: f32,
    sample_rate: f32,

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
    pub fn with_sample_rate(carrier_freq: f32, mix: f32, sample_rate: f32) -> Self {
        Self {
            carrier_freq,
            mix: mix.clamp(0.0, 1.0),
            priority: PRIORITY_MODULATION, // Modulation effects in middle-late position
            phase: 0.0,
            sample_rate,
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
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, time: f32, sample_count: u64) -> f32 {
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
        let carrier = crate::wavetable::WAVETABLE.sample(self.phase);

        // Ring modulation = multiplication
        let modulated = input * carrier;

        // Advance phase
        self.phase += self.carrier_freq / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        // Mix dry and wet using FMA
        input.mul_add(1.0 - self.mix, modulated * self.mix)
    }

    /// Reset the ring modulator state
    pub fn reset(&mut self) {
        self.phase = 0.0;
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
    sample_rate: f32,

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
    /// * `sample_rate` - Audio sample rate in Hz
    pub fn with_sample_rate(rate: f32, depth: f32, sample_rate: f32) -> Self {
        Self {
            rate: rate.max(0.01),
            depth: depth.clamp(0.0, 1.0),
            priority: PRIORITY_MODULATION,
            phase: 0.0,
            sample_rate,
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
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, time: f32, sample_count: u64) -> f32 {
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
        self.phase += self.rate / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        input * modulation
    }

    /// Reset the tremolo state
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }
}

/// AutoPan - automatic stereo panning modulation
///
/// Creates rhythmic stereo movement by automatically panning the signal
/// left and right. This is applied at the stereo stage after mono effects.
#[derive(Debug, Clone)]
pub struct AutoPan {
    pub rate: f32,    // LFO rate in Hz (typically 0.1-10 Hz)
    pub depth: f32,   // Panning depth 0.0 to 1.0 (0.5 = full L-R sweep)
    pub priority: u8, // Processing priority (processed at stereo stage)
    phase: f32,       // LFO phase (0.0 to 1.0)
    sample_rate: f32,

    // Automation (optional)
    rate_automation: Option<Automation>,
    depth_automation: Option<Automation>,
}

impl AutoPan {
    /// Create a new auto-pan effect
    ///
    /// # Arguments
    /// * `rate` - LFO frequency in Hz (typically 0.1-10 Hz)
    /// * `depth` - Pan modulation depth 0.0 to 1.0 (0.5 pans full left-right)
    /// * `sample_rate` - Audio sample rate in Hz
    pub fn with_sample_rate(rate: f32, depth: f32, sample_rate: f32) -> Self {
        Self {
            rate: rate.max(0.01),
            depth: depth.clamp(0.0, 1.0),
            priority: PRIORITY_MODULATION,
            phase: 0.0,
            sample_rate,
            rate_automation: None,
            depth_automation: None,
        }
    }

    /// Create an auto-pan with default sample rate (44100 Hz)
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

    /// Get the current pan position (-1.0 to 1.0)
    ///
    /// This should be called once per sample to advance the LFO phase.
    /// The returned value is added to the track's base pan.
    ///
    /// # Arguments
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn get_pan_offset(&mut self, time: f32, sample_count: u64) -> f32 {
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
            return 0.0;
        }

        // Generate LFO (sine wave)
        let lfo = (self.phase * 2.0 * std::f32::consts::PI).sin();

        // Advance phase
        self.phase += self.rate / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        // Return pan offset (-depth to +depth)
        lfo * self.depth
    }

    /// Reset the auto-pan state
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }
}

/// Gate - noise gate / expander
///
/// Reduces the level of signals below a threshold, useful for removing
/// background noise or creating rhythmic gating effects.
#[derive(Debug, Clone)]
pub struct Gate {
    pub threshold: f32, // Threshold in dB (e.g., -40.0)
    pub ratio: f32,     // Expansion ratio (typically 10:1 to ∞:1, where ∞ = hard gate)
    pub attack: f32,    // Attack time in seconds
    pub release: f32,   // Release time in seconds
    pub priority: u8,   // Processing priority
    envelope: f32,      // Current envelope value (0.0 to 1.0)
    _sample_rate: f32,

    // Automation (optional)
    threshold_automation: Option<Automation>,
    ratio_automation: Option<Automation>,
}

impl Gate {
    /// Create a new gate effect
    ///
    /// # Arguments
    /// * `threshold` - Threshold in dB (signals below this are reduced)
    /// * `ratio` - Expansion ratio (10.0 = 10:1, f32::INFINITY = hard gate)
    /// * `attack` - Attack time in seconds (how quickly gate opens)
    /// * `release` - Release time in seconds (how quickly gate closes)
    /// * `sample_rate` - Audio sample rate in Hz
    pub fn with_sample_rate(
        threshold: f32,
        ratio: f32,
        attack: f32,
        release: f32,
        sample_rate: f32,
    ) -> Self {
        Self {
            threshold,
            ratio: ratio.max(1.0),
            attack: attack.max(0.0001),
            release: release.max(0.001),
            priority: PRIORITY_EARLY, // Gates typically go early in the chain
            envelope: 0.0,
            _sample_rate: sample_rate,
            threshold_automation: None,
            ratio_automation: None,
        }
    }

    /// Create a gate with default sample rate (44100 Hz)
    pub fn new(threshold: f32, ratio: f32, attack: f32, release: f32) -> Self {
        Self::with_sample_rate(threshold, ratio, attack, release, DEFAULT_SAMPLE_RATE)
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the threshold parameter
    pub fn with_threshold_automation(mut self, automation: Automation) -> Self {
        self.threshold_automation = Some(automation);
        self
    }

    /// Add automation for the ratio parameter
    pub fn with_ratio_automation(mut self, automation: Automation) -> Self {
        self.ratio_automation = Some(automation);
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
        // Quantized automation lookups (every 64 samples)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.threshold_automation {
                self.threshold = auto.value_at(time);
            }
            if let Some(auto) = &self.ratio_automation {
                self.ratio = auto.value_at(time).max(1.0);
            }
        }

        // Convert input to dB
        let input_db = if input.abs() > 0.0001 {
            20.0 * input.abs().log10()
        } else {
            -100.0 // Very quiet = -100 dB
        };

        // Determine target envelope based on threshold
        let target_envelope = if input_db > self.threshold {
            1.0 // Above threshold: gate open
        } else {
            // Below threshold: apply expansion/gating
            let db_below = self.threshold - input_db;
            let expansion = db_below * (self.ratio - 1.0) / self.ratio;
            10.0_f32.powf(-expansion / 20.0) // Convert back to linear
        };

        // Smooth envelope with attack/release
        let coeff = if target_envelope > self.envelope {
            // Attack (gate opening)
            (-1.0 / (self.attack * sample_rate)).exp()
        } else {
            // Release (gate closing)
            (-1.0 / (self.release * sample_rate)).exp()
        };

        self.envelope = target_envelope + coeff * (self.envelope - target_envelope);

        // Apply gating
        input * self.envelope
    }

    /// Reset the gate state
    pub fn reset(&mut self) {
        self.envelope = 0.0;
    }
}

/// Limiter - brick-wall peak limiter
///
/// Prevents signal from exceeding a threshold, acting as a safety net
/// against clipping. Typically used as the final stage in the signal chain.
#[derive(Debug, Clone)]
pub struct Limiter {
    pub threshold: f32,  // Threshold in dB (e.g., -0.1 dB)
    pub release: f32,    // Release time in seconds
    pub priority: u8,    // Processing priority
    gain_reduction: f32, // Current gain reduction (0.0 to 1.0)
    _sample_rate: f32,

    // Automation (optional)
    threshold_automation: Option<Automation>,
}

impl Limiter {
    /// Create a new limiter effect
    ///
    /// # Arguments
    /// * `threshold` - Threshold in dB (signals above this are limited)
    /// * `release` - Release time in seconds (how quickly limiter recovers)
    /// * `sample_rate` - Audio sample rate in Hz
    pub fn with_sample_rate(threshold: f32, release: f32, sample_rate: f32) -> Self {
        Self {
            threshold,
            release: release.max(0.001),
            priority: PRIORITY_LAST, // Limiters go last to catch peaks
            gain_reduction: 1.0,
            _sample_rate: sample_rate,
            threshold_automation: None,
        }
    }

    /// Create a limiter with default sample rate (44100 Hz)
    pub fn new(threshold: f32, release: f32) -> Self {
        Self::with_sample_rate(threshold, release, DEFAULT_SAMPLE_RATE)
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the threshold parameter
    pub fn with_threshold_automation(mut self, automation: Automation) -> Self {
        self.threshold_automation = Some(automation);
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
        // Quantized automation lookups (every 64 samples)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.threshold_automation {
                self.threshold = auto.value_at(time);
            }
        }

        // Convert threshold from dB to linear
        let threshold_linear = 10.0_f32.powf(self.threshold / 20.0);

        // Detect peak
        let input_abs = input.abs();

        // Calculate required gain reduction
        let target_gain = if input_abs > threshold_linear {
            threshold_linear / input_abs
        } else {
            1.0
        };

        // Apply gain reduction with instant attack and release envelope
        // Instant attack (0 ms) for true peak limiting
        if target_gain < self.gain_reduction {
            self.gain_reduction = target_gain;
        } else {
            // Smooth release
            let release_coeff = (-1.0 / (self.release * sample_rate)).exp();
            self.gain_reduction = target_gain + release_coeff * (self.gain_reduction - target_gain);
        }

        // Apply limiting
        input * self.gain_reduction
    }

    /// Get the current gain reduction in dB
    ///
    /// Useful for metering how much limiting is occurring
    pub fn get_gain_reduction_db(&self) -> f32 {
        if self.gain_reduction > 0.0 {
            20.0 * self.gain_reduction.log10()
        } else {
            -100.0
        }
    }

    /// Reset the limiter state
    pub fn reset(&mut self) {
        self.gain_reduction = 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay() {
        let mut delay = Delay::new(0.01, 0.5, 0.5);
        let output = delay.process(1.0, 0.0, 0);
        assert!(output >= 0.0 && output <= 1.0);
    }

    #[test]
    fn test_reverb() {
        let mut reverb = Reverb::new(0.5, 0.5, 0.3);
        let output = reverb.process(1.0, 0.0, 0);
        assert!(output.is_finite());
    }

    #[test]
    fn test_distortion() {
        let mut dist = Distortion::new(5.0, 1.0);
        let output = dist.process(0.5, 0.0, 0);
        assert!(output >= -1.0 && output <= 1.0);
    }
}
