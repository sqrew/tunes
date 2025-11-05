/// Generate geometric sequence (exponential progression)
///
/// A geometric sequence is formed by multiplying each term by a constant value (the "ratio"
/// or "common ratio"): a, a×r, a×r², a×r³, a×r⁴, ...
///
/// Unlike arithmetic sequences (which grow linearly), geometric sequences grow exponentially.
/// This creates dramatic acceleration - values get much larger very quickly.
///
/// Examples:
/// - 1, 2, 4, 8, 16, 32... (ratio = 2, doubling sequence)
/// - 3, 9, 27, 81, 243... (ratio = 3, tripling sequence)
/// - 5, 25, 125, 625... (ratio = 5)
///
/// # Arguments
/// * `start` - The first value in the sequence
/// * `ratio` - The multiplier for each subsequent term (common ratio)
/// * `n` - Number of terms to generate
///
/// # Returns
/// Vector containing [start, start×ratio, start×ratio², ...]
///
/// # Warning
/// Geometric sequences with ratio > 2 grow VERY rapidly. For example, with start=2 and ratio=3:
/// - Term 5: 162
/// - Term 10: 39,366
/// - Term 15: 9,565,938
/// Use normalize() to map to usable ranges.
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Doubling sequence (same as powers of 2 but with custom start)
/// let doubling = sequences::geometric(1, 2, 8);
/// assert_eq!(doubling, vec![1, 2, 4, 8, 16, 32, 64, 128]);
///
/// // Tripling sequence
/// let tripling = sequences::geometric(1, 3, 5);
/// assert_eq!(tripling, vec![1, 3, 9, 27, 81]);
///
/// // Use for accelerating rhythms
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let accel = sequences::geometric(1, 2, 6);
/// let durations = sequences::normalize(&accel, 0.125, 1.0);
/// // Creates accelerating pattern: long → medium → short → very short
///
/// // Use for exponential volume increase (careful!)
/// let growth = sequences::geometric(1, 2, 8);
/// let volumes = sequences::normalize(&growth, 0.1, 1.0);
/// ```
///
/// # Musical Applications
/// - **Accelerando**: Exponentially decreasing note durations (tempo acceleration)
/// - **Crescendo curves**: Exponential volume increase (more dramatic than linear)
/// - **Octave stacking**: Multiply base frequency by 2ⁿ
/// - **Rhythmic density**: Exponentially increasing subdivisions
/// - **Filter sweeps**: Exponential cutoff changes (more natural than linear)
/// - **Spatial effects**: Exponential pan or reverb changes
///
/// # Musical Context
/// Geometric sequences feel more "natural" than arithmetic for many parameters because:
/// - Human hearing is logarithmic (each octave is a doubling)
/// - Perceived loudness scales logarithmically
/// - Musical intervals are multiplicative ratios, not additive
/// - Natural phenomena (sound decay, reverberation) are exponential
///
/// However, they grow very fast - almost always use normalize() to constrain the output
/// to musical ranges.
pub fn geometric(start: u32, ratio: u32, n: usize) -> Vec<u32> {
    (0..n).map(|i| start * ratio.pow(i as u32)).collect()
}
