use crate::waveform::Waveform;

/// Low Frequency Oscillator for modulating parameters over time
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub struct LFO {
    pub waveform: Waveform,
    pub frequency: f32,    // Frequency in Hz (typically 0.1 to 20 Hz)
    pub depth: f32,        // Modulation depth (0.0 to 1.0)
    pub phase_offset: f32, // Phase offset in radians (0.0 to 2*PI)
}

impl LFO {
    /// Create a new LFO
    pub fn new(waveform: Waveform, frequency: f32, depth: f32) -> Self {
        Self {
            waveform,
            frequency: frequency.clamp(0.01, 100.0),
            depth: depth.clamp(0.0, 1.0),
            phase_offset: 0.0,
        }
    }

    /// Create an LFO with phase offset
    pub fn with_phase(waveform: Waveform, frequency: f32, depth: f32, phase_offset: f32) -> Self {
        Self {
            waveform,
            frequency: frequency.clamp(0.01, 100.0),
            depth: depth.clamp(0.0, 1.0),
            phase_offset,
        }
    }

    /// Get the modulation value at a given time
    /// Returns a value between 0.0 and 1.0
    #[inline]
    pub fn value_at(&self, time: f32) -> f32 {
        // Optimize phase calculation using FMA
        let phase = time.mul_add(self.frequency, self.phase_offset / (2.0 * std::f32::consts::PI));
        let phase = phase.fract(); // Faster than modulo
        let raw_value = self.waveform.sample(phase);

        // Convert from -1.0..1.0 to 0.0..1.0 range using FMA
        let normalized = raw_value.mul_add(0.5, 0.5);

        // Apply depth: 0.5 + (normalized - 0.5) * depth
        normalized.mul_add(self.depth, 0.5 * (1.0 - self.depth))
    }

    /// Get a bipolar modulation value (-1.0 to 1.0)
    #[inline]
    pub fn bipolar_value_at(&self, time: f32) -> f32 {
        // Optimize phase calculation using FMA
        let phase = time.mul_add(self.frequency, self.phase_offset / (2.0 * std::f32::consts::PI));
        let phase = phase.fract(); // Faster than modulo
        let raw_value = self.waveform.sample(phase);

        // Apply depth to bipolar signal
        raw_value * self.depth
    }

    /// Modulate a parameter value
    /// Base value is the center point, range is how much it can vary
    #[inline]
    pub fn modulate(&self, time: f32, base_value: f32, range: f32) -> f32 {
        let mod_value = self.bipolar_value_at(time);
        base_value.mul_add(1.0, mod_value * range)
    }

    /// Common preset: Slow sine wave for smooth modulation
    pub fn slow_sine(depth: f32) -> Self {
        Self::new(Waveform::Sine, 0.5, depth)
    }

    /// Common preset: Fast sine wave for vibrato/tremolo
    pub fn fast_sine(depth: f32) -> Self {
        Self::new(Waveform::Sine, 5.0, depth)
    }

    /// Common preset: Triangle wave for smooth back-and-forth
    pub fn triangle(frequency: f32, depth: f32) -> Self {
        Self::new(Waveform::Triangle, frequency, depth)
    }

    /// Common preset: Square wave for stepped/rhythmic modulation
    pub fn square(frequency: f32, depth: f32) -> Self {
        Self::new(Waveform::Square, frequency, depth)
    }

    /// Common preset: Sample & hold effect (stepped random-ish)
    pub fn stepped(frequency: f32, depth: f32) -> Self {
        Self::new(Waveform::Square, frequency, depth)
    }
}

/// Modulation target - what parameter to modulate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModTarget {
    FilterCutoff,    // Modulate filter cutoff frequency
    FilterResonance, // Modulate filter resonance
    Volume,          // Modulate volume (tremolo)
    Pitch,           // Modulate pitch (vibrato)
    Pan,             // Modulate stereo pan
}

/// A modulation route connects an LFO to a target parameter
#[derive(Debug, Clone, Copy)]
pub struct ModRoute {
    pub lfo: LFO,
    pub target: ModTarget,
    pub amount: f32, // How much the LFO affects the target (multiplier)
}

impl ModRoute {
    pub fn new(lfo: LFO, target: ModTarget, amount: f32) -> Self {
        Self {
            lfo,
            target,
            amount,
        }
    }

    /// Apply modulation to a base value at a given time
    pub fn apply(&self, time: f32, base_value: f32) -> f32 {
        if self.amount < 0.0001 || self.lfo.depth < 0.0001 {
            return base_value;
        }

        match self.target {
            ModTarget::FilterCutoff => {
                // Filter cutoff modulation (logarithmic scaling)
                let mod_val = self.lfo.bipolar_value_at(time);
                let semitones = mod_val * self.amount * 48.0; // Up to 4 octaves
                base_value * 2f32.powf(semitones / 12.0)
            }
            ModTarget::FilterResonance => {
                // Resonance modulation (linear)
                let mod_val = self.lfo.value_at(time);
                (base_value + (mod_val - 0.5) * self.amount).clamp(0.0, 0.99)
            }
            ModTarget::Volume => {
                // Volume modulation (tremolo)
                let mod_val = self.lfo.value_at(time);
                base_value * (0.5 + mod_val * 0.5 * self.amount)
            }
            ModTarget::Pitch => {
                // Pitch modulation (vibrato) - returns frequency multiplier
                let mod_val = self.lfo.bipolar_value_at(time);
                let cents = mod_val * self.amount * 100.0; // Up to 100 cents
                2f32.powf(cents / 1200.0)
            }
            ModTarget::Pan => {
                // Pan modulation
                let mod_val = self.lfo.value_at(time);
                (base_value + (mod_val - 0.5) * self.amount).clamp(0.0, 1.0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfo_range() {
        let lfo = LFO::new(Waveform::Sine, 1.0, 1.0);

        // Test unipolar output
        for i in 0..100 {
            let time = i as f32 / 100.0;
            let value = lfo.value_at(time);
            assert!(
                value >= 0.0 && value <= 1.0,
                "Value {} out of range at time {}",
                value,
                time
            );
        }

        // Test bipolar output
        for i in 0..100 {
            let time = i as f32 / 100.0;
            let value = lfo.bipolar_value_at(time);
            assert!(
                value >= -1.0 && value <= 1.0,
                "Bipolar value {} out of range at time {}",
                value,
                time
            );
        }
    }

    #[test]
    fn test_modulation_presets() {
        let _slow = LFO::slow_sine(0.5);
        let _fast = LFO::fast_sine(0.3);
        let _tri = LFO::triangle(2.0, 0.7);
        let _sq = LFO::square(4.0, 0.8);
    }

    #[test]
    fn test_mod_route() {
        let lfo = LFO::new(Waveform::Sine, 1.0, 1.0);
        let route = ModRoute::new(lfo, ModTarget::Volume, 1.0);

        let modulated = route.apply(0.0, 0.5);
        assert!(modulated >= 0.0 && modulated <= 1.0);
    }
}
