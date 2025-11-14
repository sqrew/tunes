//! Random walk sequence generation and method

use rand::Rng;
use crate::composition::TrackBuilder;

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
    /// Generate a melodic random walk through a scale
    ///
    /// Creates a series of notes that randomly walk up and down through a scale.
    /// Each step moves 1-2 scale degrees in a random direction.
    ///
    /// # Arguments
    /// * `start_freq` - Starting frequency (should be in the scale)
    /// * `steps` - Number of notes to generate
    /// * `note_duration` - Duration of each note in beats
    /// * `scale` - Array of frequencies to walk through
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
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
}
