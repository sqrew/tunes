/// Filter Envelope - ADSR envelope specifically for controlling filter cutoff
///
/// This allows classic subtractive synthesis techniques where the filter cutoff
/// sweeps over time independently of the amplitude envelope.
///
/// The envelope modulates the filter cutoff frequency from a base frequency to
/// a peak frequency and back, following an ADSR curve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilterEnvelope {
    pub attack: f32,    // Time to reach peak cutoff (seconds)
    pub decay: f32,     // Time to decay from peak to sustain level (seconds)
    pub sustain: f32,   // Sustain level (0.0 to 1.0, relative to peak)
    pub release: f32,   // Time to return to base after note ends (seconds)
    pub base_cutoff: f32,  // Starting cutoff frequency (Hz)
    pub peak_cutoff: f32,  // Peak cutoff frequency (Hz)
    pub amount: f32,    // Envelope amount/intensity (0.0 to 1.0)
}

impl FilterEnvelope {
    /// Create a new filter envelope
    ///
    /// # Arguments
    /// * `attack` - Time to reach peak cutoff (seconds)
    /// * `decay` - Time to decay to sustain level (seconds)
    /// * `sustain` - Sustain level (0.0 to 1.0)
    /// * `release` - Time to return to base cutoff (seconds)
    /// * `base_cutoff` - Starting/resting cutoff frequency (Hz)
    /// * `peak_cutoff` - Maximum cutoff frequency (Hz)
    /// * `amount` - Envelope intensity (0.0 = no effect, 1.0 = full effect)
    pub fn new(
        attack: f32,
        decay: f32,
        sustain: f32,
        release: f32,
        base_cutoff: f32,
        peak_cutoff: f32,
        amount: f32,
    ) -> Self {
        Self {
            attack: attack.max(0.001),
            decay: decay.max(0.001),
            sustain: sustain.clamp(0.0, 1.0),
            release: release.max(0.001),
            base_cutoff: base_cutoff.clamp(20.0, 20000.0),
            peak_cutoff: peak_cutoff.clamp(20.0, 20000.0),
            amount: amount.clamp(0.0, 1.0),
        }
    }

    /// Classic analog synth filter sweep (fast attack, medium decay, low sustain)
    ///
    /// Creates that classic "wah" sound as the filter opens and closes
    pub fn classic() -> Self {
        Self::new(0.01, 0.3, 0.3, 0.5, 200.0, 5000.0, 1.0)
    }

    /// Pluck/percussive filter envelope (instant attack, fast decay, no sustain)
    ///
    /// Good for plucked strings, percussion, bass
    pub fn pluck() -> Self {
        Self::new(0.001, 0.15, 0.1, 0.2, 300.0, 4000.0, 1.0)
    }

    /// Slow pad filter sweep (slow attack and release)
    ///
    /// Creates evolving, atmospheric textures
    pub fn pad() -> Self {
        Self::new(0.8, 0.5, 0.7, 1.0, 400.0, 3000.0, 0.8)
    }

    /// Bright/open filter (starts high, stays high)
    ///
    /// Minimal filter movement
    pub fn bright() -> Self {
        Self::new(0.001, 0.1, 0.9, 0.3, 2000.0, 8000.0, 0.5)
    }

    /// Bass filter (low cutoff range for deep bass tones)
    pub fn bass() -> Self {
        Self::new(0.01, 0.2, 0.4, 0.3, 100.0, 800.0, 1.0)
    }

    /// Bypass filter envelope (no modulation)
    pub fn none() -> Self {
        Self::new(0.001, 0.001, 1.0, 0.001, 10000.0, 10000.0, 0.0)
    }

    /// Calculate the filter cutoff frequency at a given time within the note
    ///
    /// # Arguments
    /// * `time` - Time since note started (seconds)
    /// * `note_duration` - Total duration of the note (seconds)
    ///
    /// # Returns
    /// The cutoff frequency in Hz at this point in time
    pub fn cutoff_at(&self, time: f32, note_duration: f32) -> f32 {
        if self.amount == 0.0 {
            return self.base_cutoff;
        }

        let envelope_value = self.envelope_value_at(time, note_duration);

        // Interpolate between base and peak cutoff using envelope value
        // Use logarithmic interpolation for more natural filter sweeps
        let log_base = self.base_cutoff.ln();
        let log_peak = self.peak_cutoff.ln();
        let log_cutoff = log_base + (log_peak - log_base) * envelope_value * self.amount;

        log_cutoff.exp().clamp(20.0, 20000.0)
    }

