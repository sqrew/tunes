use super::TrackBuilder;
use crate::synthesis::envelope::Envelope;
use crate::synthesis::waveform::Waveform;

impl<'a> TrackBuilder<'a> {
    /// Set the volume for this track (0.0 to 2.0)
    pub fn volume(mut self, volume: f32) -> Self {
        self.get_track_mut().volume = volume.clamp(0.0, 2.0);
        self
    }
    /// Set the stereo pan for this track (-1.0 = left, 0.0 = center, 1.0 = right)
    pub fn pan(mut self, pan: f32) -> Self {
        self.get_track_mut().pan = pan.clamp(-1.0, 1.0);
        self
    }

    /// Set 3D spatial position for subsequent notes, drums, and samples
    ///
    /// Places sounds in 3D space relative to the listener. Spatial audio automatically
    /// applies distance attenuation and stereo panning based on the sound's position.
    ///
    /// # Arguments
    /// * `x` - X coordinate (left/right: negative = left, positive = right)
    /// * `y` - Y coordinate (up/down: negative = below, positive = above)
    /// * `z` - Z coordinate (forward/back: negative = behind, positive = in front)
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Place guitar 2 meters to the right and 5 meters forward
    /// comp.track("guitar")
    ///     .spatial_position(2.0, 0.0, 5.0)
    ///     .note(&[C4], 1.0);
    ///
    /// // Place drums at listener position (center)
    /// comp.track("drums")
    ///     .spatial_position(0.0, 0.0, 0.0)
    ///     .note(&[D4], 1.0);
    /// ```
    pub fn spatial_position(mut self, x: f32, y: f32, z: f32) -> Self {
        use crate::synthesis::spatial::SpatialPosition;
        self.spatial_position = Some(SpatialPosition::new(x, y, z));
        self
    }
    /// Set the MIDI program (instrument) for this track (0-127)
    ///
    /// This sets the General MIDI program number used when exporting to MIDI.
    /// Common programs: 0 = Acoustic Grand Piano, 24 = Acoustic Guitar (nylon),
    /// 33 = Acoustic Bass, 48 = String Ensemble, 80 = Square Lead, etc.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("piano")
    ///     .program(0)  // Acoustic Grand Piano
    ///     .notes(&[C4, E4, G4], 0.5);
    ///
    /// comp.track("bass")
    ///     .program(33)  // Acoustic Bass
    ///     .notes(&[C2, G2], 1.0);
    /// ```
    pub fn program(mut self, program: u8) -> Self {
        self.get_track_mut().midi_program = Some(program.min(127));
        self
    }
    /// Set pitch bend for subsequent notes (in semitones, positive = up, negative = down)
    pub fn bend(mut self, semitones: f32) -> Self {
        self.pitch_bend = semitones.clamp(-24.0, 24.0);
        self
    }
    /// Set velocity for subsequent notes (0.0 to 1.0)
    ///
    /// Velocity affects the note's expression and is used in MIDI export.
    /// Higher velocity values typically result in louder, more emphasized notes.
    ///
    /// # Arguments
    /// * `velocity` - Note velocity from 0.0 (silent) to 1.0 (maximum), clamped to range
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .velocity(0.9)  // Strong accent
    ///     .note(&[C4], 0.5)
    ///     .velocity(0.4)  // Soft note
    ///     .note(&[E4], 0.5);
    /// ```
    pub fn velocity(mut self, velocity: f32) -> Self {
        self.velocity = velocity.clamp(0.0, 1.0);
        self
    }
    /// Set the waveform for subsequent notes
    pub fn waveform(mut self, waveform: Waveform) -> Self {
        self.waveform = waveform;
        self
    }
    /// Set the envelope for subsequent notes
    pub fn envelope(mut self, envelope: Envelope) -> Self {
        self.envelope = envelope;
        self
    }
    /// Add vibrato effect to the track
    ///
    /// Applies pitch modulation (vibrato) using an LFO. This affects all notes
    /// played on this track after calling this method.
    ///
    /// # Arguments
    /// * `rate` - Vibrato speed in Hz (typical: 4-7 Hz)
    /// * `depth` - Vibrato depth (0.0 to 1.0, where 1.0 = ±100 cents)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("violin", &Instrument::synth_lead())
    ///     .vibrato(5.5, 0.3)  // Moderate vibrato at 5.5 Hz
    ///     .note(&[A4], 2.0);
    /// ```
    pub fn vibrato(mut self, rate: f32, depth: f32) -> Self {
        use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
        let vibrato_lfo = LFO::new(Waveform::Sine, rate, depth);
        let mod_route = ModRoute::new(vibrato_lfo, ModTarget::Pitch, 1.0);
        self.get_track_mut().modulation.push(mod_route);
        self
    }
    /// Fade volume from current level to a target level over a duration
    ///
    /// Transitions the track volume from its current value to the target value.
    /// This advances the cursor by the duration, treating it as a "wait during fade" period.
    ///
    /// **Note:** This is a simplified implementation that sets the final volume
    /// and advances time. For true smooth fading, notes would need per-sample
    /// volume envelopes. Consider using long envelope release times for fade-out
    /// effects on individual notes.
    ///
    /// # Arguments
    /// * `target_volume` - The destination volume (0.0 to 2.0)
    /// * `duration` - Time to wait while "fading" (advances cursor)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("pad", &Instrument::warm_pad())
    ///     .volume(1.0)        // Start at full volume
    ///     .note(&[C4, E4, G4], 4.0)
    ///     .fade_to(0.0, 2.0)  // Set to 0, wait 2 seconds
    ///     .wait(1.0);
    /// ```
    pub fn fade_to(mut self, target_volume: f32, duration: f32) -> Self {
        let target_volume = target_volume.clamp(0.0, 2.0);

        // Set the new volume
        self.get_track_mut().volume = target_volume;

        // Advance cursor to account for fade duration
        self.cursor += duration;

        self
    }

