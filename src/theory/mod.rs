//! Music theory: scales, chords, progressions, microtonal systems, and key signatures

pub mod core;
pub mod microtonal;
pub mod key_signature;

// Re-export main types for convenience
pub use core::{
    ChordPattern, ProgressionType, ScalePattern, chord, progression, scale, transpose,
    transpose_sequence,
};
pub use microtonal::{
    EDO12, EDO19, EDO24, EDO31, EDO53, Edo, cents_to_ratio, freq_from_cents, half_flat,
    half_sharp, just_major_scale, just_minor_scale, just_ratio, just_scale, pythagorean_scale,
    quarter_flat, quarter_sharp, ratio_to_cents,
};
pub use key_signature::{KeyMode, KeyRoot, KeySignature};
