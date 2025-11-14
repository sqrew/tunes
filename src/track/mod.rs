//! Audio track and event system
//!
//! This module defines the core types for representing musical tracks and audio events.
//! It provides the building blocks for composing music programmatically.
//!
//! # Main Types
//!
//! - **`Track`** - A single audio track containing timed events (notes, drums, samples)
//! - **`Mixer`** - Combines multiple tracks for playback or export
//! - **`AudioEvent`** - Enum of different event types (notes, drums, samples, tempo changes, etc.)
//!
//! # Event Types
//!
//! - **`NoteEvent`** - Synthesized notes with pitch, duration, envelopes, and effects
//! - **`DrumEvent`** - Drum hits using built-in synthesis
//! - **`SampleEvent`** - WAV sample playback with pitch shifting
//! - **`TempoChangeEvent`** - Change tempo mid-composition
//! - **`TimeSignatureEvent`** - Change time signature (e.g., 4/4 to 3/4)
//! - **`KeySignatureEvent`** - Change key signature for MIDI export
//!
//! # Track Properties
//!
//! Tracks have several global properties that affect all events:
//! - Volume and pan (stereo positioning)
//! - Filter (low-pass, high-pass, band-pass, etc.)
//! - Effects (reverb, delay, distortion, chorus, etc.)
//! - Modulation routes (LFO modulation of parameters)
//!
//! # Effect Processing Order
//!
//! Effects are processed in order of their priority (lower priority = earlier in signal chain).
//! Use the provided constants or any u8 value (0-255) for custom ordering:
//!
//! - **`PRIORITY_FIRST`** (0) - Process first
//! - **`PRIORITY_EARLY`** (25) - Early in chain (EQ, compression)
//! - **`PRIORITY_NORMAL`** (100) - Normal position (distortion, saturation)
//! - **`PRIORITY_MODULATION`** (125) - Modulation effects (chorus, phaser, flanger)
//! - **`PRIORITY_TIME_BASED`** (150) - Time-based effects (delay)
//! - **`PRIORITY_SPATIAL`** (200) - Spatial effects (reverb)
//! - **`PRIORITY_LAST`** (255) - Process last
//!
//! # Example
//!
//! ```
//! # use tunes::prelude::*;
//! # use tunes::track::*;
//! let mut track = Track::new();
//! track.volume = 0.8;
//! track.pan = -0.3;  // Slightly left
//!
//! // Tracks are typically created through Composition
//! let mut comp = Composition::new(Tempo::new(120.0));
//! comp.track("melody")
//!     .volume(0.8)
//!     .notes(&[C4, E4, G4], 0.5);
//! ```

// Effect priority constants
// Lower priority = earlier in signal chain (0 is first, 255 is last)

/// Process effect first in the chain
pub const PRIORITY_FIRST: u8 = 0;

/// Early in chain - typically EQ and compression
pub const PRIORITY_EARLY: u8 = 25;

/// Normal position - distortion, saturation, bit crushing
pub const PRIORITY_NORMAL: u8 = 100;

/// Modulation effects - chorus, phaser, flanger, ring mod
pub const PRIORITY_MODULATION: u8 = 125;

/// Time-based effects - delay
pub const PRIORITY_TIME_BASED: u8 = 150;

/// Spatial effects - reverb (usually last)
pub const PRIORITY_SPATIAL: u8 = 200;

/// Process effect last in the chain
pub const PRIORITY_LAST: u8 = 255;

// Module declarations
mod events;
mod track;
mod bus;
mod mixer;
mod export;
pub mod ids;

