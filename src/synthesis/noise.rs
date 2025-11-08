//! Noise generators for synthesis and modulation
//!
//! This module provides various noise generators useful for:
//! - **Audio synthesis**: Percussion, textures, ambient sounds
//! - **Modulation**: Random parameter variation
//! - **Sound design**: Natural, organic timbres
//!
//! # Examples
//!
//! ```
//! use tunes::synthesis::noise::{WhiteNoise, BrownNoise};
//!
//! // Generate white noise for hi-hat sound
//! let mut white = WhiteNoise::new();
//! let samples = white.generate(1000);
//!
//! // Generate brown noise for rumble/bass texture
//! let mut brown = BrownNoise::new();
//! let bass_texture = brown.generate(1000);
//! ```

use rand::{Rng, SeedableRng};

/// Noise types available for synthesis
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoiseType {
    /// White noise - equal energy at all frequencies
    White,
    /// Brown noise - more low-frequency energy, random walk
    Brown,
    /// Pink noise - 1/f spectrum, balanced across octaves
    Pink,
    /// Blue noise - emphasis on high frequencies
    Blue,
    /// Green noise - tuned to human hearing, midrange emphasis
    Green,
    /// Perlin noise - coherent gradient noise, smooth and organic
    Perlin,
}

impl NoiseType {
    /// Generate noise samples of this type
    ///
    /// # Arguments
    /// * `length` - Number of samples to generate
    ///
    /// # Returns
    /// Vector of samples in the range [-1.0, 1.0]
    pub fn generate(self, length: usize) -> Vec<f32> {
        match self {
            NoiseType::White => {
                let mut gen = WhiteNoise::new();
                gen.generate(length)
            }
            NoiseType::Brown => {
                let mut gen = BrownNoise::new();
                gen.generate(length)
            }
            NoiseType::Pink => {
                let mut gen = PinkNoise::new();
                gen.generate(length)
            }
            NoiseType::Blue => {
                let mut gen = BlueNoise::new();
                gen.generate(length)
            }
            NoiseType::Green => {
                let mut gen = GreenNoise::new();
                gen.generate(length)
            }
            NoiseType::Perlin => {
                let mut gen = PerlinNoise::new();
                gen.generate(length)
            }
        }
    }
}

/// Trait for noise generators
///
/// This trait allows any noise generator to be used with TrackBuilder
/// and other parts of the library. Implement this trait for custom
/// noise generators.
pub trait NoiseGenerator {
    /// Generate a single sample in the range [-1.0, 1.0]
    fn sample(&mut self) -> f32;

    /// Generate multiple samples
    ///
    /// Default implementation calls `sample()` repeatedly.
    /// Override for more efficient bulk generation if needed.
    fn generate(&mut self, length: usize) -> Vec<f32> {
        (0..length).map(|_| self.sample()).collect()
    }
}

/// White noise generator - equal energy at all frequencies
///
/// White noise contains all frequencies with equal amplitude, creating
/// a "static" or "hiss" sound. It's the acoustic equivalent of white light.
///
/// # Musical Applications
/// - **Hi-hats**: Short bursts of filtered white noise
/// - **Snare drums**: Mixed with tonal components for realistic snares
/// - **Wind/rain**: Natural environmental textures
/// - **Transitions**: Noise sweeps between sections
///
/// # Example
/// ```
/// use tunes::synthesis::noise::WhiteNoise;
///
/// let mut white = WhiteNoise::new();
///
/// // Generate single samples
/// let sample1 = white.sample();
/// let sample2 = white.sample();
///
/// // Generate multiple samples at once
/// let samples = white.generate(44100); // 1 second at 44.1kHz
/// ```
#[derive(Debug)]
pub struct WhiteNoise {
    rng: rand::rngs::StdRng,
}

impl WhiteNoise {
    /// Create a new white noise generator with random seed
    pub fn new() -> Self {
        Self {
            rng: rand::rngs::StdRng::from_rng(&mut rand::rng()),
        }
    }

    /// Create a new white noise generator with a specific seed
    ///
    /// Using a seed makes the noise repeatable/deterministic.
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::noise::WhiteNoise;
    ///
    /// let mut noise1 = WhiteNoise::with_seed(12345);
    /// let mut noise2 = WhiteNoise::with_seed(12345);
    ///
    /// // Both generators produce identical output
    /// assert_eq!(noise1.sample(), noise2.sample());
    /// ```
    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    /// Generate a single sample in the range [-1.0, 1.0]
    #[inline]
    pub fn sample(&mut self) -> f32 {
        self.rng.random_range(-1.0..1.0)
    }

