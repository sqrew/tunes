//! Cache storage with LRU eviction
//!
//! Stores pre-rendered samples in memory with automatic eviction when
//! memory limits are exceeded.

use super::key::CacheKey;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

/// A cached audio sample
#[derive(Debug, Clone)]
pub struct CachedSample {
    /// Pre-rendered mono samples at reference pitch (usually C4 = 261.63 Hz)
    pub samples: Arc<Vec<f32>>,

    /// Sample rate the audio was rendered at
    pub sample_rate: f32,

    /// Duration in seconds
    pub duration: f32,

    /// Reference frequency this was rendered at
    pub reference_frequency: f32,

    /// Size in bytes (for memory tracking)
    size_bytes: usize,
}

impl CachedSample {
    /// Create a new cached sample
    pub fn new(samples: Vec<f32>, sample_rate: f32, duration: f32, reference_frequency: f32) -> Self {
        let size_bytes = samples.len() * std::mem::size_of::<f32>();
        Self {
            samples: Arc::new(samples),
            sample_rate,
            duration,
            reference_frequency,
            size_bytes,
        }
    }

    /// Get the size in bytes
    pub fn size_bytes(&self) -> usize {
        self.size_bytes
    }

    /// Get the size in megabytes
    pub fn size_mb(&self) -> f32 {
        self.size_bytes as f32 / (1024.0 * 1024.0)
    }
}

/// Cache policy configuration
#[derive(Debug, Clone)]
pub struct CachePolicy {
    /// Maximum cache size in megabytes
    pub max_size_mb: usize,

    /// Minimum synthesis duration (ms) to cache
    /// Sounds shorter than this won't be cached (not worth it)
    pub min_cache_duration_ms: f32,

    /// Enable disk persistence (not implemented yet)
    pub enable_disk_cache: bool,

    /// Disk cache directory (if enabled)
    pub cache_dir: Option<std::path::PathBuf>,
}

impl Default for CachePolicy {
    fn default() -> Self {
        Self {
            max_size_mb: 500,                  // 500 MB default
            min_cache_duration_ms: 100.0,      // Only cache sounds > 100ms
            enable_disk_cache: false,          // RAM only by default
            cache_dir: None,
        }
    }
}

/// Sample cache with LRU eviction
///
/// Stores pre-rendered synthesis output in memory. When the cache
/// exceeds `max_size_mb`, least-recently-used entries are evicted.
///
/// # Example
///
/// ```no_run
/// use tunes::cache::{SampleCache, CachePolicy};
///
/// let mut cache = SampleCache::new()
///     .with_max_size_mb(500);
///
/// // Cache is automatically populated during synthesis
/// ```
#[derive(Debug, Clone)]
pub struct SampleCache {
    /// Policy configuration
    policy: CachePolicy,

    /// Cached samples by key
    cache: HashMap<CacheKey, CachedSample>,

    /// LRU tracking - most recent at the back
    lru_queue: VecDeque<CacheKey>,

    /// Total cache size in bytes
    total_size_bytes: usize,

    /// Statistics
    stats: CacheStats,
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,

    /// Number of cache misses
    pub misses: u64,

    /// Number of evictions
    pub evictions: u64,

    /// Total samples inserted
    pub insertions: u64,
}

impl CacheStats {
    /// Calculate hit rate (0.0 - 1.0)
    pub fn hit_rate(&self) -> f32 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f32 / total as f32
        }
    }
}

impl SampleCache {
    /// Create a new sample cache with default policy
    pub fn new() -> Self {
        Self {
            policy: CachePolicy::default(),
            cache: HashMap::new(),
            lru_queue: VecDeque::new(),
            total_size_bytes: 0,
            stats: CacheStats::default(),
        }
    }

    /// Set maximum cache size in megabytes
    pub fn with_max_size_mb(mut self, max_mb: usize) -> Self {
        self.policy.max_size_mb = max_mb;
        self
    }

    /// Set minimum cacheable duration in milliseconds
    pub fn with_min_duration_ms(mut self, min_ms: f32) -> Self {
        self.policy.min_cache_duration_ms = min_ms;
        self
    }

    /// Enable disk caching (not implemented yet)
    pub fn with_disk_cache(mut self, enable: bool) -> Self {
        self.policy.enable_disk_cache = enable;
        self
    }

    /// Get a cached sample if it exists (returns a clone via Arc)
    ///
    /// Returns `None` if not in cache (cache miss).
    /// Updates LRU tracking on cache hit.
    ///
    /// Note: Returns a clone of the CachedSample (cheap via Arc).
    pub fn get(&mut self, key: &CacheKey) -> Option<CachedSample> {
        if self.cache.contains_key(key) {
            // Cache hit! Update LRU
            self.stats.hits += 1;
            self.touch(key);
            // Return a clone (Arc makes this cheap)
            self.cache.get(key).cloned()
        } else {
            // Cache miss
            self.stats.misses += 1;
            None
        }
    }

    /// Insert a sample into the cache
    ///
    /// If the cache is full, evicts least-recently-used entries.
    pub fn insert(&mut self, key: CacheKey, sample: CachedSample) {
        // Check if this sample is worth caching
        if sample.duration < self.policy.min_cache_duration_ms / 1000.0 {
            return; // Too short, not worth caching
        }

        let sample_size = sample.size_bytes();

        // If this sample already exists, remove old version first
        if self.cache.contains_key(&key) {
            self.remove(&key);
        }

        // Evict until we have enough space
        let max_bytes = self.policy.max_size_mb * 1024 * 1024;
        while self.total_size_bytes + sample_size > max_bytes && !self.lru_queue.is_empty() {
            self.evict_lru();
        }

        // Insert the sample
        self.cache.insert(key, sample);
        self.lru_queue.push_back(key);
        self.total_size_bytes += sample_size;
        self.stats.insertions += 1;
    }

