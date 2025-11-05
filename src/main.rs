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
    pub use crate::synthesis::FilterEnvelope;
    pub use crate::synthesis::FMParams;
}

use composition::{Composition, Tempo};
use consts::*;
use synthesis::effects::{BitCrusher, Delay, Distortion, Reverb, Saturation};
use engine::AudioEngine;
use synthesis::filter::Filter;
use instruments::Instrument;
use synthesis::lfo::{LFO, ModRoute, ModTarget};
use synthesis::noise::NoiseType;
use sequences::euclidean;
use synthesis::waveform::Waveform;

fn main() -> Result<(), anyhow::Error> {
    let mut comp = Composition::new(Tempo::new(140.0));

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
