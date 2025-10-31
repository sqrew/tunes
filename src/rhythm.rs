/// Represents musical note durations
#[derive(Debug, Clone, Copy)]
pub enum NoteDuration {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    // Dotted notes (1.5x duration)
    DottedHalf,
    DottedQuarter,
    DottedEighth,
    // Triplets (2/3 duration)
    QuarterTriplet,
    EighthTriplet,
}

impl NoteDuration {
    /// Returns the duration as a fraction of a whole note
    pub fn beats(&self) -> f32 {
        match self {
            NoteDuration::Whole => 4.0,
            NoteDuration::Half => 2.0,
            NoteDuration::Quarter => 1.0,
            NoteDuration::Eighth => 0.5,
            NoteDuration::Sixteenth => 0.25,
            NoteDuration::ThirtySecond => 0.125,
            NoteDuration::DottedHalf => 3.0,
            NoteDuration::DottedQuarter => 1.5,
            NoteDuration::DottedEighth => 0.75,
            NoteDuration::QuarterTriplet => 2.0 / 3.0,
            NoteDuration::EighthTriplet => 1.0 / 3.0,
        }
    }
}

/// Tempo manager for converting musical time to real time
#[derive(Debug, Clone, Copy)]
pub struct Tempo {
    pub bpm: f32, // Beats per minute
}

impl Tempo {
    /// Create a new tempo with the given BPM
    ///
    /// BPM is clamped to the range [20.0, 500.0] to prevent division by zero
    /// and ensure reasonable tempo values.
    pub fn new(bpm: f32) -> Self {
        // Clamp to reasonable range: 20 BPM (very slow) to 500 BPM (extremely fast)
        let bpm = bpm.clamp(20.0, 500.0);
        debug_assert!(bpm > 0.0, "BPM must be positive after clamping");
        Self { bpm }
    }

    /// Convert a note duration to seconds based on tempo
    pub fn duration_to_seconds(&self, duration: NoteDuration) -> f32 {
        let beats = duration.beats();
        let seconds_per_beat = 60.0 / self.bpm;
        beats * seconds_per_beat
    }

    /// Convenience method: get quarter note duration in seconds
    pub fn quarter_note(&self) -> f32 {
        60.0 / self.bpm
    }

    /// Convenience method: get eighth note duration in seconds
    pub fn eighth_note(&self) -> f32 {
        30.0 / self.bpm
    }

    /// Convenience method: get sixteenth note duration in seconds
    pub fn sixteenth_note(&self) -> f32 {
        15.0 / self.bpm
    }

    /// Convenience method: get whole note duration in seconds
    pub fn whole_note(&self) -> f32 {
        240.0 / self.bpm
    }
}

impl Default for Tempo {
    fn default() -> Self {
        Self::new(120.0) // Default to 120 BPM
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tempo_120bpm() {
        let tempo = Tempo::new(120.0);
        assert_eq!(tempo.quarter_note(), 0.5); // At 120 BPM, quarter note = 0.5 seconds
        assert_eq!(tempo.eighth_note(), 0.25);
        assert_eq!(tempo.sixteenth_note(), 0.125);
    }

    #[test]
    fn test_note_durations() {
        let tempo = Tempo::new(60.0); // 60 BPM for easy math
        assert_eq!(tempo.duration_to_seconds(NoteDuration::Quarter), 1.0);
        assert_eq!(tempo.duration_to_seconds(NoteDuration::Half), 2.0);
        assert_eq!(tempo.duration_to_seconds(NoteDuration::Eighth), 0.5);
    }

    #[test]
    fn test_tempo_clamping() {
        // Test too slow (should clamp to 20)
        let tempo = Tempo::new(0.0);
        assert_eq!(tempo.bpm, 20.0);

        let tempo = Tempo::new(-100.0);
        assert_eq!(tempo.bpm, 20.0);

        // Test too fast (should clamp to 500)
        let tempo = Tempo::new(1000.0);
        assert_eq!(tempo.bpm, 500.0);

        // Test valid range (should not change)
        let tempo = Tempo::new(120.0);
        assert_eq!(tempo.bpm, 120.0);
    }

    #[test]
    fn test_no_division_by_zero() {
        // Ensure even with extreme inputs, we never divide by zero
        let tempo = Tempo::new(0.0);
        let duration = tempo.quarter_note();
        assert!(duration.is_finite());
        assert!(duration > 0.0);
    }
}
