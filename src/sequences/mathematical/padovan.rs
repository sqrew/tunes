/// Generate Padovan sequence up to n terms
///
/// The Padovan sequence, named after Richard Padovan, is defined by the recurrence:
/// P(n) = P(n-2) + P(n-3)
///
/// Starting with P(0) = 1, P(1) = 1, P(2) = 1, the sequence goes:
/// 1, 1, 1, 2, 2, 3, 4, 5, 7, 9, 12, 16, 21, 28, 37, 49...
///
/// # The Plastic Number (ρ)
/// The ratio between consecutive Padovan numbers converges to the plastic number
/// (also called the plastic constant or silver ratio):
///
/// ρ = (1 + ∛((9 + √69)/18) + ∛((9 - √69)/18)) ≈ 1.32471795...
///
/// This is the unique real solution to x³ = x + 1.
///
/// Just as Fibonacci relates to the golden ratio (φ ≈ 1.618), Padovan relates to
/// the plastic number. However, the plastic number's recurrence goes back 3 terms
/// instead of 2, creating a different rhythmic and intervallic character.
///
/// # Mathematical Properties
/// - Named after architect Richard Padovan who studied its geometric properties
/// - Related to the spiral growth of certain flowers and shells
/// - P(n) counts certain triangulation patterns
/// - Appears in the study of Pascal's triangle variants
/// - The only sequence with this particular 3-term recurrence starting with 1,1,1
///
/// # Musical Character
/// The 3-term recurrence creates a more gradual, "lazier" growth than Fibonacci:
/// - **Smoother curves**: Changes are less dramatic
/// - **Triadic relationships**: Natural for music in groups of 3
/// - **Alternative phrasing**: Different from both Fibonacci and Lucas
/// - **Subtle proportions**: The plastic number is less "obvious" than golden ratio
///
/// # Arguments
/// * `n` - Number of Padovan terms to generate
///
/// # Returns
/// Vector of the first n Padovan numbers: [1, 1, 1, 2, 2, 3, 4, 5, 7, ...]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let padovan = sequences::padovan(12);
/// assert_eq!(padovan, vec![1, 1, 1, 2, 2, 3, 4, 5, 7, 9, 12, 16]);
///
/// // Use for melodic intervals with gentler growth
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let base = 220.0; // A3
/// for (i, &interval) in padovan.iter().take(8).enumerate() {
///     let freq = base * 2.0_f32.powf(interval as f32 / 12.0);
///     comp.track("padovan_melody")
///         .at(i as f32 * 0.4)
///         .note(&[freq], 0.3);
/// }
///
/// // Use for phrase structure (triadic groupings)
/// let phrase_lengths = sequences::padovan(6); // [1, 1, 1, 2, 2, 3] bars
/// ```
///
/// # Musical Applications
/// - **Triadic structures**: Natural for music organized in 3s
/// - **Gentle curves**: Smoother dynamic or tempo changes than Fibonacci
/// - **Alternative ratios**: Use plastic number for microtonal tuning experiments
/// - **Phrase lengths**: Creates different flow than Fibonacci/Lucas
/// - **Polyrhythmic layers**: 3-against-2 patterns with Padovan relationships
/// - **Delay times**: Unique echo spacing following the sequence
/// - **Filter modulation**: Subtle, organic parameter changes
///
/// # Comparison with Fibonacci
/// ```text
/// Fibonacci: 1, 1, 2, 3, 5,  8, 13, 21, 34 (P(n) = P(n-1) + P(n-2))
/// Padovan:   1, 1, 1, 2, 2,  3,  4,  5,  7 (P(n) = P(n-2) + P(n-3))
///            ↓  ↓  ↓  ↓  ↓   ↓   ↓   ↓   ↓
///           (Padovan grows more slowly - gentler curves)
/// ```
pub fn padovan(n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![1];
    }
    if n == 2 {
        return vec![1, 1];
    }

    let mut seq = vec![1, 1, 1]; // P(0) = P(1) = P(2) = 1

    for i in 3..n {
        // P(n) = P(n-2) + P(n-3)
        let next = seq[i - 2] + seq[i - 3];
        seq.push(next);
    }

    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padovan_basic() {
        let seq = padovan(12);
        assert_eq!(seq, vec![1, 1, 1, 2, 2, 3, 4, 5, 7, 9, 12, 16]);
    }

    #[test]
    fn test_padovan_empty() {
        let seq = padovan(0);
        assert_eq!(seq, Vec::<u32>::new());
    }

    #[test]
    fn test_padovan_one() {
        let seq = padovan(1);
        assert_eq!(seq, vec![1]);
    }

    #[test]
    fn test_padovan_two() {
        let seq = padovan(2);
        assert_eq!(seq, vec![1, 1]);
    }

    #[test]
    fn test_padovan_three() {
        let seq = padovan(3);
        assert_eq!(seq, vec![1, 1, 1]);
    }

    #[test]
    fn test_padovan_recurrence() {
        let seq = padovan(15);
        // Verify the recurrence relation P(n) = P(n-2) + P(n-3)
        for i in 3..seq.len() {
            assert_eq!(seq[i], seq[i - 2] + seq[i - 3]);
        }
    }

    #[test]
    fn test_padovan_known_values() {
        let seq = padovan(16);
        assert_eq!(seq[0], 1);
        assert_eq!(seq[1], 1);
        assert_eq!(seq[2], 1);
        assert_eq!(seq[3], 2);   // 1 + 1
        assert_eq!(seq[4], 2);   // 1 + 1
        assert_eq!(seq[5], 3);   // 2 + 1
        assert_eq!(seq[6], 4);   // 2 + 2
        assert_eq!(seq[7], 5);   // 3 + 2
        assert_eq!(seq[8], 7);   // 4 + 3
        assert_eq!(seq[9], 9);   // 5 + 4
        assert_eq!(seq[10], 12); // 7 + 5
        assert_eq!(seq[11], 16); // 9 + 7
        assert_eq!(seq[12], 21); // 12 + 9
        assert_eq!(seq[13], 28); // 16 + 12
        assert_eq!(seq[14], 37); // 21 + 16
        assert_eq!(seq[15], 49); // 28 + 21
    }

    #[test]
    fn test_padovan_growth() {
        let seq = padovan(15);
        // Padovan grows, but not always monotonically at the start
        // After the initial 1,1,1, it should generally increase
        for i in 4..seq.len() {
            assert!(seq[i] >= seq[i - 1]);
        }
    }

    #[test]
    fn test_padovan_slower_than_fibonacci() {
        let padovan_seq = padovan(12);
        // Fibonacci would be: [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]
        // Padovan is slower: [1, 1, 1, 2, 2, 3,  4,  5,  7,  9, 12,  16]

        // At position 11, Fibonacci is 144, Padovan is 16
        assert!(padovan_seq[11] < 144);

        // Padovan grows slower
        assert_eq!(padovan_seq[11], 16);
    }

    #[test]
    fn test_padovan_triple_start() {
        let seq = padovan(5);
        // Padovan uniquely starts with three 1s
        assert_eq!(seq[0], 1);
        assert_eq!(seq[1], 1);
        assert_eq!(seq[2], 1);
        // Then changes
        assert_eq!(seq[3], 2);
    }

    #[test]
    fn test_padovan_plastic_ratio() {
        let seq = padovan(20);
        // The ratio should converge to the plastic number ρ ≈ 1.32471795

        // Check later ratios (they converge slowly)
        let ratio = seq[19] as f32 / seq[18] as f32;

        // Should be approaching 1.32471795
        assert!(ratio > 1.30 && ratio < 1.35);
    }

    #[test]
    fn test_padovan_longer_sequence() {
        let seq = padovan(20);
        assert_eq!(seq.len(), 20);
        // Verify a value in the middle
        assert_eq!(seq[15], 49); // P(15)
    }
}