    /// Get the envelope value (0.0 to 1.0) at a given time
    ///
    /// This follows the same ADSR curve as the amplitude envelope
    fn envelope_value_at(&self, time: f32, note_duration: f32) -> f32 {
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

impl Default for FilterEnvelope {
    /// Default filter envelope with moderate settings
    fn default() -> Self {
        Self::none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_envelope_creation() {
        let env = FilterEnvelope::new(0.1, 0.2, 0.5, 0.3, 200.0, 2000.0, 1.0);
        assert_eq!(env.attack, 0.1);
        assert_eq!(env.decay, 0.2);
        assert_eq!(env.sustain, 0.5);
        assert_eq!(env.release, 0.3);
        assert_eq!(env.base_cutoff, 200.0);
        assert_eq!(env.peak_cutoff, 2000.0);
    }

    #[test]
    fn test_filter_envelope_cutoff_progression() {
        let env = FilterEnvelope::new(0.1, 0.1, 0.5, 0.2, 200.0, 2000.0, 1.0);

        // At start (t=0), should be at base
        let start_cutoff = env.cutoff_at(0.0, 1.0);
        assert!(
            start_cutoff < 300.0,
            "Should start near base cutoff, got {}",
            start_cutoff
        );

        // During attack (t=0.05), should be rising
        let attack_cutoff = env.cutoff_at(0.05, 1.0);
        assert!(
            attack_cutoff > start_cutoff && attack_cutoff < 2000.0,
            "Should be between base and peak during attack, got {}",
            attack_cutoff
        );

        // At peak of attack (t=0.1), should be near peak
        let peak_cutoff = env.cutoff_at(0.1, 1.0);
        assert!(
            peak_cutoff > 1500.0,
            "Should be near peak cutoff, got {}",
            peak_cutoff
        );

        // During sustain (t=0.5), should be at sustain level
        let sustain_cutoff = env.cutoff_at(0.5, 1.0);
        assert!(
            sustain_cutoff > 200.0 && sustain_cutoff < peak_cutoff,
            "Sustain should be between base and peak, got {}",
            sustain_cutoff
        );

        // After release (t=1.3), should return toward base
        let release_cutoff = env.cutoff_at(1.3, 1.0);
        assert!(
            release_cutoff < sustain_cutoff,
            "Should be decaying during release, got {}",
            release_cutoff
        );
    }

    #[test]
    fn test_filter_envelope_amount() {
        // With amount = 0, should always return base cutoff
        let env_off = FilterEnvelope::new(0.1, 0.1, 0.5, 0.2, 200.0, 2000.0, 0.0);
        assert_eq!(env_off.cutoff_at(0.05, 1.0), 200.0);
        assert_eq!(env_off.cutoff_at(0.5, 1.0), 200.0);

        // With amount = 0.5, should be halfway between base and full envelope
        let env_half = FilterEnvelope::new(0.1, 0.1, 0.5, 0.2, 200.0, 2000.0, 0.5);
        let half_peak = env_half.cutoff_at(0.1, 1.0);
        let full_env = FilterEnvelope::new(0.1, 0.1, 0.5, 0.2, 200.0, 2000.0, 1.0);
        let full_peak = full_env.cutoff_at(0.1, 1.0);

        // Half amount should produce less filter movement
        assert!(half_peak < full_peak);
    }

    #[test]
    fn test_presets() {
        let _classic = FilterEnvelope::classic();
        let _pluck = FilterEnvelope::pluck();
        let _pad = FilterEnvelope::pad();
        let _bright = FilterEnvelope::bright();
        let _bass = FilterEnvelope::bass();
        let _none = FilterEnvelope::none();
    }

    #[test]
    fn test_total_duration() {
        let env = FilterEnvelope::new(0.1, 0.2, 0.5, 0.3, 200.0, 2000.0, 1.0);
        assert_eq!(env.total_duration(1.0), 1.3); // note_duration + release
    }
}
