/// Generate rhythm patterns using the Circle Map (Arnol'd Tongue)
///
/// The circle map is a one-dimensional chaotic map specifically designed to model
/// phase-locking phenomena in oscillators. It's perfect for generating rhythms that
/// transition smoothly between regular locked patterns and chaotic variations.
///
/// The map is defined as:
/// ```text
/// θ_{n+1} = (θ_n + Ω - (K/2π)sin(2πθ_n)) mod 1
/// ```
///
/// Where:
/// - θ (theta) is the angle/phase on the unit circle [0, 1)
/// - Ω (omega) is the driving frequency ratio
/// - K is the coupling strength (0 = pure rotation, higher = more locking)
///
/// # The Arnol'd Tongue
///
/// The circle map exhibits "mode-locking" where for certain (Ω, K) combinations,
/// the system locks into rational ratios. These regions form triangular wedges
/// called "Arnol'd tongues" in parameter space.
///
/// Key behaviors:
/// - **K = 0**: Pure rotation by Ω (perfectly periodic)
/// - **0 < K < 1**: Quasi-periodic, smooth rotation with slight perturbation
/// - **K = 1**: Critical point, mode-locking boundaries
/// - **K > 1**: Strong mode-locking, chaotic between locked regions
///
/// # Arguments
/// * `omega` - Driving frequency ratio (0.0 to 1.0). Try 0.3, 0.5, 0.618 (golden ratio)
/// * `k` - Coupling strength (0.0 to 2.0). 0=rotation, 1=critical, >1=strong locking
/// * `initial` - Starting phase angle (0.0 to 1.0)
/// * `iterations` - Number of iterations to generate
///
/// # Returns
/// Vec of phase angles in [0, 1) representing positions on the unit circle
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Pure rotation (K=0) - perfectly periodic
/// let rotation = sequences::generate(0.25, 0.0, 0.5, 16);
/// // Creates 1:4 rhythm (hits every 4 steps)
///
/// // Critical coupling (K=1) - interesting mode-locking
/// let critical = sequences::generate(0.333, 1.0, 0.5, 24);
/// // Creates 1:3 patterns with slight variation
///
/// // High coupling (K=2) - complex rhythms
/// let complex = sequences::generate(0.618, 2.0, 0.0, 32);
/// // Golden ratio creates non-repeating but structured rhythms
///
/// // Convert to rhythm hits (trigger when crossing threshold)
/// let hits = sequences::circle_map_to_hits(0.4, 1.5, 0.0, 16, 0.5);
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(140.0));
/// comp.track("circle_rhythm")
///     .drum_grid(16, 0.25)
///     .kick(&hits);
/// ```
///
/// # Musical Applications
/// - **Polyrhythmic patterns**: Omega as rational fractions (1/3, 2/5, etc.)
/// - **Metric modulation**: Smoothly transition between time feels
/// - **Phasing effects**: Two circle maps slightly out of sync
/// - **Groove generation**: K controls "humanization" vs rigidity
/// - **Hocket rhythms**: Multiple voices with complementary circle maps
/// - **Rhythmic fractals**: Self-similar patterns across time scales
///
/// # Parameter Tips
/// - **Omega = p/q**: Creates p:q polyrhythm when K is small
/// - **Omega = φ (0.618)**: Golden ratio, maximally irrational (never locks)
/// - **K = 0**: Perfect click track
/// - **K ≈ 0.5**: Slight groove variation
/// - **K ≈ 1.0**: On edge of chaos, interesting "almost locked" feel
/// - **K ≈ 2.0**: Complex but deterministic chaos
///
/// # Advanced: Finding Mode-Locked Regions
/// For a given omega, mode-locking occurs at specific K values.
/// The tongue for p:q ratio is centered approximately at K = 1.
pub fn generate(omega: f32, k: f32, initial: f32, iterations: usize) -> Vec<f32> {
    let mut phases = Vec::with_capacity(iterations);
    let mut theta = initial % 1.0; // Ensure initial is in [0, 1)

    for _ in 0..iterations {
        phases.push(theta);

        // Apply circle map: θ_{n+1} = (θ_n + Ω - (K/2π)sin(2πθ_n)) mod 1
        let two_pi = 2.0 * std::f32::consts::PI;
        theta = (theta + omega - (k / two_pi) * (two_pi * theta).sin()) % 1.0;

        // Handle negative values (mod in Rust can be negative)
        if theta < 0.0 {
            theta += 1.0;
        }
    }

    phases
}

