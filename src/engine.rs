use crate::error::{Result, TunesError};
use crate::synthesis::spatial::{
    ListenerConfig, SpatialParams, SpatialPosition, calculate_spatial,
};
use crate::track::Mixer;
use crate::composition::{Composition, Tempo};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam::channel::{Receiver, Sender, unbounded};
use ringbuf::{HeapRb, traits::{Split, Consumer, Producer, Observer}};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::conv::IntoSample;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::sample::Sample as SymphoniaSample;

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
    PauseAll,
    ResumeAll,
    StopAll,
    FadeOut {
        id: SoundId,
        duration: f32, // Duration in seconds
    },
    FadeIn {
        id: SoundId,
        duration: f32,      // Duration in seconds
        target_volume: f32, // Target volume (0.0-1.0)
    },
    TweenPan {
        id: SoundId,
        target_pan: f32,  // Target pan (-1.0 to 1.0)
        duration: f32,    // Duration in seconds
    },
    TweenPlaybackRate {
        id: SoundId,
        target_rate: f32, // Target playback rate
        duration: f32,    // Duration in seconds
    },
    SetSoundPosition {
        id: SoundId,
        position: SpatialPosition,
    },
    SetSoundVelocity {
        id: SoundId,
        vx: f32,
        vy: f32,
        vz: f32,
    },
    SetListenerPosition {
        x: f32,
        y: f32,
        z: f32,
    },
    SetListenerVelocity {
        vx: f32,
        vy: f32,
        vz: f32,
    },
    SetListenerForward {
        x: f32,
        y: f32,
        z: f32,
    },
    SetSpatialParams {
        params: SpatialParams,
    },
    // Streaming commands
    StreamFile {
        id: SoundId,
        path: PathBuf,
        looping: bool,
        volume: f32,
        pan: f32,
    },
    StopStream {
        id: SoundId,
    },
    PauseStream {
        id: SoundId,
    },
    ResumeStream {
        id: SoundId,
    },
    SetStreamVolume {
        id: SoundId,
        volume: f32,
    },
    SetStreamPan {
        id: SoundId,
        pan: f32,
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
    // Volume fade state
    fade_start_time: Option<f32>,
    fade_duration: f32,
    fade_start_volume: f32,
    fade_target_volume: f32,
    // Pan tween state
    pan_tween_start_time: Option<f32>,
    pan_tween_duration: f32,
    pan_tween_start_value: f32,
    pan_tween_target_value: f32,
    // Playback rate tween state
    rate_tween_start_time: Option<f32>,
    rate_tween_duration: f32,
    rate_tween_start_value: f32,
    rate_tween_target_value: f32,
}

/// State for a streaming audio source
///
/// Streams audio from disk using a background decoder thread and lock-free ring buffer.
/// This allows playing long audio files (background music, ambience) without loading
/// the entire file into memory.
struct StreamingSound {
    /// Ring buffer consumer (audio thread reads from this)
    ring_consumer: ringbuf::HeapCons<f32>,
    /// Decoder thread handle (for cleanup on stop)
    decoder_thread: Option<JoinHandle<()>>,
    /// Signal to stop the decoder thread
    stop_signal: Arc<AtomicBool>,
    /// Pause signal for decoder thread
    pause_signal: Arc<AtomicBool>,
    /// Current volume
    volume: f32,
    /// Current pan (-1.0 left, 0.0 center, 1.0 right)
    pan: f32,
    /// Whether the stream is looping
    looping: bool,
}

impl Drop for StreamingSound {
    fn drop(&mut self) {
        // Signal thread to stop and wait for it to finish
        self.stop_signal.store(true, Ordering::Relaxed);
        if let Some(handle) = self.decoder_thread.take() {
            let _ = handle.join();
        }
    }
}

/// Audio callback state (allocation-free mixing)
///
/// Holds pre-allocated buffers to avoid allocations in the real-time audio thread.
/// All buffers are reused across callback invocations.
struct AudioCallbackState {
    /// Active sounds being mixed
    active_sounds: HashMap<SoundId, ActiveSound>,
    /// Streaming sounds (separate from pre-rendered sounds)
    streaming_sounds: HashMap<SoundId, StreamingSound>,
    /// Pre-allocated temp buffer for mixing (stereo interleaved)
    /// Size is determined by the maximum buffer size we expect
    temp_buffer: Vec<f32>,
    /// Pre-allocated list for tracking finished sounds (avoids allocation during cleanup)
    finished_sounds: Vec<SoundId>,
    /// Pre-allocated list for tracking finished streams
    finished_streams: Vec<SoundId>,
}

