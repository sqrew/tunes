//! Audio effects processing
//!
//! This module provides a comprehensive collection of audio effects for real-time processing.
//! Effects can be used individually or chained together via `EffectChain`.

// Submodules
pub mod delay;
pub mod reverb;
pub mod distortion;
pub mod dynamics;
pub mod modulation;
pub mod spatial;
pub mod eq;
pub mod convolution;
pub mod spectral;

// Re-export all effect types
pub use delay::Delay;
pub use reverb::Reverb;
pub use distortion::{Distortion, BitCrusher, Saturation};
pub use dynamics::{Compressor, CompressorBand, Gate, Limiter, SidechainSource, ResolvedSidechainSource};
pub use modulation::{Chorus, Phaser, Flanger, RingModulator, Tremolo};
pub use spatial::AutoPan;
pub use eq::{EQ, EQBand, ParametricEQ, EQPreset};
pub use convolution::{Convolution, ConvolutionReverb, IRParams};
pub use spectral::{FilterType, FormantShifter, HarmonyVoice, PanPoint, PhaseVocoder, Resonance, SpectralFreeze, SpectralGate, SpectralCompressor, SpectralHarmonizer, SpectralPanner, SpectralResonator, SpectralRobotize, SpectralDelay, SpectralFilter, SpectralBlur, SpectralShift, SpectralExciter, SpectralInvert, SpectralWiden, SpectralMorph, SpectralMorphTarget, SpectralDynamics, SpectralScramble};

/// Effect chain for processing audio through multiple effects in priority order
///
/// The effect chain allows you to combine multiple effects and process audio samples
/// through them in a defined order based on priority. Lower priority values are processed
/// earlier in the chain.
///
/// # Example
/// ```no_run
/// use tunes::prelude::*;
///
/// let chain = EffectChain::new()
///     .with_eq(EQ::new(1.0, 1.0, 1.0, 250.0, 4000.0))
///     .with_compressor(Compressor::new(0.5, 4.0, 0.01, 0.1, 1.0))
///     .with_reverb(Reverb::hall());
/// ```
#[derive(Clone)]
pub struct EffectChain {
    // All available effects
    pub eq: Option<EQ>,
    pub compressor: Option<Compressor>,
    pub gate: Option<Gate>,
    pub saturation: Option<Saturation>,
    pub bitcrusher: Option<BitCrusher>,
    pub distortion: Option<Distortion>,
    pub chorus: Option<Chorus>,
    pub phaser: Option<Phaser>,
    pub flanger: Option<Flanger>,
    pub ring_mod: Option<RingModulator>,
    pub tremolo: Option<Tremolo>,
    pub autopan: Option<AutoPan>,
    pub delay: Option<Delay>,
    pub reverb: Option<Reverb>,
    pub convolution_reverb: Option<ConvolutionReverb>,
    pub limiter: Option<Limiter>,
    pub parametric_eq: Option<ParametricEQ>,
    pub phase_vocoder: Option<PhaseVocoder>,
    pub spectral_freeze: Option<SpectralFreeze>,
    pub spectral_gate: Option<SpectralGate>,
    pub spectral_compressor: Option<SpectralCompressor>,
    pub spectral_robotize: Option<SpectralRobotize>,
    pub spectral_delay: Option<SpectralDelay>,
    pub spectral_filter: Option<SpectralFilter>,
    pub spectral_blur: Option<SpectralBlur>,
    pub spectral_shift: Option<SpectralShift>,
    pub spectral_exciter: Option<SpectralExciter>,
    pub spectral_invert: Option<SpectralInvert>,
    pub spectral_widen: Option<SpectralWiden>,
    pub spectral_morph: Option<SpectralMorph>,
    pub spectral_dynamics: Option<SpectralDynamics>,
    pub spectral_scramble: Option<SpectralScramble>,
    pub formant_shifter: Option<FormantShifter>,
    pub spectral_harmonizer: Option<SpectralHarmonizer>,
    pub spectral_resonator: Option<SpectralResonator>,
    pub spectral_panner: Option<SpectralPanner>,

    // Pre-computed effect processing order (cached for performance)
    // Effect IDs: 0=EQ, 1=Compressor, 2=Gate, 3=Saturation, 4=BitCrusher, 5=Distortion,
    //             6=Chorus, 7=Phaser, 8=Flanger, 9=RingMod, 10=Tremolo,
    //             11=Delay, 12=Reverb, 13=Limiter, 14=ParametricEQ, 15=ConvolutionReverb,
    //             16=PhaseVocoder, 17=SpectralFreeze, 18=SpectralGate, 19=SpectralCompressor,
    //             20=SpectralRobotize, 21=SpectralDelay, 22=SpectralFilter, 23=SpectralBlur,
    //             24=SpectralShift, 25=SpectralExciter, 26=SpectralInvert, 27=SpectralWiden, 28=SpectralMorph,
    //             29=SpectralDynamics, 30=SpectralScramble, 31=FormantShifter, 32=SpectralHarmonizer,
    //             33=SpectralResonator, 34=SpectralPanner
    // (AutoPan excluded - handled separately in stereo stage)
    pub(crate) effect_order: Vec<u8>,
}

impl std::fmt::Debug for EffectChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EffectChain")
            .field("effect_count", &self.effect_order.len())
            .finish()
    }
}

impl EffectChain {
    /// Create a new empty effect chain
    pub fn new() -> Self {
        Self {
            eq: None,
            compressor: None,
            gate: None,
            saturation: None,
            bitcrusher: None,
            distortion: None,
            chorus: None,
            phaser: None,
            flanger: None,
            ring_mod: None,
            tremolo: None,
            autopan: None,
            delay: None,
            reverb: None,
            convolution_reverb: None,
            limiter: None,
            parametric_eq: None,
            phase_vocoder: None,
            spectral_freeze: None,
            spectral_gate: None,
            spectral_compressor: None,
            spectral_robotize: None,
            spectral_delay: None,
            spectral_filter: None,
            spectral_blur: None,
            spectral_shift: None,
            spectral_exciter: None,
            spectral_invert: None,
            spectral_widen: None,
            spectral_morph: None,
            spectral_dynamics: None,
            spectral_scramble: None,
            formant_shifter: None,
            spectral_harmonizer: None,
            spectral_resonator: None,
            spectral_panner: None,
            effect_order: Vec::new(),
        }
    }

