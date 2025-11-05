/// Map a sequence of values to a musical scale
///
/// Takes arbitrary numeric values and quantizes them to notes in a musical scale.
/// This is useful for converting mathematical sequences into melodically pleasing patterns.
///
/// # Arguments
/// * `values` - The input sequence to map
/// * `scale` - The scale to map to (list of semitone offsets from root)
/// * `root` - The root note (MIDI note number)
/// * `octave_range` - How many octaves to span
///
/// # Returns
/// A vector of MIDI note numbers quantized to the scale
///
/// # Example
/// ```
/// use tunes::sequences::{fibonacci, map_to_scale, Scale};
///
/// // Map Fibonacci to C major pentatonic
/// let fib = fibonacci(16);
/// let melody = map_to_scale(&fib, &Scale::major_pentatonic(), 60, 2);
/// // Result: MIDI notes that follow Fibonacci pattern but stay in C major pentatonic
/// ```
pub fn map_to_scale(values: &[u32], scale: &[u32], root: u32, octave_range: u32) -> Vec<u32> {
    if values.is_empty() || scale.is_empty() {
        return vec![];
    }

    let scale_len = scale.len();
    let total_notes = scale_len * octave_range as usize;

    values
        .iter()
        .map(|&val| {
            let idx = (val as usize) % total_notes;
            let octave = idx / scale_len;
            let scale_degree = idx % scale_len;

            root + (octave as u32 * 12) + scale[scale_degree]
        })
        .collect()
}

/// Common musical scales as semitone intervals
pub struct Scale;

impl Scale {
    /// Major pentatonic scale: C D E G A (no half-steps)
    pub fn major_pentatonic() -> Vec<u32> {
        vec![0, 2, 4, 7, 9]
    }

    /// Minor pentatonic scale: C Eb F G Bb (blues scale minus one note)
    pub fn minor_pentatonic() -> Vec<u32> {
        vec![0, 3, 5, 7, 10]
    }

    /// Major (Ionian) scale: C D E F G A B
    pub fn major() -> Vec<u32> {
        vec![0, 2, 4, 5, 7, 9, 11]
    }

    /// Natural minor (Aeolian) scale: C D Eb F G Ab Bb
    pub fn minor() -> Vec<u32> {
        vec![0, 2, 3, 5, 7, 8, 10]
    }

    /// Harmonic minor scale: C D Eb F G Ab B (raised 7th)
    pub fn harmonic_minor() -> Vec<u32> {
        vec![0, 2, 3, 5, 7, 8, 11]
    }

    /// Blues scale: C Eb F F# G Bb (pentatonic + blue note)
    pub fn blues() -> Vec<u32> {
        vec![0, 3, 5, 6, 7, 10]
    }

    /// Chromatic scale: all 12 semitones
    pub fn chromatic() -> Vec<u32> {
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    }

    /// Whole tone scale: C D E F# G# A# (all whole steps)
    pub fn whole_tone() -> Vec<u32> {
        vec![0, 2, 4, 6, 8, 10]
    }

    /// Dorian mode: C D Eb F G A Bb (minor with raised 6th)
    pub fn dorian() -> Vec<u32> {
        vec![0, 2, 3, 5, 7, 9, 10]
    }

    /// Phrygian mode: C Db Eb F G Ab Bb (minor with lowered 2nd)
    pub fn phrygian() -> Vec<u32> {
        vec![0, 1, 3, 5, 7, 8, 10]
    }

    /// Lydian mode: C D E F# G A B (major with raised 4th)
    pub fn lydian() -> Vec<u32> {
        vec![0, 2, 4, 6, 7, 9, 11]
    }

    /// Mixolydian mode: C D E F G A Bb (major with lowered 7th)
    pub fn mixolydian() -> Vec<u32> {
        vec![0, 2, 4, 5, 7, 9, 10]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_to_scale_basic() {
        let seq = vec![0, 1, 2, 3, 4];
        let scale = Scale::major_pentatonic();
        let mapped = map_to_scale(&seq, &scale, 60, 1);
        assert_eq!(mapped, vec![60, 62, 64, 67, 69]);
    }

    #[test]
    fn test_map_to_scale_edge_cases() {
        let empty = map_to_scale(&[], &Scale::major(), 60, 2);
        assert_eq!(empty, Vec::<u32>::new());

        let empty_scale = map_to_scale(&vec![1, 2, 3], &[], 60, 2);
        assert_eq!(empty_scale, Vec::<u32>::new());
    }

    #[test]
    fn test_scale_definitions() {
        assert_eq!(Scale::major_pentatonic().len(), 5);
        assert_eq!(Scale::minor_pentatonic().len(), 5);
        assert_eq!(Scale::major().len(), 7);
        assert_eq!(Scale::minor().len(), 7);
        assert_eq!(Scale::chromatic().len(), 12);

        assert_eq!(Scale::major()[0], 0);
        assert_eq!(Scale::minor()[0], 0);
        assert!(Scale::major().contains(&4));
        assert!(Scale::minor().contains(&3));
    }

    #[test]
    fn test_map_to_scale_with_fibonacci() {
        // Practical example: map Fibonacci to scale
        use crate::sequences::fibonacci;

        let fib = fibonacci(10);
        let scale = Scale::minor_pentatonic();
        let melody = map_to_scale(&fib, &scale, 48, 3); // E3 root, 3 octaves

        assert_eq!(melody.len(), 10);
        // All notes should be >= root note
        for &note in &melody {
            assert!(note >= 48);
        }
        // All notes should be within 3 octaves
        for &note in &melody {
            assert!(note < 48 + 36); // 3 octaves = 36 semitones
        }
    }

    #[test]
    fn test_map_to_scale_wrapping() {
        // Test that values wrap correctly across octaves
        let seq = vec![0, 5, 10]; // Indices that span multiple octaves
        let scale = Scale::major_pentatonic(); // 5 notes per octave
        let mapped = map_to_scale(&seq, &scale, 60, 2);

        // 0 -> C4 (60)
        // 5 -> C5 (60 + 12) = 72 (wraps to next octave)
        // 10 -> C6 + 0 (wraps again)
        assert_eq!(mapped[0], 60); // First note of first octave
        assert_eq!(mapped[1], 72); // First note of second octave
    }
}
