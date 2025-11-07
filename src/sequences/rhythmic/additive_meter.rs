/// Generate additive meter patterns - asymmetric rhythms from Balkans, Middle East, and beyond
///
/// Additive meters (also called asymmetric or irregular meters) are time signatures that
/// combine groups of 2s and 3s to create complex, dance-like rhythms. Unlike Western
/// "compound" meters that evenly subdivide (4/4, 6/8), additive meters create intentional
/// asymmetry that drives forward motion.
///
/// # Cultural Context
///
/// These meters are fundamental to:
/// - **Bulgarian music**: "Kopanitsa" (11/8 as 2+2+3+2+2), "Rachenitsa" (7/8 as 2+2+3)
/// - **Greek music**: "Kalamatianos" (7/8), "Hasapiko" (9/8)
/// - **Turkish music**: "Aksak" rhythms (9/8, 7/8, 5/8)
/// - **Middle Eastern**: Various odd meters in Arabic and Persian music
/// - **Progressive rock**: Tool, Dream Theater, King Crimson use these extensively
///
/// # Musical Character
///
/// The asymmetry creates:
/// - **Forward momentum**: The irregular stress pattern propels the rhythm
/// - **Dance grooves**: Despite being "odd", they're highly danceable
/// - **Sophisticated feel**: More complex than straight 4/4 or 6/8
/// - **Ethnic authenticity**: Essential for world music styles
///
/// # Common Patterns
///
/// The groupings determine where the strong beats fall:
/// - `[2, 2, 3]` - Bulgarian "quick-quick-slow" (7/8 Rachenitsa)
/// - `[3, 2, 2]` - Rotated version with slow beat first
/// - `[2, 2, 2, 3]` - 9/8 pattern (common in Greek music)
/// - `[2, 3, 2]` - 7/8 with emphasis in middle
/// - `[3, 3, 2]` - 8/8 but grouped asymmetrically
/// - `[2, 2, 3, 2, 2]` - 11/8 "Kopanitsa" pattern
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Bulgarian Rachenitsa: 2+2+3 = 7/8
/// let pattern = sequences::additive_meter(&[2, 2, 3]);
/// assert_eq!(pattern, vec![0, 2, 4]); // Strong beats at 0, 2, 4
///
/// // Greek pattern: 2+2+2+3 = 9/8
/// let pattern = sequences::additive_meter(&[2, 2, 2, 3]);
/// assert_eq!(pattern, vec![0, 2, 4, 6]);
///
/// // Kopanitsa: 2+2+3+2+2 = 11/8
/// let pattern = sequences::additive_meter(&[2, 2, 3, 2, 2]);
/// assert_eq!(pattern, vec![0, 2, 4, 7, 9]);
///
/// // Use in composition:
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// // 7/8 Bulgarian pattern with kick on strong beats
/// comp.track("bulgarian")
///     .drum_grid(7, 0.125)
///     .kick(&sequences::additive_meter(&[2, 2, 3]));
/// ```
///
/// # References
/// - "Bulgarian Rhythm" by Rumen Shopov
/// - "The Metric Matrix" by Jeff Pressing
/// - Béla Bartók's ethnomusicological work on Bulgarian rhythm

/// Generate an additive meter pattern from groupings of 2s and 3s
///
/// Takes subdivisions (groups of 2s and 3s) and returns the step indices
/// where strong beats occur. Each group creates one accent at its start.
///
/// # Arguments
/// * `groupings` - Slice of group sizes (typically 2s and 3s)
///
/// # Returns
/// Vec of step indices where accents/strong beats occur
///
/// # Examples
/// ```
/// use tunes::sequences::additive_meter;
///
/// // 7/8 as 2+2+3 (Bulgarian Rachenitsa)
/// let pattern = additive_meter(&[2, 2, 3]);
/// assert_eq!(pattern, vec![0, 2, 4]);
///
/// // 5/8 as 3+2
/// let pattern = additive_meter(&[3, 2]);
/// assert_eq!(pattern, vec![0, 3]);
///
/// // 11/8 as 2+2+3+2+2
/// let pattern = additive_meter(&[2, 2, 3, 2, 2]);
/// assert_eq!(pattern, vec![0, 2, 4, 7, 9]);
/// ```
pub fn additive_meter(groupings: &[usize]) -> Vec<usize> {
    if groupings.is_empty() {
        return vec![];
    }

    let mut accents = Vec::with_capacity(groupings.len());
    let mut position = 0;

    for &group_size in groupings {
        accents.push(position);
        position += group_size;
    }

    accents
}

/// Bulgarian Rachenitsa: 7/8 as 2+2+3 (quick-quick-slow)
///
/// The most common Bulgarian dance rhythm. Creates a distinctive
/// "short-short-long" feel that's highly danceable despite being in 7.
///
/// # Returns
/// Vec of accent positions: [0, 2, 4]
///
/// # Example
/// ```
/// use tunes::sequences::rachenitsa;
///
/// let pattern = rachenitsa();
/// assert_eq!(pattern, vec![0, 2, 4]);
/// // Total length: 2+2+3 = 7 steps
/// ```
pub fn rachenitsa() -> Vec<usize> {
    additive_meter(&[2, 2, 3])
}

