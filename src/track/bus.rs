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
//!     .drum(DrumType::Kick);
//!
//! comp.track("lead")
//!     .bus("melody")
//!     .notes(&[C4, E4, G4], 0.5);
//! ```

use crate::synthesis::effects::{
    AutoPan, BitCrusher, Chorus, Compressor, Delay, Distortion, EffectChain, EQ, Flanger, Gate,
    Limiter, ParametricEQ, Phaser, Reverb, RingModulator, Saturation, Tremolo,
};
use crate::track::Track;

/// A bus groups multiple tracks together for processing
///
/// Buses mix their tracks together, apply effects to the summed signal,
/// and send the result to the master output. This provides an intermediate
/// layer between individual tracks and the final mix.
#[derive(Debug, Clone)]
pub struct Bus {
    /// Name of this bus
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
}

impl Bus {
    /// Create a new empty bus
    pub fn new(name: String) -> Self {
        Self {
            name,
            tracks: Vec::new(),
            effects: EffectChain::new(),
            volume: 1.0,
            pan: 0.0,
            muted: false,
            soloed: false,
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
    pub(crate) fn sample_at_future(&mut self, _time: f32, _sample_rate: f32, _sample_count: u64) -> (f32, f32) {
        // Placeholder for future when Track has sample_at()
        // For now, all track processing happens in Mixer::process_track()
        (0.0, 0.0)
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new("default".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::notes::*;
    use crate::composition::drums::DrumType;

    #[test]
    fn test_bus_creation() {
        let bus = Bus::new("drums".to_string());
        assert_eq!(bus.name, "drums");
        assert_eq!(bus.tracks.len(), 0);
        assert_eq!(bus.volume, 1.0);
        assert_eq!(bus.pan, 0.0);
        assert!(!bus.muted);
        assert!(!bus.soloed);
    }

    #[test]
    fn test_bus_add_track() {
        let mut bus = Bus::new("test".to_string());
        let mut track = Track::new();
        track.add_note(&[C4], 0.0, 1.0);

        bus.add_track(track);
        assert_eq!(bus.track_count(), 1);
        assert!(!bus.is_empty());
    }

    #[test]
    fn test_bus_total_duration() {
        let mut bus = Bus::new("test".to_string());

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
        let bus = Bus::new("test".to_string()).with_volume(0.5);
        assert_eq!(bus.volume, 0.5);

        // Test clamping
        let loud_bus = Bus::new("loud".to_string()).with_volume(5.0);
        assert_eq!(loud_bus.volume, 2.0);

        let silent_bus = Bus::new("silent".to_string()).with_volume(-1.0);
        assert_eq!(silent_bus.volume, 0.0);
    }

    #[test]
    fn test_bus_with_pan() {
        let left_bus = Bus::new("left".to_string()).with_pan(-0.5);
        assert_eq!(left_bus.pan, -0.5);

        let right_bus = Bus::new("right".to_string()).with_pan(0.8);
        assert_eq!(right_bus.pan, 0.8);

        // Test clamping
        let far_left = Bus::new("far".to_string()).with_pan(-2.0);
        assert_eq!(far_left.pan, -1.0);

        let far_right = Bus::new("far".to_string()).with_pan(2.0);
        assert_eq!(far_right.pan, 1.0);
    }

    #[test]
    fn test_bus_is_empty() {
        let mut bus = Bus::new("test".to_string());
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
}
