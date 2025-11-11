/// Generate arithmetic sequence (linear progression)
///
/// An arithmetic sequence is formed by adding a constant value (the "step" or "common difference")
/// to each term: a, a+d, a+2d, a+3d, a+4d, ...
///
/// This is the simplest type of progression - just counting up (or down) by a fixed amount.
/// Examples:
/// - 1, 2, 3, 4, 5, 6... (step = 1)
/// - 2, 4, 6, 8, 10... (step = 2, even numbers)
/// - 5, 10, 15, 20, 25... (step = 5, multiples of 5)
/// - 100, 90, 80, 70... (step = -10, counting down - use with caution for u32)
///
/// # Arguments
/// * `start` - The first value in the sequence
/// * `step` - Amount to add to each subsequent term (common difference)
/// * `n` - Number of terms to generate
///
/// # Returns
/// Vector containing [start, start+step, start+2*step, ...]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Simple counting sequence
/// let count = sequences::arithmetic::generate(1, 1, 10);
/// assert_eq!(count, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
///
/// // Even numbers
/// let evens = sequences::arithmetic::generate(2, 2, 8);
/// assert_eq!(evens, vec![2, 4, 6, 8, 10, 12, 14, 16]);
///
/// // Use for ascending scale pattern
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// # use tunes::consts::scales::C4_MAJOR_SCALE;
/// let scale_indices = sequences::arithmetic::generate(0, 1, 8); // [0, 1, 2, 3, 4, 5, 6, 7]
/// comp.track("ascending")
///     .sequence_from(&scale_indices, &C4_MAJOR_SCALE, 0.25);
///
/// // Use for regular rhythm (every 4 beats)
/// let beat_positions = sequences::arithmetic::generate(0, 4, 16);
/// let times = sequences::normalize(&beat_positions, 0.0, 16.0);
/// ```
///
/// # Musical Applications
/// - **Scales**: Ascending/descending through scale degrees (0, 1, 2, 3...)
/// - **Regular rhythms**: Evenly spaced beats (every 2, 3, or 4 steps)
/// - **Velocity ramps**: Linear increase/decrease in volume
/// - **Filter sweeps**: Linear cutoff frequency changes
/// - **Time markers**: Evenly spaced section boundaries
/// - **Melodic steps**: Stepwise motion up or down
///
/// # Why Use Arithmetic Sequences
/// They're predictable and regular - perfect for:
/// - Creating steady, mechanical patterns
/// - Building foundations for more complex variations
/// - Establishing a baseline before applying transformations
/// - Simulating linear motion or growth
pub fn generate(start: u32, step: u32, n: usize) -> Vec<u32> {
    (0..n).map(|i| start + step * i as u32).collect()
}
