/// MIDI export functionality
///
/// Converts compositions to Standard MIDI Files (SMF).
/// Supports notes, drums, tempo, but not samples or effects (MIDI limitations).

use crate::drums::DrumType;
use crate::rhythm::Tempo;
use crate::track::{AudioEvent, Mixer};
use anyhow::{Context, Result};
use midly::{
    num::{u15, u24, u28, u4, u7},
    Header, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind,
};
use std::fs::File;
use std::io::Write;

/// MIDI ticks per quarter note (standard resolution)
const PPQ: u16 = 480;

/// Default MIDI velocity for notes without explicit velocity
const DEFAULT_VELOCITY: u8 = 80;

/// Convert frequency (Hz) to MIDI note number
///
/// Uses equal temperament tuning: MIDI note = 69 + 12 * log2(freq / 440)
/// Returns 0-127, clamped to valid MIDI range.
///
/// # Examples
/// ```
/// # use tunes::midi::frequency_to_midi_note;
/// assert_eq!(frequency_to_midi_note(440.0), 69); // A4
/// assert_eq!(frequency_to_midi_note(261.63), 60); // C4
/// ```
pub fn frequency_to_midi_note(freq: f32) -> u8 {
    if freq <= 0.0 {
        return 0;
    }

    // MIDI note number = 69 + 12 * log2(freq / 440)
    let note = 69.0 + 12.0 * (freq / 440.0).log2();
    note.round().clamp(0.0, 127.0) as u8
}

/// Convert time in seconds to MIDI ticks
///
/// # Arguments
/// * `time` - Time in seconds
/// * `tempo` - Tempo in BPM
/// * `ppq` - Pulses per quarter note (ticks per beat)
fn seconds_to_ticks(time: f32, tempo: f32, ppq: u16) -> u32 {
    // Beats = time * (bpm / 60)
    // Ticks = beats * ppq
    let beats = time * (tempo / 60.0);
    let ticks = beats * ppq as f32;
    ticks.round() as u32
}

/// Convert DrumType to General MIDI percussion note number
///
/// General MIDI defines percussion on channel 10 with specific note numbers.
/// See: https://en.wikipedia.org/wiki/General_MIDI#Percussion
pub fn drum_type_to_midi_note(drum_type: DrumType) -> u8 {
    match drum_type {
        // Kick drums
        DrumType::Kick => 36,        // Bass Drum 1
        DrumType::Kick808 => 35,     // Acoustic Bass Drum
        DrumType::SubKick => 35,     // Acoustic Bass Drum

        // Snare drums
        DrumType::Snare => 38,       // Acoustic Snare
        DrumType::Snare808 => 40,    // Electric Snare

        // Hi-hats
        DrumType::HiHatClosed => 42,      // Closed Hi-Hat
        DrumType::HiHat808Closed => 42,   // Closed Hi-Hat
        DrumType::HiHatOpen => 46,        // Open Hi-Hat
        DrumType::HiHat808Open => 46,     // Open Hi-Hat

        // Claps
        DrumType::Clap => 39,        // Hand Clap
        DrumType::Clap808 => 39,     // Hand Clap

        // Toms
        DrumType::Tom => 47,         // Low-Mid Tom
        DrumType::TomHigh => 50,     // High Tom
        DrumType::TomLow => 45,      // Low Tom

        // Percussion
        DrumType::Rimshot => 37,     // Side Stick
        DrumType::Cowbell => 56,     // Cowbell

        // Cymbals
        DrumType::Crash => 49,       // Crash Cymbal 1
        DrumType::Ride => 51,        // Ride Cymbal 1
        DrumType::China => 52,       // Chinese Cymbal
        DrumType::Splash => 55,      // Splash Cymbal

        // Shakers/Percussion
        DrumType::Tambourine => 54,  // Tambourine
        DrumType::Shaker => 70,      // Maracas

        // Special effects (map to toms as fallback)
        DrumType::BassDrop => 35,    // Acoustic Bass Drum
        DrumType::Boom => 35,        // Acoustic Bass Drum
    }
}

/// Convert volume (0.0-1.0) to MIDI velocity (0-127)
fn volume_to_velocity(volume: f32) -> u8 {
    (volume.clamp(0.0, 1.0) * 127.0).round() as u8
}

impl Mixer {
    /// Export the mixer to a MIDI file
    ///
    /// # Arguments
    /// * `path` - Output file path (e.g., "song.mid")
    /// * `tempo` - Tempo in BPM
    ///
    /// # Limitations
    /// MIDI export has inherent limitations compared to audio rendering:
    /// - Sample events are **ignored** (MIDI has no concept of audio samples)
    /// - Effects are **ignored** (reverb, delay, filters not in MIDI spec)
    /// - Synthesis parameters are **ignored** (MIDI doesn't specify how notes sound)
    /// - Only note pitch, velocity, and duration are exported
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody").notes(&[C4, E4, G4], 0.5);
    ///
    /// let mixer = comp.into_mixer();
    /// mixer.export_midi("song.mid", Tempo::new(120.0))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn export_midi(&self, path: &str, tempo: Tempo) -> Result<()> {
        let mut tracks = Vec::new();

        // Track 0: Tempo track (meta information)
        let mut tempo_track = Vec::new();