    /// Generate multiple samples
    ///
    /// Returns a vector of `length` samples, each in the range [-1.0, 1.0].
    pub fn generate(&mut self, length: usize) -> Vec<f32> {
        (0..length).map(|_| self.sample()).collect()
    }
}

impl Default for WhiteNoise {
    fn default() -> Self {
        Self::new()
    }
}

/// Brown noise generator (Brownian motion / random walk)
///
/// Brown noise (also called red noise or Brownian noise) is generated by
/// integrating white noise, creating a random walk. This produces a sound
/// with more energy at low frequencies, making it deeper and more "rumbling"
/// than white noise.
///
/// The frequency spectrum decreases by 6dB per octave (1/f²), giving it
/// a warm, bassy character.
///
/// # Musical Applications
/// - **Bass textures**: Deep, organic rumbles
/// - **Ocean sounds**: Waves, water movement
/// - **Thunder**: Low-frequency atmospheric effects
/// - **Ambient drones**: Evolving low-end foundation
///
/// # Example
/// ```
/// use tunes::synthesis::noise::BrownNoise;
///
/// let mut brown = BrownNoise::new();
///
/// // Generate single samples
/// let sample = brown.sample();
///
/// // Generate bass rumble texture
/// let rumble = brown.generate(44100); // 1 second
/// ```
#[derive(Debug)]
pub struct BrownNoise {
    current: f32,
    step_size: f32,
    rng: rand::rngs::StdRng,
}

impl BrownNoise {
    /// Create a new brown noise generator with random seed
    ///
    /// Uses a default step size of 0.05 which provides good balance
    /// between smoothness and variation.
    pub fn new() -> Self {
        Self {
            current: 0.0,
            step_size: 0.05,
            rng: rand::rngs::StdRng::from_rng(&mut rand::rng()),
        }
    }

