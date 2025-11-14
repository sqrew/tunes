//! Cache key computation for synthesis parameters
//!
//! Computes hash keys from synthesis parameters to identify unique sounds.
//! Handles f32 hashing by converting to bits (avoiding NaN issues).

use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::{Filter, FilterType};
use crate::synthesis::filter_envelope::FilterEnvelope;
use crate::synthesis::fm_synthesis::FMParams;
use crate::synthesis::waveform::Waveform;
use crate::track::NoteEvent;  // Re-exported from track module
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Cache key for synthesized audio
///
/// Represents the unique combination of synthesis parameters that produce
/// a specific sound. Two sounds with the same CacheKey should be identical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CacheKey(u64);

impl CacheKey {
    /// Create a cache key from a raw u64 (for testing)
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Compute cache key from synthesis parameters
    ///
    /// This creates a unique identifier based on:
    /// - Waveform type
    /// - Envelope (ADSR)
    /// - FM synthesis parameters
    /// - Filter settings
    /// - Duration (affects envelope shape)
    /// - Pitch bend
    ///
    /// Note: Frequency is NOT part of the cache key - we cache at a reference
    /// pitch and transpose during playback for better cache efficiency.
    pub fn from_note_event(note: &NoteEvent, sample_rate: f32) -> Self {
        let mut hasher = DefaultHasher::new();

        // Waveform
        hash_waveform(&note.waveform, &mut hasher);

        // Envelope (ADSR + curve)
        hash_envelope(&note.envelope, &mut hasher);

        // FM parameters
        hash_fm_params(&note.fm_params, &mut hasher);

        // Filter envelope
        hash_filter_envelope(&note.filter_envelope, &mut hasher);

        // Duration (affects envelope shape)
        hash_f32(note.duration, &mut hasher);

        // Pitch bend
        hash_f32(note.pitch_bend_semitones, &mut hasher);

        // Sample rate (affects filter frequencies)
        hash_f32(sample_rate, &mut hasher);

        // Velocity (affects amplitude)
        hash_f32(note.velocity, &mut hasher);

        // Custom wavetable (if present, use pointer address as ID)
        if let Some(ref wavetable) = note.custom_wavetable {
            // Use the wavetable reference address as a unique ID
            // This assumes wavetables are shared/immutable
            let ptr = wavetable as *const _ as usize;
            ptr.hash(&mut hasher);
        }

        CacheKey(hasher.finish())
    }

    /// Compute cache key including track filter
    ///
    /// Use this when you want to cache the filtered output.
    /// Note: Most users should cache the raw oscillator output and apply
    /// filters at runtime for more flexibility.
    pub fn from_note_with_filter(
        note: &NoteEvent,
        filter: &Filter,
        sample_rate: f32,
    ) -> Self {
        let mut hasher = DefaultHasher::new();

        // Start with the base note hash
        let base_key = Self::from_note_event(note, sample_rate);
        base_key.0.hash(&mut hasher);

        // Add filter parameters
        hash_filter(filter, &mut hasher);

        CacheKey(hasher.finish())
    }

