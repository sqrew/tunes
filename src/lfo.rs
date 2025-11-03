use crate::waveform::Waveform;

/// Low Frequency Oscillator for modulating parameters over time
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub struct LFO {
    pub waveform: Waveform,
    pub frequency: f32,    // Frequency in Hz (typically 0.1 to 20 Hz)
    pub depth: f32,        // Modulation depth (0.0 to 1.0)
    pub phase_offset: f32, // Phase offset in radians (0.0 to 2*PI)

    // Internal state for phase accumulation
    phase: f32,            // Current phase (0.0 to 1.0)
    phase_increment: f32,  // How much to increment phase per sample
}

impl LFO {
    /// Create a new LFO with default sample rate (44100 Hz)
    pub fn new(waveform: Waveform, frequency: f32, depth: f32) -> Self {
        Self::with_sample_rate(waveform, frequency, depth, 44100.0)
    }

    /// Create an LFO with phase offset
    pub fn with_phase(waveform: Waveform, frequency: f32, depth: f32, phase_offset: f32) -> Self {
        let mut lfo = Self::with_sample_rate(waveform, frequency, depth, 44100.0);
        lfo.phase_offset = phase_offset;
        lfo.phase = phase_offset / (2.0 * std::f32::consts::PI);
        lfo
    }

    /// Create a new LFO with specified sample rate
    pub fn with_sample_rate(waveform: Waveform, frequency: f32, depth: f32, sample_rate: f32) -> Self {
        let frequency = frequency.clamp(0.01, 100.0);
        let phase_increment = frequency / sample_rate;

        Self {
            waveform,
            frequency,
            depth: depth.clamp(0.0, 1.0),
            phase_offset: 0.0,
            phase: 0.0,
            phase_increment,
        }
    }

