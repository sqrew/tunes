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

fn main() -> Result<(), anyhow::Error> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let eighth = comp.tempo().eighth_note();

    comp.instrument("lead", &Instrument::baritone_sax())
        .notes(&[C4, C3, C2, C1], 0.5);

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
