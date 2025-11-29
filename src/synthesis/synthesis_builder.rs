//! Synthesis builder for configuring track synthesis parameters
//!
//! Provides a fluent API for configuring oscillators, filters, envelopes,
//! and modulation in a namespaced, discoverable way.
//!
//! # Example
//! ```ignore
//! comp.track("lead")
//!     .synthesis(|s| s
//!         .oscillator(Waveform::Saw)
//!         .filter(FilterType::LowPass, 2000.0)
//!         .adsr(0.01, 0.1, 0.7, 0.3)
//!     )
//!     .notes(&[C4, E4, G4], 0.5);
//! ```

use crate::composition::TrackBuilder;
use crate::synthesis::envelope::Envelope;
use crate::synthesis::filter::{Filter, FilterType};
use crate::synthesis::filter_envelope::FilterEnvelope;
use crate::synthesis::fm_synthesis::FMParams;
use crate::synthesis::granular::{GranularParams, create_granular_events};
use crate::synthesis::lfo::{ModRoute, ModTarget, LFO};
use crate::synthesis::sample::Sample;
use crate::synthesis::waveform::Waveform;
use crate::synthesis::wavetable::Wavetable;
use crate::track::AudioEvent;

/// Builder for configuring synthesis parameters on a track.
///
/// This provides a namespaced, discoverable API for all synthesis options.
/// Methods can be chained and the builder returns itself for fluent configuration.
pub struct SynthesisBuilder<'a> {
    pub(crate) inner: TrackBuilder<'a>,
}

