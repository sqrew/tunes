/// Envelope curve shape
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvelopeCurve {
    Linear,      // Linear ramps (fastest, but less natural)
    Exponential, // Exponential curves (most natural, default)
    Logarithmic, // Logarithmic curves (punchy attack, smooth release)
}

/// ADSR (Attack, Decay, Sustain, Release) envelope for shaping sound amplitude over time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Envelope {
    pub attack: f32,  // Time to reach peak amplitude (seconds)
    pub decay: f32,   // Time to decay from peak to sustain level (seconds)
    pub sustain: f32, // Sustain level (0.0 to 1.0)
    pub release: f32, // Time to fade from sustain to silence after note ends (seconds)
    pub curve: EnvelopeCurve, // Envelope curve shape

    // Pre-computed reciprocals to avoid division in hot path
    attack_recip: f32,
    decay_recip: f32,
    release_recip: f32,
}

impl Envelope {
    /// Create a new ADSR envelope with exponential curves (most natural)
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self::with_curve(attack, decay, sustain, release, EnvelopeCurve::Exponential)
    }

    /// Create a new ADSR envelope with specified curve shape
    pub fn with_curve(attack: f32, decay: f32, sustain: f32, release: f32, curve: EnvelopeCurve) -> Self {
        // Minimum values to avoid division by zero
        let attack = attack.max(0.001);
        let decay = decay.max(0.001);
        let release = release.max(0.001);

        Self {
            attack,
            decay,
            sustain: sustain.clamp(0.0, 1.0),
            release,
            curve,
            // Pre-compute reciprocals for fast multiplication instead of division
            attack_recip: 1.0 / attack,
            decay_recip: 1.0 / decay,
            release_recip: 1.0 / release,
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
    #[inline(always)]
    pub fn amplitude_at(&self, time: f32, note_duration: f32) -> f32 {
        // Early exit for negative time
        if time < 0.0 {
            return 0.0;
        }

        // Attack phase: 0 to 1 over attack time
        if time < self.attack {
            let progress = time * self.attack_recip; // Multiply by reciprocal instead of divide
            return self.apply_curve(progress, 0.0, 1.0);
        }

        // Decay phase: 1 to sustain over decay time
        let decay_time = time - self.attack;
        if decay_time < self.decay {
            let progress = decay_time * self.decay_recip;
            return self.apply_curve(progress, 1.0, self.sustain);
        }

        // Sustain phase: hold at sustain level until note_duration
        if time < note_duration {
            return self.sustain;
        }

        // Release phase: sustain to 0 over release time
        let release_time = time - note_duration;
        if release_time >= self.release {
            return 0.0;
        }

        let progress = release_time * self.release_recip;
        self.apply_curve(progress, self.sustain, 0.0)
    }

    /// Apply the envelope curve to interpolate between start and end values
    ///
    /// Progress is 0.0 to 1.0, returns interpolated value with curve applied
    #[inline(always)]
    fn apply_curve(&self, progress: f32, start: f32, end: f32) -> f32 {
        let shaped = match self.curve {
            EnvelopeCurve::Linear => progress,

            EnvelopeCurve::Exponential => {
                // Exponential curve: y = 1 - e^(-4x)
                // Normalized to 0..1 range, where 4 is the curve steepness
                // This creates a natural-sounding envelope
                const CURVE_STEEPNESS: f32 = 4.0;
                let exp_val = (-CURVE_STEEPNESS * progress).exp();
                1.0 - exp_val
            }

            EnvelopeCurve::Logarithmic => {
                // Logarithmic curve: y = log(1 + 9x) / log(10)
                // Fast approximation using sqrt for performance
                // Creates punchy attacks and smooth releases
                progress.sqrt()
            }
        };

        // Linear interpolation between start and end using the shaped progress
        start.mul_add(1.0 - shaped, end * shaped)
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
        // Test with linear curves for predictable values
        let env = Envelope::with_curve(0.1, 0.1, 0.5, 0.2, EnvelopeCurve::Linear);

        // Attack phase
        assert_eq!(env.amplitude_at(0.0, 1.0), 0.0);
        assert!((env.amplitude_at(0.05, 1.0) - 0.5).abs() < 0.01);
        assert!((env.amplitude_at(0.1, 1.0) - 1.0).abs() < 0.01);

        // Sustain phase
        assert_eq!(env.amplitude_at(0.5, 1.0), 0.5);

        // Release phase
        assert!(env.amplitude_at(1.1, 1.0) < 0.5);
        assert_eq!(env.amplitude_at(1.2, 1.0), 0.0);
    }

    #[test]
    fn test_exponential_envelope() {
        // Test that exponential curves produce smooth output
        let env = Envelope::new(0.1, 0.1, 0.5, 0.2);

        // At start of attack
        let start = env.amplitude_at(0.0, 1.0);
        assert_eq!(start, 0.0);

        // Middle of attack should be > 0 and < 1
        let mid = env.amplitude_at(0.05, 1.0);
        assert!(mid > 0.0 && mid < 1.0, "Mid attack = {}", mid);

        // End of attack should be close to 1.0
        let end = env.amplitude_at(0.1, 1.0);
        assert!((end - 1.0).abs() < 0.1, "End attack = {}", end);

        // Sustain should be exact
        assert_eq!(env.amplitude_at(0.5, 1.0), 0.5);
    }

    #[test]
    fn test_curve_types() {
        // Test all curve types produce valid output
        let curves = [
            EnvelopeCurve::Linear,
            EnvelopeCurve::Exponential,
            EnvelopeCurve::Logarithmic,
        ];

        for curve in &curves {
            let env = Envelope::with_curve(0.1, 0.1, 0.5, 0.2, *curve);

            // Test various time points
            for i in 0..20 {
                let time = i as f32 * 0.1;
                let amp = env.amplitude_at(time, 1.0);
                assert!(
                    amp >= 0.0 && amp <= 1.0,
                    "Curve {:?} produced out-of-range value {} at time {}",
                    curve,
                    amp,
                    time
                );
            }
        }
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
