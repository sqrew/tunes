/// Generate random walk sequence (Brownian motion)
///
/// A random walk is a path consisting of successive random steps. Each value is
/// the previous value plus a random delta. This creates smooth but unpredictable
/// variation - like a drunk person walking or a particle in fluid (Brownian motion).
///
/// Random walks appear throughout nature and music:
/// - Stock market prices
/// - Particle diffusion
/// - Melodic contours in jazz improvisation
/// - Bass line variation
///
/// # Arguments
/// * `start` - Initial value
/// * `step_size` - Maximum step size (positive or negative)
/// * `steps` - Number of steps to generate
///
/// # Returns
/// Vector of values forming a random walk (unbounded - can go anywhere)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Simple random walk
/// let walk = sequences::random_walk(440.0, 20.0, 20);
/// // Starts at 440, each step changes by up to ±20
///
/// // Use for bass line variation
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let bass_line = sequences::random_walk(110.0, 5.0, 16);
/// comp.instrument("bass", &Instrument::sub_bass())
///     .notes(&bass_line, 0.25);
/// ```
///
/// # Musical Applications
/// - **Melodic variation**: Organic-sounding pitch movement
/// - **Bass lines**: Smooth but unpredictable bass patterns
/// - **Parameter automation**: Filter cutoff, pan, volume variation
/// - **Generative composition**: Non-repetitive sequences
///
/// # Note
/// This is an unbounded walk - values can grow arbitrarily large or small.
/// Use `bounded_walk()` if you need to constrain the range.
pub fn random_walk(start: f32, step_size: f32, steps: usize) -> Vec<f32> {
    use rand::Rng;
    let mut rng = rand::rng();
    let mut seq = vec![start];

    for _ in 1..steps {
        let delta = rng.random_range(-step_size..=step_size);
        let next = seq.last().unwrap() + delta;
        seq.push(next);
    }
    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_walk_basic() {
        let walk = random_walk(100.0, 10.0, 20);

        assert_eq!(walk.len(), 20);
        assert_eq!(walk[0], 100.0); // Starts at initial value

        // Each step should change by at most step_size
        for i in 1..walk.len() {
            let delta = (walk[i] - walk[i - 1]).abs();
            assert!(delta <= 10.0, "Step {} too large: {}", i, delta);
        }
    }

    #[test]
    fn test_random_walk_unbounded() {
        // With enough steps and large step size, should be able to go far from start
        let walk = random_walk(0.0, 50.0, 100);

        assert_eq!(walk.len(), 100);
        assert_eq!(walk[0], 0.0);

        // Very likely (but not guaranteed) to have moved significantly
        // Using a statistical check rather than exact assertion
        let max_dist = walk.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
        assert!(max_dist > 20.0, "Walk should have moved away from start");
    }
}
