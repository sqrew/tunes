//! Envelope cache for sidechaining support.
//!
//! Stores RMS envelope values for tracks and buses during mixing.

use crate::track::ids::{BusId, TrackId};

/// Envelope cache for sidechaining (OPTIMIZED: Vec-based for O(1) access)
///
/// Stores RMS envelope values for tracks and buses during a single sample_at() call.
/// This allows sidechained effects to access the envelope of their source signal.
///
/// **Performance:** Uses Vec indexed by ID instead of HashMap with String keys.
/// This eliminates string hashing and allocation, providing O(1) direct access.
///
/// **Lazy clearing:** Uses generation counters for O(1) clear instead of O(n) fill.
/// Each slot tracks when it was last written; stale slots return 0.0.
#[derive(Debug, Clone)]
pub(crate) struct EnvelopeCache {
    tracks: Vec<f32>,     // Track ID -> RMS envelope (direct index)
    buses: Vec<f32>,      // Bus ID -> RMS envelope (direct index)
    track_gens: Vec<u64>, // Track ID -> generation when last written
    bus_gens: Vec<u64>,   // Bus ID -> generation when last written
    generation: u64,      // Current generation (incremented on clear)
}

impl EnvelopeCache {
    /// Create a new envelope cache with pre-allocated capacity
    ///
    /// # Arguments
    /// * `max_tracks` - Maximum number of tracks to support
    /// * `max_buses` - Maximum number of buses to support
    pub fn new(max_tracks: usize, max_buses: usize) -> Self {
        Self {
            tracks: vec![0.0; max_tracks],
            buses: vec![0.0; max_buses],
            track_gens: vec![0; max_tracks],
            bus_gens: vec![0; max_buses],
            generation: 1, // Start at 1 so generation 0 means "never written"
        }
    }

    /// Clear all cached envelope values (O(1) lazy invalidation)
    ///
    /// Called at the start of each sample_at() to reset state.
    /// Instead of zeroing all values, just increment generation counter.
    /// Stale values (wrong generation) return 0.0 on read.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        // Skip generation 0 (reserved for "never written")
        if self.generation == 0 {
            self.generation = 1;
        }
    }

    /// Store a track's envelope by ID
    ///
    /// # Arguments
    /// * `track_id` - Unique track identifier
    /// * `envelope` - RMS envelope value
    #[inline(always)]
    pub fn cache_track(&mut self, track_id: TrackId, envelope: f32) {
        let idx = track_id as usize;
        if idx < self.tracks.len() {
            self.tracks[idx] = envelope;
            self.track_gens[idx] = self.generation;
        }
    }

    /// Store a bus's envelope by ID
    ///
    /// # Arguments
    /// * `bus_id` - Unique bus identifier
    /// * `envelope` - RMS envelope value
    #[inline(always)]
    pub fn cache_bus(&mut self, bus_id: BusId, envelope: f32) {
        let idx = bus_id as usize;
        if idx < self.buses.len() {
            self.buses[idx] = envelope;
            self.bus_gens[idx] = self.generation;
        }
    }

    /// Get a track's envelope by ID (returns 0.0 if not found or stale)
    #[inline(always)]
    pub fn get_track(&self, track_id: TrackId) -> f32 {
        let idx = track_id as usize;
        if idx < self.tracks.len() && self.track_gens[idx] == self.generation {
            self.tracks[idx]
        } else {
            0.0
        }
    }

    /// Get a bus's envelope by ID (returns 0.0 if not found or stale)
    #[inline(always)]
    pub fn get_bus(&self, bus_id: BusId) -> f32 {
        let idx = bus_id as usize;
        if idx < self.buses.len() && self.bus_gens[idx] == self.generation {
            self.buses[idx]
        } else {
            0.0
        }
    }

    /// Expand capacity for buses if needed
    pub fn expand_buses(&mut self, new_size: usize) {
        if new_size >= self.buses.len() {
            self.buses.resize(new_size + 1, 0.0);
            self.bus_gens.resize(new_size + 1, 0);
        }
    }
}
