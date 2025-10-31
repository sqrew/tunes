use super::TrackBuilder;

impl<'a> TrackBuilder<'a> {
    pub fn pattern_start(mut self) -> Self {
        self.pattern_start = self.cursor;
        self
    }

    /// Repeat the pattern from pattern_start to current cursor position N times
    pub fn repeat(mut self, times: usize) -> Self {
        if times == 0 {
            return self;
        }

        let pattern_duration = self.cursor - self.pattern_start;
        if pattern_duration <= 0.0 {
            return self;
        }

        // Collect events in the pattern range
        let pattern_events: Vec<_> = self
            .track
            .events
            .iter()
            .filter(|event| {
                let event_time = match event {
                    crate::track::AudioEvent::Note(note) => note.start_time,
                    crate::track::AudioEvent::Drum(drum) => drum.start_time,
                };
                event_time >= self.pattern_start && event_time < self.cursor
            })
            .cloned()
            .collect();

        // Repeat the pattern
        for i in 0..times {
            let offset = pattern_duration * (i + 1) as f32;
            for event in &pattern_events {
                match event {
                    crate::track::AudioEvent::Note(note) => {
                        self.track.add_note_with_waveform_envelope_and_bend(
                            &note.frequencies[..note.num_freqs],
                            note.start_time + offset,
                            note.duration,
                            note.waveform,
                            note.envelope,
                            note.pitch_bend_semitones,
                        );
                    }
                    crate::track::AudioEvent::Drum(drum) => {
                        self.track
                            .add_drum(drum.drum_type, drum.start_time + offset);
                    }
                }
            }
        }

        // Move cursor to end of all repeats
        self.cursor += pattern_duration * times as f32;
        self
    }

    /// Reverse the pattern from pattern_start to current cursor
    ///
    /// Reverses the order of notes in the pattern while maintaining timing.
    /// If you played C4→D4→E4 with 0.1s spacing, reverse gives E4→D4→C4 with same spacing.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # use tunes::scales::C4_MAJOR_SCALE;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("melody", &Instrument::pluck())
    ///     .pattern_start()
    ///     .scale(&C4_MAJOR_SCALE, 0.1)    // C4→D4→E4→F4→G4→A4→B4→C5
    ///     .reverse();                     // C5→B4→A4→G4→F4→E4→D4→C4
    pub fn reverse(self) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        // Collect events in the pattern range
        let mut pattern_events: Vec<_> = self
            .track
            .events
            .iter()
            .filter(|event| {
                let event_time = match event {
                    crate::track::AudioEvent::Note(note) => note.start_time,
                    crate::track::AudioEvent::Drum(drum) => drum.start_time,
                };
                event_time >= self.pattern_start && event_time < self.cursor
            })
            .cloned()
            .collect();

        if pattern_events.is_empty() {
            return self;
        }

