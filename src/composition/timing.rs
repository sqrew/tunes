use super::TrackBuilder;

impl<'a> TrackBuilder<'a> {
    /// Set swing/groove timing (0.5 = straight, 0.67 = triplet swing, 0.75 = heavy swing)
    pub fn swing(mut self, swing: f32) -> Self {
        self.swing = swing.clamp(0.5, 0.9);
        self
    }
    pub fn at(mut self, time: f32) -> Self {
        self.cursor = time;
        self
    }

    pub fn wait(mut self, duration: f32) -> Self {
        self.cursor += duration;
        self
    }

    pub fn seek(mut self, offset: f32) -> Self {
        self.cursor += offset;
        self
    }

    /// Save the current cursor position with a name for later use
    ///
    /// Markers are stored globally in the composition and can be accessed
    /// by any track. This makes it easy to synchronize timing across tracks.
    ///
    /// # Arguments
    /// * `name` - The name to give this time marker
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .notes(&[C4, E4, G4], 0.5)
    ///     .mark("verse_start")  // Save this position
    ///     .notes(&[C4, E4, G4], 0.5);
    ///
    /// comp.track("bass")
    ///     .at_mark("verse_start")  // Jump to saved position
    ///     .notes(&[C2, G2], 1.0);
    /// ```
    pub fn mark(self, name: &str) -> Self {
        self.composition
            .markers
            .insert(name.to_string(), self.cursor);
        self
    }

    /// Jump to a previously saved marker position
    ///
    /// # Arguments
    /// * `name` - The name of the marker to jump to
    ///
    /// # Panics
    /// Panics if the marker name doesn't exist. Use `.mark()` to create markers first.
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .notes(&[C4], 0.5)
    ///     .mark("drop");
    ///
    /// comp.track("bass")
    ///     .at_mark("drop")  // Start exactly where drums marked "drop"
    ///     .notes(&[C2], 0.5);
    /// ```
    pub fn at_mark(mut self, name: &str) -> Self {
        let marker_time = match self.composition.markers.get(name) {
            Some(time) => *time,
            None => {
                eprintln!(
                    "Warning: Marker '{}' not found. Use .mark(\"{}\") to create it first. Cursor position unchanged.",
                    name, name
                );
                return self;
            }
        };
        self.cursor = marker_time;
        self
    }

    /// Get the current cursor position without modifying the builder
    ///
    /// Useful for debugging or conditional logic based on timing.
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// let builder = comp.track("melody")
    ///     .notes(&[C4, E4, G4], 0.5);
    ///
    /// let current_time = builder.peek_cursor();
    /// println!("Currently at: {} seconds", current_time);
    /// ```
    pub fn peek_cursor(&self) -> f32 {
        self.cursor
    }

    /// Insert a tempo change at the current cursor position
    ///
    /// # Arguments
    /// * `bpm` - The new tempo in beats per minute
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .notes(&[C4, E4, G4], 0.5)  // 120 BPM
    ///     .tempo(90.0)                 // Slow down to 90 BPM
    ///     .notes(&[C4, E4, G4], 0.5); // These notes at 90 BPM (affects MIDI export)
    /// ```
    pub fn tempo(mut self, bpm: f32) -> Self {
        let cursor = self.cursor;
        self.get_track_mut()
            .events
            .push(crate::track::AudioEvent::TempoChange(
                crate::track::TempoChangeEvent {
                    start_time: cursor,
                    bpm,
                },
            ));
        self.get_track_mut().invalidate_time_cache();
        self
    }

    /// Insert a time signature change at the current cursor position
    ///
    /// # Arguments
    /// * `numerator` - Top number of time signature (beats per measure)
    /// * `denominator` - Bottom number of time signature (note value per beat)
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .time_signature(4, 4)        // 4/4 time
    ///     .notes(&[C4, E4, G4, C5], 0.5)
    ///     .time_signature(3, 4)        // Switch to 3/4 time
    ///     .notes(&[C4, E4, G4], 0.5); // These notes in 3/4 (affects MIDI export)
    /// ```
    pub fn time_signature(mut self, numerator: u8, denominator: u8) -> Self {
        let cursor = self.cursor;
        self.get_track_mut()
            .events
            .push(crate::track::AudioEvent::TimeSignature(
                crate::track::TimeSignatureEvent {
                    start_time: cursor,
                    numerator,
                    denominator,
                },
            ));
        self.get_track_mut().invalidate_time_cache();
        self
    }

