//! Generative and algorithmic music composition tools
//!
//! This module provides tools for creating music algorithmically, including
//! random walks, sequence transformations, and pattern manipulations.

use super::TrackBuilder;
use super::drums::DrumType;
use crate::theory::core::ChordPattern;
use crate::track::AudioEvent;
use crate::synthesis::waveform::Waveform;
use crate::synthesis::envelope::Envelope;
use rand::Rng;

/// Generate a random walk sequence that can be used with sequence_from()
///
/// Creates a sequence of integers that randomly walk up and down, staying within bounds.
///
/// # Arguments
/// * `start` - Starting value
/// * `steps` - Number of steps to generate
/// * `min` - Minimum value (inclusive)
/// * `max` - Maximum value (exclusive)
///
/// # Example
/// ```
/// # use tunes::composition::generative::random_walk_sequence;
/// # use tunes::consts::scales::C4_MAJOR_SCALE;
/// let walk = random_walk_sequence(3, 32, 0, 7);  // Walk through scale indices
/// // Use with: comp.track("melody").sequence_from(&walk, &C4_MAJOR_SCALE, 0.25);
/// ```
pub fn random_walk_sequence(start: u32, steps: usize, min: u32, max: u32) -> Vec<u32> {
    if steps == 0 || min >= max {
        return Vec::new();
    }

    let mut sequence = Vec::with_capacity(steps);
    let mut current = start.clamp(min, max - 1);
    let mut rng = rand::rng();

    for _ in 0..steps {
        sequence.push(current);

        // Random step: -2, -1, +1, or +2
        let step = if rng.random::<bool>() {
            rng.random_range(1..=2) // Move up
        } else {
            -(rng.random_range(1..=2) as i32) // Move down
        };

        current = ((current as i32 + step).clamp(min as i32, (max - 1) as i32)) as u32;
    }

    sequence
}

/// Generate a biased random walk sequence with tendency to go up or down
///
/// # Arguments
/// * `start` - Starting value
/// * `steps` - Number of steps
/// * `min` - Minimum value
/// * `max` - Maximum value
/// * `up_bias` - Probability of moving up (0.0 = always down, 1.0 = always up, 0.5 = unbiased)
///
/// # Example
/// ```
/// # use tunes::composition::generative::biased_random_walk_sequence;
/// let ascending = biased_random_walk_sequence(0, 16, 0, 12, 0.7);  // Tends upward
/// let descending = biased_random_walk_sequence(11, 16, 0, 12, 0.3);  // Tends downward
/// ```
pub fn biased_random_walk_sequence(
    start: u32,
    steps: usize,
    min: u32,
    max: u32,
    up_bias: f32,
) -> Vec<u32> {
    if steps == 0 || min >= max {
        return Vec::new();
    }

    let mut sequence = Vec::with_capacity(steps);
    let mut current = start.clamp(min, max - 1);
    let mut rng = rand::rng();

    for _ in 0..steps {
        sequence.push(current);

        // Use bias to determine direction
        let go_up = rng.random::<f32>() < up_bias;
        let step = if go_up {
            rng.random_range(1..=2) // Move up
        } else {
            -(rng.random_range(1..=2) as i32) // Move down
        };

        current = ((current as i32 + step).clamp(min as i32, (max - 1) as i32)) as u32;
    }

    sequence
}

/// Builder for pattern transformations (accessed via `.transform()`)
///
/// Provides a scoped namespace for all pattern transformation methods.
/// Use with closure syntax for clean, organized code:
///
/// ```rust
/// # use tunes::composition::Composition;
/// # use tunes::composition::rhythm::Tempo;
/// # use tunes::consts::notes::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// comp.track("melody")
///     .pattern_start()
///     .notes(&[C4, E4, G4], 0.5)
///     .transform(|t| t
///         .shift(7)
///         .humanize(0.01, 0.05)
///         .rotate(1)
///     )
///     .wait(1.0);
/// ```
pub struct TransformBuilder<'a> {
    inner: TrackBuilder<'a>,
}