    /// Create a new brown noise generator with a specific seed
    ///
    /// Using a seed makes the noise repeatable/deterministic.
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::noise::BrownNoise;
    ///
    /// let mut noise1 = BrownNoise::with_seed(54321);
    /// let mut noise2 = BrownNoise::with_seed(54321);
    ///
    /// // Both generators produce identical output
    /// assert_eq!(noise1.sample(), noise2.sample());
    /// ```
    pub fn with_seed(seed: u64) -> Self {
        Self {
            current: 0.0,
            step_size: 0.05,
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    /// Create a brown noise generator with custom step size
    ///
    /// Larger step sizes create more variation but can drift faster.
    /// Smaller step sizes create smoother, slower evolution.
    ///
    /// # Arguments
    /// * `step_size` - How much the value can change per sample (typically 0.01 - 0.1)
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::noise::BrownNoise;
    ///
    /// // Slower, smoother brown noise
    /// let mut smooth = BrownNoise::with_step_size(0.01);
    ///
    /// // Faster, more chaotic brown noise
    /// let mut rough = BrownNoise::with_step_size(0.1);
    /// ```
    pub fn with_step_size(step_size: f32) -> Self {
        Self {
            current: 0.0,
            step_size,
            rng: rand::rngs::StdRng::from_rng(&mut rand::rng()),
        }
    }

    /// Generate a single sample in the range [-1.0, 1.0]
    ///
    /// Uses a random walk: current value changes by a small random amount
    /// each sample, with clamping to keep output in valid range.
    #[inline]
    pub fn sample(&mut self) -> f32 {
        // Take a random step
        let delta = self.rng.random_range(-self.step_size..self.step_size);
        self.current += delta;

        // Clamp to prevent drift beyond [-1, 1]
        // Use soft clamping: reflect at boundaries for smoother behavior
        if self.current > 1.0 {
            self.current = 2.0 - self.current; // Reflect
        } else if self.current < -1.0 {
            self.current = -2.0 - self.current; // Reflect
        }

        // Additional hard clamp for safety
        self.current = self.current.clamp(-1.0, 1.0);

        self.current
    }

    /// Generate multiple samples
    ///
    /// Returns a vector of `length` samples, each in the range [-1.0, 1.0].
    pub fn generate(&mut self, length: usize) -> Vec<f32> {
        (0..length).map(|_| self.sample()).collect()
    }

    /// Reset the internal state to zero
    ///
    /// Useful if you want to restart the random walk from the center.
    pub fn reset(&mut self) {
        self.current = 0.0;
    }
}

impl Default for BrownNoise {
    fn default() -> Self {
        Self::new()
    }
}

// Implement NoiseGenerator trait for built-in noise types
impl NoiseGenerator for WhiteNoise {
    fn sample(&mut self) -> f32 {
        WhiteNoise::sample(self)
    }

    fn generate(&mut self, length: usize) -> Vec<f32> {
        WhiteNoise::generate(self, length)
    }
}

impl NoiseGenerator for BrownNoise {
    fn sample(&mut self) -> f32 {
        BrownNoise::sample(self)
    }

    fn generate(&mut self, length: usize) -> Vec<f32> {
        BrownNoise::generate(self, length)
    }
}

/// Pink noise generator (1/f spectrum)
///
/// Pink noise has equal energy per octave, making it sound more "balanced"
/// than white noise. The spectrum decreases by 3dB per octave (1/f).
///
/// Commonly used in:
/// - **Audio testing**: More representative of real-world sounds
/// - **Ambient soundscapes**: Natural, organic textures
/// - **Background textures**: Gentle, non-fatiguing sound
///
/// Uses the Voss-McCartney algorithm with 7 octaves for high-quality pink noise.
#[derive(Debug)]
pub struct PinkNoise {
    white: WhiteNoise,
    rows: [f32; 7],
    running_sum: f32,
    updates: usize,
}

impl PinkNoise {
    /// Create a new pink noise generator
    pub fn new() -> Self {
        Self {
            white: WhiteNoise::new(),
            rows: [0.0; 7],
            running_sum: 0.0,
            updates: 0,
        }
    }

    /// Create with a specific seed for deterministic output
    pub fn with_seed(seed: u64) -> Self {
        Self {
            white: WhiteNoise::with_seed(seed),
            rows: [0.0; 7],
            running_sum: 0.0,
            updates: 0,
        }
    }

    /// Generate a single sample
    #[inline]
    pub fn sample(&mut self) -> f32 {
        // Voss-McCartney algorithm
        let white_value = self.white.sample();

        // Update one of the generators at different rates
        // This creates the 1/f spectral characteristic
        for (i, row) in self.rows.iter_mut().enumerate() {
            if self.updates & (1 << i) == 0 {
                self.running_sum -= *row;
                *row = self.white.sample();
                self.running_sum += *row;
                break;
            }
        }
        self.updates += 1;

        // Mix white noise with running sum and normalize
        (white_value + self.running_sum) / 8.0
    }

    /// Generate multiple samples
    pub fn generate(&mut self, length: usize) -> Vec<f32> {
        (0..length).map(|_| self.sample()).collect()
    }
}

impl Default for PinkNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseGenerator for PinkNoise {
    fn sample(&mut self) -> f32 {
        PinkNoise::sample(self)
    }

    fn generate(&mut self, length: usize) -> Vec<f32> {
        PinkNoise::generate(self, length)
    }
}

/// Blue noise generator (high-frequency emphasis)
///
/// Blue noise increases in energy at higher frequencies (+3dB per octave),
/// the opposite of pink noise. It sounds "crispy" or "sizzling".
///
/// Useful for:
/// - **Dithering**: In digital audio processing
/// - **High-frequency textures**: Sizzle, air, breath sounds
/// - **Complementary to bass**: Adds brightness without low-end buildup
#[derive(Debug)]
pub struct BlueNoise {
    white: WhiteNoise,
    prev: f32,
}

impl BlueNoise {
    /// Create a new blue noise generator
    pub fn new() -> Self {
        Self {
            white: WhiteNoise::new(),
            prev: 0.0,
        }
    }

    /// Create with a specific seed
    pub fn with_seed(seed: u64) -> Self {
        Self {
            white: WhiteNoise::with_seed(seed),
            prev: 0.0,
        }
    }

    /// Generate a single sample
    #[inline]
    pub fn sample(&mut self) -> f32 {
        // Blue noise is the derivative of white noise (differentiation emphasizes high frequencies)
        let current = self.white.sample();
        let output = current - self.prev;
        self.prev = current;

        // Normalize
        output * 0.5
    }

    /// Generate multiple samples
    pub fn generate(&mut self, length: usize) -> Vec<f32> {
        (0..length).map(|_| self.sample()).collect()
    }
}

impl Default for BlueNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseGenerator for BlueNoise {
    fn sample(&mut self) -> f32 {
        BlueNoise::sample(self)
    }

    fn generate(&mut self, length: usize) -> Vec<f32> {
        BlueNoise::generate(self, length)
    }
}

/// Green noise generator (natural/organic sound)
///
/// Green noise emphasizes frequencies in the midrange (around 500Hz),
/// resembling natural environmental sounds. It's similar to pink noise
/// but with a peak tuned to human hearing sensitivity.
///
/// Applications:
/// - **Nature sounds**: Rustling leaves, gentle rain
/// - **Relaxation/meditation**: Pleasant, non-fatiguing background
/// - **Organic textures**: Natural-sounding synthesis
#[derive(Debug)]
pub struct GreenNoise {
    pink: PinkNoise,
    prev: f32,
}

impl GreenNoise {
    /// Create a new green noise generator
    pub fn new() -> Self {
        Self {
            pink: PinkNoise::new(),
            prev: 0.0,
        }
    }

    /// Create with a specific seed
    pub fn with_seed(seed: u64) -> Self {
        Self {
            pink: PinkNoise::with_seed(seed),
            prev: 0.0,
        }
    }

    /// Generate a single sample
    #[inline]
    pub fn sample(&mut self) -> f32 {
        // Green noise is pink noise with additional midrange emphasis
        // Simple approach: pink noise with a gentle highpass
        let pink_sample = self.pink.sample();

        // Light highpass filter to remove very low frequencies
        let highpassed = pink_sample - 0.5 * self.prev;
        self.prev = pink_sample;

        // Mix back some of the original for warmth
        0.7 * pink_sample + 0.3 * highpassed
    }

    /// Generate multiple samples
    pub fn generate(&mut self, length: usize) -> Vec<f32> {
        (0..length).map(|_| self.sample()).collect()
    }
}

impl Default for GreenNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseGenerator for GreenNoise {
    fn sample(&mut self) -> f32 {
        GreenNoise::sample(self)
    }

    fn generate(&mut self, length: usize) -> Vec<f32> {
        GreenNoise::generate(self, length)
    }
}

/// Perlin noise generator (coherent gradient noise)
///
/// Perlin noise produces smooth, organic, continuous noise using gradient
/// interpolation. Unlike spectral noise types (white, pink, etc.), Perlin
/// noise is coherent - values change smoothly over time creating natural-
/// sounding curves.
///
/// This implementation uses 1D Perlin noise optimized for audio synthesis,
/// using the same algorithm as the project's generative sequences.
///
/// Applications:
/// - **Modulation**: Smooth, organic parameter variation (vibrato, filter sweeps)
/// - **Wind/breath sounds**: Natural, flowing air sounds
/// - **Organic textures**: Evolving pads, ambient drones
/// - **LFO replacement**: More natural than sine/triangle waves
///
/// # Example
/// ```
/// use tunes::synthesis::noise::PerlinNoise;
///
/// // Create Perlin noise for smooth modulation
/// let mut perlin = PerlinNoise::new();
///
/// // Adjust frequency for faster/slower variation
/// let mut fast = PerlinNoise::with_frequency(0.1); // Faster changes
/// let mut slow = PerlinNoise::with_frequency(0.01); // Slower changes
/// ```
#[derive(Debug)]
pub struct PerlinNoise {
    /// Current position in noise space
    x: f32,
    /// How fast to move through noise space (frequency)
    frequency: f32,
    /// Random seed for gradient generation
    seed: u32,
}

impl PerlinNoise {
    /// Create a new Perlin noise generator with random seed
    pub fn new() -> Self {
        let mut rng = rand::rngs::StdRng::from_rng(&mut rand::rng());
        Self {
            x: 0.0,
            frequency: 0.02, // Default frequency for smooth variation
            seed: rng.random_range(0..u32::MAX),
        }
    }

    /// Create with a specific seed for deterministic output
    pub fn with_seed(seed: u32) -> Self {
        Self {
            x: 0.0,
            frequency: 0.02,
            seed,
        }
    }

    /// Create with custom frequency
    ///
    /// # Arguments
    /// * `frequency` - Speed of variation (typically 0.001 - 0.1)
    ///   - Lower values = slower, smoother changes
    ///   - Higher values = faster, more rapid variation
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::noise::PerlinNoise;
    ///
    /// // Very slow, smooth modulation
    /// let mut lfo = PerlinNoise::with_frequency(0.005);
    ///
    /// // Faster variation for texture
    /// let mut texture = PerlinNoise::with_frequency(0.05);
    /// ```
    pub fn with_frequency(frequency: f32) -> Self {
        let mut rng = rand::rngs::StdRng::from_rng(&mut rand::rng());
        Self {
            x: 0.0,
            frequency,
            seed: rng.random_range(0..u32::MAX),
        }
    }

    /// Set the frequency (speed of variation)
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    /// Reset position to the beginning
    pub fn reset(&mut self) {
        self.x = 0.0;
    }

    /// Generate pseudo-random gradient at integer point
    /// Uses same hash function as sequences::perlin_noise for consistency
    #[inline]
    fn gradient(&self, x: i32) -> f32 {
        let mut hash = x.wrapping_mul(374761393) as u32;
        hash = hash.wrapping_add(self.seed);
        hash = hash.wrapping_mul(1103515245);
        hash = hash.wrapping_add(12345);
        hash = (hash >> 16) & 0x7fff;

        // Map to [-1, 1] gradient
        (hash as f32 / 16383.5) - 1.0
    }

    /// Fade function for smooth interpolation (smoothstep)
    /// Same as sequences::perlin_noise
    #[inline]
    fn fade(t: f32) -> f32 {
        // 6t^5 - 15t^4 + 10t^3
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    /// Linear interpolation
    #[inline]
    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }

    /// Generate a single sample in range [-1.0, 1.0]
    #[inline]
    pub fn sample(&mut self) -> f32 {
        // Get integer and fractional parts
        let xi = self.x.floor() as i32;
        let xf = self.x - xi as f32;

        // Smooth interpolation curve
        let sx = Self::fade(xf);

        // Get gradients at integer points
        let g0 = self.gradient(xi);
        let g1 = self.gradient(xi + 1);

        // Calculate dot products (in 1D, just multiplication)
        let d0 = g0 * xf;
        let d1 = g1 * (xf - 1.0);

        // Interpolate
        let result = Self::lerp(d0, d1, sx);

        // Advance position
        self.x += self.frequency;

        result
    }

    /// Generate multiple samples
    pub fn generate(&mut self, length: usize) -> Vec<f32> {
        (0..length).map(|_| self.sample()).collect()
    }
}

impl Default for PerlinNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseGenerator for PerlinNoise {
    fn sample(&mut self) -> f32 {
        PerlinNoise::sample(self)
    }

    fn generate(&mut self, length: usize) -> Vec<f32> {
        PerlinNoise::generate(self, length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_noise_range() {
        let mut white = WhiteNoise::new();

        // Generate 1000 samples and verify all are in valid range
        for _ in 0..1000 {
            let sample = white.sample();
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "White noise sample {} out of range",
                sample
            );
        }
    }

    #[test]
    fn test_white_noise_is_random() {
        let mut white = WhiteNoise::new();

        // Generate samples and verify they're not all the same
        let samples: Vec<f32> = (0..100).map(|_| white.sample()).collect();

        // Check that we have variation (not all samples identical)
        let first = samples[0];
        let has_variation = samples.iter().any(|&s| (s - first).abs() > 0.1);
        assert!(has_variation, "White noise should have variation");
    }

    #[test]
    fn test_white_noise_seeded_deterministic() {
        let mut noise1 = WhiteNoise::with_seed(12345);
        let mut noise2 = WhiteNoise::with_seed(12345);

        // Same seed should produce identical output
        for _ in 0..100 {
            assert_eq!(
                noise1.sample(),
                noise2.sample(),
                "Seeded white noise should be deterministic"
            );
        }
    }

    #[test]
    fn test_white_noise_generate() {
        let mut white = WhiteNoise::new();
        let samples = white.generate(500);

        assert_eq!(samples.len(), 500);

        // All samples should be in valid range
        for &sample in &samples {
            assert!(sample >= -1.0 && sample <= 1.0);
        }
    }

    #[test]
    fn test_brown_noise_range() {
        let mut brown = BrownNoise::new();

        // Generate 10000 samples to test clamping under stress
        for i in 0..10000 {
            let sample = brown.sample();
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "Brown noise sample {} out of range at iteration {}",
                sample,
                i
            );
        }
    }

    #[test]
    fn test_brown_noise_continuity() {
        let mut brown = BrownNoise::new();

        // Brown noise should be continuous (adjacent samples close together)
        let mut prev = brown.sample();

        for _ in 0..100 {
            let current = brown.sample();

            // Adjacent samples shouldn't differ by more than step_size
            let diff = (current - prev).abs();
            assert!(
                diff <= 0.1, // step_size default is 0.05, but with reflection can be slightly more
                "Brown noise discontinuity: {} to {} (diff: {})",
                prev,
                current,
                diff
            );

            prev = current;
        }
    }

    #[test]
    fn test_brown_noise_seeded_deterministic() {
        let mut noise1 = BrownNoise::with_seed(54321);
        let mut noise2 = BrownNoise::with_seed(54321);

        // Same seed should produce identical output
        for _ in 0..100 {
            assert_eq!(
                noise1.sample(),
                noise2.sample(),
                "Seeded brown noise should be deterministic"
            );
        }
    }

    #[test]
    fn test_brown_noise_reset() {
        let mut brown = BrownNoise::new();

        // Generate some samples to move away from zero
        for _ in 0..100 {
            brown.sample();
        }

        // Reset should return to zero
        brown.reset();
        assert_eq!(brown.current, 0.0, "Reset should set current to 0");
    }

    #[test]
    fn test_brown_noise_step_size() {
        let mut smooth = BrownNoise::with_step_size(0.01);
        let mut rough = BrownNoise::with_step_size(0.1);

        // Smooth should have smaller variations
        let smooth_samples = smooth.generate(100);
        let rough_samples = rough.generate(100);

        // Calculate variance
        let smooth_variance = calculate_variance(&smooth_samples);
        let rough_variance = calculate_variance(&rough_samples);

        // Rough should generally have more variance
        // (This is probabilistic, but with 100 samples should be reliable)
        assert!(
            rough_variance > smooth_variance * 0.5,
            "Larger step size should create more variance"
        );
    }

    #[test]
    fn test_brown_noise_generate() {
        let mut brown = BrownNoise::new();
        let samples = brown.generate(500);

        assert_eq!(samples.len(), 500);

        // All samples should be in valid range
        for &sample in &samples {
            assert!(sample >= -1.0 && sample <= 1.0);
        }
    }

    #[test]
    fn test_brown_noise_low_frequency_bias() {
        // Use seeded generator for deterministic test
        let mut brown = BrownNoise::with_seed(42);
        // Generate more samples for stable statistics
        let samples = brown.generate(10000);

        // Brown noise should be more "smooth" than white noise
        // Check that adjacent samples are correlated
        let mut correlation_sum = 0.0;
        for i in 0..samples.len() - 1 {
            correlation_sum += samples[i] * samples[i + 1];
        }
        let avg_correlation = correlation_sum / (samples.len() - 1) as f32;

        // Adjacent samples should be positively correlated (smooth evolution)
        // White noise would have near-zero correlation
        // With 10k samples and seed 42, correlation should be strong and stable
        assert!(
            avg_correlation.abs() > 0.05,
            "Brown noise should show adjacent sample correlation, got: {}",
            avg_correlation
        );
    }

    // Helper function for variance calculation
    fn calculate_variance(samples: &[f32]) -> f32 {
        let mean: f32 = samples.iter().sum::<f32>() / samples.len() as f32;
        let variance: f32 = samples
            .iter()
            .map(|&x| {
                let diff = x - mean;
                diff * diff
            })
            .sum::<f32>()
            / samples.len() as f32;
        variance
    }

    #[test]
    fn test_pink_noise_range() {
        let mut pink = PinkNoise::new();

        for _ in 0..1000 {
            let sample = pink.sample();
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "Pink noise sample {} out of range",
                sample
            );
        }
    }

    #[test]
    fn test_pink_noise_seeded_deterministic() {
        let mut noise1 = PinkNoise::with_seed(12345);
        let mut noise2 = PinkNoise::with_seed(12345);

        for _ in 0..100 {
            assert_eq!(
                noise1.sample(),
                noise2.sample(),
                "Seeded pink noise should be deterministic"
            );
        }
    }

    #[test]
    fn test_blue_noise_range() {
        let mut blue = BlueNoise::new();

        for _ in 0..1000 {
            let sample = blue.sample();
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "Blue noise sample {} out of range",
                sample
            );
        }
    }

