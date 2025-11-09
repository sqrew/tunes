use super::TrackBuilder;

impl<'a> TrackBuilder<'a> {
    /// Play a tuplet - evenly divide notes across a duration
    ///
    /// A tuplet fits a specific number of notes into the space normally occupied
    /// by a different number of beats. Each note gets equal duration.
    ///
    /// # Arguments
    /// * `notes` - The notes/frequencies to play in the tuplet
    /// * `count` - Number of notes to fit (should match notes.len() for even spacing)
    /// * `total_duration` - Total time span for the entire tuplet
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Play 5 notes evenly across 1 beat (quintuplet)
    /// comp.instrument("piano", &Instrument::acoustic_piano())
    ///     .tuplet(&[C4, D4, E4, F4, G4], 5, 1.0);
    /// ```
    pub fn tuplet(mut self, notes: &[f32], count: usize, total_duration: f32) -> Self {
        if count == 0 || total_duration <= 0.0 || !total_duration.is_finite() {
            return self;
        }

        let note_duration = total_duration / count as f32;
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        for &freq in notes {
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

        self.update_section_duration();
        self
    }

    /// Play a triplet - 3 notes in the space of 2
    ///
    /// A triplet fits 3 evenly-spaced notes into the time normally occupied by 2 notes.
    /// This is one of the most common tuplets in music.
    ///
    /// # Arguments
    /// * `notes` - The 3 notes/frequencies to play (or any number for convenience)
    /// * `total_duration` - Total time span (e.g., 1.0 for a quarter note triplet)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Triplet over one beat
    /// comp.instrument("piano", &Instrument::acoustic_piano())
    ///     .triplet(&[C4, E4, G4], 1.0);
    /// ```
    pub fn triplet(self, notes: &[f32], total_duration: f32) -> Self {
        self.tuplet(notes, 3, total_duration)
    }

    /// Play a quintuplet - 5 notes in the space of 4
    ///
    /// A quintuplet fits 5 evenly-spaced notes into the time normally occupied by 4 notes.
    /// Common in progressive and jazz music.
    ///
    /// # Arguments
    /// * `notes` - The 5 notes/frequencies to play (or any number for convenience)
    /// * `total_duration` - Total time span
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("drums", &Instrument::pluck())
    ///     .quintuplet(&[C4, D4, E4, F4, G4], 1.0);
    /// ```
    pub fn quintuplet(self, notes: &[f32], total_duration: f32) -> Self {
        self.tuplet(notes, 5, total_duration)
    }

    /// Play a sextuplet - 6 notes in the space of 4
    ///
    /// A sextuplet fits 6 evenly-spaced notes into the time normally occupied by 4 notes.
    /// Equivalent to two consecutive triplets.
    ///
    /// # Arguments
    /// * `notes` - The 6 notes/frequencies to play (or any number for convenience)
    /// * `total_duration` - Total time span
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("lead", &Instrument::synth_lead())
    ///     .sextuplet(&[C4, D4, E4, F4, G4, A4], 1.0);
    /// ```
    pub fn sextuplet(self, notes: &[f32], total_duration: f32) -> Self {
        self.tuplet(notes, 6, total_duration)
    }

    /// Play a septuplet - 7 notes in the space of 4 (or 8)
    ///
    /// A septuplet fits 7 evenly-spaced notes into a beat span.
    /// Commonly used in metal and progressive music.
    ///
    /// # Arguments
    /// * `notes` - The 7 notes/frequencies to play (or any number for convenience)
    /// * `total_duration` - Total time span
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("shred", &Instrument::synth_lead())
    ///     .septuplet(&[C4, D4, E4, F4, G4, A4, B4], 1.0);
    /// ```
    pub fn septuplet(self, notes: &[f32], total_duration: f32) -> Self {
        self.tuplet(notes, 7, total_duration)
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::consts::notes::*;
    use crate::composition::rhythm::Tempo;
    use crate::track::AudioEvent;

    #[test]
    fn test_tuplet_divides_duration_evenly() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").tuplet(&[C4, D4, E4], 3, 1.5);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 3);

        // Each note should have duration 1.5 / 3 = 0.5
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.duration, 0.5);
            }
        }
    }

    #[test]
    fn test_tuplet_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").tuplet(&[C4, D4, E4, F4], 4, 2.0);

        // 4 notes * 0.5 each = 2.0
        assert!((builder.cursor - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_tuplet_with_zero_count() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").tuplet(&[C4, D4], 0, 1.0);

        // Should be no-op
        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_tuplet_with_zero_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").tuplet(&[C4, D4], 2, 0.0);

        // Should be no-op
        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_tuplet_with_negative_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").tuplet(&[C4, D4], 2, -1.0);

        // Should be no-op
        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_triplet_creates_three_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").triplet(&[C4, E4, G4], 1.0);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 3);

        // Each should be 1.0 / 3 ≈ 0.333...
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert!((note.duration - 1.0 / 3.0).abs() < 0.01);
            }
        }
    }

    #[test]
    fn test_triplet_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").triplet(&[C4, E4, G4], 1.5);

        let track = &comp.into_mixer().tracks()[0];

        // Notes should start at 0.0, 0.5, 1.0
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert!((note.start_time - 0.5).abs() < 0.01);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            assert!((note.start_time - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_quintuplet_creates_five_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").quintuplet(&[C4, D4, E4, F4, G4], 2.0);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 5);

        // Each should be 2.0 / 5 = 0.4
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert!((note.duration - 0.4).abs() < 0.01);
            }
        }
    }

    #[test]
    fn test_sextuplet_creates_six_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").sextuplet(&[C4, D4, E4, F4, G4, A4], 3.0);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 6);

        // Each should be 3.0 / 6 = 0.5
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.duration, 0.5);
            }
        }
    }

    #[test]
    fn test_septuplet_creates_seven_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .septuplet(&[C4, D4, E4, F4, G4, A4, B4], 1.4);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 7);

        // Each should be 1.4 / 7 = 0.2
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert!((note.duration - 0.2).abs() < 0.01);
            }
        }
    }

    #[test]
    fn test_tuplet_with_more_notes_than_count() {
        let mut comp = Composition::new(Tempo::new(120.0));
        // 5 notes but count=3 means each gets 1.5/3 = 0.5s duration
        comp.track("test").tuplet(&[C4, D4, E4, F4, G4], 3, 1.5);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 5);

        // All notes should have the calculated duration
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.duration, 0.5);
            }
        }
    }

    #[test]
    fn test_tuplet_with_fewer_notes_than_count() {
        let mut comp = Composition::new(Tempo::new(120.0));
        // 2 notes but count=5 means each gets 2.0/5 = 0.4s duration
        comp.track("test").tuplet(&[C4, D4], 5, 2.0);

        let track = &comp.into_mixer().tracks()[0];
        assert_eq!(track.events.len(), 2);

        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert!((note.duration - 0.4).abs() < 0.01);
            }
        }
    }

    #[test]
    fn test_tuplet_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .triplet(&[C4, E4, G4], 1.0)
            .quintuplet(&[D4, F4, A4, C5, E5], 1.0);

        let track = &comp.into_mixer().tracks()[0];
        // 3 + 5 = 8 notes
        assert_eq!(track.events.len(), 8);
    }

    #[test]
    fn test_empty_notes_array() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").tuplet(&[], 3, 1.0);

        assert!((builder.cursor - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_tuplet_exact_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").tuplet(&[C4, D4, E4, F4], 4, 1.0);

        let track = &comp.into_mixer().tracks()[0];

        // Verify exact start times: 0, 0.25, 0.5, 0.75
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert!((note.start_time - 0.25).abs() < 0.001);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            assert!((note.start_time - 0.5).abs() < 0.001);
        }
        if let AudioEvent::Note(note) = &track.events[3] {
            assert!((note.start_time - 0.75).abs() < 0.001);
        }
    }

    #[test]
    fn test_triplet_advances_cursor_correctly() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").triplet(&[C4, E4, G4], 1.5);

        assert!((builder.cursor - 1.5).abs() < 0.01);
    }
}
