//! Arrangement and section system for composing larger musical structures
//!
//! This module provides tools for creating reusable sections (verse, chorus, bridge, etc.)
//! and arranging them into complete compositions.

use crate::composition::Composition;
use crate::envelope::Envelope;
use crate::instruments::Instrument;
use crate::rhythm::Tempo;
use crate::track::{AudioEvent, Track};
use crate::waveform::Waveform;
use std::collections::HashMap;

/// A reusable section of music that can be arranged in a composition
///
/// Sections capture a portion of music with multiple tracks that can be
/// repeated and sequenced. Think of them like verse, chorus, bridge, intro, etc.
#[derive(Clone, Debug)]
pub struct Section {
    pub(crate) name: String,
    pub(crate) tracks: HashMap<String, Track>,
    pub(crate) duration: f32, // Total duration of this section in seconds
}

impl Section {
    /// Create a new empty section
    pub fn new(name: String) -> Self {
        Self {
            name,
            tracks: HashMap::new(),
            duration: 0.0,
        }
    }

    /// Get the name of this section
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the duration of this section in seconds
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// Clone this section's events and offset them by a time delta
    ///
    /// This is used internally by the arrangement system to place sections
    /// at different points in time.
    pub(crate) fn clone_with_offset(&self, time_offset: f32) -> HashMap<String, Track> {
        self.tracks
            .iter()
            .map(|(name, track)| {
                let mut new_track = track.clone();
                // Offset all event times
                for event in &mut new_track.events {
                    match event {
                        AudioEvent::Note(note) => {
                            note.start_time += time_offset;
                        }
                        AudioEvent::Drum(drum) => {
                            drum.start_time += time_offset;
                        }
                    }
                }
                (name.clone(), new_track)
            })
            .collect()
    }
}

/// Builder for creating musical sections
///
/// SectionBuilder works similarly to Composition - you add tracks and events,
/// but the section is stored for later reuse in arrangements.
pub struct SectionBuilder<'a> {
    composition: &'a mut Composition,
    section_name: String,
    tempo: Tempo,
}

impl<'a> SectionBuilder<'a> {
    /// Create a new section builder
    pub(crate) fn new(composition: &'a mut Composition, name: String, tempo: Tempo) -> Self {
        Self {
            composition,
            section_name: name,
            tempo,
        }
    }

    /// Get or create a track within this section
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.section("verse")
    ///     .track("melody")
    ///     .notes(&[C4, E4, G4], 0.5);
    /// ```
    pub fn track(self, name: &str) -> SectionTrackBuilder<'a> {
        SectionTrackBuilder {
            composition: self.composition,
            section_name: self.section_name,
            track_name: name.to_string(),
            cursor: 0.0,
            pattern_start: 0.0,
            waveform: Waveform::Sine,
            envelope: Envelope::default(),
            swing: 0.5,
            swing_counter: 0,
            pitch_bend: 0.0,
            tempo: self.tempo,
        }
    }

    /// Create a track with an instrument preset within this section
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.section("chorus")
    ///     .instrument("lead", &Instrument::synth_lead())
    ///     .notes(&[E4, G4, B4], 0.25);
    /// ```
    pub fn instrument(self, name: &str, instrument: &Instrument) -> SectionTrackBuilder<'a> {
        let mut builder = SectionTrackBuilder {
            composition: self.composition,
            section_name: self.section_name,
            track_name: name.to_string(),
            cursor: 0.0,
            pattern_start: 0.0,
            waveform: instrument.waveform,
            envelope: instrument.envelope,
            swing: 0.5,
            swing_counter: 0,
            pitch_bend: 0.0,
            tempo: self.tempo,
        };

        // Get or create the track and apply instrument settings
        builder.get_or_create_track_mut().apply_instrument(instrument);

        builder
    }
}

/// Builder for adding events to a track within a section
///
/// This is similar to TrackBuilder but works within a section context
pub struct SectionTrackBuilder<'a> {
    composition: &'a mut Composition,
    section_name: String,
    track_name: String,
    cursor: f32,
    pattern_start: f32,
    waveform: Waveform,
    envelope: Envelope,
    swing: f32,
    swing_counter: usize,
    pitch_bend: f32,
    tempo: Tempo,
}

impl<'a> SectionTrackBuilder<'a> {
    /// Get or create the track for this section
    fn get_or_create_track_mut(&mut self) -> &mut Track {
        self.composition
            .get_or_create_section_track(&self.section_name, &self.track_name)
    }

    /// Update the section duration based on current cursor position
    fn update_section_duration(&mut self) {
        if let Some(section) = self.composition.sections.get_mut(&self.section_name) {
            section.duration = section.duration.max(self.cursor);
        }
    }
}

impl Track {
    /// Apply instrument settings to this track
    fn apply_instrument(&mut self, instrument: &Instrument) {
        self.volume = instrument.volume;
        self.pan = instrument.pan;
        self.filter = instrument.filter;
        self.delay = instrument.delay.clone();
        self.reverb = instrument.reverb.clone();
        self.distortion = instrument.distortion;
        self.modulation = instrument.modulation.clone();
    }
}

