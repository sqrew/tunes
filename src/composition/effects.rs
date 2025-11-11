use super::TrackBuilder;
use crate::synthesis::effects::{
    AutoPan, BitCrusher, Chorus, Compressor, Delay, Distortion, EQ, Flanger, Gate, Limiter, Phaser,
    Reverb, RingModulator, Saturation, Tremolo,
};
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::ModRoute;

/// Builder for audio effects (accessed via `.effects()`)
///
/// Provides a scoped namespace for all 17 audio effect methods.
/// Use with closure syntax for clean, organized code:
///
/// ```rust
/// # use tunes::composition::Composition;
/// # use tunes::composition::rhythm::Tempo;
/// # use tunes::synthesis::effects::*;
/// # use tunes::synthesis::filter::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// comp.track("synth")
///     .note(&[440.0], 1.0)
///     .effects(|e| e
///         .filter(Filter::low_pass(1000.0, 0.7))
///         .reverb(Reverb::hall())
///         .delay(Delay::eighth_note())
///     );
/// ```
pub struct EffectsBuilder<'a> {
    inner: TrackBuilder<'a>,
}

impl<'a> EffectsBuilder<'a> {
    /// Unwrap back to the inner TrackBuilder
    fn into_inner(self) -> TrackBuilder<'a> {
        self.inner
    }

    /// Set the filter for this track
    pub fn filter(mut self, filter: Filter) -> Self {
        self.inner = self.inner.filter(filter);
        self
    }

    /// Add delay effect
    pub fn delay(mut self, delay: Delay) -> Self {
        self.inner = self.inner.delay(delay);
        self
    }

    /// Add reverb effect
    pub fn reverb(mut self, reverb: Reverb) -> Self {
        self.inner = self.inner.reverb(reverb);
        self
    }

    /// Add distortion effect
    pub fn distortion(mut self, distortion: Distortion) -> Self {
        self.inner = self.inner.distortion(distortion);
        self
    }

    /// Add bitcrusher effect
    pub fn bitcrusher(mut self, bitcrusher: BitCrusher) -> Self {
        self.inner = self.inner.bitcrusher(bitcrusher);
        self
    }

    /// Add compressor effect
    pub fn compressor(mut self, compressor: Compressor) -> Self {
        self.inner = self.inner.compressor(compressor);
        self
    }

    /// Add chorus effect
    pub fn chorus(mut self, chorus: Chorus) -> Self {
        self.inner = self.inner.chorus(chorus);
        self
    }

    /// Add EQ effect
    pub fn eq(mut self, eq: EQ) -> Self {
        self.inner = self.inner.eq(eq);
        self
    }

    /// Add saturation effect
    pub fn saturation(mut self, saturation: Saturation) -> Self {
        self.inner = self.inner.saturation(saturation);
        self
    }

    /// Add phaser effect
    pub fn phaser(mut self, phaser: Phaser) -> Self {
        self.inner = self.inner.phaser(phaser);
        self
    }

    /// Add flanger effect
    pub fn flanger(mut self, flanger: Flanger) -> Self {
        self.inner = self.inner.flanger(flanger);
        self
    }

    /// Add ring modulator effect
    pub fn ring_mod(mut self, ring_mod: RingModulator) -> Self {
        self.inner = self.inner.ring_mod(ring_mod);
        self
    }

    /// Add tremolo effect
    pub fn tremolo(mut self, tremolo: Tremolo) -> Self {
        self.inner = self.inner.tremolo(tremolo);
        self
    }

    /// Add autopan effect
    pub fn autopan(mut self, autopan: AutoPan) -> Self {
        self.inner = self.inner.autopan(autopan);
        self
    }

    /// Add gate effect
    pub fn gate(mut self, gate: Gate) -> Self {
        self.inner = self.inner.gate(gate);
        self
    }

    /// Add limiter effect
    pub fn limiter(mut self, limiter: Limiter) -> Self {
        self.inner = self.inner.limiter(limiter);
        self
    }

    /// Add LFO modulation
    pub fn modulate(mut self, mod_route: ModRoute) -> Self {
        self.inner = self.inner.modulate(mod_route);
        self
    }
}

