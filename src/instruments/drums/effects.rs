//! Electronic and special effect sounds
//!
//! Special effect percussion including sub-bass impacts, electronic effects, and noise bursts.

use super::noise;

/// Generate a sub kick sample (ultra-low frequency kick)
pub(super) fn sub_kick_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn bass_drop_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn boom_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Laser zap - Sci-fi/electronic sound
pub(super) fn laser_zap_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// White noise hit - Burst/clap effect
pub(super) fn white_noise_hit_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