/// Bulgarian Kopanitsa: 11/8 as 2+2+3+2+2
///
/// A complex Bulgarian dance rhythm. The 3 in the middle creates
/// an off-balance feel that drives the dance forward.
///
/// # Returns
/// Vec of accent positions: [0, 2, 4, 7, 9]
///
/// # Example
/// ```
/// use tunes::sequences::kopanitsa;
///
/// let pattern = kopanitsa();
/// assert_eq!(pattern, vec![0, 2, 4, 7, 9]);
/// // Total length: 2+2+3+2+2 = 11 steps
/// ```
pub fn kopanitsa() -> Vec<usize> {
    additive_meter(&[2, 2, 3, 2, 2])
}

/// Greek Kalamatianos: 7/8 as 3+2+2
///
/// Popular Greek dance rhythm. Different feel from Bulgarian 7/8
/// due to the long beat coming first.
///
/// # Returns
/// Vec of accent positions: [0, 3, 5]
///
/// # Example
/// ```
/// use tunes::sequences::kalamatianos;
///
/// let pattern = kalamatianos();
/// assert_eq!(pattern, vec![0, 3, 5]);
/// // Total length: 3+2+2 = 7 steps
/// ```
pub fn kalamatianos() -> Vec<usize> {
    additive_meter(&[3, 2, 2])
}

/// Turkish/Greek 9/8: 2+2+2+3
///
/// Common in Turkish and Greek music. The "slow" beat at the end
/// creates a distinctive lilt.
///
/// # Returns
/// Vec of accent positions: [0, 2, 4, 6]
///
/// # Example
/// ```
/// use tunes::sequences::aksak_9_8;
///
/// let pattern = aksak_9_8();
/// assert_eq!(pattern, vec![0, 2, 4, 6]);
/// // Total length: 2+2+2+3 = 9 steps
/// ```
pub fn aksak_9_8() -> Vec<usize> {
    additive_meter(&[2, 2, 2, 3])
}

/// Generate all rotations of an additive meter pattern
///
/// Useful for creating variations or finding the rotation that best
/// fits your melodic phrase. Each rotation shifts which grouping
/// comes first while maintaining the same total pattern.
///
/// # Arguments
/// * `groupings` - Slice of group sizes
///
/// # Returns
/// Vec of all rotations, each as a Vec of accent positions
///
/// # Example
/// ```
/// use tunes::sequences::additive_meter_rotations;
///
/// // Get all rotations of 7/8
/// let rotations = additive_meter_rotations(&[2, 2, 3]);
/// assert_eq!(rotations.len(), 3); // [2,2,3], [2,3,2], [3,2,2]
///
/// // First rotation: 2+2+3
/// assert_eq!(rotations[0], vec![0, 2, 4]);
///
/// // Second rotation: 2+3+2
/// assert_eq!(rotations[1], vec![0, 2, 5]);
///
/// // Third rotation: 3+2+2
/// assert_eq!(rotations[2], vec![0, 3, 5]);
/// ```
pub fn additive_meter_rotations(groupings: &[usize]) -> Vec<Vec<usize>> {
    if groupings.is_empty() {
        return vec![];
    }

    let mut rotations = Vec::with_capacity(groupings.len());

    for rotation_idx in 0..groupings.len() {
        let mut rotated = Vec::with_capacity(groupings.len());

        // Build rotated grouping
        for i in 0..groupings.len() {
            let idx = (rotation_idx + i) % groupings.len();
            rotated.push(groupings[idx]);
        }

        rotations.push(additive_meter(&rotated));
    }

    rotations
}

