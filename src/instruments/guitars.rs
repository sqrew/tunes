//! Guitar instrument presets

use super::Instrument;
use crate::synthesis::effects::{Delay, Distortion, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Acoustic guitar - strummed/plucked with natural wooden decay
    pub fn acoustic_guitar() -> Self {
        Self {
            name: "Acoustic Guitar".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.002, 0.4, 0.3, 0.6), // Plucked attack, natural decay
            filter: Filter::low_pass(4500.0, 0.28),        // Warm, woody tone
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.25, 0.35, 0.2)), // Subtle room ambience
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Electric guitar (clean) - warm, sustained clean electric tone
    pub fn electric_guitar_clean() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.0, 0.2); // Subtle vibrato
        Self {
            name: "Electric Guitar (Clean)".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.005, 0.3, 0.5, 0.7), // Plucked with sustain
            filter: Filter::low_pass(5000.0, 0.32),        // Bright but smooth
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.08)],
            delay: Some(Delay::new(0.35, 0.25, 0.2)),
            reverb: Some(Reverb::new(0.35, 0.45, 0.28)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Electric guitar (distorted) - rock/metal with heavy distortion
    pub fn electric_guitar_distorted() -> Self {
        Self {
            name: "Electric Guitar (Distorted)".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.008, 0.25, 0.6, 0.5), // Aggressive attack, sustained
            filter: Filter::low_pass(4000.0, 0.55),         // Crunchy, aggressive
            modulation: Vec::new(),
            delay: Some(Delay::new(0.3, 0.3, 0.25)),
            reverb: Some(Reverb::new(0.3, 0.4, 0.25)),
            distortion: Some(Distortion::new(3.5, 0.65)), // Heavy rock/metal distortion
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// 12-string guitar - chorused, shimmering acoustic with doubled strings
    pub fn guitar_12_string() -> Self {
        let chorus = LFO::new(Waveform::Sine, 0.8, 0.5); // Chorus/shimmer effect
        Self {
            name: "12-String Guitar".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.002, 0.45, 0.35, 0.7), // Plucked, shimmering
            filter: Filter::low_pass(5500.0, 0.3),           // Bright, shimmering
            modulation: vec![ModRoute::new(chorus, ModTarget::FilterCutoff, 0.15)],
            delay: Some(Delay::new(0.25, 0.28, 0.22)),
            reverb: Some(Reverb::new(0.4, 0.48, 0.32)), // Open, spacious
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Palm muted guitar - tight, percussive metal rhythm guitar
    pub fn guitar_palm_muted() -> Self {
        Self {
            name: "Palm Muted Guitar".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.001, 0.05, 0.15, 0.08), // Very short, tight attack
            filter: Filter::low_pass(1200.0, 0.55),           // Dark, chunky mid-range
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.15, 0.25, 0.12)), // Minimal reverb for tightness
            distortion: Some(Distortion::new(3.0, 0.65)), // Heavy metal distortion
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Guitar harmonics - bell-like natural and artificial harmonics
    pub fn guitar_harmonics() -> Self {
        let shimmer = LFO::new(Waveform::Sine, 0.3, 0.2); // Subtle shimmer
        Self {
            name: "Guitar Harmonics".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.001, 0.5, 0.2, 0.8), // Bell-like attack, long decay
            filter: Filter::low_pass(7000.0, 0.2),         // Very bright, bell-like tone
            modulation: vec![ModRoute::new(shimmer, ModTarget::FilterCutoff, 0.15)],
            delay: Some(Delay::new(0.4, 0.3, 0.25)),
            reverb: Some(Reverb::new(0.6, 0.65, 0.45)), // Spacious, ethereal reverb
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }
}
