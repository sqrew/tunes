//! Smart caching system for pre-rendered synthesis
//!
//! This module provides automatic caching of expensive synthesis operations.
//! Pre-rendering synthesis to samples and playing them back is significantly
//! faster than real-time synthesis (up to 8x with SIMD sample playback).
//!
//! # Architecture
//!
//! - **Cache Keys**: Hash synthesis parameters (waveform, envelope, FM, etc.)
//! - **Cache Storage**: LRU-based in-memory cache with optional disk persistence
//! - **GPU Acceleration**: Optional GPU compute shader rendering (future)
//!
//! # Example
//!
//! ```no_run
//! use tunes::prelude::*;
//!
//! let mut cache = SampleCache::new()
//!     .max_memory_mb(500)
//!     .enable_disk_cache(false);  // RAM only, no persistence
//!
//! // Cache is automatically used during synthesis
//! let mut comp = Composition::new(Tempo::new(120.0))
//!     .with_cache(cache);
//! ```

pub mod key;
pub mod storage;

pub use key::CacheKey;
pub use storage::{SampleCache, CachePolicy, CachedSample};
