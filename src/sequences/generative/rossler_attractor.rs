/// Generate sequences from the Rössler attractor
///
/// The Rössler attractor is a 3D continuous chaotic system that creates a single-banded
/// spiral pattern. Simpler than the Lorenz attractor, it produces smooth, flowing trajectories
/// with a distinctive spiral structure perfect for melodic lines with periodic-like character
/// but chaotic variation.
///
/// The system is defined by three coupled differential equations:
/// ```text
/// dx/dt = -y - z
/// dy/dt = x + ay
/// dz/dt = b + z(x - c)
/// ```
///
/// Classic parameters (Rössler 1976):
/// - a = 0.2: Controls the frequency of rotation
/// - b = 0.2: Controls the vertical expansion
/// - c = 5.7: Controls the onset of chaos (c > 5 → chaotic)
///
/// These create a single-looped chaotic attractor with a spiral structure.
///
/// # Arguments
/// * `a` - Rotation frequency parameter (typical: 0.2, try 0.1-0.4)
/// * `b` - Vertical expansion parameter (typical: 0.2, try 0.1-0.4)
/// * `c` - Chaos threshold parameter (typical: 5.7, chaos at c > 4)
/// * `initial` - Starting point (x, y, z). Try (0.1, 0.0, 0.0)
/// * `dt` - Time step for integration (typical: 0.01)
/// * `steps` - Number of points to generate
///
/// # Returns
/// Vec of (x, y, z) coordinates tracing the attractor's path
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Classic Rössler spiral
/// let path = sequences::generate(0.2, 0.2, 5.7, (0.1, 0.0, 0.0), 0.01, 100);
///
/// // Extract x coordinates for melody and normalize to frequency range
/// let x_vals: Vec<f32> = path.iter().map(|(x, _, _)| *x).collect();
/// let melody = sequences::normalize_f32(&x_vals, 220.0, 880.0);
///
/// // Use z for rhythmic variation (z oscillates with larger amplitude)
/// let z_vals: Vec<f32> = path.iter().map(|(_, _, z)| *z).collect();
/// let rhythm = sequences::normalize_f32(&z_vals, 0.1, 0.5);
///
/// // Create evolving melody with rhythmic variation
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// for i in 0..melody.len().min(10) {
///     comp.track("rossler")
///         .at(i as f32 * rhythm[i])
///         .note(&[melody[i]], 0.15);
/// }
/// ```
///
/// # Musical Applications
/// - **Spiral melodies**: Natural rise and fall with unpredictability
/// - **Rhythmic cycles**: Z coordinate creates periodic-like rhythmic patterns
/// - **Harmonic sequences**: Smoother than Lorenz, great for chord progressions
/// - **Phase modulation**: Use X/Y as modulation sources for synthesis
/// - **Evolving drones**: Slow evolution with structure
/// - **Polyrhythmic patterns**: Different coordinates have different periodicities
///
/// # Parameter Exploration
/// - **a < 0.2**: Slower rotation, more predictable
/// - **a > 0.2**: Faster rotation, more complex
/// - **c < 4**: Periodic behavior (limit cycle)
/// - **4 < c < 6**: Transition to chaos
/// - **c > 6**: Fully chaotic
/// - **b variations**: Change vertical scale of the spiral
///
/// # Tips
/// - Use dt=0.01 for smooth paths (smaller = smoother but slower)
/// - Discard first ~100 steps (transient before settling on attractor)
/// - X and Y coordinates create the spiral (roughly -10 to 10)
/// - Z coordinate oscillates with larger amplitude (roughly 0 to 20)
/// - Try different c values to explore the transition to chaos
pub fn generate(
    a: f32,
    b: f32,
    c: f32,
    initial: (f32, f32, f32),
    dt: f32,
    steps: usize,
) -> Vec<(f32, f32, f32)> {
    let mut path = Vec::with_capacity(steps);
    let (mut x, mut y, mut z) = initial;

    for _ in 0..steps {
        // Runge-Kutta 4th order integration for accuracy
        // k1 = f(t, y)
        let k1_x = -y - z;
        let k1_y = x + a * y;
        let k1_z = b + z * (x - c);

        // k2 = f(t + dt/2, y + k1*dt/2)
        let x2 = x + k1_x * dt * 0.5;
        let y2 = y + k1_y * dt * 0.5;
        let z2 = z + k1_z * dt * 0.5;

        let k2_x = -y2 - z2;
        let k2_y = x2 + a * y2;
        let k2_z = b + z2 * (x2 - c);

        // k3 = f(t + dt/2, y + k2*dt/2)
        let x3 = x + k2_x * dt * 0.5;
        let y3 = y + k2_y * dt * 0.5;
        let z3 = z + k2_z * dt * 0.5;

        let k3_x = -y3 - z3;
        let k3_y = x3 + a * y3;
        let k3_z = b + z3 * (x3 - c);

        // k4 = f(t + dt, y + k3*dt)
        let x4 = x + k3_x * dt;
        let y4 = y + k3_y * dt;
        let z4 = z + k3_z * dt;

        let k4_x = -y4 - z4;
        let k4_y = x4 + a * y4;
        let k4_z = b + z4 * (x4 - c);

        // Update: y_next = y + (dt/6)(k1 + 2k2 + 2k3 + k4)
        x += (dt / 6.0) * (k1_x + 2.0 * k2_x + 2.0 * k3_x + k4_x);
        y += (dt / 6.0) * (k1_y + 2.0 * k2_y + 2.0 * k3_y + k4_y);
        z += (dt / 6.0) * (k1_z + 2.0 * k2_z + 2.0 * k3_z + k4_z);

        path.push((x, y, z));
    }

    path
}

