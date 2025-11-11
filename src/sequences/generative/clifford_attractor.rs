/// Generate sequences from the Clifford attractor
///
/// The Clifford attractor is a 2D iterative map that creates beautiful, flowing
/// "strange attractor" patterns. Unlike the Hénon map's discrete jumps, Clifford produces
/// smooth, organic curves that feel more natural and melodic.
///
/// The system is defined by two coupled equations:
/// ```text
/// x_n+1 = sin(a * y_n) + c * cos(a * x_n)
/// y_n+1 = sin(b * x_n) + d * cos(b * y_n)
/// ```
///
/// The attractor's behavior is highly dependent on parameters a, b, c, d.
/// Different combinations create vastly different patterns - from tight spirals
/// to flowing waves to fractal-like structures.
///
/// Classic parameter sets:
/// - (a=-1.4, b=1.6, c=1.0, d=0.7) - Flowing, organic curves
/// - (a=1.5, b=-1.8, c=1.6, d=0.9) - Tighter, more angular
/// - (a=1.7, b=1.7, c=0.6, d=1.2) - Symmetric spirals
/// - (a=-1.7, b=1.8, c=-0.9, d=-0.4) - Wide, sweeping curves
///
/// # Arguments
/// * `a` - X-Y coupling strength (typical range: -2.0 to 2.0)
/// * `b` - Y-X coupling strength (typical range: -2.0 to 2.0)
/// * `c` - X cosine amplitude (typical range: -2.0 to 2.0)
/// * `d` - Y cosine amplitude (typical range: -2.0 to 2.0)
/// * `initial` - Starting point (x, y). Try (0.0, 0.0)
/// * `n` - Number of iterations to generate
///
/// # Returns
/// Vec of (x, y) coordinates tracing the attractor's path
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Classic Clifford attractor
/// let path = sequences::generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 100);
///
/// // Extract coordinates for musical use
/// let x_vals: Vec<f32> = path.iter().map(|(x, _)| *x).collect();
/// let y_vals: Vec<f32> = path.iter().map(|(_, y)| *y).collect();
///
/// // Normalize to frequency range
/// let melody = sequences::normalize_f32(&x_vals, 220.0, 880.0);
/// let harmony = sequences::normalize_f32(&y_vals, 110.0, 440.0);
///
/// // Create two-voice composition
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// for i in 0..melody.len().min(10) {
///     comp.instrument("lead", &Instrument::synth_lead())
///         .at(i as f32 * 0.25)
///         .note(&[melody[i]], 0.2);
///
///     comp.instrument("bass", &Instrument::sub_bass())
///         .at(i as f32 * 0.25)
///         .note(&[harmony[i]], 0.2);
/// }
/// ```
///
/// # Musical Applications
/// - **Smooth melodies**: Flowing, organic melodic contours
/// - **Counterpoint**: Use x and y for two independent voices
/// - **Harmonic relationships**: Natural intervals emerge from the coupling
/// - **Evolving textures**: Never-repeating ambient patterns
/// - **Modulation paths**: Guide chord progressions through tonal space
/// - **Stereo field**: Map x→left, y→right for spatial movement
/// - **Parameter automation**: Drive filter cutoff, resonance over time
///
/// # Parameter Exploration
/// - **Opposite signs (a,b)**: Creates more chaotic, wandering patterns
/// - **Same signs (a,b)**: Creates more structured, spiral patterns
/// - **c,d near 1.0**: Larger amplitude, wider range
/// - **c,d near 0.0**: Tighter, more compact patterns
/// - **Large |a| or |b|**: More angular, faster changes
/// - **Small |a| or |b|**: Smoother, slower evolution
///
/// # Tips
/// - Coordinates typically range from -2 to 2, perfect for normalization
/// - Try different initial conditions - some parameters are sensitive
/// - Discard first ~10 steps if you want to skip transient behavior
/// - Beautiful when combined with `map_to_scale()` for modal melodies
/// - X and Y often have complementary rhythmic/melodic relationships
pub fn generate(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    initial: (f32, f32),
    n: usize,
) -> Vec<(f32, f32)> {
    let mut path = Vec::with_capacity(n);
    let (mut x, mut y) = initial;

    for _ in 0..n {
        path.push((x, y));

        let x_next = (a * y).sin() + c * (a * x).cos();
        let y_next = (b * x).sin() + d * (b * y).cos();

        x = x_next;
        y = y_next;
    }

    path
}

