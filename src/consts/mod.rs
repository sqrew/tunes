//! Musical constants: notes, scales, and chords
//!
//! This module provides pre-defined constants for common musical elements:
//! - **Notes**: Standard pitch constants (A0-C8)
//! - **Scales**: Scale patterns and generators
//! - **Chords**: Chord patterns and builders

pub mod notes;
pub mod scales;
pub mod chords;

// Re-export everything for convenience
pub use notes::*;
pub use scales::*;
pub use chords::*;
