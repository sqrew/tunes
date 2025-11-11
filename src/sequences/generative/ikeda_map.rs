/// Generate sequences from the Ikeda map
///
/// The Ikeda map is a complex-valued discrete-time dynamical system derived from
/// modeling light dynamics in an optical cavity (laser physics). It produces stunning
/// spiral and chaotic patterns with both ordered and turbulent regions.
///
/// The map operates on complex numbers z = x + iy, but we work with real coordinates:
/// ```text
/// t_n = C - D / (1 + x_n² + y_n²)
/// x_n+1 = A + B * (x_n * cos(t_n) - y_n * sin(t_n))
/// y_n+1 = B * (x_n * sin(t_n) + y_n * cos(t_n))
/// ```
///
/// Where:
/// - A: Real offset (drives the system)
/// - B: Amplitude damping (< 1 = decay, = 1 = neutral, > 1 = growth)
/// - C: Phase constant (rotation speed)
/// - D: Nonlinearity strength (controls chaos)
///
/// Classic parameters (Ikeda 1979):
/// - A = 1.0: Driving amplitude
/// - B = 0.9: Slight damping
/// - C = 0.4: Slow rotation
/// - D = 6.0: Strong nonlinearity (creates chaos)
///
/// These produce the iconic spiral attractor with chaotic dynamics.
///
/// # Arguments
/// * `a` - Real offset / driving amplitude (typical: 0.6-1.0)
/// * `b` - Amplitude damping factor (typical: 0.7-0.95)
/// * `c` - Phase constant (typical: 0.4)
/// * `d` - Nonlinearity parameter (typical: 6.0, higher = more chaos)
/// * `initial` - Starting point (x, y). Try (0.0, 0.0) or (0.1, 0.1)
/// * `n` - Number of iterations to generate
///
/// # Returns
/// Vec of (x, y) coordinates tracing the attractor's path
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Classic Ikeda spiral with chaos
/// let path = sequences::ikeda_map::generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 100);
///
/// // Extract coordinates for musical use
/// let x_vals: Vec<f32> = path.iter().map(|(x, _)| *x).collect();
/// let y_vals: Vec<f32> = path.iter().map(|(_, y)| *y).collect();
///
/// // Normalize to frequency range
/// let melody = sequences::normalize_f32(&x_vals, 220.0, 880.0);
///
/// // Use y for stereo panning (-1 to 1)
/// let panning = sequences::normalize_f32(&y_vals, -1.0, 1.0);
///
/// // Create spatial composition
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// for i in 0..melody.len().min(10) {
///     comp.instrument("ikeda", &Instrument::synth_lead())
///         .at(i as f32 * 0.2)
///         .pan(panning[i])
///         .note(&[melody[i]], 0.15);
/// }
/// ```
///
/// # Musical Applications
/// - **Spiral melodies**: Natural spiraling pitch contours
/// - **Rhythmic bursts**: Clusters of dense activity followed by sparse regions
/// - **Spatial movement**: X/Y coordinates map beautifully to stereo field
/// - **Dynamic sequences**: Alternates between ordered and chaotic sections
/// - **Percussion patterns**: Y coordinate creates interesting rhythmic triggers
/// - **Phase modulation**: Natural for FM synthesis parameters
/// - **Feedback effects**: Models well with delay/reverb (optical cavity inspiration)
///
/// # Parameter Exploration
/// - **A < 1.0**: System tends toward fixed points (less chaotic)
/// - **A = 1.0**: Classic chaotic behavior
/// - **A > 1.0**: More chaotic, wider exploration
/// - **B < 0.9**: Converges faster, tighter spirals
/// - **B = 0.9**: Classic parameters
/// - **B > 0.9**: Expands outward, may diverge if too high
/// - **D < 6.0**: More periodic behavior
/// - **D = 6.0**: Sweet spot for chaos
/// - **D > 6.0**: More complex, finer structure
///
/// # Tips
/// - Coordinates typically range from -2 to 2, perfect for normalization
/// - The map creates clusters and voids - great for rhythmic density variation
/// - Try different initial conditions - the basin of attraction is fractal
/// - Beautiful when combined with `map_to_scale()` for constrained pitch sets
/// - X tends to spiral, Y oscillates - complementary melodic/rhythmic roles
/// - Inspired by optical cavities - pairs well with reverb and delay effects!
pub fn generate(a: f32, b: f32, c: f32, d: f32, initial: (f32, f32), n: usize) -> Vec<(f32, f32)> {
    let mut path = Vec::with_capacity(n);
    let (mut x, mut y) = initial;

    for _ in 0..n {
        path.push((x, y));

        // Compute the phase term
        let denominator = 1.0 + x * x + y * y;
        let t = c - d / denominator;

        // Apply rotation and scaling
        let cos_t = t.cos();
        let sin_t = t.sin();

        let x_next = a + b * (x * cos_t - y * sin_t);
        let y_next = b * (x * sin_t + y * cos_t);

        x = x_next;
        y = y_next;
    }

    path
}

