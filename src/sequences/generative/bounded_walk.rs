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
/// * `start` - Initial value (will be clamped to min..max if outside)
/// * `step` - Maximum step size per iteration (controls variation speed)
/// * `min` - Minimum allowed value (hard floor)
/// * `max` - Maximum allowed value (hard ceiling)
/// * `steps` - Number of steps to generate
///
/// # Returns
/// Vector of values forming a bounded random walk (all values in min..=max)
///
/// # Typical Parameters
///
/// **step size** (relative to range):
/// - **Small (5-10% of range)**: Slow, smooth exploration
/// - **Medium (15-25% of range)**: Natural variation
/// - **Large (30-50% of range)**: Jumpy, energetic movement
///
/// **Common ranges:**
/// - One octave: min=220, max=440, step=20-40
/// - Two octaves: min=220, max=880, step=40-80
/// - Normalized (0-1): min=0.0, max=1.0, step=0.05-0.15
///
/// # Recipe: Melody in Range
///
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences;
///
/// let mut comp = Composition::new(Tempo::new(120.0));
///
/// // Wandering melody in 2-octave range
/// let melody = sequences::bounded_walk(
///     440.0,   // Start at A4
///     35.0,    // Steps of up to 35 Hz
///     220.0,   // Min: A3
///     880.0,   // Max: A5
///     32       // 32 notes
/// );
///
/// comp.instrument("lead", &Instrument::synth_lead())
///     .delay(Delay::new(0.375, 0.3, 0.5))
///     .notes(&melody, 0.25);
/// ```
///
/// # Recipe: Bass Line in Octave
///
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences;
///
/// let mut comp = Composition::new(Tempo::new(140.0));
///
/// // Bass that stays in one octave
/// let bass = sequences::bounded_walk(
///     110.0,  // A2
///     8.0,    // Small steps for smooth bass
///     82.4,   // E2 (low end)
///     164.8,  // E3 (high end)
///     16
/// );
///
/// comp.instrument("bass", &Instrument::sub_bass())
///     .notes(&bass, 0.5);
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
