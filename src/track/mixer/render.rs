//! Buffer rendering methods for the Mixer.
//!
//! Contains methods for rendering to in-memory buffers.

use super::Mixer;
use crate::cache::{CacheKey, CachedSample};
use crate::track::events::{AudioEvent, NoteEvent};
use std::collections::HashMap;

impl Mixer {
    /// Pre-render all unique notes in the composition
    ///
    /// This scans all tracks, finds unique notes, and batch renders them on GPU
    /// (or CPU fallback). This is called automatically before streaming if GPU or
    /// cache is enabled, eliminating per-block cache lookups.
    ///
    /// **This is the key to GPU performance!** Instead of checking cache during
    /// streaming (causing overhead), we render everything upfront and stream
    /// from a fully-populated cache.
    pub fn prerender_notes(&mut self, sample_rate: f32) {
        // Only prerender if cache is enabled
        let cache = match &self.cache {
            Some(c) => c,
            None => return,
        };

        // Collect all unique notes from all tracks
        let mut unique_notes: HashMap<CacheKey, NoteEvent> = HashMap::new();

        for bus in self.buses.iter().flatten() {
            for track in &bus.tracks {
                for event in &track.events {
                    if let AudioEvent::Note(note_event) = event {
                        let cache_key = CacheKey::from_note_event(note_event, sample_rate);

                        // Only add if not already seen
                        unique_notes
                            .entry(cache_key)
                            .or_insert_with(|| note_event.clone());
                    }
                }
            }
        }

        let total_notes = unique_notes.len();
        println!("   Found {} unique notes to render", total_notes);

        // Batch render all unique notes
        let start = std::time::Instant::now();
        let mut rendered_count = 0;

        for (cache_key, note_event) in unique_notes {
            // Check if already cached
            if cache.get(&cache_key).is_some() {
                continue; // Already in cache
            }

            // Render the note (GPU if available, CPU fallback)
            let total_duration = note_event.envelope.total_duration(note_event.duration);

            if total_duration > 0.0 && total_duration < 10.0 {
                let rendered_samples = Self::render_note_to_buffer(
                    &note_event,
                    sample_rate,
                    #[cfg(feature = "gpu")]
                    self.gpu_synthesizer.as_ref(),
                );

                let cached_sample = CachedSample::new(
                    rendered_samples,
                    sample_rate,
                    total_duration,
                    note_event.frequencies[0],
                );

                cache.insert(cache_key, cached_sample);
                rendered_count += 1;
            }
        }

        let elapsed = start.elapsed();

        #[cfg(feature = "gpu")]
        {
            if self.gpu_enabled() {
                let notes_per_second = rendered_count as f32 / elapsed.as_secs_f32();
                println!(
                    "   ✅ Pre-rendered {} notes in {:.3}s ({:.0} notes/sec)",
                    rendered_count,
                    elapsed.as_secs_f32(),
                    notes_per_second
                );
            } else {
                println!(
                    "   ✅ Pre-rendered {} notes in {:.3}s",
                    rendered_count,
                    elapsed.as_secs_f32()
                );
            }
        }
        #[cfg(not(feature = "gpu"))]
        {
            println!(
                "   ✅ Pre-rendered {} notes in {:.3}s",
                rendered_count,
                elapsed.as_secs_f32()
            );
        }

        // Mark as pre-rendered so streaming skips cache lookups
        self.prerendered = true;
    }

    /// Render the mixer to an in-memory stereo buffer
    ///
    /// Pre-renders the entire composition to a Vec of interleaved stereo samples (left, right, left, right...).
    /// This is used for efficient playback without real-time synthesis overhead.
    ///
    /// # Arguments
    /// * `sample_rate` - Sample rate in Hz
    ///
    /// # Returns
    /// A Vec of f32 samples in interleaved stereo format (left, right, left, right...)
    pub fn render_to_buffer(&mut self, sample_rate: f32) -> Vec<f32> {
        let duration = self.total_duration();
        let total_samples = (duration * sample_rate).ceil() as usize;

        // 🚀 KEY OPTIMIZATION: Pre-render all unique notes before streaming
        // This eliminates per-block cache lookups and unleashes GPU performance!
        let should_prerender = self.cache.is_some() || {
            #[cfg(feature = "gpu")]
            {
                self.gpu_synthesizer.is_some()
            }
            #[cfg(not(feature = "gpu"))]
            {
                false
            }
        };

        if should_prerender {
            self.prerender_notes(sample_rate);
        }

        // Pre-allocate buffer for interleaved stereo (2 channels)
        let mut buffer = vec![0.0; total_samples * 2];

        // Process in blocks of 512 samples for better performance
        const BLOCK_SIZE: usize = 512;
        let mut processed_samples = 0;

        while processed_samples < total_samples {
            let remaining = total_samples - processed_samples;
            let block_samples = remaining.min(BLOCK_SIZE);
            let block_frames = block_samples * 2; // stereo

            let start_time = processed_samples as f32 / sample_rate;
            let start_idx = processed_samples * 2;
            let end_idx = start_idx + block_frames;

            // Process this block
            self.process_block(
                &mut buffer[start_idx..end_idx],
                sample_rate,
                start_time,
                None,
                None,
            );

            processed_samples += block_samples;
        }

        // Clamp to valid range
        for sample in &mut buffer {
            *sample = sample.clamp(-1.0, 1.0);
        }

        buffer
    }
}
