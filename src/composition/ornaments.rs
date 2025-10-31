use super::TrackBuilder;

impl<'a> TrackBuilder<'a> {
    /// Rapidly alternate between two notes (trill)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("trill", &Instrument::pluck())
    ///     .trill(C4, D4, 16, 0.05);  // Alternates C4-D4-C4-D4... 16 times total
    /// ```
    pub fn trill(mut self, note1: f32, note2: f32, count: usize, note_duration: f32) -> Self {
        for i in 0..count {
            let freq = if i % 2 == 0 { note1 } else { note2 };
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
    /// Create a cascade/waterfall effect - notes start staggered but sustain
    ///
    /// Each note in the sequence starts slightly offset from the previous one,
    /// creating a lush, layered effect as they overlap.
    ///
    /// # Arguments
    /// * `notes` - The notes to cascade
    /// * `note_duration` - How long each note sustains
    /// * `stagger` - Time offset between each note start (smaller = more overlap)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// use tunes::chords::C4_MAJOR;
    /// comp.instrument("cascade", &Instrument::warm_pad())
    ///     .cascade(C4_MAJOR, 2.0, 0.1);  // Notes start 0.1s apart but sustain 2s each
    /// ```
    pub fn cascade(mut self, notes: &[f32], note_duration: f32, stagger: f32) -> Self {
        let start_cursor = self.cursor;
        for (i, &freq) in notes.iter().enumerate() {
            self.track.add_note_with_waveform_envelope_and_bend(
                &[freq],
                start_cursor + (i as f32 * stagger),
                note_duration,
                self.waveform,
                self.envelope,
                self.pitch_bend,
            );
        }
        // Move cursor to the end of the cascade (last note start + its duration)
        self.cursor = start_cursor + ((notes.len() - 1) as f32 * stagger) + note_duration;
        self
    }
    /// Rapidly repeat the same note (tremolo effect)
    ///
    /// Creates a rapid repetition of a single note, common in strings and synth pads.
    /// Different from trill which alternates between two notes.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("strings", &Instrument::pluck())
    ///     .tremolo(C4, 16, 0.05);  // Rapid C4-C4-C4-C4... (16 times)
    /// ```
    pub fn tremolo(mut self, note: f32, count: usize, note_duration: f32) -> Self {
        for _ in 0..count {
            self.track.add_note_with_waveform_envelope_and_bend(
                &[note],
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
    pub fn strum(self, chord: &[f32], note_duration: f32, stagger: f32) -> Self {
        // Strum is just a fast cascade - keep it simple
        self.cascade(chord, note_duration, stagger)
    }

    /// Grace note ornament
    ///
    /// Plays a quick ornamental note before the main note.
    /// Common in classical, jazz, and folk music.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("ornament", &Instrument::pluck())
    ///     .grace(C4, B3, 0.05, 0.5);  // Quick B3 grace note before sustained C4
    pub fn grace(
        mut self,
        main_note: f32,
        grace_note: f32,
        grace_duration: f32,
        main_duration: f32,
    ) -> Self {
        // Play grace note
        self.track.add_note_with_waveform_envelope_and_bend(
            &[grace_note],
            self.cursor,
            grace_duration,
            self.waveform,
            self.envelope,
            self.pitch_bend,
        );
        self.cursor += grace_duration;

        // Play main note
        self.track.add_note_with_waveform_envelope_and_bend(
            &[main_note],
            self.cursor,
            main_duration,
            self.waveform,
            self.envelope,
            self.pitch_bend,
        );
        self.cursor += main_duration;

        self
    }

    /// Classical mordent ornament
    ///
    /// Rapidly plays: main note → upper neighbor → main note
    /// Creates a quick decorative flourish common in Baroque and Classical music.
    ///
    /// # Arguments
    /// * `main_note` - The principal note
    /// * `duration` - Total duration for the ornament (typically 0.1-0.2 seconds)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("harpsichord", &Instrument::harpsichord())
    ///     .mordent(C4, 0.15);  // Quick C4→D4→C4 ornament
    /// ```
    pub fn mordent(mut self, main_note: f32, duration: f32) -> Self {
        if duration <= 0.0 || !duration.is_finite() { return self; }
        let note_duration = duration / 3.0;
        let upper_note = main_note * 2.0f32.powf(2.0 / 12.0); // Whole step up

        // Main → Upper → Main
        self.track.add_note_with_waveform_envelope_and_bend(
            &[main_note],
            self.cursor,
            note_duration,
            self.waveform,
            self.envelope,
            self.pitch_bend,
        );
        self.cursor += note_duration;

        self.track.add_note_with_waveform_envelope_and_bend(
            &[upper_note],
            self.cursor,
            note_duration,
            self.waveform,
            self.envelope,
            self.pitch_bend,
        );
        self.cursor += note_duration;

        self.track.add_note_with_waveform_envelope_and_bend(
            &[main_note],
            self.cursor,
            note_duration,
            self.waveform,
            self.envelope,
            self.pitch_bend,
        );
        self.cursor += note_duration;

        self
    }

    /// Classical inverted mordent ornament
    ///
    /// Rapidly plays: main note → lower neighbor → main note
    /// The inverted version of the standard mordent, going down instead of up.
    ///
    /// # Arguments
    /// * `main_note` - The principal note
    /// * `duration` - Total duration for the ornament (typically 0.1-0.2 seconds)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("harpsichord", &Instrument::harpsichord())
    ///     .inverted_mordent(C4, 0.15);  // Quick C4→B3→C4 ornament
    /// ```
    pub fn inverted_mordent(mut self, main_note: f32, duration: f32) -> Self {
        if duration <= 0.0 || !duration.is_finite() { return self; }
        let note_duration = duration / 3.0;
        let lower_note = main_note * 2.0f32.powf(-2.0 / 12.0); // Whole step down

        // Main → Lower → Main
        self.track.add_note_with_waveform_envelope_and_bend(
            &[main_note],
            self.cursor,
            note_duration,
            self.waveform,
            self.envelope,
            self.pitch_bend,
        );
        self.cursor += note_duration;

        self.track.add_note_with_waveform_envelope_and_bend(
            &[lower_note],
            self.cursor,
            note_duration,
            self.waveform,
            self.envelope,
            self.pitch_bend,
        );
        self.cursor += note_duration;

        self.track.add_note_with_waveform_envelope_and_bend(
            &[main_note],
            self.cursor,
            note_duration,
            self.waveform,
            self.envelope,
            self.pitch_bend,
        );
        self.cursor += note_duration;

        self
    }

    /// Classical turn ornament
    ///
    /// Rapidly plays: upper neighbor → main note → lower neighbor → main note
    /// A common embellishment in Classical and Romantic music.
    ///
    /// # Arguments
    /// * `main_note` - The principal note
    /// * `duration` - Total duration for the ornament (typically 0.15-0.3 seconds)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("flute", &Instrument::synth_lead())
    ///     .turn(E4, 0.2);  // F4→E4→D4→E4 ornament
    /// ```
    pub fn turn(mut self, main_note: f32, duration: f32) -> Self {
        if duration <= 0.0 || !duration.is_finite() { return self; }
        let note_duration = duration / 4.0;
        let upper_note = main_note * 2.0f32.powf(2.0 / 12.0); // Whole step up
        let lower_note = main_note * 2.0f32.powf(-2.0 / 12.0); // Whole step down

        // Upper → Main → Lower → Main
        let notes = [upper_note, main_note, lower_note, main_note];

        for &note in &notes {
            self.track.add_note_with_waveform_envelope_and_bend(
                &[note],
                self.cursor,
                note_duration,
                self.waveform,
                self.envelope,
                self.pitch_bend,
            );
            self.cursor += note_duration;
        }

        self
    }

    /// Classical inverted turn ornament
    ///
    /// Rapidly plays: lower neighbor → main note → upper neighbor → main note
    /// The inverted version of the standard turn, starting from below.
    ///
    /// # Arguments
    /// * `main_note` - The principal note
    /// * `duration` - Total duration for the ornament (typically 0.15-0.3 seconds)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("flute", &Instrument::synth_lead())
    ///     .inverted_turn(E4, 0.2);  // D4→E4→F4→E4 ornament
    /// ```
    pub fn inverted_turn(mut self, main_note: f32, duration: f32) -> Self {
        if duration <= 0.0 || !duration.is_finite() { return self; }
        let note_duration = duration / 4.0;
        let upper_note = main_note * 2.0f32.powf(2.0 / 12.0); // Whole step up
        let lower_note = main_note * 2.0f32.powf(-2.0 / 12.0); // Whole step down

        // Lower → Main → Upper → Main
        let notes = [lower_note, main_note, upper_note, main_note];

        for &note in &notes {
            self.track.add_note_with_waveform_envelope_and_bend(
                &[note],
                self.cursor,
                note_duration,
                self.waveform,
                self.envelope,
                self.pitch_bend,
            );
            self.cursor += note_duration;
        }

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
    fn test_trill_alternates_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("trill").trill(C4, D4, 6, 0.1);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 6);

        // Should alternate: C4, D4, C4, D4, C4, D4
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.frequencies[0], C4);
        }
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.frequencies[0], D4);
        }
        if let AudioEvent::Note(note) = track.events[2] {
            assert_eq!(note.frequencies[0], C4);
        }
        if let AudioEvent::Note(note) = track.events[3] {
            assert_eq!(note.frequencies[0], D4);
        }
    }

