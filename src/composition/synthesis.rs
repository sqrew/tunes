use crate::composition::TrackBuilder;
use crate::synthesis::granular::{GranularParams, create_granular_events};
use crate::synthesis::noise::NoiseType;
use crate::prelude::{FMParams, FilterEnvelope};
use crate::synthesis::sample::Sample;
use crate::track::{AudioEvent, SampleEvent};

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
    /// # use tunes::synthesis::wavetable::{Wavetable, DEFAULT_TABLE_SIZE};
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
    pub fn custom_waveform(mut self, wavetable: crate::synthesis::wavetable::Wavetable) -> Self {
        self.custom_wavetable = Some(wavetable);
        self
    }

    /// Use additive synthesis with specified harmonic amplitudes
    ///
    /// Creates a custom waveform by summing harmonics with the given amplitudes.
    /// Each value in the array represents the amplitude of that harmonic (1st, 2nd, 3rd, etc.).
    ///
    /// # Arguments
    /// * `harmonic_amps` - Array of harmonic amplitudes (e.g., `&[1.0, 0.5, 0.25]`)
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Sawtooth-like sound (harmonic series with decreasing amplitudes)
    /// comp.track("saw")
    ///     .additive_synth(&[1.0, 0.5, 0.33, 0.25, 0.2])
    ///     .notes(&[C4, E4, G4], 0.5);
    ///
    /// // Organ sound (odd harmonics only)
    /// comp.track("organ")
    ///     .additive_synth(&[1.0, 0.0, 0.5, 0.0, 0.3])
    ///     .notes(&[C3], 1.0);
    /// ```
    pub fn additive_synth(self, harmonic_amps: &[f32]) -> Self {
        use crate::synthesis::wavetable::{Wavetable, DEFAULT_TABLE_SIZE};

        // Convert harmonic amplitudes to (harmonic_number, amplitude) pairs
        let harmonics: Vec<(usize, f32)> = harmonic_amps
            .iter()
            .enumerate()
            .filter(|(_, &amp)| amp > 0.0)  // Skip zero-amplitude harmonics
            .map(|(i, &amp)| (i + 1, amp))
            .collect();

        // Create wavetable from harmonics
        let wavetable = Wavetable::from_harmonics(DEFAULT_TABLE_SIZE, &harmonics);

        self.custom_waveform(wavetable)
    }

    /// Use wavetable synthesis with a rich, harmonically complex waveform
    ///
    /// Convenience method that creates a wavetable with a rich harmonic spectrum,
    /// good for synth leads and pads. For custom wavetables, use `.custom_waveform()`.
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Rich wavetable lead
    /// comp.track("lead")
    ///     .wavetable()
    ///     .notes(&[C4, D4, E4, G4], 0.5);
    /// ```
    pub fn wavetable(self) -> Self {
        use crate::synthesis::wavetable::{Wavetable, DEFAULT_TABLE_SIZE};

        // Create a rich wavetable with multiple harmonics for a complex timbre
        // Similar to a sawtooth but with more controlled harmonics
        let harmonics = vec![
            (1, 1.0),   // Fundamental
            (2, 0.6),   // Octave
            (3, 0.4),   // Fifth above octave
            (4, 0.3),   // Two octaves
            (5, 0.25),  // Major third above that
            (6, 0.2),   // Fifth above two octaves
            (7, 0.15),  // Minor seventh
            (8, 0.1),   // Three octaves
        ];

        let wavetable = Wavetable::from_harmonics(DEFAULT_TABLE_SIZE, &harmonics);
        self.custom_waveform(wavetable)
    }

    /// Add noise to the track
    ///
    /// Generates noise of the specified type and adds it as a sample at the current cursor position.
    ///
    /// # Arguments
    /// * `noise_type` - Type of noise to generate (White or Brown)
    /// * `duration` - Duration in seconds
    /// * `amplitude` - Volume/amplitude (0.0 to 1.0)
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::synthesis::noise::NoiseType;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Add white noise hi-hat
    /// comp.track("drums")
    ///     .noise(NoiseType::White, 0.1, 0.3);
    ///
    /// // Add brown noise bass rumble
    /// comp.track("fx")
    ///     .noise(NoiseType::Brown, 2.0, 0.5)
    ///     .filter(Filter::new(FilterType::LowPass, 500.0, 0.8));  // Low-pass for bass
    /// ```
    pub fn noise(mut self, noise_type: NoiseType, duration: f32, amplitude: f32) -> Self {
        // Calculate sample rate and duration in samples
        let sample_rate = 44100; // TODO: Could make this configurable
        // duration is already in seconds (consistent with .note() and other methods)
        let sample_count = (duration * sample_rate as f32) as usize;

        // Capture cursor position before mutable borrow
        let start_time = self.cursor;

        // Generate noise samples
        let mut noise_samples = noise_type.generate(sample_count);

        // Apply amplitude
        for sample in &mut noise_samples {
            *sample *= amplitude;
        }

        // Create a Sample from the noise
        let noise_sample = Sample::from_mono(noise_samples, sample_rate);

        // Create a SampleEvent
        let sample_event = SampleEvent {
            sample: noise_sample,
            start_time,
            playback_rate: 1.0,
            volume: 1.0, // Already applied amplitude above
            spatial_position: None,
        };

        // Get the track and add the event
        let track = self.get_track_mut();
        track.events.push(AudioEvent::Sample(sample_event));

        // Advance cursor (duration is in seconds)
        self.cursor += duration;

        self
    }

    /// Apply granular synthesis to a sample file
    ///
    /// Granular synthesis breaks audio into tiny "grains" (5-100ms) and rearranges/overlaps them
    /// to create rich textures, time-stretching effects, or surreal sonic transformations.
    ///
    /// # Arguments
    /// * `sample_path` - Path to the audio file to granulate
    /// * `params` - Granular synthesis parameters (use presets or create custom)
    /// * `duration` - Output duration in seconds
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # use tunes::synthesis::granular::GranularParams;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Create lush texture from a vocal sample
    /// comp.track("texture")
    ///     .granular("voice.wav", GranularParams::texture(), 5.0);
    ///
    /// // Freeze a moment in time
    /// comp.track("frozen")
    ///     .granular("guitar.wav", GranularParams::freeze(), 3.0);
    ///
    /// // Glitchy stutters
    /// comp.track("glitch")
    ///     .granular("drums.wav", GranularParams::glitch(), 2.0);
    /// ```
    pub fn granular(mut self, sample_path: &str, params: GranularParams, duration: f32) -> Self {
        // Load the sample
        let source_sample = match Sample::from_file(sample_path) {
            Ok(sample) => sample,
            Err(e) => {
                eprintln!("Error loading sample '{}': {}", sample_path, e);
                return self;
            }
        };

        // Capture cursor before mutable borrow
        let start_time = self.cursor;

        // Generate all the grain events
        let grain_events = create_granular_events(&source_sample, &params, duration, start_time);

        // Add all events to the track
        let track = self.get_track_mut();
        for event in grain_events {
            track.events.push(AudioEvent::Sample(event));
        }

        // Advance cursor
        self.cursor += duration;

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::consts::notes::*;
    use crate::composition::rhythm::Tempo;
    use crate::track::AudioEvent;
    use crate::synthesis::wavetable::{DEFAULT_TABLE_SIZE, Wavetable};

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
        let track = &mixer.tracks()[0];

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
        let track = &mixer.tracks()[0];

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
        let env = crate::synthesis::envelope::Envelope::new(0.1, 0.2, 0.7, 0.3);

        // Use custom waveform with envelope
        comp.track("test")
            .custom_waveform(custom_wt)
            .envelope(env)
            .note(&[440.0], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];

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
        let track = &mixer.tracks()[0];

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
        let track = &mixer.tracks()[0];

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
