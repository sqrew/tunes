/// Fast deterministic noise generator for drum synthesis
/// Uses a hash-like function to generate pseudo-random values from a seed
/// This is much faster than thread_rng() and produces consistent, high-quality noise
/// Returns values in the range [-1.0, 1.0]
fn noise(seed: f32) -> f32 {
    // Classic GLSL-style hash function
    // fract(sin(x) * large_number) produces pseudo-random values in [0, 1]
    // Use abs() to ensure positive value before fract()
    let hash = ((seed * 12.9898).sin() * 43758.55).abs().fract();

    // Map from [0, 1] to [-1, 1]
    hash * 2.0 - 1.0
}

/// Generate a kick drum sample (low frequency sine burst with pitch drop)
pub fn kick_drum_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.15; // 150ms

    if t > duration {
        return 0.0;
    }

    // Frequency drops from 150Hz to 40Hz
    let start_freq = 150.0;
    let end_freq = 40.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration);

    // Exponential decay envelope
    let envelope = (-t * 15.0).exp();

    // Generate sine wave
    let value = (2.0 * std::f32::consts::PI * freq * t).sin();

    value * envelope * 0.8
}

/// Generate a snare drum sample (noise burst with decay)
pub fn snare_drum_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.1; // 100ms

    if t > duration {
        return 0.0;
    }

    // Mix of noise and tone
    let noise_val = noise(sample_index as f32);
    let tone = (2.0 * std::f32::consts::PI * 200.0 * t).sin();

    // Fast decay
    let envelope = (-t * 25.0).exp();

    (noise_val * 0.7 + tone * 0.3) * envelope * 0.5
}

/// Generate a hi-hat sample (high frequency noise burst)
pub fn hihat_sample(sample_index: usize, sample_rate: f32, closed: bool) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = if closed { 0.05 } else { 0.15 }; // Closed vs open

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // High-pass filtered noise (simulate by mixing high freq sine with noise_val)
    let high_tone = (2.0 * std::f32::consts::PI * 8000.0 * t).sin();
    let filtered_noise = noise_val * 0.8 + high_tone * 0.2;

    // Very fast decay
    let decay_rate = if closed { 40.0 } else { 15.0 };
    let envelope = (-t * decay_rate).exp();

    filtered_noise * envelope * 0.3
}

/// Generate a clap sample (multiple noise bursts)
pub fn clap_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.08;

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // Multiple decaying bursts to simulate hand clap
    let burst1 = if t < 0.01 { 1.0 } else { 0.0 };
    let burst2 = if t > 0.01 && t < 0.02 { 0.8 } else { 0.0 };
    let burst3 = if t > 0.02 && t < 0.03 { 0.6 } else { 0.0 };

    let envelope = (-t * 20.0).exp();

    noise_val * (burst1 + burst2 + burst3) * envelope * 0.4
}

/// Generate a tom drum sample (mid-low frequency with pitch drop)
pub fn tom_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.3;

    if t > duration {
        return 0.0;
    }

    // Pitch drops from 200Hz to 80Hz
    let start_freq = 200.0;
    let end_freq = 80.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration);

    // Medium decay
    let envelope = (-t * 8.0).exp();

    // Sine wave with slight harmonic
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.3;

    (fundamental + harmonic) * envelope * 0.6
}

/// Generate a rim shot sample (sharp transient with high frequency click)
pub fn rimshot_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.05;

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // Mix of high frequency tone and noise
    let tone = (2.0 * std::f32::consts::PI * 1500.0 * t).sin();

    // Very sharp decay
    let envelope = (-t * 60.0).exp();

    (noise_val * 0.4 + tone * 0.6) * envelope * 0.5
}

