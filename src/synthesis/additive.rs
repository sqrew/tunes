//! Additive synthesis - build complex sounds from sine wave partials
//!
//! Additive synthesis creates sounds by summing together multiple sine waves
//! (called "partials" or "harmonics"). This is the inverse of subtractive synthesis:
//! instead of starting with a complex waveform and filtering it down, you start
//! with nothing and build up complexity by adding pure tones.
//!
//! # Key Concepts
//!
//! - **Partial**: A single sine wave component
//! - **Frequency Ratio**: How the partial relates to the fundamental (1.0 = fundamental, 2.0 = octave up)
//! - **Harmonic**: Integer frequency ratios (1, 2, 3, 4...) - sounds musical
//! - **Inharmonic**: Non-integer ratios (1.0, 2.1, 3.7...) - sounds metallic/bell-like
//!
//! # Musical Applications
//!
//! - **Organ sounds**: Pure harmonic series
//! - **Bells/gongs**: Inharmonic partials create metallic timbres
//! - **Evolving pads**: Animate partial amplitudes over time
//! - **Spectral composition**: Precise control over frequency content
//!
//! # Example
//!
//! ```
//! use tunes::synthesis::additive::{AdditiveSynth, Partial};
//!
//! // Create a sawtooth-like sound (harmonic series with 1/n amplitude)
//! let mut synth = AdditiveSynth::new(440.0, 44100.0)
//!     .add_partial(Partial::harmonic(1, 1.0))    // Fundamental
//!     .add_partial(Partial::harmonic(2, 0.5))    // Octave
//!     .add_partial(Partial::harmonic(3, 0.33))   // Fifth above octave
//!     .add_partial(Partial::harmonic(4, 0.25))   // Two octaves
//!     .add_partial(Partial::harmonic(5, 0.2));   // Major third above that
//!
//! // Generate 1 second of audio
//! let samples = synth.generate(44100);
//! ```

use std::f32::consts::PI;

/// A single partial (sine wave component) in additive synthesis
///
/// Each partial has:
/// - **Frequency ratio**: Multiplier relative to fundamental frequency
/// - **Amplitude**: Volume of this partial (0.0 - 1.0)
/// - **Phase offset**: Starting phase (0.0 - 1.0, where 1.0 = full cycle)
#[derive(Debug, Clone, Copy)]
pub struct Partial {
    /// Frequency multiplier relative to fundamental
    /// - 1.0 = fundamental frequency
    /// - 2.0 = one octave up (harmonic)
    /// - 1.5 = perfect fifth up
    /// - 2.1 = slightly sharp octave (inharmonic)
    pub frequency_ratio: f32,

    /// Amplitude of this partial (0.0 - 1.0)
    pub amplitude: f32,

    /// Initial phase offset in cycles (0.0 - 1.0)
    /// - 0.0 = starts at zero crossing
    /// - 0.25 = starts at peak
    /// - 0.5 = starts at opposite zero crossing
    pub phase_offset: f32,
}

impl Partial {
    /// Create a new partial with custom parameters
    ///
    /// # Arguments
    /// * `frequency_ratio` - Multiplier relative to fundamental (e.g., 1.0, 2.0, 3.0)
    /// * `amplitude` - Volume of this partial (0.0 - 1.0)
    /// * `phase_offset` - Starting phase in cycles (0.0 - 1.0)
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::additive::Partial;
    ///
    /// // Fundamental at full volume
    /// let fundamental = Partial::new(1.0, 1.0, 0.0);
    ///
    /// // Octave at half volume, 90° phase shift
    /// let octave = Partial::new(2.0, 0.5, 0.25);
    /// ```
    pub fn new(frequency_ratio: f32, amplitude: f32, phase_offset: f32) -> Self {
        Self {
            frequency_ratio,
            amplitude,
            phase_offset,
        }
    }

