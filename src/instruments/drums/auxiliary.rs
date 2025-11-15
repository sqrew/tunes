//! Auxiliary percussion and small instruments
//!
//! Small percussion instruments including shakers, bells, wood blocks, and other accessories.

use super::noise;

pub(super) fn cowbell_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a tambourine sample (jingles)
pub(super) fn tambourine_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn shaker_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

pub(super) fn egg_shaker_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn tube_shaker_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

pub(super) fn claves_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn triangle_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Generate a wood block sample (dry, pitched click)
pub(super) fn wood_block_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Wood block (high) - High-pitched wooden click
pub(super) fn wood_block_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Castanet - Spanish wooden clapper
pub(super) fn castanet_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Maracas - Rattling shaker (different character than generic shaker)
pub(super) fn maracas_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

pub(super) fn fingersnap_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Guiro (short) - Short scraping sound
pub(super) fn guiro_short_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn guiro_long_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Cabasa - Textured shaker/scraper hybrid
pub(super) fn cabasa_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

pub(super) fn vibraslap_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Sleigh bells - Jingle bells cluster
pub(super) fn sleigh_bells_sample(sample_index: usize, sample_rate: f32) -> f32 {
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

/// Agogo bell (high) - High Brazilian cowbell
pub(super) fn agogo_high_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
pub(super) fn agogo_low_sample(sample_index: usize, sample_rate: f32) -> f32 {
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
