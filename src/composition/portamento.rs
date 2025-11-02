use super::TrackBuilder;

impl<'a> TrackBuilder<'a> {
    pub fn portamento(
        mut self,
        start_freq: f32,
        end_freq: f32,
        scale: &[f32],
        note_duration: f32,
    ) -> Self {
        // Find scale notes in the range between start and end
        let (min_freq, max_freq) = if start_freq < end_freq {
            (start_freq, end_freq)
        } else {
            (end_freq, start_freq)
        };

        let scale_notes: Vec<f32> = scale
            .iter()
            .copied()
            .filter(|&f| f >= min_freq && f <= max_freq)
            .collect();

        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        if scale_notes.is_empty() {
            // No scale notes in range, just play start and end
            let cursor = self.cursor;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[start_freq],
                    cursor,
                    note_duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            self.cursor += note_duration;

            let cursor = self.cursor;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[end_freq],
                    cursor,
                    note_duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            self.cursor += note_duration;
        } else {
            // Play all scale notes in order (ascending or descending)
            if start_freq < end_freq {
                // Ascending
                for &freq in &scale_notes {
                    let cursor = self.cursor;
                    self.get_track_mut()
                        .add_note_with_waveform_envelope_and_bend(
                            &[freq],
                            cursor,
                            note_duration,
                            waveform,
                            envelope,
                            pitch_bend,
                        );
                    let swung_duration = self.apply_swing(note_duration);
                    self.cursor += swung_duration;
                }
            } else {
                // Descending
                for &freq in scale_notes.iter().rev() {
                    let cursor = self.cursor;
                    self.get_track_mut()
                        .add_note_with_waveform_envelope_and_bend(
                            &[freq],
                            cursor,
                            note_duration,
                            waveform,
                            envelope,
                            pitch_bend,
                        );
                    let swung_duration = self.apply_swing(note_duration);
                    self.cursor += swung_duration;
                }
            }
        }

        self.update_section_duration();
        self
    }

