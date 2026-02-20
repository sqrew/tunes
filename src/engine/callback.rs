//! Audio callback state and mixing functions.
//!
//! Contains the pre-allocated buffers and mixing logic for the real-time audio callback.

use super::active_sound::ActiveSound;
use super::commands::{AudioCommand, SoundId};
#[cfg(not(target_arch = "wasm32"))]
use super::streaming::StreamingSound;
use crate::synthesis::simd::{SimdWidth, SIMD};
use crate::synthesis::spatial::{
    ListenerConfig, SpatialParams, Vec3, calculate_spatial_with_cone,
};
use crossbeam::epoch::{self, Atomic, Owned};
use ringbuf::{traits::Split, HeapRb};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::thread;
use wide::f32x8;

/// Audio callback state (allocation-free mixing)
///
/// Holds pre-allocated buffers to avoid allocations in the real-time audio thread.
/// All buffers are reused across callback invocations.
pub(crate) struct AudioCallbackState {
    /// Active sounds being mixed (sparse vector indexed by SoundId for cache-friendly iteration)
    /// Uses Vec<Option<>> instead of HashMap for sequential memory access (better cache locality)
    pub active_sounds: Vec<Option<ActiveSound>>,
    /// Streaming sounds (separate from pre-rendered sounds, native only)
    #[cfg(not(target_arch = "wasm32"))]
    pub streaming_sounds: Vec<Option<StreamingSound>>,
    /// Pre-allocated temp buffer for mixing (stereo interleaved)
    /// Size is determined by the maximum buffer size we expect
    pub temp_buffer: Vec<f32>,
    /// Pre-allocated list for tracking finished sounds (avoids allocation during cleanup)
    pub finished_sounds: Vec<SoundId>,
    /// Pre-allocated list for tracking finished streams (native only)
    #[cfg(not(target_arch = "wasm32"))]
    pub finished_streams: Vec<SoundId>,
}

impl AudioCallbackState {
    pub fn new() -> Self {
        Self {
            // Pre-allocate space for 128 concurrent sounds (typical max for games)
            active_sounds: Vec::with_capacity(128),
            #[cfg(not(target_arch = "wasm32"))]
            streaming_sounds: Vec::with_capacity(16),
            // Pre-allocate for a reasonably large buffer (2048 frames stereo = 4096 samples)
            temp_buffer: vec![0.0; 4096],
            finished_sounds: Vec::with_capacity(16),
            #[cfg(not(target_arch = "wasm32"))]
            finished_streams: Vec::with_capacity(16),
        }
    }

    /// Ensure temp buffer is large enough for the given size
    #[allow(dead_code)]
    pub fn ensure_temp_buffer_size(&mut self, required_size: usize) {
        if self.temp_buffer.len() < required_size {
            self.temp_buffer.resize(required_size, 0.0);
        }
    }
}

