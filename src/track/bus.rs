//! Bus system for grouping and processing tracks
//!
//! Buses provide an intermediate mixing stage between individual tracks and the master output.
//! Each bus owns a collection of tracks, mixes them together, applies effects, and sends
//! the result to the master chain.
//!
//! # Signal Flow
//!
//! ```text
//! Track 1 ─┐
//! Track 2 ─┤→ Bus A (EffectChain) ─┐
//! Track 3 ─┘                        │
//!                                   ├→ Master (EffectChain) → Output
//! Track 4 ─┐                        │
//! Track 5 ─┤→ Bus B (EffectChain) ─┘
//! Track 6 ─┘
//! ```
//!
//! # Use Cases
//!
//! - **Grouping**: Apply effects to multiple tracks at once (e.g., all drums, all vocals)
//! - **Organization**: Keep related tracks together for easier mixing
//! - **Sidechaining**: Duck one bus based on another bus's signal
//! - **Parallel Processing**: Send different track groups through different effect chains
//!
//! # Example
//!
//! ```
//! # use tunes::prelude::*;
//! # use tunes::track::Bus;
//! let mut comp = Composition::new(Tempo::new(120.0));
//!
//! // Create tracks on different buses
//! comp.track("kick")
//!     .bus("drums")
//!     .drum(DrumType::Kick, 0.0);
//!
//! comp.track("lead")
//!     .bus("melody")
//!     .notes(&[C4, E4, G4], 0.5);
//! ```

use crate::synthesis::effects::{
    AutoPan, BitCrusher, Chorus, Compressor, Delay, Distortion, EQ, EffectChain, Flanger,
    FormantShifter, Gate, Limiter, ParametricEQ, PhaseVocoder, Phaser,
    Reverb, RingModulator, Saturation, SpectralFreeze, SpectralGate,
    SpectralCompressor, SpectralHarmonizer, SpectralPanner, SpectralResonator, SpectralRobotize,
    SpectralDelay, SpectralFilter, SpectralBlur, SpectralShift, SpectralExciter, SpectralInvert,
    SpectralWiden, SpectralMorph, SpectralDynamics, SpectralScramble, Tremolo,
};
use crate::track::Track;
use crate::track::ids::BusId;

/// A bus groups multiple tracks together for processing
///
/// Buses mix their tracks together, apply effects to the summed signal,
/// and send the result to the master output. This provides an intermediate
/// layer between individual tracks and the final mix.
#[derive(Debug, Clone)]
pub struct Bus {
    /// Unique integer ID for fast lookups (used in real-time audio path)
    pub id: BusId,

    /// Name of this bus (used for user-facing API)
    pub name: String,

    /// Tracks routed to this bus
    pub tracks: Vec<Track>,

    /// Effects applied to the bus mix
    pub effects: EffectChain,

    /// Bus volume (0.0 to 2.0, default: 1.0)
    pub volume: f32,

    /// Bus pan (-1.0 = left, 0.0 = center, 1.0 = right)
    pub pan: f32,

    /// Whether this bus is muted
    pub muted: bool,

    /// Whether this bus is soloed
    pub soloed: bool,

    /// Pre-allocated scratch buffer for block audio processing (reused each callback)
    pub(crate) scratch_buffer: Vec<f32>,
}

impl Bus {
    /// Create a new empty bus with a unique ID
    ///
    /// # Arguments
    /// * `id` - Unique bus identifier (assigned by BusIdGenerator)
    /// * `name` - Human-readable name for the bus
    pub fn new(id: BusId, name: String) -> Self {
        Self {
            id,
            name,
            tracks: Vec::new(),
            effects: EffectChain::new(),
            volume: 1.0,
            pan: 0.0,
            muted: false,
            soloed: false,
            scratch_buffer: Vec::new(),
        }
    }

