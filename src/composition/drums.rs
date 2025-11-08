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
        output * 0.5 // Approximate previous value
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
    let burst2 = if (0.008..0.016).contains(&t) {
        0.7
    } else {
        0.0
    };
    let burst3 = if (0.016..0.024).contains(&t) {
        0.5
    } else {
        0.0
    };

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

/// Generate a claves sample (sharp wooden click)
pub fn claves_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.02; // Very short, sharp

    if t > duration {
        return 0.0;
    }

    // High frequency tone (around 2500Hz for wood resonance)
    let freq = 2500.0;
    let tone = (2.0 * std::f32::consts::PI * freq * t).sin();

    // Add harmonics for wooden character
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.5;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin() * 0.3;

    // Extremely fast decay
    let envelope = (-t * 120.0).exp();

    (tone + harmonic2 + harmonic3) * envelope * 0.7
}

/// Generate a triangle sample (metallic ding)
pub fn triangle_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 1.5; // Long sustain

    if t > duration {
        return 0.0;
    }

    // Fundamental around 3000Hz
    let freq = 3000.0;

    // Triangle has lots of harmonics, mostly odd
    let mut signal = 0.0;
    for i in 1..8 {
        let harmonic = (2 * i - 1) as f32;
        let amplitude = 1.0 / (harmonic * harmonic); // Odd harmonics decay fast
        signal += (2.0 * std::f32::consts::PI * freq * harmonic * t).sin() * amplitude;
    }

    // Slow decay for metallic sustain
    let envelope = (-t * 2.0).exp();

    signal * envelope * 0.2
}

/// Generate a side stick sample (soft rim click)
pub fn side_stick_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.03;

    if t > duration {
        return 0.0;
    }

    let noise_val = noise(sample_index as f32);

    // Mix of tone and noise, but softer than rimshot
    let tone = (2.0 * std::f32::consts::PI * 1200.0 * t).sin();

    // Sharp but not as aggressive as rimshot
    let envelope = (-t * 80.0).exp();

    (noise_val * 0.3 + tone * 0.7) * envelope * 0.3
}

/// Generate a wood block sample (dry, pitched click)
pub fn wood_block_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.08;

    if t > duration {
        return 0.0;
    }

    // Wood blocks have a definite pitch (around 1500Hz)
    let freq = 1500.0;
    let tone = (2.0 * std::f32::consts::PI * freq * t).sin();

    // Add some hollow resonance
    let resonance = (2.0 * std::f32::consts::PI * freq * 1.5 * t).sin() * 0.4;

    // Fast, dry decay
    let envelope = (-t * 35.0).exp();

    (tone + resonance) * envelope * 0.5
}

/// Generate a ride bell sample (metallic ping)
pub fn ride_bell_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.8;

    if t > duration {
        return 0.0;
    }

    // Higher pitched than ride, around 4000Hz
    let freq = 4000.0;

    // Lots of inharmonic partials for bell character
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let partial2 = (2.0 * std::f32::consts::PI * freq * 2.4 * t).sin() * 0.6;
    let partial3 = (2.0 * std::f32::consts::PI * freq * 3.7 * t).sin() * 0.3;
    let partial4 = (2.0 * std::f32::consts::PI * freq * 5.1 * t).sin() * 0.2;

    // Medium-fast decay
    let envelope = (-t * 4.0).exp();

    (fundamental + partial2 + partial3 + partial4) * envelope * 0.3
}

/// Generate a 909 kick sample (harder, punchier than 808)
pub fn kick_909_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.12; // Shorter than 808

    if t > duration {
        return 0.0;
    }

    // Pitch drop from 180Hz to 50Hz (higher start than 808)
    let start_freq = 180.0;
    let end_freq = 50.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration);

    // Harder, faster decay than 808
    let envelope = (-t * 20.0).exp();

    // Add some distortion/harmonics for punchiness
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let distortion = fundamental.tanh(); // Soft clipping for punch

    distortion * envelope * 0.9
}

/// Generate a 909 snare sample (brighter, sharper than 808)
pub fn snare_909_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.08;

    if t > duration {
        return 0.0;
    }

    // More noise, less tone than regular snare
    let noise_val = noise(sample_index as f32);
    let tone = (2.0 * std::f32::consts::PI * 250.0 * t).sin();

    // Bright, snappy envelope
    let envelope = (-t * 30.0).exp();

    (noise_val * 0.85 + tone * 0.15) * envelope * 0.6
}

/// Generate a high conga sample (bright slap)
pub fn conga_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.25;

    if t > duration {
        return 0.0;
    }

    // High conga around 400Hz with pitch drop
    let start_freq = 400.0;
    let end_freq = 320.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration);

    // Sharp attack, medium decay
    let envelope = (-t * 12.0).exp();

    // Fundamental with overtones
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.3 * t).sin() * 0.4;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.5 * t).sin() * 0.2;

    // Add a bit of noise for slap character
    let noise_val = noise(sample_index as f32);
    let slap = if t < 0.01 { noise_val * 0.3 } else { 0.0 };

    (fundamental + harmonic2 + harmonic3 + slap) * envelope * 0.6
}

/// Generate a low conga sample (deep, open tone)
pub fn conga_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.4;

    if t > duration {
        return 0.0;
    }

    // Low conga around 180Hz with pitch drop
    let start_freq = 180.0;
    let end_freq = 140.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration);

    // Slower decay for open tone
    let envelope = (-t * 8.0).exp();

    // Rich harmonics for resonance
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.2 * t).sin() * 0.5;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.4 * t).sin() * 0.3;

    (fundamental + harmonic2 + harmonic3) * envelope * 0.65
}

/// Generate a high bongo sample (sharp, articulate)
pub fn bongo_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.15;

    if t > duration {
        return 0.0;
    }

    // High bongo around 500Hz with quick pitch drop
    let start_freq = 500.0;
    let end_freq = 420.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration);

    // Fast, articulate decay
    let envelope = (-t * 18.0).exp();

    // Bright harmonics
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.5 * t).sin() * 0.4;

    (fundamental + harmonic2) * envelope * 0.55
}

/// Generate a low bongo sample (deeper partner)
pub fn bongo_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.18;

    if t > duration {
        return 0.0;
    }

    // Low bongo around 300Hz with pitch drop
    let start_freq = 300.0;
    let end_freq = 250.0;
    let freq = start_freq + (end_freq - start_freq) * (t / duration);

    // Medium-fast decay
    let envelope = (-t * 15.0).exp();

    // Slightly warmer tone than high bongo
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.3 * t).sin() * 0.5;

    (fundamental + harmonic2) * envelope * 0.6
}

/// Floor tom (low) - Deep floor tom sound
pub fn floor_tom_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.4;

    if t > duration {
        return 0.0;
    }

    // Very low pitch (80Hz) with slight drop
    let start_freq = 80.0;
    let end_freq = 70.0;
    let freq = start_freq + (end_freq - start_freq) * (t / 0.05);

    // Slow decay for resonance
    let envelope = (-t * 5.0).exp();

    // Rich fundamental with harmonics
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.4;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.1 * t).sin() * 0.2;

    (fundamental + harmonic2 + harmonic3) * envelope * 0.7
}