    /// Remove a specific key from the cache
    fn remove(&mut self, key: &CacheKey) {
        if let Some(sample) = self.cache.remove(key) {
            self.total_size_bytes = self.total_size_bytes.saturating_sub(sample.size_bytes());

            // Remove from LRU queue
            if let Some(pos) = self.lru_queue.iter().position(|k| k == key) {
                self.lru_queue.remove(pos);
            }
        }
    }

    /// Mark a key as recently used (move to back of LRU queue)
    fn touch(&mut self, key: &CacheKey) {
        // Remove from current position
        if let Some(pos) = self.lru_queue.iter().position(|k| k == key) {
            self.lru_queue.remove(pos);
        }

        // Add to back (most recent)
        self.lru_queue.push_back(*key);
    }

    /// Evict the least recently used entry
    fn evict_lru(&mut self) {
        if let Some(key) = self.lru_queue.pop_front() {
            if let Some(sample) = self.cache.remove(&key) {
                self.total_size_bytes = self.total_size_bytes.saturating_sub(sample.size_bytes());
                self.stats.evictions += 1;
            }
        }
    }

    /// Clear the entire cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.lru_queue.clear();
        self.total_size_bytes = 0;
    }

    /// Get current cache size in bytes
    pub fn size_bytes(&self) -> usize {
        self.total_size_bytes
    }

    /// Get current cache size in megabytes
    pub fn size_mb(&self) -> f32 {
        self.total_size_bytes as f32 / (1024.0 * 1024.0)
    }

    /// Get number of cached entries
    pub fn entry_count(&self) -> usize {
        self.cache.len()
    }

    /// Get cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Print cache statistics
    pub fn print_stats(&self) {
        println!("\n📊 Sample Cache Statistics:");
        println!("  Entries: {}", self.entry_count());
        println!("  Size: {:.2} MB / {} MB", self.size_mb(), self.policy.max_size_mb);
        println!("  Hits: {}", self.stats.hits);
        println!("  Misses: {}", self.stats.misses);
        println!("  Hit rate: {:.1}%", self.stats.hit_rate() * 100.0);
        println!("  Evictions: {}", self.stats.evictions);
        println!("  Insertions: {}", self.stats.insertions);
    }
}

impl Default for SampleCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_sample(duration_sec: f32, sample_rate: f32) -> CachedSample {
        let num_samples = (duration_sec * sample_rate) as usize;
        let samples = vec![0.0f32; num_samples];
        CachedSample::new(samples, sample_rate, duration_sec, 261.63)
    }

    #[test]
    fn test_cache_insert_and_get() {
        let mut cache = SampleCache::new();
        let key = CacheKey::new(12345);
        let sample = make_test_sample(1.0, 44100.0);

        cache.insert(key, sample.clone());

        assert!(cache.get(&key).is_some());
        assert_eq!(cache.entry_count(), 1);
        assert_eq!(cache.stats().hits, 1);
        assert_eq!(cache.stats().misses, 0);
    }

    #[test]
    fn test_cache_miss() {
        let mut cache = SampleCache::new();
        let key = CacheKey::new(12345);

        assert!(cache.get(&key).is_none());
        assert_eq!(cache.stats().hits, 0);
        assert_eq!(cache.stats().misses, 1);
    }

    #[test]
    fn test_lru_eviction() {
        let mut cache = SampleCache::new().with_max_size_mb(1); // 1 MB limit

        // Each sample is ~176 KB (44100 samples * 4 bytes)
        let key1 = CacheKey::new(1);
        let key2 = CacheKey::new(2);
        let key3 = CacheKey::new(3);
        let key4 = CacheKey::new(4);
        let key5 = CacheKey::new(5);
        let key6 = CacheKey::new(6);

        cache.insert(key1, make_test_sample(1.0, 44100.0));
        cache.insert(key2, make_test_sample(1.0, 44100.0));
        cache.insert(key3, make_test_sample(1.0, 44100.0));
        cache.insert(key4, make_test_sample(1.0, 44100.0));
        cache.insert(key5, make_test_sample(1.0, 44100.0));

        // Cache should be nearly full (5 * 176KB ≈ 880 KB)
        assert!(cache.size_mb() < 1.0);

        // Insert one more, should trigger eviction of key1 (oldest)
        cache.insert(key6, make_test_sample(1.0, 44100.0));

        assert!(cache.get(&key1).is_none()); // key1 should be evicted
        assert!(cache.get(&key6).is_some()); // key6 should be present
        assert!(cache.stats().evictions > 0);
    }

    #[test]
    fn test_min_duration_filter() {
        let mut cache = SampleCache::new().with_min_duration_ms(100.0);

        // Try to cache a 50ms sound (below threshold)
        let key = CacheKey::new(1);
        cache.insert(key, make_test_sample(0.05, 44100.0));

        assert_eq!(cache.entry_count(), 0); // Should not be cached
        assert_eq!(cache.stats().insertions, 0);
    }

    #[test]
    fn test_clear() {
        let mut cache = SampleCache::new();

        cache.insert(CacheKey::new(1), make_test_sample(1.0, 44100.0));
        cache.insert(CacheKey::new(2), make_test_sample(1.0, 44100.0));

        assert_eq!(cache.entry_count(), 2);

        cache.clear();

        assert_eq!(cache.entry_count(), 0);
        assert_eq!(cache.size_bytes(), 0);
    }
}