impl<'a> TransformBuilder<'a> {
    /// Unwrap back to the inner TrackBuilder
    fn into_inner(self) -> TrackBuilder<'a> {
        self.inner
    }

    /// Transpose pattern by semitones
    pub fn shift(mut self, semitones: i32) -> Self {
        self.inner = self.inner.shift(semitones);
        self
    }

    /// Add organic timing and velocity variations
    pub fn humanize(mut self, timing_variance: f32, velocity_variance: f32) -> Self {
        self.inner = self.inner.humanize(timing_variance, velocity_variance);
        self
    }

    /// Cycle pitch positions
    pub fn rotate(mut self, positions: i32) -> Self {
        self.inner = self.inner.rotate(positions);
        self
    }

    /// Reverse pitch order (classical retrograde)
    pub fn retrograde(mut self) -> Self {
        self.inner = self.inner.retrograde();
        self
    }

    /// Randomly reorder pitches
    pub fn shuffle(mut self) -> Self {
        self.inner = self.inner.shuffle();
        self
    }

    /// Probabilistically remove notes
    pub fn thin(mut self, keep_probability: f32) -> Self {
        self.inner = self.inner.thin(keep_probability);
        self
    }

    /// Layer harmonic voices (octave doubling, etc.)
    pub fn stack(mut self, semitones: i32, count: usize) -> Self {
        self.inner = self.inner.stack(semitones, count);
        self
    }

    /// Random pitch variation for each note
    pub fn mutate(mut self, max_semitones: i32) -> Self {
        self.inner = self.inner.mutate(max_semitones);
        self
    }

    /// Time stretch/compress by factor
    pub fn stretch(mut self, factor: f32) -> Self {
        self.inner = self.inner.stretch(factor);
        self
    }

    /// Compress to exact target duration
    pub fn compress(mut self, target_duration: f32) -> Self {
        self.inner = self.inner.compress(target_duration);
        self
    }

    /// Snap timing to rhythmic grid
    pub fn quantize(mut self, grid: f32) -> Self {
        self.inner = self.inner.quantize(grid);
        self
    }

    /// Mirror pattern forward then backward
    pub fn palindrome(mut self) -> Self {
        self.inner = self.inner.palindrome();
        self
    }

    /// Random glitchy stuttering
    pub fn stutter(mut self, probability: f32, repeats: usize) -> Self {
        self.inner = self.inner.stutter(probability, repeats);
        self
    }

    /// Deterministic stuttering (every Nth note)
    pub fn stutter_every(mut self, nth: usize, repeats: usize) -> Self {
        self.inner = self.inner.stutter_every(nth, repeats);
        self
    }

    /// Break notes into micro-fragments
    pub fn granularize(mut self, divisions: usize) -> Self {
        self.inner = self.inner.granularize(divisions);
        self
    }

    /// Snap pitches to nearest scale degree
    pub fn magnetize(mut self, scale_notes: &[f32]) -> Self {
        self.inner = self.inner.magnetize(scale_notes);
        self
    }

    /// Gravitational pull toward/away from center pitch
    pub fn gravity(mut self, center_pitch: f32, strength: f32) -> Self {
        self.inner = self.inner.gravity(center_pitch, strength);
        self
    }

    /// Cascading effects through time and pitch
    pub fn ripple(mut self, intensity: f32) -> Self {
        self.inner = self.inner.ripple(intensity);
        self
    }

    /// Reverse timing of all events in pattern
    pub fn reverse(mut self) -> Self {
        self.inner = self.inner.reverse();
        self
    }

    /// Invert pitches around an axis frequency
    pub fn invert(mut self, axis_freq: f32) -> Self {
        self.inner = self.inner.invert(axis_freq);
        self
    }

    /// Invert pitches around axis with range constraints
    pub fn invert_constrained(mut self, axis_freq: f32, min_freq: f32, max_freq: f32) -> Self {
        self.inner = self.inner.invert_constrained(axis_freq, min_freq, max_freq);
        self
    }

    /// Filter to keep only notes within frequency range
    pub fn sieve_inclusive(mut self, min_freq: f32, max_freq: f32) -> Self {
        self.inner = self.inner.sieve_inclusive(min_freq, max_freq);
        self
    }

    /// Filter to remove notes within frequency range
    pub fn sieve_exclusive(mut self, min_freq: f32, max_freq: f32) -> Self {
        self.inner = self.inner.sieve_exclusive(min_freq, max_freq);
        self
    }

    /// Collapse sequential notes into a single chord
    pub fn group(mut self, duration: f32) -> Self {
        self.inner = self.inner.group(duration);
        self
    }

    /// Duplicate all events in the pattern
    pub fn duplicate(mut self) -> Self {
        self.inner = self.inner.duplicate();
        self
    }

    /// Repeat the pattern N times
    pub fn repeat(mut self, times: usize) -> Self {
        self.inner = self.inner.repeat(times);
        self
    }

    /// Play harmonized notes (adds interval above/below)
    pub fn harmonize(mut self, notes: &[f32], semitones: i32, note_duration: f32) -> Self {
        self.inner = self.inner.harmonize(notes, semitones, note_duration);
        self
    }

    /// Play a drum every Nth event in the pattern
    pub fn every_n(mut self, n: usize, drum: DrumType) -> Self {
        self.inner = self.inner.every_n(n, drum);
        self
    }
}

/// Builder for note generators (accessed via `.generator()`)
///
/// Provides a scoped namespace for all note-generating pattern methods.
/// Use with closure syntax for clean, organized code:
///
/// ```rust
/// # use tunes::composition::Composition;
/// # use tunes::composition::rhythm::Tempo;
/// # use tunes::consts::notes::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// comp.track("melody")
///     .generator(|g| g
///         .arpeggiate(&[C4, E4, G4], 0.25)
///         .alberti_bass(&[C4, E4, G4, C5], 0.125)
///         .trill(C4, D4, 8, 0.0625)
///     );
/// ```
pub struct GeneratorBuilder<'a> {
    inner: TrackBuilder<'a>,
}

