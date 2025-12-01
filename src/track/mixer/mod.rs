//! Mixer implementation
//!
//! The mixer combines multiple buses together and handles the core audio rendering.
//! Each bus contains tracks, and buses are mixed through the master chain.

mod cache;
mod envelope_cache;
mod gpu;
mod master_effects;
mod output_types;
mod process_block;
mod process_track;
mod render;
mod sample_at;

use super::bus::{Bus, BusBuilder};
use super::events::*;
use super::track::Track;
use crate::cache::SampleCache;
use crate::composition::timing::Tempo;
#[cfg(feature = "gpu")]
use crate::gpu::GpuSynthesizer;
use crate::synthesis::effects::{EffectChain, ResolvedSidechainSource, SidechainSource};
use crate::track::ids::{BusId, TrackId};
use std::collections::HashMap;
use std::sync::Arc;

use envelope_cache::EnvelopeCache;
use output_types::{BusOutput, TrackOutput};

/// Mix multiple buses together (OPTIMIZED: Vec-based with pre-allocated buffers)
///
/// The Mixer organizes audio into buses, where each bus contains one or more tracks.
/// Signal flow: Tracks → Buses → Master → Output
///
/// **Performance optimizations:**
/// - Buses stored in Vec<Bus> indexed by BusId (not HashMap<String, Bus>)
/// - Pre-allocated buffers for track_outputs_by_bus, bus_outputs, envelope_cache
/// - Integer IDs instead of string comparisons in hot path
#[derive(Debug, Clone)]
pub struct Mixer {
    // Hot path: Integer-indexed buses for fast iteration
    pub(super) buses: Vec<Option<Bus>>, // Sparse Vec: Some(bus) at bus.id index, None otherwise
    bus_order: Vec<BusId>,              // Order in which to process buses

    // Cold path: String lookup for user-facing API
    bus_name_to_id: HashMap<String, BusId>,

    // Pre-allocated buffers (reused every sample_at() call)
    // OPTIMIZED: track_outputs indexed by bus_id for O(1) lookup instead of O(n) linear search
    track_outputs_by_bus: Vec<Vec<TrackOutput>>,
    bus_outputs: Vec<BusOutput>,
    envelope_cache: EnvelopeCache,

    // Sample cache for pre-rendered synthesis (lock-free with DashMap)
    pub(crate) cache: Option<Arc<SampleCache>>,

    // GPU synthesizer for experimental acceleration (optional, falls back to CPU)
    #[cfg(feature = "gpu")]
    pub(crate) gpu_synthesizer: Option<Arc<GpuSynthesizer>>,

    // Track if we've pre-rendered notes (skip cache checks during streaming)
    pub(crate) prerendered: bool,

    pub tempo: Tempo,
    pub(super) sample_count: u64, // For quantized automation lookups
    pub master: EffectChain,      // Master effects chain (stereo processing)
}

impl Mixer {
    /// Create a new mixer with the specified tempo
    ///
    /// # Arguments
    /// * `tempo` - Tempo for the composition (used for MIDI export)
    pub fn new(tempo: Tempo) -> Self {
        // Pre-allocate reasonable capacities to avoid allocations during audio rendering
        const INITIAL_BUS_CAPACITY: usize = 16;
        const INITIAL_TRACK_CAPACITY: usize = 128;

        Self {
            buses: Vec::with_capacity(INITIAL_BUS_CAPACITY),
            bus_order: Vec::with_capacity(INITIAL_BUS_CAPACITY),
            bus_name_to_id: HashMap::new(),
            track_outputs_by_bus: (0..INITIAL_BUS_CAPACITY)
                .map(|_| Vec::with_capacity(INITIAL_TRACK_CAPACITY / INITIAL_BUS_CAPACITY))
                .collect(),
            bus_outputs: Vec::with_capacity(INITIAL_BUS_CAPACITY),
            envelope_cache: EnvelopeCache::new(INITIAL_TRACK_CAPACITY, INITIAL_BUS_CAPACITY),
            cache: None, // Cache disabled by default
            #[cfg(feature = "gpu")]
            gpu_synthesizer: None, // GPU disabled by default (requires explicit enable_gpu call)
            prerendered: false,
            tempo,
            sample_count: 0,
            master: EffectChain::new(),
        }
    }

