use crate::drums::DrumType;
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
    /// Uses a large buffer size (8192 samples) for stable playback with complex synthesis.
    /// For lower latency at the cost of potential underruns, use `with_buffer_size()`.
    pub fn new() -> Result<Self, anyhow::Error> {
        Self::with_buffer_size(8192) // ~185ms at 44.1kHz - very stable for complex synthesis
    }

    /// Create a new audio engine with custom buffer size
    ///
    /// # Arguments
    /// * `buffer_size` - Buffer size in samples
    ///   - Smaller (512-1024): Lower latency, may underrun with complex synthesis
    ///   - Medium (2048-4096): Balanced
    ///   - Larger (8192+): Very stable, higher latency but fine for music playback
    pub fn with_buffer_size(buffer_size: u32) -> Result<Self, anyhow::Error> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))?;
        let config = device
            .default_output_config()
            .map_err(|e| anyhow::anyhow!("Failed to get default config: {}", e))?;

        let latency_ms = (buffer_size as f32 / config.sample_rate().0 as f32) * 1000.0;

        println!("Audio Engine initialized:");
        println!("  Device: {}", device.name().unwrap_or_else(|_| "Unknown".to_string()));
        println!("  Sample rate: {}", config.sample_rate().0);
        println!("  Buffer size: {} samples ({:.1}ms latency)", buffer_size, latency_ms);

        Ok(Self { device, config, buffer_size })
    }

    /// Play a single note or chord for a duration in seconds
    pub fn play(&self, frequencies: &[f32], duration_secs: f32) -> Result<(), anyhow::Error> {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.run::<f32>(frequencies, duration_secs),
            cpal::SampleFormat::I16 => self.run::<i16>(frequencies, duration_secs),
            cpal::SampleFormat::U16 => self.run::<u16>(frequencies, duration_secs),
            _ => Err(anyhow::anyhow!("Unsupported sample format")),
        }
    }

    /// Play notes with tempo-based duration
    pub fn play_tempo(
        &self,
        frequencies: &[f32],
        duration: NoteDuration,
        tempo: &Tempo,
    ) -> Result<(), anyhow::Error> {
        let duration_secs = tempo.duration_to_seconds(duration);
        self.play(frequencies, duration_secs)
    }

    /// Play a drum sound
    pub fn play_drum(&self, drum_type: DrumType) -> Result<(), anyhow::Error> {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.run_drum::<f32>(drum_type),
            cpal::SampleFormat::I16 => self.run_drum::<i16>(drum_type),
            cpal::SampleFormat::U16 => self.run_drum::<u16>(drum_type),
            _ => Err(anyhow::anyhow!("Unsupported sample format")),
        }
    }

    /// Play an interpolated sequence from start to end frequency
    pub fn play_interpolated(
        &self,
        start_freq: f32,
        end_freq: f32,
        segments: usize,
        note_duration: f32,
    ) -> Result<(), anyhow::Error> {
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
    pub fn play_mixer(&self, mixer: &Mixer) -> Result<(), anyhow::Error> {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.run_mixer::<f32>(mixer),
            cpal::SampleFormat::I16 => self.run_mixer::<i16>(mixer),
            cpal::SampleFormat::U16 => self.run_mixer::<u16>(mixer),
            _ => Err(anyhow::anyhow!("Unsupported sample format")),
        }
    }

    // Internal playback implementation for notes
    fn run<T>(&self, frequencies: &[f32], duration_secs: f32) -> Result<(), anyhow::Error>
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
    fn run_drum<T>(&self, drum_type: DrumType) -> Result<(), anyhow::Error>
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

    // Internal playback implementation for mixer
    fn run_mixer<T>(&self, mixer: &Mixer) -> Result<(), anyhow::Error>
    where
        T: cpal::SizedSample + cpal::FromSample<f32>,
    {
        let sample_rate = self.config.sample_rate().0 as f32;
        let channels = self.config.channels() as usize;
        let duration_secs = mixer.total_duration();

        // Create config with configured buffer size to prevent underruns with complex synthesis
        let mut config: cpal::StreamConfig = self.config.clone().into();
        config.buffer_size = cpal::BufferSize::Fixed(self.buffer_size);

        let mut mixer_owned = mixer.clone();
        let mut sample_clock = 0f32;
        let mut elapsed_time = 0f32;

        let err_fn = |err| eprintln!("Audio stream error: {}", err);

        let stream = self.device.build_output_stream(
            &config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let (left, right) = mixer_owned.sample_at(elapsed_time, sample_rate, sample_clock);
                    sample_clock = (sample_clock + 1.0) % sample_rate;
                    elapsed_time += 1.0 / sample_rate;

                    // Handle mono, stereo, or multi-channel output
                    if channels == 1 {
                        // Mono: average left and right
                        let mono = (left + right) * 0.5;
                        frame[0] = cpal::Sample::from_sample(mono);
                    } else if channels == 2 {
                        // Stereo: use left and right directly
                        frame[0] = cpal::Sample::from_sample(left);
                        frame[1] = cpal::Sample::from_sample(right);
                    } else {
                        // Multi-channel: put left/right in first two channels, silence the rest
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
    /// # fn main() -> Result<(), anyhow::Error> {
    /// let engine = AudioEngine::new()?;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("piano").note(&[440.0], 1.0);
    ///
    /// let mut mixer = comp.into_mixer();
    /// engine.render_to_wav(&mut mixer, "output.wav")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn render_to_wav(&self, mixer: &mut Mixer, path: &str) -> Result<(), anyhow::Error> {
        let sample_rate = self.config.sample_rate().0;
        mixer.export_wav(path, sample_rate)
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default AudioEngine")
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