impl<'a> GeneratorBuilder<'a> {
    /// Unwrap back to the inner TrackBuilder
    fn into_inner(self) -> TrackBuilder<'a> {
        self.inner
    }

    // === CHORDS ===
    /// Generate a chord
    pub fn chord(mut self, root: f32, pattern: &ChordPattern, duration: f32) -> Self {
        self.inner = self.inner.chord(root, pattern, duration);
        self
    }

    /// Generate an inverted chord
    pub fn chord_inverted(mut self, root: f32, pattern: &ChordPattern, inversion: usize, duration: f32) -> Self {
        self.inner = self.inner.chord_inverted(root, pattern, inversion, duration);
        self
    }

    /// Generate chord with voice leading
    pub fn chord_voice_lead(mut self, root: f32, pattern: &ChordPattern, duration: f32) -> Self {
        self.inner = self.inner.chord_voice_lead(root, pattern, duration);
        self
    }

    /// Generate chord over a bass note
    pub fn chord_over_bass(mut self, root: f32, pattern: &ChordPattern, bass: f32, duration: f32) -> Self {
        self.inner = self.inner.chord_over_bass(root, pattern, bass, duration);
        self
    }

    /// Generate a sequence of chords
    pub fn chords(mut self, chord_sequence: &[&[f32]], chord_duration: f32) -> Self {
        self.inner = self.inner.chords(chord_sequence, chord_duration);
        self
    }

    /// Generate chords from vector format
    pub fn chords_from(mut self, chord_vecs: &[Vec<f32>], chord_duration: f32) -> Self {
        self.inner = self.inner.chords_from(chord_vecs, chord_duration);
        self
    }

    // === SCALES ===
    /// Play a scale ascending
    pub fn scale(mut self, scale: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.scale(scale, note_duration);
        self
    }

    /// Play a scale descending
    pub fn scale_reverse(mut self, scale: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.scale_reverse(scale, note_duration);
        self
    }

    /// Play scale up then down
    pub fn scale_updown(mut self, scale: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.scale_updown(scale, note_duration);
        self
    }

    /// Play scale down then up
    pub fn scale_downup(mut self, scale: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.scale_downup(scale, note_duration);
        self
    }

    // === ARPEGGIOS ===
    /// Arpeggiate chord ascending
    pub fn arpeggiate(mut self, chord: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.arpeggiate(chord, note_duration);
        self
    }

    /// Arpeggiate chord descending
    pub fn arpeggiate_reverse(mut self, chord: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.arpeggiate_reverse(chord, note_duration);
        self
    }

    /// Arpeggiate up then down
    pub fn arpeggiate_updown(mut self, chord: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.arpeggiate_updown(chord, note_duration);
        self
    }

    /// Arpeggiate down then up
    pub fn arpeggiate_downup(mut self, chord: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.arpeggiate_downup(chord, note_duration);
        self
    }

    // === CLASSICAL PATTERNS ===
    /// Generate Alberti bass pattern
    pub fn alberti_bass(mut self, chord: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.alberti_bass(chord, note_duration);
        self
    }

    /// Generate waltz bass pattern
    pub fn waltz_bass(mut self, root: f32, chord: &[f32], beat_duration: f32) -> Self {
        self.inner = self.inner.waltz_bass(root, chord, beat_duration);
        self
    }

    /// Generate broken chord pattern
    pub fn broken_chord(mut self, chord: &[f32], pattern_type: u8, note_duration: f32) -> Self {
        self.inner = self.inner.broken_chord(chord, pattern_type, note_duration);
        self
    }

    /// Generate walking bass line
    pub fn walking_bass(mut self, bass_line: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.walking_bass(bass_line, note_duration);
        self
    }

    /// Generate tremolo strings effect
    pub fn tremolo_strings(mut self, notes: &[f32], total_duration: f32, note_speed: f32) -> Self {
        self.inner = self.inner.tremolo_strings(notes, total_duration, note_speed);
        self
    }

    /// Generate ostinato pattern
    pub fn ostinato(mut self, pattern: &[f32], note_duration: f32, repeats: usize) -> Self {
        self.inner = self.inner.ostinato(pattern, note_duration, repeats);
        self
    }

    // === ORNAMENTS ===
    /// Generate trill between two notes
    pub fn trill(mut self, note1: f32, note2: f32, count: usize, note_duration: f32) -> Self {
        self.inner = self.inner.trill(note1, note2, count, note_duration);
        self
    }

    /// Generate cascading notes
    pub fn cascade(mut self, notes: &[f32], note_duration: f32, stagger: f32) -> Self {
        self.inner = self.inner.cascade(notes, note_duration, stagger);
        self
    }

    /// Generate tremolo on single note
    pub fn tremolo_note(mut self, note: f32, count: usize, note_duration: f32) -> Self {
        self.inner = self.inner.tremolo_note(note, count, note_duration);
        self
    }

    /// Generate strummed chord
    pub fn strum(mut self, chord: &[f32], note_duration: f32, stagger: f32) -> Self {
        self.inner = self.inner.strum(chord, note_duration, stagger);
        self
    }

    /// Generate mordent ornament
    pub fn mordent(mut self, main_note: f32, duration: f32) -> Self {
        self.inner = self.inner.mordent(main_note, duration);
        self
    }

    /// Generate inverted mordent
    pub fn inverted_mordent(mut self, main_note: f32, duration: f32) -> Self {
        self.inner = self.inner.inverted_mordent(main_note, duration);
        self
    }

    /// Generate turn ornament
    pub fn turn(mut self, main_note: f32, duration: f32) -> Self {
        self.inner = self.inner.turn(main_note, duration);
        self
    }

    /// Generate inverted turn
    pub fn inverted_turn(mut self, main_note: f32, duration: f32) -> Self {
        self.inner = self.inner.inverted_turn(main_note, duration);
        self
    }

    // === TUPLETS ===
    /// Generate tuplet
    pub fn tuplet(mut self, notes: &[f32], count: usize, total_duration: f32) -> Self {
        self.inner = self.inner.tuplet(notes, count, total_duration);
        self
    }

    /// Generate triplet
    pub fn triplet(mut self, notes: &[f32], total_duration: f32) -> Self {
        self.inner = self.inner.triplet(notes, total_duration);
        self
    }

    /// Generate quintuplet
    pub fn quintuplet(mut self, notes: &[f32], total_duration: f32) -> Self {
        self.inner = self.inner.quintuplet(notes, total_duration);
        self
    }

    /// Generate sextuplet
    pub fn sextuplet(mut self, notes: &[f32], total_duration: f32) -> Self {
        self.inner = self.inner.sextuplet(notes, total_duration);
        self
    }

    /// Generate septuplet
    pub fn septuplet(mut self, notes: &[f32], total_duration: f32) -> Self {
        self.inner = self.inner.septuplet(notes, total_duration);
        self
    }

    // === MUSICAL PATTERNS ===
    /// Generate octave doubling
    pub fn octaves(mut self, notes: &[f32], octave_offset: i32, note_duration: f32) -> Self {
        self.inner = self.inner.octaves(notes, octave_offset, note_duration);
        self
    }

    /// Generate pedal tone with melody
    pub fn pedal(mut self, pedal_note: f32, melody_notes: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.pedal(pedal_note, melody_notes, note_duration);
        self
    }

    /// Generate notes from sequence indices
    pub fn sequence_from(mut self, sequence: &[u32], notes: &[f32], note_duration: f32) -> Self {
        self.inner = self.inner.sequence_from(sequence, notes, note_duration);
        self
    }

    // === PORTAMENTO ===
    /// Generate slide between two pitches
    pub fn slide(mut self, from: f32, to: f32, duration: f32) -> Self {
        self.inner = self.inner.slide(from, to, duration);
        self
    }

    // === TIME-BASED ===
    /// Generate whole notes
    pub fn wholes(mut self, notes: &[f32]) -> Self {
        self.inner = self.inner.wholes(notes);
        self
    }

    /// Generate half notes
    pub fn halves(mut self, notes: &[f32]) -> Self {
        self.inner = self.inner.halves(notes);
        self
    }

    /// Generate quarter notes
    pub fn quarters(mut self, notes: &[f32]) -> Self {
        self.inner = self.inner.quarters(notes);
        self
    }

    /// Generate eighth notes
    pub fn eighths(mut self, notes: &[f32]) -> Self {
        self.inner = self.inner.eighths(notes);
        self
    }

    /// Generate sixteenth notes
    pub fn sixteenths(mut self, notes: &[f32]) -> Self {
        self.inner = self.inner.sixteenths(notes);
        self
    }

    /// Generate notes in an orbital pattern around a center pitch
    pub fn orbit(mut self, center: f32, radius_semitones: f32, steps_per_rotation: usize, step_duration: f32, rotations: f32, clockwise: bool) -> Self {
        self.inner = self.inner.orbit(center, radius_semitones, steps_per_rotation, step_duration, rotations, clockwise);
        self
    }

    /// Generate a bouncing pitch pattern with damping
    pub fn bounce(mut self, start: f32, stop: f32, ratio: f32, bounces: usize, steps_per_segment: usize, step_duration: f32) -> Self {
        self.inner = self.inner.bounce(start, stop, ratio, bounces, steps_per_segment, step_duration);
        self
    }

    /// Generate random notes scattered across a frequency range
    pub fn scatter(mut self, min: f32, max: f32, count: usize, duration: f32) -> Self {
        self.inner = self.inner.scatter(min, max, count, duration);
        self
    }

    /// Generate a stream of repeated notes at a single frequency
    pub fn stream(mut self, freq: f32, count: usize, duration: f32) -> Self {
        self.inner = self.inner.stream(freq, count, duration);
        self
    }

    /// Generate random notes picked from a provided set
    pub fn random_notes(mut self, notes: &[f32], count: usize, duration: f32) -> Self {
        self.inner = self.inner.random_notes(notes, count, duration);
        self
    }

    /// Generate completely random frequencies within a range
    pub fn sprinkle(mut self, min: f32, max: f32, count: usize, duration: f32) -> Self {
        self.inner = self.inner.sprinkle(min, max, count, duration);
        self
    }
}

