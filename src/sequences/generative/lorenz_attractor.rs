/// Generate sequences from the Lorenz attractor
///
/// The Lorenz system is a famous 3D continuous chaotic system that creates the iconic
/// "butterfly" pattern. Unlike discrete maps (logistic, tent, etc.), the Lorenz attractor
/// produces smooth, flowing trajectories perfect for melodic lines and parameter automation.
///
/// The system is defined by three coupled differential equations:
/// ```text
/// dx/dt = σ(y - x)
/// dy/dt = x(ρ - z) - y
/// dz/dt = xy - βz
/// ```
///
/// Classic parameters (Lorenz 1963):
/// - σ (sigma) = 10: Prandtl number
/// - ρ (rho) = 28: Rayleigh number
/// - β (beta) = 8/3: Geometric factor
///
/// These create the iconic butterfly attractor with two lobes.
///
/// # Arguments
/// * `sigma` - Prandtl number (typical: 10.0)
/// * `rho` - Rayleigh number (typical: 28.0, chaos at ρ > 24.74)
/// * `beta` - Geometric factor (typical: 8.0/3.0 ≈ 2.667)
/// * `initial` - Starting point (x, y, z). Try (1.0, 1.0, 1.0)
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
/// // Classic Lorenz butterfly
/// let path = sequences::generate(10.0, 28.0, 8.0/3.0, (1.0, 1.0, 1.0), 0.01, 100);
///
/// // Extract x coordinates for melody and normalize to frequency range
/// let x_vals: Vec<f32> = path.iter().map(|(x, _, _)| *x).collect();
/// let melody = sequences::normalize_f32(&x_vals, 220.0, 880.0);
///
/// // Use y for volume automation
/// let y_vals: Vec<f32> = path.iter().map(|(_, y, _)| *y).collect();
/// let volumes_norm = sequences::normalize_f32(&y_vals, 0.3, 1.0);
///
/// // Create evolving melody with automation
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// for i in 0..melody.len().min(10) {
///     comp.track("lorenz")
///         .at(i as f32 * 0.25)
///         .volume(volumes_norm[i])
///         .note(&[melody[i]], 0.2);
/// }
/// ```
///
/// # Musical Applications
/// - **Smooth melodies**: Continuous flow without jumps
/// - **Parameter automation**: X/Y/Z → pitch/volume/filter
/// - **Binaural effects**: X → left ear, Y → right ear phase
/// - **Ambient textures**: Slow-moving, never-repeating patterns
/// - **Modulation sources**: Drive LFOs, tremolo, vibrato depth
/// - **Spatial movement**: Map to stereo pan + reverb send
///
/// # Typical Parameters
/// - **Classic chaos**: σ=10.0, ρ=28.0, β=8.0/3.0, dt=0.01 (strongly recommended)
/// - **Mild chaos**: σ=10.0, ρ=25.0, β=8.0/3.0, dt=0.01
/// - **Complex**: σ=10.0, ρ=35.0, β=8.0/3.0, dt=0.01
/// - **initial**: (1.0, 1.0, 1.0) works well, try (0.1, 0.0, 0.0) for different path
///
/// # Recipe: Flowing Melody in Scale
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences;
///
/// let mut comp = Composition::new(Tempo::new(100.0));
///
/// // Generate butterfly attractor (skip first 100 for transient)
/// let path = sequences::lorenz_butterfly(132);  // 32 notes + 100 skip
/// let path_stable = &path[100..];  // Use settled portion
///
/// // Extract x coordinates
/// let x_vals: Vec<f32> = path_stable.iter().map(|(x, _, _)| *x).collect();
///
/// // Map to minor pentatonic
/// let melody = sequences::map_to_scale_f32(
///     &x_vals,
///     &sequences::Scale::minor_pentatonic(),
///     A4,
///     2
/// );
///
/// comp.instrument("lorenz_melody", &Instrument::synth_lead())
///     .reverb(Reverb::new(0.6, 0.6, 0.5))
///     .notes(&melody, 0.3);
/// ```
///
/// # Parameter Exploration
/// - **σ < 10**: Less chaotic, more predictable orbits
/// - **10 < ρ < 24.74**: Stable fixed points
/// - **ρ = 28**: Classic butterfly chaos
/// - **ρ > 28**: More complex attractors
/// - **β variations**: Change lobe symmetry
///
/// # Tips
/// - Use dt=0.01 for smooth paths (smaller = smoother but slower)
/// - Discard first ~100 steps (transient before settling on attractor)
/// - Normalize coordinates to musical ranges (they span roughly -20 to 20)
/// - Try different initial conditions for different trajectories
pub fn generate(
    sigma: f32,
    rho: f32,
    beta: f32,
    initial: (f32, f32, f32),
    dt: f32,
    steps: usize,
) -> Vec<(f32, f32, f32)> {
    let mut path = Vec::with_capacity(steps);
    let (mut x, mut y, mut z) = initial;

    for _ in 0..steps {
        // Runge-Kutta 4th order integration for accuracy
        // k1 = f(t, y)
        let k1_x = sigma * (y - x);
        let k1_y = x * (rho - z) - y;
        let k1_z = x * y - beta * z;

        // k2 = f(t + dt/2, y + k1*dt/2)
        let x2 = x + k1_x * dt * 0.5;
        let y2 = y + k1_y * dt * 0.5;
        let z2 = z + k1_z * dt * 0.5;

        let k2_x = sigma * (y2 - x2);
        let k2_y = x2 * (rho - z2) - y2;
        let k2_z = x2 * y2 - beta * z2;

        // k3 = f(t + dt/2, y + k2*dt/2)
        let x3 = x + k2_x * dt * 0.5;
        let y3 = y + k2_y * dt * 0.5;
        let z3 = z + k2_z * dt * 0.5;

        let k3_x = sigma * (y3 - x3);
        let k3_y = x3 * (rho - z3) - y3;
        let k3_z = x3 * y3 - beta * z3;

        // k4 = f(t + dt, y + k3*dt)
        let x4 = x + k3_x * dt;
        let y4 = y + k3_y * dt;
        let z4 = z + k3_z * dt;

        let k4_x = sigma * (y4 - x4);
        let k4_y = x4 * (rho - z4) - y4;
        let k4_z = x4 * y4 - beta * z4;

        // Update: y_next = y + (dt/6)(k1 + 2k2 + 2k3 + k4)
        x += (dt / 6.0) * (k1_x + 2.0 * k2_x + 2.0 * k3_x + k4_x);
        y += (dt / 6.0) * (k1_y + 2.0 * k2_y + 2.0 * k3_y + k4_y);
        z += (dt / 6.0) * (k1_z + 2.0 * k2_z + 2.0 * k3_z + k4_z);

        path.push((x, y, z));
    }

    path
}