    /// Pan from current position to target position over a duration
    ///
    /// Transitions the track pan from its current value to the target value.
    /// This advances the cursor by the duration, treating it as a "wait during pan" period.
    ///
    /// **Note:** This is a simplified implementation. For smooth panning on individual
    /// notes, consider using short note sequences with gradually changing pan values.
    ///
    /// # Arguments
    /// * `target_pan` - The destination pan (-1.0 = left, 0.0 = center, 1.0 = right)
    /// * `duration` - Time to wait while "panning" (advances cursor)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("synth", &Instrument::synth_lead())
    ///     .pan(-1.0)          // Start hard left
    ///     .note(&[C4], 0.5)
    ///     .pan_to(1.0, 2.0)   // Set to right, wait 2 seconds
    ///     .notes(&[D4, E4, F4, G4], 0.5);
    /// ```
    pub fn pan_to(mut self, target_pan: f32, duration: f32) -> Self {
        let target_pan = target_pan.clamp(-1.0, 1.0);

        // Set the new pan
        self.get_track_mut().pan = target_pan;

        // Advance cursor to account for pan duration
        self.cursor += duration;

        self
    }

    /// Sweep filter cutoff from current to target frequency over a duration
    ///
    /// Transitions the filter cutoff from its current value to the target value.
    /// This advances the cursor by the duration, treating it as a "wait during sweep" period.
    ///
    /// **Note:** This is a simplified implementation. For smooth filter sweeps,
    /// consider using filter envelope or LFO modulation on the filter cutoff.
    ///
    /// # Arguments
    /// * `target_cutoff` - The destination cutoff frequency in Hz
    /// * `duration` - Time to wait while "sweeping" (advances cursor)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # use tunes::synthesis::filter::Filter;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("bass", &Instrument::pluck())
    ///     .filter(Filter::low_pass(200.0, 0.7))
    ///     .note(&[C2], 4.0)
    ///     .filter_sweep(2000.0, 4.0);  // Set to 2000Hz, wait 4 seconds
    /// ```
    pub fn filter_sweep(mut self, target_cutoff: f32, duration: f32) -> Self {
        // Set the new filter cutoff
        self.get_track_mut().filter.cutoff = target_cutoff;

        // Advance cursor to account for sweep duration
        self.cursor += duration;

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::synthesis::envelope::Envelope;
    use crate::consts::notes::*;
    use crate::composition::rhythm::Tempo;
    use crate::synthesis::waveform::Waveform;

    #[test]
    fn test_volume_sets_track_volume() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").volume(0.7);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        assert_eq!(track.volume, 0.7);
    }

