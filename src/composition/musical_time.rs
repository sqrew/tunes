use super::TrackBuilder;

impl<'a> TrackBuilder<'a> {
    // ===== TEMPO DURATION HELPERS =====

    /// Get the duration of a quarter note at the current tempo
    ///
    /// Useful for calculating durations for ornaments, custom timing, etc.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// let qtr = comp.tempo().quarter_note();
    /// comp.instrument("piano", &Instrument::acoustic_piano())
    ///     .trill(C4, D4, 16, qtr / 4.0);  // Sixteenth note trill
    /// ```
    pub fn tempo_quarter(&self) -> f32 {
        self.tempo.quarter_note()
    }

    /// Get the duration of an eighth note at the current tempo
    pub fn tempo_eighth(&self) -> f32 {
        self.tempo.eighth_note()
    }

    /// Get the duration of a sixteenth note at the current tempo
    pub fn tempo_sixteenth(&self) -> f32 {
        self.tempo.eighth_note() / 2.0
    }

    /// Get the duration of a half note at the current tempo
    pub fn tempo_half(&self) -> f32 {
        self.tempo.quarter_note() * 2.0
    }

    /// Get the duration of a whole note at the current tempo
    pub fn tempo_whole(&self) -> f32 {
        self.tempo.whole_note()
    }

    /// Get the duration of a dotted quarter note at the current tempo
    pub fn tempo_dotted_quarter(&self) -> f32 {
        self.tempo.quarter_note() * 1.5
    }

    /// Get the duration of a dotted eighth note at the current tempo
    pub fn tempo_dotted_eighth(&self) -> f32 {
        self.tempo.eighth_note() * 1.5
    }

    /// Get the duration of a thirty-second note at the current tempo
    pub fn tempo_thirty_second(&self) -> f32 {
        self.tempo.eighth_note() / 4.0
    }

    // ===== MUSICAL TIME POSITIONING =====

    /// Set cursor to a specific bar number (1-indexed)
    ///
    /// Assumes 4/4 time signature. Bar 1 starts at 0 seconds.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("piano", &Instrument::acoustic_piano())
    ///     .at_bar(5)  // Start at bar 5
    ///     .quarters(&[C4, E4, G4, C5]);
    /// ```
    pub fn at_bar(mut self, bar: u32) -> Self {
        let beats_per_bar = 4.0;
        let beat_duration = self.tempo.quarter_note();
        self.cursor = (bar as f32 - 1.0) * beats_per_bar * beat_duration;
        self
    }

    /// Set cursor to a specific beat number (1-indexed)
    ///
    /// Beat 1 is 0 seconds, beat 2 is one quarter note later, etc.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("snare", &Instrument::acoustic_piano())
    ///     .at_beat(9)  // Beat 9 (bar 3, beat 1 in 4/4)
    ///     .note(&[D4], 0.5);
    /// ```
    pub fn at_beat(mut self, beat: u32) -> Self {
        let beat_duration = self.tempo.quarter_note();
        self.cursor = (beat as f32 - 1.0) * beat_duration;
        self
    }

    // ===== WAIT IN MUSICAL TIME =====

    /// Wait for a number of beats (quarter notes)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("melody", &Instrument::synth_lead())
    ///     .note(&[C4], 0.5)
    ///     .beats(2.0)  // Wait 2 quarter notes
    ///     .note(&[E4], 0.5);
    /// ```
    pub fn beats(mut self, beats: f32) -> Self {
        let beat_duration = self.tempo.quarter_note();
        self.cursor += beats * beat_duration;
        self
    }

    /// Wait for a number of bars (measures)
    ///
    /// Assumes 4/4 time signature (4 beats per bar).
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("pad", &Instrument::warm_pad())
    ///     .note(&[C4, E4, G4], 2.0)
    ///     .bars(2.0)  // Wait 2 bars
    ///     .note(&[F4, A4, C5], 2.0);
    /// ```
    pub fn bars(mut self, bars: f32) -> Self {
        let beats_per_bar = 4.0;
        let beat_duration = self.tempo.quarter_note();
        self.cursor += bars * beats_per_bar * beat_duration;
        self
    }

