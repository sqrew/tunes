//! Mixer implementation
//!
//! The mixer combines multiple buses together and handles the core audio rendering.
//! Each bus contains tracks, and buses are mixed through the master chain.

use super::bus::{Bus, BusBuilder};
use super::events::*;
use super::track::Track;
use crate::cache::{CacheKey, SampleCache, CachedSample};
use crate::composition::timing::Tempo;
#[cfg(feature = "gpu")]
use crate::gpu::GpuSynthesizer;
use crate::synthesis::effects::{EffectChain, ResolvedSidechainSource};
use crate::track::ids::{BusId, TrackId};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Envelope cache for sidechaining (OPTIMIZED: Vec-based for O(1) access)
///
/// Stores RMS envelope values for tracks and buses during a single sample_at() call.
/// This allows sidechained effects to access the envelope of their source signal.
///
/// **Performance:** Uses Vec indexed by ID instead of HashMap with String keys.
/// This eliminates string hashing and allocation, providing O(1) direct access.
#[derive(Debug, Clone)]
struct EnvelopeCache {
    tracks: Vec<f32>, // Track ID -> RMS envelope (direct index)
    buses: Vec<f32>,  // Bus ID -> RMS envelope (direct index)
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
#[allow(dead_code)]
struct TrackOutput {
    bus_id: BusId, // Which bus this track belongs to (INTEGER!)
    left: f32,     // Left channel output
    right: f32,    // Right channel output
    envelope: f32, // RMS envelope for sidechaining
}

/// Pre-allocated bus output (avoids allocation in hot path)
///
/// Stores the output of a single bus for later master mixing.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct BusOutput {
    bus_id: BusId, // Bus identifier (unused, kept for potential future use)
    left: f32,     // Left channel output
    right: f32,    // Right channel output
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
    pub(super) buses: Vec<Option<Bus>>, // Sparse Vec: Some(bus) at bus.id index, None otherwise
    bus_order: Vec<BusId>,              // Order in which to process buses

    // Cold path: String lookup for user-facing API
    bus_name_to_id: HashMap<String, BusId>,

    // Pre-allocated buffers (reused every sample_at() call)
    track_outputs: Vec<TrackOutput>,
    bus_outputs: Vec<BusOutput>,
    envelope_cache: EnvelopeCache,

    // Sample cache for pre-rendered synthesis (thread-safe for Rayon)
    cache: Option<Arc<Mutex<SampleCache>>>,

    // GPU synthesizer for experimental acceleration (optional, falls back to CPU)
    #[cfg(feature = "gpu")]
    gpu_synthesizer: Option<Arc<GpuSynthesizer>>,

