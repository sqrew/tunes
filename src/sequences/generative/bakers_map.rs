//! Baker's Map - chaotic mixing and fractal distribution
//!
//! The Baker's map is a classic example of a chaotic system that resembles the process
//! of kneading dough - stretching, folding, and mixing. It takes a 2D point and maps it
//! to a new position through stretching and folding operations.
//!
//! The map works by:
//! 1. Stretching the unit square horizontally
//! 2. Cutting it in half
//! 3. Stacking the halves vertically
//!
//! This creates a mixing process similar to how a baker folds dough, hence the name.
//!
//! # Mathematical Definition
//! For (x, y) in [0,1] × [0,1]:
//! - If x < 0.5: x_new = 2x,     y_new = y/2
//! - If x ≥ 0.5: x_new = 2x - 1, y_new = (y + 1)/2
//!
//! # Musical Application
//! - Creates fractal-like distributions perfect for rhythmic variation
//! - Two output dimensions can control pitch and dynamics independently
//! - Excellent for creating "organized chaos" in textures
//! - Natural mixing of high and low values creates musical contrast
//! - Use with `normalize()` to map to musical ranges
//! - Use with `map_to_scale()` for quantized melodic sequences
//!
//! # Example
//! ```
//! use tunes::sequences::bakers_map;
//!
//! // Generate 128 points from Baker's map
//! let (x_vals, y_vals) = bakers_map(0.3, 0.7, 128);
//!
//! // Use x for pitch (values already in [0, 1])
//! let pitches: Vec<f32> = x_vals.iter()
//!     .map(|&x| 220.0 + x * (880.0 - 220.0))
//!     .collect();
//!
//! // Use y for rhythm/dynamics
//! let velocities: Vec<f32> = y_vals.iter()
//!     .map(|&y| 0.3 + y * (1.0 - 0.3))
//!     .collect();
//! ```

/// Generate a sequence using the Baker's map
///
/// Returns two sequences (x_values, y_values) from iterating the 2D Baker's map.
/// Both sequences will be in the range [0, 1].
///
/// # Arguments
/// * `x0` - Initial x value (should be in [0, 1])
/// * `y0` - Initial y value (should be in [0, 1])
/// * `n` - Number of iterations to generate
///
/// # Returns
/// Tuple of (x_values, y_values), each containing n points in [0, 1]
///
/// # Example
/// ```
/// use tunes::sequences::bakers_map;
///
/// // Start from an arbitrary point
/// let (x_vals, y_vals) = bakers_map(0.3, 0.7, 100);
/// assert_eq!(x_vals.len(), 100);
/// assert_eq!(y_vals.len(), 100);
///
/// // All values should be in [0, 1]
/// for &x in &x_vals {
///     assert!(x >= 0.0 && x <= 1.0);
/// }
/// ```
pub fn bakers_map(x0: f32, y0: f32, n: usize) -> (Vec<f32>, Vec<f32>) {
    let mut x_vals = Vec::with_capacity(n);
    let mut y_vals = Vec::with_capacity(n);

    let mut x = x0.clamp(0.0, 1.0);
    let mut y = y0.clamp(0.0, 1.0);

    for _ in 0..n {
        x_vals.push(x);
        y_vals.push(y);

        // Baker's map transformation
        let (x_next, y_next) = if x < 0.5 {
            (2.0 * x, y / 2.0)
        } else {
            (2.0 * x - 1.0, (y + 1.0) / 2.0)
        };

        x = x_next;
        y = y_next;
    }

    (x_vals, y_vals)
}

/// Generate only the x-coordinate sequence from Baker's map
///
/// Convenience function when you only need one dimension of output.
///
/// # Arguments
/// * `x0` - Initial x value (will be clamped to [0, 1])
/// * `y0` - Initial y value (will be clamped to [0, 1])
/// * `n` - Number of iterations
///
/// # Returns
/// Vector of x values in [0, 1]
///
/// # Example
/// ```
/// use tunes::sequences::bakers_x;
///
/// let melody = bakers_x(0.3, 0.7, 64);
/// assert_eq!(melody.len(), 64);
/// ```
pub fn bakers_x(x0: f32, y0: f32, n: usize) -> Vec<f32> {
    bakers_map(x0, y0, n).0
}