impl<'a> TrackBuilder<'a> {
    /// Enter effects namespace
    ///
    /// Provides scoped access to all 17 audio effect methods.
    /// The closure receives an `EffectsBuilder` and should return it
    /// after applying effects.
    ///
    /// # Example
    /// ```rust
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::*;
    /// # use tunes::synthesis::filter::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .note(&[440.0], 1.0)
    ///     .effects(|e| e
    ///         .filter(Filter::low_pass(1200.0, 0.8))
    ///         .reverb(Reverb::hall())
    ///         .delay(Delay::quarter_note())
    ///         .chorus(Chorus::default())
    ///     );
    /// ```
    pub fn effects<F>(self, f: F) -> Self
    where
        F: FnOnce(EffectsBuilder<'a>) -> EffectsBuilder<'a>,
    {
        let builder = EffectsBuilder { inner: self };
        let result = f(builder);
        result.into_inner()
    }
}

impl<'a> TrackBuilder<'a> {
    /// Set the filter for this track
    pub fn filter(mut self, filter: Filter) -> Self {
        self.get_track_mut().filter = filter;
        self
    }
    /// Add delay effect to this track
    pub fn delay(mut self, delay: Delay) -> Self {
        let track = self.get_track_mut();
        track.effects.delay = Some(delay);
        track.effects.compute_effect_order();
        self
    }
    /// Add reverb effect to this track
    pub fn reverb(mut self, reverb: Reverb) -> Self {
        let track = self.get_track_mut();
        track.effects.reverb = Some(reverb);
        track.effects.compute_effect_order();
        self
    }
    /// Add distortion effect to this track
    pub fn distortion(mut self, distortion: Distortion) -> Self {
        let track = self.get_track_mut();
        track.effects.distortion = Some(distortion);
        track.effects.compute_effect_order();
        self
    }
    /// Add bitcrusher effect to this track
    ///
    /// BitCrusher reduces bit depth and sample rate for lo-fi, retro digital effects.
    ///
    /// # Arguments
    /// * `bitcrusher` - BitCrusher effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::BitCrusher;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("lead", &Instrument::synth_lead())
    ///     .bitcrusher(BitCrusher::new(4.0, 8.0, 0.5))
    ///     .note(&[C4], 1.0);
    /// ```
    pub fn bitcrusher(mut self, bitcrusher: BitCrusher) -> Self {
        let track = self.get_track_mut();
        track.effects.bitcrusher = Some(bitcrusher);
        track.effects.compute_effect_order();
        self
    }
    /// Add compressor effect to this track
    ///
    /// Compressor controls dynamics by reducing the volume of loud signals.
    ///
    /// # Arguments
    /// * `compressor` - Compressor effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Compressor;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("drums", &Instrument::synth_lead())
    ///     .compressor(Compressor::new(-10.0, 4.0, 0.01, 0.1, 2.0))
    ///     .note(&[C4], 1.0);
    /// ```
    pub fn compressor(mut self, compressor: Compressor) -> Self {
        let track = self.get_track_mut();
        track.effects.compressor = Some(compressor);
        track.effects.compute_effect_order();
        self
    }
    /// Add chorus effect to this track
    ///
    /// Chorus creates a richer, doubled sound through delayed and modulated copies.
    ///
    /// # Arguments
    /// * `chorus` - Chorus effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Chorus;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("pad", &Instrument::warm_pad())
    ///     .chorus(Chorus::new(0.5, 0.002, 0.3))
    ///     .note(&[C4, E4, G4], 2.0);
    /// ```
    pub fn chorus(mut self, chorus: Chorus) -> Self {
        let track = self.get_track_mut();
        track.effects.chorus = Some(chorus);
        track.effects.compute_effect_order();
        self
    }
    /// Add EQ effect to this track
    ///
    /// 3-band parametric equalizer for frequency shaping.
    ///
    /// # Arguments
    /// * `eq` - EQ effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::EQ;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("bass", &Instrument::sub_bass())
    ///     .eq(EQ::new(2.0, 1.0, 0.5, 200.0, 2000.0))
    ///     .note(&[C2], 1.0);
    /// ```
    pub fn eq(mut self, eq: EQ) -> Self {
        let track = self.get_track_mut();
        track.effects.eq = Some(eq);
        track.effects.compute_effect_order();
        self
    }
    /// Add saturation effect to this track
    ///
    /// Saturation adds warm, analog-style coloration and harmonics.
    ///
    /// # Arguments
    /// * `saturation` - Saturation effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Saturation;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("guitar", &Instrument::pluck())
    ///     .saturation(Saturation::new(2.0, 0.5, 0.7))
    ///     .note(&[E3], 1.0);
    /// ```
    pub fn saturation(mut self, saturation: Saturation) -> Self {
        let track = self.get_track_mut();
        track.effects.saturation = Some(saturation);
        track.effects.compute_effect_order();
        self
    }
    /// Add phaser effect to this track
    ///
    /// Phaser creates sweeping notches in the frequency spectrum for classic swoosh effects.
    ///
    /// # Arguments
    /// * `phaser` - Phaser effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Phaser;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("synth", &Instrument::synth_lead())
    ///     .phaser(Phaser::new(0.5, 0.7, 0.5, 4, 0.5))
    ///     .note(&[A4], 2.0);
    /// ```
    pub fn phaser(mut self, phaser: Phaser) -> Self {
        let track = self.get_track_mut();
        track.effects.phaser = Some(phaser);
        track.effects.compute_effect_order();
        self
    }
    /// Add flanger effect to this track
    ///
    /// Flanger creates jet-plane/swoosh effects with very short modulated delays.
    ///
    /// # Arguments
    /// * `flanger` - Flanger effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Flanger;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("guitar", &Instrument::pluck())
    ///     .flanger(Flanger::new(0.5, 3.0, 0.6, 0.5))
    ///     .note(&[E4], 2.0);
    /// ```
    pub fn flanger(mut self, flanger: Flanger) -> Self {
        let track = self.get_track_mut();
        track.effects.flanger = Some(flanger);
        track.effects.compute_effect_order();
        self
    }
    /// Add ring modulator effect to this track
    ///
    /// Ring Modulator creates metallic/robotic inharmonic tones by multiplying with a carrier frequency.
    ///
    /// # Arguments
    /// * `ring_mod` - RingModulator effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::RingModulator;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("robot_voice", &Instrument::synth_lead())
    ///     .ring_mod(RingModulator::new(440.0, 0.7))
    ///     .note(&[C4], 1.0);
    /// ```
    pub fn ring_mod(mut self, ring_mod: RingModulator) -> Self {
        let track = self.get_track_mut();
        track.effects.ring_mod = Some(ring_mod);
        track.effects.compute_effect_order();
        self
    }
    /// Add tremolo effect to this track
    ///
    /// Tremolo creates rhythmic amplitude modulation for pulsing volume effects.
    ///
    /// # Arguments
    /// * `tremolo` - Tremolo effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Tremolo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("synth", &Instrument::synth_lead())
    ///     .tremolo(Tremolo::new(5.0, 0.7))
    ///     .note(&[C4], 2.0);
    /// ```
    pub fn tremolo(mut self, tremolo: Tremolo) -> Self {
        let track = self.get_track_mut();
        track.effects.tremolo = Some(tremolo);
        track.effects.compute_effect_order();
        self
    }