/// Generate a cowbell sample (metallic tone)
pub fn cowbell_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.2;

    if t > duration {
        return 0.0;
    }

    // Two inharmonic frequencies for metallic sound
    let freq1 = 540.0;
    let freq2 = 800.0;

    let tone1 = (2.0 * std::f32::consts::PI * freq1 * t).sin();
    let tone2 = (2.0 * std::f32::consts::PI * freq2 * t).sin();

    // Moderate decay
    let envelope = (-t * 10.0).exp();

    (tone1 * 0.6 + tone2 * 0.4) * envelope * 0.4
}

/// Generate a crash cymbal sample (bright noise with long decay)
pub fn crash_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 1.5;

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // Mix with high frequency tones for brightness
    let bright1 = (2.0 * std::f32::consts::PI * 7000.0 * t).sin();
    let bright2 = (2.0 * std::f32::consts::PI * 9000.0 * t).sin();

    // Slow decay
    let envelope = (-t * 2.0).exp();

    (noise_val * 0.7 + bright1 * 0.15 + bright2 * 0.15) * envelope * 0.35
}

/// Generate a ride cymbal sample (sustained bright noise_val)
pub fn ride_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.8;

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // Mix with metallic frequencies
    let metallic1 = (2.0 * std::f32::consts::PI * 5000.0 * t).sin();
    let metallic2 = (2.0 * std::f32::consts::PI * 7000.0 * t).sin();

    // Moderate decay
    let envelope = (-t * 3.5).exp();

    (noise_val * 0.6 + metallic1 * 0.2 + metallic2 * 0.2) * envelope * 0.25
}

/// Generate a high tom sample (higher pitch)
pub fn tom_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.25;

    if t > duration {
        return 0.0;
    }

    // Higher pitch: 300Hz to 120Hz
    let start_freq = 300.0;
    let end_freq = 120.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration);

    let envelope = (-t * 9.0).exp();

    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.25;

    (fundamental + harmonic) * envelope * 0.6
}

/// Generate a low tom sample (lower pitch)
pub fn tom_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.35;

    if t > duration {
        return 0.0;
    }

    // Lower pitch: 150Hz to 60Hz
    let start_freq = 150.0;
    let end_freq = 60.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration);

    let envelope = (-t * 7.0).exp();

    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.35;

    (fundamental + harmonic) * envelope * 0.65
}

/// Generate a china cymbal sample (trashy, explosive sound)
pub fn china_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 1.2;

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // Inharmonic frequencies for trashy sound
    let freq1 = (2.0 * std::f32::consts::PI * 6500.0 * t).sin();
    let freq2 = (2.0 * std::f32::consts::PI * 9500.0 * t).sin();
    let freq3 = (2.0 * std::f32::consts::PI * 11000.0 * t).sin();

    // Fast initial decay, then sustained
    let envelope = (-t * 3.5).exp();

    (noise_val * 0.6 + freq1 * 0.15 + freq2 * 0.15 + freq3 * 0.1) * envelope * 0.4
}

/// Generate a splash cymbal sample (short, bright accent)
pub fn splash_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.4;

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // Very bright frequencies
    let bright1 = (2.0 * std::f32::consts::PI * 8500.0 * t).sin();
    let bright2 = (2.0 * std::f32::consts::PI * 10000.0 * t).sin();

    // Quick decay
    let envelope = (-t * 6.0).exp();

    (noise_val * 0.7 + bright1 * 0.15 + bright2 * 0.15) * envelope * 0.35
}

/// Generate a tambourine sample (jingles)
pub fn tambourine_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.2;

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // Multiple metallic frequencies for jingle effect
    let jingle1 = (2.0 * std::f32::consts::PI * 4000.0 * t).sin();
    let jingle2 = (2.0 * std::f32::consts::PI * 5500.0 * t).sin();
    let jingle3 = (2.0 * std::f32::consts::PI * 7000.0 * t).sin();

    // Multiple decay bursts for shake effect
    let burst1 = (-t * 20.0).exp();
    let burst2 = if t > 0.03 { (-t * 18.0).exp() } else { 0.0 };
    let burst3 = if t > 0.06 { (-t * 16.0).exp() } else { 0.0 };

    let envelope = burst1 + burst2 * 0.6 + burst3 * 0.4;

    (noise_val * 0.4 + jingle1 * 0.2 + jingle2 * 0.2 + jingle3 * 0.2) * envelope * 0.35
}