        // Sort events by time
        pattern_events.sort_by(|a, b| {
            let time_a = match a {
                crate::track::AudioEvent::Note(n) => n.start_time,
                crate::track::AudioEvent::Drum(d) => d.start_time,
            };
            let time_b = match b {
                crate::track::AudioEvent::Note(n) => n.start_time,
                crate::track::AudioEvent::Drum(d) => d.start_time,
            };
            // Handle NaN values - treat them as equal (shouldn't happen, but safe)
            time_a
                .partial_cmp(&time_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Extract just the note/drum data (pitches, drum types)
        let note_data: Vec<_> = pattern_events
            .iter()
            .map(|event| match event {
                crate::track::AudioEvent::Note(note) => (
                    note.frequencies,
                    note.num_freqs,
                    note.duration,
                    note.waveform,
                    note.envelope,
                    note.pitch_bend_semitones,
                    true,
                ),
                crate::track::AudioEvent::Drum(_drum) => (
                    [0.0; 8],
                    0,
                    0.0,
                    crate::waveform::Waveform::Sine,
                    crate::envelope::Envelope::new(0.0, 0.0, 0.0, 0.0),
                    0.0,
                    false,
                ),
            })
            .collect();

        let drum_data: Vec<_> = pattern_events
            .iter()
            .filter_map(|event| match event {
                crate::track::AudioEvent::Drum(drum) => Some(drum.drum_type),
                _ => None,
            })
            .collect();

        // Get timing information
        let timings: Vec<f32> = pattern_events
            .iter()
            .map(|event| match event {
                crate::track::AudioEvent::Note(n) => n.start_time,
                crate::track::AudioEvent::Drum(d) => d.start_time,
            })
            .collect();

        // Remove original events from track
        self.track.events.retain(|event| {
            let event_time = match event {
                crate::track::AudioEvent::Note(note) => note.start_time,
                crate::track::AudioEvent::Drum(drum) => drum.start_time,
            };
            event_time < self.pattern_start || event_time >= self.cursor
        });

        // Re-add events with reversed note/drum data but original timings
        let mut drum_idx = drum_data.len();
        for (i, &timing) in timings.iter().enumerate() {
            let reversed_idx = pattern_events.len() - 1 - i;

            match &pattern_events[reversed_idx] {
                crate::track::AudioEvent::Note(_) => {
                    let (freqs, num_freqs, duration, waveform, envelope, bend, _) =
                        note_data[reversed_idx];
                    self.track.add_note_with_waveform_envelope_and_bend(
                        &freqs[..num_freqs],
                        timing,
                        duration,
                        waveform,
                        envelope,
                        bend,
                    );
                }
                crate::track::AudioEvent::Drum(_) => {
                    drum_idx -= 1;
                    if drum_idx < drum_data.len() {
                        self.track.add_drum(drum_data[drum_idx], timing);
                    }
                }
            }
        }

        self
    }

    pub fn repeat_last(mut self, duration: f32, times: usize) -> Self {
        if times == 0 || duration <= 0.0 {
            return self;
        }

        let pattern_start = (self.cursor - duration).max(0.0);

        // Collect events in the last N seconds
        let pattern_events: Vec<_> = self
            .track
            .events
            .iter()
            .filter(|event| {
                let event_time = match event {
                    crate::track::AudioEvent::Note(note) => note.start_time,
                    crate::track::AudioEvent::Drum(drum) => drum.start_time,
                };
                event_time >= pattern_start && event_time < self.cursor
            })
            .cloned()
            .collect();

        // Repeat the pattern
        for i in 0..times {
            let offset = duration * (i + 1) as f32;
            for event in &pattern_events {
                match event {
                    crate::track::AudioEvent::Note(note) => {
                        self.track.add_note_with_waveform_envelope_and_bend(
                            &note.frequencies[..note.num_freqs],
                            note.start_time + offset,
                            note.duration,
                            note.waveform,
                            note.envelope,
                            note.pitch_bend_semitones,
                        );
                    }
                    crate::track::AudioEvent::Drum(drum) => {
                        self.track
                            .add_drum(drum.drum_type, drum.start_time + offset);
                    }
                }
            }
        }

        // Move cursor to end of all repeats
        self.cursor += duration * times as f32;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::drums::DrumType;
    use crate::notes::*;
    use crate::rhythm::Tempo;
    use crate::track::AudioEvent;

    #[test]
    fn test_pattern_start_marks_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").wait(2.0).pattern_start();

        assert_eq!(builder.pattern_start, 2.0);
    }

    #[test]
    fn test_repeat_duplicates_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.5)
            .note(&[E4], 0.5)
            .repeat(2); // Repeat pattern 2 more times

        let track = &comp.into_mixer().tracks[0];
        // Original 2 notes + 2 repeats * 2 notes = 6 total
        assert_eq!(track.events.len(), 6);
    }

