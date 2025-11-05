/// Automation envelope for controlling effect parameters over time
///
/// Automation allows you to change effect parameters dynamically during playback,
/// creating sweeps, fades, buildups, and other time-based parameter changes.
///
/// # Examples
///
/// ```
/// use tunes::synthesis::automation::{Automation, Interpolation};
///
/// // Linear fade from 0 to 1 over 4 seconds
/// let fade_in = Automation::linear(&[(0.0, 0.0), (4.0, 1.0)]);
///
/// // Smooth buildup with ease curves
/// let smooth_sweep = Automation::smooth(&[
///     (0.0, 100.0),
///     (4.0, 2000.0),
///     (8.0, 100.0),
/// ]);
/// ```
#[derive(Debug, Clone)]
pub struct Automation {
    /// Time/value points sorted by time
    points: Vec<(f32, f32)>,
    /// Interpolation method between points
    interpolation: Interpolation,
}

/// Interpolation methods for automation curves
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interpolation {
    /// No interpolation - value jumps at each point
    Step,
    /// Linear interpolation between points
    Linear,
    /// Smooth interpolation with ease in/out (cubic)
    Smooth,
}

impl Automation {
    /// Create a new automation with linear interpolation
    ///
    /// # Arguments
    /// * `points` - Array of (time, value) pairs. Will be sorted by time automatically.
    ///
    /// # Panics
    /// Panics if points is empty
    pub fn linear(points: &[(f32, f32)]) -> Self {
        Self::new(points, Interpolation::Linear)
    }

    /// Create a new automation with smooth interpolation (ease in/out)
    ///
    /// # Arguments
    /// * `points` - Array of (time, value) pairs. Will be sorted by time automatically.
    ///
    /// # Panics
    /// Panics if points is empty
    pub fn smooth(points: &[(f32, f32)]) -> Self {
        Self::new(points, Interpolation::Smooth)
    }

    /// Create a new automation with step interpolation (no smoothing)
    ///
    /// # Arguments
    /// * `points` - Array of (time, value) pairs. Will be sorted by time automatically.
    ///
    /// # Panics
    /// Panics if points is empty
    pub fn steps(points: &[(f32, f32)]) -> Self {
        Self::new(points, Interpolation::Step)
    }

    /// Create a constant automation (always returns the same value)
    pub fn constant(value: f32) -> Self {
        Self::new(&[(0.0, value)], Interpolation::Step)
    }

    /// Create a new automation with specified interpolation
    ///
    /// # Arguments
    /// * `points` - Array of (time, value) pairs. Will be sorted by time automatically.
    /// * `interpolation` - Interpolation method to use
    ///
    /// # Panics
    /// Panics if points is empty
    fn new(points: &[(f32, f32)], interpolation: Interpolation) -> Self {
        assert!(
            !points.is_empty(),
            "Automation must have at least one point"
        );

        let mut sorted_points = points.to_vec();
        sorted_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        Self {
            points: sorted_points,
            interpolation,
        }
    }

    /// Get the interpolated value at a given time
    ///
    /// # Arguments
    /// * `time` - Time in seconds
    ///
    /// # Returns
    /// The interpolated value at the given time
    #[inline]
    pub fn value_at(&self, time: f32) -> f32 {
        // Handle edge cases
        if self.points.is_empty() {
            return 0.0;
        }

        if self.points.len() == 1 {
            return self.points[0].1;
        }

        // Before first point: return first value
        if time <= self.points[0].0 {
            return self.points[0].1;
        }

        // After last point: return last value
        if time >= self.points[self.points.len() - 1].0 {
            return self.points[self.points.len() - 1].1;
        }

        // Binary search to find the surrounding points
        let idx = self.points.partition_point(|&(t, _)| t < time);

        // idx is now the first point >= time
        let p1 = self.points[idx - 1];
        let p2 = self.points[idx];

        // Calculate normalized position between points (0.0 to 1.0)
        let t = (time - p1.0) / (p2.0 - p1.0);

        // Interpolate based on method
        match self.interpolation {
            Interpolation::Step => {
                // For step interpolation, snap to the new value when we reach it
                // This makes it "right-continuous" - value changes AT the time point
                if (time - p2.0).abs() < 0.00001 {
                    p2.1
                } else {
                    p1.1
                }
            }
            Interpolation::Linear => {
                // Simple linear interpolation
                p1.1 + (p2.1 - p1.1) * t
            }
            Interpolation::Smooth => {
                // Smoothstep function: 3t² - 2t³
                // Provides smooth ease in and ease out
                let smooth_t = t * t * (3.0 - 2.0 * t);
                p1.1 + (p2.1 - p1.1) * smooth_t
            }
        }
    }

