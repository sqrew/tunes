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
/// use tunes::sequences::cantor_set;
///
/// // Create a Cantor set rhythm pattern
/// let pattern = cantor_set(2, 27); // 27 = 3^3 for clean divisions
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
pub fn cantor_set(iterations: usize, resolution: usize) -> Vec<u32> {
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
            for i in (start + third)..(start + third * 2) {
                set[i] = 0;
            }

            // Keep last third
            new_segments.push((start + third * 2, end));
        }

        segments = new_segments;
    }

    set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantor_set_basic() {
        let set0 = cantor_set(0, 9);
        assert_eq!(set0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);

        let set1 = cantor_set(1, 9);
        assert_eq!(set1, vec![1, 1, 1, 0, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn test_cantor_set_edge_cases() {
        let empty = cantor_set(3, 0);
        assert_eq!(empty, Vec::<u32>::new());

        let single = cantor_set(2, 1);
        assert_eq!(single, vec![1]);
    }

    #[test]
    fn test_cantor_set_iteration_2() {
        // With 27 points (3^3), we can cleanly divide
        let set = cantor_set(2, 27);

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
        let set1 = cantor_set(1, 27);
        let set2 = cantor_set(2, 27);
        let set3 = cantor_set(3, 27);

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