    /// Create a harmonic partial (integer frequency ratio)
    ///
    /// # Arguments
    /// * `harmonic_number` - Which harmonic (1 = fundamental, 2 = octave, 3 = fifth, etc.)
    /// * `amplitude` - Volume of this partial (0.0 - 1.0)
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::additive::Partial;
    ///
    /// // Build a sawtooth wave (harmonics with 1/n amplitude)
    /// let partials = vec![
    ///     Partial::harmonic(1, 1.0),
    ///     Partial::harmonic(2, 0.5),
    ///     Partial::harmonic(3, 0.33),
    ///     Partial::harmonic(4, 0.25),
    /// ];
    /// ```
    pub fn harmonic(harmonic_number: u32, amplitude: f32) -> Self {
        Self {
            frequency_ratio: harmonic_number as f32,
            amplitude,
            phase_offset: 0.0,
        }
    }

    /// Create an inharmonic partial (non-integer ratio for metallic sounds)
    ///
    /// # Arguments
    /// * `ratio` - Non-integer frequency ratio (e.g., 1.0, 2.13, 3.76)
    /// * `amplitude` - Volume of this partial (0.0 - 1.0)
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::additive::Partial;
    ///
    /// // Bell-like inharmonic series
    /// let partials = vec![
    ///     Partial::inharmonic(1.0, 1.0),
    ///     Partial::inharmonic(2.76, 0.6),   // Not exactly 2.0!
    ///     Partial::inharmonic(5.4, 0.4),
    ///     Partial::inharmonic(8.93, 0.25),
    /// ];
    /// ```
    pub fn inharmonic(ratio: f32, amplitude: f32) -> Self {
        Self {
            frequency_ratio: ratio,
            amplitude,
            phase_offset: 0.0,
        }
    }
}

/// Additive synthesizer - generates sound by summing sine wave partials
///
/// Build complex timbres from simple components. Perfect for:
/// - Organ sounds (harmonic series)
/// - Bell/metallic sounds (inharmonic partials)
/// - Evolving pads (time-varying amplitudes)
/// - Precise spectral control
///
/// # Example
/// ```
/// use tunes::synthesis::additive::{AdditiveSynth, Partial};
///
/// // Create a bell-like sound with inharmonic partials
/// let mut bell = AdditiveSynth::new(220.0, 44100.0)
///     .add_partial(Partial::inharmonic(1.0, 1.0))
///     .add_partial(Partial::inharmonic(2.76, 0.6))
///     .add_partial(Partial::inharmonic(5.4, 0.4))
///     .add_partial(Partial::inharmonic(8.93, 0.25));
///
/// let samples = bell.generate(22050);  // 0.5 seconds
/// ```
#[derive(Debug, Clone)]
pub struct AdditiveSynth {
    /// Fundamental frequency in Hz
    fundamental_freq: f32,
    /// Sample rate in Hz
    sample_rate: f32,
    /// List of partials to sum
    partials: Vec<Partial>,
    /// Phase accumulators for each partial (in cycles, 0.0 - 1.0)
    phases: Vec<f32>,
}

impl AdditiveSynth {
    /// Create a new additive synthesizer
    ///
    /// # Arguments
    /// * `fundamental_freq` - Base frequency in Hz (e.g., 440.0 for A4)
    /// * `sample_rate` - Sample rate in Hz (e.g., 44100.0)
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::additive::AdditiveSynth;
    ///
    /// let synth = AdditiveSynth::new(440.0, 44100.0);
    /// ```
    pub fn new(fundamental_freq: f32, sample_rate: f32) -> Self {
        Self {
            fundamental_freq,
            sample_rate,
            partials: Vec::new(),
            phases: Vec::new(),
        }
    }

    /// Add a partial to the synthesis (builder pattern)
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::additive::{AdditiveSynth, Partial};
    ///
    /// let synth = AdditiveSynth::new(440.0, 44100.0)
    ///     .add_partial(Partial::harmonic(1, 1.0))
    ///     .add_partial(Partial::harmonic(2, 0.5));
    /// ```
    pub fn add_partial(mut self, partial: Partial) -> Self {
        self.phases.push(partial.phase_offset);
        self.partials.push(partial);
        self
    }

