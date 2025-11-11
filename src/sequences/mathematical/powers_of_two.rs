/// Generate powers of 2 sequence: 1, 2, 4, 8, 16, 32, 64...
///
/// Powers of 2 are fundamental to digital audio, binary systems, and musical structure.
/// The sequence is: 2⁰=1, 2¹=2, 2²=4, 2³=8, 2⁴=16, 2⁵=32...
///
/// This sequence appears everywhere in music and digital systems:
/// - **Octaves**: Each octave is a frequency doubling (power of 2)
/// - **Time signatures**: Common meters use powers of 2 (2/4, 4/4, 8/8, 16/16)
/// - **Note subdivisions**: Whole → half → quarter → eighth → sixteenth
/// - **Digital audio**: Buffer sizes, FFT sizes (512, 1024, 2048, 4096)
/// - **MIDI**: Pitch bend range often uses powers of 2
///
/// # Arguments
/// * `n` - Number of powers of 2 to generate
///
/// # Returns
/// Vector containing [1, 2, 4, 8, 16, 32, ...] (2⁰, 2¹, 2², ...)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let powers = sequences::powers_of_two::generate(8);
/// assert_eq!(powers, vec![1, 2, 4, 8, 16, 32, 64, 128]);
///
/// // Use for rhythmic subdivision (whole, half, quarter, eighth...)
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let subdivisions = sequences::powers_of_two::generate(5); // [1, 2, 4, 8, 16]
/// let durations = sequences::normalize(&subdivisions, 0.0625, 1.0);
///
/// // Use for octave relationships (frequency doubling)
/// let octaves = sequences::powers_of_two::generate(4);
/// let freqs: Vec<f32> = octaves.iter()
///     .map(|&mult| 110.0 * mult as f32)  // A2, A3, A4, A5
///     .collect();
/// comp.track("octaves").notes(&freqs, 0.5);
/// ```
///
/// # Musical Applications
/// - **Rhythmic subdivision**: Whole notes → half → quarter → eighth → sixteenth
/// - **Octave generation**: Multiply base frequency by 2ⁿ for octaves
/// - **Time signatures**: 2/4, 4/4, 8/8 patterns
/// - **Structural proportions**: Section lengths doubling (8 bars, 16 bars, 32 bars)
/// - **Polyrhythms**: Layer patterns with power-of-2 relationships (4 against 8, 8 against 16)
/// - **Digital timing**: Synchronize with digital audio buffer boundaries
pub fn generate(n: usize) -> Vec<u32> {
    (0..n).map(|i| 2u32.pow(i as u32)).collect()
}
