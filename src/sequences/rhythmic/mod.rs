//! Rhythmic pattern generators
//!
//! This module contains algorithms for generating rhythmic patterns,
//! including Euclidean rhythms, golden ratio rhythms, and Shepard tones.

pub mod euclidean;
pub mod golden_ratio_rhythm;
pub mod shepard_tone;

pub use euclidean::{euclidean, euclidean_pattern};
pub use golden_ratio_rhythm::golden_ratio_rhythm;
pub use shepard_tone::shepard_tone;
