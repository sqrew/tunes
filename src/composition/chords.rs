use super::TrackBuilder;
use crate::theory::core::{chord, chord_inversion, chord_over_bass as create_chord_over_bass, voice_lead, ChordPattern};

impl<'a> TrackBuilder<'a> {
    /// Play a chord at the current cursor position
    ///
    /// Creates a chord from a root note and pattern, plays it, and stores it for voice leading.
    ///
    /// # Arguments
    /// * `root` - Root note frequency (e.g., C4, D4)
    /// * `pattern` - Chord pattern (e.g., ChordPattern::MAJOR, ChordPattern::MINOR7)
    /// * `duration` - Duration in seconds
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::theory::core::ChordPattern;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("piano", &Instrument::electric_piano())
    ///     .chord(C4, &ChordPattern::MAJOR, 1.0)
    ///     .chord(G4, &ChordPattern::MAJOR, 1.0)
    ///     .chord(A4, &ChordPattern::MINOR, 1.0);
    /// ```
    pub fn chord(mut self, root: f32, pattern: &ChordPattern, duration: f32) -> Self {
        let chord_notes = chord(root, pattern);

        // Store this chord for voice leading
        self.last_chord = Some(chord_notes.clone());

        // Play the chord using the existing note() method
        self.note(&chord_notes, duration)
    }

    /// Play a chord inversion at the current cursor position
    ///
    /// Creates a chord, inverts it, plays it, and stores it for voice leading.
    ///
    /// # Arguments
    /// * `root` - Root note frequency (e.g., C4, D4)
    /// * `pattern` - Chord pattern (e.g., ChordPattern::MAJOR, ChordPattern::MINOR7)
    /// * `inversion` - Inversion number (0 = root position, 1 = first inversion, 2 = second inversion, etc.)
    /// * `duration` - Duration in seconds
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::theory::core::ChordPattern;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("piano", &Instrument::electric_piano())
    ///     .chord(C4, &ChordPattern::MAJOR, 1.0)          // C-E-G (root position)
    ///     .chord_inverted(C4, &ChordPattern::MAJOR, 1, 1.0)  // E-G-C (first inversion)
    ///     .chord_inverted(C4, &ChordPattern::MAJOR, 2, 1.0); // G-C-E (second inversion)
    /// ```
    pub fn chord_inverted(mut self, root: f32, pattern: &ChordPattern, inversion: usize, duration: f32) -> Self {
        let base_chord = chord(root, pattern);
        let inverted_chord = chord_inversion(&base_chord, inversion);

        // Store this chord for voice leading
        self.last_chord = Some(inverted_chord.clone());

        // Play the chord
        self.note(&inverted_chord, duration)
    }

    /// Play a chord with automatic voice leading from the previous chord
    ///
    /// Creates a chord and applies smooth voice leading from the last played chord.
    /// If no previous chord exists, plays the chord in root position.
    ///
    /// # Arguments
    /// * `root` - Root note frequency (e.g., C4, D4)
    /// * `pattern` - Chord pattern (e.g., ChordPattern::MAJOR, ChordPattern::MINOR7)
    /// * `duration` - Duration in seconds
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::theory::core::ChordPattern;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Smooth jazz progression with voice leading
    /// comp.instrument("piano", &Instrument::electric_piano())
    ///     .chord(C4, &ChordPattern::MAJOR7, 2.0)        // Cmaj7 (no voice leading - first chord)
    ///     .chord_voice_lead(D4, &ChordPattern::MINOR7, 2.0)  // Dm7 (voice led from Cmaj7)
    ///     .chord_voice_lead(G4, &ChordPattern::DOMINANT7, 2.0)  // G7 (voice led from Dm7)
    ///     .chord_voice_lead(C4, &ChordPattern::MAJOR7, 2.0);    // Cmaj7 (voice led from G7)
    /// ```
    pub fn chord_voice_lead(mut self, root: f32, pattern: &ChordPattern, duration: f32) -> Self {
        let target_chord = chord(root, pattern);

        let voiced_chord = if let Some(ref last) = self.last_chord {
            // Apply voice leading from the last chord
            voice_lead(last, &target_chord)
        } else {
            // No previous chord - use root position
            target_chord.clone()
        };

        // Store this chord for the next voice leading
        self.last_chord = Some(voiced_chord.clone());

        // Play the chord
        self.note(&voiced_chord, duration)
    }

