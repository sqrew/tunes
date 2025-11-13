#![allow(unused, dead_code)]

mod composition;
mod consts;
mod engine;
mod error;
mod instruments;
mod midi;
mod sequences;
mod synthesis;
mod templates;
mod theory;
mod track;

// Prelude module for convenient imports
pub mod prelude {
    pub use crate::instruments::Instrument;
    pub use crate::synthesis::FMParams;
    pub use crate::synthesis::FilterEnvelope;
}

use composition::{Composition, DrumType, Tempo};
use consts::*;
use engine::AudioEngine;
use instruments::Instrument;
use synthesis::{BitCrusher, Chorus, Compressor};

fn main() -> Result<(), anyhow::Error> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let eighth = comp.tempo().eighth_note();

    let engine = AudioEngine::new()?;
    engine.print_info();
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
