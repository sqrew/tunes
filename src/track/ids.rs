//! ID types and generators for tracks and buses
//!
//! This module provides integer-based identifiers for tracks and buses to replace
//! string-based lookups in the real-time audio path. String lookups require:
//! - Heap allocations for string cloning
//! - Hash computation for HashMap lookups
//! - String comparisons (O(n) where n = string length)
//!
//! Integer IDs are:
//! - Stack-allocated (4 bytes)
//! - Direct array indexing (O(1))
//! - Single CPU instruction comparison
//!
//! This improves real-time audio performance by eliminating allocations and
//! reducing comparison overhead from ~4M string operations/sec to integer compares.

/// Unique identifier for a bus (cheap to copy, compare)
///
/// BusId is a simple integer that uniquely identifies a bus within a Mixer.
/// Using integer IDs instead of string names eliminates string allocations
/// and enables direct Vec indexing instead of HashMap lookups.
///
/// # Example
/// ```
/// use tunes::track::ids::{BusId, BusIdGenerator};
///
/// let mut gen = BusIdGenerator::new();
/// let id1 = gen.next_id(); // 0
/// let id2 = gen.next_id(); // 1
/// assert_ne!(id1, id2);
/// ```
pub type BusId = u32;

/// Unique identifier for a track (cheap to copy, compare)
///
/// TrackId is a simple integer that uniquely identifies a track within a Composition.
/// Using integer IDs instead of string names eliminates string allocations
/// and enables faster lookups in the envelope cache.
pub type TrackId = u32;

/// Generator for unique bus IDs
///
/// Creates sequential BusId values starting from 0. Each call to `next_id()`
/// returns a new unique ID.
///
/// # Example
/// ```
/// use tunes::track::ids::BusIdGenerator;
///
/// let mut gen = BusIdGenerator::new();
/// assert_eq!(gen.next_id(), 0);
/// assert_eq!(gen.next_id(), 1);
/// assert_eq!(gen.next_id(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct BusIdGenerator {
    next_id: BusId,
}

impl BusIdGenerator {
    /// Create a new bus ID generator starting from 0
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    /// Generate the next unique bus ID
    ///
    /// Returns sequential IDs starting from 0. Each call increments
    /// the internal counter.
    pub fn next_id(&mut self) -> BusId {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        id
    }

    /// Get the next ID that would be generated without consuming it
    pub fn peek(&self) -> BusId {
        self.next_id
    }

    /// Reset the generator to start from 0 again
    pub fn reset(&mut self) {
        self.next_id = 0;
    }
}

impl Default for BusIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Generator for unique track IDs
///
/// Creates sequential TrackId values starting from 0. Each call to `next_id()`
/// returns a new unique ID.
///
/// # Example
/// ```
/// use tunes::track::ids::TrackIdGenerator;
///
/// let mut gen = TrackIdGenerator::new();
/// assert_eq!(gen.next_id(), 0);
/// assert_eq!(gen.next_id(), 1);
/// assert_eq!(gen.next_id(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct TrackIdGenerator {
    next_id: TrackId,
}

impl TrackIdGenerator {
    /// Create a new track ID generator starting from 0
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    /// Generate the next unique track ID
    ///
    /// Returns sequential IDs starting from 0. Each call increments
    /// the internal counter.
    pub fn next_id(&mut self) -> TrackId {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        id
    }

    /// Get the next ID that would be generated without consuming it
    pub fn peek(&self) -> TrackId {
        self.next_id
    }

    /// Reset the generator to start from 0 again
    pub fn reset(&mut self) {
        self.next_id = 0;
    }
}

impl Default for TrackIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bus_id_generator() {
        let mut gen = BusIdGenerator::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        assert_eq!(gen.next_id(), 2);
    }

    #[test]
    fn test_track_id_generator() {
        let mut gen = TrackIdGenerator::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        assert_eq!(gen.next_id(), 2);
    }

    #[test]
    fn test_bus_id_generator_peek() {
        let mut gen = BusIdGenerator::new();
        assert_eq!(gen.peek(), 0);
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.peek(), 1);
        assert_eq!(gen.next_id(), 1);
    }

    #[test]
    fn test_bus_id_generator_reset() {
        let mut gen = BusIdGenerator::new();
        gen.next_id();
        gen.next_id();
        gen.reset();
        assert_eq!(gen.next_id(), 0);
    }

    #[test]
    fn test_track_id_generator_wrapping() {
        let mut gen = TrackIdGenerator::new();
        gen.next_id = u32::MAX;
        assert_eq!(gen.next_id(), u32::MAX);
        assert_eq!(gen.next_id(), 0); // Wraps around
    }
}
