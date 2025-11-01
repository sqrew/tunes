//! Generative and algorithmic music composition tools
//!
//! This module provides tools for creating music algorithmically, including
//! random walks, sequence transformations, and pattern manipulations.

use super::TrackBuilder;
use crate::track::AudioEvent;
use rand::Rng;

/// Generate a random walk sequence that can be used with sequence_from()
///
/// Creates a sequence of integers that randomly walk up and down, staying within bounds.
///
/// # Arguments
/// * `start` - Starting value
/// * `steps` - Number of steps to generate
/// * `min` - Minimum value (inclusive)
/// * `max` - Maximum value (exclusive)
///
/// # Example
/// ```
/// # use tunes::composition::generative::random_walk_sequence;
/// # use tunes::scales::C4_MAJOR_SCALE;
/// let walk = random_walk_sequence(3, 32, 0, 7);  // Walk through scale indices
/// // Use with: comp.track("melody").sequence_from(&walk, &C4_MAJOR_SCALE, 0.25);
/// ```
pub fn random_walk_sequence(start: u32, steps: usize, min: u32, max: u32) -> Vec<u32> {
    if steps == 0 || min >= max {
        return Vec::new();
    }

    let mut sequence = Vec::with_capacity(steps);
    let mut current = start.clamp(min, max - 1);
    let mut rng = rand::rng();

    for _ in 0..steps {
        sequence.push(current);

        // Random step: -2, -1, +1, or +2
        let step = if rng.random::<bool>() {
            rng.random_range(1..=2) // Move up
        } else {
            -(rng.random_range(1..=2) as i32) // Move down
        };

        current = ((current as i32 + step).clamp(min as i32, (max - 1) as i32)) as u32;
    }

    sequence
}

/// Generate a biased random walk sequence with tendency to go up or down
///
/// # Arguments
/// * `start` - Starting value
/// * `steps` - Number of steps
/// * `min` - Minimum value
/// * `max` - Maximum value
/// * `up_bias` - Probability of moving up (0.0 = always down, 1.0 = always up, 0.5 = unbiased)
///
/// # Example
/// ```
/// # use tunes::composition::generative::biased_random_walk_sequence;
/// let ascending = biased_random_walk_sequence(0, 16, 0, 12, 0.7);  // Tends upward
/// let descending = biased_random_walk_sequence(11, 16, 0, 12, 0.3);  // Tends downward
/// ```
pub fn biased_random_walk_sequence(start: u32, steps: usize, min: u32, max: u32, up_bias: f32) -> Vec<u32> {
    if steps == 0 || min >= max {
        return Vec::new();
    }

    let mut sequence = Vec::with_capacity(steps);
    let mut current = start.clamp(min, max - 1);
    let mut rng = rand::rng();

    for _ in 0..steps {
        sequence.push(current);

        // Use bias to determine direction
        let go_up = rng.random::<f32>() < up_bias;
        let step = if go_up {
            rng.random_range(1..=2) // Move up
        } else {
            -(rng.random_range(1..=2) as i32) // Move down
        };

        current = ((current as i32 + step).clamp(min as i32, (max - 1) as i32)) as u32;
    }

    sequence
}

impl<'a> TrackBuilder<'a> {
    /// Generate notes using a random walk within a scale
    ///
    /// Starts at a given frequency and takes random steps through the scale,
    /// creating an organic, wandering melody.
    ///
    /// # Arguments
    /// * `start_freq` - Starting frequency (should be in the scale)
    /// * `steps` - Number of notes to generate
    /// * `note_duration` - Duration of each note
    /// * `scale` - Array of frequencies to constrain the walk to
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::scales::C4_MAJOR_SCALE;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("wanderer")
    ///     .random_walk(C4_MAJOR_SCALE[0], 16, 0.25, &C4_MAJOR_SCALE);
    /// ```
    pub fn random_walk(
        mut self,
        start_freq: f32,
        steps: usize,
        note_duration: f32,
        scale: &[f32],
    ) -> Self {
        if scale.is_empty() || steps == 0 {
            return self;
        }

        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        // Find starting position in scale
        let mut current_idx = scale
            .iter()
            .position(|&f| (f - start_freq).abs() < 0.1)
            .unwrap_or(0);

        let mut rng = rand::rng();

        for _ in 0..steps {
            let cursor = self.cursor;
            let freq = scale[current_idx];

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

            // Random walk: move up or down by 1-2 scale degrees
            let step = if rng.random::<bool>() {
                rng.random_range(1..=2) // Move up
            } else {
                -(rng.random_range(1..=2)) // Move down
            };

            current_idx = ((current_idx as i32 + step)
                .max(0)
                .min(scale.len() as i32 - 1)) as usize;
        }

        self.update_section_duration();
        self
    }