    /// Compute the effect processing order based on priority
    ///
    /// Called automatically when effects are added/modified.
    /// This pre-computation avoids allocating and sorting on every audio sample.
    pub fn compute_effect_order(&mut self) {
        // Build list of (priority, effect_id) for active effects
        let mut effects = Vec::with_capacity(15);

        if let Some(ref eq) = self.eq {
            effects.push((eq.priority, 0));
        }
        if let Some(ref compressor) = self.compressor {
            effects.push((compressor.priority, 1));
        }
        if let Some(ref gate) = self.gate {
            effects.push((gate.priority, 2));
        }
        if let Some(ref saturation) = self.saturation {
            effects.push((saturation.priority, 3));
        }
        if let Some(ref bitcrusher) = self.bitcrusher {
            effects.push((bitcrusher.priority, 4));
        }
        if let Some(ref distortion) = self.distortion {
            effects.push((distortion.priority, 5));
        }
        if let Some(ref chorus) = self.chorus {
            effects.push((chorus.priority, 6));
        }
        if let Some(ref phaser) = self.phaser {
            effects.push((phaser.priority, 7));
        }
        if let Some(ref flanger) = self.flanger {
            effects.push((flanger.priority, 8));
        }
        if let Some(ref ring_mod) = self.ring_mod {
            effects.push((ring_mod.priority, 9));
        }
        if let Some(ref tremolo) = self.tremolo {
            effects.push((tremolo.priority, 10));
        }
        if let Some(ref delay) = self.delay {
            effects.push((delay.priority, 11));
        }
        if let Some(ref reverb) = self.reverb {
            effects.push((reverb.priority, 12));
        }
        if let Some(ref limiter) = self.limiter {
            effects.push((limiter.priority, 13));
        }
        if let Some(ref parametric_eq) = self.parametric_eq {
            effects.push((parametric_eq.priority, 14));
        }
        if let Some(ref convolution_reverb) = self.convolution_reverb {
            effects.push((convolution_reverb.priority, 15));
        }
        if let Some(ref phase_vocoder) = self.phase_vocoder {
            effects.push((phase_vocoder.priority, 16));
        }
        if let Some(ref spectral_freeze) = self.spectral_freeze {
            effects.push((spectral_freeze.priority, 17));
        }
        if let Some(ref spectral_gate) = self.spectral_gate {
            effects.push((spectral_gate.priority, 18));
        }
        if let Some(ref spectral_compressor) = self.spectral_compressor {
            effects.push((spectral_compressor.priority, 19));
        }
        if let Some(ref spectral_robotize) = self.spectral_robotize {
            effects.push((spectral_robotize.priority, 20));
        }
        if let Some(ref spectral_delay) = self.spectral_delay {
            effects.push((spectral_delay.priority, 21));
        }
        if let Some(ref spectral_filter) = self.spectral_filter {
            effects.push((spectral_filter.priority, 22));
        }
        if let Some(ref spectral_blur) = self.spectral_blur {
            effects.push((spectral_blur.priority, 23));
        }
        if let Some(ref spectral_shift) = self.spectral_shift {
            effects.push((spectral_shift.priority, 24));
        }
        if let Some(ref spectral_exciter) = self.spectral_exciter {
            effects.push((spectral_exciter.priority, 25));
        }
        if let Some(ref spectral_invert) = self.spectral_invert {
            effects.push((spectral_invert.priority, 26));
        }
        if let Some(ref spectral_widen) = self.spectral_widen {
            effects.push((spectral_widen.priority, 27));
        }
        if let Some(ref spectral_morph) = self.spectral_morph {
            effects.push((spectral_morph.priority, 28));
        }
        if let Some(ref spectral_dynamics) = self.spectral_dynamics {
            effects.push((spectral_dynamics.priority, 29));
        }
        if let Some(ref spectral_scramble) = self.spectral_scramble {
            effects.push((spectral_scramble.priority, 30));
        }
        if let Some(ref formant_shifter) = self.formant_shifter {
            effects.push((formant_shifter.priority, 31));
        }
        if let Some(ref spectral_harmonizer) = self.spectral_harmonizer {
            effects.push((spectral_harmonizer.priority, 32));
        }
        if let Some(ref spectral_resonator) = self.spectral_resonator {
            effects.push((spectral_resonator.priority, 33));
        }
        if let Some(ref spectral_panner) = self.spectral_panner {
            effects.push((spectral_panner.priority, 34));
        }

        // Sort by priority (lower = earlier in chain)
        effects.sort_by_key(|&(priority, _)| priority);

        // Extract just the effect IDs (reuse existing capacity)
        self.effect_order.clear();
        self.effect_order.extend(effects.into_iter().map(|(_, id)| id));
    }