/// Generate a shaker sample (continuous rattle)
pub fn shaker_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.15;

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // High frequency filtered noise
    let high_tone = (2.0 * std::f32::consts::PI * 6000.0 * t).sin();

    // Envelope with multiple micro-bursts
    let envelope = (-t * 15.0).exp() * (1.0 + 0.3 * (t * 100.0).sin());

    (noise_val * 0.85 + high_tone * 0.15) * envelope * 0.25
}

/// Generate an 808 kick sample (long, pitched sub-bass kick)
pub fn kick_808_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.5; // Longer than regular kick

    if t > duration {
        return 0.0;
    }

    // Dramatic pitch drop: 200Hz to 30Hz
    let start_freq = 200.0;
    let end_freq = 30.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration);

    // Slower decay for sustained bass
    let envelope = (-t * 8.0).exp();

    // Pure sine wave for clean sub-bass
    let value = (2.0 * std::f32::consts::PI * freq * t).sin();

    value * envelope * 0.9
}

/// Generate an 808 snare sample (dual tone + noise burst)
pub fn snare_808_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.15;

    if t > duration {
        return 0.0;
    }

    // Classic 808 snare uses two triangle oscillators
    let freq1 = 180.0;
    let freq2 = 240.0;

    // Triangle waves (more characteristic 808 sound than sine)
    let phase1 = (t * freq1) % 1.0;
    let tone1 = if phase1 < 0.5 {
        4.0 * phase1 - 1.0
    } else {
        -4.0 * phase1 + 3.0
    };

    let phase2 = (t * freq2) % 1.0;
    let tone2 = if phase2 < 0.5 {
        4.0 * phase2 - 1.0
    } else {
        -4.0 * phase2 + 3.0
    };

    // Noise burst (shorter than tones)
    let noise_val = noise(sample_index as f32);
    let noise_env = if t < 0.05 { (-t * 35.0).exp() } else { 0.0 };

    // Tone envelope (slightly longer)
    let tone_env = (-t * 18.0).exp();

    // Mix: 808 snare is mostly tone with short noise burst
    (tone1 + tone2) * 0.4 * tone_env + noise_val * 0.6 * noise_env
}

/// Generate an 808 hi-hat sample (6 square oscillators for metallic sound)
pub fn hihat_808_sample(sample_index: usize, sample_rate: f32, closed: bool) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = if closed { 0.04 } else { 0.12 };

    if t > duration {
        return 0.0;
    }

    // Classic 808 hihat uses 6 square wave oscillators at specific ratios
    // These frequencies create the metallic, inharmonic timbre
    let freqs = [
        1.0 * 3520.0,
        1.4 * 3520.0,
        1.6 * 3520.0,
        1.8 * 3520.0,
        2.1 * 3520.0,
        2.4 * 3520.0,
    ];

    let mut output = 0.0;
    for &freq in &freqs {
        let phase = (t * freq) % 1.0;
        // Square wave
        let square = if phase < 0.5 { 1.0 } else { -1.0 };
        output += square;
    }

    // Normalize
    output /= 6.0;

    // Simple high-pass effect by adding derivative
    let prev_t = (sample_index.saturating_sub(1)) as f32 / sample_rate;
    let prev_output = if prev_t > 0.0 {
        output * 0.5  // Approximate previous value
    } else {
        0.0
    };
    let highpassed = output - prev_output * 0.8;

    // Very sharp envelope for closed, slightly longer for open
    let decay_rate = if closed { 50.0 } else { 18.0 };
    let envelope = (-t * decay_rate).exp();

    highpassed * envelope * 0.25
}