/// Generate a Lorenz attractor with classic parameters
///
/// Convenience function using the standard Lorenz parameters (σ=10, ρ=28, β=8/3)
/// that produce the iconic butterfly attractor.
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
/// // Get 500 points on the butterfly attractor
/// let butterfly = sequences::lorenz_butterfly(500);
///
/// // X coordinates for melody (range approximately -20 to 20)
/// let x_vals: Vec<f32> = butterfly.iter().map(|(x, _, _)| *x).collect();
/// ```
pub fn lorenz_butterfly(steps: usize) -> Vec<(f32, f32, f32)> {
    // Generate extra steps to discard transient
    let total_steps = steps + 100;
    let full_path = generate(10.0, 28.0, 8.0 / 3.0, (1.0, 1.0, 1.0), 0.01, total_steps);

    // Discard first 100 steps (transient)
    full_path.into_iter().skip(100).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorenz_attractor_length() {
        let path = generate(10.0, 28.0, 8.0 / 3.0, (1.0, 1.0, 1.0), 0.01, 100);
        assert_eq!(path.len(), 100);
    }

    #[test]
    fn test_lorenz_butterfly() {
        let path = lorenz_butterfly(50);
        assert_eq!(path.len(), 50);
    }

    #[test]
    fn test_lorenz_stays_bounded() {
        // Lorenz attractor should stay roughly within bounds
        let path = generate(10.0, 28.0, 8.0 / 3.0, (1.0, 1.0, 1.0), 0.01, 1000);

        for (x, y, z) in path {
            assert!(x.abs() < 50.0, "X should stay bounded");
            assert!(y.abs() < 50.0, "Y should stay bounded");
            assert!(z < 60.0 && z > -5.0, "Z should stay bounded");
        }
    }

    #[test]
    fn test_lorenz_is_chaotic() {
        // Two nearby initial conditions should diverge (butterfly effect)
        // Using a larger initial difference for clearer divergence
        let path1 = generate(10.0, 28.0, 8.0 / 3.0, (1.0, 1.0, 1.0), 0.01, 2000);
        let path2 = generate(10.0, 28.0, 8.0 / 3.0, (1.1, 1.0, 1.0), 0.01, 2000);

        // Check that paths diverge
        let (x1, y1, z1) = path1[1999];
        let (x2, y2, z2) = path2[1999];

        let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt();

        // With 10% initial difference and 2000 steps, should diverge
        // This verifies the system is not trivially stable
        assert!(distance > 0.01, "Lorenz paths should diverge from different initial conditions, distance was {}", distance);
    }
}