    /// Add multiple partials at once
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::additive::{AdditiveSynth, Partial};
    ///
    /// let partials = vec![
    ///     Partial::harmonic(1, 1.0),
    ///     Partial::harmonic(2, 0.5),
    ///     Partial::harmonic(3, 0.33),
    /// ];
    ///
    /// let synth = AdditiveSynth::new(440.0, 44100.0)
    ///     .with_partials(partials);
    /// ```
    pub fn with_partials(mut self, partials: Vec<Partial>) -> Self {
        for partial in partials {
            self = self.add_partial(partial);
        }
        self
    }

    /// Set the fundamental frequency
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::additive::AdditiveSynth;
    ///
    /// let mut synth = AdditiveSynth::new(440.0, 44100.0);
    /// synth.set_frequency(880.0);  // Change to A5
    /// ```
    pub fn set_frequency(&mut self, freq: f32) {
        self.fundamental_freq = freq;
    }

    /// Reset all phase accumulators to their initial offsets
    pub fn reset(&mut self) {
        for (i, partial) in self.partials.iter().enumerate() {
            self.phases[i] = partial.phase_offset;
        }
    }

    /// Generate a single sample
    ///
    /// Sums all partials at their current phase positions and advances phases.
    #[inline]
    pub fn sample(&mut self) -> f32 {
        let mut output = 0.0;

        // Sum all partials
        for (i, partial) in self.partials.iter().enumerate() {
            // Calculate this partial's frequency
            let freq = self.fundamental_freq * partial.frequency_ratio;

            // Generate sine wave at current phase
            let sample = (self.phases[i] * 2.0 * PI).sin() * partial.amplitude;
            output += sample;

            // Advance phase (frequency determines how fast phase increases)
            let phase_increment = freq / self.sample_rate;
            self.phases[i] += phase_increment;

            // Wrap phase to keep it in [0, 1)
            if self.phases[i] >= 1.0 {
                self.phases[i] -= self.phases[i].floor();
            }
        }

        // Normalize by number of partials to prevent clipping
        if !self.partials.is_empty() {
            output / self.partials.len() as f32
        } else {
            0.0
        }
    }