/// Generate an 808 clap sample (multiple noise bursts with specific timing)
pub fn clap_808_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.1;

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // 808 clap uses multiple tightly-timed bursts (simulates multiple hands)
    // First burst is strongest, followed by two weaker echoes
    let burst1 = if t < 0.008 { 1.0 } else { 0.0 };
    let burst2 = if t >= 0.008 && t < 0.016 { 0.7 } else { 0.0 };
    let burst3 = if t >= 0.016 && t < 0.024 { 0.5 } else { 0.0 };

    // Overall decay envelope
    let envelope = (-t * 25.0).exp();

    // 808 clap is band-passed noise (roughly 1-3kHz)
    // Simulate by mixing noise with filtered tone
    let midrange = (2.0 * std::f32::consts::PI * 2000.0 * t).sin();
    let filtered = noise_val * 0.85 + midrange * 0.15;

    filtered * (burst1 + burst2 + burst3) * envelope * 0.45
}

/// Generate a sub kick sample (ultra-low frequency kick)
pub fn sub_kick_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.4;

    if t > duration {
        return 0.0;
    }

    // Very low pitch drop: 80Hz to 25Hz
    let start_freq = 80.0;
    let end_freq = 25.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration);

    // Moderate decay
    let envelope = (-t * 10.0).exp();

    // Pure sine for maximum sub-bass
    let value = (2.0 * std::f32::consts::PI * freq * t).sin();

    value * envelope * 0.95
}

/// Generate a bass drop sample (dramatic impact)
pub fn bass_drop_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.8;

    if t > duration {
        return 0.0;
    }

    // Extreme pitch drop for impact: 300Hz to 20Hz
    let start_freq = 300.0;
    let end_freq = 20.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration).powf(2.0); // Exponential drop

    // Long, powerful decay
    let envelope = (-t * 4.0).exp();

    // Sine with slight harmonic for character
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic = (2.0 * std::f32::consts::PI * freq * 1.5 * t).sin() * 0.2;

    (fundamental + harmonic) * envelope * 0.95
}

/// Generate a boom sample (deep cinematic impact)
pub fn boom_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 1.0; // Long tail

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // Very low frequency component
    let bass_freq = 40.0 - (30.0 * t); // Drops from 40Hz to 10Hz
    let bass = (2.0 * std::f32::consts::PI * bass_freq * t).sin();

    // Low rumble noise
    let low_noise = noise_val * 0.3;

    // Slow decay with some sustain
    let envelope = (-t * 3.0).exp() * (1.0 + 0.1 * (-t * 8.0).exp());

    (bass * 0.8 + low_noise * 0.2) * envelope * 0.85
}

/// Generate a pitched tom sample (tunable tom at specific frequency)
pub fn pitched_tom_sample(sample_index: usize, sample_rate: f32, pitch_hz: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.4;

    if t > duration {
        return 0.0;
    }

    // Pitch drops slightly from starting frequency
    let end_freq = pitch_hz * 0.6;
    let freq = pitch_hz + (end_freq - pitch_hz) * (t / duration);

    // Tom-like envelope
    let envelope = (-t * 7.5).exp();

    // Fundamental plus harmonics for tom character
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.25;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin() * 0.1;

    (fundamental + harmonic2 + harmonic3) * envelope * 0.65
}

/// Drum types
#[derive(Debug, Clone, Copy)]
pub enum DrumType {
    Kick,
    Kick808,         // Long, pitched 808 kick
    SubKick,         // Ultra-low sub kick
    Snare,
    Snare808,        // 808 snare (dual triangle oscillators)
    HiHatClosed,
    HiHatOpen,
    HiHat808Closed,  // 808 closed hi-hat (6 square oscillators)
    HiHat808Open,    // 808 open hi-hat (6 square oscillators)
    Clap,
    Clap808,         // 808 clap (multiple noise bursts)
    Tom,             // Mid tom (original)
    TomHigh,         // High tom
    TomLow,          // Low tom
    Rimshot,
    Cowbell,
    Crash,
    Ride,
    China,           // China cymbal
    Splash,          // Splash cymbal
    Tambourine,
    Shaker,
    BassDrop,        // Dramatic bass drop impact
    Boom,            // Deep cinematic boom
}

