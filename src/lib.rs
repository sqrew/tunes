//! # tunes
//!
//! A comprehensive music composition, synthesis, and audio generation library.
//!
//! ## Quick Start
//!
//! ```no_run
//! use tunes::prelude::*;
//!
//! fn main() -> Result<(), anyhow::Error> {
//!     let engine = AudioEngine::new()?;
//!     let mut comp = Composition::new(Tempo::new(120.0));
//!
//!     comp.track("piano")
//!         .note(&[C4], 0.5)
//!         .note(&[E4], 0.5)
//!         .note(&[G4], 0.5)
//!         .note(&[C5], 0.5);
//!
//!     engine.play_mixer(&comp.into_mixer())?;
//!     Ok(())
//! }
//! ```

pub mod chords;
pub mod composition;
pub mod drum_grid;
pub mod drums;
pub mod effects;
pub mod engine;
pub mod envelope;
pub mod filter;
pub mod instruments;
pub mod lfo;
pub mod notes;
pub mod rhythm;
pub mod scales;
pub mod sequences;
pub mod theory;
pub mod track;
pub mod waveform;

/// Prelude module for convenient imports
pub mod prelude {
    // Core composition
    pub use crate::composition::Composition;
    pub use crate::engine::AudioEngine;
    pub use crate::rhythm::Tempo;

    // Notes
    pub use crate::notes::*;

    // Theory
    pub use crate::theory::{chord, scale, transpose, transpose_sequence, ChordPattern, ScalePattern, ProgressionType, progression};

    // Instruments
    pub use crate::instruments::Instrument;

    // Effects
    pub use crate::effects::*;
    pub use crate::filter::Filter;

    // Drums
    pub use crate::drums::DrumType;
    pub use crate::drum_grid::DrumGrid;

    // Envelopes and waveforms
    pub use crate::envelope::Envelope;
    pub use crate::waveform::Waveform;

    // Sequences
    pub use crate::sequences::{euclidean, euclidean_pattern};

    // LFO
    pub use crate::lfo::{LFO, ModRoute, ModTarget};
}