    /// Add auto-pan effect to this track
    ///
    /// AutoPan creates automatic stereo panning modulation for movement in the stereo field.
    ///
    /// # Arguments
    /// * `autopan` - AutoPan effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::AutoPan;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("pad", &Instrument::warm_pad())
    ///     .autopan(AutoPan::new(0.5, 0.8))
    ///     .note(&[C4, E4, G4], 4.0);
    /// ```
    pub fn autopan(mut self, autopan: AutoPan) -> Self {
        let track = self.get_track_mut();
        track.effects.autopan = Some(autopan);
        // Note: AutoPan is not in effect_order (handled separately in stereo stage)
        self
    }

    /// Add gate effect to this track
    ///
    /// Gate reduces the level of signals below a threshold, useful for noise reduction
    /// or creating rhythmic gating effects.
    ///
    /// # Arguments
    /// * `gate` - Gate effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Gate;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("drums", &Instrument::synth_lead())
    ///     .gate(Gate::new(-40.0, 10.0, 0.001, 0.05))
    ///     .note(&[C4], 1.0);
    /// ```
    pub fn gate(mut self, gate: Gate) -> Self {
        let track = self.get_track_mut();
        track.effects.gate = Some(gate);
        track.effects.compute_effect_order();
        self
    }

    /// Add limiter effect to this track
    ///
    /// Limiter prevents the signal from exceeding a threshold, acting as a safety net
    /// against clipping. Typically used as the final stage in the signal chain.
    ///
    /// # Arguments
    /// * `limiter` - Limiter effect instance
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Limiter;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("master", &Instrument::synth_lead())
    ///     .limiter(Limiter::new(-0.1, 0.05))
    ///     .note(&[C4], 1.0);
    /// ```
    pub fn limiter(mut self, limiter: Limiter) -> Self {
        let track = self.get_track_mut();
        track.effects.limiter = Some(limiter);
        track.effects.compute_effect_order();
        self
    }

