/// Generate Catalan numbers up to n terms
///
/// The Catalan numbers form a sequence of natural numbers that appear in numerous
/// combinatorial problems. Named after Eugène Charles Catalan (1814-1894), they count:
/// - Binary trees with n+1 leaves
/// - Ways to parenthesize expressions
/// - Paths that don't cross the diagonal in a grid
/// - Ways to triangulate a polygon
///
/// The sequence goes: 1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862...
///
/// # Formula
/// C(n) = (2n)! / ((n+1)! × n!)
/// Or recursively: C(0) = 1, C(n+1) = Σ C(i)×C(n-i) for i=0 to n
///
/// # Mathematical Properties
/// - Growth rate: C(n) ~ 4ⁿ / (n^(3/2) × √π)
/// - Faster than Fibonacci, slower than exponential
/// - Central binomial coefficients divided by n+1
/// - Appears in probability, graph theory, and computer science
///
/// # Musical Character
/// Catalan numbers have a beautiful moderate growth rate - not too fast (like 2^n),
/// not too slow (like n²). This makes them ideal for:
/// - Natural-feeling crescendos and accelerandos
/// - Phrase lengths that expand organically
/// - Harmonic complexity that grows but stays manageable
/// - Rhythmic subdivisions that increase gradually
///
/// # Arguments
/// * `n` - Number of Catalan terms to generate (typically 1-15 for musical use)
///
/// # Returns
/// Vector of the first n Catalan numbers: [1, 1, 2, 5, 14, 42, ...]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let catalan = sequences::catalan::generate(8);
/// assert_eq!(catalan, vec![1, 1, 2, 5, 14, 42, 132, 429]);
///
/// // Use for growing rhythmic density
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// // Normalize to note durations
/// let durations = sequences::normalize(&catalan[0..6], 0.0625, 0.5);
/// for (i, &duration) in durations.iter().enumerate() {
///     comp.track("catalan_rhythm")
///         .at(i as f32 * 0.5)
///         .note(&[440.0], duration);
/// }
///
/// // Use for expanding chord voicings
/// let chord_notes = vec![
///     vec![C4],           // C(0) = 1 note
///     vec![C4, E4],       // C(1) = 1, but use 2
///     vec![C4, E4, G4],   // C(2) = 2, but use 3
/// ];
/// ```
///
/// # Musical Applications
/// - **Dynamic expansion**: Use for crescendos with natural feel
/// - **Rhythmic acceleration**: Subdivisions that multiply organically
/// - **Harmonic density**: Number of notes in successive chords
/// - **Phrase lengths**: Measures per section (1, 2, 5, 14 bars)
/// - **Voice leading**: Number of voice movements in counterpoint
/// - **Texture building**: Layer count increasing naturally
/// - **Filter sweeps**: Cutoff frequency changes with organic curve
///
/// # Note on Size
/// Catalan numbers grow quickly! C(15) = 9,694,845. For musical applications,
/// typically use the first 8-12 terms and normalize/scale appropriately.
pub fn generate(n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    let mut cat = vec![1]; // C(0) = 1

    for i in 1..n {
        // Use the recurrence: C(n) = (2(2n-1) / (n+1)) * C(n-1)
        // This avoids computing factorials and is more efficient
        let prev = cat[i - 1] as u64;
        let next = (prev * (4 * i as u64 - 2)) / ((i + 1) as u64);
        cat.push(next as u32);
    }

    cat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalan_basic() {
        let cat = generate(8);
        assert_eq!(cat, vec![1, 1, 2, 5, 14, 42, 132, 429]);
    }

    #[test]
    fn test_catalan_empty() {
        let cat = generate(0);
        assert_eq!(cat, Vec::<u32>::new());
    }

    #[test]
    fn test_catalan_one() {
        let cat = generate(1);
        assert_eq!(cat, vec![1]);
    }

    #[test]
    fn test_catalan_two() {
        let cat = generate(2);
        assert_eq!(cat, vec![1, 1]);
    }

    #[test]
    fn test_catalan_known_values() {
        let cat = generate(10);
        assert_eq!(cat[0], 1);
        assert_eq!(cat[1], 1);
        assert_eq!(cat[2], 2);
        assert_eq!(cat[3], 5);
        assert_eq!(cat[4], 14);
        assert_eq!(cat[5], 42);
        assert_eq!(cat[6], 132);
        assert_eq!(cat[7], 429);
        assert_eq!(cat[8], 1430);
        assert_eq!(cat[9], 4862);
    }

    #[test]
    fn test_catalan_growth() {
        let cat = generate(10);
        // Catalan numbers should grow (except C(0) = C(1))
        for i in 2..cat.len() {
            assert!(cat[i] > cat[i - 1]);
        }
    }

    #[test]
    fn test_catalan_first_values() {
        let cat = generate(6);
        // First few Catalan numbers count specific things:
        // C(0) = 1 (one way to arrange nothing)
        // C(1) = 1 (one binary tree with 2 leaves)
        // C(2) = 2 (two binary trees with 3 leaves)
        // C(3) = 5 (five binary trees with 4 leaves)
        assert_eq!(cat[0], 1);
        assert_eq!(cat[1], 1);
        assert_eq!(cat[2], 2);
        assert_eq!(cat[3], 5);
        assert_eq!(cat[4], 14);
        assert_eq!(cat[5], 42);
    }

    #[test]
    fn test_catalan_moderate_growth() {
        let cat = generate(8);
        // Catalan grows faster than Fibonacci but slower than 2^n
        // Let's verify it's not exponential doubling
        for i in 2..cat.len() {
            let ratio = cat[i] as f32 / cat[i - 1] as f32;
            // Ratio should be between 2 and 5 for these terms
            assert!(ratio > 1.5 && ratio < 5.5);
        }
    }

    #[test]
    fn test_catalan_longer_sequence() {
        let cat = generate(12);
        assert_eq!(cat.len(), 12);
        assert_eq!(cat[11], 58786); // C(11)
    }

    #[test]
    fn test_catalan_third() {
        let cat = generate(4);
        assert_eq!(cat[3], 5);
        // C(3) = 5 counts:
        // - 5 ways to parenthesize 3 binary operations
        // - 5 binary trees with 4 leaves
        // - 5 ways to triangulate a pentagon
    }
}

// ========== PRESETS ==========

/// Short Catalan sequence (8 terms)
pub fn short() -> Vec<u32> {
    generate(8)
}

/// Classic Catalan sequence (10 terms)
pub fn classic() -> Vec<u32> {
    generate(10)
}

/// Extended Catalan sequence (12 terms)
pub fn extended() -> Vec<u32> {
    generate(12)
}
