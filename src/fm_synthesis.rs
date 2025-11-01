use crate::wavetable::WAVETABLE;

/// FM (Frequency Modulation) Synthesis Parameters
///
/// FM synthesis works by using one oscillator (the modulator) to modulate
/// the frequency of another oscillator (the carrier). This creates complex,
/// harmonically rich timbres that are impossible with basic subtractive synthesis.
///
/// Famous for: DX7 sounds, electric pianos, bells, brass, metallic tones
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FMParams {
    /// Ratio of modulator frequency to carrier frequency
    /// Common ratios: 1.0 (harmonic), 2.0 (octave up), 0.5 (octave down), 3.5 (inharmonic)
    pub mod_ratio: f32,

    /// Modulation index - controls the brightness/complexity of the sound
    /// Higher values = more harmonics = brighter/harsher
    /// Typical range: 0.0 to 10.0
    pub mod_index: f32,

    /// Envelope for modulation index (controls how brightness changes over time)
    /// 0.0 to 1.0, multiplied by mod_index
    pub index_envelope_attack: f32,
    pub index_envelope_decay: f32,
    pub index_envelope_sustain: f32,
    pub index_envelope_release: f32,

    /// Modulator envelope depth (0.0 = no envelope, 1.0 = full envelope effect)
    pub index_env_amount: f32,
}

impl FMParams {
    /// Create new FM synthesis parameters
    ///
    /// # Arguments
    /// * `mod_ratio` - Modulator to carrier frequency ratio
    /// * `mod_index` - Modulation index (brightness)
    pub fn new(mod_ratio: f32, mod_index: f32) -> Self {
        Self {
            mod_ratio: mod_ratio.max(0.01),
            mod_index: mod_index.max(0.0),
            index_envelope_attack: 0.01,
            index_envelope_decay: 0.1,
            index_envelope_sustain: 0.7,
            index_envelope_release: 0.2,
            index_env_amount: 0.0,
        }
    }

    /// Create FM params with modulation index envelope
    pub fn with_index_envelope(
        mod_ratio: f32,
        mod_index: f32,
        attack: f32,
        decay: f32,
        sustain: f32,
        release: f32,
        amount: f32,
    ) -> Self {
        Self {
            mod_ratio: mod_ratio.max(0.01),
            mod_index: mod_index.max(0.0),
            index_envelope_attack: attack.max(0.001),
            index_envelope_decay: decay.max(0.001),
            index_envelope_sustain: sustain.clamp(0.0, 1.0),
            index_envelope_release: release.max(0.001),
            index_env_amount: amount.clamp(0.0, 1.0),
        }
    }

    /// Classic electric piano sound (DX7-style)
    ///
    /// Modulator ratio slightly detuned from harmonic for warmth
    pub fn electric_piano() -> Self {
        Self::with_index_envelope(1.0, 2.5, 0.001, 0.8, 0.2, 0.5, 0.9)
    }

    /// Bright bell sound
    ///
    /// Inharmonic ratio creates bell-like metallic timbre
    pub fn bell() -> Self {
        Self::with_index_envelope(3.5, 8.0, 0.001, 1.2, 0.1, 0.8, 0.95)
    }

    /// Brass-like sound
    ///
    /// High modulation index with envelope for expressive brass
    pub fn brass() -> Self {
        Self::with_index_envelope(1.0, 5.0, 0.05, 0.2, 0.8, 0.3, 0.8)
    }

    /// Bass sound with harmonics
    ///
    /// Low modulation for fundamental-heavy bass with subtle harmonics
    pub fn bass() -> Self {
        Self::with_index_envelope(1.0, 1.2, 0.001, 0.15, 0.6, 0.2, 0.7)
    }

    /// Metallic pad (shimmer effect)
    ///
    /// Irrational ratio for slowly evolving inharmonic texture
    pub fn metallic_pad() -> Self {
        Self::with_index_envelope(2.414, 4.0, 0.8, 0.5, 0.7, 1.0, 0.6)
    }

    /// Growling bass
    ///
    /// Octave-down modulator with high index for aggressive bass
    pub fn growl() -> Self {
        Self::with_index_envelope(0.5, 6.0, 0.001, 0.3, 0.5, 0.2, 0.85)
    }

    /// Disable FM (bypass)
    pub fn none() -> Self {
        Self::new(1.0, 0.0)
    }