    /// Set the volume of this bus
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 2.0);
        self
    }

    /// Set the pan of this bus
    pub fn with_pan(mut self, pan: f32) -> Self {
        self.pan = pan.clamp(-1.0, 1.0);
        self
    }

    /// Add a track to this bus
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    /// Get the total duration of all tracks in this bus
    pub fn total_duration(&self) -> f32 {
        self.tracks
            .iter()
            .map(|t| t.total_duration())
            .fold(0.0, f32::max)
    }

    /// Check if this bus has any tracks
    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    /// Get the number of tracks in this bus
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// Mix all tracks in this bus at a given time and sample rate, returning stereo output
    ///
    /// Note: This method is not currently used - track processing happens in Mixer::process_track().
    /// This is kept for potential future use when Track gets its own sample_at() method.
    #[allow(dead_code)]
    pub(crate) fn sample_at_future(
        &mut self,
        _time: f32,
        _sample_rate: f32,
        _sample_count: u64,
    ) -> (f32, f32) {
        // Placeholder for future when Track has sample_at()
        // For now, all track processing happens in Mixer::process_track()
        (0.0, 0.0)
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new(0, "default".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::notes::*;

    #[test]
    fn test_bus_creation() {
        let bus = Bus::new(0, "drums".to_string());
        assert_eq!(bus.name, "drums");
        assert_eq!(bus.tracks.len(), 0);
        assert_eq!(bus.volume, 1.0);
        assert_eq!(bus.pan, 0.0);
        assert!(!bus.muted);
        assert!(!bus.soloed);
    }

    #[test]
    fn test_bus_add_track() {
        let mut bus = Bus::new(0, "test".to_string());
        let mut track = Track::new();
        track.add_note(&[C4], 0.0, 1.0);

        bus.add_track(track);
        assert_eq!(bus.track_count(), 1);
        assert!(!bus.is_empty());
    }

    #[test]
    fn test_bus_total_duration() {
        let mut bus = Bus::new(0, "test".to_string());

        let mut track1 = Track::new();
        track1.add_note(&[C4], 0.0, 2.0); // Ends at 2.0

        let mut track2 = Track::new();
        track2.add_note(&[E4], 0.0, 4.0); // Ends at 4.0

        bus.add_track(track1);
        bus.add_track(track2);

        assert_eq!(bus.total_duration(), 4.0);
    }

    #[test]
    fn test_bus_with_volume() {
        let bus = Bus::new(0, "test".to_string()).with_volume(0.5);
        assert_eq!(bus.volume, 0.5);

        // Test clamping
        let loud_bus = Bus::new(1, "loud".to_string()).with_volume(5.0);
        assert_eq!(loud_bus.volume, 2.0);

        let silent_bus = Bus::new(2, "silent".to_string()).with_volume(-1.0);
        assert_eq!(silent_bus.volume, 0.0);
    }

    #[test]
    fn test_bus_with_pan() {
        let left_bus = Bus::new(0, "left".to_string()).with_pan(-0.5);
        assert_eq!(left_bus.pan, -0.5);

        let right_bus = Bus::new(1, "right".to_string()).with_pan(0.8);
        assert_eq!(right_bus.pan, 0.8);

        // Test clamping
        let far_left = Bus::new(2, "far".to_string()).with_pan(-2.0);
        assert_eq!(far_left.pan, -1.0);

        let far_right = Bus::new(2, "far".to_string()).with_pan(2.0);
        assert_eq!(far_right.pan, 1.0);
    }

    #[test]
    fn test_bus_is_empty() {
        let mut bus = Bus::new(0, "test".to_string());
        assert!(bus.is_empty());

        bus.add_track(Track::new());
        assert!(!bus.is_empty());
    }
}

/// Builder for applying effects to a bus
///
/// Provides a fluent API for configuring bus-level effects, volume, and pan.
pub struct BusBuilder<'a> {
    bus: &'a mut Bus,
}

