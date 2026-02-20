//! Per-sample audio rendering for the Mixer.
//!
//! Contains the sample_at() method for sample-by-sample rendering.

use super::output_types::{BusOutput, TrackOutput};
use super::Mixer;
use crate::synthesis::effects::ResolvedSidechainSource;

impl Mixer {
    /// Generate a stereo sample at a given time by mixing all buses
    ///
    /// This is the core rendering method that generates audio samples by:
    /// 1. Processing each bus (which mixes its tracks and applies bus effects)
    /// 2. Summing all bus outputs
    /// 3. Applying master effects to the final mix
    ///
    /// # Arguments
    /// * `time` - The time position in seconds
    /// * `sample_rate` - Sample rate in Hz (e.g., 44100)
    /// * `_sample_clock` - Reserved for future use
    /// * `_listener` - Reserved for spatial audio (handled at track level)
    /// * `_spatial_params` - Reserved for spatial audio (handled at track level)
    ///
    /// # Returns
    /// A tuple of (left_channel, right_channel) audio samples in range -1.0 to 1.0
    #[inline(always)]
    pub fn sample_at(
        &mut self,
        time: f32,
        sample_rate: f32,
        _sample_clock: f32,
        _listener: Option<&crate::synthesis::spatial::ListenerConfig>,
        _spatial_params: Option<&crate::synthesis::spatial::SpatialParams>,
    ) -> (f32, f32) {
        // Increment sample count for quantized automation lookups
        self.sample_count = self.sample_count.wrapping_add(1);

        // Clear pre-allocated buffers (NO ALLOCATION!)
        for bus_outputs in &mut self.track_outputs_by_bus {
            bus_outputs.clear();
        }
        self.bus_outputs.clear();
        self.envelope_cache.clear();

        let mut mixed_left = 0.0;
        let mut mixed_right = 0.0;

        // PASS 1: Process tracks and cache their envelopes
        // We need to process all tracks first to build the envelope cache
        // before applying bus effects (which may use sidechaining)

        // Iterate over buses using Vec<Option<Bus>>
        for bus_opt in self.buses.iter_mut() {
            let bus = match bus_opt {
                Some(b) => b,
                None => continue,
            };

            if bus.muted {
                continue;
            }

            let sample_count = self.sample_count;
            let bus_id = bus.id;

            for track in &mut bus.tracks {
                let (track_left, track_right) =
                    Self::process_track_static(track, time, sample_rate, sample_count);

                // Calculate RMS envelope for this track
                let envelope = ((track_left * track_left + track_right * track_right) / 2.0).sqrt();

                // Cache track envelope using integer ID
                self.envelope_cache.cache_track(track.id, envelope);

                // Store output indexed by bus ID (O(1) lookup in Pass 2!)
                self.track_outputs_by_bus[bus_id as usize].push(TrackOutput {
                    bus_id,
                    left: track_left,
                    right: track_right,
                    envelope,
                });
            }
        }

        // PASS 2: Mix tracks into buses and apply bus effects with sidechain support

        // Iterate over buses for processing
        for bus_opt in self.buses.iter_mut() {
            let bus = match bus_opt {
                Some(b) => b,
                None => continue,
            };

            if bus.muted {
                continue;
            }

            let bus_id = bus.id;

            // Sum tracks belonging to this bus (O(1) lookup via indexed Vec!)
            let mut bus_left = 0.0;
            let mut bus_right = 0.0;
            for track_output in &self.track_outputs_by_bus[bus_id as usize] {
                bus_left += track_output.left;
                bus_right += track_output.right;
            }

            // Calculate bus envelope BEFORE effects
            let bus_envelope = ((bus_left * bus_left + bus_right * bus_right) / 2.0).sqrt();
            self.envelope_cache.cache_bus(bus_id, bus_envelope);

            // Look up sidechain envelope using resolved IDs (OPTIMIZED: Integer lookup!)
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

            // Apply bus effects (stereo processing) with sidechain support
            let (effected_left, effected_right) = bus.effects.process_stereo(
                bus_left,
                bus_right,
                sample_rate,
                time,
                self.sample_count,
                sidechain_env,
            );

            // Apply bus volume and pan (constant-power panning, matches process_block.rs)
            let bus_pan_angle = (bus.pan + 1.0) * 0.25 * std::f32::consts::PI;
            let pan_left = bus_pan_angle.cos();
            let pan_right = bus_pan_angle.sin();

            let final_bus_left = effected_left * bus.volume * pan_left;
            let final_bus_right = effected_right * bus.volume * pan_right;

            // Store output using integer bus ID (NO STRING CLONE!)
            self.bus_outputs.push(BusOutput {
                bus_id,
                left: final_bus_left,
                right: final_bus_right,
            });
        }

        // Sum all bus outputs
        for bus_output in &self.bus_outputs {
            mixed_left += bus_output.left;
            mixed_right += bus_output.right;
        }

        // Apply master effects (stereo processing) - no sidechaining on master
        let (master_left, master_right) = self.master.process_stereo(
            mixed_left,
            mixed_right,
            sample_rate,
            time,
            self.sample_count,
            None,
        );

        // Hard clamp to [-1.0, 1.0] — matches process_block and export paths
        // tanh was wrong here: tanh(1.0) ≈ 0.762 reduces amplitude even at 0 dBFS
        (master_left.clamp(-1.0, 1.0), master_right.clamp(-1.0, 1.0))
    }
}