    /// Smooth glide/slide between two notes
    ///
    /// Creates a smooth portamento effect by interpolating between two frequencies.
    /// Unlike `.interpolated()` which requires specifying segment count, this uses
    /// a reasonable default based on the duration for smooth results.
    ///
    /// # Arguments
    /// * `from` - Starting frequency
    /// * `to` - Ending frequency
    /// * `duration` - Total duration of the slide
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("lead", &Instrument::synth_lead())
    ///     .slide(C4, G4, 0.5);  // Smooth glide from C4 to G4 over 0.5 seconds
    /// ```
    pub fn slide(mut self, from: f32, to: f32, duration: f32) -> Self {
        if duration <= 0.0 || !duration.is_finite() {
            return self;
        }

        // Use enough segments for smooth interpolation (roughly 50ms per segment)
        let segments = ((duration / 0.05).ceil() as usize).max(4);
        let note_duration = duration / segments as f32;
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        for i in 0..segments {
            let t = i as f32 / (segments - 1) as f32;
            let freq = from + (to - from) * t;
            let cursor = self.cursor;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[freq],
                    cursor,
                    note_duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            self.cursor += note_duration;
        }
        self.update_section_duration();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::notes::*;
    use crate::rhythm::Tempo;
    use crate::track::AudioEvent;

    #[test]
    fn test_portamento_ascending() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let scale = [C4, D4, E4, F4, G4, A4, B4, C5];
        comp.track("port").portamento(D4, A4, &scale, 0.1);

        let track = &comp.into_mixer().tracks[0];
        // Should play: D4, E4, F4, G4, A4 (notes in range)
        assert_eq!(track.events.len(), 5);

        // Verify order
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], D4);
        }
        if let AudioEvent::Note(note) = &track.events[4] {
            assert_eq!(note.frequencies[0], A4);
        }
    }

    #[test]
    fn test_portamento_descending() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let scale = [C4, D4, E4, F4, G4];
        comp.track("port").portamento(G4, C4, &scale, 0.1);

        let track = &comp.into_mixer().tracks[0];
        // Should play notes in reverse: G4, F4, E4, D4, C4
        assert_eq!(track.events.len(), 5);

        // Verify descending order
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], G4);
        }
        if let AudioEvent::Note(note) = &track.events[4] {
            assert_eq!(note.frequencies[0], C4);
        }
    }

    #[test]
    fn test_portamento_no_scale_notes_in_range() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let scale = [C4, D4, E4]; // All below the range
        comp.track("port").portamento(F4, G4, &scale, 0.2);

        let track = &comp.into_mixer().tracks[0];
        // Should just play start and end
        assert_eq!(track.events.len(), 2);

        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], F4);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.frequencies[0], G4);
        }
    }

    #[test]
    fn test_portamento_empty_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("port").portamento(C4, E4, &[], 0.1);

        let track = &comp.into_mixer().tracks[0];
        // Should play start and end
        assert_eq!(track.events.len(), 2);
    }

    #[test]
    fn test_portamento_single_note_in_range() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let scale = [C4, E4, G4];
        comp.track("port").portamento(D4, F4, &scale, 0.1);

        let track = &comp.into_mixer().tracks[0];
        // Only E4 is in range [D4, F4]
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], E4);
        }
    }

    #[test]
    fn test_slide_creates_smooth_glide() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("slide").slide(C4, G4, 0.4);

        let track = &comp.into_mixer().tracks[0];
        // 0.4 / 0.05 = 8 segments minimum, max with 4
        assert!(track.events.len() >= 4);

        // First note should be close to C4
        if let AudioEvent::Note(note) = &track.events[0] {
            assert!((note.frequencies[0] - C4).abs() < 1.0);
        }

        // Last note should be close to G4
        let last_idx = track.events.len() - 1;
        if let AudioEvent::Note(note) = &track.events[last_idx] {
            assert!((note.frequencies[0] - G4).abs() < 1.0);
        }
    }

    #[test]
    fn test_slide_interpolates_correctly() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("slide").slide(100.0, 200.0, 0.2);

        let track = &comp.into_mixer().tracks[0];

        // First note should be 100
        if let AudioEvent::Note(note) = &track.events[0] {
            assert!((note.frequencies[0] - 100.0).abs() < 0.1);
        }

        // Last note should be 200
        let last_idx = track.events.len() - 1;
        if let AudioEvent::Note(note) = &track.events[last_idx] {
            assert!((note.frequencies[0] - 200.0).abs() < 0.1);
        }
    }

    #[test]
    fn test_slide_with_zero_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("slide").slide(C4, G4, 0.0);

        // Should be no-op
        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_slide_with_negative_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("slide").slide(C4, G4, -0.1);

        // Should be no-op
        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_slide_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("slide").slide(C4, G4, 0.5);

        // Should advance by total duration
        assert!((builder.cursor - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_portamento_and_slide_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let scale = [C4, D4, E4, F4, G4];
        comp.track("combo")
            .portamento(C4, E4, &scale, 0.1)
            .slide(G4, C5, 0.3);

        let track = &comp.into_mixer().tracks[0];
        // Should have events from both portamento and slide
        assert!(track.events.len() > 3);
    }

    #[test]
    fn test_slide_descending() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("down").slide(G4, C4, 0.3);

        let track = &comp.into_mixer().tracks[0];

        // First should be near G4
        if let AudioEvent::Note(note) = &track.events[0] {
            assert!((note.frequencies[0] - G4).abs() < 1.0);
        }

        // Last should be near C4
        let last_idx = track.events.len() - 1;
        if let AudioEvent::Note(note) = &track.events[last_idx] {
            assert!((note.frequencies[0] - C4).abs() < 1.0);
        }
    }

    #[test]
    fn test_portamento_boundaries_inclusive() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let scale = [C4, D4, E4];
        comp.track("port").portamento(C4, E4, &scale, 0.1);

        let track = &comp.into_mixer().tracks[0];
        // Should include both C4 and E4
        assert_eq!(track.events.len(), 3);
    }

    #[test]
    fn test_slide_minimum_segments() {
        let mut comp = Composition::new(Tempo::new(120.0));
        // Very short duration should still use minimum 4 segments
        comp.track("short").slide(C4, D4, 0.01);

        let track = &comp.into_mixer().tracks[0];
        assert!(track.events.len() >= 4, "Should use minimum 4 segments");
    }
}
