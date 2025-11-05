//! Tent Map - simple chaotic map with predictable behavior
//!
//! The tent map is one of the simplest examples of a chaotic dynamical system.
//! It's named after the tent-shaped function used to iterate values.
//!
//! The map is defined as:
//! - If x < 0.5: x_new = μ * x
//! - If x ≥ 0.5: x_new = μ * (1 - x)
//!
//! Where μ is the control parameter (typically 2.0 for full chaos).
//!
//! # Musical Application
//! - Simpler and more predictable than the logistic map
//! - Creates triangular wave-like patterns when μ = 2
//! - Good for learning about chaos in a musical context
//! - Generates sequences that feel both random and structured
//! - Use with `normalize()` to map to frequency ranges
//! - Use with `map_to_scale()` for quantized melodies
//!
//! # Parameters
//! - `μ` (mu): Control parameter
//!   - μ = 1.0: Fixed point (converges to 0)
//!   - 1.0 < μ < 2.0: Various periodic and chaotic behaviors
//!   - μ = 2.0: Fully chaotic, ergodic (fills the interval uniformly)
//!   - μ > 2.0: Values escape to infinity
//!
//! # Example
//! ```
//! use tunes::sequences::tent_map;
//!
//! // Generate 64 chaotic values with μ = 2.0 (full chaos)
//! let sequence = tent_map(2.0, 0.3, 64);
//!
//! // Map to musical frequencies (values already in [0, 1])
//! let notes: Vec<f32> = sequence.iter()
//!     .map(|&val| 220.0 + val * (880.0 - 220.0))
//!     .collect();
//! ```