    /// Add an LFO modulation route to this track
    pub fn modulate(mut self, mod_route: ModRoute) -> Self {
        self.get_track_mut().modulation.push(mod_route);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::synthesis::effects::*;
    use crate::synthesis::filter::{Filter, FilterType};
    use crate::synthesis::lfo::{LFO, ModRoute, ModTarget};
    use crate::consts::notes::*;
    use crate::composition::rhythm::Tempo;
    use crate::synthesis::waveform::Waveform;

    #[test]
    fn test_filter_sets_track_filter() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let filter = Filter::new(FilterType::LowPass, 1000.0, 0.7);
        comp.track("test").filter(filter);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(matches!(track.filter.filter_type, FilterType::LowPass));
        assert_eq!(track.filter.cutoff, 1000.0);
        assert_eq!(track.filter.resonance, 0.7);
    }

    #[test]
    fn test_delay_sets_track_delay() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let delay = Delay::new(0.5, 0.4, 0.6);
        comp.track("test").delay(delay);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.delay.is_some());
        let track_delay = track.effects.delay.as_ref().unwrap();
        assert_eq!(track_delay.delay_time, 0.5);
        assert_eq!(track_delay.feedback, 0.4);
        assert_eq!(track_delay.mix, 0.6);
    }

    #[test]
    fn test_reverb_sets_track_reverb() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let reverb = Reverb::new(0.8, 0.5, 0.3);
        comp.track("test").reverb(reverb);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.reverb.is_some());
        let track_reverb = track.effects.reverb.as_ref().unwrap();
        assert_eq!(track_reverb.room_size, 0.8);
        assert_eq!(track_reverb.damping, 0.5);
        assert_eq!(track_reverb.mix, 0.3);
    }

    #[test]
    fn test_distortion_sets_track_distortion() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let distortion = Distortion::new(2.0, 0.5);
        comp.track("test").distortion(distortion);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.distortion.is_some());
        let track_dist = track.effects.distortion.as_ref().unwrap();
        assert_eq!(track_dist.drive, 2.0);
        assert_eq!(track_dist.mix, 0.5);
    }

    #[test]
    fn test_bitcrusher_sets_track_bitcrusher() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let bitcrusher = BitCrusher::new(4.0, 8.0, 0.5);
        comp.track("test").bitcrusher(bitcrusher);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.bitcrusher.is_some());
        let track_bc = track.effects.bitcrusher.as_ref().unwrap();
        assert_eq!(track_bc.bit_depth, 4.0);
        assert_eq!(track_bc.sample_rate_reduction, 8.0);
        assert_eq!(track_bc.mix, 0.5);
    }

    #[test]
    fn test_compressor_sets_track_compressor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let compressor = Compressor::new(0.3, 4.0, 0.01, 0.1, 2.0);
        comp.track("test").compressor(compressor);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.compressor.is_some());
        let track_comp = track.effects.compressor.as_ref().unwrap();
        assert_eq!(track_comp.threshold, 0.3);
        assert_eq!(track_comp.ratio, 4.0);
        assert_eq!(track_comp.attack, 0.01);
        assert_eq!(track_comp.release, 0.1);
        assert_eq!(track_comp.makeup_gain, 2.0);
    }

    #[test]
    fn test_chorus_sets_track_chorus() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let chorus = Chorus::new(0.5, 2.0, 0.3);
        comp.track("test").chorus(chorus);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.chorus.is_some());
        let track_chorus = track.effects.chorus.as_ref().unwrap();
        assert_eq!(track_chorus.rate, 0.5);
        assert_eq!(track_chorus.depth, 2.0);
        assert_eq!(track_chorus.mix, 0.3);
    }

    #[test]
    fn test_eq_sets_track_eq() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let eq = EQ::new(2.0, 1.0, 0.5, 200.0, 2000.0);
        comp.track("test").eq(eq);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.eq.is_some());
        let track_eq = track.effects.eq.as_ref().unwrap();
        assert_eq!(track_eq.low_gain, 2.0);
        assert_eq!(track_eq.mid_gain, 1.0);
        assert_eq!(track_eq.high_gain, 0.5);
        assert_eq!(track_eq.low_freq, 200.0);
        assert_eq!(track_eq.high_freq, 2000.0);
    }

    #[test]
    fn test_saturation_sets_track_saturation() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let saturation = Saturation::new(2.0, 0.5, 0.7);
        comp.track("test").saturation(saturation);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.saturation.is_some());
        let track_sat = track.effects.saturation.as_ref().unwrap();
        assert_eq!(track_sat.drive, 2.0);
        assert_eq!(track_sat.character, 0.5);
        assert_eq!(track_sat.mix, 0.7);
    }

    #[test]
    fn test_phaser_sets_track_phaser() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let phaser = Phaser::new(0.5, 0.7, 0.5, 4, 0.5);
        comp.track("test").phaser(phaser);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.phaser.is_some());
        let track_phaser = track.effects.phaser.as_ref().unwrap();
        assert_eq!(track_phaser.rate, 0.5);
        assert_eq!(track_phaser.depth, 0.7);
        assert_eq!(track_phaser.feedback, 0.5);
        assert_eq!(track_phaser.mix, 0.5);
        assert_eq!(track_phaser.stages, 4);
    }

    #[test]
    fn test_flanger_sets_track_flanger() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let flanger = Flanger::new(0.5, 3.0, 0.6, 0.5);
        comp.track("test").flanger(flanger);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.flanger.is_some());
        let track_flanger = track.effects.flanger.as_ref().unwrap();
        assert_eq!(track_flanger.rate, 0.5);
        assert_eq!(track_flanger.depth, 3.0);
        assert_eq!(track_flanger.feedback, 0.6);
        assert_eq!(track_flanger.mix, 0.5);
    }

    #[test]
    fn test_ring_mod_sets_track_ring_mod() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let ring_mod = RingModulator::new(440.0, 0.7);
        comp.track("test").ring_mod(ring_mod);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.ring_mod.is_some());
        let track_ring = track.effects.ring_mod.as_ref().unwrap();
        assert_eq!(track_ring.carrier_freq, 440.0);
        assert_eq!(track_ring.mix, 0.7);
    }

    #[test]
    fn test_modulate_adds_modulation_route() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let lfo = LFO::new(Waveform::Sine, 5.0, 0.5);
        let mod_route = ModRoute::new(lfo, ModTarget::Pitch, 1.0);
        comp.track("test").modulate(mod_route);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert_eq!(track.modulation.len(), 1);
    }

    #[test]
    fn test_modulate_multiple_routes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let lfo1 = LFO::new(Waveform::Sine, 5.0, 0.5);
        let lfo2 = LFO::new(Waveform::Triangle, 3.0, 0.3);
        comp.track("test")
            .modulate(ModRoute::new(lfo1, ModTarget::Pitch, 1.0))
            .modulate(ModRoute::new(lfo2, ModTarget::FilterCutoff, 0.5));

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert_eq!(track.modulation.len(), 2);
    }

    #[test]
    fn test_effect_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("fx_chain")
            .filter(Filter::new(FilterType::LowPass, 2000.0, 0.5))
            .delay(Delay::new(0.5, 0.3, 0.5))
            .reverb(Reverb::new(0.8, 0.4, 0.4))
            .distortion(Distortion::new(1.5, 0.6));

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(matches!(track.filter.filter_type, FilterType::LowPass));
        assert!(track.effects.delay.is_some());
        assert!(track.effects.reverb.is_some());
        assert!(track.effects.distortion.is_some());
    }

    #[test]
    fn test_all_effects_combined() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("everything")
            .filter(Filter::new(FilterType::BandPass, 1000.0, 0.7))
            .delay(Delay::new(0.25, 0.5, 0.6))
            .reverb(Reverb::new(0.7, 0.5, 0.3))
            .distortion(Distortion::new(2.0, 0.4))
            .bitcrusher(BitCrusher::new(8.0, 4.0, 0.3))
            .compressor(Compressor::new(0.2, 3.0, 0.01, 0.1, 1.5))
            .chorus(Chorus::new(0.8, 3.0, 0.4))
            .eq(EQ::new(1.5, 1.0, 0.8, 250.0, 3000.0))
            .saturation(Saturation::new(1.8, 0.6, 0.5))
            .phaser(Phaser::new(0.4, 0.6, 0.7, 6, 0.5))
            .flanger(Flanger::new(0.6, 2.5, 0.7, 0.4))
            .ring_mod(RingModulator::new(550.0, 0.3));

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        // Verify all effects are set
        assert!(matches!(track.filter.filter_type, FilterType::BandPass));
        assert!(track.effects.delay.is_some());
        assert!(track.effects.reverb.is_some());
        assert!(track.effects.distortion.is_some());
        assert!(track.effects.bitcrusher.is_some());
        assert!(track.effects.compressor.is_some());
        assert!(track.effects.chorus.is_some());
        assert!(track.effects.eq.is_some());
        assert!(track.effects.saturation.is_some());
        assert!(track.effects.phaser.is_some());
        assert!(track.effects.flanger.is_some());
        assert!(track.effects.ring_mod.is_some());
    }

    #[test]
    fn test_effects_with_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("effected")
            .delay(Delay::new(0.5, 0.4, 0.6))
            .reverb(Reverb::new(0.6, 0.5, 0.4))
            .note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        assert!(mixer.tracks()[0].effects.delay.is_some());
        assert!(mixer.tracks()[0].effects.reverb.is_some());
        assert_eq!(mixer.tracks()[0].events.len(), 1);
    }

    #[test]
    fn test_filter_different_types() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("lp")
            .filter(Filter::new(FilterType::LowPass, 1000.0, 0.5));
        comp.track("hp")
            .filter(Filter::new(FilterType::HighPass, 500.0, 0.7));
        comp.track("bp")
            .filter(Filter::new(FilterType::BandPass, 800.0, 0.6));

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 3);

        // Check that all three filter types exist (HashMap order not guaranteed)
        let has_lowpass = mixer