/// Generate only the x-coordinate sequence from the Ikeda map
///
/// Convenience function when you only need one dimension of output.
/// The x coordinate tends to create spiral patterns.
///
/// # Arguments
/// * `a` - Real offset / driving amplitude
/// * `b` - Amplitude damping factor
/// * `c` - Phase constant
/// * `d` - Nonlinearity parameter
/// * `initial` - Starting point (x, y)
/// * `n` - Number of iterations
///
/// # Returns
/// Vector of x values
///
/// # Example
/// ```
/// use tunes::sequences::ikeda_x;
///
/// let spiral_melody = ikeda_x(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 64);
/// assert_eq!(spiral_melody.len(), 64);
/// ```
pub fn ikeda_x(a: f32, b: f32, c: f32, d: f32, initial: (f32, f32), n: usize) -> Vec<f32> {
    generate(a, b, c, d, initial, n)
        .into_iter()
        .map(|(x, _)| x)
        .collect()
}

/// Generate only the y-coordinate sequence from the Ikeda map
///
/// Convenience function when you only need one dimension of output.
/// The y coordinate tends to oscillate and create rhythmic patterns.
///
/// # Arguments
/// * `a` - Real offset / driving amplitude
/// * `b` - Amplitude damping factor
/// * `c` - Phase constant
/// * `d` - Nonlinearity parameter
/// * `initial` - Starting point (x, y)
/// * `n` - Number of iterations
///
/// # Returns
/// Vector of y values
///
/// # Example
/// ```
/// use tunes::sequences::ikeda_y;
///
/// let rhythm = ikeda_y(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 64);
/// assert_eq!(rhythm.len(), 64);
/// ```
pub fn ikeda_y(a: f32, b: f32, c: f32, d: f32, initial: (f32, f32), n: usize) -> Vec<f32> {
    generate(a, b, c, d, initial, n)
        .into_iter()
        .map(|(_, y)| y)
        .collect()
}