    /// Add a bus to the mixer
    ///
    /// # Arguments
    /// * `bus` - The bus to add
    pub fn add_bus(&mut self, bus: Bus) {
        let bus_id = bus.id;
        let bus_name = bus.name.clone();

        // Ensure buses Vec is large enough to hold this bus ID
        if bus_id as usize >= self.buses.len() {
            self.buses.resize(bus_id as usize + 1, None);
        }

        // Store the bus at its ID index
        self.buses[bus_id as usize] = Some(bus);

        // Add to processing order
        self.bus_order.push(bus_id);

        // Map name to ID for user-facing API
        self.bus_name_to_id.insert(bus_name, bus_id);

        // Expand envelope cache if needed
        self.envelope_cache.expand_buses(bus_id as usize);

        // Expand track_outputs_by_bus if needed
        while self.track_outputs_by_bus.len() <= bus_id as usize {
            self.track_outputs_by_bus.push(Vec::with_capacity(8));
        }
    }

    /// Add a track to the default bus for backward compatibility
    ///
    /// This maintains compatibility with existing code that adds tracks directly.
    /// Tracks are added to a bus named "default".
    ///
    /// # Arguments
    /// * `track` - The track to add
    pub fn add_track(&mut self, track: Track) {
        self.get_or_create_bus("default").add_track(track);
    }

    /// Get or create a bus by name
    ///
    /// # Arguments
    /// * `name` - Name of the bus
    pub fn get_or_create_bus(&mut self, name: &str) -> &mut Bus {
        // Check if bus already exists
        if let Some(&bus_id) = self.bus_name_to_id.get(name) {
            // Bus exists, return mutable reference
            return self.buses[bus_id as usize]
                .as_mut()
                .expect("Internal error: bus_name_to_id points to empty bus slot");
        }

        // Bus doesn't exist, create it
        let new_bus_id = self.buses.len() as BusId; // Use current length as new ID
        let new_bus = Bus::new(new_bus_id, name.to_string());

        self.add_bus(new_bus);

        // Return reference to the newly added bus
        self.buses[new_bus_id as usize]
            .as_mut()
            .expect("Internal error: bus not found immediately after adding")
    }

    /// Get a bus by name
    ///
    /// # Arguments
    /// * `name` - Name of the bus
    pub fn get_bus(&self, name: &str) -> Option<&Bus> {
        self.bus_name_to_id
            .get(name)
            .and_then(|&id| self.buses.get(id as usize).and_then(|opt| opt.as_ref()))
    }

    /// Get a mutable bus by name
    ///
    /// # Arguments
    /// * `name` - Name of the bus
    pub fn get_bus_mut(&mut self, name: &str) -> Option<&mut Bus> {
        self.bus_name_to_id
            .get(name)
            .copied()
            .and_then(move |id| self.buses.get_mut(id as usize).and_then(|opt| opt.as_mut()))
    }

    /// Get the BusId for a bus by name
    ///
    /// Used internally for resolving sidechain sources.
    ///
    /// # Arguments
    /// * `name` - Name of the bus
    #[allow(dead_code)]
    pub(crate) fn get_bus_id(&self, name: &str) -> Option<BusId> {
        self.bus_name_to_id.get(name).copied()
    }

    /// Resolve all sidechain sources from string names to integer IDs
    ///
    /// This is called during Composition::into_mixer() to optimize the hot path
    /// by converting user-facing string-based sidechain references to efficient
    /// integer ID lookups.
    pub(crate) fn resolve_sidechains(&mut self) {
        // Clone name mappings to avoid borrowing issues
        let bus_name_to_id = self.bus_name_to_id.clone();

        // First pass: collect all track names and IDs for resolution
        let mut track_name_to_id: HashMap<String, TrackId> = HashMap::new();
        for bus in self.buses.iter().flatten() {
            for track in &bus.tracks {
                if let Some(ref track_name) = track.name {
                    track_name_to_id.insert(track_name.clone(), track.id);
                }
            }
        }

        // Second pass: resolve sidechains with mutable access
        for bus_opt in self.buses.iter_mut() {
            let bus = match bus_opt {
                Some(b) => b,
                None => continue,
            };

            // Resolve bus-level compressor sidechain
            if let Some(ref mut compressor) = bus.effects.compressor {
                if let Some(ref source) = compressor.sidechain_source {
                    compressor.resolved_sidechain_source =
                        Self::resolve_sidechain_source(source, &track_name_to_id, &bus_name_to_id);
                }
            }

            // Resolve track-level compressor sidechains
            for track in &mut bus.tracks {
                if let Some(ref mut compressor) = track.effects.compressor {
                    if let Some(ref source) = compressor.sidechain_source {
                        compressor.resolved_sidechain_source = Self::resolve_sidechain_source(
                            source,
                            &track_name_to_id,
                            &bus_name_to_id,
                        );
                    }
                }
            }
        }
    }

