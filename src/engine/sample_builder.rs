//! Builder for ergonomic one-shot sample playback with effects.
//!
//! Created by `AudioEngine::play_sample()`. Automatically plays on drop (fire-and-forget).

use super::AudioEngine;
use crate::composition::{Composition, Tempo};
use crate::error::TunesError;
use crate::synthesis::spatial::SpatialPosition;

use super::commands::SoundId;

/// Sample transformation operations that can be applied before playback
#[derive(Clone)]
pub(crate) enum SampleTransform {
    Normalize,
    Gain(f32),
    Reverse,
    FadeIn(f32),
    FadeOut(f32),
    TimeStretch(f32),
    PitchShift(f32),
}

/// Builder for ergonomic one-shot sample playback with effects
///
/// Created by `AudioEngine::play_sample()`. Automatically plays on drop (fire-and-forget).
///
/// # Example
/// ```no_run
/// # use tunes::prelude::*;
/// # fn main() -> anyhow::Result<()> {
/// let engine = AudioEngine::new()?;
///
/// // Fire and forget with effects!
/// engine.play_sample("explosion.wav")
///     .volume(0.8)
///     .speed(1.2)
///     .reverb(Reverb::new(0.5, 0.3, 0.2));
///
/// // Auto-plays when builder drops
/// # Ok(())
/// # }
/// ```
pub struct SamplePlaybackBuilder<'a> {
    pub(crate) engine: &'a AudioEngine,
    pub(crate) path: String,
    pub(crate) volume: f32,
    pub(crate) pan: f32,
    pub(crate) speed: f32,
    pub(crate) spatial_position: Option<SpatialPosition>,
    pub(crate) filter: Option<crate::synthesis::filter::Filter>,
    pub(crate) effects: crate::synthesis::effects::EffectChain,
    pub(crate) sample_transforms: Vec<SampleTransform>,
}

impl<'a> SamplePlaybackBuilder<'a> {
    /// Create a new builder (internal - use AudioEngine::play_sample())
    pub(crate) fn new(engine: &'a AudioEngine, path: impl Into<String>) -> Self {
        Self {
            engine,
            path: path.into(),
            volume: 1.0,
            pan: 0.0,
            speed: 1.0,
            spatial_position: None,
            filter: None,
            effects: crate::synthesis::effects::EffectChain::new(),
            sample_transforms: Vec::new(),
        }
    }

