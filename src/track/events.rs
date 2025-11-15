//! Audio event types and implementations
//!
//! This module contains all event types used in tracks:
//! - AudioEvent enum
//! - NoteEvent, DrumEvent, SampleEvent
//! - TempoChangeEvent, TimeSignatureEvent, KeySignatureEvent

use crate::instruments::drums::DrumType;
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter_envelope::FilterEnvelope;
use crate::synthesis::fm_synthesis::FMParams;
use crate::synthesis::sample::Sample;
use crate::synthesis::spatial::SpatialPosition;
use crate::synthesis::waveform::Waveform;
use crate::theory::key_signature::KeySignature;

/// Represents different types of audio events
///
/// Audio events are the fundamental building blocks of a track. Each event has
/// a start time and contains specific parameters for its type (pitch, duration, etc.).
#[derive(Debug, Clone)]
pub enum AudioEvent {
    Note(NoteEvent),
    Drum(DrumEvent),
    Sample(SampleEvent),
    TempoChange(TempoChangeEvent),
    TimeSignature(TimeSignatureEvent),
    KeySignature(KeySignatureEvent),
}

impl AudioEvent {
    /// Get the start time of this event
    #[inline]
    pub fn start_time(&self) -> f32 {
        match self {
            AudioEvent::Note(note) => note.start_time,
            AudioEvent::Drum(drum) => drum.start_time,
            AudioEvent::Sample(sample) => sample.start_time,
            AudioEvent::TempoChange(tempo) => tempo.start_time,
            AudioEvent::TimeSignature(time_sig) => time_sig.start_time,
            AudioEvent::KeySignature(key_sig) => key_sig.start_time,
        }
    }

    /// Get the end time of this event (including release for notes)
    #[inline]
    pub fn end_time(&self) -> f32 {
        match self {
            AudioEvent::Note(note) => {
                let total_duration = note.envelope.total_duration(note.duration);
                note.start_time + total_duration
            }
            AudioEvent::Drum(drum) => drum.start_time + drum.drum_type.duration(),
            AudioEvent::Sample(sample) => {
                sample.start_time + (sample.sample.duration / sample.playback_rate)
            }
            AudioEvent::TempoChange(tempo) => tempo.start_time,
            AudioEvent::TimeSignature(time_sig) => time_sig.start_time,
            AudioEvent::KeySignature(key_sig) => key_sig.start_time,
        }
    }
}

/// Represents a note event with timing information
#[derive(Debug, Clone)]
pub struct NoteEvent {
    pub frequencies: [f32; 8], // Support up to 8 simultaneous frequencies
    pub num_freqs: usize,
    pub start_time: f32,    // When to start playing (in seconds from track start)
    pub duration: f32,      // How long to play (in seconds)
    pub waveform: Waveform, // Waveform type to use (ignored if FM is enabled or custom wavetable is set)
    pub envelope: Envelope, // ADSR envelope for amplitude
    pub filter_envelope: FilterEnvelope, // ADSR envelope for filter cutoff
    pub fm_params: FMParams, // FM synthesis parameters (mod_index=0 disables FM)
    pub pitch_bend_semitones: f32, // Pitch bend amount in semitones (0.0 = no bend)
    pub custom_wavetable: Option<crate::synthesis::wavetable::Wavetable>, // Custom wavetable (overrides waveform if present)
    pub velocity: f32, // Note velocity (0.0 to 1.0), affects MIDI export and can be used for expression
    pub spatial_position: Option<SpatialPosition>, // 3D spatial position for spatial audio (None = no spatial processing)
}

/// Represents a drum hit event
#[derive(Debug, Clone, Copy)]
pub struct DrumEvent {
    pub drum_type: DrumType,
    pub start_time: f32,
    pub spatial_position: Option<SpatialPosition>, // 3D spatial position for spatial audio
}

