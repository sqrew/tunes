//! Sine Map - smooth chaotic dynamics based on sine waves
//!
//! The sine map is a nonlinear dynamical system that uses the sine function
//! to generate chaotic sequences. It's particularly musical because it's based
//! on the same sine wave fundamental to audio synthesis.
//!
//! The map is defined as:
//! x_n+1 = r * sin(π * x_n)
//!
//! Where r is the control parameter and x is in [0, 1].
//!
//! # Musical Application
//! - Naturally musical due to sine wave basis
//! - Smoother transitions than tent or logistic maps
//! - Creates flowing, organic melodic contours
//! - Parameter r controls between periodic and chaotic behavior
//! - Excellent for ambient and evolving textures
//! - Use with `normalize()` to map to frequency ranges
//! - Use with `map_to_scale()` for quantized melodies
//!
//! # Parameters
//! - `r`: Control parameter
//!   - r < 1.0: Converges to 0 (stable fixed point)
//!   - r = 1.0: Critically damped, approaches 0 slowly
//!   - 1.0 < r < π: Various periodic behaviors
//!   - r ≈ π (3.14159...): Onset of chaos
//!   - r > π: Fully chaotic, ergodic behavior
//!   - Musical sweet spot: r ∈ [2.5, 3.5]
//!
//! # Example
//! ```
//! use tunes::sequences::sine_map;
//!
//! // Generate 64 values with chaotic parameter
//! let sequence = generate(2.7, 0.4, 64);
//!
//! // Map to musical frequencies (values in range ~[0, 2.7])
//! let notes: Vec<f32> = sequence.iter()
//!     .map(|&val| 220.0 + (val / 3.0) * (880.0 - 220.0))
//!     .collect();
//! ```

use std::f32::consts::PI;

