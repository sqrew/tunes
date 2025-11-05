/// Generate harmonic (overtone) series from a fundamental frequency
///
/// The harmonic series is the foundation of musical timbre and consists of integer multiples
/// of a fundamental frequency. This is what makes instruments sound different from each other -
/// they emphasize different harmonics in their overtone spectrum.
///
/// The series follows: f, 2f, 3f, 4f, 5f, 6f, 7f, 8f...
///
/// Musically, these correspond to:
/// - 1st harmonic: fundamental (unison)
/// - 2nd harmonic: octave above
/// - 3rd harmonic: perfect fifth above octave
/// - 4th harmonic: second octave
/// - 5th harmonic: major third (slightly sharp)
/// - 6th harmonic: perfect fifth above second octave
/// - 7th harmonic: minor seventh (very flat - "blue note")
/// - 8th harmonic: third octave
///
/// # Arguments
/// * `fundamental` - The base frequency in Hz (e.g., 110.0 for A2)
/// * `n` - Number of harmonics to generate (including the fundamental)
///
/// # Returns
/// Vector of frequencies representing the first n harmonics
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Generate 8 harmonics of A2 (110 Hz)
/// let harmonics = sequences::harmonic_series(110.0, 8);
/// assert_eq!(harmonics, vec![110.0, 220.0, 330.0, 440.0, 550.0, 660.0, 770.0, 880.0]);
///
/// // Use for additive synthesis - build a sawtooth from harmonics:
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let harmonics = sequences::harmonic_series(220.0, 16);
/// comp.track("sawtooth").note(&harmonics, 1.0);
///
/// // Spectral music - play harmonics as a chord cluster:
/// let partials = sequences::harmonic_series(55.0, 12);  // A1 and its first 12 partials
/// comp.track("spectral").note(&partials, 4.0);
/// ```
///
/// # Musical Applications
/// - **Additive synthesis**: Build complex timbres from sine waves at harmonic frequencies
/// - **Spectral music**: Compose using the natural harmonic spectrum as source material
/// - **Chord voicings**: Harmonics 4-5-6 create major triad, 4-5-6-7 creates dom7 chord
/// - **Just intonation**: Natural harmonics represent pure integer ratios
/// - **Timbre analysis**: Understanding instrument tone color through harmonic content
pub fn harmonic_series(fundamental: f32, n: usize) -> Vec<f32> {
    (1..=n).map(|i| fundamental * i as f32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmonic_series() {
        let harmonics = harmonic_series(110.0, 8);
        assert_eq!(harmonics.len(), 8);
        assert_eq!(
            harmonics,
            vec![110.0, 220.0, 330.0, 440.0, 550.0, 660.0, 770.0, 880.0]
        );
    }

    #[test]
    fn test_harmonic_series_single() {
        let h = harmonic_series(440.0, 1);
        assert_eq!(h, vec![440.0]);
    }

    #[test]
    fn test_harmonic_series_ratios() {
        // Verify octave relationships
        let h = harmonic_series(55.0, 16);

        // Harmonics 1, 2, 4, 8, 16 are octaves
        assert_eq!(h[1], h[0] * 2.0); // Octave relationship
        assert_eq!(h[3], h[0] * 4.0); // Two octaves
        assert_eq!(h[7], h[0] * 8.0); // Three octaves
        assert_eq!(h[15], h[0] * 16.0); // Four octaves

        // Harmonics 4-5-6 form a major triad
        // h[3] (4th harmonic) = root
        // h[4] (5th harmonic) = major third above
        // h[5] (6th harmonic) = perfect fifth above
        let ratio_maj3 = h[4] / h[3]; // 5/4 = 1.25 (major third)
        let ratio_p5 = h[5] / h[3]; // 6/4 = 1.5 (perfect fifth)
        assert!((ratio_maj3 - 1.25).abs() < 0.001);
        assert!((ratio_p5 - 1.5).abs() < 0.001);
    }
}