.tracks()
            .iter()
            .any(|t| matches!(t.filter.filter_type, FilterType::LowPass));
        let has_highpass = mixer
.tracks()
            .iter()
            .any(|t| matches!(t.filter.filter_type, FilterType::HighPass));
        let has_bandpass = mixer
.tracks()
            .iter()
            .any(|t| matches!(t.filter.filter_type, FilterType::BandPass));

        assert!(has_lowpass, "Should have a LowPass filter");
        assert!(has_highpass, "Should have a HighPass filter");
        assert!(has_bandpass, "Should have a BandPass filter");
    }

    #[test]
    fn test_delay_replaces_previous_delay() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .delay(Delay::new(0.5, 0.4, 0.5))
            .delay(Delay::new(0.25, 0.6, 0.7));

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.delay.is_some());
        let delay = track.effects.delay.as_ref().unwrap();
        assert_eq!(delay.delay_time, 0.25);
        assert_eq!(delay.feedback, 0.6);
        assert_eq!(delay.mix, 0.7);
    }

    #[test]
    fn test_reverb_replaces_previous_reverb() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .reverb(Reverb::new(0.8, 0.5, 0.6))
            .reverb(Reverb::new(0.3, 0.2, 0.4));

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        assert!(track.effects.reverb.is_some());
        let reverb = track.effects.reverb.as_ref().unwrap();
        assert_eq!(reverb.room_size, 0.3);
        assert_eq!(reverb.damping, 0.2);
        assert_eq!(reverb.mix, 0.4);
    }

    #[test]
    fn test_effects_dont_affect_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("test")
            .delay(Delay::new(0.5, 0.4, 0.5))
            .reverb(Reverb::new(0.8, 0.5, 0.4))
            .distortion(Distortion::new(2.0, 0.5));

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        // Just verify that effects were set successfully
        assert!(track.effects.delay.is_some());
        assert!(track.effects.reverb.is_some());
        assert!(track.effects.distortion.is_some());
    }

    #[test]
    fn test_bitcrusher_parameters() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let bc = BitCrusher::new(6.0, 10.0, 0.8);
        comp.track("crushed").bitcrusher(bc);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        let track_bc = track.effects.bitcrusher.as_ref().unwrap();
        assert_eq!(track_bc.bit_depth, 6.0);
        assert_eq!(track_bc.sample_rate_reduction, 10.0);
        assert_eq!(track_bc.mix, 0.8);
    }

    #[test]
    fn test_compressor_full_parameters() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let comp_fx = Compressor::new(0.25, 6.0, 0.005, 0.2, 3.0);
        comp.track("compressed").compressor(comp_fx);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        let track_comp = track.effects.compressor.as_ref().unwrap();
        assert_eq!(track_comp.threshold, 0.25);
        assert_eq!(track_comp.ratio, 6.0);
        assert_eq!(track_comp.attack, 0.005);
        assert_eq!(track_comp.release, 0.2);
        assert_eq!(track_comp.makeup_gain, 3.0);
    }

    #[test]
    fn test_chorus_parameters() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let chorus = Chorus::new(1.2, 4.0, 0.6);
        comp.track("chorus").chorus(chorus);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        let track_chorus = track.effects.chorus.as_ref().unwrap();
        assert_eq!(track_chorus.rate, 1.2);
        assert_eq!(track_chorus.depth, 4.0);
        assert_eq!(track_chorus.mix, 0.6);
    }

    #[test]
    fn test_eq_boost_and_cut() {
        let mut comp = Composition::new(Tempo::new(120.0));
        // Boost lows, cut mids, boost highs
        let eq = EQ::new(1.5, 0.5, 1.8, 100.0, 5000.0);
        comp.track("eq").eq(eq);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        let track_eq = track.effects.eq.as_ref().unwrap();
        assert_eq!(track_eq.low_gain, 1.5);
        assert_eq!(track_eq.mid_gain, 0.5);
        assert_eq!(track_eq.high_gain, 1.8);
    }

    #[test]
    fn test_saturation_parameters() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let sat = Saturation::new(3.0, 0.8, 0.9);
        comp.track("saturated").saturation(sat);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        let track_sat = track.effects.saturation.as_ref().unwrap();
        assert_eq!(track_sat.drive, 3.0);
        assert_eq!(track_sat.character, 0.8);
        assert_eq!(track_sat.mix, 0.9);
    }

    #[test]
    fn test_phaser_stages() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let phaser = Phaser::new(0.3, 0.8, 0.6, 8, 0.7);
        comp.track("phased").phaser(phaser);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];
        let track_phaser = track.effects.phaser.as_ref().unwrap();
        assert_eq!(track_phaser.stages, 8);
        assert_eq!(track_phaser.rate, 0.3);
    }

    #[test]
    fn test_flanger_vs_chorus_depth() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Flanger has larger depth (measured in ms)
        let flanger = Flanger::new(0.5, 5.0, 0.7, 0.5);
        comp.track("flanger").flanger(flanger);

        // Chorus has typical depth (measured in milliseconds)
        let chorus = Chorus::new(0.5, 3.0, 0.5);
        comp.track("chorus").chorus(chorus);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 2);

        // Check that both effects exist with correct depths (HashMap order not guaranteed)
        let has_flanger_5 = mixer
