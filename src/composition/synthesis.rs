use crate::composition::TrackBuilder;
use crate::prelude::{FMParams, FilterEnvelope};

/// Synthesis methods for TrackBuilder
///
/// These methods provide convenient access to advanced synthesis features:
/// - Filter envelopes for subtractive synthesis
/// - FM synthesis for complex harmonic timbres
impl<'a> TrackBuilder<'a> {
    /// Set the filter envelope for subsequent notes
    ///
    /// The filter envelope controls how the filter cutoff frequency changes over time,
    /// creating classic subtractive synthesis sweeps.
    ///
    /// # Arguments
    /// * `filter_env` - FilterEnvelope to use
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("synth")
    ///     .filter_envelope(FilterEnvelope::classic())
    ///     .note(&[440.0], 1.0);
    /// ```
    pub fn filter_envelope(mut self, filter_env: FilterEnvelope) -> Self {
        // Store the filter envelope for subsequent notes
        // We'll need to track this in the builder state
        self.filter_envelope = filter_env;
        self
    }

    /// Set FM synthesis parameters for subsequent notes
    ///
    /// FM (Frequency Modulation) synthesis creates complex, harmonically rich timbres
    /// by modulating the frequency of one oscillator with another.
    ///
    /// # Arguments
    /// * `fm_params` - FM synthesis parameters
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("fm_piano")
    ///     .fm(FMParams::electric_piano())
    ///     .note(&[C4], 0.5)
    ///     .note(&[E4], 0.5);
    /// ```
    pub fn fm(mut self, fm_params: FMParams) -> Self {
        self.fm_params = fm_params;
        self
    }

    /// Create a custom FM sound with specific parameters
    ///
    /// # Arguments
    /// * `mod_ratio` - Modulator to carrier frequency ratio
    /// * `mod_index` - Modulation index (brightness, 0.0 to 10.0+)
    pub fn fm_custom(self, mod_ratio: f32, mod_index: f32) -> Self {
        self.fm(FMParams::new(mod_ratio, mod_index))
    }

    /// Combine filter envelope and FM for rich, evolving timbres
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("fm_synth")
    ///     .fm(FMParams::bell())
    ///     .filter_envelope(FilterEnvelope::classic())
    ///     .note(&[C4], 1.0);
    /// ```
    pub fn fm_with_filter(self, fm_params: FMParams, filter_env: FilterEnvelope) -> Self {
        self.fm(fm_params).filter_envelope(filter_env)
    }

    /// Use a custom wavetable for subsequent notes
    ///
    /// This allows you to use custom waveforms created with `Wavetable::from_fn()`,
    /// `Wavetable::from_harmonics()`, or other wavetable constructors.
    /// When set, this overrides the standard `Waveform` enum.
    ///
    /// # Arguments
    /// * `wavetable` - Custom wavetable to use for oscillation
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::wavetable::{Wavetable, DEFAULT_TABLE_SIZE};
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Create a custom wavetable with odd harmonics
    /// let organ_wt = Wavetable::from_harmonics(
    ///     DEFAULT_TABLE_SIZE,
    ///     &[(1, 1.0), (3, 0.5), (5, 0.3), (7, 0.2)],
    /// );
    ///
    /// comp.track("organ")
    ///     .custom_waveform(organ_wt)
    ///     .notes(&[C3, E3, G3], 0.5);
    /// ```
    pub fn custom_waveform(mut self, wavetable: crate::wavetable::Wavetable) -> Self {
        self.custom_wavetable = Some(wavetable);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::notes::*;
    use crate::rhythm::Tempo;
    use crate::track::AudioEvent;
    use crate::wavetable::{DEFAULT_TABLE_SIZE, Wavetable};

    #[test]
    fn test_custom_waveform_stored_in_note() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Create a custom wavetable
        let custom_wt =
            Wavetable::from_harmonics(DEFAULT_TABLE_SIZE, &[(1, 1.0), (3, 0.5), (5, 0.3)]);

        // Use it in a track
        comp.track("test")
            .custom_waveform(custom_wt)
            .note(&[440.0], 1.0);

        // Verify the note has the custom wavetable
        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];

        if let AudioEvent::Note(note) = &track.events[0] {
            assert!(
                note.custom_wavetable.is_some(),
                "Custom wavetable should be stored in note"
            );
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_custom_waveform_persists_across_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let custom_wt = Wavetable::triangle();

        // Set custom waveform and play multiple notes
        comp.track("test")
            .custom_waveform(custom_wt)
            .notes(&[C4, E4, G4], 0.5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];

        // All three notes should have the custom wavetable
        assert_eq!(track.events.len(), 3);
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert!(
                    note.custom_wavetable.is_some(),
                    "All notes should have custom wavetable"
                );
            }
        }
    }

    #[test]
    fn test_custom_waveform_combines_with_envelope() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let custom_wt = Wavetable::pwm(0.25);
        let env = crate::envelope::Envelope::new(0.1, 0.2, 0.7, 0.3);

        // Use custom waveform with envelope
        comp.track("test")
            .custom_waveform(custom_wt)
            .envelope(env)
            .note(&[440.0], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];

        if let AudioEvent::Note(note) = &track.events[0] {
            assert!(note.custom_wavetable.is_some());
            // Verify envelope is also set
            assert_eq!(note.envelope.attack, 0.1);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_custom_waveform_combines_with_fm() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let custom_wt = Wavetable::saw_bandlimited();

        // Set both custom waveform and FM
        comp.track("test")
            .custom_waveform(custom_wt.clone())
            .fm_custom(2.0, 3.0)
            .note(&[440.0], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];

        if let AudioEvent::Note(note) = &track.events[0] {
            // When FM is active, it takes precedence in rendering
            // But custom wavetable should still be stored
            assert!(note.custom_wavetable.is_some());
            assert!(note.fm_params.mod_index > 0.0);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_custom_waveform_can_be_cleared() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let custom_wt = Wavetable::sine();

        // Use custom waveform for first note
        comp.track("test")
            .custom_waveform(custom_wt)
            .note(&[C4], 0.5);

        // Second note without custom waveform (new builder)
        comp.track("test").note(&[E4], 0.5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];

        // First note has custom wavetable
        if let AudioEvent::Note(note) = &track.events[0] {
            assert!(note.custom_wavetable.is_some());
        }

        // Second note doesn't have custom wavetable (builder was recreated)
        if let AudioEvent::Note(note) = &track.events[1] {
            assert!(note.custom_wavetable.is_none());
        }
    }
}
