/// Generate logistic map sequence (chaos theory)
///
/// The logistic map is a simple equation that exhibits complex chaotic behavior:
/// `x(n+1) = r * x(n) * (1 - x(n))`
///
/// Originally used to model population growth, it demonstrates how simple deterministic
/// systems can produce seemingly random, unpredictable behavior - the foundation of chaos theory.
///
/// The behavior dramatically changes based on the parameter `r`:
/// - **r < 1.0**: Population dies out (converges to 0)
/// - **r < 3.0**: Converges to a stable fixed point
/// - **3.0 < r < 3.57**: Oscillates between multiple values (period doubling)
/// - **r > 3.57**: Chaotic behavior (appears random but is deterministic)
/// - **r ≈ 3.828**: Extreme chaos
///
/// # Arguments
/// * `r` - Growth rate parameter (typically 0.0 to 4.0)
/// * `initial` - Starting value (typically 0.0 to 1.0, often 0.5)
/// * `n` - Number of iterations to generate
///
/// # Returns
/// Vector of values in range 0.0 to 1.0 (except edge cases where it may converge to 0)
///
/// # Typical Parameters
///
/// **r (chaos parameter):**
/// - **r = 2.5**: Stable, converges to fixed point (calm background music)
/// - **r = 3.0**: Barely stable (subtle variation)
/// - **r = 3.2**: Period-2 oscillation (moderate tension)
/// - **r = 3.5**: Period-4 oscillation (increasing complexity)
/// - **r = 3.6**: Complex oscillation (dramatic music)
/// - **r = 3.7**: Onset of chaos (boss fight intro)
/// - **r = 3.9**: Full chaos (intense action, combat)
/// - **r = 4.0**: Maximum chaos (extreme situations)
///
/// **initial:** Usually 0.5 (mid-range), but any value in (0, 1) works
///
/// # Recipe: Chaotic Melody in Scale
///
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences;
///
/// let mut comp = Composition::new(Tempo::new(140.0));
///
/// // Generate chaotic values
/// let chaos = sequences::logistic_map::generate(3.9, 0.5, 32);
///
/// // Map to D minor pentatonic scale
/// let melody = sequences::map_to_scale_f32(
///     &chaos,
///     &sequences::Scale::minor_pentatonic(),
///     D4,
///     2  // 2 octaves
/// );
///
/// comp.instrument("chaos_lead", &Instrument::synth_lead())
///     .delay(Delay::new(0.375, 0.3, 0.5))
///     .notes(&melody, 0.25);
/// ```
///
/// # Recipe: Dynamic Intensity Control
///
/// ```
/// use tunes::sequences;
///
/// // Map game state to music intensity
/// fn generate_for_intensity(intensity: f32, n: usize) -> Vec<f32> {
///     // intensity: 0.0 (calm) to 1.0 (chaotic)
///     let r = 2.5 + intensity * 1.5;  // Maps to r=2.5-4.0
///     let chaos = sequences::logistic_map::generate(r, 0.5, n);
///     sequences::normalize(
///         &chaos.iter().map(|&x| (x * 100.0) as u32).collect::<Vec<_>>(),
///         220.0,
///         880.0
///     )
/// }
///
/// // Calm exploration
/// let calm_melody = generate_for_intensity(0.0, 16);  // r=2.5, stable
///
/// // Boss fight
/// let intense_melody = generate_for_intensity(1.0, 32);  // r=4.0, chaos!
/// ```
///
/// # Musical Applications
/// - **Dynamic complexity**: Map game state to r parameter for intensity control
/// - **Melodic variation**: Chaotic but deterministic pitch sequences
/// - **Rhythm patterns**: Use values to determine hit probabilities
/// - **Texture density**: More enemies → higher r → more chaotic/dense music
/// - **Filter sweeps**: Smooth but unpredictable parameter changes
///
/// # Chaos Control
/// The logistic map is perfect for game audio because you can smoothly transition
/// from stable (calm) to chaotic (intense) music by adjusting the `r` parameter
/// based on gameplay state (enemy count, health, proximity to danger, etc.).
pub fn generate(r: f32, initial: f32, n: usize) -> Vec<f32> {
    let mut seq = vec![initial.clamp(0.0, 1.0)];
    let mut x = initial.clamp(0.0, 1.0);

    for _ in 1..n {
        x = (r * x * (1.0 - x)).clamp(0.0, 1.0);
        seq.push(x);
    }
    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logistic_map_stable() {
        // r=2.5 should converge to a stable fixed point
        let seq = generate(2.5, 0.5, 50);

        assert_eq!(seq.len(), 50);
        assert_eq!(seq[0], 0.5);

        // After many iterations, should converge (last few values nearly equal)
        let last_five: Vec<f32> = seq.iter().rev().take(5).copied().collect();
        for i in 1..last_five.len() {
            assert!(
                (last_five[i] - last_five[0]).abs() < 0.01,
                "Should converge to stable value, got {:?}",
                last_five
            );
        }

        // All values should be in 0-1 range
        for &val in &seq {
            assert!(val >= 0.0 && val <= 1.0, "Value {} out of range", val);
        }
    }

    #[test]
    fn test_logistic_map_chaotic() {
        // r=3.9 should produce chaotic behavior
        let seq = generate(3.9, 0.5, 100);

        assert_eq!(seq.len(), 100);

        // Chaotic sequence should have high variance (not converging to a single point)
        let mean: f32 = seq.iter().sum::<f32>() / seq.len() as f32;
        let variance: f32 = seq.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / seq.len() as f32;

        assert!(
            variance > 0.01,
            "Chaotic sequence should have significant variance, got {}",
            variance
        );

        // All values should still be in 0-1 range
        for &val in &seq {
            assert!(val >= 0.0 && val <= 1.0, "Value {} out of range", val);
        }
    }

    #[test]
    fn test_logistic_map_dies_out() {
        // r=0.5 should converge to 0 (population dies)
        let seq = generate(0.5, 0.5, 20);

        // Should approach 0
        let last_val = seq.last().unwrap();
        assert!(last_val < &0.1, "Should die out, last value: {}", last_val);
    }

    #[test]
    fn test_logistic_map_edge_cases() {
        // Initial value clamping
        let seq1 = generate(2.0, -0.5, 10);
        assert_eq!(seq1[0], 0.0); // Should clamp to 0

        let seq2 = generate(2.0, 1.5, 10);
        assert_eq!(seq2[0], 1.0); // Should clamp to 1

        // Single value
        let seq3 = generate(3.0, 0.5, 1);
        assert_eq!(seq3.len(), 1);
        assert_eq!(seq3[0], 0.5);
    }
}

// ========== PRESETS ==========

/// Ordered behavior (r=2.5) - converges to fixed point
pub fn ordered() -> Vec<f32> {
    generate(2.5, 0.5, 24)
}

/// Period-2 oscillation (r=3.2) - alternates between two values
pub fn oscillating() -> Vec<f32> {
    generate(3.2, 0.5, 32)
}

/// Edge of chaos (r=3.56995) - onset of chaos, very interesting
pub fn edge_of_chaos() -> Vec<f32> {
    generate(3.56995, 0.5, 48)
}

/// Chaotic (r=3.9) - fully chaotic, unpredictable
pub fn chaotic() -> Vec<f32> {
    generate(3.9, 0.5, 64)
}

/// Classic chaotic (r=3.7) - balanced chaos
pub fn classic() -> Vec<f32> {
    generate(3.7, 0.5, 48)
}
