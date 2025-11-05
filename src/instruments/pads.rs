//! Pad instrument presets

use super::Instrument;
use crate::synthesis::effects::{Delay, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Warm pad - slow attack, long release
    pub fn warm_pad() -> Self {
        let filter_lfo = LFO::new(Waveform::Sine, 0.2, 0.5);
        Self {
            name: "Warm Pad".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::pad(),
            filter: Filter::low_pass(1500.0, 0.3),
            modulation: vec![ModRoute::new(filter_lfo, ModTarget::FilterCutoff, 0.2)],
            delay: None,
            reverb: Some(Reverb::new(0.8, 0.6, 0.5)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Ambient pad - very slow, atmospheric
    pub fn ambient_pad() -> Self {
        let filter_lfo = LFO::new(Waveform::Sine, 0.15, 0.7);
        let tremolo = LFO::new(Waveform::Sine, 0.3, 0.3);
        Self {
            name: "Ambient Pad".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(1.0, 0.5, 0.8, 1.5),
            filter: Filter::low_pass(2000.0, 0.2),
            modulation: vec![
                ModRoute::new(filter_lfo, ModTarget::FilterCutoff, 0.3),
                ModRoute::new(tremolo, ModTarget::Volume, 0.2),
            ],
            delay: Some(Delay::new(0.5, 0.5, 0.3)),
            reverb: Some(Reverb::new(0.9, 0.7, 0.6)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Vocal pad - choir/vocal-like synth pad
    pub fn vocal_pad() -> Self {
        let formant_sweep = LFO::new(Waveform::Sine, 0.25, 0.4); // Slow formant-like sweep
        Self {
            name: "Vocal Pad".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.5, 0.4, 0.9, 1.0), // Very slow attack
            filter: Filter::low_pass(2500.0, 0.6),       // Vocal formant range
            modulation: vec![ModRoute::new(formant_sweep, ModTarget::FilterCutoff, 0.35)],
            delay: None,
            reverb: Some(Reverb::new(0.8, 0.7, 0.6)), // Cathedral-like
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }
}
