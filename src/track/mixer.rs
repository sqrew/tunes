//! Mixer implementation
//!
//! The mixer combines multiple buses together and handles the core audio rendering.
//! Each bus contains tracks, and buses are mixed through the master chain.

use super::bus::{Bus, BusBuilder};
use super::events::*;
use super::track::Track;
use crate::composition::rhythm::Tempo;
use crate::synthesis::effects::{EffectChain, ResolvedSidechainSource};
use crate::track::ids::{BusId, TrackId};
use std::collections::HashMap;

/// Envelope cache for sidechaining (OPTIMIZED: Vec-based for O(1) access)
///
/// Stores RMS envelope values for tracks and buses during a single sample_at() call.
/// This allows sidechained effects to access the envelope of their source signal.
///
/// **Performance:** Uses Vec indexed by ID instead of HashMap with String keys.
/// This eliminates string hashing and allocation, providing O(1) direct access.
#[derive(Debug, Clone)]
struct EnvelopeCache {
    tracks: Vec<f32>,  // Track ID -> RMS envelope (direct index)
    buses: Vec<f32>,   // Bus ID -> RMS envelope (direct index)
}

impl EnvelopeCache {
    /// Create a new envelope cache with pre-allocated capacity
    ///
    /// # Arguments
    /// * `max_tracks` - Maximum number of tracks to support
    /// * `max_buses` - Maximum number of buses to support
    fn new(max_tracks: usize, max_buses: usize) -> Self {
        Self {
            tracks: vec![0.0; max_tracks],
            buses: vec![0.0; max_buses],
        }
    }

    /// Clear all cached envelope values (resets to 0.0)
    ///
    /// Called at the start of each sample_at() to reset state.
    #[inline(always)]
    fn clear(&mut self) {
        // Fast memset-like operation
        self.tracks.fill(0.0);
        self.buses.fill(0.0);
    }

    /// Store a track's envelope by ID
    ///
    /// # Arguments
    /// * `track_id` - Unique track identifier
    /// * `envelope` - RMS envelope value
    #[inline(always)]
    fn cache_track(&mut self, track_id: TrackId, envelope: f32) {
        if let Some(slot) = self.tracks.get_mut(track_id as usize) {
            *slot = envelope;
        }
    }

    /// Store a bus's envelope by ID
    ///
    /// # Arguments
    /// * `bus_id` - Unique bus identifier
    /// * `envelope` - RMS envelope value
    #[inline(always)]
    fn cache_bus(&mut self, bus_id: BusId, envelope: f32) {
        if let Some(slot) = self.buses.get_mut(bus_id as usize) {
            *slot = envelope;
        }
    }

    /// Get a track's envelope by ID (returns 0.0 if not found)
    #[inline(always)]
    fn get_track(&self, track_id: TrackId) -> f32 {
        self.tracks.get(track_id as usize).copied().unwrap_or(0.0)
    }

    /// Get a bus's envelope by ID (returns 0.0 if not found)
    #[inline(always)]
    fn get_bus(&self, bus_id: BusId) -> f32 {
        self.buses.get(bus_id as usize).copied().unwrap_or(0.0)
    }
}

/// Pre-allocated track output (avoids allocation in hot path)
///
/// Stores the output of a single track for later bus mixing.
/// Uses integer bus_id instead of string bus_name for O(1) comparison.
#[derive(Debug, Clone, Copy)]
struct TrackOutput {
    bus_id: BusId,     // Which bus this track belongs to (INTEGER!)
    left: f32,         // Left channel output
    right: f32,        // Right channel output
    envelope: f32,     // RMS envelope for sidechaining
}

/// Pre-allocated bus output (avoids allocation in hot path)
///
/// Stores the output of a single bus for later master mixing.
#[derive(Debug, Clone, Copy)]
struct BusOutput {
    bus_id: BusId,     // Bus identifier (unused, kept for potential future use)
    left: f32,         // Left channel output
    right: f32,        // Right channel output
}

/// Mix multiple buses together (OPTIMIZED: Vec-based with pre-allocated buffers)
///
/// The Mixer organizes audio into buses, where each bus contains one or more tracks.
/// Signal flow: Tracks → Buses → Master → Output
///
/// **Performance optimizations:**
/// - Buses stored in Vec<Bus> indexed by BusId (not HashMap<String, Bus>)
/// - Pre-allocated buffers for track_outputs, bus_outputs, envelope_cache
/// - Integer IDs instead of string comparisons in hot path
#[derive(Debug, Clone)]
pub struct Mixer {
    // Hot path: Integer-indexed buses for fast iteration
    pub(super) buses: Vec<Option<Bus>>,  // Sparse Vec: Some(bus) at bus.id index, None otherwise
    bus_order: Vec<BusId>,    // Order in which to process buses

