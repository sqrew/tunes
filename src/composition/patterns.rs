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

    /// Apply speed modification to the current pattern
    ///
    /// Multiplies the speed of all events in the pattern range.
    /// - `speed > 1.0`: Faster (events compressed in time)
    /// - `speed < 1.0`: Slower (events stretched in time)
    /// - `speed = 2.0`: Double-time (all durations halved)
    /// - `speed = 0.5`: Half-time (all durations doubled)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .note(&[C4], 0.5)
    ///     .note(&[E4], 0.5)
    ///     .note(&[G4], 0.5)
    ///     .speed(2.0);  // Play twice as fast (0.25s per note)
    /// ```
    pub fn speed(mut self, factor: f32) -> Self {
        if factor <= 0.0 || !factor.is_finite() {
            return self;
        }

        let pattern_duration = self.cursor - self.pattern_start;
        if pattern_duration <= 0.0 {
            return self;
        }

        // Collect and modify events in the pattern range
        let modified_events: Vec<_> = self
            .track
            .events
            .iter()
            .filter_map(|event| {
                let event_time = match event {
                    crate::track::AudioEvent::Note(note) => note.start_time,
                    crate::track::AudioEvent::Drum(drum) => drum.start_time,
                };

                if event_time >= self.pattern_start && event_time < self.cursor {
                    // Time relative to pattern start, scaled by speed
                    let relative_time = event_time - self.pattern_start;
                    let new_time = self.pattern_start + (relative_time / factor);

                    match event {
                        crate::track::AudioEvent::Note(note) => {
                            Some((
                                true,
                                note.frequencies,
                                note.num_freqs,
                                note.duration / factor,  // Scale duration
                                new_time,
                                note.waveform,
                                note.envelope,
                                note.pitch_bend_semitones,
                                crate::drums::DrumType::Kick,
                            ))
                        }
                        crate::track::AudioEvent::Drum(drum) => {
                            Some((
                                false,
                                [0.0; 8],
                                0,
                                0.0,
                                new_time,
                                crate::waveform::Waveform::Sine,
                                crate::envelope::Envelope::new(0.0, 0.0, 0.0, 0.0),
                                0.0,
                                drum.drum_type,
                            ))
                        }
                    }
                } else {
                    None
                }
            })
            .collect();

        // Remove original events from pattern range
        self.track.events.retain(|event| {
            let event_time = match event {
                crate::track::AudioEvent::Note(note) => note.start_time,
                crate::track::AudioEvent::Drum(drum) => drum.start_time,
            };
            event_time < self.pattern_start || event_time >= self.cursor
        });

        // Add modified events
        for (is_note, freqs, num_freqs, duration, time, waveform, envelope, bend, drum_type) in modified_events {
            if is_note {
                self.track.add_note_with_waveform_envelope_and_bend(
                    &freqs[..num_freqs],
                    time,
                    duration,
                    waveform,
                    envelope,
                    bend,
                );
            } else {
                self.track.add_drum(drum_type, time);
            }
        }

        // Adjust cursor: new pattern duration is original / factor
        self.cursor = self.pattern_start + (pattern_duration / factor);
        self
    }

    /// Apply probability filter to the current pattern
    ///
    /// Each event in the pattern has a probability chance of being kept.
    /// Great for creating variation and humanization in generative music.
    ///
    /// # Arguments
    /// * `probability` - Chance each event plays (0.0 to 1.0)
    ///   - `1.0` = all events play (no effect)
    ///   - `0.5` = each event has 50% chance
    ///   - `0.0` = no events play (silence)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("hihat")
    ///     .pattern_start()
    ///     .note(&[/* hihat freq */], 0.125)
    ///     .repeat(16)
    ///     .probability(0.7);  // Each hit has 70% chance to play
    /// ```
    pub fn probability(mut self, prob: f32) -> Self {
        let prob = prob.clamp(0.0, 1.0);

        if prob >= 1.0 {
            return self;  // No filtering needed
        }

        if prob <= 0.0 {
            // Remove all events in pattern range
            self.track.events.retain(|event| {
                let event_time = match event {
                    crate::track::AudioEvent::Note(note) => note.start_time,
                    crate::track::AudioEvent::Drum(drum) => drum.start_time,
                };
                event_time < self.pattern_start || event_time >= self.cursor
            });
            return self;
        }

        // Filter events probabilistically
        use rand::Rng;
        let mut rng = rand::thread_rng();

        self.track.events.retain(|event| {
            let event_time = match event {
                crate::track::AudioEvent::Note(note) => note.start_time,
                crate::track::AudioEvent::Drum(drum) => drum.start_time,
            };

            // Keep events outside pattern range
            if event_time < self.pattern_start || event_time >= self.cursor {
                return true;
            }

            // Probabilistically keep events inside pattern range
            rng.gen::<f32>() < prob
        });

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

    #[test]
    fn test_speed_doubles_tempo() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.5)
            .note(&[E4], 0.5)
            .note(&[G4], 0.5)
            .speed(2.0);  // Double speed

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);

        // Durations should be halved
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.duration, 0.25);
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.duration, 0.25);
            assert_eq!(note.start_time, 0.25);
        }
        if let AudioEvent::Note(note) = track.events[2] {
            assert_eq!(note.duration, 0.25);
            assert_eq!(note.start_time, 0.5);
        }
    }

    #[test]
    fn test_speed_halves_tempo() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.5)
            .note(&[E4], 0.5)
            .speed(0.5);  // Half speed (slower)

        let track = &comp.into_mixer().tracks[0];

        // Durations should be doubled
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.duration, 1.0);
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.duration, 1.0);
            assert_eq!(note.start_time, 1.0);
        }
    }

    #[test]
    fn test_speed_adjusts_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.5)
            .note(&[E4], 0.5)
            .note(&[G4], 0.5)
            .speed(2.0);

        // Original pattern = 1.5s, at 2x speed = 0.75s
        assert!((builder.cursor - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_speed_with_zero_factor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.5)
            .speed(0.0);

        // Should be no-op
        assert_eq!(builder.cursor, 0.5);
    }

    #[test]
    fn test_speed_with_negative_factor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.5)
            .speed(-2.0);

        // Should be no-op
        assert_eq!(builder.cursor, 0.5);
    }

    #[test]
    fn test_speed_preserves_note_properties() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[C4, E4, G4], 0.5)  // Chord
            .speed(2.0);

        let track = &comp.into_mixer().tracks[0];

        if let AudioEvent::Note(note) = track.events[0] {
            // Chord should still be a chord
            assert_eq!(note.num_freqs, 3);
            assert_eq!(note.frequencies[0], C4);
            assert_eq!(note.frequencies[1], E4);
            assert_eq!(note.frequencies[2], G4);
            // Duration should be halved
            assert_eq!(note.duration, 0.25);
        }
    }

    #[test]
    fn test_probability_removes_some_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.125)
            .repeat(31)  // 32 total notes
            .probability(0.5);  // 50% chance each

        let track = &comp.into_mixer().tracks[0];

        // With 50% probability and 32 notes, we expect roughly 16 notes
        // Allow for variance (between 8 and 24 should be reasonable)
        assert!(track.events.len() >= 8);
        assert!(track.events.len() <= 24);
    }

    #[test]
    fn test_probability_with_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.5)
            .note(&[E4], 0.5)
            .note(&[G4], 0.5)
            .probability(0.0);  // 0% chance = silence

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 0);
    }

    #[test]
    fn test_probability_with_one() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.5)
            .note(&[E4], 0.5)
            .note(&[G4], 0.5)
            .probability(1.0);  // 100% chance = all notes

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);
    }

    #[test]
    fn test_probability_doesnt_affect_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.5)
            .note(&[E4], 0.5)
            .note(&[G4], 0.5)
            .probability(0.5);

        // Cursor should remain at 1.5 regardless of which notes were removed
        assert_eq!(builder.cursor, 1.5);
    }

    #[test]
    fn test_probability_only_affects_pattern_range() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .note(&[C4], 0.5)       // Before pattern - should stay
            .pattern_start()
            .note(&[E4], 0.5)       // In pattern - probabilistic
            .note(&[G4], 0.5)       // In pattern - probabilistic
            .probability(0.0);

        let track = &comp.into_mixer().tracks[0];

        // Should still have the note before pattern_start
        assert_eq!(track.events.len(), 1);
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.frequencies[0], C4);
        }
    }

    #[test]
    fn test_speed_and_probability_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.5)
            .note(&[E4], 0.5)
            .note(&[G4], 0.5)
            .note(&[C5], 0.5)
            .speed(2.0)          // First double the speed
            .probability(1.0);   // Then keep all (no filtering)

        let track = &comp.into_mixer().tracks[0];

        // Should have all 4 notes with halved durations
        assert_eq!(track.events.len(), 4);
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.duration, 0.25);
        }
    }

    #[test]
    fn test_speed_with_drums() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("drums")
            .pattern_start()
            .drum(DrumType::Kick)
            .at(0.5)
            .drum(DrumType::Snare)
            .at(1.0)
            .speed(2.0);

        let track = &comp.into_mixer().tracks[0];

        // Check timing compression
        if let AudioEvent::Drum(drum) = track.events[0] {
            assert_eq!(drum.start_time, 0.0);
        }
        if let AudioEvent::Drum(drum) = track.events[1] {
            assert!((drum.start_time - 0.25).abs() < 0.001);  // Was 0.5, now 0.25
        }
    }

    #[test]
    fn test_probability_with_drums() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("drums")
            .pattern_start()
            .drum(DrumType::Kick)
            .drum(DrumType::Kick)
            .drum(DrumType::Kick)
            .drum(DrumType::Kick)
            .probability(1.0);  // All should stay

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);
    }

    #[test]
    fn test_complex_generative_workflow() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .repeat(3)               // 16 notes total
            .speed(1.5)              // Make it faster
            .probability(0.8);       // Remove ~20% of notes

        let track = &comp.into_mixer().tracks[0];

        // Should have some notes (probabilistic, so can't be exact)
        assert!(track.events.len() > 0);
        assert!(track.events.len() <= 16);

        // Durations should be scaled by speed
        if let AudioEvent::Note(note) = track.events[0] {
            let expected_duration = 0.25 / 1.5;
            assert!((note.duration - expected_duration).abs() < 0.001);
        }
    }
}
