//! Pre-allocated output types for mixer hot path.
//!
//! These types avoid allocation during audio rendering.

use crate::track::ids::BusId;

/// Pre-allocated track output (avoids allocation in hot path)
///
/// Stores the output of a single track for later bus mixing.
/// Uses integer bus_id instead of string bus_name for O(1) comparison.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub(crate) struct TrackOutput {
    pub bus_id: BusId, // Which bus this track belongs to (INTEGER!)
    pub left: f32,     // Left channel output
    pub right: f32,    // Right channel output
    pub envelope: f32, // RMS envelope for sidechaining
}

/// Pre-allocated bus output (avoids allocation in hot path)
///
/// Stores the output of a single bus for later master mixing.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub(crate) struct BusOutput {
    pub bus_id: BusId, // Bus identifier (unused, kept for potential future use)
    pub left: f32,     // Left channel output
    pub right: f32,    // Right channel output
}