    // Cold path: String lookup for user-facing API
    bus_name_to_id: HashMap<String, BusId>,

    // Pre-allocated buffers (reused every sample_at() call)
    track_outputs: Vec<TrackOutput>,
    bus_outputs: Vec<BusOutput>,
    envelope_cache: EnvelopeCache,

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
            track_outputs: Vec::with_capacity(INITIAL_TRACK_CAPACITY),
            bus_outputs: Vec::with_capacity(INITIAL_BUS_CAPACITY),
            envelope_cache: EnvelopeCache::new(INITIAL_TRACK_CAPACITY, INITIAL_BUS_CAPACITY),
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
        if bus_id as usize >= self.envelope_cache.buses.len() {
            self.envelope_cache.buses.resize(bus_id as usize + 1, 0.0);
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
            return self.buses[bus_id as usize].as_mut().unwrap();
        }

        // Bus doesn't exist, create it
        let new_bus_id = self.buses.len() as BusId;  // Use current length as new ID
        let new_bus = Bus::new(new_bus_id, name.to_string());

        self.add_bus(new_bus);

        // Return reference to the newly added bus
        self.buses[new_bus_id as usize].as_mut().unwrap()
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
    pub(crate) fn get_bus_id(&self, name: &str) -> Option<BusId> {
        self.bus_name_to_id.get(name).copied()
    }

