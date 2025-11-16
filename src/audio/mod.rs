//! Real-time audio input and recording
//!
//! This module provides simple utilities for recording live audio from microphones
//! or line inputs. Recordings are saved as WAV files which can then be used with
//! the full sample playback and effects pipeline.
//!
//! # Example
//!
//! ```no_run
//! use tunes::audio::LiveInput;
//! use tunes::prelude::*;
//!
//! # fn main() -> anyhow::Result<()> {
//! // Record from microphone
//! let mut recorder = LiveInput::new()?;
//! recorder.start_recording("my_recording.wav")?;
//!
//! println!("Recording... (press Enter to stop)");
//! let mut input = String::new();
//! std::io::stdin().read_line(&mut input)?;
//!
//! recorder.stop()?;
//! println!("Saved to my_recording.wav");
//!
//! // Now use the recording with the existing pipeline
//! let mut comp = Composition::new(Tempo::new(120.0));
//! comp.load_sample("vocal", "my_recording.wav")?;
//! comp.track("vocals")
//!     .sample("vocal")
//!     .reverb(Reverb::new(0.5, 0.5, 0.3));
//!
//! comp.into_mixer().export_wav("output.wav", 44100)?;
//! # Ok(())
//! # }
//! ```

mod live_input;

pub use live_input::LiveInput;