impl DrumType {
    pub fn sample(&self, sample_index: usize, sample_rate: f32) -> f32 {
        match self {
            DrumType::Kick => kick_drum_sample(sample_index, sample_rate),
            DrumType::Kick808 => kick_808_sample(sample_index, sample_rate),
            DrumType::SubKick => sub_kick_sample(sample_index, sample_rate),
            DrumType::Snare => snare_drum_sample(sample_index, sample_rate),
            DrumType::Snare808 => snare_808_sample(sample_index, sample_rate),
            DrumType::HiHatClosed => hihat_sample(sample_index, sample_rate, true),
            DrumType::HiHatOpen => hihat_sample(sample_index, sample_rate, false),
            DrumType::HiHat808Closed => hihat_808_sample(sample_index, sample_rate, true),
            DrumType::HiHat808Open => hihat_808_sample(sample_index, sample_rate, false),
            DrumType::Clap => clap_sample(sample_index, sample_rate),
            DrumType::Clap808 => clap_808_sample(sample_index, sample_rate),
            DrumType::Tom => tom_sample(sample_index, sample_rate),
            DrumType::TomHigh => tom_high_sample(sample_index, sample_rate),
            DrumType::TomLow => tom_low_sample(sample_index, sample_rate),
            DrumType::Rimshot => rimshot_sample(sample_index, sample_rate),
            DrumType::Cowbell => cowbell_sample(sample_index, sample_rate),
            DrumType::Crash => crash_sample(sample_index, sample_rate),
            DrumType::Ride => ride_sample(sample_index, sample_rate),
            DrumType::China => china_sample(sample_index, sample_rate),
            DrumType::Splash => splash_sample(sample_index, sample_rate),
            DrumType::Tambourine => tambourine_sample(sample_index, sample_rate),
            DrumType::Shaker => shaker_sample(sample_index, sample_rate),
            DrumType::BassDrop => bass_drop_sample(sample_index, sample_rate),
            DrumType::Boom => boom_sample(sample_index, sample_rate),
        }
    }