    /// Resolve all sidechain sources from string names to integer IDs
    ///
    /// This is called during Composition::into_mixer() to optimize the hot path
    /// by converting user-facing string-based sidechain references to efficient
    /// integer ID lookups.
    pub(crate) fn resolve_sidechains(&mut self) {
        use crate::synthesis::effects::ResolvedSidechainSource;

        // Clone name mappings to avoid borrowing issues
        let bus_name_to_id = self.bus_name_to_id.clone();

        // First pass: collect all track names and IDs for resolution
        let mut track_name_to_id: HashMap<String, TrackId> = HashMap::new();
        for bus_opt in &self.buses {
            if let Some(bus) = bus_opt {
                for track in &bus.tracks {
                    if let Some(ref track_name) = track.name {
                        track_name_to_id.insert(track_name.clone(), track.id);
                    }
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
                    compressor.resolved_sidechain_source = Self::resolve_sidechain_source(
                        source,
                        &track_name_to_id,
                        &bus_name_to_id,
                    );
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
        source: &crate::synthesis::effects::SidechainSource,
        track_name_to_id: &HashMap<String, TrackId>,
        bus_name_to_id: &HashMap<String, BusId>,
    ) -> Option<ResolvedSidechainSource> {
        use crate::synthesis::effects::{ResolvedSidechainSource, SidechainSource};

        match source {
            SidechainSource::Track(name) => {
                // Look up track by name
                track_name_to_id.get(name)
                    .copied()
                    .map(ResolvedSidechainSource::Track)
                    .or_else(|| {
                        eprintln!("Warning: Sidechain track '{}' not found", name);
                        None
                    })
            }
            SidechainSource::Bus(name) => {
                // Look up bus by name
                bus_name_to_id.get(name)
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
    ///     .drum(DrumType::Kick);
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
    /// # use tunes::composition::rhythm::Tempo;
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

    /// Generate a stereo sample at a given time by mixing all buses
    ///
    /// This is the core rendering method that generates audio samples by:
    /// 1. Processing each bus (which mixes its tracks and applies bus effects)
    /// 2. Summing all bus outputs
    /// 3. Applying master effects to the final mix
    ///
    /// # Arguments
    /// * `time` - The time position in seconds
    /// * `sample_rate` - Sample rate in Hz (e.g., 44100)
    /// * `_sample_clock` - Reserved for future use
    /// * `_listener` - Reserved for spatial audio (handled at track level)
    /// * `_spatial_params` - Reserved for spatial audio (handled at track level)
    ///
    /// # Returns
    /// A tuple of (left_channel, right_channel) audio samples in range -1.0 to 1.0
    pub fn sample_at(
        &mut self,
        time: f32,
        sample_rate: f32,
        _sample_clock: f32,
        _listener: Option<&crate::synthesis::spatial::ListenerConfig>,
        _spatial_params: Option<&crate::synthesis::spatial::SpatialParams>,
    ) -> (f32, f32) {
        // Increment sample count for quantized automation lookups
        self.sample_count = self.sample_count.wrapping_add(1);

        // Clear pre-allocated buffers (NO ALLOCATION!)
        self.track_outputs.clear();
        self.bus_outputs.clear();
        self.envelope_cache.clear();

        let mut mixed_left = 0.0;
        let mut mixed_right = 0.0;

        // PASS 1: Process tracks and cache their envelopes
        // We need to process all tracks first to build the envelope cache
        // before applying bus effects (which may use sidechaining)

        // Iterate over buses using Vec<Option<Bus>>
        for bus_opt in self.buses.iter_mut() {
            let bus = match bus_opt {
                Some(b) => b,
                None => continue,
            };

            if bus.muted {
                continue;
            }

            let sample_count = self.sample_count;
            let bus_id = bus.id;

            for track in &mut bus.tracks {
                let (track_left, track_right) = Self::process_track_static(track, time, sample_rate, sample_count);

                // Calculate RMS envelope for this track
                let envelope = ((track_left * track_left + track_right * track_right) / 2.0).sqrt();

                // Cache track envelope using integer ID
                self.envelope_cache.cache_track(track.id, envelope);

                // Store output using integer bus ID (NO STRING CLONE!)
                self.track_outputs.push(TrackOutput {
                    bus_id,
                    left: track_left,
                    right: track_right,
                    envelope,
                });
            }
        }

        // PASS 2: Mix tracks into buses and apply bus effects with sidechain support

        // Iterate over buses for processing
        for bus_opt in self.buses.iter_mut() {
            let bus = match bus_opt {
                Some(b) => b,
                None => continue,
            };

            if bus.muted {
                continue;
            }

            let bus_id = bus.id;

            // Sum tracks belonging to this bus (INTEGER COMPARISON!)
            let mut bus_left = 0.0;
            let mut bus_right = 0.0;
            for track_output in &self.track_outputs {
                if track_output.bus_id == bus_id {
                    bus_left += track_output.left;
                    bus_right += track_output.right;
                }
            }

            // Calculate bus envelope BEFORE effects
            let bus_envelope = ((bus_left * bus_left + bus_right * bus_right) / 2.0).sqrt();
            self.envelope_cache.cache_bus(bus_id, bus_envelope);

            // Look up sidechain envelope using resolved IDs (OPTIMIZED: Integer lookup!)
            let sidechain_env = if let Some(ref compressor) = bus.effects.compressor {
                if let Some(ref resolved_source) = compressor.resolved_sidechain_source {
                    match resolved_source {
                        ResolvedSidechainSource::Track(track_id) => {
                            Some(self.envelope_cache.get_track(*track_id))
                        }
                        ResolvedSidechainSource::Bus(sidechain_bus_id) => {
                            Some(self.envelope_cache.get_bus(*sidechain_bus_id))
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            };

            // Apply bus effects (stereo processing) with sidechain support
            let (effected_left, effected_right) = bus.effects.process_stereo(
                bus_left,
                bus_right,
                sample_rate,
                time,
                self.sample_count,
                sidechain_env,
            );

            // Apply bus volume and pan
            let pan_left = if bus.pan <= 0.0 {
                1.0
            } else {
                1.0 - bus.pan
            };
            let pan_right = if bus.pan >= 0.0 {
                1.0
            } else {
                1.0 + bus.pan
            };

            let final_bus_left = effected_left * bus.volume * pan_left;
            let final_bus_right = effected_right * bus.volume * pan_right;

            // Store output using integer bus ID (NO STRING CLONE!)
            self.bus_outputs.push(BusOutput {
                bus_id,
                left: final_bus_left,
                right: final_bus_right,
            });
        }

        // Sum all bus outputs
        for bus_output in &self.bus_outputs {
            mixed_left += bus_output.left;
            mixed_right += bus_output.right;
        }

        // Apply master effects (stereo processing) - no sidechaining on master
        let (master_left, master_right) = self.master.process_stereo(
            mixed_left,
            mixed_right,
            sample_rate,
            time,
            self.sample_count,
            None,
        );

        // Apply soft clipping to prevent harsh distortion
        // tanh provides smooth saturation - maintains dynamics while preventing clipping
        (master_left.tanh(), master_right.tanh())
    }

    /// Process a single track and return its stereo output (static version)
    ///
    /// This is a helper method extracted from the main mixing loop.
    /// It handles event synthesis, filtering, effects, and panning for one track.
    pub(crate) fn process_track_static(track: &mut Track, time: f32, sample_rate: f32, sample_count: u64) -> (f32, f32) {
        // Ensure events are sorted by start_time for binary search
        track.ensure_sorted();

        // Quick time-bounds check: skip entire track if current time is outside its active range
        let track_start = track.start_time();
        let track_end = track.end_time();

        // Skip track entirely if we're before it starts or after it ends
        if (time < track_start || time > track_end)
            && track.effects.delay.is_none()
            && track.effects.reverb.is_none()
        {
            return (0.0, 0.0);
        }

        let mut track_value = 0.0;
        let mut has_active_event = false;

        // Binary search to find potentially active events
        let (start_idx, end_idx) = track.find_active_range(time);

        // Process events
        for event in &track.events[start_idx..end_idx] {
            match event {
                AudioEvent::Note(note_event) => {
                    let total_duration = note_event.envelope.total_duration(note_event.duration);
                    let note_end_with_release = note_event.start_time + total_duration;

                    if time >= note_event.start_time && time < note_end_with_release {
                        has_active_event = true;
                        let time_in_note = time - note_event.start_time;
                        let envelope_amp =
                            note_event.envelope.amplitude_at(time_in_note, note_event.duration);

                        for i in 0..note_event.num_freqs {
                            let base_freq = note_event.frequencies[i];

                            let freq = if note_event.pitch_bend_semitones != 0.0 {
                                let bend_progress = (time_in_note / note_event.duration).min(1.0);
                                let bend_multiplier = 2.0f32
                                    .powf((note_event.pitch_bend_semitones * bend_progress) / 12.0);
                                base_freq * bend_multiplier
                            } else {
                                base_freq
                            };

                            let sample = if note_event.fm_params.mod_index > 0.0 {
                                note_event.fm_params.sample(freq, time_in_note, note_event.duration)
                            } else if let Some(ref wavetable) = note_event.custom_wavetable {
                                let phase = (time_in_note * freq) % 1.0;
                                wavetable.sample(phase)
                            } else {
                                let phase = (time_in_note * freq) % 1.0;
                                note_event.waveform.sample(phase)
                            };

                            track_value += sample * envelope_amp;
                        }
                    }
                }
                AudioEvent::Drum(drum_event) => {
                    let drum_duration = drum_event.drum_type.duration();
                    if time >= drum_event.start_time && time < drum_event.start_time + drum_duration
                    {
                        has_active_event = true;
                        let time_in_drum = time - drum_event.start_time;
                        let sample_index = (time_in_drum * sample_rate) as usize;
                        track_value += drum_event.drum_type.sample(sample_index, sample_rate);
                    }
                }
                AudioEvent::Sample(sample_event) => {
                    let time_in_sample = time - sample_event.start_time;
                    let sample_duration = sample_event.sample.duration / sample_event.playback_rate;

                    if time_in_sample >= 0.0 && time_in_sample < sample_duration {
                        has_active_event = true;
                        let (sample_left, sample_right) = sample_event
                            .sample
                            .sample_at_interpolated(time_in_sample, sample_event.playback_rate);
                        track_value += (sample_left + sample_right) * 0.5 * sample_event.volume;
                    }
                }
                _ => {} // Tempo/time/key signatures don't generate audio
            }
        }

        // Skip effect processing if track has no active events and no tail effects
        if !has_active_event && track.effects.delay.is_none() && track.effects.reverb.is_none() {
            return (0.0, 0.0);
        }

        // Apply track volume
        track_value *= track.volume;

        // Apply filter
        track_value = track.filter.process(track_value, sample_rate);

        // Apply effects through the unified effect chain
        track_value = track
            .effects
            .process_mono(track_value, sample_rate, time, sample_count);

        // Apply stereo panning using constant power panning
        let pan_angle = (track.pan + 1.0) * 0.25 * std::f32::consts::PI;
        let left_gain = pan_angle.cos();
        let right_gain = pan_angle.sin();

        (track_value * left_gain, track_value * right_gain)
    }

    /// Render the mixer to an in-memory stereo buffer
    ///
    /// Pre-renders the entire composition to a Vec of interleaved stereo samples (left, right, left, right...).
    /// This is used for efficient playback without real-time synthesis overhead.
    ///
    /// # Arguments
    /// * `sample_rate` - Sample rate in Hz
    ///
    /// # Returns
    /// A Vec of f32 samples in interleaved stereo format (left, right, left, right...)
    pub fn render_to_buffer(&mut self, sample_rate: f32) -> Vec<f32> {
        let duration = self.total_duration();
        let total_samples = (duration * sample_rate).ceil() as usize;

        // Pre-allocate buffer for interleaved stereo (2 channels)
        let mut buffer = Vec::with_capacity(total_samples * 2);

        let mut sample_clock = 0.0;

        // Render all samples
        for i in 0..total_samples {
            let time = i as f32 / sample_rate;
            let (left, right) = self.sample_at(time, sample_rate, sample_clock, None, None);

            // Clamp to valid range and add to buffer
            buffer.push(left.clamp(-1.0, 1.0));
            buffer.push(right.clamp(-1.0, 1.0));

            sample_clock = (sample_clock + 1.0) % sample_rate;
        }

        buffer
    }

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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::AutoPan;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_autopan(AutoPan::new(0.25, 1.0));
    /// ```
    pub fn master_autopan(&mut self, autopan: crate::synthesis::effects::AutoPan) {
        self.master.autopan = Some(autopan);
        self.master.compute_effect_order();
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new(Tempo::new(120.0))
    }
}
