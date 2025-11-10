//! Hénon Map - 2D chaotic attractor
//!
//! The Hénon map is a discrete-time dynamical system that exhibits chaotic behavior.
//! Unlike 1D maps like the logistic map, it uses two coupled equations creating a
//! 2D phase space with a strange attractor.
//!
//! The equations are:
//! - x_n+1 = 1 - a * x_n^2 + y_n
//! - y_n+1 = b * x_n
//!
//! Classic parameters (a=1.4, b=0.3) produce the famous Hénon attractor.
//!
//! # Musical Application
//! - Creates complex, non-repetitive melodies with structure
//! - Two output streams (x and y) can drive different musical parameters
//! - Natural variation between ordered and chaotic regions
//! - Excellent for evolving ambient textures
//! - Pair with `normalize()` to map to frequency ranges
//! - Use with `map_to_scale()` for melodic sequences
//!
//! # Parameters
//! - `a`: Controls the nonlinearity (typical: 1.4, range: 0.0-1.5)
//! - `b`: Controls the coupling (typical: 0.3, range: 0.0-0.4)
//! - Higher `a` values increase chaos
//!
//! # Example
//! ```
//! use tunes::sequences::henon_map;
//!
//! // Generate 64 points from the classic Hénon attractor
//! let (x_vals, y_vals) = henon_map(1.4, 0.3, 0.1, 0.1, 64);
//!
//! // Use x values for melody (map to frequency range manually)
//! let melody: Vec<f32> = x_vals.iter()
//!     .map(|&x| 220.0 + (x + 1.5) / 3.0 * (880.0 - 220.0))
//!     .collect();
//!
//! // Use y values for rhythm or timbre changes
//! let dynamics: Vec<f32> = y_vals.iter()
//!     .map(|&y| 0.3 + (y + 1.5) / 3.0 * (1.0 - 0.3))
//!     .collect();
//! ```

/// Generate a sequence using the Hénon map
///
/// Returns two sequences (x_values, y_values) from iterating the 2D Hénon map.
/// Both sequences exhibit chaotic behavior with the classic parameters.
///
/// # Arguments
/// * `a` - Nonlinearity parameter (typical: 1.4, try range 0.8-1.5)
/// * `b` - Coupling parameter (typical: 0.3, try range 0.2-0.4)
/// * `x0` - Initial x value (typical: 0.1)
/// * `y0` - Initial y value (typical: 0.1)
/// * `n` - Number of iterations to generate
///
/// # Returns
/// Tuple of (x_values, y_values), each containing n points
///
/// # Typical Parameters
/// - **a = 1.4, b = 0.3**: Classic Hénon attractor (strongly recommended)
/// - **a = 1.2, b = 0.25**: Less chaotic, more periodic
/// - **a = 1.3, b = 0.3**: Moderate chaos
/// - **x0, y0**: Usually 0.1, 0.1 (avoid 0.0, 0.0)
///
/// # Recipe: Dual-Stream Melody
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences;
///
/// let mut comp = Composition::new(Tempo::new(130.0));
///
/// // Generate classic Hénon attractor
/// let (x_vals, y_vals) = sequences::henon_map(1.4, 0.3, 0.1, 0.1, 32);
///
/// // Use x for melody
/// let melody = sequences::map_to_scale_f32(
///     &x_vals,
///     &sequences::Scale::minor_pentatonic(),
///     D4,
///     2
/// );
///
/// // Use y for counter-melody (different scale position)
/// let counter = sequences::map_to_scale_f32(
///     &y_vals,
///     &sequences::Scale::minor_pentatonic(),
///     A4,
///     2
/// );
///
/// comp.instrument("henon_lead", &Instrument::synth_lead())
///     .notes(&melody, 0.25);
///
/// comp.instrument("henon_counter", &Instrument::pluck())
///     .notes(&counter, 0.25);
/// ```
///
/// # Example
/// ```
/// use tunes::sequences::henon_map;
///
/// // Classic Hénon attractor parameters
/// let (x_vals, y_vals) = henon_map(1.4, 0.3, 0.1, 0.1, 100);
/// assert_eq!(x_vals.len(), 100);
/// assert_eq!(y_vals.len(), 100);
///
/// // Different parameters create different patterns
/// let (x2, y2) = henon_map(1.2, 0.25, 0.0, 0.0, 50);
/// ```
pub fn henon_map(a: f32, b: f32, x0: f32, y0: f32, n: usize) -> (Vec<f32>, Vec<f32>) {
    let mut x_vals = Vec::with_capacity(n);
    let mut y_vals = Vec::with_capacity(n);

    let mut x = x0;
    let mut y = y0;

    for _ in 0..n {
        x_vals.push(x);
        y_vals.push(y);

        let x_next = 1.0 - a * x * x + y;
        let y_next = b * x;

        x = x_next;
        y = y_next;
    }

    (x_vals, y_vals)
}

