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
                    .add_drum(drum_type, self.start_time + relative_time + offset, None);
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
}