        // Set tempo (microseconds per quarter note)
        let bpm = tempo.bpm;
        let us_per_quarter_note = (60_000_000.0 / bpm) as u32;
        tempo_track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::Tempo(u24::new(us_per_quarter_note))),
        });

        // End of track
        tempo_track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });

        tracks.push(tempo_track);

        // Convert each audio track to a MIDI track
        for (track_idx, track) in self.tracks.iter().enumerate() {
            let mut midi_track = Vec::new();
            let mut events = Vec::new();

            // Track name (using static string to avoid lifetime issues)
            midi_track.push(TrackEvent {
                delta: u28::new(0),
                kind: TrackEventKind::Meta(MetaMessage::TrackName(b"Track")),
            });

            // Convert track events to MIDI events
            for event in &track.events {
                match event {
                    AudioEvent::Note(note) => {
                        let start_tick = seconds_to_ticks(note.start_time, bpm, PPQ);
                        let end_tick = seconds_to_ticks(note.start_time + note.duration, bpm, PPQ);
                        let velocity = volume_to_velocity(track.volume);

                        // Add a note on/off event for each frequency in the chord
                        for i in 0..note.num_freqs {
                            let freq = note.frequencies[i];
                            let midi_note = frequency_to_midi_note(freq);

                            events.push((start_tick, true, midi_note, velocity));
                            events.push((end_tick, false, midi_note, 0));
                        }
                    }
                    AudioEvent::Drum(drum) => {
                        let tick = seconds_to_ticks(drum.start_time, bpm, PPQ);
                        let midi_note = drum_type_to_midi_note(drum.drum_type);
                        let velocity = DEFAULT_VELOCITY;

                        // Drum note on (channel 10 = percussion)
                        events.push((tick, true, midi_note, velocity));
                        // Drum note off shortly after (10 ticks = ~20ms at 480 PPQ, 120 BPM)
                        events.push((tick + 10, false, midi_note, 0));
                    }
                    AudioEvent::Sample(_) => {
                        // Samples cannot be represented in MIDI - skip silently
                        // Could add a warning here if desired
                    }
                }
            }

            // Sort events by time
            events.sort_by_key(|e| e.0);

            // Convert to delta-time format
            let mut last_tick = 0u32;
            for (tick, is_note_on, note, velocity) in events {
                let delta = tick.saturating_sub(last_tick);
                last_tick = tick;

                let channel = if matches!(
                    track.events.first(),
                    Some(AudioEvent::Drum(_))
                ) {
                    u4::new(9) // Channel 10 (0-indexed as 9) for drums
                } else {
                    u4::new(0) // Channel 1 for melodic instruments
                };

                let message = if is_note_on {
                    MidiMessage::NoteOn {
                        key: u7::new(note),
                        vel: u7::new(velocity),
                    }
                } else {
                    MidiMessage::NoteOff {
                        key: u7::new(note),
                        vel: u7::new(0),
                    }
                };

                midi_track.push(TrackEvent {
                    delta: u28::new(delta),
                    kind: TrackEventKind::Midi { channel, message },
                });
            }

            // End of track
            midi_track.push(TrackEvent {
                delta: u28::new(0),
                kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
            });

            tracks.push(midi_track);
        }

        // Create SMF
        let header = Header {
            format: midly::Format::Parallel, // Type 1: Multiple tracks
            timing: Timing::Metrical(u15::new(PPQ)),
        };

        let smf = Smf {
            header,
            tracks,
        };

        // Write to file
        let mut file = File::create(path)
            .with_context(|| format!("Failed to create MIDI file: {}", path))?;

        smf.write_std(&mut file)
            .with_context(|| format!("Failed to write MIDI data to: {}", path))?;

        file.flush()
            .with_context(|| format!("Failed to flush MIDI file: {}", path))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_to_midi_note() {
        assert_eq!(frequency_to_midi_note(440.0), 69); // A4
        assert_eq!(frequency_to_midi_note(261.63), 60); // C4 (approximate)
        assert_eq!(frequency_to_midi_note(523.25), 72); // C5 (approximate)

        // Edge cases
        assert_eq!(frequency_to_midi_note(0.0), 0);
        assert_eq!(frequency_to_midi_note(-100.0), 0);
        assert_eq!(frequency_to_midi_note(20000.0), 127); // Clamps to max
    }

    #[test]
    fn test_seconds_to_ticks() {
        // At 120 BPM, 1 beat = 0.5 seconds
        // At 480 PPQ, 1 beat = 480 ticks
        // So 0.5 seconds = 480 ticks
        assert_eq!(seconds_to_ticks(0.5, 120.0, 480), 480);

        // 1 second = 2 beats = 960 ticks
        assert_eq!(seconds_to_ticks(1.0, 120.0, 480), 960);

        // At 60 BPM, 1 beat = 1 second = 480 ticks
        assert_eq!(seconds_to_ticks(1.0, 60.0, 480), 480);
    }

    #[test]
    fn test_drum_type_to_midi_note() {
        // Test a few standard mappings
        assert_eq!(drum_type_to_midi_note(DrumType::Kick), 36);
        assert_eq!(drum_type_to_midi_note(DrumType::Snare), 38);
        assert_eq!(drum_type_to_midi_note(DrumType::HiHatClosed), 42);
        assert_eq!(drum_type_to_midi_note(DrumType::HiHatOpen), 46);
        assert_eq!(drum_type_to_midi_note(DrumType::Clap), 39);
    }

    #[test]
    fn test_volume_to_velocity() {
        assert_eq!(volume_to_velocity(0.0), 0);
        assert_eq!(volume_to_velocity(1.0), 127);
        assert_eq!(volume_to_velocity(0.5), 64); // Approximate
        assert_eq!(volume_to_velocity(1.5), 127); // Clamps
        assert_eq!(volume_to_velocity(-0.5), 0); // Clamps
    }
}