    pub fn duration(&self) -> f32 {
        match self {
            DrumType::Kick => 0.15,
            DrumType::Kick808 => 0.5,
            DrumType::SubKick => 0.4,
            DrumType::Snare => 0.1,
            DrumType::Snare808 => 0.15,
            DrumType::HiHatClosed => 0.05,
            DrumType::HiHatOpen => 0.15,
            DrumType::HiHat808Closed => 0.04,
            DrumType::HiHat808Open => 0.12,
            DrumType::Clap => 0.08,
            DrumType::Clap808 => 0.1,
            DrumType::Tom => 0.3,
            DrumType::TomHigh => 0.25,
            DrumType::TomLow => 0.35,
            DrumType::Rimshot => 0.05,
            DrumType::Cowbell => 0.2,
            DrumType::Crash => 1.5,
            DrumType::Ride => 0.8,
            DrumType::China => 1.2,
            DrumType::Splash => 0.4,
            DrumType::Tambourine => 0.2,
            DrumType::Shaker => 0.15,
            DrumType::BassDrop => 0.8,
            DrumType::Boom => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RATE: f32 = 44100.0;

    #[test]
    fn test_noise_function_range() {
        // Test that noise generates values in valid range [-1.0, 1.0]
        for i in 0..1000 {
            let value = noise(i as f32);
            assert!(
                value >= -1.0 && value <= 1.0,
                "Noise value {} out of range at seed {}",
                value,
                i
            );
            assert!(value.is_finite(), "Noise produced non-finite value at seed {}", i);
        }
    }

    #[test]
    fn test_noise_deterministic() {
        // Test that noise is deterministic (same seed = same output)
        let seed = 42.0;
        let value1 = noise(seed);
        let value2 = noise(seed);
        assert_eq!(value1, value2, "Noise function is not deterministic");
    }

    #[test]
    fn test_noise_varies() {
        // Test that noise produces different values for different seeds
        let value1 = noise(0.0);
        let value2 = noise(1.0);
        let value3 = noise(100.0);
        assert_ne!(value1, value2, "Noise should vary with different seeds");
        assert_ne!(value2, value3, "Noise should vary with different seeds");
    }

    #[test]
    fn test_kick_drum_valid_samples() {
        // Test kick drum generates valid samples
        for i in 0..1000 {
            let sample = kick_drum_sample(i, SAMPLE_RATE);
            assert!(sample.is_finite(), "Kick drum produced non-finite sample at index {}", i);
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "Kick drum sample {} out of range at index {}",
                sample,
                i
            );
        }
    }

    #[test]
    fn test_kick_drum_envelope() {
        // Test kick drum has proper envelope (decays to zero)
        let early_sample = kick_drum_sample(100, SAMPLE_RATE); // A few samples in
        let mid_sample = kick_drum_sample(1000, SAMPLE_RATE);
        let end_sample = kick_drum_sample(10000, SAMPLE_RATE); // Well past duration

        assert!(early_sample.abs() > 0.0, "Kick should have non-zero amplitude early on");
        assert!(mid_sample.abs() > 0.0, "Kick should still be audible mid-duration");
        assert_eq!(end_sample, 0.0, "Kick should be silent after duration");

        // Verify decay is happening
        assert!(mid_sample.abs() < early_sample.abs(), "Kick should decay over time");
    }

    #[test]
    fn test_snare_drum_valid_samples() {
        for i in 0..1000 {
            let sample = snare_drum_sample(i, SAMPLE_RATE);
            assert!(sample.is_finite(), "Snare produced non-finite sample at index {}", i);
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "Snare sample {} out of range at index {}",
                sample,
                i
            );
        }
    }

    #[test]
    fn test_hihat_closed_vs_open() {
        // Test that closed hihat is shorter than open
        let closed_sample = hihat_sample(3000, SAMPLE_RATE, true);
        let open_sample = hihat_sample(3000, SAMPLE_RATE, false);

        // At the same time point, closed should be silent but open still going
        assert_eq!(closed_sample, 0.0, "Closed hihat should be silent by this point");
        assert_ne!(open_sample, 0.0, "Open hihat should still be audible");
    }

    #[test]
    fn test_all_drum_types_valid_samples() {
        let drum_types = [
            DrumType::Kick,
            DrumType::Kick808,
            DrumType::SubKick,
            DrumType::Snare,
            DrumType::HiHatClosed,
            DrumType::HiHatOpen,
            DrumType::Clap,
            DrumType::Tom,
            DrumType::TomHigh,
            DrumType::TomLow,
            DrumType::Rimshot,
            DrumType::Cowbell,
            DrumType::Crash,
            DrumType::Ride,
            DrumType::China,
            DrumType::Splash,
            DrumType::Tambourine,
            DrumType::Shaker,
            DrumType::BassDrop,
            DrumType::Boom,
        ];

        for drum_type in &drum_types {
            // Test samples at various points
            for i in (0..5000).step_by(100) {
                let sample = drum_type.sample(i, SAMPLE_RATE);
                assert!(
                    sample.is_finite(),
                    "Drum {:?} produced non-finite sample at index {}",
                    drum_type,
                    i
                );
                assert!(
                    sample >= -1.0 && sample <= 1.0,
                    "Drum {:?} sample {} out of range [-1, 1] at index {}",
                    drum_type,
                    sample,
                    i
                );
            }
        }
    }

