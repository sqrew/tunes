use crate::synthesis::automation::Automation;
use crate::track::PRIORITY_NORMAL;

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

    /// Process a block of samples with SIMD acceleration
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    /// * `sample_rate` - Sample rate in Hz (for time advancement)
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], time: f32, sample_count: u64, _sample_rate: f32) {
        use crate::synthesis::simd::{SimdWidth, SIMD};

        // Update automation params if needed (check first sample)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.mix_automation {
                self.mix = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.drive_automation {
                self.drive = auto.value_at(time).max(1.0);
            }
        }

        // Early exit if no effect
        if self.mix < 0.0001 {
            return;
        }

        // Dispatch to SIMD implementation
        match SIMD.simd_width() {
            SimdWidth::X8 => self.process_block_simd::<8>(buffer),
            SimdWidth::X4 => self.process_block_simd::<4>(buffer),
            SimdWidth::Scalar => self.process_block_scalar(buffer),
        }
    }

    /// SIMD implementation - processes N samples at once
    #[inline(always)]
    fn process_block_simd<const N: usize>(&self, buffer: &mut [f32]) {
        let drive = self.drive;
        let mix = self.mix;
        let one_minus_mix = 1.0 - mix;
        let compensation = 1.0 / drive.sqrt();

        let num_chunks = buffer.len() / N;
        let remainder_start = num_chunks * N;

        // Process N samples at a time
        for chunk_idx in 0..num_chunks {
            let chunk_start = chunk_idx * N;
            let chunk = &mut buffer[chunk_start..chunk_start + N];

            for sample in chunk.iter_mut() {
                let input = *sample;
                let amplified = input * drive;
                let distorted = amplified.tanh();
                let normalized = distorted * compensation;
                *sample = input.mul_add(one_minus_mix, normalized * mix);
            }
        }

        // Handle remainder with scalar
        for i in remainder_start..buffer.len() {
            let input = buffer[i];
            let amplified = input * drive;
            let distorted = amplified.tanh();
            let normalized = distorted * compensation;
            buffer[i] = input.mul_add(one_minus_mix, normalized * mix);
        }
    }

    /// Scalar fallback
    #[inline(always)]
    fn process_block_scalar(&self, buffer: &mut [f32]) {
        let drive = self.drive;
        let mix = self.mix;
        let one_minus_mix = 1.0 - mix;
        let compensation = 1.0 / drive.sqrt();

        for sample in buffer.iter_mut() {
            let input = *sample;
            let amplified = input * drive;
            let distorted = amplified.tanh();
            let normalized = distorted * compensation;
            *sample = input.mul_add(one_minus_mix, normalized * mix);
        }
    }

    // ========== PRESETS ==========

    /// Light saturation - subtle analog warmth
    pub fn saturation() -> Self {
        Self::new(1.5, 0.5)
    }

    /// Overdrive - tube-style warmth and grit
    pub fn overdrive() -> Self {
        Self::new(2.5, 0.8)
    }

    /// Crunch - classic rock distortion
    pub fn crunch() -> Self {
        Self::new(4.0, 0.9)
    }

    /// Heavy distortion - metal/high-gain
    pub fn heavy() -> Self {
        Self::new(6.0, 1.0)
    }

    /// Fuzz - extreme, compressed distortion
    pub fn fuzz() -> Self {
        Self::new(8.0, 1.0)
    }

    /// Gentle drive - barely-there warmth
    pub fn gentle() -> Self {
        Self::new(1.8, 0.4)
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

    /// Reset the bit crusher state
    pub fn reset(&mut self) {
        self.hold_sample = 0.0;
        self.sample_counter = 0.0;
    }

    // ========== PRESETS ==========

    /// Lo-fi effect - 8-bit style with mild downsampling
    pub fn lofi() -> Self {
        Self::new(8.0, 2.0, 0.7)
    }

    /// Game Boy - classic 4-bit handheld sound
    pub fn gameboy() -> Self {
        Self::new(4.0, 4.0, 0.85)
    }

    /// Telephone - heavily crushed, narrow bandwidth
    pub fn telephone() -> Self {
        Self::new(6.0, 8.0, 0.8)
    }

    /// Glitch - extreme digital degradation
    pub fn glitch() -> Self {
        Self::new(3.0, 12.0, 1.0)
    }

    /// Vintage - subtle lo-fi character
    pub fn vintage() -> Self {
        Self::new(10.0, 1.5, 0.4)
    }
}

