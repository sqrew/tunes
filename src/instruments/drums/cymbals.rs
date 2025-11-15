//! Cymbal sounds
//!
//! All cymbal types including hi-hats, crashes, rides, and specialty cymbals.

use super::noise;

/// Generate a hi-hat sample (high frequency noise burst)
pub(super) fn hihat_sample(sample_index: usize, sample_rate: f32, closed: bool) -> f32 {
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

/// Half-open hi-hat - Between closed and open
pub(super) fn hihat_half_open_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn hihat_sizzle_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Pedal hi-hat - Foot pedal "chick" sound
pub(super) fn hihat_pedal_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a crash cymbal sample (bright noise with long decay)
pub(super) fn crash_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Crash cymbal 2 - Second crash variation
pub(super) fn crash2_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Crash short - Quick crash, gated
pub(super) fn crash_short_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a ride cymbal sample (sustained bright noise_val)
pub(super) fn ride_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a ride bell sample (metallic ping)
pub(super) fn ride_bell_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Ride tip - Bell-less ride, stick tip sound
pub(super) fn ride_tip_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a china cymbal sample (trashy, explosive sound)
pub(super) fn china_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn splash_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Reverse cymbal - Reversed crash buildup
pub(super) fn reverse_cymbal_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Cymbal swell - Building cymbal wash
pub(super) fn cymbal_swell_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
