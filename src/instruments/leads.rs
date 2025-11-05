//! Lead instrument presets

use super::Instrument;
use crate::synthesis::effects::{Delay, Distortion, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Pluck lead - fast attack and decay
    pub fn pluck() -> Self {
        Self {
            name: "Pluck".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::pluck(),
            filter: Filter::low_pass(3000.0, 0.3),
            modulation: Vec::new(),
            delay: Some(Delay::new(0.25, 0.3, 0.3)),
            reverb: Some(Reverb::new(0.3, 0.4, 0.2)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Saw lead - bright, cutting lead sound
    pub fn saw_lead() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.0, 0.3);
        Self {
            name: "Saw Lead".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.01, 0.2, 0.7, 0.3),
            filter: Filter::low_pass(4000.0, 0.4),
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.2)],
            delay: Some(Delay::new(0.375, 0.3, 0.25)),
            reverb: Some(Reverb::new(0.4, 0.5, 0.2)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Square lead - hollow, retro game sound
    pub fn square_lead() -> Self {
        Self {
            name: "Square Lead".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.005, 0.1, 0.6, 0.2),
            filter: Filter::low_pass(2000.0, 0.3),
            modulation: Vec::new(),
            delay: Some(Delay::new(0.5, 0.4, 0.3)),
            reverb: Some(Reverb::new(0.5, 0.5, 0.3)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Bright lead - cutting, aggressive lead with harmonics
    pub fn bright_lead() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 6.0, 0.4);
        Self {
            name: "Bright Lead".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.005, 0.15, 0.8, 0.25),
            filter: Filter::low_pass(6000.0, 0.6), // Very bright
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.15)],
            delay: Some(Delay::new(0.25, 0.2, 0.15)),
            reverb: Some(Reverb::new(0.35, 0.4, 0.25)),
            distortion: Some(Distortion::new(1.5, 0.3)), // Slight grit
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Synth lead - warm, smooth lead with character
    pub fn synth_lead() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 4.5, 0.35);
        Self {
            name: "Synth Lead".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.01, 0.2, 0.7, 0.3),
            filter: Filter::low_pass(3500.0, 0.4),
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.2)],
            delay: Some(Delay::new(0.375, 0.25, 0.2)),
            reverb: Some(Reverb::new(0.45, 0.5, 0.3)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Arp lead - bright, fast attack for arpeggios
    pub fn arp_lead() -> Self {
        Self {
            name: "Arp Lead".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.05, 0.5, 0.1), // Very fast attack/decay
            filter: Filter::low_pass(4500.0, 0.5),
            modulation: Vec::new(),
            delay: Some(Delay::new(0.125, 0.25, 0.2)), // Eighth note delay
            reverb: Some(Reverb::new(0.3, 0.4, 0.2)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Laser lead - bright, futuristic lead with pitch sweep
    pub fn laser_lead() -> Self {
        let sweep_lfo = LFO::new(Waveform::Triangle, 8.0, 1.0); // Fast sweep
        Self {
            name: "Laser Lead".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.08, 0.6, 0.15), // Sharp attack
            filter: Filter::low_pass(8000.0, 0.8),            // Very bright, resonant
            modulation: vec![ModRoute::new(sweep_lfo, ModTarget::FilterCutoff, 0.5)],
            delay: Some(Delay::new(0.25, 0.2, 0.15)),
            reverb: Some(Reverb::new(0.25, 0.35, 0.2)),
            distortion: Some(Distortion::new(1.8, 0.35)), // Digital edge
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Detuned lead - thick, chorus-like lead from slight detuning
    pub fn detuned_lead() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 4.8, 0.25);
        Self {
            name: "Detuned Lead".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.01, 0.18, 0.75, 0.3),
            filter: Filter::low_pass(4500.0, 0.45),
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.18)],
            delay: Some(Delay::new(0.3, 0.25, 0.2)),
            reverb: Some(Reverb::new(0.4, 0.45, 0.28)),
            distortion: Some(Distortion::new(1.2, 0.15)), // Slight thickness
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Scream lead - aggressive, distorted lead
    pub fn scream_lead() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 6.5, 0.45);
        Self {
            name: "Scream Lead".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.005, 0.12, 0.85, 0.25),
            filter: Filter::low_pass(7000.0, 0.7), // Bright and resonant
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.3)],
            delay: Some(Delay::new(0.375, 0.28, 0.22)),
            reverb: Some(Reverb::new(0.35, 0.42, 0.25)),
            distortion: Some(Distortion::new(3.5, 0.6)), // Heavy distortion
            volume: 1.0,
            pan: 0.0,
        }
    }
}