/// Generate a Rössler attractor with classic parameters
///
/// Convenience function using the standard Rössler parameters (a=0.2, b=0.2, c=5.7)
/// that produce the classic single-banded spiral attractor.
///
/// # Arguments
/// * `steps` - Number of points to generate
///
/// # Returns
/// Vec of (x, y, z) coordinates with first 100 transient steps removed
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Get 500 points on the classic Rössler spiral
/// let spiral = sequences::rossler_spiral(500);
///
/// // X coordinates for melody (range approximately -10 to 10)
/// let x_vals: Vec<f32> = spiral.iter().map(|(x, _, _)| *x).collect();
///
/// // Z coordinates for rhythm (range approximately 0 to 20)
/// let z_vals: Vec<f32> = spiral.iter().map(|(_, _, z)| *z).collect();
/// ```
pub fn rossler_spiral(steps: usize) -> Vec<(f32, f32, f32)> {
    // Generate extra steps to discard transient
    let total_steps = steps + 100;
    let full_path = generate(0.2, 0.2, 5.7, (0.1, 0.0, 0.0), 0.01, total_steps);

    // Discard first 100 steps (transient)
    full_path.into_iter().skip(100).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rossler_attractor_length() {
        let path = generate(0.2, 0.2, 5.7, (0.1, 0.0, 0.0), 0.01, 100);
        assert_eq!(path.len(), 100);
    }

    #[test]
    fn test_rossler_spiral() {
        let path = rossler_spiral(50);
        assert_eq!(path.len(), 50);
    }

    #[test]
    fn test_rossler_stays_bounded() {
        // Rössler attractor should stay roughly within bounds
        let path = generate(0.2, 0.2, 5.7, (0.1, 0.0, 0.0), 0.01, 1000);

        for (x, y, z) in path {
            assert!(x.abs() < 20.0, "X should stay bounded, got {}", x);
            assert!(y.abs() < 20.0, "Y should stay bounded, got {}", y);
            assert!(z < 30.0 && z > -5.0, "Z should stay bounded, got {}", z);
        }
    }

    #[test]
    fn test_rossler_is_chaotic() {
        // Two nearby initial conditions should diverge (butterfly effect)
        let path1 = generate(0.2, 0.2, 5.7, (0.1, 0.0, 0.0), 0.01, 2000);
        let path2 = generate(0.2, 0.2, 5.7, (0.11, 0.0, 0.0), 0.01, 2000);

        // Check that paths diverge
        let (x1, y1, z1) = path1[1999];
        let (x2, y2, z2) = path2[1999];

        let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt();

        // With 10% initial difference and 2000 steps, should diverge significantly
        assert!(
            distance > 0.01,
            "Rössler paths should diverge from different initial conditions, distance was {}",
            distance
        );
    }

    #[test]
    fn test_rossler_deterministic() {
        let path1 = generate(0.2, 0.2, 5.7, (0.1, 0.0, 0.0), 0.01, 100);
        let path2 = generate(0.2, 0.2, 5.7, (0.1, 0.0, 0.0), 0.01, 100);

        // Same parameters should produce identical sequences
        for i in 0..100 {
            let (x1, y1, z1) = path1[i];
            let (x2, y2, z2) = path2[i];

            assert!((x1 - x2).abs() < 1e-6);
            assert!((y1 - y2).abs() < 1e-6);
            assert!((z1 - z2).abs() < 1e-6);
        }
    }

    #[test]
    fn test_rossler_different_c_values() {
        // c=3 should be periodic (before chaos threshold)
        let path_periodic = generate(0.2, 0.2, 3.0, (0.1, 0.0, 0.0), 0.01, 1000);

        // c=5.7 should be chaotic
        let path_chaotic = generate(0.2, 0.2, 5.7, (0.1, 0.0, 0.0), 0.01, 1000);

        // Both should stay bounded but have different dynamics
        for (x, y, z) in &path_periodic {
            assert!(x.is_finite() && y.is_finite() && z.is_finite());
        }

        for (x, y, z) in &path_chaotic {
            assert!(x.is_finite() && y.is_finite() && z.is_finite());
        }

        // The trajectories should be different - check multiple points to be sure
        let (x1_a, _, _) = path_periodic[500];
        let (x2_a, _, _) = path_chaotic[500];
        let (x1_b, _, _) = path_periodic[800];
        let (x2_b, _, _) = path_chaotic[800];

        let diff_a = (x1_a - x2_a).abs();
        let diff_b = (x1_b - x2_b).abs();

        assert!(diff_a > 0.01 || diff_b > 0.1,
            "Different c values should produce different trajectories, diffs: {}, {}", diff_a, diff_b);
    }

    #[test]
    fn test_rossler_different_a_values() {
        let path1 = generate(0.1, 0.2, 5.7, (0.1, 0.0, 0.0), 0.01, 500);
        let path2 = generate(0.3, 0.2, 5.7, (0.1, 0.0, 0.0), 0.01, 500);

        // Different a values should produce different trajectories
        let (x1, y1, _) = path1[499];
        let (x2, y2, _) = path2[499];

        let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        assert!(dist > 0.1, "Different a values should produce different trajectories");
    }

    #[test]
    fn test_rossler_all_coordinates_evolve() {
        let path = generate(0.2, 0.2, 5.7, (1.0, 1.0, 1.0), 0.01, 500);

        // Check that all coordinates change over time
        let (x0, y0, z0) = path[0];

        // Verify that coordinates move away from initial position at some point
        let mut x_changed = false;
        let mut y_changed = false;
        let mut z_changed = false;

        for i in 1..500 {
            let (x, y, z) = path[i];
            if (x - x0).abs() > 0.5 {
                x_changed = true;
            }
            if (y - y0).abs() > 0.5 {
                y_changed = true;
            }
            if (z - z0).abs() > 0.5 {
                z_changed = true;
            }
        }

        assert!(x_changed, "X coordinate should evolve over time");
        assert!(y_changed, "Y coordinate should evolve over time");
        assert!(z_changed, "Z coordinate should evolve over time");
    }
}
