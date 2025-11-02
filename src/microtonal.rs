//! Microtonal music support
//!
//! This module provides tools for working with alternative tuning systems beyond standard
//! 12-tone equal temperament (12-TET). It includes:
//!
//! - Equal temperaments with any number of divisions (EDO - Equal Divisions of the Octave)
//! - Just intonation ratios
//! - Historical temperaments
//! - Cent-based calculations
//! - Common non-Western tuning systems
//!
//! # Examples
//!
//! ```
//! use tunes::microtonal::*;
//! use tunes::notes::A4;
//!
//! // 19-tone equal temperament (common in Arabic music)
//! let edo19 = Edo::new(19);
//! let note = edo19.step(A4, 5); // 5 steps up from A4 in 19-TET
//!
//! // Quarter tones (24-TET)
//! let quarter_sharp = edo24_step(A4, 1); // A quarter-sharp
//! let half_sharp = edo24_step(A4, 2);    // A half-sharp (standard sharp)
//!
//! // Just intonation perfect fifth
//! let perfect_fifth = just_ratio(A4, 3, 2); // 3:2 ratio = 702 cents
//!
//! // Convert between cents and frequency ratios
//! let ratio = cents_to_ratio(702.0); // ~1.5 (perfect fifth)
//! let cents = ratio_to_cents(1.5);   // ~702 cents
//! ```

/// Equal temperament system with a specified number of divisions per octave
///
/// # Examples
///
/// ```
/// use tunes::microtonal::Edo;
/// use tunes::notes::C4;
///
/// // 19-tone equal temperament
/// let edo19 = Edo::new(19);
/// let note = edo19.step(C4, 3); // 3 steps up from C4
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Edo {
    /// Number of equal divisions per octave
    pub divisions: u32,
    /// Size of one step in cents
    pub step_cents: f32,
}

impl Edo {
    /// Create a new equal temperament system
    ///
    /// # Arguments
    /// * `divisions` - Number of equal divisions per octave (e.g., 12 for standard, 19, 24, 31, 53)
    pub fn new(divisions: u32) -> Self {
        Self {
            divisions,
            step_cents: 1200.0 / divisions as f32,
        }
    }

    /// Calculate frequency for a given step from a base frequency
    ///
    /// # Arguments
    /// * `base_freq` - The starting frequency (e.g., A4 = 440.0)
    /// * `steps` - Number of steps up (positive) or down (negative)
    pub fn step(&self, base_freq: f32, steps: i32) -> f32 {
        base_freq * 2.0_f32.powf(steps as f32 / self.divisions as f32)
    }

    /// Get all notes in one octave starting from a base frequency
    pub fn octave(&self, base_freq: f32) -> Vec<f32> {
        (0..self.divisions)
            .map(|step| self.step(base_freq, step as i32))
            .collect()
    }
}

// Common EDO systems as constants
/// Standard 12-tone equal temperament
pub const EDO12: Edo = Edo {
    divisions: 12,
    step_cents: 100.0,
};

/// 19-tone equal temperament (historical, good for approximating quarter tones)
pub const EDO19: Edo = Edo {
    divisions: 19,
    step_cents: 63.157894,
};

/// 24-tone equal temperament (quarter tones, used in Arabic and contemporary music)
pub const EDO24: Edo = Edo {
    divisions: 24,
    step_cents: 50.0,
};

/// 31-tone equal temperament (very close to meantone, historical significance)
pub const EDO31: Edo = Edo {
    divisions: 31,
    step_cents: 38.70968,
};

/// 53-tone equal temperament (Mercator's comma, historical, approximates Pythagorean tuning)
pub const EDO53: Edo = Edo {
    divisions: 53,
    step_cents: 22.641510,
};

// Convenience functions for common EDOs

/// Get a note in 19-TET
pub fn edo19_step(base_freq: f32, steps: i32) -> f32 {
    EDO19.step(base_freq, steps)
}

/// Get a note in 24-TET (quarter tones)
pub fn edo24_step(base_freq: f32, steps: i32) -> f32 {
    EDO24.step(base_freq, steps)
}