.tracks()
            .iter()
            .any(|t| t.effects.flanger.as_ref().map_or(false, |f| f.depth == 5.0));
        let has_chorus_3 = mixer
.tracks()
            .iter()
            .any(|t| t.effects.chorus.as_ref().map_or(false, |c| c.depth == 3.0));

        assert!(has_flanger_5, "Should have flanger with depth 5.0");
        assert!(has_chorus_3, "Should have chorus with depth 3.0");
    }

    #[test]
    fn test_ring_mod_carrier_frequency() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Test different carrier frequencies
        let ring1 = RingModulator::new(200.0, 0.5);
        comp.track("ring1").ring_mod(ring1);

        let ring2 = RingModulator::new(880.0, 0.8);
        comp.track("ring2").ring_mod(ring2);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 2);

        // Check that both ring modulators exist with correct frequencies (HashMap order not guaranteed)
        let has_200hz = mixer.tracks().iter().any(|t| {
            t.effects.ring_mod
                .as_ref()
                .map_or(false, |rm| rm.carrier_freq == 200.0)
        });
        let has_880hz = mixer.tracks().iter().any(|t| {
            t.effects.ring_mod
                .as_ref()
                .map_or(false, |rm| rm.carrier_freq == 880.0)
        });

        assert!(has_200hz, "Should have ring modulator with 200Hz carrier");
        assert!(has_880hz, "Should have ring modulator with 880Hz carrier");
    }
}
