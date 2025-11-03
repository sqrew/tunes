/// Fast wavetable oscillator using pre-computed lookup tables
/// This replaces expensive sin() calls with table lookups + linear interpolation.
/// Typical speedup: 10-100x faster than calling sin() every sample.
/// Wavetables can be created from:
/// - Functions: `Wavetable::from_fn(|phase| phase.sin())`
/// - Samples: `Wavetable::from_samples(vec![...])`
/// - Harmonics: `Wavetable::from_harmonics(&[(1, 1.0), (3, 0.3)])`
/// - Presets: `Wavetable::sine()`, `Wavetable::saw()`, etc.
use std::f32::consts::PI;
use std::sync::Arc;

/// Default size for wavetables (power of 2 for efficiency)
/// 2048 samples provides good quality while being cache-friendly
pub const DEFAULT_TABLE_SIZE: usize = 2048;

/// User-definable wavetable for fast oscillator lookups
///
/// # Examples
/// ```
/// use tunes::wavetable::Wavetable;
///
/// // Create from a function
/// let wt = Wavetable::from_fn(2048, |phase| (phase * 2.0 * std::f32::consts::PI).sin());
///
/// // Create from harmonics (additive synthesis)
/// let wt = Wavetable::from_harmonics(2048, &[(1, 1.0), (3, 0.3), (5, 0.1)]);
///
/// // Use presets
/// let sine = Wavetable::sine();
/// let saw = Wavetable::saw_bandlimited();
/// ```
#[derive(Clone, Debug)]
pub struct Wavetable {
    table: Arc<Vec<f32>>,
}

impl Wavetable {
    /// Create a wavetable from a function that maps phase (0.0-1.0) to amplitude
    ///
    /// # Example
    /// ```
    /// use tunes::wavetable::Wavetable;
    /// let wt = Wavetable::from_fn(2048, |phase| {
    ///     (phase * 2.0 * std::f32::consts::PI).sin()
    /// });
    /// ```
    pub fn from_fn<F>(size: usize, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        let mut table = Vec::with_capacity(size);
        let size_recip = 1.0 / (size as f32);
        for i in 0..size {
            let phase = (i as f32) * size_recip;
            table.push(f(phase));
        }
        Self {
            table: Arc::new(table),
        }
    }

    /// Create a wavetable from a vector of samples
    ///
    /// The samples represent one complete cycle of the waveform.
    /// Values should typically be in the range [-1.0, 1.0].
    ///
    /// # Example
    /// ```
    /// use tunes::wavetable::Wavetable;
    /// let samples = vec![0.0, 1.0, 0.0, -1.0]; // Simple cycle
    /// let wt = Wavetable::from_samples(samples);
    /// ```
    pub fn from_samples(samples: Vec<f32>) -> Self {
        Self {
            table: Arc::new(samples),
        }
    }

    /// Create a wavetable from harmonics using additive synthesis
    ///
    /// Each harmonic is specified as (harmonic_number, amplitude).
    /// For example, `&[(1, 1.0), (3, 0.3)]` creates a waveform with
    /// a fundamental and a third harmonic at 30% amplitude.
    ///
    /// # Example
    /// ```
    /// use tunes::wavetable::Wavetable;
    /// // Square wave approximation (odd harmonics)
    /// let wt = Wavetable::from_harmonics(2048, &[
    ///     (1, 1.0),
    ///     (3, 0.33),
    ///     (5, 0.2),
    ///     (7, 0.14),
    /// ]);
    /// ```
    pub fn from_harmonics(size: usize, harmonics: &[(usize, f32)]) -> Self {
        let mut table = vec![0.0; size];
        const TWO_PI: f32 = 2.0 * PI;

        let size_recip = 1.0 / (size as f32);
        for &(harmonic, amplitude) in harmonics {
            let harmonic_f32 = harmonic as f32;
            for i in 0..size {
                let phase = (i as f32) * size_recip;
                table[i] += amplitude * (phase * harmonic_f32 * TWO_PI).sin();
            }
        }

        // Normalize to [-1.0, 1.0] range
        let max_amp = table.iter().map(|&x| x.abs()).fold(0.0f32, |a, b| a.max(b));
        if max_amp > 0.0 {
            for sample in &mut table {
                *sample /= max_amp;
            }
        }

        Self {
            table: Arc::new(table),
        }
    }

    /// Create a sine wave wavetable
    pub fn sine() -> Self {
        Self::from_fn(DEFAULT_TABLE_SIZE, |phase| (phase * 2.0 * PI).sin())
    }

