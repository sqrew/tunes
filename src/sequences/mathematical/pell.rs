/// Generate Pell numbers up to n terms
///
/// The Pell numbers are defined by the recurrence relation:
/// P(n) = 2·P(n-1) + P(n-2)
///
/// Starting with P(0) = 0, P(1) = 1, the sequence goes:
/// 0, 1, 2, 5, 12, 29, 70, 169, 408, 985, 2378, 5741...
///
/// Named after English mathematician John Pell (1611-1685), though he did not
/// actually discover them. The sequence was known to Indian mathematicians
/// centuries earlier in the study of the equation x² - 2y² = ±1.
///
/// # Connection to √2
/// Pell numbers provide the best rational approximations to √2:
/// - P(n)/P(n-1) converges to √2 ≈ 1.41421356...
/// - The fractions 1/1, 3/2, 7/5, 17/12, 41/29, 99/70... approximate √2
/// - These are solutions to Pell's equation: x² - 2y² = 1
///
/// This makes Pell numbers excellent for microtonal and just intonation work,
/// as √2 is the ratio of an octave (12 semitones) to a tritone (6 semitones).
///
/// # Mathematical Properties
/// - Growth rate: P(n) ~ (1 + √2)ⁿ / (2√2)
/// - Faster growth than Fibonacci due to the coefficient 2
/// - Used in solving quadratic Diophantine equations
/// - Appears in tiling problems and continued fractions
/// - P(n)² + P(n-1)² = P(2n)
///
/// # Musical Character
/// The factor of 2 in the recurrence creates faster growth than Fibonacci,
/// making Pell numbers ideal for:
/// - **Dramatic expansion**: Crescendos and accelerandos with power
/// - **Microtonal tuning**: √2 approximations for just intonation
/// - **Aggressive growth**: More intense than Fibonacci, less than exponential
/// - **Tritone relationships**: The √2 connection relates to the tritone (augmented 4th)
///
/// # Arguments
/// * `n` - Number of Pell terms to generate
///
/// # Returns
/// Vector of the first n Pell numbers: [0, 1, 2, 5, 12, 29, 70, ...]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let pell = sequences::pell::generate(10);
/// assert_eq!(pell, vec![0, 1, 2, 5, 12, 29, 70, 169, 408, 985]);
///
/// // Use for dramatic crescendo
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let volumes = sequences::normalize(&pell[1..8], 0.3, 1.0);
/// for (i, &volume) in volumes.iter().enumerate() {
///     comp.track("pell_crescendo")
///         .at(i as f32 * 0.5)
///         .volume(volume)
///         .note(&[440.0], 0.4);
/// }
///
/// // Use for microtonal approximations
/// let base = 440.0; // A4
/// // P(5)/P(4) = 29/12 ≈ 2.4166... (for just intonation intervals)
/// let ratio = pell[5] as f32 / pell[4] as f32;
/// ```
///
/// # Musical Applications
/// - **Microtonal tuning**: √2 approximations for just intonation systems
/// - **Dramatic growth**: Crescendos, accelerandos with more power than Fibonacci
/// - **Tritone exploration**: Musical use of the √2 relationship
/// - **Aggressive expansion**: Texture, density, or complexity increases
/// - **Phrase lengths**: Rapidly expanding sections (be careful - grows fast!)
/// - **Filter sweeps**: Cutoff frequencies with exponential-like curve
/// - **Rhythmic acceleration**: Subdivision counts that grow dramatically
///
/// # Comparison with Fibonacci
/// ```text
/// Fibonacci: 0, 1,  1,  2,  3,  5,   8,  13,  21,   34,   55
/// Pell:      0, 1,  2,  5, 12, 29,  70, 169, 408,  985, 2378
///            ↑  ↑   ↑   ↑   ↑   ↑    ↑    ↑    ↑     ↑     ↑
///           (Pell grows much faster - more dramatic!)
/// ```
pub fn generate(n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![0];
    }

    let mut seq = vec![0, 1]; // P(0) = 0, P(1) = 1

    for i in 2..n {
        // P(n) = 2·P(n-1) + P(n-2)
        let next = 2 * seq[i - 1] + seq[i - 2];
        seq.push(next);
    }

    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pell_basic() {
        let seq = generate(10);
        assert_eq!(seq, vec![0, 1, 2, 5, 12, 29, 70, 169, 408, 985]);
    }

    #[test]
    fn test_pell_empty() {
        let seq = generate(0);
        assert_eq!(seq, Vec::<u32>::new());
    }

    #[test]
    fn test_pell_one() {
        let seq = generate(1);
        assert_eq!(seq, vec![0]);
    }

    #[test]
    fn test_pell_two() {
        let seq = generate(2);
        assert_eq!(seq, vec![0, 1]);
    }

    #[test]
    fn test_pell_recurrence() {
        let seq = generate(12);
        // Verify the recurrence relation P(n) = 2·P(n-1) + P(n-2)
        for i in 2..seq.len() {
            assert_eq!(seq[i], 2 * seq[i - 1] + seq[i - 2]);
        }
    }

    #[test]
    fn test_pell_known_values() {
        let seq = generate(12);
        assert_eq!(seq[0], 0);
        assert_eq!(seq[1], 1);
        assert_eq!(seq[2], 2);     // 2·1 + 0
        assert_eq!(seq[3], 5);     // 2·2 + 1
        assert_eq!(seq[4], 12);    // 2·5 + 2
        assert_eq!(seq[5], 29);    // 2·12 + 5
        assert_eq!(seq[6], 70);    // 2·29 + 12
        assert_eq!(seq[7], 169);   // 2·70 + 29
        assert_eq!(seq[8], 408);   // 2·169 + 70
        assert_eq!(seq[9], 985);   // 2·408 + 169
        assert_eq!(seq[10], 2378); // 2·985 + 408
        assert_eq!(seq[11], 5741); // 2·2378 + 985
    }

    #[test]
    fn test_pell_growth() {
        let seq = generate(12);
        // Pell numbers should grow monotonically after P(0)
        for i in 2..seq.len() {
            assert!(seq[i] > seq[i - 1]);
        }
    }

    #[test]
    fn test_pell_faster_than_fibonacci() {
        let pell_seq = generate(10);
        // Fibonacci would be: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        // Pell is faster:     [0, 1, 2, 5, 12, 29, 70, 169, 408, 985]

        // At position 9, Fibonacci is 34, Pell is 985
        assert!(pell_seq[9] > 34);
        assert_eq!(pell_seq[9], 985);
    }

    #[test]
    fn test_pell_sqrt2_approximation() {
        let seq = generate(10);
        // P(n)/P(n-1) should approximate 1 + √2 ≈ 2.41421356

        // Check later ratios (skip P(1)/P(0) which is undefined)
        for i in 3..seq.len() {
            let ratio = seq[i] as f64 / seq[i - 1] as f64;
            // Should be approaching 1 + √2
            assert!(ratio > 2.0 && ratio < 3.0);
        }

        // The ratio should get closer to 1 + √2 as n increases
        let ratio_late = seq[9] as f64 / seq[8] as f64;
        let one_plus_sqrt2 = 1.0 + 2.0_f64.sqrt();

        // Should be very close to 1 + √2 ≈ 2.414
        assert!((ratio_late - one_plus_sqrt2).abs() < 0.01);
    }

    #[test]
    fn test_pell_specific_values() {
        let seq = generate(10);
        // Verify specific known Pell numbers
        // This serves as a comprehensive check of the algorithm

        assert_eq!(seq[0], 0);
        assert_eq!(seq[1], 1);
        assert_eq!(seq[2], 2);
        assert_eq!(seq[3], 5);
        assert_eq!(seq[4], 12);
        assert_eq!(seq[5], 29);
        assert_eq!(seq[6], 70);
        assert_eq!(seq[7], 169);
        assert_eq!(seq[8], 408);
        assert_eq!(seq[9], 985);
    }

    #[test]
    fn test_pell_starts_with_zero() {
        let seq = generate(5);
        assert_eq!(seq[0], 0);
        assert_eq!(seq[1], 1);
    }

    #[test]
    fn test_pell_rapid_growth() {
        let seq = generate(12);
        // Verify rapid growth - each term should be at least double the previous
        for i in 2..seq.len() {
            assert!(seq[i] >= 2 * seq[i - 1]);
        }
    }

    #[test]
    fn test_pell_longer_sequence() {
        let seq = generate(13);
        assert_eq!(seq.len(), 13);
        assert_eq!(seq[12], 13860); // P(12)
    }
}