impl AudioCallbackState {
    fn new() -> Self {
        Self {
            active_sounds: HashMap::new(),
            streaming_sounds: HashMap::new(),
            // Pre-allocate for a reasonably large buffer (2048 frames stereo = 4096 samples)
            temp_buffer: vec![0.0; 4096],
            finished_sounds: Vec::with_capacity(16),
            finished_streams: Vec::with_capacity(16),
        }
    }

    /// Ensure temp buffer is large enough for the given size
    #[allow(dead_code)]
    fn ensure_temp_buffer_size(&mut self, required_size: usize) {
        if self.temp_buffer.len() < required_size {
            self.temp_buffer.resize(required_size, 0.0);
        }
    }
}

/// Decoder thread function for streaming audio
///
/// Runs in a background thread, decodes audio from file, and pushes samples to ring buffer.
/// The audio callback reads from the ring buffer, creating a lock-free streaming pipeline.
fn decoder_thread_func(
    path: PathBuf,
    mut ring_producer: ringbuf::HeapProd<f32>,
    stop_signal: Arc<AtomicBool>,
    pause_signal: Arc<AtomicBool>,
    looping: bool,
) {
    // Helper function to convert symphonia audio buffer to f32 samples
    fn convert_audio_buffer(decoded: &AudioBufferRef, samples: &mut Vec<f32>) {
        fn convert_samples<S>(buf: &symphonia::core::audio::AudioBuffer<S>, samples: &mut Vec<f32>)
        where
            S: SymphoniaSample + IntoSample<f32>,
        {
            let num_channels = buf.spec().channels.count();
            let num_frames = buf.frames();
            samples.clear();
            samples.reserve(num_frames * num_channels);

            // Convert planar to interleaved
            for frame_idx in 0..num_frames {
                for ch in 0..num_channels {
                    let sample: f32 = buf.chan(ch)[frame_idx].into_sample();
                    samples.push(sample);
                }
            }
        }

        match decoded {
            AudioBufferRef::U8(buf) => convert_samples(buf, samples),
            AudioBufferRef::U16(buf) => convert_samples(buf, samples),
            AudioBufferRef::U24(buf) => convert_samples(buf, samples),
            AudioBufferRef::U32(buf) => convert_samples(buf, samples),
            AudioBufferRef::S8(buf) => convert_samples(buf, samples),
            AudioBufferRef::S16(buf) => convert_samples(buf, samples),
            AudioBufferRef::S24(buf) => convert_samples(buf, samples),
            AudioBufferRef::S32(buf) => convert_samples(buf, samples),
            AudioBufferRef::F32(buf) => convert_samples(buf, samples),
            AudioBufferRef::F64(buf) => convert_samples(buf, samples),
        }
    }

    loop {
        // Check stop signal
        if stop_signal.load(Ordering::Relaxed) {
            break;
        }

        // If paused, sleep briefly and continue
        if pause_signal.load(Ordering::Relaxed) {
            thread::sleep(std::time::Duration::from_millis(10));
            continue;
        }

        // Open and decode file
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Streaming: Failed to open file {:?}: {}", path, e);
                break;
            }
        };

        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();
        let decoder_opts = DecoderOptions::default();

        let probed = match symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Streaming: Failed to probe file {:?}: {}", path, e);
                break;
            }
        };

        let mut format = probed.format;
        let track = match format.default_track() {
            Some(t) => t,
            None => {
                eprintln!("Streaming: No default track found in {:?}", path);
                break;
            }
        };

        let mut decoder = match symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Streaming: Failed to create decoder for {:?}: {}", path, e);
                break;
            }
        };

        let mut samples = Vec::new();

        // Decode loop
        loop {
            // Check stop/pause signals
            if stop_signal.load(Ordering::Relaxed) {
                return; // Exit thread entirely
            }

            if pause_signal.load(Ordering::Relaxed) {
                thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }

            // Get next packet
            let packet = match format.next_packet() {
                Ok(p) => p,
                Err(symphonia::core::errors::Error::IoError(e))
                    if e.kind() == std::io::ErrorKind::UnexpectedEof =>
                {
                    // End of file
                    if looping {
                        break; // Break inner loop, restart outer loop
                    } else {
                        return; // Exit thread
                    }
                }
                Err(e) => {
                    eprintln!("Streaming: Error reading packet: {}", e);
                    return;
                }
            };

            // Decode packet
            let decoded = match decoder.decode(&packet) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Streaming: Decode error: {}", e);
                    continue;
                }
            };

            // Convert to f32 samples
            convert_audio_buffer(&decoded, &mut samples);

            // Push samples to ring buffer (blocking if buffer is full)
            let mut offset = 0;
            while offset < samples.len() {
                // Check stop signal even while pushing
                if stop_signal.load(Ordering::Relaxed) {
                    return;
                }

                // Try to push as much as possible
                let pushed = ring_producer.push_slice(&samples[offset..]);
                offset += pushed;

                // If we couldn't push everything, the buffer is full - sleep briefly
                if pushed == 0 {
                    thread::sleep(std::time::Duration::from_millis(1));
                }
            }
        }

        // If not looping, we exit after one playthrough
        if !looping {
            break;
        }
    }
}