/// Generate an Ikeda map with classic "chaotic spiral" parameters
///
/// Convenience function using the classic Ikeda parameters (a=1.0, b=0.9, c=0.4, d=6.0)
/// that produce the iconic spiral attractor with chaotic dynamics.
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
/// // Get 500 points with the classic Ikeda parameters
/// let spiral = sequences::ikeda_spiral(500);
///
/// // X coordinates for spiraling melody (range approximately -2 to 2)
/// let x_vals: Vec<f32> = spiral.iter().map(|(x, _)| *x).collect();
/// ```
pub fn ikeda_spiral(n: usize) -> Vec<(f32, f32)> {
    // Generate extra steps to discard transient
    let total_steps = n + 10;
    let full_path = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), total_steps);

    // Discard first 10 steps (transient)
    full_path.into_iter().skip(10).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ikeda_map_length() {
        let path = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 100);
        assert_eq!(path.len(), 100);
    }

    #[test]
    fn test_ikeda_spiral() {
        let path = ikeda_spiral(50);
        assert_eq!(path.len(), 50);
    }

    #[test]
    fn test_ikeda_first_point_is_initial() {
        let path = generate(1.0, 0.9, 0.4, 6.0, (0.5, 0.5), 10);
        let (x0, y0) = path[0];
        assert_eq!(x0, 0.5);
        assert_eq!(y0, 0.5);
    }

    #[test]
    fn test_ikeda_stays_bounded() {
        // Ikeda map with classic parameters should stay roughly bounded
        let path = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 1000);

        for (x, y) in path {
            assert!(
                x.abs() < 5.0,
                "X should stay bounded, got {}",
                x
            );
            assert!(
                y.abs() < 5.0,
                "Y should stay bounded, got {}",
                y
            );
        }
    }

    #[test]
    fn test_ikeda_deterministic() {
        let path1 = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 100);
        let path2 = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 100);

        // Same parameters should produce identical sequences
        for i in 0..100 {
            let (x1, y1) = path1[i];
            let (x2, y2) = path2[i];

            assert!((x1 - x2).abs() < 1e-6);
            assert!((y1 - y2).abs() < 1e-6);
        }
    }

    #[test]
    fn test_ikeda_different_initial_conditions() {
        let path1 = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 100);
        let path2 = generate(1.0, 0.9, 0.4, 6.0, (0.5, 0.5), 100);

        // Different initial conditions should produce different paths
        let (x1_end, y1_end) = path1[99];
        let (x2_end, y2_end) = path2[99];

        let dist = ((x2_end - x1_end).powi(2) + (y2_end - y1_end).powi(2)).sqrt();
        assert!(dist > 0.1, "Different initial conditions should diverge");
    }

    #[test]
    fn test_ikeda_evolution() {
        let path = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 5);

        // Manually verify first iteration from (0, 0)
        // t0 = 0.4 - 6.0 / (1 + 0 + 0) = 0.4 - 6.0 = -5.6
        // x1 = 1.0 + 0.9 * (0 * cos(-5.6) - 0 * sin(-5.6)) = 1.0
        // y1 = 0.9 * (0 * sin(-5.6) + 0 * cos(-5.6)) = 0.0
        let (x1, y1) = path[1];
        assert!((x1 - 1.0).abs() < 0.001, "x1 should be 1.0, got {}", x1);
        assert!((y1 - 0.0).abs() < 0.001, "y1 should be 0.0, got {}", y1);
    }

    #[test]
    fn test_ikeda_different_a_values() {
        let path1 = generate(0.8, 0.9, 0.4, 6.0, (0.0, 0.0), 100);
        let path2 = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 100);

        // Different A values should create different sequences
        let (x1, y1) = path1[50];
        let (x2, y2) = path2[50];

        let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        assert!(dist > 0.1, "Different A values should produce different trajectories");
    }

    #[test]
    fn test_ikeda_different_d_values() {
        let path1 = generate(1.0, 0.9, 0.4, 4.0, (0.0, 0.0), 100);
        let path2 = generate(1.0, 0.9, 0.4, 8.0, (0.0, 0.0), 100);

        // Different D values should create different dynamics
        let (x1, y1) = path1[50];
        let (x2, y2) = path2[50];

        let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        assert!(dist > 0.1, "Different D values should produce different trajectories");
    }

    #[test]
    fn test_ikeda_x_convenience() {
        let x_only = ikeda_x(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 32);
        let full_path = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 32);
        let x_full: Vec<f32> = full_path.into_iter().map(|(x, _)| x).collect();

        assert_eq!(x_only, x_full);
    }

    #[test]
    fn test_ikeda_y_convenience() {
        let y_only = ikeda_y(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 32);
        let full_path = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 32);
        let y_full: Vec<f32> = full_path.into_iter().map(|(_, y)| y).collect();

        assert_eq!(y_only, y_full);
    }

    #[test]
    fn test_ikeda_single_iteration() {
        let path = generate(1.0, 0.9, 0.4, 6.0, (1.0, 1.0), 1);
        assert_eq!(path.len(), 1);
        let (x0, y0) = path[0];
        assert_eq!(x0, 1.0);
        assert_eq!(y0, 1.0);
    }

    #[test]
    fn test_ikeda_coordinates_evolve() {
        let path = generate(1.0, 0.9, 0.4, 6.0, (0.1, 0.1), 100);

        // Check that coordinates change over time
        let (x0, y0) = path[0];
        let (x99, y99) = path[99];

        assert!((x99 - x0).abs() > 0.1, "X coordinate should evolve");
        assert!((y99 - y0).abs() > 0.1, "Y coordinate should evolve");
    }

    #[test]
    fn test_ikeda_produces_finite_values() {
        let path = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 1000);

        for (x, y) in path {
            assert!(x.is_finite(), "X should be finite");
            assert!(y.is_finite(), "Y should be finite");
        }
    }

    #[test]
    fn test_ikeda_small_b_converges() {
        // With smaller B (more damping), system should converge
        let path = generate(1.0, 0.5, 0.4, 6.0, (2.0, 2.0), 100);

        // Should stay bounded and finite
        for (x, y) in path {
            assert!(x.is_finite() && y.is_finite());
            assert!(x.abs() < 5.0 && y.abs() < 5.0);
        }
    }

    #[test]
    fn test_ikeda_phase_parameter() {
        // Different C (phase) values should affect the dynamics
        let path1 = generate(1.0, 0.9, 0.2, 6.0, (0.0, 0.0), 100);
        let path2 = generate(1.0, 0.9, 0.6, 6.0, (0.0, 0.0), 100);

        // Should produce different trajectories
        let (x1, y1) = path1[50];
        let (x2, y2) = path2[50];

        let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        assert!(dist > 0.1, "Different C values should produce different trajectories");
    }

    #[test]
    fn test_ikeda_chaotic_behavior() {
        // Small difference in initial conditions should lead to divergence
        let path1 = generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 200);
        let path2 = generate(1.0, 0.9, 0.4, 6.0, (0.001, 0.0), 200);

        // Paths should diverge (butterfly effect)
        let (x1, y1) = path1[199];
        let (x2, y2) = path2[199];

        let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        assert!(
            distance > 0.01,
            "Ikeda map should be sensitive to initial conditions, distance was {}",
            distance
        );
    }
}

// ========== PRESETS ==========

/// Classic Ikeda map (a=1.0, b=0.9, c=0.4, d=6.0)
pub fn classic() -> Vec<(f32, f32)> {
    generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 150)
}

/// Chaotic Ikeda (higher nonlinearity d=7.0)
pub fn chaotic() -> Vec<(f32, f32)> {
    generate(1.0, 0.9, 0.4, 7.0, (0.0, 0.0), 200)
}

/// Extended Ikeda (classic parameters, more iterations)
pub fn extended() -> Vec<(f32, f32)> {
    generate(1.0, 0.9, 0.4, 6.0, (0.0, 0.0), 300)
}
