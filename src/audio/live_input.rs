//! Live audio input recording

use crate::error::{Result, TunesError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Simple live audio recorder
///
/// Records from the default input device (microphone/line-in) to a WAV file.
/// After recording, the WAV file can be used with the existing sample playback pipeline.
///
/// # Example
///
/// ```no_run
/// use tunes::audio::LiveInput;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut recorder = LiveInput::new()?;
/// recorder.start_recording("recording.wav")?;
///
/// // ... record some audio ...
///
/// recorder.stop()?;
/// # Ok(())
/// # }
/// ```
pub struct LiveInput {
    stream: Option<cpal::Stream>,
    writer: Option<Arc<Mutex<WavWriter<std::io::BufWriter<std::fs::File>>>>>,
    sample_rate: u32,
    channels: u16,
}

impl LiveInput {
    /// Create a new live input recorder
    ///
    /// This initializes the audio system but does not start recording yet.
    /// Use `start_recording()` to begin capturing audio.
    pub fn new() -> Result<Self> {
        // Get default input device
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| TunesError::AudioEngineError("No input device available".to_string()))?;

        // Get default input config
        let config = device
            .default_input_config()
            .map_err(|e| TunesError::AudioEngineError(format!("Failed to get default input config: {}", e)))?;

        Ok(Self {
            stream: None,
            writer: None,
            sample_rate: config.sample_rate().0,
            channels: config.channels(),
        })
    }

    /// Start recording to a WAV file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to save the WAV file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be created
    /// - The audio device cannot be opened
    /// - Already recording (call `stop()` first)
    pub fn start_recording(&mut self, path: impl AsRef<Path>) -> Result<()> {
        if self.stream.is_some() {
            return Err(TunesError::AudioEngineError("Already recording. Call stop() first.".to_string()));
        }

        // Create WAV writer
        let spec = WavSpec {
            channels: self.channels,
            sample_rate: self.sample_rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let writer = WavWriter::create(path.as_ref(), spec)
            .map_err(|e| TunesError::WavWriteError(e.to_string()))?;
        let writer = Arc::new(Mutex::new(writer));
        self.writer = Some(Arc::clone(&writer));

        // Get input device
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| TunesError::AudioEngineError("No input device available".to_string()))?;

        let config = device
            .default_input_config()
            .map_err(|e| TunesError::AudioEngineError(format!("Failed to get default input config: {}", e)))?;

        // Build input stream
        let err_fn = |err| eprintln!("Audio stream error: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                let config: cpal::StreamConfig = config.into();
                device.build_input_stream(
                    &config,
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        if let Ok(mut writer) = writer.lock() {
                            for &sample in data {
                                writer.write_sample(sample).ok();
                            }
                        }
                    },
                    err_fn,
                    None,
                )?
            }
            cpal::SampleFormat::I16 => {
                let config: cpal::StreamConfig = config.into();
                device.build_input_stream(
                    &config,
                    move |data: &[i16], _: &cpal::InputCallbackInfo| {
                        if let Ok(mut writer) = writer.lock() {
                            for &sample in data {
                                // Convert i16 to f32 [-1.0, 1.0]
                                let normalized = sample as f32 / i16::MAX as f32;
                                writer.write_sample(normalized).ok();
                            }
                        }
                    },
                    err_fn,
                    None,
                )?
            }
            cpal::SampleFormat::U16 => {
                let config: cpal::StreamConfig = config.into();
                device.build_input_stream(
                    &config,
                    move |data: &[u16], _: &cpal::InputCallbackInfo| {
                        if let Ok(mut writer) = writer.lock() {
                            for &sample in data {
                                // Convert u16 to f32 [-1.0, 1.0]
                                let normalized = (sample as f32 - 32768.0) / 32768.0;
                                writer.write_sample(normalized).ok();
                            }
                        }
                    },
                    err_fn,
                    None,
                )?
            }
            _ => return Err(TunesError::AudioEngineError("Unsupported sample format".to_string())),
        };

        stream.play()?;
        self.stream = Some(stream);

        println!(
            "🎙️  Recording at {} Hz, {} channel(s)",
            self.sample_rate, self.channels
        );

        Ok(())
    }

    /// Stop recording and finalize the WAV file
    ///
    /// This closes the audio stream and ensures all data is written to disk.
    /// The WAV file will be complete and ready to use after this call.
    pub fn stop(&mut self) -> Result<()> {
        // Drop stream first (stops recording)
        self.stream.take();

        // Finalize WAV file
        if let Some(writer) = self.writer.take() {
            // Try to get exclusive access to finalize
            // Use Arc::try_unwrap to get ownership if we're the last reference
            match Arc::try_unwrap(writer) {
                Ok(mutex) => {
                    let writer = mutex.into_inner().unwrap();
                    writer.finalize().map_err(|e| TunesError::WavWriteError(e.to_string()))?;
                }
                Err(arc) => {
                    // Still shared - just lock and finalize
                    if let Ok(_writer) = arc.lock() {
                        // WAV writer will finalize on drop
                    }
                }
            }
        }

        println!("✅ Recording stopped");

        Ok(())
    }

    /// Get the sample rate of the input device
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get the number of channels (1 = mono, 2 = stereo)
    pub fn channels(&self) -> u16 {
        self.channels
    }

    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        self.stream.is_some()
    }
}

impl Drop for LiveInput {
    fn drop(&mut self) {
        // Ensure recording stops cleanly on drop
        if self.is_recording() {
            self.stop().ok();
        }
    }
}
