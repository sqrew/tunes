//! Keyboard and piano instrument presets

use super::Instrument;
use crate::synthesis::effects::{Delay, Distortion, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
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

    /// Clavinet - funky, percussive electric keyboard
    pub fn clavinet() -> Self {
        Self {
            name: "Clavinet".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.08, 0.25, 0.1), // Instant attack, quick decay
            filter: Filter::low_pass(4500.0, 0.55),           // Bright, cutting with resonance
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.15, 0.25, 0.1)), // Minimal reverb
            distortion: Some(Distortion::new(1.3, 0.2)), // Slight funk grit
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Wurlitzer - warm electric piano with bell-like tone
    pub fn wurlitzer() -> Self {
        let tremolo = LFO::new(Waveform::Sine, 4.5, 0.15); // Subtle tremolo
        Self {
            name: "Wurlitzer".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.005, 0.4, 0.5, 0.8), // Warm, sustained
            filter: Filter::low_pass(3200.0, 0.22),        // Warm, bell-like
            modulation: vec![ModRoute::new(tremolo, ModTarget::Volume, 0.12)],
            delay: Some(Delay::new(0.3, 0.25, 0.18)),
            reverb: Some(Reverb::new(0.35, 0.45, 0.25)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Toy piano - small, metallic, quirky sound
    pub fn toy_piano() -> Self {
        Self {
            name: "Toy Piano".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.2, 0.05, 0.3), // Sharp attack, very short decay
            filter: Filter::low_pass(6000.0, 0.35),         // Bright, tinny, metallic
            modulation: Vec::new(),
            delay: Some(Delay::new(0.15, 0.2, 0.1)), // Short delay for character
            reverb: Some(Reverb::new(0.2, 0.3, 0.15)), // Minimal reverb
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Hammond organ - classic tonewheel organ with rotary speaker character
    pub fn hammond_organ() -> Self {
        let rotary = LFO::new(Waveform::Sine, 6.5, 0.3); // Rotary speaker effect
        Self {
            name: "Hammond Organ".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::organ(), // Instant on/off like real Hammond
            filter: Filter::low_pass(5000.0, 0.2), // Bright, full organ tone
            modulation: vec![ModRoute::new(rotary, ModTarget::Volume, 0.18)],
            delay: None,
            reverb: Some(Reverb::new(0.35, 0.45, 0.3)),
            distortion: Some(Distortion::new(1.2, 0.15)), // Slight tube warmth
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Church organ - massive pipe organ with cathedral presence
    pub fn church_organ() -> Self {
        Self {
            name: "Church Organ".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::organ(), // Instant on/off
            filter: Filter::low_pass(6000.0, 0.15), // Full, majestic
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.85, 0.8, 0.7)), // Massive cathedral space
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Reed organ - warm, breathy reed organ/harmonium
    pub fn reed_organ() -> Self {
        let breath = LFO::new(Waveform::Sine, 3.5, 0.12); // Subtle breath movement
        Self {
            name: "Reed Organ".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.05, 0.1, 0.95, 0.3), // Slightly slower attack for reeds
            filter: Filter::low_pass(3500.0, 0.25),        // Warm, breathy
            modulation: vec![ModRoute::new(breath, ModTarget::Volume, 0.08)],
            delay: None,
            reverb: Some(Reverb::new(0.4, 0.5, 0.35)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Accordion - squeezebox with characteristic bellows sound
    pub fn accordion() -> Self {
        let bellows = LFO::new(Waveform::Sine, 4.0, 0.2); // Bellows movement
        Self {
            name: "Accordion".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.03, 0.1, 0.9, 0.25), // Quick attack, sustained
            filter: Filter::low_pass(4000.0, 0.3),         // Reedy, accordion character
            modulation: vec![ModRoute::new(bellows, ModTarget::Volume, 0.12)],
            delay: None,
            reverb: Some(Reverb::new(0.3, 0.4, 0.25)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// CP-70 electric grand - Yamaha CP-70 (brighter and more percussive than Rhodes)
    pub fn cp70() -> Self {
        Self {
            name: "CP-70 Electric Grand".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.45, 0.5, 0.85), // Bright attack, piano-like decay
            filter: Filter::low_pass(5500.0, 0.25),          // Brighter than Rhodes
            modulation: Vec::new(),
            delay: Some(Delay::new(0.2, 0.18, 0.12)),
            reverb: Some(Reverb::new(0.32, 0.42, 0.25)),
            distortion: Some(Distortion::new(1.1, 0.08)), // Slight electric character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Pianet - Hohner Pianet (thin, plucky, vintage electric piano)
    pub fn pianet() -> Self {
        Self {
            name: "Pianet".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.25, 0.25, 0.4), // Quick, plucky attack
            filter: Filter::low_pass(4500.0, 0.4),           // Thin, bright character
            modulation: Vec::new(),
            delay: Some(Delay::new(0.18, 0.2, 0.15)),
            reverb: Some(Reverb::new(0.25, 0.35, 0.2)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Stage 73 - Fender Rhodes Stage 73 (classic suitcase Rhodes sound)
    pub fn stage_73() -> Self {
        let tremolo = LFO::new(Waveform::Sine, 5.0, 0.18); // Classic Rhodes tremolo
        Self {
            name: "Stage 73".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.003, 0.5, 0.5, 0.9), // Warm, sustained Rhodes attack
            filter: Filter::low_pass(3800.0, 0.22),        // Classic Rhodes warmth
            modulation: vec![ModRoute::new(tremolo, ModTarget::Volume, 0.15)],
            delay: Some(Delay::new(0.25, 0.22, 0.18)),
            reverb: Some(Reverb::new(0.38, 0.48, 0.28)),
            distortion: Some(Distortion::new(1.15, 0.1)), // Subtle tube warmth
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Mark I Rhodes - Vintage Rhodes Mark I (darker, warmer than Stage 73)
    pub fn mark_i_rhodes() -> Self {
        let tremolo = LFO::new(Waveform::Sine, 4.8, 0.2); // Slower vintage tremolo
        Self {
            name: "Mark I Rhodes".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.004, 0.6, 0.55, 1.0), // Slower, warmer attack
            filter: Filter::low_pass(3200.0, 0.2),          // Darker, vintage tone
            modulation: vec![ModRoute::new(tremolo, ModTarget::Volume, 0.18)],
            delay: Some(Delay::new(0.28, 0.25, 0.2)),
            reverb: Some(Reverb::new(0.42, 0.52, 0.32)),
            distortion: Some(Distortion::new(1.2, 0.12)), // Vintage tube character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Honky tonk piano - detuned upright piano (saloon/ragtime sound)
    pub fn honky_tonk_piano() -> Self {
        let detune = LFO::new(Waveform::Sine, 0.6, 0.35); // Out-of-tune character
        Self {
            name: "Honky Tonk Piano".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.35, 0.5, 0.75), // Percussive piano attack
            filter: Filter::low_pass(5500.0, 0.3),           // Bright, honky character
            modulation: vec![ModRoute::new(detune, ModTarget::FilterCutoff, 0.25)],
            delay: None,
            reverb: Some(Reverb::new(0.2, 0.3, 0.18)), // Dry, saloon ambience
            distortion: Some(Distortion::new(1.25, 0.15)), // Vintage character
            volume: 1.0,
            pan: 0.0,
        }
    }
}
