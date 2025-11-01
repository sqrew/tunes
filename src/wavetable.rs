/// Fast wavetable oscillator using pre-computed lookup tables
///
/// This replaces expensive sin() calls with table lookups + linear interpolation.
/// Typical speedup: 10-100x faster than calling sin() every sample.

use std::f32::consts::PI;

/// Size of the wavetable (power of 2 for efficiency)
/// 2048 samples provides good quality while being cache-friendly
const TABLE_SIZE: usize = 2048;
const TABLE_SIZE_F32: f32 = TABLE_SIZE as f32;

/// Pre-computed sine wavetable for fast oscillator lookups
pub struct Wavetable {
    sine_table: [f32; TABLE_SIZE],
}

impl Wavetable {
    /// Create a new wavetable with pre-computed sine wave
    pub fn new() -> Self {
        let mut sine_table = [0.0; TABLE_SIZE];

        // Pre-compute one complete sine wave cycle
        for i in 0..TABLE_SIZE {
            let phase = (i as f32) / TABLE_SIZE_F32;
            sine_table[i] = (phase * 2.0 * PI).sin();
        }

        Self { sine_table }
    }

    /// Sample the sine table at a given phase (0.0 to 1.0) with linear interpolation
    ///
    /// This is ~10-100x faster than calling sin() directly.
    #[inline]
    pub fn sine(&self, phase: f32) -> f32 {
        // Wrap phase to 0.0-1.0 range
        let phase = phase - phase.floor();

        // Convert phase to table index (floating point)
        let table_pos = phase * TABLE_SIZE_F32;
        let index = table_pos as usize;
        let frac = table_pos - index as f32;

        // Linear interpolation between two adjacent samples
        let sample1 = self.sine_table[index % TABLE_SIZE];
        let sample2 = self.sine_table[(index + 1) % TABLE_SIZE];

        sample1 + (sample2 - sample1) * frac
    }

    /// Sample at a specific frequency and time
    #[inline]
    pub fn sine_at(&self, frequency: f32, time: f32) -> f32 {
        let phase = time * frequency;
        self.sine(phase)
    }
}

impl Default for Wavetable {
    fn default() -> Self {
        Self::new()
    }
}

// Global wavetable instance (initialized once, used everywhere)
// This is thread-safe because Wavetable is read-only after creation
lazy_static::lazy_static! {
    pub static ref WAVETABLE: Wavetable = Wavetable::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wavetable_sine_accuracy() {
        let table = Wavetable::new();

        // Test at various points in the cycle
        let test_phases = [0.0, 0.25, 0.5, 0.75, 1.0];

        for &phase in &test_phases {
            let table_value = table.sine(phase);
            let exact_value = (phase * 2.0 * PI).sin();

            // Should be accurate to within 0.01 (linear interpolation error)
            assert!((table_value - exact_value).abs() < 0.01,
                "Phase {}: table={}, exact={}, error={}",
                phase, table_value, exact_value, (table_value - exact_value).abs());
        }
    }

    #[test]
    fn test_wavetable_wrapping() {
        let table = Wavetable::new();

        // Test that phase wrapping works correctly
        assert!((table.sine(0.0) - table.sine(1.0)).abs() < 0.01);
        assert!((table.sine(0.25) - table.sine(1.25)).abs() < 0.01);
        assert!((table.sine(0.5) - table.sine(2.5)).abs() < 0.01);
    }

    #[test]
    fn test_wavetable_continuity() {
        let table = Wavetable::new();

        // Test that there are no discontinuities
        for i in 0..100 {
            let phase1 = i as f32 / 100.0;
            let phase2 = (i + 1) as f32 / 100.0;

            let val1 = table.sine(phase1);
            let val2 = table.sine(phase2);

            // Adjacent samples should be close
            assert!((val1 - val2).abs() < 0.1,
                "Discontinuity detected between {} and {}", phase1, phase2);
        }
    }

    #[test]
    fn test_sine_at() {
        let table = Wavetable::new();

        // 440 Hz at t=0 should be ~0
        let sample = table.sine_at(440.0, 0.0);
        assert!(sample.abs() < 0.01);

        // At quarter period, should be near 1.0
        let quarter_period = 1.0 / (440.0 * 4.0);
        let sample = table.sine_at(440.0, quarter_period);
        assert!((sample - 1.0).abs() < 0.1);
    }
}