/// Handle commands from the main thread (called from audio thread)
pub(crate) fn handle_command(
    cmd: AudioCommand,
    active_sounds: &mut Vec<Option<ActiveSound>>,
    #[cfg(not(target_arch = "wasm32"))] streaming_sounds: &mut Vec<Option<StreamingSound>>,
    listener_atomic: &Arc<Atomic<ListenerConfig>>,
    spatial_atomic: &Arc<Atomic<SpatialParams>>,
    sample_rate: f32,
) {
    match cmd {
        AudioCommand::Play { id, mixer, looping } => {
            // Ensure Vec has enough capacity (sparse vector indexed by SoundId)
            let index = id as usize;
            while active_sounds.len() <= index {
                active_sounds.push(None);
            }

            active_sounds[index] = Some(ActiveSound::new(*mixer, looping));
        }
        AudioCommand::Stop { id } => {
            let index = id as usize;
            if index < active_sounds.len() {
                active_sounds[index] = None;
            }
        }
        AudioCommand::SetVolume { id, volume } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                sound.volume = volume.clamp(0.0, 1.0);
            }
        }
        AudioCommand::SetPan { id, pan } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                sound.pan = pan.clamp(-1.0, 1.0);
            }
        }
        AudioCommand::SetPlaybackRate { id, rate } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                // Clamp to reasonable range (0.1x to 4.0x speed)
                sound.playback_rate = rate.clamp(0.1, 4.0);
            }
        }
        AudioCommand::Pause { id } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                sound.paused = true;
            }
        }
        AudioCommand::Resume { id } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                sound.paused = false;
            }
        }
        AudioCommand::SetSoundPosition { id, position } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                sound.spatial_position = Some(position);
                sound.spatial_dirty = true; // Mark for recalculation
            }
        }
        AudioCommand::SetSoundVelocity { id, vx, vy, vz } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                if let Some(pos) = &mut sound.spatial_position {
                    pos.set_velocity(vx, vy, vz);
                }
            }
        }
        AudioCommand::SetListenerPosition { x, y, z } => {
            // Lock-free update: load, clone, modify, store
            let guard = epoch::pin();
            let current =
                unsafe { listener_atomic.load(Ordering::Acquire, &guard).as_ref().unwrap() };
            let mut new_config = *current;
            new_config.position.x = x;
            new_config.position.y = y;
            new_config.position.z = z;
            listener_atomic.store(Owned::new(new_config), Ordering::Release);
        }
        AudioCommand::SetListenerVelocity { vx, vy, vz } => {
            let guard = epoch::pin();
            let current =
                unsafe { listener_atomic.load(Ordering::Acquire, &guard).as_ref().unwrap() };
            let mut new_config = *current;
            new_config.velocity.x = vx;
            new_config.velocity.y = vy;
            new_config.velocity.z = vz;
            listener_atomic.store(Owned::new(new_config), Ordering::Release);
        }
        AudioCommand::SetListenerForward { x, y, z } => {
            let guard = epoch::pin();
            let current =
                unsafe { listener_atomic.load(Ordering::Acquire, &guard).as_ref().unwrap() };
            let mut new_config = *current;
            new_config.forward = Vec3::new(x, y, z).normalize();
            listener_atomic.store(Owned::new(new_config), Ordering::Release);
        }
        AudioCommand::SetSpatialParams { params } => {
            // Direct replacement - just store the new params
            spatial_atomic.store(Owned::new(params), Ordering::Release);
        }
        AudioCommand::SetSoundCone { id, cone } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                sound.spatial_cone = cone;
                sound.spatial_dirty = true; // Mark for recalculation
            }
        }
        AudioCommand::SetSoundOcclusion { id, occlusion } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                sound.occlusion = occlusion.clamp(0.0, 1.0);
                sound.spatial_dirty = true; // Mark for recalculation
            }
        }
        AudioCommand::PauseAll => {
            for sound in active_sounds.iter_mut().flatten() {
                sound.paused = true;
            }
        }
        AudioCommand::ResumeAll => {
            for sound in active_sounds.iter_mut().flatten() {
                sound.paused = false;
            }
        }
        AudioCommand::StopAll => {
            active_sounds.iter_mut().for_each(|slot| *slot = None); // Clear all slots
        }
        AudioCommand::FadeOut { id, duration } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                sound.fade_start_time = Some(sound.elapsed_time);
                sound.fade_duration = duration;
                sound.fade_start_volume = sound.volume;
                sound.fade_target_volume = 0.0;
            }
        }
        AudioCommand::FadeIn {
            id,
            duration,
            target_volume,
        } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                sound.fade_start_time = Some(sound.elapsed_time);
                sound.fade_duration = duration;
                sound.fade_start_volume = sound.volume;
                sound.fade_target_volume = target_volume.clamp(0.0, 1.0);
            }
        }
        AudioCommand::TweenPan {
            id,
            target_pan,
            duration,
        } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                sound.pan_tween_start_time = Some(sound.elapsed_time);
                sound.pan_tween_duration = duration;
                sound.pan_tween_start_value = sound.pan;
                sound.pan_tween_target_value = target_pan.clamp(-1.0, 1.0);
            }
        }
        AudioCommand::TweenPlaybackRate {
            id,
            target_rate,
            duration,
        } => {
            let index = id as usize;
            if let Some(Some(sound)) = active_sounds.get_mut(index) {
                sound.rate_tween_start_time = Some(sound.elapsed_time);
                sound.rate_tween_duration = duration;
                sound.rate_tween_start_value = sound.playback_rate;
                sound.rate_tween_target_value = target_rate.max(0.1); // Prevent division by zero
            }
        }
        // Streaming commands (native only)
        #[cfg(not(target_arch = "wasm32"))]
        AudioCommand::StreamFile {
            id,
            path,
            looping,
            volume,
            pan,
        } => {
            // Create ring buffer (5 seconds of stereo audio at 44.1kHz = ~441000 samples)
            let ring_buffer_size = (sample_rate * 5.0 * 2.0) as usize;
            let ring_buffer = HeapRb::<f32>::new(ring_buffer_size);
            let (ring_producer, ring_consumer) = ring_buffer.split();

            // Create control signals
            let stop_signal = Arc::new(AtomicBool::new(false));
            let pause_signal = Arc::new(AtomicBool::new(false));

            // Spawn decoder thread
            let stop_signal_clone = Arc::clone(&stop_signal);
            let pause_signal_clone = Arc::clone(&pause_signal);
            let decoder_thread = thread::spawn(move || {
                super::streaming::decoder_thread_func(
                    path,
                    ring_producer,
                    stop_signal_clone,
                    pause_signal_clone,
                    looping,
                );
            });

            // Add to streaming sounds (sparse vector)
            let index = id as usize;
            while streaming_sounds.len() <= index {
                streaming_sounds.push(None);
            }

            streaming_sounds[index] = Some(StreamingSound {
                ring_consumer,
                decoder_thread: Some(decoder_thread),
                stop_signal,
                pause_signal,
                volume,
                pan,
                looping,
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        AudioCommand::StopStream { id } => {
            // Setting to None will trigger Drop, which signals thread to stop
            let index = id as usize;
            if index < streaming_sounds.len() {
                streaming_sounds[index] = None;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        AudioCommand::PauseStream { id } => {
            let index = id as usize;
            if let Some(Some(stream)) = streaming_sounds.get_mut(index) {
                stream.pause_signal.store(true, Ordering::Relaxed);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        AudioCommand::ResumeStream { id } => {
            let index = id as usize;
            if let Some(Some(stream)) = streaming_sounds.get_mut(index) {
                stream.pause_signal.store(false, Ordering::Relaxed);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        AudioCommand::SetStreamVolume { id, volume } => {
            let index = id as usize;
            if let Some(Some(stream)) = streaming_sounds.get_mut(index) {
                stream.volume = volume.clamp(0.0, 1.0);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        AudioCommand::SetStreamPan { id, pan } => {
            let index = id as usize;
            if let Some(Some(stream)) = streaming_sounds.get_mut(index) {
                stream.pan = pan.clamp(-1.0, 1.0);
            }
        }
    }
}

/// Mix all active sounds into the output buffer (called from audio thread)
///
/// This function is ALLOCATION-FREE - all buffers are pre-allocated and reused.
#[allow(clippy::too_many_arguments)]
pub(crate) fn mix_sounds(
    output: &mut [f32],
    active_sounds: &mut [Option<ActiveSound>],
    temp_buffer: &mut Vec<f32>,
    finished_sounds: &mut Vec<SoundId>,
    listener: &ListenerConfig,
    spatial_params: &SpatialParams,
    sample_rate: f32,
    channels: usize,
) {
    // Clear output buffer
    output.fill(0.0);

    // Clear finished sounds list (reuse allocation)
    finished_sounds.clear();

    // Ensure temp buffer is large enough (may resize on first call, then reuses)
    let num_frames = output.len() / channels;
    let required_size = num_frames * 2;
    if temp_buffer.len() < required_size {
        temp_buffer.resize(required_size, 0.0);
    }

    // Mix each active sound using block processing (cache-friendly sequential iteration)
    for (idx, sound_opt) in active_sounds.iter_mut().enumerate() {
        let sound = match sound_opt {
            Some(s) => s,
            None => continue, // Skip empty slots
        };

        if sound.paused {
            continue;
        }

        let duration = sound.mixer.total_duration();

        // Check if sound will finish during this block
        let time_delta = 1.0 / sample_rate;

        if sound.elapsed_time >= duration {
            if sound.looping {
                sound.elapsed_time = 0.0;
                sound.sample_clock = 0.0;
            } else {
                finished_sounds.push(idx as u64);
                continue;
            }
        }

        // Only apply composition-time spatial audio if NO runtime position is set
        let (listener_for_mixer, params_for_mixer) = if sound.spatial_position.is_some() {
            (None, None) // Runtime position will handle spatial audio
        } else {
            (Some(listener), Some(spatial_params)) // Use composition-time position
        };

        // Process entire block at once
        // Note: process_block fully overwrites the buffer, no need to clear it first
        sound.mixer.process_block(
            &mut temp_buffer[..required_size],
            sample_rate,
            sound.elapsed_time,
            listener_for_mixer,
            params_for_mixer,
        );

        // Apply pan tween if active (before calculating spatial audio)
        if let Some(tween_start) = sound.pan_tween_start_time {
            let tween_elapsed = sound.elapsed_time - tween_start;
            if tween_elapsed >= sound.pan_tween_duration {
                // Tween complete
                sound.pan = sound.pan_tween_target_value;
                sound.pan_tween_start_time = None;
            } else {
                // Interpolate
                let t = (tween_elapsed / sound.pan_tween_duration).clamp(0.0, 1.0);
                sound.pan = sound.pan_tween_start_value
                    + (sound.pan_tween_target_value - sound.pan_tween_start_value) * t;
            }
        }

        // Apply playback rate tween if active
        if let Some(tween_start) = sound.rate_tween_start_time {
            let tween_elapsed = sound.elapsed_time - tween_start;
            if tween_elapsed >= sound.rate_tween_duration {
                // Tween complete
                sound.playback_rate = sound.rate_tween_target_value;
                sound.rate_tween_start_time = None;
            } else {
                // Interpolate
                let t = (tween_elapsed / sound.rate_tween_duration).clamp(0.0, 1.0);
                sound.playback_rate = sound.rate_tween_start_value
                    + (sound.rate_tween_target_value - sound.rate_tween_start_value) * t;
            }
        }

        // Calculate spatial audio if runtime position is set
        // Use cached values if nothing changed, otherwise recalculate
        let (mut spatial_volume, spatial_pan, spatial_pitch, spatial_occlusion) =
            if let Some(pos) = &sound.spatial_position {
                if sound.spatial_dirty {
                    // Recalculate spatial audio
                    let result = calculate_spatial_with_cone(
                        pos,
                        listener,
                        spatial_params,
                        sound.spatial_cone.as_ref(),
                        sound.occlusion,
                    );
                    // Cache the results
                    sound.cached_spatial_volume = result.volume;
                    sound.cached_spatial_pan = result.pan;
                    sound.cached_spatial_pitch = result.pitch;
                    sound.spatial_dirty = false; // Mark as clean
                    (result.volume, result.pan, result.pitch, result.occlusion)
                } else {
                    // Use cached values
                    (
                        sound.cached_spatial_volume,
                        sound.cached_spatial_pan,
                        sound.cached_spatial_pitch,
                        sound.occlusion, // Occlusion is just read directly, not cached
                    )
                }
            } else {
                (1.0, sound.pan, 1.0, 0.0)
            };

        // Apply occlusion as volume reduction
        // 0.0 = no occlusion (full volume), 1.0 = fully occluded (silent)
        spatial_volume *= 1.0 - spatial_occlusion;

        // Apply doppler pitch shift to playback rate
        let effective_playback_rate = sound.playback_rate * spatial_pitch;
        let base_block_duration = num_frames as f32 * time_delta;

        // Mix temp buffer into output with volume/pan/fade applied
        // Use SIMD fast path when no fade is active (common case)
        if sound.fade_start_time.is_none() && channels == 2 {
            // SIMD fast path: no fade, stereo output
            let combined_volume = sound.volume * spatial_volume;
            let num_frames = temp_buffer.len() / 2;

            // Calculate pan multipliers once
            let (left_pan, right_pan) = if spatial_pan < 0.0 {
                (1.0, 1.0 + spatial_pan)
            } else {
                (1.0 - spatial_pan, 1.0)
            };

            match SIMD.simd_width() {
                SimdWidth::X8 => {
                    // Process 8 stereo frames (16 samples) at once
                    // But only if we have enough room in the output buffer
                    let max_frames_in_output = output.len() / 2;
                    let safe_frames = num_frames.min(max_frames_in_output);
                    let chunks_of_16 = safe_frames / 8;
                    let remainder_start = chunks_of_16 * 8;

                    let vol_vec = f32x8::splat(combined_volume);
                    let left_pan_vec = f32x8::splat(left_pan);
                    let right_pan_vec = f32x8::splat(right_pan);

                    // Pre-allocate temp arrays for SIMD operations (stack allocated, fast)
                    let mut input_left = [0.0f32; 8];
                    let mut input_right = [0.0f32; 8];
                    let mut output_left = [0.0f32; 8];
                    let mut output_right = [0.0f32; 8];

                    for chunk_idx in 0..chunks_of_16 {
                        let frame_start = chunk_idx * 8;
                        let temp_start = frame_start * 2;
                        let out_start = frame_start * 2;

                        // Deinterleave input using SIMD (dispatches to AVX2/SSE/scalar)
                        SIMD.deinterleave_stereo(
                            &temp_buffer[temp_start..],
                            &mut input_left,
                            &mut input_right,
                        );

                        // Deinterleave output using SIMD
                        SIMD.deinterleave_stereo(
                            &output[out_start..],
                            &mut output_left,
                            &mut output_right,
                        );

                        // Load into SIMD vectors for processing
                        let left = f32x8::from(input_left);
                        let right = f32x8::from(input_right);
                        let out_left = f32x8::from(output_left);
                        let out_right = f32x8::from(output_right);

                        // Apply volume and pan
                        let left_out = left * vol_vec * left_pan_vec;
                        let right_out = right * vol_vec * right_pan_vec;

                        // Add (mix)
                        let mixed_left = out_left + left_out;
                        let mixed_right = out_right + right_out;

                        // Store back to arrays
                        output_left = mixed_left.to_array();
                        output_right = mixed_right.to_array();

                        // Interleave and store using SIMD (dispatches to AVX2/SSE/scalar)
                        SIMD.interleave_stereo(&output_left, &output_right, &mut output[out_start..]);
                    }

                    // Handle remainder frames with scalar code
                    for frame_idx in remainder_start..num_frames {
                        let temp_idx = frame_idx * 2;
                        let out_idx = frame_idx * 2;

                        if temp_idx + 1 < temp_buffer.len() && out_idx + 1 < output.len() {
                            let left = temp_buffer[temp_idx] * combined_volume * left_pan;
                            let right = temp_buffer[temp_idx + 1] * combined_volume * right_pan;

                            output[out_idx] += left;
                            output[out_idx + 1] += right;
                        }
                    }
                }
                _ => {
                    // Fallback: scalar path
                    for frame_idx in 0..num_frames {
                        let temp_idx = frame_idx * 2;
                        let out_idx = frame_idx * 2;

                        let left = temp_buffer[temp_idx] * combined_volume * left_pan;
                        let right = temp_buffer[temp_idx + 1] * combined_volume * right_pan;

                        if out_idx + 1 < output.len() {
                            output[out_idx] += left;
                            output[out_idx + 1] += right;
                        }
                    }
                }
            }
        } else {
            // Scalar path: fade is active or mono output
            for (frame_idx, temp_frame) in temp_buffer.chunks(2).enumerate() {
                let frame_time =
                    sound.elapsed_time + (frame_idx as f32 * time_delta * effective_playback_rate);

                // Apply fade if active
                let effective_volume = if let Some(fade_start) = sound.fade_start_time {
                    let fade_elapsed = frame_time - fade_start;
                    if fade_elapsed >= sound.fade_duration {
                        // Fade complete
                        if frame_idx == 0 {
                            sound.volume = sound.fade_target_volume;
                            sound.fade_start_time = None;
                        }
                        sound.fade_target_volume
                    } else {
                        // Interpolate
                        let t = (fade_elapsed / sound.fade_duration).clamp(0.0, 1.0);
                        sound.fade_start_volume
                            + (sound.fade_target_volume - sound.fade_start_volume) * t
                    }
                } else {
                    sound.volume
                };

                let mut left = temp_frame[0];
                let mut right = temp_frame[1];

                // Apply volume
                left *= effective_volume * spatial_volume;
                right *= effective_volume * spatial_volume;

                // Apply pan
                if spatial_pan < 0.0 {
                    right *= 1.0 + spatial_pan;
                } else if spatial_pan > 0.0 {
                    left *= 1.0 - spatial_pan;
                }

                // Mix into output
                let out_idx = frame_idx * channels;
                if out_idx + 1 < output.len() {
                    if channels == 1 {
                        output[out_idx] += (left + right) * 0.5;
                    } else {
                        output[out_idx] += left;
                        output[out_idx + 1] += right;
                    }
                }
            }
        }

        // Advance time with doppler-adjusted playback rate
        // This ensures mixer renders samples at the correct pitch
        // Note: block_duration already includes playback_rate, so we use
        // base_block_duration here to avoid applying playback_rate twice
        sound.elapsed_time += base_block_duration * effective_playback_rate;
        sound.sample_clock =
            (sound.sample_clock + (num_frames as f32 * effective_playback_rate)) % sample_rate;
    }

    // Remove finished sounds (set slots to None)
    for id in finished_sounds {
        let index = *id as usize;
        if index < active_sounds.len() {
            active_sounds[index] = None;
        }
    }

    // Clamp output to prevent distortion (SIMD accelerated)
    SIMD.clamp_buffer(output, -1.0, 1.0);
}

#[cfg(not(target_arch = "wasm32"))]
/// Mix streaming sounds into the output buffer (called from audio thread)
///
/// Reads decoded samples from ring buffers and mixes them into the output.
/// This is ALLOCATION-FREE and lock-free (uses lockless ring buffer).
pub(crate) fn mix_streaming_sounds(
    output: &mut [f32],
    streaming_sounds: &mut [Option<StreamingSound>],
    finished_streams: &mut Vec<SoundId>,
    channels: usize,
) {
    use ringbuf::traits::{Consumer, Observer};

    // Clear finished streams list
    finished_streams.clear();

    // Mix each streaming sound (cache-friendly sequential iteration)
    for (idx, stream_opt) in streaming_sounds.iter_mut().enumerate() {
        let stream = match stream_opt {
            Some(s) => s,
            None => continue, // Skip empty slots
        };

        // Check if the decoder thread has finished
        if let Some(handle) = &stream.decoder_thread {
            if handle.is_finished() {
                // Thread finished - mark for removal
                finished_streams.push(idx as u64);
                continue;
            }
        }

        // Read available samples from ring buffer
        let available = stream.ring_consumer.occupied_len();
        if available == 0 {
            // Buffer underrun - could happen at start or if decoding is slow
            continue;
        }

        // Calculate how many samples we need (limited by output buffer size)
        let samples_needed = output.len().min(available);

        // Mix samples into output
        for i in (0..samples_needed).step_by(channels) {
            // Pop samples from ring buffer
            let left = stream.ring_consumer.try_pop().unwrap_or(0.0);
            let right = if channels == 2 {
                stream.ring_consumer.try_pop().unwrap_or(0.0)
            } else {
                left // Mono - use same sample for both channels
            };

            // Apply volume and pan
            let pan = stream.pan;
            let left_gain = if pan <= 0.0 { 1.0 } else { 1.0 - pan } * stream.volume;
            let right_gain = if pan >= 0.0 { 1.0 } else { 1.0 + pan } * stream.volume;

            // Mix into output (additively)
            if i < output.len() {
                output[i] += left * left_gain;
            }
            if i + 1 < output.len() {
                output[i + 1] += right * right_gain;
            }
        }
    }

    // Remove finished streams (set slots to None)
    for id in finished_streams.iter() {
        let index = *id as usize;
        if index < streaming_sounds.len() {
            streaming_sounds[index] = None;
        }
    }
}
