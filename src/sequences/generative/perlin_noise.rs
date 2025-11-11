/// Generate Perlin noise sequences
///
/// Perlin noise is a smooth, continuous pseudo-random function that produces
/// natural-looking organic variation. Unlike random noise (which jumps randomly)
/// or chaos (which is deterministic but unpredictable), Perlin noise creates
/// **controllable smooth randomness** that sounds natural and musical.
///
/// This is the "secret sauce" used in virtually every modern synthesizer for:
/// - Smooth LFO modulation
/// - Organic filter sweeps
/// - Natural vibrato/tremolo depth variation
/// - Evolving pad textures
/// - Controlled randomness that never sounds mechanical
///
/// # Key Properties
/// - **Smooth and continuous**: No sudden jumps
/// - **Controllable frequency**: Adjust speed of variation
/// - **Deterministic**: Same seed always produces same output
/// - **Multi-scale**: Octaves add detail at different scales
/// - **Natural sounding**: Mimics organic processes
///
/// # Arguments
/// * `seed` - Random seed for reproducibility (0-4294967295)
/// * `frequency` - Speed of variation (0.01=slow drift, 1.0=fast changes)
/// * `octaves` - Number of detail layers (1-8, more = richer texture)
/// * `persistence` - How much each octave contributes (typical: 0.5)
/// * `length` - Number of samples to generate
///
/// # Returns
/// Vec of f32 values in range [0.0, 1.0]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Simple smooth noise - slow organic drift
/// let drift = sequences::generate(42, 0.05, 1, 0.5, 100);
/// // Single octave, very smooth
///
/// // Rich textured noise - multiple detail layers
/// let texture = sequences::generate(123, 0.1, 4, 0.5, 200);
/// // 4 octaves create complex but natural variation
///
/// // Fast variation for tremolo/vibrato depth
/// let vibrato = sequences::generate(7, 0.3, 2, 0.5, 500);
///
/// // Use for filter automation
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let filter_curve = sequences::generate(99, 0.08, 3, 0.5, 64);
/// for (i, &cutoff) in filter_curve.iter().enumerate() {
///     let freq = 200.0 + cutoff * 1800.0; // Map to 200-2000 Hz
///     comp.track("synth")
///         .at(i as f32 * 0.25)
///         .note(&[440.0], 0.2);
///     // Would set filter cutoff here in future API
/// }
/// ```
///
/// # Musical Applications
/// - **Filter sweeps**: Smooth, natural-sounding cutoff automation
/// - **Volume automation**: Organic breathing/swelling pads
/// - **Vibrato depth**: Varying vibrato intensity naturally
/// - **Panning**: Smooth stereo movement
/// - **Timbre evolution**: Slowly changing overtone weights
/// - **Rhythm humanization**: Subtle timing/velocity variations
/// - **Generative melody**: Pitch drift within scale
/// - **Reverb/delay send**: Evolving wetness
///
/// # Parameter Guide
///
/// **Frequency:**
/// - 0.01-0.05: Very slow drift (pads, ambient)
/// - 0.05-0.1: Slow evolution (evolving textures)
/// - 0.1-0.3: Medium variation (filter sweeps, vibrato)
/// - 0.3-1.0: Fast changes (tremolo, quick modulation)
///
/// **Octaves:**
/// - 1: Smooth, simple curve
/// - 2-3: Natural with some detail
/// - 4-6: Rich, complex texture
/// - 7-8: Very detailed (can get busy)
///
/// **Persistence:**
/// - 0.3: Dominated by low frequencies (smooth)
/// - 0.5: Balanced (most common)
/// - 0.7: More high-frequency detail (rougher)
///
/// # Why Perlin Noise?
/// - **vs Random Walk**: Much smoother, more controllable
/// - **vs Sine/Triangle**: Not repetitive, sounds organic
/// - **vs Chaos (Lorenz, etc.)**: Predictable character, stays in bounds
/// - **vs White Noise**: Smooth and continuous, not jumpy
///
/// Used everywhere: Serum, Massive, Omnisphere all use Perlin for modulation
pub fn generate(
    seed: u32,
    frequency: f32,
    octaves: u32,
    persistence: f32,
    length: usize,
) -> Vec<f32> {
    let mut result = Vec::with_capacity(length);

    for i in 0..length {
        let x = i as f32 * frequency;
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0; // For normalization

        // Fractal Brownian Motion - sum multiple octaves
        for octave in 0..octaves {
            let freq = 2.0_f32.powi(octave as i32);
            let noise = perlin_1d(x * freq, seed.wrapping_add(octave));

            value += noise * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
        }

        // Normalize to [0, 1]
        let normalized = (value / max_value + 1.0) * 0.5;
        result.push(normalized.clamp(0.0, 1.0));
    }

    result
}