    /// Set the sample rate (updates phase increment)
    #[inline]
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.phase_increment = self.frequency / sample_rate;
    }

    /// Set frequency (updates phase increment if sample rate is known)
    #[inline]
    pub fn set_frequency(&mut self, frequency: f32, sample_rate: f32) {
        self.frequency = frequency.clamp(0.01, 100.0);
        self.phase_increment = self.frequency / sample_rate;
    }

    /// Reset phase to start
    #[inline]
    pub fn reset(&mut self) {
        self.phase = self.phase_offset / (2.0 * std::f32::consts::PI);
    }

    /// Tick the LFO forward one sample (FAST - use this in audio loops!)
    ///
    /// This is the OPTIMIZED way to use LFOs in real-time audio processing.
    /// Call this once per sample, then use value() to get the current output.
    #[inline(always)]
    pub fn tick(&mut self) {
        self.phase += self.phase_increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
    }

    /// Get the current modulation value (0.0 to 1.0)
    ///
    /// IMPORTANT: Call tick() first to advance the phase!
    /// This method just reads the current state - very fast.
    #[inline(always)]
    pub fn value(&self) -> f32 {
        let raw_value = self.waveform.sample(self.phase);

        // Convert from -1.0..1.0 to 0.0..1.0 range using FMA
        let normalized = raw_value.mul_add(0.5, 0.5);

        // Apply depth: 0.5 + (normalized - 0.5) * depth
        normalized.mul_add(self.depth, 0.5 * (1.0 - self.depth))
    }

    /// Get the current bipolar modulation value (-1.0 to 1.0)
    ///
    /// IMPORTANT: Call tick() first to advance the phase!
    #[inline(always)]
    pub fn bipolar_value(&self) -> f32 {
        let raw_value = self.waveform.sample(self.phase);
        raw_value * self.depth
    }

    /// Modulate a parameter value using current LFO state
    ///
    /// Call tick() first to advance the phase!
    /// Base value is the center point, range is how much it can vary
    #[inline(always)]
    pub fn modulate(&self, base_value: f32, range: f32) -> f32 {
        let mod_value = self.bipolar_value();
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

    /// Apply modulation using current LFO state
    ///
    /// IMPORTANT: Call lfo.tick() first to advance the phase!
    /// This method uses the current LFO state for maximum performance.
    #[inline(always)]
    pub fn apply(&self, base_value: f32) -> f32 {
        // Early exit if modulation is negligible
        if self.amount < 0.0001 || self.lfo.depth < 0.0001 {
            return base_value;
        }

        match self.target {
            ModTarget::FilterCutoff => {
                // Filter cutoff modulation (logarithmic scaling)
                let mod_val = self.lfo.bipolar_value();
                let semitones = mod_val.mul_add(self.amount * 48.0, 0.0);
                base_value * 2f32.powf(semitones / 12.0)
            }
            ModTarget::FilterResonance => {
                // Resonance modulation (linear)
                let mod_val = self.lfo.value();
                base_value.mul_add(1.0, (mod_val - 0.5) * self.amount).clamp(0.0, 0.99)
            }
            ModTarget::Volume => {
                // Volume modulation (tremolo)
                let mod_val = self.lfo.value();
                base_value * mod_val.mul_add(0.5 * self.amount, 0.5)
            }
            ModTarget::Pitch => {
                // Pitch modulation (vibrato)
                let mod_val = self.lfo.bipolar_value();
                let cents = mod_val.mul_add(self.amount * 100.0, 0.0);
                2f32.powf(cents / 1200.0)
            }
            ModTarget::Pan => {
                // Pan modulation
                let mod_val = self.lfo.value();
                base_value.mul_add(1.0, (mod_val - 0.5) * self.amount).clamp(0.0, 1.0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfo_range() {
        let mut lfo = LFO::with_sample_rate(Waveform::Sine, 1.0, 1.0, 100.0);

        // Test unipolar output
        for _ in 0..100 {
            lfo.tick();
            let value = lfo.value();
            assert!(
                value >= 0.0 && value <= 1.0,
                "Value {} out of range",
                value
            );
        }

        // Reset and test bipolar output
        lfo.reset();
        for _ in 0..100 {
            lfo.tick();
            let value = lfo.bipolar_value();
            assert!(
                value >= -1.0 && value <= 1.0,
                "Bipolar value {} out of range",
                value
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
        let mut lfo = LFO::with_sample_rate(Waveform::Sine, 1.0, 1.0, 44100.0);
        let route = ModRoute::new(lfo, ModTarget::Volume, 1.0);

        lfo.tick();
        let modulated = route.apply(0.5);
        assert!(modulated >= 0.0 && modulated <= 1.0);
    }

    #[test]
    fn test_lfo_tick() {
        let mut lfo = LFO::with_sample_rate(Waveform::Sine, 1.0, 1.0, 100.0);

        // Tick through one cycle (100 samples at 1Hz at 100Hz sample rate)
        for _ in 0..100 {
            lfo.tick();
            let value = lfo.value();
            assert!(value >= 0.0 && value <= 1.0, "Value {} out of range", value);
        }
    }

    #[test]
    fn test_lfo_phase_accumulation() {
        let mut lfo = LFO::with_sample_rate(Waveform::Sine, 10.0, 1.0, 1000.0);

        // Should complete 10 cycles in 1000 samples (10Hz at 1000Hz)
        for _ in 0..1000 {
            lfo.tick();
        }

        // Phase should wrap correctly (near 0.0 after full cycles)
        assert!(lfo.phase < 0.1 || lfo.phase > 0.9);
    }

    #[test]
    fn test_lfo_consistency() {
        // Test that LFO produces consistent values
        let mut lfo = LFO::with_sample_rate(Waveform::Sine, 1.0, 1.0, 44100.0);

        let mut values = Vec::new();
        for _ in 0..100 {
            lfo.tick();
            values.push(lfo.value());
        }

        // Reset and verify we get the same values
        lfo.reset();
        for i in 0..100 {
            lfo.tick();
            let value = lfo.value();
            assert!((value - values[i]).abs() < 0.0001,
                "Value {} differs from original {} at index {}", value, values[i], i);
        }
    }

    #[test]
    fn test_mod_route_all_targets() {
        let mut lfo = LFO::with_sample_rate(Waveform::Sine, 1.0, 1.0, 44100.0);

        let targets = [
            ModTarget::FilterCutoff,
            ModTarget::FilterResonance,
            ModTarget::Volume,
            ModTarget::Pitch,
            ModTarget::Pan,
        ];

        for target in &targets {
            let route = ModRoute::new(lfo, *target, 0.5);
            lfo.tick();
            let modulated = route.apply(0.5);
            assert!(modulated.is_finite(), "Target {:?} produced non-finite value", target);
        }
    }
}
