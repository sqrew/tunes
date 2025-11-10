//! Experimental and unique sound design instruments

use super::Instrument;
use crate::synthesis::effects::{Delay, Distortion, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
use crate::synthesis::waveform::Waveform;

impl Instrument {
    /// Glitch synth - chaotic, stuttering, unpredictable digital sound
    pub fn glitch() -> Self {
        let chaos = LFO::new(Waveform::Square, 7.0, 0.9); // Fast chaotic modulation
        Self {
            name: "Glitch".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.05, 0.3, 0.08), // Very short, stuttering
            filter: Filter::low_pass(4000.0, 0.8),           // Bright, unpredictable
            modulation: vec![ModRoute::new(chaos, ModTarget::FilterCutoff, 0.7)],
            delay: Some(Delay::new(0.125, 0.5, 0.4)), // Stuttering delay
            reverb: Some(Reverb::new(0.2, 0.3, 0.15)),
            distortion: Some(Distortion::new(2.8, 0.6)), // Heavy digital distortion
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Bitcrushed noise - lo-fi digital degradation with crushing artifacts
    pub fn bitcrush_noise() -> Self {
        let wobble = LFO::new(Waveform::Sine, 1.2, 0.6);
        Self {
            name: "Bitcrush Noise".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.01, 0.15, 0.5, 0.3), // Digital attack
            filter: Filter::low_pass(2500.0, 0.7),         // Crushed, lo-fi
            modulation: vec![ModRoute::new(wobble, ModTarget::FilterCutoff, 0.5)],
            delay: Some(Delay::new(0.0625, 0.45, 0.35)), // Short digital delay
            reverb: None,                                 // Dry for lo-fi character
            distortion: Some(Distortion::new(3.5, 0.8)), // Extreme crushing
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Granular pad - textured, grainy atmospheric pad with micro-timbres
    pub fn granular_pad() -> Self {
        let grains = LFO::new(Waveform::Sine, 0.25, 0.45); // Slow granular movement
        Self {
            name: "Granular Pad".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.5, 0.6, 0.85, 1.2), // Very slow, evolving
            filter: Filter::low_pass(2800.0, 0.4),        // Grainy, textured
            modulation: vec![ModRoute::new(grains, ModTarget::FilterCutoff, 0.3)],
            delay: Some(Delay::new(0.45, 0.4, 0.35)),
            reverb: Some(Reverb::new(0.8, 0.75, 0.65)), // Massive, atmospheric
            distortion: Some(Distortion::new(1.3, 0.2)), // Subtle texture
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Ring mod bells - metallic, inharmonic bell tones from ring modulation
    pub fn ring_mod_bells() -> Self {
        let ring_mod = LFO::new(Waveform::Sine, 12.0, 0.8); // Fast ring modulation
        Self {
            name: "Ring Mod Bells".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.001, 0.8, 0.25, 1.0), // Bell-like, inharmonic decay
            filter: Filter::low_pass(7000.0, 0.35),         // Bright, metallic
            modulation: vec![ModRoute::new(ring_mod, ModTarget::FilterCutoff, 0.6)],
            delay: Some(Delay::new(0.35, 0.4, 0.3)),
            reverb: Some(Reverb::new(0.7, 0.65, 0.5)),
            distortion: Some(Distortion::new(1.5, 0.3)), // Metallic character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Formant synth - vowel-like vocal formant synthesizer
    pub fn formant_synth() -> Self {
        let formant = LFO::new(Waveform::Sine, 0.6, 0.7); // Vowel morphing
        Self {
            name: "Formant Synth".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.05, 0.2, 0.7, 0.4), // Vocal-like attack
            filter: Filter::low_pass(2200.0, 0.75),       // Resonant formant peaks
            modulation: vec![ModRoute::new(formant, ModTarget::FilterCutoff, 0.5)],
            delay: None,
            reverb: Some(Reverb::new(0.35, 0.45, 0.3)),
            distortion: Some(Distortion::new(1.4, 0.25)), // Vocal character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Theremin - ethereal, oscillating electronic instrument
    pub fn theremin() -> Self {
        let vibrato = LFO::new(Waveform::Sine, 6.0, 0.4); // Expressive vibrato
        Self {
            name: "Theremin".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.15, 0.2, 0.9, 0.5), // Smooth, gestural attack
            filter: Filter::low_pass(4500.0, 0.3),        // Pure, ethereal tone
            modulation: vec![ModRoute::new(vibrato, ModTarget::FilterCutoff, 0.25)],
            delay: Some(Delay::new(0.3, 0.35, 0.25)),
            reverb: Some(Reverb::new(0.6, 0.65, 0.5)), // Spacious, otherworldly
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Glass harmonica - crystalline, ethereal glass bowls
    pub fn glass_harmonica() -> Self {
        let shimmer = LFO::new(Waveform::Sine, 0.35, 0.4); // Glass shimmer
        Self {
            name: "Glass Harmonica".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.05, 1.0, 0.3, 1.5), // Slow rubbing attack, long decay
            filter: Filter::low_pass(6500.0, 0.25),       // Pure, glassy, crystalline
            modulation: vec![ModRoute::new(shimmer, ModTarget::FilterCutoff, 0.15)],
            delay: Some(Delay::new(0.4, 0.35, 0.28)),
            reverb: Some(Reverb::new(0.75, 0.7, 0.6)), // Ethereal, spacious
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Circuit bent organ - glitched, malfunctioning organ with unstable pitch
    pub fn circuit_bent() -> Self {
        let bend = LFO::new(Waveform::Square, 3.5, 0.8); // Circuit bending chaos
        Self {
            name: "Circuit Bent".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.02, 0.1, 0.8, 0.2), // Unstable attack
            filter: Filter::low_pass(3500.0, 0.8),        // Chaotic, unstable
            modulation: vec![ModRoute::new(bend, ModTarget::FilterCutoff, 0.65)],
            delay: Some(Delay::new(0.1875, 0.5, 0.4)), // Glitchy delay
            reverb: Some(Reverb::new(0.3, 0.4, 0.25)),
            distortion: Some(Distortion::new(2.5, 0.7)), // Broken circuit character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Vocoder synth - robotic, synthesized voice texture
    pub fn vocoder() -> Self {
        let voice = LFO::new(Waveform::Square, 2.0, 0.6); // Voice-like modulation
        Self {
            name: "Vocoder".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.03, 0.15, 0.75, 0.35), // Syllabic attack
            filter: Filter::low_pass(3000.0, 0.7),           // Robotic voice
            modulation: vec![ModRoute::new(voice, ModTarget::FilterCutoff, 0.45)],
            delay: Some(Delay::new(0.25, 0.3, 0.2)),
            reverb: Some(Reverb::new(0.3, 0.4, 0.28)),
            distortion: Some(Distortion::new(1.6, 0.35)), // Robotic character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Laser synth - sci-fi laser beam sound effect
    pub fn laser() -> Self {
        let sweep = LFO::new(Waveform::Sine, 15.0, 0.95); // Very fast sweep
        Self {
            name: "Laser".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(0.001, 0.12, 0.0, 0.15), // Sharp attack, no sustain
            filter: Filter::low_pass(8000.0, 0.85),          // Bright, sci-fi sweep
            modulation: vec![ModRoute::new(sweep, ModTarget::FilterCutoff, 0.8)],
            delay: Some(Delay::new(0.15, 0.4, 0.3)),
            reverb: Some(Reverb::new(0.4, 0.5, 0.35)),
            distortion: Some(Distortion::new(2.0, 0.5)), // Sci-fi distortion
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Drone machine - deep, evolving ambient drone
    pub fn drone_machine() -> Self {
        let evolve = LFO::new(Waveform::Sine, 0.08, 0.5); // Very slow evolution
        Self {
            name: "Drone Machine".to_string(),
            waveform: Waveform::Sawtooth,
            envelope: Envelope::new(1.0, 1.5, 0.95, 2.0), // Extremely slow, sustained
            filter: Filter::low_pass(800.0, 0.6),         // Deep, dark, evolving
            modulation: vec![ModRoute::new(evolve, ModTarget::FilterCutoff, 0.35)],
            delay: Some(Delay::new(0.65, 0.5, 0.45)),
            reverb: Some(Reverb::new(0.9, 0.85, 0.75)), // Massive ambient space
            distortion: Some(Distortion::new(1.5, 0.3)), // Subtle saturation
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Metallic percussive - harsh metallic impacts and resonances
    pub fn metallic_perc() -> Self {
        let ring = LFO::new(Waveform::Sine, 8.5, 0.7); // Metallic ringing
        Self {
            name: "Metallic Percussion".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.3, 0.1, 0.6), // Sharp metal strike
            filter: Filter::low_pass(6500.0, 0.75),        // Bright, metallic resonance
            modulation: vec![ModRoute::new(ring, ModTarget::FilterCutoff, 0.5)],
            delay: Some(Delay::new(0.18, 0.35, 0.28)),
            reverb: Some(Reverb::new(0.55, 0.6, 0.45)),
            distortion: Some(Distortion::new(2.2, 0.6)), // Harsh metallic character
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Data stream - digital information flow, computer sounds
    pub fn data_stream() -> Self {
        let stream = LFO::new(Waveform::Square, 11.0, 0.85); // Fast digital stream
        Self {
            name: "Data Stream".to_string(),
            waveform: Waveform::Square,
            envelope: Envelope::new(0.001, 0.08, 0.4, 0.1), // Fast digital burst
            filter: Filter::low_pass(5500.0, 0.8),          // Digital, computational
            modulation: vec![ModRoute::new(stream, ModTarget::FilterCutoff, 0.7)],
            delay: Some(Delay::new(0.0833, 0.55, 0.45)), // Very short digital delay
            reverb: None,                                 // Dry digital sound
            distortion: Some(Distortion::new(3.0, 0.75)), // Digital clipping
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Wind chimes - delicate, random-sounding metallic chimes
    pub fn wind_chimes() -> Self {
        let flutter = LFO::new(Waveform::Sine, 0.4, 0.45); // Random flutter
        Self {
            name: "Wind Chimes".to_string(),
            waveform: Waveform::Triangle,
            envelope: Envelope::new(0.001, 0.6, 0.15, 1.2), // Delicate strike, long decay
            filter: Filter::low_pass(7500.0, 0.3),          // Bright, bell-like, airy
            modulation: vec![ModRoute::new(flutter, ModTarget::FilterCutoff, 0.2)],
            delay: Some(Delay::new(0.32, 0.4, 0.3)),
            reverb: Some(Reverb::new(0.75, 0.7, 0.6)), // Open air, outdoor space
            distortion: None,
            volume: 1.0,
            pan: 0.0,
        }
    }

    /// Cosmic rays - otherworldly, space-like atmospheric texture
    pub fn cosmic_rays() -> Self {
        let cosmic = LFO::new(Waveform::Sine, 0.15, 0.6); // Very slow cosmic movement
        Self {
            name: "Cosmic Rays".to_string(),
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.8, 1.2, 0.9, 2.0), // Extremely slow, cosmic
            filter: Filter::low_pass(3500.0, 0.35),      // Ethereal, spacey
            modulation: vec![ModRoute::new(cosmic, ModTarget::FilterCutoff, 0.4)],
            delay: Some(Delay::new(0.75, 0.55, 0.5)),
            reverb: Some(Reverb::new(0.95, 0.9, 0.85)), // Infinite space
            distortion: Some(Distortion::new(1.2, 0.15)), // Subtle cosmic saturation
            volume: 1.0,
            pan: 0.0,
        }
    }
}
