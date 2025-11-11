//! Generate phase-shifting patterns - Steve Reich-style minimalist technique
//!
//! Phase shifting (or phasing) is a compositional technique where two or more identical
//! patterns are played simultaneously, with one gradually shifting out of phase with the other.
//! As the patterns drift apart and realign, complex polyrhythmic textures emerge from simple
//! material.
//!
//! # Historical Context
//!
//! Developed by Steve Reich in the 1960s:
//! - **"It's Gonna Rain" (1965)**: Tape loops gradually going out of phase
//! - **"Piano Phase" (1967)**: Two pianos, one gradually speeds up
//! - **"Clapping Music" (1972)**: Hand claps, one performer shifts by one beat every 12 bars
//! - **"Music for 18 Musicians" (1976)**: Multiple interlocking phasing patterns
//!
//! The technique creates:
//! - **Emergent patterns**: New rhythmic patterns appear from the phase relationship
//! - **Gradual process**: The change is slow enough to hear but fast enough to stay interesting
//! - **Trance-like quality**: Repetition with slow transformation is hypnotic
//! - **Polyrhythmic complexity**: Simple patterns create intricate textures
//!
//! # Musical Applications
//!
//! - **Minimalist composition**: Core technique of minimalism
//! - **Electronic music**: Techno, ambient, IDM use phasing extensively
//! - **Generative music**: Brian Eno's ambient work uses phasing
//! - **Live performance**: Creates constantly evolving texture from fixed material
//!
//! # Examples
//! ```
//! use tunes::sequences;
//!
//! // Simple 4-note pattern
//! let pattern = vec![0, 2, 5, 7];
//!
//! // Generate 8 phase steps (each shifts by 1)
//! let phases = sequences::phase_shift::generate(&pattern, 8, 16);
//!
//! // phases[0] = [0, 2, 5, 7]        (original)
//! // phases[1] = [1, 3, 6, 8]        (shift +1)
//! // phases[2] = [2, 4, 7, 9]        (shift +2)
//! // ... and so on
//!
//! // Reich's "Clapping Music" pattern
//! let clap_pattern = vec![0, 1, 2, 3, 5, 6, 8, 9, 10];
//! let clapping_music = sequences::phase_shift::generate(&clap_pattern, 12, 12);
//!
//! // Use in composition with gradual transition:
//! # use tunes::prelude::*;
//! # let mut comp = Composition::new(Tempo::new(120.0));
//! let pattern = vec![0, 3, 7, 10];
//! let phases = sequences::phase_shift::generate(&pattern, 8, 16);
//!
//! // Voice 1: original pattern, repeating
//! for i in 0..8 {
//!     for &hit in &pattern {
//!         comp.track("voice1")
//!             .at(i as f32 * 2.0 + hit as f32 * 0.125)
//!             .note(&[440.0], 0.1);
//!     }
//! }
//!
//! // Voice 2: gradually shifts through phases
//! for (i, phase_pattern) in phases.iter().enumerate() {
//!     for &hit in phase_pattern {
//!         comp.track("voice2")
//!             .at(i as f32 * 2.0 + hit as f32 * 0.125)
//!             .note(&[660.0], 0.1);
//!     }
//! }
//! ```
//!
//! # References
//! - "Music as a Gradual Process" by Steve Reich (1968)
//! - "Writings on Music, 1965-2000" by Steve Reich
//! - "The Music of Steve Reich" by K. Robert Schwarz
/// Generate a sequence of phase-shifted versions of a pattern
///
/// Takes a rhythmic pattern (as step indices) and generates multiple versions,
/// each shifted by `shift_amount` steps, wrapping around at `cycle_length`.
///
/// # Arguments
/// * `pattern` - Original pattern as step indices
/// * `phases` - Number of phase variations to generate
/// * `cycle_length` - Total pattern length for wrapping (typically 8, 12, or 16)
///
/// # Returns
/// Vec of patterns, each shifted incrementally
///
/// # Examples
/// ```
/// use tunes::sequences::phase_shift::generate;
///
/// // Simple 3-note pattern over 8 steps
/// let pattern = vec![0, 2, 5];
/// let phases = generate(&pattern, 4, 8);
///
/// assert_eq!(phases[0], vec![0, 2, 5]); // Original
/// assert_eq!(phases[1], vec![1, 3, 6]); // +1
/// assert_eq!(phases[2], vec![2, 4, 7]); // +2
/// assert_eq!(phases[3], vec![3, 5, 0]); // +3, wraps around
/// ```
pub fn generate(pattern: &[usize], phases: usize, cycle_length: usize) -> Vec<Vec<usize>> {
    if pattern.is_empty() || cycle_length == 0 {
        return vec![vec![]; phases];
    }

    let mut result = Vec::with_capacity(phases);

    for phase_num in 0..phases {
        let mut shifted_pattern = Vec::with_capacity(pattern.len());

        for &hit in pattern {
            let shifted_hit = (hit + phase_num) % cycle_length;
            shifted_pattern.push(shifted_hit);
        }

        result.push(shifted_pattern);
    }

    result
}

