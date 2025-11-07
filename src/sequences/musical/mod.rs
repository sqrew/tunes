//! Musical transformation functions
//!
//! This module contains functions for transforming and mapping sequences
//! into musical parameters like frequencies, scales, and ranges.

pub mod harmonic_series;
pub mod golden_ratio;
pub mod golden_sections;
pub mod normalize;
pub mod map_to_scale;
pub mod undertone_series;
pub mod circle_of_fifths;
pub mod pythagorean_tuning;

pub use harmonic_series::harmonic_series;
pub use golden_ratio::golden_ratio;
pub use golden_sections::golden_sections;
pub use normalize::{normalize, normalize_f32};
pub use map_to_scale::{map_to_scale, map_to_scale_f32, Scale};
pub use undertone_series::undertone_series;
pub use circle_of_fifths::{circle_of_fifths, circle_of_fourths};
pub use pythagorean_tuning::{pythagorean_tuning, just_intonation_major, just_intonation_minor};
