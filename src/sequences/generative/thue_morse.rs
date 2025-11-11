/// Generate Thue-Morse sequence (fair division sequence)
///
/// The Thue-Morse sequence is a binary sequence with remarkable fairness properties.
/// It's constructed by starting with 0, then repeatedly appending the bitwise complement:
/// - Start: `0`
/// - Append complement: `0 1`
/// - Append complement: `0 1 1 0`
/// - Append complement: `0 1 1 0 1 0 0 1`
/// - Continue...
///
/// This sequence has fascinating properties:
/// - **No three consecutive identical blocks** (avoids repetition)
/// - **Fairest possible coin flip sequence** (equal distribution)
/// - **Self-similar** (contains itself at different scales)
/// - **Appears in chemistry** (protein folding), music, and computer science
///
/// Named after mathematicians Axel Thue (1906) and Marston Morse (1921).
///
/// # Arguments
/// * `n` - Number of terms to generate
///
/// # Returns
/// Vector of 0s and 1s forming the Thue-Morse sequence
///
/// # Typical Values
/// - **n = 8-16**: Short patterns (accent patterns, simple alternation)
/// - **n = 16-32**: Medium patterns (drum programming, rhythm)
/// - **n = 32-64**: Long patterns (extended sequences, formal structure)
/// - Powers of 2 (8, 16, 32, 64) align with the sequence's natural structure
///
/// # Recipe: Non-Repetitive Drum Pattern
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences;
///
/// let mut comp = Composition::new(Tempo::new(128.0));
///
/// // Extract hit positions from Thue-Morse
/// let tm = sequences::generate(32);
/// let hits: Vec<usize> = tm.iter()
///     .enumerate()
///     .filter(|(_, &v)| v == 1)
///     .map(|(i, _)| i)
///     .collect();
///
/// comp.track("tm_drums")
///     .drum_grid(32, 0.125)
///     .kick(&hits)
///     .hihat(&sequences::euclidean::generate(16, 32));  // Layer with Euclidean
/// ```
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let tm = sequences::generate(16);
/// // [0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0]
///
/// // Use as rhythm pattern (0 = rest, 1 = hit)
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let pattern: Vec<usize> = sequences::generate(32)
///     .iter()
///     .enumerate()
///     .filter(|(_, &v)| v == 1)
///     .map(|(i, _)| i)
///     .collect();
///
/// comp.track("thue_drums")
///     .drum_grid(32, 0.125)
///     .kick(&pattern);
///
/// // Use for parameter switching
/// let tm_seq = sequences::generate(8);
/// for (i, &val) in tm_seq.iter().enumerate() {
///     let freq = if val == 0 { 440.0 } else { 554.37 };
///     comp.track("alternating").note(&[freq], 0.25);
/// }
/// ```
///
/// # Musical Applications
/// - **Non-repetitive rhythms**: Creates patterns that don't sound mechanical
/// - **Timbral alternation**: Switch between two instruments/sounds fairly
/// - **Accent patterns**: Alternate strong/weak beats without predictability
/// - **Chord voicings**: Alternate between two chord inversions
/// - **Stereo panning**: Fair left/right distribution
/// - **Minimalist composition**: Used by composers like Tom Johnson
///
/// # Why It Matters for Music
/// The Thue-Morse sequence avoids the monotony of simple alternation (0,1,0,1,...)
/// while maintaining perfect fairness. It sounds organic and interesting without
/// being truly random - ideal for generative music that needs structure but
/// wants to avoid repetitive patterns.
pub fn generate(n: usize) -> Vec<u32> {
    let mut seq = vec![0];

    while seq.len() < n {
        let complement: Vec<u32> = seq.iter().map(|&x| 1 - x).collect();
        seq.extend(complement);
    }

    seq.truncate(n);
    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thue_morse_basic() {
        // Test the first few terms match the known sequence
        let tm = generate(16);

        assert_eq!(tm.len(), 16);

        // Known Thue-Morse sequence
        let expected = vec![0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0];
        assert_eq!(tm, expected);
    }

    #[test]
    fn test_thue_morse_construction() {
        // Verify the construction algorithm works
        let tm4 = generate(4);
        assert_eq!(tm4, vec![0, 1, 1, 0]);

        let tm8 = generate(8);
        assert_eq!(tm8, vec![0, 1, 1, 0, 1, 0, 0, 1]);

        // Verify that tm8 = tm4 + complement(tm4)
        let complement: Vec<u32> = tm4.iter().map(|&x| 1 - x).collect();
        let mut expected = tm4.clone();
        expected.extend(complement);
        assert_eq!(tm8, expected);
    }

    #[test]
    fn test_thue_morse_properties() {
        let tm = generate(64);

        // Should have roughly equal 0s and 1s (fairness property)
        let ones = tm.iter().filter(|&&x| x == 1).count();
        let zeros = tm.iter().filter(|&&x| x == 0).count();

        assert_eq!(ones + zeros, 64);
        assert_eq!(ones, zeros); // Exactly equal for power-of-2 lengths
    }

    #[test]
    fn test_thue_morse_no_aaa() {
        // Thue-Morse never has three consecutive identical blocks
        let tm = generate(100);

        // Check no "000" pattern
        for i in 0..tm.len().saturating_sub(2) {
            if tm[i] == 0 && tm[i + 1] == 0 {
                assert_ne!(tm[i + 2], 0, "Found three consecutive 0s at position {}", i);
            }
        }

        // Check no "111" pattern
        for i in 0..tm.len().saturating_sub(2) {
            if tm[i] == 1 && tm[i + 1] == 1 {
                assert_ne!(tm[i + 2], 1, "Found three consecutive 1s at position {}", i);
            }
        }
    }

    #[test]
    fn test_thue_morse_edge_cases() {
        let tm1 = generate(1);
        assert_eq!(tm1, vec![0]);

        let tm2 = generate(2);
        assert_eq!(tm2, vec![0, 1]);

        let tm3 = generate(3);
        assert_eq!(tm3, vec![0, 1, 1]); // Truncated from [0,1,1,0]
    }

    #[test]
    fn test_thue_morse_as_rhythm() {
        // Convert to hit indices like Euclidean rhythms
        let tm = generate(16);
        let hits: Vec<usize> = tm
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| i)
            .collect();

        // Should have 8 hits (half of 16)
        assert_eq!(hits.len(), 8);

        // Hits should be at positions where tm[i] == 1
        // Sequence: [0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0]
        // Indices:   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
        let expected_hits = vec![1, 2, 4, 7, 8, 11, 13, 14];
        assert_eq!(hits, expected_hits);
    }
}
