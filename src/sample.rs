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

    /// Cached number of frames (samples / channels) for fast access
    num_frames: usize,

    /// Loop start point in frames (None = no loop)
    loop_start: Option<usize>,

    /// Loop end point in frames (None = no loop)
    loop_end: Option<usize>,
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
            num_frames,
            loop_start: None,
            loop_end: None,
        })
    }

    /// Create a sample from raw mono audio data
    ///
    /// # Arguments
    /// * `samples` - Vector of audio samples in the range [-1.0, 1.0]
    /// * `sample_rate` - Sample rate in Hz
    ///
    /// # Example
    /// ```
    /// use tunes::sample::Sample;
    ///
    /// // Create 1 second of silence at 44.1kHz
    /// let silence = vec![0.0; 44100];
    /// let sample = Sample::from_mono(silence, 44100);
    /// ```
    pub fn from_mono(samples: Vec<f32>, sample_rate: u32) -> Self {
        let num_frames = samples.len();
        let duration = num_frames as f32 / sample_rate as f32;

        Self {
            data: Arc::new(samples),
            channels: 1,
            sample_rate,
            duration,
            num_frames,
            loop_start: None,
            loop_end: None,
        }
    }

    /// Get a sample at a specific time position
    ///
    /// Returns (left, right) channels. For mono samples, both channels are the same.
    /// If looping is enabled, the sample will loop between loop_start and loop_end.
    ///
    /// # Arguments
    /// * `time` - Time in seconds from the start of the sample
    /// * `playback_rate` - Speed multiplier (1.0 = normal, 2.0 = double speed, 0.5 = half speed)
    #[inline]
    pub fn sample_at(&self, time: f32, playback_rate: f32) -> (f32, f32) {
        // Calculate the position in the original sample
        let position_seconds = time * playback_rate;
        let sample_position = position_seconds * self.sample_rate as f32;
        let mut frame_index = sample_position as usize;

        // Handle looping
        if let (Some(loop_start), Some(loop_end)) = (self.loop_start, self.loop_end) {
            if frame_index >= loop_end {
                let loop_length = loop_end - loop_start;
                frame_index = loop_start + (frame_index - loop_start) % loop_length;
            }
        } else if frame_index >= self.num_frames {
            // No loop, beyond bounds
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
    /// If looping is enabled, the sample will loop between loop_start and loop_end.
    #[inline]
    pub fn sample_at_interpolated(&self, time: f32, playback_rate: f32) -> (f32, f32) {
        let position_seconds = time * playback_rate;
        let sample_position = position_seconds * self.sample_rate as f32;
        let mut frame_index = sample_position as usize;
        let frac = sample_position.fract();

        // Handle looping
        if let (Some(loop_start), Some(loop_end)) = (self.loop_start, self.loop_end) {
            if frame_index >= loop_end {
                let loop_length = loop_end - loop_start;
                frame_index = loop_start + (frame_index - loop_start) % loop_length;
            }
        } else if frame_index >= self.num_frames - 1 {
            // No loop, near end - fall back to non-interpolated
            return self.sample_at(time, playback_rate);
        }

        match self.channels {
            1 => {
                // Mono with interpolation - use unsafe for performance
                unsafe {
                    let sample1 = *self.data.get_unchecked(frame_index);
                    let sample2 = *self.data.get_unchecked(frame_index + 1);
                    let value = sample1 + (sample2 - sample1) * frac;
                    (value, value)
                }
            }
            2 => {
                // Stereo with interpolation - use unsafe for performance
                let sample_index = frame_index * 2;
                unsafe {
                    let left1 = *self.data.get_unchecked(sample_index);
                    let right1 = *self.data.get_unchecked(sample_index + 1);
                    let left2 = *self.data.get_unchecked(sample_index + 2);
                    let right2 = *self.data.get_unchecked(sample_index + 3);

                    let left = left1 + (left2 - left1) * frac;
                    let right = right1 + (right2 - right1) * frac;

                    (left, right)
                }
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

    /// Create a sub-sample from a time range
    ///
    /// # Arguments
    /// * `start_time` - Start time in seconds
    /// * `end_time` - End time in seconds
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::sample::Sample;
    /// let sample = Sample::from_wav("kick.wav")?;
    /// let attack = sample.slice(0.0, 0.05)?; // First 50ms
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn slice(&self, start_time: f32, end_time: f32) -> Result<Self> {
        let start_frame = (start_time * self.sample_rate as f32) as usize;
        let end_frame = (end_time * self.sample_rate as f32) as usize;
        self.slice_frames(start_frame, end_frame)
    }

    /// Create a sub-sample from frame indices
    ///
    /// # Arguments
    /// * `start_frame` - Starting frame index
    /// * `end_frame` - Ending frame index (exclusive)
    pub fn slice_frames(&self, start_frame: usize, end_frame: usize) -> Result<Self> {
        if start_frame >= end_frame {
            return Err(TunesError::InvalidAudioFormat(
                "Start frame must be before end frame".to_string(),
            ));
        }

        if end_frame > self.num_frames {
            return Err(TunesError::InvalidAudioFormat(format!(
                "End frame {} exceeds sample length {}",
                end_frame, self.num_frames
            )));
        }

        let start_index = start_frame * self.channels as usize;
        let end_index = end_frame * self.channels as usize;
        let sliced_data = self.data[start_index..end_index].to_vec();

        let num_frames = sliced_data.len() / self.channels as usize;
        let duration = num_frames as f32 / self.sample_rate as f32;

        Ok(Self {
            data: Arc::new(sliced_data),
            channels: self.channels,
            sample_rate: self.sample_rate,
            duration,
            num_frames,
            loop_start: None,
            loop_end: None,
        })
    }

    /// Enable looping between two time points
    ///
    /// # Arguments
    /// * `loop_start` - Start of loop in seconds
    /// * `loop_end` - End of loop in seconds
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::sample::Sample;
    /// let sample = Sample::from_wav("synth.wav")?
    ///     .with_loop(0.5, 2.0)?; // Loop between 0.5s and 2.0s
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn with_loop(mut self, loop_start: f32, loop_end: f32) -> Result<Self> {
        let start_frame = (loop_start * self.sample_rate as f32) as usize;
        let end_frame = (loop_end * self.sample_rate as f32) as usize;

        if start_frame >= end_frame {
            return Err(TunesError::InvalidAudioFormat(
                "Loop start must be before loop end".to_string(),
            ));
        }

        if end_frame > self.num_frames {
            return Err(TunesError::InvalidAudioFormat(
                "Loop end exceeds sample length".to_string(),
            ));
        }

        self.loop_start = Some(start_frame);
        self.loop_end = Some(end_frame);
        Ok(self)
    }

    /// Enable looping between frame indices
    pub fn with_loop_frames(mut self, loop_start: usize, loop_end: usize) -> Result<Self> {
        if loop_start >= loop_end {
            return Err(TunesError::InvalidAudioFormat(
                "Loop start must be before loop end".to_string(),
            ));
        }

        if loop_end > self.num_frames {
            return Err(TunesError::InvalidAudioFormat(
                "Loop end exceeds sample length".to_string(),
            ));
        }

        self.loop_start = Some(loop_start);
        self.loop_end = Some(loop_end);
        Ok(self)
    }

    /// Disable looping
    pub fn without_loop(mut self) -> Self {
        self.loop_start = None;
        self.loop_end = None;
        self
    }

    /// Normalize the sample to peak amplitude
    ///
    /// Scales the sample so the loudest point reaches ±1.0 without clipping.
    pub fn normalize(&self) -> Self {
        let max_amp = self
            .data
            .iter()
            .map(|&x| x.abs())
            .fold(0.0f32, |a, b| a.max(b));

        if max_amp < 0.0001 {
            // Sample is silent or nearly silent
            return self.clone();
        }

        let gain = 1.0 / max_amp;
        let normalized_data: Vec<f32> = self.data.iter().map(|&x| x * gain).collect();

        Self {
            data: Arc::new(normalized_data),
            channels: self.channels,
            sample_rate: self.sample_rate,
            duration: self.duration,
            num_frames: self.num_frames,
            loop_start: self.loop_start,
            loop_end: self.loop_end,
        }
    }

    /// Apply gain (volume) to the sample
    ///
    /// # Arguments
    /// * `gain` - Gain multiplier (1.0 = unchanged, 0.5 = half volume, 2.0 = double volume)
    pub fn with_gain(&self, gain: f32) -> Self {
        let gained_data: Vec<f32> = self.data.iter().map(|&x| x * gain).collect();

        Self {
            data: Arc::new(gained_data),
            channels: self.channels,
            sample_rate: self.sample_rate,
            duration: self.duration,
            num_frames: self.num_frames,
            loop_start: self.loop_start,
            loop_end: self.loop_end,
        }
    }

    /// Reverse the sample
    pub fn reverse(&self) -> Self {
        let mut reversed_data = Vec::with_capacity(self.data.len());

        match self.channels {
            1 => {
                // Mono: simple reverse
                reversed_data.extend(self.data.iter().rev());
            }
            2 => {
                // Stereo: reverse frames but keep L/R order within each frame
                for frame_idx in (0..self.num_frames).rev() {
                    let sample_idx = frame_idx * 2;
                    reversed_data.push(self.data[sample_idx]);
                    reversed_data.push(self.data[sample_idx + 1]);
                }
            }
            _ => {
                // Multi-channel: reverse frames
                let channels = self.channels as usize;
                for frame_idx in (0..self.num_frames).rev() {
                    let sample_idx = frame_idx * channels;
                    for ch in 0..channels {
                        reversed_data.push(self.data[sample_idx + ch]);
                    }
                }
            }
        }

        Self {
            data: Arc::new(reversed_data),
            channels: self.channels,
            sample_rate: self.sample_rate,
            duration: self.duration,
            num_frames: self.num_frames,
            loop_start: self.loop_start,
            loop_end: self.loop_end,
        }
    }

    /// Fade in over a duration
    ///
    /// # Arguments
    /// * `fade_duration` - Duration of fade in seconds
    pub fn with_fade_in(&self, fade_duration: f32) -> Self {
        let fade_frames = (fade_duration * self.sample_rate as f32) as usize;
        let fade_frames = fade_frames.min(self.num_frames);

        let mut faded_data = self.data.as_ref().clone();

        for frame_idx in 0..fade_frames {
            let gain = frame_idx as f32 / fade_frames as f32;
            let sample_idx = frame_idx * self.channels as usize;
            for ch in 0..self.channels as usize {
                faded_data[sample_idx + ch] *= gain;
            }
        }

        Self {
            data: Arc::new(faded_data),
            channels: self.channels,
            sample_rate: self.sample_rate,
            duration: self.duration,
            num_frames: self.num_frames,
            loop_start: self.loop_start,
            loop_end: self.loop_end,
        }
    }

    /// Fade out over a duration
    ///
    /// # Arguments
    /// * `fade_duration` - Duration of fade in seconds
    pub fn with_fade_out(&self, fade_duration: f32) -> Self {
        let fade_frames = (fade_duration * self.sample_rate as f32) as usize;
        let fade_frames = fade_frames.min(self.num_frames);
        let fade_start = self.num_frames - fade_frames;

        let mut faded_data = self.data.as_ref().clone();

        for frame_idx in fade_start..self.num_frames {
            let progress = (frame_idx - fade_start) as f32 / fade_frames as f32;
            let gain = 1.0 - progress;
            let sample_idx = frame_idx * self.channels as usize;
            for ch in 0..self.channels as usize {
                faded_data[sample_idx + ch] *= gain;
            }
        }

        Self {
            data: Arc::new(faded_data),
            channels: self.channels,
            sample_rate: self.sample_rate,
            duration: self.duration,
            num_frames: self.num_frames,
            loop_start: self.loop_start,
            loop_end: self.loop_end,
        }
    }

    /// Get the number of frames in the sample
    pub fn num_frames(&self) -> usize {
        self.num_frames
    }

    /// Check if the sample has looping enabled
    pub fn is_looping(&self) -> bool {
        self.loop_start.is_some() && self.loop_end.is_some()
    }

    /// Get the loop points in frames (if looping is enabled)
    pub fn loop_points(&self) -> Option<(usize, usize)> {
        match (self.loop_start, self.loop_end) {
            (Some(start), Some(end)) => Some((start, end)),
            _ => None,
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
            data: Arc::new(data.clone()),
            channels: 1,
            sample_rate: 44100,
            duration: 7.0 / 44100.0,
            num_frames: data.len(),
            loop_start: None,
            loop_end: None,
        };

        // Sample at time 0 (first sample)
        let (left, right) = sample.sample_at(0.0, 1.0);
        assert_eq!(left, 0.0);
        assert_eq!(right, 0.0); // Mono duplicated to both channels

        // Sample at peak
        let (left, right) = sample.sample_at(2.0 / 44100.0, 1.0);
        assert_eq!(left, 1.0);
        assert_eq!(right, 1.0);
    }

    #[test]
    fn test_stereo_sample_playback() {
        // Create a simple stereo sample (L, R, L, R...)
        let data = vec![1.0, 0.0, 0.5, 0.5, 0.0, 1.0];
        let sample = Sample {
            data: Arc::new(data.clone()),
            channels: 2,
            sample_rate: 44100,
            duration: 3.0 / 44100.0,
            num_frames: data.len() / 2,
            loop_start: None,
            loop_end: None,
        };

        // First frame
        let (left, right) = sample.sample_at(0.0, 1.0);
        assert_eq!(left, 1.0);
        assert_eq!(right, 0.0);

        // Second frame
        let (left, right) = sample.sample_at(1.0 / 44100.0, 1.0);
        assert_eq!(left, 0.5);
        assert_eq!(right, 0.5);
    }

    #[test]
    fn test_playback_rate() {
        let data = vec![0.0, 0.5, 1.0, 0.5];
        let sample = Sample {
            data: Arc::new(data.clone()),
            channels: 1,
            sample_rate: 44100,
            duration: 4.0 / 44100.0,
            num_frames: data.len(),
            loop_start: None,
            loop_end: None,
        };

        // Double speed: at time 1/44100, we should be at sample index 2
        let (left, _) = sample.sample_at(1.0 / 44100.0, 2.0);
        assert_eq!(left, 1.0);
    }

    #[test]
    fn test_bounds_checking() {
        let data = vec![1.0, 2.0, 3.0];
        let sample = Sample {
            data: Arc::new(data.clone()),
            channels: 1,
            sample_rate: 44100,
            duration: 3.0 / 44100.0,
            num_frames: data.len(),
            loop_start: None,
            loop_end: None,
        };

        // Beyond the end of sample
        let (left, right) = sample.sample_at(10.0, 1.0);
        assert_eq!(left, 0.0);
        assert_eq!(right, 0.0);
    }

    #[test]
    fn test_normalize() {
        let data = vec![0.0, 0.25, 0.5, -0.25];
        let sample = Sample {
            data: Arc::new(data),
            channels: 1,
            sample_rate: 44100,
            duration: 4.0 / 44100.0,
            num_frames: 4,
            loop_start: None,
            loop_end: None,
        };

        let normalized = sample.normalize();
        let (left, _) = normalized.sample_at(2.0 / 44100.0, 1.0);
        assert!((left - 1.0).abs() < 0.01); // Peak should be at 1.0
    }

    #[test]
    fn test_reverse() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let sample = Sample {
            data: Arc::new(data),
            channels: 1,
            sample_rate: 44100,
            duration: 4.0 / 44100.0,
            num_frames: 4,
            loop_start: None,
            loop_end: None,
        };

        let reversed = sample.reverse();
        let (left, _) = reversed.sample_at(0.0, 1.0);
        assert_eq!(left, 4.0); // First sample should be the last
    }

    #[test]
    fn test_slice() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let sample = Sample {
            data: Arc::new(data),
            channels: 1,
            sample_rate: 44100,
            duration: 5.0 / 44100.0,
            num_frames: 5,
            loop_start: None,
            loop_end: None,
        };

        let sliced = sample.slice_frames(1, 4).unwrap();
        assert_eq!(sliced.num_frames(), 3);
        let (left, _) = sliced.sample_at(0.0, 1.0);
        assert_eq!(left, 2.0); // First sample of slice
    }

    #[test]
    fn test_looping() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let sample = Sample {
            data: Arc::new(data),
            channels: 1,
            sample_rate: 44100,
            duration: 5.0 / 44100.0,
            num_frames: 5,
            loop_start: None,
            loop_end: None,
        }.with_loop_frames(1, 4).unwrap();

        assert!(sample.is_looping());
        assert_eq!(sample.loop_points(), Some((1, 4)));

        // Playing beyond loop end should wrap back
        let (left, _) = sample.sample_at(4.0 / 44100.0, 1.0);
        assert_eq!(left, 2.0); // Should wrap to loop_start (frame 1)
    }

    #[test]
    fn test_gain() {
        let data = vec![1.0, 0.5, -0.5, -1.0];
        let sample = Sample {
            data: Arc::new(data),
            channels: 1,
            sample_rate: 44100,
            duration: 4.0 / 44100.0,
            num_frames: 4,
            loop_start: None,
            loop_end: None,
        };

        let gained = sample.with_gain(0.5);
        let (left, _) = gained.sample_at(0.0, 1.0);
        assert_eq!(left, 0.5); // 1.0 * 0.5 = 0.5
    }
}
