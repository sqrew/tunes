//! Audio engine for real-time playback and control.
//!
//! The `AudioEngine` provides a high-level interface for playing compositions,
//! samples, and streaming audio files with real-time control over volume, pan,
//! spatial positioning, and more.
//!
//! # Architecture
//!
//! The engine uses a command-based architecture where the main thread sends
//! commands to the audio thread via a lock-free channel. This ensures the
//! audio thread never blocks on the main thread.
//!
//! # Modules
//!
//! - `commands` - Command enum for thread communication
//! - `active_sound` - State for playing sounds
//! - `streaming` - Background audio file streaming (native only)
//! - `callback` - Audio callback and mixing logic
//! - `sample_builder` - Builder for one-shot sample playback

mod active_sound;
mod callback;
mod commands;
mod sample_builder;
#[cfg(not(target_arch = "wasm32"))]
mod streaming;

pub use commands::SoundId;
pub use sample_builder::SamplePlaybackBuilder;

// Composition and Tempo are used for examples in doc comments
use crate::error::{Result, TunesError};
use crate::synthesis::simd::{SimdWidth, SIMD};
use crate::synthesis::spatial::{ListenerConfig, SoundCone, SpatialParams, SpatialPosition};
use crate::synthesis::Sample;
use crate::track::Mixer;
use std::thread;
use std::time::Duration;

use callback::{handle_command, mix_sounds, AudioCallbackState};
use commands::AudioCommand;
#[cfg(not(target_arch = "wasm32"))]
use callback::mix_streaming_sounds;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::epoch::{self, Atomic};
use dashmap::DashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// Type alias for the inner monitor callback function
type MonitorCallbackFn = Option<Box<dyn Fn(&[f32]) + Send + 'static>>;

/// Type alias for the thread-safe monitor callback
type MonitorCallback = Arc<Mutex<MonitorCallbackFn>>;

/// Central audio engine that manages playback with concurrent mixing
pub struct AudioEngine {
    command_tx: Sender<AudioCommand>,
    next_id: Arc<AtomicU64>,
    callback_state: Arc<Mutex<AudioCallbackState>>,
    #[allow(dead_code)] // Reserved for future spatial audio runtime control
    listener_config: Arc<Atomic<ListenerConfig>>, // Lock-free reads via epoch-based reclamation
    #[allow(dead_code)] // Reserved for future spatial audio runtime control
    spatial_params: Arc<Atomic<SpatialParams>>, // Lock-free reads via epoch-based reclamation
    sample_rate: f32,
    pub(crate) sample_cache: Arc<DashMap<String, crate::synthesis::Sample>>, // Lock-free sample caching
    _stream: cpal::Stream, // Persistent stream, kept alive
    // Info for optional printing
    device_name: String,
    buffer_size: u32,
    channels: usize,
    // GPU acceleration flag for play_sample()
    #[allow(dead_code)]
    pub(crate) enable_gpu_for_samples: bool,
    // Monitor callback for real-time audio visualization and analysis
    monitor_callback: MonitorCallback,
}

impl AudioEngine {
    /// Create a new audio engine with default output device
    ///
    /// Uses a moderate buffer size (4096 samples) optimized for pre-rendered playback.
    /// Since play_mixer() pre-renders audio, buffer size only affects latency, not stability.
    /// For lower latency, use `with_buffer_size()`.
    ///
    /// # Performance
    /// Default performance: 50-200x realtime (SIMD + Rayon automatic)
    pub fn new() -> Result<Self> {
        Self::with_buffer_size_and_gpu(4096, false)
    }

    /// Create a new audio engine with GPU acceleration enabled
    ///
    /// Enables transparent GPU acceleration for synthesis and export operations.
    /// GPU performance scales with hardware capabilities - discrete GPUs show
    /// significantly better performance than integrated GPUs.
    ///
    /// Automatically enables GPU for `export_wav()`, `export_flac()`, and
    /// `play_mixer_realtime()` operations without requiring manual `enable_gpu()` calls.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// let engine = AudioEngine::new_with_gpu()?;
    ///
    /// // Transparent GPU acceleration for export and playback
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("synth").note(&[440.0], 1.0);
    /// let mut mixer = comp.into_mixer();
    /// engine.export_wav(&mut mixer, "output.wav")?;  // GPU accelerated
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    ///
    /// # Performance
    /// - Integrated GPUs: 1.0-1.2x speedup (measured on Intel HD 530)
    /// - Discrete GPUs: Performance scales with compute capacity and memory bandwidth
    /// - CPU fallback: Automatic if GPU unavailable
    /// - Warning: Auto-detects integrated GPUs and displays performance advisory
    pub fn new_with_gpu() -> Result<Self> {
        Self::with_buffer_size_and_gpu(4096, true)
    }

    /// Create a new audio engine with custom buffer size
    ///
    /// Creates a persistent audio stream that can play multiple sounds concurrently.
    ///
    /// # Arguments
    /// * `buffer_size` - Buffer size in samples
    ///   - Smaller (512-1024): Lower latency, may underrun with complex synthesis
    ///   - Medium (2048-4096): Balanced
    ///   - Large (8192-16384): Very stable for most cases
    pub fn with_buffer_size(buffer_size: u32) -> Result<Self> {
        Self::with_buffer_size_and_gpu(buffer_size, false)
    }

    /// Create a new audio engine with custom buffer size and GPU flag (internal)
    fn with_buffer_size_and_gpu(buffer_size: u32, enable_gpu: bool) -> Result<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or_else(|| {
            TunesError::AudioEngineError("No output device available".to_string())
        })?;
        let config = device.default_output_config().map_err(|e| {
            TunesError::AudioEngineError(format!("Failed to get default config: {}", e))
        })?;

        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;
        let device_name = device.name().unwrap_or_else(|_| "Unknown".to_string());

