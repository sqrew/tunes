/// ADSR (Attack, Decay, Sustain, Release) envelope for shaping sound amplitude over time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Envelope {
    pub attack: f32,   // Time to reach peak amplitude (seconds)
    pub decay: f32,    // Time to decay from peak to sustain level (seconds)
    pub sustain: f32,  // Sustain level (0.0 to 1.0)
    pub release: f32,  // Time to fade from sustain to silence after note ends (seconds)
}

impl Envelope {
    /// Create a new ADSR envelope
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack: attack.max(0.001), // Minimum to avoid division by zero
            decay: decay.max(0.001),
            sustain: sustain.clamp(0.0, 1.0),
            release: release.max(0.001),
        }
    }

    /// Piano-like envelope (quick attack, medium decay, no sustain, medium release)
    pub fn piano() -> Self {
        Self::new(0.005, 0.2, 0.3, 0.3)
    }

    /// Organ-like envelope (no attack/decay, full sustain, quick release)
    pub fn organ() -> Self {
        Self::new(0.001, 0.001, 1.0, 0.05)
    }

    /// Pad-like envelope (slow attack, slow release)
    pub fn pad() -> Self {
        Self::new(0.5, 0.3, 0.8, 0.8)
    }

    /// Pluck-like envelope (instant attack, quick decay, low sustain, short release)
    pub fn pluck() -> Self {
        Self::new(0.001, 0.05, 0.2, 0.1)
    }

    /// Calculate the amplitude multiplier at a given time within the note
    ///
    /// # Arguments
    /// * `time` - Time since note started (seconds)
    /// * `note_duration` - Total duration of the note (seconds)
    /// * `released` - Whether the note has been released (for calculating release phase)
    pub fn amplitude_at(&self, time: f32, note_duration: f32) -> f32 {
        if time < 0.0 {
            return 0.0;
        }

        // Attack phase: 0 to 1 over attack time
        if time < self.attack {
            return time / self.attack;
        }

        // Decay phase: 1 to sustain over decay time
        let decay_start = self.attack;
        if time < decay_start + self.decay {
            let decay_progress = (time - decay_start) / self.decay;
            return 1.0 - (1.0 - self.sustain) * decay_progress;
        }

        // Sustain phase: hold at sustain level until note_duration
        let sustain_end = note_duration;
        if time < sustain_end {
            return self.sustain;
        }

        // Release phase: sustain to 0 over release time
        let release_progress = (time - sustain_end) / self.release;
        if release_progress >= 1.0 {
            return 0.0;
        }

        self.sustain * (1.0 - release_progress)
    }

    /// Get the total duration of the envelope including release
    pub fn total_duration(&self, note_duration: f32) -> f32 {
        note_duration + self.release
    }
}

impl Default for Envelope {
    /// Default envelope (quick attack, medium decay/release, high sustain)
    fn default() -> Self {
        Self::new(0.01, 0.1, 0.7, 0.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_phases() {
        let env = Envelope::new(0.1, 0.1, 0.5, 0.2);

        // Attack phase
        assert_eq!(env.amplitude_at(0.0, 1.0), 0.0);
        assert_eq!(env.amplitude_at(0.05, 1.0), 0.5);
        assert_eq!(env.amplitude_at(0.1, 1.0), 1.0);

        // Sustain phase
        assert_eq!(env.amplitude_at(0.5, 1.0), 0.5);

        // Release phase
        assert!(env.amplitude_at(1.1, 1.0) < 0.5);
        assert_eq!(env.amplitude_at(1.2, 1.0), 0.0);
    }

    #[test]
    fn test_presets() {
        let _piano = Envelope::piano();
        let _organ = Envelope::organ();
        let _pad = Envelope::pad();
        let _pluck = Envelope::pluck();

        // Just ensure they don't panic
    }
}
