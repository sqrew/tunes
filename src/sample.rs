/// Sample playback module for loading and playing WAV files
/// This module provides functionality to load audio samples from WAV files
/// and play them back with pitch shifting, looping, and effects.
use crate::error::{Result, TunesError};
use std::path::Path;
use std::sync::Arc;

/// An audio sample loaded from a WAV file
/// Samples are stored in memory as f32 values (-1.0 to 1.0) and can be
/// shared efficiently between multiple playback instances using Arc.
#[derive(Debug, Clone)]
pub struct Sample {
    /// Sample data (interleaved stereo: L, R, L, R... or mono)
    pub data: Arc<Vec<f32>>,

    /// Number of channels (1 = mono, 2 = stereo)
    pub channels: u16,

    /// Original sample rate the file was recorded at
    pub sample_rate: u32,

    /// Duration in seconds
    pub duration: f32,
}

impl Sample {
    /// Load a sample from a WAV file
    ///
    /// # Arguments
    /// * `path` - Path to the WAV file
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::sample::Sample;
    /// let kick = Sample::from_wav("samples/kick.wav")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn from_wav<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut reader = hound::WavReader::open(path.as_ref())?;
        let spec = reader.spec();

        // Read all samples and convert to f32
        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Int => {
                match spec.bits_per_sample {
                    16 => reader
                        .samples::<i16>()
                        .map(|s| {
                            s.map(|sample| sample as f32 / 32768.0)
                                .map_err(TunesError::from)
                        })
                        .collect::<Result<Vec<f32>>>()?,
                    24 => {
                        reader
                            .samples::<i32>()
                            .map(|s| {
                                s.map(|sample| sample as f32 / 8388608.0)
                                    .map_err(TunesError::from)
                            }) // 2^23
                            .collect::<Result<Vec<f32>>>()?
                    }
                    32 => {
                        reader
                            .samples::<i32>()
                            .map(|s| {
                                s.map(|sample| sample as f32 / 2147483648.0)
                                    .map_err(TunesError::from)
                            }) // 2^31
                            .collect::<Result<Vec<f32>>>()?
                    }
                    _ => {
                        return Err(TunesError::WavReadError(format!(
                            "Unsupported bit depth: {}",
                            spec.bits_per_sample
                        )));
                    }
                }
            }
            hound::SampleFormat::Float => reader
                .samples::<f32>()
                .map(|s| s.map_err(TunesError::from))
                .collect::<Result<Vec<f32>>>()?,
        };

        let num_frames = samples.len() / spec.channels as usize;
        let duration = num_frames as f32 / spec.sample_rate as f32;

        Ok(Self {
            data: Arc::new(samples),
            channels: spec.channels,
            sample_rate: spec.sample_rate,
            duration,
        })
    }

    /// Get a sample at a specific time position
    ///
    /// Returns (left, right) channels. For mono samples, both channels are the same.
    ///
    /// # Arguments
    /// * `time` - Time in seconds from the start of the sample
    /// * `playback_rate` - Speed multiplier (1.0 = normal, 2.0 = double speed, 0.5 = half speed)
    /// * `target_sample_rate` - The sample rate of the audio engine
    pub fn sample_at(&self, time: f32, playback_rate: f32, _target_sample_rate: f32) -> (f32, f32) {
        // Calculate the position in the original sample
        let position_seconds = time * playback_rate;
        let sample_position = position_seconds * self.sample_rate as f32;
        let frame_index = sample_position as usize;

        // Check bounds
        let num_frames = self.data.len() / self.channels as usize;
        if frame_index >= num_frames {
            return (0.0, 0.0);
        }

        match self.channels {
            1 => {
                // Mono: use same value for both channels
                let value = self.data.get(frame_index).copied().unwrap_or(0.0);
                (value, value)
            }
            2 => {
                // Stereo: interleaved L, R, L, R...
                let sample_index = frame_index * 2;
                let left = self.data.get(sample_index).copied().unwrap_or(0.0);
                let right = self.data.get(sample_index + 1).copied().unwrap_or(0.0);
                (left, right)
            }
            _ => {
                // Multi-channel: just use first two channels
                let sample_index = frame_index * self.channels as usize;
                let left = self.data.get(sample_index).copied().unwrap_or(0.0);
                let right = self.data.get(sample_index + 1).copied().unwrap_or(0.0);
                (left, right)
            }
        }
    }

    /// Get interpolated sample at a specific time position (higher quality)
    ///
    /// Uses linear interpolation for smoother playback when pitch shifting.
    pub fn sample_at_interpolated(
        &self,
        time: f32,
        playback_rate: f32,
        target_sample_rate: f32,
    ) -> (f32, f32) {
        let position_seconds = time * playback_rate;
        let sample_position = position_seconds * self.sample_rate as f32;
        let frame_index = sample_position as usize;
        let frac = sample_position.fract();

        let num_frames = self.data.len() / self.channels as usize;
        if frame_index >= num_frames - 1 {
            return self.sample_at(time, playback_rate, target_sample_rate);
        }

        match self.channels {
            1 => {
                // Mono with interpolation
                let sample1 = self.data.get(frame_index).copied().unwrap_or(0.0);
                let sample2 = self.data.get(frame_index + 1).copied().unwrap_or(0.0);
                let value = sample1 + (sample2 - sample1) * frac;
                (value, value)
            }
            2 => {
                // Stereo with interpolation
                let sample_index = frame_index * 2;

                let left1 = self.data.get(sample_index).copied().unwrap_or(0.0);
                let right1 = self.data.get(sample_index + 1).copied().unwrap_or(0.0);
                let left2 = self.data.get(sample_index + 2).copied().unwrap_or(0.0);
                let right2 = self.data.get(sample_index + 3).copied().unwrap_or(0.0);

                let left = left1 + (left2 - left1) * frac;
                let right = right1 + (right2 - right1) * frac;

                (left, right)
            }
            _ => {
                // Multi-channel with interpolation (use first two channels)
                let sample_index = frame_index * self.channels as usize;
                let next_index = (frame_index + 1) * self.channels as usize;

                let left1 = self.data.get(sample_index).copied().unwrap_or(0.0);
                let right1 = self.data.get(sample_index + 1).copied().unwrap_or(0.0);
                let left2 = self.data.get(next_index).copied().unwrap_or(0.0);
                let right2 = self.data.get(next_index + 1).copied().unwrap_or(0.0);

                let left = left1 + (left2 - left1) * frac;
                let right = right1 + (right2 - right1) * frac;

                (left, right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mono_sample_playback() {
        // Create a simple mono sample
        let data = vec![0.0, 0.5, 1.0, 0.5, 0.0, -0.5, -1.0];
        let sample = Sample {
            data: Arc::new(data),
            channels: 1,
            sample_rate: 44100,
            duration: 7.0 / 44100.0,
        };

        // Sample at time 0 (first sample)
        let (left, right) = sample.sample_at(0.0, 1.0, 44100.0);
        assert_eq!(left, 0.0);
        assert_eq!(right, 0.0); // Mono duplicated to both channels

        // Sample at peak
        let (left, right) = sample.sample_at(2.0 / 44100.0, 1.0, 44100.0);
        assert_eq!(left, 1.0);
        assert_eq!(right, 1.0);
    }

    #[test]
    fn test_stereo_sample_playback() {
        // Create a simple stereo sample (L, R, L, R...)
        let data = vec![1.0, 0.0, 0.5, 0.5, 0.0, 1.0];
        let sample = Sample {
            data: Arc::new(data),
            channels: 2,
            sample_rate: 44100,
            duration: 3.0 / 44100.0,
        };

        // First frame
        let (left, right) = sample.sample_at(0.0, 1.0, 44100.0);
        assert_eq!(left, 1.0);
        assert_eq!(right, 0.0);

        // Second frame
        let (left, right) = sample.sample_at(1.0 / 44100.0, 1.0, 44100.0);
        assert_eq!(left, 0.5);
        assert_eq!(right, 0.5);
    }

    #[test]
    fn test_playback_rate() {
        let data = vec![0.0, 0.5, 1.0, 0.5];
        let sample = Sample {
            data: Arc::new(data),
            channels: 1,
            sample_rate: 44100,
            duration: 4.0 / 44100.0,
        };

        // Double speed: at time 1/44100, we should be at sample index 2
        let (left, _) = sample.sample_at(1.0 / 44100.0, 2.0, 44100.0);
        assert_eq!(left, 1.0);
    }

    #[test]
    fn test_bounds_checking() {
        let data = vec![1.0, 2.0, 3.0];
        let sample = Sample {
            data: Arc::new(data),
            channels: 1,
            sample_rate: 44100,
            duration: 3.0 / 44100.0,
        };

        // Beyond the end of sample
        let (left, right) = sample.sample_at(10.0, 1.0, 44100.0);
        assert_eq!(left, 0.0);
        assert_eq!(right, 0.0);
    }
}
