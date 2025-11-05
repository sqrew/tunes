#![allow(unused, dead_code)]

// mod actions;  // Unused module with broken imports
mod composition;
mod consts;
mod engine;
mod error;
mod instruments;
mod midi;
mod sequences;
mod synthesis;
mod theory;
mod track;

// Prelude module for convenient imports
pub mod prelude {
    pub use crate::synthesis::FMParams;
    pub use crate::synthesis::FilterEnvelope;
}

use composition::{Composition, Tempo};
use consts::*;
use engine::AudioEngine;
use instruments::Instrument;
use sequences::euclidean;
use synthesis::effects::{BitCrusher, Delay, Distortion, Reverb, Saturation};
use synthesis::filter::Filter;
use synthesis::lfo::{LFO, ModRoute, ModTarget};
use synthesis::noise::NoiseType;
use synthesis::waveform::Waveform;

fn main() -> Result<(), anyhow::Error> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let eighth = comp.tempo().eighth_note();

    comp.instrument("lead", &Instrument::electric_piano())
        .scale_updown(C4_MAJOR_SCALE, eighth);

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}

/*
- bass_808() - Classic TR-808 drum machine bass with punchy attack and sub-heavy tone
- slap_bass() - Percussive electric bass with bright attack and punch
- synth_bass() - Modern electronic bass with filter modulation and movement

Leads category (src/instruments/leads.rs) - Added 3 presets:
- laser_lead() - Bright, futuristic lead with fast filter sweep
- detuned_lead() - Thick, chorus-like lead with slight detuning effect
- scream_lead() - Aggressive, heavily distorted lead

Pads category (src/instruments/pads.rs) - Added 3 presets:
- dark_pad() - Brooding, atmospheric pad with low frequency content
- shimmer_pad() - Bright, evolving pad with high-frequency sparkle
- string_pad() - Lush string ensemble pad with vibrato

Orchestral category (src/instruments/orchestral.rs) - Added 4 presets:
- oboe() - Nasal, reedy double-reed woodwind
- bassoon() - Deep, woody double-reed woodwind
- french_horn() - Warm, mellow brass instrument
- harp() - Plucked string instrument with bright, shimmering tone

Keys category (src/instruments/keys.rs) - Added 3 presets:
- clavinet() - Funky, percussive electric keyboard
- wurlitzer() - Warm electric piano with bell-like tone and tremolo
- toy_piano() - Small, metallic, quirky sound

Synths category (src/instruments/synths.rs) - Added 3 presets:
- acid_synth() - Classic TB-303 style acid sound with resonant sweep
- trance_synth() - Uplifting, anthem-style trance lead
- analog_brass() - Warm, vintage synth brass section

  */