/// Central audio engine that manages playback with concurrent mixing
pub struct AudioEngine {
    command_tx: Sender<AudioCommand>,
    next_id: Arc<AtomicU64>,
    callback_state: Arc<Mutex<AudioCallbackState>>,
    #[allow(dead_code)] // Reserved for future spatial audio runtime control
    listener_config: Arc<Mutex<ListenerConfig>>,
    #[allow(dead_code)] // Reserved for future spatial audio runtime control
    spatial_params: Arc<Mutex<SpatialParams>>,
    sample_rate: f32,
    sample_cache: Arc<Mutex<HashMap<String, crate::synthesis::Sample>>>, // Automatic sample caching
    _stream: cpal::Stream, // Persistent stream, kept alive
    // Info for optional printing
    device_name: String,
    buffer_size: u32,
    channels: usize,
    // GPU acceleration flag for play_sample()
    enable_gpu_for_samples: bool,
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
    /// This enables GPU compute shaders for `play_sample()` and related methods,
    /// providing 500-5000x realtime performance on discrete GPUs.
    ///
    /// **This is the recommended constructor for game audio with discrete GPUs.**
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// let engine = AudioEngine::new_with_gpu()?;
    ///
    /// // GPU acceleration automatic for all samples!
    /// engine.play_sample("explosion.wav")?;  // 500-5000x realtime
    /// engine.play_sample("footstep.wav")?;   // 500-5000x realtime
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    ///
    /// # Performance
    /// - Discrete GPUs (RTX/RX): 500-5000x realtime
    /// - Integrated GPUs: May be slower than CPU (auto-detected with warning)
    /// - CPU fallback: Automatic if GPU unavailable
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
                    // Lock once for entire audio callback
                    let mut state = callback_state_for_stream.lock().unwrap();
                    let mut listener = listener_config_for_stream.lock().unwrap();
                    let mut spatial = spatial_params_for_stream.lock().unwrap();

                    // Destructure state FIRST to get separate mutable references (satisfies borrow checker)
                    let AudioCallbackState {
                        ref mut active_sounds,
                        ref mut streaming_sounds,
                        ref mut temp_buffer,
                        ref mut finished_sounds,
                        ref mut finished_streams,
                    } = *state;

                    // Process all pending commands (non-blocking)
                    while let Ok(cmd) = command_rx.try_recv() {
                        Self::handle_command(
                            cmd,
                            active_sounds,
                            streaming_sounds,
                            &mut listener,
                            &mut spatial,
                            sample_rate,
                        );
                    }

                    // Mix all active sounds into the output buffer (allocation-free)
                    Self::mix_sounds(
                        data,
                        active_sounds,
                        temp_buffer,
                        finished_sounds,
                        &listener,
                        &spatial,
                        sample_rate,
                        channels,
                    );

