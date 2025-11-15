//! Builder facades for pattern transformations and generators
//!
//! This module provides scoped namespaces for organizing pattern transformation
//! and generator methods using the builder pattern with closures.

use crate::composition::TrackBuilder;
use crate::instruments::drums::DrumType;
use crate::theory::core::ChordPattern;

/// Builder for pattern transformations (accessed via `.transform()`)
///
/// Provides a scoped namespace for all pattern transformation methods.
/// Use with closure syntax for clean, organized code:
///
/// ```rust
/// # use tunes::composition::Composition;
/// # use tunes::composition::timing::Tempo;
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

    /// Dilate pitch range around center
    pub fn range_dilation(mut self, factor: f32) -> Self {
        self.inner = self.inner.range_dilation(factor);
        self
    }

    /// Shape melodic contour (smooth/exaggerate)
    pub fn shape_contour(mut self, factor: f32) -> Self {
        self.inner = self.inner.shape_contour(factor);
        self
    }

    /// Create echo/delay trail
    pub fn echo(mut self, delay: f32, repeats: usize, decay: f32) -> Self {
        self.inner = self.inner.echo(delay, repeats, decay);
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
/// # use tunes::composition::timing::Tempo;
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
    /// # use tunes::composition::timing::Tempo;
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
    /// # use tunes::composition::timing::Tempo;
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
}
