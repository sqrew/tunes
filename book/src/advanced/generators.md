# Note Generators

Note generators are powerful musical pattern creation tools that produce chords, scales, arpeggios, classical patterns, ornaments, and algorithmic sequences. Tunes provides two syntaxes: direct method calls and a cleaner namespaced API using `.generator()`.

## Overview

Note generators let you:
- **Create chords** - major, minor, seventh chords with inversions
- **Play scales** - ascending, descending, up-down patterns
- **Arpeggiate** - break chords into sequential notes
- **Classical patterns** - Alberti bass, waltz bass, walking bass
- **Ornaments** - trills, mordents, turns, cascades
- **Tuplets** - triplets, quintuplets, and custom divisions
- **Algorithmic patterns** - orbits, bounces, scatter effects

All generators are chainable and work seamlessly with transformations and effects.

---

## Two Ways to Generate Notes

### Direct Method Calls (Classic Syntax)

You can call generator methods directly on the track builder:

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

comp.track("direct")
    .chord(C4, &ChordPattern::MAJOR, 0.5)      // Direct call
    .arpeggiate(&[C4, E4, G4, C5], 0.125)      // Direct call
    .scale(&[C4, D4, E4, F4, G4], 0.25);       // Direct call
```

**Pros:**
- Familiar if you're used to the original API
- Slightly more concise for single operations

**Cons:**
- Clutters autocomplete with 50+ generator methods
- Less organized for complex musical phrases

### Generator Namespace (New Syntax)

Encapsulate generators in a `.generator()` closure for better organization:

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

comp.track("organized")
    .generator(|g| g  // Enter generator namespace
        .chord(C4, &ChordPattern::MAJOR, 0.5)
        .arpeggiate(&[C4, E4, G4, C5], 0.125)
        .scale(&[C4, D4, E4, F4, G4], 0.25)
    );                // Automatically exits namespace
```

**Pros:**
- Cleaner autocomplete - generators only appear inside `.generator()`
- Better organization - visual grouping of related operations
- More readable - clear intent "I'm generating musical patterns here"
- Easy to chain with `.transform()` for complete control

**Cons:**
- Slightly more verbose for single generators

**Both syntaxes work and are fully compatible!** Choose whichever fits your workflow.

---

## Chords

### Basic Chords

Generate chords with a root note and pattern:

```rust
use tunes::prelude::*;
use tunes::theory::core::ChordPattern;

comp.track("chords")
    .generator(|g| g
        .chord(C4, &ChordPattern::MAJOR, 0.5)       // C major
        .chord(F4, &ChordPattern::MAJOR, 0.5)       // F major
        .chord(G4, &ChordPattern::MAJOR7, 0.5)      // G major 7
        .chord(C4, &ChordPattern::MINOR, 0.5)       // C minor
    );
```

**Parameters:**
- `root: f32` - Root note frequency
- `pattern: &ChordPattern` - Chord type (MAJOR, MINOR, DIM, AUG, etc.)
- `duration: f32` - How long the chord plays

**Available chord patterns:**
- `ChordPattern::MAJOR` - Root, major 3rd, perfect 5th
- `ChordPattern::MINOR` - Root, minor 3rd, perfect 5th
- `ChordPattern::DIM` - Diminished
- `ChordPattern::AUG` - Augmented
- `ChordPattern::MAJOR7` - Major 7th
- `ChordPattern::MINOR7` - Minor 7th
- `ChordPattern::DOM7` - Dominant 7th
- And many more...

### Chord Inversions

Rearrange chord notes to change the bass:

```rust
comp.track("inversions")
    .generator(|g| g
        .chord(C4, &ChordPattern::MAJOR, 0.5)           // Root position: C-E-G
        .chord_inverted(C4, &ChordPattern::MAJOR, 1, 0.5)  // 1st inversion: E-G-C
        .chord_inverted(C4, &ChordPattern::MAJOR, 2, 0.5)  // 2nd inversion: G-C-E
    );
```