    /// Calculate the modulation index at a given time using the index envelope
    ///
    /// # Arguments
    /// * `time_in_note` - Time since note started (seconds)
    /// * `note_duration` - Total note duration (seconds)
    pub fn index_at(&self, time_in_note: f32, note_duration: f32) -> f32 {
        if self.index_env_amount == 0.0 {
            return self.mod_index;
        }

        let env_value = self.envelope_value_at(time_in_note, note_duration);

        // Interpolate between base index and zero based on envelope
        // When envelope is 1.0, use full mod_index
        // When envelope is 0.0, reduce mod_index
        self.mod_index * (1.0 - self.index_env_amount + env_value * self.index_env_amount)
    }

    /// Get envelope value (0.0 to 1.0) at a given time
    fn envelope_value_at(&self, time: f32, note_duration: f32) -> f32 {
        if time < 0.0 {
            return 0.0;
        }

        // Attack phase
        if time < self.index_envelope_attack {
            return time / self.index_envelope_attack;
        }

        // Decay phase
        let decay_start = self.index_envelope_attack;
        if time < decay_start + self.index_envelope_decay {
            let decay_progress = (time - decay_start) / self.index_envelope_decay;
            return 1.0 - (1.0 - self.index_envelope_sustain) * decay_progress;
        }

        // Sustain phase
        if time < note_duration {
            return self.index_envelope_sustain;
        }

        // Release phase
        let release_progress = (time - note_duration) / self.index_envelope_release;
        if release_progress >= 1.0 {
            return 0.0;
        }

        self.index_envelope_sustain * (1.0 - release_progress)
    }

    /// Generate an FM synthesis sample
    ///
    /// # Arguments
    /// * `carrier_freq` - Carrier oscillator frequency (Hz)
    /// * `time_in_note` - Time within the note (seconds)
    /// * `note_duration` - Total note duration (seconds)
    ///
    /// # Returns
    /// Sample value between -1.0 and 1.0
    pub fn sample(&self, carrier_freq: f32, time_in_note: f32, note_duration: f32) -> f32 {
        if self.mod_index == 0.0 {
            // Bypass - just return carrier sine wave (using fast wavetable)
            let phase = time_in_note * carrier_freq;
            return WAVETABLE.sine(phase);
        }

        // Calculate modulator frequency
        let modulator_freq = carrier_freq * self.mod_ratio;

        // Get current modulation index (with envelope)
        let current_index = self.index_at(time_in_note, note_duration);

        // Generate modulator signal (using fast wavetable)
        let mod_phase = time_in_note * modulator_freq;
        let modulator = WAVETABLE.sine(mod_phase);

        // Modulate carrier frequency
        let frequency_offset = modulator * current_index * modulator_freq;
        let modulated_freq = carrier_freq + frequency_offset;

        // Generate carrier with modulated frequency (using fast wavetable)
        let carrier_phase = time_in_note * modulated_freq;
        WAVETABLE.sine(carrier_phase)
    }
}

impl Default for FMParams {
    fn default() -> Self {
        Self::none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fm_params_creation() {
        let fm = FMParams::new(2.0, 3.0);
        assert_eq!(fm.mod_ratio, 2.0);
        assert_eq!(fm.mod_index, 3.0);
    }

    #[test]
    fn test_fm_bypass() {
        let fm = FMParams::none();
        assert_eq!(fm.mod_index, 0.0);

        // Should produce a sine wave
        let sample = fm.sample(440.0, 0.0, 1.0);
        assert!(sample.abs() < 0.1); // At t=0, sin(0) ≈ 0
    }

    #[test]
    fn test_fm_synthesis() {
        let fm = FMParams::new(1.0, 5.0);

        // Sample should be between -1 and 1
        for i in 0..100 {
            let t = i as f32 / 100.0;
            let sample = fm.sample(440.0, t, 1.0);
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "Sample {} out of range at t={}",
                sample,
                t
            );
        }
    }

    #[test]
    fn test_index_envelope() {
        let fm = FMParams::with_index_envelope(1.0, 10.0, 0.1, 0.1, 0.5, 0.2, 1.0);

        // At t=0, should start at low index
        let start_index = fm.index_at(0.0, 1.0);
        assert!(start_index < 1.0, "Should start with low index");

        // At peak of attack, should be at full index
        let peak_index = fm.index_at(0.1, 1.0);
        assert!(peak_index > 8.0, "Should reach high index at attack peak");

        // During sustain, should be at sustain level
        let sustain_index = fm.index_at(0.5, 1.0);
        assert!(
            sustain_index > start_index && sustain_index < peak_index,
            "Sustain should be between start and peak"
        );
    }

    #[test]
    fn test_presets() {
        let _ep = FMParams::electric_piano();
        let _bell = FMParams::bell();
        let _brass = FMParams::brass();
        let _bass = FMParams::bass();
        let _pad = FMParams::metallic_pad();
        let _growl = FMParams::growl();
    }
}
