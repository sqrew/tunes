use crate::drums::DrumType;
use crate::effects::{BitCrusher, Chorus, Compressor, Delay, Distortion, EQ, Flanger, Phaser, Reverb, RingModulator, Saturation};
use crate::envelope::Envelope;
use crate::filter::Filter;
use crate::lfo::ModRoute;
use crate::waveform::Waveform;

/// Represents different types of audio events
#[derive(Debug, Clone, Copy)]
pub enum AudioEvent {
    Note(NoteEvent),
    Drum(DrumEvent),
}

/// Represents a note event with timing information
#[derive(Debug, Clone, Copy)]
pub struct NoteEvent {
    pub frequencies: [f32; 8], // Support up to 8 simultaneous frequencies
    pub num_freqs: usize,
    pub start_time: f32,    // When to start playing (in seconds from track start)
    pub duration: f32,      // How long to play (in seconds)
    pub waveform: Waveform, // Waveform type to use
    pub envelope: Envelope, // ADSR envelope
    pub pitch_bend_semitones: f32, // Pitch bend amount in semitones (0.0 = no bend)
}

/// Represents a drum hit event
#[derive(Debug, Clone, Copy)]
pub struct DrumEvent {
    pub drum_type: DrumType,
    pub start_time: f32,
}

impl NoteEvent {
    pub fn new(frequencies: &[f32], start_time: f32, duration: f32) -> Self {
        Self::with_waveform(frequencies, start_time, duration, Waveform::Sine)
    }

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

    pub fn with_waveform_envelope_and_bend(
        frequencies: &[f32],
        start_time: f32,
        duration: f32,
        waveform: Waveform,
        envelope: Envelope,
        pitch_bend_semitones: f32,
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
            pitch_bend_semitones,
        }
    }
}

/// A track contains a sequence of audio events (notes and drums)
#[derive(Debug, Clone)]
pub struct Track {
    pub events: Vec<AudioEvent>,
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
}

impl Track {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
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
        }
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 2.0);
        self
    }

    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.filter = filter;
        self
    }

    pub fn with_delay(mut self, delay: Delay) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn with_reverb(mut self, reverb: Reverb) -> Self {
        self.reverb = Some(reverb);
        self
    }

    pub fn with_distortion(mut self, distortion: Distortion) -> Self {
        self.distortion = Some(distortion);
        self
    }

    pub fn with_modulation(mut self, mod_route: ModRoute) -> Self {
        self.modulation.push(mod_route);
        self
    }

    /// Add a note event to the track
    pub fn add_note(&mut self, frequencies: &[f32], start_time: f32, duration: f32) {
        self.events.push(AudioEvent::Note(NoteEvent::new(
            frequencies,
            start_time,
            duration,
        )));
    }

    /// Add a note event with a specific waveform
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
    }

    /// Add a note event with waveform and envelope
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
    }

    /// Add a note event with waveform, envelope, and pitch bend
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
    }

    /// Add a drum event to the track
    pub fn add_drum(&mut self, drum_type: DrumType, start_time: f32) {
        self.events.push(AudioEvent::Drum(DrumEvent {
            drum_type,
            start_time,
        }));
    }

    /// Add a sequence of notes with equal duration
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

    /// Get the total duration of the track
    pub fn total_duration(&self) -> f32 {
        self.events
            .iter()
            .map(|e| match e {
                AudioEvent::Note(n) => n.start_time + n.duration,
                AudioEvent::Drum(d) => d.start_time + d.drum_type.duration(),
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
}

impl Mixer {
    pub fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    /// Get the total duration across all tracks
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
    /// # use musicrs::composition::Composition;
    /// # use musicrs::rhythm::Tempo;
    /// # use musicrs::engine::AudioEngine;
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
                    }
                }
            }
        }

        self
    }

    /// Generate a stereo sample at a given time by mixing all active tracks
    /// Returns (left, right) channel values
    pub fn sample_at(&mut self, time: f32, sample_rate: f32, _sample_clock: f32) -> (f32, f32) {
        let mut mixed_left = 0.0;
        let mut mixed_right = 0.0;

        for track in &mut self.tracks {
            let mut track_value = 0.0;

            for event in &track.events {
                match event {
                    AudioEvent::Note(note_event) => {
                        let total_duration =
                            note_event.envelope.total_duration(note_event.duration);
                        let note_end_with_release = note_event.start_time + total_duration;

                        // Check if this note event is active (including release phase)
                        if time >= note_event.start_time && time < note_end_with_release {
                            // Calculate time within the note
                            let time_in_note = time - note_event.start_time;

                            // Get envelope amplitude at this point in time
                            let envelope_amp = note_event
                                .envelope
                                .amplitude_at(time_in_note, note_event.duration);

                            // Generate waves for all frequencies in this event using the specified waveform
                            for i in 0..note_event.num_freqs {
                                let base_freq = note_event.frequencies[i];

                                // Apply pitch bend (linear over note duration)
                                let bend_progress = (time_in_note / note_event.duration).min(1.0);
                                let bend_multiplier = 2.0f32
                                    .powf((note_event.pitch_bend_semitones * bend_progress) / 12.0);
                                let freq = base_freq * bend_multiplier;

                                // Calculate phase relative to note start time to avoid clicks
                                let phase = (time_in_note * freq) % 1.0;
                                let sample = note_event.waveform.sample(phase);
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
                            // Calculate sample index relative to drum start
                            let time_in_drum = time - drum_event.start_time;
                            let sample_index = (time_in_drum * sample_rate) as usize;
                            track_value += drum_event.drum_type.sample(sample_index, sample_rate);
                        }
                    }
                }
            }

            // Apply modulation
            let mut modulated_volume = track.volume;
            let mut modulated_cutoff = track.filter.cutoff;
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
                    track_value = reverb.process(track_value);
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
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new()
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

        match track.events[0] {
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

        match track.events[0] {
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
        if let AudioEvent::Note(note) = track.events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = track.events[1] {
            assert_eq!(note.start_time, 0.5);
        }
        if let AudioEvent::Note(note) = track.events[2] {
            assert_eq!(note.start_time, 1.0);
        }
    }

    #[test]
    fn test_mixer_creation() {
        let mixer = Mixer::new();
        assert_eq!(mixer.tracks.len(), 0);
    }

    #[test]
    fn test_mixer_add_track() {
        let mut mixer = Mixer::new();
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
        let mut mixer = Mixer::new();

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
        let mixer = Mixer::new();
        assert_eq!(mixer.total_duration(), 0.0);
    }

    #[test]
    fn test_mixer_repeat() {
        let mut mixer = Mixer::new();
        let mut track = Track::new();
        track.add_note(&[440.0], 0.0, 1.0); // Ends at 1.0
        mixer.add_track(track);

        let repeated_mixer = mixer.repeat(2); // Repeat 2 MORE times

        // Should still have 1 track (repeats events within the track)
        assert_eq!(repeated_mixer.tracks.len(), 1);

        // Track should now have 3 events total (original + 2 repeats)
        assert_eq!(repeated_mixer.tracks[0].events.len(), 3);

        // Verify timing offsets
        if let AudioEvent::Note(note) = repeated_mixer.tracks[0].events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = repeated_mixer.tracks[0].events[1] {
            assert_eq!(note.start_time, 1.0); // Original duration offset
        }
        if let AudioEvent::Note(note) = repeated_mixer.tracks[0].events[2] {
            assert_eq!(note.start_time, 2.0); // 2x original duration
        }
    }

    #[test]
    fn test_mixer_repeat_zero_times() {
        let mut mixer = Mixer::new();
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