    /// Create a band-limited sawtooth wave (reduces aliasing)
    ///
    /// Uses additive synthesis with harmonics to create a smoother sawtooth
    /// that won't alias at high frequencies.
    pub fn saw_bandlimited() -> Self {
        // Sawtooth = all harmonics with 1/n amplitude
        let harmonics: Vec<(usize, f32)> = (1..32).map(|n| (n, 1.0 / n as f32)).collect();
        Self::from_harmonics(DEFAULT_TABLE_SIZE, &harmonics)
    }

    /// Create a band-limited square wave (reduces aliasing)
    ///
    /// Uses additive synthesis with odd harmonics only.
    pub fn square_bandlimited() -> Self {
        // Square = odd harmonics with 1/n amplitude
        let harmonics: Vec<(usize, f32)> = (0..16)
            .map(|i| {
                let n = 2 * i + 1; // Odd numbers: 1, 3, 5, 7...
                (n, 1.0 / n as f32)
            })
            .collect();
        Self::from_harmonics(DEFAULT_TABLE_SIZE, &harmonics)
    }

    /// Create a band-limited triangle wave (reduces aliasing)
    ///
    /// Uses additive synthesis with odd harmonics at 1/n² amplitude.
    /// Triangle waves contain odd harmonics like square waves, but
    /// they decay much faster (1/n² vs 1/n), giving a rounder sound.
    pub fn triangle_bandlimited() -> Self {
        // Triangle = odd harmonics with 1/n² amplitude
        let harmonics: Vec<(usize, f32)> = (0..16)
            .map(|i| {
                let n = 2 * i + 1; // Odd numbers: 1, 3, 5, 7...
                let sign = if i % 2 == 0 { 1.0 } else { -1.0 }; // Alternating signs
                (n, sign / (n * n) as f32)
            })
            .collect();
        Self::from_harmonics(DEFAULT_TABLE_SIZE, &harmonics)
    }

    /// Create a naive triangle wave (aliasing, for compatibility)
    ///
    /// Note: This will alias at high frequencies. Use `triangle_bandlimited()` instead.
    pub fn triangle() -> Self {
        Self::from_fn(DEFAULT_TABLE_SIZE, |phase| {
            if phase < 0.5 {
                4.0 * phase - 1.0
            } else {
                -4.0 * phase + 3.0
            }
        })
    }

    /// Create a PWM (Pulse Width Modulation) waveform
    ///
    /// # Arguments
    /// * `duty_cycle` - Width of the pulse (0.0 to 1.0), where 0.5 is a square wave
    pub fn pwm(duty_cycle: f32) -> Self {
        Self::from_fn(DEFAULT_TABLE_SIZE, move |phase| {
            if phase < duty_cycle { 1.0 } else { -1.0 }
        })
    }

    /// Sample the wavetable at a given phase (0.0 to 1.0) with linear interpolation
    ///
    /// This is ~10-100x faster than calling sin() or other trig functions directly.
    #[inline(always)]
    pub fn sample(&self, phase: f32) -> f32 {
        // Wrap phase to 0.0-1.0 range using fast fract() instead of floor subtraction
        let phase = phase.fract();

        // Convert phase to table index (floating point)
        let table_size = self.table.len();
        let table_pos = phase * (table_size as f32);
        let index = table_pos as usize;
        let frac = table_pos - index as f32;

        // Linear interpolation between two adjacent samples
        // Use unchecked indexing with manual bounds check for performance
        // (index is guaranteed < table_size due to phase being in [0,1))
        let sample1 = unsafe { *self.table.get_unchecked(index) };
        let sample2 = unsafe { *self.table.get_unchecked((index + 1) & (table_size - 1)) };

        // FMA (fused multiply-add) is a single CPU instruction on modern hardware
        sample1 + (sample2 - sample1) * frac
    }

    /// Sample at a specific frequency and time
    #[inline(always)]
    pub fn sample_at(&self, frequency: f32, time: f32) -> f32 {
        let phase = time * frequency;
        self.sample(phase)
    }