/// Generate Steve Reich's "Clapping Music" pattern with all 12 phases
///
/// The complete pattern from Reich's 1972 piece "Clapping Music". One performer
/// plays the original pattern while the other shifts through all 12 phases,
/// returning to unison at the end.
///
/// Pattern: X.XX.XX.X.X. (dots are rests)
/// As indices: [0, 2, 3, 5, 6, 8, 9, 10]
///
/// # Returns
/// Vec of 13 patterns (original + 12 phases, ending back in unison)
///
/// # Example
/// ```
/// use tunes::sequences::clapping_music;
///
/// let phases = clapping_music();
/// assert_eq!(phases.len(), 13); // Original + 12 shifts + return to original
///
/// // First and last are the same (full cycle)
/// assert_eq!(phases[0], phases[12]);
/// ```
pub fn clapping_music() -> Vec<Vec<usize>> {
    // Reich's actual pattern: X.XX.XX.X.X.
    let pattern = vec![0, 2, 3, 5, 6, 8, 9, 10];

    // Generate 12 phases (which cycles back to original at phase 12)
    let mut phases = generate(&pattern, 12, 12);

    // Add the original again at the end to complete the cycle
    phases.push(pattern);

    phases
}

/// Generate phase shift with custom shift increment
///
/// Instead of shifting by 1 step each phase, shift by a custom amount.
/// Useful for faster or slower phasing effects.
///
/// # Arguments
/// * `pattern` - Original pattern as step indices
/// * `phases` - Number of phase variations to generate
/// * `shift_amount` - How many steps to shift each phase
/// * `cycle_length` - Total pattern length for wrapping
///
/// # Returns
/// Vec of patterns, each shifted by `shift_amount`
///
/// # Examples
/// ```
/// use tunes::sequences::phase_shift_by;
///
/// let pattern = vec![0, 4, 8];
///
/// // Shift by 2 each time instead of 1
/// let phases = phase_shift_by(&pattern, 4, 2, 16);
///
/// assert_eq!(phases[0], vec![0, 4, 8]);   // Original
/// assert_eq!(phases[1], vec![2, 6, 10]);  // +2
/// assert_eq!(phases[2], vec![4, 8, 12]);  // +4
/// assert_eq!(phases[3], vec![6, 10, 14]); // +6
/// ```
pub fn phase_shift_by(
    pattern: &[usize],
    phases: usize,
    shift_amount: usize,
    cycle_length: usize,
) -> Vec<Vec<usize>> {
    if pattern.is_empty() || cycle_length == 0 {
        return vec![vec![]; phases];
    }

    let mut result = Vec::with_capacity(phases);

    for phase_num in 0..phases {
        let mut shifted_pattern = Vec::with_capacity(pattern.len());
        let shift = (phase_num * shift_amount) % cycle_length;

        for &hit in pattern {
            let shifted_hit = (hit + shift) % cycle_length;
            shifted_pattern.push(shifted_hit);
        }

        result.push(shifted_pattern);
    }

    result
}

/// Generate phase shift with gradual time-based transformation
///
/// Creates a series of patterns where each pattern is scheduled at a specific time,
/// useful for creating time-based phase compositions. Returns both the shifted patterns
/// and their starting times.
///
/// # Arguments
/// * `pattern` - Original pattern as step indices
/// * `phases` - Number of phase variations
/// * `cycle_length` - Pattern cycle length
/// * `duration_per_phase` - How long (in seconds/beats) each phase lasts
///
/// # Returns
/// Vec of (start_time, pattern) tuples
///
/// # Example
/// ```
/// use tunes::sequences::phase_shift_timed;
///
/// let pattern = vec![0, 3, 7];
/// let timed_phases = phase_shift_timed(&pattern, 4, 8, 2.0);
///
/// // Each entry is (time, pattern)
/// assert_eq!(timed_phases[0].0, 0.0);   // Starts at t=0
/// assert_eq!(timed_phases[1].0, 2.0);   // Starts at t=2
/// assert_eq!(timed_phases[2].0, 4.0);   // Starts at t=4
/// assert_eq!(timed_phases[3].0, 6.0);   // Starts at t=6
/// ```
pub fn phase_shift_timed(
    pattern: &[usize],
    phases: usize,
    cycle_length: usize,
    duration_per_phase: f32,
) -> Vec<(f32, Vec<usize>)> {
    let shifted_patterns = generate(pattern, phases, cycle_length);

    shifted_patterns
        .into_iter()
        .enumerate()
        .map(|(i, pattern)| (i as f32 * duration_per_phase, pattern))
        .collect()
}

