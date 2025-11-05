//! Orchestral and acoustic instrument presets

use super::Instrument;
use crate::synthesis::effects::{Distortion, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Strings - violin/cello ensemble sound
    pub fn strings() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.5, 0.3); // Natural string vibrato
        Self {
            name: "Strings".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.15, 0.3, 0.85, 0.6), // Slow attack, sustained
            filter: Filter::low_pass(4000.0, 0.25),        // Warm, not too bright
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.15)],
            delay: None,
            reverb: Some(Reverb::new(0.6, 0.6, 0.45)), // Concert hall ambience
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Brass - trumpet/horn section sound
    pub fn brass() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 6.0, 0.25);
        Self {
            name: "Brass".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.05, 0.15, 0.8, 0.4), // Moderate attack for brass punch
            filter: Filter::low_pass(5000.0, 0.5),         // Bright and brassy
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.12)],
            delay: None,
            reverb: Some(Reverb::new(0.35, 0.5, 0.25)),
            distortion: Some(Distortion::new(1.3, 0.2)), // Slight grit for realism
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Flute - soft, breathy woodwind sound
    pub fn flute() -> Self {
        let breath_lfo = LFO::new(Waveform::Sine, 4.0, 0.2); // Subtle breath modulation
        Self {
            name: "Flute".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.08, 0.2, 0.7, 0.5), // Gentle attack
            filter: Filter::low_pass(6000.0, 0.2),        // Soft, airy
            modulation: vec![ModRoute::new(breath_lfo, ModTarget::Volume, 0.08)],
            delay: None,
            reverb: Some(Reverb::new(0.4, 0.5, 0.3)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Clarinet - warm, woody wind sound
    pub fn clarinet() -> Self {
        Self {
            name: "Clarinet".to_string(),
            waveform: Waveform::Square, // Square wave for hollow tone
            envelope: Envelope::new(0.06, 0.2, 0.75, 0.4), // Moderate attack
            filter: Filter::low_pass(3500.0, 0.3), // Warm, woody
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.35, 0.45, 0.25)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }
}
