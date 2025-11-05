//! Bass instrument presets

use super::Instrument;
use crate::synthesis::effects::{Distortion, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Deep sub bass - pure sine wave with long sustain
    pub fn sub_bass() -> Self {
        Self {
            name: "Sub Bass".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.01, 0.1, 0.9, 0.3),
            filter: Filter::low_pass(150.0, 0.1),
            modulation: Vec::new(),
            delay: None,
            reverb: None,
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Reese bass - detuned sawtooth with filter sweep
    pub fn reese_bass() -> Self {
        let filter_lfo = LFO::new(Waveform::Sine, 0.3, 0.8);
        Self {
            name: "Reese Bass".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.01, 0.2, 0.7, 0.4),
            filter: Filter::low_pass(800.0, 0.5),
            modulation: vec![ModRoute::new(filter_lfo, ModTarget::FilterCutoff, 0.3)],
            delay: None,
            reverb: None,
            distortion: Some(Distortion::new(1.5, 0.3)),
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Acid bass - resonant filter with heavy modulation
    pub fn acid_bass() -> Self {
        let filter_lfo = LFO::new(Waveform::Sine, 0.5, 1.0);
        Self {
            name: "Acid Bass".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.001, 0.3, 0.3, 0.2),
            filter: Filter::low_pass(400.0, 0.6),
            modulation: vec![ModRoute::new(filter_lfo, ModTarget::FilterCutoff, 0.6)],
            delay: None,
            reverb: None,
            distortion: Some(Distortion::new(2.0, 0.4)),
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Wobble bass - LFO on filter for dubstep-style wobble
    pub fn wobble_bass() -> Self {
        let wobble_lfo = LFO::new(Waveform::Sine, 4.0, 1.0);
        Self {
            name: "Wobble Bass".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.01, 0.1, 0.8, 0.3),
            filter: Filter::low_pass(600.0, 0.7),
            modulation: vec![ModRoute::new(wobble_lfo, ModTarget::FilterCutoff, 0.7)],
            delay: None,
            reverb: None,
            distortion: Some(Distortion::new(2.5, 0.5)),
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Deep bass - ultra-low sub with slight movement
    pub fn deep_bass() -> Self {
        Self {
            name: "Deep Bass".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.02, 0.15, 0.95, 0.4),
            filter: Filter::low_pass(100.0, 0.2), // Very low cutoff
            modulation: Vec::new(),
            delay: None,
            reverb: None,
            distortion: Some(Distortion::new(1.2, 0.2)), // Slight warmth
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Funk bass - punchy, percussive bass with bite
    pub fn funk_bass() -> Self {
        Self {
            name: "Funk Bass".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.08, 0.4, 0.1), // Sharp attack, quick decay
            filter: Filter::low_pass(600.0, 0.6),           // Mid-bass range
            modulation: Vec::new(),
            delay: None,
            reverb: None,
            distortion: Some(Distortion::new(2.2, 0.4)), // Gritty tone
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Bass percussion - punchy, percussive bass hit
    pub fn bass_percussion() -> Self {
        Self {
            name: "Bass Percussion".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.001, 0.08, 0.2, 0.15), // Sharp attack, quick decay
            filter: Filter::low_pass(300.0, 0.4),
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.2, 0.3, 0.15)),
            distortion: Some(Distortion::new(1.8, 0.3)), // Slight grit
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Upright bass - plucked acoustic bass
    pub fn upright_bass() -> Self {
        Self {
            name: "Upright Bass".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.002, 0.2, 0.4, 0.3), // Plucky attack, quick decay
            filter: Filter::low_pass(800.0, 0.4),          // Warm, woody tone
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.2, 0.3, 0.15)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// 808 bass - classic TR-808 drum machine bass
    pub fn bass_808() -> Self {
        Self {
            name: "808 Bass".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.001, 0.15, 0.0, 0.4), // Punchy attack, no sustain, long release
            filter: Filter::low_pass(80.0, 0.3),            // Very sub-heavy
            modulation: Vec::new(),
            delay: None,
            reverb: None,
            distortion: Some(Distortion::new(1.3, 0.25)), // Slight warmth
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Slap bass - percussive electric bass with bright attack
    pub fn slap_bass() -> Self {
        Self {
            name: "Slap Bass".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.0005, 0.05, 0.3, 0.15), // Very sharp attack, quick decay
            filter: Filter::low_pass(2000.0, 0.7),            // Bright, percussive
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.15, 0.25, 0.1)),
            distortion: Some(Distortion::new(1.6, 0.3)), // Adds punch
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Synth bass - modern electronic bass with movement
    pub fn synth_bass() -> Self {
        let pulse_lfo = LFO::new(Waveform::Sine, 0.4, 0.6);
        Self {
            name: "Synth Bass".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.005, 0.15, 0.7, 0.35),
            filter: Filter::low_pass(500.0, 0.5),
            modulation: vec![ModRoute::new(pulse_lfo, ModTarget::FilterCutoff, 0.25)],
            delay: None,
            reverb: None,
            distortion: Some(Distortion::new(1.4, 0.25)),
            volume: 1.0,
            pan: 0.0,
        }
    }
}