impl<'a> BusBuilder<'a> {
    /// Create a new BusBuilder
    pub(crate) fn new(bus: &'a mut Bus) -> Self {
        Self { bus }
    }

    /// Set the volume of this bus
    pub fn volume(self, volume: f32) -> Self {
        self.bus.volume = volume.clamp(0.0, 2.0);
        self
    }

    /// Set the pan of this bus
    pub fn pan(self, pan: f32) -> Self {
        self.bus.pan = pan.clamp(-1.0, 1.0);
        self
    }

    /// Mute this bus
    pub fn mute(self) -> Self {
        self.bus.muted = true;
        self
    }

    /// Unmute this bus
    pub fn unmute(self) -> Self {
        self.bus.muted = false;
        self
    }

    /// Solo this bus
    pub fn solo(self) -> Self {
        self.bus.soloed = true;
        self
    }

    /// Unsolo this bus
    pub fn unsolo(self) -> Self {
        self.bus.soloed = false;
        self
    }

    // ===== Effect Methods =====

    /// Add EQ (3-band equalizer) effect to this bus
    pub fn eq(self, eq: EQ) -> Self {
        self.bus.effects.eq = Some(eq);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add parametric EQ effect to this bus
    pub fn parametric_eq(self, eq: ParametricEQ) -> Self {
        self.bus.effects.parametric_eq = Some(eq);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add compressor effect to this bus
    pub fn compressor(self, compressor: Compressor) -> Self {
        self.bus.effects.compressor = Some(compressor);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add gate effect to this bus
    pub fn gate(self, gate: Gate) -> Self {
        self.bus.effects.gate = Some(gate);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add limiter effect to this bus
    pub fn limiter(self, limiter: Limiter) -> Self {
        self.bus.effects.limiter = Some(limiter);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add saturation effect to this bus
    pub fn saturation(self, saturation: Saturation) -> Self {
        self.bus.effects.saturation = Some(saturation);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add distortion effect to this bus
    pub fn distortion(self, distortion: Distortion) -> Self {
        self.bus.effects.distortion = Some(distortion);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add bitcrusher effect to this bus
    pub fn bitcrusher(self, bitcrusher: BitCrusher) -> Self {
        self.bus.effects.bitcrusher = Some(bitcrusher);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add chorus effect to this bus
    pub fn chorus(self, chorus: Chorus) -> Self {
        self.bus.effects.chorus = Some(chorus);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add phaser effect to this bus
    pub fn phaser(self, phaser: Phaser) -> Self {
        self.bus.effects.phaser = Some(phaser);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add flanger effect to this bus
    pub fn flanger(self, flanger: Flanger) -> Self {
        self.bus.effects.flanger = Some(flanger);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add ring modulator effect to this bus
    pub fn ring_mod(self, ring_mod: RingModulator) -> Self {
        self.bus.effects.ring_mod = Some(ring_mod);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add tremolo effect to this bus
    pub fn tremolo(self, tremolo: Tremolo) -> Self {
        self.bus.effects.tremolo = Some(tremolo);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add autopan effect to this bus
    pub fn autopan(self, autopan: AutoPan) -> Self {
        self.bus.effects.autopan = Some(autopan);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add delay effect to this bus
    pub fn delay(self, delay: Delay) -> Self {
        self.bus.effects.delay = Some(delay);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add reverb effect to this bus
    pub fn reverb(self, reverb: Reverb) -> Self {
        self.bus.effects.reverb = Some(reverb);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add phase vocoder effect to this bus for time-stretching and pitch-shifting
    pub fn phase_vocoder(self, phase_vocoder: PhaseVocoder) -> Self {
        self.bus.effects.phase_vocoder = Some(phase_vocoder);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral freeze effect to this bus for capturing and holding frequency spectrum
    pub fn spectral_freeze(self, spectral_freeze: SpectralFreeze) -> Self {
        self.bus.effects.spectral_freeze = Some(spectral_freeze);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral gate effect to this bus for frequency-selective noise gating
    pub fn spectral_gate(self, spectral_gate: SpectralGate) -> Self {
        self.bus.effects.spectral_gate = Some(spectral_gate);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral compressor effect to this bus for frequency-selective dynamic range compression
    pub fn spectral_compressor(self, spectral_compressor: SpectralCompressor) -> Self {
        self.bus.effects.spectral_compressor = Some(spectral_compressor);
        self.bus.effects.compute_effect_order();
        self
    }

    pub fn spectral_robotize(self, spectral_robotize: SpectralRobotize) -> Self {
        self.bus.effects.spectral_robotize = Some(spectral_robotize);
        self.bus.effects.compute_effect_order();
        self
    }

    pub fn spectral_delay(self, spectral_delay: SpectralDelay) -> Self {
        self.bus.effects.spectral_delay = Some(spectral_delay);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral filter effect to this bus for frequency-domain filtering
    pub fn spectral_filter(self, spectral_filter: SpectralFilter) -> Self {
        self.bus.effects.spectral_filter = Some(spectral_filter);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral blur effect to this bus for temporal smoothing in frequency domain
    pub fn spectral_blur(self, spectral_blur: SpectralBlur) -> Self {
        self.bus.effects.spectral_blur = Some(spectral_blur);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral shift effect to this bus for frequency shifting
    pub fn spectral_shift(self, spectral_shift: SpectralShift) -> Self {
        self.bus.effects.spectral_shift = Some(spectral_shift);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add formant shifter effect to this bus for vocal character transformation
    pub fn formant_shifter(self, formant_shifter: FormantShifter) -> Self {
        self.bus.effects.formant_shifter = Some(formant_shifter);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral harmonizer effect to this bus for pitch-shifted harmonies
    pub fn spectral_harmonizer(self, spectral_harmonizer: SpectralHarmonizer) -> Self {
        self.bus.effects.spectral_harmonizer = Some(spectral_harmonizer);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral resonator effect to this bus for resonant frequency peaks
    pub fn spectral_resonator(self, spectral_resonator: SpectralResonator) -> Self {
        self.bus.effects.spectral_resonator = Some(spectral_resonator);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral panner effect to this bus for frequency-based spatial positioning
    pub fn spectral_panner(self, spectral_panner: SpectralPanner) -> Self {
        self.bus.effects.spectral_panner = Some(spectral_panner);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral exciter effect to this bus for harmonic enhancement
    pub fn spectral_exciter(self, spectral_exciter: SpectralExciter) -> Self {
        self.bus.effects.spectral_exciter = Some(spectral_exciter);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral invert effect to this bus for frequency spectrum reversal
    pub fn spectral_invert(self, spectral_invert: SpectralInvert) -> Self {
        self.bus.effects.spectral_invert = Some(spectral_invert);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral widen effect to this bus for stereo widening via phase manipulation
    pub fn spectral_widen(self, spectral_widen: SpectralWiden) -> Self {
        self.bus.effects.spectral_widen = Some(spectral_widen);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral morph effect to this bus for morphing spectrum toward target shapes
    pub fn spectral_morph(self, spectral_morph: SpectralMorph) -> Self {
        self.bus.effects.spectral_morph = Some(spectral_morph);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral dynamics effect (frequency-dependent compression/expansion)
    pub fn spectral_dynamics(self, spectral_dynamics: SpectralDynamics) -> Self {
        self.bus.effects.spectral_dynamics = Some(spectral_dynamics);
        self.bus.effects.compute_effect_order();
        self
    }

    /// Add spectral scramble effect (glitchy frequency bin randomization)
    pub fn spectral_scramble(self, spectral_scramble: SpectralScramble) -> Self {
        self.bus.effects.spectral_scramble = Some(spectral_scramble);
        self.bus.effects.compute_effect_order();
        self
    }
}