/// Represents a sample playback event
#[derive(Debug, Clone)]
pub struct SampleEvent {
    pub sample: Sample,
    pub start_time: f32,
    pub playback_rate: f32, // 1.0 = normal speed, 2.0 = double speed, 0.5 = half speed
    pub volume: f32,        // 0.0 to 1.0
    pub spatial_position: Option<SpatialPosition>, // 3D spatial position for spatial audio
}

/// Represents a tempo change event
#[derive(Debug, Clone, Copy)]
pub struct TempoChangeEvent {
    pub start_time: f32,
    pub bpm: f32,
}

/// Represents a time signature change event
#[derive(Debug, Clone, Copy)]
pub struct TimeSignatureEvent {
    pub start_time: f32,
    pub numerator: u8,   // Top number (e.g., 3 in 3/4)
    pub denominator: u8, // Bottom number (e.g., 4 in 3/4)
}

/// Represents a key signature change event
#[derive(Debug, Clone, Copy)]
pub struct KeySignatureEvent {
    pub start_time: f32,
    pub key_signature: KeySignature,
}

impl SampleEvent {
    /// Create a sample playback event with default settings
    ///
    /// # Arguments
    /// * `sample` - The audio sample to play
    /// * `start_time` - When to start playback (in seconds from track start)
    pub fn new(sample: Sample, start_time: f32) -> Self {
        Self {
            sample,
            start_time,
            playback_rate: 1.0,
            volume: 1.0,
            spatial_position: None,
        }
    }

    /// Set playback rate (pitch shifting)
    ///
    /// # Arguments
    /// * `rate` - Playback speed multiplier (1.0 = normal, 2.0 = double speed/octave up, 0.5 = half speed/octave down)
    pub fn with_playback_rate(mut self, rate: f32) -> Self {
        self.playback_rate = rate;
        self
    }

    /// Set sample volume
    ///
    /// # Arguments
    /// * `volume` - Volume level 0.0 (silent) to 1.0 (full)
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 1.0);
        self
    }
}

impl NoteEvent {
    /// Create a simple note event with sine wave
    ///
    /// Creates a note with default settings: sine waveform, default envelope,
    /// no pitch bend, no FM synthesis, no custom wavetable.
    ///
    /// # Arguments
    /// * `frequencies` - Note frequencies in Hz (up to 8 for chords)
    /// * `start_time` - When to start playing (in seconds from track start)
    /// * `duration` - How long to sustain the note (in seconds)
    pub fn new(frequencies: &[f32], start_time: f32, duration: f32) -> Self {
        Self::with_waveform(frequencies, start_time, duration, Waveform::Sine)
    }

    /// Create a note event with a specific waveform
    ///
    /// # Arguments
    /// * `frequencies` - Note frequencies in Hz (up to 8 for chords)
    /// * `start_time` - When to start playing (in seconds from track start)
    /// * `duration` - How long to sustain the note (in seconds)
    /// * `waveform` - Waveform type (Sine, Square, Saw, Triangle)
    pub fn with_waveform(
        frequencies: &[f32],
        start_time: f32,
        duration: f32,
        waveform: Waveform,
    ) -> Self {
        Self::with_waveform_and_envelope(
            frequencies,
            start_time,
            duration,
            waveform,
            Envelope::default(),
        )
    }

    /// Create a note event with waveform and ADSR envelope
    ///
    /// # Arguments
    /// * `frequencies` - Note frequencies in Hz (up to 8 for chords)
    /// * `start_time` - When to start playing (in seconds from track start)
    /// * `duration` - How long to sustain the note (in seconds)
    /// * `waveform` - Waveform type (Sine, Square, Saw, Triangle)
    /// * `envelope` - ADSR amplitude envelope
    pub fn with_waveform_and_envelope(
        frequencies: &[f32],
        start_time: f32,
        duration: f32,
        waveform: Waveform,
        envelope: Envelope,
    ) -> Self {
        Self::with_waveform_envelope_and_bend(
            frequencies,
            start_time,
            duration,
            waveform,
            envelope,
            0.0,
        )
    }