    /// Invert the pattern between pattern_start and current cursor
    ///
    /// Musical inversion mirrors pitches around a center point (axis).
    /// If a note was 2 semitones above the axis, it becomes 2 semitones below.
    ///
    /// # Arguments
    /// * `axis_freq` - The frequency to mirror around (typically the tonic)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, F4], 0.5)
    ///     .invert(C4);  // Mirror around C4
    /// ```
    pub fn invert(mut self, axis_freq: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect note events in the pattern range and invert their frequencies
        let inverted_events: Vec<_> = self
            .get_track_mut()
            .events
            .iter()
            .filter_map(|event| match event {
                AudioEvent::Note(note) if note.start_time >= pattern_start && note.start_time < cursor => {
                    // Invert each frequency in the note/chord
                    let mut inverted_freqs = [0.0f32; 8];
                    for i in 0..note.num_freqs {
                        let freq = note.frequencies[i];
                        // Calculate distance from axis in semitones
                        let semitones_from_axis = 12.0 * (freq / axis_freq).log2();
                        // Mirror it
                        let inverted_semitones = -semitones_from_axis;
                        // Convert back to frequency
                        inverted_freqs[i] = axis_freq * 2.0_f32.powf(inverted_semitones / 12.0);
                    }

                    Some(AudioEvent::Note(crate::track::NoteEvent {
                        frequencies: inverted_freqs,
                        num_freqs: note.num_freqs,
                        start_time: note.start_time,
                        duration: note.duration,
                        waveform: note.waveform,
                        envelope: note.envelope,
                        filter_envelope: note.filter_envelope,
                        fm_params: note.fm_params,
                        pitch_bend_semitones: note.pitch_bend_semitones,
                    }))
                }
                _ => None,
            })
            .collect();

        // Remove original pattern events
        let pattern_start = self.pattern_start;
        let cursor = self.cursor;
        self.get_track_mut().events.retain(|event| {
            let event_time = match event {
                AudioEvent::Note(note) => note.start_time,
                AudioEvent::Drum(drum) => drum.start_time,
            };
            event_time < pattern_start || event_time >= cursor
        });

        // Add inverted events
        self.get_track_mut().events.extend(inverted_events);

        self
    }