    #[test]
    fn test_repeat_maintains_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[440.0], 0.25)
            .note(&[550.0], 0.25)
            .repeat(1); // Repeat once

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        // Original pattern
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.start_time, 0.0);
            assert_eq!(note.frequencies[0], 440.0);
        }
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.start_time, 0.25);
            assert_eq!(note.frequencies[0], 550.0);
        }

        // Repeated pattern (starts at 0.5)
        if let AudioEvent::Note(note) = track.events[2] {
            assert_eq!(note.start_time, 0.5);
            assert_eq!(note.frequencies[0], 440.0);
        }
        if let AudioEvent::Note(note) = track.events[3] {
            assert_eq!(note.start_time, 0.75);
            assert_eq!(note.frequencies[0], 550.0);
        }
    }

    #[test]
    fn test_repeat_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody")
            .pattern_start()
            .note(&[440.0], 0.5)
            .note(&[550.0], 0.5)
            .repeat(2);

        // Original pattern = 1.0s, repeated 2 times = 2.0s more
        // Total cursor should be at 3.0s
        assert_eq!(builder.cursor, 3.0);
    }

    #[test]
    fn test_repeat_with_zero_times() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[440.0], 0.5)
            .repeat(0);

        let track = &comp.into_mixer().tracks[0];
        // Should only have original note
        assert_eq!(track.events.len(), 1);
    }

    #[test]
    fn test_repeat_with_drums() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("drums")
            .pattern_start()
            .drum(DrumType::Kick)
            .drum(DrumType::Snare)
            .repeat(1);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4); // 2 original + 2 repeated
    }

    #[test]
    fn test_repeat_with_offset_pattern_start() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .note(&[C4], 0.5)       // Not in pattern
            .pattern_start()         // Mark start here
            .note(&[E4], 0.5)       // In pattern
            .note(&[G4], 0.5)       // In pattern
            .repeat(1);

        let track = &comp.into_mixer().tracks[0];
        // Should have 1 note before pattern + 2 in pattern + 2 repeated = 5
        assert_eq!(track.events.len(), 5);
    }

    #[test]
    fn test_reverse_flips_note_order() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.25)
            .note(&[E4], 0.25)
            .note(&[G4], 0.25)
            .reverse();

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);

        // Timing should stay the same, but notes reversed
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.start_time, 0.0);
            assert_eq!(note.frequencies[0], G4); // Was last, now first
        }
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.start_time, 0.25);
            assert_eq!(note.frequencies[0], E4); // Middle stays middle
        }
        if let AudioEvent::Note(note) = track.events[2] {
            assert_eq!(note.start_time, 0.5);
            assert_eq!(note.frequencies[0], C4); // Was first, now last
        }
    }

    #[test]
    fn test_reverse_with_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody").pattern_start().reverse();

        // Should be no-op
        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_reverse_with_single_note() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[440.0], 0.5)
            .reverse();

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        // Single note should be unchanged
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.frequencies[0], 440.0);
            assert_eq!(note.start_time, 0.0);
        }
    }

    #[test]
    fn test_reverse_maintains_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.25)
            .note(&[E4], 0.25)
            .reverse();

        // Cursor should remain at end
        assert_eq!(builder.cursor, 0.5);
    }

    #[test]
    fn test_repeat_last_duplicates_recent_events() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .note(&[C4], 0.5)
            .note(&[E4], 0.5)
            .repeat_last(1.0, 2); // Repeat last 1 second, 2 times

        let track = &comp.into_mixer().tracks[0];
        // 2 original + 2 repeated twice = 6 total
        assert_eq!(track.events.len(), 6);
    }

    #[test]
    fn test_repeat_last_with_zero_times() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .note(&[440.0], 0.5)
            .repeat_last(0.5, 0);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1); // Only original note
    }

    #[test]
    fn test_repeat_last_with_zero_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody")
            .note(&[440.0], 0.5)
            .repeat_last(0.0, 2);

        // Check cursor first before moving comp
        assert_eq!(builder.cursor, 0.5); // Cursor unchanged

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1); // No repeat should happen
    }

    #[test]
    fn test_repeat_last_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody")
            .note(&[440.0], 0.5)
            .repeat_last(0.5, 2);

        // Original 0.5s + 2 repeats * 0.5s = 1.5s total
        assert_eq!(builder.cursor, 1.5);
    }

    #[test]
    fn test_repeat_last_partial_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .note(&[C4], 0.5)       // Starts at 0.0
            .note(&[E4], 0.5)       // Starts at 0.5
            .note(&[G4], 0.5)       // Starts at 1.0
            .repeat_last(0.6, 1);   // Repeat last 0.6s (1.5 - 0.6 = 0.9, so captures G4 starting at 1.0)

        let track = &comp.into_mixer().tracks[0];
        // 3 original + 1 repeated (G4) = 4 total
        assert_eq!(track.events.len(), 4);
    }

    #[test]
    fn test_pattern_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.25)
            .note(&[E4], 0.25)
            .repeat(1)              // Now have 4 notes
            .reverse();             // Reverse all 4

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);
    }

    #[test]
    fn test_repeat_with_mixed_notes_and_drums() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("mixed")
            .pattern_start()
            .note(&[440.0], 0.25)
            .drum(DrumType::Kick)
            .repeat(1);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4); // 1 note + 1 drum, repeated = 4 total

        // Verify mix of types
        assert!(matches!(track.events[0], AudioEvent::Note(_)));
        assert!(matches!(track.events[1], AudioEvent::Drum(_)));
        assert!(matches!(track.events[2], AudioEvent::Note(_)));
        assert!(matches!(track.events[3], AudioEvent::Drum(_)));
    }

    #[test]
    fn test_complex_pattern_workflow() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)  // 3 notes in pattern
            .repeat(2)                     // Repeat 2 more times = 9 notes total
            .pattern_start()               // Mark new pattern start
            .note(&[C5], 0.5)             // Add one more note
            .repeat(1);                    // Repeat just this last note

        // Verify cursor advanced correctly first
        // 3 notes * 0.25 = 0.75 original
        // 0.75 * 2 = 1.5 for repeats
        // Total after first repeat = 2.25
        // Then 1 note * 0.5 = 0.5
        // Total = 2.75
        // Then repeat last note once = +0.5
        // Final = 3.25
        assert_eq!(builder.cursor, 3.25);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 11); // 9 + 2 = 11
    }

    #[test]
    fn test_reverse_preserves_note_properties() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[440.0, 550.0], 0.5)  // Chord with 2 frequencies
            .note(&[660.0], 0.25)         // Single note
            .reverse();

        let track = &comp.into_mixer().tracks[0];

        // First note should now be the single note (was second)
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.num_freqs, 1);
            assert_eq!(note.frequencies[0], 660.0);
            assert_eq!(note.duration, 0.25);
        }

        // Second note should now be the chord (was first)
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.num_freqs, 2);
            assert_eq!(note.frequencies[0], 440.0);
            assert_eq!(note.frequencies[1], 550.0);
            assert_eq!(note.duration, 0.5);
        }
    }
}