**Parameters:**
- `inversion: usize` - Which inversion (0 = root, 1 = first, 2 = second, etc.)

### Chord Over Bass

Separate bass note from chord:

```rust
comp.track("slash_chords")
    .generator(|g| g
        .chord_over_bass(C4, &ChordPattern::MAJOR, F3, 0.5)  // C/F (C major over F bass)
        .chord_over_bass(A4, &ChordPattern::MINOR, G3, 0.5)  // Am/G
    );
```

**Parameters:**
- `bass: f32` - Bass note frequency (played below the chord)

### Chord Progressions

Generate multiple chords in sequence:

```rust
// Using chords() with slices
comp.track("progression")
    .generator(|g| g
        .chords(&[
            &[C4, E4, G4],      // C major
            &[F4, A4, C5],      // F major
            &[G4, B4, D5],      // G major
            &[C4, E4, G4],      // C major
        ], 0.5)
    );

// Using chords_from() with Vec
let progression = vec![
    vec![C4, E4, G4],
    vec![A4, C5, E5],
];
comp.track("from_vec")
    .generator(|g| g.chords_from(&progression, 0.5));
```

---

## Scales

### Basic Scale Patterns

Play scales in different directions:

```rust
let c_major = [C4, D4, E4, F4, G4, A4, B4, C5];

comp.track("scales")
    .generator(|g| g
        .scale(&c_major, 0.25)          // Ascending: C D E F G A B C
        .scale_reverse(&c_major, 0.25)  // Descending: C B A G F E D C
        .scale_updown(&c_major, 0.25)   // Up then down
        .scale_downup(&c_major, 0.25)   // Down then up
    );
```

**Parameters:**
- `scale: &[f32]` - Array of frequencies in order
- `note_duration: f32` - Duration of each note

**Use cases:**
- Scale practice patterns
- Melodic runs
- Smooth transitions between sections

---

## Arpeggios

Break chords into sequential notes:

```rust
let c_major_chord = [C4, E4, G4, C5];

comp.track("arpeggios")
    .generator(|g| g
        .arpeggiate(&c_major_chord, 0.25)          // Up: C E G C
        .arpeggiate_reverse(&c_major_chord, 0.25)  // Down: C G E C
        .arpeggiate_updown(&c_major_chord, 0.25)   // Up then down
        .arpeggiate_downup(&c_major_chord, 0.25)   // Down then up
    );
```

**What's happening:**
- Takes a chord (simultaneous notes) and plays them sequentially
- Four patterns: ascending, descending, up-then-down, down-then-up

**Use cases:**
- Harp-like patterns
- Piano accompaniment
- Melodic elaboration of harmony

---

## Classical Patterns

### Alberti Bass

Classic piano accompaniment pattern (low-high-middle-high):

```rust
comp.track("alberti")
    .generator(|g| g
        .alberti_bass(&[C3, E3, G3], 0.125)  // C-G-E-G-C-G-E-G...
    );
```

**What's happening:**
- Takes a 3-note chord
- Plays pattern: 1st note, 3rd note, 2nd note, 3rd note (repeat)
- Named after Domenico Alberti (18th century composer)

**Use cases:**
- Classical piano style
- Gentle, flowing accompaniment
- Mozart/Haydn era music

### Waltz Bass

Three-beat waltz pattern (root on beat 1, chord on beats 2-3):

```rust
comp.track("waltz")
    .generator(|g| g
        .waltz_bass(C3, &[E3, G3], 0.333)  // C - (E+G) - (E+G)
    );
```

**Parameters:**
- `root: f32` - Bass note (plays first)
- `chord: &[f32]` - Upper notes (play on beats 2-3)
- `beat_duration: f32` - Duration of each beat

**Use cases:**
- 3/4 time signatures
- Waltz, mazurka styles
- Oom-pah-pah accompaniment

### Walking Bass

Jazz-style bass line:

```rust
comp.track("walking")
    .generator(|g| g
        .walking_bass(&[C3, E3, G3, A3, F3, A3, G3, F3], 0.25)
    );
```

**What's happening:**
- Plays each note in sequence with equal duration
- Creates a "walking" quarter-note bass line

**Use cases:**
- Jazz and swing
- Blues progressions
- Steady rhythmic foundation

### Broken Chord

Various chord-breaking patterns:

```rust
comp.track("broken")
    .generator(|g| g
        .broken_chord(&[C4, E4, G4], 0, 0.25)  // Pattern 0: 1-2-3
        .broken_chord(&[C4, E4, G4], 1, 0.25)  // Pattern 1: 1-3-2
        .broken_chord(&[C4, E4, G4], 2, 0.25)  // Pattern 2: 3-2-1
    );
```

**Parameters:**
- `pattern_type: u8` - Different ordering patterns (0-7)

### Ostinato

Repeating rhythmic/melodic pattern:

```rust
comp.track("ostinato")
    .generator(|g| g
        .ostinato(&[C4, E4, G4, E4], 0.25, 4)  // Repeat pattern 4 times
    );
```

**Parameters:**
- `pattern: &[f32]` - Notes to repeat
- `note_duration: f32` - Duration per note
- `repeats: usize` - How many times to repeat

**Use cases:**
- Riff-based music
- Minimalist compositions
- Hypnotic backgrounds

### Tremolo Strings

Rapid alternating notes (string tremolo effect):

```rust
comp.track("tremolo")
    .generator(|g| g
        .tremolo_strings(&[C4, E4], 2.0, 0.0625)  // Alternate fast for 2 seconds
    );
```

**Parameters:**
- `notes: &[f32]` - Notes to alternate between
- `total_duration: f32` - Total duration
- `note_speed: f32` - Duration of each micro-note

---

## Ornaments

### Trill

Rapid alternation between two adjacent notes:

```rust
comp.track("trill")
    .generator(|g| g
        .trill(C5, D5, 8, 0.0625)  // C-D-C-D-C-D-C-D (8 notes)
    );
```

**Parameters:**
- `note1: f32`, `note2: f32` - Two notes to alternate
- `count: usize` - Total number of notes
- `note_duration: f32` - Duration per note

### Mordent

Quick ornamental figure (main note, neighbor, main):

```rust
comp.track("mordents")
    .generator(|g| g
        .mordent(C5, 0.25)           // C-B-C (lower neighbor)
        .inverted_mordent(C5, 0.25)  // C-D-C (upper neighbor)
    );
```

**What's happening:**
- Regular mordent: main → lower neighbor → main
- Inverted mordent: main → upper neighbor → main

### Turn

Ornament that "turns" around the main note:

```rust
comp.track("turns")
    .generator(|g| g
        .turn(C5, 0.25)           // D-C-B-C (upper-main-lower-main)
        .inverted_turn(C5, 0.25)  // B-C-D-C (lower-main-upper-main)
    );
```

### Cascade

Notes with increasing delay (harp glissando effect):

```rust
comp.track("cascade")
    .generator(|g| g
        .cascade(&[C4, E4, G4, C5, E5], 0.5, 0.05)  // 50ms stagger between notes
    );
```

**Parameters:**
- `notes: &[f32]` - Notes to cascade
- `note_duration: f32` - Duration of each note
- `stagger: f32` - Delay increment between notes

**Use cases:**
- Harp glissandos
- String section sweeps
- Cinematic rises

### Strum

Guitar-style chord strumming:

```rust
comp.track("strum")
    .generator(|g| g
        .strum(&[C3, G3, C4, E4, G4], 1.0, 0.02)  // 20ms stagger
    );
```

**What's happening:**
- Similar to cascade but optimized for guitar-style strumming
- Each string starts slightly after the previous

### Tremolo Note

Rapid repetition of a single note:

```rust
comp.track("tremolo_note")
    .generator(|g| g
        .tremolo_note(C5, 16, 0.0625)  // C repeated 16 times
    );
```

