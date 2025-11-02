/// Standard audio sample rate
const DEFAULT_SAMPLE_RATE: f32 = 44100.0;

/// Delay effect with feedback
#[derive(Debug, Clone)]
pub struct Delay {
    pub delay_time: f32,  // Delay time in seconds
    pub feedback: f32,    // Feedback amount (0.0 to 0.99)
    pub mix: f32,         // Wet/dry mix (0.0 = dry, 1.0 = wet)
    buffer: Vec<f32>,
    write_pos: usize,
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
            buffer: vec![0.0; buffer_size.max(1)],
            write_pos: 0,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
        if self.buffer.is_empty() || self.mix < 0.0001 {
            return input;
        }

        // Read from delay buffer
        let delayed = self.buffer[self.write_pos];

        // Write input + feedback to buffer
        self.buffer[self.write_pos] = input + delayed * self.feedback;

        // Advance write position
        self.write_pos = (self.write_pos + 1) % self.buffer.len();

        // Mix dry and wet signals
        input * (1.0 - self.mix) + delayed * self.mix
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
    pub room_size: f32,   // Room size (0.0 to 1.0)
    pub damping: f32,     // High frequency damping (0.0 to 1.0)
    pub mix: f32,         // Wet/dry mix (0.0 = dry, 1.0 = wet)
    comb_buffers: Vec<Vec<f32>>,
    comb_positions: Vec<usize>,
    filter_state: Vec<f32>,
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
            comb_positions: vec![0; comb_buffers.len()],
            filter_state: vec![0.0; comb_buffers.len()],
            comb_buffers,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
        if self.mix < 0.0001 {
            return input;
        }

        let mut output = 0.0;
        let feedback = 0.5 + self.room_size * 0.48;

        // Process through all comb filters
        for i in 0..self.comb_buffers.len() {
            let buffer = &mut self.comb_buffers[i];
            let pos = self.comb_positions[i];

            // Read from buffer
            let delayed = buffer[pos];

            // Apply damping filter (simple lowpass)
            self.filter_state[i] = delayed * (1.0 - self.damping) + self.filter_state[i] * self.damping;

            // Write to buffer with feedback
            buffer[pos] = input + self.filter_state[i] * feedback;

            // Advance position
            self.comb_positions[i] = (pos + 1) % buffer.len();

            // Accumulate output
            output += delayed;
        }

        // Average and mix
        output /= self.comb_buffers.len() as f32;
        input * (1.0 - self.mix) + output * self.mix
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
#[derive(Debug, Clone, Copy)]
pub struct Distortion {
    pub drive: f32,       // Drive amount (1.0 = no distortion, higher = more)
    pub mix: f32,         // Wet/dry mix (0.0 = dry, 1.0 = wet)
}

impl Distortion {
    /// Create a new distortion effect
    pub fn new(drive: f32, mix: f32) -> Self {
        Self {
            drive: drive.max(1.0),
            mix: mix.clamp(0.0, 1.0),
        }
    }

    /// Process a single sample using soft clipping
    pub fn process(&self, input: f32) -> f32 {
        if self.mix < 0.0001 {
            return input;
        }

        let amplified = input * self.drive;

        // Soft clipping using tanh
        let distorted = amplified.tanh();

        // Compensate for gain increase
        let normalized = distorted / self.drive.sqrt();

        // Mix dry and wet
        input * (1.0 - self.mix) + normalized * self.mix
    }
}

/// Bit crusher - lo-fi digital degradation effect
#[derive(Debug, Clone)]
pub struct BitCrusher {
    pub bit_depth: f32,        // Bit depth (1.0 to 16.0, lower = more crushing)
    pub sample_rate_reduction: f32, // Sample rate divisor (1.0 = no reduction, higher = more lo-fi)
    pub mix: f32,              // Wet/dry mix (0.0 = dry, 1.0 = wet)
    hold_sample: f32,
    sample_counter: f32,
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
            hold_sample: 0.0,
            sample_counter: 0.0,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
        // Sample rate reduction (sample & hold)
        self.sample_counter += 1.0;
        if self.sample_counter >= self.sample_rate_reduction {
            self.hold_sample = input.clamp(-2.0, 2.0);
            self.sample_counter = 0.0;
        }

        // Bit depth reduction (quantization)
        let levels = 2.0_f32.powf(self.bit_depth);
        let quantized = (self.hold_sample * levels).round() / levels;

