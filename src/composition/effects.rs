use super::TrackBuilder;
use crate::effects::{BitCrusher, Chorus, Compressor, Delay, Distortion, EQ, Flanger, Phaser, Reverb, RingModulator, Saturation};
use crate::filter::Filter;
use crate::lfo::ModRoute;

impl<'a> TrackBuilder<'a> {
    /// Set the filter for this track
    pub fn filter(self, filter: Filter) -> Self {
        self.track.filter = filter;
        self
    }
    /// Add delay effect to this track
    pub fn delay(self, delay: Delay) -> Self {
        self.track.delay = Some(delay);
        self
    }
    /// Add reverb effect to this track
    pub fn reverb(self, reverb: Reverb) -> Self {
        self.track.reverb = Some(reverb);
        self
    }
    /// Add distortion effect to this track
    pub fn distortion(self, distortion: Distortion) -> Self {
        self.track.distortion = Some(distortion);
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
    /// # use musicrs::composition::Composition;
    /// # use musicrs::instruments::Instrument;
    /// # use musicrs::rhythm::Tempo;
    /// # use musicrs::effects::BitCrusher;
    /// # use musicrs::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("lead", &Instrument::synth_lead())
    ///     .bitcrusher(BitCrusher::new(4.0, 8.0, 0.5))
    ///     .note(&[C4], 1.0);
    /// ```
    pub fn bitcrusher(self, bitcrusher: BitCrusher) -> Self {
        self.track.bitcrusher = Some(bitcrusher);
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
    /// # use musicrs::composition::Composition;
    /// # use musicrs::instruments::Instrument;
    /// # use musicrs::rhythm::Tempo;
    /// # use musicrs::effects::Compressor;
    /// # use musicrs::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("drums", &Instrument::synth_lead())
    ///     .compressor(Compressor::new(-10.0, 4.0, 0.01, 0.1, 2.0))
    ///     .note(&[C4], 1.0);
    /// ```
    pub fn compressor(self, compressor: Compressor) -> Self {
        self.track.compressor = Some(compressor);
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
    /// # use musicrs::composition::Composition;
    /// # use musicrs::instruments::Instrument;
    /// # use musicrs::rhythm::Tempo;
    /// # use musicrs::effects::Chorus;
    /// # use musicrs::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("pad", &Instrument::warm_pad())
    ///     .chorus(Chorus::new(0.5, 0.002, 0.3))
    ///     .note(&[C4, E4, G4], 2.0);
    /// ```
    pub fn chorus(self, chorus: Chorus) -> Self {
        self.track.chorus = Some(chorus);
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
    /// # use musicrs::composition::Composition;
    /// # use musicrs::instruments::Instrument;
    /// # use musicrs::rhythm::Tempo;
    /// # use musicrs::effects::EQ;
    /// # use musicrs::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("bass", &Instrument::sub_bass())
    ///     .eq(EQ::new(2.0, 1.0, 0.5, 200.0, 2000.0))
    ///     .note(&[C2], 1.0);
    /// ```
    pub fn eq(self, eq: EQ) -> Self {
        self.track.eq = Some(eq);
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
    /// # use musicrs::composition::Composition;
    /// # use musicrs::instruments::Instrument;
    /// # use musicrs::rhythm::Tempo;
    /// # use musicrs::effects::Saturation;
    /// # use musicrs::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("guitar", &Instrument::pluck())
    ///     .saturation(Saturation::new(2.0, 0.5, 0.7))
    ///     .note(&[E3], 1.0);
    /// ```
    pub fn saturation(self, saturation: Saturation) -> Self {
        self.track.saturation = Some(saturation);
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
    /// # use musicrs::composition::Composition;
    /// # use musicrs::instruments::Instrument;
    /// # use musicrs::rhythm::Tempo;
    /// # use musicrs::effects::Phaser;
    /// # use musicrs::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("synth", &Instrument::synth_lead())
    ///     .phaser(Phaser::new(0.5, 0.7, 0.5, 0.5, 4))
    ///     .note(&[A4], 2.0);
    /// ```
    pub fn phaser(self, phaser: Phaser) -> Self {
        self.track.phaser = Some(phaser);
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
    /// # use musicrs::composition::Composition;
    /// # use musicrs::instruments::Instrument;
    /// # use musicrs::rhythm::Tempo;
    /// # use musicrs::effects::Flanger;
    /// # use musicrs::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("guitar", &Instrument::pluck())
    ///     .flanger(Flanger::new(0.5, 3.0, 0.6, 0.5))
    ///     .note(&[E4], 2.0);
    /// ```
    pub fn flanger(self, flanger: Flanger) -> Self {
        self.track.flanger = Some(flanger);
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
    /// # use musicrs::composition::Composition;
    /// # use musicrs::instruments::Instrument;
    /// # use musicrs::rhythm::Tempo;
    /// # use musicrs::effects::RingModulator;
    /// # use musicrs::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.instrument("robot_voice", &Instrument::synth_lead())
    ///     .ring_mod(RingModulator::new(440.0, 0.7))
    ///     .note(&[C4], 1.0);
    /// ```
    pub fn ring_mod(self, ring_mod: RingModulator) -> Self {
        self.track.ring_mod = Some(ring_mod);
        self
    }
    /// Add an LFO modulation route to this track
    pub fn modulate(self, mod_route: ModRoute) -> Self {
        self.track.modulation.push(mod_route);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::effects::*;
    use crate::filter::{Filter, FilterType};
    use crate::lfo::{ModRoute, ModTarget, LFO};
    use crate::notes::*;
    use crate::rhythm::Tempo;
    use crate::waveform::Waveform;

    #[test]
    fn test_filter_sets_track_filter() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let filter = Filter::new(FilterType::LowPass, 1000.0, 0.7);
        let builder = comp.track("test").filter(filter);

        assert!(matches!(builder.track.filter.filter_type, FilterType::LowPass));
        assert_eq!(builder.track.filter.cutoff, 1000.0);
        assert_eq!(builder.track.filter.resonance, 0.7);
    }

    #[test]
    fn test_delay_sets_track_delay() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let delay = Delay::new(0.5, 0.4, 0.6);
        let builder = comp.track("test").delay(delay);

        assert!(builder.track.delay.is_some());
        let track_delay = builder.track.delay.as_ref().unwrap();
        assert_eq!(track_delay.delay_time, 0.5);
        assert_eq!(track_delay.feedback, 0.4);
        assert_eq!(track_delay.mix, 0.6);
    }

    #[test]
    fn test_reverb_sets_track_reverb() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let reverb = Reverb::new(0.8, 0.5, 0.3);
        let builder = comp.track("test").reverb(reverb);

        assert!(builder.track.reverb.is_some());
        let track_reverb = builder.track.reverb.as_ref().unwrap();
        assert_eq!(track_reverb.room_size, 0.8);
        assert_eq!(track_reverb.damping, 0.5);
        assert_eq!(track_reverb.mix, 0.3);
    }

    #[test]
    fn test_distortion_sets_track_distortion() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let distortion = Distortion::new(2.0, 0.5);
        let builder = comp.track("test").distortion(distortion);

        assert!(builder.track.distortion.is_some());
        let track_dist = builder.track.distortion.as_ref().unwrap();
        assert_eq!(track_dist.drive, 2.0);
        assert_eq!(track_dist.mix, 0.5);
    }

    #[test]
    fn test_bitcrusher_sets_track_bitcrusher() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let bitcrusher = BitCrusher::new(4.0, 8.0, 0.5);
        let builder = comp.track("test").bitcrusher(bitcrusher);

        assert!(builder.track.bitcrusher.is_some());
        let track_bc = builder.track.bitcrusher.as_ref().unwrap();
        assert_eq!(track_bc.bit_depth, 4.0);
        assert_eq!(track_bc.sample_rate_reduction, 8.0);
        assert_eq!(track_bc.mix, 0.5);
    }

    #[test]
    fn test_compressor_sets_track_compressor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let compressor = Compressor::new(0.3, 4.0, 0.01, 0.1, 2.0);
        let builder = comp.track("test").compressor(compressor);

        assert!(builder.track.compressor.is_some());
        let track_comp = builder.track.compressor.as_ref().unwrap();
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
        let builder = comp.track("test").chorus(chorus);

        assert!(builder.track.chorus.is_some());
        let track_chorus = builder.track.chorus.as_ref().unwrap();
        assert_eq!(track_chorus.rate, 0.5);
        assert_eq!(track_chorus.depth, 2.0);
        assert_eq!(track_chorus.mix, 0.3);
    }

    #[test]
    fn test_eq_sets_track_eq() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let eq = EQ::new(2.0, 1.0, 0.5, 200.0, 2000.0);
        let builder = comp.track("test").eq(eq);

        assert!(builder.track.eq.is_some());
        let track_eq = builder.track.eq.as_ref().unwrap();
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
        let builder = comp.track("test").saturation(saturation);

        assert!(builder.track.saturation.is_some());
        let track_sat = builder.track.saturation.as_ref().unwrap();
        assert_eq!(track_sat.drive, 2.0);
        assert_eq!(track_sat.character, 0.5);
        assert_eq!(track_sat.mix, 0.7);
    }

    #[test]
    fn test_phaser_sets_track_phaser() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let phaser = Phaser::new(0.5, 0.7, 0.5, 0.5, 4);
        let builder = comp.track("test").phaser(phaser);

        assert!(builder.track.phaser.is_some());
        let track_phaser = builder.track.phaser.as_ref().unwrap();
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
        let builder = comp.track("test").flanger(flanger);

        assert!(builder.track.flanger.is_some());
        let track_flanger = builder.track.flanger.as_ref().unwrap();
        assert_eq!(track_flanger.rate, 0.5);
        assert_eq!(track_flanger.depth, 3.0);
        assert_eq!(track_flanger.feedback, 0.6);
        assert_eq!(track_flanger.mix, 0.5);
    }

    #[test]
    fn test_ring_mod_sets_track_ring_mod() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let ring_mod = RingModulator::new(440.0, 0.7);
        let builder = comp.track("test").ring_mod(ring_mod);

        assert!(builder.track.ring_mod.is_some());
        let track_ring = builder.track.ring_mod.as_ref().unwrap();
        assert_eq!(track_ring.carrier_freq, 440.0);
        assert_eq!(track_ring.mix, 0.7);
    }