/// Calculate the phase relationship between two patterns
///
/// Given two patterns, determine how many steps pattern B is shifted relative to pattern A.
/// Returns None if the patterns aren't related by a simple shift.
///
/// # Arguments
/// * `pattern_a` - First pattern
/// * `pattern_b` - Second pattern
/// * `cycle_length` - Pattern cycle length for modular arithmetic
///
/// # Returns
/// Some(shift) if patterns are related by shift, None otherwise
///
/// # Example
/// ```
/// use tunes::sequences::phase_relationship;
///
/// let pattern_a = vec![0, 2, 5];
/// let pattern_b = vec![3, 5, 8]; // Shifted by +3
///
/// assert_eq!(phase_relationship(&pattern_a, &pattern_b, 12), Some(3));
///
/// // Different lengths or unrelated patterns return None
/// let pattern_c = vec![0, 2];
/// assert_eq!(phase_relationship(&pattern_a, &pattern_c, 12), None);
/// ```
pub fn phase_relationship(
    pattern_a: &[usize],
    pattern_b: &[usize],
    cycle_length: usize,
) -> Option<usize> {
    // Patterns must be same length to be phase-related
    if pattern_a.len() != pattern_b.len() || pattern_a.is_empty() || cycle_length == 0 {
        return None;
    }

    // Calculate the shift from the first elements
    let potential_shift = if pattern_b[0] >= pattern_a[0] {
        pattern_b[0] - pattern_a[0]
    } else {
        cycle_length - (pattern_a[0] - pattern_b[0])
    };

    // Verify all elements are shifted by the same amount
    for (a, b) in pattern_a.iter().zip(pattern_b.iter()) {
        let expected_b = (a + potential_shift) % cycle_length;
        if *b != expected_b {
            return None;
        }
    }

    Some(potential_shift)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_shift_basic() {
        let pattern = vec![0, 2, 5];
        let phases = generate(&pattern, 3, 8);

        assert_eq!(phases.len(), 3);
        assert_eq!(phases[0], vec![0, 2, 5]);
        assert_eq!(phases[1], vec![1, 3, 6]);
        assert_eq!(phases[2], vec![2, 4, 7]);
    }

    #[test]
    fn test_phase_shift_wrapping() {
        let pattern = vec![7];
        let phases = generate(&pattern, 3, 8);

        assert_eq!(phases[0], vec![7]);
        assert_eq!(phases[1], vec![0]); // Wraps around
        assert_eq!(phases[2], vec![1]);
    }

    #[test]
    fn test_phase_shift_empty_pattern() {
        let pattern = vec![];
        let phases = generate(&pattern, 3, 8);

        assert_eq!(phases.len(), 3);
        for phase in phases {
            assert_eq!(phase, vec![]);
        }
    }

    #[test]
    fn test_phase_shift_zero_cycle() {
        let pattern = vec![0, 2];
        let phases = generate(&pattern, 2, 0);

        assert_eq!(phases.len(), 2);
        for phase in phases {
            assert_eq!(phase, vec![]);
        }
    }

    #[test]
    fn test_clapping_music() {
        let phases = clapping_music();

        assert_eq!(phases.len(), 13); // 12 phases + original at end
        assert_eq!(phases[0], phases[12]); // Full cycle returns to start

        // Pattern should have 8 hits
        for phase in &phases {
            assert_eq!(phase.len(), 8);
        }
    }

    #[test]
    fn test_clapping_music_pattern() {
        let phases = clapping_music();

        // Reich's pattern: X.XX.XX.X.X.
        let expected_original = vec![0, 2, 3, 5, 6, 8, 9, 10];
        assert_eq!(phases[0], expected_original);
    }

    #[test]
    fn test_phase_shift_by() {
        let pattern = vec![0, 4];
        let phases = phase_shift_by(&pattern, 4, 2, 8);

        assert_eq!(phases[0], vec![0, 4]); // +0
        assert_eq!(phases[1], vec![2, 6]); // +2
        assert_eq!(phases[2], vec![4, 0]); // +4, wraps
        assert_eq!(phases[3], vec![6, 2]); // +6
    }

    #[test]
    fn test_phase_shift_by_one_equals_regular() {
        let pattern = vec![0, 3, 7];
        let phases1 = generate(&pattern, 4, 12);
        let phases2 = phase_shift_by(&pattern, 4, 1, 12);

        assert_eq!(phases1, phases2);
    }

    #[test]
    fn test_phase_shift_timed() {
        let pattern = vec![0, 2];
        let timed = phase_shift_timed(&pattern, 3, 8, 1.5);

        assert_eq!(timed.len(), 3);

        assert_eq!(timed[0].0, 0.0);
        assert_eq!(timed[0].1, vec![0, 2]);

        assert_eq!(timed[1].0, 1.5);
        assert_eq!(timed[1].1, vec![1, 3]);

        assert_eq!(timed[2].0, 3.0);
        assert_eq!(timed[2].1, vec![2, 4]);
    }

    #[test]
    fn test_phase_relationship_simple() {
        let pattern_a = vec![0, 2, 5];
        let pattern_b = vec![3, 5, 8];

        assert_eq!(phase_relationship(&pattern_a, &pattern_b, 12), Some(3));
    }

    #[test]
    fn test_phase_relationship_wrapping() {
        let pattern_a = vec![10, 11];
        let pattern_b = vec![0, 1]; // Shifted by +2 with wrap

        assert_eq!(phase_relationship(&pattern_a, &pattern_b, 12), Some(2));
    }

    #[test]
    fn test_phase_relationship_no_shift() {
        let pattern = vec![0, 3, 7];

        assert_eq!(phase_relationship(&pattern, &pattern, 12), Some(0));
    }

    #[test]
    fn test_phase_relationship_unrelated() {
        let pattern_a = vec![0, 2, 4];
        let pattern_b = vec![0, 3, 6]; // Different intervals

        assert_eq!(phase_relationship(&pattern_a, &pattern_b, 12), None);
    }

    #[test]
    fn test_phase_relationship_different_lengths() {
        let pattern_a = vec![0, 2, 4];
        let pattern_b = vec![0, 2];

        assert_eq!(phase_relationship(&pattern_a, &pattern_b, 12), None);
    }

    #[test]
    fn test_phase_relationship_empty() {
        let pattern_a = vec![];
        let pattern_b = vec![];

        assert_eq!(phase_relationship(&pattern_a, &pattern_b, 12), None);
    }

    #[test]
    fn test_phase_shift_full_cycle() {
        let pattern = vec![0, 3];
        let phases = generate(&pattern, 9, 8);

        // Phase 8 should wrap back close to original
        assert_eq!(phases[8], vec![0, 3]); // Full cycle
    }

    #[test]
    fn test_phase_shift_preserves_pattern_length() {
        let pattern = vec![0, 2, 4, 6, 8];
        let phases = generate(&pattern, 10, 16);

        for phase in phases {
            assert_eq!(phase.len(), pattern.len());
        }
    }

    #[test]
    fn test_phase_shift_by_larger_than_cycle() {
        let pattern = vec![0, 4];
        let phases = phase_shift_by(&pattern, 3, 10, 8);

        assert_eq!(phases[0], vec![0, 4]);
        assert_eq!(phases[1], vec![2, 6]); // +10 % 8 = +2
        assert_eq!(phases[2], vec![4, 0]); // +20 % 8 = +4
    }

    #[test]
    fn test_all_phases_have_same_length() {
        let pattern = vec![0, 1, 5, 9];
        let phases = generate(&pattern, 12, 12);

        for phase in phases {
            assert_eq!(phase.len(), pattern.len());
        }
    }

    #[test]
    fn test_phase_shift_single_element() {
        let pattern = vec![3];
        let phases = generate(&pattern, 5, 8);

        assert_eq!(phases[0], vec![3]);
        assert_eq!(phases[1], vec![4]);
        assert_eq!(phases[2], vec![5]);
        assert_eq!(phases[3], vec![6]);
        assert_eq!(phases[4], vec![7]);
    }

    #[test]
    fn test_phase_relationship_full_cycle() {
        let pattern = vec![0, 3, 7];
        let shifted = vec![0, 3, 7]; // Same = 0 shift or full cycle

        // Both 0 and 12 are valid (they're equivalent modulo 12)
        let rel = phase_relationship(&pattern, &shifted, 12);
        assert_eq!(rel, Some(0));
    }
}
