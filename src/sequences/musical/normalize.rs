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

/// Normalize a floating-point sequence to a range
///
/// Works just like `normalize()` but for continuous sequences (f32) like Lorenz attractor,
/// Perlin noise, or any other float-based sequence. Perfect for mapping continuous
/// modulation sources to musical parameters.
///
/// # Arguments
/// * `sequence` - The f32 sequence to normalize
/// * `min` - The minimum value in the output range
/// * `max` - The maximum value in the output range
///
/// # Returns
/// Vector of f32 values scaled to the range [min, max]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Normalize Lorenz attractor to pitch range
/// let butterfly = sequences::lorenz_butterfly(100);
/// let x_vals: Vec<f32> = butterfly.iter().map(|(x, _, _)| *x).collect();
/// let melody = sequences::normalize_f32(&x_vals, 220.0, 880.0);
///
/// // Normalize Perlin noise to volume range
/// let noise = sequences::perlin_noise(42, 0.1, 3, 0.5, 64);
/// let volumes = sequences::normalize_f32(&noise, 0.3, 0.8);
///
/// // Map bipolar noise to pan (-1.0 to 1.0)
/// let pan_noise = sequences::perlin_noise_bipolar(7, 0.15, 2, 0.5, 32);
/// let panning = sequences::normalize_f32(&pan_noise, -1.0, 1.0);
/// ```
///
/// # Musical Applications
/// - Map Lorenz coordinates to pitch/volume/filter
/// - Normalize Perlin noise for parameter automation
/// - Scale any continuous modulation to musical ranges
/// - Convert sensor data or external input to musical parameters
pub fn normalize_f32(sequence: &[f32], min: f32, max: f32) -> Vec<f32> {
    if sequence.is_empty() {
        return vec![];
    }

    // Find min and max in the sequence
    let seq_min = sequence.iter().cloned().fold(f32::INFINITY, f32::min);
    let seq_max = sequence.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    // Handle case where all values are the same
    if (seq_max - seq_min).abs() < f32::EPSILON {
        return vec![min; sequence.len()];
    }

    sequence
        .iter()
        .map(|&x| {
            let normalized = (x - seq_min) / (seq_max - seq_min);
            min + normalized * (max - min)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_f32() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let output = normalize_f32(&input, 0.0, 100.0);

        assert_eq!(output[0], 0.0);   // min maps to 0
        assert_eq!(output[4], 100.0); // max maps to 100
        assert_eq!(output[2], 50.0);  // middle maps to 50
    }

    #[test]
    fn test_normalize_f32_negative() {
        let input = vec![-10.0, -5.0, 0.0, 5.0, 10.0];
        let output = normalize_f32(&input, 220.0, 880.0);

        assert_eq!(output[0], 220.0);  // -10 maps to min
        assert_eq!(output[4], 880.0);  // 10 maps to max
        assert_eq!(output[2], 550.0);  // 0 maps to middle
    }

    #[test]
    fn test_normalize_f32_empty() {
        let output = normalize_f32(&[], 0.0, 1.0);
        assert!(output.is_empty());
    }

    #[test]
    fn test_normalize_f32_constant() {
        let input = vec![5.0, 5.0, 5.0];
        let output = normalize_f32(&input, 10.0, 20.0);

        // All same value should map to min
        assert_eq!(output, vec![10.0, 10.0, 10.0]);
    }
}