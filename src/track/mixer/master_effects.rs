//! Master effects methods for the Mixer.
//!
//! Contains all master_* methods for adding effects to the master chain.

use super::Mixer;

impl Mixer {
    /// Add a compressor to the master output
    ///
    /// Applies dynamic range compression to the final stereo mix. Master compression
    /// uses stereo-linked processing to preserve the stereo image.
    ///
    /// # Arguments
    /// * `compressor` - Compressor effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::Compressor;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_compressor(Compressor::new(-10.0, 4.0, 0.01, 0.1, 2.0));
    /// ```
    pub fn master_compressor(&mut self, compressor: crate::synthesis::effects::Compressor) {
        self.master.compressor = Some(compressor);
        self.master.compute_effect_order();
    }

    /// Add a limiter to the master output
    ///
    /// Applies limiting to prevent clipping on the final stereo mix. Master limiting
    /// uses stereo-linked processing to preserve the stereo image. This is typically
    /// the last effect in the master chain.
    ///
    /// # Arguments
    /// * `limiter` - Limiter effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::Limiter;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_limiter(Limiter::new(0.0, 0.01));
    /// ```
    pub fn master_limiter(&mut self, limiter: crate::synthesis::effects::Limiter) {
        self.master.limiter = Some(limiter);
        self.master.compute_effect_order();
    }

    /// Add EQ to the master output
    ///
    /// Applies 3-band equalization to the final stereo mix.
    ///
    /// # Arguments
    /// * `eq` - EQ effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::EQ;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_eq(EQ::new(1.5, 1.0, 1.2, 200.0, 3000.0));
    /// ```
    pub fn master_eq(&mut self, eq: crate::synthesis::effects::EQ) {
        self.master.eq = Some(eq);
        self.master.compute_effect_order();
    }

    /// Add parametric EQ to the master output
    ///
    /// Applies multi-band parametric equalization to the final stereo mix for
    /// precise frequency shaping and mastering.
    ///
    /// # Arguments
    /// * `parametric_eq` - ParametricEQ effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::ParametricEQ;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// let eq = ParametricEQ::new()
    ///     .band(100.0, -3.0, 0.7)  // Cut low rumble
    ///     .band(3000.0, 2.0, 1.5); // Boost presence
    /// mixer.master_parametric_eq(eq);
    /// ```
    pub fn master_parametric_eq(&mut self, parametric_eq: crate::synthesis::effects::ParametricEQ) {
        self.master.parametric_eq = Some(parametric_eq);
        self.master.compute_effect_order();
    }

    /// Add reverb to the master output
    ///
    /// Applies reverb to the final stereo mix. Use sparingly as master reverb
    /// affects the entire mix.
    ///
    /// # Arguments
    /// * `reverb` - Reverb effect configuration
    pub fn master_reverb(&mut self, reverb: crate::synthesis::effects::Reverb) {
        self.master.reverb = Some(reverb);
        self.master.compute_effect_order();
    }

    /// Add delay to the master output
    ///
    /// Applies delay to the final stereo mix.
    ///
    /// # Arguments
    /// * `delay` - Delay effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::Delay;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_delay(Delay::new(0.5, 0.4, 0.3));
    /// ```
    pub fn master_delay(&mut self, delay: crate::synthesis::effects::Delay) {
        self.master.delay = Some(delay);
        self.master.compute_effect_order();
    }

    /// Add gate to the master output
    ///
    /// Applies noise gate to the final stereo mix. Useful for cutting unwanted
    /// background noise or creating rhythmic gating effects.
    ///
    /// # Arguments
    /// * `gate` - Gate effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::Gate;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_gate(Gate::new(-40.0, 4.0, 0.01, 0.1));
    /// ```
    pub fn master_gate(&mut self, gate: crate::synthesis::effects::Gate) {
        self.master.gate = Some(gate);
        self.master.compute_effect_order();
    }

    /// Add saturation to the master output
    ///
    /// Applies saturation/warmth to the final stereo mix.
    ///
    /// # Arguments
    /// * `saturation` - Saturation effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::Saturation;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_saturation(Saturation::new(2.0, 0.5, 1.0));
    /// ```
    pub fn master_saturation(&mut self, saturation: crate::synthesis::effects::Saturation) {
        self.master.saturation = Some(saturation);
        self.master.compute_effect_order();
    }