    /// Resolve a single sidechain source to an integer ID
    fn resolve_sidechain_source(
        source: &SidechainSource,
        track_name_to_id: &HashMap<String, TrackId>,
        bus_name_to_id: &HashMap<String, BusId>,
    ) -> Option<ResolvedSidechainSource> {
        match source {
            SidechainSource::Track(name) => {
                // Look up track by name
                track_name_to_id
                    .get(name)
                    .copied()
                    .map(ResolvedSidechainSource::Track)
                    .or_else(|| {
                        eprintln!("Warning: Sidechain track '{}' not found", name);
                        None
                    })
            }
            SidechainSource::Bus(name) => {
                // Look up bus by name
                bus_name_to_id
                    .get(name)
                    .copied()
                    .map(ResolvedSidechainSource::Bus)
                    .or_else(|| {
                        eprintln!("Warning: Sidechain bus '{}' not found", name);
                        None
                    })
            }
        }
    }

    /// Get a builder for applying effects to a bus
    ///
    /// Creates or gets an existing bus and returns a builder for applying effects,
    /// volume, and pan settings in a fluent API.
    ///
    /// # Arguments
    /// * `name` - Name of the bus
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::synthesis::effects::{Reverb, Compressor};
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("kick")
    ///     .bus("drums")
    ///     .drum(DrumType::Kick, 0.0);
    ///
    /// let mut mixer = comp.into_mixer();
    ///
    /// // Apply effects to the drums bus
    /// mixer.bus("drums")
    ///     .reverb(Reverb::new(0.3, 0.4, 0.3))
    ///     .compressor(Compressor::new(0.65, 4.0, 0.01, 0.08, 1.0))
    ///     .volume(0.9);
    /// ```
    pub fn bus(&mut self, name: &str) -> BusBuilder<'_> {
        let bus = self.get_or_create_bus(name);
        BusBuilder::new(bus)
    }

    /// Get the total duration across all buses in seconds
    ///
    /// Returns the end time of the longest bus.
    /// Returns 0.0 if the mixer has no buses.
    pub fn total_duration(&self) -> f32 {
        self.buses
            .iter()
            .filter_map(|opt| opt.as_ref())
            .map(|b| b.total_duration())
            .fold(0.0, f32::max)
    }

    /// Check if the mixer has any audio events
    ///
    /// Returns `true` if all buses/tracks are empty (no notes, drums, or samples).
    /// Useful for detecting empty compositions before playback.
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mixer = comp.into_mixer();
    /// assert!(mixer.is_empty());
    ///
    /// let mut comp2 = Composition::new(Tempo::new(120.0));
    /// comp2.instrument("piano", &Instrument::electric_piano())
    ///     .note(&[440.0], 1.0);
    /// let mixer2 = comp2.into_mixer();
    /// assert!(!mixer2.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.buses
            .iter()
            .filter_map(|opt| opt.as_ref())
            .all(|b| b.tracks.iter().all(|t| t.events.is_empty()))
    }

    /// Get all tracks across all buses as a flat vector
    ///
    /// This is useful for export functions that need to iterate over all tracks.
    /// Note: This creates a new vector, so use sparingly.
    pub fn all_tracks(&self) -> Vec<&Track> {
        self.buses
            .iter()
            .filter_map(|opt| opt.as_ref())
            .flat_map(|bus| bus.tracks.iter())
            .collect()
    }

    /// Get all tracks across all buses as a mutable flat vector
    ///
    /// This is useful for export functions that need to iterate over all tracks.
    /// Note: This creates a new vector, so use sparingly.
    pub fn all_tracks_mut(&mut self) -> Vec<&mut Track> {
        self.buses
            .iter_mut()
            .filter_map(|opt| opt.as_mut())
            .flat_map(|bus| bus.tracks.iter_mut())
            .collect()
    }

    /// Get the tracks field for backward compatibility with tests
    ///
    /// Returns a cloned Vec of tracks from the default bus.
    /// This works around lifetime issues in tests where `comp.into_mixer().tracks()`
    /// would create a temporary.
    #[cfg(test)]
    pub fn tracks(&self) -> Vec<Track> {
        self.get_bus("default")
            .map(|b| b.tracks.clone())
            .unwrap_or_default()
    }

    /// Get mutable access to the tracks field for backward compatibility with tests
    #[cfg(test)]
    pub fn tracks_mut(&mut self) -> &mut Vec<Track> {
        &mut self.get_or_create_bus("default").tracks
    }

    /// Repeat all tracks in the mixer N times
    ///
    /// This duplicates all events in all tracks, placing copies sequentially.
    /// Useful for looping an entire composition.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::engine::AudioEngine;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// # let engine = AudioEngine::new()?;
    /// let mixer = comp.into_mixer().repeat(3); // Play composition 4 times total
    /// engine.play_mixer(&mixer)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn repeat(mut self, times: usize) -> Self {
        if times == 0 {
            return self;
        }

        let total_duration = self.total_duration();

        // For each bus, repeat all its track events
        for bus_opt in self.buses.iter_mut() {
            let bus = match bus_opt {
                Some(b) => b,
                None => continue,
            };

            for track in &mut bus.tracks {
                let original_events: Vec<_> = track.events.clone();
                // Pre-allocate space for all repetitions to avoid reallocations
                track.events.reserve(original_events.len() * times);

                for i in 0..times {
                    let offset = total_duration * (i + 1) as f32;

                    for event in &original_events {
                        match event {
                            AudioEvent::Note(note) => {
                                track.add_note_with_waveform_envelope_and_bend(
                                    &note.frequencies[..note.num_freqs],
                                    note.start_time + offset,
                                    note.duration,
                                    note.waveform,
                                    note.envelope,
                                    note.pitch_bend_semitones,
                                );
                            }
                            AudioEvent::Drum(drum) => {
                                track.add_drum(
                                    drum.drum_type,
                                    drum.start_time + offset,
                                    drum.spatial_position,
                                );
                            }
                            AudioEvent::Sample(sample) => {
                                track
                                    .events
                                    .push(AudioEvent::Sample(crate::track::SampleEvent {
                                        sample: sample.sample.clone(),
                                        start_time: sample.start_time + offset,
                                        playback_rate: sample.playback_rate,
                                        volume: sample.volume,
                                        spatial_position: sample.spatial_position,
                                    }));
                                track.invalidate_time_cache();
                            }
                            AudioEvent::TempoChange(tempo) => {
                                track.events.push(AudioEvent::TempoChange(
                                    crate::track::TempoChangeEvent {
                                        start_time: tempo.start_time + offset,
                                        bpm: tempo.bpm,
                                    },
                                ));
                                track.invalidate_time_cache();
                            }
                            AudioEvent::TimeSignature(time_sig) => {
                                track.events.push(AudioEvent::TimeSignature(
                                    crate::track::TimeSignatureEvent {
                                        start_time: time_sig.start_time + offset,
                                        numerator: time_sig.numerator,
                                        denominator: time_sig.denominator,
                                    },
                                ));
                                track.invalidate_time_cache();
                            }
                            AudioEvent::KeySignature(key_sig) => {
                                track
                                    .events
                                    .push(AudioEvent::KeySignature(KeySignatureEvent {
                                        start_time: key_sig.start_time + offset,
                                        key_signature: key_sig.key_signature,
                                    }));
                                track.invalidate_time_cache();
                            }
                        }
                    }
                }
            }
        }

        self
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new(Tempo::new(120.0))
    }
}