        // Mix dry and wet, clamp output
        let output = input * (1.0 - self.mix) + quantized * self.mix;
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
    pub threshold: f32,    // Threshold in amplitude 0.0-1.0 (NOT dB! 0.3 ≈ -10dB, 0.5 ≈ -6dB)
    pub ratio: f32,        // Compression ratio (1.0 = no compression, 10.0 = heavy)
    pub attack: f32,       // Attack time in seconds
    pub release: f32,      // Release time in seconds
    pub makeup_gain: f32,  // Makeup gain to compensate for volume reduction
    envelope: f32,
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
            envelope: 0.0,
        }
    }

    /// Process a single sample at given sample rate
    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        let input_level = input.abs();

        // Envelope follower
        let attack_coeff = (-1.0 / (self.attack * sample_rate)).exp();
        let release_coeff = (-1.0 / (self.release * sample_rate)).exp();

        if input_level > self.envelope {
            self.envelope = attack_coeff * self.envelope + (1.0 - attack_coeff) * input_level;
        } else {
            self.envelope = release_coeff * self.envelope + (1.0 - release_coeff) * input_level;
        }

        // Clamp envelope to prevent runaway values
        self.envelope = self.envelope.clamp(0.0, 2.0);

        // Calculate gain reduction
        let mut gain = 1.0;
        if self.envelope > self.threshold {
            let over_threshold = self.envelope / self.threshold.max(0.001); // Prevent division by zero
            let compressed = over_threshold.powf(1.0 / self.ratio);
            gain = (compressed * self.threshold / self.envelope).clamp(0.0, 1.0);
        }

        // Apply compression and makeup gain, clamp output to prevent clipping
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
    pub rate: f32,         // LFO rate in Hz (typical: 0.5 to 3.0)
    pub depth: f32,        // Modulation depth in milliseconds (typical: 2.0 to 10.0)
    pub mix: f32,          // Wet/dry mix (0.0 = dry, 1.0 = wet)
    buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
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
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            lfo_phase: 0.0,
        }
    }

    /// Process a single sample at given sample rate
    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        if self.mix < 0.0001 {
            return input;
        }

        // Write input to buffer
        self.buffer[self.write_pos] = input;

        // Calculate modulated delay time using sine LFO
        let lfo = (self.lfo_phase * 2.0 * std::f32::consts::PI).sin();
        let delay_ms = self.depth * (1.0 + lfo) * 0.5;
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

        // Mix dry and wet
        input * (1.0 - self.mix) + delayed * self.mix
    }

    /// Reset the chorus state
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }
}

/// Parametric EQ - 3-band equalizer
#[derive(Debug, Clone, Copy)]
pub struct EQ {
    pub low_gain: f32,     // Low frequency gain (0.0 to 2.0, 1.0 = unity)
    pub mid_gain: f32,     // Mid frequency gain (0.0 to 2.0, 1.0 = unity)
    pub high_gain: f32,    // High frequency gain (0.0 to 2.0, 1.0 = unity)
    pub low_freq: f32,     // Low band center frequency (Hz)
    pub high_freq: f32,    // High band center frequency (Hz)
    // State variables for filters
    low_state: [f32; 2],
    mid_state: [f32; 2],
    high_state: [f32; 2],
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
    pub fn new(low_gain: f32, mid_gain: f32, high_gain: f32, low_freq: f32, high_freq: f32) -> Self {
        Self {
            low_gain: low_gain.clamp(0.0, 4.0),
            mid_gain: mid_gain.clamp(0.0, 4.0),
            high_gain: high_gain.clamp(0.0, 4.0),
            low_freq: low_freq.clamp(20.0, 20000.0),
            high_freq: high_freq.clamp(20.0, 20000.0),
            low_state: [0.0; 2],
            mid_state: [0.0; 2],
            high_state: [0.0; 2],
        }
    }

    /// Process a single sample at given sample rate
    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        // Early exit if all gains are unity (no EQ needed)
        if (self.low_gain - 1.0).abs() < 0.01
            && (self.mid_gain - 1.0).abs() < 0.01
            && (self.high_gain - 1.0).abs() < 0.01 {
            return input;
        }

        // Simple biquad filter approximations
        let low_coeff = (2.0 * std::f32::consts::PI * self.low_freq / sample_rate).min(0.9);
        let high_coeff = (2.0 * std::f32::consts::PI * self.high_freq / sample_rate).min(0.9);

        // Low shelf (one-pole lowpass)
        self.low_state[0] = self.low_state[0] + low_coeff * (input - self.low_state[0]);
        let low = self.low_state[0] * self.low_gain;

        // High shelf (one-pole highpass)
        self.high_state[0] = self.high_state[0] + high_coeff * (input - self.high_state[0]);
        let high = (input - self.high_state[0]) * self.high_gain;