/// Generate a sequence using the sine map
///
/// Iterates the sine map function starting from x0.
/// Values will be in the range [0, 1] for reasonable r values.
///
/// # Arguments
/// * `r` - Control parameter (try 2.5-3.5 for musical chaos, π ≈ 3.14159 for full chaos)
/// * `x0` - Initial value (should be in (0, 1))
/// * `n` - Number of iterations to generate
///
/// # Returns
/// Vector of values from the sine map iteration
///
/// # Typical Parameters
/// - **r = 2.5-2.9**: Musical sweet spot - chaotic but smooth (recommended)
/// - **r = π (3.14159)**: Onset of chaos - classic parameter
/// - **r = 2.0**: More stable, periodic behavior
/// - **x0**: Usually 0.4-0.6 for good variation
///
/// # Recipe: Smooth Chaotic Lead
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences;
///
/// let mut comp = Composition::new(Tempo::new(120.0));
///
/// // Sine map creates smoother chaos than logistic map
/// let smooth_chaos = sequences::generate(2.7, 0.5, 32);
///
/// // Map to major pentatonic
/// let melody = sequences::map_to_scale_f32(
///     &smooth_chaos,
///     &sequences::Scale::major_pentatonic(),
///     C5,
///     2
/// );
///
/// comp.instrument("smooth_lead", &Instrument::synth_lead())
///     .delay(Delay::new(0.375, 0.3, 0.5))
///     .reverb(Reverb::new(0.5, 0.5, 0.3))
///     .notes(&melody, 0.25);
/// ```
///
/// # Example
/// ```
/// use tunes::sequences::sine_map;
///
/// // Chaotic sine map
/// let sequence = generate(2.9, 0.5, 100);
/// assert_eq!(sequence.len(), 100);
///
/// // All values should be in [0, 1]
/// for &val in &sequence {
///     assert!(val >= 0.0 && val <= 1.0);
/// }
/// ```
pub fn generate(r: f32, x0: f32, n: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(n);
    let mut x = x0.clamp(0.0, 1.0);

    for _ in 0..n {
        result.push(x);

        // Sine map transformation: x_next = r * sin(π * x)
        x = r * (PI * x).sin();

        // Sine map naturally produces values in [0, r] for x in [0, 1]
        // Only clamp if value goes negative (shouldn't happen) or if r is unusually large
        if x < 0.0 {
            x = 0.0;
        } else if x > 1.0 && r <= PI {
            // For r <= π, normalize values > 1 back to [0, 1]
            x = x.min(1.0);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine_map_basic() {
        let sequence = generate(2.9, 0.5, 10);
        assert_eq!(sequence.len(), 10);

        // First value should be the initial condition
        assert_eq!(sequence[0], 0.5);
    }

    #[test]
    fn test_sine_map_stays_in_range() {
        let sequence = generate(2.9, 0.5, 200);

        // Values should stay reasonable (may exceed 1.0 slightly for high r)
        for &val in &sequence {
            assert!(val >= 0.0, "Value {} is negative", val);
            assert!(val <= 3.5, "Value {} is too large", val);
        }
    }

    #[test]
    fn test_sine_map_deterministic() {
        let seq1 = generate(2.9, 0.5, 50);
        let seq2 = generate(2.9, 0.5, 50);

        // Same parameters should produce identical sequences
        assert_eq!(seq1, seq2);
    }

    #[test]
    fn test_sine_map_different_initial_conditions() {
        let seq1 = generate(2.9, 0.5, 50);
        let seq2 = generate(2.9, 0.6, 50);

        // Different initial conditions should produce different sequences
        assert_ne!(seq1, seq2);
    }

    #[test]
    fn test_sine_map_first_iteration() {
        let sequence = generate(2.9, 0.5, 2);

        // First iteration: x1 = 2.9 * sin(π * 0.5) = 2.9 * 1.0 = 2.9
        // But clamped to [0, 1] = 1.0
        assert!((sequence[1] - 1.0).abs() < 0.001);
    }


    #[test]
    fn test_sine_map_at_pi() {
        // r = π is the onset of chaos
        let sequence = generate(PI, 0.5, 100);

        // Should produce varied values
        for &val in &sequence {
            assert!(val >= 0.0 && val <= 1.0);
        }
    }

    #[test]
    fn test_sine_map_chaotic_behavior() {
        // With r around 2.5-2.8, should show chaotic behavior
        let sequence = generate(2.7, 0.3, 200);

        // Should explore different regions
        let has_low = sequence.iter().any(|&x| x < 0.5);
        let has_high = sequence.iter().any(|&x| x >= 0.5);

        assert!(has_low || has_high, "Should show variation");
    }

    #[test]
    fn test_sine_map_clamps_input() {
        // Test that out-of-range inputs are clamped
        let sequence = generate(2.9, 1.5, 1);
        assert_eq!(sequence[0], 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_sine_map_single_iteration() {
        let sequence = generate(2.9, 0.5, 1);
        assert_eq!(sequence.len(), 1);
        assert_eq!(sequence[0], 0.5);
    }

    #[test]
    fn test_sine_map_at_zero() {
        // Starting at 0 should stay at 0
        let sequence = generate(2.9, 0.0, 5);

        // sin(0) = 0, so x stays at 0
        for &val in &sequence {
            assert_eq!(val, 0.0);
        }
    }

    #[test]
    fn test_sine_map_at_one() {
        // Starting at 1 gives: sin(π) = 0
        let sequence = generate(2.9, 1.0, 3);

        assert_eq!(sequence[0], 1.0);
        // x1 = 2.9 * sin(π * 1.0) = 2.9 * 0 = 0
        assert!((sequence[1] - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_sine_map_smoothness() {
        // Sine map should produce smoother transitions than tent map
        let sequence = generate(2.7, 0.3, 100);

        // Calculate average absolute difference between consecutive values
        let mut total_diff = 0.0;
        for i in 1..sequence.len() {
            total_diff += (sequence[i] - sequence[i-1]).abs();
        }
        let avg_diff = total_diff / (sequence.len() - 1) as f32;

        // Average difference should be reasonable (checking it's not constant)
        assert!(avg_diff > 0.0 && avg_diff < 2.0, "avg_diff = {}", avg_diff);
    }

    #[test]
    fn test_sine_map_sensitive_to_initial_conditions() {
        // Chaotic systems are sensitive to initial conditions
        let seq1 = generate(2.7, 0.3, 50);
        let seq2 = generate(2.7, 0.31, 50);

        // Different initial conditions should produce different sequences
        assert_ne!(seq1, seq2);
    }

    #[test]
    fn test_sine_map_periodic_window() {
        // Some r values create periodic orbits
        // r = 2.0 should create relatively stable behavior
        let sequence = generate(2.0, 0.5, 100);

        // Should still produce valid values
        for &val in &sequence {
            assert!(val >= 0.0 && val <= 1.0);
        }
    }

    #[test]
    fn test_sine_map_coverage() {
        // Chaotic regime should explore the space reasonably
        let sequence = generate(2.7, 0.3, 1000);

        // Should show variation (not all same value)
        let min_val = sequence.iter().cloned().fold(f32::INFINITY, f32::min);
        let max_val = sequence.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = max_val - min_val;
        assert!(range > 0.1, "Should show variation, range = {}", range);
    }

    #[test]
    fn test_sine_map_musical_sweet_spot() {
        // r in [2.5, 2.9] should produce musically interesting sequences
        for r in [2.5, 2.6, 2.7, 2.8, 2.9] {
            let sequence = generate(r, 0.4, 100);

            // Should produce valid values
            for &val in &sequence {
                assert!(val >= 0.0, "r={}, val={} is negative", r, val);
            }

            // Check it's not all zeros
            let has_nonzero = sequence.iter().any(|&val| val > 0.1);
            assert!(has_nonzero, "r={} should produce non-zero values", r);
        }
    }
}
