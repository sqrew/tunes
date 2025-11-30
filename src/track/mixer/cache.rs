//! Sample cache management for the Mixer.
//!
//! Provides methods for enabling, configuring, and managing the sample cache.

use super::Mixer;
use crate::cache::SampleCache;
use std::sync::Arc;

impl Mixer {
    /// Enable sample caching with default settings
    ///
    /// This enables automatic caching of synthesized notes, dramatically improving
    /// performance when the same synthesis parameters are used multiple times.
    ///
    /// Default cache settings:
    /// - 500 MB memory limit
    /// - Only cache sounds > 100ms duration
    /// - LRU eviction when full
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// let mut mixer = Composition::new(Tempo::new(120.0)).into_mixer();
    /// mixer.enable_cache();
    /// ```
    pub fn enable_cache(&mut self) -> &mut Self {
        self.cache = Some(Arc::new(SampleCache::new()));
        self
    }

    /// Enable sample caching with custom settings
    ///
    /// # Arguments
    /// * `cache` - Pre-configured SampleCache
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # use tunes::cache::SampleCache;
    /// let cache = SampleCache::new()
    ///     .with_max_size_mb(1000)
    ///     .with_min_duration_ms(50.0);
    ///
    /// let mut mixer = Composition::new(Tempo::new(120.0)).into_mixer();
    /// mixer.enable_cache_with(cache);
    /// ```
    pub fn enable_cache_with(&mut self, cache: SampleCache) -> &mut Self {
        self.cache = Some(Arc::new(cache));
        self
    }

    /// Disable sample caching
    pub fn disable_cache(&mut self) -> &mut Self {
        self.cache = None;
        self
    }

    /// Get cache statistics (if caching is enabled)
    ///
    /// Returns `None` if caching is disabled.
    pub fn cache_stats(&self) -> Option<crate::cache::CacheStatsSnapshot> {
        self.cache.as_ref().map(|c| c.stats().snapshot())
    }

    /// Print cache statistics (if caching is enabled)
    pub fn print_cache_stats(&self) {
        if let Some(cache) = &self.cache {
            cache.print_stats();
        } else {
            println!("Sample caching is disabled");
        }
    }

    /// Clear the sample cache (if caching is enabled)
    pub fn clear_cache(&self) {
        if let Some(cache) = &self.cache {
            cache.clear();
        }
    }
}