    /// Add bit crusher to the master output
    ///
    /// Applies bit reduction and sample rate reduction to the final stereo mix
    /// for lo-fi effects.
    ///
    /// # Arguments
    /// * `bitcrusher` - BitCrusher effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::BitCrusher;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_bitcrusher(BitCrusher::new(8.0, 2.0, 1.0));
    /// ```
    pub fn master_bitcrusher(&mut self, bitcrusher: crate::synthesis::effects::BitCrusher) {
        self.master.bitcrusher = Some(bitcrusher);
        self.master.compute_effect_order();
    }

    /// Add distortion to the master output
    ///
    /// Applies distortion to the final stereo mix.
    ///
    /// # Arguments
    /// * `distortion` - Distortion effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::Distortion;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_distortion(Distortion::new(2.0, 0.5));
    /// ```
    pub fn master_distortion(&mut self, distortion: crate::synthesis::effects::Distortion) {
        self.master.distortion = Some(distortion);
        self.master.compute_effect_order();
    }

    /// Add chorus to the master output
    ///
    /// Applies chorus modulation to the final stereo mix for widening effects.
    ///
    /// # Arguments
    /// * `chorus` - Chorus effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::Chorus;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_chorus(Chorus::new(0.003, 0.5, 0.3));
    /// ```
    pub fn master_chorus(&mut self, chorus: crate::synthesis::effects::Chorus) {
        self.master.chorus = Some(chorus);
        self.master.compute_effect_order();
    }

    /// Add phaser to the master output
    ///
    /// Applies phaser modulation to the final stereo mix.
    ///
    /// # Arguments
    /// * `phaser` - Phaser effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::Phaser;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_phaser(Phaser::new(0.5, 0.7, 0.5, 4, 0.5));
    /// ```
    pub fn master_phaser(&mut self, phaser: crate::synthesis::effects::Phaser) {
        self.master.phaser = Some(phaser);
        self.master.compute_effect_order();
    }

    /// Add flanger to the master output
    ///
    /// Applies flanger modulation to the final stereo mix.
    ///
    /// # Arguments
    /// * `flanger` - Flanger effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::Flanger;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_flanger(Flanger::new(0.5, 3.0, 0.7, 0.5));
    /// ```
    pub fn master_flanger(&mut self, flanger: crate::synthesis::effects::Flanger) {
        self.master.flanger = Some(flanger);
        self.master.compute_effect_order();
    }

    /// Add ring modulator to the master output
    ///
    /// Applies ring modulation to the final stereo mix for metallic/robotic effects.
    ///
    /// # Arguments
    /// * `ring_mod` - RingModulator effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::RingModulator;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_ring_mod(RingModulator::new(30.0, 0.5));
    /// ```
    pub fn master_ring_mod(&mut self, ring_mod: crate::synthesis::effects::RingModulator) {
        self.master.ring_mod = Some(ring_mod);
        self.master.compute_effect_order();
    }

    /// Add tremolo to the master output
    ///
    /// Applies tremolo (amplitude modulation) to the final stereo mix.
    ///
    /// # Arguments
    /// * `tremolo` - Tremolo effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::Tremolo;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_tremolo(Tremolo::new(4.0, 0.5));
    /// ```
    pub fn master_tremolo(&mut self, tremolo: crate::synthesis::effects::Tremolo) {
        self.master.tremolo = Some(tremolo);
        self.master.compute_effect_order();
    }

    /// Add auto-pan to the master output
    ///
    /// Applies automatic panning to the final stereo mix, moving the sound
    /// between left and right channels.
    ///
    /// # Arguments
    /// * `autopan` - AutoPan effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::AutoPan;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_autopan(AutoPan::new(0.25, 1.0));
    /// ```
    pub fn master_autopan(&mut self, autopan: crate::synthesis::effects::AutoPan) {
        self.master.autopan = Some(autopan);
        self.master.compute_effect_order();
    }

    /// Add a phase vocoder to the master chain for time-stretching and pitch-shifting
    ///
    /// Phase vocoder allows independent control of time and pitch using STFT analysis.
    /// Perfect for creative time/pitch manipulation with phase coherence preservation.
    ///
    /// **Note**: This is a block-based spectral effect with ~23ms latency @ 44.1kHz.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::PhaseVocoder;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// let vocoder = PhaseVocoder::new(1.0, 7.0, 44100.0); // Perfect fifth up
    /// mixer.master_phase_vocoder(vocoder);
    /// ```
    pub fn master_phase_vocoder(&mut self, phase_vocoder: crate::synthesis::effects::PhaseVocoder) {
        self.master.phase_vocoder = Some(phase_vocoder);
        self.master.compute_effect_order();
    }

