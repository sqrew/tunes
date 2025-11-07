/// Map a sequence of integer values to a musical scale
///
/// Takes arbitrary numeric values and quantizes them to notes in a musical scale.
/// This is useful for converting mathematical sequences into melodically pleasing patterns.
///
/// **Returns frequencies directly** - No need for MIDI conversion!
///
/// # Arguments
/// * `values` - The input sequence to map
/// * `scale` - The scale to map to (list of semitone offsets from root)
/// * `root` - The root note frequency (use constants like C4, D4, etc.)
/// * `octave_range` - How many octaves to span
///
/// # Returns
/// A vector of frequencies (f32) quantized to the scale - ready to use directly!
///
/// # Example
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences::{fibonacci, map_to_scale, Scale};
///
/// // Map Fibonacci to C major pentatonic - use note constants!
/// let fib = fibonacci(16);
/// let melody = map_to_scale(&fib, &Scale::major_pentatonic(), C4, 2);
/// // Result: Frequencies following Fibonacci pattern but staying in C major pentatonic
///
/// // Use directly - no conversion needed!
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// comp.track("fib").notes(&melody, 0.25);
/// ```
pub fn map_to_scale(values: &[u32], scale: &[u32], root: f32, octave_range: u32) -> Vec<f32> {
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

            // Calculate frequency using equal temperament
            let semitones = (octave * 12) as f32 + scale[scale_degree] as f32;
            root * 2_f32.powf(semitones / 12.0)
        })
        .collect()
}

