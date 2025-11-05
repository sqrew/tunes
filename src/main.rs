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

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