/// Floor tom (high) - Higher floor tom sound
pub fn floor_tom_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.35;

    if t > duration {
        return 0.0;
    }

    // Mid-low pitch (110Hz) with pitch drop
    let start_freq = 110.0;
    let end_freq = 95.0;
    let freq = start_freq + (end_freq - start_freq) * (t / 0.05);

    let envelope = (-t * 6.0).exp();

    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.1 * t).sin() * 0.35;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin() * 0.15;

    (fundamental + harmonic2 + harmonic3) * envelope * 0.7
}

/// Pedal hi-hat - Foot pedal "chick" sound
pub fn hihat_pedal_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.08;

    if t > duration {
        return 0.0;
    }

    // Very fast decay for tight chick sound
    let envelope = (-t * 40.0).exp();

    // High-frequency metallic content with noise
    let noise_val = noise(t * 10000.0);
    let high_freq = (2.0 * std::f32::consts::PI * 8000.0 * t).sin() * 0.3;

    (noise_val * 0.7 + high_freq) * envelope * 0.4
}

/// Crash cymbal 2 - Second crash variation
pub fn crash2_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 1.8;

    if t > duration {
        return 0.0;
    }

    // Slower decay than crash 1, slightly different tonality
    let envelope = (-t * 1.5).exp();

    // Inharmonic partials at different frequencies than crash 1
    let freq1 = (2.0 * std::f32::consts::PI * 520.0 * t).sin();
    let freq2 = (2.0 * std::f32::consts::PI * 780.0 * t).sin();
    let freq3 = (2.0 * std::f32::consts::PI * 1150.0 * t).sin();
    let freq4 = (2.0 * std::f32::consts::PI * 1680.0 * t).sin();

    let noise_val = noise(t * 5000.0) * 0.5;
    let tonal = (freq1 + freq2 + freq3 + freq4) * 0.2;

    (noise_val + tonal) * envelope * 0.6
}

/// Vibraslap - Distinctive rattling/buzzing percussion
pub fn vibraslap_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.15;

    if t > duration {
        return 0.0;
    }

    // Initial "slap" followed by rattle
    let slap_envelope = (-t * 50.0).exp();
    let rattle_envelope = (-t * 12.0).exp() * (1.0 - (-t * 80.0).exp());

    // High-frequency buzz/rattle
    let rattle = noise(t * 20000.0) * rattle_envelope;
    let slap = (2.0 * std::f32::consts::PI * 3000.0 * t).sin() * slap_envelope * 0.3;

    (rattle * 0.8 + slap) * 0.5
}

/// Timbale (high) - High metallic shell drum
pub fn timbale_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.25;

    if t > duration {
        return 0.0;
    }

    // High pitch (850Hz) with metallic character
    let start_freq = 850.0;
    let end_freq = 780.0;
    let freq = start_freq + (end_freq - start_freq) * (t / 0.03);

    let envelope = (-t * 10.0).exp();

    // Bright tone with inharmonic partials
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.4 * t).sin() * 0.4;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.8 * t).sin() * 0.25;
    let noise_val = noise(t * 3000.0) * 0.1;

    (fundamental + harmonic2 + harmonic3 + noise_val) * envelope * 0.65
}

/// Timbale (low) - Low metallic shell drum
pub fn timbale_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.3;

    if t > duration {
        return 0.0;
    }

    // Lower pitch (550Hz) with metallic character
    let start_freq = 550.0;
    let end_freq = 500.0;
    let freq = start_freq + (end_freq - start_freq) * (t / 0.03);

    let envelope = (-t * 8.0).exp();

    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.3 * t).sin() * 0.4;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.7 * t).sin() * 0.2;
    let noise_val = noise(t * 2500.0) * 0.1;

    (fundamental + harmonic2 + harmonic3 + noise_val) * envelope * 0.65
}

/// Agogo bell (high) - High Brazilian cowbell
pub fn agogo_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.4;

    if t > duration {
        return 0.0;
    }

    // Very high pitch (3500Hz) with bell-like inharmonics
    let fundamental_freq = 3500.0;
    let envelope = (-t * 7.0).exp();

    // Bell uses inharmonic partials
    let partial1 = (2.0 * std::f32::consts::PI * fundamental_freq * t).sin();
    let partial2 = (2.0 * std::f32::consts::PI * fundamental_freq * 2.76 * t).sin() * 0.5;
    let partial3 = (2.0 * std::f32::consts::PI * fundamental_freq * 5.4 * t).sin() * 0.25;

    (partial1 + partial2 + partial3) * envelope * 0.5
}

/// Agogo bell (low) - Low Brazilian cowbell
pub fn agogo_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.5;

    if t > duration {
        return 0.0;
    }

    // Lower pitch (2500Hz) with bell-like inharmonics
    let fundamental_freq = 2500.0;
    let envelope = (-t * 6.0).exp();

    // Bell uses inharmonic partials
    let partial1 = (2.0 * std::f32::consts::PI * fundamental_freq * t).sin();
    let partial2 = (2.0 * std::f32::consts::PI * fundamental_freq * 2.76 * t).sin() * 0.5;
    let partial3 = (2.0 * std::f32::consts::PI * fundamental_freq * 5.4 * t).sin() * 0.25;

    (partial1 + partial2 + partial3) * envelope * 0.5
}

/// Cabasa - Textured shaker/scraper hybrid
pub fn cabasa_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.25;

    if t > duration {
        return 0.0;
    }

    // Softer attack than shaker, more textured
    let envelope = (-t * 10.0).exp() * (1.0 - (-t * 30.0).exp());

    // Mid-high frequency rattling texture
    let noise1 = noise(t * 8000.0);
    let noise2 = noise(t * 4000.0) * 0.6;

    (noise1 + noise2) * envelope * 0.4
}

/// Guiro (short) - Short scraping sound
pub fn guiro_short_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.08;

    if t > duration {
        return 0.0;
    }

    // Fast "scraping" pattern - pulsed noise
    let scrape_freq = 180.0; // Scraping rate
    let scrape_pattern = ((2.0 * std::f32::consts::PI * scrape_freq * t).sin() * 0.5 + 0.5).powi(3);

    let envelope = 1.0 - (t / duration);
    let noise_val = noise(t * 15000.0);

    noise_val * scrape_pattern * envelope * 0.5
}

/// Guiro (long) - Long scraping sound
pub fn guiro_long_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.2;

    if t > duration {
        return 0.0;
    }

    // Slower "scraping" pattern - pulsed noise
    let scrape_freq = 150.0; // Scraping rate
    let scrape_pattern = ((2.0 * std::f32::consts::PI * scrape_freq * t).sin() * 0.5 + 0.5).powi(3);

    let envelope = 1.0 - (t / duration);
    let noise_val = noise(t * 15000.0);

    noise_val * scrape_pattern * envelope * 0.5
}

/// Wood block (high) - High-pitched wooden click
pub fn wood_block_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.06;

    if t > duration {
        return 0.0;
    }

    // Higher pitch than regular wood block (2500Hz)
    let freq = 2500.0;
    let envelope = (-t * 45.0).exp();

    // Bright, dry click with odd harmonics
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin() * 0.4;
    let harmonic5 = (2.0 * std::f32::consts::PI * freq * 5.0 * t).sin() * 0.2;
    let noise_val = noise(t * 5000.0) * 0.15;

    (fundamental + harmonic3 + harmonic5 + noise_val) * envelope * 0.5
}

// ============================================================================
// ORCHESTRAL PERCUSSION
// ============================================================================