    /// Get the time range covered by this automation
    ///
    /// # Returns
    /// (start_time, end_time) tuple
    pub fn time_range(&self) -> (f32, f32) {
        if self.points.is_empty() {
            return (0.0, 0.0);
        }
        (self.points[0].0, self.points[self.points.len() - 1].0)
    }

    /// Get the value range covered by this automation
    ///
    /// # Returns
    /// (min_value, max_value) tuple
    pub fn value_range(&self) -> (f32, f32) {
        if self.points.is_empty() {
            return (0.0, 0.0);
        }

        let mut min = self.points[0].1;
        let mut max = self.points[0].1;

        for &(_, value) in &self.points {
            min = min.min(value);
            max = max.max(value);
        }

        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_interpolation() {
        let auto = Automation::linear(&[(0.0, 0.0), (4.0, 1.0)]);

        assert_eq!(auto.value_at(0.0), 0.0);
        assert_eq!(auto.value_at(2.0), 0.5);
        assert_eq!(auto.value_at(4.0), 1.0);
    }

    #[test]
    fn test_step_interpolation() {
        let auto = Automation::steps(&[(0.0, 0.0), (2.0, 1.0), (4.0, 0.5)]);

        assert_eq!(auto.value_at(0.0), 0.0);
        assert_eq!(auto.value_at(1.9), 0.0); // Still at first value
        assert_eq!(auto.value_at(2.0), 1.0);
        assert_eq!(auto.value_at(3.9), 1.0); // Still at second value
    }

    #[test]
    fn test_smooth_interpolation() {
        let auto = Automation::smooth(&[(0.0, 0.0), (4.0, 1.0)]);

        assert_eq!(auto.value_at(0.0), 0.0);
        assert_eq!(auto.value_at(4.0), 1.0);

        // Smooth interpolation should have different curve than linear
        let _linear_mid = 0.5; // Linear would be 0.5 at t=2.0
        let smooth_mid = auto.value_at(2.0);
        assert_eq!(smooth_mid, 0.5); // At midpoint, smoothstep == linear

        // But at 1/4 point, smoothstep should be < linear
        let smooth_quarter = auto.value_at(1.0);
        assert!(smooth_quarter < 0.25); // Ease in
    }

    #[test]
    fn test_constant() {
        let auto = Automation::constant(0.5);

        assert_eq!(auto.value_at(0.0), 0.5);
        assert_eq!(auto.value_at(100.0), 0.5);
        assert_eq!(auto.value_at(-100.0), 0.5);
    }

    #[test]
    fn test_before_first_point() {
        let auto = Automation::linear(&[(2.0, 1.0), (4.0, 2.0)]);

        assert_eq!(auto.value_at(0.0), 1.0); // Should return first value
        assert_eq!(auto.value_at(1.0), 1.0);
    }

    #[test]
    fn test_after_last_point() {
        let auto = Automation::linear(&[(0.0, 0.0), (2.0, 1.0)]);

        assert_eq!(auto.value_at(3.0), 1.0); // Should return last value
        assert_eq!(auto.value_at(100.0), 1.0);
    }

    #[test]
    fn test_unsorted_points() {
        // Points should be sorted automatically
        let auto = Automation::linear(&[(4.0, 1.0), (0.0, 0.0), (2.0, 0.5)]);

        assert_eq!(auto.value_at(0.0), 0.0);
        assert_eq!(auto.value_at(2.0), 0.5);
        assert_eq!(auto.value_at(4.0), 1.0);
    }

    #[test]
    fn test_time_range() {
        let auto = Automation::linear(&[(1.0, 0.0), (5.0, 1.0)]);
        assert_eq!(auto.time_range(), (1.0, 5.0));
    }

    #[test]
    fn test_value_range() {
        let auto = Automation::linear(&[(0.0, 0.5), (2.0, -1.0), (4.0, 2.0)]);
        assert_eq!(auto.value_range(), (-1.0, 2.0));
    }

    #[test]
    #[should_panic(expected = "Automation must have at least one point")]
    fn test_empty_points() {
        Automation::linear(&[]);
    }
}
