use super::TrackBuilder;
use crate::drums::DrumType;

impl<'a> TrackBuilder<'a> {
    /// Add a note or chord at the current cursor position
    pub fn note(mut self, frequencies: &[f32], duration: f32) -> Self {
        let cursor = self.cursor;
        let waveform = self.waveform;
        let envelope = self.envelope;
        let filter_envelope = self.filter_envelope;
        let fm_params = self.fm_params;
        let pitch_bend = self.pitch_bend;
        let custom_wavetable = self.custom_wavetable.clone();
        let velocity = self.velocity;

        self.get_track_mut().add_note_with_complete_params(
            frequencies,
            cursor,
            duration,
            waveform,
            envelope,
            filter_envelope,
            fm_params,
            pitch_bend,
            custom_wavetable,
            velocity,
        );
        let swung_duration = self.apply_swing(duration);
        self.cursor += swung_duration;
        self.update_section_duration();
        self
    }

    /// Add a drum hit at the current cursor position
    pub fn drum(mut self, drum_type: DrumType) -> Self {
        let cursor = self.cursor;
        self.get_track_mut().add_drum(drum_type, cursor);
        let base_duration = drum_type.duration();
        let swung_duration = self.apply_swing(base_duration);
        self.cursor += swung_duration;
        self.update_section_duration();
        self
    }

    /// Play a sample at the current cursor position
    ///
    /// The sample must be previously loaded using `comp.load_sample()`.
    ///
    /// # Arguments
    /// * `sample_name` - Name of the loaded sample
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.load_sample("kick", "samples/kick.wav")?;
    ///
    /// comp.track("drums")
    ///     .sample("kick")  // Play at cursor position
    ///     .sample("kick");  // Play again
    /// # Ok(())
    /// # }
    /// ```
    pub fn sample(mut self, sample_name: &str) -> Self {
        let cursor = self.cursor;

        // Get the sample from the composition's cache
        let sample = match self.composition.get_sample(sample_name) {
            Some(s) => s.clone(),
            None => {
                eprintln!(
                    "Warning: Sample '{}' not found. Load it first with comp.load_sample(). Skipping sample event.",
                    sample_name
                );
                return self;
            }
        };

        // Add the sample event
        use crate::track::{AudioEvent, SampleEvent};
        let sample_event = SampleEvent::new(sample.clone(), cursor);
        let duration = sample.duration;

        self.get_track_mut()
            .events
            .push(AudioEvent::Sample(sample_event));
        self.get_track_mut().invalidate_time_cache();

        let swung_duration = self.apply_swing(duration);
        self.cursor += swung_duration;
        self.update_section_duration();
        self
    }

    /// Play a sample with custom playback rate
    ///
    /// # Arguments
    /// * `sample_name` - Name of the loaded sample
    /// * `playback_rate` - Speed multiplier (1.0 = normal, 2.0 = double speed/octave up, 0.5 = half speed/octave down)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.load_sample("kick", "samples/kick.wav")?;
    ///
    /// comp.track("drums")
    ///     .sample_with_rate("kick", 1.0)   // Normal speed
    ///     .sample_with_rate("kick", 2.0)   // Double speed (octave up)
    ///     .sample_with_rate("kick", 0.5);  // Half speed (octave down)
    /// # Ok(())
    /// # }
    /// ```
    pub fn sample_with_rate(mut self, sample_name: &str, playback_rate: f32) -> Self {
        let cursor = self.cursor;

        let sample = match self.composition.get_sample(sample_name) {
            Some(s) => s.clone(),
            None => {
                eprintln!(
                    "Warning: Sample '{}' not found. Load it first with comp.load_sample(). Skipping sample event.",
                    sample_name
                );
                return self;
            }
        };

        use crate::track::{AudioEvent, SampleEvent};
        let sample_event =
            SampleEvent::new(sample.clone(), cursor).with_playback_rate(playback_rate);
        let duration = sample.duration / playback_rate;

        self.get_track_mut()
            .events
            .push(AudioEvent::Sample(sample_event));
        self.get_track_mut().invalidate_time_cache();

        let swung_duration = self.apply_swing(duration);
        self.cursor += swung_duration;
        self.update_section_duration();
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

        let waveform = self.waveform;
        let envelope = self.envelope;

        if segments == 1 {
            // Just play the start frequency
            let cursor = self.cursor;
            self.get_track_mut().add_note_with_waveform_and_envelope(
                &[start_freq],
                cursor,
                note_duration,
                waveform,
                envelope,
            );
            self.cursor += note_duration;
            self.update_section_duration();
            return self;
        }

        for i in 0..segments {
            let t = i as f32 / (segments - 1) as f32;
            let freq = start_freq + (end_freq - start_freq) * t;
            let cursor = self.cursor;
            self.get_track_mut().add_note_with_waveform_and_envelope(
                &[freq],
                cursor,
                note_duration,
                waveform,
                envelope,
            );
            self.cursor += note_duration;
        }
        self.update_section_duration();
        self
    }

    /// Add a sequence of notes with equal duration starting at the current cursor position
    pub fn notes(mut self, frequencies: &[f32], note_duration: f32) -> Self {
        let waveform = self.waveform;
        let envelope = self.envelope;
        let filter_envelope = self.filter_envelope;
        let fm_params = self.fm_params;
        let pitch_bend = self.pitch_bend;
        let custom_wavetable = self.custom_wavetable.clone();
        let velocity = self.velocity;

        for &freq in frequencies {
            let cursor = self.cursor;
            self.get_track_mut().add_note_with_complete_params(
                &[freq],
                cursor,
                note_duration,
                waveform,
                envelope,
                filter_envelope,
                fm_params,
                pitch_bend,
                custom_wavetable.clone(),
                velocity,
            );
            let swung_duration = self.apply_swing(note_duration);
            self.cursor += swung_duration;
        }
        self.update_section_duration();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::Composition;
    use crate::notes::*;
    use crate::rhythm::Tempo;
    use crate::track::AudioEvent;

    #[test]
    fn test_note_adds_single_note() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").note(&[440.0], 1.0);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
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
    fn test_note_with_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").note(&[440.0, 554.37, 659.25], 1.0); // A major chord

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
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

        if let AudioEvent::Drum(drum) = &track.events[0] {
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
        if let AudioEvent::Drum(drum) = &track.events[0] {
            assert!(matches!(drum.drum_type, DrumType::Kick));
        }
        if let AudioEvent::Drum(drum) = &track.events[1] {
            assert!(matches!(drum.drum_type, DrumType::Snare));
        }
        if let AudioEvent::Drum(drum) = &track.events[2] {
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
            if let AudioEvent::Note(note) = &track.events[i] {
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

        let mixer = comp.into_mixer();
        // With zero segments, no track is created since interpolated returns early
        assert_eq!(
            mixer.tracks.len(),
            0,
            "Zero segments should create no track"
        );
    }

    #[test]
    fn test_interpolated_with_one_segment() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").interpolated(440.0, 880.0, 1, 0.5);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
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
            if let AudioEvent::Note(note) = &track.events[i] {
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
        assert_eq!(
            builder.cursor, 0.0,
            "Cursor should not advance for empty array"
        );

        let mixer = comp.into_mixer();
        // With empty array, no track is created since loop doesn't execute
        assert_eq!(mixer.tracks.len(), 0, "Empty array should create no track");
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
