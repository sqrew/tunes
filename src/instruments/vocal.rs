//! Vocal and choir instrument presets

use super::Instrument;
use crate::synthesis::effects::{Delay, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Choir aahs - warm vocal choir with "ah" sound
    pub fn choir_aahs() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 4.8, 0.25); // Natural vocal vibrato
        Self {
            name: "Choir Aahs".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.4, 0.3, 0.9, 0.8), // Slow vocal attack
            filter: Filter::low_pass(2500.0, 0.3),       // Vocal formant range
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.12)],
            delay: None,
            reverb: Some(Reverb::new(0.75, 0.7, 0.6)), // Cathedral space
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Choir oohs - closed vocal choir with "oo" sound
    pub fn choir_oohs() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 4.5, 0.22);
        Self {
            name: "Choir Oohs".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.45, 0.35, 0.92, 0.85), // Gentle vocal swell
            filter: Filter::low_pass(1800.0, 0.25),          // Darker, closed vowel
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.1)],
            delay: None,
            reverb: Some(Reverb::new(0.78, 0.72, 0.62)), // Spacious
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Synth voice - electronic vocoder-like voice
    pub fn synth_voice() -> Self {
        let formant = LFO::new(Waveform::Sine, 0.3, 0.4); // Slow formant shift
        Self {
            name: "Synth Voice".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.2, 0.25, 0.85, 0.5), // Synthetic vocal
            filter: Filter::low_pass(2200.0, 0.45),        // Formant-like
            modulation: vec![ModRoute::new(formant, ModTarget::FilterCutoff, 0.25)],
            delay: Some(Delay::new(0.3, 0.3, 0.25)),
            reverb: Some(Reverb::new(0.5, 0.55, 0.4)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }
}
