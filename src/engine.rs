use crate::error::{Result, TunesError};
use crate::synthesis::spatial::{
    ListenerConfig, SpatialParams, SpatialPosition, calculate_spatial,
};
use crate::track::Mixer;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam::channel::{Receiver, Sender, unbounded};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// Unique identifier for playing sounds
pub type SoundId = u64;

/// Commands sent from main thread to audio thread
enum AudioCommand {
    Play {
        id: SoundId,
        mixer: Mixer,
        looping: bool,
    },
    Stop {
        id: SoundId,
    },
    SetVolume {
        id: SoundId,
        volume: f32,
    },
    SetPan {
        id: SoundId,
        pan: f32, // -1.0 (left) to 1.0 (right)
    },
    SetPlaybackRate {
        id: SoundId,
        rate: f32, // 1.0 = normal, 2.0 = double speed/pitch
    },
    Pause {
        id: SoundId,
    },
    Resume {
        id: SoundId,
    },
    SetSoundPosition {
        id: SoundId,
        position: SpatialPosition,
    },
    SetListenerPosition {
        x: f32,
        y: f32,
        z: f32,
    },
    SetListenerForward {
        x: f32,
        y: f32,
        z: f32,
    },
    SetSpatialParams {
        params: SpatialParams,
    },
}

/// State for an actively playing sound
struct ActiveSound {
    mixer: Mixer,
    sample_clock: f32,
    elapsed_time: f32,
    volume: f32,
    pan: f32,
    playback_rate: f32, // 1.0 = normal, 2.0 = double speed/pitch
    paused: bool,
    looping: bool,
    spatial_position: Option<SpatialPosition>, // 3D position for spatial audio
}

/// Central audio engine that manages playback with concurrent mixing
pub struct AudioEngine {
    command_tx: Sender<AudioCommand>,
    next_id: Arc<AtomicU64>,
    active_sounds: Arc<Mutex<HashMap<SoundId, ActiveSound>>>,
    listener_config: Arc<Mutex<ListenerConfig>>,
    spatial_params: Arc<Mutex<SpatialParams>>,
    sample_rate: f32,
    _stream: cpal::Stream, // Persistent stream, kept alive
}

