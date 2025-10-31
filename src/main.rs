#![allow(unused, dead_code)]

mod chords;
mod composition;
mod drum_grid;
mod drums;
mod effects;
mod engine;
mod envelope;
mod filter;
mod instruments;
mod lfo;
mod notes;
mod rhythm;
mod scales;
mod sequences;
mod track;
mod waveform;

use chords::*;
use composition::Composition;
use effects::{BitCrusher, Delay, Distortion, Reverb, Saturation};
use engine::AudioEngine;
use filter::Filter;
use instruments::Instrument;
use lfo::{LFO, ModRoute, ModTarget};
use notes::{C1, *};
use rhythm::Tempo;
use scales::*;
use sequences::euclidean;
use waveform::Waveform;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
