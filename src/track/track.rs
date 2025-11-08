#![allow(clippy::module_inception)]
//! Track implementation
//!
//! A track contains a sequence of audio events with global properties like
//! volume, pan, filter, and effects.

use super::events::*;
use crate::composition::drums::DrumType;
use crate::synthesis::effects::{Delay, Distortion, EffectChain, Reverb};
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::Filter;
use crate::synthesis::filter_envelope::FilterEnvelope;
use crate::synthesis::fm_synthesis::FMParams;
use crate::synthesis::lfo::ModRoute;
use crate::synthesis::waveform::Waveform;

/// A track contains a sequence of audio events (notes and drums)
#[derive(Debug, Clone)]
pub struct Track {
    pub events: Vec<AudioEvent>,
    pub name: Option<String>,     // Track name (used in MIDI export)
    pub midi_program: Option<u8>, // MIDI program number (0-127) for this track
    pub volume: f32,              // 0.0 to 1.0
    pub pan: f32,                 // -1.0 (left) to 1.0 (right), 0.0 = center
    pub filter: Filter,           // Filter applied to this track

    // Unified effect chain
    pub effects: EffectChain,

    pub modulation: Vec<ModRoute>, // LFO modulation routes

    // Cached time bounds for performance (computed on-demand)
    pub(super) cached_start_time: Option<f32>,
    pub(super) cached_end_time: Option<f32>,

    // Flag to track if events are sorted by start_time
    pub(super) events_sorted: bool,
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

            // Unified effect chain
            effects: EffectChain::new(),

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
    pub(super) fn start_time(&mut self) -> f32 {
        if let Some(cached) = self.cached_start_time {
            return cached;
        }

        let start = self
            .events
            .iter()
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
    pub(super) fn end_time(&mut self) -> f32 {
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
    pub(super) fn ensure_sorted(&mut self) {
        if !self.events_sorted {
            self.events.sort_by(|a, b| {
                a.start_time()
                    .partial_cmp(&b.start_time())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            self.events_sorted = true;
        }
    }

    /// Find the range of events that could be active at the given time using binary search
    /// Returns (start_index, end_index) where events[start_index..end_index] may be active
    ///
    /// This is O(log n) instead of O(n), dramatically faster for large event counts
    pub(super) fn find_active_range(&self, time: f32) -> (usize, usize) {
        if self.events.is_empty() {
            return (0, 0);
        }

        // Binary search to find first event that MIGHT be active
        // An event is potentially active if: time >= (event.start_time - max_release_time)
        // For simplicity, we search for events that start before or at current time + lookahead

        // Find first event that ends after current time
        let start_idx = self
            .events
            .partition_point(|event| event.end_time() <= time);

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
        self.effects = self.effects.with_delay(delay);
        self
    }

    /// Add reverb effect to track (builder pattern)
    ///
    /// # Arguments
    /// * `reverb` - Reverb effect configuration
    pub fn with_reverb(mut self, reverb: Reverb) -> Self {
        self.effects = self.effects.with_reverb(reverb);
        self
    }

    /// Add distortion effect to track (builder pattern)
    ///
    /// # Arguments
    /// * `distortion` - Distortion effect configuration
    pub fn with_distortion(mut self, distortion: Distortion) -> Self {
        self.effects = self.effects.with_distortion(distortion);
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
    #[allow(clippy::too_many_arguments)]
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
        custom_wavetable: Option<crate::synthesis::wavetable::Wavetable>,
        velocity: f32,
        spatial_position: Option<crate::synthesis::spatial::SpatialPosition>,
    ) {
        let mut note = NoteEvent::with_complete_params(
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
        );
        note.spatial_position = spatial_position;
        self.events.push(AudioEvent::Note(note));
        self.invalidate_time_cache();
    }

    /// Add a drum hit event to the track
    ///
    /// # Arguments
    /// * `drum_type` - Type of drum (Kick, Snare, HiHat, etc.)
    /// * `start_time` - When to trigger the drum (in seconds from track start)
    /// * `spatial_position` - Optional 3D spatial position
    pub fn add_drum(&mut self, drum_type: DrumType, start_time: f32, spatial_position: Option<crate::synthesis::spatial::SpatialPosition>) {
        self.events.push(AudioEvent::Drum(DrumEvent {
            drum_type,
            start_time,
            spatial_position,
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
