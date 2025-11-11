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
/// # use tunes::consts::scales::C4_MAJOR_SCALE;
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
pub fn biased_random_walk_sequence(
    start: u32,
    steps: usize,
    min: u32,
    max: u32,
    up_bias: f32,
) -> Vec<u32> {
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
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::scales::C4_MAJOR_SCALE;
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

    /// Add human feel to pattern by randomizing timing and velocity
    ///
    /// Makes programmed music feel more natural by adding slight random variations
    /// to note timing and velocity within the pattern.
    ///
    /// # Arguments
    /// * `timing_variance` - Max timing offset in seconds (e.g., 0.02 = ±20ms)
    /// * `velocity_variance` - Max velocity change (e.g., 0.1 = ±10%)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("piano")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .humanize(0.02, 0.1);  // Subtle humanization
    ///
    /// comp.track("drums")
    ///     .pattern_start()
    ///     .notes(&[C4, C4, C4, C4], 0.25)
    ///     .humanize(0.005, 0.15);  // Tight timing, varied velocity
    /// ```
    pub fn humanize(mut self, timing_variance: f32, velocity_variance: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Apply humanization to notes in the pattern
        for event in &mut self.get_track_mut().events {
            let event_time = match event {
                AudioEvent::Note(note) => note.start_time,
                AudioEvent::Drum(drum) => drum.start_time,
                _ => continue,
            };

            if event_time >= pattern_start && event_time < cursor {
                match event {
                    AudioEvent::Note(note) => {
                        // Randomize timing
                        use rand::Rng;
                        let mut rng = rand::rng();
                        let timing_offset = rng.random_range(-timing_variance..=timing_variance);
                        note.start_time = (note.start_time + timing_offset).max(0.0);

                        // Randomize velocity
                        let velocity_offset = rng.random_range(-velocity_variance..=velocity_variance);
                        note.velocity = (note.velocity + velocity_offset).clamp(0.0, 1.0);
                    }
                    AudioEvent::Drum(drum) => {
                        // Randomize drum timing only
                        use rand::Rng;
                        let mut rng = rand::rng();
                        let timing_offset = rng.random_range(-timing_variance..=timing_variance);
                        drum.start_time = (drum.start_time + timing_offset).max(0.0);
                    }
                    _ => {}
                }
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Rotate notes in the pattern by n positions
    ///
    /// Cycles the pitch sequence while keeping timing the same.
    /// Positive values rotate forward, negative rotate backward.
    ///
    /// # Arguments
    /// * `positions` - Number of positions to rotate (positive = forward, negative = backward)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Original: C4, E4, G4, C5
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .rotate(1);  // Result: E4, G4, C5, C4
    ///
    /// comp.track("bass")
    ///     .pattern_start()
    ///     .notes(&[C3, E3, G3], 0.5)
    ///     .rotate(-1);  // Result: G3, C3, E3
    /// ```
    pub fn rotate(mut self, positions: i32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || positions == 0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect note events in the pattern
        let mut note_events: Vec<(usize, [f32; 8], usize)> = Vec::new();
        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    note_events.push((idx, note.frequencies, note.num_freqs));
                }
            }
        }

        if note_events.is_empty() {
            return self;
        }

        // Extract frequencies in order
        let freqs: Vec<([f32; 8], usize)> = note_events
            .iter()
            .map(|(_, f, n)| (*f, *n))
            .collect();

        // Rotate the frequencies
        let len = freqs.len() as i32;
        let normalized_rotation = ((positions % len) + len) % len; // Handle negative rotations

        // Apply rotated frequencies back to the notes
        for (i, (event_idx, _, _)) in note_events.iter().enumerate() {
            let rotated_idx = ((i as i32 + normalized_rotation) % len) as usize;
            if let AudioEvent::Note(note) = &mut self.get_track_mut().events[*event_idx] {
                note.frequencies = freqs[rotated_idx].0;
                note.num_freqs = freqs[rotated_idx].1;
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Shuffle pitches in random order (keeps timing)
    ///
    /// Randomly reorders the pitch sequence while maintaining the original timing.
    /// Each call produces a different random ordering.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Original: C4, E4, G4, C5
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .shuffle();  // Result: random order like G4, C4, C5, E4
    /// ```
    pub fn shuffle(mut self) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect note events in the pattern
        let mut note_events: Vec<(usize, [f32; 8], usize)> = Vec::new();
        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    note_events.push((idx, note.frequencies, note.num_freqs));
                }
            }
        }

        if note_events.is_empty() {
            return self;
        }

        // Extract and shuffle frequencies
        let mut freqs: Vec<([f32; 8], usize)> = note_events
            .iter()
            .map(|(_, f, n)| (*f, *n))
            .collect();

        // Shuffle using Fisher-Yates
        use rand::Rng;
        let mut rng = rand::rng();
        for i in (1..freqs.len()).rev() {
            let j = rng.random_range(0..=i);
            freqs.swap(i, j);
        }

        // Apply shuffled frequencies back to the notes
        for (i, (event_idx, _, _)) in note_events.iter().enumerate() {
            if let AudioEvent::Note(note) = &mut self.get_track_mut().events[*event_idx] {
                note.frequencies = freqs[i].0;
                note.num_freqs = freqs[i].1;
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Randomly remove notes from the pattern
    ///
    /// Reduces note density by probabilistically keeping or removing each note.
    /// Great for creating space or variations with less density.
    ///
    /// # Arguments
    /// * `keep_probability` - Chance to keep each note (0.0 = remove all, 1.0 = keep all)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("hihat")
    ///     .pattern_start()
    ///     .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.125)
    ///     .thin(0.5);  // Keep ~50% of notes
    ///
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5, E4, G4], 0.25)
    ///     .thin(0.7);  // Keep ~70% of notes
    /// ```
    pub fn thin(mut self, keep_probability: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let keep_probability = keep_probability.clamp(0.0, 1.0);

        // If probability is 1.0, keep everything
        if keep_probability >= 1.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Remove notes based on probability
        use rand::Rng;
        let mut rng = rand::rng();

        self.get_track_mut().events.retain(|event| {
            match event {
                AudioEvent::Note(note) => {
                    if note.start_time >= pattern_start && note.start_time < cursor {
                        // Randomly decide to keep or remove
                        rng.random_range(0.0..1.0) < keep_probability
                    } else {
                        true // Keep notes outside pattern
                    }
                }
                _ => true // Keep non-note events
            }
        });

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Retrograde: reverse the melodic contour (pitches backwards, timing forward)
    ///
    /// Classic compositional technique - plays the pitch sequence in reverse order
    /// while keeping the original timing. Different from `.reverse()` which reverses time.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Original: C4 at t=0, E4 at t=0.25, G4 at t=0.5
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.25)
    ///     .retrograde();  // Result: G4 at t=0, E4 at t=0.25, C4 at t=0.5
    /// ```
    pub fn retrograde(mut self) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect note events in the pattern
        let mut note_events: Vec<(usize, [f32; 8], usize)> = Vec::new();
        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    note_events.push((idx, note.frequencies, note.num_freqs));
                }
            }
        }

        if note_events.is_empty() {
            return self;
        }

        // Extract frequencies in order
        let freqs: Vec<([f32; 8], usize)> = note_events
            .iter()
            .map(|(_, f, n)| (*f, *n))
            .collect();

        // Reverse the frequencies
        let reversed_freqs: Vec<([f32; 8], usize)> = freqs.into_iter().rev().collect();

        // Apply reversed frequencies back to the notes (keeping original timing)
        for (i, (event_idx, _, _)) in note_events.iter().enumerate() {
            if let AudioEvent::Note(note) = &mut self.get_track_mut().events[*event_idx] {
                note.frequencies = reversed_freqs[i].0;
                note.num_freqs = reversed_freqs[i].1;
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Shift all notes in the pattern by semitones
    ///
    /// Transposes all notes between pattern_start and current cursor.
    /// Positive values shift up, negative values shift down.
    ///
    /// # Arguments
    /// * `semitones` - Number of semitones to shift (positive = up, negative = down)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.5)
    ///     .shift(5);  // Transpose up a perfect fourth
    ///
    /// comp.track("bass")
    ///     .pattern_start()
    ///     .notes(&[C3, G3, C4], 0.5)
    ///     .shift(-12);  // Transpose down an octave
    /// ```
    pub fn shift(mut self, semitones: i32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || semitones == 0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;
        let shift_ratio = 2.0_f32.powf(semitones as f32 / 12.0);

        // Collect events in the pattern range - shift notes, pass through drums and samples
        let shifted_events: Vec<_> = self
            .get_track_mut()
            .events
            .iter()
            .filter_map(|event| {
                let event_time = event.start_time();
                if event_time >= pattern_start && event_time < cursor {
                    match event {
                        AudioEvent::Note(note) => {
                            // Shift each frequency in the note/chord
                            let mut shifted_freqs = [0.0f32; 8];
                            for i in 0..note.num_freqs {
                                shifted_freqs[i] = note.frequencies[i] * shift_ratio;
                            }

                            Some(AudioEvent::Note(crate::track::NoteEvent {
                                frequencies: shifted_freqs,
                                num_freqs: note.num_freqs,
                                start_time: note.start_time,
                                duration: note.duration,
                                waveform: note.waveform,
                                envelope: note.envelope,
                                filter_envelope: note.filter_envelope,
                                fm_params: note.fm_params,
                                pitch_bend_semitones: note.pitch_bend_semitones,
                                custom_wavetable: note.custom_wavetable.clone(),
                                velocity: note.velocity,
                                spatial_position: note.spatial_position,
                            }))
                        }
                        AudioEvent::Drum(_)
                        | AudioEvent::Sample(_)
                        | AudioEvent::TempoChange(_)
                        | AudioEvent::TimeSignature(_)
                        | AudioEvent::KeySignature(_) => {
                            // Pass through drums, samples, tempo changes, and time signatures unchanged
                            Some(event.clone())
                        }
                    }
                } else {
                    None
                }
            })
            .collect();

        // Remove original pattern events
        let pattern_start = self.pattern_start;
        let cursor = self.cursor;
        self.get_track_mut().events.retain(|event| {
            let event_time = match event {
                AudioEvent::Note(note) => note.start_time,
                AudioEvent::Drum(drum) => drum.start_time,
                AudioEvent::Sample(sample) => sample.start_time,
                AudioEvent::TempoChange(tempo) => tempo.start_time,
                AudioEvent::TimeSignature(time_sig) => time_sig.start_time,
                AudioEvent::KeySignature(key_sig) => key_sig.start_time,
            };
            event_time < pattern_start || event_time >= cursor
        });

        // Add shifted events
        self.get_track_mut().events.extend(shifted_events);
        self.get_track_mut().invalidate_time_cache();

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
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
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

        // Collect events in the pattern range - invert notes, pass through drums and samples
        let inverted_events: Vec<_> = self
            .get_track_mut()
            .events
            .iter()
            .filter_map(|event| {
                let event_time = event.start_time();
                if event_time >= pattern_start && event_time < cursor {
                    match event {
                        AudioEvent::Note(note) => {
                            // Invert each frequency in the note/chord
                            let mut inverted_freqs = [0.0f32; 8];
                            for i in 0..note.num_freqs {
                                let freq = note.frequencies[i];
                                // Calculate distance from axis in semitones
                                let semitones_from_axis = 12.0 * (freq / axis_freq).log2();
                                // Mirror it
                                let inverted_semitones = -semitones_from_axis;
                                // Convert back to frequency
                                inverted_freqs[i] =
                                    axis_freq * 2.0_f32.powf(inverted_semitones / 12.0);
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
                                custom_wavetable: note.custom_wavetable.clone(),
                                velocity: note.velocity,
                                spatial_position: note.spatial_position,
                            }))
                        }
                        AudioEvent::Drum(_)
                        | AudioEvent::Sample(_)
                        | AudioEvent::TempoChange(_)
                        | AudioEvent::TimeSignature(_)
                        | AudioEvent::KeySignature(_) => {
                            // Pass through drums, samples, tempo changes, and time signatures unchanged
                            Some(event.clone())
                        }
                    }
                } else {
                    None
                }
            })
            .collect();

        // Remove original pattern events
        let pattern_start = self.pattern_start;
        let cursor = self.cursor;
        self.get_track_mut().events.retain(|event| {
            let event_time = match event {
                AudioEvent::Note(note) => note.start_time,
                AudioEvent::Drum(drum) => drum.start_time,
                AudioEvent::Sample(sample) => sample.start_time,
                AudioEvent::TempoChange(tempo) => tempo.start_time,
                AudioEvent::TimeSignature(time_sig) => time_sig.start_time,
                AudioEvent::KeySignature(key_sig) => key_sig.start_time,
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
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
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

        // Collect and invert events, constraining to range - pass through drums and samples
        let inverted_events: Vec<_> = self
            .get_track_mut()
            .events
            .iter()
            .filter_map(|event| {
                let event_time = event.start_time();
                if event_time >= pattern_start && event_time < cursor {
                    match event {
                        AudioEvent::Note(note) => {
                            let mut inverted_freqs = [0.0f32; 8];
                            for i in 0..note.num_freqs {
                                let freq = note.frequencies[i];
                                let semitones_from_axis = 12.0 * (freq / axis_freq).log2();
                                let inverted_semitones = -semitones_from_axis;
                                let mut inverted_freq =
                                    axis_freq * 2.0_f32.powf(inverted_semitones / 12.0);

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
                                custom_wavetable: note.custom_wavetable.clone(),
                                velocity: note.velocity,
                                spatial_position: note.spatial_position,
                            }))
                        }
                        AudioEvent::Drum(_)
                        | AudioEvent::Sample(_)
                        | AudioEvent::TempoChange(_)
                        | AudioEvent::TimeSignature(_)
                        | AudioEvent::KeySignature(_) => {
                            // Pass through drums, samples, tempo changes, and time signatures unchanged
                            Some(event.clone())
                        }
                    }
                } else {
                    None
                }
            })
            .collect();

        // Remove original and add inverted
        let pattern_start = self.pattern_start;
        let cursor = self.cursor;
        self.get_track_mut().events.retain(|event| {
            let event_time = match event {
                AudioEvent::Note(note) => note.start_time,
                AudioEvent::Drum(drum) => drum.start_time,
                AudioEvent::Sample(sample) => sample.start_time,
                AudioEvent::TempoChange(tempo) => tempo.start_time,
                AudioEvent::TimeSignature(time_sig) => time_sig.start_time,
                AudioEvent::KeySignature(key_sig) => key_sig.start_time,
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
    use crate::composition::rhythm::Tempo;
    use crate::consts::notes::*;
    use crate::consts::scales::C4_MAJOR_SCALE;

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
            assert!(val < 12);
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
        assert!(
            avg > 5.0,
            "Average {} should be > 5.0 with upward bias",
            avg
        );
    }

    #[test]
    fn test_biased_random_walk_tends_downward() {
        let seq = biased_random_walk_sequence(19, 50, 0, 20, 0.2);
        // Starting at 19 with 20% upward bias (80% down), should generally decrease
        let avg = seq.iter().sum::<u32>() as f32 / seq.len() as f32;
        assert!(
            avg < 15.0,
            "Average {} should be < 15.0 with downward bias",
            avg
        );
    }

    #[test]
    fn test_random_walk_sequence_with_sequence_from() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let walk = random_walk_sequence(3, 16, 0, 7);
        comp.track("walk")
            .sequence_from(&walk, &C4_MAJOR_SCALE, 0.25);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks()[0].events.len(), 16);
    }

    // TrackBuilder method tests

    #[test]
    fn test_random_walk_generates_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("walk")
            .random_walk(C4, 16, 0.25, &C4_MAJOR_SCALE);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks()[0].events.len(), 16);
    }

    #[test]
    fn test_random_walk_stays_in_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("walk")
            .random_walk(C4, 32, 0.25, &C4_MAJOR_SCALE);

        let mixer = comp.into_mixer();

        // Check that all generated notes are in the scale
        for event in &mixer.tracks()[0].events {
            if let AudioEvent::Note(note) = event {
                let freq = note.frequencies[0];
                let in_scale = C4_MAJOR_SCALE
                    .iter()
                    .any(|&scale_note| (freq - scale_note).abs() < 0.1);
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
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_shift_transposes_up() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .shift(12); // Transpose up one octave

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Check frequencies are transposed up an octave
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C5).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E5).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - G5).abs() < 0.1);
        }
    }

    #[test]
    fn test_shift_transposes_down() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .shift(-12); // Transpose down one octave

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Check frequencies are transposed down an octave
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C3).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E3).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - G3).abs() < 0.1);
        }
    }

    #[test]
    fn test_shift_by_zero_no_change() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .shift(0); // No transposition

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should still have original frequencies
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_shift_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().shift(12); // Shift with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_humanize_adds_variance() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4, C4], 0.25)
            .humanize(0.05, 0.2);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // Check that at least one note has non-exact timing or velocity
        // (with high probability due to randomness)
        let mut has_variance = false;
        for event in events {
            if let AudioEvent::Note(note) = event {
                // Check if timing is offset from exact multiples of 0.25
                let expected_times = [0.0, 0.25, 0.5, 0.75];
                let time_exact = expected_times.iter().any(|&t| (note.start_time - t).abs() < 0.001);
                if !time_exact || (note.velocity - 0.7).abs() > 0.01 {
                    has_variance = true;
                    break;
                }
            }
        }
        // With timing_variance=0.05 and velocity_variance=0.2, very likely to have variance
        assert!(has_variance, "Humanize should add some variance");
    }

    #[test]
    fn test_rotate_cycles_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .rotate(1); // Rotate forward by 1

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // After rotate(1): should be E4, G4, C5, C4
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
            assert_eq!(note.start_time, 0.0); // Timing unchanged
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
            assert_eq!(note.start_time, 0.25);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - C5).abs() < 0.1);
            assert_eq!(note.start_time, 0.5);
        }
        if let AudioEvent::Note(note) = &events[3] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.75);
        }
    }

    #[test]
    fn test_rotate_negative() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .rotate(-1); // Rotate backward by 1

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // After rotate(-1): should be G4, C4, E4
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
        }
    }

    #[test]
    fn test_rotate_by_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .rotate(0); // No rotation

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should remain unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_retrograde_reverses_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .retrograde(); // Reverse pitch sequence

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // After retrograde: should be C5, G4, E4, C4 (reversed pitches)
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C5).abs() < 0.1);
            assert_eq!(note.start_time, 0.0); // Timing unchanged
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
            assert_eq!(note.start_time, 0.25);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
            assert_eq!(note.start_time, 0.5);
        }
        if let AudioEvent::Note(note) = &events[3] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.75);
        }
    }

    #[test]
    fn test_retrograde_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().retrograde(); // Retrograde with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_shuffle_reorders_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .shuffle(); // Random reorder

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // Check that timing is preserved
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert_eq!(note.start_time, 0.25);
        }

        // Check that all original frequencies are still present (just reordered)
        let original_freqs = vec![C4, E4, G4, C5];
        let mut result_freqs = Vec::new();
        for event in events {
            if let AudioEvent::Note(note) = event {
                result_freqs.push(note.frequencies[0]);
            }
        }
        result_freqs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut sorted_original = original_freqs.clone();
        sorted_original.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for (a, b) in result_freqs.iter().zip(sorted_original.iter()) {
            assert!((a - b).abs() < 0.1);
        }
    }

    #[test]
    fn test_shuffle_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().shuffle(); // Shuffle with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_thin_removes_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.125)
            .thin(0.5); // Keep ~50%

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have removed some notes (probabilistic, but with 8 notes and 50% probability,
        // very unlikely to keep all or remove all)
        assert!(events.len() < 8, "Should remove some notes");
        assert!(events.len() > 0, "Should keep some notes");
    }

    #[test]
    fn test_thin_keep_all() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .thin(1.0); // Keep all

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4); // All notes kept
    }

    #[test]
    fn test_thin_remove_all() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .thin(0.0); // Remove all

        let mixer = comp.into_mixer();
        // Track exists but all notes removed
        assert_eq!(mixer.tracks().len(), 1);
        assert_eq!(mixer.tracks()[0].events.len(), 0);
    }

    #[test]
    fn test_thin_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().thin(0.5); // Thin with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_invert_mirrors_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .invert(C4);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];

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

        comp.track("melody").pattern_start().invert(C4); // Invert with no notes

        let mixer = comp.into_mixer();
        // Should create no track
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_invert_constrained_keeps_in_range() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.5)
            .invert_constrained(C4, C3, C5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];

        // All notes should be between C3 and C5
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert!(note.frequencies[0] >= C3 - 1.0);
                assert!(note.frequencies[0] <= C5 + 1.0);
            }
        }
    }
}