/// Map a sequence of floating-point values to a musical scale
///
/// Just like `map_to_scale()` but for continuous sequences (f32) like Lorenz attractor,
/// Perlin noise, or circle maps. Automatically normalizes the input range to map evenly
/// across the scale and octave range.
///
/// **Returns frequencies directly** - No need for MIDI conversion!
///
/// Perfect for quantizing chaotic or continuous generators to musical scales!
///
/// # Arguments
/// * `values` - The f32 sequence to map
/// * `scale` - The scale to map to (list of semitone offsets from root)
/// * `root` - The root note frequency (use constants like C4, D4, etc.)
/// * `octave_range` - How many octaves to span
///
/// # Returns
/// A vector of frequencies (f32) quantized to the scale - ready to use directly!
///
/// # Examples
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences;
///
/// // Map Lorenz attractor to D minor - use note constants!
/// let butterfly = sequences::lorenz_butterfly(100);
/// let x_vals: Vec<f32> = butterfly.iter().map(|(x, _, _)| *x).collect();
/// let melody = sequences::map_to_scale_f32(&x_vals, &sequences::Scale::minor(), D4, 2);
/// // Result: D minor frequencies following the chaotic attractor's path
///
/// // Map Perlin noise to C major pentatonic
/// let noise = sequences::perlin_noise(42, 0.15, 3, 0.5, 64);
/// let melody = sequences::map_to_scale_f32(&noise, &sequences::Scale::major_pentatonic(), C4, 3);
///
/// // Map circle map phases to blues scale
/// let phases = sequences::circle_map(0.618, 1.5, 0.0, 32);
/// let melody = sequences::map_to_scale_f32(&phases, &sequences::Scale::blues(), E3, 2);
///
/// // Use directly - no conversion needed!
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// comp.track("chaos").notes(&melody, 0.25);
/// ```
///
/// # Musical Applications
/// - Lorenz attractor melodies that stay in key
/// - Perlin noise for evolving scale-based patterns
/// - Circle map for quantized rhythmic melodies
/// - Any continuous generator + musical scales
///
/// # Technical Notes
/// The function first normalizes the input to [0, 1], then maps to scale indices.
/// The smallest input value maps to the root note, the largest to the top of the range.
/// Frequencies are calculated using equal temperament: freq = root * 2^(semitones/12)
pub fn map_to_scale_f32(values: &[f32], scale: &[u32], root: f32, octave_range: u32) -> Vec<f32> {
    if values.is_empty() || scale.is_empty() {
        return vec![];
    }

    // Find min and max for normalization
    let min = values.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    // Handle constant sequence
    if (max - min).abs() < f32::EPSILON {
        return vec![root; values.len()];
    }

    let scale_len = scale.len();
    let total_notes = scale_len * octave_range as usize;

    values
        .iter()
        .map(|&val| {
            // Normalize to [0, 1]
            let normalized = (val - min) / (max - min);

            // Map to scale index
            let idx = (normalized * (total_notes - 1) as f32).round() as usize;
            let idx = idx.min(total_notes - 1); // Clamp to valid range

            let octave = idx / scale_len;
            let scale_degree = idx % scale_len;

            // Calculate frequency using equal temperament
            let semitones = (octave * 12) as f32 + scale[scale_degree] as f32;
            root * 2_f32.powf(semitones / 12.0)
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
        use crate::consts::*;

        let seq = vec![0, 1, 2, 3, 4];
        let scale = Scale::major_pentatonic(); // C D E G A
        let mapped = map_to_scale(&seq, &scale, C4, 1);

        assert_eq!(mapped.len(), 5);
        assert!((mapped[0] - C4).abs() < 0.1);
        assert!((mapped[1] - D4).abs() < 0.1);
        assert!((mapped[2] - E4).abs() < 0.1);
        assert!((mapped[3] - G4).abs() < 0.1);
        assert!((mapped[4] - A4).abs() < 0.1);
    }

    #[test]
    fn test_map_to_scale_edge_cases() {
        use crate::consts::*;

        let empty = map_to_scale(&[], &Scale::major(), C4, 2);
        assert_eq!(empty, Vec::<f32>::new());

        let empty_scale = map_to_scale(&vec![1, 2, 3], &[], C4, 2);
        assert_eq!(empty_scale, Vec::<f32>::new());
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
        use crate::consts::*;
        use crate::sequences::fibonacci;

        // Practical example: map Fibonacci to scale
        let fib = fibonacci(10);
        let scale = Scale::minor_pentatonic();
        let melody = map_to_scale(&fib, &scale, E3, 3); // E3 root, 3 octaves

        assert_eq!(melody.len(), 10);
        // All frequencies should be >= root frequency
        for &freq in &melody {
            assert!(freq >= E3);
        }
        // All frequencies should be within 3 octaves (8x the root)
        for &freq in &melody {
            assert!(freq < E3 * 8.0);
        }
    }

    #[test]
    fn test_map_to_scale_wrapping() {
        use crate::consts::*;

        // Test that values wrap correctly across octaves
        let seq = vec![0, 5, 10]; // Indices that span multiple octaves
        let scale = Scale::major_pentatonic(); // 5 notes per octave
        let mapped = map_to_scale(&seq, &scale, C4, 2);

        // 0 -> C4
        // 5 -> C5 (wraps to next octave)
        // 10 -> wraps again
        assert!((mapped[0] - C4).abs() < 0.1); // First note of first octave
        assert!((mapped[1] - C5).abs() < 0.1); // First note of second octave
    }

    // Tests for map_to_scale_f32
    #[test]
    fn test_map_to_scale_f32_basic() {
        use crate::consts::*;

        // Values from 0.0 to 1.0 should map across the scale
        let seq = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let scale = Scale::major_pentatonic(); // 5 notes: C D E G A
        let mapped = map_to_scale_f32(&seq, &scale, C4, 1);

        // Should map to all 5 notes of the scale
        assert_eq!(mapped.len(), 5);
        assert!((mapped[0] - C4).abs() < 0.1); // Lowest maps to C4
        assert!((mapped[4] - A4).abs() < 0.1); // Highest maps to A4
    }

    #[test]
    fn test_map_to_scale_f32_normalization() {
        use crate::consts::*;

        // Any range should normalize to scale
        let seq = vec![-10.0, -5.0, 0.0, 5.0, 10.0];
        let scale = Scale::major_pentatonic();
        let mapped = map_to_scale_f32(&seq, &scale, C4, 1);

        assert!((mapped[0] - C4).abs() < 0.1); // Min maps to root
        assert!((mapped[4] - A4).abs() < 0.1); // Max maps to top
    }

    #[test]
    fn test_map_to_scale_f32_edge_cases() {
        use crate::consts::*;

        // Empty sequence
        let empty = map_to_scale_f32(&[], &Scale::major(), C4, 2);
        assert_eq!(empty, Vec::<f32>::new());

        // Empty scale
        let empty_scale = map_to_scale_f32(&vec![1.0, 2.0, 3.0], &[], C4, 2);
        assert_eq!(empty_scale, Vec::<f32>::new());

        // Constant sequence (all same value)
        let constant = map_to_scale_f32(&vec![5.0, 5.0, 5.0], &Scale::major(), C4, 2);
        for &freq in &constant {
            assert!((freq - C4).abs() < 0.1); // All map to root
        }
    }

    #[test]
    fn test_map_to_scale_f32_with_lorenz() {
        use crate::consts::*;
        use crate::sequences::lorenz_butterfly;

        // Practical example: Lorenz-like values
        let butterfly = lorenz_butterfly(50);
        let x_vals: Vec<f32> = butterfly.iter().map(|(x, _, _)| *x).collect();

        let scale = Scale::minor();
        let melody = map_to_scale_f32(&x_vals, &scale, E3, 3);

        assert_eq!(melody.len(), 50);
        // All frequencies should be >= root frequency
        for &freq in &melody {
            assert!(freq >= E3);
        }
        // All frequencies should be within 3 octaves (8x the root)
        for &freq in &melody {
            assert!(freq < E3 * 8.0);
        }
    }

    #[test]
    fn test_map_to_scale_f32_octave_spanning() {
        use crate::consts::*;

        // Values should span multiple octaves
        let seq = vec![0.0, 0.5, 1.0];
        let scale = Scale::major_pentatonic(); // 5 notes
        let mapped = map_to_scale_f32(&seq, &scale, C4, 2); // 2 octaves

        assert!((mapped[0] - C4).abs() < 0.1); // First note
        // Middle value should be roughly in the middle
        assert!(mapped[1] > C4 && mapped[1] < C4 * 4.0);
        // Last note should be near A5 (C4 * 2^(21/12))
        assert!((mapped[2] - (C4 * 2_f32.powf(21.0/12.0))).abs() < 1.0);
    }

    #[test]
    fn test_map_to_scale_f32_stays_in_scale() {
        use crate::consts::*;

        // All output should be in the scale
        let seq = vec![0.1, 0.3, 0.5, 0.7, 0.9];
        let scale = Scale::major(); // C D E F G A B
        let mapped = map_to_scale_f32(&seq, &scale, C4, 1);

        // All frequencies should be scale degrees of C major
        let valid_freqs = vec![C4, D4, E4, F4, G4, A4, B4];
        for &freq in &mapped {
            let is_valid = valid_freqs.iter().any(|&v| (freq - v).abs() < 0.1);
            assert!(is_valid, "Frequency {} not in C major scale", freq);
        }
    }
}
