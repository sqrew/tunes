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

    /// Oboe - nasal, reedy double-reed woodwind
    pub fn oboe() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.8, 0.3); // Moderate vibrato
        Self {
            name: "Oboe".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.04, 0.18, 0.8, 0.45), // Relatively quick attack
            filter: Filter::low_pass(2800.0, 0.55),         // Nasal, reedy range with resonance
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.15)],
            delay: None,
            reverb: Some(Reverb::new(0.35, 0.45, 0.25)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Bassoon - deep, woody double-reed woodwind
    pub fn bassoon() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 4.5, 0.2); // Subtle vibrato
        Self {
            name: "Bassoon".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.08, 0.25, 0.75, 0.5), // Gentle attack
            filter: Filter::low_pass(1200.0, 0.4),          // Deep, woody tone
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.12)],
            delay: None,
            reverb: Some(Reverb::new(0.4, 0.5, 0.3)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// French horn - warm, mellow brass instrument
    pub fn french_horn() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.0, 0.25);
        Self {
            name: "French Horn".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.1, 0.2, 0.82, 0.5), // Smooth, mellow attack
            filter: Filter::low_pass(3800.0, 0.38),       // Warm, not too bright
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.1)],
            delay: None,
            reverb: Some(Reverb::new(0.55, 0.6, 0.4)), // Concert hall
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Harp - plucked string instrument with bright, shimmering tone
    pub fn harp() -> Self {
        let shimmer = LFO::new(Waveform::Sine, 0.3, 0.15); // Slow shimmer
        Self {
            name: "Harp".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.002, 0.35, 0.15, 0.7), // Very fast attack, quick decay
            filter: Filter::low_pass(7000.0, 0.25),          // Bright, clear
            modulation: vec![ModRoute::new(shimmer, ModTarget::FilterCutoff, 0.08)],
            delay: None,
            reverb: Some(Reverb::new(0.5, 0.55, 0.35)), // Hall reverb for sparkle
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Alto saxophone - bright, expressive, mid-range jazz and classical
    pub fn alto_sax() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.5, 0.35); // Expressive vibrato
        Self {
            name: "Alto Sax".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.03, 0.15, 0.85, 0.4), // Quick attack, long sustain
            filter: Filter::low_pass(3200.0, 0.48),         // Bright, reedy
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.15)],
            delay: None,
            reverb: Some(Reverb::new(0.3, 0.4, 0.25)),
            distortion: Some(Distortion::new(1.2, 0.15)), // Slight breath/reed character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Tenor saxophone - warm, smooth, lower jazz tone
    pub fn tenor_sax() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.0, 0.3); // Smooth vibrato
        Self {
            name: "Tenor Sax".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.04, 0.18, 0.88, 0.45), // Smooth, sustained
            filter: Filter::low_pass(2400.0, 0.45),          // Warm, full
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.12)],
            delay: None,
            reverb: Some(Reverb::new(0.35, 0.45, 0.3)),
            distortion: Some(Distortion::new(1.15, 0.12)), // Subtle warmth
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Soprano saxophone - high, piercing, straight horn tone
    pub fn soprano_sax() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 6.0, 0.4); // Faster vibrato
        Self {
            name: "Soprano Sax".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.025, 0.12, 0.85, 0.35), // Quick, bright attack
            filter: Filter::low_pass(4200.0, 0.5),            // Piercing, clear
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.18)],
            delay: None,
            reverb: Some(Reverb::new(0.28, 0.38, 0.22)),
            distortion: Some(Distortion::new(1.25, 0.18)), // Edgy tone
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Baritone saxophone - very low, full-bodied, foundational
    pub fn baritone_sax() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 4.5, 0.25); // Slower, subtle vibrato
        Self {
            name: "Baritone Sax".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.05, 0.2, 0.9, 0.5), // Slower attack, very sustained
            filter: Filter::low_pass(1800.0, 0.42),       // Deep, full
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.1)],
            delay: None,
            reverb: Some(Reverb::new(0.4, 0.5, 0.35)),
            distortion: Some(Distortion::new(1.1, 0.1)), // Very subtle warmth
            volume: 1.0,
            pan: 0.0,
        }
    }
}