    /// Process a mono audio sample through the effect chain
    ///
    /// Used for track-level effects. Processes a single sample through all active effects
    /// in priority order.
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    ///
    /// # Returns
    /// Processed mono sample
    #[inline]
    pub fn process_mono(
        &mut self,
        input: f32,
        sample_rate: f32,
        time: f32,
        sample_count: u64,
    ) -> f32 {
        let mut signal = input;

        // Process effects in pre-computed priority order
        for &effect_id in &self.effect_order {
            signal = match effect_id {
                0 => {
                    // EQ
                    if let Some(ref mut eq) = self.eq {
                        eq.process(signal, sample_rate, time, sample_count)
                    } else {
                        signal
                    }
                }
                1 => {
                    // Compressor
                    if let Some(ref mut compressor) = self.compressor {
                        compressor.process(signal, sample_rate, time, sample_count, None)
                    } else {
                        signal
                    }
                }
                2 => {
                    // Gate
                    if let Some(ref mut gate) = self.gate {
                        gate.process(signal, sample_rate, time, sample_count)
                    } else {
                        signal
                    }
                }
                3 => {
                    // Saturation
                    if let Some(ref mut saturation) = self.saturation {
                        saturation.process(signal, time, sample_count)
                    } else {
                        signal
                    }
                }
                4 => {
                    // BitCrusher
                    if let Some(ref mut bitcrusher) = self.bitcrusher {
                        bitcrusher.process(signal, time, sample_count)
                    } else {
                        signal
                    }
                }
                5 => {
                    // Distortion
                    if let Some(ref mut distortion) = self.distortion {
                        distortion.process(signal, time, sample_count)
                    } else {
                        signal
                    }
                }
                6 => {
                    // Chorus
                    if let Some(ref mut chorus) = self.chorus {
                        chorus.process(signal, sample_rate, time, sample_count)
                    } else {
                        signal
                    }
                }
                7 => {
                    // Phaser
                    if let Some(ref mut phaser) = self.phaser {
                        phaser.process(signal, sample_rate, time, sample_count)
                    } else {
                        signal
                    }
                }
                8 => {
                    // Flanger
                    if let Some(ref mut flanger) = self.flanger {
                        flanger.process(signal, sample_rate, time, sample_count)
                    } else {
                        signal
                    }
                }
                9 => {
                    // Ring Modulator
                    if let Some(ref mut ring_mod) = self.ring_mod {
                        ring_mod.process(signal, sample_rate, time, sample_count)
                    } else {
                        signal
                    }
                }
                10 => {
                    // Tremolo
                    if let Some(ref mut tremolo) = self.tremolo {
                        tremolo.process(signal, sample_rate, time, sample_count)
                    } else {
                        signal
                    }
                }
                11 => {
                    // Delay
                    if let Some(ref mut delay) = self.delay {
                        delay.process(signal, time, sample_count)
                    } else {
                        signal
                    }
                }
                12 => {
                    // Reverb
                    if let Some(ref mut reverb) = self.reverb {
                        reverb.process(signal, time, sample_count)
                    } else {
                        signal
                    }
                }
                13 => {
                    // Limiter
                    if let Some(ref mut limiter) = self.limiter {
                        limiter.process(signal, sample_rate, time, sample_count)
                    } else {
                        signal
                    }
                }
                14 => {
                    // ParametricEQ
                    if let Some(ref mut parametric_eq) = self.parametric_eq {
                        parametric_eq.process(signal, time, sample_count as usize)
                    } else {
                        signal
                    }
                }
                15 => {
                    // ConvolutionReverb
                    if let Some(ref mut convolution_reverb) = self.convolution_reverb {
                        convolution_reverb.process(signal)
                    } else {
                        signal
                    }
                }
                _ => signal,
            };
        }

        signal
    }

