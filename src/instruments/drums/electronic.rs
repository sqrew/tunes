//! Electronic drum sounds (808 and 909 series)
//!
//! Classic analog drum machine sounds from the Roland TR-808 and TR-909.

use super::noise;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn triangle_wave(phase: f32) -> f32 {
    let normalized = (phase / (2.0 * std::f32::consts::PI)) % 1.0;
    if normalized < 0.5 {
        4.0 * normalized - 1.0
    } else {
        3.0 - 4.0 * normalized
    }
}

// ============================================================================
// 808 DRUM MACHINE
// ============================================================================

/// Generate an 808 kick sample (long, pitched sub-bass kick)
pub(super) fn kick_808_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn snare_808_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn hihat_808_sample(sample_index: usize, sample_rate: f32, closed: bool) -> f32 {
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
pub(super) fn clap_808_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// 808 Tom Low - Deep, pitched 808 tom
pub(super) fn tom_808_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn tom_808_mid_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn tom_808_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn cowbell_808_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn clave_808_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
// 909 DRUM MACHINE
// ============================================================================

/// Generate a 909 kick sample (harder, punchier than 808)
pub(super) fn kick_909_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn snare_909_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// 909 Hi-Hat Closed - Bright, metallic closed hat
pub(super) fn hihat_909_closed_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn hihat_909_open_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn clap_909_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn cowbell_909_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn rim_909_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