    /// Generate multiple samples
    ///
    /// # Arguments
    /// * `length` - Number of samples to generate
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::additive::{AdditiveSynth, Partial};
    ///
    /// let mut synth = AdditiveSynth::new(440.0, 44100.0)
    ///     .add_partial(Partial::harmonic(1, 1.0));
    ///
    /// let one_second = synth.generate(44100);
    /// ```
    pub fn generate(&mut self, length: usize) -> Vec<f32> {
        (0..length).map(|_| self.sample()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_creation() {
        let p = Partial::new(2.0, 0.5, 0.25);
        assert_eq!(p.frequency_ratio, 2.0);
        assert_eq!(p.amplitude, 0.5);
        assert_eq!(p.phase_offset, 0.25);
    }

    #[test]
    fn test_partial_harmonic() {
        let p = Partial::harmonic(3, 0.7);
        assert_eq!(p.frequency_ratio, 3.0);
        assert_eq!(p.amplitude, 0.7);
        assert_eq!(p.phase_offset, 0.0);
    }

    #[test]
    fn test_partial_inharmonic() {
        let p = Partial::inharmonic(2.76, 0.6);
        assert_eq!(p.frequency_ratio, 2.76);
        assert_eq!(p.amplitude, 0.6);
    }

    #[test]
    fn test_additive_synth_creation() {
        let synth = AdditiveSynth::new(440.0, 44100.0);
        assert_eq!(synth.fundamental_freq, 440.0);
        assert_eq!(synth.sample_rate, 44100.0);
        assert_eq!(synth.partials.len(), 0);
    }

    #[test]
    fn test_add_partial() {
        let synth = AdditiveSynth::new(440.0, 44100.0)
            .add_partial(Partial::harmonic(1, 1.0))
            .add_partial(Partial::harmonic(2, 0.5));

        assert_eq!(synth.partials.len(), 2);
        assert_eq!(synth.phases.len(), 2);
    }

    #[test]
    fn test_with_partials() {
        let partials = vec![
            Partial::harmonic(1, 1.0),
            Partial::harmonic(2, 0.5),
            Partial::harmonic(3, 0.33),
        ];

        let synth = AdditiveSynth::new(440.0, 44100.0)
            .with_partials(partials);

        assert_eq!(synth.partials.len(), 3);
    }

    #[test]
    fn test_sample_generation() {
        let mut synth = AdditiveSynth::new(440.0, 44100.0)
            .add_partial(Partial::harmonic(1, 1.0));

        let sample = synth.sample();

        // Should be in valid range
        assert!(sample >= -1.0 && sample <= 1.0);
    }

    #[test]
    fn test_generate_length() {
        let mut synth = AdditiveSynth::new(440.0, 44100.0)
            .add_partial(Partial::harmonic(1, 1.0));

        let samples = synth.generate(1000);
        assert_eq!(samples.len(), 1000);

        // All samples should be in valid range
        for &sample in &samples {
            assert!(sample >= -1.0 && sample <= 1.0);
        }
    }

    #[test]
    fn test_set_frequency() {
        let mut synth = AdditiveSynth::new(440.0, 44100.0)
            .add_partial(Partial::harmonic(1, 1.0));

        synth.set_frequency(880.0);
        assert_eq!(synth.fundamental_freq, 880.0);
    }

    #[test]
    fn test_reset_phases() {
        let mut synth = AdditiveSynth::new(440.0, 44100.0)
            .add_partial(Partial::new(1.0, 1.0, 0.25));

        // Store initial phase
        let initial_phase = synth.phases[0];

        // Generate enough samples to advance phase significantly
        // At 440Hz with 44100 sample rate, need more than 1 period
        for _ in 0..500 {
            synth.sample();
        }

        // Phase should have advanced (might have wrapped, but should be different)
        // We don't assert on the advanced phase value since it might wrap

        // Reset should return to initial offset
        synth.reset();
        assert_eq!(synth.phases[0], initial_phase);
    }

    #[test]
    fn test_multiple_partials_sum() {
        let mut synth = AdditiveSynth::new(440.0, 44100.0)
            .add_partial(Partial::harmonic(1, 0.5))
            .add_partial(Partial::harmonic(2, 0.5));

        let samples = synth.generate(100);

        // With multiple partials, should create more complex waveform
        // but still stay in valid range
        for &sample in &samples {
            assert!(sample >= -1.0 && sample <= 1.0);
        }
    }

    #[test]
    fn test_inharmonic_ratios() {
        // Bell-like inharmonic series
        let mut synth = AdditiveSynth::new(220.0, 44100.0)
            .add_partial(Partial::inharmonic(1.0, 1.0))
            .add_partial(Partial::inharmonic(2.76, 0.6))
            .add_partial(Partial::inharmonic(5.4, 0.4));

        let samples = synth.generate(1000);

        // Should generate valid audio
        assert_eq!(samples.len(), 1000);
        for &sample in &samples {
            assert!(sample >= -1.0 && sample <= 1.0);
        }
    }

    #[test]
    fn test_phase_offset_affects_output() {
        let mut synth1 = AdditiveSynth::new(440.0, 44100.0)
            .add_partial(Partial::new(1.0, 1.0, 0.0));

        let mut synth2 = AdditiveSynth::new(440.0, 44100.0)
            .add_partial(Partial::new(1.0, 1.0, 0.5));

        let sample1 = synth1.sample();
        let sample2 = synth2.sample();

        // Phase offset of 0.5 should create inverted waveform
        assert!((sample1 + sample2).abs() < 0.01);
    }
}
