//! String instrument presets

use super::Instrument;
use crate::synthesis::effects::Reverb;
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Violin - bright, expressive, high string instrument
    pub fn violin() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 6.0, 0.35); // Expressive vibrato
        Self {
            name: "Violin".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.08, 0.25, 0.88, 0.6), // Bowed attack, sustained
            filter: Filter::low_pass(5500.0, 0.28),        // Bright, clear
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.15)],
            delay: None,
            reverb: Some(Reverb::new(0.5, 0.55, 0.4)), // Concert hall
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Viola - mellow, mid-range string instrument
    pub fn viola() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.5, 0.3);
        Self {
            name: "Viola".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.1, 0.28, 0.86, 0.65), // Warm, mellow attack
            filter: Filter::low_pass(4200.0, 0.26),         // Warm, rich mid-range
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.14)],
            delay: None,
            reverb: Some(Reverb::new(0.5, 0.55, 0.4)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Cello - deep, rich, warm low string instrument
    pub fn cello() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.0, 0.28);
        Self {
            name: "Cello".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.12, 0.3, 0.88, 0.7), // Deep, expressive
            filter: Filter::low_pass(2800.0, 0.25),        // Deep, warm
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.12)],
            delay: None,
            reverb: Some(Reverb::new(0.55, 0.6, 0.45)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Double bass - very deep, foundational string instrument
    pub fn double_bass() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 4.5, 0.22);
        Self {
            name: "Double Bass".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.15, 0.35, 0.9, 0.8), // Very deep, sustained
            filter: Filter::low_pass(1800.0, 0.24),        // Very low, foundational
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.1)],
            delay: None,
            reverb: Some(Reverb::new(0.55, 0.6, 0.45)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Pizzicato strings - plucked string ensemble
    pub fn pizzicato_strings() -> Self {
        Self {
            name: "Pizzicato Strings".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.15, 0.25, 0.4), // Short pluck
            filter: Filter::low_pass(4500.0, 0.3),           // Bright, percussive
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.4, 0.5, 0.35)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Tremolo strings - rapid volume modulation string ensemble
    pub fn tremolo_strings() -> Self {
        let tremolo = LFO::new(Waveform::Sine, 12.0, 0.6); // Fast tremolo
        Self {
            name: "Tremolo Strings".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.08, 0.25, 0.9, 0.6), // Sustained
            filter: Filter::low_pass(4500.0, 0.26),        // Warm ensemble
            modulation: vec![ModRoute::new(tremolo, ModTarget::Volume, 0.5)],
            delay: None,
            reverb: Some(Reverb::new(0.55, 0.6, 0.45)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Slow strings - very slow attack string ensemble for pads
    pub fn slow_strings() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.0, 0.25);
        Self {
            name: "Slow Strings".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(1.5, 0.5, 0.95, 1.2), // Very slow swell
            filter: Filter::low_pass(4000.0, 0.22),       // Soft, warm
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.1)],
            delay: None,
            reverb: Some(Reverb::new(0.7, 0.7, 0.55)), // Lush, spacious
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }
}
