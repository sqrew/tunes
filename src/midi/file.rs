//! MIDI file import and export
//!
//! This module provides functionality to export compositions to Standard MIDI Files (SMF)
//! and import MIDI files into Mixer objects for playback or further processing.

use crate::error::{Result, TunesError};
use crate::track::{AudioEvent, Mixer};
use midly::{
    Header, MetaMessage, MidiMessage, PitchBend, Smf, Timing, TrackEvent, TrackEventKind,
    num::{u4, u7, u14, u15, u24, u28},
};
use std::fs::File;
use std::io::Write;

use super::convert::{
    drum_type_to_midi_note, frequency_to_midi_note, gm_program_to_instrument,
    midi_note_to_drum_type, midi_note_to_frequency, mod_value_to_cc,
    pitch_bend_to_semitones_from_signed, semitones_to_pitch_bend, ticks_to_seconds,
    volume_to_velocity, TempoMap, DEFAULT_VELOCITY, PPQ,
};

impl Mixer {
    /// Export the mixer to a MIDI file
    ///
    /// Uses the tempo from the composition automatically.
    ///
    /// # Arguments
    /// * `path` - Output file path (e.g., "song.mid")
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
    /// # fn main() -> anyhow::Result<()> {
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody").notes(&[C4, E4, G4], 0.5);
    ///
    /// let mixer = comp.into_mixer();
    /// mixer.export_midi("song.mid")?;  // Uses composition's tempo automatically
    /// # Ok(())
    /// # }
    /// ```
    pub fn export_midi(&self, path: &str) -> Result<()> {
        let mut tracks = Vec::new();

        // Keep initial BPM for reference
        let bpm = self.tempo.bpm;

        // Build tempo map for accurate time-to-tick conversions
        let mut tempo_map = TempoMap::new(bpm, PPQ);

        // Collect all tempo changes from all tracks
        let mut tempo_changes = Vec::new();

        // Add initial tempo
        tempo_changes.push((0.0, bpm));

        // Collect tempo changes from all tracks
        for track in self.all_tracks() {
            for event in &track.events {
                if let AudioEvent::TempoChange(tempo_event) = event {
                    tempo_changes.push((tempo_event.start_time, tempo_event.bpm));
                }
            }
        }

        // Sort by time and remove duplicates at same time (keep last)
        tempo_changes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        tempo_changes.dedup_by(|a, b| (a.0 - b.0).abs() < 0.001);

        // Add all tempo changes to the tempo map
        for (time, tempo_bpm) in &tempo_changes {
            if *time > 0.0 {
                // Skip initial tempo (already added in TempoMap::new)
                tempo_map.add_change(*time, *tempo_bpm);
            }
        }

        // Finalize the tempo map (sorts and deduplicates)
        tempo_map.finalize();

        // Track 0: Tempo track (meta information)
        let mut tempo_track = Vec::new();

        // Collect all time signature changes from all tracks
        let mut time_sig_changes: Vec<(f32, u8, u8)> = Vec::new();

        // Add default time signature (4/4) at the start
        time_sig_changes.push((0.0, 4, 4));

        // Collect time signature changes from all tracks
        for track in self.all_tracks() {
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

        // Collect all key signature changes from all tracks
        let mut key_sig_changes: Vec<(f32, crate::theory::key_signature::KeySignature)> =
            Vec::new();

        // Collect key signature changes from all tracks
        for track in self.all_tracks() {
            for event in &track.events {
                if let AudioEvent::KeySignature(key_sig_event) = event {
                    key_sig_changes
                        .push((key_sig_event.start_time, key_sig_event.key_signature));
                }
            }
        }

        // Sort by time and remove duplicates at same time (keep last)
        key_sig_changes
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        key_sig_changes.dedup_by(|a, b| (a.0 - b.0).abs() < 0.001);

        // Combine tempo, time signature, and key signature changes into a single sorted list
        // We'll use an enum to distinguish between the types
        #[derive(Debug, Clone, Copy)]
        enum MetaChange {
            Tempo(f32, f32),            // (time, bpm)
            TimeSignature(f32, u8, u8), // (time, numerator, denominator)
            KeySignature(f32, crate::theory::key_signature::KeySignature), // (time, key_signature)
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

        // Add all key signature changes
        for (time, key_signature) in key_sig_changes {
            meta_changes.push(MetaChange::KeySignature(time, key_signature));
        }

        // Sort by time
        meta_changes.sort_by(|a, b| {
            let time_a = match a {
                MetaChange::Tempo(t, _) => *t,
                MetaChange::TimeSignature(t, _, _) => *t,
                MetaChange::KeySignature(t, _) => *t,
            };
            let time_b = match b {
                MetaChange::Tempo(t, _) => *t,
                MetaChange::TimeSignature(t, _, _) => *t,
                MetaChange::KeySignature(t, _) => *t,
            };
            time_a
                .partial_cmp(&time_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Convert to MIDI events with delta times
        let mut last_tick = 0u32;
        for meta_change in meta_changes {
            match meta_change {
                MetaChange::Tempo(time, tempo_bpm) => {
                    let tick = tempo_map.seconds_to_ticks(time);
                    let delta = tick.saturating_sub(last_tick);
                    last_tick = tick;

                    let us_per_quarter_note = (60_000_000.0 / tempo_bpm) as u32;
                    tempo_track.push(TrackEvent {
                        delta: u28::new(delta),
                        kind: TrackEventKind::Meta(MetaMessage::Tempo(u24::new(
                            us_per_quarter_note,
                        ))),
                    });
                }
                MetaChange::TimeSignature(time, numerator, denominator) => {
                    let tick = tempo_map.seconds_to_ticks(time);
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
                MetaChange::KeySignature(time, key_signature) => {
                    let tick = tempo_map.seconds_to_ticks(time);
                    let delta = tick.saturating_sub(last_tick);
                    last_tick = tick;

                    // Convert key signature to MIDI format
                    // sf: -7 to +7 (negative = flats, positive = sharps)
                    // mi: false = major, true = minor
                    let sharps_flats = key_signature.to_midi_sharps_flats();
                    let is_minor = key_signature.is_minor();

                    tempo_track.push(TrackEvent {
                        delta: u28::new(delta),
                        kind: TrackEventKind::Meta(MetaMessage::KeySignature(
                            sharps_flats,
                            is_minor,
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

        // Channel allocator for melodic tracks
        // MIDI has 16 channels (0-15): channel 9 is reserved for drums
        // Available melodic channels: 0-8, 10-15 (15 channels total)
        let melodic_channels: Vec<u8> = (0..16).filter(|&ch| ch != 9).collect();
        let mut next_channel_idx = 0;

        // Convert each audio track to a MIDI track
        for track in self.all_tracks().iter() {
            let mut midi_track = Vec::new();
            let mut events = Vec::new();

            // Track name from actual track name
            let track_name_bytes = track.name.as_deref().unwrap_or("Track").as_bytes();
            midi_track.push(TrackEvent {
                delta: u28::new(0),
                kind: TrackEventKind::Meta(MetaMessage::TrackName(track_name_bytes)),
            });

            // Determine channel based on track type
            let channel = if matches!(track.events.first(), Some(AudioEvent::Drum(_))) {
                // Drums always go to channel 10 (0-indexed as 9)
                u4::new(9)
            } else {
                // Melodic tracks: allocate next available channel
                // If we run out of channels, wrap around (multiple tracks can share a channel)
                let ch = melodic_channels[next_channel_idx % melodic_channels.len()];
                next_channel_idx += 1;
                u4::new(ch)
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
                            controller: u7::new(7), // Volume CC
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
                            controller: u7::new(10), // Pan CC
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
                        let start_tick = tempo_map.seconds_to_ticks(note.start_time);
                        let end_tick = tempo_map.seconds_to_ticks(note.start_time + note.duration);
                        // Combine per-note velocity with track volume for final MIDI velocity
                        let combined_velocity = (note.velocity * track.volume).clamp(0.0, 1.0);
                        let velocity = volume_to_velocity(combined_velocity);

                        // Add pitch bend event if needed (before the notes)
                        if note.pitch_bend_semitones != 0.0 {
                            let pitch_bend_value =
                                semitones_to_pitch_bend(note.pitch_bend_semitones, 2.0);
                            events.push((
                                start_tick,
                                MidiEventType::PitchBend {
                                    value: pitch_bend_value,
                                },
                            ));
                        }

                        // Add a note on/off event for each frequency in the chord
                        for i in 0..note.num_freqs {
                            let freq = note.frequencies[i];
                            let midi_note = frequency_to_midi_note(freq);

                            events.push((
                                start_tick,
                                MidiEventType::NoteOn {
                                    note: midi_note,
                                    velocity,
                                },
                            ));
                            events.push((end_tick, MidiEventType::NoteOff { note: midi_note }));
                        }

                        // Reset pitch bend to center after the note ends
                        if note.pitch_bend_semitones != 0.0 {
                            events.push((end_tick, MidiEventType::PitchBend { value: 8192 }));
                        }
                    }
                    AudioEvent::Drum(drum) => {
                        let tick = tempo_map.seconds_to_ticks(drum.start_time);
                        let midi_note = drum_type_to_midi_note(drum.drum_type);
                        let velocity = DEFAULT_VELOCITY;

                        // Drum note on (channel 10 = percussion)
                        events.push((
                            tick,
                            MidiEventType::NoteOn {
                                note: midi_note,
                                velocity,
                            },
                        ));
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
                            crate::synthesis::lfo::ModTarget::Pitch => (1, true), // CC1: Modulation Wheel
                            crate::synthesis::lfo::ModTarget::Volume => (11, false), // CC11: Expression
                            crate::synthesis::lfo::ModTarget::Pan => (10, true),     // CC10: Pan
                            _ => continue, // Skip filter parameters (synthesis-specific)
                        };

                        // Sample the LFO at regular intervals
                        // Make a mutable copy to tick through
                        let mut lfo_copy = mod_route.lfo;
                        for i in 0..num_samples {
                            let time = i as f32 * sample_interval;
                            let tick = tempo_map.seconds_to_ticks(time);

                            // Tick the LFO and get value
                            lfo_copy.tick();
                            let lfo_value = if bipolar {
                                lfo_copy.bipolar_value() * mod_route.amount
                            } else {
                                lfo_copy.value()
                            };

                            // Convert to CC value
                            let cc_value = mod_value_to_cc(lfo_value, bipolar);

                            // Add CC event
                            events.push((
                                tick,
                                MidiEventType::ControlChange {
                                    controller: cc_number,
                                    value: cc_value,
                                },
                            ));
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
                    MidiEventType::NoteOn { note, velocity } => MidiMessage::NoteOn {
                        key: u7::new(note),
                        vel: u7::new(velocity),
                    },
                    MidiEventType::NoteOff { note } => MidiMessage::NoteOff {
                        key: u7::new(note),
                        vel: u7::new(0),
                    },
                    MidiEventType::PitchBend { value } => MidiMessage::PitchBend {
                        bend: PitchBend(u14::new(value)),
                    },
                    MidiEventType::ControlChange { controller, value } => MidiMessage::Controller {
                        controller: u7::new(controller),
                        value: u7::new(value),
                    },
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

        let smf = Smf { header, tracks };

        // Write to file
        let mut file = File::create(path).map_err(|e| {
            TunesError::MidiError(format!("Failed to create MIDI file {}: {}", path, e))
        })?;

        smf.write_std(&mut file).map_err(|e| {
            TunesError::MidiError(format!("Failed to write MIDI data to {}: {}", path, e))
        })?;

        file.flush().map_err(|e| {
            TunesError::MidiError(format!("Failed to flush MIDI file {}: {}", path, e))
        })?;

        Ok(())
    }

    /// Import a MIDI file and create a Mixer from it
    ///
    /// Reads a Standard MIDI File and converts it to a Mixer that can be played,
    /// exported to WAV, or re-exported to MIDI.
    ///
    /// # Arguments
    /// * `path` - Path to the MIDI file (e.g., "song.mid")
    ///
    /// # Supported Features
    /// - Note events (converted to NoteEvent with frequency from MIDI note number)
    /// - Drum events on channel 10 (converted to DrumEvent)
    /// - Tempo changes (meta events)
    /// - Time signatures (meta events)
    /// - Multiple tracks
    /// - Track names
    ///
    /// # Limitations
    /// - Tempo changes occurring mid-track use the initial tempo for time calculations
    ///   (tempo change events are still preserved and exported correctly)
    /// - Pitch bend events are converted to static pitch offsets (not continuous)
    /// - Control change (CC) events are ignored
    /// - Program changes are stored but don't affect playback
    /// - Velocity is normalized to 0.0-1.0 range
    /// - Notes without proper Note Off events are given a default 0.1 second duration
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// // Import a MIDI file
    /// let mut mixer = Mixer::import_midi("song.mid")?;
    ///
    /// // Play it
    /// let engine = AudioEngine::new()?;
    /// engine.play_mixer(&mixer)?;
    ///
    /// // Or export to WAV
    /// mixer.export_wav("output.wav", 44100)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn import_midi(path: &str) -> Result<Self> {
        use crate::track::Track;
        use std::fs;

        // Read MIDI file
        let data = fs::read(path).map_err(|e| {
            TunesError::MidiError(format!("Failed to read MIDI file {}: {}", path, e))
        })?;

        let smf = Smf::parse(&data).map_err(|e| {
            TunesError::MidiError(format!("Failed to parse MIDI file {}: {}", path, e))
        })?;

        // Extract timing info (PPQ)
        let ppq = match smf.header.timing {
            Timing::Metrical(ticks) => ticks.as_int(),
            Timing::Timecode(_, _) => {
                return Err(TunesError::MidiError(
                    "SMPTE timecode timing not supported".to_string(),
                ));
            }
        };

        // Default tempo (120 BPM) - will be updated if tempo meta event is found
        let mut current_tempo = 120.0;
        let mut tempo_changes: Vec<(f32, f32)> = Vec::new(); // (time, bpm)
        let mut time_sig_changes: Vec<(f32, u8, u8)> = Vec::new(); // (time, numerator, denominator)

        // First pass: Extract tempo and time signature from all tracks
        for (track_idx, track) in smf.tracks.iter().enumerate() {
            let mut absolute_tick = 0u32;

            for event in track {
                absolute_tick += event.delta.as_int();

                if let TrackEventKind::Meta(meta) = &event.kind {
                    match meta {
                        MetaMessage::Tempo(tempo) => {
                            let us_per_quarter = tempo.as_int();
                            let bpm = 60_000_000.0 / us_per_quarter as f32;
                            let time = ticks_to_seconds(absolute_tick, current_tempo, ppq);
                            tempo_changes.push((time, bpm));

                            // Update current tempo for future time calculations
                            if track_idx == 0 {
                                current_tempo = bpm;
                            }
                        }
                        MetaMessage::TimeSignature(num, denom, _, _) => {
                            let denominator = 2u8.pow(*denom as u32);
                            let time = ticks_to_seconds(absolute_tick, current_tempo, ppq);
                            time_sig_changes.push((time, *num, denominator));
                        }
                        _ => {}
                    }
                }
            }
        }

        // Reset tempo to initial value for second pass
        current_tempo = if let Some((_, bpm)) = tempo_changes.first() {
            *bpm
        } else {
            120.0
        };

        // Create mixer with the initial tempo
        let mut mixer = Mixer::new(crate::composition::timing::Tempo::new(current_tempo));
        let mut audio_tracks: Vec<Track> = Vec::new();

        // Second pass: Convert MIDI tracks to audio tracks
        for midi_track in smf.tracks.iter() {
            let mut track = Track::new();
            let mut absolute_tick = 0u32;
            let mut track_name: Option<String> = None;
            let mut channel: Option<u8> = None;
            let mut midi_program: Option<u8> = None;

            // Track CC values for this track
            let mut track_volume: Option<f32> = None; // CC7
            let mut track_pan: Option<f32> = None;    // CC10

            // Track pitch bend state per channel
            // Key: channel, Value: pitch bend in semitones
            let mut pitch_bend_state: std::collections::HashMap<u8, f32> =
                std::collections::HashMap::new();

            // Track instrument per channel (for program changes)
            // Key: channel, Value: Instrument preset
            let mut channel_instruments: std::collections::HashMap<u8, crate::instruments::Instrument> =
                std::collections::HashMap::new();

            // Track active notes for Note On/Off pairing
            // Key: (channel, note), Value: (start_time, velocity, pitch_bend)
            let mut active_notes: std::collections::HashMap<(u8, u8), (f32, u8, f32)> =
                std::collections::HashMap::new();

            for event in midi_track {
                absolute_tick += event.delta.as_int();
                let time = ticks_to_seconds(absolute_tick, current_tempo, ppq);

                match &event.kind {
                    TrackEventKind::Meta(MetaMessage::TrackName(name)) => {
                        track_name = Some(String::from_utf8_lossy(name).to_string());
                    }
                    TrackEventKind::Meta(_) => {} // Ignore other meta messages
                    TrackEventKind::Midi {
                        channel: ch,
                        message,
                    } => {
                        let ch_num = ch.as_int();
                        if channel.is_none() {
                            channel = Some(ch_num);
                        }

                        match message {
                            MidiMessage::NoteOn { key, vel } => {
                                let note = key.as_int();
                                let velocity = vel.as_int();

                                if velocity == 0 {
                                    // Note off (velocity 0)
                                    if let Some((start_time, start_vel, pitch_bend)) =
                                        active_notes.remove(&(ch_num, note))
                                    {
                                        let duration = time - start_time;

                                        // Check if this is a drum track (channel 10 = channel index 9)
                                        if ch_num == 9 {
                                            // Drum track
                                            if let Some(drum_type) = midi_note_to_drum_type(note) {
                                                track.add_drum(drum_type, start_time, None);
                                            }
                                        } else {
                                            // Melodic track
                                            let freq = midi_note_to_frequency(note);
                                            let vel_normalized = start_vel as f32 / 127.0;

                                            // Get waveform and envelope from channel's instrument (if set via program change)
                                            // Otherwise use defaults (Sine, default envelope)
                                            let (waveform, envelope) = channel_instruments
                                                .get(&ch_num)
                                                .map(|inst| (inst.waveform, inst.envelope))
                                                .unwrap_or((
                                                    crate::synthesis::waveform::Waveform::Sine,
                                                    crate::synthesis::envelope::Envelope::default(),
                                                ));

                                            let note_event = crate::track::NoteEvent::with_complete_params(
                                                &[freq],
                                                start_time,
                                                duration,
                                                waveform,
                                                envelope,
                                                crate::synthesis::filter_envelope::FilterEnvelope::default(),
                                                crate::synthesis::fm_synthesis::FMParams::default(),
                                                pitch_bend, // Apply captured pitch bend
                                                None,       // No custom wavetable
                                                vel_normalized,
                                            );
                                            track
                                                .events
                                                .push(crate::track::AudioEvent::Note(note_event));
                                            track.invalidate_time_cache();
                                        }
                                    }
                                } else {
                                    // Note on - capture current pitch bend for this channel
                                    let current_pitch_bend = *pitch_bend_state.get(&ch_num).unwrap_or(&0.0);
                                    active_notes.insert((ch_num, note), (time, velocity, current_pitch_bend));
                                }
                            }
                            MidiMessage::NoteOff { key, .. } => {
                                let note = key.as_int();

                                if let Some((start_time, start_vel, pitch_bend)) =
                                    active_notes.remove(&(ch_num, note))
                                {
                                    let duration = time - start_time;

                                    // Check if this is a drum track (channel 10 = channel index 9)
                                    if ch_num == 9 {
                                        // Drum track
                                        if let Some(drum_type) = midi_note_to_drum_type(note) {
                                            track.add_drum(drum_type, start_time, None);
                                        }
                                    } else {
                                        // Melodic track
                                        let freq = midi_note_to_frequency(note);
                                        let vel_normalized = start_vel as f32 / 127.0;

                                        let note_event = crate::track::NoteEvent::with_complete_params(
                                            &[freq],
                                            start_time,
                                            duration,
                                            crate::synthesis::waveform::Waveform::Sine,
                                            crate::synthesis::envelope::Envelope::default(),
                                            crate::synthesis::filter_envelope::FilterEnvelope::default(),
                                            crate::synthesis::fm_synthesis::FMParams::default(),
                                            pitch_bend, // Apply captured pitch bend
                                            None,       // No custom wavetable
                                            vel_normalized,
                                        );
                                        track
                                            .events
                                            .push(crate::track::AudioEvent::Note(note_event));
                                        track.invalidate_time_cache();
                                    }
                                }
                            }
                            MidiMessage::ProgramChange { program } => {
                                let program_num = program.as_int();
                                midi_program = Some(program_num);

                                // Map GM program to instrument preset and store for this channel
                                let instrument = gm_program_to_instrument(program_num);
                                channel_instruments.insert(ch_num, instrument);
                            }
                            MidiMessage::Controller { controller, value } => {
                                let cc_num = controller.as_int();
                                let cc_value = value.as_int();

                                match cc_num {
                                    7 => {
                                        // Volume (CC7): 0-127 → 0.0-1.0
                                        track_volume = Some(cc_value as f32 / 127.0);
                                    }
                                    10 => {
                                        // Pan (CC10): 0-127 → -1.0 to 1.0 (64 = center)
                                        track_pan = Some((cc_value as f32 - 64.0) / 63.5);
                                    }
                                    11 => {
                                        // Expression (CC11): Treat as volume
                                        // If both CC7 and CC11 are present, CC11 takes precedence
                                        track_volume = Some(cc_value as f32 / 127.0);
                                    }
                                    _ => {
                                        // Ignore other CCs (modulation wheel, sustain pedal, etc.)
                                        // These could be added in the future
                                    }
                                }
                            }
                            MidiMessage::PitchBend { bend } => {
                                // Convert MIDI pitch bend to semitones
                                // Standard range is ±2 semitones
                                // Note: midly returns signed value relative to center
                                let bend_value = bend.as_int();
                                let semitones = pitch_bend_to_semitones_from_signed(bend_value, 2.0);
                                pitch_bend_state.insert(ch_num, semitones);
                            }
                            _ => {
                                // Ignore other MIDI messages (aftertouch, etc.)
                            }
                        }
                    }
                    _ => {}
                }
            }

            // Handle any "hanging" notes that never received a Note Off
            // Give them a default duration of 0.1 seconds
            for ((ch_num, note), (start_time, start_vel, pitch_bend)) in active_notes.drain() {
                let duration = 0.1; // Default duration for hanging notes

                if ch_num == 9 {
                    // Drum track
                    if let Some(drum_type) = midi_note_to_drum_type(note) {
                        track.add_drum(drum_type, start_time, None);
                    }
                } else {
                    // Melodic track
                    let freq = midi_note_to_frequency(note);
                    let vel_normalized = start_vel as f32 / 127.0;

                    // Get waveform and envelope from channel's instrument (if set via program change)
                    // Otherwise use defaults (Sine, default envelope)
                    let (waveform, envelope) = channel_instruments
                        .get(&ch_num)
                        .map(|inst| (inst.waveform, inst.envelope))
                        .unwrap_or((
                            crate::synthesis::waveform::Waveform::Sine,
                            crate::synthesis::envelope::Envelope::default(),
                        ));

                    let note_event = crate::track::NoteEvent::with_complete_params(
                        &[freq],
                        start_time,
                        duration,
                        waveform,
                        envelope,
                        crate::synthesis::filter_envelope::FilterEnvelope::default(),
                        crate::synthesis::fm_synthesis::FMParams::default(),
                        pitch_bend, // Apply captured pitch bend
                        None,
                        vel_normalized,
                    );
                    track
                        .events
                        .push(crate::track::AudioEvent::Note(note_event));
                    track.invalidate_time_cache();
                }
            }

            // Set track metadata
            track.name = track_name;
            track.midi_program = midi_program;

            // Apply CC values to track
            if let Some(volume) = track_volume {
                track.volume = volume;
            }
            if let Some(pan) = track_pan {
                track.pan = pan.clamp(-1.0, 1.0);
            }

            // Only add tracks that have events
            if !track.events.is_empty() {
                audio_tracks.push(track);
            }
        }

        // Add tempo changes to the first track (or create a tempo track if needed)
        for (time, bpm) in tempo_changes.iter().skip(1) {
            // Skip the first tempo change (it's the initial tempo)
            if let Some(first_track) = audio_tracks.first_mut() {
                first_track
                    .events
                    .push(crate::track::AudioEvent::TempoChange(
                        crate::track::TempoChangeEvent {
                            start_time: *time,
                            bpm: *bpm,
                        },
                    ));
                first_track.invalidate_time_cache();
            }
        }

        // Add time signature changes to the first track
        for (time, num, denom) in time_sig_changes {
            if let Some(first_track) = audio_tracks.first_mut() {
                first_track
                    .events
                    .push(crate::track::AudioEvent::TimeSignature(
                        crate::track::TimeSignatureEvent {
                            start_time: time,
                            numerator: num,
                            denominator: denom,
                        },
                    ));
                first_track.invalidate_time_cache();
            }
        }

        // Add all tracks to mixer
        for track in audio_tracks {
            mixer.add_track(track);
        }

        Ok(mixer)
    }
}