/// Calculate the total length (in steps) of an additive meter
///
/// Sums all the groupings to determine how many steps the full pattern takes.
///
/// # Arguments
/// * `groupings` - Slice of group sizes
///
/// # Returns
/// Total number of steps
///
/// # Example
/// ```
/// use tunes::sequences::additive_meter_length;
///
/// assert_eq!(additive_meter_length(&[2, 2, 3]), 7);
/// assert_eq!(additive_meter_length(&[2, 2, 3, 2, 2]), 11);
/// assert_eq!(additive_meter_length(&[3, 3, 2]), 8);
/// ```
pub fn additive_meter_length(groupings: &[usize]) -> usize {
    groupings.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_additive_meter_basic() {
        let pattern = additive_meter(&[2, 2, 3]);
        assert_eq!(pattern, vec![0, 2, 4]);
    }

    #[test]
    fn test_additive_meter_empty() {
        let pattern = additive_meter(&[]);
        assert_eq!(pattern, vec![]);
    }

    #[test]
    fn test_additive_meter_single_group() {
        let pattern = additive_meter(&[5]);
        assert_eq!(pattern, vec![0]);
    }

    #[test]
    fn test_additive_meter_various_groupings() {
        assert_eq!(additive_meter(&[3, 2]), vec![0, 3]);
        assert_eq!(additive_meter(&[2, 3, 2]), vec![0, 2, 5]);
        assert_eq!(additive_meter(&[3, 3, 2]), vec![0, 3, 6]);
    }

    #[test]
    fn test_rachenitsa() {
        let pattern = rachenitsa();
        assert_eq!(pattern, vec![0, 2, 4]);
        assert_eq!(pattern.len(), 3); // 3 accents
    }

    #[test]
    fn test_kopanitsa() {
        let pattern = kopanitsa();
        assert_eq!(pattern, vec![0, 2, 4, 7, 9]);
        assert_eq!(pattern.len(), 5); // 5 accents
    }

    #[test]
    fn test_kalamatianos() {
        let pattern = kalamatianos();
        assert_eq!(pattern, vec![0, 3, 5]);
        assert_eq!(pattern.len(), 3); // 3 accents
    }

    #[test]
    fn test_aksak_9_8() {
        let pattern = aksak_9_8();
        assert_eq!(pattern, vec![0, 2, 4, 6]);
        assert_eq!(pattern.len(), 4); // 4 accents
    }

    #[test]
    fn test_additive_meter_length() {
        assert_eq!(additive_meter_length(&[2, 2, 3]), 7);
        assert_eq!(additive_meter_length(&[2, 2, 3, 2, 2]), 11);
        assert_eq!(additive_meter_length(&[3, 2, 2]), 7);
        assert_eq!(additive_meter_length(&[2, 2, 2, 3]), 9);
    }

    #[test]
    fn test_additive_meter_length_empty() {
        assert_eq!(additive_meter_length(&[]), 0);
    }

    #[test]
    fn test_additive_meter_rotations() {
        let rotations = additive_meter_rotations(&[2, 2, 3]);
        assert_eq!(rotations.len(), 3);

        // Rotation 0: 2+2+3
        assert_eq!(rotations[0], vec![0, 2, 4]);

        // Rotation 1: 2+3+2
        assert_eq!(rotations[1], vec![0, 2, 5]);

        // Rotation 2: 3+2+2
        assert_eq!(rotations[2], vec![0, 3, 5]);
    }

    #[test]
    fn test_additive_meter_rotations_empty() {
        let rotations = additive_meter_rotations(&[]);
        assert_eq!(rotations, Vec::<Vec<usize>>::new());
    }

    #[test]
    fn test_additive_meter_rotations_single() {
        let rotations = additive_meter_rotations(&[5]);
        assert_eq!(rotations.len(), 1);
        assert_eq!(rotations[0], vec![0]);
    }

    #[test]
    fn test_all_patterns_start_at_zero() {
        assert_eq!(rachenitsa()[0], 0);
        assert_eq!(kopanitsa()[0], 0);
        assert_eq!(kalamatianos()[0], 0);
        assert_eq!(aksak_9_8()[0], 0);
    }

    #[test]
    fn test_patterns_are_sorted() {
        let patterns = vec![rachenitsa(), kopanitsa(), kalamatianos(), aksak_9_8()];

        for pattern in patterns {
            let mut sorted = pattern.clone();
            sorted.sort_unstable();
            assert_eq!(pattern, sorted);
        }
    }

    #[test]
    fn test_rachenitsa_vs_kalamatianos() {
        // Both are 7/8 but different groupings
        let rach = rachenitsa(); // 2+2+3
        let kalam = kalamatianos(); // 3+2+2

        assert_ne!(rach, kalam);
        assert_eq!(rach.len(), kalam.len()); // Same number of accents

        // Different total lengths when computed
        assert_eq!(additive_meter_length(&[2, 2, 3]), 7);
        assert_eq!(additive_meter_length(&[3, 2, 2]), 7);
    }

    #[test]
    fn test_additive_meter_with_larger_groups() {
        let pattern = additive_meter(&[4, 3, 5]);
        assert_eq!(pattern, vec![0, 4, 7]);
        assert_eq!(additive_meter_length(&[4, 3, 5]), 12);
    }

    #[test]
    fn test_additive_meter_all_twos() {
        let pattern = additive_meter(&[2, 2, 2, 2]);
        assert_eq!(pattern, vec![0, 2, 4, 6]);
        // This would just be 4/4 with quarter note accents
    }

    #[test]
    fn test_additive_meter_rotations_kopanitsa() {
        let rotations = additive_meter_rotations(&[2, 2, 3, 2, 2]);
        assert_eq!(rotations.len(), 5);

        // Each rotation should have 5 accents
        for rotation in &rotations {
            assert_eq!(rotation.len(), 5);
        }

        // All should start at 0
        for rotation in &rotations {
            assert_eq!(rotation[0], 0);
        }
    }

    #[test]
    fn test_complex_meter_13_8() {
        let pattern = additive_meter(&[3, 2, 2, 3, 3]);
        assert_eq!(pattern, vec![0, 3, 5, 7, 10]);
        assert_eq!(additive_meter_length(&[3, 2, 2, 3, 3]), 13);
    }
}
