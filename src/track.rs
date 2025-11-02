//! Audio track and event system
//!
//! This module defines the core types for representing musical tracks and audio events.
//! It provides the building blocks for composing music programmatically.
//!
//! # Main Types
//!
//! - **`Track`** - A single audio track containing timed events (notes, drums, samples)
//! - **`Mixer`** - Combines multiple tracks for playback or export
//! - **`AudioEvent`** - Enum of different event types (notes, drums, samples, tempo changes, etc.)
//!
//! # Event Types
//!
//! - **`NoteEvent`** - Synthesized notes with pitch, duration, envelopes, and effects
//! - **`DrumEvent`** - Drum hits using built-in synthesis
//! - **`SampleEvent`** - WAV sample playback with pitch shifting
//! - **`TempoChangeEvent`** - Change tempo mid-composition
//! - **`TimeSignatureEvent`** - Change time signature (e.g., 4/4 to 3/4)
//! - **`KeySignatureEvent`** - Change key signature for MIDI export
//!
//! # Track Properties
//!
//! Tracks have several global properties that affect all events:
//! - Volume and pan (stereo positioning)
//! - Filter (low-pass, high-pass, band-pass, etc.)
//! - Effects (reverb, delay, distortion, chorus, etc.)
//! - Modulation routes (LFO modulation of parameters)
//!
//! # Example
//!
//! ```
//! # use tunes::prelude::*;
//! # use tunes::track::*;
//! let mut track = Track::new();
//! track.volume = 0.8;
//! track.pan = -0.3;  // Slightly left
//!
//! // Tracks are typically created through Composition
//! let mut comp = Composition::new(Tempo::new(120.0));
//! comp.track("melody")
//!     .volume(0.8)
//!     .notes(&[C4, E4, G4], 0.5);
//! ```

use crate::drums::DrumType;
use crate::effects::{BitCrusher, Chorus, Compressor, Delay, Distortion, EQ, Flanger, Phaser, Reverb, RingModulator, Saturation};
use crate::envelope::Envelope;
use crate::filter::Filter;
use crate::filter_envelope::FilterEnvelope;
use crate::fm_synthesis::FMParams;
use crate::key_signature::KeySignature;
use crate::lfo::ModRoute;
use crate::rhythm::Tempo;
use crate::sample::Sample;
use crate::waveform::Waveform;

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
    pub custom_wavetable: Option<crate::wavetable::Wavetable>, // Custom wavetable (overrides waveform if present)
    pub velocity: f32,      // Note velocity (0.0 to 1.0), affects MIDI export and can be used for expression
}

/// Represents a drum hit event
#[derive(Debug, Clone, Copy)]
pub struct DrumEvent {
    pub drum_type: DrumType,
    pub start_time: f32,
}

/// Represents a sample playback event
#[derive(Debug, Clone)]
pub struct SampleEvent {
    pub sample: Sample,
    pub start_time: f32,
    pub playback_rate: f32,  // 1.0 = normal speed, 2.0 = double speed, 0.5 = half speed
    pub volume: f32,         // 0.0 to 1.0
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
    pub fn with_complete_params(
        frequencies: &[f32],
        start_time: f32,
        duration: f32,
        waveform: Waveform,
        envelope: Envelope,
        filter_envelope: FilterEnvelope,
        fm_params: FMParams,
        pitch_bend_semitones: f32,
        custom_wavetable: Option<crate::wavetable::Wavetable>,
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
        }
    }
}

/// A track contains a sequence of audio events (notes and drums)
#[derive(Debug, Clone)]
pub struct Track {
    pub events: Vec<AudioEvent>,
    pub name: Option<String>,           // Track name (used in MIDI export)
    pub midi_program: Option<u8>,       // MIDI program number (0-127) for this track
    pub volume: f32,                    // 0.0 to 1.0
    pub pan: f32,                       // -1.0 (left) to 1.0 (right), 0.0 = center
    pub filter: Filter,                 // Filter applied to this track
    pub delay: Option<Delay>,           // Optional delay effect
    pub reverb: Option<Reverb>,         // Optional reverb effect
    pub distortion: Option<Distortion>, // Optional distortion effect
    pub bitcrusher: Option<BitCrusher>, // Optional bitcrusher effect
    pub compressor: Option<Compressor>, // Optional compressor effect
    pub chorus: Option<Chorus>,         // Optional chorus effect
    pub eq: Option<EQ>,                 // Optional EQ effect
    pub saturation: Option<Saturation>, // Optional saturation effect
    pub phaser: Option<Phaser>,         // Optional phaser effect
    pub flanger: Option<Flanger>,       // Optional flanger effect
    pub ring_mod: Option<RingModulator>, // Optional ring modulator effect
    pub modulation: Vec<ModRoute>,      // LFO modulation routes

    // Cached time bounds for performance (computed on-demand)
    cached_start_time: Option<f32>,
    cached_end_time: Option<f32>,

    // Flag to track if events are sorted by start_time
    events_sorted: bool,
}