    // ===== NOTE DURATION HELPERS =====

    /// Play notes with whole note duration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("pad", &Instrument::warm_pad())
    ///     .wholes(&[C4, E4, G4]);  // Each note lasts a whole note
    /// ```
    pub fn wholes(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.whole_note();
        for &freq in notes {
            let cursor = self.cursor;
            let waveform = self.waveform;
            let envelope = self.envelope;
            let pitch_bend = self.pitch_bend;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[freq],
                    cursor,
                    duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            let swung_duration = self.apply_swing(duration);
            self.cursor += swung_duration;
        }
        self
    }

    /// Play notes with half note duration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("bass", &Instrument::sub_bass())
    ///     .halves(&[C2, G2]);  // Each note lasts a half note
    /// ```
    pub fn halves(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.quarter_note() * 2.0;
        for &freq in notes {
            let cursor = self.cursor;
            let waveform = self.waveform;
            let envelope = self.envelope;
            let pitch_bend = self.pitch_bend;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[freq],
                    cursor,
                    duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            let swung_duration = self.apply_swing(duration);
            self.cursor += swung_duration;
        }
        self
    }

    /// Play notes with quarter note duration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("melody", &Instrument::acoustic_piano())
    ///     .quarters(&[C4, D4, E4, F4]);  // Four quarter notes
    /// ```
    pub fn quarters(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.quarter_note();
        for &freq in notes {
            let cursor = self.cursor;
            let waveform = self.waveform;
            let envelope = self.envelope;
            let pitch_bend = self.pitch_bend;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[freq],
                    cursor,
                    duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            let swung_duration = self.apply_swing(duration);
            self.cursor += swung_duration;
        }
        self
    }

    /// Play notes with eighth note duration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("hihat", &Instrument::pluck())
    ///     .eighths(&[C4, C4, C4, C4, C4, C4, C4, C4]);  // Eight eighth notes
    /// ```
    pub fn eighths(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.eighth_note();
        for &freq in notes {
            let cursor = self.cursor;
            let waveform = self.waveform;
            let envelope = self.envelope;
            let pitch_bend = self.pitch_bend;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[freq],
                    cursor,
                    duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            let swung_duration = self.apply_swing(duration);
            self.cursor += swung_duration;
        }
        self
    }

    /// Play notes with sixteenth note duration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("fast_run", &Instrument::synth_lead())
    ///     .sixteenths(&[C4, D4, E4, F4, G4, A4, B4, C5]);  // Fast 16th note run
    /// ```
    pub fn sixteenths(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.eighth_note() / 2.0;
        for &freq in notes {
            let cursor = self.cursor;
            let waveform = self.waveform;
            let envelope = self.envelope;
            let pitch_bend = self.pitch_bend;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[freq],
                    cursor,
                    duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            let swung_duration = self.apply_swing(duration);
            self.cursor += swung_duration;
        }
        self
    }

    /// Play notes with dotted quarter note duration (1.5 beats)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("melody", &Instrument::acoustic_piano())
    ///     .dotted_quarters(&[C4, E4, G4]);  // Three dotted quarter notes
    /// ```
    pub fn dotted_quarters(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.quarter_note() * 1.5;
        for &freq in notes {
            let cursor = self.cursor;
            let waveform = self.waveform;
            let envelope = self.envelope;
            let pitch_bend = self.pitch_bend;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[freq],
                    cursor,
                    duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            let swung_duration = self.apply_swing(duration);
            self.cursor += swung_duration;
        }
        self
    }

