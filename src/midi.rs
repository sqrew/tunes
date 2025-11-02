/// MIDI export functionality
///
/// Converts compositions to Standard MIDI Files (SMF).
/// Supports notes, drums, tempo, but not samples or effects (MIDI limitations).

use crate::drums::DrumType;
use crate::error::{Result, TunesError};
use crate::rhythm::Tempo;
use crate::track::{AudioEvent, Mixer};
use midly::{
    num::{u14, u15, u24, u28, u4, u7},
    Header, MetaMessage, MidiMessage, PitchBend, Smf, Timing, TrackEvent, TrackEventKind,
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

/// Convert pitch bend in semitones to MIDI pitch bend value (14-bit)
///
/// MIDI pitch bend is a 14-bit value (0-16383) with center at 8192.
/// Standard pitch bend range is ±2 semitones.
///
/// # Arguments
/// * `semitones` - Pitch bend amount in semitones (positive = up, negative = down)
/// * `range` - Pitch bend range in semitones (default is 2.0 for ±2 semitones)
///
/// # Returns
/// 14-bit pitch bend value (0-16383), clamped to valid range
fn semitones_to_pitch_bend(semitones: f32, range: f32) -> u16 {
    // Center value is 8192 (no bend)
    // Each semitone within the range corresponds to ±8192/range units
    let bend_value = 8192.0 + (semitones / range) * 8192.0;
    bend_value.round().clamp(0.0, 16383.0) as u16
}

/// Convert a modulation LFO value to MIDI CC value (0-127)
///
/// For unipolar modulation (volume): 0.0 -> 0, 1.0 -> 127
/// For bipolar modulation (pitch, pan): -1.0 -> 0, 0.0 -> 64, 1.0 -> 127
fn mod_value_to_cc(value: f32, bipolar: bool) -> u8 {
    if bipolar {
        // Bipolar: -1.0 to 1.0 -> 0 to 127
        ((value + 1.0) * 63.5).round().clamp(0.0, 127.0) as u8
    } else {
        // Unipolar: 0.0 to 1.0 -> 0 to 127
        (value * 127.0).round().clamp(0.0, 127.0) as u8
    }
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
    /// - Per-note velocity and track volume are combined for MIDI velocity export
    /// - Only note pitch, velocity, duration, and timing are exported
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

        // Keep initial BPM for time-to-tick conversions
        let bpm = tempo.bpm;

        // Track 0: Tempo track (meta information)
        let mut tempo_track = Vec::new();

        // Collect all tempo changes from all tracks
        let mut tempo_changes = Vec::new();

        // Add initial tempo
        tempo_changes.push((0.0, bpm));

        // Collect tempo changes from all tracks
        for track in &self.tracks {
            for event in &track.events {
                if let AudioEvent::TempoChange(tempo_event) = event {
                    tempo_changes.push((tempo_event.start_time, tempo_event.bpm));
                }
            }
        }

        // Sort by time and remove duplicates at same time (keep last)
        tempo_changes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        tempo_changes.dedup_by(|a, b| (a.0 - b.0).abs() < 0.001);

        // Collect all time signature changes from all tracks
        let mut time_sig_changes: Vec<(f32, u8, u8)> = Vec::new();

        // Add default time signature (4/4) at the start
        time_sig_changes.push((0.0, 4, 4));

        // Collect time signature changes from all tracks
        for track in &self.tracks {
            for event in &track.events {
                if let AudioEvent::TimeSignature(time_sig_event) = event {
                    time_sig_changes.push((
                        time_sig_event.start_time,
                        time_sig_event.numerator,
                        time_sig_event.denominator,
                    ));
                }
            }
        }

        // Sort by time and remove duplicates at same time (keep last)
        time_sig_changes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        time_sig_changes.dedup_by(|a, b| (a.0 - b.0).abs() < 0.001);

        // Combine tempo and time signature changes into a single sorted list
        // We'll use an enum to distinguish between the two types
        #[derive(Debug, Clone, Copy)]
        enum MetaChange {
            Tempo(f32, f32),      // (time, bpm)
            TimeSignature(f32, u8, u8), // (time, numerator, denominator)
        }

        let mut meta_changes: Vec<MetaChange> = Vec::new();

        // Add all tempo changes
        for (time, tempo_bpm) in tempo_changes {
            meta_changes.push(MetaChange::Tempo(time, tempo_bpm));
        }

        // Add all time signature changes
        for (time, numerator, denominator) in time_sig_changes {
            meta_changes.push(MetaChange::TimeSignature(time, numerator, denominator));
        }

        // Sort by time
        meta_changes.sort_by(|a, b| {
            let time_a = match a {
                MetaChange::Tempo(t, _) => *t,
                MetaChange::TimeSignature(t, _, _) => *t,
            };
            let time_b = match b {
                MetaChange::Tempo(t, _) => *t,
                MetaChange::TimeSignature(t, _, _) => *t,
            };
            time_a.partial_cmp(&time_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Convert to MIDI events with delta times
        let mut last_tick = 0u32;
        for meta_change in meta_changes {
            match meta_change {
                MetaChange::Tempo(time, tempo_bpm) => {
                    let tick = seconds_to_ticks(time, bpm, PPQ);
                    let delta = tick.saturating_sub(last_tick);
                    last_tick = tick;

                    let us_per_quarter_note = (60_000_000.0 / tempo_bpm) as u32;
                    tempo_track.push(TrackEvent {
                        delta: u28::new(delta),
                        kind: TrackEventKind::Meta(MetaMessage::Tempo(u24::new(us_per_quarter_note))),
                    });
                }
                MetaChange::TimeSignature(time, numerator, denominator) => {
                    let tick = seconds_to_ticks(time, bpm, PPQ);
                    let delta = tick.saturating_sub(last_tick);
                    last_tick = tick;

                    // Convert denominator to MIDI format (log2)
                    // 2 -> 1, 4 -> 2, 8 -> 3, 16 -> 4, etc.
                    let denominator_midi = match denominator {
                        2 => 1,
                        4 => 2,
                        8 => 3,
                        16 => 4,
                        32 => 5,
                        _ => 2, // Default to 4 if invalid
                    };

                    tempo_track.push(TrackEvent {
                        delta: u28::new(delta),
                        kind: TrackEventKind::Meta(MetaMessage::TimeSignature(
                            numerator,
                            denominator_midi,
                            24, // MIDI clocks per metronome click (standard)
                            8,  // 32nd notes per quarter note (standard)
                        )),
                    });
                }
            }
        }

        // End of track
        tempo_track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });

        tracks.push(tempo_track);

        // Convert each audio track to a MIDI track
        for (_track_idx, track) in self.tracks.iter().enumerate() {
            let mut midi_track = Vec::new();
            let mut events = Vec::new();

            // Track name from actual track name
            let track_name_bytes = track.name.as_deref().unwrap_or("Track").as_bytes();
            midi_track.push(TrackEvent {
                delta: u28::new(0),
                kind: TrackEventKind::Meta(MetaMessage::TrackName(track_name_bytes)),
            });

            // Determine channel (drums on channel 10, melodic on channel 0)
            let channel = if matches!(track.events.first(), Some(AudioEvent::Drum(_))) {
                u4::new(9) // Channel 10 (0-indexed as 9) for drums
            } else {
                u4::new(0) // Channel 1 for melodic instruments (we'll improve this later)
            };

            // Add program change if specified
            if let Some(program) = track.midi_program {
                midi_track.push(TrackEvent {
                    delta: u28::new(0),
                    kind: TrackEventKind::Midi {
                        channel,
                        message: MidiMessage::ProgramChange {
                            program: u7::new(program),
                        },
                    },
                });
            }

            // Add CC for volume (CC7)
            if track.volume != 1.0 {
                let volume_cc = volume_to_velocity(track.volume);
                midi_track.push(TrackEvent {
                    delta: u28::new(0),
                    kind: TrackEventKind::Midi {
                        channel,
                        message: MidiMessage::Controller {
                            controller: u7::new(7),  // Volume CC
                            value: u7::new(volume_cc),
                        },
                    },
                });
            }

            // Add CC for pan (CC10)
            if track.pan != 0.0 {
                // Convert pan from -1.0..1.0 to MIDI 0..127 (64 = center)
                let pan_midi = ((track.pan + 1.0) * 63.5).round().clamp(0.0, 127.0) as u8;
                midi_track.push(TrackEvent {
                    delta: u28::new(0),
                    kind: TrackEventKind::Midi {
                        channel,
                        message: MidiMessage::Controller {
                            controller: u7::new(10),  // Pan CC
                            value: u7::new(pan_midi),
                        },
                    },
                });
            }

            // Internal enum for MIDI events during processing
            #[derive(Debug, Clone, Copy)]
            enum MidiEventType {
                NoteOn { note: u8, velocity: u8 },
                NoteOff { note: u8 },
                PitchBend { value: u16 },
                ControlChange { controller: u8, value: u8 },
            }

            // Convert track events to MIDI events
            for event in &track.events {
                match event {
                    AudioEvent::Note(note) => {
                        let start_tick = seconds_to_ticks(note.start_time, bpm, PPQ);
                        let end_tick = seconds_to_ticks(note.start_time + note.duration, bpm, PPQ);
                        // Combine per-note velocity with track volume for final MIDI velocity
                        let combined_velocity = (note.velocity * track.volume).clamp(0.0, 1.0);
                        let velocity = volume_to_velocity(combined_velocity);

                        // Add pitch bend event if needed (before the notes)
                        if note.pitch_bend_semitones != 0.0 {
                            let pitch_bend_value = semitones_to_pitch_bend(note.pitch_bend_semitones, 2.0);
                            events.push((start_tick, MidiEventType::PitchBend { value: pitch_bend_value }));
                        }

                        // Add a note on/off event for each frequency in the chord
                        for i in 0..note.num_freqs {
                            let freq = note.frequencies[i];
                            let midi_note = frequency_to_midi_note(freq);

                            events.push((start_tick, MidiEventType::NoteOn { note: midi_note, velocity }));
                            events.push((end_tick, MidiEventType::NoteOff { note: midi_note }));
                        }

                        // Reset pitch bend to center after the note ends
                        if note.pitch_bend_semitones != 0.0 {
                            events.push((end_tick, MidiEventType::PitchBend { value: 8192 }));
                        }
                    }
                    AudioEvent::Drum(drum) => {
                        let tick = seconds_to_ticks(drum.start_time, bpm, PPQ);
                        let midi_note = drum_type_to_midi_note(drum.drum_type);
                        let velocity = DEFAULT_VELOCITY;

                        // Drum note on (channel 10 = percussion)
                        events.push((tick, MidiEventType::NoteOn { note: midi_note, velocity }));
                        // Drum note off shortly after (10 ticks = ~20ms at 480 PPQ, 120 BPM)
                        events.push((tick + 10, MidiEventType::NoteOff { note: midi_note }));
                    }
                    AudioEvent::Sample(_) => {
                        // Samples cannot be represented in MIDI - skip silently
                        // Could add a warning here if desired
                    }
                    AudioEvent::TempoChange(_) => {
                        // Tempo changes will be handled separately
                        // (Added to track-level tempo changes, not event-level)
                    }
                    AudioEvent::TimeSignature(_) => {
                        // Time signatures will be handled separately
                        // (Added to tempo track with time signature meta messages)
                    }
                    AudioEvent::KeySignature(_) => {
                        // Key signatures will be handled separately
                        // (Added to tempo track with key signature meta messages)
                    }
                }
            }

            // Sample LFO modulation and add CC automation events
            // Only export modulation that translates well to MIDI (Pitch, Volume, Pan)
            if !track.modulation.is_empty() {
                // Determine track duration
                let track_duration = track.total_duration();

                if track_duration > 0.0 {
                    // Sample interval: every 1/32 note or 50ms, whichever is more frequent
                    let beats_per_second = bpm / 60.0;
                    let seconds_per_32nd = 1.0 / (beats_per_second * 8.0);
                    let sample_interval = seconds_per_32nd.min(0.05); // Min of 1/32 note or 50ms

                    // Generate sample times
                    let num_samples = (track_duration / sample_interval).ceil() as usize;

                    for mod_route in &track.modulation {
                        // Only export modulation that maps to standard MIDI CCs
                        let (cc_number, bipolar) = match mod_route.target {
                            crate::lfo::ModTarget::Pitch => (1, true),   // CC1: Modulation Wheel
                            crate::lfo::ModTarget::Volume => (11, false), // CC11: Expression
                            crate::lfo::ModTarget::Pan => (10, true),    // CC10: Pan
                            _ => continue, // Skip filter parameters (synthesis-specific)
                        };

                        // Sample the LFO at regular intervals
                        for i in 0..num_samples {
                            let time = i as f32 * sample_interval;
                            let tick = seconds_to_ticks(time, bpm, PPQ);

                            // Get LFO value
                            let lfo_value = if bipolar {
                                mod_route.lfo.bipolar_value_at(time) * mod_route.amount
                            } else {
                                mod_route.lfo.value_at(time)
                            };

                            // Convert to CC value
                            let cc_value = mod_value_to_cc(lfo_value, bipolar);

                            // Add CC event
                            events.push((tick, MidiEventType::ControlChange {
                                controller: cc_number,
                                value: cc_value,
                            }));
                        }
                    }
                }
            }

            // Sort events by time
            events.sort_by_key(|e| e.0);

            // Convert to delta-time format
            let mut last_tick = 0u32;
            for (tick, event_type) in events {
                let delta = tick.saturating_sub(last_tick);
                last_tick = tick;

                let message = match event_type {
                    MidiEventType::NoteOn { note, velocity } => {
                        MidiMessage::NoteOn {
                            key: u7::new(note),
                            vel: u7::new(velocity),
                        }
                    }
                    MidiEventType::NoteOff { note } => {
                        MidiMessage::NoteOff {
                            key: u7::new(note),
                            vel: u7::new(0),
                        }
                    }
                    MidiEventType::PitchBend { value } => {
                        MidiMessage::PitchBend {
                            bend: PitchBend(u14::new(value)),
                        }
                    }
                    MidiEventType::ControlChange { controller, value } => {
                        MidiMessage::Controller {
                            controller: u7::new(controller),
                            value: u7::new(value),
                        }
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
            .map_err(|e| TunesError::MidiError(format!("Failed to create MIDI file {}: {}", path, e)))?;

        smf.write_std(&mut file)
            .map_err(|e| TunesError::MidiError(format!("Failed to write MIDI data to {}: {}", path, e)))?;

        file.flush()
            .map_err(|e| TunesError::MidiError(format!("Failed to flush MIDI file {}: {}", path, e)))?;

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

    #[test]
    fn test_semitones_to_pitch_bend() {
        // Center (no bend) should be 8192
        assert_eq!(semitones_to_pitch_bend(0.0, 2.0), 8192);

        // +2 semitones (max of standard range) should be 16383
        assert_eq!(semitones_to_pitch_bend(2.0, 2.0), 16383);

        // -2 semitones (min of standard range) should be 0
        assert_eq!(semitones_to_pitch_bend(-2.0, 2.0), 0);

        // +1 semitone (half of range) should be halfway between center and max
        assert_eq!(semitones_to_pitch_bend(1.0, 2.0), 12288);

        // -1 semitone should be halfway between center and min
        assert_eq!(semitones_to_pitch_bend(-1.0, 2.0), 4096);

        // Test clamping - values beyond range should clamp
        assert_eq!(semitones_to_pitch_bend(10.0, 2.0), 16383); // Clamps to max
        assert_eq!(semitones_to_pitch_bend(-10.0, 2.0), 0);    // Clamps to min
    }

    #[test]
    fn test_semitones_to_pitch_bend_different_range() {
        // Test with 12 semitone range (full octave)
        assert_eq!(semitones_to_pitch_bend(0.0, 12.0), 8192);
        assert_eq!(semitones_to_pitch_bend(12.0, 12.0), 16383);
        assert_eq!(semitones_to_pitch_bend(-12.0, 12.0), 0);
        assert_eq!(semitones_to_pitch_bend(6.0, 12.0), 12288);
    }

    #[test]
    fn test_pitch_bend_fractional_semitones() {
        // Test fractional semitones (for microtonal bends)
        let bend_quarter_tone = semitones_to_pitch_bend(0.5, 2.0);
        // Should be between center (8192) and +1 semitone (12288)
        assert!(bend_quarter_tone > 8192 && bend_quarter_tone < 12288);
        assert_eq!(bend_quarter_tone, 10240); // Exactly halfway

        let bend_eighth_tone = semitones_to_pitch_bend(0.25, 2.0);
        // Should be between center and quarter tone
        assert!(bend_eighth_tone > 8192 && bend_eighth_tone < bend_quarter_tone);
    }

    #[test]
    fn test_mod_value_to_cc_unipolar() {
        // Unipolar modulation (volume): 0.0 -> 0, 1.0 -> 127
        assert_eq!(mod_value_to_cc(0.0, false), 0);
        assert_eq!(mod_value_to_cc(1.0, false), 127);
        assert_eq!(mod_value_to_cc(0.5, false), 64);

        // Test clamping
        assert_eq!(mod_value_to_cc(-0.5, false), 0);
        assert_eq!(mod_value_to_cc(1.5, false), 127);
    }

    #[test]
    fn test_mod_value_to_cc_bipolar() {
        // Bipolar modulation (pitch, pan): -1.0 -> 0, 0.0 -> 64, 1.0 -> 127
        assert_eq!(mod_value_to_cc(-1.0, true), 0);
        assert_eq!(mod_value_to_cc(0.0, true), 64);
        assert_eq!(mod_value_to_cc(1.0, true), 127);

        // Test intermediate values
        assert_eq!(mod_value_to_cc(0.5, true), 95); // Halfway between 64 and 127
        assert_eq!(mod_value_to_cc(-0.5, true), 32); // Halfway between 0 and 64

        // Test clamping
        assert_eq!(mod_value_to_cc(-2.0, true), 0);
        assert_eq!(mod_value_to_cc(2.0, true), 127);
    }
}