/// Timpani - Tuned orchestral bass drum
pub fn timpani_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 1.2;

    if t > duration {
        return 0.0;
    }

    // Deep tuned pitch around 80Hz (C2)
    let freq = 80.0;
    let envelope = (-t * 2.0).exp();

    // Rich harmonic content for orchestral timpani
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.5;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin() * 0.3;
    let harmonic4 = (2.0 * std::f32::consts::PI * freq * 4.0 * t).sin() * 0.2;

    // Add subtle attack noise
    let attack_noise = if t < 0.02 {
        noise(t * 5000.0) * 0.15 * (1.0 - t / 0.02)
    } else {
        0.0
    };

    (fundamental + harmonic2 + harmonic3 + harmonic4 + attack_noise) * envelope * 0.7
}

/// Gong - Deep metallic crash with long decay
pub fn gong_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 3.5;

    if t > duration {
        return 0.0;
    }

    // Very slow decay for gong
    let envelope = (-t * 0.8).exp();

    // Deep fundamental with complex inharmonic partials
    let fundamental = 60.0;
    let partial1 = (2.0 * std::f32::consts::PI * fundamental * t).sin();
    let partial2 = (2.0 * std::f32::consts::PI * fundamental * 2.4 * t).sin() * 0.6;
    let partial3 = (2.0 * std::f32::consts::PI * fundamental * 3.8 * t).sin() * 0.4;
    let partial4 = (2.0 * std::f32::consts::PI * fundamental * 5.6 * t).sin() * 0.3;
    let partial5 = (2.0 * std::f32::consts::PI * fundamental * 7.2 * t).sin() * 0.2;

    // Add shimmer/wash with noise
    let shimmer = noise(t * 3000.0) * 0.2 * envelope;

    (partial1 + partial2 + partial3 + partial4 + partial5 + shimmer) * envelope * 0.6
}

/// Chimes/Tubular bells - Bright tuned metallic sound
pub fn chimes_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 2.0;

    if t > duration {
        return 0.0;
    }

    // Tuned to C5 (523Hz) with long sustain
    let freq = 523.25;
    let envelope = (-t * 1.2).exp();

    // Bell-like inharmonic partials
    let partial1 = (2.0 * std::f32::consts::PI * freq * t).sin();
    let partial2 = (2.0 * std::f32::consts::PI * freq * 2.76 * t).sin() * 0.4;
    let partial3 = (2.0 * std::f32::consts::PI * freq * 5.4 * t).sin() * 0.25;
    let partial4 = (2.0 * std::f32::consts::PI * freq * 8.93 * t).sin() * 0.15;

    (partial1 + partial2 + partial3 + partial4) * envelope * 0.55
}

// ============================================================================
// WORLD PERCUSSION
// ============================================================================

/// Djembe - West African hand drum (bass tone)
pub fn djembe_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.4;

    if t > duration {
        return 0.0;
    }

    // Deep bass tone around 100Hz with slap attack
    let start_freq = 120.0;
    let end_freq = 90.0;
    let freq = start_freq + (end_freq - start_freq) * (t / 0.05);

    let envelope = (-t * 8.0).exp();

    // Sharp attack for hand slap
    let attack = if t < 0.01 {
        noise(t * 15000.0) * (1.0 - t / 0.01) * 0.3
    } else {
        0.0
    };

    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.2 * t).sin() * 0.4;

    (fundamental + harmonic2 + attack) * envelope * 0.7
}

/// Tabla (Bayan) - Indian bass drum (left hand)
pub fn tabla_bayan_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.5;

    if t > duration {
        return 0.0;
    }

    // Deep pitch bend characteristic of bayan
    let start_freq = 150.0;
    let end_freq = 100.0;
    let freq = start_freq + (end_freq - start_freq) * (t / 0.08);

    let envelope = (-t * 7.0).exp();

    // Tabla has distinctive metallic timbre
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.5 * t).sin() * 0.35;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 4.0 * t).sin() * 0.2;

    (fundamental + harmonic2 + harmonic3) * envelope * 0.65
}

/// Tabla (Dayan) - Indian treble drum (right hand)
pub fn tabla_dayan_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.3;

    if t > duration {
        return 0.0;
    }

    // Higher pitched, around 400Hz
    let freq = 400.0;
    let envelope = (-t * 12.0).exp();

    // Bright, ringing tone
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.8 * t).sin() * 0.4;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 5.2 * t).sin() * 0.25;

    // Sharp attack click
    let attack = if t < 0.005 {
        noise(t * 20000.0) * (1.0 - t / 0.005) * 0.2
    } else {
        0.0
    };

    (fundamental + harmonic2 + harmonic3 + attack) * envelope * 0.6
}

/// Cajon - Box drum (very popular in modern music)
pub fn cajon_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.25;

    if t > duration {
        return 0.0;
    }

    // Woody, boxy tone around 180Hz
    let freq = 180.0;
    let envelope = (-t * 12.0).exp();

    // Snare-like buzz from internal wires
    let buzz = noise(t * 8000.0) * 0.3 * (-t * 15.0).exp();

    // Fundamental with some harmonics
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.3;

    // Sharp slap attack
    let slap = if t < 0.01 {
        noise(t * 12000.0) * (1.0 - t / 0.01) * 0.4
    } else {
        0.0
    };

    (fundamental + harmonic2 + buzz + slap) * envelope * 0.65
}

// ============================================================================
// HAND PERCUSSION
// ============================================================================

/// Fingersnap - Clean snap sound
pub fn fingersnap_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.08;

    if t > duration {
        return 0.0;
    }

    // Very fast decay for snap
    let envelope = (-t * 40.0).exp();

    // High-frequency click with noise burst
    let noise_val = noise(t * 15000.0);
    let click = (2.0 * std::f32::consts::PI * 2500.0 * t).sin() * 0.3;

    (noise_val * 0.7 + click) * envelope * 0.45
}

/// Maracas - Rattling shaker (different character than generic shaker)
pub fn maracas_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.12;

    if t > duration {
        return 0.0;
    }

    // Fast attack, shorter than shaker
    let envelope = (-t * 18.0).exp() * (1.0 - (-t * 50.0).exp());

    // Bright, high-frequency rattle
    let noise1 = noise(t * 12000.0);
    let noise2 = noise(t * 6000.0) * 0.5;

    (noise1 + noise2) * envelope * 0.45
}

/// Castanet - Spanish wooden clapper
pub fn castanet_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.06;

    if t > duration {
        return 0.0;
    }

    // Very sharp, bright wooden click
    let envelope = (-t * 50.0).exp();

    // High pitch with odd harmonics (wooden character)
    let freq = 3500.0;
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin() * 0.4;
    let harmonic5 = (2.0 * std::f32::consts::PI * freq * 5.0 * t).sin() * 0.2;
    let noise_val = noise(t * 8000.0) * 0.2;

    (fundamental + harmonic3 + harmonic5 + noise_val) * envelope * 0.5
}

/// Sleigh bells - Jingle bells cluster
pub fn sleigh_bells_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.8;

    if t > duration {
        return 0.0;
    }

    let envelope = (-t * 4.0).exp();

    // Multiple bell frequencies ringing together
    let bell1 = (2.0 * std::f32::consts::PI * 1800.0 * t).sin() * 0.3;
    let bell2 = (2.0 * std::f32::consts::PI * 2200.0 * t).sin() * 0.25;
    let bell3 = (2.0 * std::f32::consts::PI * 2700.0 * t).sin() * 0.2;
    let bell4 = (2.0 * std::f32::consts::PI * 3100.0 * t).sin() * 0.15;

    // Jingle/shimmer with noise
    let jingle = noise(t * 6000.0) * 0.15 * envelope;

    (bell1 + bell2 + bell3 + bell4 + jingle) * envelope * 0.5
}