                    // Mix streaming sounds into the output buffer
                    Self::mix_streaming_sounds(
                        data,
                        streaming_sounds,
                        finished_streams,
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
            callback_state,
            listener_config,
            spatial_params,
            sample_rate,
            sample_cache: Arc::new(Mutex::new(HashMap::new())),
            _stream: stream,
            device_name,
            buffer_size,
            channels,
            enable_gpu_for_samples: enable_gpu,
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
        use crate::synthesis::simd::{SimdWidth, SIMD};

        let latency_ms = (self.buffer_size as f32 / self.sample_rate) * 1000.0;
        let simd_width = SIMD.simd_width();
        let simd_lanes = SIMD.width();
        let simd_name = match simd_width {
            SimdWidth::X8 => "AVX2",
            SimdWidth::X4 => {
                #[cfg(target_arch = "x86_64")]
                { "SSE" }
                #[cfg(not(target_arch = "x86_64"))]
                { "NEON" }
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

    /// Handle commands from the main thread (called from audio thread)
    fn handle_command(
        cmd: AudioCommand,
        active_sounds: &mut HashMap<SoundId, ActiveSound>,
        streaming_sounds: &mut HashMap<SoundId, StreamingSound>,
        listener: &mut ListenerConfig,
        spatial: &mut SpatialParams,
        sample_rate: f32,
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
                        fade_start_time: None,
                        fade_duration: 0.0,
                        fade_start_volume: 1.0,
                        fade_target_volume: 1.0,
                        pan_tween_start_time: None,
                        pan_tween_duration: 0.0,
                        pan_tween_start_value: 0.0,
                        pan_tween_target_value: 0.0,
                        rate_tween_start_time: None,
                        rate_tween_duration: 0.0,
                        rate_tween_start_value: 1.0,
                        rate_tween_target_value: 1.0,
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
            AudioCommand::SetSoundVelocity { id, vx, vy, vz } => {
                if let Some(sound) = active_sounds.get_mut(&id) {
                    if let Some(pos) = &mut sound.spatial_position {
                        pos.set_velocity(vx, vy, vz);
                    }
                }
            }
            AudioCommand::SetListenerPosition { x, y, z } => {
                listener.position.x = x;
                listener.position.y = y;
                listener.position.z = z;
            }
            AudioCommand::SetListenerVelocity { vx, vy, vz } => {
                listener.velocity.x = vx;
                listener.velocity.y = vy;
                listener.velocity.z = vz;
            }
            AudioCommand::SetListenerForward { x, y, z } => {
                use crate::synthesis::spatial::Vec3;
                listener.forward = Vec3::new(x, y, z).normalize();
            }
            AudioCommand::SetSpatialParams { params } => {
                *spatial = params;
            }
            AudioCommand::PauseAll => {
                for sound in active_sounds.values_mut() {
                    sound.paused = true;
                }
            }
            AudioCommand::ResumeAll => {
                for sound in active_sounds.values_mut() {
                    sound.paused = false;
                }
            }
            AudioCommand::StopAll => {
                active_sounds.clear();
            }
            AudioCommand::FadeOut { id, duration } => {
                if let Some(sound) = active_sounds.get_mut(&id) {
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
                if let Some(sound) = active_sounds.get_mut(&id) {
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
                if let Some(sound) = active_sounds.get_mut(&id) {
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
                if let Some(sound) = active_sounds.get_mut(&id) {
                    sound.rate_tween_start_time = Some(sound.elapsed_time);
                    sound.rate_tween_duration = duration;
                    sound.rate_tween_start_value = sound.playback_rate;
                    sound.rate_tween_target_value = target_rate.max(0.1); // Prevent division by zero
                }
            }
            // Streaming commands
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
                    decoder_thread_func(path, ring_producer, stop_signal_clone, pause_signal_clone, looping);
                });

                // Add to streaming sounds
                streaming_sounds.insert(
                    id,
                    StreamingSound {
                        ring_consumer,
                        decoder_thread: Some(decoder_thread),
                        stop_signal,
                        pause_signal,
                        volume,
                        pan,
                        looping,
                    },
                );
            }
            AudioCommand::StopStream { id } => {
                // Removing from HashMap will trigger Drop, which signals thread to stop
                streaming_sounds.remove(&id);
            }
            AudioCommand::PauseStream { id } => {
                if let Some(stream) = streaming_sounds.get_mut(&id) {
                    stream.pause_signal.store(true, Ordering::Relaxed);
                }
            }
            AudioCommand::ResumeStream { id } => {
                if let Some(stream) = streaming_sounds.get_mut(&id) {
                    stream.pause_signal.store(false, Ordering::Relaxed);
                }
            }
            AudioCommand::SetStreamVolume { id, volume } => {
                if let Some(stream) = streaming_sounds.get_mut(&id) {
                    stream.volume = volume.clamp(0.0, 1.0);
                }
            }
            AudioCommand::SetStreamPan { id, pan } => {
                if let Some(stream) = streaming_sounds.get_mut(&id) {
                    stream.pan = pan.clamp(-1.0, 1.0);
                }
            }
        }
    }

    /// Mix all active sounds into the output buffer (called from audio thread)
    ///
    /// This function is ALLOCATION-FREE - all buffers are pre-allocated and reused.
    fn mix_sounds(
        output: &mut [f32],
        active_sounds: &mut HashMap<SoundId, ActiveSound>,
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

        // Mix each active sound using block processing
        for (id, sound) in active_sounds.iter_mut() {
            if sound.paused {
                continue;
            }

            let duration = sound.mixer.total_duration();

            // Check if sound will finish during this block
            let time_delta = 1.0 / sample_rate;
            let block_duration = num_frames as f32 * time_delta * sound.playback_rate;

            if sound.elapsed_time >= duration {
                if sound.looping {
                    sound.elapsed_time = 0.0;
                    sound.sample_clock = 0.0;
                } else {
                    finished_sounds.push(*id);
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
            temp_buffer.fill(0.0);
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
            let (spatial_volume, spatial_pan, spatial_pitch) = if let Some(pos) = &sound.spatial_position {
                let result = calculate_spatial(pos, listener, spatial_params);
                (result.volume, result.pan, result.pitch)
            } else {
                (1.0, sound.pan, 1.0)
            };

            // Apply doppler pitch shift to playback rate
            let effective_playback_rate = sound.playback_rate * spatial_pitch;

            // Mix temp buffer into output with volume/pan/fade applied per-sample
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

            // Advance time with doppler-adjusted playback rate
            // This ensures mixer renders samples at the correct pitch
            sound.elapsed_time += block_duration * effective_playback_rate;
            sound.sample_clock =
                (sound.sample_clock + (num_frames as f32 * effective_playback_rate)) % sample_rate;
        }

        // Remove finished sounds
        for id in finished_sounds {
            active_sounds.remove(id);
        }

        // Clamp output to prevent distortion
        for sample in output.iter_mut() {
            *sample = sample.clamp(-1.0, 1.0);
        }
    }

    /// Mix streaming sounds into the output buffer (called from audio thread)
    ///
    /// Reads decoded samples from ring buffers and mixes them into the output.
    /// This is ALLOCATION-FREE and lock-free (uses lockless ring buffer).
    fn mix_streaming_sounds(
        output: &mut [f32],
        streaming_sounds: &mut HashMap<SoundId, StreamingSound>,
        finished_streams: &mut Vec<SoundId>,
        channels: usize,
    ) {
        // Clear finished streams list
        finished_streams.clear();

        // Mix each streaming sound
        for (id, stream) in streaming_sounds.iter_mut() {
            // Check if the decoder thread has finished
            if let Some(handle) = &stream.decoder_thread {
                if handle.is_finished() {
                    // Thread finished - mark for removal
                    finished_streams.push(*id);
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
                let left_gain = if pan <= 0.0 {
                    1.0
                } else {
                    1.0 - pan
                } * stream.volume;
                let right_gain = if pan >= 0.0 {
                    1.0
                } else {
                    1.0 + pan
                } * stream.volume;

                // Mix into output (additively)
                if i < output.len() {
                    output[i] += left * left_gain;
                }
                if i + 1 < output.len() {
                    output[i + 1] += right * right_gain;
                }
            }
        }

        // Remove finished streams
        for id in finished_streams.iter() {
            streaming_sounds.remove(id);
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
        self.command_tx
            .send(AudioCommand::Play {
                id,
                mixer: mixer.clone(),
                looping: true,
            })
            .map_err(|_| TunesError::AudioEngineError("Audio engine stopped".to_string()))?;
        Ok(id)
    }

    /// Play a one-shot sample immediately (convenience method with automatic caching)
    ///
    /// Simplified non-blocking interface for playing sound effects without manual Composition setup.
    /// Perfect for game sound effects - fire and forget!
    ///
    /// **Automatic caching:** Samples are automatically cached by path on first load. Subsequent
    /// calls with the same path reuse the cached sample (cheap Arc clone), making repeated sounds
    /// efficient without any extra code.
    ///
    /// # Arguments
    /// * `path` - Path to the sample file (WAV, OGG, MP3, FLAC supported)
    ///
    /// # Returns
    /// `SoundId` - Unique identifier if you need to control the sound (optional - can be ignored)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = AudioEngine::new()?;
    ///
    /// // First call: loads from disk, caches it
    /// engine.play_sample("assets/footstep.wav")?;
    ///
    /// // Subsequent calls: instant! Uses cached sample
    /// engine.play_sample("assets/footstep.wav")?;
    /// engine.play_sample("assets/footstep.wav")?;
    ///
    /// // Different sound: loads and caches separately
    /// engine.play_sample("assets/explosion.wav")?;
    ///
    /// // Optional: Keep the ID if you need to control it
    /// let sfx_id = engine.play_sample("assets/ambience.wav")?;
    /// engine.set_volume(sfx_id, 0.5)?;
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
    /// This method is **non-blocking** and returns immediately. The sound plays concurrently
    /// in the background. Multiple sounds can play at the same time.
    ///
    /// For cache management, see `preload_sample()`, `clear_sample_cache()`, and `remove_cached_sample()`.
    ///
    /// For more control over synthesis, effects, or timing, use the full Composition API.
    pub fn play_sample(&self, path: &str) -> Result<SoundId> {
        use crate::synthesis::Sample;

        // Check cache first, load if not present
        let sample = {
            let mut cache = self.sample_cache.lock().unwrap();

            if let Some(cached) = cache.get(path) {
                // Cache hit - cheap Arc clone!
                cached.clone()
            } else {
                // Cache miss - load and cache
                let loaded = Sample::from_file(path)
                    .map_err(|e| TunesError::AudioEngineError(format!("Failed to load sample '{}': {}", path, e)))?;
                cache.insert(path.to_string(), loaded.clone());
                loaded
            }
        };

        // Create a minimal composition and play it
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("_oneshot").play_sample(&sample, 1.0);

        // Enable GPU if requested via new_with_gpu()
        let mut mixer = if self.enable_gpu_for_samples {
            comp.into_mixer_with_gpu()
        } else {
            comp.into_mixer()
        };

        self.play_mixer_realtime(&mixer)
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
    /// engine.play_sample("assets/footstep.wav")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn preload_sample(&self, path: &str) -> Result<()> {
        use crate::synthesis::Sample;

        let mut cache = self.sample_cache.lock().unwrap();

        if !cache.contains_key(path) {
            let sample = Sample::from_file(path)
                .map_err(|e| TunesError::AudioEngineError(format!("Failed to preload sample '{}': {}", path, e)))?;
            cache.insert(path.to_string(), sample);
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
    /// engine.play_sample("level1_music.wav")?;
    ///
    /// // Level complete - free the memory
    /// engine.remove_cached_sample("level1_music.wav")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove_cached_sample(&self, path: &str) -> Result<()> {
        let mut cache = self.sample_cache.lock().unwrap();
        cache.remove(path);
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
    /// engine.play_sample("sound1.wav")?;
    /// engine.play_sample("sound2.wav")?;
    ///
    /// // Level complete - clear all cached samples
    /// engine.clear_sample_cache()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn clear_sample_cache(&self) -> Result<()> {
        let mut cache = self.sample_cache.lock().unwrap();
        cache.clear();
        Ok(())
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
        self.callback_state
            .lock()
            .unwrap()
            .active_sounds
            .contains_key(&id)
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

    // ============================================================================
    // End Spatial Audio Control Methods
    // ============================================================================

    // ============================================================================
    // Streaming Audio Methods
    // ============================================================================

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
    // End Streaming Audio Methods
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
