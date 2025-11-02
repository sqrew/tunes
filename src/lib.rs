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

pub mod automation;
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
    pub use crate::chords::*;
    pub use crate::scales::*;

    // Theory
    pub use crate::key_signature::{KeyMode, KeyRoot, KeySignature};
    pub use crate::theory::{
        ChordPattern, ProgressionType, ScalePattern, chord, progression, scale, transpose,
        transpose_sequence,
    };

    // Instruments
    pub use crate::instruments::Instrument;

    // Effects
    pub use crate::effects::*;
    pub use crate::filter::{Filter, FilterType};

    // Drums
    pub use crate::drum_grid::DrumGrid;
    pub use crate::drums::DrumType;

    // Envelopes and waveforms
    pub use crate::envelope::Envelope;
    pub use crate::filter_envelope::FilterEnvelope;
    pub use crate::fm_synthesis::FMParams;
    pub use crate::waveform::Waveform;

    // Sequences
    pub use crate::sequences::{
        euclidean, euclidean_pattern, golden_ratio, golden_ratio_rhythm, golden_sections,
        harmonic_series,
    };

    // LFO
    pub use crate::lfo::{LFO, ModRoute, ModTarget};

    // Automation
    pub use crate::automation::{Automation, Interpolation};

    // Microtonal
    pub use crate::microtonal::{
        EDO12, EDO19, EDO24, EDO31, EDO53, Edo, cents_to_ratio, freq_from_cents, half_flat,
        half_sharp, just_major_scale, just_minor_scale, just_ratio, just_scale, pythagorean_scale,
        quarter_flat, quarter_sharp, ratio_to_cents,
    };
}
