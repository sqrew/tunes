//! Mixer implementation
//!
//! The mixer combines multiple tracks together and handles the core audio rendering.

use super::events::*;
use super::track::Track;
use crate::composition::rhythm::Tempo;
use crate::synthesis::effects::EffectChain;

/// Mix multiple tracks together
#[derive(Debug, Clone)]
pub struct Mixer {
    pub tracks: Vec<Track>,
    pub tempo: Tempo,
    pub(super) sample_count: u64, // For quantized automation lookups
    pub master: EffectChain,      // Master effects chain (stereo processing)
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
            sample_count: 0,
            master: EffectChain::new(),
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

    /// Check if the mixer has any audio events
    ///
    /// Returns `true` if all tracks are empty (no notes, drums, or samples).
    /// Useful for detecting empty compositions before playback.
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mixer = comp.into_mixer();
    /// assert!(mixer.is_empty());
    ///
    /// let mut comp2 = Composition::new(Tempo::new(120.0));
    /// comp2.instrument("piano", &Instrument::electric_piano())
    ///     .note(&[440.0], 1.0);
    /// let mixer2 = comp2.into_mixer();
    /// assert!(!mixer2.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.tracks.iter().all(|t| t.events.is_empty())
    }

    /// Repeat all tracks in the mixer N times
    ///
    /// This duplicates all events in all tracks, placing copies sequentially.
    /// Useful for looping an entire composition.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
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
                            track.add_drum(
                                drum.drum_type,
                                drum.start_time + offset,
                                drum.spatial_position,
                            );
                        }
                        AudioEvent::Sample(sample) => {
                            track
                                .events
                                .push(AudioEvent::Sample(crate::track::SampleEvent {
                                    sample: sample.sample.clone(),
                                    start_time: sample.start_time + offset,
                                    playback_rate: sample.playback_rate,
                                    volume: sample.volume,
                                    spatial_position: sample.spatial_position,
                                }));
                            track.invalidate_time_cache();
                        }
                        AudioEvent::TempoChange(tempo) => {
                            track.events.push(AudioEvent::TempoChange(
                                crate::track::TempoChangeEvent {
                                    start_time: tempo.start_time + offset,
                                    bpm: tempo.bpm,
                                },
                            ));
                            track.invalidate_time_cache();
                        }
                        AudioEvent::TimeSignature(time_sig) => {
                            track.events.push(AudioEvent::TimeSignature(
                                crate::track::TimeSignatureEvent {
                                    start_time: time_sig.start_time + offset,
                                    numerator: time_sig.numerator,
                                    denominator: time_sig.denominator,
                                },
                            ));
                            track.invalidate_time_cache();
                        }
                        AudioEvent::KeySignature(key_sig) => {
                            track
                                .events
                                .push(AudioEvent::KeySignature(KeySignatureEvent {
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
    /// 3. Applying event-level spatial audio (if spatial_position is set on events)
    /// 4. Applying track-level effects (filter, reverb, delay, etc.)
    /// 5. Mixing tracks with stereo panning
    ///
    /// # Arguments
    /// * `time` - The time position in seconds
    /// * `sample_rate` - Sample rate in Hz (e.g., 44100)
    /// * `_sample_clock` - Reserved for future use
    /// * `listener` - Optional listener configuration for spatial audio
    /// * `spatial_params` - Optional spatial audio parameters
    ///
    /// # Returns
    /// A tuple of (left_channel, right_channel) audio samples in range -1.0 to 1.0
    pub fn sample_at(
        &mut self,
        time: f32,
        sample_rate: f32,
        _sample_clock: f32,
        listener: Option<&crate::synthesis::spatial::ListenerConfig>,
        spatial_params: Option<&crate::synthesis::spatial::SpatialParams>,
    ) -> (f32, f32) {
        // Increment sample count for quantized automation lookups
        self.sample_count = self.sample_count.wrapping_add(1);

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
                && track.effects.delay.is_none()
                && track.effects.reverb.is_none()
            {
                continue;
            }

            // Check if any event in this track has a spatial position
            // If so, we'll use it for spatial audio for the whole track
            let track_spatial_position = if listener.is_some() && spatial_params.is_some() {
                track.events.iter().find_map(|event| match event {
                    AudioEvent::Note(note) => note.spatial_position,
                    AudioEvent::Drum(drum) => drum.spatial_position,
                    AudioEvent::Sample(sample) => sample.spatial_position,
                    _ => None,
                })
            } else {
                None
            };

            let mut track_value = 0.0;
            let mut has_active_event = false;
            // Store the base filter parameters for proper modulation (don't use potentially modulated values)
            let base_filter_cutoff = track.filter.cutoff;
            let base_filter_resonance = track.filter.resonance;
            let mut filter_env_cutoff = base_filter_cutoff;
            let mut filter_env_found = false;

            // Binary search to find potentially active events (O(log n) instead of O(n))
            let (start_idx, end_idx) = track.find_active_range(time);

            // Only iterate through events that could possibly be active
            for event in &track.events[start_idx..end_idx] {
                match event {
                    AudioEvent::Sample(sample_event) => {
                        let time_in_sample = time - sample_event.start_time;
                        let sample_duration =
                            sample_event.sample.duration / sample_event.playback_rate;

                        if time_in_sample >= 0.0 && time_in_sample < sample_duration {
                            has_active_event = true;
                            let (sample_left, sample_right) = sample_event
                                .sample
                                .sample_at_interpolated(time_in_sample, sample_event.playback_rate);
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
                                let filter_total_duration = note_event
                                    .filter_envelope
                                    .total_duration(note_event.duration);
                                let filter_end = note_event.start_time + filter_total_duration;
                                if time >= note_event.start_time && time < filter_end {
                                    filter_env_cutoff = note_event
                                        .filter_envelope
                                        .cutoff_at(time_in_note, note_event.duration);
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
                                    let bend_progress =
                                        (time_in_note / note_event.duration).min(1.0);
                                    let bend_multiplier = 2.0f32.powf(
                                        (note_event.pitch_bend_semitones * bend_progress) / 12.0,
                                    );
                                    base_freq * bend_multiplier
                                } else {
                                    base_freq
                                };

                                let sample = if note_event.fm_params.mod_index > 0.0 {
                                    // Use FM synthesis
                                    note_event.fm_params.sample(
                                        freq,
                                        time_in_note,
                                        note_event.duration,
                                    )
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
            if !has_active_event && track.effects.delay.is_none() && track.effects.reverb.is_none()
            {
                continue;
            }

            // Filter envelope was already collected in the event loop above

            // Apply LFO modulation on top of filter envelope
            let mut modulated_volume = track.volume;
            let mut modulated_cutoff = filter_env_cutoff;
            let mut modulated_resonance = base_filter_resonance;

            for mod_route in &mut track.modulation {
                // Tick the LFO to advance its phase
                mod_route.lfo.tick();

                match mod_route.target {
                    crate::synthesis::lfo::ModTarget::Volume => {
                        modulated_volume = mod_route.apply(modulated_volume);
                    }
                    crate::synthesis::lfo::ModTarget::FilterCutoff => {
                        modulated_cutoff = mod_route.apply(modulated_cutoff);
                    }
                    crate::synthesis::lfo::ModTarget::FilterResonance => {
                        modulated_resonance = mod_route.apply(modulated_resonance);
                    }
                    _ => {} // Other modulation targets handled elsewhere
                }
            }

            // Only process effects if there's actual audio
            if track_value.abs() > 0.0001
                || track.effects.delay.is_some()
                || track.effects.reverb.is_some()
            {
                // Apply track volume (with modulation)
                track_value *= modulated_volume;

                // Apply filter (with modulation)
                // Temporarily set modulated values, process, then restore base values
                track.filter.cutoff = modulated_cutoff;
                track.filter.resonance = modulated_resonance;
                track_value = track.filter.process(track_value, sample_rate);
                // Restore base values to prevent compounding modulation on next sample
                track.filter.cutoff = base_filter_cutoff;
                track.filter.resonance = base_filter_resonance;

                // Apply effects through the unified effect chain
                track_value =
                    track
                        .effects
                        .process_mono(track_value, sample_rate, time, self.sample_count);
            }

            // Apply spatial audio or stereo panning
            let (final_volume, final_pan) = if let Some(pos) = track_spatial_position {
                // Apply spatial audio for this track
                let listener_cfg = listener.unwrap();
                let spatial_cfg = spatial_params.unwrap();
                let result =
                    crate::synthesis::spatial::calculate_spatial(&pos, listener_cfg, spatial_cfg);
                (result.volume, result.pan)
            } else {
                // Use normal panning
                // Add AutoPan offset if present
                let pan_offset = if let Some(ref mut autopan) = track.effects.autopan {
                    autopan.get_pan_offset(time, self.sample_count)
                } else {
                    0.0
                };
                (1.0, (track.pan + pan_offset).clamp(-1.0, 1.0))
            };

            // Apply volume attenuation
            let attenuated_value = track_value * final_volume;

            // Apply stereo panning using constant power panning
            // pan: -1.0 (full left), 0.0 (center), 1.0 (full right)
            let pan_angle = (final_pan + 1.0) * 0.25 * std::f32::consts::PI; // 0 to PI/2
            let left_gain = pan_angle.cos();
            let right_gain = pan_angle.sin();

            // Add to stereo mix
            mixed_left += attenuated_value * left_gain;
            mixed_right += attenuated_value * right_gain;
        }

        // Apply master effects (stereo processing)
        let (master_left, master_right) = self.master.process_stereo(
            mixed_left,
            mixed_right,
            sample_rate,
            time,
            self.sample_count,
        );

        // Apply soft clipping to prevent harsh distortion
        // tanh provides smooth saturation - maintains dynamics while preventing clipping
        // This is much better than dividing by track count, which unnecessarily
        // reduces volume even when tracks don't play simultaneously
        (master_left.tanh(), master_right.tanh())
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
            let (left, right) = self.sample_at(time, sample_rate, sample_clock, None, None);

            // Clamp to valid range and add to buffer
            buffer.push(left.clamp(-1.0, 1.0));
            buffer.push(right.clamp(-1.0, 1.0));

            sample_clock = (sample_clock + 1.0) % sample_rate;
        }

        buffer
    }

    /// Add a compressor to the master output
    ///
    /// Applies dynamic range compression to the final stereo mix. Master compression
    /// uses stereo-linked processing to preserve the stereo image.
    ///
    /// # Arguments
    /// * `compressor` - Compressor effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Compressor;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_compressor(Compressor::new(-10.0, 4.0, 0.01, 0.1, 2.0));
    /// ```
    pub fn master_compressor(&mut self, compressor: crate::synthesis::effects::Compressor) {
        self.master.compressor = Some(compressor);
        self.master.compute_effect_order();
    }

    /// Add a limiter to the master output
    ///
    /// Applies limiting to prevent clipping on the final stereo mix. Master limiting
    /// uses stereo-linked processing to preserve the stereo image. This is typically
    /// the last effect in the master chain.
    ///
    /// # Arguments
    /// * `limiter` - Limiter effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Limiter;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_limiter(Limiter::new(0.0, 0.01));
    /// ```
    pub fn master_limiter(&mut self, limiter: crate::synthesis::effects::Limiter) {
        self.master.limiter = Some(limiter);
        self.master.compute_effect_order();
    }

    /// Add EQ to the master output
    ///
    /// Applies 3-band equalization to the final stereo mix.
    ///
    /// # Arguments
    /// * `eq` - EQ effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::EQ;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_eq(EQ::new(1.5, 1.0, 1.2, 200.0, 3000.0));
    /// ```
    pub fn master_eq(&mut self, eq: crate::synthesis::effects::EQ) {
        self.master.eq = Some(eq);
        self.master.compute_effect_order();
    }

    /// Add parametric EQ to the master output
    ///
    /// Applies multi-band parametric equalization to the final stereo mix for
    /// precise frequency shaping and mastering.
    ///
    /// # Arguments
    /// * `parametric_eq` - ParametricEQ effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::ParametricEQ;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// let eq = ParametricEQ::new()
    ///     .band(100.0, -3.0, 0.7)  // Cut low rumble
    ///     .band(3000.0, 2.0, 1.5); // Boost presence
    /// mixer.master_parametric_eq(eq);
    /// ```
    pub fn master_parametric_eq(&mut self, parametric_eq: crate::synthesis::effects::ParametricEQ) {
        self.master.parametric_eq = Some(parametric_eq);
        self.master.compute_effect_order();
    }

    /// Add reverb to the master output
    ///
    /// Applies reverb to the final stereo mix. Use sparingly as master reverb
    /// affects the entire mix.
    ///
    /// # Arguments
    /// * `reverb` - Reverb effect configuration
    pub fn master_reverb(&mut self, reverb: crate::synthesis::effects::Reverb) {
        self.master.reverb = Some(reverb);
        self.master.compute_effect_order();
    }

    /// Add delay to the master output
    ///
    /// Applies delay to the final stereo mix.
    ///
    /// # Arguments
    /// * `delay` - Delay effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Delay;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_delay(Delay::new(0.5, 0.4, 0.3));
    /// ```
    pub fn master_delay(&mut self, delay: crate::synthesis::effects::Delay) {
        self.master.delay = Some(delay);
        self.master.compute_effect_order();
    }

    /// Add gate to the master output
    ///
    /// Applies noise gate to the final stereo mix. Useful for cutting unwanted
    /// background noise or creating rhythmic gating effects.
    ///
    /// # Arguments
    /// * `gate` - Gate effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Gate;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_gate(Gate::new(-40.0, 4.0, 0.01, 0.1));
    /// ```
    pub fn master_gate(&mut self, gate: crate::synthesis::effects::Gate) {
        self.master.gate = Some(gate);
        self.master.compute_effect_order();
    }

    /// Add saturation to the master output
    ///
    /// Applies saturation/warmth to the final stereo mix.
    ///
    /// # Arguments
    /// * `saturation` - Saturation effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Saturation;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_saturation(Saturation::new(2.0, 0.5, 1.0));
    /// ```
    pub fn master_saturation(&mut self, saturation: crate::synthesis::effects::Saturation) {
        self.master.saturation = Some(saturation);
        self.master.compute_effect_order();
    }

    /// Add bit crusher to the master output
    ///
    /// Applies bit reduction and sample rate reduction to the final stereo mix
    /// for lo-fi effects.
    ///
    /// # Arguments
    /// * `bitcrusher` - BitCrusher effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::BitCrusher;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_bitcrusher(BitCrusher::new(8.0, 2.0, 1.0));
    /// ```
    pub fn master_bitcrusher(&mut self, bitcrusher: crate::synthesis::effects::BitCrusher) {
        self.master.bitcrusher = Some(bitcrusher);
        self.master.compute_effect_order();
    }

    /// Add distortion to the master output
    ///
    /// Applies distortion to the final stereo mix.
    ///
    /// # Arguments
    /// * `distortion` - Distortion effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Distortion;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_distortion(Distortion::new(2.0, 0.5));
    /// ```
    pub fn master_distortion(&mut self, distortion: crate::synthesis::effects::Distortion) {
        self.master.distortion = Some(distortion);
        self.master.compute_effect_order();
    }

    /// Add chorus to the master output
    ///
    /// Applies chorus modulation to the final stereo mix for widening effects.
    ///
    /// # Arguments
    /// * `chorus` - Chorus effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Chorus;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_chorus(Chorus::new(0.003, 0.5, 0.3));
    /// ```
    pub fn master_chorus(&mut self, chorus: crate::synthesis::effects::Chorus) {
        self.master.chorus = Some(chorus);
        self.master.compute_effect_order();
    }

    /// Add phaser to the master output
    ///
    /// Applies phaser modulation to the final stereo mix.
    ///
    /// # Arguments
    /// * `phaser` - Phaser effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Phaser;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_phaser(Phaser::new(0.5, 0.7, 0.5, 0.5, 4));
    /// ```
    pub fn master_phaser(&mut self, phaser: crate::synthesis::effects::Phaser) {
        self.master.phaser = Some(phaser);
        self.master.compute_effect_order();
    }

    /// Add flanger to the master output
    ///
    /// Applies flanger modulation to the final stereo mix.
    ///
    /// # Arguments
    /// * `flanger` - Flanger effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Flanger;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_flanger(Flanger::new(0.5, 3.0, 0.7, 0.5));
    /// ```
    pub fn master_flanger(&mut self, flanger: crate::synthesis::effects::Flanger) {
        self.master.flanger = Some(flanger);
        self.master.compute_effect_order();
    }

    /// Add ring modulator to the master output
    ///
    /// Applies ring modulation to the final stereo mix for metallic/robotic effects.
    ///
    /// # Arguments
    /// * `ring_mod` - RingModulator effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::RingModulator;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_ring_mod(RingModulator::new(30.0, 0.5));
    /// ```
    pub fn master_ring_mod(&mut self, ring_mod: crate::synthesis::effects::RingModulator) {
        self.master.ring_mod = Some(ring_mod);
        self.master.compute_effect_order();
    }

    /// Add tremolo to the master output
    ///
    /// Applies tremolo (amplitude modulation) to the final stereo mix.
    ///
    /// # Arguments
    /// * `tremolo` - Tremolo effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::Tremolo;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_tremolo(Tremolo::new(4.0, 0.5));
    /// ```
    pub fn master_tremolo(&mut self, tremolo: crate::synthesis::effects::Tremolo) {
        self.master.tremolo = Some(tremolo);
        self.master.compute_effect_order();
    }

    /// Add auto-pan to the master output
    ///
    /// Applies automatic panning to the final stereo mix, moving the sound
    /// between left and right channels.
    ///
    /// # Arguments
    /// * `autopan` - AutoPan effect configuration
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::synthesis::effects::AutoPan;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// let mut mixer = comp.into_mixer();
    /// mixer.master_autopan(AutoPan::new(0.25, 1.0));
    /// ```
    pub fn master_autopan(&mut self, autopan: crate::synthesis::effects::AutoPan) {
        self.master.autopan = Some(autopan);
        self.master.compute_effect_order();
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new(Tempo::new(120.0))
    }
}