    #[test]
    fn test_modulate_adds_modulation_route() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let lfo = LFO::new(Waveform::Sine, 5.0, 0.5);
        let mod_route = ModRoute::new(lfo, ModTarget::Pitch, 1.0);
        let builder = comp.track("test").modulate(mod_route);

        assert_eq!(builder.track.modulation.len(), 1);
    }

    #[test]
    fn test_modulate_multiple_routes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let lfo1 = LFO::new(Waveform::Sine, 5.0, 0.5);
        let lfo2 = LFO::new(Waveform::Triangle, 3.0, 0.3);
        let builder = comp.track("test")
            .modulate(ModRoute::new(lfo1, ModTarget::Pitch, 1.0))
            .modulate(ModRoute::new(lfo2, ModTarget::FilterCutoff, 0.5));

        assert_eq!(builder.track.modulation.len(), 2);
    }

    #[test]
    fn test_effect_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("fx_chain")
            .filter(Filter::new(FilterType::LowPass, 2000.0, 0.5))
            .delay(Delay::new(0.5, 0.3, 0.5))
            .reverb(Reverb::new(0.8, 0.4, 0.4))
            .distortion(Distortion::new(1.5, 0.6));

        assert!(matches!(builder.track.filter.filter_type, FilterType::LowPass));
        assert!(builder.track.delay.is_some());
        assert!(builder.track.reverb.is_some());
        assert!(builder.track.distortion.is_some());
    }

    #[test]
    fn test_all_effects_combined() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("everything")
            .filter(Filter::new(FilterType::BandPass, 1000.0, 0.7))
            .delay(Delay::new(0.25, 0.5, 0.6))
            .reverb(Reverb::new(0.7, 0.5, 0.3))
            .distortion(Distortion::new(2.0, 0.4))
            .bitcrusher(BitCrusher::new(8.0, 4.0, 0.3))
            .compressor(Compressor::new(0.2, 3.0, 0.01, 0.1, 1.5))
            .chorus(Chorus::new(0.8, 3.0, 0.4))
            .eq(EQ::new(1.5, 1.0, 0.8, 250.0, 3000.0))
            .saturation(Saturation::new(1.8, 0.6, 0.5))
            .phaser(Phaser::new(0.4, 0.6, 0.7, 0.5, 6))
            .flanger(Flanger::new(0.6, 2.5, 0.7, 0.4))
            .ring_mod(RingModulator::new(550.0, 0.3));

        // Verify all effects are set
        assert!(matches!(builder.track.filter.filter_type, FilterType::BandPass));
        assert!(builder.track.delay.is_some());
        assert!(builder.track.reverb.is_some());
        assert!(builder.track.distortion.is_some());
        assert!(builder.track.bitcrusher.is_some());
        assert!(builder.track.compressor.is_some());
        assert!(builder.track.chorus.is_some());
        assert!(builder.track.eq.is_some());
        assert!(builder.track.saturation.is_some());
        assert!(builder.track.phaser.is_some());
        assert!(builder.track.flanger.is_some());
        assert!(builder.track.ring_mod.is_some());
    }

    #[test]
    fn test_effects_with_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("effected")
            .delay(Delay::new(0.5, 0.4, 0.6))
            .reverb(Reverb::new(0.6, 0.5, 0.4))
            .note(&[C4], 1.0);

        let mixer = comp.into_mixer();
        assert!(mixer.tracks[0].delay.is_some());
        assert!(mixer.tracks[0].reverb.is_some());
        assert_eq!(mixer.tracks[0].events.len(), 1);
    }

    #[test]
    fn test_filter_different_types() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let lowpass = comp.track("lp").filter(Filter::new(FilterType::LowPass, 1000.0, 0.5));
        assert!(matches!(lowpass.track.filter.filter_type, FilterType::LowPass));

        let highpass = comp.track("hp").filter(Filter::new(FilterType::HighPass, 500.0, 0.7));
        assert!(matches!(highpass.track.filter.filter_type, FilterType::HighPass));

        let bandpass = comp.track("bp").filter(Filter::new(FilterType::BandPass, 800.0, 0.6));
        assert!(matches!(bandpass.track.filter.filter_type, FilterType::BandPass));
    }

    #[test]
    fn test_delay_replaces_previous_delay() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .delay(Delay::new(0.5, 0.4, 0.5))
            .delay(Delay::new(0.25, 0.6, 0.7));

        assert!(builder.track.delay.is_some());
        let delay = builder.track.delay.as_ref().unwrap();
        assert_eq!(delay.delay_time, 0.25);
        assert_eq!(delay.feedback, 0.6);
        assert_eq!(delay.mix, 0.7);
    }

    #[test]
    fn test_reverb_replaces_previous_reverb() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .reverb(Reverb::new(0.8, 0.5, 0.6))
            .reverb(Reverb::new(0.3, 0.2, 0.4));

        assert!(builder.track.reverb.is_some());
        let reverb = builder.track.reverb.as_ref().unwrap();
        assert_eq!(reverb.room_size, 0.3);
        assert_eq!(reverb.damping, 0.2);
        assert_eq!(reverb.mix, 0.4);
    }

    #[test]
    fn test_effects_dont_affect_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let builder = comp.track("test")
            .delay(Delay::new(0.5, 0.4, 0.5))
            .reverb(Reverb::new(0.8, 0.5, 0.4))
            .distortion(Distortion::new(2.0, 0.5));

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_bitcrusher_parameters() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let bc = BitCrusher::new(6.0, 10.0, 0.8);
        let builder = comp.track("crushed").bitcrusher(bc);

        let track_bc = builder.track.bitcrusher.as_ref().unwrap();
        assert_eq!(track_bc.bit_depth, 6.0);
        assert_eq!(track_bc.sample_rate_reduction, 10.0);
        assert_eq!(track_bc.mix, 0.8);
    }

    #[test]
    fn test_compressor_full_parameters() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let comp_fx = Compressor::new(0.25, 6.0, 0.005, 0.2, 3.0);
        let builder = comp.track("compressed").compressor(comp_fx);

        let track_comp = builder.track.compressor.as_ref().unwrap();
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
        let builder = comp.track("chorus").chorus(chorus);

        let track_chorus = builder.track.chorus.as_ref().unwrap();
        assert_eq!(track_chorus.rate, 1.2);
        assert_eq!(track_chorus.depth, 4.0);
        assert_eq!(track_chorus.mix, 0.6);
    }

    #[test]
    fn test_eq_boost_and_cut() {
        let mut comp = Composition::new(Tempo::new(120.0));
        // Boost lows, cut mids, boost highs
        let eq = EQ::new(1.5, 0.5, 1.8, 100.0, 5000.0);
        let builder = comp.track("eq").eq(eq);

        let track_eq = builder.track.eq.as_ref().unwrap();
        assert_eq!(track_eq.low_gain, 1.5);
        assert_eq!(track_eq.mid_gain, 0.5);
        assert_eq!(track_eq.high_gain, 1.8);
    }

    #[test]
    fn test_saturation_parameters() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let sat = Saturation::new(3.0, 0.8, 0.9);
        let builder = comp.track("saturated").saturation(sat);

        let track_sat = builder.track.saturation.as_ref().unwrap();
        assert_eq!(track_sat.drive, 3.0);
        assert_eq!(track_sat.character, 0.8);
        assert_eq!(track_sat.mix, 0.9);
    }

    #[test]
    fn test_phaser_stages() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let phaser = Phaser::new(0.3, 0.8, 0.6, 0.7, 8);
        let builder = comp.track("phased").phaser(phaser);

        let track_phaser = builder.track.phaser.as_ref().unwrap();
        assert_eq!(track_phaser.stages, 8);
        assert_eq!(track_phaser.rate, 0.3);
    }

    #[test]
    fn test_flanger_vs_chorus_depth() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Flanger has larger depth (measured in ms)
        let flanger = Flanger::new(0.5, 5.0, 0.7, 0.5);
        let builder1 = comp.track("flanger").flanger(flanger);
        assert_eq!(builder1.track.flanger.as_ref().unwrap().depth, 5.0);

        // Chorus has typical depth (measured in milliseconds)
        let chorus = Chorus::new(0.5, 3.0, 0.5);
        let builder2 = comp.track("chorus").chorus(chorus);
        assert_eq!(builder2.track.chorus.as_ref().unwrap().depth, 3.0);
    }

    #[test]
    fn test_ring_mod_carrier_frequency() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Test different carrier frequencies
        let ring1 = RingModulator::new(200.0, 0.5);
        let builder1 = comp.track("ring1").ring_mod(ring1);
        assert_eq!(builder1.track.ring_mod.as_ref().unwrap().carrier_freq, 200.0);

        let ring2 = RingModulator::new(880.0, 0.8);
        let builder2 = comp.track("ring2").ring_mod(ring2);
        assert_eq!(builder2.track.ring_mod.as_ref().unwrap().carrier_freq, 880.0);
    }
}