/// Generate bipolar Perlin noise in range [-1.0, 1.0]
///
/// Same as `generate()` but returns values centered around zero.
/// Useful when you need symmetric modulation (like LFO, panning).
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Stereo panning automation (-1.0 = left, 1.0 = right)
/// let pan_curve = sequences::perlin_noise_bipolar(42, 0.1, 2, 0.5, 100);
///
/// // Pitch detune in cents
/// let detune = sequences::perlin_noise_bipolar(7, 0.2, 3, 0.5, 200);
/// // Map to ±50 cents: detune * 50.0
/// ```
pub fn perlin_noise_bipolar(
    seed: u32,
    frequency: f32,
    octaves: u32,
    persistence: f32,
    length: usize,
) -> Vec<f32> {
    let mut result = Vec::with_capacity(length);

    for i in 0..length {
        let x = i as f32 * frequency;
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;

        for octave in 0..octaves {
            let freq = 2.0_f32.powi(octave as i32);
            let noise = perlin_1d(x * freq, seed.wrapping_add(octave));

            value += noise * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
        }

        // Normalize to [-1, 1]
        let normalized = value / max_value;
        result.push(normalized.clamp(-1.0, 1.0));
    }

    result
}

/// Core 1D Perlin noise function
///
/// Returns a smooth noise value at continuous position x.
/// Uses gradient interpolation with smoothstep for natural curves.
fn perlin_1d(x: f32, seed: u32) -> f32 {
    // Integer and fractional parts
    let x0 = x.floor() as i32;
    let x1 = x0 + 1;
    let fx = x - x0 as f32;

    // Smooth interpolation curve (smoothstep)
    let sx = fade(fx);

    // Get gradients at integer points
    let g0 = gradient(x0, seed);
    let g1 = gradient(x1, seed);

    // Calculate dot products
    let d0 = g0 * fx;
    let d1 = g1 * (fx - 1.0);

    // Interpolate
    lerp(d0, d1, sx)
}

/// Smoothstep fade function (6t^5 - 15t^4 + 10t^3)
///
/// This is Ken Perlin's improved fade function that has zero
/// first and second derivatives at t=0 and t=1, making the
/// noise extra smooth.
#[inline]
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Linear interpolation
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

/// Generate pseudo-random gradient at integer point
///
/// Uses hash function to generate consistent random gradient
/// for each integer x coordinate.
fn gradient(x: i32, seed: u32) -> f32 {
    // Hash function to generate pseudo-random value
    let mut hash = x.wrapping_mul(374761393) as u32;
    hash = hash.wrapping_add(seed);
    hash = hash.wrapping_mul(1103515245);
    hash = hash.wrapping_add(12345);
    hash = (hash >> 16) & 0x7fff;

    // Map to [-1, 1] gradient
    (hash as f32 / 16383.5) - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perlin_noise_length() {
        let noise = generate(42, 0.1, 2, 0.5, 100);
        assert_eq!(noise.len(), 100);
    }

    #[test]
    fn test_perlin_noise_bounded() {
        let noise = generate(123, 0.2, 4, 0.5, 200);
        for &value in &noise {
            assert!(value >= 0.0 && value <= 1.0, "Value {} out of range [0, 1]", value);
        }
    }

    #[test]
    fn test_perlin_noise_bipolar_bounded() {
        let noise = perlin_noise_bipolar(7, 0.15, 3, 0.5, 150);
        for &value in &noise {
            assert!(value >= -1.0 && value <= 1.0, "Value {} out of range [-1, 1]", value);
        }
    }

    #[test]
    fn test_perlin_noise_smoothness() {
        // Perlin noise should be smooth - check no huge jumps
        let noise = generate(99, 0.1, 2, 0.5, 100);

        for i in 1..noise.len() {
            let diff = (noise[i] - noise[i - 1]).abs();
            assert!(diff < 0.3, "Jump too large: {} between indices {} and {}", diff, i - 1, i);
        }
    }

    #[test]
    fn test_perlin_noise_deterministic() {
        // Same seed should produce same output
        let noise1 = generate(42, 0.1, 2, 0.5, 50);
        let noise2 = generate(42, 0.1, 2, 0.5, 50);

        for i in 0..noise1.len() {
            assert_eq!(noise1[i], noise2[i], "Noise not deterministic at index {}", i);
        }
    }

    #[test]
    fn test_perlin_noise_different_seeds() {
        // Different seeds should produce different output
        let noise1 = generate(42, 0.2, 3, 0.5, 100);
        let noise2 = generate(99, 0.2, 3, 0.5, 100);

        let mut differences = 0;
        for i in 0..noise1.len() {
            if (noise1[i] - noise2[i]).abs() > 0.05 {
                differences += 1;
            }
        }

        // Expect at least 15% of samples to differ significantly
        assert!(differences > 15, "Different seeds should produce different sequences, found {} differences", differences);
    }

    #[test]
    fn test_fade_function() {
        // Fade should be 0 at t=0, 1 at t=1
        assert_eq!(fade(0.0), 0.0);
        assert_eq!(fade(1.0), 1.0);

        // Should be smooth curve
        assert!(fade(0.5) > 0.4 && fade(0.5) < 0.6);
    }

    #[test]
    fn test_octaves_add_detail() {
        // More octaves should add more variation
        let simple = generate(42, 0.1, 1, 0.5, 100);
        let complex = generate(42, 0.1, 4, 0.5, 100);

        // Measure variation (simplified variance)
        let simple_var = simple.windows(2).map(|w| (w[1] - w[0]).abs()).sum::<f32>();
        let complex_var = complex.windows(2).map(|w| (w[1] - w[0]).abs()).sum::<f32>();

        // More octaves generally means more local variation
        // (This is not always strictly true, but holds on average)
        assert!(complex_var > simple_var * 0.5, "More octaves should add detail");
    }
}