/// Convert circle map phases to rhythm hits using threshold crossing
///
/// Generates a boolean rhythm pattern by triggering whenever the phase
/// crosses a threshold value. This is the most common way to convert
/// continuous circle map output to discrete rhythm events.
///
/// # Arguments
/// * `omega` - Driving frequency ratio
/// * `k` - Coupling strength
/// * `initial` - Starting phase
/// * `iterations` - Number of steps
/// * `threshold` - Trigger threshold (0.0 to 1.0, typically 0.5)
///
/// # Returns
/// Vec of step indices where hits occur (empty if no hits)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // 3:8 polyrhythm with slight variation
/// let hits = sequences::circle_map_to_hits(0.375, 1.0, 0.0, 16, 0.5);
/// // Returns indices like [0, 3, 5, 8, 11, 13] (approximately)
///
/// // Golden ratio rhythm (never repeats)
/// let golden_hits = sequences::circle_map_to_hits(0.618, 1.5, 0.0, 32, 0.5);
/// ```
pub fn circle_map_to_hits(
    omega: f32,
    k: f32,
    initial: f32,
    iterations: usize,
    threshold: f32,
) -> Vec<usize> {
    let phases = generate(omega, k, initial, iterations);
    let mut hits = Vec::new();
    let mut prev_phase = initial;

    for (i, &phase) in phases.iter().enumerate() {
        // Trigger when crossing threshold from below or on wraparound
        if (prev_phase < threshold && phase >= threshold)
            || (prev_phase > phase && (prev_phase < threshold || phase >= threshold))
        {
            hits.push(i);
        }

        prev_phase = phase;
    }

    hits
}

/// Generate complementary circle map rhythm (hocket pattern)
///
/// Creates a second rhythm that fills the gaps of the first rhythm,
/// perfect for creating call-and-response or hocket patterns.
///
/// # Arguments
/// * `omega` - Driving frequency ratio
/// * `k` - Coupling strength
/// * `initial` - Starting phase
/// * `iterations` - Number of steps
/// * `threshold` - Trigger threshold
///
/// # Returns
/// Tuple of (primary_hits, complement_hits)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Create kick/snare hocket
/// let (kick_hits, snare_hits) = sequences::circle_map_hocket(0.4, 1.5, 0.0, 16, 0.5);
///
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(130.0));
/// comp.track("hocket")
///     .drum_grid(16, 0.25)
///     .kick(&kick_hits)
///     .snare(&snare_hits);
/// ```
pub fn circle_map_hocket(
    omega: f32,
    k: f32,
    initial: f32,
    iterations: usize,
    threshold: f32,
) -> (Vec<usize>, Vec<usize>) {
    let primary_hits = circle_map_to_hits(omega, k, initial, iterations, threshold);

    // Generate complement by finding all indices not in primary
    let complement_hits: Vec<usize> = (0..iterations)
        .filter(|i| !primary_hits.contains(i))
        .collect();

    (primary_hits, complement_hits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_map_length() {
        let phases = generate(0.3, 1.0, 0.5, 100);
        assert_eq!(phases.len(), 100);
    }

    #[test]
    fn test_circle_map_bounded() {
        let phases = generate(0.5, 2.0, 0.0, 200);
        for phase in phases {
            assert!(phase >= 0.0 && phase < 1.0, "Phase should be in [0, 1)");
        }
    }

    #[test]
    fn test_pure_rotation() {
        // K=0 should give pure rotation
        let phases = generate(0.25, 0.0, 0.0, 4);
        assert_eq!(phases.len(), 4);
        // Should be 0, 0.25, 0.5, 0.75 (approximately)
        assert!((phases[0] - 0.0).abs() < 0.001);
        assert!((phases[1] - 0.25).abs() < 0.001);
        assert!((phases[2] - 0.5).abs() < 0.001);
        assert!((phases[3] - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_circle_map_to_hits() {
        // Should produce some hits
        let hits = circle_map_to_hits(0.4, 1.0, 0.0, 16, 0.5);
        assert!(!hits.is_empty(), "Should generate some hits");
        assert!(hits.len() <= 16, "Hits should not exceed iterations");

        // All hits should be valid indices
        for &hit in &hits {
            assert!(hit < 16);
        }
    }

    #[test]
    fn test_circle_map_hocket() {
        let (primary, complement) = circle_map_hocket(0.4, 1.5, 0.0, 16, 0.5);

        // Together they should cover all indices
        let mut combined = primary.clone();
        combined.extend(&complement);
        combined.sort();

        assert_eq!(combined.len(), 16, "Should cover all steps");

        // No overlap
        for &p in &primary {
            assert!(!complement.contains(&p), "Should not overlap");
        }
    }
}
