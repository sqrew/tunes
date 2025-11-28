use crate::instruments::drums::DrumType;
use crate::track::Track;

/// A trait for types that can be converted into drum step positions.
///
/// This allows `DrumGrid` methods to accept either:
/// - Index arrays: `&[0, 4, 8, 12]`
/// - String patterns: `"x--- x--- x--- x---"`
///
/// # String Pattern Syntax
/// - Hit characters: `x`, `X`, `1`, `*`
/// - Rest characters: `-`, `_`, `.`, `~`, `0`, space (and any other character)
pub trait DrumPattern {
    /// Convert to a list of step indices where hits occur
    fn into_steps(&self) -> Vec<usize>;
}

impl DrumPattern for [usize] {
    fn into_steps(&self) -> Vec<usize> {
        self.to_vec()
    }
}

// Implement for fixed-size arrays using const generics
impl<const N: usize> DrumPattern for [usize; N] {
    fn into_steps(&self) -> Vec<usize> {
        self.to_vec()
    }
}

// Implement for Vec<usize>
impl DrumPattern for Vec<usize> {
    fn into_steps(&self) -> Vec<usize> {
        self.clone()
    }
}

impl DrumPattern for str {
    fn into_steps(&self) -> Vec<usize> {
        self.chars()
            .enumerate()
            .filter_map(|(i, c)| {
                if matches!(c, 'x' | 'X' | '1' | '*') {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }
}

/// A step sequencer-style drum grid for easy drum pattern programming
pub struct DrumGrid<'a> {
    track: &'a mut Track,
    start_time: f32,
    steps: usize,
    step_duration: f32,
}

impl<'a> DrumGrid<'a> {
    /// Create a new drum grid
    ///
    /// # Arguments
    /// * `track` - The track to add drum events to
    /// * `start_time` - When the grid starts (in seconds)
    /// * `steps` - Number of steps in the grid (e.g., 16 for a bar of 16th notes)
    /// * `step_duration` - Duration of each step (e.g., 0.125 for 16th notes at 120bpm)
    pub fn new(track: &'a mut Track, start_time: f32, steps: usize, step_duration: f32) -> Self {
        Self {
            track,
            start_time,
            steps,
            step_duration,
        }
    }

    /// Add a drum sound at specific step positions
    ///
    /// Accepts either an array of step indices or a string pattern:
    /// - Array: `&[0, 4, 8, 12]` - explicit step positions
    /// - String: `"x---x---x---x---"` - pattern notation where `x`/`X`/`1`/`*` are hits
    ///
    /// All 90+ drum sounds from `DrumType` are available.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .sound(DrumType::Kick, "x---x---x---x---")
    ///         .sound(DrumType::Snare, "----x-------x---")
    ///         .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-"));
    /// ```
    pub fn sound<P: DrumPattern + ?Sized>(self, drum_type: DrumType, pattern: &P) -> Self {
        for step in pattern.into_steps() {
            if step < self.steps {
                let time = self.start_time + (step as f32 * self.step_duration);
                self.track.add_drum(drum_type, time, None);
            }
        }
        self
    }

    /// Get the total duration of the grid
    pub fn duration(&self) -> f32 {
        self.steps as f32 * self.step_duration
    }

    /// Apply an accent pattern to all drums in the grid
    ///
    /// Accented steps get high velocity (1.0), unaccented steps get lower velocity (0.5).
    /// Use `accent_with_levels` for custom velocity levels.
    ///
    /// # Pattern Syntax
    /// - Accent characters: `x`, `X`, `1`, `*` (velocity 1.0)
    /// - Unaccented: `-`, `_`, `.`, `~`, `0`, space (velocity 0.5)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-")
    ///         .accent("x---x---x---x---")); // Accent every 4th hi-hat
    /// ```
    pub fn accent<P: DrumPattern + ?Sized>(self, pattern: &P) -> Self {
        self.accent_with_levels(pattern, 1.0, 0.5)
    }

    /// Apply an accent pattern with custom high/low velocity levels
    ///
    /// # Arguments
    /// * `pattern` - Pattern where hits are accented, rests are unaccented
    /// * `high_velocity` - Velocity for accented steps (0.0-1.0)
    /// * `low_velocity` - Velocity for unaccented steps (0.0-1.0)
    pub fn accent_with_levels<P: DrumPattern + ?Sized>(
        self,
        pattern: &P,
        high_velocity: f32,
        low_velocity: f32,
    ) -> Self {
        let accent_steps: std::collections::HashSet<usize> =
            pattern.into_steps().into_iter().collect();
        let grid_end_time = self.start_time + self.duration();

        for event in &mut self.track.events {
            if let crate::track::AudioEvent::Drum(drum) = event {
                if drum.start_time >= self.start_time && drum.start_time < grid_end_time {
                    // Calculate which step this drum lands on
                    let relative_time = drum.start_time - self.start_time;
                    let step = (relative_time / self.step_duration).round() as usize;

                    if accent_steps.contains(&step) {
                        drum.velocity = high_velocity.clamp(0.0, 1.0);
                    } else {
                        drum.velocity = low_velocity.clamp(0.0, 1.0);
                    }
                }
            }
        }
        self
    }

    /// Apply explicit per-step velocity values to all drums
    ///
    /// Each velocity value corresponds to a step. Drums landing on that step
    /// get the specified velocity. Steps beyond the velocity slice use 1.0.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(8, 0.125, |g| g
    ///         .sound(DrumType::HiHatClosed, "xxxxxxxx")
    ///         .velocity(&[1.0, 0.5, 0.7, 0.5, 1.0, 0.5, 0.7, 0.5])); // Swung velocity
    /// ```
    pub fn velocity(self, velocities: &[f32]) -> Self {
        let grid_end_time = self.start_time + self.duration();

        for event in &mut self.track.events {
            if let crate::track::AudioEvent::Drum(drum) = event {
                if drum.start_time >= self.start_time && drum.start_time < grid_end_time {
                    let relative_time = drum.start_time - self.start_time;
                    let step = (relative_time / self.step_duration).round() as usize;

                    if step < velocities.len() {
                        drum.velocity = velocities[step].clamp(0.0, 1.0);
                    }
                }
            }
        }
        self
    }

    /// Add probabilistic drum hits - each hit has a chance to be played
    ///
    /// # Arguments
    /// * `drum_type` - The drum sound to add
    /// * `pattern` - Step pattern (same as `sound`)
    /// * `probability` - Chance of each hit occurring (0.0 = never, 1.0 = always)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .sound(DrumType::Kick, "x---x---x---x---")
    ///         .maybe(DrumType::HiHatOpen, "x-x-x-x-x-x-x-x-", 0.3)); // 30% chance per step
    /// ```
    pub fn maybe<P: DrumPattern + ?Sized>(
        self,
        drum_type: DrumType,
        pattern: &P,
        probability: f32,
    ) -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        let prob = probability.clamp(0.0, 1.0);

        for step in pattern.into_steps() {
            if step < self.steps && rng.random::<f32>() < prob {
                let time = self.start_time + (step as f32 * self.step_duration);
                self.track.add_drum(drum_type, time, None);
            }
        }
        self
    }

    /// Add ghost notes - quieter hits that add groove
    ///
    /// Ghost notes are typically played at lower velocity to add subtle texture.
    ///
    /// # Arguments
    /// * `drum_type` - The drum sound for ghost notes
    /// * `pattern` - Step pattern for ghost note positions
    /// * `velocity` - Ghost note velocity (typically 0.2-0.4)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .sound(DrumType::Snare, "----x-------x---")
    ///         .ghost(DrumType::Snare, "-x----x--x----x-", 0.3)); // Ghost notes between hits
    /// ```
    pub fn ghost<P: DrumPattern + ?Sized>(
        self,
        drum_type: DrumType,
        pattern: &P,
        velocity: f32,
    ) -> Self {
        let vel = velocity.clamp(0.0, 1.0);
        for step in pattern.into_steps() {
            if step < self.steps {
                let time = self.start_time + (step as f32 * self.step_duration);
                self.track.add_drum_with_velocity(drum_type, time, vel, None);
            }
        }
        self
    }

    /// Add a flam - two quick hits, the first quieter (grace note)
    ///
    /// A flam is a rudiment where a grace note precedes the main hit.
    /// The grace note is typically 20-40ms before the main hit.
    ///
    /// # Arguments
    /// * `drum_type` - The drum sound
    /// * `pattern` - Step pattern for flam positions
    /// * `grace_offset` - Time before main hit for grace note (in seconds, e.g., 0.03)
    /// * `grace_velocity` - Velocity of the grace note (typically 0.3-0.5)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .flam(DrumType::Snare, "----x-------x---", 0.03, 0.4));
    /// ```
    pub fn flam<P: DrumPattern + ?Sized>(
        self,
        drum_type: DrumType,
        pattern: &P,
        grace_offset: f32,
        grace_velocity: f32,
    ) -> Self {
        let grace_vel = grace_velocity.clamp(0.0, 1.0);
        for step in pattern.into_steps() {
            if step < self.steps {
                let main_time = self.start_time + (step as f32 * self.step_duration);
                let grace_time = (main_time - grace_offset).max(0.0);

                // Add grace note (quieter, slightly before)
                self.track.add_drum_with_velocity(drum_type, grace_time, grace_vel, None);
                // Add main hit
                self.track.add_drum(drum_type, main_time, None);
            }
        }
        self
    }

    /// Add a drag - main hit followed by a quieter grace note
    ///
    /// A drag is a rudiment where a grace note follows the main hit.
    /// This is the opposite of a flam.
    ///
    /// # Arguments
    /// * `drum_type` - The drum sound
    /// * `pattern` - Step pattern for drag positions
    /// * `drag_offset` - Time after main hit for grace note (in seconds, e.g., 0.03)
    /// * `drag_velocity` - Velocity of the trailing grace note (typically 0.3-0.5)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .drag(DrumType::Snare, "----x-------x---", 0.03, 0.4));
    /// ```
    pub fn drag<P: DrumPattern + ?Sized>(
        self,
        drum_type: DrumType,
        pattern: &P,
        drag_offset: f32,
        drag_velocity: f32,
    ) -> Self {
        let drag_vel = drag_velocity.clamp(0.0, 1.0);
        for step in pattern.into_steps() {
            if step < self.steps {
                let main_time = self.start_time + (step as f32 * self.step_duration);
                let drag_time = main_time + drag_offset;

                // Add main hit
                self.track.add_drum(drum_type, main_time, None);
                // Add drag note (quieter, slightly after)
                self.track
                    .add_drum_with_velocity(drum_type, drag_time, drag_vel, None);
            }
        }
        self
    }

    /// Add a ruff - two grace notes before the main hit
    ///
    /// A ruff (or drag) is a rudiment with two grace notes preceding the main hit.
    /// This extends the flam concept with an additional grace note.
    ///
    /// # Arguments
    /// * `drum_type` - The drum sound
    /// * `pattern` - Step pattern for ruff positions
    /// * `first_offset` - Time before main hit for first grace note (e.g., 0.05)
    /// * `second_offset` - Time before main hit for second grace note (e.g., 0.025)
    /// * `grace_velocity` - Velocity of both grace notes (typically 0.3-0.5)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .ruff(DrumType::Snare, "----x-------x---", 0.05, 0.025, 0.35));
    /// ```
    pub fn ruff<P: DrumPattern + ?Sized>(
        self,
        drum_type: DrumType,
        pattern: &P,
        first_offset: f32,
        second_offset: f32,
        grace_velocity: f32,
    ) -> Self {
        let grace_vel = grace_velocity.clamp(0.0, 1.0);
        for step in pattern.into_steps() {
            if step < self.steps {
                let main_time = self.start_time + (step as f32 * self.step_duration);
                let first_grace_time = (main_time - first_offset).max(0.0);
                let second_grace_time = (main_time - second_offset).max(0.0);

                // Add first grace note (earliest)
                self.track
                    .add_drum_with_velocity(drum_type, first_grace_time, grace_vel, None);
                // Add second grace note
                self.track
                    .add_drum_with_velocity(drum_type, second_grace_time, grace_vel, None);
                // Add main hit
                self.track.add_drum(drum_type, main_time, None);
            }
        }
        self
    }

    /// Add a diddle - quick double stroke
    ///
    /// A diddle is two hits played in quick succession at equal velocity.
    /// Simpler than a roll, just two rapid hits.
    ///
    /// # Arguments
    /// * `drum_type` - The drum sound
    /// * `pattern` - Step pattern for diddle positions
    /// * `spacing` - Time between the two hits (in seconds, e.g., 0.03)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .diddle(DrumType::Snare, "----x-------x---", 0.03));
    /// ```
    pub fn diddle<P: DrumPattern + ?Sized>(
        self,
        drum_type: DrumType,
        pattern: &P,
        spacing: f32,
    ) -> Self {
        for step in pattern.into_steps() {
            if step < self.steps {
                let first_time = self.start_time + (step as f32 * self.step_duration);
                let second_time = first_time + spacing;

                // Both hits at full velocity
                self.track.add_drum(drum_type, first_time, None);
                self.track.add_drum(drum_type, second_time, None);
            }
        }
        self
    }

    /// Add a buzz roll - rapid hits with decaying velocity
    ///
    /// A buzz roll simulates the sound of a stick bouncing on the drum head,
    /// with each successive hit quieter than the last.
    ///
    /// # Arguments
    /// * `drum_type` - The drum sound
    /// * `pattern` - Step pattern for buzz positions
    /// * `hits` - Number of hits in the buzz
    /// * `decay` - Velocity multiplier per hit (e.g., 0.7 means each hit is 70% of previous)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .buzz(DrumType::Snare, "---------------x", 6, 0.7));
    /// ```
    pub fn buzz<P: DrumPattern + ?Sized>(
        self,
        drum_type: DrumType,
        pattern: &P,
        hits: usize,
        decay: f32,
    ) -> Self {
        if hits == 0 {
            return self;
        }

        let decay = decay.clamp(0.0, 1.0);
        let hit_spacing = self.step_duration / hits as f32;

        for step in pattern.into_steps() {
            if step < self.steps {
                let step_start = self.start_time + (step as f32 * self.step_duration);
                let mut velocity = 1.0f32;

                for i in 0..hits {
                    let time = step_start + (i as f32 * hit_spacing);
                    self.track
                        .add_drum_with_velocity(drum_type, time, velocity, None);
                    velocity *= decay;
                }
            }
        }
        self
    }

    /// Add a double flam - two grace notes after the main hit
    ///
    /// A double flam is like a ruff but in the opposite direction - two grace
    /// notes follow the main hit. This is the inverse of ruff, similar to how
    /// drag is the inverse of flam.
    ///
    /// # Arguments
    /// * `drum_type` - The drum sound
    /// * `pattern` - Step pattern for double flam positions
    /// * `first_offset` - Time after main hit for first grace note (e.g., 0.025)
    /// * `second_offset` - Time after main hit for second grace note (e.g., 0.05)
    /// * `grace_velocity` - Velocity of both grace notes (typically 0.3-0.5)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .double_flam(DrumType::Snare, "----x-------x---", 0.025, 0.05, 0.35));
    /// ```
    pub fn double_flam<P: DrumPattern + ?Sized>(
        self,
        drum_type: DrumType,
        pattern: &P,
        first_offset: f32,
        second_offset: f32,
        grace_velocity: f32,
    ) -> Self {
        let grace_vel = grace_velocity.clamp(0.0, 1.0);
        for step in pattern.into_steps() {
            if step < self.steps {
                let main_time = self.start_time + (step as f32 * self.step_duration);
                let first_grace_time = main_time + first_offset;
                let second_grace_time = main_time + second_offset;

                // Add main hit
                self.track.add_drum(drum_type, main_time, None);
                // Add first grace note (closer to main hit)
                self.track
                    .add_drum_with_velocity(drum_type, first_grace_time, grace_vel, None);
                // Add second grace note (further from main hit)
                self.track
                    .add_drum_with_velocity(drum_type, second_grace_time, grace_vel, None);
            }
        }
        self
    }

    /// Add a drum roll - rapid repeated hits
    ///
    /// Creates multiple hits spread evenly across the step duration.
    ///
    /// # Arguments
    /// * `drum_type` - The drum sound
    /// * `pattern` - Step pattern for roll positions
    /// * `subdivisions` - Number of hits per step (e.g., 4 for a quick roll)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .roll(DrumType::Snare, "---------------x", 8)); // Roll on last step
    /// ```
    pub fn roll<P: DrumPattern + ?Sized>(
        self,
        drum_type: DrumType,
        pattern: &P,
        subdivisions: usize,
    ) -> Self {
        if subdivisions == 0 {
            return self;
        }

        let sub_duration = self.step_duration / subdivisions as f32;

        for step in pattern.into_steps() {
            if step < self.steps {
                let step_start = self.start_time + (step as f32 * self.step_duration);
                for i in 0..subdivisions {
                    let time = step_start + (i as f32 * sub_duration);
                    self.track.add_drum(drum_type, time, None);
                }
            }
        }
        self
    }

    /// Repeat the drum grid pattern N times
    ///
    /// This will duplicate all drum events that were added to the grid,
    /// placing copies sequentially after the original pattern.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::instruments::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125, |g| g
    ///         .sound(DrumType::Kick, &[0, 4, 8, 12])
    ///         .sound(DrumType::Snare, &[4, 12])
    ///         .repeat(3));  // Plays the pattern 4 times total (original + 3 repeats)
    /// ```
    pub fn repeat(self, times: usize) -> Self {
        if times == 0 {
            return self;
        }

        let grid_duration = self.duration();
        let grid_end_time = self.start_time + grid_duration;

        // Collect all drum events in this grid's time range, preserving all properties
        let pattern_events: Vec<_> = self
            .track
            .events
            .iter()
            .filter_map(|event| match event {
                crate::track::AudioEvent::Drum(drum) => {
                    if drum.start_time >= self.start_time && drum.start_time < grid_end_time {
                        // Capture all drum properties, not just type and time
                        Some((
                            drum.drum_type,
                            drum.start_time - self.start_time, // relative_time
                            drum.velocity,
                            drum.pitch_offset,
                            drum.spatial_position,
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        // Repeat the pattern, preserving velocity, pitch_offset, and spatial_position
        for i in 0..times {
            let offset = grid_duration * (i + 1) as f32;
            for &(drum_type, relative_time, velocity, pitch_offset, spatial_position) in &pattern_events {
                self.track.events.push(crate::track::AudioEvent::Drum(
                    crate::track::DrumEvent {
                        drum_type,
                        start_time: self.start_time + relative_time + offset,
                        velocity,
                        pitch_offset,
                        spatial_position,
                    },
                ));
            }
        }

        // Invalidate cache so events get sorted before playback
        self.track.invalidate_time_cache();

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::track::AudioEvent;

    #[test]
    fn test_drum_grid_creation() {
        let mut track = Track::new();
        let grid = DrumGrid::new(&mut track, 0.0, 16, 0.125);

        assert_eq!(grid.start_time, 0.0);
        assert_eq!(grid.steps, 16);
        assert_eq!(grid.step_duration, 0.125);
    }

    #[test]
    fn test_drum_grid_duration() {
        let mut track = Track::new();
        let grid = DrumGrid::new(&mut track, 0.0, 16, 0.125);

        assert_eq!(grid.duration(), 2.0); // 16 steps * 0.125 = 2.0 seconds
    }

    #[test]
    fn test_drum_grid_sound_basic() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125).sound(DrumType::Kick, &[0, 4, 8, 12]);

        assert_eq!(track.events.len(), 4);

        // Verify first hit
        if let AudioEvent::Drum(drum) = &track.events[0] {
            assert!(matches!(drum.drum_type, DrumType::Kick));
            assert_eq!(drum.start_time, 0.0);
        }

        // Verify last hit
        if let AudioEvent::Drum(drum) = &track.events[3] {
            assert!(matches!(drum.drum_type, DrumType::Kick));
            assert_eq!(drum.start_time, 1.5); // step 12 * 0.125
        }
    }

    #[test]
    fn test_drum_grid_sound_with_offset() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 2.0, 8, 0.25).sound(DrumType::Snare, &[2, 6]);

        assert_eq!(track.events.len(), 2);

        if let AudioEvent::Drum(drum) = &track.events[0] {
            assert_eq!(drum.start_time, 2.5); // 2.0 start + (2 * 0.25)
        }

        if let AudioEvent::Drum(drum) = &track.events[1] {
            assert_eq!(drum.start_time, 3.5); // 2.0 start + (6 * 0.25)
        }
    }

    #[test]
    fn test_drum_grid_out_of_bounds_steps() {
        let mut track = Track::new();
        let _grid =
            DrumGrid::new(&mut track, 0.0, 16, 0.125).sound(DrumType::Kick, &[0, 8, 16, 20, 100]); // 16, 20, 100 are out of bounds

        // Should only add hits for valid steps (0, 8)
        assert_eq!(track.events.len(), 2);
    }

    #[test]
    fn test_drum_grid_chaining() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125)
            .sound(DrumType::Kick, &[0, 4, 8, 12])
            .sound(DrumType::Snare, &[4, 12])
            .sound(DrumType::HiHatClosed, &[0, 2, 4, 6, 8, 10, 12, 14]);

        // Should have 4 kicks + 2 snares + 8 hihats = 14 events
        assert_eq!(track.events.len(), 14);
    }

    #[test]
    fn test_drum_grid_empty_pattern() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125).sound(DrumType::Kick, &[]);

        assert_eq!(track.events.len(), 0);
    }