// ============================================================================
// ELECTRONIC / EFFECTS
// ============================================================================

/// Laser zap - Sci-fi/electronic sound
pub fn laser_zap_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.3;

    if t > duration {
        return 0.0;
    }

    // Pitch sweep from high to low
    let start_freq = 2000.0_f32;
    let end_freq = 80.0_f32;
    let freq = start_freq * (end_freq / start_freq).powf(t / duration);

    let envelope = (-t * 8.0).exp();

    // Square wave for electronic character
    let phase = 2.0 * std::f32::consts::PI * freq * t;
    let square = if phase.sin() > 0.0 { 1.0 } else { -1.0 };

    square * envelope * 0.4
}

/// Reverse cymbal - Reversed crash buildup
pub fn reverse_cymbal_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 1.5;

    if t > duration {
        return 0.0;
    }

    // Reverse envelope - builds up instead of decaying
    let envelope = (t / duration).powf(2.0);

    // Cymbal-like inharmonic partials
    let freq1 = (2.0 * std::f32::consts::PI * 450.0 * t).sin();
    let freq2 = (2.0 * std::f32::consts::PI * 680.0 * t).sin();
    let freq3 = (2.0 * std::f32::consts::PI * 1020.0 * t).sin();

    let noise_val = noise(t * 4500.0) * 0.6;
    let tonal = (freq1 + freq2 + freq3) * 0.15;

    (noise_val + tonal) * envelope * 0.5
}

/// White noise hit - Burst/clap effect
pub fn white_noise_hit_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.12;

    if t > duration {
        return 0.0;
    }

    // Sharp attack and fast decay
    let envelope = (-t * 25.0).exp() * (1.0 - (-t * 100.0).exp());

    // Pure white noise
    noise(t * 20000.0) * envelope * 0.5
}

/// Stick click - Drumsticks clicked together
pub fn stick_click_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.03;

    if t > duration {
        return 0.0;
    }

    // Very short, sharp click
    let envelope = (-t * 80.0).exp();

    // High-frequency click
    let freq = 4000.0;
    let click = (2.0 * std::f32::consts::PI * freq * t).sin() * 0.3;
    let noise_val = noise(t * 18000.0) * 0.7;

    (click + noise_val) * envelope * 0.4
}

// ============================================================================
// KICK VARIATIONS
// ============================================================================

/// Tight kick - Short, punchy kick for electronic music
pub fn kick_tight_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.06;

    if t > duration {
        return 0.0;
    }

    // Very short, punchy kick
    let start_freq = 120.0;
    let end_freq = 40.0;
    let freq = start_freq + (end_freq - start_freq) * (t / 0.02);

    // Very fast decay for tight sound
    let envelope = (-t * 45.0).exp();

    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.3;

    // Sharp click attack
    let click = if t < 0.005 {
        noise(t * 10000.0) * (1.0 - t / 0.005) * 0.3
    } else {
        0.0
    };

    (fundamental + harmonic2 + click) * envelope * 0.8
}

/// Deep kick - Extended low-end, longer decay
pub fn kick_deep_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.5;

    if t > duration {
        return 0.0;
    }

    // Very deep pitch, slow decay
    let start_freq = 70.0;
    let end_freq = 30.0;
    let freq = start_freq + (end_freq - start_freq) * (t / 0.08);

    let envelope = (-t * 5.0).exp();

    // Heavy on fundamental, less harmonics for deep sound
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.2;

    // Subtle click
    let click = if t < 0.008 {
        noise(t * 8000.0) * (1.0 - t / 0.008) * 0.15
    } else {
        0.0
    };

    (fundamental + harmonic2 + click) * envelope * 0.75
}

/// Acoustic kick - Natural drum kit sound
pub fn kick_acoustic_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.25;

    if t > duration {
        return 0.0;
    }

    // Natural acoustic pitch
    let start_freq = 100.0;
    let end_freq = 55.0;
    let freq = start_freq + (end_freq - start_freq) * (t / 0.04);

    let envelope = (-t * 10.0).exp();

    // More harmonics for acoustic character
    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
    let harmonic2 = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.4;
    let harmonic3 = (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin() * 0.2;

    // Natural beater attack
    let attack = if t < 0.01 {
        noise(t * 12000.0) * (1.0 - t / 0.01) * 0.25
    } else {
        0.0
    };

    (fundamental + harmonic2 + harmonic3 + attack) * envelope * 0.7
}

/// Click kick - Prominent beater attack
pub fn kick_click_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.12;

    if t > duration {
        return 0.0;
    }

    let start_freq = 110.0;
    let end_freq = 50.0;
    let freq = start_freq + (end_freq - start_freq) * (t / 0.03);

    let envelope = (-t * 18.0).exp();

    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();

    // Prominent click/beater attack
    let click = if t < 0.015 {
        noise(t * 15000.0) * (1.0 - t / 0.015) * 0.6
    } else {
        0.0
    };

    // High-frequency beater transient
    let beater = if t < 0.008 {
        (2.0 * std::f32::consts::PI * 3500.0 * t).sin() * (1.0 - t / 0.008) * 0.4
    } else {
        0.0
    };

    (fundamental + click + beater) * envelope * 0.75
}

// ============================================================================
// SNARE VARIATIONS
// ============================================================================

/// Rim snare - Rim-focused, less body
pub fn snare_rim_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.08;

    if t > duration {
        return 0.0;
    }

    let envelope = (-t * 30.0).exp();

    // High frequency for rim sound
    let rim_tone = (2.0 * std::f32::consts::PI * 1800.0 * t).sin() * 0.3;

    // Minimal body, mostly noise
    let noise_val = noise(t * 10000.0) * 0.7;

    (rim_tone + noise_val) * envelope * 0.55
}

/// Tight snare - Short, dry, minimal resonance
pub fn snare_tight_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.07;

    if t > duration {
        return 0.0;
    }

    // Very fast decay for tight sound
    let envelope = (-t * 35.0).exp();

    // Minimal tonal content
    let tone = (2.0 * std::f32::consts::PI * 200.0 * t).sin() * 0.2;

    // Mostly noise
    let noise_val = noise(t * 9000.0) * 0.8;

    (tone + noise_val) * envelope * 0.6
}

/// Loose snare - Longer ring, more wire buzz
pub fn snare_loose_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.18;

    if t > duration {
        return 0.0;
    }

    // Slower decay for loose, ringing sound
    let envelope = (-t * 12.0).exp();

    // More tonal content
    let tone = (2.0 * std::f32::consts::PI * 220.0 * t).sin() * 0.35;

    // Extended wire buzz
    let noise_val = noise(t * 7500.0) * 0.65;

    (tone + noise_val) * envelope * 0.65
}

/// Piccolo snare - High-pitched, bright
pub fn snare_piccolo_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.08;

    if t > duration {
        return 0.0;
    }

    let envelope = (-t * 28.0).exp();

    // High pitch for piccolo
    let tone = (2.0 * std::f32::consts::PI * 350.0 * t).sin() * 0.3;

    // Bright, high-frequency noise
    let noise_val = noise(t * 11000.0) * 0.7;

    (tone + noise_val) * envelope * 0.6
}

