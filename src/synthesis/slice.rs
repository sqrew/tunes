/// Sample slicing module for dividing audio samples into playable segments
///
/// This module provides tools for slicing samples in various ways:
/// - Equal divisions
/// - Time-based slicing
/// - Transient/onset detection
/// - Beat-based slicing
///
/// Slices are lightweight references to regions of the parent sample,
/// avoiding unnecessary copying of audio data.

use crate::error::{Result, TunesError};
use crate::synthesis::sample::Sample;
use std::sync::Arc;

/// A lightweight reference to a slice of a sample
///
/// Instead of copying audio data, SampleSlice stores the parent sample
/// and the start/end frame indices. This makes slicing very efficient.
#[derive(Debug, Clone)]
pub struct SampleSlice {
    /// Reference to the parent sample
    pub sample: Arc<Sample>,

    /// Start frame index (inclusive)
    pub start_frame: usize,

    /// End frame index (exclusive)
    pub end_frame: usize,

    /// Duration of this slice in seconds
    pub duration: f32,

    /// Slice index (useful for identifying which slice this is)
    pub index: usize,
}

impl SampleSlice {
    /// Create a new sample slice
    pub fn new(sample: Arc<Sample>, start_frame: usize, end_frame: usize, index: usize) -> Result<Self> {
        if start_frame >= end_frame {
            return Err(TunesError::InvalidAudioFormat(
                "Start frame must be before end frame".to_string(),
            ));
        }

        let num_frames = sample.num_frames();
        if end_frame > num_frames {
            return Err(TunesError::InvalidAudioFormat(format!(
                "End frame {} exceeds sample length {}",
                end_frame, num_frames
            )));
        }

        let slice_frames = end_frame - start_frame;
        let duration = slice_frames as f32 / sample.sample_rate() as f32;

        Ok(Self {
            sample,
            start_frame,
            end_frame,
            duration,
            index,
        })
    }

    /// Get the start time of this slice in the parent sample (seconds)
    pub fn start_time(&self) -> f32 {
        self.start_frame as f32 / self.sample.sample_rate() as f32
    }

    /// Get the end time of this slice in the parent sample (seconds)
    pub fn end_time(&self) -> f32 {
        self.end_frame as f32 / self.sample.sample_rate() as f32
    }

    /// Get the number of frames in this slice
    pub fn num_frames(&self) -> usize {
        self.end_frame - self.start_frame
    }

    /// Get an audio sample at a specific time within this slice
    ///
    /// # Arguments
    /// * `time` - Time in seconds from the START of this slice (not the parent sample)
    /// * `playback_rate` - Speed multiplier (1.0 = normal)
    ///
    /// Returns (left, right) channels
    #[inline]
    pub fn sample_at(&self, time: f32, playback_rate: f32) -> (f32, f32) {
        // Convert slice-local time to parent sample time
        let slice_start_time = self.start_time();
        let parent_time = slice_start_time + (time * playback_rate);

        // Clamp to slice bounds
        let slice_end_time = self.end_time();
        if parent_time >= slice_end_time {
            return (0.0, 0.0);
        }

        self.sample.sample_at_interpolated(parent_time, 1.0)
    }

    /// Convert this slice to a new independent Sample
    ///
    /// This creates a copy of the audio data for this slice.
    /// Useful if you want to apply effects or manipulations to just this slice.
    pub fn to_sample(&self) -> Result<Sample> {
        self.sample.slice_frames(self.start_frame, self.end_frame)
    }
}

/// Methods for slicing samples into multiple segments
impl Sample {
    /// Slice the sample into N equal parts
    ///
    /// # Arguments
    /// * `num_slices` - Number of equal slices to create
    ///
    /// # Returns
    /// Vector of SampleSlice references
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::synthesis::sample::Sample;
    /// let sample = Sample::from_wav("drumloop.wav")?;
    /// let slices = sample.slice_equal(16)?; // 16 equal slices
    ///
    /// // Play slice 4
    /// // comp.track("slice_4").sample_slice(&slices[4], 1.0);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn slice_equal(&self, num_slices: usize) -> Result<Vec<SampleSlice>> {
        if num_slices == 0 {
            return Err(TunesError::InvalidAudioFormat(
                "Number of slices must be greater than 0".to_string(),
            ));
        }

