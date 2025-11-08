//! Mixer implementation
//!
//! The mixer combines multiple tracks together and handles the core audio rendering.

use super::events::*;
use super::track::Track;
use crate::composition::rhythm::Tempo;

/// Mix multiple tracks together
#[derive(Debug, Clone)]
pub struct Mixer {
    pub tracks: Vec<Track>,
    pub tempo: Tempo,
    pub(super) sample_count: u64, // For quantized automation lookups
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
                            track.add_drum(drum.drum_type, drum.start_time + offset);
                        }
                        AudioEvent::Sample(sample) => {
                            track
                                .events
                                .push(AudioEvent::Sample(crate::track::SampleEvent {
                                    sample: sample.sample.clone(),
                                    start_time: sample.start_time + offset,
                                    playback_rate: sample.playback_rate,
                                    volume: sample.volume,
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
                && track.delay.is_none()
                && track.reverb.is_none()
            {
                continue;
            }

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
                            let (sample_left, sample_right) =
                                sample_event.sample.sample_at_interpolated(
                                    time_in_sample,
                                    sample_event.playback_rate,
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
            if !has_active_event && track.delay.is_none() && track.reverb.is_none() {
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
            if track_value.abs() > 0.0001 || track.delay.is_some() || track.reverb.is_some() {
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

                // Apply effects in pre-computed priority order
                // Effect IDs: 0=EQ, 1=Compressor, 2=Gate, 3=Saturation, 4=BitCrusher, 5=Distortion,
                //             6=Chorus, 7=Phaser, 8=Flanger, 9=RingMod, 10=Tremolo,
                //             11=Delay, 12=Reverb, 13=Limiter
                // Note: AutoPan is handled separately at the stereo stage
                for &effect_id in &track.effect_order {
                    match effect_id {
                        0 => {
                            // EQ
                            if let Some(ref mut eq) = track.eq {
                                track_value =
                                    eq.process(track_value, sample_rate, time, self.sample_count);
                            }
                        }
                        1 => {
                            // Compressor
                            if let Some(ref mut compressor) = track.compressor {
                                track_value = compressor.process(
                                    track_value,
                                    sample_rate,
                                    time,
                                    self.sample_count,
                                );
                            }
                        }
                        2 => {
                            // Gate
                            if let Some(ref mut gate) = track.gate {
                                track_value =
                                    gate.process(track_value, sample_rate, time, self.sample_count);
                            }
                        }
                        3 => {
                            // Saturation
                            if let Some(ref mut saturation) = track.saturation {
                                track_value =
                                    saturation.process(track_value, time, self.sample_count);
                            }
                        }
                        4 => {
                            // BitCrusher
                            if let Some(ref mut bitcrusher) = track.bitcrusher {
                                track_value =
                                    bitcrusher.process(track_value, time, self.sample_count);
                            }
                        }
                        5 => {
                            // Distortion
                            if let Some(ref mut distortion) = track.distortion {
                                track_value =
                                    distortion.process(track_value, time, self.sample_count);
                            }
                        }
                        6 => {
                            // Chorus
                            if let Some(ref mut chorus) = track.chorus {
                                track_value = chorus.process(
                                    track_value,
                                    sample_rate,
                                    time,
                                    self.sample_count,
                                );
                            }
                        }
                        7 => {
                            // Phaser
                            if let Some(ref mut phaser) = track.phaser {
                                track_value = phaser.process(track_value, time, self.sample_count);
                            }
                        }
                        8 => {
                            // Flanger
                            if let Some(ref mut flanger) = track.flanger {
                                track_value = flanger.process(track_value, time, self.sample_count);
                            }
                        }
                        9 => {
                            // Ring Modulator
                            if let Some(ref mut ring_mod) = track.ring_mod {
                                track_value =
                                    ring_mod.process(track_value, time, self.sample_count);
                            }
                        }
                        10 => {
                            // Tremolo
                            if let Some(ref mut tremolo) = track.tremolo {
                                track_value = tremolo.process(track_value, time, self.sample_count);
                            }
                        }
                        11 => {
                            // Delay
                            if let Some(ref mut delay) = track.delay {
                                track_value = delay.process(track_value, time, self.sample_count);
                            }
                        }
                        12 => {
                            // Reverb
                            if let Some(ref mut reverb) = track.reverb {
                                track_value = reverb.process(track_value, time, self.sample_count);
                            }
                        }
                        13 => {
                            // Limiter
                            if let Some(ref mut limiter) = track.limiter {
                                track_value = limiter.process(
                                    track_value,
                                    sample_rate,
                                    time,
                                    self.sample_count,
                                );
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }

            // Apply stereo panning using constant power panning
            // pan: -1.0 (full left), 0.0 (center), 1.0 (full right)

            // Add AutoPan offset if present
            let pan_offset = if let Some(ref mut autopan) = track.autopan {
                autopan.get_pan_offset(time, self.sample_count)
            } else {
                0.0
            };

            let pan_clamped = (track.pan + pan_offset).clamp(-1.0, 1.0);
            let pan_angle = (pan_clamped + 1.0) * 0.25 * std::f32::consts::PI; // 0 to PI/2
            let left_gain = pan_angle.cos();
            let right_gain = pan_angle.sin();

            // Add to stereo mix
            mixed_left += track_value * left_gain;
            mixed_right += track_value * right_gain;
        }

        // Apply soft clipping to prevent harsh distortion
        // tanh provides smooth saturation - maintains dynamics while preventing clipping
        // This is much better than dividing by track count, which unnecessarily
        // reduces volume even when tracks don't play simultaneously
        (mixed_left.tanh(), mixed_right.tanh())
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
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new(Tempo::new(120.0))
    }
}
