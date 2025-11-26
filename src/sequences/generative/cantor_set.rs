/// Cantor Set - Fractal pattern generator for rhythmic structures
///
/// The Cantor set is created by recursively removing the middle third of line segments.
/// This creates a self-similar fractal pattern that's excellent for generating
/// interesting rhythmic subdivisions.
///
/// # Musical Applications
/// - **Rhythmic patterns**: Use the kept segments as hit positions
/// - **Time-based effects**: Apply effects only during "kept" time regions
/// - **Polyrhythmic structures**: Combine different iteration depths
///
/// # Arguments
/// * `iterations` - How many times to subdivide (0 = full line, 1 = remove middle third, etc.)
/// * `resolution` - How many time points to check (higher = more precise)
///
/// # Returns
/// A vector of 0s and 1s where 1 indicates the point is in the Cantor set
///
/// # Example
/// ```
/// use tunes::sequences::cantor_set::generate;
///
/// // Create a Cantor set rhythm pattern
/// let pattern = generate(2, 27); // 27 = 3^3 for clean divisions
/// // iteration 0: [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
/// // iteration 1: [1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1]
/// // iteration 2: [1,1,1,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,1,1,1]
///
/// // Convert to rhythm hit positions
/// let hits: Vec<usize> = pattern.iter()
///     .enumerate()
///     .filter(|(_, &v)| v == 1)
///     .map(|(i, _)| i)
///     .collect();
/// ```
pub fn generate(iterations: usize, resolution: usize) -> Vec<u32> {
    if resolution == 0 {
        return vec![];
    }

    // Start with all points in the set
    let mut set = vec![1u32; resolution];

    if iterations == 0 {
        return set;
    }

    // Track which segments are active (not yet removed)
    let mut segments = vec![(0, resolution)]; // (start, end)

    // For each iteration, split each segment and remove middle thirds
    for _ in 0..iterations {
        let mut new_segments = Vec::new();

        for (start, end) in segments {
            let len = end - start;
            if len < 3 {
                // Can't subdivide further, keep as is
                new_segments.push((start, end));
                continue;
            }

            let third = len / 3;

            // Keep first third
            new_segments.push((start, start + third));

            // Remove middle third
            for val in set[(start + third)..(start + third * 2)].iter_mut() {
                *val = 0;
            }

            // Keep last third
            new_segments.push((start + third * 2, end));
        }

        segments = new_segments;
    }

    set
}

// ============================================================================
// PRESETS - Ready-to-use Cantor set configurations
// ============================================================================

/// 16-step Cantor rhythm pattern (2 iterations)
///
/// Creates a fractal rhythm pattern over 16 steps, perfect for
/// standard 4/4 time signatures. Returns hit positions as indices.
///
/// # Returns
/// Vec of step indices where hits occur
///
/// # Example
/// ```
/// use tunes::sequences::cantor_rhythm_16;
///
/// let hits = cantor_rhythm_16();
/// // Use in drum grid:
/// // comp.track("drums").drum_grid(16, 0.125).kick(&hits);
/// ```
pub fn cantor_rhythm_16() -> Vec<usize> {
    generate(2, 16)
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect()
}

/// 27-step Cantor rhythm pattern (3 iterations)
///
/// Creates a more detailed fractal rhythm over 27 steps (3^3).
/// Perfect for clean Cantor set divisions. Returns hit positions.
///
/// # Returns
/// Vec of step indices where hits occur
///
/// # Example
/// ```
/// use tunes::sequences::cantor_rhythm_27;
///
/// let hits = cantor_rhythm_27();
/// assert_eq!(hits.len(), 8); // 2^3 = 8 hits remain after 3 iterations
/// ```
pub fn cantor_rhythm_27() -> Vec<usize> {
    generate(3, 27)
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect()
}

/// 9-step Cantor rhythm pattern (1 iteration)
///
/// Simple Cantor pattern with just one iteration over 9 steps.
/// Creates a basic "3-gap-3" pattern. Good for triplet-based rhythms.
///
/// # Returns
/// Vec of step indices where hits occur
///
/// # Example
/// ```
/// use tunes::sequences::cantor_rhythm_9;
///
/// let hits = cantor_rhythm_9();
/// assert_eq!(hits, vec![0, 1, 2, 6, 7, 8]); // First and last thirds
/// ```
pub fn cantor_rhythm_9() -> Vec<usize> {
    generate(1, 9)
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect()
}

/// 81-step Cantor rhythm pattern (4 iterations)
///
/// Highly detailed fractal rhythm over 81 steps (3^4).
/// Creates complex, self-similar rhythmic structures.
///
/// # Returns
/// Vec of step indices where hits occur
///
/// # Example
/// ```
/// use tunes::sequences::cantor_rhythm_81;
///
/// let hits = cantor_rhythm_81();
/// assert_eq!(hits.len(), 16); // 2^4 = 16 hits remain after 4 iterations
/// ```
pub fn cantor_rhythm_81() -> Vec<usize> {
    generate(4, 81)
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantor_set_basic() {
        let set0 = generate(0, 9);
        assert_eq!(set0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);

        let set1 = generate(1, 9);
        assert_eq!(set1, vec![1, 1, 1, 0, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn test_cantor_set_edge_cases() {
        let empty = generate(3, 0);
        assert_eq!(empty, Vec::<u32>::new());

        let single = generate(2, 1);
        assert_eq!(single, vec![1]);
    }

    #[test]
    fn test_cantor_set_iteration_2() {
        // With 27 points (3^3), we can cleanly divide
        let set = generate(2, 27);

        // First third (0-8): keep first and last thirds, remove middle
        assert_eq!(set[0], 1); // First point kept
        assert_eq!(set[1], 1);
        assert_eq!(set[2], 1);
        assert_eq!(set[3], 0); // Middle third removed
        assert_eq!(set[4], 0);
        assert_eq!(set[5], 0);
        assert_eq!(set[6], 1); // Last third kept
        assert_eq!(set[7], 1);
        assert_eq!(set[8], 1);

        // Middle third (9-17): all removed
        for i in 9..18 {
            assert_eq!(set[i], 0, "Middle third should be removed at position {}", i);
        }

        // Last third (18-26): keep first and last thirds, remove middle
        assert_eq!(set[18], 1);
        assert_eq!(set[19], 1);
        assert_eq!(set[20], 1);
        assert_eq!(set[21], 0);
        assert_eq!(set[22], 0);
        assert_eq!(set[23], 0);
        assert_eq!(set[24], 1);
        assert_eq!(set[25], 1);
        assert_eq!(set[26], 1);
    }

    #[test]
    fn test_cantor_set_properties() {
        // As iterations increase, fewer points remain
        let set1 = generate(1, 27);
        let set2 = generate(2, 27);
        let set3 = generate(3, 27);

        let count1: u32 = set1.iter().sum();
        let count2: u32 = set2.iter().sum();
        let count3: u32 = set3.iter().sum();

        // Each iteration should have fewer or equal points
        assert!(count2 <= count1);
        assert!(count3 <= count2);

        // Mathematically, Cantor set has 2^n/3^n density
        // After 2 iterations: (2/3)^2 = 4/9 of points remain
        let expected_2 = (27.0 * (2.0_f32 / 3.0_f32).powi(2)) as u32;
        assert!(
            (count2 as i32 - expected_2 as i32).abs() <= 2,
            "Expected ~{}, got {}",
            expected_2,
            count2
        );
    }
}
