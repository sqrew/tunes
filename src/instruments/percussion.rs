//! Metallic and mallet percussion instrument presets

use super::Instrument;
use crate::synthesis::effects::{Delay, Distortion, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Vibraphone - warm metallic mallet with subtle tremolo
    pub fn vibraphone() -> Self {
        let tremolo = LFO::new(Waveform::Sine, 5.8, 0.3); // Classic vibraphone tremolo
        Self {
            name: "Vibraphone".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.002, 0.8, 0.4, 1.2), // Soft mallet strike, long resonance
            filter: Filter::low_pass(5500.0, 0.2),         // Warm, bell-like
            modulation: vec![ModRoute::new(tremolo, ModTarget::Volume, 0.25)],
            delay: Some(Delay::new(0.3, 0.25, 0.2)),
            reverb: Some(Reverb::new(0.6, 0.65, 0.5)), // Spacious, resonant
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Glockenspiel - very bright, crystalline bells (music box-like)
    pub fn glockenspiel() -> Self {
        Self {
            name: "Glockenspiel".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.001, 0.5, 0.15, 0.8), // Sharp strike, quick decay
            filter: Filter::low_pass(8000.0, 0.15),         // Very bright, clear
            modulation: Vec::new(),
            delay: Some(Delay::new(0.25, 0.2, 0.15)),
            reverb: Some(Reverb::new(0.5, 0.55, 0.4)), // Crystalline space
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Tubular bells - deep, resonant church-like chimes
    pub fn tubular_bells() -> Self {
        let shimmer = LFO::new(Waveform::Sine, 0.25, 0.2); // Very slow shimmer
        Self {
            name: "Tubular Bells".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.002, 1.5, 0.3, 2.0), // Long, resonant decay
            filter: Filter::low_pass(4000.0, 0.25),        // Deep, clear tones
            modulation: vec![ModRoute::new(shimmer, ModTarget::FilterCutoff, 0.08)],
            delay: Some(Delay::new(0.4, 0.3, 0.25)),
            reverb: Some(Reverb::new(0.8, 0.75, 0.65)), // Cathedral-like
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Steel drums - Caribbean melodic percussion with warm shimmer
    pub fn steel_drums() -> Self {
        let shimmer = LFO::new(Waveform::Sine, 0.5, 0.4); // Metallic shimmer
        Self {
            name: "Steel Drums".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.6, 0.25, 0.9), // Strike with warm resonance
            filter: Filter::low_pass(6000.0, 0.35),         // Bright, metallic, warm
            modulation: vec![ModRoute::new(shimmer, ModTarget::FilterCutoff, 0.2)],
            delay: Some(Delay::new(0.2, 0.25, 0.18)),
            reverb: Some(Reverb::new(0.45, 0.5, 0.35)), // Tropical ambience
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Music box - delicate, mechanical, nostalgic sound
    pub fn music_box() -> Self {
        Self {
            name: "Music Box".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.001, 0.3, 0.1, 0.6), // Delicate pluck
            filter: Filter::low_pass(7000.0, 0.15),        // Very bright, crystalline
            modulation: Vec::new(),
            delay: Some(Delay::new(0.2, 0.25, 0.18)),
            reverb: Some(Reverb::new(0.35, 0.42, 0.3)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Celesta - ethereal, bell-like keyboard instrument
    pub fn celesta() -> Self {
        Self {
            name: "Celesta".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.001, 0.6, 0.2, 0.9), // Hammered bell
            filter: Filter::low_pass(7500.0, 0.18),        // Ethereal, bell-like
            modulation: Vec::new(),
            delay: Some(Delay::new(0.28, 0.25, 0.2)),
            reverb: Some(Reverb::new(0.55, 0.6, 0.45)), // Magical space
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Xylophone - bright, wooden percussion
    pub fn xylophone() -> Self {
        Self {
            name: "Xylophone".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.3, 0.1, 0.5), // Sharp, woody strike
            filter: Filter::low_pass(6500.0, 0.25),        // Bright, wooden
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.35, 0.42, 0.28)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Marimba - warm, deep wooden percussion
    pub fn marimba() -> Self {
        Self {
            name: "Marimba".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.002, 0.5, 0.2, 0.8), // Warm mallet strike
            filter: Filter::low_pass(4500.0, 0.22),        // Warm, woody
            modulation: Vec::new(),
            delay: Some(Delay::new(0.25, 0.22, 0.18)),
            reverb: Some(Reverb::new(0.45, 0.52, 0.38)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Bells - church bells, large resonant metallic sound
    pub fn bells() -> Self {
        let shimmer = LFO::new(Waveform::Sine, 0.18, 0.25); // Slow bell shimmer
        Self {
            name: "Bells".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.002, 1.8, 0.25, 2.5), // Long, resonant decay
            filter: Filter::low_pass(5500.0, 0.28),         // Metallic, resonant
            modulation: vec![ModRoute::new(shimmer, ModTarget::FilterCutoff, 0.1)],
            delay: Some(Delay::new(0.4, 0.35, 0.28)),
            reverb: Some(Reverb::new(0.75, 0.72, 0.62)), // Large space
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Cowbell - bright, metallic percussion hit
    pub fn cowbell() -> Self {
        Self {
            name: "Cowbell".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.1, 0.08, 0.2), // Short, metallic
            filter: Filter::low_pass(4000.0, 0.55),         // Bright, metallic
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.25, 0.32, 0.22)),
            distortion: Some(Distortion::new(1.5, 0.3)), // Adds metallic edge
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Timpani - orchestral kettle drum with deep, resonant tone
    pub fn timpani() -> Self {
        Self {
            name: "Timpani".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.002, 0.8, 0.15, 1.2), // Deep drum resonance
            filter: Filter::low_pass(800.0, 0.45),          // Very deep, tonal
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.6, 0.62, 0.48)), // Concert hall
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Taiko drum - Japanese ceremonial drum with powerful attack
    pub fn taiko_drum() -> Self {
        Self {
            name: "Taiko Drum".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.4, 0.1, 0.8), // Powerful strike
            filter: Filter::low_pass(1200.0, 0.5),         // Deep, resonant
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.5, 0.55, 0.42)),
            distortion: Some(Distortion::new(1.3, 0.25)), // Adds power
            volume: 1.0,
            pan: 0.0,
        }
    }
}
