//! Generate clave rhythm patterns - fundamental to Afro-Cuban and Latin music
//!
//! Clave (Spanish for "key") is the rhythmic foundation of Latin music, providing
//! the timeline that all other instruments follow. These patterns create the characteristic
//! tension and release that defines genres like salsa, son, rumba, and bossa nova.
//!
//! The clave is traditionally played on claves (wooden sticks), but its pattern
//! permeates all instruments in the ensemble. Understanding clave is essential
//! for authentic Latin rhythm programming.
//!
//! # Clave Types
//!
//! ## Son Clave
//! The most common clave in salsa, son, and mambo. Has two sides:
//! - **3-2**: Three hits in first bar, two in second (forward/tresillo side)
//! - **2-3**: Two hits in first bar, three in second (reverse side)
//!
//! ## Rumba Clave
//! Similar to son but with the third hit delayed, creating more syncopation.
//! Common in Afro-Cuban rumba, guaguancó, and Columbia.
//!
//! ## Bossa Nova Clave
//! Brazilian variation used in bossa nova and samba. More symmetrical
//! than Cuban claves with a smoother, less syncopated feel.
//!
//! # Musical Context
//! - Typically played over 2 bars (16 steps at 1/8 note resolution)
//! - The "strong" side (usually 3-side) creates more tension
//! - Musicians must be aware of clave direction to maintain groove
//! - Breaking or contradicting the clave sounds wrong to trained ears
//!
//! # Examples
//! ```
//! use tunes::sequences;
//!
//! // Son clave 3-2 (most common in salsa)
//! let pattern = sequences::son_clave_3_2();
//! assert_eq!(pattern, vec![0, 3, 6, 10, 12]);
//!
//! // Son clave 2-3 (reverse)
//! let pattern = sequences::son_clave_2_3();
//! assert_eq!(pattern, vec![0, 2, 6, 9, 12]);
//!
//! // Rumba clave 3-2 (more syncopated)
//! let pattern = sequences::rumba_clave_3_2();
//! assert_eq!(pattern, vec![0, 3, 7, 10, 12]);
//!
//! // Bossa nova (Brazilian feel)
//! let pattern = sequences::bossa_clave();
//! assert_eq!(pattern, vec![0, 3, 6, 10, 13]);
//!
//! // Use in composition:
//! # use tunes::prelude::*;
//! # let mut comp = Composition::new(Tempo::new(120.0));
//! comp.track("clave")
//!     .drum_grid(16, 0.125)
//!     .rimshot(&sequences::son_clave_3_2());
//! ```
//!
//! # References
//! - "The Clave Matrix: Afro-Cuban Rhythm" by David Peñalosa
//! - "Afro-Cuban Rhythms for Drumset" by Frank Malabe & Bob Weiner
/// Son clave in 3-2 direction (forward clave)
///
/// The most fundamental clave pattern in salsa and Afro-Cuban music.
/// Creates the classic "tresillo" feel on the first side.
///
/// Pattern over 16 steps (2 bars of 4/4 at eighth notes):
/// ```text
/// Bar 1: X..X..X.....  (3 hits: 0, 3, 6)
/// Bar 2: ..X.X.......  (2 hits: 10, 12)
/// ```
///
/// # Returns
/// Vec of step indices where clave hits occur: [0, 3, 6, 10, 12]
///
/// # Example
/// ```
/// use tunes::sequences::son_clave_3_2;
///
/// let pattern = son_clave_3_2();
/// assert_eq!(pattern.len(), 5); // 5 total hits
/// assert_eq!(pattern, vec![0, 3, 6, 10, 12]);
/// ```
pub fn son_clave_3_2() -> Vec<usize> {
    vec![0, 3, 6, 10, 12]
}

/// Son clave in 2-3 direction (reverse clave)
///
/// The reverse of the 3-2 son clave. Often used to create variety
/// or match the phrasing of a melody.
///
/// Pattern over 16 steps (2 bars of 4/4 at eighth notes):
/// ```text
/// Bar 1: X.X.........  (2 hits: 0, 2)
/// Bar 2: ..X..X.X....  (3 hits: 6, 9, 12)
/// ```
///
/// # Returns
/// Vec of step indices where clave hits occur: [0, 2, 6, 9, 12]
///
/// # Example
/// ```
/// use tunes::sequences::son_clave_2_3;
///
/// let pattern = son_clave_2_3();
/// assert_eq!(pattern.len(), 5);
/// assert_eq!(pattern, vec![0, 2, 6, 9, 12]);
/// ```
pub fn son_clave_2_3() -> Vec<usize> {
    vec![0, 2, 6, 9, 12]
}

