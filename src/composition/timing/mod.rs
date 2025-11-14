//! Musical timing and tempo management
//!
//! This module provides tools for working with musical time, tempo, and rhythmic navigation.

pub mod navigation;
pub mod musical_time;
pub mod tempo;

// Re-export everything from submodules to maintain API compatibility
pub use tempo::*;
