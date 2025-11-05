//! Instrument presets organized by category

pub mod bass;
pub mod leads;
pub mod pads;
pub mod keys;
pub mod orchestral;
pub mod fx;
pub mod synths;
pub mod ethnic;
pub mod percussion;
pub mod guitars;
pub mod strings;
pub mod vocal;

use crate::synthesis::effects::{Delay, Distortion, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::ModRoute;
use crate::track::Track;
use crate::synthesis::waveform::Waveform;

/// Instrument preset that combines all synthesis parameters
#[derive(Debug, Clone)]
pub struct Instrument {
    pub name: String,
    pub waveform: Waveform,
    pub envelope: Envelope,
    pub filter: Filter,
    pub modulation: Vec<ModRoute>,
    pub delay: Option<Delay>,
    pub reverb: Option<Reverb>,
    pub distortion: Option<Distortion>,
    pub volume: f32,
    pub pan: f32,
}

impl Instrument {
    /// Create a custom instrument from components
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::default(),
            filter: Filter::none(),
            modulation: Vec::new(),
            delay: None,
            reverb: None,
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Apply this instrument's settings to a track
    pub fn apply_to_track(&self, mut track: Track) -> Track {
        track.volume = self.volume;
        track.pan = self.pan;
        track.filter = self.filter;
        track.delay = self.delay.clone();
        track.reverb = self.reverb.clone();
        track.distortion = self.distortion.clone();
        track.modulation = self.modulation.clone();
        track
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instrument_creation() {
        let bass = Instrument::sub_bass();
        assert_eq!(bass.name, "Sub Bass");

        let lead = Instrument::saw_lead();
        assert_eq!(lead.name, "Saw Lead");

        let pad = Instrument::warm_pad();
        assert_eq!(pad.name, "Warm Pad");
    }
}
