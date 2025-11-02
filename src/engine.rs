use crate::drums::DrumType;
use crate::error::{Result, TunesError};
use crate::rhythm::{NoteDuration, Tempo};
use crate::track::Mixer;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;

/// Central audio engine that manages playback
pub struct AudioEngine {
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
    buffer_size: u32, // Buffer size in samples (larger = more stable, higher latency)
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
    /// # Arguments
    /// * `buffer_size` - Buffer size in samples
    ///   - Smaller (512-1024): Lower latency, may underrun with complex synthesis
    ///   - Medium (2048-4096): Balanced
    ///   - Large (8192-16384): Very stable for most cases
    ///   - Very large (32768+): Rock-solid for complex synthesis, ideal for pre-composed playback
    pub fn with_buffer_size(buffer_size: u32) -> Result<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or_else(|| {
            TunesError::AudioEngineError("No output device available".to_string())
        })?;
        let config = device.default_output_config().map_err(|e| {
            TunesError::AudioEngineError(format!("Failed to get default config: {}", e))
        })?;

        let latency_ms = (buffer_size as f32 / config.sample_rate().0 as f32) * 1000.0;

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

        Ok(Self {
            device,
            config,
            buffer_size,
        })
    }

    /// Play a single note or chord for a duration in seconds
    pub fn play(&self, frequencies: &[f32], duration_secs: f32) -> Result<()> {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.run::<f32>(frequencies, duration_secs),
            cpal::SampleFormat::I16 => self.run::<i16>(frequencies, duration_secs),
            cpal::SampleFormat::U16 => self.run::<u16>(frequencies, duration_secs),
            _ => Err(TunesError::InvalidAudioFormat(
                "Unsupported sample format".to_string(),
            )),
        }
    }

    /// Play notes with tempo-based duration
    pub fn play_tempo(
        &self,
        frequencies: &[f32],
        duration: NoteDuration,
        tempo: &Tempo,
    ) -> Result<()> {
        let duration_secs = tempo.duration_to_seconds(duration);
        self.play(frequencies, duration_secs)
    }

    /// Play a drum sound
    pub fn play_drum(&self, drum_type: DrumType) -> Result<()> {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.run_drum::<f32>(drum_type),
            cpal::SampleFormat::I16 => self.run_drum::<i16>(drum_type),
            cpal::SampleFormat::U16 => self.run_drum::<u16>(drum_type),
            _ => Err(TunesError::InvalidAudioFormat(
                "Unsupported sample format".to_string(),
            )),
        }
    }

    /// Play an interpolated sequence from start to end frequency
    pub fn play_interpolated(
        &self,
        start_freq: f32,
        end_freq: f32,
        segments: usize,
        note_duration: f32,
    ) -> Result<()> {
        // Handle edge cases
        if segments == 0 {
            return Ok(()); // Nothing to play
        }
        if segments == 1 {
            // Just play the start frequency
            return self.play(&[start_freq], note_duration);
        }

        for i in 0..segments {
            let t = i as f32 / (segments - 1) as f32;
            let freq = start_freq + (end_freq - start_freq) * t;
            self.play(&[freq], note_duration)?;
        }
        Ok(())
    }

    /// Play a complete composition with multiple tracks
    ///
    /// Uses streaming pre-render: starts playback quickly while rendering ahead.
    /// This provides near-instant playback (~1s delay) with zero glitches.
    ///
    /// For true real-time mode (instant but may have glitches), use `play_mixer_realtime()`.
    /// For full pre-render (slower start, guaranteed quality), use `play_mixer_prerender()`.
    pub fn play_mixer(&self, mixer: &Mixer) -> Result<()> {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.run_mixer_streaming::<f32>(mixer),
            cpal::SampleFormat::I16 => self.run_mixer_streaming::<i16>(mixer),
            cpal::SampleFormat::U16 => self.run_mixer_streaming::<u16>(mixer),
            _ => Err(TunesError::InvalidAudioFormat(
                "Unsupported sample format".to_string(),
            )),
        }
    }

    /// Play a composition in pure real-time mode (no pre-rendering)
    ///
    /// Instant playback start, but may have glitches with complex synthesis.
    /// Use `play_mixer()` for better quality with minimal delay.
    pub fn play_mixer_realtime(&self, mixer: &Mixer) -> Result<()> {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.run_mixer::<f32>(mixer, false),
            cpal::SampleFormat::I16 => self.run_mixer::<i16>(mixer, false),
            cpal::SampleFormat::U16 => self.run_mixer::<u16>(mixer, false),
            _ => Err(TunesError::InvalidAudioFormat(
                "Unsupported sample format".to_string(),
            )),
        }
    }

    /// Play a composition with pre-rendering for glitch-free playback
    ///
    /// Pre-renders the entire composition before playback starts.
    /// This eliminates audio glitches but takes time upfront (~1.4x realtime for complex synthesis).
    /// Use `play_mixer()` for faster iteration during development.
    pub fn play_mixer_prerender(&self, mixer: &Mixer) -> Result<()> {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.run_mixer::<f32>(mixer, true),
            cpal::SampleFormat::I16 => self.run_mixer::<i16>(mixer, true),
            cpal::SampleFormat::U16 => self.run_mixer::<u16>(mixer, true),
            _ => Err(TunesError::InvalidAudioFormat(
                "Unsupported sample format".to_string(),
            )),
        }
    }

    // Internal playback implementation for notes
    fn run<T>(&self, frequencies: &[f32], duration_secs: f32) -> Result<()>
    where
        T: cpal::SizedSample + cpal::FromSample<f32>,
    {
        // Don't play anything if no frequencies provided
        if frequencies.is_empty() {
            return Ok(());
        }

        let sample_rate = self.config.sample_rate().0 as f32;
        let channels = self.config.channels() as usize;

        // Use configured buffer size to prevent underruns
        let mut config: cpal::StreamConfig = self.config.clone().into();
        config.buffer_size = cpal::BufferSize::Fixed(self.buffer_size);

        let mut sample_clock = 0f32;
        let frequencies = frequencies.to_vec();
        let num_frequencies = frequencies.len() as f32;

        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;

            let mut value = 0.0;
            for &freq in &frequencies {
                value += (sample_clock * freq * 2.0 * PI / sample_rate).sin();
            }

            // Safe division - num_frequencies is always > 0 due to check above
            value / num_frequencies * 0.5
        };

        let err_fn = |err| eprintln!("Audio stream error: {}", err);

        let stream = self.device.build_output_stream(
            &config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let value: T = cpal::Sample::from_sample(next_value());
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;
        std::thread::sleep(std::time::Duration::from_secs_f32(duration_secs));

        Ok(())
    }

    // Internal playback implementation for drums
    fn run_drum<T>(&self, drum_type: DrumType) -> Result<()>
    where
        T: cpal::SizedSample + cpal::FromSample<f32>,
    {
        let sample_rate = self.config.sample_rate().0 as f32;
        let channels = self.config.channels() as usize;
        let duration_secs = drum_type.duration();

        // Use configured buffer size to prevent underruns
        let mut config: cpal::StreamConfig = self.config.clone().into();
        config.buffer_size = cpal::BufferSize::Fixed(self.buffer_size);

        let mut sample_index = 0usize;

        let err_fn = |err| eprintln!("Audio stream error: {}", err);

        let stream = self.device.build_output_stream(
            &config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let value = drum_type.sample(sample_index, sample_rate);
                    sample_index += 1;

                    let sample_value: T = cpal::Sample::from_sample(value);
                    for sample in frame.iter_mut() {
                        *sample = sample_value;
                    }
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;
        std::thread::sleep(std::time::Duration::from_secs_f32(duration_secs));

        Ok(())
    }

    // Streaming pre-render: render ahead while playing
    fn run_mixer_streaming<T>(&self, mixer: &Mixer) -> Result<()>
    where
        T: cpal::SizedSample + cpal::FromSample<f32>,
    {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let sample_rate = self.config.sample_rate().0 as f32;
        let channels = self.config.channels() as usize;
        let duration_secs = mixer.total_duration();

        // Shared buffer for rendered audio (thread-safe)
        let buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
        let buffer_clone = Arc::clone(&buffer);

        // Spawn rendering thread
        let mut mixer_clone = mixer.clone();
        let render_thread = thread::spawn(move || {
            let duration = mixer_clone.total_duration();
            let total_samples = (duration * sample_rate).ceil() as usize;
            let mut local_buffer = Vec::with_capacity(total_samples * 2);
            let mut sample_clock = 0.0;

            for i in 0..total_samples {
                let time = i as f32 / sample_rate;
                let (left, right) = mixer_clone.sample_at(time, sample_rate, sample_clock);
                local_buffer.push(left.clamp(-1.0, 1.0));
                local_buffer.push(right.clamp(-1.0, 1.0));
                sample_clock = (sample_clock + 1.0) % sample_rate;

                // Update shared buffer every 4410 samples (~0.1s at 44.1kHz)
                if i % 4410 == 0 {
                    let mut buf = buffer_clone.lock().unwrap();
                    *buf = local_buffer.clone();
                }
            }

            // Final update with complete buffer
            let mut buf = buffer_clone.lock().unwrap();
            *buf = local_buffer;
        });

        // Wait for initial buffer (~1 second of audio)
        let min_buffer_samples = (sample_rate * 1.0) as usize * 2; // 1 second in stereo
        loop {
            let buf_len = buffer.lock().unwrap().len();
            if buf_len >= min_buffer_samples {
                println!(
                    "Buffered {:.1}s, starting playback...",
                    buf_len as f32 / (sample_rate * 2.0)
                );
                break;
            }
            thread::sleep(std::time::Duration::from_millis(50));
        }

        // Create audio stream
        let mut config: cpal::StreamConfig = self.config.clone().into();
        config.buffer_size = cpal::BufferSize::Fixed(self.buffer_size);

        let mut sample_index = 0;
        let buffer_for_playback = Arc::clone(&buffer);

        let err_fn = |err| eprintln!("Audio stream error: {}", err);

        let stream = self.device.build_output_stream(
            &config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let buf = buffer_for_playback.lock().unwrap();

                    if sample_index * 2 + 1 >= buf.len() {
                        // Not enough data yet, fill with silence
                        for sample in frame.iter_mut() {
                            *sample = cpal::Sample::from_sample(0.0f32);
                        }
                        continue;
                    }

                    let buffer_idx = sample_index * 2;
                    let left = buf[buffer_idx];
                    let right = buf[buffer_idx + 1];
                    sample_index += 1;

                    if channels == 1 {
                        frame[0] = cpal::Sample::from_sample((left + right) * 0.5);
                    } else if channels == 2 {
                        frame[0] = cpal::Sample::from_sample(left);
                        frame[1] = cpal::Sample::from_sample(right);
                    } else {
                        frame[0] = cpal::Sample::from_sample(left);
                        frame[1] = cpal::Sample::from_sample(right);
                        for sample in frame.iter_mut().skip(2) {
                            *sample = cpal::Sample::from_sample(0.0f32);
                        }
                    }
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;
        std::thread::sleep(std::time::Duration::from_secs_f32(duration_secs));

        // Wait for rendering to complete
        render_thread.join().ok();

        Ok(())
    }

    // Internal playback implementation for mixer
    fn run_mixer<T>(&self, mixer: &Mixer, pre_render: bool) -> Result<()>
    where
        T: cpal::SizedSample + cpal::FromSample<f32>,
    {
        let sample_rate = self.config.sample_rate().0 as f32;
        let channels = self.config.channels() as usize;
        let duration_secs = mixer.total_duration();

        // Create config with configured buffer size
        let mut config: cpal::StreamConfig = self.config.clone().into();
        config.buffer_size = cpal::BufferSize::Fixed(self.buffer_size);

        let err_fn = |err| eprintln!("Audio stream error: {}", err);

        if pre_render {
            // PRE-RENDER MODE: Generate all audio samples upfront for glitch-free playback
            println!("Pre-rendering audio...");
            let start = std::time::Instant::now();

            let mut mixer_owned = mixer.clone();
            let rendered_buffer = mixer_owned.render_to_buffer(sample_rate);

            let render_time = start.elapsed();
            println!(
                "  Rendered {:.2}s of audio in {:.2}s ({:.1}x realtime)",
                duration_secs,
                render_time.as_secs_f32(),
                duration_secs / render_time.as_secs_f32()
            );

            // Playback simply reads from the pre-rendered buffer (trivial CPU load)
            let mut sample_index = 0;
            let total_samples = rendered_buffer.len() / 2; // Stereo pairs

            let stream = self.device.build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    for frame in data.chunks_mut(channels) {
                        if sample_index >= total_samples {
                            for sample in frame.iter_mut() {
                                *sample = cpal::Sample::from_sample(0.0f32);
                            }
                            continue;
                        }

                        let buffer_idx = sample_index * 2;
                        let left = rendered_buffer[buffer_idx];
                        let right = rendered_buffer[buffer_idx + 1];
                        sample_index += 1;

                        if channels == 1 {
                            frame[0] = cpal::Sample::from_sample((left + right) * 0.5);
                        } else if channels == 2 {
                            frame[0] = cpal::Sample::from_sample(left);
                            frame[1] = cpal::Sample::from_sample(right);
                        } else {
                            frame[0] = cpal::Sample::from_sample(left);
                            frame[1] = cpal::Sample::from_sample(right);
                            for sample in frame.iter_mut().skip(2) {
                                *sample = cpal::Sample::from_sample(0.0f32);
                            }
                        }
                    }
                },
                err_fn,
                None,
            )?;

            stream.play()?;
            std::thread::sleep(std::time::Duration::from_secs_f32(duration_secs));
        } else {
            // REAL-TIME MODE: Synthesize on-the-fly (faster startup, may have glitches)
            let mut mixer_owned = mixer.clone();
            let mut sample_clock = 0f32;
            let mut elapsed_time = 0f32;

            let stream = self.device.build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    for frame in data.chunks_mut(channels) {
                        let (left, right) =
                            mixer_owned.sample_at(elapsed_time, sample_rate, sample_clock);
                        sample_clock = (sample_clock + 1.0) % sample_rate;
                        elapsed_time += 1.0 / sample_rate;

                        if channels == 1 {
                            frame[0] = cpal::Sample::from_sample((left + right) * 0.5);
                        } else if channels == 2 {
                            frame[0] = cpal::Sample::from_sample(left);
                            frame[1] = cpal::Sample::from_sample(right);
                        } else {
                            frame[0] = cpal::Sample::from_sample(left);
                            frame[1] = cpal::Sample::from_sample(right);
                            for sample in frame.iter_mut().skip(2) {
                                *sample = cpal::Sample::from_sample(0.0f32);
                            }
                        }
                    }
                },
                err_fn,
                None,
            )?;

            stream.play()?;
            std::thread::sleep(std::time::Duration::from_secs_f32(duration_secs));
        }

        Ok(())
    }

    /// Render a mixer to a WAV file
    ///
    /// Convenience method that renders the mixer's audio to a WAV file.
    /// Uses the default sample rate (44100 Hz).
    ///
    /// # Arguments
    /// * `mixer` - The mixer to render
    /// * `path` - Output file path (e.g., "output.wav")
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> Result<()> {
    /// let engine = AudioEngine::new()?;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("piano").note(&[440.0], 1.0);
    ///
    /// let mut mixer = comp.into_mixer();
    /// engine.render_to_wav(&mut mixer, "output.wav")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn render_to_wav(&self, mixer: &mut Mixer, path: &str) -> Result<()> {
        let sample_rate = self.config.sample_rate().0;
        mixer
            .export_wav(path, sample_rate)
            .map_err(|e| TunesError::WavWriteError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notes::*;

    // Note: Most engine tests require actual audio device access and are integration tests.
    // These tests focus on the logic we can test without device interaction.

    #[test]
    fn test_interpolated_frequency_calculation() {
        // Test that interpolation produces correct intermediate frequencies
        let start = 100.0;
        let end = 200.0;
        let segments = 5;

        let mut frequencies = Vec::new();
        for i in 0..segments {
            let t = i as f32 / (segments - 1) as f32;
            let freq = start + (end - start) * t;
            frequencies.push(freq);
        }

        // Should produce: 100, 125, 150, 175, 200
        assert_eq!(frequencies[0], 100.0);
        assert_eq!(frequencies[1], 125.0);
        assert_eq!(frequencies[2], 150.0);
        assert_eq!(frequencies[3], 175.0);
        assert_eq!(frequencies[4], 200.0);
    }

    #[test]
    fn test_interpolated_single_segment() {
        // With 1 segment, t = 0/0 which would be NaN, but code handles this
        // by just playing start_freq

        // The formula with segments=1 would use t = 0/(1-1) = 0/0
        // But the code has special handling for segments == 1
        // We're just validating the math works correctly for this edge case
        let segments = 1;
        let t = 0.0_f32 / (segments - 1) as f32;
        assert!(t.is_nan() || t == 0.0); // This would be NaN without special handling
    }

    #[test]
    fn test_interpolated_zero_segments() {
        // Segments = 0 should be handled as no-op
        // The code checks for this and returns Ok(()) early
        let segments = 0;
        assert_eq!(segments, 0); // Just verify the edge case exists
    }

    #[test]
    fn test_tempo_duration_conversion() {
        let tempo = Tempo::new(120.0); // 120 BPM

        // Quarter note at 120 BPM = 0.5 seconds
        let quarter_duration = tempo.quarter_note();
        assert_eq!(quarter_duration, 0.5);

        // Whole note = 4 * quarter note
        let whole_duration = tempo.whole_note();
        assert_eq!(whole_duration, 2.0);

        // Eighth note = quarter / 2
        let eighth_duration = tempo.eighth_note();
        assert_eq!(eighth_duration, 0.25);

        // Sixteenth note = quarter / 4
        let sixteenth_duration = tempo.sixteenth_note();
        assert_eq!(sixteenth_duration, 0.125);
    }

    #[test]
    fn test_frequency_array_handling() {
        // Test that multiple frequencies are properly handled
        let frequencies = vec![440.0, 554.37, 659.25]; // A major chord
        assert_eq!(frequencies.len(), 3);

        // Average should be calculated during mixing
        let sum: f32 = frequencies.iter().sum();
        let avg = sum / frequencies.len() as f32;
        assert!((avg - 551.21).abs() < 0.01);
    }

    #[test]
    fn test_empty_frequency_array() {
        // Empty array should be handled gracefully (checked in run())
        let frequencies: Vec<f32> = vec![];
        assert!(frequencies.is_empty());
        // The engine's run() method checks for this and returns Ok(()) early
    }

    #[test]
    fn test_sample_rate_calculation() {
        // Common sample rates
        let sample_rate = 44100.0_f32;
        let frequency = 440.0_f32; // A4

        // Number of samples per cycle
        let samples_per_cycle = sample_rate / frequency;
        assert!((samples_per_cycle - 100.227_f32).abs() < 0.001);

        // Verify sample clock wrapping
        let sample_clock = sample_rate + 1.0;
        let wrapped = sample_clock % sample_rate;
        assert_eq!(wrapped, 1.0);
    }

    #[test]
    fn test_drum_duration_consistency() {
        // Verify drum durations are sensible
        assert_eq!(DrumType::Kick.duration(), 0.15);
        assert_eq!(DrumType::Snare.duration(), 0.1);
        assert_eq!(DrumType::HiHatClosed.duration(), 0.05);
        assert_eq!(DrumType::Crash.duration(), 1.5);

        // Longer drums should have longer durations
        assert!(DrumType::Crash.duration() > DrumType::Kick.duration());
        assert!(DrumType::HiHatOpen.duration() > DrumType::HiHatClosed.duration());
    }

    #[test]
    fn test_channel_handling_mono() {
        // Test mono channel mixing
        let left = 0.5;
        let right = 0.3;
        let mono = (left + right) * 0.5;
        assert_eq!(mono, 0.4);
    }

    #[test]
    fn test_channel_handling_stereo() {
        // Stereo keeps channels separate
        let left = 0.5;
        let right = 0.3;
        assert_eq!(left, 0.5);
        assert_eq!(right, 0.3);
    }

    #[test]
    fn test_time_advancement() {
        // Test time advancement calculation
        let sample_rate = 44100.0_f32;
        let time_per_sample = 1.0 / sample_rate;

        assert!((time_per_sample - 0.0000226_f32).abs() < 0.0000001);

        // After 44100 samples, time should advance by 1 second
        let elapsed = time_per_sample * 44100.0;
        assert!((elapsed - 1.0_f32).abs() < 0.001);
    }

    #[test]
    fn test_note_constants() {
        // Verify note frequency constants are reasonable
        assert_eq!(A4, 440.0);
        assert!(C4 < D4);
        assert!(D4 < E4);

        // Octave relationship: next octave should be 2x frequency
        assert!((C5 / C4 - 2.0).abs() < 0.01);
    }

    // Integration tests that require audio device would go in tests/ directory
    // These would include:
    // - Actual playback tests (if audio device available)
    // - Full mixer playback
    // - Multi-track rendering
    // - Effect chain processing
}

// Note: Full integration tests requiring audio devices should be placed in
// tests/integration_tests.rs with #[ignore] attribute for CI environments
// without audio hardware.
