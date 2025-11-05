/// Generate golden section divisions of a value
///
/// Divides a number into two parts according to the golden ratio (a/b = φ),
/// then recursively subdivides to create multiple golden sections.
///
/// This is useful for:
/// - Musical form (dividing piece into sections)
/// - Time-based structures (section durations)
/// - Amplitude scaling (dynamic levels)
///
/// # Arguments
/// * `value` - The value to divide
/// * `divisions` - Number of golden section points to generate
///
/// # Returns
/// Vector of values representing golden section divisions
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Divide 60 seconds into golden sections
/// let sections = sequences::golden_sections(60.0, 5);
/// // Use these as time markers for form: [60.0, 37.08, 22.92, ...]
///
/// // Use for dynamics (0.0 to 1.0)
/// let dynamics = sequences::golden_sections(1.0, 8);
/// // Creates naturally decreasing dynamic levels
/// ```
///
/// # Musical Applications
/// - **Sonata form**: Place development/recapitulation at golden ratio point
/// - **Climax placement**: Put emotional peak at φ proportion (≈61.8% through)
/// - **Phrase lengths**: Natural-feeling asymmetric phrase structures
/// - **Tempo changes**: Scale tempo by golden ratio for smooth transitions
pub fn golden_sections(value: f32, divisions: usize) -> Vec<f32> {
    const PHI: f32 = 1.618033988749;
    let mut sections = vec![value];

    for _ in 0..divisions {
        let last = *sections.last().unwrap();
        sections.push(last / PHI);
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_sections() {
        const PHI: f32 = 1.618033988749;
        let sections = golden_sections(100.0, 4);

        assert_eq!(sections.len(), 5);
        assert_eq!(sections[0], 100.0);

        for i in 1..sections.len() {
            let expected = sections[i - 1] / PHI;
            assert!((sections[i] - expected).abs() < 0.01);
        }
    }

    #[test]
    fn test_golden_sections_zero_divisions() {
        let sections = golden_sections(42.0, 0);
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0], 42.0);
    }

    #[test]
    fn test_golden_sections_single() {
        let sections = golden_sections(60.0, 1);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0], 60.0);

        // Second value should be 60 / φ ≈ 37.08
        let expected = 60.0 / 1.618033988749;
        assert!((sections[1] - expected).abs() < 0.01);
    }
}