    /// Insert a key signature change at the current cursor position
    ///
    /// Sets the key signature for MIDI export and musical notation context.
    ///
    /// # Arguments
    /// * `key_signature` - The key signature to set (e.g., KeySignature::C_MAJOR)
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .key_signature(KeySignature::C_MAJOR)
    ///     .notes(&[C4, E4, G4], 0.5)
    ///     .key_signature(KeySignature::A_MINOR)  // Modulate to A minor
    ///     .notes(&[A3, C4, E4], 0.5);
    /// ```
    pub fn key_signature(mut self, key_signature: crate::theory::key_signature::KeySignature) -> Self {
        let cursor = self.cursor;
        self.get_track_mut()
            .events
            .push(crate::track::AudioEvent::KeySignature(
                crate::track::KeySignatureEvent {
                    start_time: cursor,
                    key_signature,
                },
            ));
        self.get_track_mut().invalidate_time_cache();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::consts::notes::{C3, C4, E3, E4, G3, G4};
    use crate::composition::rhythm::Tempo;

    #[test]
    fn test_swing_sets_value() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").swing(0.67);

        assert_eq!(builder.swing, 0.67);
    }

    #[test]
    fn test_swing_clamps_low_values() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").swing(0.2);

        // Should clamp to 0.5
        assert_eq!(builder.swing, 0.5);
    }

    #[test]
    fn test_swing_clamps_high_values() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").swing(0.95);

        // Should clamp to 0.9
        assert_eq!(builder.swing, 0.9);
    }

    #[test]
    fn test_swing_allows_boundary_values() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder_min = comp.track("test1").swing(0.5);
        assert_eq!(builder_min.swing, 0.5);

        let builder_max = comp.track("test2").swing(0.9);
        assert_eq!(builder_max.swing, 0.9);
    }

    #[test]
    fn test_swing_common_values() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Straight timing
        let straight = comp.track("straight").swing(0.5);
        assert_eq!(straight.swing, 0.5);

        // Triplet swing
        let triplet = comp.track("triplet").swing(0.67);
        assert!((triplet.swing - 0.67).abs() < 0.01);

        // Heavy swing
        let heavy = comp.track("heavy").swing(0.75);
        assert_eq!(heavy.swing, 0.75);
    }

    #[test]
    fn test_at_sets_absolute_cursor_position() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").at(5.0);

        assert_eq!(builder.cursor, 5.0);
    }

    #[test]
    fn test_at_can_move_backward() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").wait(10.0).at(3.0);

        // Should be at 3.0, not 13.0
        assert_eq!(builder.cursor, 3.0);
    }

    #[test]
    fn test_at_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").wait(5.0).at(0.0);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_wait_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").wait(2.5);

        assert_eq!(builder.cursor, 2.5);
    }

    #[test]
    fn test_wait_is_additive() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").wait(1.0).wait(2.0).wait(0.5);

        assert_eq!(builder.cursor, 3.5);
    }

    #[test]
    fn test_wait_with_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").wait(5.0).wait(0.0);

        assert_eq!(builder.cursor, 5.0);
    }

    #[test]
    fn test_seek_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").seek(3.0);

        assert_eq!(builder.cursor, 3.0);
    }

    #[test]
    fn test_seek_is_additive() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").seek(1.5).seek(2.0);

        assert_eq!(builder.cursor, 3.5);
    }

    #[test]
    fn test_seek_backward_with_negative() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").seek(5.0).seek(-2.0);

        // Should be at 3.0
        assert_eq!(builder.cursor, 3.0);
    }

    #[test]
    fn test_timing_methods_chain_together() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").at(0.0).wait(2.0).seek(1.0).swing(0.67);

        assert_eq!(builder.cursor, 3.0);
        assert!((builder.swing - 0.67).abs() < 0.01);
    }

    #[test]
    fn test_complex_timing_sequence() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp
            .track("test")
            .at(10.0) // Jump to 10.0
            .wait(5.0) // Now at 15.0
            .seek(-3.0) // Now at 12.0
            .at(0.0) // Back to 0.0
            .wait(1.0); // Now at 1.0

        assert_eq!(builder.cursor, 1.0);
    }

    #[test]
    fn test_wait_and_seek_are_equivalent() {
        let mut comp1 = Composition::new(Tempo::new(120.0));
        let mut comp2 = Composition::new(Tempo::new(120.0));

        let with_wait = comp1.track("wait").wait(2.5);
        let with_seek = comp2.track("seek").seek(2.5);

        assert_eq!(with_wait.cursor, with_seek.cursor);
    }

    #[test]
    fn test_timing_with_note_placement() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .at(0.0)
            .note(&[440.0], 0.5) // Note at 0.0, cursor advances to 0.5
            .wait(1.0) // Cursor advances to 1.5
            .note(&[550.0], 0.5); // Note at 1.5

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 2);

        if let crate::track::AudioEvent::Note(note1) = &track.events[0] {
            assert_eq!(note1.start_time, 0.0);
        }
        if let crate::track::AudioEvent::Note(note2) = &track.events[1] {
            assert_eq!(note2.start_time, 1.5); // 0.5 (after first note) + 1.0 (wait)
        }
    }

    #[test]
    fn test_tempo_adds_tempo_change_event() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").tempo(90.0);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 1);

        if let crate::track::AudioEvent::TempoChange(tempo) = &track.events[0] {
            assert_eq!(tempo.bpm, 90.0);
            assert_eq!(tempo.start_time, 0.0);
        } else {
            panic!("Expected TempoChange event");
        }
    }

    #[test]
    fn test_tempo_at_specific_time() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").at(5.0).tempo(140.0);

        let track = &comp.into_mixer().tracks()[0];
        if let crate::track::AudioEvent::TempoChange(tempo) = &track.events[0] {
            assert_eq!(tempo.bpm, 140.0);
            assert_eq!(tempo.start_time, 5.0);
        }
    }

    #[test]
    fn test_multiple_tempo_changes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .tempo(100.0)
            .wait(2.0)
            .tempo(120.0)
            .wait(2.0)
            .tempo(140.0);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 3);

        if let crate::track::AudioEvent::TempoChange(tempo1) = &track.events[0] {
            assert_eq!(tempo1.bpm, 100.0);
            assert_eq!(tempo1.start_time, 0.0);
        }

        if let crate::track::AudioEvent::TempoChange(tempo2) = &track.events[1] {
            assert_eq!(tempo2.bpm, 120.0);
            assert_eq!(tempo2.start_time, 2.0);
        }

        if let crate::track::AudioEvent::TempoChange(tempo3) = &track.events[2] {
            assert_eq!(tempo3.bpm, 140.0);
            assert_eq!(tempo3.start_time, 4.0);
        }
    }

    #[test]
    fn test_tempo_with_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .note(&[C4], 0.5)
            .tempo(90.0)
            .note(&[E4], 0.5);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 3); // 2 notes + 1 tempo change

        // First note
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.start_time, 0.0);
        }

        // Tempo change
        if let crate::track::AudioEvent::TempoChange(tempo) = &track.events[1] {
            assert_eq!(tempo.bpm, 90.0);
            assert_eq!(tempo.start_time, 0.5);
        }

        // Second note
        if let crate::track::AudioEvent::Note(note) = &track.events[2] {
            assert_eq!(note.start_time, 0.5);
        }
    }

    #[test]
    fn test_time_signature_adds_event() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").time_signature(3, 4);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 1);

        if let crate::track::AudioEvent::TimeSignature(time_sig) = &track.events[0] {
            assert_eq!(time_sig.numerator, 3);
            assert_eq!(time_sig.denominator, 4);
            assert_eq!(time_sig.start_time, 0.0);
        } else {
            panic!("Expected TimeSignature event");
        }
    }

    #[test]
    fn test_time_signature_at_specific_time() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").at(8.0).time_signature(6, 8);

        let track = &comp.into_mixer().tracks()[0];
        if let crate::track::AudioEvent::TimeSignature(time_sig) = &track.events[0] {
            assert_eq!(time_sig.numerator, 6);
            assert_eq!(time_sig.denominator, 8);
            assert_eq!(time_sig.start_time, 8.0);
        }
    }

    #[test]
    fn test_multiple_time_signature_changes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .time_signature(4, 4)
            .wait(4.0)
            .time_signature(3, 4)
            .wait(3.0)
            .time_signature(7, 8);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 3);

        if let crate::track::AudioEvent::TimeSignature(time_sig1) = &track.events[0] {
            assert_eq!(time_sig1.numerator, 4);
            assert_eq!(time_sig1.denominator, 4);
            assert_eq!(time_sig1.start_time, 0.0);
        }

        if let crate::track::AudioEvent::TimeSignature(time_sig2) = &track.events[1] {
            assert_eq!(time_sig2.numerator, 3);
            assert_eq!(time_sig2.denominator, 4);
            assert_eq!(time_sig2.start_time, 4.0);
        }

        if let crate::track::AudioEvent::TimeSignature(time_sig3) = &track.events[2] {
            assert_eq!(time_sig3.numerator, 7);
            assert_eq!(time_sig3.denominator, 8);
            assert_eq!(time_sig3.start_time, 7.0);
        }
    }

    #[test]
    fn test_time_signature_with_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .time_signature(4, 4)
            .note(&[C4], 0.5)
            .note(&[E4], 0.5)
            .time_signature(3, 4)
            .note(&[G4], 0.5);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 5); // 2 time sigs + 3 notes

        // First time signature
        if let crate::track::AudioEvent::TimeSignature(time_sig) = &track.events[0] {
            assert_eq!(time_sig.numerator, 4);
            assert_eq!(time_sig.denominator, 4);
            assert_eq!(time_sig.start_time, 0.0);
        }

        // First note
        if let crate::track::AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.start_time, 0.0);
        }

        // Second note
        if let crate::track::AudioEvent::Note(note) = &track.events[2] {
            assert_eq!(note.start_time, 0.5);
        }

        // Second time signature
        if let crate::track::AudioEvent::TimeSignature(time_sig) = &track.events[3] {
            assert_eq!(time_sig.numerator, 3);
            assert_eq!(time_sig.denominator, 4);
            assert_eq!(time_sig.start_time, 1.0);
        }

        // Third note
        if let crate::track::AudioEvent::Note(note) = &track.events[4] {
            assert_eq!(note.start_time, 1.0);
        }
    }

    #[test]
    fn test_time_signature_common_values() {
        let mut comp1 = Composition::new(Tempo::new(120.0));
        comp1.track("test").time_signature(4, 4);
        let mixer1 = comp1.into_mixer();
        if let crate::track::AudioEvent::TimeSignature(time_sig) = &mixer1.tracks()[0].events[0] {
            assert_eq!(time_sig.numerator, 4);
            assert_eq!(time_sig.denominator, 4);
        }

        let mut comp2 = Composition::new(Tempo::new(120.0));
        comp2.track("test").time_signature(3, 4);
        let mixer2 = comp2.into_mixer();
        if let crate::track::AudioEvent::TimeSignature(time_sig) = &mixer2.tracks()[0].events[0] {
            assert_eq!(time_sig.numerator, 3);
            assert_eq!(time_sig.denominator, 4);
        }

        let mut comp3 = Composition::new(Tempo::new(120.0));
        comp3.track("test").time_signature(6, 8);
        let mixer3 = comp3.into_mixer();
        if let crate::track::AudioEvent::TimeSignature(time_sig) = &mixer3.tracks()[0].events[0] {
            assert_eq!(time_sig.numerator, 6);
            assert_eq!(time_sig.denominator, 8);
        }

        let mut comp4 = Composition::new(Tempo::new(120.0));
        comp4.track("test").time_signature(5, 4);
        let mixer4 = comp4.into_mixer();
        if let crate::track::AudioEvent::TimeSignature(time_sig) = &mixer4.tracks()[0].events[0] {
            assert_eq!(time_sig.numerator, 5);
            assert_eq!(time_sig.denominator, 4);
        }

        let mut comp5 = Composition::new(Tempo::new(120.0));
        comp5.track("test").time_signature(7, 8);
        let mixer5 = comp5.into_mixer();
        if let crate::track::AudioEvent::TimeSignature(time_sig) = &mixer5.tracks()[0].events[0] {
            assert_eq!(time_sig.numerator, 7);
            assert_eq!(time_sig.denominator, 8);
        }

        let mut comp6 = Composition::new(Tempo::new(120.0));
        comp6.track("test").time_signature(12, 8);
        let mixer6 = comp6.into_mixer();
        if let crate::track::AudioEvent::TimeSignature(time_sig) = &mixer6.tracks()[0].events[0] {
            assert_eq!(time_sig.numerator, 12);
            assert_eq!(time_sig.denominator, 8);
        }
    }

    #[test]
    fn test_mark_saves_cursor_position() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").wait(5.0).mark("test_marker");

        assert_eq!(*comp.markers.get("test_marker").unwrap(), 5.0);
    }

    #[test]
    fn test_at_mark_moves_to_saved_position() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("track1").wait(10.0).mark("drop");

        let builder = comp.track("track2").at_mark("drop");

        assert_eq!(builder.cursor, 10.0);
    }

    #[test]
    fn test_at_mark_handles_missing_marker() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Should not panic, just leave cursor unchanged
        comp.track("test")
            .at(1.0) // Set cursor to 1.0
            .at_mark("nonexistent") // Try to use non-existent marker
            .note(&[C4], 0.5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];

        // Note should still be at 1.0 (cursor unchanged by missing marker)
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.start_time, 1.0);
        }
    }

    #[test]
    fn test_multiple_markers() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("main")
            .wait(2.0)
            .mark("verse")
            .wait(4.0)
            .mark("chorus")
            .wait(3.0)
            .mark("bridge");

        assert_eq!(*comp.markers.get("verse").unwrap(), 2.0);
        assert_eq!(*comp.markers.get("chorus").unwrap(), 6.0);
        assert_eq!(*comp.markers.get("bridge").unwrap(), 9.0);
    }

    #[test]
    fn test_markers_across_tracks() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Track 1 sets markers
        comp.track("drums")
            .note(&[C4], 0.5)
            .mark("intro_end")
            .wait(2.0)
            .mark("verse_start");

        // Track 2 uses those markers
        let bass_builder = comp.track("bass").at_mark("verse_start");

        assert_eq!(bass_builder.cursor, 2.5); // 0.5 + 2.0
    }

    #[test]
    fn test_marker_can_be_overwritten() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test1").wait(5.0).mark("position");

        comp.track("test2").wait(10.0).mark("position"); // Overwrites previous

        assert_eq!(*comp.markers.get("position").unwrap(), 10.0);
    }

    #[test]
    fn test_peek_cursor_returns_current_position() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").wait(3.5);

        assert_eq!(builder.peek_cursor(), 3.5);
    }

    #[test]
    fn test_peek_cursor_doesnt_advance() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").wait(2.0);

        let pos1 = builder.peek_cursor();
        let pos2 = builder.peek_cursor();

        assert_eq!(pos1, 2.0);
        assert_eq!(pos2, 2.0);
    }

    #[test]
    fn test_mark_and_at_mark_workflow() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Create arrangement structure with markers
        comp.track("structure")
            .mark("start")
            .wait(8.0)
            .mark("verse")
            .wait(16.0)
            .mark("chorus")
            .wait(8.0)
            .mark("bridge")
            .wait(16.0)
            .mark("outro");

        // Different instruments start at different sections
        comp.track("lead").at_mark("verse").note(&[E4], 0.5);

        comp.track("pad").at_mark("chorus").note(&[C3, E3, G3], 2.0);

        let mixer = comp.into_mixer();

        // Find tracks with notes (structure track has no notes, so only lead and pad)
        let mut note_times = vec![];
        for track in &mixer.tracks() {
            if let Some(crate::track::AudioEvent::Note(note)) = track.events.first() {
                note_times.push(note.start_time);
            }
        }

        // Sort to get consistent ordering
        note_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Verify we have 2 notes at the correct times
        assert_eq!(note_times.len(), 2);
        assert_eq!(note_times[0], 8.0); // verse
        assert_eq!(note_times[1], 24.0); // chorus
    }

    #[test]
    fn test_markers_with_complex_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("timing")
            .at(0.0)
            .wait(2.0)
            .mark("a")
            .seek(-1.0) // Back to 1.0
            .mark("b")
            .at(10.0)
            .mark("c")
            .wait(5.0)
            .seek(-3.0) // At 12.0
            .mark("d");

        assert_eq!(*comp.markers.get("a").unwrap(), 2.0);
        assert_eq!(*comp.markers.get("b").unwrap(), 1.0);
        assert_eq!(*comp.markers.get("c").unwrap(), 10.0);
        assert_eq!(*comp.markers.get("d").unwrap(), 12.0);
    }
}
