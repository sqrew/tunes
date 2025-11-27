//! MIDI support for Tunes
//!
//! This module provides MIDI functionality including:
//! - **File I/O**: Import and export Standard MIDI Files (SMF)
//! - **Conversion**: Convert between MIDI values and Tunes internal representations
//!
//! # Module Structure
//!
//! - [`convert`]: Utility functions for MIDI conversions (notes, drums, timing, etc.)
//! - [`file`]: MIDI file import/export via [`Mixer::import_midi`] and [`Mixer::export_midi`]
//!
//! # Examples
//!
//! ## Export a composition to MIDI
//! ```no_run
//! # use tunes::prelude::*;
//! # fn main() -> anyhow::Result<()> {
//! let mut comp = Composition::new(Tempo::new(120.0));
//! comp.track("melody").notes(&[C4, E4, G4], 0.5);
//!
//! let mixer = comp.into_mixer();
//! mixer.export_midi("song.mid")?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Import a MIDI file
//! ```no_run
//! # use tunes::prelude::*;
//! # fn main() -> anyhow::Result<()> {
//! let mixer = Mixer::import_midi("song.mid")?;
//!
//! // Play it
//! let engine = AudioEngine::new()?;
//! engine.play_mixer(&mixer)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Convert between MIDI and frequency
//! ```
//! use tunes::midi::{frequency_to_midi_note, midi_note_to_frequency};
//!
//! // A4 = 440 Hz = MIDI note 69
//! assert_eq!(frequency_to_midi_note(440.0), 69);
//! assert_eq!(midi_note_to_frequency(69), 440.0);
//! ```

pub mod convert;
pub mod file;

// Re-export commonly used items at the module level
pub use convert::{
    drum_type_to_midi_note, frequency_to_midi_note, gm_program_to_instrument,
    midi_note_to_drum_type, midi_note_to_frequency, mod_value_to_cc,
    pitch_bend_to_semitones_from_signed, seconds_to_ticks, semitones_to_pitch_bend,
    ticks_to_seconds, velocity_to_volume, volume_to_velocity, TempoMap, DEFAULT_VELOCITY, PPQ,
};