---

## Tuplets

Fit N notes into unusual time divisions:

```rust
comp.track("tuplets")
    .generator(|g| g
        .triplet(&[C4, E4, G4], 1.0)           // 3 notes in 1 beat
        .quintuplet(&[C4, D4, E4, F4, G4], 1.0)  // 5 notes in 1 beat
        .sextuplet(&[C4, D4, E4, F4, G4, A4], 1.0)  // 6 notes in 1 beat
        .septuplet(&[C4, D4, E4, F4, G4, A4, B4], 1.0)  // 7 notes in 1 beat
    );
```

**What's happening:**
- Takes `total_duration` and divides it equally among notes
- Common in complex rhythmic music

**Custom tuplet:**
```rust
comp.track("custom")
    .generator(|g| g
        .tuplet(&[C4, E4, G4, C5], 11, 2.0)  // 11 notes in 2 beats
    );
```

---

## Time-Based Generators

Shorthand for common note durations:

```rust
comp.track("rhythms")
    .generator(|g| g
        .wholes(&[C4])              // Whole notes (4 beats each)
        .halves(&[C4, E4])          // Half notes (2 beats each)
        .quarters(&[C4, E4, G4])    // Quarter notes (1 beat each)
        .eighths(&[C4, E4, G4, C5]) // Eighth notes (0.5 beats each)
        .sixteenths(&[C4, D4, E4, F4, G4, A4, B4, C5])  // 16ths (0.25 beats)
    );
```

**What's happening:**
- Automatically sets appropriate durations based on note value
- Assumes 4/4 time signature at 120 BPM

---

## Musical Patterns

### Octaves

Play melody with octave doubling:

```rust
comp.track("octaves")
    .generator(|g| g
        .octaves(&[C4, E4, G4], 1, 0.5)   // Each note + octave above
        .octaves(&[C4, E4, G4], -1, 0.5)  // Each note + octave below
    );
```

**Parameters:**
- `octave_offset: i32` - How many octaves to double (positive = up, negative = down)

### Pedal Tone

Sustained bass note with moving melody:

```rust
comp.track("pedal")
    .generator(|g| g
        .pedal(C3, &[C4, D4, E4, F4, G4], 0.25)  // C3 sustained under melody
    );
```

**What's happening:**
- `C3` plays simultaneously with each melody note
- Creates harmonic tension/release as melody moves

### Sequence From Indices

Map numeric sequences to notes:

```rust
comp.track("sequence")
    .generator(|g| g
        .sequence_from(
            &[0, 2, 1, 3, 2, 0],           // Index pattern
            &[C4, E4, G4, C5],              // Note palette
            0.25
        )  // Result: C4, G4, E4, C5, G4, C4
    );
```

**Parameters:**
- `sequence: &[u32]` - Array of indices into the note array
- `notes: &[f32]` - Note palette to choose from

**Use cases:**
- Applying algorithmic sequences to specific note sets
- Converting numeric patterns to music

---

## Portamento

### Slide (Pitch Bend)

Smooth pitch glide between two notes:

```rust
comp.track("slide")
    .generator(|g| g
        .slide(C4, G4, 0.5)   // Glide from C4 to G4 over 0.5 seconds
        .slide(G4, C4, 0.5)   // Glide back down
    );
```

**Parameters:**
- `from: f32` - Starting frequency
- `to: f32` - Ending frequency
- `duration: f32` - Time for the slide

**What's happening:**
- Creates many micro-notes between start and end frequencies
- Simulates trombone slide, theremin, or synth portamento

---

## Algorithmic Generators

### Orbit

Notes that orbit around a center pitch:

```rust
comp.track("orbit")
    .generator(|g| g
        .orbit(
            A4,      // Center frequency
            7.0,     // Radius in semitones
            16,      // Steps per rotation
            0.0625,  // Step duration
            2.0,     // Number of rotations
            true     // Clockwise
        )
    );
```

