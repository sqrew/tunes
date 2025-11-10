//! Karplus-Strong plucked string synthesis
//!
//! Physical modeling synthesis that simulates plucked strings using a delay line
//! with feedback and filtering. Produces realistic guitar, harp, and other
//! plucked instrument sounds with minimal computational cost.
//!
//! # How It Works
//!
//! 1. A delay line (circular buffer) is filled with noise
//! 2. The buffer is fed back through a lowpass filter
//! 3. This creates a decaying harmonic series that sounds like a plucked string
//! 4. Buffer length determines pitch, filter strength determines decay/brightness
//!
//! # Example
//!
//! ```
//! use tunes::synthesis::karplus_strong::KarplusStrong;
//!
//! // Create a plucked string at A440
//! let mut string = KarplusStrong::new(440.0, 44100.0);
//!
//! // Set decay and brightness
//! string.set_decay(0.995);      // Longer sustain
//! string.set_brightness(0.5);   // Medium brightness
//!
//! // Generate samples
//! let samples = string.generate(44100); // 1 second of audio
//! ```

use rand::{Rng, SeedableRng};

/// Karplus-Strong plucked string synthesizer
///
/// Simulates a plucked string using a delay line with feedback filtering.
/// The algorithm produces natural-sounding plucks with harmonic overtones
/// that decay realistically.
///
/// # Parameters
/// - **Frequency**: Determined by delay line length
/// - **Decay**: How quickly the sound fades (0.0 - 1.0)
/// - **Brightness**: High-frequency content (0.0 = dark, 1.0 = bright)
///
/// # Example
/// ```
/// use tunes::synthesis::karplus_strong::KarplusStrong;
///
/// // Guitar-like pluck at C4 (261.63 Hz)
/// let mut guitar = KarplusStrong::new(261.63, 44100.0)
///     .with_decay(0.996)
///     .with_brightness(0.7);
///
/// let pluck = guitar.generate(22050); // 0.5 seconds
/// ```
#[derive(Debug)]
pub struct KarplusStrong {
    /// Circular delay buffer
    buffer: Vec<f32>,
    /// Current position in buffer
    position: usize,
    /// Decay factor (0.0 - 1.0) - how long the sound sustains
    decay: f32,
    /// Brightness factor (0.0 - 1.0) - amount of high frequency content
    brightness: f32,
    /// Previous sample for lowpass filtering
    prev_sample: f32,
    /// Random number generator for initial excitation
    rng: rand::rngs::StdRng,
}

impl KarplusStrong {
    /// Create a new Karplus-Strong synthesizer
    ///
    /// # Arguments
    /// * `frequency` - Pitch in Hz (e.g., 440.0 for A4)
    /// * `sample_rate` - Sample rate in Hz (e.g., 44100.0)
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::karplus_strong::KarplusStrong;
    ///
    /// let string = KarplusStrong::new(440.0, 44100.0);
    /// ```
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        // Calculate delay line length from frequency
        let buffer_length = (sample_rate / frequency).round() as usize;
        let buffer_length = buffer_length.max(1); // Prevent zero-length buffer

        let mut rng = rand::rngs::StdRng::from_rng(&mut rand::rng());

        // Initialize buffer with white noise
        let buffer: Vec<f32> = (0..buffer_length)
            .map(|_| rng.random_range(-1.0..1.0))
            .collect();