    /// Process a block of mono audio samples through the effect chain
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    #[inline]
    pub fn process_mono_block(
        &mut self,
        buffer: &mut [f32],
        sample_rate: f32,
        time: f32,
        sample_count: u64,
    ) {
        // Process effects in pre-computed priority order
        // Each effect processes the entire buffer before moving to the next effect
        for &effect_id in &self.effect_order {
            match effect_id {
                0 => {
                    // EQ
                    if let Some(ref mut eq) = self.eq {
                        eq.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                1 => {
                    // Compressor
                    if let Some(ref mut compressor) = self.compressor {
                        compressor.process_block(buffer, sample_rate, time, sample_count, None);
                    }
                }
                2 => {
                    // Gate
                    if let Some(ref mut gate) = self.gate {
                        gate.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                3 => {
                    // Saturation
                    if let Some(ref mut saturation) = self.saturation {
                        saturation.process_block(buffer, time, sample_count, sample_rate);
                    }
                }
                4 => {
                    // BitCrusher
                    if let Some(ref mut bitcrusher) = self.bitcrusher {
                        bitcrusher.process_block(buffer, time, sample_count, sample_rate);
                    }
                }
                5 => {
                    // Distortion
                    if let Some(ref mut distortion) = self.distortion {
                        distortion.process_block(buffer, time, sample_count, sample_rate);
                    }
                }
                6 => {
                    // Chorus
                    if let Some(ref mut chorus) = self.chorus {
                        chorus.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                7 => {
                    // Phaser
                    if let Some(ref mut phaser) = self.phaser {
                        phaser.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                8 => {
                    // Flanger
                    if let Some(ref mut flanger) = self.flanger {
                        flanger.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                9 => {
                    // Ring Modulator
                    if let Some(ref mut ring_mod) = self.ring_mod {
                        ring_mod.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                10 => {
                    // Tremolo
                    if let Some(ref mut tremolo) = self.tremolo {
                        tremolo.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                11 => {
                    // Delay
                    if let Some(ref mut delay) = self.delay {
                        delay.process_block(buffer, time, sample_count, sample_rate);
                    }
                }
                12 => {
                    // Reverb
                    if let Some(ref mut reverb) = self.reverb {
                        reverb.process_block(buffer, time, sample_count, sample_rate);
                    }
                }
                13 => {
                    // Limiter
                    if let Some(ref mut limiter) = self.limiter {
                        limiter.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                14 => {
                    // ParametricEQ
                    if let Some(ref mut parametric_eq) = self.parametric_eq {
                        parametric_eq.process_block(buffer, time, sample_count as usize, sample_rate);
                    }
                }
                15 => {
                    // ConvolutionReverb
                    if let Some(ref mut convolution_reverb) = self.convolution_reverb {
                        convolution_reverb.process_block_direct(buffer);
                    }
                }
                16 => {
                    // PhaseVocoder (block-based spectral effect)
                    if let Some(ref mut phase_vocoder) = self.phase_vocoder {
                        phase_vocoder.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                17 => {
                    // SpectralFreeze (block-based spectral effect)
                    if let Some(ref mut spectral_freeze) = self.spectral_freeze {
                        spectral_freeze.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                18 => {
                    // SpectralGate (block-based spectral effect)
                    if let Some(ref mut spectral_gate) = self.spectral_gate {
                        spectral_gate.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                19 => {
                    // SpectralCompressor (block-based spectral effect)
                    if let Some(ref mut spectral_compressor) = self.spectral_compressor {
                        spectral_compressor.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                20 => {
                    // SpectralRobotize (block-based spectral effect)
                    if let Some(ref mut spectral_robotize) = self.spectral_robotize {
                        spectral_robotize.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                21 => {
                    // SpectralDelay (block-based spectral effect)
                    if let Some(ref mut spectral_delay) = self.spectral_delay {
                        spectral_delay.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                22 => {
                    // SpectralFilter (block-based spectral effect)
                    if let Some(ref mut spectral_filter) = self.spectral_filter {
                        spectral_filter.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                23 => {
                    // SpectralBlur (block-based spectral effect)
                    if let Some(ref mut spectral_blur) = self.spectral_blur {
                        spectral_blur.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                24 => {
                    // SpectralShift (block-based spectral effect)
                    if let Some(ref mut spectral_shift) = self.spectral_shift {
                        spectral_shift.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                25 => {
                    // SpectralExciter (block-based spectral effect)
                    if let Some(ref mut spectral_exciter) = self.spectral_exciter {
                        spectral_exciter.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                26 => {
                    // SpectralInvert (block-based spectral effect)
                    if let Some(ref mut spectral_invert) = self.spectral_invert {
                        spectral_invert.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                27 => {
                    // SpectralWiden (block-based spectral effect)
                    if let Some(ref mut spectral_widen) = self.spectral_widen {
                        spectral_widen.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                28 => {
                    // SpectralMorph (block-based spectral effect)
                    if let Some(ref mut spectral_morph) = self.spectral_morph {
                        spectral_morph.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                29 => {
                    // SpectralDynamics (block-based spectral effect)
                    if let Some(ref mut spectral_dynamics) = self.spectral_dynamics {
                        spectral_dynamics.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                30 => {
                    // SpectralScramble (block-based spectral effect)
                    if let Some(ref mut spectral_scramble) = self.spectral_scramble {
                        spectral_scramble.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                31 => {
                    // FormantShifter (block-based spectral effect)
                    if let Some(ref mut formant_shifter) = self.formant_shifter {
                        formant_shifter.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                32 => {
                    // SpectralHarmonizer (block-based spectral effect)
                    if let Some(ref mut spectral_harmonizer) = self.spectral_harmonizer {
                        spectral_harmonizer.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                33 => {
                    // SpectralResonator (block-based spectral effect)
                    if let Some(ref mut spectral_resonator) = self.spectral_resonator {
                        spectral_resonator.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                34 => {
                    // SpectralPanner (block-based spectral effect)
                    if let Some(ref mut spectral_panner) = self.spectral_panner {
                        spectral_panner.process_block(buffer, sample_rate, time, sample_count);
                    }
                }
                _ => {}
            };
        }
    }

    /// Process a stereo audio sample through the effect chain
    ///
    /// Used for master and bus-level effects. Processes stereo samples through all active
    /// effects in priority order. Some effects (like compressor/limiter) use stereo-linked
    /// processing to prevent image shifting.
    ///
    /// # Arguments
    /// * `left` - Left channel input
    /// * `right` - Right channel input
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    /// * `sidechain_envelope` - Optional sidechain envelope for compressor (looked up by mixer)
    ///
    /// # Returns
    /// Processed stereo sample as (left, right)
    #[inline]
    pub fn process_stereo(
        &mut self,
        left: f32,
        right: f32,
        sample_rate: f32,
        time: f32,
        sample_count: u64,
        sidechain_envelope: Option<f32>,
    ) -> (f32, f32) {
        let mut left_signal = left;
        let mut right_signal = right;

        // Process effects in pre-computed priority order
        // Compressor and limiter use stereo-linked processing to prevent image shift
        for &effect_id in &self.effect_order {
            match effect_id {
                0 => {
                    // EQ (process each channel)
                    if let Some(ref mut eq) = self.eq {
                        left_signal = eq.process(left_signal, sample_rate, time, sample_count);
                        right_signal = eq.process(right_signal, sample_rate, time, sample_count);
                    }
                }
                1 => {
                    // Compressor (stereo-linked - detects from max, applies same gain to both channels)
                    if let Some(ref mut compressor) = self.compressor {
                        let (left_out, right_out) = compressor.process_stereo_linked(
                            left_signal,
                            right_signal,
                            sample_rate,
                            time,
                            sample_count,
                            sidechain_envelope,
                        );
                        left_signal = left_out;
                        right_signal = right_out;
                    }
                }
                2 => {
                    // Gate (process each channel)
                    if let Some(ref mut gate) = self.gate {
                        left_signal = gate.process(left_signal, sample_rate, time, sample_count);
                        right_signal = gate.process(right_signal, sample_rate, time, sample_count);
                    }
                }
                3 => {
                    // Saturation (process each channel)
                    if let Some(ref mut saturation) = self.saturation {
                        left_signal = saturation.process(left_signal, time, sample_count);
                        right_signal = saturation.process(right_signal, time, sample_count);
                    }
                }
                4 => {
                    // BitCrusher (process each channel)
                    if let Some(ref mut bitcrusher) = self.bitcrusher {
                        left_signal = bitcrusher.process(left_signal, time, sample_count);
                        right_signal = bitcrusher.process(right_signal, time, sample_count);
                    }
                }
                5 => {
                    // Distortion (process each channel)
                    if let Some(ref mut distortion) = self.distortion {
                        left_signal = distortion.process(left_signal, time, sample_count);
                        right_signal = distortion.process(right_signal, time, sample_count);
                    }
                }
                6 => {
                    // Chorus (process each channel)
                    if let Some(ref mut chorus) = self.chorus {
                        left_signal = chorus.process(left_signal, sample_rate, time, sample_count);
                        right_signal = chorus.process(right_signal, sample_rate, time, sample_count);
                    }
                }
                7 => {
                    // Phaser (process each channel)
                    if let Some(ref mut phaser) = self.phaser {
                        left_signal = phaser.process(left_signal, sample_rate, time, sample_count);
                        right_signal = phaser.process(right_signal, sample_rate, time, sample_count);
                    }
                }
                8 => {
                    // Flanger (process each channel)
                    if let Some(ref mut flanger) = self.flanger {
                        left_signal = flanger.process(left_signal, sample_rate, time, sample_count);
                        right_signal = flanger.process(right_signal, sample_rate, time, sample_count);
                    }
                }
                9 => {
                    // Ring Modulator (process each channel)
                    if let Some(ref mut ring_mod) = self.ring_mod {
                        left_signal = ring_mod.process(left_signal, sample_rate, time, sample_count);
                        right_signal = ring_mod.process(right_signal, sample_rate, time, sample_count);
                    }
                }
                10 => {
                    // Tremolo (process each channel)
                    if let Some(ref mut tremolo) = self.tremolo {
                        left_signal = tremolo.process(left_signal, sample_rate, time, sample_count);
                        right_signal = tremolo.process(right_signal, sample_rate, time, sample_count);
                    }
                }
                11 => {
                    // Delay (process each channel)
                    if let Some(ref mut delay) = self.delay {
                        left_signal = delay.process(left_signal, time, sample_count);
                        right_signal = delay.process(right_signal, time, sample_count);
                    }
                }
                12 => {
                    // Reverb (process each channel)
                    if let Some(ref mut reverb) = self.reverb {
                        left_signal = reverb.process(left_signal, time, sample_count);
                        right_signal = reverb.process(right_signal, time, sample_count);
                    }
                }
                13 => {
                    // Limiter (stereo-linked - detects from max, applies same gain to both channels)
                    if let Some(ref mut limiter) = self.limiter {
                        let (left_out, right_out) = limiter.process_stereo_linked(
                            left_signal,
                            right_signal,
                            sample_rate,
                            time,
                            sample_count,
                        );
                        left_signal = left_out;
                        right_signal = right_out;
                    }
                }
                14 => {
                    // ParametricEQ (process each channel)
                    if let Some(ref mut parametric_eq) = self.parametric_eq {
                        left_signal = parametric_eq.process(left_signal, time, sample_count as usize);
                        right_signal = parametric_eq.process(right_signal, time, sample_count as usize);
                    }
                }
                _ => {}
            }
        }

        (left_signal, right_signal)
    }

    /// Process a block of stereo audio samples through the effect chain
    ///
    /// OPTIMIZED: Process effect-by-effect on full buffer instead of sample-by-sample.
    /// Uses temporary buffers to deinterleave, process with optimized process_block methods,
    /// then reinterleave.
    ///
    /// # Arguments
    /// * `buffer` - Interleaved stereo buffer [L0, R0, L1, R1, ...] to process in-place
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    /// * `sidechain_envelope` - Optional sidechain envelope for compressor
    #[inline]
    pub fn process_stereo_block(
        &mut self,
        buffer: &mut [f32],
        sample_rate: f32,
        time: f32,
        sample_count: u64,
        sidechain_envelope: Option<f32>,
    ) {
        // OPTIMIZATION: Process each effect on the full buffer instead of
        // processing each sample through all effects. This allows effects
        // to calculate constants once and improves cache locality.

        let num_frames = buffer.len() / 2;

        // Allocate temporary buffers for deinterleaved L/R channels
        // Note: Modern allocators are fast enough that caching these buffers
        // actually hurts performance due to resize/clear overhead
        let mut left_buffer = vec![0.0f32; num_frames];
        let mut right_buffer = vec![0.0f32; num_frames];

        // Deinterleave stereo buffer into separate L/R channels
        for (i, frame) in buffer.chunks_exact(2).enumerate() {
            left_buffer[i] = frame[0];
            right_buffer[i] = frame[1];
        }

        // Process each effect on the full buffer
        for &effect_id in &self.effect_order {
            match effect_id {
                1 => {
                    // Compressor (stereo-linked): reinterleave, process, deinterleave
                    for (i, frame) in buffer.chunks_exact_mut(2).enumerate() {
                        frame[0] = left_buffer[i];
                        frame[1] = right_buffer[i];
                    }
                    if let Some(ref mut compressor) = self.compressor {
                        compressor.process_stereo_block(buffer, sample_rate, time, sample_count, sidechain_envelope);
                    }
                    for (i, frame) in buffer.chunks_exact(2).enumerate() {
                        left_buffer[i] = frame[0];
                        right_buffer[i] = frame[1];
                    }
                }
                13 => {
                    // Limiter (stereo-linked): reinterleave, process, deinterleave
                    for (i, frame) in buffer.chunks_exact_mut(2).enumerate() {
                        frame[0] = left_buffer[i];
                        frame[1] = right_buffer[i];
                    }
                    if let Some(ref mut limiter) = self.limiter {
                        limiter.process_stereo_block(buffer, sample_rate, time, sample_count);
                    }
                    for (i, frame) in buffer.chunks_exact(2).enumerate() {
                        left_buffer[i] = frame[0];
                        right_buffer[i] = frame[1];
                    }
                }
                6 => {
                    // Chorus: use optimized process_block
                    if let Some(ref mut chorus) = self.chorus {
                        chorus.process_block(&mut left_buffer, sample_rate, time, sample_count);
                        chorus.process_block(&mut right_buffer, sample_rate, time, sample_count);
                    }
                }
                7 => {
                    // Phaser: use optimized process_block
                    if let Some(ref mut phaser) = self.phaser {
                        phaser.process_block(&mut left_buffer, sample_rate, time, sample_count);
                        phaser.process_block(&mut right_buffer, sample_rate, time, sample_count);
                    }
                }
                8 => {
                    // Flanger: use optimized process_block
                    if let Some(ref mut flanger) = self.flanger {
                        flanger.process_block(&mut left_buffer, sample_rate, time, sample_count);
                        flanger.process_block(&mut right_buffer, sample_rate, time, sample_count);
                    }
                }
                11 => {
                    // Delay: use optimized process_block
                    if let Some(ref mut delay) = self.delay {
                        delay.process_block(&mut left_buffer, time, sample_count, sample_rate);
                        delay.process_block(&mut right_buffer, time, sample_count, sample_rate);
                    }
                }
                12 => {
                    // Reverb: use optimized process_block
                    if let Some(ref mut reverb) = self.reverb {
                        reverb.process_block(&mut left_buffer, time, sample_count, sample_rate);
                        reverb.process_block(&mut right_buffer, time, sample_count, sample_rate);
                    }
                }
                2 => {
                    // Gate: use optimized process_block
                    if let Some(ref mut gate) = self.gate {
                        gate.process_block(&mut left_buffer, sample_rate, time, sample_count);
                        gate.process_block(&mut right_buffer, sample_rate, time, sample_count);
                    }
                }
                _ => {
                    // Other effects: fall back to per-sample processing
                    for i in 0..num_frames {
                        match effect_id {
                            0 => {
                                // EQ
                                if let Some(ref mut eq) = self.eq {
                                    left_buffer[i] = eq.process(left_buffer[i], sample_rate, time, sample_count);
                                    right_buffer[i] = eq.process(right_buffer[i], sample_rate, time, sample_count);
                                }
                            }
                            3 => {
                                // Saturation
                                if let Some(ref mut saturation) = self.saturation {
                                    left_buffer[i] = saturation.process(left_buffer[i], time, sample_count);
                                    right_buffer[i] = saturation.process(right_buffer[i], time, sample_count);
                                }
                            }
                            4 => {
                                // BitCrusher
                                if let Some(ref mut bitcrusher) = self.bitcrusher {
                                    left_buffer[i] = bitcrusher.process(left_buffer[i], time, sample_count);
                                    right_buffer[i] = bitcrusher.process(right_buffer[i], time, sample_count);
                                }
                            }
                            5 => {
                                // Distortion
                                if let Some(ref mut distortion) = self.distortion {
                                    left_buffer[i] = distortion.process(left_buffer[i], time, sample_count);
                                    right_buffer[i] = distortion.process(right_buffer[i], time, sample_count);
                                }
                            }
                            9 => {
                                // Ring Modulator
                                if let Some(ref mut ring_mod) = self.ring_mod {
                                    left_buffer[i] = ring_mod.process(left_buffer[i], sample_rate, time, sample_count);
                                    right_buffer[i] = ring_mod.process(right_buffer[i], sample_rate, time, sample_count);
                                }
                            }
                            10 => {
                                // Tremolo
                                if let Some(ref mut tremolo) = self.tremolo {
                                    left_buffer[i] = tremolo.process(left_buffer[i], sample_rate, time, sample_count);
                                    right_buffer[i] = tremolo.process(right_buffer[i], sample_rate, time, sample_count);
                                }
                            }
                            14 => {
                                // ParametricEQ
                                if let Some(ref mut parametric_eq) = self.parametric_eq {
                                    left_buffer[i] = parametric_eq.process(left_buffer[i], time, sample_count as usize);
                                    right_buffer[i] = parametric_eq.process(right_buffer[i], time, sample_count as usize);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Reinterleave L/R channels back into stereo buffer
        for (i, frame) in buffer.chunks_exact_mut(2).enumerate() {
            frame[0] = left_buffer[i];
            frame[1] = right_buffer[i];
        }
    }

    /// Add EQ effect (builder pattern)
    pub fn with_eq(mut self, eq: EQ) -> Self {
        self.eq = Some(eq);
        self.compute_effect_order();
        self
    }

    /// Add compressor effect (builder pattern)
    pub fn with_compressor(mut self, compressor: Compressor) -> Self {
        self.compressor = Some(compressor);
        self.compute_effect_order();
        self
    }

    /// Add gate effect (builder pattern)
    pub fn with_gate(mut self, gate: Gate) -> Self {
        self.gate = Some(gate);
        self.compute_effect_order();
        self
    }

    /// Add saturation effect (builder pattern)
    pub fn with_saturation(mut self, saturation: Saturation) -> Self {
        self.saturation = Some(saturation);
        self.compute_effect_order();
        self
    }

    /// Add bitcrusher effect (builder pattern)
    pub fn with_bitcrusher(mut self, bitcrusher: BitCrusher) -> Self {
        self.bitcrusher = Some(bitcrusher);
        self.compute_effect_order();
        self
    }

    /// Add distortion effect (builder pattern)
    pub fn with_distortion(mut self, distortion: Distortion) -> Self {
        self.distortion = Some(distortion);
        self.compute_effect_order();
        self
    }

    /// Add chorus effect (builder pattern)
    pub fn with_chorus(mut self, chorus: Chorus) -> Self {
        self.chorus = Some(chorus);
        self.compute_effect_order();
        self
    }

    /// Add phaser effect (builder pattern)
    pub fn with_phaser(mut self, phaser: Phaser) -> Self {
        self.phaser = Some(phaser);
        self.compute_effect_order();
        self
    }

    /// Add flanger effect (builder pattern)
    pub fn with_flanger(mut self, flanger: Flanger) -> Self {
        self.flanger = Some(flanger);
        self.compute_effect_order();
        self
    }

    /// Add ring modulator effect (builder pattern)
    pub fn with_ring_mod(mut self, ring_mod: RingModulator) -> Self {
        self.ring_mod = Some(ring_mod);
        self.compute_effect_order();
        self
    }

    /// Add tremolo effect (builder pattern)
    pub fn with_tremolo(mut self, tremolo: Tremolo) -> Self {
        self.tremolo = Some(tremolo);
        self.compute_effect_order();
        self
    }

    /// Add auto-pan effect (builder pattern)
    pub fn with_autopan(mut self, autopan: AutoPan) -> Self {
        self.autopan = Some(autopan);
        // Note: AutoPan not added to effect_order, handled separately
        self
    }

    /// Add delay effect (builder pattern)
    pub fn with_delay(mut self, delay: Delay) -> Self {
        self.delay = Some(delay);
        self.compute_effect_order();
        self
    }

    /// Add reverb effect (builder pattern)
    pub fn with_reverb(mut self, reverb: Reverb) -> Self {
        self.reverb = Some(reverb);
        self.compute_effect_order();
        self
    }

    /// Add limiter effect (builder pattern)
    pub fn with_limiter(mut self, limiter: Limiter) -> Self {
        self.limiter = Some(limiter);
        self.compute_effect_order();
        self
    }

    /// Add parametric EQ effect (builder pattern)
    pub fn with_parametric_eq(mut self, parametric_eq: ParametricEQ) -> Self {
        self.parametric_eq = Some(parametric_eq);
        self.compute_effect_order();
        self
    }

    /// Add phase vocoder effect (builder pattern)
    ///
    /// Phase vocoder provides high-quality time-stretching and pitch-shifting.
    ///
    /// **Note**: This is a block-based effect. Use `process_mono_block` or
    /// `process_stereo_block` for best results.
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, PhaseVocoder};
    /// let mut chain = EffectChain::new();
    /// let vocoder = PhaseVocoder::new(1.0, 7.0, 44100.0); // 1.0x speed, perfect fifth up
    /// chain = chain.with_phase_vocoder(vocoder);
    /// ```
    pub fn with_phase_vocoder(mut self, phase_vocoder: PhaseVocoder) -> Self {
        self.phase_vocoder = Some(phase_vocoder);
        self.compute_effect_order();
        self
    }

    /// Add a spectral freeze effect to the chain
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralFreeze};
    /// let mut chain = EffectChain::new();
    /// let freeze = SpectralFreeze::new(true, 0.75, 44100.0); // Frozen, 75% wet
    /// chain = chain.with_spectral_freeze(freeze);
    /// ```
    pub fn with_spectral_freeze(mut self, spectral_freeze: SpectralFreeze) -> Self {
        self.spectral_freeze = Some(spectral_freeze);
        self.compute_effect_order();
        self
    }

    /// Add a spectral gate effect to the chain
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralGate};
    /// let mut chain = EffectChain::new();
    /// let gate = SpectralGate::new(-40.0, 0.001, 0.050, 0.0, 44100.0);
    /// chain = chain.with_spectral_gate(gate);
    /// ```
    pub fn with_spectral_gate(mut self, spectral_gate: SpectralGate) -> Self {
        self.spectral_gate = Some(spectral_gate);
        self.compute_effect_order();
        self
    }

    /// Add a spectral compressor effect to the chain
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralCompressor};
    /// let mut chain = EffectChain::new();
    /// let comp = SpectralCompressor::new(-20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
    /// chain = chain.with_spectral_compressor(comp);
    /// ```
    pub fn with_spectral_compressor(mut self, spectral_compressor: SpectralCompressor) -> Self {
        self.spectral_compressor = Some(spectral_compressor);
        self.compute_effect_order();
        self
    }

    /// Add a spectral robotize effect to the chain
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralRobotize};
    /// let mut chain = EffectChain::new();
    /// let robotize = SpectralRobotize::new(0.0, 1.0, 44100.0); // Full robotization
    /// chain = chain.with_spectral_robotize(robotize);
    /// ```
    pub fn with_spectral_robotize(mut self, spectral_robotize: SpectralRobotize) -> Self {
        self.spectral_robotize = Some(spectral_robotize);
        self.compute_effect_order();
        self
    }

    /// Add spectral delay with frequency-dependent delay times
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralDelay};
    /// let mut chain = EffectChain::new();
    /// let delay = SpectralDelay::new(200.0, 0.5, 1.0, 0.5, 44100.0);
    /// chain = chain.with_spectral_delay(delay);
    /// ```
    pub fn with_spectral_delay(mut self, spectral_delay: SpectralDelay) -> Self {
        self.spectral_delay = Some(spectral_delay);
        self.compute_effect_order();
        self
    }

    /// Add spectral filter for frequency-domain filtering
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralFilter, FilterType};
    /// let mut chain = EffectChain::new();
    /// let filter = SpectralFilter::new(FilterType::LowPass, 1000.0, 2.0, 1.0, 44100.0);
    /// chain = chain.with_spectral_filter(filter);
    /// ```
    pub fn with_spectral_filter(mut self, spectral_filter: SpectralFilter) -> Self {
        self.spectral_filter = Some(spectral_filter);
        self.compute_effect_order();
        self
    }

    /// Add spectral blur for temporal smoothing in frequency domain
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralBlur};
    /// let mut chain = EffectChain::new();
    /// let blur = SpectralBlur::new(0.5, 0.3, 1.0, 44100.0);
    /// chain = chain.with_spectral_blur(blur);
    /// ```
    pub fn with_spectral_blur(mut self, spectral_blur: SpectralBlur) -> Self {
        self.spectral_blur = Some(spectral_blur);
        self.compute_effect_order();
        self
    }

    /// Add spectral shift for frequency shifting
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralShift};
    /// let mut chain = EffectChain::new();
    /// let shift = SpectralShift::subtle();
    /// chain = chain.with_spectral_shift(shift);
    /// ```
    pub fn with_spectral_shift(mut self, spectral_shift: SpectralShift) -> Self {
        self.spectral_shift = Some(spectral_shift);
        self.compute_effect_order();
        self
    }

    /// Add spectral exciter for harmonic enhancement
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralExciter};
    /// let mut chain = EffectChain::new();
    /// let exciter = SpectralExciter::gentle();
    /// chain = chain.with_spectral_exciter(exciter);
    /// ```
    pub fn with_spectral_exciter(mut self, spectral_exciter: SpectralExciter) -> Self {
        self.spectral_exciter = Some(spectral_exciter);
        self.compute_effect_order();
        self
    }

    /// Add spectral invert for frequency spectrum reversal
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralInvert};
    /// let mut chain = EffectChain::new();
    /// let invert = SpectralInvert::full();
    /// chain = chain.with_spectral_invert(invert);
    /// ```
    pub fn with_spectral_invert(mut self, spectral_invert: SpectralInvert) -> Self {
        self.spectral_invert = Some(spectral_invert);
        self.compute_effect_order();
        self
    }

    /// Add spectral widen for stereo widening via phase manipulation
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralWiden};
    /// let mut chain = EffectChain::new();
    /// let widen = SpectralWiden::wide();
    /// chain = chain.with_spectral_widen(widen);
    /// ```
    pub fn with_spectral_widen(mut self, spectral_widen: SpectralWiden) -> Self {
        self.spectral_widen = Some(spectral_widen);
        self.compute_effect_order();
        self
    }

    /// Add spectral morph for morphing spectrum toward target shapes
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralMorph};
    /// let mut chain = EffectChain::new();
    /// let morph = SpectralMorph::robot();
    /// chain = chain.with_spectral_morph(morph);
    /// ```
    pub fn with_spectral_morph(mut self, spectral_morph: SpectralMorph) -> Self {
        self.spectral_morph = Some(spectral_morph);
        self.compute_effect_order();
        self
    }

    /// Add spectral dynamics for frequency-dependent compression/expansion
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralDynamics};
    /// let mut chain = EffectChain::new();
    /// let dynamics = SpectralDynamics::gentle();
    /// chain = chain.with_spectral_dynamics(dynamics);
    /// ```
    pub fn with_spectral_dynamics(mut self, spectral_dynamics: SpectralDynamics) -> Self {
        self.spectral_dynamics = Some(spectral_dynamics);
        self.compute_effect_order();
        self
    }

    /// Add spectral scramble for glitchy frequency bin randomization
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{EffectChain, SpectralScramble};
    /// let mut chain = EffectChain::new();
    /// let scramble = SpectralScramble::glitch();
    /// chain = chain.with_spectral_scramble(scramble);
    /// ```
    pub fn with_spectral_scramble(mut self, spectral_scramble: SpectralScramble) -> Self {
        self.spectral_scramble = Some(spectral_scramble);
        self.compute_effect_order();
        self
    }

    pub fn with_formant_shifter(mut self, formant_shifter: FormantShifter) -> Self {
        self.formant_shifter = Some(formant_shifter);
        self.compute_effect_order();
        self
    }

    pub fn with_spectral_harmonizer(mut self, spectral_harmonizer: SpectralHarmonizer) -> Self {
        self.spectral_harmonizer = Some(spectral_harmonizer);
        self.compute_effect_order();
        self
    }

    pub fn with_spectral_resonator(mut self, spectral_resonator: SpectralResonator) -> Self {
        self.spectral_resonator = Some(spectral_resonator);
        self.compute_effect_order();
        self
    }

    pub fn with_spectral_panner(mut self, spectral_panner: SpectralPanner) -> Self {
        self.spectral_panner = Some(spectral_panner);
        self.compute_effect_order();
        self
    }

    pub fn with_convolution_reverb(mut self, convolution_reverb: ConvolutionReverb) -> Self {
        self.convolution_reverb = Some(convolution_reverb);
        self.compute_effect_order();
        self
    }
}

impl Default for EffectChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay() {
        let mut delay = Delay::new(0.01, 0.5, 0.5);
        let output = delay.process(1.0, 0.0, 0);
        assert!(output >= 0.0 && output <= 1.0);
    }

    #[test]
    fn test_reverb() {
        let mut reverb = Reverb::new(0.5, 0.5, 0.3);
        let output = reverb.process(1.0, 0.0, 0);
        assert!(output.is_finite());
    }

    #[test]
    fn test_distortion() {
        let mut dist = Distortion::new(5.0, 1.0);
        let output = dist.process(0.5, 0.0, 0);
        assert!(output >= -1.0 && output <= 1.0);
    }

    #[test]
    fn test_eq_band_creation() {
        let band = EQBand::new(1000.0, 6.0, 2.0);
        assert_eq!(band.frequency, 1000.0);
        assert_eq!(band.gain_db, 6.0);
        assert_eq!(band.q, 2.0);
        assert!(band.enabled);
    }

    // Note: EQBand::process() is private - it's only called internally by ParametricEQ
    // Testing is done via ParametricEQ instead

    #[test]
    fn test_parametric_eq_creation() {
        let eq = ParametricEQ::new();
        assert_eq!(eq.bands.len(), 0);
    }

    #[test]
    fn test_parametric_eq_add_band() {
        let eq = ParametricEQ::new()
            .band(100.0, -6.0, 1.0)
            .band(3000.0, 4.0, 2.0);

        assert_eq!(eq.bands.len(), 2);
    }

    #[test]
    fn test_parametric_eq_process() {
        let mut eq = ParametricEQ::new()
            .band(1000.0, 3.0, 2.0);

        let output = eq.process(0.5, 0.0, 0);
        assert!(output.is_finite());
    }

    #[test]
    fn test_parametric_eq_preset() {
        let eq = ParametricEQ::new().preset(EQPreset::VocalClarity);
        assert_eq!(eq.bands.len(), 4);
    }

    #[test]
    fn test_parametric_eq_enable_disable_band() {
        let mut eq = ParametricEQ::new()
            .band(1000.0, 3.0, 2.0);

        eq.enable_band(0, false);
        assert!(!eq.bands[0].enabled);

        eq.enable_band(0, true);
        assert!(eq.bands[0].enabled);
    }

    #[test]
    fn test_parametric_eq_reset() {
        let mut eq = ParametricEQ::new()
            .band(1000.0, 3.0, 2.0);

        // Process some samples to build up state
        for _ in 0..10 {
            eq.process(0.5, 0.0, 0);
        }

        // Reset should clear state (no panic = success)
        // Note: x1, y1 are private fields, so we can't directly test them,
        // but reset() should clear internal filter state
        eq.reset();
    }
}
