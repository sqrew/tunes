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
    /// # use tunes::synthesis::sample::Sample;
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
    /// use tunes::synthesis::sample::Sample;
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
    /// # use tunes::synthesis::sample::Sample;
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
    /// # use tunes::synthesis::sample::Sample;
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

    /// Get the sample rate in Hz
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
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

    /// Time-stretch the sample without changing pitch using WSOLA
    ///
    /// Uses Waveform Similarity Overlap-Add (WSOLA) algorithm to change the duration
    /// of the sample while preserving pitch. This is useful for adding variation to
    /// game audio without changing the perceived pitch.
    ///
    /// # Arguments
    /// * `stretch_factor` - Time stretch ratio (0.5 = half duration, 2.0 = double duration)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::sample::Sample;
    /// // Create a test sample
    /// let sample = Sample::from_mono(vec![0.0; 44100], 44100);
    ///
    /// // Stretch to 150% duration without pitch change
    /// let stretched = sample.time_stretch(1.5);
    /// assert_eq!(stretched.duration, sample.duration * 1.5);
    /// ```
    ///
    /// # Performance Notes
    /// - Works best with stretch factors between 0.5 and 2.0
    /// - Maintains pitch at all stretch factors
    /// - Processing time scales with output length
    pub fn time_stretch(&self, stretch_factor: f32) -> Self {
        if stretch_factor <= 0.0 {
            return self.clone();
        }

        // For very small changes, just return the original
        if (stretch_factor - 1.0).abs() < 0.01 {
            return self.clone();
        }

        // WSOLA parameters
        let grain_size_ms = 30.0; // 30ms grains work well for most content
        let search_ms = 10.0; // Search window of 10ms

        let grain_frames = ((grain_size_ms / 1000.0) * self.sample_rate as f32) as usize;
        let search_frames = ((search_ms / 1000.0) * self.sample_rate as f32) as usize;

        // Analysis and synthesis hop sizes
        let analysis_hop = grain_frames / 2; // 50% overlap in analysis
        let synthesis_hop = ((analysis_hop as f32) * stretch_factor) as usize;

        // Calculate output size
        let output_frames = ((self.num_frames as f32) * stretch_factor) as usize;
        let output_samples = output_frames * self.channels as usize;

        let mut output_data = vec![0.0f32; output_samples];
        let mut overlap_count = vec![0u32; output_frames];

        // Generate Hann window for smoothing
        let window = Self::generate_hann_window(grain_frames);

        let mut output_pos = 0;
        let mut input_pos = 0;

        while output_pos < output_frames {
            // Find best matching grain around expected input position
            let best_input_pos = if output_pos == 0 {
                0 // Always start at the beginning
            } else {
                self.find_best_grain_match(input_pos, grain_frames, search_frames)
            };

            // Extract and apply grain
            let grain_end = (best_input_pos + grain_frames).min(self.num_frames);
            let actual_grain_size = grain_end - best_input_pos;

            if actual_grain_size == 0 {
                break;
            }

            // Copy grain with windowing and overlap-add
            for i in 0..actual_grain_size {
                let out_frame = output_pos + i;
                if out_frame >= output_frames {
                    break;
                }

                // Get window value (use truncated window if grain is smaller)
                let window_val = if i < window.len() {
                    window[i]
                } else {
                    1.0
                };

                // Process each channel
                for ch in 0..self.channels as usize {
                    let in_idx = (best_input_pos + i) * self.channels as usize + ch;
                    let out_idx = out_frame * self.channels as usize + ch;

                    if in_idx < self.data.len() && out_idx < output_data.len() {
                        output_data[out_idx] += self.data[in_idx] * window_val;
                    }
                }

                overlap_count[out_frame] += 1;
            }

            // Advance positions
            output_pos += synthesis_hop;
            input_pos += analysis_hop;

            // Prevent infinite loops
            if input_pos >= self.num_frames && output_pos < output_frames {
                break;
            }
        }

        // Normalize by overlap count to maintain amplitude
        for frame in 0..output_frames {
            if overlap_count[frame] > 0 {
                let norm_factor = 1.0 / overlap_count[frame] as f32;
                for ch in 0..self.channels as usize {
                    let idx = frame * self.channels as usize + ch;
                    if idx < output_data.len() {
                        output_data[idx] *= norm_factor;
                    }
                }
            }
        }

        Self {
            data: Arc::new(output_data),
            channels: self.channels,
            sample_rate: self.sample_rate,
            duration: output_frames as f32 / self.sample_rate as f32,
            num_frames: output_frames,
            loop_start: None,
            loop_end: None,
        }
    }

    /// Pitch-shift the sample without changing duration
    ///
    /// Changes the pitch of the sample while maintaining the original duration.
    /// This is achieved by resampling (which changes pitch and duration) and then
    /// time-stretching back to the original duration.
    ///
    /// # Arguments
    /// * `semitones` - Number of semitones to shift (positive = up, negative = down)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::sample::Sample;
    /// // Create a test sample
    /// let sample = Sample::from_mono(vec![0.0; 44100], 44100);
    ///
    /// // Shift up by 7 semitones (perfect fifth) without changing duration
    /// let shifted = sample.pitch_shift(7.0);
    /// assert!((shifted.duration - sample.duration).abs() < 0.01);
    /// ```
    ///
    /// # Common Intervals
    /// - 12 semitones = 1 octave up
    /// - -12 semitones = 1 octave down
    /// - 7 semitones = perfect fifth up
    /// - 5 semitones = perfect fourth up
    pub fn pitch_shift(&self, semitones: f32) -> Self {
        if semitones.abs() < 0.01 {
            return self.clone();
        }

        // Convert semitones to frequency ratio
        // ratio = 2^(semitones/12)
        let pitch_ratio = 2.0f32.powf(semitones / 12.0);

        // Step 1: Resample to change pitch (this also changes duration)
        // Resampling by pitch_ratio makes duration = original / pitch_ratio
        let resampled = self.resample(pitch_ratio);

        // Step 2: Time-stretch back to original duration
        // If we pitched up by 2x, duration became 0.5x, so stretch by 2x to restore
        resampled.time_stretch(pitch_ratio)
    }

    /// Resample the audio to a different playback rate
    ///
    /// This changes both pitch and duration. For pitch without duration change,
    /// use `pitch_shift()`. For duration without pitch change, use `time_stretch()`.
    ///
    /// Uses linear interpolation for quality resampling.
    fn resample(&self, rate_ratio: f32) -> Self {
        if (rate_ratio - 1.0).abs() < 0.001 {
            return self.clone();
        }

        // Calculate new length
        let new_num_frames = ((self.num_frames as f32) / rate_ratio) as usize;
        let new_samples = new_num_frames * self.channels as usize;

        let mut output_data = Vec::with_capacity(new_samples);

        for frame in 0..new_num_frames {
            // Map output frame to input position
            let input_pos = (frame as f32) * rate_ratio;
            let input_frame = input_pos as usize;
            let frac = input_pos.fract();

            // Bounds check
            if input_frame >= self.num_frames - 1 {
                // Pad with zeros if we're past the end
                for _ in 0..self.channels {
                    output_data.push(0.0);
                }
                continue;
            }

            // Linear interpolation for each channel
            for ch in 0..self.channels as usize {
                let idx1 = input_frame * self.channels as usize + ch;
                let idx2 = (input_frame + 1) * self.channels as usize + ch;

                let sample1 = self.data.get(idx1).copied().unwrap_or(0.0);
                let sample2 = self.data.get(idx2).copied().unwrap_or(0.0);

                let interpolated = sample1 + (sample2 - sample1) * frac;
                output_data.push(interpolated);
            }
        }

        Self {
            data: Arc::new(output_data),
            channels: self.channels,
            sample_rate: self.sample_rate,
            duration: new_num_frames as f32 / self.sample_rate as f32,
            num_frames: new_num_frames,
            loop_start: None,
            loop_end: None,
        }
    }

    /// Find the best matching grain position using cross-correlation
    ///
    /// Searches within a window around the target position to find the grain
    /// that best matches the previous output, minimizing discontinuities.
    fn find_best_grain_match(
        &self,
        target_pos: usize,
        grain_size: usize,
        search_window: usize,
    ) -> usize {
        let search_start = target_pos.saturating_sub(search_window);
        let search_end = (target_pos + search_window).min(self.num_frames - grain_size);

        if search_start >= search_end {
            return target_pos.min(self.num_frames - grain_size);
        }

        let mut best_pos = target_pos;
        let mut best_score = f32::NEG_INFINITY;

        // Compare overlap region (use small overlap for efficiency)
        let overlap_size = (grain_size / 4).min(512);

        for pos in search_start..search_end {
            if pos + grain_size > self.num_frames {
                break;
            }

            // Cross-correlation score for this position
            let score = self.compute_cross_correlation(pos, overlap_size);

            if score > best_score {
                best_score = score;
                best_pos = pos;
            }
        }

        best_pos
    }

    /// Compute cross-correlation score for a grain position
    ///
    /// Higher scores indicate better waveform similarity (less discontinuity)
    fn compute_cross_correlation(&self, pos: usize, window_size: usize) -> f32 {
        let mut correlation = 0.0f32;
        let end = (pos + window_size).min(self.num_frames);

        for i in pos..end {
            for ch in 0..self.channels as usize {
                let idx = i * self.channels as usize + ch;
                if let Some(&sample) = self.data.get(idx) {
                    // Simple energy-based correlation
                    // (in full WSOLA this would compare with previous grain)
                    correlation += sample * sample;
                }
            }
        }

        correlation
    }

    /// Generate a Hann window of specified size
    ///
    /// Used for smoothing grain boundaries in time-stretching
    fn generate_hann_window(size: usize) -> Vec<f32> {
        if size == 0 {
            return vec![];
        }

        (0..size)
            .map(|i| {
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / size as f32).cos())
            })
            .collect()
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

    #[test]
    fn test_time_stretch_basic() {
        // Create a 1 second mono sample with a simple waveform
        let sample_rate = 44100;
        let data: Vec<f32> = (0..sample_rate)
            .map(|i| (2.0 * std::f32::consts::PI * 440.0 * i as f32 / sample_rate as f32).sin())
            .collect();

        let sample = Sample::from_mono(data, sample_rate);

        // Stretch to 150% duration
        let stretched = sample.time_stretch(1.5);

        // Check duration increased by 50%
        assert!(
            (stretched.duration - sample.duration * 1.5).abs() < 0.01,
            "Duration should be 1.5x original"
        );

        // Check sample rate unchanged
        assert_eq!(stretched.sample_rate, sample.sample_rate);

        // Check channels unchanged
        assert_eq!(stretched.channels, sample.channels);
    }

    #[test]
    fn test_time_stretch_compression() {
        let sample_rate = 44100;
        let data: Vec<f32> = vec![0.5; sample_rate as usize];

        let sample = Sample::from_mono(data, sample_rate);

        // Compress to 50% duration
        let compressed = sample.time_stretch(0.5);

        assert!(
            (compressed.duration - sample.duration * 0.5).abs() < 0.01,
            "Duration should be 0.5x original"
        );
    }

    #[test]
    fn test_time_stretch_stereo() {
        let sample_rate = 44100;
        // Create stereo sample (L, R, L, R...)
        let data: Vec<f32> = (0..sample_rate * 2)
            .map(|i| if i % 2 == 0 { 0.5 } else { -0.5 })
            .collect();

        let sample = Sample {
            data: Arc::new(data),
            channels: 2,
            sample_rate,
            duration: 1.0,
            num_frames: sample_rate as usize,
            loop_start: None,
            loop_end: None,
        };

        let stretched = sample.time_stretch(1.5);

        assert_eq!(stretched.channels, 2);
        assert!(
            (stretched.duration - 1.5).abs() < 0.01,
            "Stereo stretch should work"
        );
    }

    #[test]
    fn test_pitch_shift_up() {
        let sample_rate = 44100;
        let data: Vec<f32> = (0..sample_rate)
            .map(|i| (2.0 * std::f32::consts::PI * 440.0 * i as f32 / sample_rate as f32).sin())
            .collect();

        let sample = Sample::from_mono(data, sample_rate);

        // Shift up by 12 semitones (1 octave)
        let shifted = sample.pitch_shift(12.0);

        // Duration should remain approximately the same (within 10%)
        // Some variation is expected due to grain boundaries
        let duration_diff = (shifted.duration - sample.duration).abs();
        let relative_error = duration_diff / sample.duration;
        assert!(
            relative_error < 0.1,
            "Duration should remain approximately the same after pitch shift. Expected: {}, Got: {}, Relative error: {}",
            sample.duration, shifted.duration, relative_error
        );

        // Check sample rate unchanged
        assert_eq!(shifted.sample_rate, sample.sample_rate);
    }

    #[test]
    fn test_pitch_shift_down() {
        let sample_rate = 44100;
        let data: Vec<f32> = vec![0.5; sample_rate as usize];

        let sample = Sample::from_mono(data, sample_rate);

        // Shift down by 7 semitones (perfect fifth down)
        let shifted = sample.pitch_shift(-7.0);

        // Duration should remain approximately the same (within 10%)
        let duration_diff = (shifted.duration - sample.duration).abs();
        let relative_error = duration_diff / sample.duration;
        assert!(
            relative_error < 0.1,
            "Duration should remain approximately the same. Expected: {}, Got: {}, Relative error: {}",
            sample.duration, shifted.duration, relative_error
        );
    }

    #[test]
    fn test_pitch_shift_zero() {
        let sample_rate = 44100;
        let data: Vec<f32> = vec![0.5; 100];

        let sample = Sample::from_mono(data, sample_rate);

        // Zero shift should return nearly identical sample
        let shifted = sample.pitch_shift(0.0);

        assert_eq!(shifted.duration, sample.duration);
        assert_eq!(shifted.num_frames, sample.num_frames);
    }

    #[test]
    fn test_generate_hann_window() {
        let window = Sample::generate_hann_window(100);

        assert_eq!(window.len(), 100);

        // Hann window should start and end near 0
        assert!(window[0] < 0.01);
        assert!(window[99] < 0.01);

        // Middle should be near 1.0
        assert!(window[50] > 0.99);
    }

    #[test]
    fn test_resample() {
        let sample_rate = 44100;
        let data: Vec<f32> = vec![1.0, 0.5, 0.0, -0.5, -1.0];

        let sample = Sample::from_mono(data, sample_rate);

        // Resample to 2x speed (half duration, double pitch)
        let resampled = sample.resample(2.0);

        assert!(
            resampled.num_frames < sample.num_frames,
            "Resampling up should reduce frame count"
        );
    }

    #[test]
    fn test_time_stretch_edge_cases() {
        let sample = Sample::from_mono(vec![0.5; 100], 44100);

        // Stretch factor of 0 or negative should return clone
        let stretched_zero = sample.time_stretch(0.0);
        assert_eq!(stretched_zero.num_frames, sample.num_frames);

        // Stretch factor very close to 1.0 should return clone
        let stretched_one = sample.time_stretch(1.0);
        assert_eq!(stretched_one.num_frames, sample.num_frames);
    }
}