/// Rumba clave in 3-2 direction
///
/// Similar to son clave but with the third hit delayed by one eighth note.
/// This creates more syncopation and is characteristic of rumba and guaguancó.
/// The delay gives it a "tumbling" feel compared to son's more direct groove.
///
/// Pattern over 16 steps (2 bars of 4/4 at eighth notes):
/// ```text
/// Bar 1: X..X...X....  (3 hits: 0, 3, 7)
/// Bar 2: ..X.X.......  (2 hits: 10, 12)
/// ```
///
/// # Returns
/// Vec of step indices where clave hits occur: [0, 3, 7, 10, 12]
///
/// # Example
/// ```
/// use tunes::sequences::rumba_clave_3_2;
///
/// let pattern = rumba_clave_3_2();
/// assert_eq!(pattern.len(), 5);
/// assert_eq!(pattern, vec![0, 3, 7, 10, 12]);
/// ```
pub fn rumba_clave_3_2() -> Vec<usize> {
    vec![0, 3, 7, 10, 12]
}

/// Rumba clave in 2-3 direction
///
/// The reverse of the 3-2 rumba clave.
///
/// Pattern over 16 steps (2 bars of 4/4 at eighth notes):
/// ```text
/// Bar 1: X.X.........  (2 hits: 0, 2)
/// Bar 2: ..X...X.X...  (3 hits: 6, 10, 12)
/// ```
///
/// # Returns
/// Vec of step indices where clave hits occur: [0, 2, 6, 10, 12]
///
/// # Example
/// ```
/// use tunes::sequences::rumba_clave_2_3;
///
/// let pattern = rumba_clave_2_3();
/// assert_eq!(pattern.len(), 5);
/// assert_eq!(pattern, vec![0, 2, 6, 10, 12]);
/// ```
pub fn rumba_clave_2_3() -> Vec<usize> {
    vec![0, 2, 6, 10, 12]
}

/// Bossa nova clave pattern
///
/// Brazilian clave used in bossa nova and samba. More symmetrical and less
/// syncopated than Cuban claves. Creates the characteristic lilting feel
/// of bossa nova with hits falling on different positions relative to son.
///
/// Pattern over 16 steps (2 bars of 4/4 at eighth notes):
/// ```text
/// Bar 1: X..X..X.....  (3 hits: 0, 3, 6)
/// Bar 2: ..X..X......  (2 hits: 10, 13)
/// ```
///
/// # Returns
/// Vec of step indices where clave hits occur: [0, 3, 6, 10, 13]
///
/// # Example
/// ```
/// use tunes::sequences::bossa_clave;
///
/// let pattern = bossa_clave();
/// assert_eq!(pattern.len(), 5);
/// assert_eq!(pattern, vec![0, 3, 6, 10, 13]);
/// ```
pub fn bossa_clave() -> Vec<usize> {
    vec![0, 3, 6, 10, 13]
}