/// Generate only the x-coordinate sequence from the Clifford attractor
///
/// Convenience function when you only need one dimension of output.
///
/// # Arguments
/// * `a` - X-Y coupling strength
/// * `b` - Y-X coupling strength
/// * `c` - X cosine amplitude
/// * `d` - Y cosine amplitude
/// * `initial` - Starting point (x, y)
/// * `n` - Number of iterations
///
/// # Returns
/// Vector of x values
///
/// # Example
/// ```
/// use tunes::sequences::clifford_x;
///
/// let melody = clifford_x(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 64);
/// assert_eq!(melody.len(), 64);
/// ```
pub fn clifford_x(a: f32, b: f32, c: f32, d: f32, initial: (f32, f32), n: usize) -> Vec<f32> {
    generate(a, b, c, d, initial, n)
        .into_iter()
        .map(|(x, _)| x)
        .collect()
}

/// Generate only the y-coordinate sequence from the Clifford attractor
///
/// Convenience function when you only need one dimension of output.
///
/// # Arguments
/// * `a` - X-Y coupling strength
/// * `b` - Y-X coupling strength
/// * `c` - X cosine amplitude
/// * `d` - Y cosine amplitude
/// * `initial` - Starting point (x, y)
/// * `n` - Number of iterations
///
/// # Returns
/// Vector of y values
///
/// # Example
/// ```
/// use tunes::sequences::clifford_y;
///
/// let harmony = clifford_y(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 64);
/// assert_eq!(harmony.len(), 64);
/// ```
pub fn clifford_y(a: f32, b: f32, c: f32, d: f32, initial: (f32, f32), n: usize) -> Vec<f32> {
    generate(a, b, c, d, initial, n)
        .into_iter()
        .map(|(_, y)| y)
        .collect()
}

