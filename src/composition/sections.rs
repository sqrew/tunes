//! Arrangement and section system for composing larger musical structures
//!
//! This module provides tools for creating reusable sections (verse, chorus, bridge, etc.)
//! and arranging them into complete compositions.

use crate::composition::Composition;
use crate::envelope::Envelope;
use crate::filter_envelope::FilterEnvelope;
use crate::fm_synthesis::FMParams;
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
    pub fn track(self, name: &str) -> crate::composition::TrackBuilder<'a> {
        crate::composition::TrackBuilder {
            composition: self.composition,
            context: crate::composition::BuilderContext::Section(self.section_name),
            track_name: name.to_string(),
            cursor: 0.0,
            pattern_start: 0.0,
            waveform: Waveform::Sine,
            envelope: Envelope::default(),
            filter_envelope: FilterEnvelope::default(),
            fm_params: FMParams::default(),
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
    pub fn instrument(self, name: &str, instrument: &Instrument) -> crate::composition::TrackBuilder<'a> {
        let mut builder = crate::composition::TrackBuilder {
            composition: self.composition,
            context: crate::composition::BuilderContext::Section(self.section_name),
            track_name: name.to_string(),
            cursor: 0.0,
            pattern_start: 0.0,
            waveform: instrument.waveform,
            envelope: instrument.envelope,
            filter_envelope: FilterEnvelope::default(),
            fm_params: FMParams::default(),
            swing: 0.5,
            swing_counter: 0,
            pitch_bend: 0.0,
            tempo: self.tempo,
        };

        // Get or create the track and apply instrument settings
        builder.get_track_mut().apply_instrument(instrument);

        builder
    }
}

impl Track {
    /// Apply instrument settings to this track
    pub(crate) fn apply_instrument(&mut self, instrument: &Instrument) {
        self.volume = instrument.volume;
        self.pan = instrument.pan;
        self.filter = instrument.filter;
        self.delay = instrument.delay.clone();
        self.reverb = instrument.reverb.clone();
        self.distortion = instrument.distortion;
        self.modulation = instrument.modulation.clone();
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
