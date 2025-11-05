//! Pythagorean tuning - pure fifth-based tuning system
//!
//! Pythagorean tuning is one of the oldest tuning systems, based on stacking perfect fifths (3:2 ratio).
//! Starting from a fundamental frequency and repeatedly multiplying by 3/2, then reducing to one octave,
//! you generate all 12 chromatic notes with mathematically pure fifths.
//!
//! This tuning system was fundamental to Western music theory and provides a different color
//! than equal temperament - some intervals are purer, others have more "bite."
//!
//! # Musical Application
//! - Historical/period music composition
//! - Exploring alternative tuning systems
//! - Creating harmonic sequences with pure fifths
//! - Microtonality and tuning experiments
//! - Pair with `map_to_scale()` to quantize to musical scales
//!
//! # Theory
//! Perfect fifth ratio: 3:2 = 1.5
//! Starting from 1.0, each successive fifth: 1.0 → 1.5 → 2.25 → 3.375 → ...
//! Reduced to one octave (divide by 2 until in range [1.0, 2.0))
//!
//! # Example
//! ```
//! use tunes::sequences::pythagorean_tuning;
//!
//! // Generate 12-note Pythagorean chromatic scale
//! let tuning = pythagorean_tuning(12);
//! // Returns frequency ratios for all 12 chromatic notes
//!
//! // Use with a fundamental frequency
//! let fundamental = 440.0; // A4
//! let frequencies: Vec<f32> = tuning.iter()
//!     .map(|&ratio| fundamental * ratio)
//!     .collect();
//! ```

/// Generate Pythagorean tuning by stacking perfect fifths
///
/// Creates a tuning system by stacking perfect fifths (3:2 ratio) and reducing to one octave.
/// Returns frequency ratios sorted in ascending order within one octave [1.0, 2.0).
///
/// # Arguments
/// * `n` - Number of notes to generate (typically 12 for chromatic scale)
///
/// # Returns
/// Vector of frequency ratios representing Pythagorean tuning, sorted ascending
///
/// # Example
/// ```
/// use tunes::sequences::pythagorean_tuning;
///
/// // Generate 12-note Pythagorean chromatic scale
/// let tuning = pythagorean_tuning(12);
/// assert_eq!(tuning.len(), 12);
/// assert_eq!(tuning[0], 1.0); // Fundamental
///
/// // Each interval is built from stacking perfect fifths
/// // Compare to equal temperament for different harmonic colors
/// ```
pub fn pythagorean_tuning(n: usize) -> Vec<f32> {
    let mut ratios = Vec::with_capacity(n);
    let fifth: f32 = 3.0 / 2.0; // Perfect fifth ratio

    // Generate n notes by stacking fifths
    for i in 0..n {
        let mut ratio = fifth.powi(i as i32);

        // Reduce to one octave [1.0, 2.0)
        while ratio >= 2.0 {
            ratio /= 2.0;
        }
        while ratio < 1.0 {
            ratio *= 2.0;
        }

        ratios.push(ratio);
    }

    // Sort ratios in ascending order for musical use
    ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());

    ratios
}

/// Generate just intonation major scale
///
/// Just intonation uses simple integer ratios for pure harmonic intervals.
/// This generates a 7-note major scale with the classic ratios:
/// 1/1, 9/8, 5/4, 4/3, 3/2, 5/3, 15/8, 2/1
///
/// # Returns
/// Vector of frequency ratios representing just intonation major scale
///
/// # Example
/// ```
/// use tunes::sequences::just_intonation_major;
///
/// let scale = just_intonation_major();
/// assert_eq!(scale.len(), 8); // 8 notes including octave
/// assert_eq!(scale[0], 1.0);  // Tonic
/// assert_eq!(scale[7], 2.0);  // Octave
/// ```
pub fn just_intonation_major() -> Vec<f32> {
    vec![
        1.0,      // Tonic (1/1)
        9.0/8.0,  // Major second (9/8)
        5.0/4.0,  // Major third (5/4)
        4.0/3.0,  // Perfect fourth (4/3)
        3.0/2.0,  // Perfect fifth (3/2)
        5.0/3.0,  // Major sixth (5/3)
        15.0/8.0, // Major seventh (15/8)
        2.0,      // Octave (2/1)
    ]
}

