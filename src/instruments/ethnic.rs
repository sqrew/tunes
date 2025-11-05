//! Ethnic and world instrument presets

use super::Instrument;
use crate::synthesis::effects::{Delay, Distortion, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Sitar - Indian string instrument with sympathetic resonance and twang
    pub fn sitar() -> Self {
        let shimmer = LFO::new(Waveform::Sine, 0.4, 0.6); // Resonant shimmer
        Self {
            name: "Sitar".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.001, 0.3, 0.4, 0.8), // Plucked with long resonance
            filter: Filter::low_pass(4500.0, 0.6),         // Bright, resonant, metallic
            modulation: vec![ModRoute::new(shimmer, ModTarget::FilterCutoff, 0.25)],
            delay: Some(Delay::new(0.15, 0.35, 0.25)), // Sympathetic string effect
            reverb: Some(Reverb::new(0.5, 0.55, 0.4)),
            distortion: Some(Distortion::new(1.4, 0.2)), // Adds twang
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Pan flute - breathy, hollow, meditative wind sound
    pub fn pan_flute() -> Self {
        let breath = LFO::new(Waveform::Sine, 3.5, 0.15); // Subtle breath variation
        Self {
            name: "Pan Flute".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.1, 0.25, 0.65, 0.6), // Gentle, breathy attack
            filter: Filter::low_pass(3000.0, 0.15),        // Soft, hollow
            modulation: vec![ModRoute::new(breath, ModTarget::Volume, 0.1)],
            delay: None,
            reverb: Some(Reverb::new(0.6, 0.6, 0.45)), // Open air, meditative space
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Didgeridoo - deep, droning Aboriginal instrument with overtones
    pub fn didgeridoo() -> Self {
        let drone_lfo = LFO::new(Waveform::Sine, 0.15, 0.4); // Very slow drone movement
        Self {
            name: "Didgeridoo".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.15, 0.3, 0.95, 0.8), // Slow, sustained drone
            filter: Filter::low_pass(400.0, 0.5),          // Very low, overtone-rich
            modulation: vec![ModRoute::new(drone_lfo, ModTarget::FilterCutoff, 0.15)],
            delay: None,
            reverb: Some(Reverb::new(0.5, 0.55, 0.4)),
            distortion: Some(Distortion::new(1.3, 0.25)), // Adds harmonics
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Shamisen - Japanese plucked string with sharp, percussive attack
    pub fn shamisen() -> Self {
        Self {
            name: "Shamisen".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.2, 0.3, 0.5), // Sharp pluck, quick decay
            filter: Filter::low_pass(3500.0, 0.4),         // Percussive, woody
            modulation: Vec::new(),
            delay: Some(Delay::new(0.2, 0.25, 0.15)),
            reverb: Some(Reverb::new(0.3, 0.4, 0.25)),
            distortion: Some(Distortion::new(1.5, 0.25)), // Percussive character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Bagpipes - Scottish droning instrument with persistent pitch and harmonics
    pub fn bagpipes() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.0, 0.2); // Slight bagpipe waver
        Self {
            name: "Bagpipes".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.08, 0.1, 0.95, 0.4), // Droning sustain
            filter: Filter::low_pass(2800.0, 0.65),        // Nasal, droning, resonant
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.1)],
            delay: None,
            reverb: Some(Reverb::new(0.4, 0.5, 0.35)),
            distortion: Some(Distortion::new(1.6, 0.3)), // Adds harmonics and drone character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Kalimba - African thumb piano with bright, melodic tone
    pub fn kalimba() -> Self {
        Self {
            name: "Kalimba".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.4, 0.2, 0.8), // Thumb pluck with resonance
            filter: Filter::low_pass(5500.0, 0.3),         // Bright, bell-like
            modulation: Vec::new(),
            delay: Some(Delay::new(0.2, 0.3, 0.25)),
            reverb: Some(Reverb::new(0.5, 0.55, 0.4)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Koto - Japanese 13-string zither with delicate plucked sound
    pub fn koto() -> Self {
        let shimmer = LFO::new(Waveform::Sine, 0.35, 0.3);
        Self {
            name: "Koto".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.002, 0.35, 0.25, 0.7), // Plucked with resonance
            filter: Filter::low_pass(4800.0, 0.35),          // Delicate, resonant
            modulation: vec![ModRoute::new(shimmer, ModTarget::FilterCutoff, 0.12)],
            delay: Some(Delay::new(0.25, 0.28, 0.22)),
            reverb: Some(Reverb::new(0.45, 0.52, 0.38)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Banjo - bright, twangy American folk instrument
    pub fn banjo() -> Self {
        Self {
            name: "Banjo".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.15, 0.2, 0.3), // Bright pluck, quick decay
            filter: Filter::low_pass(5000.0, 0.5),          // Bright, twangy
            modulation: Vec::new(),
            delay: Some(Delay::new(0.15, 0.25, 0.18)),
            reverb: Some(Reverb::new(0.25, 0.35, 0.22)),
            distortion: Some(Distortion::new(1.3, 0.2)), // Adds twang
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Tabla - Indian hand drum with characteristic tonal resonance
    pub fn tabla() -> Self {
        Self {
            name: "Tabla".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.12, 0.15, 0.25), // Sharp drum hit
            filter: Filter::low_pass(2500.0, 0.6),            // Tonal drum resonance
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.3, 0.4, 0.25)),
            distortion: Some(Distortion::new(1.4, 0.25)),
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Erhu - Chinese two-string fiddle with expressive, vocal quality
    pub fn erhu() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.5, 0.4); // Expressive vibrato
        Self {
            name: "Erhu".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.12, 0.28, 0.85, 0.6), // Bowed, expressive
            filter: Filter::low_pass(3800.0, 0.45),         // Vocal-like, nasal
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.2)],
            delay: None,
            reverb: Some(Reverb::new(0.4, 0.5, 0.35)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Dulcimer - hammered string instrument with shimmering, bright tone
    pub fn dulcimer() -> Self {
        let shimmer = LFO::new(Waveform::Sine, 0.4, 0.35);
        Self {
            name: "Dulcimer".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.002, 0.45, 0.3, 0.8), // Hammered strike
            filter: Filter::low_pass(5500.0, 0.32),         // Bright, shimmering
            modulation: vec![ModRoute::new(shimmer, ModTarget::FilterCutoff, 0.15)],
            delay: Some(Delay::new(0.22, 0.28, 0.22)),
            reverb: Some(Reverb::new(0.5, 0.55, 0.42)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }
}