    /// Set playback volume (0.0 to 2.0, default: 1.0)
    pub fn volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 2.0);
        self
    }

    /// Set stereo pan (-1.0 = left, 0.0 = center, 1.0 = right, default: 0.0)
    pub fn pan(mut self, pan: f32) -> Self {
        self.pan = pan.clamp(-1.0, 1.0);
        self
    }

    /// Set playback speed/pitch (1.0 = normal, 2.0 = double speed, 0.5 = half speed)
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.01); // Prevent zero/negative speeds
        self
    }

    /// Set 3D spatial position (x, y, z coordinates)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # let engine = AudioEngine::new().unwrap();
    /// // Position sound 5 meters to the right and 3 meters forward
    /// engine.play_sample("footstep.wav")
    ///     .spatial(5.0, 0.0, 3.0);
    /// ```
    pub fn spatial(mut self, x: f32, y: f32, z: f32) -> Self {
        self.spatial_position = Some(SpatialPosition::new(x, y, z));
        self
    }

    /// Apply a filter to the sample
    pub fn filter(mut self, filter: crate::synthesis::filter::Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Add reverb effect
    pub fn reverb(mut self, reverb: crate::synthesis::effects::Reverb) -> Self {
        self.effects.reverb = Some(reverb);
        self.effects.compute_effect_order();
        self
    }

    /// Add delay effect
    pub fn delay(mut self, delay: crate::synthesis::effects::Delay) -> Self {
        self.effects.delay = Some(delay);
        self.effects.compute_effect_order();
        self
    }

    /// Add distortion effect
    pub fn distortion(mut self, distortion: crate::synthesis::effects::Distortion) -> Self {
        self.effects.distortion = Some(distortion);
        self.effects.compute_effect_order();
        self
    }

    /// Add chorus effect
    pub fn chorus(mut self, chorus: crate::synthesis::effects::Chorus) -> Self {
        self.effects.chorus = Some(chorus);
        self.effects.compute_effect_order();
        self
    }

    /// Add phaser effect
    pub fn phaser(mut self, phaser: crate::synthesis::effects::Phaser) -> Self {
        self.effects.phaser = Some(phaser);
        self.effects.compute_effect_order();
        self
    }

    /// Add flanger effect
    pub fn flanger(mut self, flanger: crate::synthesis::effects::Flanger) -> Self {
        self.effects.flanger = Some(flanger);
        self.effects.compute_effect_order();
        self
    }

    /// Add tremolo effect
    pub fn tremolo(mut self, tremolo: crate::synthesis::effects::Tremolo) -> Self {
        self.effects.tremolo = Some(tremolo);
        self.effects.compute_effect_order();
        self
    }

    /// Add bitcrusher effect
    pub fn bitcrusher(mut self, bitcrusher: crate::synthesis::effects::BitCrusher) -> Self {
        self.effects.bitcrusher = Some(bitcrusher);
        self.effects.compute_effect_order();
        self
    }

    /// Add saturation effect
    pub fn saturation(mut self, saturation: crate::synthesis::effects::Saturation) -> Self {
        self.effects.saturation = Some(saturation);
        self.effects.compute_effect_order();
        self
    }

    /// Add compressor effect
    pub fn compressor(mut self, compressor: crate::synthesis::effects::Compressor) -> Self {
        self.effects.compressor = Some(compressor);
        self.effects.compute_effect_order();
        self
    }

    /// Add limiter effect
    pub fn limiter(mut self, limiter: crate::synthesis::effects::Limiter) -> Self {
        self.effects.limiter = Some(limiter);
        self.effects.compute_effect_order();
        self
    }

    /// Add convolution reverb effect
    pub fn convolution_reverb(
        mut self,
        conv_reverb: crate::synthesis::effects::ConvolutionReverb,
    ) -> Self {
        self.effects.convolution_reverb = Some(conv_reverb);
        self.effects.compute_effect_order();
        self
    }

    /// Add EQ effect
    pub fn eq(mut self, eq: crate::synthesis::effects::EQ) -> Self {
        self.effects.eq = Some(eq);
        self.effects.compute_effect_order();
        self
    }

    /// Add ring modulator effect
    pub fn ring_mod(mut self, ring_mod: crate::synthesis::effects::RingModulator) -> Self {
        self.effects.ring_mod = Some(ring_mod);
        self.effects.compute_effect_order();
        self
    }

    /// Add auto-pan effect
    pub fn autopan(mut self, autopan: crate::synthesis::effects::AutoPan) -> Self {
        self.effects.autopan = Some(autopan);
        self.effects.compute_effect_order();
        self
    }

    /// Add gate effect
    pub fn gate(mut self, gate: crate::synthesis::effects::Gate) -> Self {
        self.effects.gate = Some(gate);
        self.effects.compute_effect_order();
        self
    }

    // ========== Spectral Effects ==========

    /// Add phase vocoder effect for time-stretching and pitch-shifting
    pub fn phase_vocoder(
        mut self,
        phase_vocoder: crate::synthesis::effects::PhaseVocoder,
    ) -> Self {
        self.effects.phase_vocoder = Some(phase_vocoder);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral freeze effect
    pub fn spectral_freeze(
        mut self,
        spectral_freeze: crate::synthesis::effects::SpectralFreeze,
    ) -> Self {
        self.effects.spectral_freeze = Some(spectral_freeze);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral gate effect for frequency-selective noise gating
    pub fn spectral_gate(
        mut self,
        spectral_gate: crate::synthesis::effects::SpectralGate,
    ) -> Self {
        self.effects.spectral_gate = Some(spectral_gate);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral compressor effect for frequency-selective compression
    pub fn spectral_compressor(
        mut self,
        spectral_compressor: crate::synthesis::effects::SpectralCompressor,
    ) -> Self {
        self.effects.spectral_compressor = Some(spectral_compressor);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral robotize effect for robotic voice transformation
    pub fn spectral_robotize(
        mut self,
        spectral_robotize: crate::synthesis::effects::SpectralRobotize,
    ) -> Self {
        self.effects.spectral_robotize = Some(spectral_robotize);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral delay effect for frequency-dependent delay
    pub fn spectral_delay(
        mut self,
        spectral_delay: crate::synthesis::effects::SpectralDelay,
    ) -> Self {
        self.effects.spectral_delay = Some(spectral_delay);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral filter effect for frequency-domain filtering
    pub fn spectral_filter(
        mut self,
        spectral_filter: crate::synthesis::effects::SpectralFilter,
    ) -> Self {
        self.effects.spectral_filter = Some(spectral_filter);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral blur effect for frequency smearing
    pub fn spectral_blur(
        mut self,
        spectral_blur: crate::synthesis::effects::SpectralBlur,
    ) -> Self {
        self.effects.spectral_blur = Some(spectral_blur);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral shift effect for frequency shifting
    pub fn spectral_shift(
        mut self,
        spectral_shift: crate::synthesis::effects::SpectralShift,
    ) -> Self {
        self.effects.spectral_shift = Some(spectral_shift);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral exciter effect for harmonic enhancement
    pub fn spectral_exciter(
        mut self,
        spectral_exciter: crate::synthesis::effects::SpectralExciter,
    ) -> Self {
        self.effects.spectral_exciter = Some(spectral_exciter);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral invert effect for frequency spectrum reversal
    pub fn spectral_invert(
        mut self,
        spectral_invert: crate::synthesis::effects::SpectralInvert,
    ) -> Self {
        self.effects.spectral_invert = Some(spectral_invert);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral widen effect for stereo widening
    pub fn spectral_widen(
        mut self,
        spectral_widen: crate::synthesis::effects::SpectralWiden,
    ) -> Self {
        self.effects.spectral_widen = Some(spectral_widen);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral morph effect for morphing spectrum toward target shapes
    pub fn spectral_morph(
        mut self,
        spectral_morph: crate::synthesis::effects::SpectralMorph,
    ) -> Self {
        self.effects.spectral_morph = Some(spectral_morph);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral dynamics effect for frequency-dependent compression/expansion
    pub fn spectral_dynamics(
        mut self,
        spectral_dynamics: crate::synthesis::effects::SpectralDynamics,
    ) -> Self {
        self.effects.spectral_dynamics = Some(spectral_dynamics);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral scramble effect for glitchy frequency bin randomization
    pub fn spectral_scramble(
        mut self,
        spectral_scramble: crate::synthesis::effects::SpectralScramble,
    ) -> Self {
        self.effects.spectral_scramble = Some(spectral_scramble);
        self.effects.compute_effect_order();
        self
    }

    /// Add formant shifter effect for vocal character transformation
    pub fn formant_shifter(
        mut self,
        formant_shifter: crate::synthesis::effects::FormantShifter,
    ) -> Self {
        self.effects.formant_shifter = Some(formant_shifter);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral harmonizer effect for pitch-shifted harmonies
    pub fn spectral_harmonizer(
        mut self,
        spectral_harmonizer: crate::synthesis::effects::SpectralHarmonizer,
    ) -> Self {
        self.effects.spectral_harmonizer = Some(spectral_harmonizer);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral resonator effect for resonant frequency peaks
    pub fn spectral_resonator(
        mut self,
        spectral_resonator: crate::synthesis::effects::SpectralResonator,
    ) -> Self {
        self.effects.spectral_resonator = Some(spectral_resonator);
        self.effects.compute_effect_order();
        self
    }

    /// Add spectral panner effect for frequency-based spatial positioning
    pub fn spectral_panner(
        mut self,
        spectral_panner: crate::synthesis::effects::SpectralPanner,
    ) -> Self {
        self.effects.spectral_panner = Some(spectral_panner);
        self.effects.compute_effect_order();
        self
    }

    // ========== Sample Transformation Methods ==========

    /// Normalize the sample to peak amplitude
    ///
    /// Scales the sample so the loudest point reaches ±1.0 without clipping.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # let engine = AudioEngine::new().unwrap();
    /// engine.play_sample("quiet_sound.wav")
    ///     .normalize()
    ///     .volume(0.8);
    /// ```
    pub fn normalize(mut self) -> Self {
        self.sample_transforms.push(SampleTransform::Normalize);
        self
    }

    /// Apply gain to the sample data itself (pre-effect)
    ///
    /// This is different from `.volume()` - it modifies the sample data before effects,
    /// while `.volume()` adjusts the track volume after effects.
    ///
    /// # Arguments
    /// * `gain` - Gain multiplier (1.0 = unchanged, 0.5 = half, 2.0 = double)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # let engine = AudioEngine::new().unwrap();
    /// engine.play_sample("quiet.wav")
    ///     .gain(2.0)  // Double the sample amplitude
    ///     .reverb(Reverb::hall());
    /// ```
    pub fn gain(mut self, gain: f32) -> Self {
        self.sample_transforms.push(SampleTransform::Gain(gain));
        self
    }

    /// Reverse the sample
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # let engine = AudioEngine::new().unwrap();
    /// // Play backwards cymbal
    /// engine.play_sample("cymbal.wav")
    ///     .reverse();
    /// ```
    pub fn reverse(mut self) -> Self {
        self.sample_transforms.push(SampleTransform::Reverse);
        self
    }

    /// Fade in over a duration
    ///
    /// # Arguments
    /// * `duration` - Fade in duration in seconds
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # let engine = AudioEngine::new().unwrap();
    /// engine.play_sample("ambience.wav")
    ///     .fade_in(2.0);  // 2 second fade in
    /// ```
    pub fn fade_in(mut self, duration: f32) -> Self {
        self.sample_transforms
            .push(SampleTransform::FadeIn(duration));
        self
    }

    /// Fade out over a duration
    ///
    /// # Arguments
    /// * `duration` - Fade out duration in seconds
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # let engine = AudioEngine::new().unwrap();
    /// engine.play_sample("music.wav")
    ///     .fade_out(3.0);  // 3 second fade out
    /// ```
    pub fn fade_out(mut self, duration: f32) -> Self {
        self.sample_transforms
            .push(SampleTransform::FadeOut(duration));
        self
    }

    /// Time-stretch the sample without changing pitch
    ///
    /// Uses WSOLA algorithm to change duration while preserving pitch.
    ///
    /// # Arguments
    /// * `factor` - Time stretch factor (0.5 = half duration, 2.0 = double duration)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # let engine = AudioEngine::new().unwrap();
    /// // Make footstep twice as long without changing pitch
    /// engine.play_sample("footstep.wav")
    ///     .time_stretch(2.0);
    /// ```
    pub fn time_stretch(mut self, factor: f32) -> Self {
        self.sample_transforms
            .push(SampleTransform::TimeStretch(factor));
        self
    }

    /// Pitch shift the sample
    ///
    /// Changes the pitch while maintaining duration using time-stretch + resample.
    ///
    /// # Arguments
    /// * `semitones` - Pitch shift in semitones (12 = one octave up, -12 = one octave down)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # let engine = AudioEngine::new().unwrap();
    /// // Play voice one octave higher
    /// engine.play_sample("voice.wav")
    ///     .pitch_shift(12.0);
    /// ```
    pub fn pitch_shift(mut self, semitones: f32) -> Self {
        self.sample_transforms
            .push(SampleTransform::PitchShift(semitones));
        self
    }

    /// Internal method to execute playback
    pub(crate) fn execute_play(&self) -> crate::error::Result<SoundId> {
        use crate::synthesis::Sample;

        // Check cache first, load if not present (lock-free with DashMap)
        let mut sample = if let Some(cached) = self.engine.sample_cache.get(&self.path) {
            cached.clone()
        } else {
            let loaded = Sample::from_file(&self.path).map_err(|e| {
                TunesError::AudioEngineError(format!(
                    "Failed to load sample '{}': {}",
                    self.path, e
                ))
            })?;
            self.engine
                .sample_cache
                .entry(self.path.clone())
                .or_insert(loaded)
                .clone()
        };

        // Apply sample transformations in order
        for transform in &self.sample_transforms {
            sample = match transform {
                SampleTransform::Normalize => sample.normalize(),
                SampleTransform::Gain(gain) => sample.with_gain(*gain),
                SampleTransform::Reverse => sample.reverse(),
                SampleTransform::FadeIn(duration) => sample.with_fade_in(*duration),
                SampleTransform::FadeOut(duration) => sample.with_fade_out(*duration),
                SampleTransform::TimeStretch(factor) => sample.time_stretch(*factor),
                SampleTransform::PitchShift(semitones) => sample.pitch_shift(*semitones),
            };
        }

        // Create composition with all the builder settings
        let mut comp = Composition::new(Tempo::new(120.0));
        let mut track = comp.track("_oneshot");

        // Apply all settings
        track = track.volume(self.volume).pan(self.pan);

        if let Some(pos) = self.spatial_position {
            track = track.spatial_position(pos.position.x, pos.position.y, pos.position.z);
        }

        if let Some(filter) = &self.filter {
            track = track.filter(*filter);
        }

        // Apply effects
        if self.effects.reverb.is_some() {
            track = track.reverb(self.effects.reverb.clone().unwrap());
        }
        if self.effects.delay.is_some() {
            track = track.delay(self.effects.delay.clone().unwrap());
        }
        if self.effects.distortion.is_some() {
            track = track.distortion(self.effects.distortion.clone().unwrap());
        }
        if self.effects.chorus.is_some() {
            track = track.chorus(self.effects.chorus.clone().unwrap());
        }
        if self.effects.phaser.is_some() {
            track = track.phaser(self.effects.phaser.clone().unwrap());
        }
        if self.effects.flanger.is_some() {
            track = track.flanger(self.effects.flanger.clone().unwrap());
        }
        if self.effects.tremolo.is_some() {
            track = track.tremolo(self.effects.tremolo.clone().unwrap());
        }
        if self.effects.bitcrusher.is_some() {
            track = track.bitcrusher(self.effects.bitcrusher.clone().unwrap());
        }
        if self.effects.saturation.is_some() {
            track = track.saturation(self.effects.saturation.clone().unwrap());
        }
        if self.effects.compressor.is_some() {
            track = track.compressor(self.effects.compressor.clone().unwrap());
        }
        if self.effects.limiter.is_some() {
            track = track.limiter(self.effects.limiter.clone().unwrap());
        }
        if self.effects.convolution_reverb.is_some() {
            track = track.convolution_reverb(self.effects.convolution_reverb.clone().unwrap());
        }
        if self.effects.eq.is_some() {
            track = track.eq(self.effects.eq.clone().unwrap());
        }
        if self.effects.ring_mod.is_some() {
            track = track.ring_mod(self.effects.ring_mod.clone().unwrap());
        }
        if self.effects.autopan.is_some() {
            track = track.autopan(self.effects.autopan.clone().unwrap());
        }
        if self.effects.gate.is_some() {
            track = track.gate(self.effects.gate.clone().unwrap());
        }

        // Apply spectral effects
        if self.effects.phase_vocoder.is_some() {
            track = track.phase_vocoder(self.effects.phase_vocoder.clone().unwrap());
        }
        if self.effects.spectral_freeze.is_some() {
            track = track.spectral_freeze(self.effects.spectral_freeze.clone().unwrap());
        }
        if self.effects.spectral_gate.is_some() {
            track = track.spectral_gate(self.effects.spectral_gate.clone().unwrap());
        }
        if self.effects.spectral_compressor.is_some() {
            track = track.spectral_compressor(self.effects.spectral_compressor.clone().unwrap());
        }
        if self.effects.spectral_robotize.is_some() {
            track = track.spectral_robotize(self.effects.spectral_robotize.clone().unwrap());
        }
        if self.effects.spectral_delay.is_some() {
            track = track.spectral_delay(self.effects.spectral_delay.clone().unwrap());
        }
        if self.effects.spectral_filter.is_some() {
            track = track.spectral_filter(self.effects.spectral_filter.clone().unwrap());
        }
        if self.effects.spectral_blur.is_some() {
            track = track.spectral_blur(self.effects.spectral_blur.clone().unwrap());
        }
        if self.effects.spectral_shift.is_some() {
            track = track.spectral_shift(self.effects.spectral_shift.clone().unwrap());
        }
        if self.effects.spectral_exciter.is_some() {
            track = track.spectral_exciter(self.effects.spectral_exciter.clone().unwrap());
        }
        if self.effects.spectral_invert.is_some() {
            track = track.spectral_invert(self.effects.spectral_invert.clone().unwrap());
        }
        if self.effects.spectral_widen.is_some() {
            track = track.spectral_widen(self.effects.spectral_widen.clone().unwrap());
        }
        if self.effects.spectral_morph.is_some() {
            track = track.spectral_morph(self.effects.spectral_morph.clone().unwrap());
        }
        if self.effects.spectral_dynamics.is_some() {
            track = track.spectral_dynamics(self.effects.spectral_dynamics.clone().unwrap());
        }
        if self.effects.spectral_scramble.is_some() {
            track = track.spectral_scramble(self.effects.spectral_scramble.clone().unwrap());
        }
        if self.effects.formant_shifter.is_some() {
            track = track.formant_shifter(self.effects.formant_shifter.clone().unwrap());
        }
        if self.effects.spectral_harmonizer.is_some() {
            track = track.spectral_harmonizer(self.effects.spectral_harmonizer.clone().unwrap());
        }
        if self.effects.spectral_resonator.is_some() {
            track = track.spectral_resonator(self.effects.spectral_resonator.clone().unwrap());
        }
        if self.effects.spectral_panner.is_some() {
            track = track.spectral_panner(self.effects.spectral_panner.clone().unwrap());
        }

        // Play the sample
        track.play_sample(&sample, self.speed);

        // Convert to mixer
        #[cfg(feature = "gpu")]
        let mut mixer = comp.into_mixer();
        #[cfg(not(feature = "gpu"))]
        let mixer = comp.into_mixer();

        #[cfg(feature = "gpu")]
        {
            if self.engine.enable_gpu_for_samples {
                mixer.enable_cache_and_gpu();
            }
        }

        self.engine.play_mixer_realtime(&mixer)
    }
}

impl<'a> Drop for SamplePlaybackBuilder<'a> {
    fn drop(&mut self) {
        // Fire and forget - play on drop, but report errors
        if let Err(e) = self.execute_play() {
            eprintln!("Failed to play sample '{}': {}", self.path, e);
        }
    }
}