    /// Add a spectral freeze to the master chain for capturing and holding frequency spectrum
    ///
    /// Spectral freeze captures the current frequency content and holds it indefinitely,
    /// creating sustained "frozen" sounds. Perfect for ambient textures and drones.
    ///
    /// **Note**: This is a block-based spectral effect with ~23ms latency @ 44.1kHz.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralFreeze;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// let freeze = SpectralFreeze::new(true, 0.75, 44100.0); // 75% frozen, 25% live
    /// mixer.master_spectral_freeze(freeze);
    /// ```
    pub fn master_spectral_freeze(
        &mut self,
        spectral_freeze: crate::synthesis::effects::SpectralFreeze,
    ) {
        self.master.spectral_freeze = Some(spectral_freeze);
        self.master.compute_effect_order();
    }

    /// Add a spectral gate to the master chain for frequency-selective noise gating
    ///
    /// Spectral gate applies independent gating to each frequency bin, enabling surgical
    /// noise reduction. Unlike traditional gates that gate the entire signal, spectral gate
    /// can remove hum, hiss, and unwanted frequencies while preserving the wanted signal.
    ///
    /// **Note**: This is a block-based spectral effect with ~23ms latency @ 44.1kHz.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralGate;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// let gate = SpectralGate::new(-40.0, 0.001, 0.050, 0.0, 44100.0);
    /// mixer.master_spectral_gate(gate);
    /// ```
    pub fn master_spectral_gate(&mut self, spectral_gate: crate::synthesis::effects::SpectralGate) {
        self.master.spectral_gate = Some(spectral_gate);
        self.master.compute_effect_order();
    }

    /// Add a spectral compressor to the master chain for frequency-selective dynamic range compression
    ///
    /// Spectral compressor applies independent compression to each frequency bin, enabling
    /// multiband compression at extreme resolution (1024+ bands). This allows for surgical
    /// dynamic control that can't be achieved with traditional multiband compressors.
    ///
    /// **Note**: This is a block-based spectral effect with ~23ms latency @ 44.1kHz.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralCompressor;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// let compressor = SpectralCompressor::new(-20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
    /// mixer.master_spectral_compressor(compressor);
    /// ```
    pub fn master_spectral_compressor(
        &mut self,
        spectral_compressor: crate::synthesis::effects::SpectralCompressor,
    ) {
        self.master.spectral_compressor = Some(spectral_compressor);
        self.master.compute_effect_order();
    }

    pub fn master_spectral_robotize(
        &mut self,
        spectral_robotize: crate::synthesis::effects::SpectralRobotize,
    ) {
        self.master.spectral_robotize = Some(spectral_robotize);
        self.master.compute_effect_order();
    }

    pub fn master_spectral_delay(
        &mut self,
        spectral_delay: crate::synthesis::effects::SpectralDelay,
    ) {
        self.master.spectral_delay = Some(spectral_delay);
        self.master.compute_effect_order();
    }

    /// Add spectral filter to the master output for frequency-domain filtering
    ///
    /// Applies filtering in the frequency domain, allowing precise control over the
    /// frequency response with minimal artifacts. Unlike time-domain filters, spectral
    /// filtering can achieve brick-wall responses and frequency-dependent effects.
    ///
    /// **Note**: This is a block-based spectral effect with ~23ms latency @ 44.1kHz.
    ///
    /// # Arguments
    /// * `spectral_filter` - SpectralFilter effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralFilter;
    /// # use tunes::synthesis::spectral::FilterType;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// let filter = SpectralFilter::new(FilterType::LowPass, 8000.0, 1.0, 1.0, 44100.0);
    /// mixer.master_spectral_filter(filter);
    /// ```
    pub fn master_spectral_filter(
        &mut self,
        spectral_filter: crate::synthesis::effects::SpectralFilter,
    ) -> &mut Self {
        self.master.spectral_filter = Some(spectral_filter);
        self.master.compute_effect_order();
        self
    }

    /// Add spectral shift effect to the master chain for frequency shifting
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralShift;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_spectral_shift(SpectralShift::subtle());
    /// ```
    pub fn master_spectral_shift(
        &mut self,
        spectral_shift: crate::synthesis::effects::SpectralShift,
    ) -> &mut Self {
        self.master.spectral_shift = Some(spectral_shift);
        self.master.compute_effect_order();
        self
    }