impl<'a> SynthesisBuilder<'a> {
    /// Get the inner TrackBuilder (used internally)
    pub fn into_inner(self) -> TrackBuilder<'a> {
        self.inner
    }

    // ==================== Oscillator Source ====================

    /// Set the oscillator waveform (sine, square, saw, triangle)
    ///
    /// # Example
    /// ```ignore
    /// .synthesis(|s| s.oscillator(Waveform::Saw))
    /// ```
    pub fn oscillator(mut self, waveform: Waveform) -> Self {
        self.inner.waveform = waveform;
        self
    }

    /// Set a custom wavetable for the oscillator
    ///
    /// This overrides the basic waveform with a custom wavetable.
    ///
    /// # Example
    /// ```ignore
    /// let wt = Wavetable::from_harmonics(2048, &[(1, 1.0), (3, 0.5), (5, 0.25)]);
    /// .synthesis(|s| s.wavetable(wt))
    /// ```
    pub fn wavetable(mut self, table: Wavetable) -> Self {
        self.inner.custom_wavetable = Some(table);
        self
    }

    /// Use additive synthesis with specified harmonic amplitudes
    ///
    /// Creates a custom waveform by summing harmonics with the given amplitudes.
    /// Each value in the array represents the amplitude of that harmonic (1st, 2nd, 3rd, etc.).
    ///
    /// # Arguments
    /// * `harmonic_amps` - Array of harmonic amplitudes (e.g., `&[1.0, 0.5, 0.25]`)
    ///
    /// # Example
    /// ```ignore
    /// // Sawtooth-like sound (harmonic series with decreasing amplitudes)
    /// .synthesis(|s| s.additive(&[1.0, 0.5, 0.33, 0.25, 0.2]))
    ///
    /// // Organ sound (odd harmonics only)
    /// .synthesis(|s| s.additive(&[1.0, 0.0, 0.5, 0.0, 0.3]))
    /// ```
    pub fn additive(mut self, harmonic_amps: &[f32]) -> Self {
        use crate::synthesis::wavetable::DEFAULT_TABLE_SIZE;

        // Convert harmonic amplitudes to (harmonic_number, amplitude) pairs
        let harmonics: Vec<(usize, f32)> = harmonic_amps
            .iter()
            .enumerate()
            .filter(|(_, &amp)| amp > 0.0)
            .map(|(i, &amp)| (i + 1, amp))
            .collect();

        // Create wavetable from harmonics
        let wavetable = Wavetable::from_harmonics(DEFAULT_TABLE_SIZE, &harmonics);
        self.inner.custom_wavetable = Some(wavetable);
        self
    }

    /// Supersaw - multiple detuned sawtooth oscillators
    ///
    /// Creates the classic trance/EDM supersaw sound by layering multiple
    /// sawtooth waves with slight detuning. The detuning creates a thick,
    /// chorus-like effect with natural beating.
    ///
    /// # Arguments
    /// * `voices` - Number of oscillator voices (3-9 typical, 7 is classic)
    /// * `detune` - Detune spread in cents (10-50 typical, higher = wider)
    ///
    /// # Example
    /// ```ignore
    /// // Classic 7-voice supersaw
    /// .synthesis(|s| s.supersaw(7, 25.0))
    ///
    /// // Massive 9-voice with wide detune
    /// .synthesis(|s| s.supersaw(9, 40.0))
    /// ```
    pub fn supersaw(mut self, voices: u8, detune_cents: f32) -> Self {
        use crate::synthesis::wavetable::DEFAULT_TABLE_SIZE;
        use std::f32::consts::PI;

        let voices = voices.max(1) as usize;
        let table_size = DEFAULT_TABLE_SIZE;
        let mut table = vec![0.0f32; table_size];

        // Generate each detuned voice
        for voice in 0..voices {
            // Spread voices evenly across detune range
            // Center voice has 0 detune, others spread symmetrically
            let voice_offset = if voices == 1 {
                0.0
            } else {
                let normalized = (voice as f32 / (voices - 1) as f32) * 2.0 - 1.0; // -1 to 1
                normalized * detune_cents
            };

            // Convert cents to frequency ratio
            let freq_ratio = 2.0_f32.powf(voice_offset / 1200.0);

            // Voice amplitude (center voice slightly louder)
            let center_distance = ((voice as f32 / (voices.max(2) - 1) as f32) - 0.5).abs();
            let amplitude = 1.0 - center_distance * 0.3;

            // Add this voice's sawtooth to the table
            for (i, table_slot) in table.iter_mut().enumerate() {
                let phase = (i as f32 / table_size as f32) * freq_ratio;
                let phase = phase.fract(); // Wrap to 0-1

                // Band-limited sawtooth via additive synthesis (first 32 harmonics)
                let mut sample = 0.0;
                for harmonic in 1..=32 {
                    let h = harmonic as f32;
                    sample += (2.0 * PI * h * phase).sin() / h;
                }
                sample *= 2.0 / PI; // Normalize

                *table_slot += sample * amplitude / voices as f32;
            }
        }

        let wavetable = Wavetable::from_samples(table);
        self.inner.custom_wavetable = Some(wavetable);
        self
    }

    /// Unison - multiple detuned copies of any waveform
    ///
    /// Like supersaw but works with the currently selected waveform.
    /// Call after `.oscillator()` to apply unison to that waveform.
    ///
    /// # Arguments
    /// * `voices` - Number of voices (2-9 typical)
    /// * `detune` - Detune spread in cents
    ///
    /// # Example
    /// ```ignore
    /// // Unison square wave
    /// .synthesis(|s| s.oscillator(Waveform::Square).unison(5, 20.0))
    /// ```
    pub fn unison(mut self, voices: u8, detune_cents: f32) -> Self {
        use crate::synthesis::wavetable::DEFAULT_TABLE_SIZE;

        let voices = voices.max(1) as usize;
        let table_size = DEFAULT_TABLE_SIZE;
        let mut table = vec![0.0f32; table_size];

        // Get the base waveform
        let base_waveform = self.inner.waveform;

        // Generate each detuned voice
        for voice in 0..voices {
            let voice_offset = if voices == 1 {
                0.0
            } else {
                let normalized = (voice as f32 / (voices - 1) as f32) * 2.0 - 1.0;
                normalized * detune_cents
            };

            let freq_ratio = 2.0_f32.powf(voice_offset / 1200.0);

            let center_distance = ((voice as f32 / (voices.max(2) - 1) as f32) - 0.5).abs();
            let amplitude = 1.0 - center_distance * 0.3;

            for (i, table_slot) in table.iter_mut().enumerate() {
                let phase = (i as f32 / table_size as f32) * freq_ratio;
                let phase = phase.fract();

                let sample = base_waveform.sample(phase);
                *table_slot += sample * amplitude / voices as f32;
            }
        }

        let wavetable = Wavetable::from_samples(table);
        self.inner.custom_wavetable = Some(wavetable);
        self
    }

    // ==================== Filter ====================

    /// Set the filter type and cutoff frequency
    ///
    /// # Example
    /// ```ignore
    /// .synthesis(|s| s.filter(FilterType::LowPass, 2000.0))
    /// ```
    pub fn filter(mut self, filter_type: FilterType, cutoff: f32) -> Self {
        self.inner.get_track_mut().filter = Filter::new(filter_type, cutoff, 0.707);
        self
    }

    /// Set the filter resonance (Q factor)
    ///
    /// Higher values create a peak at the cutoff frequency.
    /// Typical range: 0.5 to 10.0
    ///
    /// # Example
    /// ```ignore
    /// .synthesis(|s| s.filter(FilterType::LowPass, 1000.0).resonance(4.0))
    /// ```
    pub fn resonance(mut self, q: f32) -> Self {
        self.inner.get_track_mut().filter.resonance = q;
        self
    }

    /// Set the filter envelope (modulates cutoff over time)
    ///
    /// # Example
    /// ```ignore
    /// .synthesis(|s| s
    ///     .filter(FilterType::LowPass, 800.0)
    ///     .filter_env(FilterEnvelope::new(0.01, 0.2, 0.3, 0.5, 4000.0))
    /// )
    /// ```
    pub fn filter_env(mut self, env: FilterEnvelope) -> Self {
        self.inner.filter_envelope = env;
        self
    }

    /// Set filter envelope with ADSR and cutoff range
    ///
    /// # Arguments
    /// * `attack` - Attack time in seconds
    /// * `decay` - Decay time in seconds
    /// * `sustain` - Sustain level (0.0 to 1.0)
    /// * `release` - Release time in seconds
    /// * `base_cutoff` - Starting/resting cutoff frequency (Hz)
    /// * `peak_cutoff` - Maximum cutoff frequency (Hz)
    pub fn filter_adsr(mut self, attack: f32, decay: f32, sustain: f32, release: f32, base_cutoff: f32, peak_cutoff: f32) -> Self {
        self.inner.filter_envelope = FilterEnvelope::new(attack, decay, sustain, release, base_cutoff, peak_cutoff, 1.0);
        self
    }

    // ==================== Amplitude Envelope ====================

    /// Set the amplitude envelope
    ///
    /// # Example
    /// ```ignore
    /// .synthesis(|s| s.envelope(Envelope::pad()))
    /// ```
    pub fn envelope(mut self, env: Envelope) -> Self {
        self.inner.envelope = env;
        self
    }

    /// Set amplitude envelope with ADSR values
    ///
    /// # Arguments
    /// * `attack` - Attack time in seconds
    /// * `decay` - Decay time in seconds
    /// * `sustain` - Sustain level (0.0 to 1.0)
    /// * `release` - Release time in seconds
    pub fn adsr(mut self, attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        self.inner.envelope = Envelope::new(attack, decay, sustain, release);
        self
    }

    // ==================== FM Synthesis ====================

    /// Configure FM synthesis parameters
    ///
    /// # Example
    /// ```ignore
    /// .synthesis(|s| s.fm(FMParams::new(2.0, 1.0)))
    /// ```
    pub fn fm(mut self, params: FMParams) -> Self {
        self.inner.fm_params = params;
        self
    }

    /// Set FM synthesis with ratio and modulation index
    ///
    /// # Arguments
    /// * `ratio` - Frequency ratio of modulator to carrier
    /// * `index` - Modulation index (depth of FM effect)
    pub fn fm_ratio(mut self, ratio: f32, index: f32) -> Self {
        self.inner.fm_params = FMParams::new(ratio, index);
        self
    }

    // ==================== Modulation ====================

    /// Add an LFO modulation route
    ///
    /// # Arguments
    /// * `target` - What parameter to modulate (Pitch, Volume, FilterCutoff, etc.)
    /// * `waveform` - LFO waveform shape
    /// * `rate` - LFO frequency in Hz
    /// * `depth` - LFO depth/amount
    /// * `amount` - How much the LFO affects the target (multiplier)
    ///
    /// # Example
    /// ```ignore
    /// .synthesis(|s| s.lfo(ModTarget::Pitch, Waveform::Sine, 5.0, 0.5, 1.0))
    /// ```
    pub fn lfo(mut self, target: ModTarget, waveform: Waveform, rate: f32, depth: f32, amount: f32) -> Self {
        self.inner.get_track_mut().modulation.push(ModRoute {
            lfo: LFO::new(waveform, rate, depth),
            target,
            amount,
        });
        self
    }

    /// Add vibrato (pitch LFO with sine wave)
    ///
    /// # Arguments
    /// * `rate` - Vibrato speed in Hz (typical: 4-7 Hz)
    /// * `depth` - Vibrato depth in semitones (typical: 0.1-0.5)
    pub fn vibrato(self, rate: f32, depth: f32) -> Self {
        self.lfo(ModTarget::Pitch, Waveform::Sine, rate, depth, 1.0)
    }

    /// Add tremolo (volume LFO with sine wave)
    ///
    /// # Arguments
    /// * `rate` - Tremolo speed in Hz
    /// * `depth` - Tremolo depth (0.0 to 1.0)
    pub fn tremolo(self, rate: f32, depth: f32) -> Self {
        self.lfo(ModTarget::Volume, Waveform::Sine, rate, depth, 1.0)
    }

    // ==================== Sample-based Synthesis ====================

    /// Generate granular synthesis from a sample file
    ///
    /// Unlike other methods that configure note synthesis, this immediately
    /// generates audio by chopping a sample into grains and reassembling them.
    ///
    /// # Arguments
    /// * `sample_path` - Path to the source audio file
    /// * `params` - Granular synthesis parameters
    /// * `duration` - Output duration in seconds
    ///
    /// # Example
    /// ```ignore
    /// .synthesis(|s| s
    ///     .granular("voice.wav", GranularParams::texture(), 5.0)
    /// )
    /// ```
    pub fn granular(mut self, sample_path: &str, params: GranularParams, duration: f32) -> Self {
        // Load the sample
        let source_sample = match Sample::from_file(sample_path) {
            Ok(sample) => sample,
            Err(e) => {
                eprintln!("Error loading sample '{}': {}", sample_path, e);
                return self;
            }
        };

        // Capture cursor before mutable borrow
        let start_time = self.inner.cursor;

        // Generate all the grain events
        let grain_events = create_granular_events(&source_sample, &params, duration, start_time);

        // Add all events to the track
        let track = self.inner.get_track_mut();
        for event in grain_events {
            track.events.push(AudioEvent::Sample(event));
        }

        // Advance cursor
        self.inner.cursor += duration;

        self
    }
}