    /// Play notes with dotted eighth note duration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("rhythm", &Instrument::pluck())
    ///     .dotted_eighths(&[C4, E4, G4, E4]);
    /// ```
    pub fn dotted_eighths(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.eighth_note() * 1.5;
        for &freq in notes {
            let cursor = self.cursor;
            let waveform = self.waveform;
            let envelope = self.envelope;
            let pitch_bend = self.pitch_bend;
            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[freq],
                    cursor,
                    duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );
            let swung_duration = self.apply_swing(duration);
            self.cursor += swung_duration;
        }
        self
    }

    // ===== SINGLE NOTE WITH MUSICAL DURATION =====

    /// Play a single note or chord with quarter note duration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("chord", &Instrument::warm_pad())
    ///     .quarter(&[C4, E4, G4]);  // One quarter note chord
    /// ```
    pub fn quarter(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.quarter_note();
        let cursor = self.cursor;
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;
        self.get_track_mut()
            .add_note_with_waveform_envelope_and_bend(
                notes, cursor, duration, waveform, envelope, pitch_bend,
            );
        let swung_duration = self.apply_swing(duration);
        self.cursor += swung_duration;
        self
    }

    /// Play a single note or chord with half note duration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("bass", &Instrument::sub_bass())
    ///     .half(&[C2]);  // One half note
    /// ```
    pub fn half(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.quarter_note() * 2.0;
        let cursor = self.cursor;
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;
        self.get_track_mut()
            .add_note_with_waveform_envelope_and_bend(
                notes, cursor, duration, waveform, envelope, pitch_bend,
            );
        let swung_duration = self.apply_swing(duration);
        self.cursor += swung_duration;
        self
    }

    /// Play a single note or chord with whole note duration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("pad", &Instrument::warm_pad())
    ///     .whole(&[C4, E4, G4, C5]);  // One whole note chord
    /// ```
    pub fn whole(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.whole_note();
        let cursor = self.cursor;
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;
        self.get_track_mut()
            .add_note_with_waveform_envelope_and_bend(
                notes, cursor, duration, waveform, envelope, pitch_bend,
            );
        let swung_duration = self.apply_swing(duration);
        self.cursor += swung_duration;
        self
    }

    /// Play a single note or chord with eighth note duration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("stab", &Instrument::stab())
    ///     .eighth(&[C4, E4, G4]);  // One eighth note stab
    /// ```
    pub fn eighth(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.eighth_note();
        let cursor = self.cursor;
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;
        self.get_track_mut()
            .add_note_with_waveform_envelope_and_bend(
                notes, cursor, duration, waveform, envelope, pitch_bend,
            );
        let swung_duration = self.apply_swing(duration);
        self.cursor += swung_duration;
        self
    }

    /// Play a single note or chord with sixteenth note duration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("perc", &Instrument::pluck())
    ///     .sixteenth(&[C4]);  // One sixteenth note
    /// ```
    pub fn sixteenth(mut self, notes: &[f32]) -> Self {
        let duration = self.tempo.eighth_note() / 2.0;
        let cursor = self.cursor;
        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;
        self.get_track_mut()
            .add_note_with_waveform_envelope_and_bend(
                notes, cursor, duration, waveform, envelope, pitch_bend,
            );
        let swung_duration = self.apply_swing(duration);
        self.cursor += swung_duration;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::consts::notes::*;
    use crate::composition::rhythm::Tempo;
    use crate::track::AudioEvent;

    // ===== TEMPO DURATION HELPERS =====

    #[test]
    fn test_tempo_quarter_at_120_bpm() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test");

        // At 120 BPM, quarter note = 60/120 = 0.5 seconds
        assert_eq!(builder.tempo_quarter(), 0.5);
    }

    #[test]
    fn test_tempo_eighth_at_120_bpm() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test");

        // Eighth note = half of quarter
        assert_eq!(builder.tempo_eighth(), 0.25);
    }

    #[test]
    fn test_tempo_sixteenth_at_120_bpm() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test");

        // Sixteenth note = quarter of quarter
        assert_eq!(builder.tempo_sixteenth(), 0.125);
    }

    #[test]
    fn test_tempo_half_at_120_bpm() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test");

        // Half note = 2 quarters
        assert_eq!(builder.tempo_half(), 1.0);
    }

    #[test]
    fn test_tempo_whole_at_120_bpm() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test");

        // Whole note = 4 quarters
        assert_eq!(builder.tempo_whole(), 2.0);
    }

    #[test]
    fn test_tempo_dotted_quarter_at_120_bpm() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test");

        // Dotted quarter = 1.5 * quarter
        assert_eq!(builder.tempo_dotted_quarter(), 0.75);
    }

    #[test]
    fn test_tempo_dotted_eighth_at_120_bpm() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test");

        // Dotted eighth = 1.5 * eighth
        assert_eq!(builder.tempo_dotted_eighth(), 0.375);
    }

    #[test]
    fn test_tempo_thirty_second_at_120_bpm() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test");

        // Thirty-second note = eighth / 4
        assert_eq!(builder.tempo_thirty_second(), 0.0625);
    }

    #[test]
    fn test_tempo_durations_at_different_bpm() {
        let mut comp = Composition::new(Tempo::new(60.0));
        let builder = comp.track("test");

        // At 60 BPM, quarter = 1 second
        assert_eq!(builder.tempo_quarter(), 1.0);
        assert_eq!(builder.tempo_eighth(), 0.5);
        assert_eq!(builder.tempo_half(), 2.0);
    }

    // ===== TIME POSITIONING =====

    #[test]
    fn test_at_bar_sets_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").at_bar(1);

        // Bar 1 starts at 0
        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_at_bar_fifth_bar() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").at_bar(5);

        // Bar 5 = 4 bars * 4 beats * 0.5 seconds = 8.0
        assert_eq!(builder.cursor, 8.0);
    }

    #[test]
    fn test_at_beat_sets_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").at_beat(1);

        // Beat 1 starts at 0
        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_at_beat_ninth_beat() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").at_beat(9);

        // Beat 9 = 8 beats * 0.5 = 4.0 seconds
        assert_eq!(builder.cursor, 4.0);
    }

    // ===== WAIT METHODS =====

    #[test]
    fn test_beats_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").beats(2.0);

        // 2 beats at 120 BPM = 2 * 0.5 = 1.0
        assert_eq!(builder.cursor, 1.0);
    }

    #[test]
    fn test_beats_fractional() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").beats(1.5);

        // 1.5 beats = 0.75 seconds at 120 BPM
        assert_eq!(builder.cursor, 0.75);
    }

    #[test]
    fn test_bars_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").bars(2.0);

        // 2 bars = 2 * 4 beats * 0.5 = 4.0
        assert_eq!(builder.cursor, 4.0);
    }

    #[test]
    fn test_bars_fractional() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").bars(0.5);

        // Half bar = 2 beats = 1.0 second at 120 BPM
        assert_eq!(builder.cursor, 1.0);
    }

    // ===== MULTIPLE NOTES METHODS =====

    #[test]
    fn test_wholes_creates_notes_with_correct_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").wholes(&[C4, D4, E4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);

        // Each whole note = 2.0 seconds at 120 BPM
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.duration, 2.0);
            }
        }
    }

    #[test]
    fn test_halves_creates_notes_with_correct_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").halves(&[C4, E4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 2);

        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.duration, 1.0);
            }
        }
    }

    #[test]
    fn test_quarters_creates_notes_with_correct_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").quarters(&[C4, D4, E4, F4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.duration, 0.5);
            }
        }
    }

    #[test]
    fn test_eighths_creates_notes_with_correct_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").eighths(&[C4, D4, E4, F4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.duration, 0.25);
            }
        }
    }

    #[test]
    fn test_sixteenths_creates_notes_with_correct_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .sixteenths(&[C4, D4, E4, F4, G4, A4, B4, C5]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 8);

        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.duration, 0.125);
            }
        }
    }

    #[test]
    fn test_dotted_quarters_creates_notes_with_correct_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").dotted_quarters(&[C4, E4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 2);

        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.duration, 0.75);
            }
        }
    }

    #[test]
    fn test_dotted_eighths_creates_notes_with_correct_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").dotted_eighths(&[C4, E4, G4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 3);

        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.duration, 0.375);
            }
        }
    }

    #[test]
    fn test_quarters_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").quarters(&[C4, D4, E4]);

        // 3 quarter notes = 3 * 0.5 = 1.5
        assert_eq!(builder.cursor, 1.5);
    }

    // ===== SINGLE CHORD METHODS =====

    #[test]
    fn test_quarter_single_note() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").quarter(&[C4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.duration, 0.5);
            assert_eq!(note.frequencies[0], C4);
        }
    }

    #[test]
    fn test_quarter_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").quarter(&[C4, E4, G4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.duration, 0.5);
            assert!(note.frequencies.contains(&C4));
            assert!(note.frequencies.contains(&E4));
            assert!(note.frequencies.contains(&G4));
        }
    }

    #[test]
    fn test_half_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").half(&[C4, E4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.duration, 1.0);
        }
    }

    #[test]
    fn test_whole_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").whole(&[C4, E4, G4, C5]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.duration, 2.0);
        }
    }

    #[test]
    fn test_eighth_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").eighth(&[C4, E4, G4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.duration, 0.25);
        }
    }

    #[test]
    fn test_sixteenth_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").sixteenth(&[C4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.duration, 0.125);
        }
    }

    // ===== CHAINING AND INTEGRATION =====

    #[test]
    fn test_at_bar_then_quarters() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .at_bar(3) // Bar 3 = 4.0 seconds
            .quarters(&[C4, D4]);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 2);

        // First note should start at bar 3
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.start_time, 4.0);
        }
    }

    #[test]
    fn test_complex_musical_time_sequence() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp
            .track("test")
            .at_bar(2) // Start at bar 2 (2.0 seconds)
            .quarter(&[C4]) // +0.5 = 2.5
            .eighth(&[D4]) // +0.25 = 2.75
            .beats(1.0) // +0.5 = 3.25
            .half(&[E4]); // +1.0 = 4.25

        assert!((builder.cursor - 4.25).abs() < 0.01);
    }

    #[test]
    fn test_empty_notes_array() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").quarters(&[]);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_wholes_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").wholes(&[C4, E4]);

        let track = &comp.into_mixer().tracks[0];

        // First note at 0.0, second at 2.0
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.start_time, 2.0);
        }
    }

    #[test]
    fn test_mixed_note_durations() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp
            .track("test")
            .wholes(&[C4]) // 2.0
            .halves(&[D4]) // +1.0 = 3.0
            .quarters(&[E4]) // +0.5 = 3.5
            .eighths(&[F4]) // +0.25 = 3.75
            .sixteenths(&[G4]); // +0.125 = 3.875

        assert!((builder.cursor - 3.875).abs() < 0.01);
    }

    #[test]
    fn test_at_beat_then_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .at_beat(5) // Beat 5 = 2.0 seconds
            .quarter(&[C4]);

        let track = &comp.into_mixer().tracks[0];

        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.start_time, 2.0);
        }
    }

    #[test]
    fn test_bars_between_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp
            .track("test")
            .quarter(&[C4]) // 0.0-0.5
            .bars(1.0) // +2.0 = 2.5
            .quarter(&[E4]); // 2.5-3.0

        assert_eq!(builder.cursor, 3.0);
    }

    #[test]
    fn test_dotted_notes_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .dotted_quarters(&[C4, E4]) // 0.75 each
            .dotted_eighths(&[G4, B4]); // 0.375 each

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);

        // Verify the progression
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.start_time, 0.0);
            assert_eq!(note.duration, 0.75);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.start_time, 0.75);
            assert_eq!(note.duration, 0.75);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            assert_eq!(note.start_time, 1.5);
            assert_eq!(note.duration, 0.375);
        }
    }

    #[test]
    fn test_single_and_multiple_methods_together() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp
            .track("test")
            .quarter(&[C4, E4, G4]) // Chord: 0.5
            .quarters(&[D4, E4]); // Notes: +1.0 = 1.5

        assert_eq!(builder.cursor, 1.5);
    }
}