    #[test]
    fn test_drum_grid_repeat() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5) // 2 second pattern
            .sound(DrumType::Kick, &[0, 2])
            .repeat(2); // Repeat 2 more times

        // Original 2 kicks + 2 repeats * 2 kicks = 6 total
        assert_eq!(track.events.len(), 6);

        // Verify timing of repeats
        if let AudioEvent::Drum(drum) = &track.events[0] {
            assert_eq!(drum.start_time, 0.0); // Original
        }
        if let AudioEvent::Drum(drum) = &track.events[2] {
            assert_eq!(drum.start_time, 2.0); // First repeat
        }
        if let AudioEvent::Drum(drum) = &track.events[4] {
            assert_eq!(drum.start_time, 4.0); // Second repeat
        }
    }

    #[test]
    fn test_drum_grid_repeat_zero_times() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5)
            .sound(DrumType::Kick, &[0, 2])
            .repeat(0);

        // Repeating 0 times should leave pattern unchanged
        assert_eq!(track.events.len(), 2);
    }

    #[test]
    fn test_drum_grid_repeat_with_chained_patterns() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 8, 0.25)
            .sound(DrumType::Kick, &[0, 4])
            .sound(DrumType::Snare, &[2, 6])
            .repeat(1);

        // 2 kicks + 2 snares = 4 original, repeated once = 8 total
        assert_eq!(track.events.len(), 8);
    }

    #[test]
    fn test_drum_grid_many_drum_types() {
        // Test that sound() works with various drum types
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 10, 0.1)
            .sound(DrumType::Kick, &[0])
            .sound(DrumType::Kick808, &[1])
            .sound(DrumType::Snare, &[2])
            .sound(DrumType::HiHatClosed, &[3])
            .sound(DrumType::Clap, &[4])
            .sound(DrumType::Tom, &[5])
            .sound(DrumType::Crash, &[6])
            .sound(DrumType::Cowbell, &[7])
            .sound(DrumType::Djembe, &[8])
            .sound(DrumType::LaserZap, &[9]);

        assert_eq!(track.events.len(), 10, "Should have one of each drum type");
    }

    #[test]
    fn test_drum_grid_duplicate_steps() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125).sound(DrumType::Kick, &[0, 4, 4, 8]); // Step 4 is duplicated

        // Should add all 4 events (including duplicate)
        assert_eq!(track.events.len(), 4);
    }

    #[test]
    fn test_drum_grid_overlapping_drums() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125)
            .sound(DrumType::Kick, &[0])
            .sound(DrumType::Snare, &[0]); // Same step as kick

        // Both should be added at the same time
        assert_eq!(track.events.len(), 2);

        if let AudioEvent::Drum(drum1) = &track.events[0] {
            if let AudioEvent::Drum(drum2) = &track.events[1] {
                assert_eq!(drum1.start_time, drum2.start_time);
            }
        }
    }

    #[test]
    fn test_drum_grid_fine_step_resolution() {
        let mut track = Track::new();
        // 32nd note grid
        let _grid = DrumGrid::new(&mut track, 0.0, 32, 0.0625)
            .sound(DrumType::HiHatClosed, &[0, 1, 2, 3, 4, 5, 6, 7]);

        assert_eq!(track.events.len(), 8);

        // Verify tight spacing
        if let AudioEvent::Drum(drum1) = &track.events[0] {
            if let AudioEvent::Drum(drum2) = &track.events[1] {
                assert_eq!(drum2.start_time - drum1.start_time, 0.0625);
            }
        }
    }

    // ===== String Pattern Tests =====

    #[test]
    fn test_string_pattern_basic() {
        let mut track = Track::new();
        let _grid =
            DrumGrid::new(&mut track, 0.0, 16, 0.125).sound(DrumType::Kick, "x---x---x---x---");

        assert_eq!(track.events.len(), 4);

        // Verify timing: hits at positions 0, 4, 8, 12
        if let AudioEvent::Drum(drum) = &track.events[0] {
            assert_eq!(drum.start_time, 0.0);
        }
        if let AudioEvent::Drum(drum) = &track.events[1] {
            assert_eq!(drum.start_time, 0.5); // 4 * 0.125
        }
        if let AudioEvent::Drum(drum) = &track.events[2] {
            assert_eq!(drum.start_time, 1.0); // 8 * 0.125
        }
        if let AudioEvent::Drum(drum) = &track.events[3] {
            assert_eq!(drum.start_time, 1.5); // 12 * 0.125
        }
    }

    #[test]
    fn test_string_pattern_with_spaces() {
        let mut track = Track::new();
        let _grid =
            DrumGrid::new(&mut track, 0.0, 16, 0.125).sound(DrumType::Kick, "x--- x--- x--- x---");

        // Spaces count as steps (rests), so pattern is 19 chars
        // Hits at positions 0, 5, 10, 15
        assert_eq!(track.events.len(), 4);
    }

    #[test]
    fn test_string_pattern_different_hit_chars() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 8, 0.125).sound(DrumType::Snare, "xX1*----");

        // All four hit markers should work
        assert_eq!(track.events.len(), 4);
    }

    #[test]
    fn test_string_pattern_different_rest_chars() {
        let mut track = Track::new();
        let _grid =
            DrumGrid::new(&mut track, 0.0, 16, 0.125).sound(DrumType::HiHatClosed, "x-x_x.x~x0x x");

        // Hits at positions: 0, 2, 4, 6, 8, 10, 12 = 7 hits
        assert_eq!(track.events.len(), 7);
    }

    #[test]
    fn test_string_pattern_all_hits() {
        let mut track = Track::new();
        let _grid =
            DrumGrid::new(&mut track, 0.0, 8, 0.125).sound(DrumType::HiHatClosed, "xxxxxxxx");

        assert_eq!(track.events.len(), 8);
    }

    #[test]
    fn test_string_pattern_all_rests() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 8, 0.125).sound(DrumType::Kick, "--------");

        assert_eq!(track.events.len(), 0);
    }

    #[test]
    fn test_string_pattern_empty() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125).sound(DrumType::Kick, "");

        assert_eq!(track.events.len(), 0);
    }

    #[test]
    fn test_string_pattern_mixed_with_array() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125)
            .sound(DrumType::Kick, &[0, 4, 8, 12]) // Array syntax
            .sound(DrumType::Snare, "----x-------x---") // String syntax
            .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-"); // String syntax

        // 4 kicks + 2 snares + 8 hihats = 14
        assert_eq!(track.events.len(), 14);

        // Count each type
        let mut kicks = 0;
        let mut snares = 0;
        let mut hihats = 0;
        for event in &track.events {
            if let AudioEvent::Drum(drum) = event {
                match drum.drum_type {
                    DrumType::Kick => kicks += 1,
                    DrumType::Snare => snares += 1,
                    DrumType::HiHatClosed => hihats += 1,
                    _ => {}
                }
            }
        }
        assert_eq!(kicks, 4);
        assert_eq!(snares, 2);
        assert_eq!(hihats, 8);
    }

    #[test]
    fn test_string_pattern_out_of_bounds() {
        let mut track = Track::new();
        // Grid is only 8 steps, but pattern is 16 chars
        let _grid =
            DrumGrid::new(&mut track, 0.0, 8, 0.125).sound(DrumType::Kick, "x---x---x---x---");

        // Only first 2 hits should be added (positions 0 and 4)
        // Positions 8 and 12 are out of bounds
        assert_eq!(track.events.len(), 2);
    }

    #[test]
    fn test_string_pattern_repeat() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 8, 0.25)
            .sound(DrumType::Kick, "x---x---")
            .sound(DrumType::Snare, "--x---x-")
            .repeat(1);

        // 2 kicks + 2 snares = 4, repeated once = 8
        assert_eq!(track.events.len(), 8);
    }

    #[test]
    fn test_string_pattern_classic_rock_beat() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125)
            .sound(DrumType::Kick, "x---x---x---x---") // Four on the floor
            .sound(DrumType::Snare, "----x-------x---") // Backbeat
            .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-"); // Eighth notes

        // 4 kicks + 2 snares + 8 hihats = 14
        assert_eq!(track.events.len(), 14);
    }

    #[test]
    fn test_string_pattern_numeric_notation() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 8, 0.125).sound(DrumType::Cowbell, "10011001");

        // Hits at 0, 3, 4, 7 = 4 hits
        assert_eq!(track.events.len(), 4);
    }

    #[test]
    fn test_drum_pattern_trait_into_steps() {
        // Test the trait directly
        let array: &[usize] = &[0, 4, 8, 12];
        assert_eq!(array.into_steps(), vec![0, 4, 8, 12]);

        let pattern = "x---x---";
        assert_eq!(pattern.into_steps(), vec![0, 4]);

        let pattern2 = "xXx1*";
        assert_eq!(pattern2.into_steps(), vec![0, 1, 2, 3, 4]);

        let empty = "";
        assert_eq!(empty.into_steps(), Vec::<usize>::new());

        let all_rests = "----";
        assert_eq!(all_rests.into_steps(), Vec::<usize>::new());
    }

    // ===== Accent and Velocity Tests =====

    #[test]
    fn test_accent_basic() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 8, 0.125)
            .sound(DrumType::HiHatClosed, "xxxxxxxx")
            .accent("x---x---"); // Accent steps 0 and 4

        assert_eq!(track.events.len(), 8);

        // Check velocities
        for (i, event) in track.events.iter().enumerate() {
            if let AudioEvent::Drum(drum) = event {
                if i == 0 || i == 4 {
                    assert_eq!(drum.velocity, 1.0, "Step {} should be accented", i);
                } else {
                    assert_eq!(drum.velocity, 0.5, "Step {} should be unaccented", i);
                }
            }
        }
    }

    #[test]
    fn test_accent_with_levels() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.125)
            .sound(DrumType::HiHatClosed, "xxxx")
            .accent_with_levels("x-x-", 0.9, 0.3);

        // Check custom velocity levels
        if let AudioEvent::Drum(drum) = &track.events[0] {
            assert_eq!(drum.velocity, 0.9); // Accented
        }
        if let AudioEvent::Drum(drum) = &track.events[1] {
            assert_eq!(drum.velocity, 0.3); // Unaccented
        }
    }

    #[test]
    fn test_velocity_explicit() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.125)
            .sound(DrumType::HiHatClosed, "xxxx")
            .velocity(&[1.0, 0.5, 0.7, 0.3]);

        let velocities: Vec<f32> = track
            .events
            .iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    Some(d.velocity)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(velocities, vec![1.0, 0.5, 0.7, 0.3]);
    }

    #[test]
    fn test_ghost_notes() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 8, 0.125)
            .sound(DrumType::Snare, "----x---") // Main hit at step 4
            .ghost(DrumType::Snare, "-x-x----", 0.3); // Ghost notes at steps 1 and 3

        assert_eq!(track.events.len(), 3);

        // Count velocities
        let mut full_hits = 0;
        let mut ghost_hits = 0;
        for event in &track.events {
            if let AudioEvent::Drum(drum) = event {
                if drum.velocity == 1.0 {
                    full_hits += 1;
                } else if (drum.velocity - 0.3).abs() < 0.01 {
                    ghost_hits += 1;
                }
            }
        }
        assert_eq!(full_hits, 1, "Should have 1 main hit");
        assert_eq!(ghost_hits, 2, "Should have 2 ghost notes");
    }

    #[test]
    fn test_flam() {
        let mut track = Track::new();
        // Use step 2 so there's room for the grace note before the main hit
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5)
            .flam(DrumType::Snare, "--x-", 0.03, 0.4);

        assert_eq!(track.events.len(), 2); // Grace note + main hit

        // Sort events by time to check order
        let mut times: Vec<(f32, f32)> = track
            .events
            .iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    Some((d.start_time, d.velocity))
                } else {
                    None
                }
            })
            .collect();
        times.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Main hit should be at step 2 = 1.0s
        // Grace note should be at 1.0 - 0.03 = 0.97s
        assert!(times[0].0 < times[1].0, "Grace note should be before main hit");
        assert!((times[0].0 - 0.97).abs() < 0.01, "Grace note at 0.97s");
        assert_eq!(times[0].1, 0.4, "Grace note velocity");
        assert_eq!(times[1].0, 1.0, "Main hit at 1.0s");
        assert_eq!(times[1].1, 1.0, "Main hit velocity");
    }

    #[test]
    fn test_drag() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5)
            .drag(DrumType::Snare, "--x-", 0.03, 0.4);

        assert_eq!(track.events.len(), 2); // Main hit + drag note

        // Sort events by time to check order
        let mut times: Vec<(f32, f32)> = track
            .events
            .iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    Some((d.start_time, d.velocity))
                } else {
                    None
                }
            })
            .collect();
        times.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Main hit should be at step 2 = 1.0s
        // Drag note should be at 1.0 + 0.03 = 1.03s
        assert_eq!(times[0].0, 1.0, "Main hit at 1.0s");
        assert_eq!(times[0].1, 1.0, "Main hit velocity");
        assert!((times[1].0 - 1.03).abs() < 0.001, "Drag note at 1.03s");
        assert_eq!(times[1].1, 0.4, "Drag note velocity");
    }

    #[test]
    fn test_ruff() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5)
            .ruff(DrumType::Snare, "--x-", 0.05, 0.025, 0.35);

        assert_eq!(track.events.len(), 3); // Two grace notes + main hit

        // Sort events by time
        let mut times: Vec<(f32, f32)> = track
            .events
            .iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    Some((d.start_time, d.velocity))
                } else {
                    None
                }
            })
            .collect();
        times.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Main hit at step 2 = 1.0s
        // First grace at 1.0 - 0.05 = 0.95s
        // Second grace at 1.0 - 0.025 = 0.975s
        assert!((times[0].0 - 0.95).abs() < 0.001, "First grace at 0.95s");
        assert_eq!(times[0].1, 0.35, "First grace velocity");
        assert!((times[1].0 - 0.975).abs() < 0.001, "Second grace at 0.975s");
        assert_eq!(times[1].1, 0.35, "Second grace velocity");
        assert_eq!(times[2].0, 1.0, "Main hit at 1.0s");
        assert_eq!(times[2].1, 1.0, "Main hit velocity");
    }

    #[test]
    fn test_diddle() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5)
            .diddle(DrumType::Snare, "x---", 0.03);

        assert_eq!(track.events.len(), 2); // Two equal hits

        let times: Vec<(f32, f32)> = track
            .events
            .iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    Some((d.start_time, d.velocity))
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(times[0].0, 0.0, "First hit at 0.0s");
        assert_eq!(times[0].1, 1.0, "First hit full velocity");
        assert!((times[1].0 - 0.03).abs() < 0.001, "Second hit at 0.03s");
        assert_eq!(times[1].1, 1.0, "Second hit full velocity");
    }

    #[test]
    fn test_buzz() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5)
            .buzz(DrumType::Snare, "x---", 4, 0.5);

        assert_eq!(track.events.len(), 4); // 4 hits with decay

        let velocities: Vec<f32> = track
            .events
            .iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    Some(d.velocity)
                } else {
                    None
                }
            })
            .collect();

        // Decay: 1.0, 0.5, 0.25, 0.125
        assert_eq!(velocities[0], 1.0);
        assert_eq!(velocities[1], 0.5);
        assert_eq!(velocities[2], 0.25);
        assert_eq!(velocities[3], 0.125);
    }

    #[test]
    fn test_buzz_zero_hits() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5)
            .buzz(DrumType::Snare, "x---", 0, 0.5);

        assert_eq!(track.events.len(), 0, "Zero hits should add nothing");
    }

    #[test]
    fn test_double_flam() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5)
            .double_flam(DrumType::Snare, "--x-", 0.025, 0.05, 0.35);

        assert_eq!(track.events.len(), 3); // Main hit + two grace notes

        // Sort events by time
        let mut times: Vec<(f32, f32)> = track
            .events
            .iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    Some((d.start_time, d.velocity))
                } else {
                    None
                }
            })
            .collect();
        times.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Main hit at step 2 = 1.0s
        // First grace at 1.0 + 0.025 = 1.025s
        // Second grace at 1.0 + 0.05 = 1.05s
        assert_eq!(times[0].0, 1.0, "Main hit at 1.0s");
        assert_eq!(times[0].1, 1.0, "Main hit velocity");
        assert!((times[1].0 - 1.025).abs() < 0.001, "First grace at 1.025s");
        assert_eq!(times[1].1, 0.35, "First grace velocity");
        assert!((times[2].0 - 1.05).abs() < 0.001, "Second grace at 1.05s");
        assert_eq!(times[2].1, 0.35, "Second grace velocity");
    }

    #[test]
    fn test_roll() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5)
            .roll(DrumType::Snare, "x---", 4); // 4 subdivisions

        assert_eq!(track.events.len(), 4); // 4 hits in the roll

        // Check timing spacing
        let times: Vec<f32> = track
            .events
            .iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    Some(d.start_time)
                } else {
                    None
                }
            })
            .collect();

        // All should be within step 0 (0.0 to 0.5)
        assert_eq!(times[0], 0.0);
        assert_eq!(times[1], 0.125); // 0.5 / 4
        assert_eq!(times[2], 0.25);
        assert_eq!(times[3], 0.375);
    }

    #[test]
    fn test_roll_zero_subdivisions() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5)
            .roll(DrumType::Snare, "x---", 0);

        assert_eq!(track.events.len(), 0, "Zero subdivisions should add nothing");
    }

    #[test]
    fn test_maybe_always() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.125)
            .maybe(DrumType::Kick, "xxxx", 1.0); // 100% probability

        assert_eq!(track.events.len(), 4, "All hits should be added with probability 1.0");
    }

    #[test]
    fn test_maybe_never() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.125)
            .maybe(DrumType::Kick, "xxxx", 0.0); // 0% probability

        assert_eq!(track.events.len(), 0, "No hits should be added with probability 0.0");
    }

    #[test]
    fn test_accent_multiple_drums_same_step() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.125)
            .sound(DrumType::Kick, "x---")
            .sound(DrumType::HiHatClosed, "x-x-")
            .accent("x---"); // Only step 0 is accented

        // All drums at step 0 should be accented
        for event in &track.events {
            if let AudioEvent::Drum(drum) = event {
                let step = (drum.start_time / 0.125).round() as usize;
                if step == 0 {
                    assert_eq!(drum.velocity, 1.0, "Step 0 drums should be accented");
                } else {
                    assert_eq!(drum.velocity, 0.5, "Other steps should be unaccented");
                }
            }
        }
    }

    #[test]
    fn test_ghost_repeat_alignment() {
        let mut track = Track::new();
        let step_dur = 0.125; // 8th note at ~120bpm
        let _grid = DrumGrid::new(&mut track, 0.0, 8, step_dur)
            .sound(DrumType::Snare, "----x---")  // Step 4 = 0.5s
            .ghost(DrumType::Snare, "-x------", 0.3)  // Step 1 = 0.125s
            .repeat(1);

        // Collect all snare events sorted by time
        let mut snares: Vec<_> = track.events.iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    if d.drum_type == DrumType::Snare {
                        Some((d.start_time, d.velocity))
                    } else { None }
                } else { None }
            })
            .collect();
        snares.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        println!("Snare events:");
        for (i, (time, vel)) in snares.iter().enumerate() {
            println!("  [{}] time={:.4}, vel={:.2}", i, time, vel);
        }

        // Expected: 4 events
        // Original: ghost at 0.125 (vel 0.3), sound at 0.5 (vel 1.0)
        // Repeat:   ghost at 1.125 (vel 0.3), sound at 1.5 (vel 1.0)
        assert_eq!(snares.len(), 4, "Should have 4 snare events (2 original + 2 repeated)");

        let grid_duration = 8.0 * step_dur; // 1.0 second

        // Check original pattern
        assert!((snares[0].0 - 0.125).abs() < 0.001, "First ghost at step 1");
        assert!((snares[0].1 - 0.3).abs() < 0.001, "First ghost velocity 0.3");
        assert!((snares[1].0 - 0.5).abs() < 0.001, "First sound at step 4");
        assert!((snares[1].1 - 1.0).abs() < 0.001, "First sound velocity 1.0");

        // Check repeated pattern - should be offset by grid_duration (1.0)
        assert!((snares[2].0 - (0.125 + grid_duration)).abs() < 0.001,
            "Repeated ghost should be at {} but was at {}", 0.125 + grid_duration, snares[2].0);
        assert!((snares[2].1 - 0.3).abs() < 0.001, "Repeated ghost velocity 0.3");
        assert!((snares[3].0 - (0.5 + grid_duration)).abs() < 0.001,
            "Repeated sound should be at {} but was at {}", 0.5 + grid_duration, snares[3].0);
        assert!((snares[3].1 - 1.0).abs() < 0.001, "Repeated sound velocity 1.0");
    }

    #[test]
    fn test_voice_stealing_with_ghost_repeat() {
        // Simulate what the mixer does: at each time point, find the latest drum
        // and verify that ghost notes are not incorrectly silenced
        use std::collections::HashMap;

        let mut track = Track::new();
        let step_dur = 0.125;
        let _grid = DrumGrid::new(&mut track, 0.0, 8, step_dur)
            .sound(DrumType::Snare, "----x---")  // Step 4 = 0.5s
            .ghost(DrumType::Snare, "-x------", 0.3)  // Step 1 = 0.125s
            .repeat(1);

        // Snare duration is 0.1 seconds
        let snare_duration = 0.1;

        // Collect expected active drums at various time points
        let test_times = vec![
            (0.13, "first ghost"),      // During first ghost (0.125 to 0.225)
            (0.51, "first sound"),      // During first sound (0.5 to 0.6)
            (1.13, "repeated ghost"),   // During repeated ghost (1.125 to 1.225)
            (1.51, "repeated sound"),   // During repeated sound (1.5 to 1.6)
        ];

        for (time, label) in test_times {
            // Find which drums are active at this time
            let mut latest_drum_starts: HashMap<DrumType, f32> = HashMap::new();
            for event in &track.events {
                if let AudioEvent::Drum(d) = event {
                    if time >= d.start_time && time < d.start_time + snare_duration {
                        let entry = latest_drum_starts.entry(d.drum_type).or_insert(f32::MIN);
                        if d.start_time > *entry {
                            *entry = d.start_time;
                        }
                    }
                }
            }

            // Count how many snares would actually render (voice stealing check)
            let mut rendered_count = 0;
            let mut rendered_vel = 0.0;
            for event in &track.events {
                if let AudioEvent::Drum(d) = event {
                    if d.drum_type == DrumType::Snare
                        && time >= d.start_time
                        && time < d.start_time + snare_duration
                        && latest_drum_starts.get(&d.drum_type) == Some(&d.start_time)
                    {
                        rendered_count += 1;
                        rendered_vel = d.velocity;
                    }
                }
            }

            println!("At time {:.2} ({}): {} drum(s) rendered, vel={:.2}",
                time, label, rendered_count, rendered_vel);

            // Exactly one drum should render at each test time
            assert_eq!(rendered_count, 1,
                "Expected 1 drum at time {} ({}) but got {}", time, label, rendered_count);
        }
    }

    #[test]
    fn test_flam_repeat_offset_preserved() {
        // Test that flam grace note offset is preserved across repeats
        let mut track = Track::new();
        let step_dur = 0.125;
        let grace_offset = 0.03;
        let _grid = DrumGrid::new(&mut track, 0.0, 8, step_dur)
            .flam(DrumType::Snare, "----x---", grace_offset, 0.4)
            .repeat(1);

        let mut snares: Vec<_> = track.events.iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    if d.drum_type == DrumType::Snare {
                        Some((d.start_time, d.velocity))
                    } else { None }
                } else { None }
            })
            .collect();
        snares.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        println!("\nFlam repeat test:");
        for (i, (time, vel)) in snares.iter().enumerate() {
            println!("  [{}] time={:.6}, vel={:.2}", i, time, vel);
        }

        // Should have 4 events: 2 original (grace + main) + 2 repeated
        assert_eq!(snares.len(), 4, "Should have 4 snare events");

        let main_time = 4.0 * step_dur; // 0.5
        let grace_time = main_time - grace_offset; // 0.47
        let grid_dur = 8.0 * step_dur; // 1.0

        // Original: grace at 0.47, main at 0.5
        assert!((snares[0].0 - grace_time).abs() < 0.001,
            "Original grace should be at {} but was at {}", grace_time, snares[0].0);
        assert!((snares[0].1 - 0.4).abs() < 0.001, "Grace velocity should be 0.4");
        assert!((snares[1].0 - main_time).abs() < 0.001,
            "Original main should be at {} but was at {}", main_time, snares[1].0);
        assert!((snares[1].1 - 1.0).abs() < 0.001, "Main velocity should be 1.0");

        // Repeated: grace at 1.47, main at 1.5
        assert!((snares[2].0 - (grace_time + grid_dur)).abs() < 0.001,
            "Repeated grace should be at {} but was at {}", grace_time + grid_dur, snares[2].0);
        assert!((snares[2].1 - 0.4).abs() < 0.001, "Repeated grace velocity should be 0.4");
        assert!((snares[3].0 - (main_time + grid_dur)).abs() < 0.001,
            "Repeated main should be at {} but was at {}", main_time + grid_dur, snares[3].0);
        assert!((snares[3].1 - 1.0).abs() < 0.001, "Repeated main velocity should be 1.0");

        // Verify the offset is preserved (main - grace should be grace_offset in both cases)
        let original_offset = snares[1].0 - snares[0].0;
        let repeated_offset = snares[3].0 - snares[2].0;
        assert!((original_offset - grace_offset).abs() < 0.001,
            "Original grace offset should be {} but was {}", grace_offset, original_offset);
        assert!((repeated_offset - grace_offset).abs() < 0.001,
            "Repeated grace offset should be {} but was {}", grace_offset, repeated_offset);
    }

    #[test]
    fn test_ghost_repeat_same_step() {
        // Test ghost at same step as sound - voice stealing should let both play
        // since they have same start_time (no one is "later")
        let mut track = Track::new();
        let step_dur = 0.125;
        let _grid = DrumGrid::new(&mut track, 0.0, 8, step_dur)
            .sound(DrumType::Snare, "----x---")  // Step 4
            .ghost(DrumType::Snare, "----x---", 0.3)  // Same step 4 (ghost velocity)
            .repeat(1);

        let mut snares: Vec<_> = track.events.iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    if d.drum_type == DrumType::Snare {
                        Some((d.start_time, d.velocity))
                    } else { None }
                } else { None }
            })
            .collect();
        snares.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        println!("\nSame-step ghost test:");
        for (i, (time, vel)) in snares.iter().enumerate() {
            println!("  [{}] time={:.6}, vel={:.2}", i, time, vel);
        }

        // Both hits at same time - two different events exist at 0.5 and at 1.5
        assert_eq!(snares.len(), 4, "Should have 4 events");
        // Check both exist at same times (they'll be sorted by time, then by insertion order)
        assert!((snares[0].0 - 0.5).abs() < 0.001);
        assert!((snares[1].0 - 0.5).abs() < 0.001);
        assert!((snares[2].0 - 1.5).abs() < 0.001);
        assert!((snares[3].0 - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_ghost_repeat_multiple() {
        // Test with 3 repeats to check if the issue compounds
        let mut track = Track::new();
        let step_dur = 0.125;
        let _grid = DrumGrid::new(&mut track, 0.0, 8, step_dur)
            .sound(DrumType::Snare, "----x---")
            .ghost(DrumType::Snare, "-x------", 0.3)
            .repeat(3);

        let mut snares: Vec<_> = track.events.iter()
            .filter_map(|e| {
                if let AudioEvent::Drum(d) = e {
                    if d.drum_type == DrumType::Snare {
                        Some((d.start_time, d.velocity))
                    } else { None }
                } else { None }
            })
            .collect();
        snares.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        println!("\nMultiple repeat test - all snare events:");
        let grid_dur = 8.0 * step_dur;
        for (i, (time, vel)) in snares.iter().enumerate() {
            // Calculate which repetition this belongs to
            let rep = (time / grid_dur).floor() as usize;
            let relative = time - (rep as f32 * grid_dur);
            println!("  [{}] time={:.6}, vel={:.2}, rep={}, relative={:.6}",
                i, time, vel, rep, relative);
        }

        // Should have 8 events: 2 original + 2*3 repeated
        assert_eq!(snares.len(), 8, "Should have 8 snare events");

        // Check that relative positions are consistent across all repetitions
        for rep in 0..4 {
            let base = rep as f32 * grid_dur;
            let ghost_time = base + 0.125;
            let sound_time = base + 0.5;

            let ghost = snares.iter().find(|(t, _)| (t - ghost_time).abs() < 0.001);
            let sound = snares.iter().find(|(t, _)| (t - sound_time).abs() < 0.001);

            assert!(ghost.is_some(), "Rep {} ghost at {} not found", rep, ghost_time);
            assert!(sound.is_some(), "Rep {} sound at {} not found", rep, sound_time);

            if let Some((_, vel)) = ghost {
                assert!((vel - 0.3).abs() < 0.001, "Rep {} ghost velocity wrong", rep);
            }
            if let Some((_, vel)) = sound {
                assert!((vel - 1.0).abs() < 0.001, "Rep {} sound velocity wrong", rep);
            }
        }
    }
}
