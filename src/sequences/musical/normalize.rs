/// Normalize a sequence to a range (map values to a new min/max)
///
/// Takes a sequence of integers and scales them proportionally to fit within a new range.
/// This is essential for converting abstract numeric sequences into musical parameters
/// like frequencies, durations, velocities, or filter cutoffs.
///
/// The normalization preserves the relative proportions of the original sequence:
/// - The smallest input value maps to `min`
/// - The largest input value maps to `max`
/// - All other values are linearly interpolated between them
///
/// # Arguments
/// * `sequence` - The sequence of values to normalize
/// * `min` - The minimum value in the output range
/// * `max` - The maximum value in the output range
///
/// # Returns
/// Vector of f32 values scaled to the range [min, max]
/// - Returns empty vector if input is empty
/// - Returns vector of `min` values if all input values are identical
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Map Fibonacci to frequency range (200-800 Hz)
/// let fib = sequences::fibonacci(8);
/// let freqs = sequences::normalize(&fib, 200.0, 800.0);
/// // Result: [200.0, 200.0, 230.0, 260.0, 320.0, 410.0, 530.0, 800.0]
///
/// // Map to MIDI velocity (0-127)
/// let primes = sequences::primes(10);
/// let velocities = sequences::normalize(&primes, 40.0, 127.0);
///
/// // Map to note durations (eighth to whole note)
/// let seq = sequences::arithmetic(1, 2, 8);
/// let durations = sequences::normalize(&seq, 0.125, 1.0);
///
/// // Use normalized sequence for melody
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let collatz = sequences::collatz(27, 20);
/// let melody = sequences::normalize(&collatz, 220.0, 880.0);
/// comp.track("collatz_melody").notes(&melody, 0.25);
/// ```
///
/// # Musical Applications
/// - **Pitch mapping**: Convert any sequence to frequency range (e.g., 200-800 Hz for melody)
/// - **Rhythm variation**: Map to note durations (0.125 = eighth note, 1.0 = whole note)
/// - **Dynamics**: Convert to velocity/volume levels (0.0-1.0 or MIDI 0-127)
/// - **Parameter automation**: Map to filter cutoff (0.0-1.0), pan (-1.0 to 1.0), etc.
/// - **Microtonal scales**: Distribute values across pitch space
/// - **Tempo changes**: Map to BPM range for tempo modulation
///
/// # Technical Notes
/// This is a linear normalization (min-max scaling). The formula is:
/// ```text
/// normalized = (value - old_min) / (old_max - old_min) * (new_max - new_min) + new_min
/// ```
pub fn normalize(sequence: &[u32], min: f32, max: f32) -> Vec<f32> {
    if sequence.is_empty() {
        return vec![];
    }

    let seq_min = *sequence.iter().min().unwrap() as f32; // Safe: already checked non-empty
    let seq_max = *sequence.iter().max().unwrap() as f32; // Safe: already checked non-empty

    // Handle case where all values are the same (seq_max == seq_min)
    if (seq_max - seq_min).abs() < f32::EPSILON {
        return vec![min; sequence.len()];
    }

    sequence
        .iter()
        .map(|&x| {
            let normalized = (x as f32 - seq_min) / (seq_max - seq_min);
            min + normalized * (max - min)
        })
        .collect()
}