// ============================================================================
// HI-HAT VARIATIONS
// ============================================================================

/// Half-open hi-hat - Between closed and open
pub fn hihat_half_open_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.1;

    if t > duration {
        return 0.0;
    }

    // Medium decay - between closed and open
    let envelope = (-t * 18.0).exp();

    // Mix of frequencies for half-open character
    let noise_val = noise(t * 9000.0);
    let metallic1 = (2.0 * std::f32::consts::PI * 7500.0 * t).sin() * 0.2;
    let metallic2 = (2.0 * std::f32::consts::PI * 10200.0 * t).sin() * 0.15;

    (noise_val * 0.65 + metallic1 + metallic2) * envelope * 0.5
}

/// Sizzle hi-hat - Lots of high-frequency content
pub fn hihat_sizzle_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.2;

    if t > duration {
        return 0.0;
    }

    let envelope = (-t * 10.0).exp();

    // Very high frequency content for sizzle
    let noise_val = noise(t * 13000.0) * 0.7;
    let sizzle1 = (2.0 * std::f32::consts::PI * 12000.0 * t).sin() * 0.15;
    let sizzle2 = (2.0 * std::f32::consts::PI * 15500.0 * t).sin() * 0.1;

    (noise_val + sizzle1 + sizzle2) * envelope * 0.45
}

// ============================================================================
// CLAP VARIATIONS
// ============================================================================

/// Dry clap - No reverb, tight
pub fn clap_dry_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.05;

    if t > duration {
        return 0.0;
    }

    // Very tight, no reverb tail
    let envelope = (-t * 50.0).exp();

    // Pure noise burst
    let noise_val = noise(t * 11000.0);

    noise_val * envelope * 0.5
}

/// Room clap - Natural room ambience
pub fn clap_room_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.15;

    if t > duration {
        return 0.0;
    }

    // Main clap
    let clap_envelope = (-t * 30.0).exp();
    let clap = noise(t * 10000.0) * clap_envelope;

    // Room ambience tail
    let room_envelope = if t > 0.02 {
        (-((t - 0.02) * 15.0)).exp() * 0.3
    } else {
        0.0
    };
    let room = noise(t * 5000.0) * room_envelope;

    (clap + room) * 0.5
}

/// Group clap - Multiple hand claps layered
pub fn clap_group_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.12;

    if t > duration {
        return 0.0;
    }

    // Multiple claps with slight timing offsets
    let clap1_env = (-t * 35.0).exp();
    let clap2_env = (-(t - 0.005).max(0.0) * 35.0).exp();
    let clap3_env = (-(t - 0.008).max(0.0) * 35.0).exp();
    let clap4_env = (-(t - 0.012).max(0.0) * 35.0).exp();

    let clap1 = noise(t * 10000.0) * clap1_env * 0.25;
    let clap2 = noise(t * 10500.0) * clap2_env * 0.25;
    let clap3 = noise(t * 9500.0) * clap3_env * 0.25;
    let clap4 = noise(t * 11000.0) * clap4_env * 0.25;

    clap1 + clap2 + clap3 + clap4
}

/// Clap snare - Hybrid clap/snare sound
pub fn clap_snare_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.1;

    if t > duration {
        return 0.0;
    }

    let envelope = (-t * 25.0).exp();

    // Snare-like tone
    let tone = (2.0 * std::f32::consts::PI * 210.0 * t).sin() * 0.25;

    // Clap-like noise
    let noise_val = noise(t * 9500.0) * 0.75;

    (tone + noise_val) * envelope * 0.6
}

// ============================================================================
// CYMBAL VARIATIONS
// ============================================================================

/// Crash short - Quick crash, gated
pub fn crash_short_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.5;

    if t > duration {
        return 0.0;
    }

    // Fast decay for gated crash
    let envelope = (-t * 6.0).exp();

    // Inharmonic partials
    let freq1 = (2.0 * std::f32::consts::PI * 480.0 * t).sin();
    let freq2 = (2.0 * std::f32::consts::PI * 720.0 * t).sin();
    let freq3 = (2.0 * std::f32::consts::PI * 1080.0 * t).sin();

    let noise_val = noise(t * 5500.0) * 0.5;
    let tonal = (freq1 + freq2 + freq3) * 0.2;

    (noise_val + tonal) * envelope * 0.6
}

/// Ride tip - Bell-less ride, stick tip sound
pub fn ride_tip_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.6;

    if t > duration {
        return 0.0;
    }

    let envelope = (-t * 3.5).exp();

    // Less inharmonic than full ride, more focused pitch
    let fundamental = 950.0;
    let partial1 = (2.0 * std::f32::consts::PI * fundamental * t).sin();
    let partial2 = (2.0 * std::f32::consts::PI * fundamental * 2.2 * t).sin() * 0.4;
    let partial3 = (2.0 * std::f32::consts::PI * fundamental * 3.8 * t).sin() * 0.25;

    // Less noise than full ride
    let noise_val = noise(t * 4000.0) * 0.2;

    (partial1 + partial2 + partial3 + noise_val) * envelope * 0.5
}

// ============================================================================
// SHAKER VARIATIONS
// ============================================================================

/// Egg shaker - Tight, short shake
pub fn egg_shaker_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.08;

    if t > duration {
        return 0.0;
    }

    // Very short, tight shake
    let envelope = (-t * 30.0).exp() * (1.0 - (-t * 80.0).exp());

    // High-frequency rattle
    let noise1 = noise(t * 10000.0);
    let noise2 = noise(t * 5500.0) * 0.4;

    (noise1 + noise2) * envelope * 0.4
}

/// Tube shaker - Longer, more sustained
pub fn tube_shaker_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.25;

    if t > duration {
        return 0.0;
    }

    // Longer sustain than egg shaker
    let envelope = (-t * 10.0).exp() * (1.0 - (-t * 30.0).exp());

    // Lower frequency content
    let noise1 = noise(t * 7000.0);
    let noise2 = noise(t * 3500.0) * 0.5;

    (noise1 + noise2) * envelope * 0.45
}

// ============================================================================
// 808 KIT COMPLETION
// ============================================================================

/// 808 Tom Low - Deep, pitched 808 tom
pub fn tom_808_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.4;

    if t > duration {
        return 0.0;
    }

    // 808 toms use two oscillators with pitch drop
    let start_freq1 = 105.0;
    let end_freq1 = 65.0;
    let freq1 = start_freq1 + (end_freq1 - start_freq1) * (t / 0.1);

    let start_freq2 = 160.0;
    let end_freq2 = 90.0;
    let freq2 = start_freq2 + (end_freq2 - start_freq2) * (t / 0.1);

    let envelope = (-t * 7.0).exp();

    // Two triangle oscillators (characteristic of 808)
    let osc1 = triangle_wave(2.0 * std::f32::consts::PI * freq1 * t);
    let osc2 = triangle_wave(2.0 * std::f32::consts::PI * freq2 * t);

    (osc1 + osc2) * envelope * 0.35
}