    /// Create a note event with waveform, envelope, and pitch bend
    ///
    /// # Arguments
    /// * `frequencies` - Note frequencies in Hz (up to 8 for chords)
    /// * `start_time` - When to start playing (in seconds from track start)
    /// * `duration` - How long to sustain the note (in seconds)
    /// * `waveform` - Waveform type (Sine, Square, Saw, Triangle)
    /// * `envelope` - ADSR amplitude envelope
    /// * `pitch_bend_semitones` - Pitch bend in semitones (e.g., 2.0 for up 2 semitones)
    pub fn with_waveform_envelope_and_bend(
        frequencies: &[f32],
        start_time: f32,
        duration: f32,
        waveform: Waveform,
        envelope: Envelope,
        pitch_bend_semitones: f32,
    ) -> Self {
        Self::with_full_params(
            frequencies,
            start_time,
            duration,
            waveform,
            envelope,
            FilterEnvelope::default(),
            pitch_bend_semitones,
        )
    }

    /// Create a note event with waveform, envelope, filter envelope, and pitch bend
    ///
    /// # Arguments
    /// * `frequencies` - Note frequencies in Hz (up to 8 for chords)
    /// * `start_time` - When to start playing (in seconds from track start)
    /// * `duration` - How long to sustain the note (in seconds)
    /// * `waveform` - Waveform type (Sine, Square, Saw, Triangle)
    /// * `envelope` - ADSR amplitude envelope
    /// * `filter_envelope` - ADSR filter envelope for filter cutoff modulation
    /// * `pitch_bend_semitones` - Pitch bend in semitones
    pub fn with_full_params(
        frequencies: &[f32],
        start_time: f32,
        duration: f32,
        waveform: Waveform,
        envelope: Envelope,
        filter_envelope: FilterEnvelope,
        pitch_bend_semitones: f32,
    ) -> Self {
        Self::with_complete_params(
            frequencies,
            start_time,
            duration,
            waveform,
            envelope,
            filter_envelope,
            FMParams::default(),
            pitch_bend_semitones,
            None,
            0.8, // Default velocity
        )
    }

    /// Create a note event with all possible parameters
    ///
    /// This is the most flexible constructor, allowing full control over all
    /// synthesis parameters including FM synthesis and custom wavetables.
    ///
    /// # Arguments
    /// * `frequencies` - Note frequencies in Hz (up to 8 for chords)
    /// * `start_time` - When to start playing (in seconds from track start)
    /// * `duration` - How long to sustain the note (in seconds)
    /// * `waveform` - Basic waveform (sine, square, saw, triangle)
    /// * `envelope` - ADSR amplitude envelope
    /// * `filter_envelope` - ADSR filter envelope
    /// * `fm_params` - FM synthesis parameters
    /// * `pitch_bend_semitones` - Pitch bend in semitones
    /// * `custom_wavetable` - Custom wavetable (overrides waveform if Some)
    /// * `velocity` - Note velocity 0.0-1.0 (affects MIDI export)
    #[allow(clippy::too_many_arguments)]
    pub fn with_complete_params(
        frequencies: &[f32],
        start_time: f32,
        duration: f32,
        waveform: Waveform,
        envelope: Envelope,
        filter_envelope: FilterEnvelope,
        fm_params: FMParams,
        pitch_bend_semitones: f32,
        custom_wavetable: Option<crate::synthesis::wavetable::Wavetable>,
        velocity: f32,
    ) -> Self {
        let mut freq_array = [0.0; 8];
        let num_freqs = frequencies.len().min(8);

        // Safe bounds check: num_freqs is guaranteed to be <= both frequencies.len() and 8
        // This copy is safe because both slices have exactly num_freqs elements
        if num_freqs > 0 {
            freq_array[..num_freqs].copy_from_slice(&frequencies[..num_freqs]);
        }

        Self {
            frequencies: freq_array,
            num_freqs,
            start_time,
            duration,
            waveform,
            envelope,
            filter_envelope,
            fm_params,
            pitch_bend_semitones,
            custom_wavetable,
            velocity,
            spatial_position: None,
        }
    }
}