    #[test]
    fn test_trill_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("trill").trill(C4, D4, 4, 0.25);

        // 4 notes * 0.25 = 1.0
        assert_eq!(builder.cursor, 1.0);
    }

    #[test]
    fn test_trill_with_zero_count() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("trill").trill(C4, D4, 0, 0.1);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 0);
    }

    #[test]
    fn test_cascade_staggers_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let chord = [C4, E4, G4];
        comp.track("cascade").cascade(&chord, 2.0, 0.1);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);

        // Notes should start at 0.0, 0.1, 0.2 but all sustain 2.0s
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.start_time, 0.0);
            assert_eq!(note.duration, 2.0);
        }
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.start_time, 0.1);
            assert_eq!(note.duration, 2.0);
        }
        if let AudioEvent::Note(note) = track.events[2] {
            assert_eq!(note.start_time, 0.2);
            assert_eq!(note.duration, 2.0);
        }
    }

    #[test]
    fn test_cascade_cursor_position() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("cascade").cascade(&[C4, E4, G4], 2.0, 0.1);

        // Last note starts at 0.2 (2 staggers of 0.1) + duration 2.0 = 2.2
        assert_eq!(builder.cursor, 2.2);
    }

    #[test]
    fn test_tremolo_repeats_note() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("tremolo").tremolo(C4, 5, 0.05);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 5);

        // All should be same note
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.frequencies[0], C4);
            }
        }
    }

    #[test]
    fn test_tremolo_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("tremolo").tremolo(C4, 10, 0.1);

        // 10 notes * 0.1 = 1.0 (with floating point tolerance)
        assert!((builder.cursor - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_strum_is_cascade() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let chord = [C4, E4, G4];
        comp.track("strum").strum(&chord, 1.0, 0.05);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);

        // Should behave like cascade with small stagger
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.start_time, 0.05);
        }
    }

    #[test]
    fn test_grace_plays_two_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("grace").grace(C4, B3, 0.05, 0.5);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 2);

        // Grace note first
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.frequencies[0], B3);
            assert_eq!(note.start_time, 0.0);
            assert_eq!(note.duration, 0.05);
        }

        // Main note second
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.frequencies[0], C4);
            assert_eq!(note.start_time, 0.05);
            assert_eq!(note.duration, 0.5);
        }
    }

    #[test]
    fn test_grace_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("grace").grace(C4, B3, 0.1, 0.9);

        // 0.1 + 0.9 = 1.0
        assert_eq!(builder.cursor, 1.0);
    }

    #[test]
    fn test_mordent_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("mord").mordent(C4, 0.3);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);

        // Pattern: Main → Upper → Main
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.frequencies[0], C4); // Main
            assert_eq!(note.duration, 0.1); // 0.3 / 3
        }
        if let AudioEvent::Note(note) = track.events[1] {
            // Upper (whole step up = 2 semitones)
            let expected_upper = C4 * 2.0f32.powf(2.0 / 12.0);
            assert!((note.frequencies[0] - expected_upper).abs() < 0.1);
            assert_eq!(note.duration, 0.1);
        }
        if let AudioEvent::Note(note) = track.events[2] {
            assert_eq!(note.frequencies[0], C4); // Main
        }
    }

    #[test]
    fn test_mordent_with_zero_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("mord").mordent(C4, 0.0);

        // Should be no-op
        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_inverted_mordent_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("inv_mord").inverted_mordent(C4, 0.3);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);

        // Pattern: Main → Lower → Main
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.frequencies[0], C4); // Main
        }
        if let AudioEvent::Note(note) = track.events[1] {
            // Lower (whole step down = -2 semitones)
            let expected_lower = C4 * 2.0f32.powf(-2.0 / 12.0);
            assert!((note.frequencies[0] - expected_lower).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = track.events[2] {
            assert_eq!(note.frequencies[0], C4); // Main
        }
    }

    #[test]
    fn test_turn_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("turn").turn(C4, 0.4);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        // Pattern: Upper → Main → Lower → Main
        let upper = C4 * 2.0f32.powf(2.0 / 12.0);
        let lower = C4 * 2.0f32.powf(-2.0 / 12.0);

        if let AudioEvent::Note(note) = track.events[0] {
            assert!((note.frequencies[0] - upper).abs() < 0.1);
            assert_eq!(note.duration, 0.1); // 0.4 / 4
        }
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.frequencies[0], C4);
        }
        if let AudioEvent::Note(note) = track.events[2] {
            assert!((note.frequencies[0] - lower).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = track.events[3] {
            assert_eq!(note.frequencies[0], C4);
        }
    }

    #[test]
    fn test_inverted_turn_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("inv_turn").inverted_turn(C4, 0.4);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        // Pattern: Lower → Main → Upper → Main
        let upper = C4 * 2.0f32.powf(2.0 / 12.0);
        let lower = C4 * 2.0f32.powf(-2.0 / 12.0);

        if let AudioEvent::Note(note) = track.events[0] {
            assert!((note.frequencies[0] - lower).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.frequencies[0], C4);
        }
        if let AudioEvent::Note(note) = track.events[2] {
            assert!((note.frequencies[0] - upper).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = track.events[3] {
            assert_eq!(note.frequencies[0], C4);
        }
    }

    #[test]
    fn test_ornament_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("chain")
            .grace(C4, B3, 0.05, 0.5)
            .trill(D4, E4, 4, 0.1)
            .tremolo(G4, 3, 0.1);

        let track = &comp.into_mixer().tracks[0];
        // 2 (grace) + 4 (trill) + 3 (tremolo) = 9 total
        assert_eq!(track.events.len(), 9);
    }

    #[test]
    fn test_turn_with_negative_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("neg").turn(C4, -0.1);

        // Should be no-op due to guard
        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_mordent_advances_cursor_correctly() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("mord").mordent(C4, 0.6);

        // 0.6 total duration
        assert_eq!(builder.cursor, 0.6);
    }
}