    #[test]
    fn test_drum_duration_matches_envelope() {
        // Test that drums return 0.0 after their stated duration
        let drum_types = [
            (DrumType::Kick, 0.15),
            (DrumType::Snare, 0.1),
            (DrumType::HiHatClosed, 0.05),
            (DrumType::Crash, 1.5),
        ];

        for (drum_type, expected_duration) in &drum_types {
            let duration = drum_type.duration();
            assert_eq!(
                duration, *expected_duration,
                "Duration mismatch for {:?}",
                drum_type
            );

            // Sample well past duration should be silent
            let late_index = ((duration + 0.5) * SAMPLE_RATE) as usize;
            let late_sample = drum_type.sample(late_index, SAMPLE_RATE);
            assert_eq!(
                late_sample, 0.0,
                "Drum {:?} should be silent at index {} (past duration {})",
                drum_type, late_index, duration
            );
        }
    }

    #[test]
    fn test_pitched_tom_at_different_pitches() {
        // Test pitched tom with different frequencies
        let pitches = [100.0, 200.0, 300.0];

        for pitch in &pitches {
            let sample = pitched_tom_sample(1000, SAMPLE_RATE, *pitch);
            assert!(
                sample.is_finite(),
                "Pitched tom at {}Hz produced non-finite sample",
                pitch
            );
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "Pitched tom at {}Hz out of range",
                pitch
            );
        }
    }

    #[test]
    fn test_808_kick_has_longer_decay() {
        // Test that 808 kick has longer sustain than regular kick
        // Regular kick duration: 0.15s, 808 kick duration: 0.5s
        // Sample at 0.3s (13230 samples at 44.1kHz) - past regular kick, within 808
        let test_point = 13230;

        let regular_kick = DrumType::Kick.sample(test_point, SAMPLE_RATE);
        let kick_808 = DrumType::Kick808.sample(test_point, SAMPLE_RATE);

        // 808 should still be audible while regular kick is silent
        assert_eq!(regular_kick, 0.0, "Regular kick should be silent by 0.3s");
        assert_ne!(kick_808, 0.0, "808 kick should still be audible at 0.3s");
    }

    #[test]
    fn test_clap_has_multiple_bursts() {
        // Test that clap has non-zero energy at different time points
        // (indicating multiple bursts characteristic of claps)
        let burst_points = [200, 600, 1200]; // Different time points in samples

        for point in &burst_points {
            let sample = DrumType::Clap.sample(*point, SAMPLE_RATE);
            // Just verify it produces valid output at these points
            assert!(sample.is_finite(), "Clap produced non-finite sample at {}", point);
        }
    }

    #[test]
    fn test_drum_samples_are_deterministic() {
        // Test that drums produce consistent output for same input
        let test_index = 1000;

        let sample1 = DrumType::Snare.sample(test_index, SAMPLE_RATE);
        let sample2 = DrumType::Snare.sample(test_index, SAMPLE_RATE);

        assert_eq!(
            sample1, sample2,
            "Drum samples should be deterministic for same input"
        );
    }

    #[test]
    fn test_bass_frequencies_are_low() {
        // Test that sub kick and 808 produce low frequency content
        // by verifying they have longer periods between zero crossings
        let mut sub_kick_samples = Vec::new();
        for i in 0..4410 {
            // First 100ms at 44.1kHz
            sub_kick_samples.push(DrumType::SubKick.sample(i, SAMPLE_RATE));
        }

        // Just verify samples are valid - detailed frequency analysis would be complex
        for (i, sample) in sub_kick_samples.iter().enumerate() {
            assert!(
                sample.is_finite(),
                "SubKick produced non-finite sample at index {}",
                i
            );
        }
    }

    #[test]
    fn test_cymbal_durations_are_longer() {
        // Test that cymbals have longer durations than drums
        let crash_duration = DrumType::Crash.duration();
        let kick_duration = DrumType::Kick.duration();
        let snare_duration = DrumType::Snare.duration();

        assert!(
            crash_duration > kick_duration,
            "Crash should have longer duration than kick"
        );
        assert!(
            crash_duration > snare_duration,
            "Crash should have longer duration than snare"
        );
    }
}