/// Generate a Clifford attractor with classic "organic flow" parameters
///
/// Convenience function using parameters (a=-1.4, b=1.6, c=1.0, d=0.7)
/// that produce beautiful, flowing organic patterns.
///
/// # Arguments
/// * `n` - Number of points to generate
///
/// # Returns
/// Vec of (x, y) coordinates with first 10 transient steps removed
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Get 500 points with the classic parameters
/// let flow = sequences::clifford_flow(500);
///
/// // X coordinates for melody (range approximately -2 to 2)
/// let x_vals: Vec<f32> = flow.iter().map(|(x, _)| *x).collect();
/// ```
pub fn clifford_flow(n: usize) -> Vec<(f32, f32)> {
    // Generate extra steps to discard transient
    let total_steps = n + 10;
    let full_path = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), total_steps);

    // Discard first 10 steps (transient)
    full_path.into_iter().skip(10).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clifford_attractor_length() {
        let path = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 100);
        assert_eq!(path.len(), 100);
    }

    #[test]
    fn test_clifford_flow() {
        let path = clifford_flow(50);
        assert_eq!(path.len(), 50);
    }

    #[test]
    fn test_clifford_first_point_is_initial() {
        let path = generate(-1.4, 1.6, 1.0, 0.7, (0.5, 0.5), 10);
        let (x0, y0) = path[0];
        assert_eq!(x0, 0.5);
        assert_eq!(y0, 0.5);
    }

    #[test]
    fn test_clifford_stays_bounded() {
        // Clifford attractor should stay roughly within bounds
        let path = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 1000);

        for (x, y) in path {
            assert!(
                x.abs() < 3.0,
                "X should stay bounded, got {}",
                x
            );
            assert!(
                y.abs() < 3.0,
                "Y should stay bounded, got {}",
                y
            );
        }
    }

    #[test]
    fn test_clifford_deterministic() {
        let path1 = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 100);
        let path2 = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 100);

        // Same parameters should produce identical sequences
        for i in 0..100 {
            let (x1, y1) = path1[i];
            let (x2, y2) = path2[i];

            assert!((x1 - x2).abs() < 1e-6);
            assert!((y1 - y2).abs() < 1e-6);
        }
    }

    #[test]
    fn test_clifford_different_initial_conditions() {
        let path1 = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 50);
        let path2 = generate(-1.4, 1.6, 1.0, 0.7, (0.5, 0.5), 50);

        // After skipping initial condition, paths may converge or diverge
        // Just verify they start different
        let (x1, y1) = path1[0];
        let (x2, y2) = path2[0];

        assert!((x1 - x2).abs() > 0.1 || (y1 - y2).abs() > 0.1);
    }

    #[test]
    fn test_clifford_evolution() {
        let path = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 5);

        // Manually verify first iteration from (0, 0)
        // x1 = sin(-1.4 * 0) + 1.0 * cos(-1.4 * 0) = sin(0) + 1.0 * cos(0) = 0 + 1.0 = 1.0
        // y1 = sin(1.6 * 0) + 0.7 * cos(1.6 * 0) = sin(0) + 0.7 * cos(0) = 0 + 0.7 = 0.7
        let (x1, y1) = path[1];
        assert!((x1 - 1.0).abs() < 0.001, "x1 should be 1.0, got {}", x1);
        assert!((y1 - 0.7).abs() < 0.001, "y1 should be 0.7, got {}", y1);
    }

    #[test]
    fn test_clifford_different_parameters() {
        let path1 = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 100);
        let path2 = generate(1.5, -1.8, 1.6, 0.9, (0.0, 0.0), 100);

        // Different parameters should create different sequences
        let (x1, y1) = path1[50];
        let (x2, y2) = path2[50];

        let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        assert!(dist > 0.1, "Different parameters should produce different trajectories");
    }

    #[test]
    fn test_clifford_x_convenience() {
        let x_only = clifford_x(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 32);
        let full_path = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 32);
        let x_full: Vec<f32> = full_path.into_iter().map(|(x, _)| x).collect();

        assert_eq!(x_only, x_full);
    }

    #[test]
    fn test_clifford_y_convenience() {
        let y_only = clifford_y(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 32);
        let full_path = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 32);
        let y_full: Vec<f32> = full_path.into_iter().map(|(_, y)| y).collect();

        assert_eq!(y_only, y_full);
    }

    #[test]
    fn test_clifford_single_iteration() {
        let path = generate(-1.4, 1.6, 1.0, 0.7, (1.0, 1.0), 1);
        assert_eq!(path.len(), 1);
        let (x0, y0) = path[0];
        assert_eq!(x0, 1.0);
        assert_eq!(y0, 1.0);
    }

    #[test]
    fn test_clifford_coordinates_evolve() {
        let path = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 200);

        // Check that coordinates change over time
        let (x0, y0) = path[0];

        // The attractor may visit the same regions, so check that they're not static
        // by verifying some movement has occurred
        let mut x_changed = false;
        let mut y_changed = false;

        for i in 1..200 {
            let (x, y) = path[i];
            if (x - x0).abs() > 0.1 {
                x_changed = true;
            }
            if (y - y0).abs() > 0.1 {
                y_changed = true;
            }
        }

        assert!(x_changed, "X coordinate should evolve over time");
        assert!(y_changed, "Y coordinate should evolve over time");
    }

    #[test]
    fn test_clifford_produces_finite_values() {
        let path = generate(-1.4, 1.6, 1.0, 0.7, (0.0, 0.0), 1000);

        for (x, y) in path {
            assert!(x.is_finite(), "X should be finite");
            assert!(y.is_finite(), "Y should be finite");
        }
    }

    #[test]
    fn test_clifford_symmetric_parameters() {
        // Test with symmetric parameters
        let path = generate(1.7, 1.7, 0.6, 1.2, (0.0, 0.0), 100);

        // Should still produce bounded, finite values
        for (x, y) in path {
            assert!(x.is_finite() && y.is_finite());
            assert!(x.abs() < 3.0 && y.abs() < 3.0);
        }
    }
}