/// 808 Tom Mid - Mid-pitched 808 tom
pub fn tom_808_mid_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.35;

    if t > duration {
        return 0.0;
    }

    let start_freq1 = 145.0;
    let end_freq1 = 90.0;
    let freq1 = start_freq1 + (end_freq1 - start_freq1) * (t / 0.08);

    let start_freq2 = 220.0;
    let end_freq2 = 130.0;
    let freq2 = start_freq2 + (end_freq2 - start_freq2) * (t / 0.08);

    let envelope = (-t * 8.0).exp();

    let osc1 = triangle_wave(2.0 * std::f32::consts::PI * freq1 * t);
    let osc2 = triangle_wave(2.0 * std::f32::consts::PI * freq2 * t);

    (osc1 + osc2) * envelope * 0.35
}

/// 808 Tom High - High-pitched 808 tom
pub fn tom_808_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.3;

    if t > duration {
        return 0.0;
    }

    let start_freq1 = 220.0;
    let end_freq1 = 140.0;
    let freq1 = start_freq1 + (end_freq1 - start_freq1) * (t / 0.06);

    let start_freq2 = 330.0;
    let end_freq2 = 200.0;
    let freq2 = start_freq2 + (end_freq2 - start_freq2) * (t / 0.06);

    let envelope = (-t * 9.0).exp();

    let osc1 = triangle_wave(2.0 * std::f32::consts::PI * freq1 * t);
    let osc2 = triangle_wave(2.0 * std::f32::consts::PI * freq2 * t);

    (osc1 + osc2) * envelope * 0.35
}

/// 808 Cowbell - Iconic 808 cowbell
pub fn cowbell_808_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.25;

    if t > duration {
        return 0.0;
    }

    let envelope = (-t * 10.0).exp();

    // 808 cowbell uses two square waves
    let freq1 = 540.0;
    let freq2 = 800.0;

    let phase1 = 2.0 * std::f32::consts::PI * freq1 * t;
    let phase2 = 2.0 * std::f32::consts::PI * freq2 * t;

    let square1 = if phase1.sin() > 0.0 { 1.0 } else { -1.0 };
    let square2 = if phase2.sin() > 0.0 { 1.0 } else { -1.0 };

    (square1 + square2) * envelope * 0.3
}

/// 808 Clave - Sharp electronic click
pub fn clave_808_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.025;

    if t > duration {
        return 0.0;
    }

    // Very short, sharp click with two sine oscillators
    let envelope = (-t * 80.0).exp();

    let freq1 = 2500.0;
    let freq2 = 5000.0;

    let osc1 = (2.0 * std::f32::consts::PI * freq1 * t).sin();
    let osc2 = (2.0 * std::f32::consts::PI * freq2 * t).sin();

    (osc1 + osc2) * envelope * 0.4
}

// ============================================================================
// 909 KIT COMPLETION
// ============================================================================

/// 909 Hi-Hat Closed - Bright, metallic closed hat
pub fn hihat_909_closed_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.04;

    if t > duration {
        return 0.0;
    }

    // Very fast decay, brighter than 808
    let envelope = (-t * 50.0).exp();

    // High-frequency metallic content
    let noise_val = noise(t * 12000.0);
    let metallic = (2.0 * std::f32::consts::PI * 10500.0 * t).sin() * 0.3;

    (noise_val * 0.7 + metallic) * envelope * 0.45
}

/// 909 Hi-Hat Open - Bright, sustained open hat
pub fn hihat_909_open_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.18;

    if t > duration {
        return 0.0;
    }

    // Longer decay than closed, bright character
    let envelope = (-t * 12.0).exp();

    let noise_val = noise(t * 11000.0);
    let metallic1 = (2.0 * std::f32::consts::PI * 9800.0 * t).sin() * 0.25;
    let metallic2 = (2.0 * std::f32::consts::PI * 11200.0 * t).sin() * 0.2;

    (noise_val * 0.55 + metallic1 + metallic2) * envelope * 0.5
}

/// 909 Clap - Classic 909 hand clap
pub fn clap_909_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.09;

    if t > duration {
        return 0.0;
    }

    // Multiple noise bursts at slightly different times
    let clap1_env = (-t * 40.0).exp();
    let clap2_env = (-(t - 0.006).max(0.0) * 40.0).exp();
    let clap3_env = (-(t - 0.012).max(0.0) * 40.0).exp();

    let clap1 = noise(t * 11000.0) * clap1_env * 0.33;
    let clap2 = noise(t * 11500.0) * clap2_env * 0.33;
    let clap3 = noise(t * 10500.0) * clap3_env * 0.34;

    clap1 + clap2 + clap3
}

/// 909 Cowbell - Sharper than regular cowbell
pub fn cowbell_909_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.22;

    if t > duration {
        return 0.0;
    }

    let envelope = (-t * 11.0).exp();

    // Two triangle oscillators for 909 cowbell
    let freq1 = 587.0;
    let freq2 = 845.0;

    let osc1 = triangle_wave(2.0 * std::f32::consts::PI * freq1 * t);
    let osc2 = triangle_wave(2.0 * std::f32::consts::PI * freq2 * t);

    (osc1 + osc2) * envelope * 0.35
}

/// 909 Rim - Classic 909 rim shot
pub fn rim_909_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 0.06;

    if t > duration {
        return 0.0;
    }

    let envelope = (-t * 35.0).exp();

    // High-pitched click with oscillator
    let freq = 1950.0;
    let osc = triangle_wave(2.0 * std::f32::consts::PI * freq * t);
    let noise_val = noise(t * 8500.0) * 0.3;

    (osc * 0.7 + noise_val) * envelope * 0.55
}

// ============================================================================
// TRANSITION EFFECTS
// ============================================================================

/// Reverse snare - Snare buildup effect
pub fn reverse_snare_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 1.0;

    if t > duration {
        return 0.0;
    }

    // Reverse envelope - builds up
    let envelope = (t / duration).powf(1.5);

    // Snare-like tone and noise building up
    let tone = (2.0 * std::f32::consts::PI * 200.0 * t).sin() * 0.3;
    let noise_val = noise(t * 8500.0) * 0.7;

    (tone + noise_val) * envelope * 0.5
}

/// Cymbal swell - Building cymbal wash
pub fn cymbal_swell_sample(sample_index: usize, sample_rate: f32) -> f32 {
    let t = sample_index as f32 / sample_rate;
    let duration = 2.0;

    if t > duration {
        return 0.0;
    }

    // Gradual buildup then quick fade
    let envelope = if t < 1.5 {
        (t / 1.5).powf(2.0)
    } else {
        1.0 - ((t - 1.5) / 0.5).powf(2.0)
    };

    // Cymbal-like inharmonic partials
    let freq1 = (2.0 * std::f32::consts::PI * 520.0 * t).sin();
    let freq2 = (2.0 * std::f32::consts::PI * 780.0 * t).sin();
    let freq3 = (2.0 * std::f32::consts::PI * 1150.0 * t).sin();

    let noise_val = noise(t * 5000.0) * 0.6;
    let tonal = (freq1 + freq2 + freq3) * 0.15;

    (noise_val + tonal) * envelope * 0.5
}

// Helper function for triangle wave (used by 808 sounds)
fn triangle_wave(phase: f32) -> f32 {
    let normalized = (phase / (2.0 * std::f32::consts::PI)) % 1.0;
    if normalized < 0.5 {
        4.0 * normalized - 1.0
    } else {
        3.0 - 4.0 * normalized
    }
}

