//! Special effects and transition instrument presets

use super::Instrument;
use crate::synthesis::effects::{Delay, Distortion, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Riser - sweep up effect (use with longer notes for best effect)
    pub fn riser() -> Self {
        let sweep_lfo = LFO::new(Waveform::Triangle, 0.5, 1.0); // 2 second sweep up
        Self {
            name: "Riser".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.2, 0.3, 0.9, 0.5),
            filter: Filter::low_pass(200.0, 0.5),
            modulation: vec![ModRoute::new(sweep_lfo, ModTarget::FilterCutoff, 0.9)],
            delay: Some(Delay::new(0.25, 0.4, 0.3)),
            reverb: Some(Reverb::new(0.7, 0.6, 0.5)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Impact - short, punchy hit
    pub fn impact() -> Self {
        Self {
            name: "Impact".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.001, 0.05, 0.1, 0.3),
            filter: Filter::low_pass(500.0, 0.6),
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.5, 0.5, 0.4)),
            distortion: Some(Distortion::new(3.0, 0.6)),
            volume: 1.0,
            pan: 0.0,
        }
    }
}