**Parameters:**
- `center: f32` - Center pitch to orbit around
- `radius_semitones: f32` - Distance from center in semitones
- `steps_per_rotation: usize` - How many notes per full circle
- `step_duration: f32` - Duration per note
- `rotations: f32` - How many complete orbits (can be fractional)
- `clockwise: bool` - Direction

**What's happening:**
- Creates circular melodic motion around a tonal center
- Like planets orbiting a sun, but with pitch

### Bounce

Bouncing ball physics applied to pitch:

```rust
comp.track("bounce")
    .generator(|g| g
        .bounce(
            C5,     // Start pitch (high)
            C4,     // Stop pitch (low/ground)
            0.6,    // Damping ratio (0.0-1.0)
            3,      // Number of bounces
            6,      // Steps per segment
            0.0833  // Step duration
        )
    );
```

**What's happening:**
- Pitch "falls" from start to stop
- "Bounces" back up, each time reaching lower height
- Damping controls how much energy is lost per bounce

**Use cases:**
- Descending melodic patterns
- Cartoon sound effects
- Natural decay patterns

### Scatter

Random notes scattered across a frequency range:

```rust
comp.track("scatter")
    .generator(|g| g
        .scatter(200.0, 800.0, 16, 0.125)  // 16 random notes, 200-800Hz
    );
```

**Parameters:**
- `min: f32`, `max: f32` - Frequency range
- `count: usize` - How many notes
- `duration: f32` - Duration per note

**What's happening:**
- Each note frequency is randomly chosen from the range
- No scale quantization - pure random frequencies

### Stream

Repeated notes (drone/ostinato):

```rust
comp.track("stream")
    .generator(|g| g
        .stream(440.0, 32, 0.0625)  // A4 repeated 32 times
    );
```

**Use cases:**
- Drone tones
- Rhythmic pulses
- Techno kick drums

### Random Notes

Random selection from a note set:

```rust
comp.track("random")
    .generator(|g| g
        .random_notes(&[C4, E4, G4, C5], 16, 0.25)  // Random from C major triad
    );
```

**What's happening:**
- Unlike `scatter`, this picks from a defined set of notes
- Scale-aware randomness

### Sprinkle

Continuous random frequencies (like scatter, but f32):

```rust
comp.track("sprinkle")
    .generator(|g| g
        .sprinkle(250.0, 850.0, 16, 0.125)
    );
```

**What's happening:**
- Similar to scatter but operates on continuous f32 frequencies
- No snapping or quantization

---

## Combining Generators and Transformations

The real power comes from chaining generators with transformations:

```rust
comp.track("complex")
    // Generate notes
    .generator(|g| g
        .chord(C4, &ChordPattern::MAJOR, 0.5)
        .arpeggiate(&[C5, E5, G5, C6], 0.125)
    )
    // Transform the generated pattern
    .transform(|t| t
        .shift(7)              // Transpose up
        .humanize(0.01, 0.05)  // Add feel
        .echo(0.25, 2, 0.5)    // Add echo
    );
```

**Workflow:**
1. Generate musical material (chords, scales, arpeggios)
2. Transform the material (transpose, humanize, etc.)
3. Apply effects (reverb, delay, filters)

---

## Multiple Generator Blocks

You can chain multiple generator blocks for complex phrases:

```rust
comp.track("phrase")
    // First phrase: Chord progression
    .generator(|g| g
        .chord(C4, &ChordPattern::MAJOR, 0.5)
        .chord(A4, &ChordPattern::MINOR, 0.5)
    )
    // Second phrase: Melody on top
    .generator(|g| g
        .arpeggiate(&[E5, G5, B5], 0.125)
        .scale(&[C5, D5, E5, F5, G5], 0.125)
    )
    // Third phrase: Bass line
    .generator(|g| g
        .walking_bass(&[C3, E3, A3, F3], 0.25)
    );
```

**What's happening:**
- Each `.generator()` block runs after the previous completes
- Build complex arrangements by layering different generator types
- All notes from all blocks play on the same track

