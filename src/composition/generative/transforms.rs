//! Pattern transformation methods

use crate::composition::TrackBuilder;
use crate::track::AudioEvent;
use crate::synthesis::waveform::Waveform;
use crate::synthesis::envelope::Envelope;
use rand::Rng;

impl<'a> TrackBuilder<'a> {
    /// Add human feel to pattern by randomizing timing and velocity
    ///
    /// Makes programmed music feel more natural by adding slight random variations
    /// to note timing and velocity within the pattern.
    ///
    /// # Arguments
    /// * `timing_variance` - Max timing offset in seconds (e.g., 0.02 = ±20ms)
    /// * `velocity_variance` - Max velocity change (e.g., 0.1 = ±10%)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("piano")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .humanize(0.02, 0.1);  // Subtle humanization
    ///
    /// comp.track("drums")
    ///     .pattern_start()
    ///     .notes(&[C4, C4, C4, C4], 0.25)
    ///     .humanize(0.005, 0.15);  // Tight timing, varied velocity
    /// ```
    pub fn humanize(mut self, timing_variance: f32, velocity_variance: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Apply humanization to notes in the pattern
        for event in &mut self.get_track_mut().events {
            let event_time = match event {
                AudioEvent::Note(note) => note.start_time,
                AudioEvent::Drum(drum) => drum.start_time,
                _ => continue,
            };

            if event_time >= pattern_start && event_time < cursor {
                match event {
                    AudioEvent::Note(note) => {
                        // Randomize timing
                        use rand::Rng;
                        let mut rng = rand::rng();
                        let timing_offset = rng.random_range(-timing_variance..=timing_variance);
                        note.start_time = (note.start_time + timing_offset).max(0.0);

                        // Randomize velocity
                        let velocity_offset = rng.random_range(-velocity_variance..=velocity_variance);
                        note.velocity = (note.velocity + velocity_offset).clamp(0.0, 1.0);
                    }
                    AudioEvent::Drum(drum) => {
                        // Randomize drum timing only
                        use rand::Rng;
                        let mut rng = rand::rng();
                        let timing_offset = rng.random_range(-timing_variance..=timing_variance);
                        drum.start_time = (drum.start_time + timing_offset).max(0.0);
                    }
                    _ => {}
                }
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Rotate notes in the pattern by n positions
    ///
    /// Cycles the pitch sequence while keeping timing the same.
    /// Positive values rotate forward, negative rotate backward.
    ///
    /// # Arguments
    /// * `positions` - Number of positions to rotate (positive = forward, negative = backward)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Original: C4, E4, G4, C5
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .rotate(1);  // Result: E4, G4, C5, C4
    ///
    /// comp.track("bass")
    ///     .pattern_start()
    ///     .notes(&[C3, E3, G3], 0.5)
    ///     .rotate(-1);  // Result: G3, C3, E3
    /// ```
    pub fn rotate(mut self, positions: i32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || positions == 0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect note events in the pattern
        let mut note_events: Vec<(usize, [f32; 8], usize)> = Vec::new();
        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    note_events.push((idx, note.frequencies, note.num_freqs));
                }
            }
        }

        if note_events.is_empty() {
            return self;
        }

        // Extract frequencies in order
        let freqs: Vec<([f32; 8], usize)> = note_events
            .iter()
            .map(|(_, f, n)| (*f, *n))
            .collect();

        // Rotate the frequencies
        let len = freqs.len() as i32;
        let normalized_rotation = ((positions % len) + len) % len; // Handle negative rotations

        // Apply rotated frequencies back to the notes
        for (i, (event_idx, _, _)) in note_events.iter().enumerate() {
            let rotated_idx = ((i as i32 + normalized_rotation) % len) as usize;
            if let AudioEvent::Note(note) = &mut self.get_track_mut().events[*event_idx] {
                note.frequencies = freqs[rotated_idx].0;
                note.num_freqs = freqs[rotated_idx].1;
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Mutate pitches by random semitone offsets (evolutionary variation)
    ///
    /// Randomly adjusts each note by up to ±amount semitones, creating subtle to dramatic
    /// variations while maintaining the overall melodic shape. Great for generative music
    /// and creating organic variations of existing patterns.
    ///
    /// # Arguments
    /// * `max_semitones` - Maximum random shift in semitones (positive values only)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Subtle variation - like a slightly drunk pianist
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .mutate(1);  // Each note shifts by -1, 0, or +1 semitones
    ///
    /// // Dramatic variation
    /// comp.track("wild")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .mutate(5);  // Each note can shift by -5 to +5 semitones
    /// ```
    pub fn mutate(mut self, max_semitones: i32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || max_semitones == 0 {
            return self;
        }

        let max_semitones = max_semitones.abs(); // Ensure positive
        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        use rand::Rng;
        let mut rng = rand::rng();

        // Mutate notes in the pattern
        for event in &mut self.get_track_mut().events {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    // Apply random mutation to each frequency in the note/chord
                    for i in 0..note.num_freqs {
                        // Random offset: -max_semitones to +max_semitones
                        let offset = rng.random_range(-max_semitones..=max_semitones);
                        if offset != 0 {
                            let shift_ratio = 2.0_f32.powf(offset as f32 / 12.0);
                            note.frequencies[i] *= shift_ratio;
                        }
                    }
                }
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Stack harmonic layers on each note
    ///
    /// Adds `count` additional voices to each note, each shifted by `semitones` from the previous.
    /// Creates thick unison sounds, octave stacking, or complex harmonic layers - a fundamental
    /// technique in music production for making sounds bigger and richer.
    ///
    /// # Arguments
    /// * `semitones` - Semitone interval between each layer (can be negative)
    /// * `count` - Number of layers to add (1 = two voices, 2 = three voices, etc.)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Classic octave stacking - C4 becomes [C4, C5] playing simultaneously
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.5)
    ///     .stack(12, 1);  // Stack octave above each note
    ///
    /// // Thick three-octave unison - C4 becomes [C4, C5, C6]
    /// comp.track("lead")
    ///     .pattern_start()
    ///     .notes(&[C4], 1.0)
    ///     .stack(12, 2);  // Stack two octaves
    ///
    /// // Stack perfect fifth and major ninth - C4 becomes [C4, G4, D5]
    /// comp.track("chord")
    ///     .pattern_start()
    ///     .notes(&[C4], 1.0)
    ///     .stack(7, 2);
    ///
    /// // Bass reinforcement - stack octave below
    /// comp.track("bass")
    ///     .pattern_start()
    ///     .notes(&[C4], 1.0)
    ///     .stack(-12, 1);
    /// ```
    pub fn stack(mut self, semitones: i32, count: usize) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || count == 0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Duplicate notes in the pattern
        for event in &mut self.get_track_mut().events {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    let original_count = note.num_freqs;

                    // For each duplicate layer
                    for layer in 1..=count {
                        let shift = semitones * layer as i32;
                        let shift_ratio = 2.0_f32.powf(shift as f32 / 12.0);

                        // Duplicate each original frequency
                        for i in 0..original_count {
                            if note.num_freqs < 8 {
                                note.frequencies[note.num_freqs] = note.frequencies[i] * shift_ratio;
                                note.num_freqs += 1;
                            } else {
                                // Max 8 frequencies - silently stop if we hit the limit
                                break;
                            }
                        }
                    }
                }
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Stretch pattern timing by a factor
    ///
    /// Multiplies all note start times and durations within the pattern by the given factor.
    /// Values > 1.0 slow down the pattern, values < 1.0 speed it up.
    ///
    /// # Arguments
    /// * `factor` - Time multiplication factor (e.g., 2.0 = half speed, 0.5 = double speed)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Original pattern at normal speed
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.25);
    ///
    /// // Same pattern at half speed (twice as long)
    /// comp.track("slow")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.25)
    ///     .stretch(2.0);
    ///
    /// // Same pattern at double speed (half duration)
    /// comp.track("fast")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.25)
    ///     .stretch(0.5);
    /// ```
    pub fn stretch(mut self, factor: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || factor <= 0.0 || (factor - 1.0).abs() < 0.001 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Stretch all events in the pattern
        for event in &mut self.get_track_mut().events {
            match event {
                AudioEvent::Note(note) => {
                    if note.start_time >= pattern_start && note.start_time < cursor {
                        // Stretch timing relative to pattern start
                        let offset = note.start_time - pattern_start;
                        note.start_time = pattern_start + (offset * factor);
                        note.duration *= factor;
                    }
                }
                AudioEvent::Drum(drum) => {
                    if drum.start_time >= pattern_start && drum.start_time < cursor {
                        // Stretch timing relative to pattern start
                        let offset = drum.start_time - pattern_start;
                        drum.start_time = pattern_start + (offset * factor);
                    }
                }
                _ => {} // Ignore other event types
            }
        }

        // Update cursor to reflect stretched duration
        self.cursor = pattern_start + (pattern_duration * factor);

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Compress pattern to fit within a specific duration
    ///
    /// Ergonomic wrapper around `.stretch()` - instead of calculating ratios manually,
    /// simply specify the target duration and the pattern will be stretched to fit.
    ///
    /// # Arguments
    /// * `target_duration` - Desired duration in beats (e.g., 1.0 = one beat, 2.5 = two and a half beats)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Pattern naturally takes 0.75 beats
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.25)
    ///     .compress(0.5);  // Now fits in exactly 0.5 beats
    ///
    /// // Compress multiple notes into 1 beat
    /// comp.track("fast")
    ///     .pattern_start()
    ///     .notes(&[C4, D4, E4, F4, G4], 0.5)  // Naturally 2.5 beats
    ///     .compress(1.0);  // Now exactly 1 beat
    /// ```
    pub fn compress(self, target_duration: f32) -> Self {
        let current_duration = self.cursor - self.pattern_start;

        if current_duration <= 0.0 || target_duration <= 0.0 {
            return self;
        }

        // Calculate stretch factor to reach target duration
        let factor = target_duration / current_duration;

        // Reuse stretch implementation
        self.stretch(factor)
    }

    /// Quantize note timings to a rhythmic grid
    ///
    /// Snaps all note start times to the nearest grid position, useful for cleaning up
    /// timing after humanization or ensuring tight rhythmic accuracy.
    ///
    /// # Arguments
    /// * `grid` - Grid size in beats (e.g., 0.25 = 16th notes, 0.5 = 8th notes, 1.0 = quarter notes)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Humanized pattern with timing variations
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .humanize(0.05, 0.1)  // Add timing jitter
    ///     .quantize(0.25);       // Snap back to 16th note grid
    ///
    /// // Snap to 8th note grid (less strict)
    /// comp.track("loose")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .quantize(0.5);  // 8th note grid
    /// ```
    pub fn quantize(mut self, grid: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || grid <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Quantize all events in the pattern
        for event in &mut self.get_track_mut().events {
            match event {
                AudioEvent::Note(note) => {
                    if note.start_time >= pattern_start && note.start_time < cursor {
                        // Quantize to nearest grid position
                        let offset = note.start_time - pattern_start;
                        let quantized_offset = (offset / grid).round() * grid;
                        note.start_time = pattern_start + quantized_offset;
                    }
                }
                AudioEvent::Drum(drum) => {
                    if drum.start_time >= pattern_start && drum.start_time < cursor {
                        // Quantize to nearest grid position
                        let offset = drum.start_time - pattern_start;
                        let quantized_offset = (offset / grid).round() * grid;
                        drum.start_time = pattern_start + quantized_offset;
                    }
                }
                _ => {} // Ignore other event types
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Create a palindrome - pattern plays forward then backward
    ///
    /// Mirrors the pattern by appending a reversed copy. Creates symmetrical musical phrases
    /// that return to the starting point. Timing is reversed but pitches play in reverse order.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Original: C4, E4, G4
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.25)
    ///     .palindrome();  // Becomes: C4, E4, G4, G4, E4, C4
    ///
    /// // Great for creating symmetrical phrases
    /// comp.track("symmetrical")
    ///     .pattern_start()
    ///     .notes(&[C4, D4, E4, F4], 0.25)
    ///     .palindrome();  // → C4, D4, E4, F4, F4, E4, D4, C4
    /// ```
    pub fn palindrome(mut self) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect all events in the pattern
        let mut events_to_mirror: Vec<AudioEvent> = Vec::new();
        for event in &self.get_track_mut().events {
            match event {
                AudioEvent::Note(note) => {
                    if note.start_time >= pattern_start && note.start_time < cursor {
                        events_to_mirror.push(event.clone());
                    }
                }
                AudioEvent::Drum(drum) => {
                    if drum.start_time >= pattern_start && drum.start_time < cursor {
                        events_to_mirror.push(event.clone());
                    }
                }
                _ => {}
            }
        }

        if events_to_mirror.is_empty() {
            return self;
        }

        // Reverse and append the mirrored events
        for event in events_to_mirror.iter().rev() {
            let mut mirrored_event = event.clone();

            // Calculate mirrored timing (relative to end of original pattern)
            let original_offset = match event {
                AudioEvent::Note(note) => note.start_time - pattern_start,
                AudioEvent::Drum(drum) => drum.start_time - pattern_start,
                _ => 0.0,
            };

            let mirrored_offset = pattern_duration - original_offset;
            let new_start_time = cursor + mirrored_offset;

            match &mut mirrored_event {
                AudioEvent::Note(note) => {
                    // Position reversed notes after the original pattern
                    note.start_time = new_start_time - note.duration;
                }
                AudioEvent::Drum(drum) => {
                    drum.start_time = new_start_time;
                }
                _ => {}
            }

            self.get_track_mut().events.push(mirrored_event);
        }

        // Update cursor to reflect doubled length
        self.cursor = cursor + pattern_duration;

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Add random stuttering (glitch effect) - rapidly repeat notes
    ///
    /// Randomly triggers rapid repetitions of notes, creating glitchy stuttering effects
    /// popular in electronic music and trap production.
    ///
    /// # Arguments
    /// * `probability` - Chance (0.0-1.0) that each note will stutter
    /// * `repeats` - Number of rapid repeats to create (typically 2-8)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // 50% chance each note stutters 4 times
    /// comp.track("glitch")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.5)
    ///     .stutter(0.5, 4);  // Random notes become: C-C-C-C or E-E-E-E (fast)
    ///
    /// // Trap hi-hat rolls
    /// comp.track("hats")
    ///     .pattern_start()
    ///     .notes(&[C4, C4, C4, C4], 0.25)
    ///     .stutter(0.25, 8);  // Occasional 8x rolls
    /// ```
    pub fn stutter(mut self, probability: f32, repeats: usize) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || probability <= 0.0 || repeats == 0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        use rand::Rng;
        let mut rng = rand::rng();

        // Collect events that will stutter
        let mut stutter_events: Vec<(usize, AudioEvent, f32)> = Vec::new();

        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            let should_stutter = rng.random_range(0.0..1.0) < probability;

            if !should_stutter {
                continue;
            }

            match event {
                AudioEvent::Note(note) => {
                    if note.start_time >= pattern_start && note.start_time < cursor {
                        stutter_events.push((idx, event.clone(), note.duration));
                    }
                }
                AudioEvent::Drum(drum) => {
                    if drum.start_time >= pattern_start && drum.start_time < cursor {
                        // Drums don't have duration, use small interval
                        stutter_events.push((idx, event.clone(), 0.05));
                    }
                }
                _ => {}
            }
        }

        // Add stutter repeats
        for (_idx, event, base_duration) in stutter_events {
            // Calculate rapid repeat interval (divide duration by number of repeats)
            let stutter_interval = base_duration / repeats as f32;

            for i in 1..=repeats {
                let mut stutter_copy = event.clone();
                let offset = stutter_interval * i as f32;

                match &mut stutter_copy {
                    AudioEvent::Note(note) => {
                        note.start_time += offset;
                        note.duration = stutter_interval * 0.8; // Slightly shorter for separation
                    }
                    AudioEvent::Drum(drum) => {
                        drum.start_time += offset;
                    }
                    _ => {}
                }

                self.get_track_mut().events.push(stutter_copy);
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Stutter every Nth note (deterministic glitch effect)
    ///
    /// Applies stuttering to every Nth note in the pattern, creating rhythmic glitch effects.
    /// Unlike `.stutter()` which is random, this version is predictable and great for
    /// creating consistent rhythmic patterns like trap hi-hat rolls.
    ///
    /// # Arguments
    /// * `nth` - Which note to stutter (e.g., 4 = every 4th note)
    /// * `repeats` - Number of rapid repeats to create
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Every 4th note stutters 8 times (trap hi-hat roll)
    /// comp.track("hats")
    ///     .pattern_start()
    ///     .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.25)
    ///     .stutter_every(4, 8);  // 4th and 8th notes roll
    ///
    /// // Kick drum pattern with stutter on 2 and 4
    /// comp.track("kicks")
    ///     .pattern_start()
    ///     .notes(&[C2, C2, C2, C2], 0.5)
    ///     .stutter_every(2, 4);  // 2nd and 4th kicks stutter
    /// ```
    pub fn stutter_every(mut self, nth: usize, repeats: usize) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || nth == 0 || repeats == 0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect events to stutter (every nth event)
        let mut stutter_events: Vec<(usize, AudioEvent, f32)> = Vec::new();
        let mut note_count = 0;

        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            match event {
                AudioEvent::Note(note) => {
                    if note.start_time >= pattern_start && note.start_time < cursor {
                        note_count += 1;
                        if note_count % nth == 0 {
                            stutter_events.push((idx, event.clone(), note.duration));
                        }
                    }
                }
                AudioEvent::Drum(drum) => {
                    if drum.start_time >= pattern_start && drum.start_time < cursor {
                        note_count += 1;
                        if note_count % nth == 0 {
                            stutter_events.push((idx, event.clone(), 0.05));
                        }
                    }
                }
                _ => {}
            }
        }

        // Add stutter repeats
        for (_idx, event, base_duration) in stutter_events {
            let stutter_interval = base_duration / repeats as f32;

            for i in 1..=repeats {
                let mut stutter_copy = event.clone();
                let offset = stutter_interval * i as f32;

                match &mut stutter_copy {
                    AudioEvent::Note(note) => {
                        note.start_time += offset;
                        note.duration = stutter_interval * 0.8;
                    }
                    AudioEvent::Drum(drum) => {
                        drum.start_time += offset;
                    }
                    _ => {}
                }

                self.get_track_mut().events.push(stutter_copy);
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Break each note into micro-fragments (granularize)
    ///
    /// Splits each note into multiple tiny notes across its duration, creating granular textures.
    /// Great for creating shimmering effects, especially when combined with other transformations
    /// like `.mutate()` or `.shuffle()`.
    ///
    /// # Arguments
    /// * `divisions` - Number of fragments to create per note (typically 4-50)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Break each note into 10 grains
    /// comp.track("texture")
    ///     .pattern_start()
    ///     .note(&[C4], 1.0)
    ///     .granularize(10);  // → 10 tiny 0.1s notes
    ///
    /// // Granular with pitch variation
    /// comp.track("shimmer")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.5)
    ///     .granularize(20)   // Break into 20 grains each
    ///     .mutate(3);        // Add pitch variation to grains
    /// ```
    pub fn granularize(mut self, divisions: usize) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || divisions == 0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect notes to granularize and remove originals
        let mut notes_to_granularize: Vec<AudioEvent> = Vec::new();
        let mut indices_to_remove: Vec<usize> = Vec::new();

        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            // Only granularize notes, not drums
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    notes_to_granularize.push(event.clone());
                    indices_to_remove.push(idx);
                }
            }
        }

        // Remove original notes in reverse order to maintain indices
        for &idx in indices_to_remove.iter().rev() {
            self.get_track_mut().events.remove(idx);
        }

        // Create granularized versions
        for event in notes_to_granularize {
            if let AudioEvent::Note(note) = event {
                let grain_duration = note.duration / divisions as f32;

                for i in 0..divisions {
                    let mut grain = note.clone();
                    grain.start_time = note.start_time + (grain_duration * i as f32);
                    grain.duration = grain_duration * 0.9; // Slight gap between grains

                    self.get_track_mut()
                        .events
                        .push(AudioEvent::Note(grain));
                }
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Snap all note pitches to the nearest note in a given scale
    ///
    /// Quantizes pitch (not time) by snapping each note frequency to the closest
    /// frequency in the provided scale. Great for forcing melodies into a specific
    /// tonality or correcting out-of-scale notes.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Chromatic melody snapped to C major pentatonic (C, D, E, G, A)
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, CS4, D4, DS4, E4, F4, FS4, G4, GS4, A4], 0.25)
    ///     .magnetize(&[C4, D4, E4, G4, A4]);  // Snap to pentatonic
    /// ```
    pub fn magnetize(mut self, scale_notes: &[f32]) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || scale_notes.is_empty() {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Snap each note frequency to nearest scale note
        for event in &mut self.get_track_mut().events {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    for i in 0..note.num_freqs {
                        let original_freq = note.frequencies[i];

                        // Find nearest frequency in scale
                        let mut closest_freq = scale_notes[0];
                        let mut min_distance = (original_freq / closest_freq).log2().abs();

                        for &scale_freq in scale_notes.iter().skip(1) {
                            let distance = (original_freq / scale_freq).log2().abs();
                            if distance < min_distance {
                                min_distance = distance;
                                closest_freq = scale_freq;
                            }
                        }

                        note.frequencies[i] = closest_freq;
                    }
                }
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Apply gravitational pull toward or away from a center pitch
    ///
    /// Notes are attracted (positive strength) or repelled (negative strength)
    /// from a center frequency. The effect is proportional to distance - notes
    /// closer to the center are affected more strongly.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C3, E4, G5, C6], 0.5)
    ///     .gravity(C4, 0.3);  // Pull toward middle C (30% of distance)
    /// ```
    pub fn gravity(mut self, center_pitch: f32, strength: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || strength == 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Apply gravitational force to each note
        for event in &mut self.get_track_mut().events {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    for i in 0..note.num_freqs {
                        let original_freq = note.frequencies[i];

                        // Calculate distance in semitones
                        let semitone_distance = 12.0 * (original_freq / center_pitch).log2();

                        // Apply gravity - move by (strength * distance) toward center
                        let pull_semitones = -semitone_distance * strength;
                        let shift_ratio = 2.0_f32.powf(pull_semitones / 12.0);

                        note.frequencies[i] = original_freq * shift_ratio;
                    }
                }
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Create cascading effects where each note influences subsequent notes
    ///
    /// Each note creates a "ripple" that affects the timing and pitch of following
    /// notes. The effect decays over time. Positive intensity pushes notes forward
    /// in time and up in pitch, negative pulls them back and down.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, C4, C4, C4, C4], 0.25)
    ///     .ripple(0.02);  // Each note pushes the next one slightly
    /// ```
    pub fn ripple(mut self, intensity: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || intensity == 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect notes in time order
        let mut note_data: Vec<(usize, f32, [f32; 8], usize)> = Vec::new();
        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    note_data.push((idx, note.start_time, note.frequencies, note.num_freqs));
                }
            }
        }

        // Sort by time
        note_data.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        // Apply cascading ripple effects
        let mut accumulated_time_shift = 0.0;
        let mut accumulated_pitch_shift = 0.0;
        let decay = 0.7; // Each ripple decays to 70% of previous

        for (i, (idx, _original_time, _original_freqs, num_freqs)) in note_data.iter().enumerate() {
            if i > 0 {
                // Apply accumulated effects from previous notes
                if let AudioEvent::Note(note) = &mut self.get_track_mut().events[*idx] {
                    // Apply timing shift
                    note.start_time += accumulated_time_shift;

                    // Apply pitch shift
                    let pitch_shift_ratio = 2.0_f32.powf(accumulated_pitch_shift / 12.0);
                    for j in 0..*num_freqs {
                        note.frequencies[j] *= pitch_shift_ratio;
                    }
                }
            }

            // Add this note's contribution to the ripple (decayed)
            accumulated_time_shift = (accumulated_time_shift + intensity) * decay;
            accumulated_pitch_shift = (accumulated_pitch_shift + intensity * 2.0) * decay;
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Shuffle pitches in random order (keeps timing)
    ///
    /// Randomly reorders the pitch sequence while maintaining the original timing.
    /// Each call produces a different random ordering.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Original: C4, E4, G4, C5
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .shuffle();  // Result: random order like G4, C4, C5, E4
    /// ```
    pub fn shuffle(mut self) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect note events in the pattern
        let mut note_events: Vec<(usize, [f32; 8], usize)> = Vec::new();
        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    note_events.push((idx, note.frequencies, note.num_freqs));
                }
            }
        }

        if note_events.is_empty() {
            return self;
        }

        // Extract and shuffle frequencies
        let mut freqs: Vec<([f32; 8], usize)> = note_events
            .iter()
            .map(|(_, f, n)| (*f, *n))
            .collect();

        // Shuffle using Fisher-Yates
        use rand::Rng;
        let mut rng = rand::rng();
        for i in (1..freqs.len()).rev() {
            let j = rng.random_range(0..=i);
            freqs.swap(i, j);
        }

        // Apply shuffled frequencies back to the notes
        for (i, (event_idx, _, _)) in note_events.iter().enumerate() {
            if let AudioEvent::Note(note) = &mut self.get_track_mut().events[*event_idx] {
                note.frequencies = freqs[i].0;
                note.num_freqs = freqs[i].1;
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Randomly remove notes from the pattern
    ///
    /// Reduces note density by probabilistically keeping or removing each note.
    /// Great for creating space or variations with less density.
    ///
    /// # Arguments
    /// * `keep_probability` - Chance to keep each note (0.0 = remove all, 1.0 = keep all)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("hihat")
    ///     .pattern_start()
    ///     .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.125)
    ///     .thin(0.5);  // Keep ~50% of notes
    ///
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5, E4, G4], 0.25)
    ///     .thin(0.7);  // Keep ~70% of notes
    /// ```
    pub fn thin(mut self, keep_probability: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let keep_probability = keep_probability.clamp(0.0, 1.0);

        // If probability is 1.0, keep everything
        if keep_probability >= 1.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Remove notes based on probability
        use rand::Rng;
        let mut rng = rand::rng();

        self.get_track_mut().events.retain(|event| {
            match event {
                AudioEvent::Note(note) => {
                    if note.start_time >= pattern_start && note.start_time < cursor {
                        // Randomly decide to keep or remove
                        rng.random_range(0.0..1.0) < keep_probability
                    } else {
                        true // Keep notes outside pattern
                    }
                }
                _ => true // Keep non-note events
            }
        });

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Retrograde: reverse the melodic contour (pitches backwards, timing forward)
    ///
    /// Classic compositional technique - plays the pitch sequence in reverse order
    /// while keeping the original timing. Different from `.reverse()` which reverses time.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Original: C4 at t=0, E4 at t=0.25, G4 at t=0.5
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.25)
    ///     .retrograde();  // Result: G4 at t=0, E4 at t=0.25, C4 at t=0.5
    /// ```
    pub fn retrograde(mut self) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect note events in the pattern
        let mut note_events: Vec<(usize, [f32; 8], usize)> = Vec::new();
        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    note_events.push((idx, note.frequencies, note.num_freqs));
                }
            }
        }

        if note_events.is_empty() {
            return self;
        }

        // Extract frequencies in order
        let freqs: Vec<([f32; 8], usize)> = note_events
            .iter()
            .map(|(_, f, n)| (*f, *n))
            .collect();

        // Reverse the frequencies
        let reversed_freqs: Vec<([f32; 8], usize)> = freqs.into_iter().rev().collect();

        // Apply reversed frequencies back to the notes (keeping original timing)
        for (i, (event_idx, _, _)) in note_events.iter().enumerate() {
            if let AudioEvent::Note(note) = &mut self.get_track_mut().events[*event_idx] {
                note.frequencies = reversed_freqs[i].0;
                note.num_freqs = reversed_freqs[i].1;
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Shift all notes in the pattern by semitones
    ///
    /// Transposes all notes between pattern_start and current cursor.
    /// Positive values shift up, negative values shift down.
    ///
    /// # Arguments
    /// * `semitones` - Number of semitones to shift (positive = up, negative = down)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.5)
    ///     .shift(5);  // Transpose up a perfect fourth
    ///
    /// comp.track("bass")
    ///     .pattern_start()
    ///     .notes(&[C3, G3, C4], 0.5)
    ///     .shift(-12);  // Transpose down an octave
    /// ```
    pub fn shift(mut self, semitones: i32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || semitones == 0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;
        let shift_ratio = 2.0_f32.powf(semitones as f32 / 12.0);

        // Collect events in the pattern range - shift notes, pass through drums and samples
        let shifted_events: Vec<_> = self
            .get_track_mut()
            .events
            .iter()
            .filter_map(|event| {
                let event_time = event.start_time();
                if event_time >= pattern_start && event_time < cursor {
                    match event {
                        AudioEvent::Note(note) => {
                            // Shift each frequency in the note/chord
                            let mut shifted_freqs = [0.0f32; 8];
                            for (i, freq) in shifted_freqs.iter_mut().enumerate().take(note.num_freqs) {
                                *freq = note.frequencies[i] * shift_ratio;
                            }

                            Some(AudioEvent::Note(crate::track::NoteEvent {
                                frequencies: shifted_freqs,
                                num_freqs: note.num_freqs,
                                start_time: note.start_time,
                                duration: note.duration,
                                waveform: note.waveform,
                                envelope: note.envelope,
                                filter_envelope: note.filter_envelope,
                                fm_params: note.fm_params,
                                pitch_bend_semitones: note.pitch_bend_semitones,
                                custom_wavetable: note.custom_wavetable.clone(),
                                velocity: note.velocity,
                                spatial_position: note.spatial_position,
                            }))
                        }
                        AudioEvent::Drum(_)
                        | AudioEvent::Sample(_)
                        | AudioEvent::TempoChange(_)
                        | AudioEvent::TimeSignature(_)
                        | AudioEvent::KeySignature(_) => {
                            // Pass through drums, samples, tempo changes, and time signatures unchanged
                            Some(event.clone())
                        }
                    }
                } else {
                    None
                }
            })
            .collect();

        // Remove original pattern events
        let pattern_start = self.pattern_start;
        let cursor = self.cursor;
        self.get_track_mut().events.retain(|event| {
            let event_time = match event {
                AudioEvent::Note(note) => note.start_time,
                AudioEvent::Drum(drum) => drum.start_time,
                AudioEvent::Sample(sample) => sample.start_time,
                AudioEvent::TempoChange(tempo) => tempo.start_time,
                AudioEvent::TimeSignature(time_sig) => time_sig.start_time,
                AudioEvent::KeySignature(key_sig) => key_sig.start_time,
            };
            event_time < pattern_start || event_time >= cursor
        });

        // Add shifted events
        self.get_track_mut().events.extend(shifted_events);
        self.get_track_mut().invalidate_time_cache();

        self.update_section_duration();
        self
    }

    /// Invert the pattern between pattern_start and current cursor
    ///
    /// Musical inversion mirrors pitches around a center point (axis).
    /// If a note was 2 semitones above the axis, it becomes 2 semitones below.
    ///
    /// # Arguments
    /// * `axis_freq` - The frequency to mirror around (typically the tonic)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, F4], 0.5)
    ///     .invert(C4);  // Mirror around C4
    /// ```
    pub fn invert(mut self, axis_freq: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect events in the pattern range - invert notes, pass through drums and samples
        let inverted_events: Vec<_> = self
            .get_track_mut()
            .events
            .iter()
            .filter_map(|event| {
                let event_time = event.start_time();
                if event_time >= pattern_start && event_time < cursor {
                    match event {
                        AudioEvent::Note(note) => {
                            // Invert each frequency in the note/chord
                            let mut inverted_freqs = [0.0f32; 8];
                            for (i, inv_freq) in inverted_freqs.iter_mut().enumerate().take(note.num_freqs) {
                                let freq = note.frequencies[i];
                                // Calculate distance from axis in semitones
                                let semitones_from_axis = 12.0 * (freq / axis_freq).log2();
                                // Mirror it
                                let inverted_semitones = -semitones_from_axis;
                                // Convert back to frequency
                                *inv_freq = axis_freq * 2.0_f32.powf(inverted_semitones / 12.0);
                            }

                            Some(AudioEvent::Note(crate::track::NoteEvent {
                                frequencies: inverted_freqs,
                                num_freqs: note.num_freqs,
                                start_time: note.start_time,
                                duration: note.duration,
                                waveform: note.waveform,
                                envelope: note.envelope,
                                filter_envelope: note.filter_envelope,
                                fm_params: note.fm_params,
                                pitch_bend_semitones: note.pitch_bend_semitones,
                                custom_wavetable: note.custom_wavetable.clone(),
                                velocity: note.velocity,
                                spatial_position: note.spatial_position,
                            }))
                        }
                        AudioEvent::Drum(_)
                        | AudioEvent::Sample(_)
                        | AudioEvent::TempoChange(_)
                        | AudioEvent::TimeSignature(_)
                        | AudioEvent::KeySignature(_) => {
                            // Pass through drums, samples, tempo changes, and time signatures unchanged
                            Some(event.clone())
                        }
                    }
                } else {
                    None
                }
            })
            .collect();

        // Remove original pattern events
        let pattern_start = self.pattern_start;
        let cursor = self.cursor;
        self.get_track_mut().events.retain(|event| {
            let event_time = match event {
                AudioEvent::Note(note) => note.start_time,
                AudioEvent::Drum(drum) => drum.start_time,
                AudioEvent::Sample(sample) => sample.start_time,
                AudioEvent::TempoChange(tempo) => tempo.start_time,
                AudioEvent::TimeSignature(time_sig) => time_sig.start_time,
                AudioEvent::KeySignature(key_sig) => key_sig.start_time,
            };
            event_time < pattern_start || event_time >= cursor
        });

        // Add inverted events
        self.get_track_mut().events.extend(inverted_events);

        self
    }

    /// Invert and transpose to keep the result in a reasonable range
    ///
    /// This is a more musical version of invert that ensures inverted notes
    /// stay near the original range by octave-shifting as needed.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, F4], 0.5)
    ///     .invert_constrained(C4, C3, C5);  // Keep between C3 and C5
    /// ```
    pub fn invert_constrained(mut self, axis_freq: f32, min_freq: f32, max_freq: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect and invert events, constraining to range - pass through drums and samples
        let inverted_events: Vec<_> = self
            .get_track_mut()
            .events
            .iter()
            .filter_map(|event| {
                let event_time = event.start_time();
                if event_time >= pattern_start && event_time < cursor {
                    match event {
                        AudioEvent::Note(note) => {
                            let mut inverted_freqs = [0.0f32; 8];
                            for (i, inv_freq_slot) in inverted_freqs.iter_mut().enumerate().take(note.num_freqs) {
                                let freq = note.frequencies[i];
                                let semitones_from_axis = 12.0 * (freq / axis_freq).log2();
                                let inverted_semitones = -semitones_from_axis;
                                let mut inverted_freq =
                                    axis_freq * 2.0_f32.powf(inverted_semitones / 12.0);

                                // Octave-shift to keep in range
                                while inverted_freq < min_freq {
                                    inverted_freq *= 2.0;
                                }
                                while inverted_freq > max_freq {
                                    inverted_freq /= 2.0;
                                }

                                *inv_freq_slot = inverted_freq;
                            }

                            Some(AudioEvent::Note(crate::track::NoteEvent {
                                frequencies: inverted_freqs,
                                num_freqs: note.num_freqs,
                                start_time: note.start_time,
                                duration: note.duration,
                                waveform: note.waveform,
                                envelope: note.envelope,
                                filter_envelope: note.filter_envelope,
                                fm_params: note.fm_params,
                                pitch_bend_semitones: note.pitch_bend_semitones,
                                custom_wavetable: note.custom_wavetable.clone(),
                                velocity: note.velocity,
                                spatial_position: note.spatial_position,
                            }))
                        }
                        AudioEvent::Drum(_)
                        | AudioEvent::Sample(_)
                        | AudioEvent::TempoChange(_)
                        | AudioEvent::TimeSignature(_)
                        | AudioEvent::KeySignature(_) => {
                            // Pass through drums, samples, tempo changes, and time signatures unchanged
                            Some(event.clone())
                        }
                    }
                } else {
                    None
                }
            })
            .collect();

        // Remove original and add inverted
        let pattern_start = self.pattern_start;
        let cursor = self.cursor;
        self.get_track_mut().events.retain(|event| {
            let event_time = match event {
                AudioEvent::Note(note) => note.start_time,
                AudioEvent::Drum(drum) => drum.start_time,
                AudioEvent::Sample(sample) => sample.start_time,
                AudioEvent::TempoChange(tempo) => tempo.start_time,
                AudioEvent::TimeSignature(time_sig) => time_sig.start_time,
                AudioEvent::KeySignature(key_sig) => key_sig.start_time,
            };
            event_time < pattern_start || event_time >= cursor
        });

        self.get_track_mut().events.extend(inverted_events);

        self
    }

    /// Filter to keep only notes within frequency range
    ///
    /// Removes all notes whose frequencies fall outside [min_freq, max_freq].
    /// Useful for isolating specific frequency bands or removing unwanted ranges.
    ///
    /// # Examples
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Keep only bass frequencies (20-200 Hz)
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C3, E3, G3, C4, E4, G4], 0.25)
    ///     .sieve_inclusive(20.0, 200.0);  // Only bass notes remain
    /// ```
    pub fn sieve_inclusive(mut self, min_freq: f32, max_freq: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Filter notes to keep only those within frequency range
        self.get_track_mut().events.retain(|event| {
            match event {
                AudioEvent::Note(note) => {
                    if note.start_time >= pattern_start && note.start_time < cursor {
                        // Check if any frequency in the note is within range
                        (0..note.num_freqs).any(|i| {
                            let freq = note.frequencies[i];
                            freq >= min_freq && freq <= max_freq
                        })
                    } else {
                        true // Keep notes outside pattern
                    }
                }
                _ => true, // Keep non-note events
            }
        });

        self
    }

    /// Filter to remove notes within frequency range
    ///
    /// Removes all notes whose frequencies fall within [min_freq, max_freq].
    /// Useful for removing specific frequency bands (e.g., muddy midrange).
    ///
    /// # Examples
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Remove midrange frequencies (200-800 Hz)
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C3, E3, G3, C4, E4, G4], 0.25)
    ///     .sieve_exclusive(200.0, 800.0);  // Low and high notes remain
    /// ```
    pub fn sieve_exclusive(mut self, min_freq: f32, max_freq: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Filter notes to remove those within frequency range
        self.get_track_mut().events.retain(|event| {
            match event {
                AudioEvent::Note(note) => {
                    if note.start_time >= pattern_start && note.start_time < cursor {
                        // Keep note only if ALL frequencies are outside range
                        (0..note.num_freqs).all(|i| {
                            let freq = note.frequencies[i];
                            freq < min_freq || freq > max_freq
                        })
                    } else {
                        true // Keep notes outside pattern
                    }
                }
                _ => true, // Keep non-note events
            }
        });

        self
    }

    /// Collapse all notes in the pattern into a single chord
    ///
    /// Takes all notes from the pattern and plays them simultaneously as a chord.
    /// Useful for converting melodies/arpeggios into harmonic blocks.
    ///
    /// # Examples
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Turn arpeggio into chord
    /// comp.track("arp_to_chord")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .group(2.0);  // All notes play together for 2 seconds
    /// ```
    pub fn group(mut self, duration: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect all note frequencies from the pattern
        let mut all_freqs = Vec::new();
        let mut waveform = Waveform::Sine;
        let mut envelope = Envelope::default();
        let mut pitch_bend = 0.0;
        let mut velocity = 1.0;

        for event in &self.get_track_mut().events {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    // Collect all frequencies from this note
                    for i in 0..note.num_freqs {
                        all_freqs.push(note.frequencies[i]);
                    }
                    // Use properties from first note
                    if all_freqs.len() <= note.num_freqs {
                        waveform = note.waveform;
                        envelope = note.envelope;
                        pitch_bend = note.pitch_bend_semitones;
                        velocity = note.velocity;
                    }
                }
            }
        }

        if all_freqs.is_empty() {
            return self;
        }

        // Remove all notes from the pattern
        self.get_track_mut().events.retain(|event| {
            match event {
                AudioEvent::Note(note) => {
                    note.start_time < pattern_start || note.start_time >= cursor
                }
                _ => true, // Keep non-note events
            }
        });

        // Add a single chord with all frequencies
        let mut freq_array = [0.0f32; 8];
        let num_freqs = all_freqs.len().min(8);
        for (i, &freq) in all_freqs.iter().take(8).enumerate() {
            freq_array[i] = freq;
        }

        let chord_event = AudioEvent::Note(crate::track::NoteEvent {
            frequencies: freq_array,
            num_freqs,
            start_time: pattern_start,
            duration,
            waveform,
            envelope,
            filter_envelope: crate::synthesis::filter_envelope::FilterEnvelope::default(),
            fm_params: crate::synthesis::fm_synthesis::FMParams::default(),
            pitch_bend_semitones: pitch_bend,
            custom_wavetable: None,
            velocity,
            spatial_position: None,
        });

        self.get_track_mut().events.push(chord_event);

        // Update cursor to after the chord
        self.cursor = pattern_start + duration;

        self
    }

    /// Duplicate all events in the pattern
    ///
    /// Creates a copy of all events and appends them after the pattern.
    /// Unlike `.repeat()`, this allows transforms to be applied to the duplicated events.
    ///
    /// # Examples
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Create melody with octave doubling
    /// comp.track("harmony")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.25)
    ///     .duplicate()
    ///     .transform(|t| t.shift(12));  // Add octave above
    /// ```
    pub fn duplicate(mut self) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect all events in the pattern
        let duplicated_events: Vec<_> = self
            .get_track_mut()
            .events
            .iter()
            .filter_map(|event| {
                let event_time = event.start_time();
                if event_time >= pattern_start && event_time < cursor {
                    // Clone and shift to end of pattern
                    let mut cloned = event.clone();
                    match &mut cloned {
                        AudioEvent::Note(note) => {
                            note.start_time = note.start_time - pattern_start + cursor;
                        }
                        AudioEvent::Drum(drum) => {
                            drum.start_time = drum.start_time - pattern_start + cursor;
                        }
                        AudioEvent::Sample(sample) => {
                            sample.start_time = sample.start_time - pattern_start + cursor;
                        }
                        AudioEvent::TempoChange(tempo) => {
                            tempo.start_time = tempo.start_time - pattern_start + cursor;
                        }
                        AudioEvent::TimeSignature(time_sig) => {
                            time_sig.start_time = time_sig.start_time - pattern_start + cursor;
                        }
                        AudioEvent::KeySignature(key_sig) => {
                            key_sig.start_time = key_sig.start_time - pattern_start + cursor;
                        }
                    }
                    Some(cloned)
                } else {
                    None
                }
            })
            .collect();

        // Add duplicated events
        self.get_track_mut().events.extend(duplicated_events);

        // Update cursor and pattern_start
        // Set pattern_start to beginning of duplicated section so transforms only affect duplicated notes
        self.pattern_start = cursor;
        self.cursor = cursor + pattern_duration;

        self
    }

    /// Dilate (expand/compress) the pitch range around its center
    ///
    /// Adjusts the distance of all pitches from the pattern's center pitch.
    /// - `factor < 1.0` compresses toward center (0.5 = half range)
    /// - `factor = 1.0` no change
    /// - `factor > 1.0` expands from center (2.0 = double range)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Compress range to 70% of original
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C3, C5, C2, C6], 0.25)
    ///     .range_dilation(0.7);
    ///
    /// // Expand range to 150% of original
    /// comp.track("wide")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.25)
    ///     .range_dilation(1.5);
    /// ```
    pub fn range_dilation(mut self, factor: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || factor == 1.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Find center pitch (geometric mean)
        let mut sum_log_freq = 0.0;
        let mut count = 0;

        for event in &self.get_track_mut().events {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    for i in 0..note.num_freqs {
                        sum_log_freq += note.frequencies[i].ln();
                        count += 1;
                    }
                }
            }
        }

        if count == 0 {
            return self;
        }

        let center_pitch = (sum_log_freq / count as f32).exp();

        // Apply dilation
        for event in &mut self.get_track_mut().events {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    for i in 0..note.num_freqs {
                        let original_freq = note.frequencies[i];

                        // Calculate distance from center in semitones
                        let semitone_distance = 12.0 * (original_freq / center_pitch).log2();

                        // Scale distance by factor
                        let new_distance = semitone_distance * factor;
                        let shift_ratio = 2.0_f32.powf(new_distance / 12.0);

                        note.frequencies[i] = center_pitch * shift_ratio;
                    }
                }
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Shape melodic contour by scaling interval sizes
    ///
    /// Modifies the size of melodic intervals (jumps between consecutive notes).
    /// - `factor < 1.0` smooths (reduces interval jumps)
    /// - `factor = 1.0` no change
    /// - `factor > 1.0` exaggerates (increases interval jumps)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Smooth out large interval jumps
    /// comp.track("smooth")
    ///     .pattern_start()
    ///     .notes(&[C4, C6, C3, C5], 0.25)
    ///     .shape_contour(0.5);  // 50% of original intervals
    ///
    /// // Exaggerate melodic motion
    /// comp.track("dramatic")
    ///     .pattern_start()
    ///     .notes(&[C4, D4, E4, F4], 0.25)
    ///     .shape_contour(2.0);  // Double the intervals
    /// ```
    pub fn shape_contour(mut self, factor: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || factor == 1.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect note events in time order
        let mut note_refs: Vec<(f32, usize)> = Vec::new();
        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                if note.start_time >= pattern_start && note.start_time < cursor {
                    note_refs.push((note.start_time, idx));
                }
            }
        }

        if note_refs.len() < 2 {
            return self;
        }

        // Sort by time
        note_refs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Get first note's primary frequency as anchor
        let first_idx = note_refs[0].1;
        let anchor_freq = if let AudioEvent::Note(note) = &self.get_track_mut().events[first_idx] {
            note.frequencies[0]
        } else {
            return self;
        };

        // Shape intervals relative to first note
        for i in 1..note_refs.len() {
            let note_idx = note_refs[i].1;

            if let AudioEvent::Note(note) = &mut self.get_track_mut().events[note_idx] {
                for j in 0..note.num_freqs {
                    let original_freq = note.frequencies[j];

                    // Calculate interval from anchor in semitones
                    let semitone_interval = 12.0 * (original_freq / anchor_freq).log2();

                    // Scale interval by factor
                    let new_interval = semitone_interval * factor;
                    let shift_ratio = 2.0_f32.powf(new_interval / 12.0);

                    note.frequencies[j] = anchor_freq * shift_ratio;
                }
            }
        }

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Create echo/delay trail of the pattern
    ///
    /// Duplicates the pattern multiple times with time delay and volume decay.
    ///
    /// # Arguments
    /// * `delay` - Time between echoes in seconds
    /// * `repeats` - Number of echo repetitions
    /// * `decay` - Volume reduction per echo (0.0-1.0)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Create 3 echoes, 0.5s apart, each 60% volume of previous
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4], 0.25)
    ///     .echo(0.5, 3, 0.6);
    /// ```
    pub fn echo(mut self, delay: f32, repeats: usize, decay: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || repeats == 0 || delay <= 0.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect original events
        let original_events: Vec<AudioEvent> = self
            .get_track_mut()
            .events
            .iter()
            .filter(|event| {
                let start_time = match event {
                    AudioEvent::Note(n) => n.start_time,
                    AudioEvent::Drum(d) => d.start_time,
                    AudioEvent::Sample(s) => s.start_time,
                    _ => return false,
                };
                start_time >= pattern_start && start_time < cursor
            })
            .cloned()
            .collect();

        // Create echoes
        for repeat in 1..=repeats {
            let time_offset = delay * repeat as f32;
            let volume_scale = decay.powi(repeat as i32);

            for event in &original_events {
                let mut echoed_event = event.clone();

                match &mut echoed_event {
                    AudioEvent::Note(note) => {
                        note.start_time += time_offset;
                        note.velocity *= volume_scale;
                    }
                    AudioEvent::Drum(drum) => {
                        drum.start_time += time_offset;
                        // Drums don't have velocity, decay is implicit
                    }
                    AudioEvent::Sample(sample) => {
                        sample.start_time += time_offset;
                        sample.volume *= volume_scale;
                    }
                    _ => {}
                }

                self.get_track_mut().events.push(echoed_event);
            }
        }

        // Update cursor to end of last echo
        let total_echo_duration = delay * repeats as f32;
        self.cursor = cursor + total_echo_duration;

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }

    /// Apply gradual tempo change (accelerando/ritardando) across pattern
    ///
    /// Progressively adjusts note spacing to create tempo acceleration or deceleration.
    /// - `end_factor < 1.0` ritardando (slow down - notes spread apart)
    /// - `end_factor = 1.0` no change (constant tempo)
    /// - `end_factor > 1.0` accelerando (speed up - notes compress together)
    ///
    /// # Arguments
    /// * `end_factor` - Target tempo multiplier at end of pattern
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::timing::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Ritardando - slow down to 60% of original tempo
    /// comp.track("slow")
    ///     .pattern_start()
    ///     .notes(&[C4, D4, E4, F4, G4, A4, B4, C5], 0.25)
    ///     .tempo_curve(0.6);
    ///
    /// // Accelerando - speed up to 150% of original tempo
    /// comp.track("fast")
    ///     .pattern_start()
    ///     .notes(&[C5, B4, A4, G4, F4, E4, D4, C4], 0.25)
    ///     .tempo_curve(1.5);
    /// ```
    pub fn tempo_curve(mut self, end_factor: f32) -> Self {
        let pattern_duration = self.cursor - self.pattern_start;

        if pattern_duration <= 0.0 || end_factor == 1.0 {
            return self;
        }

        let pattern_start = self.pattern_start;
        let cursor = self.cursor;

        // Collect all events with their original relative times
        let mut events_with_times: Vec<(usize, f32)> = Vec::new();
        for (idx, event) in self.get_track_mut().events.iter().enumerate() {
            let start_time = match event {
                AudioEvent::Note(n) => n.start_time,
                AudioEvent::Drum(d) => d.start_time,
                AudioEvent::Sample(s) => s.start_time,
                _ => continue,
            };

            if start_time >= pattern_start && start_time < cursor {
                events_with_times.push((idx, start_time));
            }
        }

        if events_with_times.is_empty() {
            return self;
        }

        // Apply progressive time scaling
        // Each event's position gets scaled by lerp(1.0, end_factor, progress)
        for (idx, original_time) in events_with_times {
            let relative_time = original_time - pattern_start;
            let progress = relative_time / pattern_duration; // 0.0 to 1.0

            // Linear interpolation from 1.0 (start) to end_factor (end)
            let time_scale = 1.0 + (end_factor - 1.0) * progress;
            let new_relative_time = relative_time * time_scale;
            let new_time = pattern_start + new_relative_time;

            // Update event timing
            match &mut self.get_track_mut().events[idx] {
                AudioEvent::Note(note) => note.start_time = new_time,
                AudioEvent::Drum(drum) => drum.start_time = new_time,
                AudioEvent::Sample(sample) => sample.start_time = new_time,
                _ => {}
            }
        }

        // Recalculate pattern end time based on last event
        let new_end_time = self
            .get_track_mut()
            .events
            .iter()
            .filter_map(|event| match event {
                AudioEvent::Note(n) if n.start_time >= pattern_start => Some(n.start_time),
                AudioEvent::Drum(d) if d.start_time >= pattern_start => Some(d.start_time),
                AudioEvent::Sample(s) if s.start_time >= pattern_start => Some(s.start_time),
                _ => None,
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(cursor);

        self.cursor = new_end_time.max(pattern_start);

        self.get_track_mut().invalidate_time_cache();
        self.update_section_duration();
        self
    }
}
