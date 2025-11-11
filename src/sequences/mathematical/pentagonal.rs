/// Generate pentagonal numbers up to n terms
///
/// Pentagonal numbers are figurate numbers that represent pentagons arranged in
/// nested layers. They are given by the formula:
///
/// P(n) = n(3n - 1) / 2
///
/// The sequence goes: 1, 5, 12, 22, 35, 51, 70, 92, 117, 145, 176, 210...
///
/// These numbers count dots in pentagonal arrangements:
/// ```text
/// n=1:    •           (1 dot)
///
/// n=2:    • •         (5 dots = 1 + 4)
///        • • •
///         • •
///
/// n=3:   • • •        (12 dots = 1 + 4 + 7)
///       • • • •
///      • • • • •
///       • • • •
///        • • •
/// ```
///
/// # Mathematical Properties
/// - Every pentagonal number is 1/3 of a triangular number
/// - P(n) = n + 3·T(n-1) where T(n) is the nth triangular number
/// - Related to Euler's pentagonal number theorem in partition theory
/// - Appears in the generalized pentagonal numbers: ±1, ±2, ±5, ±7, ±12, ±15...
/// - Used in proving identities about integer partitions
///
/// # Musical Character
/// Pentagonal numbers have a quadratic growth rate (n²), creating a steady,
/// predictable expansion that's faster than linear but smoother than exponential:
/// - **Steady acceleration**: Predictable, mathematical feel
/// - **Visual connection**: The pentagon shape has musical significance (pentatonic!)
/// - **Moderate growth**: Between arithmetic and geometric progressions
/// - **Euler connection**: Links to deep number theory (partition functions)
///
/// The differences between consecutive pentagonal numbers form an arithmetic sequence:
/// 4, 7, 10, 13, 16, 19... (multiples of 3, offset by 1)
///
/// # Arguments
/// * `n` - Number of pentagonal terms to generate (starting from n=1)
///
/// # Returns
/// Vector of the first n pentagonal numbers: [1, 5, 12, 22, 35, ...]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let pent = sequences::pentagonal::generate(10);
/// assert_eq!(pent, vec![1, 5, 12, 22, 35, 51, 70, 92, 117, 145]);
///
/// // Use for rhythmic acceleration
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// // Normalize to beat positions
/// let beat_positions = sequences::normalize(&pent[0..8], 0.0, 32.0);
/// for &beat in &beat_positions {
///     comp.track("pent_rhythm")
///         .at(beat)
///         .note(&[440.0], 0.1);
/// }
///
/// // Use for expanding intervals
/// let intervals = vec![1, 5, 12]; // Pentagonal: semitone, perfect 4th, octave
/// ```
///
/// # Musical Applications
/// - **Phrase structure**: Measures per section with quadratic growth
/// - **Rhythmic placement**: Accelerating beat positions
/// - **Harmonic series**: Pentagonal relationships in overtone selection
/// - **Dynamic curves**: Smooth crescendos/decrescendos
/// - **Filter automation**: Cutoff frequencies with quadratic curve
/// - **Delay networks**: Tap delay times following pentagonal spacing
/// - **Pentatonic connection**: Use with pentatonic scales for thematic unity
///
/// # Connection to Pentatonic Scales
/// The pentagon (5-sided) has the same root as pentatonic (5-note scales).
/// While not mathematically linked, this creates a nice conceptual connection
/// when using pentagonal numbers with pentatonic melodies.
///
/// # Growth Comparison
/// ```text
/// n:          1   2   3   4   5   6   7   8   9  10
/// Linear:     1   2   3   4   5   6   7   8   9  10  (n)
/// Triangular: 1   3   6  10  15  21  28  36  45  55  (n(n+1)/2)
/// Pentagonal: 1   5  12  22  35  51  70  92 117 145  (n(3n-1)/2)
/// Square:     1   4   9  16  25  36  49  64  81 100  (n²)
///
/// Pentagonal grows faster than triangular but not as fast as square
/// ```
pub fn generate(n: usize) -> Vec<u32> {
    (1..=n)
        .map(|i| {
            let i = i as u32;
            i * (3 * i - 1) / 2
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pentagonal_basic() {
        let seq = generate(10);
        assert_eq!(seq, vec![1, 5, 12, 22, 35, 51, 70, 92, 117, 145]);
    }

    #[test]
    fn test_pentagonal_empty() {
        let seq = generate(0);
        assert_eq!(seq, Vec::<u32>::new());
    }

    #[test]
    fn test_pentagonal_one() {
        let seq = generate(1);
        assert_eq!(seq, vec![1]);
    }

    #[test]
    fn test_pentagonal_formula() {
        let seq = generate(12);
        // Verify each value matches the formula P(n) = n(3n-1)/2
        for (i, &value) in seq.iter().enumerate() {
            let n = (i + 1) as u32;
            let expected = n * (3 * n - 1) / 2;
            assert_eq!(value, expected);
        }
    }

    #[test]
    fn test_pentagonal_known_values() {
        let seq = generate(8);
        assert_eq!(seq[0], 1);   // 1(3·1-1)/2 = 1·2/2 = 1
        assert_eq!(seq[1], 5);   // 2(3·2-1)/2 = 2·5/2 = 5
        assert_eq!(seq[2], 12);  // 3(3·3-1)/2 = 3·8/2 = 12
        assert_eq!(seq[3], 22);  // 4(3·4-1)/2 = 4·11/2 = 22
        assert_eq!(seq[4], 35);  // 5(3·5-1)/2 = 5·14/2 = 35
        assert_eq!(seq[5], 51);  // 6(3·6-1)/2 = 6·17/2 = 51
        assert_eq!(seq[6], 70);  // 7(3·7-1)/2 = 7·20/2 = 70
        assert_eq!(seq[7], 92);  // 8(3·8-1)/2 = 8·23/2 = 92
    }

    #[test]
    fn test_pentagonal_growth() {
        let seq = generate(15);
        // Pentagonal numbers should grow monotonically
        for i in 1..seq.len() {
            assert!(seq[i] > seq[i - 1]);
        }
    }

    #[test]
    fn test_pentagonal_differences() {
        let seq = generate(10);
        // The differences between consecutive pentagonal numbers
        // form an arithmetic sequence: 4, 7, 10, 13, 16, 19, 22, 25, 28...
        // (increasing by 3 each time)

        let mut diffs = Vec::new();
        for i in 1..seq.len() {
            diffs.push(seq[i] - seq[i - 1]);
        }

        // Check that differences increase by 3
        for i in 1..diffs.len() {
            assert_eq!(diffs[i] - diffs[i - 1], 3);
        }

        // First difference should be 4 (5 - 1)
        assert_eq!(diffs[0], 4);
    }

    #[test]
    fn test_pentagonal_second_differences() {
        let seq = generate(10);

        // First differences
        let mut diffs1 = Vec::new();
        for i in 1..seq.len() {
            diffs1.push(seq[i] - seq[i - 1]);
        }

        // Second differences (should all be 3 for quadratic sequence)
        for i in 1..diffs1.len() {
            let second_diff = diffs1[i] - diffs1[i - 1];
            assert_eq!(second_diff, 3);
        }
    }

    #[test]
    fn test_pentagonal_quadratic_growth() {
        let seq = generate(10);
        // Pentagonal has quadratic growth, roughly proportional to n²

        // Later terms should be roughly 9x the early terms when n triples
        // P(3) = 12, P(9) = 117
        // 117/12 ≈ 9.75 (close to 9 = 3²)
        let ratio = seq[8] as f32 / seq[2] as f32;
        assert!(ratio > 8.0 && ratio < 11.0);
    }

    #[test]
    fn test_pentagonal_faster_than_triangular() {
        let pent = generate(10);
        // Triangular numbers: 1, 3, 6, 10, 15, 21, 28, 36, 45, 55
        // Pentagonal numbers: 1, 5, 12, 22, 35, 51, 70, 92, 117, 145

        // At position 10, triangular is 55, pentagonal is 145
        assert!(pent[9] > 55);
        assert_eq!(pent[9], 145);
    }

    #[test]
    fn test_pentagonal_relation_to_triangular() {
        let pent = generate(8);
        // P(n) = n + 3·T(n-1) where T(n) = n(n+1)/2 is triangular

        for i in 1..pent.len() {
            let n = (i + 1) as u32;
            let triangular_prev = if i == 0 {
                0
            } else {
                let m = i as u32;
                m * (m + 1) / 2
            };

            let expected = n + 3 * triangular_prev;
            assert_eq!(pent[i], expected);
        }
    }

    #[test]
    fn test_pentagonal_longer_sequence() {
        let seq = generate(15);
        assert_eq!(seq.len(), 15);
        assert_eq!(seq[14], 330); // P(15) = 15(3·15-1)/2 = 15·44/2 = 330
    }

    #[test]
    fn test_pentagonal_specific_values() {
        let seq = generate(5);
        // These are the classic pentagonal numbers
        assert_eq!(seq, vec![1, 5, 12, 22, 35]);
    }
}