---

## Complete Example

Here's a full composition using various generators:

```rust
use tunes::prelude::*;
use tunes::theory::core::ChordPattern;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // === CHORD PROGRESSION ===
    comp.instrument("chords", &Instrument::electric_piano())
        .reverb(Reverb::new(0.4, 0.3, 0.5))
        .generator(|g| g
            .chord(C4, &ChordPattern::MAJOR, 2.0)
            .chord(A4, &ChordPattern::MINOR, 2.0)
            .chord(F4, &ChordPattern::MAJOR, 2.0)
            .chord(G4, &ChordPattern::MAJOR7, 2.0)
        );

    // === ARPEGGIO MELODY ===
    comp.instrument("melody", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .generator(|g| g
            .arpeggiate(&[C5, E5, G5, C6], 0.25)
            .arpeggiate(&[A4, C5, E5, A5], 0.25)
        )
        .transform(|t| t
            .humanize(0.01, 0.05)
        );

    // === ALBERTI BASS ===
    comp.instrument("bass", &Instrument::electric_piano())
        .generator(|g| g
            .alberti_bass(&[C3, E3, G3], 0.125)
        )
        .transform(|t| t
            .thin(0.8)  // Remove 20% of notes for breathing room
        );

    // === ORNAMENTAL FLOURISH ===
    comp.instrument("flourish", &Instrument::pluck())
        .wait(14.0)  // Wait until near the end
        .generator(|g| g
            .trill(C6, D6, 16, 0.0625)
            .cascade(&[C6, A5, F5, D5, C5], 0.5, 0.03)
        );

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

**What's happening:**
1. **Chords** provide harmonic foundation
2. **Arpeggios** create melodic interest with delay effect
3. **Alberti bass** adds classical accompaniment texture
4. **Ornaments** provide climactic flourish at the end

---

## Generator Reference

### Chords (6 methods)
- `.chord(root, pattern, duration)` - Basic chord
- `.chord_inverted(root, pattern, inversion, duration)` - Inverted chord
- `.chord_voice_lead(root, pattern, duration)` - Voice-led progression
- `.chord_over_bass(root, pattern, bass, duration)` - Slash chord
- `.chords(chord_sequence, duration)` - Chord progression
- `.chords_from(chord_vecs, duration)` - From Vec format

### Scales (4 methods)
- `.scale(scale, duration)` - Ascending
- `.scale_reverse(scale, duration)` - Descending
- `.scale_updown(scale, duration)` - Up then down
- `.scale_downup(scale, duration)` - Down then up

### Arpeggios (4 methods)
- `.arpeggiate(chord, duration)` - Ascending
- `.arpeggiate_reverse(chord, duration)` - Descending
- `.arpeggiate_updown(chord, duration)` - Up then down
- `.arpeggiate_downup(chord, duration)` - Down then up

### Classical Patterns (6 methods)
- `.alberti_bass(chord, duration)` - Alberti bass pattern
- `.waltz_bass(root, chord, duration)` - 3/4 waltz pattern
- `.broken_chord(chord, pattern, duration)` - Various orderings
- `.walking_bass(bass_line, duration)` - Jazz walking bass
- `.tremolo_strings(notes, total_duration, speed)` - String tremolo
- `.ostinato(pattern, duration, repeats)` - Repeating pattern

### Ornaments (8 methods)
- `.trill(note1, note2, count, duration)` - Rapid alternation
- `.cascade(notes, duration, stagger)` - Harp glissando
- `.tremolo_note(note, count, duration)` - Single note repetition
- `.strum(chord, duration, stagger)` - Guitar strum
- `.mordent(note, duration)` - Main-lower-main
- `.inverted_mordent(note, duration)` - Main-upper-main
- `.turn(note, duration)` - Upper-main-lower-main
- `.inverted_turn(note, duration)` - Lower-main-upper-main

### Tuplets (5 methods)
- `.tuplet(notes, count, total_duration)` - Custom tuplet
- `.triplet(notes, total_duration)` - 3 notes in one beat
- `.quintuplet(notes, total_duration)` - 5 notes
- `.sextuplet(notes, total_duration)` - 6 notes
- `.septuplet(notes, total_duration)` - 7 notes

### Musical Patterns (3 methods)
- `.octaves(notes, offset, duration)` - Octave doubling
- `.pedal(pedal_note, melody, duration)` - Pedal tone
- `.sequence_from(sequence, notes, duration)` - Index mapping

### Portamento (1 method)
- `.slide(from, to, duration)` - Pitch glide

### Time-Based (5 methods)
- `.wholes(notes)` - Whole notes (4 beats)
- `.halves(notes)` - Half notes (2 beats)
- `.quarters(notes)` - Quarter notes (1 beat)
- `.eighths(notes)` - Eighth notes (0.5 beats)
- `.sixteenths(notes)` - Sixteenth notes (0.25 beats)

### Algorithmic (6 methods)
- `.orbit(center, radius, steps, duration, rotations, clockwise)` - Circular motion
- `.bounce(start, stop, ratio, bounces, steps, duration)` - Bouncing physics
- `.scatter(min, max, count, duration)` - Random in range
- `.stream(freq, count, duration)` - Repeated note
- `.random_notes(notes, count, duration)` - Random from set
- `.sprinkle(min, max, count, duration)` - Continuous random

**Total: 50+ methods**

---

## Tips and Best Practices

### 1. Use Generator Blocks for Organization

Group related musical ideas:

```rust
// ✅ GOOD: Clear sections
comp.track("organized")
    .generator(|g| g  // Harmonic foundation
        .chord(C4, &ChordPattern::MAJOR, 1.0)
        .chord(G4, &ChordPattern::MAJOR, 1.0)
    )
    .generator(|g| g  // Melodic elaboration
        .arpeggiate(&[C5, E5, G5], 0.25)
    );