/// Generate a generic clave pattern from hit positions
///
/// Allows creation of custom clave-like patterns or variations. Useful for
/// experimentation or creating hybrid patterns.
///
/// # Arguments
/// * `hits` - Slice of step indices (0-15) where clave hits should occur
/// * `steps` - Total pattern length (typically 16 for 2-bar patterns)
///
/// # Returns
/// Vec of step indices where hits occur, filtered to be within bounds
///
/// # Example
/// ```
/// use tunes::sequences::clave_pattern;
///
/// // Create a custom 6/8 clave pattern
/// let pattern = clave_pattern(&[0, 3, 5, 9, 11], 12);
/// assert_eq!(pattern, vec![0, 3, 5, 9, 11]);
///
/// // Hits beyond bounds are filtered out
/// let pattern = clave_pattern(&[0, 5, 20], 16);
/// assert_eq!(pattern, vec![0, 5]); // 20 is out of bounds
/// ```
pub fn clave_pattern(hits: &[usize], steps: usize) -> Vec<usize> {
    hits.iter().copied().filter(|&h| h < steps).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_son_clave_3_2() {
        let pattern = son_clave_3_2();
        assert_eq!(pattern.len(), 5);
        assert_eq!(pattern, vec![0, 3, 6, 10, 12]);
    }

    #[test]
    fn test_son_clave_2_3() {
        let pattern = son_clave_2_3();
        assert_eq!(pattern.len(), 5);
        assert_eq!(pattern, vec![0, 2, 6, 9, 12]);
    }

    #[test]
    fn test_rumba_clave_3_2() {
        let pattern = rumba_clave_3_2();
        assert_eq!(pattern.len(), 5);
        assert_eq!(pattern, vec![0, 3, 7, 10, 12]);
    }

    #[test]
    fn test_rumba_clave_2_3() {
        let pattern = rumba_clave_2_3();
        assert_eq!(pattern.len(), 5);
        assert_eq!(pattern, vec![0, 2, 6, 10, 12]);
    }

    #[test]
    fn test_bossa_clave() {
        let pattern = bossa_clave();
        assert_eq!(pattern.len(), 5);
        assert_eq!(pattern, vec![0, 3, 6, 10, 13]);
    }

    #[test]
    fn test_all_claves_start_with_zero() {
        // All clave patterns should start on beat 1 (index 0)
        assert_eq!(son_clave_3_2()[0], 0);
        assert_eq!(son_clave_2_3()[0], 0);
        assert_eq!(rumba_clave_3_2()[0], 0);
        assert_eq!(rumba_clave_2_3()[0], 0);
        assert_eq!(bossa_clave()[0], 0);
    }

    #[test]
    fn test_all_claves_have_five_hits() {
        // Standard clave patterns have 5 hits over 16 steps
        assert_eq!(son_clave_3_2().len(), 5);
        assert_eq!(son_clave_2_3().len(), 5);
        assert_eq!(rumba_clave_3_2().len(), 5);
        assert_eq!(rumba_clave_2_3().len(), 5);
        assert_eq!(bossa_clave().len(), 5);
    }

    #[test]
    fn test_son_3_2_vs_2_3_difference() {
        let clave_3_2 = son_clave_3_2();
        let clave_2_3 = son_clave_2_3();

        // They should be different patterns
        assert_ne!(clave_3_2, clave_2_3);

        // Both should have same number of hits
        assert_eq!(clave_3_2.len(), clave_2_3.len());
    }

    #[test]
    fn test_son_vs_rumba_difference() {
        let son = son_clave_3_2();
        let rumba = rumba_clave_3_2();

        // Son and rumba should be different
        assert_ne!(son, rumba);

        // The difference is in the third hit (index 2)
        assert_eq!(son[2], 6); // Son: third hit at 6
        assert_eq!(rumba[2], 7); // Rumba: third hit at 7 (delayed)
    }

    #[test]
    fn test_clave_pattern_custom() {
        let pattern = clave_pattern(&[0, 4, 8, 12], 16);
        assert_eq!(pattern, vec![0, 4, 8, 12]);
    }

    #[test]
    fn test_clave_pattern_filters_out_of_bounds() {
        let pattern = clave_pattern(&[0, 5, 10, 20, 25], 16);
        assert_eq!(pattern, vec![0, 5, 10]);
    }

    #[test]
    fn test_clave_pattern_empty() {
        let pattern = clave_pattern(&[], 16);
        assert_eq!(pattern, vec![]);
    }

    #[test]
    fn test_clave_pattern_all_out_of_bounds() {
        let pattern = clave_pattern(&[20, 30, 40], 16);
        assert_eq!(pattern, vec![]);
    }

    #[test]
    fn test_son_clave_hits_within_bounds() {
        let pattern = son_clave_3_2();
        for &hit in &pattern {
            assert!(hit < 16, "Hit {} should be within 16 steps", hit);
        }
    }

    #[test]
    fn test_rumba_clave_hits_within_bounds() {
        let pattern = rumba_clave_3_2();
        for &hit in &pattern {
            assert!(hit < 16, "Hit {} should be within 16 steps", hit);
        }
    }

    #[test]
    fn test_bossa_clave_hits_within_bounds() {
        let pattern = bossa_clave();
        for &hit in &pattern {
            assert!(hit < 16, "Hit {} should be within 16 steps", hit);
        }
    }

    #[test]
    fn test_clave_patterns_are_sorted() {
        // All clave patterns should return hits in ascending order
        let patterns = vec![
            son_clave_3_2(),
            son_clave_2_3(),
            rumba_clave_3_2(),
            rumba_clave_2_3(),
            bossa_clave(),
        ];

        for pattern in patterns {
            let mut sorted = pattern.clone();
            sorted.sort_unstable();
            assert_eq!(pattern, sorted, "Pattern should be sorted");
        }
    }

    #[test]
    fn test_no_duplicate_hits() {
        // Clave patterns should not have duplicate hits
        let patterns = vec![
            son_clave_3_2(),
            son_clave_2_3(),
            rumba_clave_3_2(),
            rumba_clave_2_3(),
            bossa_clave(),
        ];

        for pattern in patterns {
            let mut unique = pattern.clone();
            unique.dedup();
            assert_eq!(
                pattern.len(),
                unique.len(),
                "Pattern should have no duplicates"
            );
        }
    }
}