impl<'a> TrackBuilder<'a> {
    /// Enter note generator namespace
    ///
    /// Provides scoped access to all note-generating pattern methods.
    /// The closure receives a `GeneratorBuilder` and should return it
    /// after generating notes.
    ///
    /// # Example
    /// ```rust
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .generator(|g| g
    ///         .arpeggiate(&[C4, E4, G4], 0.25)
    ///         .scale(&[C4, D4, E4, F4, G4], 0.25)
    ///         .trill(C4, D4, 8, 0.0625)
    ///     );
    /// ```
    pub fn generator<F>(self, f: F) -> Self
    where
        F: FnOnce(GeneratorBuilder<'a>) -> GeneratorBuilder<'a>,
    {
        let builder = GeneratorBuilder { inner: self };
        let result = f(builder);
        result.into_inner()
    }
}

impl<'a> TrackBuilder<'a> {
    /// Enter pattern transformation namespace
    ///
    /// Provides scoped access to all 18 pattern transformation methods.
    /// The closure receives a `TransformBuilder` and should return it
    /// after applying transformations.
    ///
    /// # Example
    /// ```rust
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("melody")
    ///     .pattern_start()
    ///     .notes(&[C4, E4, G4, C5], 0.25)
    ///     .transform(|t| t
    ///         .mutate(3)
    ///         .rotate(1)
    ///         .humanize(0.01, 0.05)
    ///     )
    ///     .wait(1.0);
    /// ```
    pub fn transform<F>(self, f: F) -> Self
    where
        F: FnOnce(TransformBuilder<'a>) -> TransformBuilder<'a>,
    {
        let builder = TransformBuilder { inner: self };
        let result = f(builder);
        result.into_inner()
    }
    /// Generate notes using a random walk within a scale
    ///
    /// Starts at a given frequency and takes random steps through the scale,
    /// creating an organic, wandering melody.
    ///
    /// # Arguments
    /// * `start_freq` - Starting frequency (should be in the scale)
    /// * `steps` - Number of notes to generate
    /// * `note_duration` - Duration of each note
    /// * `scale` - Array of frequencies to constrain the walk to
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::scales::C4_MAJOR_SCALE;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("wanderer")
    ///     .random_walk(C4_MAJOR_SCALE[0], 16, 0.25, &C4_MAJOR_SCALE);
    /// ```
    pub fn random_walk(
        mut self,
        start_freq: f32,
        steps: usize,
        note_duration: f32,
        scale: &[f32],
    ) -> Self {
        if scale.is_empty() || steps == 0 {
            return self;
        }

        let waveform = self.waveform;
        let envelope = self.envelope;
        let pitch_bend = self.pitch_bend;

        // Find starting position in scale
        let mut current_idx = scale
            .iter()
            .position(|&f| (f - start_freq).abs() < 0.1)
            .unwrap_or(0);

        let mut rng = rand::rng();

        for _ in 0..steps {
            let cursor = self.cursor;
            let freq = scale[current_idx];

            self.get_track_mut()
                .add_note_with_waveform_envelope_and_bend(
                    &[freq],
                    cursor,
                    note_duration,
                    waveform,
                    envelope,
                    pitch_bend,
                );

            let swung_duration = self.apply_swing(note_duration);
            self.cursor += swung_duration;

            // Random walk: move up or down by 1-2 scale degrees
            let step = if rng.random::<bool>() {
                rng.random_range(1..=2) // Move up
            } else {
                -(rng.random_range(1..=2)) // Move down
            };

            current_idx = ((current_idx as i32 + step)
                .max(0)
                .min(scale.len() as i32 - 1)) as usize;
        }

        self.update_section_duration();
        self
    }

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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
            match event {
                AudioEvent::Note(note) => {
                    if note.start_time >= pattern_start && note.start_time < cursor {
                        notes_to_granularize.push(event.clone());
                        indices_to_remove.push(idx);
                    }
                }
                _ => {} // Only granularize notes, not drums
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
    /// # use tunes::composition::rhythm::Tempo;
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
                            for i in 0..note.num_freqs {
                                shifted_freqs[i] = note.frequencies[i] * shift_ratio;
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
    /// # use tunes::composition::rhythm::Tempo;
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
                            for i in 0..note.num_freqs {
                                let freq = note.frequencies[i];
                                // Calculate distance from axis in semitones
                                let semitones_from_axis = 12.0 * (freq / axis_freq).log2();
                                // Mirror it
                                let inverted_semitones = -semitones_from_axis;
                                // Convert back to frequency
                                inverted_freqs[i] =
                                    axis_freq * 2.0_f32.powf(inverted_semitones / 12.0);
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
    /// # use tunes::composition::rhythm::Tempo;
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
                            for i in 0..note.num_freqs {
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

                                inverted_freqs[i] = inverted_freq;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::Composition;
    use crate::composition::rhythm::Tempo;
    use crate::consts::notes::*;
    use crate::consts::scales::C4_MAJOR_SCALE;

    // Sequence generation tests

    #[test]
    fn test_random_walk_sequence_generates_correct_length() {
        let seq = random_walk_sequence(3, 16, 0, 7);
        assert_eq!(seq.len(), 16);
    }

    #[test]
    fn test_random_walk_sequence_stays_in_bounds() {
        let seq = random_walk_sequence(5, 100, 0, 12);
        for &val in &seq {
            assert!(val < 12);
        }
    }

    #[test]
    fn test_random_walk_sequence_empty() {
        let seq = random_walk_sequence(0, 0, 0, 10);
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_biased_random_walk_tends_upward() {
        let seq = biased_random_walk_sequence(0, 50, 0, 20, 0.8);
        // Starting at 0 with 80% upward bias, should generally increase
        let avg = seq.iter().sum::<u32>() as f32 / seq.len() as f32;
        assert!(
            avg > 5.0,
            "Average {} should be > 5.0 with upward bias",
            avg
        );
    }

    #[test]
    fn test_biased_random_walk_tends_downward() {
        let seq = biased_random_walk_sequence(19, 50, 0, 20, 0.2);
        // Starting at 19 with 20% upward bias (80% down), should generally decrease
        let avg = seq.iter().sum::<u32>() as f32 / seq.len() as f32;
        assert!(
            avg < 15.0,
            "Average {} should be < 15.0 with downward bias",
            avg
        );
    }

    #[test]
    fn test_random_walk_sequence_with_sequence_from() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let walk = random_walk_sequence(3, 16, 0, 7);
        comp.track("walk")
            .sequence_from(&walk, &C4_MAJOR_SCALE, 0.25);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks()[0].events.len(), 16);
    }

    // TrackBuilder method tests

    #[test]
    fn test_random_walk_generates_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("walk")
            .random_walk(C4, 16, 0.25, &C4_MAJOR_SCALE);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks()[0].events.len(), 16);
    }

    #[test]
    fn test_random_walk_stays_in_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("walk")
            .random_walk(C4, 32, 0.25, &C4_MAJOR_SCALE);

        let mixer = comp.into_mixer();

        // Check that all generated notes are in the scale
        for event in &mixer.tracks()[0].events {
            if let AudioEvent::Note(note) = event {
                let freq = note.frequencies[0];
                let in_scale = C4_MAJOR_SCALE
                    .iter()
                    .any(|&scale_note| (freq - scale_note).abs() < 0.1);
                assert!(in_scale, "Generated note {} not in scale", freq);
            }
        }
    }

    #[test]
    fn test_random_walk_empty_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("walk").random_walk(C4, 16, 0.25, &[]);

        let mixer = comp.into_mixer();
        // Should create no track with empty scale
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_shift_transposes_up() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .shift(12); // Transpose up one octave

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Check frequencies are transposed up an octave
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C5).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E5).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - G5).abs() < 0.1);
        }
    }

    #[test]
    fn test_shift_transposes_down() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .shift(-12); // Transpose down one octave

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Check frequencies are transposed down an octave
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C3).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E3).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - G3).abs() < 0.1);
        }
    }

    #[test]
    fn test_shift_by_zero_no_change() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .shift(0); // No transposition

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should still have original frequencies
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_shift_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().shift(12); // Shift with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_humanize_adds_variance() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4, C4], 0.25)
            .humanize(0.05, 0.2);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // Check that at least one note has non-exact timing or velocity
        // (with high probability due to randomness)
        let mut has_variance = false;
        for event in events {
            if let AudioEvent::Note(note) = event {
                // Check if timing is offset from exact multiples of 0.25
                let expected_times = [0.0, 0.25, 0.5, 0.75];
                let time_exact = expected_times.iter().any(|&t| (note.start_time - t).abs() < 0.001);
                if !time_exact || (note.velocity - 0.7).abs() > 0.01 {
                    has_variance = true;
                    break;
                }
            }
        }
        // With timing_variance=0.05 and velocity_variance=0.2, very likely to have variance
        assert!(has_variance, "Humanize should add some variance");
    }

    #[test]
    fn test_rotate_cycles_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .rotate(1); // Rotate forward by 1

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // After rotate(1): should be E4, G4, C5, C4
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
            assert_eq!(note.start_time, 0.0); // Timing unchanged
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
            assert_eq!(note.start_time, 0.25);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - C5).abs() < 0.1);
            assert_eq!(note.start_time, 0.5);
        }
        if let AudioEvent::Note(note) = &events[3] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.75);
        }
    }

    #[test]
    fn test_rotate_negative() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .rotate(-1); // Rotate backward by 1

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // After rotate(-1): should be G4, C4, E4
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
        }
    }

    #[test]
    fn test_rotate_by_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .rotate(0); // No rotation

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should remain unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_retrograde_reverses_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .retrograde(); // Reverse pitch sequence

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // After retrograde: should be C5, G4, E4, C4 (reversed pitches)
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C5).abs() < 0.1);
            assert_eq!(note.start_time, 0.0); // Timing unchanged
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
            assert_eq!(note.start_time, 0.25);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
            assert_eq!(note.start_time, 0.5);
        }
        if let AudioEvent::Note(note) = &events[3] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.75);
        }
    }

    #[test]
    fn test_retrograde_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().retrograde(); // Retrograde with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_mutate_changes_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .mutate(2); // Each note can shift by -2, -1, 0, +1, or +2 semitones

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // Check that timing is preserved
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert_eq!(note.start_time, 0.25);
        }

        // At least one note should be different (with very high probability)
        let original_freqs = vec![C4, E4, G4, C5];
        let mut has_mutation = false;
        for (i, event) in events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                // Allow for ±2 semitones of variation
                let diff = (note.frequencies[0] - original_freqs[i]).abs();
                if diff > 0.1 {
                    has_mutation = true;
                    break;
                }
            }
        }
        // With 4 notes and mutate(2), very likely at least one changes
        assert!(has_mutation, "Mutate should change at least one note");
    }

    #[test]
    fn test_mutate_by_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .mutate(0); // No mutation

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should remain unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
        }
    }

    #[test]
    fn test_mutate_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().mutate(2); // Mutate with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_stack_octave_above() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4], 0.5)
            .stack(12, 1); // Add one octave above

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 1);

        // Should have 2 frequencies: C4 and C5
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 2);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.frequencies[1] - C5).abs() < 0.1);
        }
    }

    #[test]
    fn test_stack_two_octaves() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4], 0.5)
            .stack(12, 2); // Add two octaves above

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 1);

        // Should have 3 frequencies: C4, C5, C6
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 3);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.frequencies[1] - C5).abs() < 0.1);
            assert!((note.frequencies[2] - C6).abs() < 0.1);
        }
    }

    #[test]
    fn test_stack_octave_below() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4], 0.5)
            .stack(-12, 1); // Add one octave below

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 1);

        // Should have 2 frequencies: C4 and C3
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 2);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.frequencies[1] - C3).abs() < 0.1);
        }
    }

    #[test]
    fn test_stack_perfect_fifth() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4], 0.5)
            .stack(7, 2); // Add perfect fifth and major ninth

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 1);

        // Should have 3 frequencies: C4, G4 (+7), D5 (+14)
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 3);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.frequencies[1] - G4).abs() < 0.1);
            // D5 is 14 semitones above C4
            let d5 = C4 * 2.0_f32.powf(14.0 / 12.0);
            assert!((note.frequencies[2] - d5).abs() < 1.0);
        }
    }

    #[test]
    fn test_stack_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25) // C major chord
            .stack(12, 1); // Add octave above each note

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Each note should be doubled
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 2);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.frequencies[1] - C5).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert_eq!(note.num_freqs, 2);
            assert!((note.frequencies[0] - E4).abs() < 0.1);
            assert!((note.frequencies[1] - E5).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert_eq!(note.num_freqs, 2);
            assert!((note.frequencies[0] - G4).abs() < 0.1);
            assert!((note.frequencies[1] - G5).abs() < 0.1);
        }
    }

    #[test]
    fn test_stack_count_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4], 0.5)
            .stack(12, 0); // No stacking

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should remain unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 1);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_stack_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().stack(12, 1); // Stack with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_stretch_double_speed() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .stretch(2.0); // Half speed (twice as long)

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Check timing is stretched (doubled)
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.start_time - 0.0).abs() < 0.01);
            assert!((note.duration - 1.0).abs() < 0.01); // 0.5 * 2.0 = 1.0
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 1.0).abs() < 0.01); // 0.5 * 2.0 = 1.0
            assert!((note.duration - 1.0).abs() < 0.01);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.start_time - 2.0).abs() < 0.01); // 1.0 * 2.0 = 2.0
            assert!((note.duration - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_stretch_half_speed() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 1.0)
            .stretch(0.5); // Double speed (half duration)

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 2);

        // Check timing is compressed (halved)
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.start_time - 0.0).abs() < 0.01);
            assert!((note.duration - 0.5).abs() < 0.01); // 1.0 * 0.5 = 0.5
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 0.5).abs() < 0.01); // 1.0 * 0.5 = 0.5
            assert!((note.duration - 0.5).abs() < 0.01);
        }
    }

    #[test]
    fn test_stretch_by_one() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 0.25)
            .stretch(1.0); // No change

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should remain unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.start_time - 0.0).abs() < 0.01);
            assert!((note.duration - 0.25).abs() < 0.01);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 0.25).abs() < 0.01);
            assert!((note.duration - 0.25).abs() < 0.01);
        }
    }

    #[test]
    fn test_stretch_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().stretch(2.0); // Stretch with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_compress_to_target_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)  // Naturally 0.75 beats
            .compress(0.5);  // Compress to 0.5 beats

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Should be compressed by factor of 0.5/0.75 = 0.666...
        // First note at 0.0 with duration ~0.167 (0.25 * 0.666)
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.start_time, 0.0);
            assert!((note.duration - 0.167).abs() < 0.01);
        }
        // Second note at ~0.167
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 0.167).abs() < 0.01);
        }
    }

    #[test]
    fn test_compress_expand_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 0.5)  // Naturally 1.0 beat
            .compress(2.0);  // Expand to 2.0 beats

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should be expanded by factor of 2.0
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.start_time, 0.0);
            assert!((note.duration - 1.0).abs() < 0.01);  // 0.5 * 2.0
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 1.0).abs() < 0.01);  // 0.5 * 2.0
        }
    }

    #[test]
    fn test_compress_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().compress(1.0);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_quantize_to_grid() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .wait(0.12)
            .note(&[C4], 0.25)
            .wait(0.11)
            .note(&[E4], 0.25)
            .wait(0.04)
            .note(&[G4], 0.25)
            .quantize(0.25);  // Snap to 16th note grid

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Check quantized to nearest 0.25 grid
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.start_time - 0.0).abs() < 0.01);  // 0.12 → 0.0
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 0.5).abs() < 0.01);  // 0.48 → 0.5
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.start_time - 0.75).abs() < 0.01); // 0.77 → 0.75
        }
    }

    #[test]
    fn test_quantize_eighth_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .wait(0.13)
            .note(&[C4], 0.25)
            .wait(0.24)
            .note(&[E4], 0.25)
            .quantize(0.5);  // Snap to 8th note grid

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Check quantized to nearest 0.5 grid
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.start_time - 0.0).abs() < 0.01);  // 0.13 → 0.0
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 0.5).abs() < 0.01);  // 0.62 → 0.5
        }
    }

    #[test]
    fn test_quantize_preserves_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .wait(0.12)
            .note(&[C4], 0.25)
            .wait(0.01)
            .note(&[E4], 0.25)
            .quantize(0.25);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Pitches should remain unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
        }
    }

    #[test]
    fn test_quantize_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().quantize(0.25); // Quantize with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_palindrome_mirrors_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .palindrome();  // Should become: C4, E4, G4, G4, E4, C4

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 6);  // Original 3 + mirrored 3

        // Check forward sequence
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
            assert_eq!(note.start_time, 0.25);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
            assert_eq!(note.start_time, 0.5);
        }

        // Check reversed sequence (should be G4, E4, C4)
        if let AudioEvent::Note(note) = &events[3] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[4] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[5] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_palindrome_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().palindrome();

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_stutter_adds_repeats() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 0.25)
            .stutter(1.0, 3);  // 100% probability, 3 repeats

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have original 2 notes + 3 stutters for each = 2 + 6 = 8 total
        assert_eq!(events.len(), 8);

        // Check first note and its stutters
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.0);
        }
    }

    #[test]
    fn test_stutter_with_zero_probability() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .stutter(0.0, 4);  // 0% probability

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should remain unchanged (3 notes)
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_stutter_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().stutter(1.0, 4);

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_stutter_every_nth_note() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .stutter_every(2, 3);  // Every 2nd note stutters 3 times

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 4 original + (2 notes * 3 stutters) = 10 total
        assert_eq!(events.len(), 10);

        // Check that 2nd and 4th notes got stuttered
        // Original: C4, E4, G4, C5
        // E4 (2nd) and C5 (4th) should have 3 additional copies each
    }

    #[test]
    fn test_stutter_every_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().stutter_every(2, 4);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_granularize_splits_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("texture")
            .pattern_start()
            .note(&[C4], 1.0)
            .granularize(10);  // Split into 10 grains

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 10);  // 1 note → 10 grains

        // Check first grain
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.0);
            assert!((note.duration - 0.09).abs() < 0.01);  // 1.0/10 * 0.9 = 0.09
        }

        // Check second grain
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.start_time - 0.1).abs() < 0.01);  // 1.0/10 = 0.1
        }
    }

    #[test]
    fn test_granularize_multiple_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("shimmer")
            .pattern_start()
            .notes(&[C4, E4], 0.5)
            .granularize(5);  // Split each into 5 grains

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 10);  // 2 notes * 5 grains = 10 total
    }

    #[test]
    fn test_granularize_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("texture").pattern_start().granularize(10);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_shuffle_reorders_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .shuffle(); // Random reorder

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // Check that timing is preserved
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert_eq!(note.start_time, 0.25);
        }

        // Check that all original frequencies are still present (just reordered)
        let original_freqs = vec![C4, E4, G4, C5];
        let mut result_freqs = Vec::new();
        for event in events {
            if let AudioEvent::Note(note) = event {
                result_freqs.push(note.frequencies[0]);
            }
        }
        result_freqs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut sorted_original = original_freqs.clone();
        sorted_original.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for (a, b) in result_freqs.iter().zip(sorted_original.iter()) {
            assert!((a - b).abs() < 0.1);
        }
    }

    #[test]
    fn test_shuffle_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().shuffle(); // Shuffle with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_thin_removes_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.125)
            .thin(0.5); // Keep ~50%

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have removed some notes (probabilistic, but with 8 notes and 50% probability,
        // very unlikely to keep all or remove all)
        assert!(events.len() < 8, "Should remove some notes");
        assert!(events.len() > 0, "Should keep some notes");
    }

    #[test]
    fn test_thin_keep_all() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .thin(1.0); // Keep all

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4); // All notes kept
    }

    #[test]
    fn test_thin_remove_all() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .thin(0.0); // Remove all

        let mixer = comp.into_mixer();
        // Track exists but all notes removed
        assert_eq!(mixer.tracks().len(), 1);
        assert_eq!(mixer.tracks()[0].events.len(), 0);
    }

    #[test]
    fn test_thin_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().thin(0.5); // Thin with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_invert_mirrors_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .invert(C4);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];

        assert_eq!(track.events.len(), 3);

        // C4 inverted around C4 should stay C4
        if let AudioEvent::Note(note) = &track.events[0] {
            assert!((note.frequencies[0] - C4).abs() < 1.0);
        }

        // E4 is 4 semitones above C4, so inverted should be 4 semitones below
        // E4 = C4 * 2^(4/12), inverted = C4 * 2^(-4/12)
        if let AudioEvent::Note(note) = &track.events[1] {
            let expected = C4 * 2.0_f32.powf(-4.0 / 12.0); // G#3
            assert!((note.frequencies[0] - expected).abs() < 1.0);
        }
    }

    #[test]
    fn test_invert_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody").pattern_start().invert(C4); // Invert with no notes

        let mixer = comp.into_mixer();
        // Should create no track
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_invert_constrained_keeps_in_range() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.5)
            .invert_constrained(C4, C3, C5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];

        // All notes should be between C3 and C5
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert!(note.frequencies[0] >= C3 - 1.0);
                assert!(note.frequencies[0] <= C5 + 1.0);
            }
        }
    }

    #[test]
    fn test_magnetize_snaps_to_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Chromatic notes that should snap to C major pentatonic (C, D, E, G, A)
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, CS4, D4, DS4, E4], 0.25)
            .magnetize(&[C4, D4, E4, G4, A4]);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 5);

        // CS4 should snap to C4 or D4 (equidistant, so could be either)
        if let AudioEvent::Note(note) = &events[1] {
            // CS4 is equidistant from C4 and D4 (1 semitone each way)
            let snapped_to_c = (note.frequencies[0] - C4).abs() < 1.0;
            let snapped_to_d = (note.frequencies[0] - D4).abs() < 1.0;
            assert!(snapped_to_c || snapped_to_d);
        }

        // DS4 should snap to D4 or E4 (equidistant)
        if let AudioEvent::Note(note) = &events[3] {
            // DS4 is equidistant from D4 and E4 (1 semitone each way)
            let snapped_to_d = (note.frequencies[0] - D4).abs() < 1.0;
            let snapped_to_e = (note.frequencies[0] - E4).abs() < 1.0;
            assert!(snapped_to_d || snapped_to_e);
        }
    }

    #[test]
    fn test_magnetize_empty_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .magnetize(&[]); // Empty scale should do nothing

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Notes should be unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_magnetize_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .notes(&[C4, E4, G4], 0.25)
            .magnetize(&[C4, D4, E4]); // No pattern_start()

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_gravity_pulls_toward_center() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C3, C5], 0.5)
            .gravity(C4, 0.5); // 50% pull toward C4

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 2);

        // C3 should move up toward C4
        if let AudioEvent::Note(note) = &events[0] {
            assert!(note.frequencies[0] > C3);
            assert!(note.frequencies[0] < C4);
        }

        // C5 should move down toward C4
        if let AudioEvent::Note(note) = &events[1] {
            assert!(note.frequencies[0] < C5);
            assert!(note.frequencies[0] > C4);
        }
    }

    #[test]
    fn test_gravity_repels_from_center() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 0.5)
            .gravity(D4, -0.3); // Negative strength = repulsion

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 2);

        // C4 should move away (down) from D4
        if let AudioEvent::Note(note) = &events[0] {
            assert!(note.frequencies[0] < C4);
        }

        // E4 should move away (up) from D4
        if let AudioEvent::Note(note) = &events[1] {
            assert!(note.frequencies[0] > E4);
        }
    }

    #[test]
    fn test_gravity_zero_strength() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .gravity(D4, 0.0); // Zero strength = no effect

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Notes should be unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_gravity_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .notes(&[C4, E4, G4], 0.5)
            .gravity(D4, 0.5); // No pattern_start()

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_ripple_affects_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4, C4], 0.25)
            .ripple(0.02);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // Later notes should be shifted more in time
        if let (AudioEvent::Note(note1), AudioEvent::Note(note2)) = (&events[0], &events[1]) {
            let expected_interval = 0.25;
            let actual_interval = note2.start_time - note1.start_time;
            // Second note should be pushed forward
            assert!(actual_interval > expected_interval);
        }

        if let (AudioEvent::Note(note2), AudioEvent::Note(note3)) = (&events[1], &events[2]) {
            let interval = note3.start_time - note2.start_time;
            // Third note interval should be even larger due to accumulation
            assert!(interval > 0.25);
        }
    }

    #[test]
    fn test_ripple_affects_pitch() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4], 0.25)
            .ripple(0.05);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // First note should be unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }

        // Later notes should be shifted up in pitch
        if let AudioEvent::Note(note) = &events[1] {
            assert!(note.frequencies[0] > C4);
        }

        if let AudioEvent::Note(note) = &events[2] {
            assert!(note.frequencies[0] > C4);
        }
    }

    #[test]
    fn test_ripple_zero_intensity() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4], 0.25)
            .ripple(0.0); // Zero intensity = no effect

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // All notes should be unchanged
        for event in events {
            if let AudioEvent::Note(note) = event {
                assert!((note.frequencies[0] - C4).abs() < 0.1);
            }
        }
    }

    #[test]
    fn test_ripple_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .notes(&[C4, C4, C4], 0.25)
            .ripple(0.05); // No pattern_start()

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_transform_closure_syntax() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Test the new closure-based .transform() API
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .transform(|t| t
                .shift(7)           // Transpose up a fifth
                .humanize(0.01, 0.05)
                .rotate(1)          // Rotate pitches
            )
            .wait(1.0);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // First note should be shifted and rotated (originally C4, rotated to E4, then shifted +7)
        if let AudioEvent::Note(note) = &events[0] {
            // E4 + 7 semitones = B4
            let expected = E4 * 2.0_f32.powf(7.0 / 12.0);
            assert!((note.frequencies[0] - expected).abs() < 1.0);
        }
    }

    #[test]
    fn test_transform_chaining_multiple_calls() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Test chaining multiple .transform() calls
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, D4, E4], 0.25)
            .transform(|t| t.shift(12))   // First transform block: up an octave
            .transform(|t| t.rotate(1))   // Second transform block: rotate
            .wait(1.0);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // All notes should be shifted up an octave
        // Original: C4, D4, E4 -> Shifted: C5, D5, E5 -> Rotated: D5, E5, C5
        for event in events {
            if let AudioEvent::Note(note) = event {
                // Should be in the 5th octave (C5 and above)
                assert!(note.frequencies[0] >= C5 - 1.0);
                assert!(note.frequencies[0] <= E5 + 1.0);
            }
        }
    }

    #[test]
    fn test_sieve_inclusive_keeps_only_range() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // C3 (~130Hz), E3 (~165Hz), G3 (~196Hz), C4 (~261Hz), E4 (~330Hz), G4 (~392Hz)
        comp.track("melody")
            .pattern_start()
            .notes(&[C3, E3, G3, C4, E4, G4], 0.25)
            .sieve_inclusive(150.0, 300.0);  // Keep only E3, G3, C4

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 3 notes remaining (E3, G3, C4)
        assert_eq!(events.len(), 3);

        // Verify all remaining frequencies are in range
        for event in events {
            if let AudioEvent::Note(note) = event {
                assert!(note.frequencies[0] >= 150.0);
                assert!(note.frequencies[0] <= 300.0);
            }
        }
    }

    #[test]
    fn test_sieve_inclusive_with_namespace() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .transform(|t| t.sieve_inclusive(250.0, 400.0));

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // C4 (~261Hz), E4 (~330Hz), and G4 (~392Hz) are in range
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_sieve_exclusive_removes_range() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Remove midrange frequencies
        comp.track("melody")
            .pattern_start()
            .notes(&[C3, E3, G3, C4, E4, G4], 0.25)
            .sieve_exclusive(150.0, 300.0);  // Remove E3, G3, C4

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 3 notes remaining (C3, E4, G4)
        assert_eq!(events.len(), 3);

        // Verify all remaining frequencies are outside range
        for event in events {
            if let AudioEvent::Note(note) = event {
                assert!(note.frequencies[0] < 150.0 || note.frequencies[0] > 300.0);
            }
        }
    }

    #[test]
    fn test_sieve_exclusive_with_namespace() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C3, E3, G3, C4, E4, G4], 0.25)
            .transform(|t| t.sieve_exclusive(150.0, 300.0));

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_sieve_inclusive_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Pattern start but no notes - should not crash
        let builder = comp.track("empty")
            .pattern_start()
            .sieve_inclusive(100.0, 500.0);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_sieve_exclusive_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder = comp.track("empty")
            .pattern_start()
            .sieve_exclusive(100.0, 500.0);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_sieve_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Chain both sieves
        comp.track("melody")
            .pattern_start()
            .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25)
            .transform(|t| t
                .sieve_exclusive(100.0, 200.0)  // Remove low frequencies (C3, E3, G3)
                .sieve_exclusive(380.0, 600.0)  // Remove high frequencies (G4, C5)
            );

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should only have C4 and E4 remaining (G3 removed by first, G4+C5 removed by second)
        assert_eq!(events.len(), 2);

        for event in events {
            if let AudioEvent::Note(note) = event {
                let freq = note.frequencies[0];
                // Should be between 200-380 Hz (C4 and E4)
                assert!(freq > 200.0 && freq < 380.0);
            }
        }
    }

    #[test]
    fn test_group_collapses_to_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Sequential notes -> chord
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .group(2.0);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 1 chord instead of 4 notes
        assert_eq!(events.len(), 1);

        // Check it's a chord with 4 frequencies
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 4);
            assert_eq!(note.duration, 2.0);
        }
    }

    #[test]
    fn test_group_with_namespace() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .transform(|t| t.group(1.5));

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_group_updates_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder = comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .group(3.0);

        // Cursor should be at pattern_start + duration
        assert_eq!(builder.cursor, 3.0);
    }

    #[test]
    fn test_group_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder = comp.track("empty")
            .pattern_start()
            .group(2.0);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_duplicate_doubles_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .duplicate();

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 6 notes (3 original + 3 duplicated)
        assert_eq!(events.len(), 6);
    }

    #[test]
    fn test_duplicate_with_transform() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("harmony")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .duplicate()
            .transform(|t| t.shift(12));  // Shift duplicated notes

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 6 notes total
        assert_eq!(events.len(), 6);

        // Last 3 notes should be an octave higher
        if let (AudioEvent::Note(original), AudioEvent::Note(shifted)) = (&events[0], &events[3]) {
            let expected = original.frequencies[0] * 2.0; // One octave up
            assert!((shifted.frequencies[0] - expected).abs() < 1.0);
        }
    }

    #[test]
    fn test_duplicate_preserves_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 0.5)
            .duplicate();

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Check timing is preserved but shifted
        if let (AudioEvent::Note(note1), AudioEvent::Note(note3)) = (&events[0], &events[2]) {
            assert_eq!(note1.start_time, 0.0);
            assert_eq!(note3.start_time, 1.0); // After 2 notes of 0.5 duration each
        }
    }

    #[test]
    fn test_duplicate_with_namespace() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .transform(|t| t.duplicate());

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        assert_eq!(events.len(), 6);
    }

    #[test]
    fn test_duplicate_updates_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder = comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .duplicate();

        // Cursor should be doubled (3 notes * 0.25 * 2 = 1.5)
        assert_eq!(builder.cursor, 1.5);
    }

    #[test]
    fn test_duplicate_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder = comp.track("empty")
            .pattern_start()
            .duplicate();

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_group_then_duplicate() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Group into chord, then duplicate
        comp.track("chords")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .group(1.0)
            .duplicate();

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 2 chords
        assert_eq!(events.len(), 2);

        // Both should be chords with 3 frequencies
        for event in events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.num_freqs, 3);
            }
        }
    }
}