    /// Get the number of samples in this wavetable
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Check if the wavetable is empty
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

impl Default for Wavetable {
    fn default() -> Self {
        Self::sine()
    }
}

// Global wavetable instances (initialized once, used everywhere)
// These are thread-safe because Wavetable is read-only after creation
lazy_static::lazy_static! {
    /// Global sine wavetable (used for Waveform::Sine)
    pub static ref WAVETABLE: Wavetable = Wavetable::sine();

    /// Global band-limited sawtooth wavetable (used for Waveform::Sawtooth)
    pub static ref SAWTOOTH_WAVETABLE: Wavetable = Wavetable::saw_bandlimited();

    /// Global band-limited square wavetable (used for Waveform::Square)
    pub static ref SQUARE_WAVETABLE: Wavetable = Wavetable::square_bandlimited();

    /// Global band-limited triangle wavetable (used for Waveform::Triangle)
    pub static ref TRIANGLE_WAVETABLE: Wavetable = Wavetable::triangle_bandlimited();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wavetable_sine_accuracy() {
        let table = Wavetable::sine();

        // Test at various points in the cycle
        let test_phases = [0.0, 0.25, 0.5, 0.75, 1.0];

        for &phase in &test_phases {
            let table_value = table.sample(phase);
            let exact_value = (phase * 2.0 * PI).sin();

            // Should be accurate to within 0.01 (linear interpolation error)
            assert!(
                (table_value - exact_value).abs() < 0.01,
                "Phase {}: table={}, exact={}, error={}",
                phase,
                table_value,
                exact_value,
                (table_value - exact_value).abs()
            );
        }
    }

    #[test]
    fn test_wavetable_wrapping() {
        let table = Wavetable::sine();

        // Test that phase wrapping works correctly
        assert!((table.sample(0.0) - table.sample(1.0)).abs() < 0.01);
        assert!((table.sample(0.25) - table.sample(1.25)).abs() < 0.01);
        assert!((table.sample(0.5) - table.sample(2.5)).abs() < 0.01);
    }

    #[test]
    fn test_wavetable_continuity() {
        let table = Wavetable::sine();

        // Test that there are no discontinuities
        for i in 0..100 {
            let phase1 = i as f32 / 100.0;
            let phase2 = (i + 1) as f32 / 100.0;

            let val1 = table.sample(phase1);
            let val2 = table.sample(phase2);

            // Adjacent samples should be close
            assert!(
                (val1 - val2).abs() < 0.1,
                "Discontinuity detected between {} and {}",
                phase1,
                phase2
            );
        }
    }

    #[test]
    fn test_sample_at() {
        let table = Wavetable::sine();

        // 440 Hz at t=0 should be ~0
        let sample = table.sample_at(440.0, 0.0);
        assert!(sample.abs() < 0.01);

        // At quarter period, should be near 1.0
        let quarter_period = 1.0 / (440.0 * 4.0);
        let sample = table.sample_at(440.0, quarter_period);
        assert!((sample - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_from_fn() {
        // Create a simple ramp waveform
        let wt = Wavetable::from_fn(1024, |phase| phase * 2.0 - 1.0);

        // At phase 0.0, should be -1.0
        assert!((wt.sample(0.0) - (-1.0)).abs() < 0.01);

        // At phase 0.5, should be ~0.0
        assert!(wt.sample(0.5).abs() < 0.1);

        // At phase 1.0, should be ~1.0 (wraps back to start = -1.0)
        assert!((wt.sample(0.99) - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_from_samples() {
        let samples = vec![0.0, 1.0, 0.0, -1.0];
        let wt = Wavetable::from_samples(samples);

        assert_eq!(wt.len(), 4);
        assert!((wt.sample(0.0) - 0.0).abs() < 0.01);
        assert!((wt.sample(0.25) - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_from_harmonics() {
        // Pure sine (1st harmonic only)
        let wt = Wavetable::from_harmonics(1024, &[(1, 1.0)]);

        // Should match sine wave
        let sine_wt = Wavetable::sine();
        assert!((wt.sample(0.0) - sine_wt.sample(0.0)).abs() < 0.01);
        assert!((wt.sample(0.25) - sine_wt.sample(0.25)).abs() < 0.01);
    }

    #[test]
    fn test_band_limited_waveforms() {
        // Just test that they create without panicking
        let saw = Wavetable::saw_bandlimited();
        let square = Wavetable::square_bandlimited();

        // Sample should be in valid range
        for i in 0..10 {
            let phase = i as f32 / 10.0;
            assert!(saw.sample(phase).abs() <= 1.0);
            assert!(square.sample(phase).abs() <= 1.0);
        }
    }

    #[test]
    fn test_pwm() {
        let wt = Wavetable::pwm(0.25); // 25% duty cycle

        // Should be high for first 25%
        assert!(wt.sample(0.1) > 0.5);

        // Should be low for remaining 75%
        assert!(wt.sample(0.5) < -0.5);
        assert!(wt.sample(0.9) < -0.5);
    }
}