/// Get a note in 31-TET
pub fn edo31_step(base_freq: f32, steps: i32) -> f32 {
    EDO31.step(base_freq, steps)
}

/// Get a note in 53-TET
pub fn edo53_step(base_freq: f32, steps: i32) -> f32 {
    EDO53.step(base_freq, steps)
}

// Cent-based calculations

/// Convert cents to frequency ratio
///
/// Cents are a logarithmic unit of measure for musical intervals.
/// 1200 cents = 1 octave, 100 cents = 1 semitone (in 12-TET)
///
/// # Example
/// ```
/// use tunes::microtonal::cents_to_ratio;
///
/// let perfect_fifth = cents_to_ratio(702.0); // ~1.5 (3:2 ratio)
/// let quarter_tone = cents_to_ratio(50.0);   // Half of a semitone
/// ```
pub fn cents_to_ratio(cents: f32) -> f32 {
    2.0_f32.powf(cents / 1200.0)
}

/// Convert frequency ratio to cents
///
/// # Example
/// ```
/// use tunes::microtonal::ratio_to_cents;
///
/// let cents = ratio_to_cents(1.5); // ~702 cents (perfect fifth)
/// ```
pub fn ratio_to_cents(ratio: f32) -> f32 {
    1200.0 * ratio.log2()
}

/// Calculate frequency from a base frequency and cent offset
///
/// # Example
/// ```
/// use tunes::microtonal::freq_from_cents;
/// use tunes::notes::A4;
///
/// let quarter_sharp = freq_from_cents(A4, 50.0); // A + 50 cents
/// ```
pub fn freq_from_cents(base_freq: f32, cents: f32) -> f32 {
    base_freq * cents_to_ratio(cents)
}

// Just Intonation

/// Calculate frequency using a just intonation ratio
///
/// Just intonation uses simple integer ratios for pure intervals.
///
/// # Arguments
/// * `base_freq` - The starting frequency
/// * `numerator` - Top number of the ratio
/// * `denominator` - Bottom number of the ratio
///
/// # Common Ratios
/// - 1:1 = unison
/// - 9:8 = major second (204 cents)
/// - 5:4 = major third (386 cents)
/// - 4:3 = perfect fourth (498 cents)
/// - 3:2 = perfect fifth (702 cents)
/// - 5:3 = major sixth (884 cents)
/// - 15:8 = major seventh (1088 cents)
/// - 2:1 = octave
///
/// # Example
/// ```
/// use tunes::microtonal::just_ratio;
/// use tunes::notes::C4;
///
/// let perfect_fifth = just_ratio(C4, 3, 2);  // Pure fifth
/// let major_third = just_ratio(C4, 5, 4);    // Pure major third
/// ```
pub fn just_ratio(base_freq: f32, numerator: u32, denominator: u32) -> f32 {
    base_freq * (numerator as f32 / denominator as f32)
}

/// Build a just intonation scale from ratios
///
/// # Example
/// ```
/// use tunes::microtonal::just_scale;
/// use tunes::notes::C4;
///
/// // Ptolemy's intense diatonic scale
/// let ratios = [(1,1), (9,8), (5,4), (4,3), (3,2), (5,3), (15,8), (2,1)];
/// let scale = just_scale(C4, &ratios);
/// ```
pub fn just_scale(base_freq: f32, ratios: &[(u32, u32)]) -> Vec<f32> {
    ratios
        .iter()
        .map(|&(num, den)| just_ratio(base_freq, num, den))
        .collect()
}

// Common just intonation ratios
pub mod just_ratios {
    /// Unison
    pub const UNISON: (u32, u32) = (1, 1);

    /// Minor second (diatonic semitone)
    pub const MINOR_SECOND: (u32, u32) = (16, 15);

    /// Major second (whole tone)
    pub const MAJOR_SECOND: (u32, u32) = (9, 8);

    /// Minor third
    pub const MINOR_THIRD: (u32, u32) = (6, 5);