impl AudioEngine {
    /// Create a new audio engine with default output device
    ///
    /// Uses a moderate buffer size (4096 samples) optimized for pre-rendered playback.
    /// Since play_mixer() pre-renders audio, buffer size only affects latency, not stability.
    /// For lower latency, use `with_buffer_size()`.
    pub fn new() -> Result<Self> {
        Self::with_buffer_size(4096) // ~93ms at 44.1kHz - good balance for pre-rendered playback
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
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or_else(|| {
            TunesError::AudioEngineError("No output device available".to_string())
        })?;
        let config = device.default_output_config().map_err(|e| {
            TunesError::AudioEngineError(format!("Failed to get default config: {}", e))
        })?;

        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;
        let latency_ms = (buffer_size as f32 / sample_rate) * 1000.0;

        println!("Audio Engine initialized:");
        println!(
            "  Device: {}",
            device.name().unwrap_or_else(|_| "Unknown".to_string())
        );
        println!("  Sample rate: {}", config.sample_rate().0);
        println!(
            "  Buffer size: {} samples ({:.1}ms latency)",
            buffer_size, latency_ms
        );
        println!("  Concurrent mixing: enabled");

        // Create command channel for communication with audio thread
        let (command_tx, command_rx): (Sender<AudioCommand>, Receiver<AudioCommand>) = unbounded();

        // Shared state for active sounds
        let active_sounds: Arc<Mutex<HashMap<SoundId, ActiveSound>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let active_sounds_for_stream = Arc::clone(&active_sounds);

        // Shared state for spatial audio
        let listener_config = Arc::new(Mutex::new(ListenerConfig::new()));
        let listener_config_for_stream = Arc::clone(&listener_config);

        let spatial_params = Arc::new(Mutex::new(SpatialParams::default()));
        let spatial_params_for_stream = Arc::clone(&spatial_params);

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
                    // Lock once for entire audio callback (better granularity)
                    let mut active_sounds = active_sounds_for_stream.lock().unwrap();
                    let mut listener = listener_config_for_stream.lock().unwrap();
                    let mut spatial = spatial_params_for_stream.lock().unwrap();

                    // Process all pending commands (non-blocking)
                    while let Ok(cmd) = command_rx.try_recv() {
                        Self::handle_command(cmd, &mut active_sounds, &mut listener, &mut spatial);
                    }

                    // Mix all active sounds into the output buffer
                    Self::mix_sounds(
                        data,
                        &mut active_sounds,
                        &listener,
                        &spatial,
                        sample_rate,
                        channels,
                    );

                    // Unlock at end of scope
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
            active_sounds,
            listener_config,
            spatial_params,
            sample_rate,
            _stream: stream,
        })
    }

    /// Handle commands from the main thread (called from audio thread)
    fn handle_command(
        cmd: AudioCommand,
        active_sounds: &mut HashMap<SoundId, ActiveSound>,
        listener: &mut ListenerConfig,
        spatial: &mut SpatialParams,
    ) {
        match cmd {
            AudioCommand::Play { id, mixer, looping } => {
                active_sounds.insert(
                    id,
                    ActiveSound {
                        mixer,
                        sample_clock: 0.0,
                        elapsed_time: 0.0,
                        volume: 1.0,
                        pan: 0.0,
                        playback_rate: 1.0,
                        paused: false,
                        looping,
                        spatial_position: None,
                    },
                );
            }
            AudioCommand::Stop { id } => {
                active_sounds.remove(&id);
            }
            AudioCommand::SetVolume { id, volume } => {
                if let Some(sound) = active_sounds.get_mut(&id) {
                    sound.volume = volume.clamp(0.0, 1.0);
                }
            }
            AudioCommand::SetPan { id, pan } => {
                if let Some(sound) = active_sounds.get_mut(&id) {
                    sound.pan = pan.clamp(-1.0, 1.0);
                }
            }
            AudioCommand::SetPlaybackRate { id, rate } => {
                if let Some(sound) = active_sounds.get_mut(&id) {
                    // Clamp to reasonable range (0.1x to 4.0x speed)
                    sound.playback_rate = rate.clamp(0.1, 4.0);
                }
            }
            AudioCommand::Pause { id } => {
                if let Some(sound) = active_sounds.get_mut(&id) {
                    sound.paused = true;
                }
            }
            AudioCommand::Resume { id } => {
                if let Some(sound) = active_sounds.get_mut(&id) {
                    sound.paused = false;
                }
            }
            AudioCommand::SetSoundPosition { id, position } => {
                if let Some(sound) = active_sounds.get_mut(&id) {
                    sound.spatial_position = Some(position);
                }
            }
            AudioCommand::SetListenerPosition { x, y, z } => {
                listener.position.x = x;
                listener.position.y = y;
                listener.position.z = z;
            }
            AudioCommand::SetListenerForward { x, y, z } => {
                use crate::synthesis::spatial::Vec3;
                listener.forward = Vec3::new(x, y, z).normalize();
            }
            AudioCommand::SetSpatialParams { params } => {
                *spatial = params;
            }
        }
    }

    /// Mix all active sounds into the output buffer (called from audio thread)
    fn mix_sounds(
        output: &mut [f32],
        active_sounds: &mut HashMap<SoundId, ActiveSound>,
        listener: &ListenerConfig,
        spatial_params: &SpatialParams,
        sample_rate: f32,
        channels: usize,
    ) {
        // Clear output buffer
        for sample in output.iter_mut() {
            *sample = 0.0;
        }

        let mut finished_sounds = Vec::new();

        // Mix each active sound
        for (id, sound) in active_sounds.iter_mut() {
            if sound.paused {
                continue;
            }

            let duration = sound.mixer.total_duration();

            for frame in output.chunks_mut(channels) {
                // Check if sound has finished
                if sound.elapsed_time >= duration {
                    if sound.looping {
                        sound.elapsed_time = 0.0;
                        sound.sample_clock = 0.0;
                    } else {
                        finished_sounds.push(*id);
                        break;
                    }
                }

                // Only apply composition-time spatial audio if NO runtime position is set
                // Runtime position (set_sound_position) overrides composition-time position
                let (listener_for_mixer, params_for_mixer) = if sound.spatial_position.is_some() {
                    (None, None) // Runtime position will handle spatial audio
                } else {
                    (Some(listener), Some(spatial_params)) // Use composition-time position
                };

                // Get stereo sample from mixer (with spatial audio support for events)
                let (mut left, mut right) = sound.mixer.sample_at(
                    sound.elapsed_time,
                    sample_rate,
                    sound.sample_clock,
                    listener_for_mixer,
                    params_for_mixer,
                );

                // Calculate spatial audio if runtime position is set
                let (spatial_volume, spatial_pan) = if let Some(pos) = &sound.spatial_position {
                    let result = calculate_spatial(pos, listener, spatial_params);
                    (result.volume, result.pan)
                    // Note: Doppler (result.pitch) would require resampling - not implemented yet
                } else {
                    (1.0, sound.pan)
                };

                // Apply volume (sound volume * spatial attenuation)
                left *= sound.volume * spatial_volume;
                right *= sound.volume * spatial_volume;

                // Apply pan (spatial pan overrides manual pan when spatial position is set)
                if spatial_pan < 0.0 {
                    // Pan left: reduce right channel
                    right *= 1.0 + spatial_pan;
                } else if spatial_pan > 0.0 {
                    // Pan right: reduce left channel
                    left *= 1.0 - spatial_pan;
                }

                // Mix into output (additive mixing)
                if channels == 1 {
                    // Mono: average left and right
                    frame[0] += (left + right) * 0.5;
                } else if channels == 2 {
                    // Stereo
                    frame[0] += left;
                    frame[1] += right;
                } else {
                    // Multi-channel: use first two channels for stereo, silence others
                    frame[0] += left;
                    frame[1] += right;
                }

                // Advance time (affected by playback rate)
                sound.elapsed_time += (1.0 / sample_rate) * sound.playback_rate;
                sound.sample_clock = (sound.sample_clock + sound.playback_rate) % sample_rate;
            }
        }

        // Remove finished sounds
        for id in finished_sounds {
            active_sounds.remove(&id);
        }

        // Clamp output to prevent distortion from overlapping sounds
        for sample in output.iter_mut() {
            *sample = sample.clamp(-1.0, 1.0);
        }
    }

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
        self.command_tx
            .send(AudioCommand::Play {
                id,
                mixer: mixer.clone(),
                looping: false,
            })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
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
        self.command_tx
            .send(AudioCommand::Play {
                id,
                mixer: mixer.clone(),
                looping: true,
            })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(id)
    }

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

    /// Check if a sound is still playing
    pub fn is_playing(&self, id: SoundId) -> bool {
        self.active_sounds.lock().unwrap().contains_key(&id)
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

    // ============================================================================
    // End Spatial Audio Control Methods
    // ============================================================================

    /// Block until a sound finishes playing
    ///
    /// Used internally by `play_mixer()` to provide blocking behavior.
    ///
    /// # Arguments
    /// * `id` - The sound ID to wait for
    /// * `is_empty` - Whether the mixer is known to be empty (improves error messages)
    fn wait_for(&self, id: SoundId, is_empty: bool) -> Result<()> {
        use std::thread;
        use std::time::Duration;

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
    pub fn export_wav(&self, mixer: &mut crate::track::Mixer, path: &str) -> anyhow::Result<()> {
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
    pub fn export_flac(&self, mixer: &mut crate::track::Mixer, path: &str) -> anyhow::Result<()> {
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
    pub fn render_to_buffer(&self, mixer: &mut crate::track::Mixer) -> Vec<f32> {
        mixer.render_to_buffer(self.sample_rate)
    }
}

// Note: Full integration tests requiring audio devices should be placed in
// tests/integration_tests.rs with #[ignore] attribute for CI environments
// without audio hardware.
