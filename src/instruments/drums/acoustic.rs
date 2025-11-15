//! Acoustic drum kit sounds
//!
//! Natural drum sounds including kicks, snares, toms, and related acoustic percussion.

use super::noise;

/// Generate a kick drum sample (low frequency sine burst with pitch drop)
pub(super) fn kick_drum_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Tight kick - Short, punchy kick for electronic music
pub(super) fn kick_tight_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn kick_deep_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn kick_acoustic_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn kick_click_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a snare drum sample (noise burst with decay)
pub(super) fn snare_drum_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Rim snare - Rim-focused, less body
pub(super) fn snare_rim_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn snare_tight_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn snare_loose_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn snare_piccolo_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a tom drum sample (mid-low frequency with pitch drop)
pub(super) fn tom_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a high tom sample (higher pitch)
pub(super) fn tom_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn tom_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Floor tom (low) - Deep floor tom sound
pub(super) fn floor_tom_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn floor_tom_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a rim shot sample (sharp transient with high frequency click)
pub(super) fn rimshot_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a clap sample (multiple noise bursts)
pub(super) fn clap_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Dry clap - No reverb, tight
pub(super) fn clap_dry_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn clap_room_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn clap_group_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn clap_snare_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Stick click - Drumsticks clicked together
pub(super) fn stick_click_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a side stick sample (soft rim click)
pub(super) fn side_stick_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Timpani - Tuned orchestral bass drum
pub(super) fn timpani_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn gong_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn chimes_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Reverse snare - Snare buildup effect
pub(super) fn reverse_snare_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a pitched tom sample (tunable tom at specific frequency)
pub(super) fn pitched_tom_sample(sample_index: usize, sample_rate: f32, pitch_hz: f32) -> f32 {
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