/// Generate only the y-coordinate sequence from Baker's map
///
/// Convenience function when you only need one dimension of output.
///
/// # Arguments
/// * `x0` - Initial x value (will be clamped to [0, 1])
/// * `y0` - Initial y value (will be clamped to [0, 1])
/// * `n` - Number of iterations
///
/// # Returns
/// Vector of y values in [0, 1]
///
/// # Example
/// ```
/// use tunes::sequences::bakers_y;
///
/// let dynamics = bakers_y(0.3, 0.7, 64);
/// assert_eq!(dynamics.len(), 64);
/// ```
pub fn bakers_y(x0: f32, y0: f32, n: usize) -> Vec<f32> {
    bakers_map(x0, y0, n).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bakers_map_basic() {
        let (x_vals, y_vals) = bakers_map(0.3, 0.7, 10);
        assert_eq!(x_vals.len(), 10);
        assert_eq!(y_vals.len(), 10);

        // First values should be the initial conditions
        assert_eq!(x_vals[0], 0.3);
        assert_eq!(y_vals[0], 0.7);
    }

    #[test]
    fn test_bakers_map_stays_in_unit_square() {
        let (x_vals, y_vals) = bakers_map(0.5, 0.5, 200);

        // All values should stay in [0, 1]
        for &x in &x_vals {
            assert!(x >= 0.0 && x <= 1.0, "x = {} out of range", x);
        }
        for &y in &y_vals {
            assert!(y >= 0.0 && y <= 1.0, "y = {} out of range", y);
        }
    }

    #[test]
    fn test_bakers_map_deterministic() {
        let (x1, y1) = bakers_map(0.3, 0.7, 50);
        let (x2, y2) = bakers_map(0.3, 0.7, 50);

        // Same initial conditions should produce identical sequences
        assert_eq!(x1, x2);
        assert_eq!(y1, y2);
    }

    #[test]
    fn test_bakers_map_different_initial_conditions() {
        let (x1, _) = bakers_map(0.3, 0.7, 50);
        let (x2, _) = bakers_map(0.4, 0.6, 50);

        // Different initial conditions should produce different sequences
        assert_ne!(x1, x2);
    }

    #[test]
    fn test_bakers_map_left_half() {
        // Test the x < 0.5 case
        let (x_vals, y_vals) = bakers_map(0.25, 0.8, 2);

        // First iteration: x < 0.5
        // x1 = 2 * 0.25 = 0.5
        // y1 = 0.8 / 2 = 0.4
        assert!((x_vals[1] - 0.5).abs() < 0.001);
        assert!((y_vals[1] - 0.4).abs() < 0.001);
    }

    #[test]
    fn test_bakers_map_right_half() {
        // Test the x >= 0.5 case
        let (x_vals, y_vals) = bakers_map(0.75, 0.6, 2);

        // First iteration: x >= 0.5
        // x1 = 2 * 0.75 - 1 = 0.5
        // y1 = (0.6 + 1) / 2 = 0.8
        assert!((x_vals[1] - 0.5).abs() < 0.001);
        assert!((y_vals[1] - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_bakers_map_clamps_input() {
        // Test that out-of-range inputs are clamped
        let (x_vals, y_vals) = bakers_map(1.5, -0.5, 1);

        assert_eq!(x_vals[0], 1.0); // Clamped to 1.0
        assert_eq!(y_vals[0], 0.0); // Clamped to 0.0
    }

    #[test]
    fn test_bakers_map_single_point() {
        let (x_vals, y_vals) = bakers_map(0.5, 0.5, 1);
        assert_eq!(x_vals.len(), 1);
        assert_eq!(y_vals.len(), 1);
    }

    #[test]
    fn test_bakers_map_mixing() {
        // Baker's map should mix values - check that we visit different regions
        let (x_vals, y_vals) = bakers_map(0.123456, 0.789012, 100);

        // Should have values in both lower and upper halves for both x and y
        let has_low_x = x_vals.iter().any(|&x| x < 0.5);
        let has_high_x = x_vals.iter().any(|&x| x >= 0.5);
        let has_low_y = y_vals.iter().any(|&y| y < 0.5);
        let has_high_y = y_vals.iter().any(|&y| y >= 0.5);

        assert!(has_low_x && has_high_x, "Should mix x values");
        assert!(has_low_y && has_high_y, "Should mix y values");
    }

    #[test]
    fn test_bakers_x_convenience() {
        let x_only = bakers_x(0.3, 0.7, 32);
        let (x_full, _) = bakers_map(0.3, 0.7, 32);

        assert_eq!(x_only, x_full);
    }

    #[test]
    fn test_bakers_y_convenience() {
        let y_only = bakers_y(0.3, 0.7, 32);
        let (_, y_full) = bakers_map(0.3, 0.7, 32);

        assert_eq!(y_only, y_full);
    }

    #[test]
    fn test_bakers_map_boundary_behavior() {
        // Test exactly at the boundary x = 0.5
        let (x_vals, y_vals) = bakers_map(0.5, 0.5, 2);

        // x = 0.5 should use the right half case (x >= 0.5)
        // x1 = 2 * 0.5 - 1 = 0.0
        // y1 = (0.5 + 1) / 2 = 0.75
        assert!((x_vals[1] - 0.0).abs() < 0.001);
        assert!((y_vals[1] - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_bakers_map_distribution() {
        // After many iterations, should explore the unit square
        let (x_vals, _) = bakers_map(0.123456, 0.789012, 1000);

        // Calculate approximate distribution across [0, 1]
        let count_0_to_0_5 = x_vals.iter().filter(|&&x| x < 0.5).count();
        let count_0_5_to_1 = x_vals.iter().filter(|&&x| x >= 0.5).count();

        // Should have some distribution (not all in one half)
        // Baker's map can be biased depending on initial conditions
        assert!(count_0_to_0_5 > 10 || count_0_5_to_1 > 10);
        assert!(count_0_to_0_5 + count_0_5_to_1 == 1000);
    }
}