/// Generate a sequence using the tent map
///
/// Iterates the tent map function starting from x0.
/// Values will be in the range [0, 1] for μ ≤ 2.0.
///
/// # Arguments
/// * `mu` - Control parameter (try 2.0 for chaos, 1.5-1.99 for different behaviors)
/// * `x0` - Initial value (should be in (0, 1))
/// * `n` - Number of iterations to generate
///
/// # Returns
/// Vector of values from the tent map iteration
///
/// # Example
/// ```
/// use tunes::sequences::tent_map;
///
/// // Classic chaotic tent map
/// let sequence = tent_map(2.0, 0.3, 100);
/// assert_eq!(sequence.len(), 100);
///
/// // All values should be in [0, 1] for mu = 2.0
/// for &val in &sequence {
///     assert!(val >= 0.0 && val <= 1.0);
/// }
/// ```
pub fn tent_map(mu: f32, x0: f32, n: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(n);
    let mut x = x0.clamp(0.0, 1.0);

    for _ in 0..n {
        result.push(x);

        // Tent map transformation
        x = if x < 0.5 {
            mu * x
        } else {
            mu * (1.0 - x)
        };

        // Clamp to prevent escape for mu > 2.0
        x = x.clamp(0.0, 1.0);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tent_map_basic() {
        let sequence = tent_map(2.0, 0.3, 10);
        assert_eq!(sequence.len(), 10);

        // First value should be the initial condition
        assert_eq!(sequence[0], 0.3);
    }

    #[test]
    fn test_tent_map_stays_in_range() {
        let sequence = tent_map(2.0, 0.5, 200);

        // All values should stay in [0, 1]
        for &val in &sequence {
            assert!(val >= 0.0 && val <= 1.0, "Value {} out of range", val);
        }
    }

    #[test]
    fn test_tent_map_deterministic() {
        let seq1 = tent_map(2.0, 0.3, 50);
        let seq2 = tent_map(2.0, 0.3, 50);

        // Same parameters should produce identical sequences
        assert_eq!(seq1, seq2);
    }

    #[test]
    fn test_tent_map_different_initial_conditions() {
        let seq1 = tent_map(2.0, 0.3, 50);
        let seq2 = tent_map(2.0, 0.4, 50);

        // Different initial conditions should produce different sequences
        assert_ne!(seq1, seq2);
    }

    #[test]
    fn test_tent_map_left_side() {
        // Test x < 0.5 case
        let sequence = tent_map(2.0, 0.25, 2);

        // First iteration: x < 0.5
        // x1 = 2.0 * 0.25 = 0.5
        assert!((sequence[1] - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_tent_map_right_side() {
        // Test x >= 0.5 case
        let sequence = tent_map(2.0, 0.75, 2);

        // First iteration: x >= 0.5
        // x1 = 2.0 * (1 - 0.75) = 2.0 * 0.25 = 0.5
        assert!((sequence[1] - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_tent_map_at_boundary() {
        // Test exactly at x = 0.5
        let sequence = tent_map(2.0, 0.5, 2);

        // x = 0.5 uses right side (x >= 0.5)
        // x1 = 2.0 * (1 - 0.5) = 1.0
        assert!((sequence[1] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_tent_map_mu_equals_2_is_chaotic() {
        let sequence = tent_map(2.0, 0.123456, 100);

        // With mu = 2.0, should explore the full range
        let has_low = sequence.iter().any(|&x| x < 0.3);
        let has_mid = sequence.iter().any(|&x| x >= 0.3 && x < 0.7);
        let has_high = sequence.iter().any(|&x| x >= 0.7);

        assert!(has_low, "Should have values in low range");
        assert!(has_mid, "Should have values in mid range");
        assert!(has_high, "Should have values in high range");
    }

    #[test]
    fn test_tent_map_mu_less_than_2() {
        let sequence = tent_map(1.5, 0.5, 100);

        // With mu < 2, values should still stay bounded
        for &val in &sequence {
            assert!(val >= 0.0 && val <= 1.0);
        }
    }


    #[test]
    fn test_tent_map_symmetry() {
        // Tent map is symmetric around 0.5
        let seq1 = tent_map(2.0, 0.3, 10);
        let seq2 = tent_map(2.0, 0.7, 10);

        // After first iteration, both should map to same value
        // 0.3 -> 2*0.3 = 0.6
        // 0.7 -> 2*(1-0.7) = 0.6
        assert!((seq1[1] - seq2[1]).abs() < 0.001);
    }

    #[test]
    fn test_tent_map_clamps_input() {
        // Test that out-of-range inputs are clamped
        let sequence = tent_map(2.0, 1.5, 1);
        assert_eq!(sequence[0], 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_tent_map_single_iteration() {
        let sequence = tent_map(2.0, 0.5, 1);
        assert_eq!(sequence.len(), 1);
        assert_eq!(sequence[0], 0.5);
    }

    #[test]
    fn test_tent_map_different_mu_values() {
        let seq1 = tent_map(1.5, 0.5, 50);
        let seq2 = tent_map(2.0, 0.5, 50);

        // Different mu should produce different sequences
        assert_ne!(seq1, seq2);
    }

    #[test]
    fn test_tent_map_period_2_orbit() {
        // For mu = 2.0, starting at x = 2/3 creates period-2 orbit
        let sequence = tent_map(2.0, 2.0/3.0, 5);

        // x0 = 2/3 ~0.6667
        // x1 = 2 * (1 - 2/3) = 2/3 ~0.6667
        // Should oscillate
        assert!((sequence[0] - 2.0/3.0).abs() < 0.01);
        assert!((sequence[1] - 2.0/3.0).abs() < 0.01);
    }

    #[test]
    fn test_tent_map_coverage() {
        // mu = 2.0 is ergodic - should cover the interval fairly uniformly
        let sequence = tent_map(2.0, 0.123456, 1000);

        // Divide [0,1] into 10 bins and check coverage
        let mut bins = vec![0; 10];
        for &val in &sequence {
            let bin = (val * 10.0).floor() as usize;
            bins[bin.min(9)] += 1;
        }

        // Each bin should have some values (not perfect uniform, but some coverage)
        for &count in &bins {
            assert!(count > 0, "Should have coverage in all bins");
        }
    }
}
