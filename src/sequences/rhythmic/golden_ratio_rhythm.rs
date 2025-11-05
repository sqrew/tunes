/// Generate golden ratio rhythm pattern (Wythoff's sequence / Beatty sequence)
///
/// Uses the golden ratio to distribute beats evenly but non-periodically across steps.
/// This creates a rhythm that is neither regular nor random - it has structure but
/// never exactly repeats (until the full cycle).
///
/// The pattern is generated using the Beatty sequence: floor((n+1) * φ) for each step,
/// producing a binary sequence of 0s and 1s (rests and hits) with golden ratio spacing.
///
/// This is related to the Sturmian sequence and appears in:
/// - Phyllotaxis (leaf arrangement on stems)
/// - Musical canons and rounds
/// - Minimalist composition (Steve Reich, Philip Glass)
///
/// # Arguments
/// * `steps` - Number of steps in the rhythm pattern
///
/// # Returns
/// Vector of step indices where hits occur (like Euclidean rhythms)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let pattern = sequences::golden_ratio_rhythm(16);
/// // Creates a non-repeating, naturally flowing rhythm over 16 steps
///
/// // Use with drum_grid:
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let phi_rhythm = sequences::golden_ratio_rhythm(32);
/// comp.track("phi_drums")
///     .drum_grid(32, 0.125)
///     .kick(&phi_rhythm);
/// ```
///
/// # Properties
/// - **Non-periodic**: Pattern doesn't repeat (maximally even distribution)
/// - **Self-similar**: Zooming in/out reveals similar structure
/// - **Balanced**: Neither too sparse nor too dense
/// - **Organic**: Sounds natural, not mechanical
pub fn golden_ratio_rhythm(steps: usize) -> Vec<usize> {
    const PHI: f32 = 1.618033988749;

    (0..steps)
        .filter(|&i| {
            // Check if this position gets a beat using the lower Wythoff sequence
            let floor_current = ((i + 1) as f32 / PHI).floor() as usize;
            let floor_previous = (i as f32 / PHI).floor() as usize;
            floor_current != floor_previous
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_ratio_rhythm() {
        let pattern = golden_ratio_rhythm(16);

        // Should have hits but not all steps
        assert!(!pattern.is_empty());
        assert!(pattern.len() < 16);

        // Hits should be in ascending order
        for i in 1..pattern.len() {
            assert!(pattern[i] > pattern[i - 1]);
        }

        // All indices should be valid (< 16)
        for &idx in &pattern {
            assert!(idx < 16);
        }

        // Verify some expected properties
        // Golden ratio rhythm should have around 10 hits in 16 steps (16/φ ≈ 9.9)
        assert!(pattern.len() >= 8 && pattern.len() <= 11);
    }

    #[test]
    fn test_golden_ratio_rhythm_properties() {
        // Golden ratio rhythm has interesting mathematical properties
        let pattern = golden_ratio_rhythm(100);

        // The ratio of hits to total steps should approach 1/φ ≈ 0.618
        let ratio = pattern.len() as f32 / 100.0;
        let expected = 1.0 / 1.618033988749;
        assert!((ratio - expected).abs() < 0.05); // Within 5%
    }
}