/// Generate only the x-coordinate sequence from the Hénon map
///
/// Convenience function when you only need one dimension of output.
///
/// # Arguments
/// * `a` - Nonlinearity parameter
/// * `b` - Coupling parameter
/// * `x0` - Initial x value
/// * `y0` - Initial y value
/// * `n` - Number of iterations
///
/// # Returns
/// Vector of x values
///
/// # Example
/// ```
/// use tunes::sequences::henon_x;
///
/// let melody = henon_x(1.4, 0.3, 0.1, 0.1, 32);
/// assert_eq!(melody.len(), 32);
/// ```
pub fn henon_x(a: f32, b: f32, x0: f32, y0: f32, n: usize) -> Vec<f32> {
    henon_map(a, b, x0, y0, n).0
}

/// Generate only the y-coordinate sequence from the Hénon map
///
/// Convenience function when you only need one dimension of output.
///
/// # Arguments
/// * `a` - Nonlinearity parameter
/// * `b` - Coupling parameter
/// * `x0` - Initial x value
/// * `y0` - Initial y value
/// * `n` - Number of iterations
///
/// # Returns
/// Vector of y values
///
/// # Example
/// ```
/// use tunes::sequences::henon_y;
///
/// let rhythm = henon_y(1.4, 0.3, 0.1, 0.1, 32);
/// assert_eq!(rhythm.len(), 32);
/// ```
pub fn henon_y(a: f32, b: f32, x0: f32, y0: f32, n: usize) -> Vec<f32> {
    henon_map(a, b, x0, y0, n).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_henon_map_basic() {
        let (x_vals, y_vals) = henon_map(1.4, 0.3, 0.1, 0.1, 10);
        assert_eq!(x_vals.len(), 10);
        assert_eq!(y_vals.len(), 10);

        // First values should be the initial conditions
        assert_eq!(x_vals[0], 0.1);
        assert_eq!(y_vals[0], 0.1);
    }

    #[test]
    fn test_henon_map_classic_parameters() {
        let (x_vals, y_vals) = henon_map(1.4, 0.3, 0.1, 0.1, 100);

        // Classic parameters should produce bounded values
        for &x in &x_vals {
            assert!(x.abs() < 2.0, "x value out of expected range");
        }
        for &y in &y_vals {
            assert!(y.abs() < 2.0, "y value out of expected range");
        }
    }

    #[test]
    fn test_henon_map_deterministic() {
        let (x1, y1) = henon_map(1.4, 0.3, 0.1, 0.1, 50);
        let (x2, y2) = henon_map(1.4, 0.3, 0.1, 0.1, 50);

        // Same parameters should produce identical sequences
        assert_eq!(x1, x2);
        assert_eq!(y1, y2);
    }

    #[test]
    fn test_henon_map_different_initial_conditions() {
        let (x1, _) = henon_map(1.4, 0.3, 0.1, 0.1, 50);
        let (x2, _) = henon_map(1.4, 0.3, 0.2, 0.2, 50);

        // Different initial conditions should diverge
        assert_ne!(x1, x2);
    }

    #[test]
    fn test_henon_map_evolution() {
        let (x_vals, y_vals) = henon_map(1.4, 0.3, 0.1, 0.1, 5);

        // Manually verify first iteration
        // x1 = 1 - 1.4 * 0.1^2 + 0.1 = 1 - 0.014 + 0.1 = 1.086
        // y1 = 0.3 * 0.1 = 0.03
        assert!((x_vals[1] - 1.086).abs() < 0.001);
        assert!((y_vals[1] - 0.03).abs() < 0.001);
    }

    #[test]
    fn test_henon_map_single_point() {
        let (x_vals, y_vals) = henon_map(1.4, 0.3, 0.5, 0.5, 1);
        assert_eq!(x_vals.len(), 1);
        assert_eq!(y_vals.len(), 1);
        assert_eq!(x_vals[0], 0.5);
        assert_eq!(y_vals[0], 0.5);
    }

    #[test]
    fn test_henon_x_convenience() {
        let x_only = henon_x(1.4, 0.3, 0.1, 0.1, 32);
        let (x_full, _) = henon_map(1.4, 0.3, 0.1, 0.1, 32);

        assert_eq!(x_only, x_full);
    }

    #[test]
    fn test_henon_y_convenience() {
        let y_only = henon_y(1.4, 0.3, 0.1, 0.1, 32);
        let (_, y_full) = henon_map(1.4, 0.3, 0.1, 0.1, 32);

        assert_eq!(y_only, y_full);
    }

    #[test]
    fn test_henon_map_different_parameters() {
        let (x1, _) = henon_map(1.2, 0.3, 0.1, 0.1, 50);
        let (x2, _) = henon_map(1.4, 0.3, 0.1, 0.1, 50);

        // Different a parameter should create different sequences
        assert_ne!(x1, x2);
    }

    #[test]
    fn test_henon_map_coupling_parameter() {
        let (_, y1) = henon_map(1.4, 0.2, 0.1, 0.1, 50);
        let (_, y2) = henon_map(1.4, 0.4, 0.1, 0.1, 50);

        // Different b parameter should create different y sequences
        assert_ne!(y1, y2);
    }

    #[test]
    fn test_henon_map_non_chaotic_parameters() {
        // Lower a value should produce more ordered behavior
        let (x_vals, _) = henon_map(0.5, 0.3, 0.1, 0.1, 100);

        // Should still produce bounded values
        for &x in &x_vals {
            assert!(x.is_finite());
        }
    }
}