// Re-export public types
pub use events::*;
pub use track::Track;
pub use bus::{Bus, BusBuilder};
pub use mixer::Mixer;
pub use ids::{BusId, TrackId, BusIdGenerator, TrackIdGenerator};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::notes::*;
    use crate::composition::drums::DrumType;
    use crate::composition::timing::Tempo;

    #[test]
    fn test_note_event_construction() {
        let freqs = [440.0, 554.37]; // A4 and C#5
        let note = NoteEvent::new(&freqs, 0.0, 1.0);

        assert_eq!(note.start_time, 0.0);
        assert_eq!(note.duration, 1.0);
        assert_eq!(note.num_freqs, 2);
        assert_eq!(note.frequencies[0], 440.0);
        assert_eq!(note.frequencies[1], 554.37);
        assert_eq!(note.waveform, crate::synthesis::waveform::Waveform::Sine);
        assert_eq!(note.pitch_bend_semitones, 0.0);
    }

    #[test]
    fn test_note_event_truncates_frequencies() {
        // Test that more than 8 frequencies are truncated
        let freqs = [
            100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0,
        ];
        let note = NoteEvent::new(&freqs, 0.0, 1.0);

        assert_eq!(note.num_freqs, 8, "Should truncate to max 8 frequencies");
        assert_eq!(
            note.frequencies[7], 800.0,
            "Should include first 8 frequencies"
        );
    }

    #[test]
    fn test_note_event_empty_frequencies() {
        // Test handling of empty frequency array
        let note = NoteEvent::new(&[], 0.0, 1.0);

        assert_eq!(note.num_freqs, 0);
        assert_eq!(note.frequencies[0], 0.0);
    }

    #[test]
    fn test_track_creation() {
        let track = Track::new();

        assert_eq!(track.events.len(), 0);
        assert_eq!(track.volume, 1.0);
        assert_eq!(track.pan, 0.0);
        assert!(track.effects.delay.is_none());
        assert!(track.effects.reverb.is_none());
    }

    #[test]
    fn test_track_add_note() {
        let mut track = Track::new();
        track.add_note(&[440.0], 0.0, 1.0);
        track.add_note(&[880.0], 1.0, 0.5);

        assert_eq!(track.events.len(), 2);

        match &track.events[0] {
            AudioEvent::Note(note) => {
                assert_eq!(note.frequencies[0], 440.0);
                assert_eq!(note.start_time, 0.0);
                assert_eq!(note.duration, 1.0);
            }
            _ => panic!("Expected NoteEvent"),
        }
    }

    #[test]
    fn test_track_add_drum() {
        let mut track = Track::new();
        track.add_drum(DrumType::Kick, 0.0, None);
        track.add_drum(DrumType::Snare, 0.5, None);

        assert_eq!(track.events.len(), 2);

        match &track.events[0] {
            AudioEvent::Drum(drum) => {
                assert!(matches!(drum.drum_type, DrumType::Kick));
                assert_eq!(drum.start_time, 0.0);
            }
            _ => panic!("Expected DrumEvent"),
        }
    }

    #[test]
    fn test_track_total_duration_notes() {
        let mut track = Track::new();
        track.add_note(&[440.0], 0.0, 1.0); // Ends at 1.0
        track.add_note(&[880.0], 1.5, 2.0); // Ends at 3.5

        assert_eq!(track.total_duration(), 3.5);
    }

    #[test]
    fn test_track_total_duration_drums() {
        let mut track = Track::new();
        track.add_drum(DrumType::Kick, 0.0, None); // Duration 0.15
        track.add_drum(DrumType::Crash, 1.0, None); // Duration 1.5, ends at 2.5

        let duration = track.total_duration();
        assert_eq!(duration, 2.5, "Should account for drum durations");
    }

    #[test]
    fn test_track_total_duration_empty() {
        let track = Track::new();
        assert_eq!(track.total_duration(), 0.0);
    }

    #[test]
    fn test_track_with_volume() {
        let track = Track::new().with_volume(0.5);
        assert_eq!(track.volume, 0.5);

        // Test clamping
        let loud_track = Track::new().with_volume(5.0);
        assert_eq!(loud_track.volume, 2.0, "Volume should be clamped to 2.0");

        let silent_track = Track::new().with_volume(-1.0);
        assert_eq!(silent_track.volume, 0.0, "Volume should be clamped to 0.0");
    }

    #[test]
    fn test_track_add_sequence() {
        let mut track = Track::new();
        let c4_slice: &[f32] = &[C4];
        let e4_slice: &[f32] = &[E4];
        let g4_slice: &[f32] = &[G4];
        let notes = vec![c4_slice, e4_slice, g4_slice]; // C major chord
        track.add_sequence(notes, 0.0, 0.5);

        assert_eq!(track.events.len(), 3);

        // Verify timing
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.start_time, 0.5);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            assert_eq!(note.start_time, 1.0);
        }
    }

    #[test]
    fn test_mixer_creation() {
        let mixer = Mixer::new(Tempo::new(120.0));
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_mixer_add_track() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let mut track1 = Track::new();
        track1.add_note(&[440.0], 0.0, 1.0);

        let mut track2 = Track::new();
        track2.add_drum(DrumType::Kick, 0.0, None);

        mixer.add_track(track1);
        mixer.add_track(track2);

        assert_eq!(mixer.tracks().len(), 2);
    }

    #[test]
    fn test_mixer_total_duration() {
        let mut mixer = Mixer::new(Tempo::new(120.0));

        let mut track1 = Track::new();
        track1.add_note(&[440.0], 0.0, 2.0); // Ends at 2.0

        let mut track2 = Track::new();
        track2.add_note(&[880.0], 1.0, 3.0); // Ends at 4.0

        mixer.add_track(track1);
        mixer.add_track(track2);

        assert_eq!(
            mixer.total_duration(),
            4.0,
            "Should return longest track duration"
        );
    }

    #[test]
    fn test_mixer_total_duration_empty() {
        let mixer = Mixer::new(Tempo::new(120.0));
        assert_eq!(mixer.total_duration(), 0.0);
    }

    #[test]
    fn test_mixer_repeat() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let mut track = Track::new();
        track.add_note(&[440.0], 0.0, 1.0); // Ends at 1.0
        mixer.add_track(track);

        let repeated_mixer = mixer.repeat(2); // Repeat 2 MORE times

        // Should still have 1 track (repeats events within the track)
        assert_eq!(repeated_mixer.tracks().len(), 1);

        // Track should now have 3 events total (original + 2 repeats)
        assert_eq!(repeated_mixer.tracks()[0].events.len(), 3);

        // Verify timing offsets
        if let AudioEvent::Note(note) = &repeated_mixer.tracks()[0].events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &repeated_mixer.tracks()[0].events[1] {
            assert_eq!(note.start_time, 1.0); // Original duration offset
        }
        if let AudioEvent::Note(note) = &repeated_mixer.tracks()[0].events[2] {
            assert_eq!(note.start_time, 2.0); // 2x original duration
        }
    }

    #[test]
    fn test_mixer_repeat_zero_times() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let mut track = Track::new();
        track.add_note(&[440.0], 0.0, 1.0);
        mixer.add_track(track);

        let repeated_mixer = mixer.repeat(0);

        // Repeating 0 times returns the mixer unchanged
        assert_eq!(repeated_mixer.tracks().len(), 1);
        assert_eq!(repeated_mixer.tracks()[0].events.len(), 1);
    }

    #[test]
    fn test_drum_event_construction() {
        let drum = DrumEvent {
            drum_type: DrumType::Snare,
            start_time: 0.5,
            spatial_position: None,
        };

        assert!(matches!(drum.drum_type, DrumType::Snare));
        assert_eq!(drum.start_time, 0.5);
    }

    #[test]
    fn test_audio_event_enum() {
        let note = NoteEvent::new(&[440.0], 0.0, 1.0);
        let note_event = AudioEvent::Note(note);

        match note_event {
            AudioEvent::Note(n) => assert_eq!(n.frequencies[0], 440.0),
            _ => panic!("Expected Note variant"),
        }

        let drum_event = AudioEvent::Drum(DrumEvent {
            drum_type: DrumType::Kick,
            start_time: 0.0,
            spatial_position: None,
        });

        match drum_event {
            AudioEvent::Drum(d) => assert!(matches!(d.drum_type, DrumType::Kick)),
            _ => panic!("Expected Drum variant"),
        }
    }
}
