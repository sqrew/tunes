#![allow(unused, dead_code)]

// mod actions;  // Unused module with broken imports
mod automation;
mod chords;
mod composition;
mod drum_grid;
mod drums;
mod effects;
mod engine;
mod envelope;
mod error;
mod filter;
mod filter_envelope;
mod fm_synthesis;
mod instruments;
mod key_signature;
mod lfo;
mod microtonal;
mod midi;
mod notes;
mod rhythm;
mod sample;
mod scales;
mod sequences;
mod theory;
mod track;
mod waveform;
mod wavetable;

// Prelude module for convenient imports
pub mod prelude {
    pub use crate::filter_envelope::FilterEnvelope;
    pub use crate::fm_synthesis::FMParams;
}

use chords::*;
use composition::Composition;
use effects::{BitCrusher, Delay, Distortion, Reverb, Saturation};
use engine::AudioEngine;
use filter::Filter;
use instruments::Instrument;
use lfo::{LFO, ModRoute, ModTarget};
use notes::*;
use rhythm::Tempo;
use scales::*;
use sequences::euclidean;
use waveform::Waveform;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    let x = sequences::collatz(27, 300);
    let x = sequences::normalize(&x, 20.0, 10000.0);

    comp.instrument("collatz", &Instrument::harpsichord())
        .notes(&x, 0.05);

    let y = &ScalePattern::

cioEoBH1FfwBpKqh5Uy71qXTaJiJE0KjtY6
    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
