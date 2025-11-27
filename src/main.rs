#![allow(unused, dead_code)]

mod cache;
mod composition;
mod consts;
mod engine;
mod error;
#[cfg(feature = "gpu")]
mod gpu;
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
use synthesis::{BitCrusher, Chorus, Compressor, Distortion};
use tunes::play_sample;

fn main() -> Result<(), anyhow::Error> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let eighth = comp.tempo().eighth_note();

    comp.instrument("a", &Instrument::analog_brass())
        .notes(&[C4, C3, C2, C1], 1.0);

    comp.track("track")
        .drum_grid(
            16,
            1.5,
            (|f| f.sound(DrumType::Castanet, "xx--xx--xx--xx--")),
        )
        .transform(|t| t.mutate(2));

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