    /// Get the raw u64 value
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

// Helper functions for hashing synthesis parameters

fn hash_f32(value: f32, hasher: &mut DefaultHasher) {
    // Convert f32 to bits for hashing (handles NaN/infinity correctly)
    value.to_bits().hash(hasher);
}

fn hash_waveform(waveform: &Waveform, hasher: &mut DefaultHasher) {
    // Waveform is an enum, can use discriminant
    std::mem::discriminant(waveform).hash(hasher);
}

fn hash_envelope(envelope: &Envelope, hasher: &mut DefaultHasher) {
    hash_f32(envelope.attack, hasher);
    hash_f32(envelope.decay, hasher);
    hash_f32(envelope.sustain, hasher);
    hash_f32(envelope.release, hasher);
    std::mem::discriminant(&envelope.curve).hash(hasher);
}

fn hash_fm_params(fm: &FMParams, hasher: &mut DefaultHasher) {
    hash_f32(fm.mod_ratio, hasher);
    hash_f32(fm.mod_index, hasher);
    hash_f32(fm.index_envelope_attack, hasher);
    hash_f32(fm.index_envelope_decay, hasher);
    hash_f32(fm.index_envelope_sustain, hasher);
    hash_f32(fm.index_envelope_release, hasher);
    hash_f32(fm.index_env_amount, hasher);
}

fn hash_filter_envelope(filter_env: &FilterEnvelope, hasher: &mut DefaultHasher) {
    hash_f32(filter_env.attack, hasher);
    hash_f32(filter_env.decay, hasher);
    hash_f32(filter_env.sustain, hasher);
    hash_f32(filter_env.release, hasher);
    hash_f32(filter_env.base_cutoff, hasher);
    hash_f32(filter_env.peak_cutoff, hasher);
    hash_f32(filter_env.amount, hasher);
}

fn hash_filter(filter: &Filter, hasher: &mut DefaultHasher) {
    std::mem::discriminant(&filter.filter_type).hash(hasher);
    hash_f32(filter.cutoff, hasher);
    hash_f32(filter.resonance, hasher);
    std::mem::discriminant(&filter.slope).hash(hasher);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::synthesis::envelope::EnvelopeCurve;

    #[test]
    fn test_same_parameters_same_key() {
        let note1 = NoteEvent {
            frequencies: [440.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            num_freqs: 1,
            start_time: 0.0,
            duration: 1.0,
            waveform: Waveform::Sine,
            envelope: Envelope::with_curve(0.01, 0.1, 0.7, 0.2, EnvelopeCurve::Linear),
            filter_envelope: FilterEnvelope::default(),
            fm_params: FMParams::default(),
            pitch_bend_semitones: 0.0,
            custom_wavetable: None,
            velocity: 1.0,
            spatial_position: None,
        };

        let note2 = note1.clone();

        let key1 = CacheKey::from_note_event(&note1, 44100.0);
        let key2 = CacheKey::from_note_event(&note2, 44100.0);

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_different_envelope_different_key() {
        let mut note1 = NoteEvent {
            frequencies: [440.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            num_freqs: 1,
            start_time: 0.0,
            duration: 1.0,
            waveform: Waveform::Sine,
            envelope: Envelope::with_curve(0.01, 0.1, 0.7, 0.2, EnvelopeCurve::Linear),
            filter_envelope: FilterEnvelope::default(),
            fm_params: FMParams::default(),
            pitch_bend_semitones: 0.0,
            custom_wavetable: None,
            velocity: 1.0,
            spatial_position: None,
        };

        let mut note2 = note1.clone();
        note2.envelope = Envelope::with_curve(0.05, 0.1, 0.7, 0.2, EnvelopeCurve::Linear); // Different attack

        let key1 = CacheKey::from_note_event(&note1, 44100.0);
        let key2 = CacheKey::from_note_event(&note2, 44100.0);

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_frequency_not_in_key() {
        let mut note1 = NoteEvent {
            frequencies: [440.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            num_freqs: 1,
            start_time: 0.0,
            duration: 1.0,
            waveform: Waveform::Sine,
            envelope: Envelope::default(),
            filter_envelope: FilterEnvelope::default(),
            fm_params: FMParams::default(),
            pitch_bend_semitones: 0.0,
            custom_wavetable: None,
            velocity: 1.0,
            spatial_position: None,
        };

        let mut note2 = note1.clone();
        note2.frequencies[0] = 880.0; // Different frequency (octave higher)

        let key1 = CacheKey::from_note_event(&note1, 44100.0);
        let key2 = CacheKey::from_note_event(&note2, 44100.0);

        // Keys should be THE SAME - we don't cache per-frequency
        // We cache the waveform shape and transpose during playback
        assert_eq!(key1, key2);
    }
}
