//! Pattern generation and manipulation
//!
//! This module provides tools for generating and transforming musical patterns.

pub mod rhythm;
pub mod musical;
pub mod classical;

// Re-export everything from submodules to maintain API compatibility
pub use rhythm::*;
pub use musical::*;
pub use classical::*;