    /// Play a slash chord (chord with a different bass note)
    ///
    /// Creates a chord with a specific bass note, plays it, and stores it for voice leading.
    ///
    /// # Arguments
    /// * `root` - Root note frequency for the chord (e.g., C4)
    /// * `pattern` - Chord pattern (e.g., ChordPattern::MAJOR)
    /// * `bass` - Bass note frequency (e.g., E3 for C/E)
    /// * `duration` - Duration in seconds
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::theory::core::ChordPattern;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("piano", &Instrument::electric_piano())
    ///     .chord(C4, &ChordPattern::MAJOR, 1.0)         // C major (C-E-G)
    ///     .chord_over_bass(C4, &ChordPattern::MAJOR, E3, 1.0)  // C/E (E-C-E-G)
    ///     .chord_over_bass(C4, &ChordPattern::MAJOR, G3, 1.0); // C/G (G-C-E-G)
    /// ```
    pub fn chord_over_bass(mut self, root: f32, pattern: &ChordPattern, bass: f32, duration: f32) -> Self {
        let base_chord = chord(root, pattern);
        let slash_chord = create_chord_over_bass(&base_chord, bass);

        // Store this chord for voice leading
        self.last_chord = Some(slash_chord.clone());

        // Play the chord
        self.note(&slash_chord, duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::Composition;
    use crate::composition::rhythm::Tempo;
    use crate::consts::notes::*;
    use crate::instruments::Instrument;
    use crate::track::AudioEvent;

    #[test]
    fn test_chord_creates_and_plays_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("piano", &Instrument::electric_piano())
            .chord(C4, &ChordPattern::MAJOR, 1.0);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.num_freqs, 3); // Major triad has 3 notes
            assert_eq!(note.duration, 1.0);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_chord_inverted_creates_inversion() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("piano", &Instrument::electric_piano())
            .chord_inverted(C4, &ChordPattern::MAJOR, 1, 1.0);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
            // First inversion should have E as lowest note
            assert!(note.frequencies[0] > C4 && note.frequencies[0] < G4);
        }
    }

    #[test]
    fn test_chord_voice_lead_without_previous() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("piano", &Instrument::electric_piano())
            .chord_voice_lead(C4, &ChordPattern::MAJOR, 1.0);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);
    }

    #[test]
    fn test_chord_voice_lead_with_previous() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("piano", &Instrument::electric_piano())
            .chord(C4, &ChordPattern::MAJOR, 1.0)
            .chord_voice_lead(G4, &ChordPattern::MAJOR, 1.0);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 2);
    }

    #[test]
    fn test_chord_over_bass() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("piano", &Instrument::electric_piano())
            .chord_over_bass(C4, &ChordPattern::MAJOR, E3, 1.0);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 1);

        if let AudioEvent::Note(note) = &track.events[0] {
            // Bass note should be the lowest frequency
            assert_eq!(note.frequencies[0], E3);
        }
    }

    #[test]
    fn test_chaining_multiple_chords() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("piano", &Instrument::electric_piano())
            .chord(C4, &ChordPattern::MAJOR, 1.0)
            .chord(G4, &ChordPattern::MAJOR, 1.0)
            .chord(A4, &ChordPattern::MINOR, 1.0)
            .chord(F4, &ChordPattern::MAJOR, 1.0);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);
    }

    #[test]
    fn test_mixed_chord_methods() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("piano", &Instrument::electric_piano())
            .chord(C4, &ChordPattern::MAJOR, 1.0)
            .chord_inverted(C4, &ChordPattern::MAJOR, 1, 1.0)
            .chord_voice_lead(G4, &ChordPattern::MAJOR, 1.0)
            .chord_over_bass(F4, &ChordPattern::MAJOR, A3, 1.0);

        let track = &comp.into_mixer().tracks[0];
        assert_eq!(track.events.len(), 4);
    }
}