    /// Invert and transpose to keep the result in a reasonable range
    ///
    /// This is a more musical version of invert that ensures inverted notes
    /// stay near the original range by octave-shifting as needed.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, F4], 0.5)
    ///     .invert_constrained(C4, C3, C5);  // Keep between C3 and C5
    /// ```
    pub fn invert_constrained(mut self, axis_freq: f32, min_freq: f32, max_freq: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect and invert events, constraining to range
        let inverted_events: Vec<_> = self
            .get_track_mut()
            .events
            .iter()
            .filter_map(|event| match event {
                AudioEvent::Note(note) if note.start_time >= pattern_start && note.start_time < cursor => {
                    let mut inverted_freqs = [0.0f32; 8];
                    for i in 0..note.num_freqs {
                        let freq = note.frequencies[i];
                        let semitones_from_axis = 12.0 * (freq / axis_freq).log2();
                        let inverted_semitones = -semitones_from_axis;
                        let mut inverted_freq = axis_freq * 2.0_f32.powf(inverted_semitones / 12.0);

                        // Octave-shift to keep in range
                        while inverted_freq < min_freq {
                            inverted_freq *= 2.0;
                        }
                        while inverted_freq > max_freq {
                            inverted_freq /= 2.0;
                        }

                        inverted_freqs[i] = inverted_freq;
                    }

                    Some(AudioEvent::Note(crate::track::NoteEvent {
                        frequencies: inverted_freqs,
                        num_freqs: note.num_freqs,
                        start_time: note.start_time,
                        duration: note.duration,
                        waveform: note.waveform,
                        envelope: note.envelope,
                        filter_envelope: note.filter_envelope,
                        fm_params: note.fm_params,
                        pitch_bend_semitones: note.pitch_bend_semitones,
                    }))
                }
                _ => None,
            })
            .collect();

        // Remove original and add inverted
        let pattern_start = self.pattern_start;
        let cursor = self.cursor;
        self.get_track_mut().events.retain(|event| {
            let event_time = match event {
                AudioEvent::Note(note) => note.start_time,
                AudioEvent::Drum(drum) => drum.start_time,
            };
            event_time < pattern_start || event_time >= cursor
        });

        self.get_track_mut().events.extend(inverted_events);

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::Composition;
    use crate::notes::*;
    use crate::rhythm::Tempo;
    use crate::scales::C4_MAJOR_SCALE;

    // Sequence generation tests

    #[test]
    fn test_random_walk_sequence_generates_correct_length() {
        let seq = random_walk_sequence(3, 16, 0, 7);
        assert_eq!(seq.len(), 16);
    }

    #[test]
    fn test_random_walk_sequence_stays_in_bounds() {
        let seq = random_walk_sequence(5, 100, 0, 12);
        for &val in &seq {
            assert!(val >= 0 && val < 12);
        }
    }

    #[test]
    fn test_random_walk_sequence_empty() {
        let seq = random_walk_sequence(0, 0, 0, 10);
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_biased_random_walk_tends_upward() {
        let seq = biased_random_walk_sequence(0, 50, 0, 20, 0.8);
        // Starting at 0 with 80% upward bias, should generally increase
        let avg = seq.iter().sum::<u32>() as f32 / seq.len() as f32;
        assert!(avg > 5.0, "Average {} should be > 5.0 with upward bias", avg);
    }

    #[test]
    fn test_biased_random_walk_tends_downward() {
        let seq = biased_random_walk_sequence(19, 50, 0, 20, 0.2);
        // Starting at 19 with 20% upward bias (80% down), should generally decrease
        let avg = seq.iter().sum::<u32>() as f32 / seq.len() as f32;
        assert!(avg < 15.0, "Average {} should be < 15.0 with downward bias", avg);
    }

    #[test]
    fn test_random_walk_sequence_with_sequence_from() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let walk = random_walk_sequence(3, 16, 0, 7);
        comp.track("walk").sequence_from(&walk, &C4_MAJOR_SCALE, 0.25);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks[0].events.len(), 16);
    }

    // TrackBuilder method tests

    #[test]
    fn test_random_walk_generates_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("walk").random_walk(C4, 16, 0.25, &C4_MAJOR_SCALE);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks[0].events.len(), 16);
    }

    #[test]
    fn test_random_walk_stays_in_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("walk").random_walk(C4, 32, 0.25, &C4_MAJOR_SCALE);

        let mixer = comp.into_mixer();

        // Check that all generated notes are in the scale
        for event in &mixer.tracks[0].events {
            if let AudioEvent::Note(note) = event {
                let freq = note.frequencies[0];
                let in_scale = C4_MAJOR_SCALE.iter().any(|&scale_note| {
                    (freq - scale_note).abs() < 0.1
                });
                assert!(in_scale, "Generated note {} not in scale", freq);
            }
        }
    }

    #[test]
    fn test_random_walk_empty_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("walk").random_walk(C4, 16, 0.25, &[]);

        let mixer = comp.into_mixer();
        // Should create no track with empty scale
        assert_eq!(mixer.tracks.len(), 0);
    }

    #[test]
    fn test_invert_mirrors_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .invert(C4);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];

        assert_eq!(track.events.len(), 3);

        // C4 inverted around C4 should stay C4
        if let AudioEvent::Note(note) = &track.events[0] {
            assert!((note.frequencies[0] - C4).abs() < 1.0);
        }

        // E4 is 4 semitones above C4, so inverted should be 4 semitones below
        // E4 = C4 * 2^(4/12), inverted = C4 * 2^(-4/12)
        if let AudioEvent::Note(note) = &track.events[1] {
            let expected = C4 * 2.0_f32.powf(-4.0 / 12.0); // G#3
            assert!((note.frequencies[0] - expected).abs() < 1.0);
        }
    }

    #[test]
    fn test_invert_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .invert(C4);  // Invert with no notes

        let mixer = comp.into_mixer();
        // Should create no track
        assert_eq!(mixer.tracks.len(), 0);
    }

    #[test]
    fn test_invert_constrained_keeps_in_range() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.5)
            .invert_constrained(C4, C3, C5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];

        // All notes should be between C3 and C5
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert!(note.frequencies[0] >= C3 - 1.0);
                assert!(note.frequencies[0] <= C5 + 1.0);
            }
        }
    }
}
