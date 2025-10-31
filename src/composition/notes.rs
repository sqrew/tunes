use super::TrackBuilder;
use crate::drums::DrumType;

impl<'a> TrackBuilder<'a> {
    /// Add a note or chord at the current cursor position
    pub fn note(mut self, frequencies: &[f32], duration: f32) -> Self {
        self.track.add_note_with_waveform_envelope_and_bend(
            frequencies,
            self.cursor,
            duration,
            self.waveform,
            self.envelope,
            self.pitch_bend,
        );
        let swung_duration = self.apply_swing(duration);
        self.cursor += swung_duration;
        self
    }

    /// Add a drum hit at the current cursor position
    pub fn drum(mut self, drum_type: DrumType) -> Self {
        self.track.add_drum(drum_type, self.cursor);
        let base_duration = drum_type.duration();
        let swung_duration = self.apply_swing(base_duration);
        self.cursor += swung_duration;
        self
    }

    /// Add an interpolated sequence starting at the current cursor position
    pub fn interpolated(
        mut self,
        start_freq: f32,
        end_freq: f32,
        segments: usize,
        note_duration: f32,
    ) -> Self {
        // Handle edge cases
        if segments == 0 {
            return self; // Nothing to play
        }
        if segments == 1 {
            // Just play the start frequency
            self.track.add_note_with_waveform_and_envelope(
                &[start_freq],
                self.cursor,
                note_duration,
                self.waveform,
                self.envelope,
            );
            self.cursor += note_duration;
            return self;
        }

        for i in 0..segments {
            let t = i as f32 / (segments - 1) as f32;
            let freq = start_freq + (end_freq - start_freq) * t;
            self.track.add_note_with_waveform_and_envelope(
                &[freq],
                self.cursor,
                note_duration,
                self.waveform,
                self.envelope,
            );
            self.cursor += note_duration;
        }
        self
    }

    /// Add a sequence of notes with equal duration starting at the current cursor position
    pub fn notes(mut self, frequencies: &[f32], note_duration: f32) -> Self {
        for &freq in frequencies {
            self.track.add_note_with_waveform_envelope_and_bend(
                &[freq],
                self.cursor,
                note_duration,
                self.waveform,
                self.envelope,
                self.pitch_bend,
            );
            let swung_duration = self.apply_swing(note_duration);
            self.cursor += swung_duration;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::Composition;
    use crate::rhythm::Tempo;
    use crate::notes::*;
    use crate::track::AudioEvent;

    #[test]
    fn test_note_adds_single_note() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").note(&[440.0], 1.0);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.frequencies[0], 440.0);
            assert_eq!(note.start_time, 0.0);
            assert_eq!(note.duration, 1.0);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_note_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").note(&[440.0], 1.0);

        // Cursor should have advanced by the note duration
        assert_eq!(builder.cursor, 1.0);
    }

    #[test]
    fn test_note_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .note(&[440.0], 0.5)
            .note(&[550.0], 0.5)
            .note(&[660.0], 0.5);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);

        // Verify timing
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.start_time, 0.5);
        }
        if let AudioEvent::Note(note) = track.events[2] {
            assert_eq!(note.start_time, 1.0);
        }
    }

    #[test]
    fn test_note_with_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").note(&[440.0, 554.37, 659.25], 1.0); // A major chord

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.num_freqs, 3);
            assert_eq!(note.frequencies[0], 440.0);
            assert_eq!(note.frequencies[1], 554.37);
            assert_eq!(note.frequencies[2], 659.25);
        }
    }

    #[test]
    fn test_drum_adds_drum_hit() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("drums").drum(DrumType::Kick);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Drum(drum) = track.events[0] {
            assert!(matches!(drum.drum_type, DrumType::Kick));
            assert_eq!(drum.start_time, 0.0);
        } else {
            panic!("Expected DrumEvent");
        }
    }

    #[test]
    fn test_drum_advances_cursor_by_drum_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("drums").drum(DrumType::Kick);

        // Cursor should advance by kick duration (0.15s)
        assert_eq!(builder.cursor, 0.15);
    }

    #[test]
    fn test_drum_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("drums")
            .drum(DrumType::Kick)
            .drum(DrumType::Snare)
            .drum(DrumType::HiHatClosed);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);

        // Verify different drum types
        if let AudioEvent::Drum(drum) = track.events[0] {
            assert!(matches!(drum.drum_type, DrumType::Kick));
        }
        if let AudioEvent::Drum(drum) = track.events[1] {
            assert!(matches!(drum.drum_type, DrumType::Snare));
        }
        if let AudioEvent::Drum(drum) = track.events[2] {
            assert!(matches!(drum.drum_type, DrumType::HiHatClosed));
        }
    }

    #[test]
    fn test_interpolated_creates_smooth_glide() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").interpolated(440.0, 880.0, 5, 0.1);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 5);

        // Verify frequencies interpolate smoothly
        let expected_freqs = [440.0, 550.0, 660.0, 770.0, 880.0];
        for (i, expected) in expected_freqs.iter().enumerate() {
            if let AudioEvent::Note(note) = track.events[i] {
                assert_eq!(note.frequencies[0], *expected);
                assert_eq!(note.start_time, i as f32 * 0.1);
            }
        }
    }

    #[test]
    fn test_interpolated_with_zero_segments() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody").interpolated(440.0, 880.0, 0, 0.1);

        // Check cursor first before moving comp
        assert_eq!(builder.cursor, 0.0, "Cursor should not advance");

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 0, "Zero segments should create no notes");
    }

    #[test]
    fn test_interpolated_with_one_segment() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").interpolated(440.0, 880.0, 1, 0.5);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.frequencies[0], 440.0); // Should use start freq
            assert_eq!(note.duration, 0.5);
        }
    }

    #[test]
    fn test_notes_creates_sequence() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let freqs = [C4, E4, G4, C5]; // C major arpeggio
        comp.track("melody").notes(&freqs, 0.25);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        for (i, &expected_freq) in freqs.iter().enumerate() {
            if let AudioEvent::Note(note) = track.events[i] {
                assert_eq!(note.frequencies[0], expected_freq);
                assert_eq!(note.start_time, i as f32 * 0.25);
                assert_eq!(note.duration, 0.25);
            }
        }
    }

    #[test]
    fn test_notes_with_empty_array() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody").notes(&[], 0.5);

        // Check cursor first before moving comp
        assert_eq!(builder.cursor, 0.0, "Cursor should not advance for empty array");

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 0);
    }

    #[test]
    fn test_notes_advances_cursor_correctly() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let freqs = [440.0, 550.0, 660.0];
        let builder = comp.track("melody").notes(&freqs, 0.5);

        // Cursor should advance by num_notes * duration
        assert_eq!(builder.cursor, 1.5); // 3 notes * 0.5s
    }

    #[test]
    fn test_mixed_notes_and_drums() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("mixed")
            .note(&[440.0], 0.5)
            .drum(DrumType::Kick)
            .note(&[550.0], 0.5)
            .drum(DrumType::Snare);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        // Verify alternating pattern
        assert!(matches!(track.events[0], AudioEvent::Note(_)));
        assert!(matches!(track.events[1], AudioEvent::Drum(_)));
        assert!(matches!(track.events[2], AudioEvent::Note(_)));
        assert!(matches!(track.events[3], AudioEvent::Drum(_)));
    }
}