// ❌ BAD: Everything mixed together
comp.track("messy")
    .chord(C4, &ChordPattern::MAJOR, 1.0)
    .arpeggiate(&[C5, E5, G5], 0.25)
    .chord(G4, &ChordPattern::MAJOR, 1.0);
```

### 2. Combine with Transformations

Generators create, transformations modify:

```rust
comp.track("complete")
    .generator(|g| g
        .arpeggiate(&[C4, E4, G4, C5], 0.25)
    )
    .transform(|t| t
        .shift(7)              // Transpose
        .humanize(0.01, 0.05)  // Add realism
    );
```

### 3. Use Classical Patterns for Authentic Styles

Each pattern has historical context:

- **Alberti bass** → Classical era piano (Mozart, Clementi)
- **Waltz bass** → Ballroom dance music
- **Walking bass** → Jazz, swing, blues
- **Ostinato** → Minimalism, riff-based rock

### 4. Layer Multiple Generators

Build rich arrangements:

```rust
// Pad layer
comp.instrument("pad", &Instrument::warm_pad())
    .generator(|g| g.chord(C4, &ChordPattern::MAJOR, 4.0));

// Arpeggio layer
comp.instrument("arp", &Instrument::synth_lead())
    .generator(|g| g.arpeggiate(&[C5, E5, G5], 0.125));

// Bass layer
comp.instrument("bass", &Instrument::sub_bass())
    .generator(|g| g.walking_bass(&[C2, E2, G2, A2], 0.5));
```

### 5. Use Algorithmic Generators for Variety

For generative/dynamic music:

```rust
// Static
comp.track("static")
    .generator(|g| g.chord(C4, &ChordPattern::MAJOR, 2.0));

// Dynamic
comp.track("dynamic")
    .generator(|g| g
        .orbit(C4, 5.0, 16, 0.125, 1.5, true)
        .scatter(200.0, 800.0, 16, 0.125)
    );
```

---

**Next:** Explore [Pattern Transformations](./transformations.md) to modify your generated patterns →
