//! Genre-specific synthesizer presets

use super::Instrument;
use crate::synthesis::effects::{Delay, Distortion, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Supersaw - massive trance lead (simulated detuned saw stack)
    pub fn supersaw() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.0, 0.35);
        Self {
            name: "Supersaw".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.02, 0.25, 0.75, 0.4),
            filter: Filter::low_pass(6000.0, 0.5), // Very bright
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.25)],
            delay: Some(Delay::new(0.375, 0.25, 0.2)),
            reverb: Some(Reverb::new(0.5, 0.5, 0.35)),
            distortion: Some(Distortion::new(1.2, 0.15)), // Slight warmth
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// FM bells - digital bell-like tones
    pub fn fm_bells() -> Self {
        Self {
            name: "FM Bells".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.001, 1.0, 0.3, 1.2), // Long decay like bells
            filter: Filter::low_pass(8000.0, 0.2),         // Bright and clear
            modulation: Vec::new(),
            delay: Some(Delay::new(0.5, 0.4, 0.3)),
            reverb: Some(Reverb::new(0.7, 0.6, 0.5)), // Spacious
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Hoover synth - classic rave/hardcore sound
    pub fn hoover() -> Self {
        let sweep = LFO::new(Waveform::Sine, 0.8, 0.9); // Slow sweep
        Self {
            name: "Hoover".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.01, 0.2, 0.8, 0.3),
            filter: Filter::low_pass(1200.0, 0.8), // Resonant sweep
            modulation: vec![ModRoute::new(sweep, ModTarget::FilterCutoff, 0.7)],
            delay: Some(Delay::new(0.25, 0.3, 0.2)),
            reverb: Some(Reverb::new(0.4, 0.5, 0.3)),
            distortion: Some(Distortion::new(2.0, 0.4)), // Gritty
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Stabby lead - sharp, aggressive synth stabs
    pub fn stab() -> Self {
        Self {
            name: "Stab".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.08, 0.2, 0.15), // Very sharp
            filter: Filter::low_pass(3000.0, 0.6),
            modulation: Vec::new(),
            delay: Some(Delay::new(0.375, 0.3, 0.25)),
            reverb: Some(Reverb::new(0.3, 0.4, 0.25)),
            distortion: Some(Distortion::new(2.2, 0.5)), // Aggressive
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Chiptune lead - retro 8-bit game sound
    pub fn chiptune() -> Self {
        Self {
            name: "Chiptune".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.03, 0.5, 0.05), // Very fast, clicky
            filter: Filter::low_pass(4000.0, 0.2),
            modulation: Vec::new(),
            delay: None,
            reverb: None, // Dry, retro sound
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Acid synth - classic TB-303 style acid sound
    pub fn acid_synth() -> Self {
        let acid_sweep = LFO::new(Waveform::Sine, 0.7, 0.8); // Fast resonant sweep
        Self {
            name: "Acid Synth".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.001, 0.25, 0.3, 0.15), // Sharp, quick
            filter: Filter::low_pass(700.0, 0.75),            // Highly resonant
            modulation: vec![ModRoute::new(acid_sweep, ModTarget::FilterCutoff, 0.7)],
            delay: None,
            reverb: Some(Reverb::new(0.2, 0.3, 0.15)), // Minimal reverb
            distortion: Some(Distortion::new(2.5, 0.4)), // Gritty acid character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Trance synth - uplifting, anthem-style trance lead
    pub fn trance_synth() -> Self {
        let trance_lfo = LFO::new(Waveform::Sine, 0.3, 0.6); // Slow movement
        let vibrato = LFO::new(Waveform::Sine, 5.5, 0.25);
        Self {
            name: "Trance Synth".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.05, 0.3, 0.8, 0.5), // Medium attack for anthem feel
            filter: Filter::low_pass(5500.0, 0.45),       // Bright, uplifting
            modulation: vec![
                ModRoute::new(trance_lfo, ModTarget::FilterCutoff, 0.3),
                ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.15),
            ],
            delay: Some(Delay::new(0.375, 0.3, 0.25)),
            reverb: Some(Reverb::new(0.6, 0.6, 0.45)), // Spacious, euphoric
            distortion: Some(Distortion::new(1.3, 0.2)),
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Analog brass - warm, vintage synth brass section
    pub fn analog_brass() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.8, 0.3);
        Self {
            name: "Analog Brass".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.08, 0.2, 0.8, 0.4), // Punchy attack for brass
            filter: Filter::low_pass(3500.0, 0.42),       // Warm, full
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.12)],
            delay: None,
            reverb: Some(Reverb::new(0.35, 0.45, 0.3)),
            distortion: Some(Distortion::new(1.4, 0.25)), // Analog warmth
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// FM bass - metallic, digital bell-like bass with FM character
    pub fn fm_bass() -> Self {
        let fm_sweep = LFO::new(Waveform::Sine, 0.6, 0.5); // Metallic sweep
        Self {
            name: "FM Bass".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.001, 0.25, 0.5, 0.35), // Metallic attack
            filter: Filter::low_pass(1500.0, 0.6),           // Metallic, digital
            modulation: vec![ModRoute::new(fm_sweep, ModTarget::FilterCutoff, 0.35)],
            delay: None,
            reverb: Some(Reverb::new(0.25, 0.35, 0.2)),
            distortion: Some(Distortion::new(2.0, 0.4)), // Digital character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// PWM bass - pulse width modulation for thick, evolving bass
    pub fn pwm_bass() -> Self {
        let pwm_lfo = LFO::new(Waveform::Sine, 0.5, 0.7); // Pulse width sweep
        Self {
            name: "PWM Bass".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.01, 0.2, 0.75, 0.4), // Thick, sustained
            filter: Filter::low_pass(700.0, 0.5),          // Deep, evolving
            modulation: vec![ModRoute::new(pwm_lfo, ModTarget::FilterCutoff, 0.3)],
            delay: None,
            reverb: None,
            distortion: Some(Distortion::new(1.5, 0.3)), // Adds thickness
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Pluck bass - short, percussive bass hits for rhythmic patterns
    pub fn pluck_bass() -> Self {
        Self {
            name: "Pluck Bass".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.08, 0.15, 0.12), // Very short, plucky
            filter: Filter::low_pass(800.0, 0.45),            // Punchy, percussive
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.2, 0.3, 0.15)),
            distortion: Some(Distortion::new(1.6, 0.3)), // Adds punch
            volume: 1.0,
            pan: 0.0,
        }
    }
}