    #[test]
    fn test_blue_noise_seeded_deterministic() {
        let mut noise1 = BlueNoise::with_seed(54321);
        let mut noise2 = BlueNoise::with_seed(54321);

        for _ in 0..100 {
            assert_eq!(
                noise1.sample(),
                noise2.sample(),
                "Seeded blue noise should be deterministic"
            );
        }
    }

    #[test]
    fn test_green_noise_range() {
        let mut green = GreenNoise::new();

        for _ in 0..1000 {
            let sample = green.sample();
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "Green noise sample {} out of range",
                sample
            );
        }
    }

    #[test]
    fn test_green_noise_seeded_deterministic() {
        let mut noise1 = GreenNoise::with_seed(11111);
        let mut noise2 = GreenNoise::with_seed(11111);

        for _ in 0..100 {
            assert_eq!(
                noise1.sample(),
                noise2.sample(),
                "Seeded green noise should be deterministic"
            );
        }
    }

    #[test]
    fn test_perlin_noise_range() {
        let mut perlin = PerlinNoise::new();

        for _ in 0..1000 {
            let sample = perlin.sample();
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "Perlin noise sample {} out of range",
                sample
            );
        }
    }

    #[test]
    fn test_perlin_noise_smoothness() {
        // Perlin noise should be smooth - no huge jumps
        let mut perlin = PerlinNoise::with_seed(42);
        let samples = perlin.generate(200);

        for i in 1..samples.len() {
            let diff = (samples[i] - samples[i - 1]).abs();
            // With default frequency 0.02, changes should be very small
            assert!(
                diff < 0.1,
                "Perlin noise jump too large: {} at index {}",
                diff,
                i
            );
        }
    }

    #[test]
    fn test_perlin_noise_seeded_deterministic() {
        let mut noise1 = PerlinNoise::with_seed(99999);
        let mut noise2 = PerlinNoise::with_seed(99999);

        for _ in 0..100 {
            assert_eq!(
                noise1.sample(),
                noise2.sample(),
                "Seeded Perlin noise should be deterministic"
            );
        }
    }

    #[test]
    fn test_perlin_noise_frequency() {
        // Higher frequency should create faster variation
        let mut slow = PerlinNoise::with_seed(42);
        slow.set_frequency(0.01);

        let mut fast = PerlinNoise::with_seed(42);
        fast.set_frequency(0.1);

        let slow_samples = slow.generate(100);
        let fast_samples = fast.generate(100);

        // Measure total variation
        let slow_var: f32 = slow_samples.windows(2).map(|w| (w[1] - w[0]).abs()).sum();
        let fast_var: f32 = fast_samples.windows(2).map(|w| (w[1] - w[0]).abs()).sum();

        assert!(
            fast_var > slow_var,
            "Higher frequency should create more variation: slow={}, fast={}",
            slow_var,
            fast_var
        );
    }

    #[test]
    fn test_perlin_noise_reset() {
        let mut perlin = PerlinNoise::with_seed(777);

        let first_sample = perlin.sample();

        // Generate more samples
        for _ in 0..50 {
            perlin.sample();
        }

        // Reset and check we get the same first sample
        perlin.reset();
        let reset_sample = perlin.sample();

        assert_eq!(
            first_sample,
            reset_sample,
            "Reset should return to beginning"
        );
    }

    #[test]
    fn test_noise_type_enum() {
        // Test that all noise types can be generated
        let white_samples = NoiseType::White.generate(10);
        let brown_samples = NoiseType::Brown.generate(10);
        let pink_samples = NoiseType::Pink.generate(10);
        let blue_samples = NoiseType::Blue.generate(10);
        let green_samples = NoiseType::Green.generate(10);
        let perlin_samples = NoiseType::Perlin.generate(10);

        assert_eq!(white_samples.len(), 10);
        assert_eq!(brown_samples.len(), 10);
        assert_eq!(pink_samples.len(), 10);
        assert_eq!(blue_samples.len(), 10);
        assert_eq!(green_samples.len(), 10);
        assert_eq!(perlin_samples.len(), 10);
    }
}
