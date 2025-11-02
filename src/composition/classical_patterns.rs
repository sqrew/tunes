use super::TrackBuilder;

impl<'a> TrackBuilder<'a> {
    /// Alberti bass pattern - the quintessential Classical-era accompaniment
    ///
    /// Plays a broken chord in the pattern: lowest-highest-middle-highest
    /// (e.g., C-G-E-G for a C major triad). Named after Domenico Alberti,
    /// this pattern is ubiquitous in Mozart, Haydn, and Classical piano music.
    ///
    /// # Arguments
    /// * `chord` - The chord to arpeggiate (typically a triad)
    /// * `note_duration` - Duration of each note in the pattern
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("piano_left", &Instrument::acoustic_piano())
    ///     .alberti_bass(&[C3, E3, G3], 0.125);  // Classic C major Alberti bass
    ///     // Plays: C3, G3, E3, G3
    /// ```
    pub fn alberti_bass(mut self, chord: &[f32], note_duration: f32) -> Self {
        if chord.len() < 3 {
            // Need at least 3 notes for Alberti pattern, fall back to simple arpeggio
            return self.arpeggiate(chord, note_duration);
        }

        // Classic pattern: lowest (0), highest (n-1), middle (1), highest (n-1)
        let pattern = [0, chord.len() - 1, 1, chord.len() - 1];
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        for &index in &pattern {
            let cursor = self.cursor;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[chord[index]],
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

    /// Waltz bass pattern - the classic oom-pah-pah accompaniment
    ///
    /// Plays root note on beat 1, then chord on beats 2 and 3.
    /// The quintessential 3/4 time waltz accompaniment pattern.
    ///
    /// # Arguments
    /// * `root` - The bass root note (plays on beat 1)
    /// * `chord` - The chord to play on beats 2 and 3
    /// * `beat_duration` - Duration of each beat
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("waltz_bass", &Instrument::acoustic_piano())
    ///     .waltz_bass(C2, &[C3, E3, G3], 0.5);  // One measure of waltz
    ///     // Plays: C2 (low), then [C3,E3,G3] chord twice
    /// ```
    pub fn waltz_bass(mut self, root: f32, chord: &[f32], beat_duration: f32) -> Self {
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        // Beat 1: Root note alone
        let cursor = self.cursor;
        self.get_track_mut()
            .add_note_with_waveform_envelope_and_bend(
                &[root],
                cursor,
                beat_duration,
                waveform,
                envelope,
                pitch_bend,
            );
        self.cursor += beat_duration;

        // Beats 2 and 3: Chord
        for _ in 0..2 {
            let cursor = self.cursor;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    chord,
                    cursor,
                    beat_duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            self.cursor += beat_duration;
        }

        self.update_section_duration();
        self
    }

    /// Stride piano pattern - classic jazz/ragtime left hand
    ///
    /// Alternates between a low bass note (beats 1 & 3) and a mid-range chord (beats 2 & 4).
    /// The signature "boom-chick" sound of stride and ragtime piano.
    ///
    /// # Arguments
    /// * `root` - The low bass note
    /// * `chord` - The mid-range chord
    /// * `beat_duration` - Duration of each beat
    /// * `measures` - Number of measures to play (4 beats per measure)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("stride", &Instrument::acoustic_piano())
    ///     .stride_bass(C2, &[C3, E3, G3], 0.5, 2);  // Two measures of stride
    ///     // Plays: C2, chord, C2, chord, C2, chord, C2, chord
    /// ```
    pub fn stride_bass(
        mut self,
        root: f32,
        chord: &[f32],
        beat_duration: f32,
        measures: usize,
    ) -> Self {
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        for _ in 0..(measures * 2) {
            // Beats 1 & 3: Bass note
            let cursor = self.cursor;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[root],
                    cursor,
                    beat_duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            self.cursor += beat_duration;

            // Beats 2 & 4: Chord
            let cursor = self.cursor;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    chord,
                    cursor,
                    beat_duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            self.cursor += beat_duration;
        }

        self.update_section_duration();
        self
    }

    /// Broken chord pattern - various classical accompaniment patterns
    ///
    /// Plays a chord in different broken patterns commonly used in Classical music.
    ///
    /// # Pattern Types
    /// * 0: Up and back - C, E, G, E (returns to middle)
    /// * 1: Down and back - G, E, C, E (descends then returns)
    /// * 2: Up twice - C, E, G, C, E, G (repeats ascending)
    /// * 3: Ascending pairs - C, C, E, E, G, G (doubled notes)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("arpeggios", &Instrument::acoustic_piano())
    ///     .broken_chord(&[C4, E4, G4], 0, 0.125);  // Up and back pattern
    /// ```
    pub fn broken_chord(mut self, chord: &[f32], pattern_type: u8, note_duration: f32) -> Self {
        if chord.is_empty() {
            return self;
        }

        let pattern: Vec<usize> = match pattern_type {
            0 => {
                // Up and back: 0, 1, 2, 1
                if chord.len() >= 3 {
                    vec![0, 1, 2, 1]
                } else {
                    (0..chord.len()).collect()
                }
            }
            1 => {
                // Down and back: 2, 1, 0, 1
                if chord.len() >= 3 {
                    vec![2, 1, 0, 1]
                } else {
                    (0..chord.len()).rev().collect()
                }
            }
            2 => {
                // Up twice: 0, 1, 2, 0, 1, 2
                let indices: Vec<usize> = (0..chord.len()).collect();
                indices
                    .iter()
                    .cycle()
                    .take(chord.len() * 2)
                    .copied()
                    .collect()
            }
            3 => {
                // Ascending pairs: 0, 0, 1, 1, 2, 2
                (0..chord.len()).flat_map(|i| vec![i, i]).collect()
            }
            _ => (0..chord.len()).collect(), // Default: simple ascending
        };

        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        for index in pattern {
            if index < chord.len() {
                let cursor = self.cursor;
                self.get_track_mut()
                    .add_note_with_waveform_envelope_and_bend(
                        &[chord[index]],
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

        self.update_section_duration();
        self
    }

    /// Baroque walking bass - stepwise bass line movement
    ///
    /// Creates a smooth, stepwise bass line by playing each note in the sequence.
    /// Common in Baroque music for creating harmonic foundation with melodic interest.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("baroque_bass", &Instrument::upright_bass())
    ///     .walking_bass(&[C2, D2, E2, F2, G2, F2, E2, D2], 0.25);
    /// ```
    pub fn walking_bass(self, bass_line: &[f32], note_duration: f32) -> Self {
        // Walking bass is just a melodic line in the bass register
        self.notes(bass_line, note_duration)
    }

    /// Tremolo strings pattern - rapid note repetition
    ///
    /// Creates the classic string tremolo effect by rapidly repeating notes.
    /// More musical than raw tremolo() as it's designed for sustained string effects.
    ///
    /// # Arguments
    /// * `notes` - The note(s) to tremolo (can be chord)
    /// * `total_duration` - Total duration of the tremolo
    /// * `note_speed` - Duration of each individual note repetition (e.g., 0.03 for fast tremolo)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("strings", &Instrument::strings())
    ///     .tremolo_strings(&[C4, E4, G4], 2.0, 0.03);  // Tremolo chord for 2 seconds
    /// ```
    pub fn tremolo_strings(mut self, notes: &[f32], total_duration: f32, note_speed: f32) -> Self {
        if note_speed <= 0.0 || !note_speed.is_finite() || total_duration <= 0.0 {
            return self;
        }

        let repetitions = (total_duration / note_speed).floor() as usize;
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        for _ in 0..repetitions {
            let cursor = self.cursor;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    notes, cursor, note_speed, waveform, envelope, pitch_bend,
                );
            self.cursor += note_speed;
        }

        self.update_section_duration();
        self
    }

    /// Ostinato pattern - repeating musical phrase
    ///
    /// Repeats a melodic or rhythmic pattern N times. Common in Baroque and minimalist music.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("ostinato", &Instrument::pluck())
    ///     .ostinato(&[C4, E4, G4, E4], 0.125, 8);  // Repeat pattern 8 times
    /// ```
    pub fn ostinato(mut self, pattern: &[f32], note_duration: f32, repeats: usize) -> Self {
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        for _ in 0..repeats {
            for &freq in pattern {
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

        self.update_section_duration();
        self
    }

    /// Pedal point pattern - sustained note with changing harmonies above
    ///
    /// A common classical technique where a bass note is held while harmonies change above it.
    /// This is an alternative to the pedal() method with more classical-focused usage.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// use tunes::notes::*;
    /// use tunes::theory::chord;
    /// use tunes::theory::ChordPattern;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    ///
    /// comp.instrument("organ", &Instrument::warm_pad())
    ///     .pedal_point(
    ///         C2,
    ///         &[
    ///             chord(C3, &ChordPattern::MAJOR),
    ///             chord(F3, &ChordPattern::MAJOR),
    ///             chord(G3, &ChordPattern::MAJOR),
    ///         ],
    ///         1.0
    ///     );  // C pedal with changing chords above
    /// ```
    pub fn pedal_point(
        mut self,
        pedal_note: f32,
        chord_sequence: &[Vec<f32>],
        chord_duration: f32,
    ) -> Self {
        let start_cursor = self.cursor;
        let total_duration = chord_sequence.len() as f32 * chord_duration;
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        // Add sustained pedal note for entire duration
        self.get_track_mut()
            .add_note_with_waveform_envelope_and_bend(
                &[pedal_note],
                start_cursor,
                total_duration,
                waveform,
                envelope,
                pitch_bend,
            );

        // Add chord progression above the pedal
        for chord in chord_sequence {
            let cursor = self.cursor;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    chord.as_slice(),
                    cursor,
                    chord_duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            self.cursor += chord_duration;
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
    fn test_alberti_bass_creates_correct_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").alberti_bass(&[C3, E3, G3], 0.125);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        // Pattern should be: lowest (0), highest (2), middle (1), highest (2)
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], C3); // Lowest
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.frequencies[0], G3); // Highest
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            assert_eq!(note.frequencies[0], E3); // Middle
        }
        if let AudioEvent::Note(note) = &track.events[3] {
            assert_eq!(note.frequencies[0], G3); // Highest again
        }
    }

    #[test]
    fn test_alberti_bass_with_less_than_3_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").alberti_bass(&[C3, E3], 0.125);

        let track = &comp.into_mixer().tracks[0];
        // Falls back to arpeggiate with 2 notes
        assert_eq!(track.events.len(), 2);
    }

    #[test]
    fn test_alberti_bass_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").alberti_bass(&[C3, E3, G3], 0.125);

        // 4 notes * 0.125 = 0.5
        assert!((builder.cursor - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_waltz_bass_creates_three_beats() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").waltz_bass(C2, &[C3, E3, G3], 0.5);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);

        // Beat 1: Root alone
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], C2);
            // Check it's just one note (rest should be 0)
            let non_zero = note.frequencies.iter().filter(|&&f| f != 0.0).count();
            assert_eq!(non_zero, 1);
        }

        // Beats 2 & 3: Chord
        if let AudioEvent::Note(note) = &track.events[1] {
            assert!(note.frequencies.contains(&C3));
            assert!(note.frequencies.contains(&E3));
            assert!(note.frequencies.contains(&G3));
            let non_zero = note.frequencies.iter().filter(|&&f| f != 0.0).count();
            assert_eq!(non_zero, 3);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            let non_zero = note.frequencies.iter().filter(|&&f| f != 0.0).count();
            assert_eq!(non_zero, 3);
        }
    }

