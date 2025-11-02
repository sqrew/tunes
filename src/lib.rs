//! # tunes
//!
//! A comprehensive music composition, synthesis, and audio generation library.
//!
//! ## Quick Start
//!
//! ```no_run
//! use tunes::prelude::*;
//!
//! fn main() -> anyhow::Result<()> {
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
pub mod error;
pub mod filter;
pub mod filter_envelope;
pub mod fm_synthesis;
pub mod instruments;
pub mod key_signature;
pub mod lfo;
pub mod microtonal;
pub mod midi;
pub mod notes;
pub mod rhythm;
pub mod sample;
pub mod scales;
pub mod sequences;
pub mod theory;
pub mod track;
pub mod waveform;
pub mod wavetable;

/// Prelude module for convenient imports
pub mod prelude {
    // Core composition
    pub use crate::composition::Composition;
    pub use crate::engine::AudioEngine;
    pub use crate::rhythm::Tempo;
    pub use crate::track::Mixer;

    // Error handling
    pub use crate::error::{Result, TunesError};

    // Notes
    pub use crate::notes::*;

    // Scales and Chords
    pub use crate::scales::*;
    pub use crate::chords::*;

    // Theory
    pub use crate::theory::{chord, scale, transpose, transpose_sequence, ChordPattern, ScalePattern, ProgressionType, progression};
    pub use crate::key_signature::{KeySignature, KeyRoot, KeyMode};

    // Instruments
    pub use crate::instruments::Instrument;

    // Effects
    pub use crate::effects::*;
    pub use crate::filter::{Filter, FilterType};

    // Drums
    pub use crate::drums::DrumType;
    pub use crate::drum_grid::DrumGrid;

    // Envelopes and waveforms
    pub use crate::envelope::Envelope;
    pub use crate::filter_envelope::FilterEnvelope;
    pub use crate::fm_synthesis::FMParams;
    pub use crate::waveform::Waveform;

    // Sequences
    pub use crate::sequences::{
        euclidean, euclidean_pattern, harmonic_series,
        golden_ratio, golden_ratio_rhythm, golden_sections,
    };

    // LFO
    pub use crate::lfo::{LFO, ModRoute, ModTarget};

    // Microtonal
    pub use crate::microtonal::{
        Edo, EDO12, EDO19, EDO24, EDO31, EDO53,
        cents_to_ratio, ratio_to_cents, freq_from_cents,
        just_ratio, just_scale, just_major_scale, just_minor_scale,
        pythagorean_scale,
        quarter_sharp, quarter_flat, half_sharp, half_flat,
    };
}
