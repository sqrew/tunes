//! Rhythmic pattern generators
//!
//! This module contains algorithms for generating rhythmic patterns,
//! including Euclidean rhythms, golden ratio rhythms, Shepard tones,
//! circle maps, polyrhythms, clave patterns, additive meters, and phase shifting.

pub mod euclidean;
pub mod golden_ratio_rhythm;
pub mod shepard_tone;
pub mod circle_map;
pub mod polyrhythm;
pub mod clave;
pub mod additive_meter;
pub mod phase_shift;

// All rhythmic sequences are now modules with generate() functions
// Use as: euclidean::generate(), golden_ratio_rhythm::generate(), etc.
//
// Helper functions and presets are still available:
pub use circle_map::{circle_map_to_hits, circle_map_hocket};
pub use polyrhythm::{polyrhythm_cycle, polyrhythm_timings, lcm};
pub use clave::{
    son_clave_3_2, son_clave_2_3, rumba_clave_3_2, rumba_clave_2_3, bossa_clave,
};
pub use additive_meter::{
    additive_meter_length, additive_meter_rotations, rachenitsa, kopanitsa,
    kalamatianos, aksak_9_8,
};
pub use phase_shift::{
    phase_shift_by, phase_shift_timed, phase_relationship, clapping_music,
};