    #[test]
    fn test_waltz_bass_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").waltz_bass(C2, &[C3, E3, G3], 0.5);

        let track = &comp.into_mixer().tracks[0];

        // Verify timing: 0.0, 0.5, 1.0
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
    fn test_stride_bass_alternates_correctly() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").stride_bass(C2, &[C3, E3, G3], 0.5, 1);

        let track = &comp.into_mixer().tracks[0];
        // 1 measure * 4 beats per measure (2 bass-chord pairs) = 4 notes
        assert_eq!(track.events.len(), 4);

        // Should alternate: bass, chord, bass, chord
        if let AudioEvent::Note(note) = &track.events[0] {
            let non_zero = note.frequencies.iter().filter(|&&f| f != 0.0).count();
            assert_eq!(non_zero, 1);
            assert_eq!(note.frequencies[0], C2);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            let non_zero = note.frequencies.iter().filter(|&&f| f != 0.0).count();
            assert_eq!(non_zero, 3);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            let non_zero = note.frequencies.iter().filter(|&&f| f != 0.0).count();
            assert_eq!(non_zero, 1);
        }
        if let AudioEvent::Note(note) = &track.events[3] {
            let non_zero = note.frequencies.iter().filter(|&&f| f != 0.0).count();
            assert_eq!(non_zero, 3);
        }
    }

    #[test]
    fn test_stride_bass_multiple_measures() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").stride_bass(C2, &[C3, E3], 0.25, 2);

        // Total time: 8 * 0.25 = 2.0
        assert!((builder.cursor - 2.0).abs() < 0.01);

        let track = &comp.into_mixer().tracks[0];
        // 2 measures * 2 pairs * 2 notes = 8 notes
        assert_eq!(track.events.len(), 8);
    }

    #[test]
    fn test_broken_chord_pattern_0_up_and_back() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").broken_chord(&[C4, E4, G4], 0, 0.125);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        // Pattern 0: Up and back - 0, 1, 2, 1
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], C4);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.frequencies[0], E4);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            assert_eq!(note.frequencies[0], G4);
        }
        if let AudioEvent::Note(note) = &track.events[3] {
            assert_eq!(note.frequencies[0], E4);
        }
    }

    #[test]
    fn test_broken_chord_pattern_1_down_and_back() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").broken_chord(&[C4, E4, G4], 1, 0.125);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        // Pattern 1: Down and back - 2, 1, 0, 1
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], G4);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.frequencies[0], E4);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            assert_eq!(note.frequencies[0], C4);
        }
        if let AudioEvent::Note(note) = &track.events[3] {
            assert_eq!(note.frequencies[0], E4);
        }
    }

    #[test]
    fn test_broken_chord_pattern_2_up_twice() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").broken_chord(&[C4, E4, G4], 2, 0.125);

        let track = &comp.into_mixer().tracks[0];
        // Pattern 2: Up twice - 0, 1, 2, 0, 1, 2
        assert_eq!(track.events.len(), 6);
    }

    #[test]
    fn test_broken_chord_pattern_3_ascending_pairs() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").broken_chord(&[C4, E4, G4], 3, 0.125);

        let track = &comp.into_mixer().tracks[0];
        // Pattern 3: Ascending pairs - 0, 0, 1, 1, 2, 2
        assert_eq!(track.events.len(), 6);

        // Check pairs
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], C4);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.frequencies[0], C4);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            assert_eq!(note.frequencies[0], E4);
        }
        if let AudioEvent::Note(note) = &track.events[3] {
            assert_eq!(note.frequencies[0], E4);
        }
    }

    #[test]
    fn test_broken_chord_empty_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").broken_chord(&[], 0, 0.125);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_walking_bass_creates_bass_line() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").walking_bass(&[C2, D2, E2, F2], 0.25);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        // Verify notes in order
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], C2);
        }
        if let AudioEvent::Note(note) = &track.events[3] {
            assert_eq!(note.frequencies[0], F2);
        }
    }

    #[test]
    fn test_tremolo_strings_repetition() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").tremolo_strings(&[C4, E4, G4], 0.3, 0.05);

        let track = &comp.into_mixer().tracks[0];
        // 0.3 / 0.05 = 6 repetitions
        assert_eq!(track.events.len(), 6);

        // All should be the same chord
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                let non_zero = note.frequencies.iter().filter(|&&f| f != 0.0).count();
                assert_eq!(non_zero, 3);
                assert_eq!(note.duration, 0.05);
            }
        }
    }

    #[test]
    fn test_tremolo_strings_with_zero_speed() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").tremolo_strings(&[C4], 1.0, 0.0);

        // Should be no-op
        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_tremolo_strings_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").tremolo_strings(&[C4], 0.5, 0.1);

        // 0.5 / 0.1 = 5 repetitions * 0.1 = 0.5
        assert!((builder.cursor - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_ostinato_repeats_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").ostinato(&[C4, E4, G4], 0.125, 3);

        let track = &comp.into_mixer().tracks[0];
        // 3 notes * 3 repeats = 9 notes
        assert_eq!(track.events.len(), 9);

        // Verify pattern repeats
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], C4);
        }
        if let AudioEvent::Note(note) = &track.events[3] {
            assert_eq!(note.frequencies[0], C4); // Pattern starts again
        }
        if let AudioEvent::Note(note) = &track.events[6] {
            assert_eq!(note.frequencies[0], C4); // Pattern starts third time
        }
    }

    #[test]
    fn test_ostinato_zero_repeats() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").ostinato(&[C4, E4], 0.125, 0);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_ostinato_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").ostinato(&[C4, E4, G4], 0.1, 4);

        // 3 notes * 4 repeats * 0.1 = 1.2
        assert!((builder.cursor - 1.2).abs() < 0.01);
    }

    #[test]
    fn test_pedal_point_creates_sustained_bass() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let chords = vec![vec![C3, E3, G3], vec![F3, A3, C4], vec![G3, B3, D4]];
        comp.track("test").pedal_point(C2, &chords, 1.0);

        let track = &comp.into_mixer().tracks[0];
        // 1 pedal note + 3 chords = 4 events
        assert_eq!(track.events.len(), 4);

        // First event should be the pedal note with long duration
        if let AudioEvent::Note(note) = &track.events[0] {
            let non_zero = note.frequencies.iter().filter(|&&f| f != 0.0).count();
            assert_eq!(non_zero, 1);
            assert_eq!(note.frequencies[0], C2);
            assert_eq!(note.duration, 3.0); // Total duration of all chords
        }

        // Subsequent events should be the chords
        if let AudioEvent::Note(note) = &track.events[1] {
            let non_zero = note.frequencies.iter().filter(|&&f| f != 0.0).count();
            assert_eq!(non_zero, 3);
            assert_eq!(note.duration, 1.0);
        }
    }

    #[test]
    fn test_pedal_point_chord_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let chords = vec![vec![C3, E3, G3], vec![F3, A3, C4]];
        comp.track("test").pedal_point(C2, &chords, 0.5);

        let track = &comp.into_mixer().tracks[0];

        // Check chord timing
        if let AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            assert!((note.start_time - 0.5).abs() < 0.01);
        }
    }

    #[test]
    fn test_pedal_point_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let chords = vec![vec![C3, E3, G3], vec![F3, A3, C4], vec![G3, B3, D4]];
        let builder = comp.track("test").pedal_point(C2, &chords, 0.75);

        // 3 chords * 0.75 = 2.25
        assert!((builder.cursor - 2.25).abs() < 0.01);
    }

    #[test]
    fn test_alberti_bass_with_four_note_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").alberti_bass(&[C3, E3, G3, C4], 0.125);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        // Pattern: lowest (0), highest (3), middle (1), highest (3)
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.frequencies[0], C3);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.frequencies[0], C4);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            assert_eq!(note.frequencies[0], E3);
        }
        if let AudioEvent::Note(note) = &track.events[3] {
            assert_eq!(note.frequencies[0], C4);
        }
    }

    #[test]
    fn test_stride_bass_zero_measures() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").stride_bass(C2, &[C3, E3], 0.5, 0);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_broken_chord_invalid_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").broken_chord(&[C4, E4, G4], 99, 0.125);

        let track = &comp.into_mixer().tracks[0];
        // Should default to simple ascending
        assert_eq!(track.events.len(), 3);
    }

    #[test]
    fn test_ostinato_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").ostinato(&[], 0.125, 5);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_pedal_point_empty_chords() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let chords: Vec<Vec<f32>> = vec![];
        let builder = comp.track("test").pedal_point(C2, &chords, 1.0);

        // Cursor should not advance
        assert_eq!(builder.cursor, 0.0);

        let track = &comp.into_mixer().tracks[0];
        // Should only have the pedal note with 0 duration
        assert_eq!(track.events.len(), 1);
    }
}