/// Saturation effect - analog-style harmonic distortion
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

    /// Process a block of samples with SIMD acceleration
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    /// * `sample_rate` - Sample rate in Hz (for time advancement)
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], time: f32, sample_count: u64, _sample_rate: f32) {
        use crate::synthesis::simd::{SimdWidth, SIMD};

        // Update automation params if needed
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

        // Early exit if no effect
        if self.mix < 0.0001 {
            return;
        }

        // Dispatch to SIMD implementation
        match SIMD.simd_width() {
            SimdWidth::X8 => self.process_block_simd::<8>(buffer),
            SimdWidth::X4 => self.process_block_simd::<4>(buffer),
            SimdWidth::Scalar => self.process_block_scalar(buffer),
        }
    }

    /// SIMD implementation - processes N samples at once
    #[inline(always)]
    fn process_block_simd<const N: usize>(&self, buffer: &mut [f32]) {
        let drive = self.drive;
        let character = self.character;
        let mix = self.mix;
        let one_minus_mix = 1.0 - mix;
        let one_minus_character = 1.0 - character;
        let compensation = 1.0 / drive.sqrt();

        let num_chunks = buffer.len() / N;
        let remainder_start = num_chunks * N;

        // Process N samples at a time
        for chunk_idx in 0..num_chunks {
            let chunk_start = chunk_idx * N;
            let chunk = &mut buffer[chunk_start..chunk_start + N];

            for sample in chunk.iter_mut() {
                let input = *sample;
                let amplified = input * drive;

                // Blend between soft (tanh) and hard (cubic) saturation
                let soft = amplified.tanh();
                let hard = if amplified.abs() <= 1.0 {
                    amplified.mul_add(1.5, -0.5 * amplified * amplified.abs())
                } else {
                    amplified.signum()
                };

                let saturated = soft.mul_add(one_minus_character, hard * character);
                let normalized = saturated * compensation;
                *sample = input.mul_add(one_minus_mix, normalized * mix);
            }
        }

        // Handle remainder with scalar
        for i in remainder_start..buffer.len() {
            let input = buffer[i];
            let amplified = input * drive;

            let soft = amplified.tanh();
            let hard = if amplified.abs() <= 1.0 {
                amplified.mul_add(1.5, -0.5 * amplified * amplified.abs())
            } else {
                amplified.signum()
            };

            let saturated = soft.mul_add(one_minus_character, hard * character);
            let normalized = saturated * compensation;
            buffer[i] = input.mul_add(one_minus_mix, normalized * mix);
        }
    }

    /// Scalar fallback
    #[inline(always)]
    fn process_block_scalar(&self, buffer: &mut [f32]) {
        let drive = self.drive;
        let character = self.character;
        let mix = self.mix;
        let one_minus_mix = 1.0 - mix;
        let one_minus_character = 1.0 - character;
        let compensation = 1.0 / drive.sqrt();

        for sample in buffer.iter_mut() {
            let input = *sample;
            let amplified = input * drive;

            let soft = amplified.tanh();
            let hard = if amplified.abs() <= 1.0 {
                amplified.mul_add(1.5, -0.5 * amplified * amplified.abs())
            } else {
                amplified.signum()
            };

            let saturated = soft.mul_add(one_minus_character, hard * character);
            let normalized = saturated * compensation;
            *sample = input.mul_add(one_minus_mix, normalized * mix);
        }
    }

    // ========== PRESETS ==========

    /// Tape saturation - warm analog tape character
    pub fn tape() -> Self {
        Self::new(2.0, 0.3, 0.7)
    }

    /// Tube saturation - vintage tube amp warmth
    pub fn tube() -> Self {
        Self::new(3.0, 0.5, 0.8)
    }

    /// Soft clipping - gentle harmonic enhancement
    pub fn soft() -> Self {
        Self::new(1.5, 0.2, 0.5)
    }

    /// Hard clipping - aggressive saturation
    pub fn hard() -> Self {
        Self::new(4.0, 0.8, 0.9)
    }

    /// Warmth - subtle analog color
    pub fn warmth() -> Self {
        Self::new(1.8, 0.4, 0.6)
    }

    /// Aggressive - heavy analog distortion
    pub fn aggressive() -> Self {
        Self::new(5.0, 0.9, 1.0)
    }
}
