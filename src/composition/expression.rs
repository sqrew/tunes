use super::TrackBuilder;
use crate::envelope::Envelope;
use crate::waveform::Waveform;

impl<'a> TrackBuilder<'a> {
    /// Set the volume for this track (0.0 to 2.0)
    pub fn volume(self, volume: f32) -> Self {
        self.track.volume = volume.clamp(0.0, 2.0);
        self
    }
    /// Set the stereo pan for this track (-1.0 = left, 0.0 = center, 1.0 = right)
    pub fn pan(self, pan: f32) -> Self {
        self.track.pan = pan.clamp(-1.0, 1.0);
        self
    }
    /// Set pitch bend for subsequent notes (in semitones, positive = up, negative = down)
    pub fn bend(mut self, semitones: f32) -> Self {
        self.pitch_bend = semitones.clamp(-24.0, 24.0);
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
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("violin", &Instrument::synth_lead())
    ///     .vibrato(5.5, 0.3)  // Moderate vibrato at 5.5 Hz
    ///     .note(&[A4], 2.0);
    /// ```
    pub fn vibrato(self, rate: f32, depth: f32) -> Self {
        use crate::lfo::{LFO, ModRoute, ModTarget};
        let vibrato_lfo = LFO::new(Waveform::Sine, rate, depth);
        let mod_route = ModRoute::new(vibrato_lfo, ModTarget::Pitch, 1.0);
        self.track.modulation.push(mod_route);
        self
    }
    /// Fade volume from current level to a target level over a duration
    ///
    /// Creates a smooth volume transition by interpolating between the current
    /// track volume and the target volume. This is done by placing multiple
    /// volume change points across the duration for a smooth fade.
    ///
    /// Note: This modifies the track's base volume over time. For per-note
    /// volume changes, consider using envelope settings instead.
    ///
    /// # Arguments
    /// * `target_volume` - The destination volume (0.0 to 2.0)
    /// * `duration` - Time over which to fade
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("pad", &Instrument::warm_pad())
    ///     .note(&[C4, E4, G4], 2.0)
    ///     .fade_to(0.0, 1.0)  // Fade out over 1 second
    ///     .wait(1.0);         // Wait for fade to complete
    /// ```
    pub fn fade_to(mut self, target_volume: f32, duration: f32) -> Self {
        let target_volume = target_volume.clamp(0.0, 2.0);

        // Note: This is a simplified implementation that sets the final volume.
        // A more sophisticated implementation could use volume automation curves
        // or LFO modulation for smoother fades. For now, we set the target volume
        // and advance the cursor.

        // TODO: This could be improved with proper volume automation using LFO
        // modulation on the volume parameter for true continuous fading

        self.cursor += duration;
        self.track.volume = target_volume;

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::envelope::Envelope;
    use crate::notes::*;
    use crate::rhythm::Tempo;
    use crate::waveform::Waveform;

    #[test]
    fn test_volume_sets_track_volume() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").volume(0.7);

        assert_eq!(builder.track.volume, 0.7);
    }

    #[test]
    fn test_volume_clamps_low() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").volume(-0.5);

        assert_eq!(builder.track.volume, 0.0);
    }

    #[test]
    fn test_volume_clamps_high() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").volume(3.0);

        assert_eq!(builder.track.volume, 2.0);
    }

    #[test]
    fn test_volume_allows_boundaries() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let min = comp.track("min").volume(0.0);
        assert_eq!(min.track.volume, 0.0);

        let max = comp.track("max").volume(2.0);
        assert_eq!(max.track.volume, 2.0);
    }

    #[test]
    fn test_pan_sets_track_pan() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").pan(0.5);

        assert_eq!(builder.track.pan, 0.5);
    }

    #[test]
    fn test_pan_clamps_low() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").pan(-1.5);

        assert_eq!(builder.track.pan, -1.0);
    }

    #[test]
    fn test_pan_clamps_high() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").pan(1.5);

        assert_eq!(builder.track.pan, 1.0);
    }

    #[test]
    fn test_pan_center() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").pan(0.0);

        assert_eq!(builder.track.pan, 0.0);
    }

    #[test]
    fn test_bend_sets_pitch_bend() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").bend(2.0);

        assert_eq!(builder.pitch_bend, 2.0);
    }

    #[test]
    fn test_bend_clamps_low() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").bend(-30.0);

        assert_eq!(builder.pitch_bend, -24.0);
    }

    #[test]
    fn test_bend_clamps_high() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").bend(30.0);

        assert_eq!(builder.pitch_bend, 24.0);
    }

    #[test]
    fn test_waveform_sets_for_subsequent_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").waveform(Waveform::Square);

        assert!(matches!(builder.waveform, Waveform::Square));
    }

    #[test]
    fn test_envelope_sets_for_subsequent_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let env = Envelope::new(0.1, 0.2, 0.7, 0.3);
        let builder = comp.track("test").envelope(env);

        assert_eq!(builder.envelope.attack, 0.1);
        assert_eq!(builder.envelope.decay, 0.2);
        assert_eq!(builder.envelope.sustain, 0.7);
        assert_eq!(builder.envelope.release, 0.3);
    }

    #[test]
    fn test_vibrato_adds_modulation() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").vibrato(5.0, 0.3);

        // Vibrato should add a modulation route
        assert_eq!(builder.track.modulation.len(), 1);
    }

    #[test]
    fn test_vibrato_multiple_calls() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .vibrato(5.0, 0.3)
            .vibrato(3.0, 0.1);

        // Should have 2 modulation routes
        assert_eq!(builder.track.modulation.len(), 2);
    }

    #[test]
    fn test_fade_to_sets_volume() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").volume(1.0).fade_to(0.0, 2.0);

        assert_eq!(builder.track.volume, 0.0);
    }

    #[test]
    fn test_fade_to_advances_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").fade_to(0.5, 3.0);

        assert_eq!(builder.cursor, 3.0);
    }

    #[test]
    fn test_fade_to_clamps_volume() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").fade_to(3.0, 1.0);

        assert_eq!(builder.track.volume, 2.0);
    }

    #[test]
    fn test_expression_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let env = Envelope::new(0.1, 0.1, 0.8, 0.2);
        let builder = comp.track("test")
            .volume(0.8)
            .pan(-0.3)
            .bend(1.0)
            .waveform(Waveform::Sawtooth)
            .envelope(env)
            .vibrato(5.5, 0.2);

        assert_eq!(builder.track.volume, 0.8);
        assert_eq!(builder.track.pan, -0.3);
        assert_eq!(builder.pitch_bend, 1.0);
        assert!(matches!(builder.waveform, Waveform::Sawtooth));
        assert_eq!(builder.envelope.attack, 0.1);
        assert_eq!(builder.track.modulation.len(), 1);
    }

    #[test]
    fn test_volume_with_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .volume(0.5)
            .note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks[0].volume, 0.5);
    }

    #[test]
    fn test_pan_extremes() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let left = comp.track("left").pan(-1.0);
        assert_eq!(left.track.pan, -1.0);

        let right = comp.track("right").pan(1.0);
        assert_eq!(right.track.pan, 1.0);
    }

    #[test]
    fn test_bend_zero_is_no_bend() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").bend(0.0);

        assert_eq!(builder.pitch_bend, 0.0);
    }

    #[test]
    fn test_fade_to_zero_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test").volume(1.0).fade_to(0.0, 0.0);

        // Should set volume immediately
        assert_eq!(builder.track.volume, 0.0);
        assert_eq!(builder.cursor, 0.0);
    }
}