    #[test]
    fn test_volume_clamps_low() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").volume(-0.5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        assert_eq!(track.volume, 0.0);
    }

    #[test]
    fn test_volume_clamps_high() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").volume(3.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        assert_eq!(track.volume, 2.0);
    }

    #[test]
    fn test_volume_allows_boundaries() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("min").volume(0.0);
        comp.track("max").volume(2.0);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks.len(), 2);

        // Check that both boundary volumes exist (HashMap order not guaranteed)
        let has_min = mixer.tracks.iter().any(|t| t.volume == 0.0);
        let has_max = mixer.tracks.iter().any(|t| t.volume == 2.0);

        assert!(has_min, "Should have a track with volume 0.0");
        assert!(has_max, "Should have a track with volume 2.0");
    }

    #[test]
    fn test_pan_sets_track_pan() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").pan(0.5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        assert_eq!(track.pan, 0.5);
    }

    #[test]
    fn test_pan_clamps_low() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").pan(-1.5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        assert_eq!(track.pan, -1.0);
    }

    #[test]
    fn test_pan_clamps_high() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").pan(1.5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        assert_eq!(track.pan, 1.0);
    }

    #[test]
    fn test_pan_center() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").pan(0.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        assert_eq!(track.pan, 0.0);
    }

    #[test]
    fn test_bend_sets_pitch_bend() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").bend(2.0).note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.pitch_bend_semitones, 2.0);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_bend_clamps_low() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").bend(-30.0).note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.pitch_bend_semitones, -24.0);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_bend_clamps_high() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").bend(30.0).note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.pitch_bend_semitones, 24.0);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_waveform_sets_for_subsequent_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .waveform(Waveform::Square)
            .note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert!(matches!(note.waveform, Waveform::Square));
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_envelope_sets_for_subsequent_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let env = Envelope::new(0.1, 0.2, 0.7, 0.3);
        comp.track("test").envelope(env).note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.envelope.attack, 0.1);
            assert_eq!(note.envelope.decay, 0.2);
            assert_eq!(note.envelope.sustain, 0.7);
            assert_eq!(note.envelope.release, 0.3);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_vibrato_adds_modulation() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").vibrato(5.0, 0.3);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        // Vibrato should add a modulation route
        assert_eq!(track.modulation.len(), 1);
    }

    #[test]
    fn test_vibrato_multiple_calls() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").vibrato(5.0, 0.3).vibrato(3.0, 0.1);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        // Should have 2 modulation routes
        assert_eq!(track.modulation.len(), 2);
    }

    #[test]
    fn test_fade_to_sets_volume() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").volume(1.0).fade_to(0.0, 2.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        assert_eq!(track.volume, 0.0);
    }

    #[test]
    fn test_fade_to_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").fade_to(0.5, 3.0).note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            // Note should start at 3.0 (after the fade duration)
            assert_eq!(note.start_time, 3.0);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_fade_to_clamps_volume() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").fade_to(3.0, 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        assert_eq!(track.volume, 2.0);
    }

    #[test]
    fn test_expression_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let env = Envelope::new(0.1, 0.1, 0.8, 0.2);
        comp.track("test")
            .volume(0.8)
            .pan(-0.3)
            .bend(1.0)
            .waveform(Waveform::Sawtooth)
            .envelope(env)
            .vibrato(5.5, 0.2)
            .note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        assert_eq!(track.volume, 0.8);
        assert_eq!(track.pan, -0.3);
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.pitch_bend_semitones, 1.0);
            assert!(matches!(note.waveform, Waveform::Sawtooth));
            assert_eq!(note.envelope.attack, 0.1);
        } else {
            panic!("Expected NoteEvent");
        }
        assert_eq!(track.modulation.len(), 1);
    }

    #[test]
    fn test_volume_with_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").volume(0.5).note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks[0].volume, 0.5);
    }

    #[test]
    fn test_pan_extremes() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("left").pan(-1.0);
        comp.track("right").pan(1.0);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks.len(), 2);

        // Check that both pan extremes exist (HashMap order not guaranteed)
        let has_left = mixer.tracks.iter().any(|t| t.pan == -1.0);
        let has_right = mixer.tracks.iter().any(|t| t.pan == 1.0);

        assert!(has_left, "Should have a track panned hard left (-1.0)");
        assert!(has_right, "Should have a track panned hard right (1.0)");
    }

    #[test]
    fn test_bend_zero_is_no_bend() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").bend(0.0).note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.pitch_bend_semitones, 0.0);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_velocity_sets_note_velocity() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").velocity(0.6).note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.velocity, 0.6);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_velocity_clamps_low() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").velocity(-0.5).note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.velocity, 0.0);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_velocity_clamps_high() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").velocity(1.5).note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.velocity, 1.0);
        } else {
            panic!("Expected NoteEvent");
        }
    }

    #[test]
    fn test_velocity_affects_multiple_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test").velocity(0.9).notes(&[C4, E4, G4], 0.5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];

        // All notes should have the set velocity
        for event in &track.events {
            if let crate::track::AudioEvent::Note(note) = event {
                assert_eq!(note.velocity, 0.9);
            }
        }
    }

    #[test]
    fn test_velocity_can_change_between_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .velocity(0.9)
            .note(&[C4], 0.5)
            .velocity(0.3)
            .note(&[E4], 0.5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];

        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.velocity, 0.9);
        }
        if let crate::track::AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.velocity, 0.3);
        }
    }

    #[test]
    fn test_fade_to_zero_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .volume(1.0)
            .fade_to(0.0, 0.0)
            .note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks[0];
        // Should set volume immediately
        assert_eq!(track.volume, 0.0);
        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            // Note should start at 0.0 (no fade duration)
            assert_eq!(note.start_time, 0.0);
        } else {
            panic!("Expected NoteEvent");
        }
    }
}
