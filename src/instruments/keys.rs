//! Keyboard and piano instrument presets

use super::Instrument;
use crate::synthesis::effects::{Delay, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Organ - no attack/decay, full sustain
    pub fn organ() -> Self {
        Self {
            name: "Organ".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::organ(),
            filter: Filter::none(),
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.3, 0.4, 0.2)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Electric piano
    pub fn electric_piano() -> Self {
        Self {
            name: "Electric Piano".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::piano(),
            filter: Filter::low_pass(4000.0, 0.2),
            modulation: Vec::new(),
            delay: Some(Delay::new(0.25, 0.2, 0.15)),
            reverb: Some(Reverb::new(0.4, 0.5, 0.25)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Acoustic piano - warm, expressive piano sound
    pub fn acoustic_piano() -> Self {
        Self {
            name: "Acoustic Piano".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.3, 0.6, 0.8), // Natural piano decay
            filter: Filter::low_pass(8000.0, 0.15),        // Full frequency range
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.25, 0.4, 0.2)), // Subtle room ambience
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Harpsichord - bright, percussive keyboard sound
    pub fn harpsichord() -> Self {
        Self {
            name: "Harpsichord".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.15, 0.3, 0.2), // Plucky with quick decay
            filter: Filter::low_pass(5000.0, 0.2),          // Bright but controlled
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.3, 0.4, 0.2)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Mallet - marimba/xylophone-like sound
    pub fn mallet() -> Self {
        Self {
            name: "Mallet".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.001, 0.4, 0.2, 0.6), // Sharp attack, quick decay
            filter: Filter::low_pass(6000.0, 0.15),
            modulation: Vec::new(),
            delay: Some(Delay::new(0.25, 0.2, 0.15)),
            reverb: Some(Reverb::new(0.4, 0.5, 0.3)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }
}