    /// Major third
    pub const MAJOR_THIRD: (u32, u32) = (5, 4);

    /// Perfect fourth
    pub const PERFECT_FOURTH: (u32, u32) = (4, 3);

    /// Tritone (augmented fourth)
    pub const TRITONE: (u32, u32) = (45, 32);

    /// Perfect fifth
    pub const PERFECT_FIFTH: (u32, u32) = (3, 2);

    /// Minor sixth
    pub const MINOR_SIXTH: (u32, u32) = (8, 5);

    /// Major sixth
    pub const MAJOR_SIXTH: (u32, u32) = (5, 3);

    /// Minor seventh
    pub const MINOR_SEVENTH: (u32, u32) = (9, 5);

    /// Major seventh
    pub const MAJOR_SEVENTH: (u32, u32) = (15, 8);

    /// Octave
    pub const OCTAVE: (u32, u32) = (2, 1);
}

// Preset just intonation scales

/// Ptolemy's intense diatonic scale (classical just intonation major scale)
pub fn just_major_scale(root: f32) -> Vec<f32> {
    use just_ratios::*;
    just_scale(
        root,
        &[
            UNISON,
            MAJOR_SECOND,
            MAJOR_THIRD,
            PERFECT_FOURTH,
            PERFECT_FIFTH,
            MAJOR_SIXTH,
            MAJOR_SEVENTH,
            OCTAVE,
        ],
    )
}

/// Just intonation minor scale
pub fn just_minor_scale(root: f32) -> Vec<f32> {
    use just_ratios::*;
    just_scale(
        root,
        &[
            UNISON,
            MAJOR_SECOND,
            MINOR_THIRD,
            PERFECT_FOURTH,
            PERFECT_FIFTH,
            MINOR_SIXTH,
            MINOR_SEVENTH,
            OCTAVE,
        ],
    )
}

// Historical temperaments

/// Pythagorean tuning - based on stacking perfect fifths (3:2 ratio)
///
/// Creates a scale by tuning fifths pure, which results in pure fourths
/// but impure thirds (Pythagorean thirds are sharper than just thirds)
pub fn pythagorean_scale(root: f32) -> Vec<f32> {
    vec![
        root * 1.0,           // C (1:1)
        root * 256.0 / 243.0, // C# (Pythagorean minor second)
        root * 9.0 / 8.0,     // D (Pythagorean major second)
        root * 32.0 / 27.0,   // D# (Pythagorean minor third)
        root * 81.0 / 64.0,   // E (Pythagorean major third)
        root * 4.0 / 3.0,     // F (Perfect fourth)
        root * 729.0 / 512.0, // F# (Pythagorean tritone)
        root * 3.0 / 2.0,     // G (Perfect fifth)
        root * 128.0 / 81.0,  // G# (Pythagorean minor sixth)
        root * 27.0 / 16.0,   // A (Pythagorean major sixth)
        root * 16.0 / 9.0,    // A# (Pythagorean minor seventh)
        root * 243.0 / 128.0, // B (Pythagorean major seventh)
        root * 2.0,           // C (octave)
    ]
}

// Quarter-tone helpers (24-TET)

/// Calculate a quarter-tone sharp from a base frequency
pub fn quarter_sharp(freq: f32) -> f32 {
    edo24_step(freq, 1)
}

/// Calculate a half-sharp (standard sharp) from a base frequency
pub fn half_sharp(freq: f32) -> f32 {
    edo24_step(freq, 2)
}

/// Calculate a three-quarter sharp from a base frequency
pub fn three_quarter_sharp(freq: f32) -> f32 {
    edo24_step(freq, 3)
}

/// Calculate a quarter-tone flat from a base frequency
pub fn quarter_flat(freq: f32) -> f32 {
    edo24_step(freq, -1)
}

/// Calculate a half-flat (standard flat) from a base frequency
pub fn half_flat(freq: f32) -> f32 {
    edo24_step(freq, -2)
}

