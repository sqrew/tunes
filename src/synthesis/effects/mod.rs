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

// Re-export all effect types
pub use delay::Delay;
pub use reverb::Reverb;
pub use distortion::{Distortion, BitCrusher, Saturation};
pub use dynamics::{Compressor, CompressorBand, Gate, Limiter, SidechainSource, ResolvedSidechainSource};
pub use modulation::{Chorus, Phaser, Flanger, RingModulator, Tremolo};
pub use spatial::AutoPan;
pub use eq::{EQ, EQBand, ParametricEQ, EQPreset};

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
///     .with_eq(EQ::new(1.0, 1.0, 1.0))
///     .with_compressor(Compressor::new(0.5, 4.0, 0.01, 0.1, 1.0))
///     .with_reverb(Reverb::hall());
/// ```
#[derive(Debug, Clone)]
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
    pub limiter: Option<Limiter>,
    pub parametric_eq: Option<ParametricEQ>,

    // Pre-computed effect processing order (cached for performance)
    // Effect IDs: 0=EQ, 1=Compressor, 2=Gate, 3=Saturation, 4=BitCrusher, 5=Distortion,
    //             6=Chorus, 7=Phaser, 8=Flanger, 9=RingMod, 10=Tremolo,
    //             11=Delay, 12=Reverb, 13=Limiter, 14=ParametricEQ
    // (AutoPan excluded - handled separately in stereo stage)
    pub(crate) effect_order: Vec<u8>,
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
            limiter: None,
            parametric_eq: None,
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

        // Sort by priority (lower = earlier in chain)
        effects.sort_by_key(|&(priority, _)| priority);

        // Extract just the effect IDs
        self.effect_order = effects.into_iter().map(|(_, id)| id).collect();
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
        let time_delta = 1.0 / sample_rate;

        // Process each stereo frame (2 samples: left + right)
        for (i, frame) in buffer.chunks_mut(2).enumerate() {
            if frame.len() == 2 {
                let current_time = time + (i as f32 * time_delta);
                let current_sample_count = sample_count + i as u64;

                let (left, right) = self.process_stereo(
                    frame[0],
                    frame[1],
                    sample_rate,
                    current_time,
                    current_sample_count,
                    sidechain_envelope,
                );

                frame[0] = left;
                frame[1] = right;
            }
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