        let num_frames = self.num_frames();
        let frames_per_slice = num_frames / num_slices;
        let parent = Arc::new(self.clone());
        let mut slices = Vec::with_capacity(num_slices);

        for i in 0..num_slices {
            let start_frame = i * frames_per_slice;
            let end_frame = if i == num_slices - 1 {
                num_frames // Last slice gets any remainder frames
            } else {
                (i + 1) * frames_per_slice
            };

            slices.push(SampleSlice::new(parent.clone(), start_frame, end_frame, i)?);
        }

        Ok(slices)
    }

    /// Slice the sample at specific time points
    ///
    /// # Arguments
    /// * `times` - Slice points in seconds. Will create len(times) + 1 slices
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::synthesis::sample::Sample;
    /// let sample = Sample::from_wav("phrase.wav")?;
    ///
    /// // Slice at 0.5s, 1.2s, and 2.5s
    /// // Creates 4 slices: [0.0-0.5], [0.5-1.2], [1.2-2.5], [2.5-end]
    /// let slices = sample.slice_at_times(&[0.5, 1.2, 2.5])?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn slice_at_times(&self, times: &[f32]) -> Result<Vec<SampleSlice>> {
        if times.is_empty() {
            return Err(TunesError::InvalidAudioFormat(
                "Must provide at least one slice time".to_string(),
            ));
        }

        // Convert times to frames and verify they're in order
        let mut frames: Vec<usize> = times
            .iter()
            .map(|&t| (t * self.sample_rate as f32) as usize)
            .collect();

        // Check sorted
        for i in 1..frames.len() {
            if frames[i] <= frames[i - 1] {
                return Err(TunesError::InvalidAudioFormat(
                    "Slice times must be in ascending order".to_string(),
                ));
            }
        }

        // Add start and end boundaries
        let mut all_frames = vec![0];
        all_frames.extend(frames);
        all_frames.push(self.num_frames());

        // Create slices between each pair of boundaries
        let parent = Arc::new(self.clone());
        let mut slices = Vec::with_capacity(all_frames.len() - 1);

        for i in 0..all_frames.len() - 1 {
            let start = all_frames[i];
            let end = all_frames[i + 1];
            slices.push(SampleSlice::new(parent.clone(), start, end, i)?);
        }

        Ok(slices)
    }

    /// Slice the sample at frame indices
    ///
    /// Like `slice_at_times()` but uses frame indices instead of seconds.
    ///
    /// # Arguments
    /// * `frame_indices` - Slice points in frames
    pub fn slice_at_frames(&self, frame_indices: &[usize]) -> Result<Vec<SampleSlice>> {
        if frame_indices.is_empty() {
            return Err(TunesError::InvalidAudioFormat(
                "Must provide at least one frame index".to_string(),
            ));
        }

        // Check sorted
        for i in 1..frame_indices.len() {
            if frame_indices[i] <= frame_indices[i - 1] {
                return Err(TunesError::InvalidAudioFormat(
                    "Frame indices must be in ascending order".to_string(),
                ));
            }
        }

        // Add start and end boundaries
        let mut all_frames = vec![0];
        all_frames.extend_from_slice(frame_indices);
        all_frames.push(self.num_frames());

        // Create slices
        let parent = Arc::new(self.clone());
        let mut slices = Vec::with_capacity(all_frames.len() - 1);

        for i in 0..all_frames.len() - 1 {
            let start = all_frames[i];
            let end = all_frames[i + 1];
            slices.push(SampleSlice::new(parent.clone(), start, end, i)?);
        }

        Ok(slices)
    }

    /// Detect transients (onsets) in the audio signal
    ///
    /// Uses energy-based onset detection to find sudden increases in amplitude.
    /// Useful for automatically finding hit points in drum loops or percussion.
    ///
    /// # Arguments
    /// * `threshold` - Sensitivity (0.0-1.0). Lower = more sensitive, higher = fewer detections
    /// * `min_gap_ms` - Minimum time between detections in milliseconds (prevents duplicates)
    ///
    /// # Returns
    /// Vector of frame indices where transients were detected
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::synthesis::sample::Sample;
    /// let sample = Sample::from_wav("drumloop.wav")?;
    ///
    /// // Detect transients with moderate sensitivity
    /// let transient_frames = sample.detect_transients(0.3, 50.0)?;
    ///
    /// // Slice at the detected transients
    /// let slices = sample.slice_at_frames(&transient_frames)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn detect_transients(&self, threshold: f32, min_gap_ms: f32) -> Result<Vec<usize>> {
        if threshold < 0.0 || threshold > 1.0 {
            return Err(TunesError::InvalidAudioFormat(
                "Threshold must be between 0.0 and 1.0".to_string(),
            ));
        }

        // Convert min gap to frames
        let min_gap_frames = ((min_gap_ms / 1000.0) * self.sample_rate as f32) as usize;

        // Window size for energy calculation (in frames)
        let window_size = (0.02 * self.sample_rate as f32) as usize; // 20ms window
        let hop_size = window_size / 4; // 75% overlap

        // Calculate energy envelope
        let num_frames = self.num_frames();
        let num_windows = (num_frames - window_size) / hop_size;
        let mut energies = Vec::with_capacity(num_windows);

        for i in 0..num_windows {
            let start_frame = i * hop_size;
            let mut energy = 0.0f32;

            // Sum squared amplitude over window
            for frame in start_frame..(start_frame + window_size).min(num_frames) {
                let sample_idx = frame * self.channels as usize;

                for ch in 0..self.channels as usize {
                    if let Some(&sample) = self.data.get(sample_idx + ch) {
                        energy += sample * sample;
                    }
                }
            }

            energies.push(energy / (window_size * self.channels as usize) as f32);
        }

        // Find local maxima in energy that exceed threshold
        let max_energy = energies.iter().cloned().fold(0.0f32, f32::max);
        let threshold_energy = max_energy * threshold;

        let mut transients = Vec::new();
        let mut last_transient_frame = 0;

        for i in 1..energies.len() - 1 {
            let prev = energies[i - 1];
            let curr = energies[i];
            let next = energies[i + 1];

            // Local maximum above threshold
            if curr > prev && curr > next && curr > threshold_energy {
                let frame = i * hop_size;

                // Check minimum gap
                if frame - last_transient_frame >= min_gap_frames {
                    transients.push(frame);
                    last_transient_frame = frame;
                }
            }
        }

        Ok(transients)
    }

    /// Slice the sample by detecting transients
    ///
    /// Convenience method that combines `detect_transients()` and `slice_at_frames()`.
    ///
    /// # Arguments
    /// * `threshold` - Detection sensitivity (0.0-1.0)
    /// * `min_gap_ms` - Minimum time between slices in milliseconds
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::synthesis::sample::Sample;
    /// let sample = Sample::from_wav("drumloop.wav")?;
    /// let slices = sample.slice_by_transients(0.3, 50.0)?;
    ///
    /// println!("Detected {} hits in the loop", slices.len());
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn slice_by_transients(&self, threshold: f32, min_gap_ms: f32) -> Result<Vec<SampleSlice>> {
        let transient_frames = self.detect_transients(threshold, min_gap_ms)?;

        if transient_frames.is_empty() {
            // No transients detected - return the whole sample as one slice
            let parent = Arc::new(self.clone());
            return Ok(vec![SampleSlice::new(parent, 0, self.num_frames(), 0)?]);
        }

        self.slice_at_frames(&transient_frames)
    }

    /// Slice the sample at beat divisions
    ///
    /// Divides the sample based on a given BPM and beat subdivision.
    ///
    /// # Arguments
    /// * `bpm` - Tempo in beats per minute
    /// * `beats_per_slice` - Beat subdivision (0.25 = 16th notes, 0.5 = 8ths, 1.0 = quarters, etc.)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::synthesis::sample::Sample;
    /// let sample = Sample::from_wav("loop_140bpm.wav")?;
    ///
    /// // Slice into 16th notes at 140 BPM
    /// let slices = sample.slice_at_beats(140.0, 0.25)?;
    ///
    /// // Slice into quarter notes
    /// let quarter_slices = sample.slice_at_beats(140.0, 1.0)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn slice_at_beats(&self, bpm: f32, beats_per_slice: f32) -> Result<Vec<SampleSlice>> {
        if bpm <= 0.0 {
            return Err(TunesError::InvalidAudioFormat(
                "BPM must be greater than 0".to_string(),
            ));
        }

        if beats_per_slice <= 0.0 {
            return Err(TunesError::InvalidAudioFormat(
                "Beats per slice must be greater than 0".to_string(),
            ));
        }

        // Calculate time per slice
        let seconds_per_beat = 60.0 / bpm;
        let seconds_per_slice = seconds_per_beat * beats_per_slice;

        // Calculate number of slices
        let num_slices = (self.duration / seconds_per_slice).ceil() as usize;

        // Generate slice times
        let mut times = Vec::with_capacity(num_slices - 1);
        for i in 1..num_slices {
            times.push(i as f32 * seconds_per_slice);
        }

        self.slice_at_times(&times)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_sample() -> Sample {
        // Create a simple 1-second mono test sample at 44.1kHz
        let sample_rate: u32 = 44100;
        let duration = 1.0;
        let num_samples = sample_rate as usize;

        // Generate a simple waveform
        let mut samples = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            samples.push((t * 440.0 * 2.0 * std::f32::consts::PI).sin() * 0.5);
        }

        Sample::from_mono(samples, sample_rate)
    }

    #[test]
    fn test_slice_equal() {
        let sample = create_test_sample();
        let slices = sample.slice_equal(4).unwrap();

        assert_eq!(slices.len(), 4);

        // Each slice should be roughly 0.25 seconds
        for (i, slice) in slices.iter().enumerate() {
            assert!(slice.duration >= 0.24 && slice.duration <= 0.26,
                "Slice {} duration was {}", i, slice.duration);
            assert_eq!(slice.index, i);
        }
    }

    #[test]
    fn test_slice_at_times() {
        let sample = create_test_sample();
        let slices = sample.slice_at_times(&[0.25, 0.5, 0.75]).unwrap();

        assert_eq!(slices.len(), 4);

        // Verify slice boundaries
        assert!(slices[0].duration >= 0.24 && slices[0].duration <= 0.26);
        assert!(slices[1].duration >= 0.24 && slices[1].duration <= 0.26);
    }

    #[test]
    fn test_slice_at_beats() {
        let sample = create_test_sample();

        // At 120 BPM, 1 beat = 0.5 seconds
        // So a 1-second sample should have 2 slices at 1.0 beats per slice
        let slices = sample.slice_at_beats(120.0, 1.0).unwrap();

        assert_eq!(slices.len(), 2);
    }

    #[test]
    fn test_detect_transients() {
        let sample = create_test_sample();
        let transients = sample.detect_transients(0.1, 50.0).unwrap();

        // Should detect at least one transient (even in a sine wave)
        assert!(!transients.is_empty());
    }

    #[test]
    fn test_sample_slice_to_sample() {
        let sample = create_test_sample();
        let slices = sample.slice_equal(4).unwrap();

        // Convert first slice to independent sample
        let slice_sample = slices[0].to_sample().unwrap();

        // Verify it has the right duration
        assert!(slice_sample.duration >= 0.24 && slice_sample.duration <= 0.26);
    }
}