/// Generate just intonation minor scale
///
/// Just intonation minor scale using simple integer ratios:
/// 1/1, 9/8, 6/5, 4/3, 3/2, 8/5, 9/5, 2/1
///
/// # Returns
/// Vector of frequency ratios representing just intonation minor scale
///
/// # Example
/// ```
/// use tunes::sequences::just_intonation_minor;
///
/// let scale = just_intonation_minor();
/// assert_eq!(scale.len(), 8); // 8 notes including octave
/// assert_eq!(scale[0], 1.0);  // Tonic
/// assert_eq!(scale[2], 6.0/5.0); // Minor third
/// ```
pub fn just_intonation_minor() -> Vec<f32> {
    vec![
        1.0,     // Tonic (1/1)
        9.0/8.0, // Major second (9/8)
        6.0/5.0, // Minor third (6/5)
        4.0/3.0, // Perfect fourth (4/3)
        3.0/2.0, // Perfect fifth (3/2)
        8.0/5.0, // Minor sixth (8/5)
        9.0/5.0, // Minor seventh (9/5)
        2.0,     // Octave (2/1)
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pythagorean_tuning_12_notes() {
        let tuning = pythagorean_tuning(12);
        assert_eq!(tuning.len(), 12);

        // First note should be fundamental
        assert!((tuning[0] - 1.0).abs() < 0.0001);

        // All ratios should be within one octave
        for &ratio in &tuning {
            assert!(ratio >= 1.0 && ratio < 2.0);
        }

        // Should be sorted ascending
        for i in 0..tuning.len() - 1 {
            assert!(tuning[i] <= tuning[i + 1]);
        }
    }

    #[test]
    fn test_pythagorean_tuning_has_pure_fifth() {
        let tuning = pythagorean_tuning(12);

        // The fifth note in the cycle should be close to 3/2 = 1.5
        // But after sorting it may be in a different position
        let has_pure_fifth = tuning.iter().any(|&r| (r - 1.5).abs() < 0.0001);
        assert!(has_pure_fifth, "Pythagorean tuning should contain a pure fifth (3/2)");
    }

    #[test]
    fn test_pythagorean_tuning_single_note() {
        let tuning = pythagorean_tuning(1);
        assert_eq!(tuning.len(), 1);
        assert!((tuning[0] - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_pythagorean_tuning_different_sizes() {
        let tuning_5 = pythagorean_tuning(5);
        assert_eq!(tuning_5.len(), 5);

        let tuning_7 = pythagorean_tuning(7);
        assert_eq!(tuning_7.len(), 7);

        // All notes should be unique (after sorting, no duplicates)
        for i in 0..tuning_7.len() - 1 {
            assert!(tuning_7[i] < tuning_7[i + 1], "Notes should be unique");
        }
    }

    #[test]
    fn test_just_intonation_major() {
        let scale = just_intonation_major();
        assert_eq!(scale.len(), 8);

        // Check specific ratios
        assert_eq!(scale[0], 1.0);        // Tonic
        assert_eq!(scale[1], 9.0/8.0);    // Major second
        assert_eq!(scale[2], 5.0/4.0);    // Major third
        assert_eq!(scale[3], 4.0/3.0);    // Perfect fourth
        assert_eq!(scale[4], 3.0/2.0);    // Perfect fifth
        assert_eq!(scale[5], 5.0/3.0);    // Major sixth
        assert_eq!(scale[6], 15.0/8.0);   // Major seventh
        assert_eq!(scale[7], 2.0);        // Octave
    }

    #[test]
    fn test_just_intonation_minor() {
        let scale = just_intonation_minor();
        assert_eq!(scale.len(), 8);

        // Check specific ratios
        assert_eq!(scale[0], 1.0);        // Tonic
        assert_eq!(scale[2], 6.0/5.0);    // Minor third (characteristic of minor)
        assert_eq!(scale[5], 8.0/5.0);    // Minor sixth
        assert_eq!(scale[6], 9.0/5.0);    // Minor seventh
        assert_eq!(scale[7], 2.0);        // Octave
    }

    #[test]
    fn test_just_intonation_scales_ascending() {
        let major = just_intonation_major();
        let minor = just_intonation_minor();

        // Both should be ascending
        for i in 0..major.len() - 1 {
            assert!(major[i] < major[i + 1]);
        }
        for i in 0..minor.len() - 1 {
            assert!(minor[i] < minor[i + 1]);
        }
    }

    #[test]
    fn test_just_intonation_pure_intervals() {
        let major = just_intonation_major();

        // Perfect fifth should be exactly 3/2
        assert_eq!(major[4], 3.0/2.0);

        // Perfect fourth should be exactly 4/3
        assert_eq!(major[3], 4.0/3.0);

        // Major third should be exactly 5/4 (purer than equal temperament)
        assert_eq!(major[2], 5.0/4.0);
    }
}