        // Mid (bandpass - what's left)
        let mid = (input - self.low_state[0] - (input - self.high_state[0])) * self.mid_gain;

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
#[derive(Debug, Clone, Copy)]
pub struct Saturation {
    pub drive: f32,        // Drive amount (1.0 to 10.0)
    pub character: f32,    // Saturation character (0.0 = soft, 1.0 = hard)
    pub mix: f32,          // Wet/dry mix (0.0 = dry, 1.0 = wet)
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
        }
    }

    /// Process a single sample
    pub fn process(&self, input: f32) -> f32 {
        if self.mix < 0.0001 {
            return input;
        }

        let amplified = input * self.drive;

        // Blend between soft (tanh) and hard (cubic) saturation
        let soft = amplified.tanh();
        let hard = if amplified.abs() <= 1.0 {
            amplified * (1.5 - 0.5 * amplified.abs())
        } else {
            amplified.signum()
        };

        let saturated = soft * (1.0 - self.character) + hard * self.character;

        // Compensate for gain and mix
        let normalized = saturated / self.drive.sqrt();
        input * (1.0 - self.mix) + normalized * self.mix
    }
}
/// Phaser - creates sweeping notches in the frequency spectrum
#[derive(Debug, Clone)]
pub struct Phaser {
    pub rate: f32,      // LFO rate in Hz (typical: 0.1 to 5.0)
    pub depth: f32,     // Modulation depth (0.0 to 1.0)
    pub feedback: f32,  // Feedback amount (0.0 to 0.95)
    pub mix: f32,       // Wet/dry mix (0.0 = dry, 1.0 = wet)
    pub stages: usize,  // Number of all-pass filter stages (2, 4, 6, or 8)
    allpass_states: Vec<AllPassFilter>,
    lfo_phase: f32,
    sample_rate: f32,
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
            allpass_states: vec![AllPassFilter::new(); stages],
            lfo_phase: 0.0,
            sample_rate,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
        if self.mix < 0.0001 || self.depth < 0.0001 {
            return input;
        }

        // Generate LFO
        let lfo = (self.lfo_phase * 2.0 * std::f32::consts::PI).sin();

        // Map LFO to delay range (affects frequency of notches)
        let min_delay = 0.5;
        let max_delay = 5.0;
        let delay = min_delay + (max_delay - min_delay) * (0.5 + 0.5 * lfo * self.depth);

        // Process through all-pass filter stages
        let mut output = input;
        for filter in &mut self.allpass_states {
            output = filter.process(output, delay);
        }

        // Apply feedback
        let feedback_sample = output * self.feedback;
        output = input + feedback_sample;

        // Advance LFO phase
        self.lfo_phase += self.rate / self.sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        // Mix dry and wet
        input * (1.0 - self.mix) + output * self.mix
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
    pub rate: f32,      // LFO rate in Hz (typical: 0.1 to 2.0)
    pub depth: f32,     // Modulation depth in milliseconds (typical: 1.0 to 5.0)
    pub feedback: f32,  // Feedback amount (0.0 to 0.95)
    pub mix: f32,       // Wet/dry mix (0.0 = dry, 1.0 = wet)
    buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
    sample_rate: f32,
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
    pub fn with_sample_rate(rate: f32, depth: f32, feedback: f32, mix: f32, sample_rate: f32) -> Self {
        // Buffer size needs to accommodate maximum delay (in samples)
        let max_delay_samples = ((depth * 2.0) * sample_rate / 1000.0) as usize;
        let buffer_size = max_delay_samples.max(1);

        Self {
            rate,
            depth,
            feedback: feedback.clamp(0.0, 0.95),
            mix: mix.clamp(0.0, 1.0),
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            lfo_phase: 0.0,
            sample_rate,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
        // Safety check: if buffer is empty, just pass through
        if self.buffer.is_empty() || self.mix < 0.0001 {
            return input;
        }

        // Calculate modulated delay time using sine LFO
        let lfo = (self.lfo_phase * 2.0 * std::f32::consts::PI).sin();
        let delay_ms = self.depth * (1.0 + lfo) * 0.5; // 0 to depth milliseconds
        let delay_samples = ((delay_ms * self.sample_rate / 1000.0) as usize).min(self.buffer.len() - 1);

        // Read from delayed position
        let read_pos = if self.write_pos >= delay_samples {
            self.write_pos - delay_samples
        } else {
            self.buffer.len() - (delay_samples - self.write_pos)
        };
        let delayed = self.buffer[read_pos];

        // Write to buffer with feedback
        self.buffer[self.write_pos] = input + delayed * self.feedback;

        // Advance LFO phase
        self.lfo_phase += self.rate / self.sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        // Advance write position
        self.write_pos = (self.write_pos + 1) % self.buffer.len();

        // Mix dry and wet
        input * (1.0 - self.mix) + delayed * self.mix
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
    pub carrier_freq: f32,  // Carrier frequency in Hz (typical: 50 to 5000)
    pub mix: f32,           // Wet/dry mix (0.0 = dry, 1.0 = wet)
    phase: f32,
    sample_rate: f32,
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
            phase: 0.0,
            sample_rate,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
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

        // Mix dry and wet
        input * (1.0 - self.mix) + modulated * self.mix
    }

    /// Reset the ring modulator state
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay() {
        let mut delay = Delay::new(0.01, 0.5, 0.5);
        let output = delay.process(1.0);
        assert!(output >= 0.0 && output <= 1.0);
    }

    #[test]
    fn test_reverb() {
        let mut reverb = Reverb::new(0.5, 0.5, 0.3);
        let output = reverb.process(1.0);
        assert!(output.is_finite());
    }

    #[test]
    fn test_distortion() {
        let dist = Distortion::new(5.0, 1.0);
        let output = dist.process(0.5);
        assert!(output >= -1.0 && output <= 1.0);
    }
}
