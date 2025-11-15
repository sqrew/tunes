//! Hand percussion and ethnic drums
//!
//! Hand drums and traditional percussion from various world music traditions.

use super::noise;

/// Generate a high conga sample (bright slap)
pub(super) fn conga_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn conga_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn bongo_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn bongo_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Djembe - West African hand drum (bass tone)
pub(super) fn djembe_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn tabla_bayan_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn tabla_dayan_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn cajon_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Timbale (high) - High metallic shell drum
pub(super) fn timbale_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn timbale_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
