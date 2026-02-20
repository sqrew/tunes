//! Block-based audio processing for the Mixer.
//!
//! Contains the process_block() method for efficient batch processing.

use super::Mixer;
use crate::synthesis::effects::ResolvedSidechainSource;
use crate::synthesis::simd::{SimdLanes, SIMD};
use crate::track::ids::{BusId, TrackId};

// Use rayon for parallel processing on native platforms
#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

impl Mixer {
    /// Process a block of samples
    ///
    /// This is the new block-based processing API that processes multiple samples at once,
    /// significantly reducing function call overhead and enabling future optimizations.
    ///
    /// # Arguments
    /// * `buffer` - Interleaved stereo buffer [L0, R0, L1, R1, ...] to fill
    /// * `sample_rate` - Sample rate in Hz
    /// * `start_time` - Starting time in seconds
    /// * `listener` - Optional spatial audio listener configuration
    /// * `spatial_params` - Optional spatial audio parameters
    #[allow(unused_variables)]
    pub fn process_block(
        &mut self,
        buffer: &mut [f32],
        sample_rate: f32,
        start_time: f32,
        listener: Option<&crate::synthesis::spatial::ListenerConfig>,
        spatial_params: Option<&crate::synthesis::spatial::SpatialParams>,
    ) {
        // Clear output buffer
        buffer.fill(0.0);

        let num_frames = buffer.len() / 2;
        let start_sample_count = self.sample_count;
        self.sample_count = self.sample_count.wrapping_add(num_frames as u64);

        // Clear envelope cache for this block
        self.envelope_cache.clear();

        // TWO-PASS BUS PROCESSING for parallelization with sidechain support:
        // Pass 1: Render all bus audio + calculate envelopes (can be parallel)
        // Pass 2: Apply effects + mix to output (can be parallel, envelopes now available)

        // PASS 1: Render bus audio and calculate envelopes in PARALLEL
        // BusRenderResult carries only metadata — audio data lives in bus.scratch_buffer
        struct BusRenderResult {
            bus_id: BusId,
            bus_envelope: f32,
            track_envelopes: Vec<(TrackId, f32)>,
        }

        // Parallel bus processing on native, sequential on web
        #[cfg(not(target_arch = "wasm32"))]
        let bus_results: Vec<BusRenderResult> = self
            .buses
            .par_iter_mut()
            .filter_map(|bus_opt| {
                let bus = bus_opt.as_mut()?;
                if bus.muted {
                    return None;
                }

                let bus_id = bus.id;

                // Reuse pre-allocated bus scratch buffer (resize only if block size changed)
                bus.scratch_buffer.resize(buffer.len(), 0.0);
                bus.scratch_buffer.fill(0.0);

                // Clone the Arc to share the cache across threads (cheap - just incrementing ref count)
                let cache_clone = self.cache.clone();
                #[cfg(feature = "gpu")]
                let gpu_clone = self.gpu_synthesizer.clone();
                let prerendered = self.prerendered;

                // Process each track in this bus IN PARALLEL using Rayon
                let track_results: Vec<_> = bus
                    .tracks
                    .par_iter_mut()
                    .map(|track| {
                        let track_id = track.id;
                        let pan = track.pan;

                        // Resize scratch buffer to required size
                        track.scratch_buffer.resize(num_frames, 0.0);

                        // std::mem::take splits the borrow: we temporarily own the Vec,
                        // pass it to process_track_block, then put it back. Zero allocations.
                        let mut buf = std::mem::take(&mut track.scratch_buffer);

                        // Generate mono track audio using block processing
                        // Cache is thread-safe via Arc<Mutex>, GPU synthesizer via Arc
                        Mixer::process_track_block(
                            track,
                            &mut buf,
                            sample_rate,
                            start_time,
                            start_sample_count,
                            cache_clone.as_ref(),
                            #[cfg(feature = "gpu")]
                            gpu_clone.as_ref(),
                            prerendered,
                        );

                        // Calculate RMS envelope for this track (SIMD-optimized)
                        let sum_squares = SIMD.sum_of_squares(&buf);
                        let track_envelope = (sum_squares / num_frames as f32).sqrt();

                        // Return the buffer to the track
                        track.scratch_buffer = buf;

                        (track_id, track_envelope, pan)
                    })
                    .collect();

                // Mix all track scratch buffers into the bus scratch buffer
                let mut track_envelopes = Vec::with_capacity(track_results.len());
                for (track, (track_id, track_envelope, pan)) in bus.tracks.iter().zip(track_results.iter()) {
                    track_envelopes.push((*track_id, *track_envelope));
                    let pan_angle = (pan + 1.0) * 0.25 * std::f32::consts::PI;
                    let left_gain = pan_angle.fast_cos();
                    let right_gain = pan_angle.fast_sin();
                    SIMD.mix_mono_to_stereo(&mut bus.scratch_buffer, &track.scratch_buffer, left_gain, right_gain);
                }

                // Calculate bus envelope (before effects) - SIMD-optimized
                // For stereo, RMS = sqrt((sum(L²) + sum(R²)) / (2 * num_frames))
                let total_sum_squares = SIMD.sum_of_squares(&bus.scratch_buffer);
                let bus_envelope = (total_sum_squares / (2.0 * num_frames as f32)).sqrt();

                Some(BusRenderResult {
                    bus_id,
                    bus_envelope,
                    track_envelopes,
                })
            })
            .collect();

        #[cfg(target_arch = "wasm32")]
        let bus_results: Vec<BusRenderResult> = self
            .buses
            .iter_mut()
            .filter_map(|bus_opt| {
                let bus = bus_opt.as_mut()?;
                if bus.muted {
                    return None;
                }

                let bus_id = bus.id;

                // Reuse pre-allocated bus scratch buffer (resize only if block size changed)
                bus.scratch_buffer.resize(buffer.len(), 0.0);
                bus.scratch_buffer.fill(0.0);

                // Clone the Arc to share the cache (not actually across threads on web)
                let cache_clone = self.cache.clone();
                #[cfg(feature = "gpu")]
                let gpu_clone = self.gpu_synthesizer.clone();
                let prerendered = self.prerendered;

                // Process each track in this bus SEQUENTIALLY on web
                let mut track_envelopes = Vec::with_capacity(bus.tracks.len());
                for track in bus.tracks.iter_mut() {
                    let track_id = track.id;
                    let pan = track.pan;

                    // Resize scratch buffer to required size
                    track.scratch_buffer.resize(num_frames, 0.0);

                    // std::mem::take splits the borrow: temporarily own the Vec,
                    // pass to process_track_block, then return it. Zero allocations.
                    let mut buf = std::mem::take(&mut track.scratch_buffer);

                    // Generate mono track audio using block processing
                    Mixer::process_track_block(
                        track,
                        &mut buf,
                        sample_rate,
                        start_time,
                        start_sample_count,
                        cache_clone.as_ref(),
                        #[cfg(feature = "gpu")]
                        gpu_clone.as_ref(),
                        prerendered,
                    );

                    // Calculate RMS envelope for this track (SIMD-optimized)
                    let sum_squares = SIMD.sum_of_squares(&buf);
                    let track_envelope = (sum_squares / num_frames as f32).sqrt();
                    track_envelopes.push((track_id, track_envelope));

                    // Mix into bus buffer and return buffer to track
                    let pan_angle = (pan + 1.0) * 0.25 * std::f32::consts::PI;
                    let left_gain = pan_angle.fast_cos();
                    let right_gain = pan_angle.fast_sin();
                    SIMD.mix_mono_to_stereo(&mut bus.scratch_buffer, &buf, left_gain, right_gain);

                    track.scratch_buffer = buf;
                }

                // Calculate bus envelope (before effects) - SIMD-optimized
                // For stereo, RMS = sqrt((sum(L²) + sum(R²)) / (2 * num_frames))
                let total_sum_squares = SIMD.sum_of_squares(&bus.scratch_buffer);
                let bus_envelope = (total_sum_squares / (2.0 * num_frames as f32)).sqrt();

                Some(BusRenderResult {
                    bus_id,
                    bus_envelope,
                    track_envelopes,
                })
            })
            .collect();

        // Cache all track and bus envelopes (now safe since all are computed)
        for result in &bus_results {
            for (track_id, envelope) in &result.track_envelopes {
                self.envelope_cache.cache_track(*track_id, *envelope);
            }
            self.envelope_cache
                .cache_bus(result.bus_id, result.bus_envelope);
        }

        // PASS 2: Apply effects and mix to output (sequential — effects have state)
        // Audio data lives in bus.scratch_buffer; result carries only metadata.
        for result in bus_results {
            // Find the original bus to apply effects and read its scratch buffer
            let bus = self
                .buses
                .iter_mut()
                .find_map(|b| b.as_mut().filter(|bus| bus.id == result.bus_id))
                .expect("Bus should exist");

            // Look up sidechain envelope (now safe - all envelopes cached in pass 1)
            let sidechain_env = if let Some(ref compressor) = bus.effects.compressor {
                if let Some(ref resolved_source) = compressor.resolved_sidechain_source {
                    match resolved_source {
                        ResolvedSidechainSource::Track(track_id) => {
                            Some(self.envelope_cache.get_track(*track_id))
                        }
                        ResolvedSidechainSource::Bus(sidechain_bus_id) => {
                            Some(self.envelope_cache.get_bus(*sidechain_bus_id))
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            };

            // Apply bus effects in-place to the scratch buffer
            bus.effects.process_stereo_block(
                &mut bus.scratch_buffer,
                sample_rate,
                start_time,
                start_sample_count,
                sidechain_env,
            );

            // Mix into output buffer with SIMD optimization (automatic fallback)
            let bus_pan_angle = (bus.pan + 1.0) * 0.25 * std::f32::consts::PI;
            let bus_left_gain = bus_pan_angle.fast_cos() * bus.volume;
            let bus_right_gain = bus_pan_angle.fast_sin() * bus.volume;

            SIMD.mix_stereo_interleaved(buffer, &bus.scratch_buffer, bus_left_gain, bus_right_gain);
        }

        // Look up master sidechain envelope if configured
        let master_sidechain_env = if let Some(ref compressor) = self.master.compressor {
            if let Some(ref resolved_source) = compressor.resolved_sidechain_source {
                match resolved_source {
                    ResolvedSidechainSource::Track(track_id) => {
                        Some(self.envelope_cache.get_track(*track_id))
                    }
                    ResolvedSidechainSource::Bus(bus_id) => {
                        Some(self.envelope_cache.get_bus(*bus_id))
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        // Apply master effects (block processing) with sidechain support
        self.master.process_stereo_block(
            buffer,
            sample_rate,
            start_time,
            start_sample_count,
            master_sidechain_env,
        );
    }
}
