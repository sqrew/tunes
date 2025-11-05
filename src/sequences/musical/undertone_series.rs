//! Undertone series - the harmonic mirror
//!
//! The undertone series is the mathematical mirror of the overtone/harmonic series.
//! While overtones multiply the fundamental (1, 2, 3, 4...), undertones divide it (1, 1/2, 1/3, 1/4...).
//!
//! In musical terms, this creates intervals *below* the fundamental frequency.
//! The undertone series has a darker, more mysterious sound compared to the bright overtone series.
//!
//! # Musical Application
//! - Creates descending harmonic patterns
//! - Useful for bass lines and subharmonic textures
//! - Can be inverted to create rising patterns with harmonic relationships
//! - Pair with `map_to_scale()` to create harmonic melodies in any scale
//!
//! # Example
//! ```
//! use tunes::sequences::undertone_series;
//!
//! // Generate first 8 undertone ratios
//! let undertones = undertone_series(8);
//! // undertones = [1.0, 0.5, 0.333..., 0.25, 0.2, 0.166..., 0.142..., 0.125]
//!
//! // Apply to a fundamental frequency for actual notes
//! let fundamental = 440.0; // A4
//! let frequencies: Vec<f32> = undertones.iter()
//!     .map(|&ratio| fundamental * ratio)
//!     .collect();
//! // Creates descending harmonic series: A4, A3, E3, A2, etc.
//! ```

/// Generate the undertone series up to n terms
///
/// The undertone series consists of ratios 1/1, 1/2, 1/3, 1/4, ...
/// These represent divisions of the fundamental frequency.
///
/// # Arguments
/// * `n` - Number of undertones to generate
///
/// # Returns
/// Vector of undertone ratios from 1/1 down to 1/n
///
/// # Example
/// ```
/// use tunes::sequences::undertone_series;
///
/// let undertones = undertone_series(5);
/// assert_eq!(undertones.len(), 5);
/// assert_eq!(undertones[0], 1.0);      // Fundamental (1/1)
/// assert_eq!(undertones[1], 0.5);      // Octave below (1/2)
/// assert_eq!(undertones[2], 1.0/3.0);  // Perfect twelfth below (1/3)
/// ```
pub fn undertone_series(n: usize) -> Vec<f32> {
    (1..=n).map(|i| 1.0 / i as f32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undertone_series_basic() {
        let undertones = undertone_series(4);
        assert_eq!(undertones.len(), 4);
        assert_eq!(undertones[0], 1.0);
        assert_eq!(undertones[1], 0.5);
        assert!((undertones[2] - 0.333333).abs() < 0.001);
        assert_eq!(undertones[3], 0.25);
    }

    #[test]
    fn test_undertone_series_descending() {
        let undertones = undertone_series(8);
        // Should be strictly descending
        for i in 0..undertones.len() - 1 {
            assert!(undertones[i] > undertones[i + 1]);
        }
    }

    #[test]
    fn test_undertone_series_single() {
        let undertones = undertone_series(1);
        assert_eq!(undertones.len(), 1);
        assert_eq!(undertones[0], 1.0);
    }

    #[test]
    fn test_undertone_series_ratios() {
        let undertones = undertone_series(6);
        // Check specific harmonic relationships
        assert_eq!(undertones[0], 1.0);           // Fundamental
        assert_eq!(undertones[1], 0.5);           // Octave
        assert!((undertones[3] - 0.25).abs() < 0.0001);  // Two octaves
        assert!((undertones[5] - 1.0/6.0).abs() < 0.0001);
    }
}