    // Track if we've pre-rendered notes (skip cache checks during streaming)
    prerendered: bool,

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
            return self.buses[bus_id as usize].as_mut()
                .expect("Internal error: bus_name_to_id points to empty bus slot");
        }

        // Bus doesn't exist, create it
        let new_bus_id = self.buses.len() as BusId; // Use current length as new ID
        let new_bus = Bus::new(new_bus_id, name.to_string());

        self.add_bus(new_bus);

        // Return reference to the newly added bus
        self.buses[new_bus_id as usize].as_mut()
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
        source: &crate::synthesis::effects::SidechainSource,
        track_name_to_id: &HashMap<String, TrackId>,
        bus_name_to_id: &HashMap<String, BusId>,
    ) -> Option<ResolvedSidechainSource> {
        use crate::synthesis::effects::{ResolvedSidechainSource, SidechainSource};

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

    /// Enable sample caching with default settings
    ///
    /// This enables automatic caching of synthesized notes, dramatically improving
    /// performance when the same synthesis parameters are used multiple times.
    ///
    /// Default cache settings:
    /// - 500 MB memory limit
    /// - Only cache sounds > 100ms duration
    /// - LRU eviction when full
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// let mut mixer = Composition::new(Tempo::new(120.0)).into_mixer();
    /// mixer.enable_cache();
    /// ```
    pub fn enable_cache(&mut self) -> &mut Self {
        self.cache = Some(Arc::new(Mutex::new(SampleCache::new())));
        self
    }

    /// Enable sample caching with custom settings
    ///
    /// # Arguments
    /// * `cache` - Pre-configured SampleCache
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # use tunes::cache::SampleCache;
    /// let cache = SampleCache::new()
    ///     .with_max_size_mb(1000)
    ///     .with_min_duration_ms(50.0);
    ///
    /// let mut mixer = Composition::new(Tempo::new(120.0)).into_mixer();
    /// mixer.enable_cache_with(cache);
    /// ```
    pub fn enable_cache_with(&mut self, cache: SampleCache) -> &mut Self {
        self.cache = Some(Arc::new(Mutex::new(cache)));
        self
    }

    /// Disable sample caching
    pub fn disable_cache(&mut self) -> &mut Self {
        self.cache = None;
        self
    }

    /// Get cache statistics (if caching is enabled)
    ///
    /// Returns `None` if caching is disabled.
    pub fn cache_stats(&self) -> Option<crate::cache::storage::CacheStats> {
        self.cache.as_ref().map(|c| c.lock().unwrap_or_else(|e| e.into_inner()).stats().clone())
    }

    /// Print cache statistics (if caching is enabled)
    pub fn print_cache_stats(&self) {
        if let Some(cache) = &self.cache {
            cache.lock().unwrap_or_else(|e| e.into_inner()).print_stats();
        } else {
            println!("Sample caching is disabled");
        }
    }

    /// Clear the sample cache (if caching is enabled)
    pub fn clear_cache(&mut self) {
        if let Some(cache) = &mut self.cache {
            cache.lock().unwrap_or_else(|e| e.into_inner()).clear();
        }
    }

    /// Enable GPU-accelerated synthesis
    ///
    /// This enables GPU compute shaders for potentially 500-1000x faster synthesis
    /// **on discrete GPUs**. Performance depends heavily on GPU hardware:
    ///
    /// - **Discrete GPUs** (RTX 3060+, RX 6000+): 50-500x faster than CPU
    /// - **Integrated GPUs** (Intel HD/UHD): May be slower than CPU
    /// - **No GPU**: Automatic fallback to fast CPU synthesis
    ///
    /// **Note**: GPU acceleration works best with:
    /// - Large workloads (100+ unique sounds)
    /// - Complex synthesis (multi-oscillator FM, filters)
    /// - Discrete graphics cards
    ///
    /// **Important**: GPU synthesis requires caching to be enabled.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// let mut mixer = Composition::new(Tempo::new(120.0)).into_mixer();
    /// mixer.enable_cache();  // Required!
    /// mixer.enable_gpu();    // Try GPU acceleration
    /// ```
    #[cfg(feature = "gpu")]
    pub fn enable_gpu(&mut self) -> &mut Self {
        self.enable_gpu_with_output(true)
    }

    /// Enable GPU with optional console output
    #[cfg(feature = "gpu")]
    pub fn enable_gpu_with_output(&mut self, print_info: bool) -> &mut Self {
        use crate::gpu::{GpuDevice, GpuSynthesizer};

        // Try to initialize GPU
        match GpuDevice::new() {
            Ok(device) => {
                match GpuSynthesizer::new(device) {
                    Ok(synthesizer) => {
                        self.gpu_synthesizer = Some(Arc::new(synthesizer));
                        if print_info {
                            println!("✅ GPU synthesis enabled");
                        }
                    }
                    Err(e) => {
                        if print_info {
                            eprintln!("⚠️  Failed to create GPU synthesizer: {}", e);
                            eprintln!("   Falling back to CPU synthesis");
                        }
                    }
                }
            }
            Err(e) => {
                if print_info {
                    eprintln!("⚠️  GPU not available: {}", e);
                    eprintln!("   Using CPU synthesis");
                }
            }
        }

        self
    }

    /// Disable GPU synthesis (fall back to CPU)
    #[cfg(feature = "gpu")]
    pub fn disable_gpu(&mut self) -> &mut Self {
        self.gpu_synthesizer = None;
        self
    }

    /// Check if GPU synthesis is enabled
    #[cfg(feature = "gpu")]
    pub fn gpu_enabled(&self) -> bool {
        self.gpu_synthesizer.is_some()
    }

    /// Enable both cache and GPU acceleration in one call
    ///
    /// This is a convenience wrapper that enables both the sample cache and GPU
    /// compute shaders. Since GPU acceleration requires caching to be effective,
    /// this is the recommended way to enable maximum performance.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums").note(&[C4], 0.5);
    ///
    /// let mut mixer = comp.into_mixer();
    /// mixer.enable_cache_and_gpu();  // Experimental GPU acceleration
    ///
    /// // Now export or play with GPU acceleration
    /// # let engine = AudioEngine::new()?;
    /// engine.export_wav(&mut mixer, "output.wav")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    #[cfg(feature = "gpu")]
    pub fn enable_cache_and_gpu(&mut self) -> &mut Self {
        self.enable_cache();
        self.enable_gpu();
        self
    }

    /// Pre-render all unique notes in the composition
    ///
    /// This scans all tracks, finds unique notes, and batch renders them on GPU
    /// (or CPU fallback). This is called automatically before streaming if GPU or
    /// cache is enabled, eliminating per-block cache lookups.
    ///
    /// **This is the key to GPU performance!** Instead of checking cache during
    /// streaming (causing overhead), we render everything upfront and stream
    /// from a fully-populated cache.
    pub fn prerender_notes(&mut self, sample_rate: f32) {
        use std::collections::HashSet;

        // Only prerender if cache is enabled
        let cache = match &self.cache {
            Some(c) => c,
            None => return,
        };

        println!("🔄 Pre-rendering unique notes...");

        // Collect all unique notes from all tracks
        let mut unique_notes: HashMap<CacheKey, NoteEvent> = HashMap::new();

        for bus_opt in &self.buses {
            if let Some(bus) = bus_opt {
                for track in &bus.tracks {
                    for event in &track.events {
                        if let AudioEvent::Note(note_event) = event {
                            let cache_key = CacheKey::from_note_event(note_event, sample_rate);

                            // Only add if not already seen
                            if !unique_notes.contains_key(&cache_key) {
                                unique_notes.insert(cache_key, note_event.clone());
                            }
                        }
                    }
                }
            }
        }

        let total_notes = unique_notes.len();
        println!("   Found {} unique notes to render", total_notes);

        // Batch render all unique notes
        let start = std::time::Instant::now();
        let mut rendered_count = 0;

        for (cache_key, note_event) in unique_notes {
            // Check if already cached
            if cache.lock().unwrap_or_else(|e| e.into_inner()).get(&cache_key).is_some() {
                continue; // Already in cache
            }

            // Render the note (GPU if available, CPU fallback)
            let total_duration = note_event.envelope.total_duration(note_event.duration);

            if total_duration > 0.0 && total_duration < 10.0 {
                let rendered_samples = Self::render_note_to_buffer(
                    &note_event,
                    sample_rate,
                    #[cfg(feature = "gpu")]
                    self.gpu_synthesizer.as_ref(),
                );

                let cached_sample = CachedSample::new(
                    rendered_samples,
                    sample_rate,
                    total_duration,
                    note_event.frequencies[0],
                );

                cache.lock().unwrap_or_else(|e| e.into_inner()).insert(cache_key, cached_sample);
                rendered_count += 1;
            }
        }

        let elapsed = start.elapsed();

        #[cfg(feature = "gpu")]
        {
            if self.gpu_enabled() {
                let notes_per_second = rendered_count as f32 / elapsed.as_secs_f32();
                println!("   ✅ Pre-rendered {} notes in {:.3}s ({:.0} notes/sec)",
                    rendered_count, elapsed.as_secs_f32(), notes_per_second);
            } else {
                println!("   ✅ Pre-rendered {} notes in {:.3}s", rendered_count, elapsed.as_secs_f32());
            }
        }
        #[cfg(not(feature = "gpu"))]
        {
            println!("   ✅ Pre-rendered {} notes in {:.3}s", rendered_count, elapsed.as_secs_f32());
        }

        // Mark as pre-rendered so streaming skips cache lookups
        self.prerendered = true;
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
                let (track_left, track_right) =
                    Self::process_track_static(track, time, sample_rate, sample_count);

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
            let pan_left = if bus.pan <= 0.0 { 1.0 } else { 1.0 - bus.pan };
            let pan_right = if bus.pan >= 0.0 { 1.0 } else { 1.0 + bus.pan };

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

    /// Process a block of samples
    ///
    /// This is the new block-based processing API that processes multiple samples at once,
    /// significantly reducing function call overhead and enabling future optimizations.
    ///
    /// # Arguments
    /// * `buffer` - Interleaved stereo buffer [L0, R0, L1, R1, ...] to fill
    /// * `sample_rate` - Sample rate in Hz
    /// * `start_time` - Starting time in seconds
    /// * `listener` - Optional spatial audio listener configuration
    /// * `spatial_params` - Optional spatial audio parameters
    #[allow(unused_variables)]
    pub fn process_block(
        &mut self,
        buffer: &mut [f32],
        sample_rate: f32,
        start_time: f32,
        listener: Option<&crate::synthesis::spatial::ListenerConfig>,
        spatial_params: Option<&crate::synthesis::spatial::SpatialParams>,
    ) {
        // Clear output buffer
        buffer.fill(0.0);

        let num_frames = buffer.len() / 2;
        let start_sample_count = self.sample_count;
        self.sample_count = self.sample_count.wrapping_add(num_frames as u64);

        // Clear envelope cache for this block
        self.envelope_cache.clear();

        // Temporary mono buffer for track processing (will be reused)
        let mut track_buffer = vec![0.0f32; num_frames];

        // Temporary stereo buffers for bus outputs
        let mut bus_buffer = vec![0.0f32; buffer.len()];

        // TWO-PASS BUS PROCESSING for parallelization with sidechain support:
        // Pass 1: Render all bus audio + calculate envelopes (can be parallel)
        // Pass 2: Apply effects + mix to output (can be parallel, envelopes now available)

        // PASS 1: Render bus audio and calculate envelopes in PARALLEL
        struct BusRenderResult {
            bus_id: BusId,
            bus_buffer: Vec<f32>,
            bus_envelope: f32,
            track_envelopes: Vec<(TrackId, f32)>,
        }

        let bus_results: Vec<BusRenderResult> = self.buses
            .par_iter_mut()
            .filter_map(|bus_opt| {
                let bus = bus_opt.as_mut()?;
                if bus.muted {
                    return None;
                }

                let bus_id = bus.id;
                let mut bus_buffer = vec![0.0f32; buffer.len()];

                // Clone the Arc to share the cache across threads (cheap - just incrementing ref count)
                let cache_clone = self.cache.clone();
                #[cfg(feature = "gpu")]
                let gpu_clone = self.gpu_synthesizer.clone();
                let prerendered = self.prerendered;

                // Process each track in this bus IN PARALLEL using Rayon
                let track_results: Vec<_> = bus.tracks
                    .par_iter_mut()
                    .map(|track| {
                        let track_id = track.id;
                        let mut track_buffer = vec![0.0f32; num_frames];

                        // Generate mono track audio using block processing
                        // Cache is thread-safe via Arc<Mutex>, GPU synthesizer via Arc
                        Self::process_track_block(
                            track,
                            &mut track_buffer,
                            sample_rate,
                            start_time,
                            start_sample_count,
                            cache_clone.as_ref(),
                            #[cfg(feature = "gpu")]
                            gpu_clone.as_ref(),
                            prerendered,
                        );

                        // Calculate RMS envelope for this track
                        let mut sum_squares = 0.0;
                        for &sample in track_buffer.iter() {
                            sum_squares += sample * sample;
                        }
                        let track_envelope = (sum_squares / num_frames as f32).sqrt();

                        (track_id, track_buffer, track_envelope, track.pan)
                    })
                    .collect();

                // Mix track results into bus buffer
                let mut track_envelopes = Vec::new();
                for (track_id, track_buffer, track_envelope, pan) in track_results {
                    track_envelopes.push((track_id, track_envelope));

                    // Apply stereo panning and mix
                    let pan_angle = (pan + 1.0) * 0.25 * std::f32::consts::PI;
                    let left_gain = pan_angle.cos();
                    let right_gain = pan_angle.sin();

                    for (frame_idx, &mono_sample) in track_buffer.iter().enumerate() {
                        let stereo_idx = frame_idx * 2;
                        bus_buffer[stereo_idx] += mono_sample * left_gain;
                        bus_buffer[stereo_idx + 1] += mono_sample * right_gain;
                    }
                }

                // Calculate bus envelope (before effects)
                let mut bus_sum_squares = 0.0;
                for chunk in bus_buffer.chunks_exact(2) {
                    let left = chunk[0];
                    let right = chunk[1];
                    bus_sum_squares += (left * left + right * right) / 2.0;
                }
                let bus_envelope = (bus_sum_squares / num_frames as f32).sqrt();

                Some(BusRenderResult {
                    bus_id,
                    bus_buffer,
                    bus_envelope,
                    track_envelopes,
                })
            })
            .collect();

        // Cache all track and bus envelopes (now safe since all are computed)
        for result in &bus_results {
            for (track_id, envelope) in &result.track_envelopes {
                self.envelope_cache.cache_track(*track_id, *envelope);
            }
            self.envelope_cache.cache_bus(result.bus_id, result.bus_envelope);
        }

        // PASS 2: Apply effects and mix to output (can still check for parallelization)
        // Note: We keep this sequential for now since effects have state, but envelopes are cached
        for result in bus_results {
            let mut bus_buffer = result.bus_buffer;

            // Find the original bus to apply effects
            let bus = self.buses
                .iter_mut()
                .find_map(|b| b.as_mut().filter(|bus| bus.id == result.bus_id))
                .expect("Bus should exist");

            // Look up sidechain envelope (now safe - all envelopes cached in pass 1)
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

            // Apply bus effects
            bus.effects.process_stereo_block(
                &mut bus_buffer,
                sample_rate,
                start_time,
                start_sample_count,
                sidechain_env,
            );

            // Mix into output buffer
            let bus_pan_angle = (bus.pan + 1.0) * 0.25 * std::f32::consts::PI;
            let bus_left_gain = bus_pan_angle.cos() * bus.volume;
            let bus_right_gain = bus_pan_angle.sin() * bus.volume;

            for (idx, sample) in bus_buffer.iter().enumerate() {
                if idx % 2 == 0 {
                    buffer[idx] += sample * bus_left_gain;
                } else {
                    buffer[idx] += sample * bus_right_gain;
                }
            }
        }

        // Look up master sidechain envelope if configured
        let master_sidechain_env = if let Some(ref compressor) = self.master.compressor {
            if let Some(ref resolved_source) = compressor.resolved_sidechain_source {
                match resolved_source {
                    ResolvedSidechainSource::Track(track_id) => {
                        Some(self.envelope_cache.get_track(*track_id))
                    }
                    ResolvedSidechainSource::Bus(bus_id) => {
                        Some(self.envelope_cache.get_bus(*bus_id))
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        // Apply master effects (block processing) with sidechain support
        self.master.process_stereo_block(
            buffer,
            sample_rate,
            start_time,
            start_sample_count,
            master_sidechain_env,
        );
    }

    /// Render a complete note into a buffer
    ///
    /// This synthesizes an entire note from start to finish, used for caching.
    /// If GPU synthesizer is provided, uses GPU, otherwise falls back to CPU.
    fn render_note_to_buffer(
        note: &NoteEvent,
        sample_rate: f32,
        #[cfg(feature = "gpu")]
        gpu_synthesizer: Option<&Arc<GpuSynthesizer>>,
    ) -> Vec<f32> {
        // Try GPU first if available
        #[cfg(feature = "gpu")]
        if let Some(gpu) = gpu_synthesizer {
            if let Ok(samples) = gpu.synthesize_note(note, sample_rate) {
                return samples;
            }
            // GPU failed, fall through to CPU
        }

        // CPU synthesis fallback
        let total_duration = note.envelope.total_duration(note.duration);
        let num_samples = (total_duration * sample_rate) as usize;
        let mut buffer = vec![0.0f32; num_samples];

        let time_delta = 1.0 / sample_rate;

        for (i, sample_out) in buffer.iter_mut().enumerate() {
            let time_in_note = i as f32 * time_delta;
            let envelope_amp = note.envelope.amplitude_at(time_in_note, note.duration);

            let mut note_value = 0.0;

            // Synthesize for the first frequency only (monophonic cache)
            // Polyphonic notes will be handled separately
            if note.num_freqs > 0 {
                let base_freq = note.frequencies[0];

                let freq = if note.pitch_bend_semitones != 0.0 {
                    let bend_progress = (time_in_note / note.duration).min(1.0);
                    let bend_multiplier = 2.0f32.powf(
                        (note.pitch_bend_semitones * bend_progress) / 12.0,
                    );
                    base_freq * bend_multiplier
                } else {
                    base_freq
                };

                let sample = if note.fm_params.mod_index > 0.0 {
                    note.fm_params.sample(freq, time_in_note, note.duration)
                } else if let Some(ref wavetable) = note.custom_wavetable {
                    let phase = (time_in_note * freq) % 1.0;
                    wavetable.sample(phase)
                } else {
                    let phase = (time_in_note * freq) % 1.0;
                    note.waveform.sample(phase)
                };

                note_value += sample * envelope_amp;
            }

            *sample_out = note_value;
        }

        buffer
    }

    /// Process a single track into a mono buffer (block-processing version)
    ///
    /// This is the high-performance version that generates multiple samples at once,
    /// reducing function call overhead and enabling better cache locality.
    ///
    /// # Arguments
    /// * `track` - The track to process
    /// * `buffer` - Output mono buffer to fill
    /// * `sample_rate` - Sample rate in Hz
    /// * `start_time` - Starting time for the block
    /// * `start_sample_count` - Starting sample counter
    /// * `cache` - Optional sample cache for pre-rendered synthesis
    /// * `gpu_synthesizer` - Optional GPU synthesizer for 500-1000x faster rendering
    /// * `prerendered` - If true, skip cache-miss detection (already pre-rendered)
    pub(crate) fn process_track_block(
        track: &mut Track,
        buffer: &mut [f32],
        sample_rate: f32,
        start_time: f32,
        start_sample_count: u64,
        cache: Option<&Arc<Mutex<SampleCache>>>,
        #[cfg(feature = "gpu")]
        gpu_synthesizer: Option<&Arc<GpuSynthesizer>>,
        prerendered: bool,
    ) {
        // Clear output buffer
        buffer.fill(0.0);

        // Ensure events are sorted by start_time for binary search
        track.ensure_sorted();

        let track_start = track.start_time();
        let track_end = track.end_time();
        let time_delta = 1.0 / sample_rate;
        let block_duration = buffer.len() as f32 * time_delta;
        let block_end_time = start_time + block_duration;

        // Skip track entirely if we're completely outside its active range
        if (block_end_time < track_start || start_time > track_end)
            && track.effects.delay.is_none()
            && track.effects.reverb.is_none()
        {
            return;
        }

        // Binary search ONCE to find events that might be active during this block
        // We need to search at the start of the block
        let (start_idx, end_idx) = track.find_active_range(start_time);

        // Check cache for NoteEvents and render on miss (if cache is enabled)
        // SKIP if already pre-rendered (batch rendering already populated cache)
        if !prerendered && cache.is_some() {
            let cache_arc = cache.unwrap();
            // Handle poisoned mutex gracefully (don't panic in audio thread)
            let mut cache_lock = cache_arc.lock().unwrap_or_else(|e| e.into_inner());
            for event in &track.events[start_idx..end_idx] {
                if let AudioEvent::Note(note_event) = event {
                    // Compute cache key for this note
                    let cache_key = CacheKey::from_note_event(note_event, sample_rate);

                    // Check if we have this note cached
                    if cache_lock.get(&cache_key).is_none() {
                        // Cache miss - render the full note and store it
                        let total_duration = note_event.envelope.total_duration(note_event.duration);

                        // Only cache notes with reasonable duration
                        if total_duration > 0.0 && total_duration < 10.0 {
                            // Render the complete note (GPU if available, otherwise CPU)
                            let rendered_samples = Self::render_note_to_buffer(
                                note_event,
                                sample_rate,
                                #[cfg(feature = "gpu")]
                                gpu_synthesizer,
                            );

                            let cached_sample = CachedSample::new(
                                rendered_samples,
                                sample_rate,
                                total_duration,
                                note_event.frequencies[0],  // Reference frequency
                            );
                            cache_lock.insert(cache_key, cached_sample);
                        }
                    }
                    // On cache hit: cached sample will be used during playback below
                }
            }
        }

        // Build a set of cached note indices to avoid per-sample cache lookups
        // SKIP if pre-rendered (all notes are cached, mark them all as cached)
        let mut cached_note_indices = std::collections::HashSet::new();
        if prerendered && cache.is_some() {
            // All notes are pre-rendered, mark ALL NoteEvents as cached
            for (idx, event) in track.events[start_idx..end_idx].iter().enumerate() {
                if let AudioEvent::Note(_) = event {
                    cached_note_indices.insert(start_idx + idx);
                }
            }
        } else if let Some(cache_arc) = cache {
            // Not pre-rendered, check cache for each note (slower path)
            // Handle poisoned mutex gracefully (don't panic in audio thread)
            let mut cache_lock = cache_arc.lock().unwrap_or_else(|e| e.into_inner());
            for (idx, event) in track.events[start_idx..end_idx].iter().enumerate() {
                if let AudioEvent::Note(note_event) = event {
                    let cache_key = CacheKey::from_note_event(note_event, sample_rate);
                    if cache_lock.get(&cache_key).is_some() {
                        cached_note_indices.insert(start_idx + idx);
                    }
                }
            }
        }

        // Pre-render cached notes into buffer (if cache is enabled)
        let mut cached_notes_buffer = vec![0.0f32; buffer.len()];
        if let Some(cache_arc) = cache {
            // Handle poisoned mutex gracefully (don't panic in audio thread)
            let mut cache_lock = cache_arc.lock().unwrap_or_else(|e| e.into_inner());
            for (idx, event) in track.events[start_idx..end_idx].iter().enumerate() {
                if let AudioEvent::Note(note_event) = event {
                    let cache_key = CacheKey::from_note_event(note_event, sample_rate);

                    if let Some(cached_sample) = cache_lock.get(&cache_key) {
                        // Cache hit! Use the cached sample
                        let note_start = note_event.start_time;
                        let note_end = note_start + cached_sample.duration;

                        // Skip if note doesn't overlap with current block
                        if note_end < start_time || note_start >= start_time + (buffer.len() as f32 / sample_rate) {
                            continue;
                        }

                        // Compute sample ranges (buffer space and cached sample space)
                        let time_offset_in_note = (start_time - note_start).max(0.0);
                        let cache_start_sample = (time_offset_in_note * sample_rate) as usize;

                        let buffer_start_sample = if note_start > start_time {
                            ((note_start - start_time) * sample_rate) as usize
                        } else {
                            0
                        };

                        // Calculate how many samples to copy
                        let samples_remaining_in_cache = cached_sample.samples.len().saturating_sub(cache_start_sample);
                        let samples_remaining_in_buffer = buffer.len().saturating_sub(buffer_start_sample);
                        let num_samples_to_copy = samples_remaining_in_cache.min(samples_remaining_in_buffer);

                        // Bulk copy with addition (vectorizable)
                        if num_samples_to_copy > 0 && cache_start_sample < cached_sample.samples.len() {
                            for i in 0..num_samples_to_copy {
                                cached_notes_buffer[buffer_start_sample + i] +=
                                    cached_sample.samples[cache_start_sample + i];
                            }
                        }
                    }
                }
            }
        }

        // Pre-render sample events with SIMD for better performance
        // This processes whole blocks instead of per-sample, enabling vectorization
        let mut sample_buffer = vec![0.0f32; buffer.len()];
        for event in &track.events[start_idx..end_idx] {
            if let AudioEvent::Sample(sample_event) = event {
                sample_event.sample.fill_buffer_simd_mono(
                    &mut sample_buffer,
                    sample_event.start_time,
                    start_time,
                    time_delta,
                    sample_event.playback_rate,
                    sample_event.volume,
                );
            }
        }

        // For each sample in the block
        for (i, sample_out) in buffer.iter_mut().enumerate() {
            let time = start_time + (i as f32 * time_delta);
            let mut track_value = 0.0;

            // Process events (reuse binary search result for entire block)
            for (relative_idx, event) in track.events[start_idx..end_idx].iter().enumerate() {
                let absolute_idx = start_idx + relative_idx;

                match event {
                    AudioEvent::Note(note_event) => {
                        // Check if this note is cached using the pre-built HashSet (O(1) lookup, no mutex!)
                        // We built cached_note_indices earlier specifically to avoid cache locking in this hot loop
                        if cached_note_indices.contains(&absolute_idx) {
                            // Skip - already rendered in cached_notes_buffer
                            continue;
                        }

                        let total_duration =
                            note_event.envelope.total_duration(note_event.duration);
                        let note_end_with_release = note_event.start_time + total_duration;

                        if time >= note_event.start_time && time < note_end_with_release {
                            let time_in_note = time - note_event.start_time;
                            let envelope_amp = note_event
                                .envelope
                                .amplitude_at(time_in_note, note_event.duration);

                            for freq_idx in 0..note_event.num_freqs {
                                let base_freq = note_event.frequencies[freq_idx];

                                let freq = if note_event.pitch_bend_semitones != 0.0 {
                                    let bend_progress =
                                        (time_in_note / note_event.duration).min(1.0);
                                    let bend_multiplier = 2.0f32.powf(
                                        (note_event.pitch_bend_semitones * bend_progress) / 12.0,
                                    );
                                    base_freq * bend_multiplier
                                } else {
                                    base_freq
                                };

                                let sample = if note_event.fm_params.mod_index > 0.0 {
                                    note_event.fm_params.sample(
                                        freq,
                                        time_in_note,
                                        note_event.duration,
                                    )
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
                        if time >= drum_event.start_time
                            && time < drum_event.start_time + drum_duration
                        {
                            let time_in_drum = time - drum_event.start_time;
                            let sample_index = (time_in_drum * sample_rate) as usize;
                            track_value += drum_event.drum_type.sample(sample_index, sample_rate);
                        }
                    }
                    AudioEvent::Sample(_) => {
                        // Samples are pre-rendered above with SIMD for better performance
                        // They'll be added to track_value after this loop
                    }
                    _ => {} // Tempo/time/key signatures don't generate audio
                }
            }

            // Add pre-rendered samples (processed with SIMD above)
            track_value += sample_buffer[i];

            // Add cached notes (if cache is enabled)
            track_value += cached_notes_buffer[i];

            // Apply track volume
            track_value *= track.volume;

            // Apply filter (per-sample, maintains state)
            track_value = track.filter.process(track_value, sample_rate);

            *sample_out = track_value;
        }

        // Apply effects to entire buffer (block processing!)
        track
            .effects
            .process_mono_block(buffer, sample_rate, start_time, start_sample_count);
    }

    /// Process a single track and return its stereo output (static version)
    ///
    /// This is a helper method extracted from the main mixing loop.
    /// It handles event synthesis, filtering, effects, and panning for one track.
    pub(crate) fn process_track_static(
        track: &mut Track,
        time: f32,
        sample_rate: f32,
        sample_count: u64,
    ) -> (f32, f32) {
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
                        let envelope_amp = note_event
                            .envelope
                            .amplitude_at(time_in_note, note_event.duration);

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
                                note_event
                                    .fm_params
                                    .sample(freq, time_in_note, note_event.duration)
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

        // 🚀 KEY OPTIMIZATION: Pre-render all unique notes before streaming
        // This eliminates per-block cache lookups and unleashes GPU performance!
        let should_prerender = self.cache.is_some()
            || {
                #[cfg(feature = "gpu")]
                { self.gpu_synthesizer.is_some() }
                #[cfg(not(feature = "gpu"))]
                { false }
            };

        if should_prerender {
            self.prerender_notes(sample_rate);
        }

        // Pre-allocate buffer for interleaved stereo (2 channels)
        let mut buffer = vec![0.0; total_samples * 2];

        // Process in blocks of 512 samples for better performance
        const BLOCK_SIZE: usize = 512;
        let mut processed_samples = 0;

        while processed_samples < total_samples {
            let remaining = total_samples - processed_samples;
            let block_samples = remaining.min(BLOCK_SIZE);
            let block_frames = block_samples * 2; // stereo

            let start_time = processed_samples as f32 / sample_rate;
            let start_idx = processed_samples * 2;
            let end_idx = start_idx + block_frames;

            // Process this block
            self.process_block(
                &mut buffer[start_idx..end_idx],
                sample_rate,
                start_time,
                None,
                None,
            );

            processed_samples += block_samples;
        }

        // Clamp to valid range
        for sample in &mut buffer {
            *sample = sample.clamp(-1.0, 1.0);
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
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new(Tempo::new(120.0))
    }
}