impl Track {
    /// Create a new empty track with default settings
    ///
    /// Creates a track with volume 1.0, center pan, no filter, and no effects.
    /// Events can be added using methods like `add_note()` and `add_drum()`.
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            name: None,
            midi_program: None,
            volume: 1.0,
            pan: 0.0, // Center by default
            filter: Filter::none(),
            delay: None,
            reverb: None,
            distortion: None,
            bitcrusher: None,
            compressor: None,
            chorus: None,
            eq: None,
            saturation: None,
            phaser: None,
            flanger: None,
            ring_mod: None,
            modulation: Vec::new(),
            cached_start_time: None,
            cached_end_time: None,
            events_sorted: true, // Empty list is sorted
        }
    }

    /// Get the start time of the first event (cached for performance)
    ///
    /// Returns the earliest start time among all events in the track.
    /// This value is cached and only recomputed when events are added.
    fn start_time(&mut self) -> f32 {
        if let Some(cached) = self.cached_start_time {
            return cached;
        }

        let start = self.events.iter()
            .map(|e| match e {
                AudioEvent::Note(n) => n.start_time,
                AudioEvent::Drum(d) => d.start_time,
                AudioEvent::Sample(s) => s.start_time,
                AudioEvent::TempoChange(t) => t.start_time,
                AudioEvent::TimeSignature(ts) => ts.start_time,
                AudioEvent::KeySignature(ks) => ks.start_time,
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        self.cached_start_time = Some(start);
        start
    }

    /// Get the end time of the last event (cached for performance)
    ///
    /// Returns the latest end time among all events in the track (including release times).
    /// This value is cached and only recomputed when events are added.
    fn end_time(&mut self) -> f32 {
        if let Some(cached) = self.cached_end_time {
            return cached;
        }

        let end = self.total_duration();
        self.cached_end_time = Some(end);
        end
    }

    /// Invalidate time caches (call when events are added)
    ///
    /// Called internally whenever events are added or modified.
    /// Clears cached start/end times and marks events as needing re-sort.
    pub(crate) fn invalidate_time_cache(&mut self) {
        self.cached_start_time = None;
        self.cached_end_time = None;
        self.events_sorted = false; // Events need to be re-sorted
    }

    /// Ensure events are sorted by start_time (lazy sorting for performance)
    ///
    /// Events are sorted on-demand rather than on every insert.
    /// This is more efficient when adding multiple events in a batch.
    fn ensure_sorted(&mut self) {
        if !self.events_sorted {
            self.events.sort_by(|a, b| {
                a.start_time().partial_cmp(&b.start_time())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            self.events_sorted = true;
        }
    }

    /// Find the range of events that could be active at the given time using binary search
    /// Returns (start_index, end_index) where events[start_index..end_index] may be active
    ///
    /// This is O(log n) instead of O(n), dramatically faster for large event counts
    fn find_active_range(&self, time: f32) -> (usize, usize) {
        if self.events.is_empty() {
            return (0, 0);
        }

        // Binary search to find first event that MIGHT be active
        // An event is potentially active if: time >= (event.start_time - max_release_time)
        // For simplicity, we search for events that start before or at current time + lookahead

        // Find first event that ends after current time
        let start_idx = self.events.partition_point(|event| event.end_time() <= time);

        // All remaining events could potentially be active (they end after current time)
        // We could further optimize by finding events that start after current time,
        // but for now this gives us O(log n) search + O(k) iteration where k = active events

        (start_idx, self.events.len())
    }

    /// Set track volume (builder pattern)
    ///
    /// # Arguments
    /// * `volume` - Volume level 0.0 (silent) to 2.0 (double), clamped to this range
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 2.0);
        self
    }

    /// Set track filter (builder pattern)
    ///
    /// # Arguments
    /// * `filter` - Filter to apply (low-pass, high-pass, band-pass, etc.)
    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.filter = filter;
        self
    }

    /// Add delay effect to track (builder pattern)
    ///
    /// # Arguments
    /// * `delay` - Delay effect configuration
    pub fn with_delay(mut self, delay: Delay) -> Self {
        self.delay = Some(delay);
        self
    }

    /// Add reverb effect to track (builder pattern)
    ///
    /// # Arguments
    /// * `reverb` - Reverb effect configuration
    pub fn with_reverb(mut self, reverb: Reverb) -> Self {
        self.reverb = Some(reverb);
        self
    }

    /// Add distortion effect to track (builder pattern)
    ///
    /// # Arguments
    /// * `distortion` - Distortion effect configuration
    pub fn with_distortion(mut self, distortion: Distortion) -> Self {
        self.distortion = Some(distortion);
        self
    }

    /// Add LFO modulation route to track (builder pattern)
    ///
    /// # Arguments
    /// * `mod_route` - Modulation route (LFO modulating a parameter like volume or filter cutoff)
    pub fn with_modulation(mut self, mod_route: ModRoute) -> Self {
        self.modulation.push(mod_route);
        self
    }

    /// Add a simple note event to the track
    ///
    /// Creates a note with default sine waveform and envelope.
    ///
    /// # Arguments
    /// * `frequencies` - Note frequencies in Hz (up to 8 for chords)
    /// * `start_time` - When to start playing (in seconds from track start)
    /// * `duration` - How long to sustain the note (in seconds)
    pub fn add_note(&mut self, frequencies: &[f32], start_time: f32, duration: f32) {
        self.events.push(AudioEvent::Note(NoteEvent::new(
            frequencies,
            start_time,
            duration,
        )));
        self.invalidate_time_cache();
    }

    /// Add a note event with a specific waveform
    ///
    /// # Arguments
    /// * `frequencies` - Note frequencies in Hz (up to 8 for chords)
    /// * `start_time` - When to start playing (in seconds from track start)
    /// * `duration` - How long to sustain the note (in seconds)
    /// * `waveform` - Waveform type (Sine, Square, Saw, Triangle)
    pub fn add_note_with_waveform(
        &mut self,
        frequencies: &[f32],
        start_time: f32,
        duration: f32,
        waveform: Waveform,
    ) {
        self.events.push(AudioEvent::Note(NoteEvent::with_waveform(
            frequencies,
            start_time,
            duration,
            waveform,
        )));
        self.invalidate_time_cache();
    }

    /// Add a note event with waveform and ADSR envelope
    ///
    /// # Arguments
    /// * `frequencies` - Note frequencies in Hz (up to 8 for chords)
    /// * `start_time` - When to start playing (in seconds from track start)
    /// * `duration` - How long to sustain the note (in seconds)
    /// * `waveform` - Waveform type (Sine, Square, Saw, Triangle)
    /// * `envelope` - ADSR amplitude envelope
    pub fn add_note_with_waveform_and_envelope(
        &mut self,
        frequencies: &[f32],
        start_time: f32,
        duration: f32,
        waveform: Waveform,
        envelope: Envelope,
    ) {
        self.events
            .push(AudioEvent::Note(NoteEvent::with_waveform_and_envelope(
                frequencies,
                start_time,
                duration,
                waveform,
                envelope,
            )));
        self.invalidate_time_cache();
    }

    /// Add a note event with waveform, envelope, and pitch bend
    ///
    /// # Arguments
    /// * `frequencies` - Note frequencies in Hz (up to 8 for chords)
    /// * `start_time` - When to start playing (in seconds from track start)
    /// * `duration` - How long to sustain the note (in seconds)
    /// * `waveform` - Waveform type (Sine, Square, Saw, Triangle)
    /// * `envelope` - ADSR amplitude envelope
    /// * `pitch_bend_semitones` - Pitch bend in semitones (e.g., 2.0 for up 2 semitones)
    pub fn add_note_with_waveform_envelope_and_bend(
        &mut self,
        frequencies: &[f32],
        start_time: f32,
        duration: f32,
        waveform: Waveform,
        envelope: Envelope,
        pitch_bend_semitones: f32,
    ) {
        self.events.push(AudioEvent::Note(
            NoteEvent::with_waveform_envelope_and_bend(
                frequencies,
                start_time,
                duration,
                waveform,
                envelope,
                pitch_bend_semitones,
            ),
        ));
        self.invalidate_time_cache();
    }

    /// Add a note event with all possible synthesis parameters
    ///
    /// This is the most flexible method, allowing full control over all
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
    pub fn add_note_with_complete_params(
        &mut self,
        frequencies: &[f32],
        start_time: f32,
        duration: f32,
        waveform: Waveform,
        envelope: Envelope,
        filter_envelope: FilterEnvelope,
        fm_params: FMParams,
        pitch_bend_semitones: f32,
        custom_wavetable: Option<crate::wavetable::Wavetable>,
        velocity: f32,
    ) {
        self.events.push(AudioEvent::Note(
            NoteEvent::with_complete_params(
                frequencies,
                start_time,
                duration,
                waveform,
                envelope,
                filter_envelope,
                fm_params,
                pitch_bend_semitones,
                custom_wavetable,
                velocity,
            ),
        ));
        self.invalidate_time_cache();
    }

    /// Add a drum hit event to the track
    ///
    /// # Arguments
    /// * `drum_type` - Type of drum (Kick, Snare, HiHat, etc.)
    /// * `start_time` - When to trigger the drum (in seconds from track start)
    pub fn add_drum(&mut self, drum_type: DrumType, start_time: f32) {
        self.events.push(AudioEvent::Drum(DrumEvent {
            drum_type,
            start_time,
        }));
        self.invalidate_time_cache();
    }

    /// Add a sequence of notes with equal duration
    ///
    /// Convenience method for adding multiple notes sequentially,
    /// each playing for the same duration.
    ///
    /// # Arguments
    /// * `frequencies_list` - List of note frequency arrays to play sequentially
    /// * `start_time` - When to start the sequence (in seconds from track start)
    /// * `note_duration` - Duration of each note in the sequence (in seconds)
    pub fn add_sequence(
        &mut self,
        frequencies_list: Vec<&[f32]>,
        start_time: f32,
        note_duration: f32,
    ) {
        let mut current_time = start_time;
        for freqs in frequencies_list {
            self.add_note(freqs, current_time, note_duration);
            current_time += note_duration;
        }
    }

    /// Get the total duration of the track in seconds
    ///
    /// Returns the end time of the last event (including release times for notes).
    /// Returns 0.0 for empty tracks.
    pub fn total_duration(&self) -> f32 {
        self.events
            .iter()
            .map(|e| match e {
                AudioEvent::Note(n) => n.start_time + n.duration,
                AudioEvent::Drum(d) => d.start_time + d.drum_type.duration(),
                AudioEvent::Sample(s) => s.start_time + (s.sample.duration / s.playback_rate),
                AudioEvent::TempoChange(t) => t.start_time,
                AudioEvent::TimeSignature(ts) => ts.start_time,
                AudioEvent::KeySignature(ks) => ks.start_time,
            })
            .fold(0.0, f32::max)
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}

/// Mix multiple tracks together
#[derive(Debug, Clone)]
pub struct Mixer {
    pub tracks: Vec<Track>,
    pub tempo: Tempo,
}

impl Mixer {
    /// Create a new mixer with the specified tempo
    ///
    /// # Arguments
    /// * `tempo` - Tempo for the composition (used for MIDI export)
    pub fn new(tempo: Tempo) -> Self {
        Self {
            tracks: Vec::new(),
            tempo,
        }
    }

    /// Add a track to the mixer
    ///
    /// Tracks are played simultaneously when the mixer is rendered or played.
    ///
    /// # Arguments
    /// * `track` - The track to add
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    /// Get the total duration across all tracks in seconds
    ///
    /// Returns the end time of the longest track.
    /// Returns 0.0 if the mixer has no tracks.
    pub fn total_duration(&self) -> f32 {
        self.tracks
            .iter()
            .map(|t| t.total_duration())
            .fold(0.0, f32::max)
    }

    /// Repeat all tracks in the mixer N times
    ///
    /// This duplicates all events in all tracks, placing copies sequentially.
    /// Useful for looping an entire composition.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::composition::Composition;
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::engine::AudioEngine;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// # let engine = AudioEngine::new()?;
    /// let mixer = comp.into_mixer().repeat(3); // Play composition 4 times total
    /// engine.play_mixer(&mixer)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn repeat(mut self, times: usize) -> Self {
        if times == 0 {
            return self;
        }

        let total_duration = self.total_duration();

        // For each track, repeat its events
        for track in &mut self.tracks {
            let original_events: Vec<_> = track.events.clone();

            for i in 0..times {
                let offset = total_duration * (i + 1) as f32;

                for event in &original_events {
                    match event {
                        AudioEvent::Note(note) => {
                            track.add_note_with_waveform_envelope_and_bend(
                                &note.frequencies[..note.num_freqs],
                                note.start_time + offset,
                                note.duration,
                                note.waveform,
                                note.envelope,
                                note.pitch_bend_semitones,
                            );
                        }
                        AudioEvent::Drum(drum) => {
                            track.add_drum(drum.drum_type, drum.start_time + offset);
                        }
                        AudioEvent::Sample(sample) => {
                            track.events.push(AudioEvent::Sample(crate::track::SampleEvent {
                                sample: sample.sample.clone(),
                                start_time: sample.start_time + offset,
                                playback_rate: sample.playback_rate,
                                volume: sample.volume,
                            }));
                            track.invalidate_time_cache();
                        }
                        AudioEvent::TempoChange(tempo) => {
                            track.events.push(AudioEvent::TempoChange(crate::track::TempoChangeEvent {
                                start_time: tempo.start_time + offset,
                                bpm: tempo.bpm,
                            }));
                            track.invalidate_time_cache();
                        }
                        AudioEvent::TimeSignature(time_sig) => {
                            track.events.push(AudioEvent::TimeSignature(crate::track::TimeSignatureEvent {
                                start_time: time_sig.start_time + offset,
                                numerator: time_sig.numerator,
                                denominator: time_sig.denominator,
                            }));
                            track.invalidate_time_cache();
                        }
                        AudioEvent::KeySignature(key_sig) => {
                            track.events.push(AudioEvent::KeySignature(KeySignatureEvent {
                                start_time: key_sig.start_time + offset,
                                key_signature: key_sig.key_signature,
                            }));
                            track.invalidate_time_cache();
                        }
                    }
                }
            }
        }

        self
    }

    /// Generate a stereo sample at a given time by mixing all active tracks
    ///
    /// This is the core rendering method that generates audio samples by:
    /// 1. Finding active events on all tracks at the given time
    /// 2. Synthesizing audio for each event
    /// 3. Applying track-level effects (filter, reverb, delay, etc.)
    /// 4. Mixing tracks with stereo panning
    ///
    /// # Arguments
    /// * `time` - The time position in seconds
    /// * `sample_rate` - Sample rate in Hz (e.g., 44100)
    /// * `_sample_clock` - Reserved for future use
    ///
    /// # Returns
    /// A tuple of (left_channel, right_channel) audio samples in range -1.0 to 1.0
    pub fn sample_at(&mut self, time: f32, sample_rate: f32, _sample_clock: f32) -> (f32, f32) {
        let mut mixed_left = 0.0;
        let mut mixed_right = 0.0;

        for track in &mut self.tracks {
            // Ensure events are sorted by start_time for binary search
            track.ensure_sorted();

            // Quick time-bounds check: skip entire track if current time is outside its active range
            // This avoids iterating through all events on inactive tracks
            let track_start = track.start_time();
            let track_end = track.end_time();

            // Skip track entirely if we're before it starts or after it ends
            // (unless it has delay/reverb which can extend beyond the events)
            if (time < track_start || time > track_end)
                && track.delay.is_none()
                && track.reverb.is_none() {
                continue;
            }

            let mut track_value = 0.0;
            let mut has_active_event = false;
            let mut filter_env_cutoff = track.filter.cutoff;
            let mut filter_env_found = false;

            // Binary search to find potentially active events (O(log n) instead of O(n))
            let (start_idx, end_idx) = track.find_active_range(time);

            // Only iterate through events that could possibly be active
            for event in &track.events[start_idx..end_idx] {
                match event {
                    AudioEvent::Sample(sample_event) => {
                        let time_in_sample = time - sample_event.start_time;
                        let sample_duration = sample_event.sample.duration / sample_event.playback_rate;

                        if time_in_sample >= 0.0 && time_in_sample < sample_duration {
                            has_active_event = true;
                            let (sample_left, sample_right) = sample_event.sample.sample_at_interpolated(
                                time_in_sample,
                                sample_event.playback_rate,
                                sample_rate
                            );
                            track_value += (sample_left + sample_right) * 0.5 * sample_event.volume;
                        }
                    }
                    AudioEvent::Note(note_event) => {
                        let total_duration =
                            note_event.envelope.total_duration(note_event.duration);
                        let note_end_with_release = note_event.start_time + total_duration;

                        // Check if this note event is active (including release phase)
                        if time >= note_event.start_time && time < note_end_with_release {
                            has_active_event = true;

                            // Calculate time within the note
                            let time_in_note = time - note_event.start_time;

                            // Get filter envelope from this note (if it has one)
                            // Use the first active note's filter envelope we encounter
                            if !filter_env_found && note_event.filter_envelope.amount > 0.0 {
                                let filter_total_duration = note_event.filter_envelope.total_duration(note_event.duration);
                                let filter_end = note_event.start_time + filter_total_duration;
                                if time >= note_event.start_time && time < filter_end {
                                    filter_env_cutoff = note_event.filter_envelope.cutoff_at(time_in_note, note_event.duration);
                                    filter_env_found = true;
                                }
                            }

                            // Get envelope amplitude at this point in time
                            let envelope_amp = note_event
                                .envelope
                                .amplitude_at(time_in_note, note_event.duration);

                            // Generate waves for all frequencies in this event
                            for i in 0..note_event.num_freqs {
                                let base_freq = note_event.frequencies[i];

                                // Apply pitch bend (linear over note duration)
                                // Skip expensive math if no pitch bend
                                let freq = if note_event.pitch_bend_semitones != 0.0 {
                                    let bend_progress = (time_in_note / note_event.duration).min(1.0);
                                    let bend_multiplier = 2.0f32
                                        .powf((note_event.pitch_bend_semitones * bend_progress) / 12.0);
                                    base_freq * bend_multiplier
                                } else {
                                    base_freq
                                };

                                let sample = if note_event.fm_params.mod_index > 0.0 {
                                    // Use FM synthesis
                                    note_event.fm_params.sample(freq, time_in_note, note_event.duration)
                                } else if let Some(ref wavetable) = note_event.custom_wavetable {
                                    // Use custom wavetable
                                    let phase = (time_in_note * freq) % 1.0;
                                    wavetable.sample(phase)
                                } else {
                                    // Use standard waveform
                                    let phase = (time_in_note * freq) % 1.0;
                                    note_event.waveform.sample(phase)
                                };

                                track_value += sample * envelope_amp;
                            }
                        }
                    }
                    AudioEvent::Drum(drum_event) => {
                        let drum_duration = drum_event.drum_type.duration();
                        // Check if this drum event is active at the current time
                        if time >= drum_event.start_time
                            && time < drum_event.start_time + drum_duration
                        {
                            has_active_event = true;

                            // Calculate sample index relative to drum start
                            let time_in_drum = time - drum_event.start_time;
                            let sample_index = (time_in_drum * sample_rate) as usize;
                            track_value += drum_event.drum_type.sample(sample_index, sample_rate);
                        }
                    }
                    AudioEvent::TempoChange(_) => {
                        // Tempo changes don't generate audio, they're metadata for MIDI export
                    }
                    AudioEvent::TimeSignature(_) => {
                        // Time signatures don't generate audio, they're metadata for MIDI export
                    }
                    AudioEvent::KeySignature(_) => {
                        // Key signatures don't generate audio, they're metadata for MIDI export
                    }
                }
            }

            // Skip all effect processing if track has no active events
            if !has_active_event && track.delay.is_none() && track.reverb.is_none() {
                continue;
            }

            // Filter envelope was already collected in the event loop above

            // Apply LFO modulation on top of filter envelope
            let mut modulated_volume = track.volume;
            let mut modulated_cutoff = filter_env_cutoff;
            let mut modulated_resonance = track.filter.resonance;

            for mod_route in &track.modulation {
                match mod_route.target {
                    crate::lfo::ModTarget::Volume => {
                        modulated_volume = mod_route.apply(time, modulated_volume);
                    }
                    crate::lfo::ModTarget::FilterCutoff => {
                        modulated_cutoff = mod_route.apply(time, modulated_cutoff);
                    }
                    crate::lfo::ModTarget::FilterResonance => {
                        modulated_resonance = mod_route.apply(time, modulated_resonance);
                    }
                    _ => {} // Other modulation targets handled elsewhere
                }
            }

            // Only process effects if there's actual audio
            if track_value.abs() > 0.0001 || track.delay.is_some() || track.reverb.is_some() {
                // Apply track volume (with modulation)
                track_value *= modulated_volume;

                // Apply filter (with modulation)
                track.filter.cutoff = modulated_cutoff;
                track.filter.resonance = modulated_resonance;
                track_value = track.filter.process(track_value, sample_rate);

                // Apply EQ (shape frequencies early in chain)
                if let Some(ref mut eq) = track.eq {
                    track_value = eq.process(track_value, sample_rate);
                }

                // Apply compressor (dynamics control)
                if let Some(ref mut compressor) = track.compressor {
                    track_value = compressor.process(track_value, sample_rate);
                }

                // Apply saturation (warm coloration)
                if let Some(ref saturation) = track.saturation {
                    track_value = saturation.process(track_value);
                }

                // Apply bitcrusher (lo-fi degradation)
                if let Some(ref mut bitcrusher) = track.bitcrusher {
                    track_value = bitcrusher.process(track_value);
                }

                // Apply distortion
                if let Some(ref distortion) = track.distortion {
                    track_value = distortion.process(track_value);
                }

                // Apply chorus (modulation effect)
                if let Some(ref mut chorus) = track.chorus {
                    track_value = chorus.process(track_value, sample_rate);
                }

                // Apply phaser (sweeping notch filter)
                if let Some(ref mut phaser) = track.phaser {
                    track_value = phaser.process(track_value);
                }

                // Apply flanger (jet-plane swoosh)
                if let Some(ref mut flanger) = track.flanger {
                    track_value = flanger.process(track_value);
                }

                // Apply ring modulator (metallic/robotic tones)
                if let Some(ref mut ring_mod) = track.ring_mod {
                    track_value = ring_mod.process(track_value);
                }

                // Apply delay
                if let Some(ref mut delay) = track.delay {
                    track_value = delay.process(track_value);
                }

                // Apply reverb
                if let Some(ref mut reverb) = track.reverb {
                    track_value = reverb.process(track_value, time);
                }
            }

            // Apply stereo panning using constant power panning
            // pan: -1.0 (full left), 0.0 (center), 1.0 (full right)
            let pan_clamped = track.pan.clamp(-1.0, 1.0);
            let pan_angle = (pan_clamped + 1.0) * 0.25 * std::f32::consts::PI; // 0 to PI/2
            let left_gain = pan_angle.cos();
            let right_gain = pan_angle.sin();

            // Add to stereo mix
            mixed_left += track_value * left_gain;
            mixed_right += track_value * right_gain;
        }

        // Normalize by number of tracks to prevent clipping
        if !self.tracks.is_empty() {
            let scale = 1.0 / (self.tracks.len() as f32 * 2.0);
            (mixed_left * scale, mixed_right * scale)
        } else {
            (0.0, 0.0)
        }
    }

    /// Render the mixer to an in-memory stereo buffer
    ///
    /// Pre-renders the entire composition to a Vec of interleaved stereo samples (left, right, left, right...).
    /// This is used for efficient playback without real-time synthesis overhead.
    ///
    /// # Arguments
    /// * `sample_rate` - Sample rate in Hz
    ///
    /// # Returns
    /// A Vec of f32 samples in interleaved stereo format (left, right, left, right...)
    pub fn render_to_buffer(&mut self, sample_rate: f32) -> Vec<f32> {
        let duration = self.total_duration();
        let total_samples = (duration * sample_rate).ceil() as usize;

        // Pre-allocate buffer for interleaved stereo (2 channels)
        let mut buffer = Vec::with_capacity(total_samples * 2);

        let mut sample_clock = 0.0;

        // Render all samples
        for i in 0..total_samples {
            let time = i as f32 / sample_rate;
            let (left, right) = self.sample_at(time, sample_rate, sample_clock);

            // Clamp to valid range and add to buffer
            buffer.push(left.clamp(-1.0, 1.0));
            buffer.push(right.clamp(-1.0, 1.0));

            sample_clock = (sample_clock + 1.0) % sample_rate;
        }

        buffer
    }

    /// Export the mixed audio to a WAV file
    ///
    /// Renders the entire composition to a stereo WAV file with the specified sample rate.
    ///
    /// # Arguments
    /// * `path` - Output file path (e.g., "output.wav")
    /// * `sample_rate` - Sample rate in Hz (44100 is CD quality, 48000 is professional)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("piano").note(&[440.0], 1.0);
    ///
    /// let mut mixer = comp.into_mixer();
    /// mixer.export_wav("output.wav", 44100)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn export_wav(&mut self, path: &str, sample_rate: u32) -> anyhow::Result<()> {
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(path, spec)?;

        let duration = self.total_duration();
        let total_samples = (duration * sample_rate as f32).ceil() as usize;

        let sample_rate_f32 = sample_rate as f32;
        let mut sample_clock = 0.0;

        println!("Rendering to WAV...");
        println!("  Duration: {:.2}s", duration);
        println!("  Sample rate: {} Hz", sample_rate);
        println!("  Total samples: {}", total_samples);

        for i in 0..total_samples {
            let time = i as f32 / sample_rate_f32;

            // Generate stereo sample
            let (left, right) = self.sample_at(time, sample_rate_f32, sample_clock);

            // Convert from f32 (-1.0 to 1.0) to i16 (-32768 to 32767)
            let left_i16 = (left.clamp(-1.0, 1.0) * 32767.0) as i16;
            let right_i16 = (right.clamp(-1.0, 1.0) * 32767.0) as i16;

            writer.write_sample(left_i16)?;
            writer.write_sample(right_i16)?;

            sample_clock = (sample_clock + 1.0) % sample_rate_f32;

            // Progress indicator every second
            if i % sample_rate as usize == 0 {
                let progress = (i as f32 / total_samples as f32) * 100.0;
                print!("\r  Progress: {:.0}%", progress);
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
        }

        println!("\r  Progress: 100%");
        writer.finalize()?;

        println!("✅ Exported to: {}", path);
        Ok(())
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new(Tempo::new(120.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notes::*;

    #[test]
    fn test_note_event_construction() {
        let freqs = [440.0, 554.37]; // A4 and C#5
        let note = NoteEvent::new(&freqs, 0.0, 1.0);

        assert_eq!(note.start_time, 0.0);
        assert_eq!(note.duration, 1.0);
        assert_eq!(note.num_freqs, 2);
        assert_eq!(note.frequencies[0], 440.0);
        assert_eq!(note.frequencies[1], 554.37);
        assert_eq!(note.waveform, Waveform::Sine);
        assert_eq!(note.pitch_bend_semitones, 0.0);
    }

    #[test]
    fn test_note_event_truncates_frequencies() {
        // Test that more than 8 frequencies are truncated
        let freqs = [100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0];
        let note = NoteEvent::new(&freqs, 0.0, 1.0);

        assert_eq!(note.num_freqs, 8, "Should truncate to max 8 frequencies");
        assert_eq!(note.frequencies[7], 800.0, "Should include first 8 frequencies");
    }

    #[test]
    fn test_note_event_empty_frequencies() {
        // Test handling of empty frequency array
        let note = NoteEvent::new(&[], 0.0, 1.0);

        assert_eq!(note.num_freqs, 0);
        assert_eq!(note.frequencies[0], 0.0);
    }

    #[test]
    fn test_track_creation() {
        let track = Track::new();

        assert_eq!(track.events.len(), 0);
        assert_eq!(track.volume, 1.0);
        assert_eq!(track.pan, 0.0);
        assert!(track.delay.is_none());
        assert!(track.reverb.is_none());
    }

    #[test]
    fn test_track_add_note() {
        let mut track = Track::new();
        track.add_note(&[440.0], 0.0, 1.0);
        track.add_note(&[880.0], 1.0, 0.5);

        assert_eq!(track.events.len(), 2);

        match &track.events[0] {
            AudioEvent::Note(note) => {
                assert_eq!(note.frequencies[0], 440.0);
                assert_eq!(note.start_time, 0.0);
                assert_eq!(note.duration, 1.0);
            }
            _ => panic!("Expected NoteEvent"),
        }
    }

    #[test]
    fn test_track_add_drum() {
        let mut track = Track::new();
        track.add_drum(DrumType::Kick, 0.0);
        track.add_drum(DrumType::Snare, 0.5);

        assert_eq!(track.events.len(), 2);

        match &track.events[0] {
            AudioEvent::Drum(drum) => {
                assert!(matches!(drum.drum_type, DrumType::Kick));
                assert_eq!(drum.start_time, 0.0);
            }
            _ => panic!("Expected DrumEvent"),
        }
    }

    #[test]
    fn test_track_total_duration_notes() {
        let mut track = Track::new();
        track.add_note(&[440.0], 0.0, 1.0); // Ends at 1.0
        track.add_note(&[880.0], 1.5, 2.0); // Ends at 3.5

        assert_eq!(track.total_duration(), 3.5);
    }

    #[test]
    fn test_track_total_duration_drums() {
        let mut track = Track::new();
        track.add_drum(DrumType::Kick, 0.0); // Duration 0.15
        track.add_drum(DrumType::Crash, 1.0); // Duration 1.5, ends at 2.5

        let duration = track.total_duration();
        assert_eq!(duration, 2.5, "Should account for drum durations");
    }

    #[test]
    fn test_track_total_duration_empty() {
        let track = Track::new();
        assert_eq!(track.total_duration(), 0.0);
    }

    #[test]
    fn test_track_with_volume() {
        let track = Track::new().with_volume(0.5);
        assert_eq!(track.volume, 0.5);

        // Test clamping
        let loud_track = Track::new().with_volume(5.0);
        assert_eq!(loud_track.volume, 2.0, "Volume should be clamped to 2.0");

        let silent_track = Track::new().with_volume(-1.0);
        assert_eq!(silent_track.volume, 0.0, "Volume should be clamped to 0.0");
    }

    #[test]
    fn test_track_add_sequence() {
        let mut track = Track::new();
        let c4_slice: &[f32] = &[C4];
        let e4_slice: &[f32] = &[E4];
        let g4_slice: &[f32] = &[G4];
        let notes = vec![c4_slice, e4_slice, g4_slice]; // C major chord
        track.add_sequence(notes, 0.0, 0.5);

        assert_eq!(track.events.len(), 3);

        // Verify timing
        if let AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &track.events[1] {
            assert_eq!(note.start_time, 0.5);
        }
        if let AudioEvent::Note(note) = &track.events[2] {
            assert_eq!(note.start_time, 1.0);
        }
    }

    #[test]
    fn test_mixer_creation() {
        let mixer = Mixer::new(Tempo::new(120.0));
        assert_eq!(mixer.tracks.len(), 0);
    }

    #[test]
    fn test_mixer_add_track() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let mut track1 = Track::new();
        track1.add_note(&[440.0], 0.0, 1.0);

        let mut track2 = Track::new();
        track2.add_drum(DrumType::Kick, 0.0);

        mixer.add_track(track1);
        mixer.add_track(track2);

        assert_eq!(mixer.tracks.len(), 2);
    }

    #[test]
    fn test_mixer_total_duration() {
        let mut mixer = Mixer::new(Tempo::new(120.0));

        let mut track1 = Track::new();
        track1.add_note(&[440.0], 0.0, 2.0); // Ends at 2.0

        let mut track2 = Track::new();
        track2.add_note(&[880.0], 1.0, 3.0); // Ends at 4.0

        mixer.add_track(track1);
        mixer.add_track(track2);

        assert_eq!(mixer.total_duration(), 4.0, "Should return longest track duration");
    }

    #[test]
    fn test_mixer_total_duration_empty() {
        let mixer = Mixer::new(Tempo::new(120.0));
        assert_eq!(mixer.total_duration(), 0.0);
    }

    #[test]
    fn test_mixer_repeat() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let mut track = Track::new();
        track.add_note(&[440.0], 0.0, 1.0); // Ends at 1.0
        mixer.add_track(track);

        let repeated_mixer = mixer.repeat(2); // Repeat 2 MORE times

        // Should still have 1 track (repeats events within the track)
        assert_eq!(repeated_mixer.tracks.len(), 1);

        // Track should now have 3 events total (original + 2 repeats)
        assert_eq!(repeated_mixer.tracks[0].events.len(), 3);

        // Verify timing offsets
        if let AudioEvent::Note(note) = &repeated_mixer.tracks[0].events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &repeated_mixer.tracks[0].events[1] {
            assert_eq!(note.start_time, 1.0); // Original duration offset
        }
        if let AudioEvent::Note(note) = &repeated_mixer.tracks[0].events[2] {
            assert_eq!(note.start_time, 2.0); // 2x original duration
        }
    }

    #[test]
    fn test_mixer_repeat_zero_times() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let mut track = Track::new();
        track.add_note(&[440.0], 0.0, 1.0);
        mixer.add_track(track);

        let repeated_mixer = mixer.repeat(0);

        // Repeating 0 times returns the mixer unchanged
        assert_eq!(repeated_mixer.tracks.len(), 1);
        assert_eq!(repeated_mixer.tracks[0].events.len(), 1);
    }

    #[test]
    fn test_drum_event_construction() {
        let drum = DrumEvent {
            drum_type: DrumType::Snare,
            start_time: 0.5,
        };

        assert!(matches!(drum.drum_type, DrumType::Snare));
        assert_eq!(drum.start_time, 0.5);
    }

    #[test]
    fn test_audio_event_enum() {
        let note = NoteEvent::new(&[440.0], 0.0, 1.0);
        let note_event = AudioEvent::Note(note);

        match note_event {
            AudioEvent::Note(n) => assert_eq!(n.frequencies[0], 440.0),
            _ => panic!("Expected Note variant"),
        }

        let drum_event = AudioEvent::Drum(DrumEvent {
            drum_type: DrumType::Kick,
            start_time: 0.0,
        });

        match drum_event {
            AudioEvent::Drum(d) => assert!(matches!(d.drum_type, DrumType::Kick)),
            _ => panic!("Expected Drum variant"),
        }
    }
}
