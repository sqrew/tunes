/// Generate golden ratio (phi) sequence: 1, φ, φ², φ³, φ⁴...
///
/// The golden ratio (φ ≈ 1.618033988749) is found throughout nature and has been used
/// in music composition for centuries. This sequence generates successive powers of phi,
/// creating naturally pleasing proportional relationships.
///
/// The golden ratio appears in:
/// - Nautilus shells, flower petals, pine cones
/// - Classical architecture (Parthenon, pyramids)
/// - Musical form (sonata proportions, phrase lengths)
/// - The ratio that Fibonacci numbers converge to
///
/// # Arguments
/// * `n` - Number of terms to generate
///
/// # Returns
/// Vector of successive powers of phi: [φ⁰, φ¹, φ², φ³, ...]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let phi_seq = sequences::golden_ratio(6);
/// // Returns approximately: [1.0, 1.618, 2.618, 4.236, 6.854, 11.090]
///
/// // Use with normalize() to map to frequencies:
/// let values = sequences::golden_ratio(8);
/// let freqs = sequences::normalize(&values.iter().map(|&x| x as u32).collect::<Vec<_>>(), 200.0, 800.0);
/// ```
///
/// # Musical Applications
/// - **Form and structure**: Divide piece duration by phi for natural section lengths
/// - **Melodic intervals**: Map phi powers to pitch space for organic contours
/// - **Rhythm**: Use phi ratios for timing relationships (not strictly metric)
/// - **Texture density**: Scale number of voices/layers by phi
pub fn golden_ratio(n: usize) -> Vec<f32> {
    const PHI: f32 = 1.618_034;
    (0..n).map(|i| PHI.powi(i as i32)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_ratio() {
        const PHI: f32 = 1.618033988749;
        let seq = golden_ratio(5);

        assert_eq!(seq.len(), 5);
        assert!((seq[0] - 1.0).abs() < 0.001);
        assert!((seq[1] - PHI).abs() < 0.001);
        assert!((seq[2] - PHI * PHI).abs() < 0.001);
    }
}