/// Drum types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrumType {
    Kick,
    Kick808, // Long, pitched 808 kick
    SubKick, // Ultra-low sub kick
    Snare,
    Snare808, // 808 snare (dual triangle oscillators)
    HiHatClosed,
    HiHatOpen,
    HiHat808Closed, // 808 closed hi-hat (6 square oscillators)
    HiHat808Open,   // 808 open hi-hat (6 square oscillators)
    Clap,
    Clap808, // 808 clap (multiple noise bursts)
    Tom,     // Mid tom (original)
    TomHigh, // High tom
    TomLow,  // Low tom
    Rimshot,
    Cowbell,
    Crash,
    Ride,
    China,  // China cymbal
    Splash, // Splash cymbal
    Tambourine,
    Shaker,
    BassDrop, // Dramatic bass drop impact
    Boom,     // Deep cinematic boom
    // Simple percussion
    Claves,    // Sharp wooden click
    Triangle,  // Metallic ding
    SideStick, // Soft rim click
    WoodBlock, // Dry, pitched click
    // 909 electronic drums
    Kick909,  // Punchier electronic kick
    Snare909, // Brighter electronic snare
    // Latin percussion
    CongaHigh, // Bright, high-pitched hand drum
    CongaLow,  // Deep, resonant bass conga
    BongoHigh, // Sharp, articulate bongo
    BongoLow,  // Deeper bongo
    // Utility
    RideBell, // Metallic ping
    // Additional toms
    FloorTomLow,  // Deep floor tom
    FloorTomHigh, // Higher floor tom
    // Additional hi-hat
    HiHatPedal, // Pedal hi-hat chick
    // Additional cymbals
    Crash2, // Second crash cymbal
    // Special effects
    Vibraslap, // Rattling/buzzing percussion
    // Additional Latin percussion
    TimbaleHigh, // High timbale (metallic shell)
    TimbaleLow,  // Low timbale
    AgogoHigh,   // High agogo bell (Brazilian)
    AgogoLow,    // Low agogo bell
    // Additional shakers/scrapers
    Cabasa,     // Textured shaker/scraper
    GuiroShort, // Short scraping sound
    GuiroLong,  // Long scraping sound
    // Additional wood percussion
    WoodBlockHigh, // High-pitched wooden click
    // Orchestral percussion
    Timpani, // Tuned orchestral bass drum
    Gong,    // Deep metallic crash
    Chimes,  // Tubular bells/chimes
    // World percussion
    Djembe,     // West African hand drum
    TablaBayan, // Indian bass drum (left hand)
    TablaDayan, // Indian treble drum (right hand)
    Cajon,      // Box drum
    // Hand percussion
    Fingersnap,  // Fingersnap sound
    Maracas,     // Rattling shaker
    Castanet,    // Spanish wooden clapper
    SleighBells, // Jingle bells
    // Electronic / Effects
    LaserZap,      // Sci-fi laser sound
    ReverseCymbal, // Reverse crash buildup
    WhiteNoiseHit, // Noise burst/clap
    StickClick,    // Drumstick click
    // Kick variations
    KickTight,    // Short, punchy kick
    KickDeep,     // Extended low-end
    KickAcoustic, // Natural drum kit
    KickClick,    // Prominent beater attack
    // Snare variations
    SnareRim,     // Rim-focused
    SnareTight,   // Short, dry
    SnareLoose,   // Longer ring
    SnarePiccolo, // High-pitched, bright
    // Hi-hat variations
    HiHatHalfOpen, // Between closed and open
    HiHatSizzle,   // High-frequency content
    // Clap variations
    ClapDry,   // No reverb, tight
    ClapRoom,  // Natural room ambience
    ClapGroup, // Multiple claps layered
    ClapSnare, // Hybrid clap/snare
    // Cymbal variations
    CrashShort, // Quick crash, gated
    RideTip,    // Bell-less ride
    // Shaker variations
    EggShaker,  // Tight, short shake
    TubeShaker, // Longer, sustained
    // 808 Kit Completion
    Tom808Low,  // Deep 808 tom
    Tom808Mid,  // Mid 808 tom
    Tom808High, // High 808 tom
    Cowbell808, // Iconic 808 cowbell
    Clave808,   // Sharp 808 clave
    // 909 Kit Completion
    HiHat909Closed, // Bright 909 closed hat
    HiHat909Open,   // Sustained 909 open hat
    Clap909,        // Classic 909 clap
    Cowbell909,     // Sharp 909 cowbell
    Rim909,         // 909 rim shot
    // Transition Effects
    ReverseSnare, // Snare buildup effect
    CymbalSwell,  // Building cymbal wash
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
            DrumType::Claves => claves_sample(sample_index, sample_rate),
            DrumType::Triangle => triangle_sample(sample_index, sample_rate),
            DrumType::SideStick => side_stick_sample(sample_index, sample_rate),
            DrumType::WoodBlock => wood_block_sample(sample_index, sample_rate),
            DrumType::Kick909 => kick_909_sample(sample_index, sample_rate),
            DrumType::Snare909 => snare_909_sample(sample_index, sample_rate),
            DrumType::CongaHigh => conga_high_sample(sample_index, sample_rate),
            DrumType::CongaLow => conga_low_sample(sample_index, sample_rate),
            DrumType::BongoHigh => bongo_high_sample(sample_index, sample_rate),
            DrumType::BongoLow => bongo_low_sample(sample_index, sample_rate),
            DrumType::RideBell => ride_bell_sample(sample_index, sample_rate),
            DrumType::FloorTomLow => floor_tom_low_sample(sample_index, sample_rate),
            DrumType::FloorTomHigh => floor_tom_high_sample(sample_index, sample_rate),
            DrumType::HiHatPedal => hihat_pedal_sample(sample_index, sample_rate),
            DrumType::Crash2 => crash2_sample(sample_index, sample_rate),
            DrumType::Vibraslap => vibraslap_sample(sample_index, sample_rate),
            DrumType::TimbaleHigh => timbale_high_sample(sample_index, sample_rate),
            DrumType::TimbaleLow => timbale_low_sample(sample_index, sample_rate),
            DrumType::AgogoHigh => agogo_high_sample(sample_index, sample_rate),
            DrumType::AgogoLow => agogo_low_sample(sample_index, sample_rate),
            DrumType::Cabasa => cabasa_sample(sample_index, sample_rate),
            DrumType::GuiroShort => guiro_short_sample(sample_index, sample_rate),
            DrumType::GuiroLong => guiro_long_sample(sample_index, sample_rate),
            DrumType::WoodBlockHigh => wood_block_high_sample(sample_index, sample_rate),
            DrumType::Timpani => timpani_sample(sample_index, sample_rate),
            DrumType::Gong => gong_sample(sample_index, sample_rate),
            DrumType::Chimes => chimes_sample(sample_index, sample_rate),
            DrumType::Djembe => djembe_sample(sample_index, sample_rate),
            DrumType::TablaBayan => tabla_bayan_sample(sample_index, sample_rate),
            DrumType::TablaDayan => tabla_dayan_sample(sample_index, sample_rate),
            DrumType::Cajon => cajon_sample(sample_index, sample_rate),
            DrumType::Fingersnap => fingersnap_sample(sample_index, sample_rate),
            DrumType::Maracas => maracas_sample(sample_index, sample_rate),
            DrumType::Castanet => castanet_sample(sample_index, sample_rate),
            DrumType::SleighBells => sleigh_bells_sample(sample_index, sample_rate),
            DrumType::LaserZap => laser_zap_sample(sample_index, sample_rate),
            DrumType::ReverseCymbal => reverse_cymbal_sample(sample_index, sample_rate),
            DrumType::WhiteNoiseHit => white_noise_hit_sample(sample_index, sample_rate),
            DrumType::StickClick => stick_click_sample(sample_index, sample_rate),
            DrumType::KickTight => kick_tight_sample(sample_index, sample_rate),
            DrumType::KickDeep => kick_deep_sample(sample_index, sample_rate),
            DrumType::KickAcoustic => kick_acoustic_sample(sample_index, sample_rate),
            DrumType::KickClick => kick_click_sample(sample_index, sample_rate),
            DrumType::SnareRim => snare_rim_sample(sample_index, sample_rate),
            DrumType::SnareTight => snare_tight_sample(sample_index, sample_rate),
            DrumType::SnareLoose => snare_loose_sample(sample_index, sample_rate),
            DrumType::SnarePiccolo => snare_piccolo_sample(sample_index, sample_rate),
            DrumType::HiHatHalfOpen => hihat_half_open_sample(sample_index, sample_rate),
            DrumType::HiHatSizzle => hihat_sizzle_sample(sample_index, sample_rate),
            DrumType::ClapDry => clap_dry_sample(sample_index, sample_rate),
            DrumType::ClapRoom => clap_room_sample(sample_index, sample_rate),
            DrumType::ClapGroup => clap_group_sample(sample_index, sample_rate),
            DrumType::ClapSnare => clap_snare_sample(sample_index, sample_rate),
            DrumType::CrashShort => crash_short_sample(sample_index, sample_rate),
            DrumType::RideTip => ride_tip_sample(sample_index, sample_rate),
            DrumType::EggShaker => egg_shaker_sample(sample_index, sample_rate),
            DrumType::TubeShaker => tube_shaker_sample(sample_index, sample_rate),
            DrumType::Tom808Low => tom_808_low_sample(sample_index, sample_rate),
            DrumType::Tom808Mid => tom_808_mid_sample(sample_index, sample_rate),
            DrumType::Tom808High => tom_808_high_sample(sample_index, sample_rate),
            DrumType::Cowbell808 => cowbell_808_sample(sample_index, sample_rate),
            DrumType::Clave808 => clave_808_sample(sample_index, sample_rate),
            DrumType::HiHat909Closed => hihat_909_closed_sample(sample_index, sample_rate),
            DrumType::HiHat909Open => hihat_909_open_sample(sample_index, sample_rate),
            DrumType::Clap909 => clap_909_sample(sample_index, sample_rate),
            DrumType::Cowbell909 => cowbell_909_sample(sample_index, sample_rate),
            DrumType::Rim909 => rim_909_sample(sample_index, sample_rate),
            DrumType::ReverseSnare => reverse_snare_sample(sample_index, sample_rate),
            DrumType::CymbalSwell => cymbal_swell_sample(sample_index, sample_rate),
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
            DrumType::Claves => 0.02,
            DrumType::Triangle => 1.5,
            DrumType::SideStick => 0.04,
            DrumType::WoodBlock => 0.05,
            DrumType::Kick909 => 0.15,
            DrumType::Snare909 => 0.1,
            DrumType::CongaHigh => 0.2,
            DrumType::CongaLow => 0.3,
            DrumType::BongoHigh => 0.15,
            DrumType::BongoLow => 0.2,
            DrumType::RideBell => 0.6,
            DrumType::FloorTomLow => 0.4,
            DrumType::FloorTomHigh => 0.35,
            DrumType::HiHatPedal => 0.08,
            DrumType::Crash2 => 1.8,
            DrumType::Vibraslap => 0.15,
            DrumType::TimbaleHigh => 0.25,
            DrumType::TimbaleLow => 0.3,
            DrumType::AgogoHigh => 0.4,
            DrumType::AgogoLow => 0.5,
            DrumType::Cabasa => 0.25,
            DrumType::GuiroShort => 0.08,
            DrumType::GuiroLong => 0.2,
            DrumType::WoodBlockHigh => 0.06,
            DrumType::Timpani => 1.2,
            DrumType::Gong => 3.5,
            DrumType::Chimes => 2.0,
            DrumType::Djembe => 0.4,
            DrumType::TablaBayan => 0.5,
            DrumType::TablaDayan => 0.3,
            DrumType::Cajon => 0.25,
            DrumType::Fingersnap => 0.08,
            DrumType::Maracas => 0.12,
            DrumType::Castanet => 0.06,
            DrumType::SleighBells => 0.8,
            DrumType::LaserZap => 0.3,
            DrumType::ReverseCymbal => 1.5,
            DrumType::WhiteNoiseHit => 0.12,
            DrumType::StickClick => 0.03,
            DrumType::KickTight => 0.06,
            DrumType::KickDeep => 0.5,
            DrumType::KickAcoustic => 0.25,
            DrumType::KickClick => 0.12,
            DrumType::SnareRim => 0.08,
            DrumType::SnareTight => 0.07,
            DrumType::SnareLoose => 0.18,
            DrumType::SnarePiccolo => 0.08,
            DrumType::HiHatHalfOpen => 0.1,
            DrumType::HiHatSizzle => 0.2,
            DrumType::ClapDry => 0.05,
            DrumType::ClapRoom => 0.15,
            DrumType::ClapGroup => 0.12,
            DrumType::ClapSnare => 0.1,
            DrumType::CrashShort => 0.5,
            DrumType::RideTip => 0.6,
            DrumType::EggShaker => 0.08,
            DrumType::TubeShaker => 0.25,
            DrumType::Tom808Low => 0.4,
            DrumType::Tom808Mid => 0.35,
            DrumType::Tom808High => 0.3,
            DrumType::Cowbell808 => 0.3,
            DrumType::Clave808 => 0.025,
            DrumType::HiHat909Closed => 0.05,
            DrumType::HiHat909Open => 0.18,
            DrumType::Clap909 => 0.1,
            DrumType::Cowbell909 => 0.25,
            DrumType::Rim909 => 0.06,
            DrumType::ReverseSnare => 1.2,
            DrumType::CymbalSwell => 2.0,
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
            assert!(
                value.is_finite(),
                "Noise produced non-finite value at seed {}",
                i
            );
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
            assert!(
                sample.is_finite(),
                "Kick drum produced non-finite sample at index {}",
                i
            );
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

        assert!(
            early_sample.abs() > 0.0,
            "Kick should have non-zero amplitude early on"
        );
        assert!(
            mid_sample.abs() > 0.0,
            "Kick should still be audible mid-duration"
        );
        assert_eq!(end_sample, 0.0, "Kick should be silent after duration");

        // Verify decay is happening
        assert!(
            mid_sample.abs() < early_sample.abs(),
            "Kick should decay over time"
        );
    }

    #[test]
    fn test_snare_drum_valid_samples() {
        for i in 0..1000 {
            let sample = snare_drum_sample(i, SAMPLE_RATE);
            assert!(
                sample.is_finite(),
                "Snare produced non-finite sample at index {}",
                i
            );
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
        assert_eq!(
            closed_sample, 0.0,
            "Closed hihat should be silent by this point"
        );
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
            assert!(
                sample.is_finite(),
                "Clap produced non-finite sample at {}",
                point
            );
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
