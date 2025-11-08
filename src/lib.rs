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

pub mod composition;
pub mod consts;
pub mod engine;
pub mod error;
pub mod instruments;
pub mod live_coding;
pub mod midi;
pub mod sequences;
pub mod synthesis;
pub mod theory;
pub mod track;

/// Prelude module for convenient imports
pub mod prelude {
    // Core composition
    pub use crate::composition::{Composition, DrumGrid, DrumType, Tempo};
    pub use crate::engine::{AudioEngine, SoundId};
    pub use crate::track::Mixer;

    // Error handling
    pub use crate::error::{Result, TunesError};

    // Notes, Scales, and Chords
    pub use crate::consts::*;

    // Theory
    pub use crate::theory::{
        ChordPattern, ProgressionType, ScalePattern, chord, progression, scale, transpose,
        transpose_sequence, KeyMode, KeyRoot, KeySignature,
    };

    // Instruments
    pub use crate::instruments::Instrument;

    // Effects and filters
    pub use crate::synthesis::{Filter, FilterType};
    pub use crate::synthesis::effects::*;

    // Advanced synthesis
    pub use crate::synthesis::{Envelope, FMParams, FilterEnvelope, GranularParams, NoiseType, Waveform, Wavetable, KarplusStrong};

    // Noise generators
    pub use crate::synthesis::{NoiseGenerator, WhiteNoise, BrownNoise, PinkNoise, BlueNoise, GreenNoise, PerlinNoise};

    // LFO
    pub use crate::synthesis::{LFO, ModRoute, ModTarget};

    // Sequences
    pub use crate::sequences::{
        euclidean, euclidean_pattern, golden_ratio, golden_ratio_rhythm, golden_sections,
        harmonic_series,
    };

    // Automation
    pub use crate::synthesis::{Automation, Interpolation};

    // Microtonal
    pub use crate::theory::{
        EDO12, EDO19, EDO24, EDO31, EDO53, Edo, cents_to_ratio, freq_from_cents, half_flat,
        half_sharp, just_major_scale, just_minor_scale, just_ratio, just_scale, pythagorean_scale,
        quarter_flat, quarter_sharp, ratio_to_cents,
    };

    // MIDI utilities
    pub use crate::midi::{
        frequency_to_midi_note, midi_note_to_frequency, midi_note_to_drum_type,
        drum_type_to_midi_note,
    };
}