        Self {
            buffer,
            position: 0,
            decay: 0.996,      // Good default for realistic decay
            brightness: 0.5,   // Medium brightness
            prev_sample: 0.0,
            rng,
        }
    }

    /// Create with a specific seed for deterministic output
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::karplus_strong::KarplusStrong;
    ///
    /// let string1 = KarplusStrong::with_seed(440.0, 44100.0, 12345);
    /// let string2 = KarplusStrong::with_seed(440.0, 44100.0, 12345);
    /// // Both will produce identical output
    /// ```
    pub fn with_seed(frequency: f32, sample_rate: f32, seed: u64) -> Self {
        let buffer_length = (sample_rate / frequency).round() as usize;
        let buffer_length = buffer_length.max(1);

        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let buffer: Vec<f32> = (0..buffer_length)
            .map(|_| rng.random_range(-1.0..1.0))
            .collect();

        Self {
            buffer,
            position: 0,
            decay: 0.996,
            brightness: 0.5,
            prev_sample: 0.0,
            rng,
        }
    }

    /// Set the decay factor (how long the sound sustains)
    ///
    /// # Arguments
    /// * `decay` - Decay coefficient (0.0 - 1.0)
    ///   - 0.99 = fast decay (staccato, short pluck)
    ///   - 0.996 = medium decay (realistic guitar)
    ///   - 0.999 = slow decay (long, ringing sustain)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::karplus_strong::KarplusStrong;
    /// let mut string = KarplusStrong::new(440.0, 44100.0);
    /// string.set_decay(0.999); // Long sustain
    /// ```
    pub fn set_decay(&mut self, decay: f32) {
        self.decay = decay.clamp(0.0, 1.0);
    }

    /// Set the decay factor (builder pattern)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::karplus_strong::KarplusStrong;
    /// let string = KarplusStrong::new(440.0, 44100.0)
    ///     .with_decay(0.998);
    /// ```
    pub fn with_decay(mut self, decay: f32) -> Self {
        self.set_decay(decay);
        self
    }

    /// Set the brightness (high-frequency content)
    ///
    /// # Arguments
    /// * `brightness` - Brightness factor (0.0 - 1.0)
    ///   - 0.0 = very dark, muffled (like a bass guitar)
    ///   - 0.5 = balanced (realistic guitar)
    ///   - 1.0 = very bright, metallic (like a harpsichord)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::karplus_strong::KarplusStrong;
    /// let mut string = KarplusStrong::new(440.0, 44100.0);
    /// string.set_brightness(0.8); // Bright, twangy sound
    /// ```
    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness.clamp(0.0, 1.0);
    }

    /// Set the brightness (builder pattern)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::karplus_strong::KarplusStrong;
    /// let string = KarplusStrong::new(440.0, 44100.0)
    ///     .with_brightness(0.3);
    /// ```
    pub fn with_brightness(mut self, brightness: f32) -> Self {
        self.set_brightness(brightness);
        self
    }

    /// Re-excite the string with a new pluck
    ///
    /// Fills the delay buffer with fresh noise, creating a new attack.
    /// Use this to re-trigger the sound without recreating the object.
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::karplus_strong::KarplusStrong;
    /// let mut string = KarplusStrong::new(440.0, 44100.0);
    ///
    /// let pluck1 = string.generate(22050); // First pluck
    /// string.pluck(); // Re-excite
    /// let pluck2 = string.generate(22050); // Second pluck
    /// ```
    pub fn pluck(&mut self) {
        for sample in &mut self.buffer {
            *sample = self.rng.random_range(-1.0..1.0);
        }
        self.prev_sample = 0.0;
    }

    /// Generate a single sample
    ///
    /// This implements the core Karplus-Strong algorithm:
    /// 1. Read current buffer position
    /// 2. Apply lowpass filter (average with previous sample)
    /// 3. Apply decay
    /// 4. Write back to buffer
    /// 5. Advance position
    #[inline]
    pub fn sample(&mut self) -> f32 {
        // Read current sample from buffer
        let current = self.buffer[self.position];

        // Calculate next position (wrapping)
        let next_pos = (self.position + 1) % self.buffer.len();

        // Lowpass filter: blend current and previous based on brightness
        // brightness = 0.0 -> full averaging (dark)
        // brightness = 1.0 -> no averaging (bright)
        let filtered = (1.0 - self.brightness) * 0.5 * (current + self.prev_sample)
                     + self.brightness * current;

        // Apply decay
        let output = filtered * self.decay;

        // Write filtered output back to buffer for next cycle
        self.buffer[self.position] = output;

        // Update state
        self.prev_sample = current;
        self.position = next_pos;

        output
    }

    /// Generate multiple samples
    ///
    /// Returns a vector of synthesized audio samples.
    ///
    /// # Arguments
    /// * `length` - Number of samples to generate
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::karplus_strong::KarplusStrong;
    /// let mut string = KarplusStrong::new(440.0, 44100.0);
    ///
    /// // Generate 1 second of audio at 44.1kHz
    /// let samples = string.generate(44100);
    /// ```
    pub fn generate(&mut self, length: usize) -> Vec<f32> {
        (0..length).map(|_| self.sample()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_karplus_strong_creates_valid_buffer() {
        let ks = KarplusStrong::new(440.0, 44100.0);

        // Buffer length should be sample_rate / frequency
        let expected_len = (44100.0_f32 / 440.0_f32).round() as usize;
        assert_eq!(ks.buffer.len(), expected_len);
    }

    #[test]
    fn test_karplus_strong_sample_in_range() {
        let mut ks = KarplusStrong::new(440.0, 44100.0);

        // Generate samples and verify range
        for _ in 0..1000 {
            let sample = ks.sample();
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "Sample {} out of range",
                sample
            );
        }
    }

    #[test]
    fn test_karplus_strong_decays() {
        let mut ks = KarplusStrong::new(440.0, 44100.0)
            .with_decay(0.99); // Fast decay for testing

        // Initial samples should be louder
        let initial: Vec<f32> = (0..100).map(|_| ks.sample().abs()).collect();
        let initial_avg: f32 = initial.iter().sum::<f32>() / initial.len() as f32;

        // Later samples should be quieter (after some decay)
        let later: Vec<f32> = (0..100).map(|_| ks.sample().abs()).collect();
        let later_avg: f32 = later.iter().sum::<f32>() / later.len() as f32;

        assert!(
            later_avg < initial_avg,
            "Sound should decay over time: initial {} vs later {}",
            initial_avg,
            later_avg
        );
    }

    #[test]
    fn test_karplus_strong_seeded_deterministic() {
        let mut ks1 = KarplusStrong::with_seed(440.0, 44100.0, 12345);
        let mut ks2 = KarplusStrong::with_seed(440.0, 44100.0, 12345);

        // Same seed should produce identical output
        for _ in 0..1000 {
            assert_eq!(ks1.sample(), ks2.sample());
        }
    }

    #[test]
    fn test_karplus_strong_pluck_resets() {
        let mut ks = KarplusStrong::with_seed(440.0, 44100.0, 42)
            .with_decay(0.99); // Faster decay for testing

        // Generate some samples to let it decay
        for _ in 0..10000 {
            ks.sample();
        }

        // Sample should be very quiet now
        let quiet_sample = ks.sample().abs();

        // Re-pluck
        ks.pluck();

        // After pluck, sample several times and check that at least one is louder
        // (the first sample might be small by random chance)
        let mut max_loud_sample: f32 = 0.0;
        for _ in 0..10 {
            max_loud_sample = max_loud_sample.max(ks.sample().abs());
        }

        assert!(
            max_loud_sample > quiet_sample,
            "Pluck should excite the string: {} vs {}",
            max_loud_sample,
            quiet_sample
        );
    }

    #[test]
    fn test_karplus_strong_brightness_affects_tone() {
        // Create two identical strings with different brightness
        let mut bright = KarplusStrong::with_seed(440.0, 44100.0, 42)
            .with_brightness(1.0); // Maximum brightness

        let mut dark = KarplusStrong::with_seed(440.0, 44100.0, 42)
            .with_brightness(0.0); // Minimum brightness

        // Generate samples
        let bright_samples = bright.generate(1000);
        let dark_samples = dark.generate(1000);

        // Bright should maintain more high-frequency content
        // (Quick test: check that they differ significantly)
        let mut differences = 0;
        for (b, d) in bright_samples.iter().zip(dark_samples.iter()) {
            if (b - d).abs() > 0.01 {
                differences += 1;
            }
        }

        assert!(
            differences > 100,
            "Brightness should affect the sound: {} differences found",
            differences
        );
    }

    #[test]
    fn test_karplus_strong_generate() {
        let mut ks = KarplusStrong::new(440.0, 44100.0);
        let samples = ks.generate(1000);

        assert_eq!(samples.len(), 1000);

        // All samples should be in valid range
        for &sample in &samples {
            assert!(sample >= -1.0 && sample <= 1.0);
        }
    }

    #[test]
    fn test_karplus_strong_different_frequencies() {
        // Low frequency (larger buffer)
        let low = KarplusStrong::new(110.0, 44100.0);
        let low_buffer_len = low.buffer.len();

        // High frequency (smaller buffer)
        let high = KarplusStrong::new(880.0, 44100.0);
        let high_buffer_len = high.buffer.len();

        assert!(
            low_buffer_len > high_buffer_len,
            "Lower frequency should have larger buffer"
        );
    }

    #[test]
    fn test_decay_clamping() {
        let mut ks = KarplusStrong::new(440.0, 44100.0);

        // Test clamping
        ks.set_decay(1.5); // Too high
        assert!(ks.decay <= 1.0);

        ks.set_decay(-0.5); // Too low
        assert!(ks.decay >= 0.0);
    }

    #[test]
    fn test_brightness_clamping() {
        let mut ks = KarplusStrong::new(440.0, 44100.0);

        // Test clamping
        ks.set_brightness(1.5); // Too high
        assert!(ks.brightness <= 1.0);

        ks.set_brightness(-0.5); // Too low
        assert!(ks.brightness >= 0.0);
    }
}