/// Calculate a three-quarter flat from a base frequency
pub fn three_quarter_flat(freq: f32) -> f32 {
    edo24_step(freq, -3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edo12_equals_standard_tuning() {
        let base = 440.0; // A4
        let edo12 = Edo::new(12);

        // One octave up should double frequency
        assert!((edo12.step(base, 12) - 880.0).abs() < 0.01);

        // Perfect fifth (7 semitones) should be close to 3:2 ratio
        let fifth = edo12.step(base, 7);
        let just_fifth = base * 1.5;
        assert!((fifth - just_fifth).abs() < 2.0);
    }

    #[test]
    fn test_edo24_quarter_tones() {
        let base = 440.0;

        // 2 steps in 24-TET = 1 step in 12-TET (semitone)
        let semitone_12 = base * 2.0_f32.powf(1.0 / 12.0);
        let semitone_24 = edo24_step(base, 2);
        assert!((semitone_12 - semitone_24).abs() < 0.01);

        // Quarter tone should be between unison and semitone
        let quarter = edo24_step(base, 1);
        assert!(quarter > base && quarter < semitone_24);
    }

    #[test]
    fn test_cents_conversion() {
        // Perfect octave
        assert!((cents_to_ratio(1200.0) - 2.0).abs() < 0.0001);
        assert!((ratio_to_cents(2.0) - 1200.0).abs() < 0.01);

        // Perfect fifth (~702 cents)
        let fifth_ratio = 1.5;
        let cents = ratio_to_cents(fifth_ratio);
        assert!((cents - 701.955).abs() < 0.01);

        // Round trip
        let original = 1.25;
        let cents = ratio_to_cents(original);
        let back = cents_to_ratio(cents);
        assert!((original - back).abs() < 0.0001);
    }

    #[test]
    fn test_just_intonation() {
        let base = 440.0;

        // Perfect fifth
        let fifth = just_ratio(base, 3, 2);
        assert!((fifth - 660.0).abs() < 0.01);

        // Major third
        let third = just_ratio(base, 5, 4);
        assert!((third - 550.0).abs() < 0.01);

        // Octave
        let octave = just_ratio(base, 2, 1);
        assert!((octave - 880.0).abs() < 0.01);
    }

    #[test]
    fn test_just_major_scale() {
        let scale = just_major_scale(440.0);

        // Should have 8 notes (including octave)
        assert_eq!(scale.len(), 8);

        // First note is root
        assert_eq!(scale[0], 440.0);

        // Last note is octave
        assert_eq!(scale[7], 880.0);

        // All notes should be in ascending order
        for i in 0..scale.len() - 1 {
            assert!(scale[i] < scale[i + 1]);
        }
    }

    #[test]
    fn test_pythagorean_scale() {
        let scale = pythagorean_scale(440.0);

        // Should have 13 notes (chromatic including octave)
        assert_eq!(scale.len(), 13);

        // Perfect fifth (G, index 7) should be exact 3:2 ratio
        assert_eq!(scale[7], 440.0 * 1.5);

        // Perfect fourth (F, index 5) should be exact 4:3 ratio
        assert_eq!(scale[5], 440.0 * 4.0 / 3.0);
    }

    #[test]
    fn test_quarter_tone_helpers() {
        let base = 440.0;

        let qs = quarter_sharp(base);
        let hs = half_sharp(base);

        // Quarter sharp should be less than half sharp
        assert!(qs < hs);
        assert!(qs > base);

        // Half sharp is 2 quarter tones
        assert!((hs - edo24_step(base, 2)).abs() < 0.01);
    }

    #[test]
    fn test_edo_octave() {
        let edo19 = Edo::new(19);
        let octave = edo19.octave(440.0);

        // Should have 19 notes
        assert_eq!(octave.len(), 19);

        // All notes should be ascending
        for i in 0..octave.len() - 1 {
            assert!(octave[i] < octave[i + 1]);
        }

        // Last note should be close to (but not equal to) the octave
        // In 19-EDO, we need 19 steps to reach the octave
        let next_octave = edo19.step(440.0, 19);
        assert!((next_octave - 880.0).abs() < 0.01);
    }
}
