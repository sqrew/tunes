/// Generate bounded random walk sequence
///
/// Like `random_walk()`, but constrained to stay within a specified range.
/// When the walk would exceed the bounds, it's clamped to min/max.
///
/// This is useful when you want random variation but need to guarantee
/// the values stay within musical constraints (e.g., a specific frequency range,
/// or normalized 0.0-1.0 values for parameters).
///
/// # Arguments
/// * `start` - Initial value (should be within min..max)
/// * `step` - Maximum step size (positive or negative)
/// * `min` - Minimum allowed value
/// * `max` - Maximum allowed value
/// * `steps` - Number of steps to generate
///
/// # Returns
/// Vector of values forming a bounded random walk (all values in min..=max)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Random walk constrained to one octave
/// let melody = sequences::bounded_walk(440.0, 30.0, 220.0, 880.0, 32);
/// // Wanders around 440Hz but stays in 220-880 range
///
/// // Random filter cutoff (normalized 0-1)
/// let cutoff_walk = sequences::bounded_walk(0.5, 0.1, 0.0, 1.0, 64);
/// // Smooth filter movement staying in 0-1 range
///
/// // Use for bass line that stays in key
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(140.0));
/// let bass_notes = sequences::bounded_walk(110.0, 10.0, 80.0, 150.0, 16);
/// comp.instrument("bass", &Instrument::sub_bass())
///     .notes(&bass_notes, 0.25);
/// ```
///
/// # Musical Applications
/// - **Constrained melodies**: Wandering pitch that stays in range
/// - **Bass lines**: Random variation within an octave
/// - **Parameter automation**: Filter, pan, volume staying in valid ranges
/// - **Scale-based melodies**: Step between scale degrees randomly
/// - **Dynamic contrast**: Volume variation within acceptable range
pub fn bounded_walk(start: f32, step: f32, min: f32, max: f32, steps: usize) -> Vec<f32> {
    use rand::Rng;
    let mut rng = rand::rng();
    let start_clamped = start.clamp(min, max);
    let mut seq = vec![start_clamped];

    for _ in 1..steps {
        let delta = rng.random_range(-step..=step);
        let next = (seq.last().unwrap() + delta).clamp(min, max);
        seq.push(next);
    }
    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_walk_stays_in_range() {
        let walk = bounded_walk(50.0, 10.0, 0.0, 100.0, 50);

        assert_eq!(walk.len(), 50);
        assert_eq!(walk[0], 50.0);

        // All values should be within bounds
        for &val in &walk {
            assert!(val >= 0.0 && val <= 100.0, "Value {} out of bounds", val);
        }
    }

    #[test]
    fn test_bounded_walk_clamping() {
        // Start near boundary and use large steps - should clamp
        let walk = bounded_walk(95.0, 20.0, 0.0, 100.0, 20);

        // Should still be within bounds despite large steps
        for &val in &walk {
            assert!(val >= 0.0 && val <= 100.0, "Value {} out of bounds", val);
        }
    }

    #[test]
    fn test_bounded_walk_clamps_initial() {
        // Initial value outside bounds should be clamped
        let walk1 = bounded_walk(-10.0, 5.0, 0.0, 100.0, 10);
        assert_eq!(walk1[0], 0.0); // Clamped to min

        let walk2 = bounded_walk(150.0, 5.0, 0.0, 100.0, 10);
        assert_eq!(walk2[0], 100.0); // Clamped to max
    }

    #[test]
    fn test_bounded_walk_normalized() {
        // Useful for parameter automation (0.0 to 1.0)
        let walk = bounded_walk(0.5, 0.1, 0.0, 1.0, 32);

        for &val in &walk {
            assert!(
                val >= 0.0 && val <= 1.0,
                "Normalized value {} out of range",
                val
            );
        }
    }
}