// We'll implement the note/drum methods directly to avoid complex delegation

use crate::drums::DrumType;
use crate::track::NoteEvent;

impl<'a> SectionTrackBuilder<'a> {
    /// Continue building on this section's track (returns the section builder)
    ///
    /// This is a helper to chain section operations while ensuring the section is finalized
    pub fn and(mut self) -> SectionBuilder<'a> {
        self.update_section_duration();
        SectionBuilder {
            composition: self.composition,
            section_name: self.section_name,
            tempo: self.tempo,
        }
    }

    /// Add a single note or chord
    pub fn note(mut self, frequencies: &[f32], duration: f32) -> Self {
        // Copy values before borrowing
        let cursor = self.cursor;
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        let track = self.get_or_create_track_mut();
        let note = NoteEvent::with_waveform_envelope_and_bend(
            frequencies,
            cursor,
            duration,
            waveform,
            envelope,
            pitch_bend,
        );
        track.events.push(AudioEvent::Note(note));

        self.cursor += duration;
        self.update_section_duration();
        self
    }

    /// Add multiple notes in sequence
    pub fn notes(mut self, frequencies: &[f32], duration: f32) -> Self {
        // Copy values before borrowing
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        for freq in frequencies {
            let cursor = self.cursor;
            let track = self.get_or_create_track_mut();
            let note = NoteEvent::with_waveform_envelope_and_bend(
                &[*freq],
                cursor,
                duration,
                waveform,
                envelope,
                pitch_bend,
            );
            track.events.push(AudioEvent::Note(note));
            self.cursor += duration;
        }
        self.update_section_duration();
        self
    }

    /// Add a drum hit
    pub fn drum(mut self, drum_type: DrumType) -> Self {
        let cursor = self.cursor;
        let track = self.get_or_create_track_mut();
        track.add_drum(drum_type, cursor);
        self.cursor += 0.1; // Default drum duration
        self.update_section_duration();
        self
    }

    /// Set the waveform for subsequent notes
    pub fn waveform(mut self, waveform: Waveform) -> Self {
        self.waveform = waveform;
        self
    }

    /// Set the envelope for subsequent notes
    pub fn envelope(mut self, envelope: Envelope) -> Self {
        self.envelope = envelope;
        self
    }

    /// Set the cursor position (time in seconds)
    pub fn at(mut self, time: f32) -> Self {
        self.cursor = time;
        self.update_section_duration();
        self
    }

    /// Wait/advance the cursor by a duration
    pub fn wait(mut self, duration: f32) -> Self {
        self.cursor += duration;
        self.update_section_duration();
        self
    }

    /// Mark the start of a pattern for looping/repeating
    pub fn pattern_start(mut self) -> Self {
        self.pattern_start = self.cursor;
        self
    }

    /// Repeat the pattern since pattern_start
    pub fn repeat(mut self, times: usize) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;
        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect events in the pattern range
        {
            let track = self.get_or_create_track_mut();
            let pattern_events: Vec<_> = track
                .events
                .iter()
                .filter(|event| {
                    let event_time = match event {
                        AudioEvent::Note(note) => note.start_time,
                        AudioEvent::Drum(drum) => drum.start_time,
                    };
                    event_time >= pattern_start && event_time < cursor
                })
                .cloned()
                .collect();

            // Repeat the pattern
            for i in 0..times {
                let offset = pattern_duration * (i + 1) as f32;
                for event in &pattern_events {
                    let mut new_event = *event;
                    match &mut new_event {
                        AudioEvent::Note(note) => {
                            note.start_time += offset;
                        }
                        AudioEvent::Drum(drum) => {
                            drum.start_time += offset;
                        }
                    }
                    track.events.push(new_event);
                }
            }
        }

        self.cursor += pattern_duration * times as f32;
        self.update_section_duration();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notes::*;
    use crate::drums::DrumType;

    #[test]
    fn test_create_empty_section() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.section("intro");

        assert!(comp.sections.contains_key("intro"));
    }

    #[test]
    fn test_section_duration_tracking() {
        let section = Section::new("test".to_string());
        assert_eq!(section.duration(), 0.0);
    }

    #[test]
    fn test_section_clone_with_offset() {
        let mut section = Section::new("test".to_string());
        let mut track = Track::new();
        track.add_note(&[C4], 0.0, 1.0);
        track.add_note(&[E4], 1.0, 1.0);
        section.tracks.insert("melody".to_string(), track);

        let offset_tracks = section.clone_with_offset(4.0);
        let melody = &offset_tracks["melody"];

        if let AudioEvent::Note(note) = melody.events[0] {
            assert_eq!(note.start_time, 4.0);
        }
        if let AudioEvent::Note(note) = melody.events[1] {
            assert_eq!(note.start_time, 5.0);
        }
    }

    #[test]
    fn test_section_with_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.section("verse")
            .track("melody")
            .notes(&[C4, E4, G4, C5], 0.5);

        let section = comp.sections.get("verse").unwrap();
        let melody = section.tracks.get("melody").unwrap();

        assert_eq!(melody.events.len(), 4);
        assert_eq!(section.duration, 2.0); // 4 notes * 0.5s each
    }

    #[test]
    fn test_section_with_multiple_tracks() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.section("chorus")
            .track("melody")
            .notes(&[C4, E4], 0.5)
            .and()
            .track("bass")
            .notes(&[C2, G2], 1.0);

        let section = comp.sections.get("chorus").unwrap();

        assert_eq!(section.tracks.len(), 2);
        assert!(section.tracks.contains_key("melody"));
        assert!(section.tracks.contains_key("bass"));
        assert_eq!(section.duration, 2.0); // Bass track is longer
    }

    #[test]
    fn test_section_with_drums() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.section("beat")
            .track("drums")
            .drum(DrumType::Kick)
            .drum(DrumType::Snare)
            .drum(DrumType::Kick)
            .drum(DrumType::Snare);

        let section = comp.sections.get("beat").unwrap();
        let drums = section.tracks.get("drums").unwrap();

        assert_eq!(drums.events.len(), 4);
    }

    #[test]
    fn test_arrange_single_section() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.section("verse")
            .track("melody")
            .notes(&[C4, E4], 0.5);

        comp.arrange(&["verse"]);

        let melody = comp.tracks.get("melody").unwrap();
        assert_eq!(melody.events.len(), 2);
    }

    #[test]
    fn test_arrange_multiple_sections() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.section("intro")
            .track("melody")
            .note(&[C4], 1.0);

        comp.section("verse")
            .track("melody")
            .notes(&[E4, G4], 0.5);

        comp.arrange(&["intro", "verse"]);

        let melody = comp.tracks.get("melody").unwrap();
        assert_eq!(melody.events.len(), 3); // 1 from intro + 2 from verse

        // Check timing
        if let AudioEvent::Note(note) = melody.events[0] {
            assert_eq!(note.start_time, 0.0); // Intro note
        }
        if let AudioEvent::Note(note) = melody.events[1] {
            assert_eq!(note.start_time, 1.0); // First verse note (after 1s intro)
        }
        if let AudioEvent::Note(note) = melody.events[2] {
            assert_eq!(note.start_time, 1.5); // Second verse note
        }
    }

    #[test]
    fn test_arrange_repeated_section() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.section("chorus")
            .track("melody")
            .notes(&[C4, E4], 0.5);

        comp.arrange(&["chorus", "chorus"]);

        let melody = comp.tracks.get("melody").unwrap();
        assert_eq!(melody.events.len(), 4); // 2 notes × 2 repetitions

        // Check timing
        if let AudioEvent::Note(note) = melody.events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = melody.events[2] {
            assert_eq!(note.start_time, 1.0); // Second chorus starts after first (1s)
        }
    }

    #[test]
    fn test_section_pattern_repeat() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.section("riff")
            .track("guitar")
            .pattern_start()
            .notes(&[C4, E4], 0.25)
            .repeat(2);

        let section = comp.sections.get("riff").unwrap();
        let guitar = section.tracks.get("guitar").unwrap();

        assert_eq!(guitar.events.len(), 6); // 2 notes + (2 notes × 2 repeats)
        assert_eq!(section.duration, 1.5); // Original 0.5s + 2 repeats × 0.5s
    }

    #[test]
    fn test_complex_arrangement() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Define sections
        comp.section("intro")
            .track("melody")
            .notes(&[C4], 2.0);

        comp.section("verse")
            .track("melody")
            .notes(&[E4, G4], 1.0);

        comp.section("chorus")
            .track("melody")
            .notes(&[C5, B4, A4, G4], 0.5);

        // Arrange: intro, verse, chorus, verse
        comp.arrange(&["intro", "verse", "chorus", "verse"]);

        let melody = comp.tracks.get("melody").unwrap();

        // Total events: 1 + 2 + 4 + 2 = 9
        assert_eq!(melody.events.len(), 9);

        // Check last note timing
        // intro: 2s, verse: 2s, chorus: 2s, verse starts at 6s
        if let AudioEvent::Note(note) = melody.events[8] {
            assert_eq!(note.start_time, 7.0); // 6s + 1s
        }
    }

    #[test]
    fn test_section_at_positioning() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.section("test")
            .track("melody")
            .at(1.0)
            .note(&[C4], 0.5)
            .at(3.0)
            .note(&[E4], 0.5);

        let section = comp.sections.get("test").unwrap();
        let melody = section.tracks.get("melody").unwrap();

        if let AudioEvent::Note(note) = melody.events[0] {
            assert_eq!(note.start_time, 1.0);
        }
        if let AudioEvent::Note(note) = melody.events[1] {
            assert_eq!(note.start_time, 3.0);
        }
    }
}
