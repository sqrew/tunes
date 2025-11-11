/// Generate Lucas sequence up to n terms
///
/// The Lucas sequence is the companion to Fibonacci, using the same recurrence relation
/// L(n) = L(n-1) + L(n-2) but with different starting values: 2, 1 instead of 1, 1.
///
/// The sequence goes: 2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199...
///
/// Named after François Édouard Anatole Lucas (1842-1891), this sequence shares many
/// properties with Fibonacci but creates different intervallic relationships. Like Fibonacci,
/// the ratio between consecutive Lucas numbers converges to the golden ratio φ ≈ 1.618.
///
/// # Mathematical Properties
/// - L(n) = F(n-1) + F(n+1) where F is Fibonacci
/// - L(n)² = 5·F(n)² + 4·(-1)ⁿ (Lucas-Fibonacci identity)
/// - Used in primality testing (Lucas-Lehmer test)
/// - Appears in combinatorics and number theory
///
/// # Musical Character
/// Unlike Fibonacci's gradual 1→1→2→3 start, Lucas begins with 2→1→3→4, creating
/// larger initial jumps. This makes it excellent for:
/// - More dramatic melodic leaps early in the sequence
/// - Different harmonic series than Fibonacci
/// - Creating tension through larger intervals
///
/// # Arguments
/// * `n` - Number of Lucas terms to generate
///
/// # Returns
/// Vector of the first n Lucas numbers: [2, 1, 3, 4, 7, 11, 18, ...]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let lucas = sequences::lucas::generate(8);
/// assert_eq!(lucas, vec![2, 1, 3, 4, 7, 11, 18, 29]);
///
/// // Use for melodic intervals (semitones)
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let base = 440.0; // A4
/// for (i, &interval) in lucas.iter().take(6).enumerate() {
///     let freq = base * 2.0_f32.powf(interval as f32 / 12.0);
///     comp.track("lucas_melody")
///         .at(i as f32 * 0.5)
///         .note(&[freq], 0.4);
/// }
///
/// // Use for phrase lengths (different from Fibonacci)
/// let phrase_lengths = sequences::lucas::generate(5); // [2, 1, 3, 4, 7] beats
/// ```
///
/// # Musical Applications
/// - **Melodic intervals**: Creates larger jumps than Fibonacci, more dramatic
/// - **Harmonic series**: Different overtone relationships
/// - **Phrase structure**: Alternative to Fibonacci for organic phrasing
/// - **Countermelody**: Use Lucas against Fibonacci for natural counterpoint
/// - **Rhythmic variation**: Note durations or rest lengths
/// - **Dynamic curves**: Volume envelopes with different shape than Fibonacci
///
/// # Comparison with Fibonacci
/// ```text
/// Fibonacci: 1, 1, 2, 3, 5,  8, 13, 21, 34, 55
/// Lucas:     2, 1, 3, 4, 7, 11, 18, 29, 47, 76
///            ↑  ↓  ↑  ↑  ↑   ↑   ↑   ↑   ↑   ↑
///           (Lucas starts higher, dips, then diverges upward)
/// ```
pub fn generate(n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![2];
    }

    let mut seq = vec![2, 1];

    for i in 2..n {
        let next = seq[i - 1] + seq[i - 2];
        seq.push(next);
    }

    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lucas_basic() {
        let seq = generate(8);
        assert_eq!(seq, vec![2, 1, 3, 4, 7, 11, 18, 29]);
    }

    #[test]
    fn test_lucas_empty() {
        let seq = generate(0);
        assert_eq!(seq, Vec::<u32>::new());
    }

    #[test]
    fn test_lucas_one() {
        let seq = generate(1);
        assert_eq!(seq, vec![2]);
    }

    #[test]
    fn test_lucas_two() {
        let seq = generate(2);
        assert_eq!(seq, vec![2, 1]);
    }

    #[test]
    fn test_lucas_long_sequence() {
        let seq = generate(12);
        assert_eq!(seq, vec![2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199]);
    }

    #[test]
    fn test_lucas_recurrence() {
        let seq = generate(10);
        // Verify the recurrence relation L(n) = L(n-1) + L(n-2)
        for i in 2..seq.len() {
            assert_eq!(seq[i], seq[i - 1] + seq[i - 2]);
        }
    }

    #[test]
    fn test_lucas_growth() {
        let seq = generate(10);
        // Lucas numbers should grow
        for i in 2..seq.len() {
            assert!(seq[i] > seq[i - 1]);
        }
    }

    #[test]
    fn test_lucas_differs_from_fibonacci() {
        let lucas_seq = generate(8);
        // Fibonacci would be: [1, 1, 2, 3, 5, 8, 13, 21]
        // Lucas should be different (except they might share some values)
        assert_eq!(lucas_seq[0], 2); // Not 1
        assert_eq!(lucas_seq[1], 1); // Same
        assert_eq!(lucas_seq[2], 3); // Not 2
    }
}

// ========== PRESETS ==========

/// Short Lucas sequence (6 terms)
pub fn short() -> Vec<u32> {
    generate(6)
}

/// Classic Lucas sequence (12 terms) - balanced length
pub fn classic() -> Vec<u32> {
    generate(12)
}

/// Extended Lucas sequence (16 terms)
pub fn extended() -> Vec<u32> {
    generate(16)
}
