//! Circle of fifths - key relationships in Western music
//!
//! The circle of fifths is a fundamental concept in music theory showing the relationship
//! between the twelve tones of the chromatic scale through intervals of perfect fifths.
//!
//! Starting from any note and moving up by a perfect fifth (7 semitones) repeatedly,
//! you traverse all 12 chromatic notes before returning to the starting note.
//!
//! # Musical Application
//! - Generate chord progressions (jazz, classical, pop)
//! - Key modulations and transitions
//! - Harmonic exploration and voice leading
//! - Bass lines with strong harmonic movement
//! - Combine with scales for harmonic melodies
//!
//! # Theory
//! Starting from C (0): C(0) → G(7) → D(2) → A(9) → E(4) → B(11) → F#(6) → C#(1) → G#(8) → D#(3) → A#(10) → F(5) → C(0)
//!
//! # Example
//! ```
//! use tunes::sequences::circle_of_fifths;
//! use tunes::consts::{C4, G4, D4, A4, E4};
//!
//! // Generate 8 steps through circle of fifths starting from C
//! let fifths = circle_of_fifths(8, 0);
//! // Returns chromatic intervals: [0, 7, 2, 9, 4, 11, 6, 1]
//! // Which maps to notes: C, G, D, A, E, B, F#, C#
//!
//! // Add to root note to get actual frequencies
//! let root = C4;
//! let notes: Vec<f32> = fifths.iter()
//!     .map(|&semitones| root * 2.0f32.powf(semitones as f32 / 12.0))
//!     .collect();
//! ```

/// Generate a sequence through the circle of fifths
///
/// Returns chromatic semitone offsets (0-11) moving through the circle of fifths.
/// Each step is a perfect fifth (7 semitones) up from the previous note, wrapped to one octave.
///
/// # Arguments
/// * `n` - Number of steps to generate
/// * `start` - Starting chromatic note (0-11, where 0 = C, 1 = C#, etc.)
///
/// # Returns
/// Vector of chromatic semitone offsets (0-11) representing steps through the circle
///
/// # Example
/// ```
/// use tunes::sequences::circle_of_fifths;
///
/// // Starting from C (0)
/// let fifths = circle_of_fifths(5, 0);
/// assert_eq!(fifths, vec![0, 7, 2, 9, 4]);  // C, G, D, A, E
///
/// // Starting from F (5)
/// let fifths_from_f = circle_of_fifths(4, 5);
/// assert_eq!(fifths_from_f, vec![5, 0, 7, 2]);  // F, C, G, D
/// ```
pub fn circle_of_fifths(n: usize, start: u8) -> Vec<u8> {
    let mut result = Vec::with_capacity(n);
    let mut current = start % 12;

    for _ in 0..n {
        result.push(current);
        current = (current + 7) % 12;  // Perfect fifth is 7 semitones
    }

    result
}

/// Generate a sequence through the circle of fourths (reverse of fifths)
///
/// Returns chromatic semitone offsets moving backwards through the circle of fifths.
/// Each step is a perfect fourth (5 semitones) up, or equivalently a fifth down.
///
/// # Arguments
/// * `n` - Number of steps to generate
/// * `start` - Starting chromatic note (0-11)
///
/// # Returns
/// Vector of chromatic semitone offsets (0-11)
///
/// # Example
/// ```
/// use tunes::sequences::circle_of_fourths;
///
/// let fourths = circle_of_fourths(5, 0);
/// assert_eq!(fourths, vec![0, 5, 10, 3, 8]);  // C, F, Bb, Eb, Ab
/// ```
pub fn circle_of_fourths(n: usize, start: u8) -> Vec<u8> {
    let mut result = Vec::with_capacity(n);
    let mut current = start % 12;

    for _ in 0..n {
        result.push(current);
        current = (current + 5) % 12;  // Perfect fourth is 5 semitones
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_of_fifths_from_c() {
        let fifths = circle_of_fifths(12, 0);
        assert_eq!(fifths.len(), 12);
        // Should traverse all 12 chromatic notes
        assert_eq!(fifths[0], 0);   // C
        assert_eq!(fifths[1], 7);   // G
        assert_eq!(fifths[2], 2);   // D
        assert_eq!(fifths[3], 9);   // A
        assert_eq!(fifths[4], 4);   // E
        assert_eq!(fifths[5], 11);  // B
    }

    #[test]
    fn test_circle_of_fifths_wraps() {
        let fifths = circle_of_fifths(13, 0);
        // 13th element should wrap back to start
        assert_eq!(fifths[0], fifths[12]);
    }

    #[test]
    fn test_circle_of_fifths_different_start() {
        let fifths = circle_of_fifths(4, 5);
        assert_eq!(fifths[0], 5);   // F
        assert_eq!(fifths[1], 0);   // C
        assert_eq!(fifths[2], 7);   // G
        assert_eq!(fifths[3], 2);   // D
    }

    #[test]
    fn test_circle_of_fourths_from_c() {
        let fourths = circle_of_fourths(5, 0);
        assert_eq!(fourths[0], 0);   // C
        assert_eq!(fourths[1], 5);   // F
        assert_eq!(fourths[2], 10);  // Bb
        assert_eq!(fourths[3], 3);   // Eb
        assert_eq!(fourths[4], 8);   // Ab
    }

    #[test]
    fn test_circle_of_fourths_is_reverse_fifths() {
        // Moving by fourths forward = moving by fifths backward
        let fourths = circle_of_fourths(12, 0);
        let fifths = circle_of_fifths(12, 0);

        // Both start from the same note (0), then diverge in opposite directions
        assert_eq!(fourths[0], fifths[0]); // Both start at 0

        // The rest of the fourths sequence matches fifths in reverse
        for i in 1..12 {
            assert_eq!(fourths[i], fifths[12 - i]);
        }
    }

    #[test]
    fn test_fifths_covers_all_chromatic() {
        let fifths = circle_of_fifths(12, 0);
        let mut sorted = fifths.clone();
        sorted.sort();

        // Should contain all 12 chromatic notes exactly once
        for i in 0..12 {
            assert_eq!(sorted[i], i as u8);
        }
    }
}