        // Create command channel for communication with audio thread
        let (command_tx, command_rx): (Sender<AudioCommand>, Receiver<AudioCommand>) = unbounded();

        // Shared state for audio callback (includes pre-allocated buffers)
        let callback_state: Arc<Mutex<AudioCallbackState>> =
            Arc::new(Mutex::new(AudioCallbackState::new()));
        let callback_state_for_stream = Arc::clone(&callback_state);

        // Lock-free shared state for spatial audio (epoch-based reclamation)
        let listener_config = Arc::new(Atomic::new(ListenerConfig::new()));
        let listener_config_for_stream = Arc::clone(&listener_config);

        let spatial_params = Arc::new(Atomic::new(SpatialParams::default()));
        let spatial_params_for_stream = Arc::clone(&spatial_params);

        // Monitor callback for audio visualization/analysis
        let monitor_callback: MonitorCallback = Arc::new(Mutex::new(None));
        let monitor_callback_for_stream = Arc::clone(&monitor_callback);

        // Build stream configuration
        let mut stream_config: cpal::StreamConfig = config.clone().into();
        stream_config.buffer_size = cpal::BufferSize::Fixed(buffer_size);

        // Error handler
        let err_fn = |err| eprintln!("Audio stream error: {}", err);

        // Build the persistent output stream
        let stream = device
            .build_output_stream(
                &stream_config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // Lock only AudioCallbackState (one lock instead of three!)
                    // If mutex is poisoned, output silence and return early
                    let mut state = match callback_state_for_stream.lock() {
                        Ok(state) => state,
                        Err(e) => {
                            eprintln!("Audio callback: mutex poisoned: {}", e);
                            // Fill buffer with silence
                            for sample in data.iter_mut() {
                                *sample = 0.0;
                            }
                            return;
                        }
                    };

                    // Lock-free reads of spatial audio config via epoch-based reclamation
                    let guard = epoch::pin();

                    // Use defaults if atomic loads return null (graceful degradation)
                    let default_listener = ListenerConfig::default();
                    let listener = unsafe {
                        listener_config_for_stream
                            .load(Ordering::Acquire, &guard)
                            .as_ref()
                    };
                    let listener = listener.unwrap_or(&default_listener);

                    let default_spatial = SpatialParams::default();
                    let spatial = unsafe {
                        spatial_params_for_stream
                            .load(Ordering::Acquire, &guard)
                            .as_ref()
                    };
                    let spatial = spatial.unwrap_or(&default_spatial);

                    // Destructure state FIRST to get separate mutable references (satisfies borrow checker)
                    let AudioCallbackState {
                        ref mut active_sounds,
                        #[cfg(not(target_arch = "wasm32"))]
                        ref mut streaming_sounds,
                        ref mut temp_buffer,
                        ref mut finished_sounds,
                        #[cfg(not(target_arch = "wasm32"))]
                        ref mut finished_streams,
                    } = *state;

                    // Process all pending commands (non-blocking)
                    while let Ok(cmd) = command_rx.try_recv() {
                        handle_command(
                            cmd,
                            active_sounds,
                            #[cfg(not(target_arch = "wasm32"))]
                            streaming_sounds,
                            &listener_config_for_stream,
                            &spatial_params_for_stream,
                            sample_rate,
                        );
                    }

                    // Mix all active sounds into the output buffer (allocation-free)
                    mix_sounds(
                        data,
                        active_sounds,
                        temp_buffer,
                        finished_sounds,
                        listener,
                        spatial,
                        sample_rate,
                        channels,
                    );

                    // Mix streaming sounds into the output buffer
                    #[cfg(not(target_arch = "wasm32"))]
                    mix_streaming_sounds(data, streaming_sounds, finished_streams, channels);

                    // Call monitor callback if set (for visualization/analysis)
                    if let Ok(callback_guard) = monitor_callback_for_stream.lock() {
                        if let Some(ref callback) = *callback_guard {
                            callback(data);
                        }
                    }

                    // Guard dropped here - safe to reclaim old epochs
                },
                err_fn,
                None,
            )
            .map_err(|e| {
                TunesError::AudioEngineError(format!("Failed to build output stream: {}", e))
            })?;

        // Start the stream
        stream.play().map_err(|e| {
            TunesError::AudioEngineError(format!("Failed to start audio stream: {}", e))
        })?;

        Ok(Self {
            command_tx,
            next_id: Arc::new(AtomicU64::new(1)),
            callback_state,
            listener_config,
            spatial_params,
            sample_rate,
            sample_cache: Arc::new(DashMap::new()),
            _stream: stream,
            device_name,
            buffer_size,
            channels,
            enable_gpu_for_samples: enable_gpu,
            monitor_callback,
        })
    }

    /// Print audio engine initialization information
    ///
    /// Displays device name, sample rate, buffer size, latency, and configuration.
    /// This is an opt-in method - call it if you want to see engine initialization details.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    /// engine.print_info(); // Optional - only if you want to see initialization info
    /// # Ok(())
    /// # }
    /// ```
    pub fn print_info(&self) {
        let latency_ms = (self.buffer_size as f32 / self.sample_rate) * 1000.0;
        let simd_width = SIMD.simd_width();
        let simd_lanes = SIMD.width();
        let simd_name = match simd_width {
            SimdWidth::X8 => "AVX2",
            SimdWidth::X4 => {
                #[cfg(target_arch = "x86_64")]
                {
                    "SSE"
                }
                #[cfg(not(target_arch = "x86_64"))]
                {
                    "NEON"
                }
            }
            SimdWidth::Scalar => "Scalar (no SIMD)",
        };

        println!("Audio Engine initialized:");
        println!("  Device: {}", self.device_name);
        println!("  Sample rate: {} Hz", self.sample_rate as u32);
        println!(
            "  Buffer size: {} samples ({:.1}ms latency)",
            self.buffer_size, latency_ms
        );
        println!("  Channels: {}", self.channels);
        println!("  SIMD: {} ({} lanes)", simd_name, simd_lanes);
        println!("  Concurrent mixing: enabled");
    }

    // ============================================================================
    // Playback Methods
    // ============================================================================

    /// Play a composition and block until it finishes
    ///
    /// This is the main method for simple use cases, examples, and scripts.
    /// It plays the composition and blocks until playback is complete.
    ///
    /// For non-blocking playback (games, interactive use), use `play_mixer_realtime()`.
    ///
    /// # Returns
    /// `Ok(())` on successful playback. Note that this returns success even if the
    /// mixer is empty - check with `mixer.is_empty()` first if you want to detect this.
    pub fn play_mixer(&self, mixer: &Mixer) -> Result<()> {
        let id = self.play_mixer_realtime(mixer)?;
        self.wait_for(id, mixer.is_empty())
    }

    /// Play a composition in real-time mode, returns immediately
    ///
    /// **BREAKING CHANGE:** This method now returns `SoundId` instead of blocking.
    /// This enables concurrent playback for games and interactive applications.
    ///
    /// # Returns
    /// `SoundId` - Unique identifier for this sound, use with control methods
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// let engine = AudioEngine::new()?;
    ///
    /// // Non-blocking - returns immediately
    /// let sound_id = engine.play_mixer_realtime(&comp.into_mixer())?;
    ///
    /// // Control the sound while it plays
    /// engine.set_volume(sound_id, 0.5)?;
    /// engine.set_pan(sound_id, -0.5)?; // Pan left
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_mixer_realtime(&self, mixer: &Mixer) -> Result<SoundId> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        // Clone mixer and automatically enable GPU if engine was created with GPU support
        #[cfg(feature = "gpu")]
        let mut mixer_clone = mixer.clone();
        #[cfg(not(feature = "gpu"))]
        let mixer_clone = mixer.clone();

        #[cfg(feature = "gpu")]
        if self.enable_gpu_for_samples {
            mixer_clone.enable_gpu();
        }

        self.command_tx
            .send(AudioCommand::Play {
                id,
                mixer: Box::new(mixer_clone),
                looping: false,
            })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(id)
    }

    /// Play a mixer at a custom playback rate and block until finished
    ///
    /// This is a convenience method that combines `play_mixer_realtime()` and
    /// `set_playback_rate()` for the common case of playing at a different speed.
    ///
    /// # Arguments
    /// * `mixer` - The mixer to play
    /// * `rate` - Playback rate multiplier (1.0 = normal, 2.0 = double speed, 0.5 = half speed)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// let engine = AudioEngine::new()?;
    /// let mixer = comp.into_mixer();
    ///
    /// // Play at 2x speed (chipmunk effect)
    /// engine.play_mixer_at_rate(&mixer, 2.0)?;
    ///
    /// // Play at half speed (slow motion)
    /// engine.play_mixer_at_rate(&mixer, 0.5)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_mixer_at_rate(&self, mixer: &Mixer, rate: f32) -> Result<()> {
        let id = self.play_mixer_realtime(mixer)?;
        self.set_playback_rate(id, rate)?;
        self.wait_for(id, mixer.is_empty())
    }

    /// Play a mixer at a custom playback rate and return immediately
    ///
    /// Returns a `SoundId` for controlling the playing instance. The playback rate
    /// is set immediately after starting playback.
    ///
    /// # Arguments
    /// * `mixer` - The mixer to play
    /// * `rate` - Playback rate multiplier (1.0 = normal, 2.0 = double speed, 0.5 = half speed)
    ///
    /// # Returns
    /// `SoundId` - Unique identifier for this sound, use with control methods
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// let engine = AudioEngine::new()?;
    /// let mixer = comp.into_mixer();
    ///
    /// // Start playing at 1.5x speed, non-blocking
    /// let id = engine.play_mixer_realtime_at_rate(&mixer, 1.5)?;
    ///
    /// // Can still control it further
    /// engine.set_volume(id, 0.7)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_mixer_realtime_at_rate(&self, mixer: &Mixer, rate: f32) -> Result<SoundId> {
        let id = self.play_mixer_realtime(mixer)?;
        self.set_playback_rate(id, rate)?;
        Ok(id)
    }

    /// Play a composition with pre-rendering, blocks until finished
    ///
    /// Currently behaves the same as `play_mixer()` but reserved for future
    /// pre-rendering optimizations.
    pub fn play_mixer_prerender(&self, mixer: &Mixer) -> Result<()> {
        // For now, same as play_mixer - concurrent engine handles this efficiently
        // In the future, could pre-render to buffer for guaranteed zero glitches
        self.play_mixer(mixer)
    }

    /// Play a composition in a loop
    ///
    /// Returns immediately with a `SoundId`. The sound will loop until stopped.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// let engine = AudioEngine::new()?;
    /// let loop_id = engine.play_looping(&comp.into_mixer())?;
    ///
    /// // Later: stop the loop
    /// engine.stop(loop_id)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_looping(&self, mixer: &Mixer) -> Result<SoundId> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        // Clone mixer and automatically enable GPU if engine was created with GPU support
        #[cfg(feature = "gpu")]
        let mut mixer_clone = mixer.clone();
        #[cfg(not(feature = "gpu"))]
        let mixer_clone = mixer.clone();

        #[cfg(feature = "gpu")]
        if self.enable_gpu_for_samples {
            mixer_clone.enable_gpu();
        }

        self.command_tx
            .send(AudioCommand::Play {
                id,
                mixer: Box::new(mixer_clone),
                looping: true,
            })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(id)
    }

    /// Play a one-shot sample immediately (convenience method with automatic caching)
    ///
    /// Returns a builder that allows you to chain effects, volume, pan, speed, and more.
    /// The sample plays automatically when the builder drops (fire-and-forget).
    ///
    /// **Automatic caching:** Samples are automatically cached by path on first load. Subsequent
    /// calls with the same path reuse the cached sample (cheap Arc clone), making repeated sounds
    /// efficient without any extra code.
    ///
    /// # Arguments
    /// * `path` - Path to the sample file (WAV, OGG, MP3, FLAC supported)
    ///
    /// # Returns
    /// `SamplePlaybackBuilder` - Builder for chaining effects and parameters
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    ///
    /// // Simple playback
    /// engine.play_sample("assets/footstep.wav");
    ///
    /// // With effects (still fire-and-forget!)
    /// engine.play_sample("assets/explosion.wav")
    ///     .volume(0.8)
    ///     .speed(1.2)
    ///     .reverb(Reverb::new(0.5, 0.3, 0.2));
    ///
    /// // Spatial audio
    /// engine.play_sample("assets/gunshot.wav")
    ///     .spatial(5.0, 0.0, 10.0)  // 5m right, 10m forward
    ///     .volume(0.9);
    ///
    /// // Chain multiple effects
    /// engine.play_sample("assets/voice.wav")
    ///     .speed(0.8)
    ///     .reverb(Reverb::hall())
    ///     .delay(Delay::new(0.3, 0.4, 0.5))
    ///     .filter(Filter::low_pass(1200.0, 0.7));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Performance
    /// - **First call per unique path:** Loads from disk (~1-10ms depending on file size)
    /// - **Subsequent calls:** Instant (Arc clone from cache)
    /// - **Memory:** Cached samples remain in memory until cleared with `clear_sample_cache()`
    ///
    /// # Note
    /// This method is **non-blocking** and returns immediately. The sound plays when the builder
    /// drops, which happens at the end of the statement. Multiple sounds can play concurrently.
    ///
    /// For cache management, see `preload_sample()`, `clear_sample_cache()`, and `remove_cached_sample()`.
    ///
    /// For more control over synthesis or timing, use the full Composition API.
    pub fn play_sample(&self, path: &str) -> SamplePlaybackBuilder<'_> {
        SamplePlaybackBuilder::new(self, path)
    }

    /// Preload a sample into the cache without playing it
    ///
    /// Useful for loading frequently-used samples during initialization to avoid
    /// any loading delay on first playback.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    ///
    /// // Load samples during game initialization
    /// engine.preload_sample("assets/footstep.wav")?;
    /// engine.preload_sample("assets/jump.wav")?;
    /// engine.preload_sample("assets/explosion.wav")?;
    ///
    /// // Later: instant playback (already cached)
    /// engine.play_sample("assets/footstep.wav");
    /// # Ok(())
    /// # }
    /// ```
    pub fn preload_sample(&self, path: &str) -> Result<()> {
        if !self.sample_cache.contains_key(path) {
            let sample = Sample::from_file(path).map_err(|e| {
                TunesError::AudioEngineError(format!("Failed to preload sample '{}': {}", path, e))
            })?;
            // Use entry API to avoid race condition
            self.sample_cache.entry(path.to_string()).or_insert(sample);
        }

        Ok(())
    }

    /// Remove a specific sample from the cache
    ///
    /// Useful for freeing memory when a sample is no longer needed.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    ///
    /// // Use sample during level
    /// engine.play_sample("level1_music.wav");
    ///
    /// // Level complete - free the memory
    /// engine.remove_cached_sample("level1_music.wav")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove_cached_sample(&self, path: &str) -> Result<()> {
        self.sample_cache.remove(path);
        Ok(())
    }

    /// Clear all cached samples to free memory
    ///
    /// Useful for freeing memory between levels or game states.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    ///
    /// // Play various sounds during level
    /// engine.play_sample("sound1.wav");
    /// engine.play_sample("sound2.wav");
    ///
    /// // Level complete - clear all cached samples
    /// engine.clear_sample_cache()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn clear_sample_cache(&self) -> Result<()> {
        self.sample_cache.clear();
        Ok(())
    }

    // ============================================================================
    // Control Methods
    // ============================================================================

    /// Stop a playing sound
    pub fn stop(&self, id: SoundId) -> Result<()> {
        self.command_tx
            .send(AudioCommand::Stop { id })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Set the volume of a playing sound
    ///
    /// # Arguments
    /// * `id` - The sound to modify
    /// * `volume` - Volume level (0.0 = silence, 1.0 = full volume)
    pub fn set_volume(&self, id: SoundId, volume: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetVolume { id, volume })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Set the stereo pan of a playing sound
    ///
    /// # Arguments
    /// * `id` - The sound to modify
    /// * `pan` - Pan position (-1.0 = full left, 0.0 = center, 1.0 = full right)
    pub fn set_pan(&self, id: SoundId, pan: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetPan { id, pan })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Set the playback rate (speed and pitch) of a playing sound
    ///
    /// Changes both the speed and pitch of the sound. Higher values = faster/higher,
    /// lower values = slower/lower. Clamped to 0.1x - 4.0x for stability.
    ///
    /// # Arguments
    /// * `id` - The sound to modify
    /// * `rate` - Playback rate multiplier (1.0 = normal, 2.0 = double speed/octave up, 0.5 = half speed/octave down)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// # comp.track("sfx").note(&[440.0], 1.0);
    /// let engine = AudioEngine::new()?;
    /// let sound_id = engine.play_mixer_realtime(&comp.into_mixer())?;
    ///
    /// // Play at double speed (one octave higher)
    /// engine.set_playback_rate(sound_id, 2.0)?;
    ///
    /// // Play at half speed (one octave lower)
    /// engine.set_playback_rate(sound_id, 0.5)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Common use cases
    /// - Footstep variations (0.9 - 1.1 for subtle variation)
    /// - Impact sounds based on velocity (0.8 - 1.5)
    /// - Voice pitch shifting
    /// - Retro game sound effects
    pub fn set_playback_rate(&self, id: SoundId, rate: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetPlaybackRate { id, rate })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Pause a playing sound
    pub fn pause(&self, id: SoundId) -> Result<()> {
        self.command_tx
            .send(AudioCommand::Pause { id })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Resume a paused sound
    pub fn resume(&self, id: SoundId) -> Result<()> {
        self.command_tx
            .send(AudioCommand::Resume { id })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Pause all currently playing sounds
    ///
    /// Useful for game pause menus or when the application loses focus.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = AudioEngine::new()?;
    /// // Pause all audio when game pauses
    /// engine.pause_all()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn pause_all(&self) -> Result<()> {
        self.command_tx
            .send(AudioCommand::PauseAll)
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Resume all paused sounds
    ///
    /// Useful for resuming from a pause menu.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = AudioEngine::new()?;
    /// // Resume all audio when game unpauses
    /// engine.resume_all()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn resume_all(&self) -> Result<()> {
        self.command_tx
            .send(AudioCommand::ResumeAll)
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Stop all currently playing sounds
    ///
    /// Immediately stops and removes all active sounds. Useful for level transitions
    /// or when you need to clear all audio.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = AudioEngine::new()?;
    /// // Clear all audio when transitioning levels
    /// engine.stop_all()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn stop_all(&self) -> Result<()> {
        self.command_tx
            .send(AudioCommand::StopAll)
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Set a callback function to monitor the audio output stream
    ///
    /// The callback receives the final mixed audio buffer that will be sent to the speakers.
    /// This is useful for real-time visualization (oscilloscopes, waveforms, spectrum analyzers),
    /// audio recording, or analysis. The callback is called from the audio thread, so it should
    /// be fast and non-blocking.
    ///
    /// **Performance note:** The callback is called once per audio buffer (typically every 10-20ms).
    /// Keep processing minimal to avoid audio dropouts. For heavy processing, copy the data
    /// and process it in a separate thread.
    ///
    /// # Arguments
    /// * `callback` - Function that receives audio samples. Set to `None` to disable monitoring.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # use std::sync::{Arc, Mutex};
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    /// let sample_buffer = Arc::new(Mutex::new(Vec::new()));
    /// let buffer_clone = sample_buffer.clone();
    ///
    /// // Set up monitoring for oscilloscope visualization
    /// engine.set_monitor_callback(Some(Box::new(move |samples: &[f32]| {
    ///     let mut buffer = buffer_clone.lock().unwrap();
    ///     buffer.clear();
    ///     buffer.extend_from_slice(samples);
    /// })));
    ///
    /// // Now play audio and visualize it
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("synth").note(&[440.0], 1.0);
    /// engine.play_mixer(&comp.into_mixer())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_monitor_callback(&self, callback: MonitorCallbackFn) {
        if let Ok(mut guard) = self.monitor_callback.lock() {
            *guard = callback;
        }
    }

    /// Fade out a playing sound to silence
    ///
    /// Gradually reduces the volume to 0 over the specified duration, creating a
    /// smooth fade out effect. The sound will stop automatically when the fade completes.
    ///
    /// # Arguments
    /// * `id` - The sound to fade
    /// * `duration` - Fade duration in seconds
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// # let engine = AudioEngine::new()?;
    /// let id = engine.play_mixer_realtime(&comp.into_mixer())?;
    ///
    /// // Fade out over 2 seconds
    /// engine.fade_out(id, 2.0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn fade_out(&self, id: SoundId, duration: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::FadeOut { id, duration })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Fade in a playing sound from current volume to target volume
    ///
    /// Gradually increases the volume from its current level to the target volume,
    /// creating a smooth fade in effect.
    ///
    /// # Arguments
    /// * `id` - The sound to fade
    /// * `duration` - Fade duration in seconds
    /// * `target_volume` - Target volume (0.0-1.0)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// # let engine = AudioEngine::new()?;
    /// let id = engine.play_mixer_realtime(&comp.into_mixer())?;
    /// engine.set_volume(id, 0.0)?; // Start silent
    ///
    /// // Fade in to 80% volume over 3 seconds
    /// engine.fade_in(id, 3.0, 0.8)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn fade_in(&self, id: SoundId, duration: f32, target_volume: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::FadeIn {
                id,
                duration,
                target_volume,
            })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Smoothly tween the pan of a playing sound
    ///
    /// Gradually changes the pan from its current position to the target pan position
    /// over the specified duration. Perfect for creating smooth panning effects like
    /// sounds moving from left to right.
    ///
    /// # Arguments
    /// * `id` - The sound to pan
    /// * `target_pan` - Target pan position (-1.0 = full left, 0.0 = center, 1.0 = full right)
    /// * `duration` - Tween duration in seconds
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// # let engine = AudioEngine::new()?;
    /// let id = engine.play_mixer_realtime(&comp.into_mixer())?;
    ///
    /// // Smoothly pan from left to right over 5 seconds (helicopter flyby effect)
    /// engine.set_pan(id, -1.0)?; // Start at full left
    /// engine.tween_pan(id, 1.0, 5.0)?; // Pan to full right over 5 seconds
    /// # Ok(())
    /// # }
    /// ```
    pub fn tween_pan(&self, id: SoundId, target_pan: f32, duration: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::TweenPan {
                id,
                target_pan,
                duration,
            })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Smoothly tween the playback rate (pitch and speed) of a playing sound
    ///
    /// Gradually changes the playback rate from its current value to the target rate
    /// over the specified duration. Since playback rate affects both pitch and speed,
    /// this creates effects like engine sounds ramping up or slowing down.
    ///
    /// # Arguments
    /// * `id` - The sound to modify
    /// * `target_rate` - Target playback rate (1.0 = normal, 2.0 = double speed/pitch, 0.5 = half speed/pitch)
    /// * `duration` - Tween duration in seconds
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// # let engine = AudioEngine::new()?;
    /// let id = engine.play_mixer_realtime(&comp.into_mixer())?;
    ///
    /// // Smoothly speed up engine sound over 3 seconds (acceleration)
    /// engine.tween_playback_rate(id, 2.0, 3.0)?;
    ///
    /// // Later: slow down over 2 seconds (deceleration)
    /// engine.tween_playback_rate(id, 0.5, 2.0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tween_playback_rate(&self, id: SoundId, target_rate: f32, duration: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::TweenPlaybackRate {
                id,
                target_rate,
                duration,
            })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    /// Check if a sound is still playing
    pub fn is_playing(&self, id: SoundId) -> bool {
        let state = self.callback_state.lock().unwrap();
        let index = id as usize;
        index < state.active_sounds.len() && state.active_sounds[index].is_some()
    }

    // ============================================================================
    // Spatial Audio Control Methods
    // ============================================================================

    /// Set the 3D position of a playing sound
    ///
    /// Updates the spatial position of a sound in real-time. The sound will be
    /// automatically panned and attenuated based on its position relative to the listener.
    ///
    /// # Arguments
    /// * `id` - The sound ID returned from `play_mixer_realtime()`
    /// * `x` - X coordinate (left/right: negative = left, positive = right)
    /// * `y` - Y coordinate (up/down: negative = below, positive = above)
    /// * `z` - Z coordinate (forward/back: negative = behind, positive = in front)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("guitar").note(&[440.0], 2.0);
    ///
    /// let sound_id = engine.play_mixer_realtime(&comp.into_mixer())?;
    ///
    /// // Move sound to the right over time
    /// for i in 0..10 {
    ///     engine.set_sound_position(sound_id, i as f32, 0.0, 5.0)?;
    ///     std::thread::sleep(std::time::Duration::from_millis(100));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_sound_position(&self, id: SoundId, x: f32, y: f32, z: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetSoundPosition {
                id,
                position: SpatialPosition::new(x, y, z),
            })
            .map_err(|_| TunesError::AudioEngineError("Failed to send command".to_string()))
    }

    /// Set the listener's 3D position
    ///
    /// The listener represents the "ears" or camera position in your 3D world.
    /// All spatial audio is calculated relative to the listener's position and orientation.
    ///
    /// # Arguments
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    /// * `z` - Z coordinate
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    ///
    /// // Set listener at standing height
    /// engine.set_listener_position(0.0, 1.7, 0.0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_listener_position(&self, x: f32, y: f32, z: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetListenerPosition { x, y, z })
            .map_err(|_| TunesError::AudioEngineError("Failed to send command".to_string()))
    }

    /// Set the listener's forward direction
    ///
    /// Controls which direction the listener is facing. This affects how sounds
    /// are panned (sounds in front are centered, sounds to the right are panned right, etc.).
    ///
    /// The vector will be automatically normalized.
    ///
    /// # Arguments
    /// * `x` - X component of forward direction
    /// * `y` - Y component of forward direction
    /// * `z` - Z component of forward direction
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    ///
    /// // Face forward (+Z direction)
    /// engine.set_listener_forward(0.0, 0.0, 1.0)?;
    ///
    /// // Face right (+X direction)
    /// engine.set_listener_forward(1.0, 0.0, 0.0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_listener_forward(&self, x: f32, y: f32, z: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetListenerForward { x, y, z })
            .map_err(|_| TunesError::AudioEngineError("Failed to send command".to_string()))
    }

    /// Set the velocity of a sound source for Doppler effect
    ///
    /// The velocity determines the Doppler shift for moving sound sources.
    /// Sounds moving toward the listener will have higher pitch, sounds moving
    /// away will have lower pitch.
    ///
    /// Velocity is in units per second (typically meters/second).
    ///
    /// # Arguments
    /// * `id` - The sound to modify
    /// * `vx` - X velocity component (units per second)
    /// * `vy` - Y velocity component (units per second)
    /// * `vz` - Z velocity component (units per second)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("car").note(&[110.0], 5.0);
    /// let car_id = engine.play_mixer_realtime(&comp.into_mixer())?;
    ///
    /// // Set position and velocity for a car passing by
    /// engine.set_sound_position(car_id, -20.0, 0.0, 5.0)?;
    /// engine.set_sound_velocity(car_id, 30.0, 0.0, 0.0)?; // 30 m/s to the right
    ///
    /// // You'll hear the pitch shift as it approaches and passes
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_sound_velocity(&self, id: SoundId, vx: f32, vy: f32, vz: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetSoundVelocity { id, vx, vy, vz })
            .map_err(|_| TunesError::AudioEngineError("Failed to send command".to_string()))
    }

    /// Set the listener's velocity for Doppler effect
    ///
    /// The listener velocity affects Doppler calculations for all sounds.
    /// Useful when the player/camera is moving through the world.
    ///
    /// Velocity is in units per second (typically meters/second).
    ///
    /// # Arguments
    /// * `vx` - X velocity component (units per second)
    /// * `vy` - Y velocity component (units per second)
    /// * `vz` - Z velocity component (units per second)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    ///
    /// // Player is moving forward at 5 m/s
    /// engine.set_listener_velocity(0.0, 0.0, 5.0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_listener_velocity(&self, vx: f32, vy: f32, vz: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetListenerVelocity { vx, vy, vz })
            .map_err(|_| TunesError::AudioEngineError("Failed to send command".to_string()))
    }

    /// Configure spatial audio parameters
    ///
    /// Controls how spatial audio behaves, including distance attenuation model,
    /// maximum audible distance, Doppler effect, etc.
    ///
    /// # Arguments
    /// * `params` - Spatial audio parameters
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    ///
    /// let mut params = SpatialParams::default();
    /// params.max_distance = 50.0;  // Sounds silent beyond 50 units
    /// params.attenuation_model = AttenuationModel::Linear;
    ///
    /// engine.set_spatial_params(params)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_spatial_params(&self, params: SpatialParams) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetSpatialParams { params })
            .map_err(|_| TunesError::AudioEngineError("Failed to send command".to_string()))
    }

    /// Set directional cone for a sound source
    ///
    /// Makes a sound source directional, so it's louder when the listener is
    /// in front of the source and quieter when behind or to the sides.
    ///
    /// # Arguments
    /// * `id` - Sound ID
    /// * `cone` - Optional sound cone (None for omnidirectional)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = AudioEngine::new()?;
    /// # let sound_id = 1;
    /// use tunes::synthesis::spatial::{SoundCone, Vec3};
    ///
    /// // Create a narrow directional cone (like a megaphone)
    /// let cone = SoundCone::narrow().with_direction(0.0, 0.0, 1.0);
    /// engine.set_sound_cone(sound_id, Some(cone))?;
    ///
    /// // Remove directionality (make omnidirectional)
    /// engine.set_sound_cone(sound_id, None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_sound_cone(&self, id: SoundId, cone: Option<SoundCone>) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetSoundCone { id, cone })
            .map_err(|_| TunesError::AudioEngineError("Failed to send command".to_string()))
    }

    /// Set occlusion amount for a sound
    ///
    /// Occlusion represents how much a sound is blocked by geometry/obstacles.
    /// The game should use raycasting or other detection to determine occlusion
    /// and then set this value.
    ///
    /// # Arguments
    /// * `id` - Sound ID
    /// * `occlusion` - Occlusion amount (0.0 = no occlusion, 1.0 = fully occluded)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = AudioEngine::new()?;
    /// # let sound_id = 1;
    /// // No occlusion (sound has clear path to listener)
    /// engine.set_sound_occlusion(sound_id, 0.0)?;
    ///
    /// // Partial occlusion (sound is partially blocked)
    /// engine.set_sound_occlusion(sound_id, 0.6)?;
    ///
    /// // Full occlusion (sound is completely blocked)
    /// engine.set_sound_occlusion(sound_id, 1.0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_sound_occlusion(&self, id: SoundId, occlusion: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetSoundOcclusion { id, occlusion })
            .map_err(|_| TunesError::AudioEngineError("Failed to send command".to_string()))
    }

    // ============================================================================
    // Streaming Audio Methods
    // ============================================================================

    #[cfg(not(target_arch = "wasm32"))]
    /// Stream an audio file from disk without loading it entirely into memory
    ///
    /// Ideal for long background music, ambient sounds, or any audio where memory
    /// usage is a concern. The file is decoded on-the-fly in a background thread
    /// and streamed through a lock-free ring buffer.
    ///
    /// Supports MP3, OGG, FLAC, WAV, and AAC formats via symphonia.
    ///
    /// # Arguments
    /// * `path` - Path to the audio file to stream
    ///
    /// # Returns
    /// `SoundId` - Unique identifier for controlling this stream
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    ///
    /// // Stream a long background music file
    /// let music_id = engine.stream_file("assets/background_music.mp3")?;
    ///
    /// // Control the stream
    /// engine.set_stream_volume(music_id, 0.5)?;
    /// engine.pause_stream(music_id)?;
    /// engine.resume_stream(music_id)?;
    /// engine.stop_stream(music_id)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn stream_file<P: Into<PathBuf>>(&self, path: P) -> Result<SoundId> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.command_tx
            .send(AudioCommand::StreamFile {
                id,
                path: path.into(),
                looping: false,
                volume: 1.0,
                pan: 0.0,
            })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(id)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Stream an audio file in a loop
    ///
    /// Like `stream_file()`, but automatically restarts the file from the beginning
    /// when it finishes. Perfect for looping background music.
    ///
    /// # Arguments
    /// * `path` - Path to the audio file to stream
    ///
    /// # Returns
    /// `SoundId` - Unique identifier for controlling this stream
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    ///
    /// // Loop background music forever
    /// let music_id = engine.stream_file_looping("assets/music_loop.mp3")?;
    ///
    /// // Stop when done
    /// engine.stop_stream(music_id)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn stream_file_looping<P: Into<PathBuf>>(&self, path: P) -> Result<SoundId> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.command_tx
            .send(AudioCommand::StreamFile {
                id,
                path: path.into(),
                looping: true,
                volume: 1.0,
                pan: 0.0,
            })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(id)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Stop a streaming audio file
    ///
    /// Stops the decoder thread and removes the stream. The sound will stop immediately.
    ///
    /// # Arguments
    /// * `id` - The stream ID returned by `stream_file()` or `stream_file_looping()`
    pub fn stop_stream(&self, id: SoundId) -> Result<()> {
        self.command_tx
            .send(AudioCommand::StopStream { id })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Pause a streaming audio file
    ///
    /// Pauses playback without stopping the decoder thread. Use `resume_stream()` to continue.
    ///
    /// # Arguments
    /// * `id` - The stream ID to pause
    pub fn pause_stream(&self, id: SoundId) -> Result<()> {
        self.command_tx
            .send(AudioCommand::PauseStream { id })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Resume a paused streaming audio file
    ///
    /// Resumes playback of a stream that was paused with `pause_stream()`.
    ///
    /// # Arguments
    /// * `id` - The stream ID to resume
    pub fn resume_stream(&self, id: SoundId) -> Result<()> {
        self.command_tx
            .send(AudioCommand::ResumeStream { id })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Set the volume of a streaming audio file
    ///
    /// # Arguments
    /// * `id` - The stream ID to modify
    /// * `volume` - Volume level (0.0 = silence, 1.0 = full volume)
    pub fn set_stream_volume(&self, id: SoundId, volume: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetStreamVolume { id, volume })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Set the stereo pan of a streaming audio file
    ///
    /// # Arguments
    /// * `id` - The stream ID to modify
    /// * `pan` - Pan position (-1.0 = full left, 0.0 = center, 1.0 = full right)
    pub fn set_stream_pan(&self, id: SoundId, pan: f32) -> Result<()> {
        self.command_tx
            .send(AudioCommand::SetStreamPan { id, pan })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(())
    }

    // ============================================================================
    // Export Methods
    // ============================================================================

    /// Export mixer to WAV file using the engine's sample rate
    ///
    /// This is a convenience method that automatically uses the AudioEngine's sample rate,
    /// ensuring the exported audio matches what you hear during playback.
    ///
    /// # Arguments
    /// * `mixer` - The mixer to export
    /// * `path` - Output file path (e.g., "output.wav")
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("piano").note(&[440.0], 1.0);
    ///
    /// let mut mixer = comp.into_mixer();
    /// engine.export_wav(&mut mixer, "output.wav")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    /// If you need a specific sample rate (e.g., for upsampling/downsampling),
    /// use `mixer.export_wav(path, sample_rate)` directly.
    pub fn export_wav(&self, mixer: &mut Mixer, path: &str) -> anyhow::Result<()> {
        // Automatically enable GPU if engine was created with GPU support
        #[cfg(feature = "gpu")]
        if self.enable_gpu_for_samples {
            mixer.enable_gpu();
        }
        mixer.export_wav(path, self.sample_rate as u32)
    }

    /// Export mixer to FLAC file using the engine's sample rate
    ///
    /// This is a convenience method that automatically uses the AudioEngine's sample rate.
    /// FLAC provides lossless compression (typically 50-60% of WAV size).
    ///
    /// # Arguments
    /// * `mixer` - The mixer to export
    /// * `path` - Output file path (e.g., "output.flac")
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("piano").note(&[440.0], 1.0);
    ///
    /// let mut mixer = comp.into_mixer();
    /// engine.export_flac(&mut mixer, "output.flac")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    /// If you need a specific sample rate, use `mixer.export_flac(path, sample_rate)` directly.
    pub fn export_flac(&self, mixer: &mut Mixer, path: &str) -> anyhow::Result<()> {
        // Automatically enable GPU if engine was created with GPU support
        #[cfg(feature = "gpu")]
        if self.enable_gpu_for_samples {
            mixer.enable_gpu();
        }
        mixer.export_flac(path, self.sample_rate as u32)
    }

    /// Render mixer to an in-memory buffer using the engine's sample rate
    ///
    /// Useful for pre-rendering sounds for later playback or further processing.
    ///
    /// # Returns
    /// Stereo interleaved samples as `Vec<f32>` (left, right, left, right, ...)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("sfx").note(&[440.0], 0.1);
    ///
    /// let mut mixer = comp.into_mixer();
    /// let buffer = engine.render_to_buffer(&mut mixer);
    /// println!("Rendered {} samples", buffer.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn render_to_buffer(&self, mixer: &mut Mixer) -> Vec<f32> {
        mixer.render_to_buffer(self.sample_rate)
    }

    // ============================================================================
    // Private Helper Methods
    // ============================================================================

    /// Block until a sound finishes playing
    ///
    /// Used internally by `play_mixer()` to provide blocking behavior.
    ///
    /// # Arguments
    /// * `id` - The sound ID to wait for
    /// * `is_empty` - Whether the mixer is known to be empty (improves error messages)
    fn wait_for(&self, id: SoundId, is_empty: bool) -> Result<()> {
        // Wait for sound to start playing (avoid race condition)
        // The audio thread needs time to process the Play command
        let mut started = false;
        for _ in 0..100 {
            // Try for up to 1 second
            if self.is_playing(id) {
                started = true;
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        if !started {
            // Sound never started - could be:
            // 1. Empty mixer (no events) - expected, not an error
            // 2. Very short sound (< 10ms, finished before we checked) - expected
            // 3. Audio thread not processing commands - critical failure

            if is_empty {
                // Empty mixer - this is expected, no warning needed
                return Ok(());
            } else {
                // Non-empty mixer didn't play - unexpected
                eprintln!(
                    "Warning: Sound {} never started or finished very quickly (< 10ms)",
                    id
                );
                return Ok(());
            }
        }

        // Now wait for it to finish
        while self.is_playing(id) {
            thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    }
}

// Note: Full integration tests requiring audio devices should be placed in
// tests/integration_tests.rs with #[ignore] attribute for CI environments
// without audio hardware.
