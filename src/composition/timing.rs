use super::TrackBuilder;

impl<'a> TrackBuilder<'a> {
    /// Set swing/groove timing (0.5 = straight, 0.67 = triplet swing, 0.75 = heavy swing)
    pub fn swing(mut self, swing: f32) -> Self {
        self.swing = swing.clamp(0.5, 0.9);
        self
    }
    pub fn at(mut self, time: f32) -> Self {
        self.cursor = time;
        self
    }

    pub fn wait(mut self, duration: f32) -> Self {
        self.cursor += duration;
        self
    }

    pub fn seek(mut self, offset: f32) -> Self {
        self.cursor += offset;
        self
    }

}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::rhythm::Tempo;

    #[test]
    fn test_swing_sets_value() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").swing(0.67);

        assert_eq!(builder.swing, 0.67);
    }

    #[test]
    fn test_swing_clamps_low_values() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").swing(0.2);

        // Should clamp to 0.5
        assert_eq!(builder.swing, 0.5);
    }

    #[test]
    fn test_swing_clamps_high_values() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").swing(0.95);

        // Should clamp to 0.9
        assert_eq!(builder.swing, 0.9);
    }

    #[test]
    fn test_swing_allows_boundary_values() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder_min = comp.track("test1").swing(0.5);
        assert_eq!(builder_min.swing, 0.5);

        let builder_max = comp.track("test2").swing(0.9);
        assert_eq!(builder_max.swing, 0.9);
    }

    #[test]
    fn test_swing_common_values() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Straight timing
        let straight = comp.track("straight").swing(0.5);
        assert_eq!(straight.swing, 0.5);

        // Triplet swing
        let triplet = comp.track("triplet").swing(0.67);
        assert!((triplet.swing - 0.67).abs() < 0.01);

        // Heavy swing
        let heavy = comp.track("heavy").swing(0.75);
        assert_eq!(heavy.swing, 0.75);
    }

    #[test]
    fn test_at_sets_absolute_cursor_position() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").at(5.0);

        assert_eq!(builder.cursor, 5.0);
    }

    #[test]
    fn test_at_can_move_backward() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .wait(10.0)
            .at(3.0);

        // Should be at 3.0, not 13.0
        assert_eq!(builder.cursor, 3.0);
    }

    #[test]
    fn test_at_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .wait(5.0)
            .at(0.0);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_wait_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").wait(2.5);

        assert_eq!(builder.cursor, 2.5);
    }

    #[test]
    fn test_wait_is_additive() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .wait(1.0)
            .wait(2.0)
            .wait(0.5);

        assert_eq!(builder.cursor, 3.5);
    }

    #[test]
    fn test_wait_with_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .wait(5.0)
            .wait(0.0);

        assert_eq!(builder.cursor, 5.0);
    }

    #[test]
    fn test_seek_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").seek(3.0);

        assert_eq!(builder.cursor, 3.0);
    }

    #[test]
    fn test_seek_is_additive() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .seek(1.5)
            .seek(2.0);

        assert_eq!(builder.cursor, 3.5);
    }

    #[test]
    fn test_seek_backward_with_negative() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .seek(5.0)
            .seek(-2.0);

        // Should be at 3.0
        assert_eq!(builder.cursor, 3.0);
    }

    #[test]
    fn test_timing_methods_chain_together() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .at(0.0)
            .wait(2.0)
            .seek(1.0)
            .swing(0.67);

        assert_eq!(builder.cursor, 3.0);
        assert!((builder.swing - 0.67).abs() < 0.01);
    }

    #[test]
    fn test_complex_timing_sequence() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .at(10.0)       // Jump to 10.0
            .wait(5.0)      // Now at 15.0
            .seek(-3.0)     // Now at 12.0
            .at(0.0)        // Back to 0.0
            .wait(1.0);     // Now at 1.0

        assert_eq!(builder.cursor, 1.0);
    }

    #[test]
    fn test_wait_and_seek_are_equivalent() {
        let mut comp1 = Composition::new(Tempo::new(120.0));
        let mut comp2 = Composition::new(Tempo::new(120.0));

        let with_wait = comp1.track("wait").wait(2.5);
        let with_seek = comp2.track("seek").seek(2.5);

        assert_eq!(with_wait.cursor, with_seek.cursor);
    }

    #[test]
    fn test_timing_with_note_placement() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .at(0.0)
            .note(&[440.0], 0.5)    // Note at 0.0, cursor advances to 0.5
            .wait(1.0)               // Cursor advances to 1.5
            .note(&[550.0], 0.5);    // Note at 1.5

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 2);

        if let crate::track::AudioEvent::Note(note1) = track.events[0] {
            assert_eq!(note1.start_time, 0.0);
        }
        if let crate::track::AudioEvent::Note(note2) = track.events[1] {
            assert_eq!(note2.start_time, 1.5); // 0.5 (after first note) + 1.0 (wait)
        }
    }
}
