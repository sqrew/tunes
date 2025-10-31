use crate::effects::{Delay, Distortion, Reverb};
use crate::envelope::Envelope;
use crate::filter::Filter;
use crate::lfo::{LFO, ModRoute, ModTarget};
use crate::track::Track;
use crate::waveform::Waveform;

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
        track.distortion = self.distortion;
        track.modulation = self.modulation.clone();
        track
    }

    // ===== BASS INSTRUMENTS =====

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

    // ===== LEAD INSTRUMENTS =====

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

    // ===== PAD INSTRUMENTS =====

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

    // ===== ORGAN/KEYS =====

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

    // ===== SPECIAL FX =====

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

    // ===== MORE BASS INSTRUMENTS =====

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

    // ===== MORE LEAD INSTRUMENTS =====

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

    // ===== NEW INSTRUMENTS =====

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

    // ===== REALISTIC INSTRUMENTS =====

    /// Acoustic piano - warm, expressive piano sound
    pub fn acoustic_piano() -> Self {
        Self {
            name: "Acoustic Piano".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.3, 0.6, 0.8), // Natural piano decay
            filter: Filter::low_pass(8000.0, 0.15),         // Full frequency range
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.25, 0.4, 0.2)), // Subtle room ambience
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Strings - violin/cello ensemble sound
    pub fn strings() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 5.5, 0.3); // Natural string vibrato
        Self {
            name: "Strings".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.15, 0.3, 0.85, 0.6), // Slow attack, sustained
            filter: Filter::low_pass(4000.0, 0.25),         // Warm, not too bright
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.15)],
            delay: None,
            reverb: Some(Reverb::new(0.6, 0.6, 0.45)), // Concert hall ambience
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Brass - trumpet/horn section sound
    pub fn brass() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 6.0, 0.25);
        Self {
            name: "Brass".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.05, 0.15, 0.8, 0.4), // Moderate attack for brass punch
            filter: Filter::low_pass(5000.0, 0.5),          // Bright and brassy
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.12)],
            delay: None,
            reverb: Some(Reverb::new(0.35, 0.5, 0.25)),
            distortion: Some(Distortion::new(1.3, 0.2)), // Slight grit for realism
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Flute - soft, breathy woodwind sound
    pub fn flute() -> Self {
        let breath_lfo = LFO::new(Waveform::Sine, 4.0, 0.2); // Subtle breath modulation
        Self {
            name: "Flute".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.08, 0.2, 0.7, 0.5), // Gentle attack
            filter: Filter::low_pass(6000.0, 0.2),         // Soft, airy
            modulation: vec![ModRoute::new(breath_lfo, ModTarget::Volume, 0.08)],
            delay: None,
            reverb: Some(Reverb::new(0.4, 0.5, 0.3)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Clarinet - warm, woody wind sound
    pub fn clarinet() -> Self {
        Self {
            name: "Clarinet".to_string(),
            waveform: Waveform::Square,                    // Square wave for hollow tone
            envelope: Envelope::new(0.06, 0.2, 0.75, 0.4), // Moderate attack
            filter: Filter::low_pass(3500.0, 0.3),         // Warm, woody
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.35, 0.45, 0.25)),
            distortion: None,
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
            filter: Filter::low_pass(800.0, 0.4),           // Warm, woody tone
            modulation: Vec::new(),
            delay: None,
            reverb: Some(Reverb::new(0.2, 0.3, 0.15)),
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    // ===== GENRE-SPECIFIC SYNTHS =====

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
            filter: Filter::low_pass(8000.0, 0.2),          // Bright and clear
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

    /// Vocal pad - choir/vocal-like synth pad
    pub fn vocal_pad() -> Self {
        let formant_sweep = LFO::new(Waveform::Sine, 0.25, 0.4); // Slow formant-like sweep
        Self {
            name: "Vocal Pad".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.5, 0.4, 0.9, 1.0), // Very slow attack
            filter: Filter::low_pass(2500.0, 0.6),        // Vocal formant range
            modulation: vec![ModRoute::new(formant_sweep, ModTarget::FilterCutoff, 0.35)],
            delay: None,
            reverb: Some(Reverb::new(0.8, 0.7, 0.6)), // Cathedral-like
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