    /// Add formant shifter effect to the master chain for vocal character transformation
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::FormantShifter;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_formant_shifter(FormantShifter::male_to_female());
    /// ```
    pub fn master_formant_shifter(
        &mut self,
        formant_shifter: crate::synthesis::effects::FormantShifter,
    ) -> &mut Self {
        self.master.formant_shifter = Some(formant_shifter);
        self.master.compute_effect_order();
        self
    }

    /// Add spectral harmonizer effect to the master chain for pitch-shifted harmonies
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralHarmonizer;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_spectral_harmonizer(SpectralHarmonizer::fifth());
    /// ```
    pub fn master_spectral_harmonizer(
        &mut self,
        spectral_harmonizer: crate::synthesis::effects::SpectralHarmonizer,
    ) -> &mut Self {
        self.master.spectral_harmonizer = Some(spectral_harmonizer);
        self.master.compute_effect_order();
        self
    }

    /// Add spectral resonator effect to the master chain for resonant frequency peaks
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralResonator;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_spectral_resonator(SpectralResonator::bell());
    /// ```
    pub fn master_spectral_resonator(
        &mut self,
        spectral_resonator: crate::synthesis::effects::SpectralResonator,
    ) -> &mut Self {
        self.master.spectral_resonator = Some(spectral_resonator);
        self.master.compute_effect_order();
        self
    }

    /// Add spectral panner effect to the master chain for frequency-based spatial positioning
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralPanner;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_spectral_panner(SpectralPanner::circular());
    /// ```
    pub fn master_spectral_panner(
        &mut self,
        spectral_panner: crate::synthesis::effects::SpectralPanner,
    ) -> &mut Self {
        self.master.spectral_panner = Some(spectral_panner);
        self.master.compute_effect_order();
        self
    }

    /// Add spectral exciter effect to the master chain for harmonic enhancement
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralExciter;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_spectral_exciter(SpectralExciter::gentle());
    /// ```
    pub fn master_spectral_exciter(
        &mut self,
        spectral_exciter: crate::synthesis::effects::SpectralExciter,
    ) -> &mut Self {
        self.master.spectral_exciter = Some(spectral_exciter);
        self.master.compute_effect_order();
        self
    }

    /// Add spectral invert effect to the master chain for frequency spectrum reversal
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralInvert;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_spectral_invert(SpectralInvert::full());
    /// ```
    pub fn master_spectral_invert(
        &mut self,
        spectral_invert: crate::synthesis::effects::SpectralInvert,
    ) -> &mut Self {
        self.master.spectral_invert = Some(spectral_invert);
        self.master.compute_effect_order();
        self
    }

    /// Add spectral widen effect to the master chain for stereo widening
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralWiden;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_spectral_widen(SpectralWiden::wide());
    /// ```
    pub fn master_spectral_widen(
        &mut self,
        spectral_widen: crate::synthesis::effects::SpectralWiden,
    ) -> &mut Self {
        self.master.spectral_widen = Some(spectral_widen);
        self.master.compute_effect_order();
        self
    }

    /// Add spectral morph effect to the master chain for morphing spectrum toward target shapes
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralMorph;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_spectral_morph(SpectralMorph::robot());
    /// ```
    pub fn master_spectral_morph(
        &mut self,
        spectral_morph: crate::synthesis::effects::SpectralMorph,
    ) -> &mut Self {
        self.master.spectral_morph = Some(spectral_morph);
        self.master.compute_effect_order();
        self
    }

    /// Add spectral dynamics to the master bus for frequency-dependent compression/expansion
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralDynamics;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_spectral_dynamics(SpectralDynamics::gentle());
    /// ```
    pub fn master_spectral_dynamics(
        &mut self,
        spectral_dynamics: crate::synthesis::effects::SpectralDynamics,
    ) -> &mut Self {
        self.master.spectral_dynamics = Some(spectral_dynamics);
        self.master.compute_effect_order();
        self
    }

    /// Add spectral scramble to the master bus for glitchy frequency bin randomization
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::synthesis::effects::SpectralScramble;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_spectral_scramble(SpectralScramble::glitch());
    /// ```
    pub fn master_spectral_scramble(
        &mut self,
        spectral_scramble: crate::synthesis::effects::SpectralScramble,
    ) -> &mut Self {
        self.master.spectral_scramble = Some(spectral_scramble);
        self.master.compute_effect_order();
        self
    }
}
