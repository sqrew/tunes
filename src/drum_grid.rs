use crate::drums::DrumType;
use crate::track::Track;

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

    /// Add a drum hit at specific step positions
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::drums::DrumType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125)
    ///     .hit(DrumType::Kick, &[0, 4, 8, 12]); // Kick on beats 1, 2, 3, 4
    /// ```
    pub fn hit(self, drum_type: DrumType, steps: &[usize]) -> Self {
        for &step in steps {
            if step < self.steps {
                let time = self.start_time + (step as f32 * self.step_duration);
                self.track.add_drum(drum_type, time);
            }
        }
        self
    }

    /// Add kick drum hits at specific steps
    pub fn kick(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Kick, steps)
    }

    /// Add snare drum hits at specific steps
    pub fn snare(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Snare, steps)
    }

    /// Add closed hi-hat hits at specific steps
    pub fn hihat(self, steps: &[usize]) -> Self {
        self.hit(DrumType::HiHatClosed, steps)
    }

    /// Add open hi-hat hits at specific steps
    pub fn hihat_open(self, steps: &[usize]) -> Self {
        self.hit(DrumType::HiHatOpen, steps)
    }

    /// Add clap hits at specific steps
    pub fn clap(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Clap, steps)
    }

    /// Add tom hits at specific steps
    pub fn tom(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Tom, steps)
    }

    /// Add rimshot hits at specific steps
    pub fn rimshot(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Rimshot, steps)
    }

    /// Add cowbell hits at specific steps
    pub fn cowbell(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Cowbell, steps)
    }

    /// Add crash cymbal hits at specific steps
    pub fn crash(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Crash, steps)
    }

    /// Add ride cymbal hits at specific steps
    pub fn ride(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Ride, steps)
    }

    /// Add high tom hits at specific steps
    pub fn tom_high(self, steps: &[usize]) -> Self {
        self.hit(DrumType::TomHigh, steps)
    }

    /// Add low tom hits at specific steps
    pub fn tom_low(self, steps: &[usize]) -> Self {
        self.hit(DrumType::TomLow, steps)
    }

    /// Add china cymbal hits at specific steps
    pub fn china(self, steps: &[usize]) -> Self {
        self.hit(DrumType::China, steps)
    }

    /// Add splash cymbal hits at specific steps
    pub fn splash(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Splash, steps)
    }

    /// Add tambourine hits at specific steps
    pub fn tambourine(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Tambourine, steps)
    }

    /// Add shaker hits at specific steps
    pub fn shaker(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Shaker, steps)
    }

    /// Add 808 kick hits at specific steps
    pub fn kick_808(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Kick808, steps)
    }

    /// Add sub kick hits at specific steps
    pub fn sub_kick(self, steps: &[usize]) -> Self {
        self.hit(DrumType::SubKick, steps)
    }

    /// Add bass drop hits at specific steps
    pub fn bass_drop(self, steps: &[usize]) -> Self {
        self.hit(DrumType::BassDrop, steps)
    }

    /// Add boom hits at specific steps
    pub fn boom(self, steps: &[usize]) -> Self {
        self.hit(DrumType::Boom, steps)
    }

    /// Get the total duration of the grid
    pub fn duration(&self) -> f32 {
        self.steps as f32 * self.step_duration
    }

    /// Repeat the drum grid pattern N times
    ///
    /// This will duplicate all drum events that were added to the grid,
    /// placing copies sequentially after the original pattern.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::rhythm::Tempo;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125)
    ///     .kick(&[0, 4, 8, 12])
    ///     .snare(&[4, 12])
    ///     .repeat(3);  // Plays the pattern 4 times total (original + 3 repeats)
    /// ```
    pub fn repeat(self, times: usize) -> Self {
        if times == 0 {
            return self;
        }

        let grid_duration = self.duration();
        let grid_end_time = self.start_time + grid_duration;

        // Collect all drum events in this grid's time range
        let pattern_events: Vec<_> = self
            .track
            .events
            .iter()
            .filter_map(|event| match event {
                crate::track::AudioEvent::Drum(drum) => {
                    if drum.start_time >= self.start_time && drum.start_time < grid_end_time {
                        Some((drum.drum_type, drum.start_time - self.start_time))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        // Repeat the pattern
        for i in 0..times {
            let offset = grid_duration * (i + 1) as f32;
            for &(drum_type, relative_time) in &pattern_events {
                self.track
                    .add_drum(drum_type, self.start_time + relative_time + offset);
            }
        }

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
    fn test_drum_grid_hit_basic() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125).hit(DrumType::Kick, &[0, 4, 8, 12]);

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
    fn test_drum_grid_hit_with_offset() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 2.0, 8, 0.25).hit(DrumType::Snare, &[2, 6]);

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
            DrumGrid::new(&mut track, 0.0, 16, 0.125).hit(DrumType::Kick, &[0, 8, 16, 20, 100]); // 16, 20, 100 are out of bounds

        // Should only add hits for valid steps (0, 8)
        assert_eq!(track.events.len(), 2);
    }

    #[test]
    fn test_drum_grid_kick_convenience_method() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125).kick(&[0, 4, 8, 12]);

        assert_eq!(track.events.len(), 4);

        for event in &track.events {
            if let AudioEvent::Drum(drum) = event {
                assert!(matches!(drum.drum_type, DrumType::Kick));
            }
        }
    }

    #[test]
    fn test_drum_grid_snare_convenience_method() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125).snare(&[4, 12]);

        assert_eq!(track.events.len(), 2);

        for event in &track.events {
            if let AudioEvent::Drum(drum) = event {
                assert!(matches!(drum.drum_type, DrumType::Snare));
            }
        }
    }

    #[test]
    fn test_drum_grid_hihat_convenience_method() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125).hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

        assert_eq!(track.events.len(), 8);

        for event in &track.events {
            if let AudioEvent::Drum(drum) = event {
                assert!(matches!(drum.drum_type, DrumType::HiHatClosed));
            }
        }
    }

    #[test]
    fn test_drum_grid_chaining() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125)
            .kick(&[0, 4, 8, 12])
            .snare(&[4, 12])
            .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

        // Should have 4 kicks + 2 snares + 8 hihats = 14 events
        assert_eq!(track.events.len(), 14);
    }

    #[test]
    fn test_drum_grid_empty_pattern() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125).kick(&[]);

        assert_eq!(track.events.len(), 0);
    }

    #[test]
    fn test_drum_grid_repeat() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 4, 0.5) // 2 second pattern
            .kick(&[0, 2])
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
            .kick(&[0, 2])
            .repeat(0);

        // Repeating 0 times should leave pattern unchanged
        assert_eq!(track.events.len(), 2);
    }

    #[test]
    fn test_drum_grid_repeat_with_chained_patterns() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 8, 0.25)
            .kick(&[0, 4])
            .snare(&[2, 6])
            .repeat(1);

        // 2 kicks + 2 snares = 4 original, repeated once = 8 total
        assert_eq!(track.events.len(), 8);
    }

    #[test]
    fn test_drum_grid_all_drum_types() {
        // Test that all convenience methods work
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 20, 0.1)
            .kick(&[0])
            .kick_808(&[1])
            .sub_kick(&[2])
            .snare(&[3])
            .hihat(&[4])
            .hihat_open(&[5])
            .clap(&[6])
            .tom(&[7])
            .tom_high(&[8])
            .tom_low(&[9])
            .rimshot(&[10])
            .cowbell(&[11])
            .crash(&[12])
            .ride(&[13])
            .china(&[14])
            .splash(&[15])
            .tambourine(&[16])
            .shaker(&[17])
            .bass_drop(&[18])
            .boom(&[19]);

        assert_eq!(track.events.len(), 20, "Should have one of each drum type");
    }

    #[test]
    fn test_drum_grid_duplicate_steps() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125).kick(&[0, 4, 4, 8]); // Step 4 is duplicated

        // Should add all 4 events (including duplicate)
        assert_eq!(track.events.len(), 4);
    }

    #[test]
    fn test_drum_grid_overlapping_drums() {
        let mut track = Track::new();
        let _grid = DrumGrid::new(&mut track, 0.0, 16, 0.125)
            .kick(&[0])
            .snare(&[0]); // Same step as kick

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
        let _grid = DrumGrid::new(&mut track, 0.0, 32, 0.0625).hihat(&[0, 1, 2, 3, 4, 5, 6, 7]);

        assert_eq!(track.events.len(), 8);

        // Verify tight spacing
        if let AudioEvent::Drum(drum1) = &track.events[0] {
            if let AudioEvent::Drum(drum2) = &track.events[1] {
                assert_eq!(drum2.start_time - drum1.start_time, 0.0625);
            }
        }
    }
}
