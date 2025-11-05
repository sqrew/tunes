//! Musical transformation functions
//!
//! This module contains functions for transforming and mapping sequences
//! into musical parameters like frequencies, scales, and ranges.

pub mod harmonic_series;
pub mod golden_ratio;
pub mod golden_sections;
pub mod normalize;
pub mod map_to_scale;

pub use harmonic_series::harmonic_series;
pub use golden_ratio::golden_ratio;
pub use golden_sections::golden_sections;
pub use normalize::normalize;
pub use map_to_scale::{map_to_scale, Scale};
