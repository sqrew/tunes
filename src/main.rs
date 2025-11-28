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

    // comp.instrument("a", &Instrument::analog_brass())
    //     .notes(&[C4, C3, C2, C1], 1.0);

    comp.track("breaks")
        .swing(0.54)
        .drum_grid(32, 0.05, |g| {
            g // 32 steps at 174bpm
                .sound(DrumType::KickTight, "x-------x-------x-------x-x-----")
                .sound(DrumType::Snare, "----x-------x-x-----x-------x---")
                .ghost(DrumType::Snare, "-x-x--x--x-x---x-x-x--x--x-x---x", 0.25)
                .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-x-x-x-x-x-x-x-x-")
        })
        .humanize(0.008, 0.1)
        .repeat(3);

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
