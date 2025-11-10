# Composition Layer

The `Composition` is where you think musically. It's the creative layer where you define melodies, harmonies, rhythms, and structure using musical concepts like tempo, notes, scales, and instruments.

## What is Composition?

`Composition` understands musical concepts:

- **Tempo** - Beats per minute, note durations (quarter notes, eighths, etc.)
- **Musical Theory** - Scales, chords, progressions, harmonization
- **Structure** - Sections (verse, chorus), arrangements, repeats
- **Instruments** - Synthesis presets with configured sounds
- **Expression** - Volume, panning, pitch bends, vibrato
- **Effects** - Reverb, delay, filters, distortion, and more

**Think of it as:** Musical notation and composition, not raw audio.

---

## Basic Usage

Create a composition and add musical content:

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

// Add a melody
comp.track("melody")
    .notes(&[C4, E4, G4, C5], 0.5);

// Convert to audio
let mixer = comp.into_mixer();
```

All composition work happens through **builder methods** that chain together.

---

## Tempo and Time

### Setting Tempo

```rust
let comp = Composition::new(Tempo::new(120.0));  // 120 BPM
```

### Tempo-Aware Durations

```rust
let quarter = comp.tempo().quarter_note();   // Duration in seconds
let eighth = comp.tempo().eighth_note();
let sixteenth = comp.tempo().sixteenth_note();
let whole = comp.tempo().whole_note();
```

**Example:**

```rust
let mut comp = Composition::new(Tempo::new(140.0));
let quarter = comp.tempo().quarter_note();

comp.track("drums")
    .note(&[80.0], quarter)   // Quarter note at 140 BPM
    .note(&[80.0], quarter);
```

### Time Signatures

Insert time signature changes:

```rust
comp.track("drums")
    .time_signature(4, 4)  // 4/4 time
    .drum_grid(16, 0.125)
    .kick(&[0, 4, 8, 12]);
```

### Tempo Changes

Insert tempo changes mid-composition:

```rust
comp.track("melody")
    .tempo(120.0)
    .notes(&[C4, E4], 0.5)
    .tempo(140.0)  // Speed up
    .notes(&[G4, C5], 0.5);
```

---

## Tracks vs Instruments

This is a key distinction in the API.

### `.track()` - Raw Audio

Use for samples, drums, and direct synthesis:

```rust
comp.track("drums")
    .drum_grid(16, 0.125)
    .kick(&[0, 4, 8, 12])
    .snare(&[4, 12]);
```

**When to use:**
- Drum programming
- Sample playback
- Custom synthesis without presets
- Raw audio events

### `.instrument()` - Synthesis Presets

Use for melodic content with pre-configured sounds:

```rust
comp.instrument("piano", &Instrument::electric_piano())
    .notes(&[C4, E4, G4, C5], 0.5);
```

**When to use:**
- Melodic instruments
- Bass lines
- Pads and textures
- Any time you want a preset sound
**Over 100 presets** 

---

## Playing Notes

### Single Notes and Chords

```rust
// Single note (440 Hz = A4)
comp.track("lead")
    .note(&[440.0], 0.5);  // 0.5 second duration

// Chord (multiple frequencies)
comp.track("piano")
    .note(&[C4, E4, G4], 1.0);  // C major chord
```

### Note Sequences

```rust
// Play sequence of notes, each 0.25 seconds
comp.track("melody")
    .notes(&[C4, D4, E4, F4, G4], 0.25);
```

### Using Note Constants

Tunes provides constants for all notes:

```rust
comp.track("melody")
    .notes(&[C4, E4, G4, C5], 0.5);  // C major arpeggio
```

**Available:** `C0` through `B8` with sharps (`CS4`, `DS4`, etc.) and flats (`DB4`, `EB4`, `AB4`, `BB4`, etc.)

Note constants use uppercase for consistency (e.g., `CS4` for C#4, `BB4` for Bb4).

---

## Drums

### Individual Drum Hits

```rust
use tunes::prelude::*;

comp.track("drums")
    .drum(DrumType::Kick)
    .rest(0.25)
    .drum(DrumType::Snare)
    .rest(0.25);
```

**Available drum types:** `Kick`, `Snare`, `HiHat`, `ClosedHiHat`, `OpenHiHat`, `Tom`, `Clap`, `Rimshot`, `Cowbell`, `Crash`, `Ride`

### Drum Grid Pattern

More convenient for beat programming:

```rust
comp.track("drums")
    .drum_grid(16, 0.125)    // 16 steps, each 0.125 seconds
    .kick(&[0, 4, 8, 12])    // Kick on beats 1, 2, 3, 4
    .snare(&[4, 12])         // Snare on 2 and 4
    .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);  // Eighth note hi-hats
```

### Rhythm Strings

Parse rhythm patterns from strings:

```rust
comp.track("drums")
    .rhythm("x-x- x-x-", DrumType::Kick, 0.125);
    // 'x' = hit, '-' = rest
```

---

## Musical Patterns

### Scales

Play scales automatically:

```rust
use tunes::theory::core::{scale, ScalePattern};

let c_major = scale(C4, &ScalePattern::MAJOR);

comp.track("melody")
    .scale(&c_major, 0.25);  // Ascending scale
```

Other scale methods:

```rust
comp.track("melody")
    .scale_reverse(&c_major, 0.25)      // Descending
    .scale_updown(&c_major, 0.25)       // Up then down
    .scale_downup(&c_major, 0.25);      // Down then up
```

**Available scales:** `MAJOR`, `MINOR`, `HARMONIC_MINOR`, `MELODIC_MINOR`, `MAJOR_PENTATONIC`, `MINOR_PENTATONIC`, `BLUES`, `DORIAN`, `PHRYGIAN`, `LYDIAN`, `MIXOLYDIAN`, and 40+ more.

### Arpeggios

Play chords as arpeggios:

```rust
use tunes::theory::core::{chord, ChordPattern};

let c_maj7 = chord(C4, &ChordPattern::MAJOR7);

comp.track("arp")
    .arpeggiate(&c_maj7, 0.25);  // Ascending arpeggio
```

Other arpeggio methods:

```rust
comp.track("arp")
    .arpeggiate_reverse(&c_maj7, 0.25)   // Descending
    .arpeggiate_updown(&c_maj7, 0.25)    // Up then down
    .arpeggiate_downup(&c_maj7, 0.25);   // Down then up
```

**Available chords:** `MAJOR`, `MINOR`, `DIMINISHED`, `AUGMENTED`, `MAJOR7`, `MINOR7`, `DOMINANT7`, `DIMINISHED7`, `SUS2`, `SUS4`, `ADD9`, `NINTH`, `POWER`

### Chord Progressions

Generate chord progressions from scale degrees:

```rust
// I-V-vi-IV progression (pop progression)
comp.track("chords")
    .progression(C4, &ScalePattern::MAJOR, &[1, 5, 6, 4], 1.0);

// Jazz ii-V-I with 7th chords
comp.track("chords")
    .progression_7th(C4, &ScalePattern::MAJOR, &[2, 5, 1], 2.0);
```

### Playing Multiple Chords

```rust
let chords = [
    &[C4, E4, G4][..],   // C major
    &[F4, A4, C5][..],   // F major
    &[G4, B4, D5][..],   // G major
    &[C4, E4, G4][..],   // C major
];

comp.track("progression")
    .chords(&chords, 2.0);  // Each chord for 2 seconds
```

### Harmonization

Add harmony intervals to melodies:

```rust
// Harmonize a third above
comp.track("harmony")
    .harmonize(&[C4, D4, E4, F4], 4, 0.5);  // 4 semitones = major third

// Octave doubling
comp.track("octaves")
    .octaves(&[C3, D3, E3, F3], 12, 0.5);  // 12 semitones = octave
```

### Pedal Tones

Sustained bass note with melody above:

```rust
comp.track("pedal")
    .pedal(C2, &[E4, G4, C5, G4], 0.5);  // C2 sustained, melody plays
```

---

## Synthesis

### Waveforms

Set the basic waveform:

```rust
comp.track("synth")
    .waveform(Waveform::Sawtooth)
    .notes(&[C4, E4, G4], 0.5);
```

**Available:** `Sine`, `Square`, `Sawtooth`, `Triangle`, `Noise`

### Envelopes (ADSR)

Control how notes fade in and out:

```rust
comp.track("pluck")
    .envelope(Envelope::new(0.001, 0.1, 0.0, 0.1))
    .notes(&[C4, E4, G4], 0.5);
```

**Parameters:** `Envelope::new(attack, decay, sustain, release)`
- `attack` - Time to reach full volume
- `decay` - Time to drop to sustain level
- `sustain` - Sustained volume level (0.0-1.0)
- `release` - Time to fade after note ends

**Presets:**
```rust
Envelope::pluck()    // Quick attack, fast decay
Envelope::pad()      // Slow attack, long release
Envelope::organ()    // Instant on/off
```

### FM Synthesis

Add harmonic complexity with frequency modulation:

```rust
comp.track("fm_bass")
    .fm_custom(2.0, 3.0)  // modulation ratio, modulation index
    .notes(&[C2, G2], 0.5);
```

Or use presets:

```rust
use tunes::synthesis::fm::FMParams;

comp.track("bell")
    .fm(FMParams::bell())
    .note(&[C5], 2.0);
```

**FM presets:** `bell()`, `electric_piano()`, `brass()`, `bass()`

### Filter Envelopes

Modulate filter cutoff over time:

```rust
use tunes::synthesis::subtractive::FilterEnvelope;

comp.track("acid")
    .filter(Filter::low_pass(200.0, 0.7))
    .filter_envelope(FilterEnvelope::new(200.0, 2000.0, 0.1, 0.5))
    .notes(&[C3, C3, C3, C3], 0.25);
```

### Granular Synthesis

Texture generation from samples:

```rust
use tunes::synthesis::granular::GranularParams;

comp.track("texture")
    .granular(
        "sample.wav",
        GranularParams::new(0.05, 0.8, 1.0),  // grain size, density, pitch
        5.0  // duration
    );
```

### Additive Synthesis

Build sounds from harmonic components:

```rust
// Sawtooth-like sound (harmonic series)
comp.track("saw")
    .additive_synth(&[1.0, 0.5, 0.33, 0.25, 0.2])
    .notes(&[C4, E4, G4], 0.5);

// Organ sound (odd harmonics only)
comp.track("organ")
    .additive_synth(&[1.0, 0.0, 0.5, 0.0, 0.3])
    .notes(&[C3], 1.0);
```

**Parameters:** Array of harmonic amplitudes (1st, 2nd, 3rd, etc.). Zero values are skipped for efficiency.

### Wavetable Synthesis

Rich, harmonically complex waveforms:

```rust
// Rich wavetable preset (8 harmonics)
comp.track("lead")
    .wavetable()
    .notes(&[C4, D4, E4, G4], 0.5);
```

For custom wavetables, use `.custom_waveform()` with `Wavetable::from_harmonics()`.

---

## Effects

Add effects to any track with method chaining.

### Filters

```rust
comp.track("bass")
    .filter(Filter::low_pass(500.0, 0.7))  // cutoff, resonance
    .notes(&[C2, G2], 0.5);
```

**Types:** `low_pass()`, `high_pass()`, `band_pass()`

### Time-Based Effects

```rust
// Delay
comp.track("echo")
    .delay(Delay::new(0.375, 0.4, 0.5))  // time, feedback, mix
    .notes(&[C4, E4], 1.0);

// Reverb
comp.track("hall")
    .reverb(Reverb::new(0.8, 0.5, 0.3))  // room_size, damping, mix
    .notes(&[C4, E4, G4], 2.0);

// Chorus
comp.track("wide")
    .chorus(Chorus::new(0.5, 0.3, 0.5))  // rate, depth, mix
    .notes(&[C4, E4, G4], 1.0);
```

### Distortion

```rust
// Distortion
comp.track("dirty")
    .distortion(Distortion::new(0.7, 0.8))  // drive, mix
    .notes(&[C3, G3], 0.5);

// Saturation (warmer than distortion)
comp.track("warm")
    .saturation(Saturation::new(0.5, 0.7, 0.6))  // drive, character, mix
    .notes(&[C4, E4], 1.0);

// Bitcrusher (lo-fi)
comp.track("lofi")
    .bitcrusher(BitCrusher::new(8, 0.5, 0.7))  // bits, rate_reduction, mix
    .notes(&[C4, E4, G4], 0.5);
```

### Dynamics

```rust
// Compressor
comp.track("punchy")
    .compressor(Compressor::new(-20.0, 4.0, 0.01, 0.1, 3.0))
    // threshold, ratio, attack, release, makeup
    .notes(&[C4, E4], 0.5);

// Limiter
comp.track("limited")
    .limiter(Limiter::new(-3.0, 0.05))  // threshold, release
    .notes(&[C4, E4, G4], 1.0);
```

### Modulation

```rust
// Tremolo (amplitude modulation)
comp.track("tremolo")
    .tremolo(Tremolo::new(4.0, 0.5))  // rate, depth
    .notes(&[C4], 2.0);

// Auto-pan
comp.track("moving")
    .autopan(AutoPan::new(0.5, 0.8))  // rate, depth
    .notes(&[C4, E4], 2.0);
```

### Chaining Effects

Stack multiple effects:

```rust
comp.instrument("lead", &Instrument::synth_lead())
    .filter(Filter::low_pass(2000.0, 0.5))
    .distortion(Distortion::new(0.3, 0.5))
    .delay(Delay::new(0.375, 0.3, 0.4))
    .reverb(Reverb::new(0.5, 0.5, 0.3))
    .notes(&[C4, E4, G4, C5], 0.5);
```

Effects are processed in the optimal order automatically.

---

## Expression and Dynamics

### Volume

```rust
comp.track("quiet")
    .volume(0.5)  // 50% volume
    .notes(&[C4, E4], 0.5);
```

### Panning

```rust
comp.track("left")
    .pan(-1.0)  // Full left (-1.0 to 1.0)
    .notes(&[C4], 1.0);

comp.track("right")
    .pan(1.0)   // Full right
    .notes(&[G4], 1.0);
```

### Velocity

Control note intensity:

```rust
comp.track("dynamics")
    .velocity(0.3)  // Quiet
    .note(&[C4], 0.5)
    .velocity(0.8)  // Loud
    .note(&[E4], 0.5)
    .velocity(1.0)  // Maximum
    .note(&[G4], 0.5);
```

### Pitch Bend

Bend pitch of subsequent notes:

```rust
comp.track("bend")
    .bend(2.0)  // Bend up 2 semitones
    .note(&[C4], 1.0);
```

### Vibrato

Add pitch wobble:

```rust
comp.track("vibrato")
    .vibrato(5.0, 0.5)  // rate (Hz), depth (semitones)
    .notes(&[C4, E4, G4], 1.0);
```

### Automation

Set parameters and advance time:

```rust
// Fade volume (sets final volume, advances cursor)
comp.track("fade")
    .volume(1.0)
    .note(&[C4], 0.5)
    .fade_to(0.0, 2.0);  // Sets volume to 0.0, waits 2 seconds

// Pan sweep (sets final pan, advances cursor)
comp.track("sweep")
    .pan(-1.0)
    .note(&[C4], 0.5)
    .pan_to(1.0, 2.0);  // Sets pan to 1.0, waits 2 seconds

// Filter sweep (sets final filter, advances cursor)
comp.track("filter")
    .filter(Filter::low_pass(200.0, 0.7))
    .note(&[C3], 1.0)
    .filter_sweep(2000.0, 3.0);  // Sets cutoff to 2000Hz, waits 3 seconds
```

> **Note:** These methods set the target value instantly and advance the cursor by the duration. For smooth per-sample automation, use the AudioEngine's real-time control methods during playback or use long envelope release times for volume fades.

---

## Timing and Positioning

### Cursor Position

The composition maintains a "cursor" position that advances as you add events.

### Absolute Positioning

Jump to a specific time:

```rust
comp.track("melody")
    .at(0.0)     // Jump to start
    .note(&[C4], 0.5)
    .at(2.0)     // Jump to 2 seconds
    .note(&[E4], 0.5);
```

### Relative Positioning

```rust
// Wait (advance cursor)
comp.track("delayed")
    .wait(1.0)  // Wait 1 second
    .note(&[C4], 0.5);

// Seek (can be negative)
comp.track("backtrack")
    .note(&[C4], 0.5)
    .seek(-0.25)  // Back up 0.25 seconds
    .note(&[E4], 0.5);  // Overlaps with previous note
```

### Rest

Convenient alias for `wait()`:

```rust
comp.track("spaced")
    .note(&[C4], 0.25)
    .rest(0.25)  // Rest for a quarter note
    .note(&[E4], 0.25);
```

### Markers

Save and return to positions:

```rust
comp.track("structure")
    .mark("verse_start")  // Save position
    .notes(&[C4, E4, G4], 0.5)
    .at_mark("verse_start")  // Return to saved position
    .notes(&[C3, E3, G3], 0.5);  // Play at same time (layered)
```

### Swing

Add groove to timing:

```rust
comp.track("groovy")
    .swing(0.67)  // Triplet swing
    .notes(&[C4, D4, E4, F4, G4, F4, E4, D4], 0.125);
```

**Swing values:**
- `0.5` - Straight (no swing)
- `0.67` - Triplet feel
- `0.75` - Heavy swing

---

## Sections and Arrangement

Sections let you define reusable parts of your composition.

### Defining Sections

```rust
comp.section("verse")
    .instrument("bass", &Instrument::sub_bass())
    .notes(&[C2, C2, G2, F2], 0.5)
    .and()  // Switch to another track
    .track("drums")
    .drum_grid(16, 0.125)
    .kick(&[0, 4, 8, 12])
    .snare(&[4, 12]);

comp.section("chorus")
    .instrument("lead", &Instrument::synth_lead())
    .notes(&[C4, E4, G4, C5], 0.25)
    .and()
    .track("drums")
    .drum_grid(16, 0.125)
    .kick(&[0, 2, 4, 6, 8, 10, 12, 14])  // Double-time
    .snare(&[4, 12]);
```

### Arranging Sections

```rust
comp.arrange(&[
    "verse",
    "verse",
    "chorus",
    "verse",
    "chorus",
    "chorus"
]);
```

This creates the full song structure by sequencing your sections.

### Exporting Sections

Export individual sections:

```rust
let verse_mixer = comp.section_to_mixer("verse")?;
verse_mixer.export_wav("verse.wav", 44100)?;
```

---

## Sample Loading and Playback

### Loading Samples

Load WAV files into the composition:

```rust
comp.load_sample("kick", "samples/kick.wav")?;
comp.load_sample("snare", "samples/snare.wav")?;
```

### Playing Samples

```rust
comp.track("sampler")
    .sample("kick")
    .rest(0.5)
    .sample("snare")
    .rest(0.5);
```

### Pitch Shifting Samples

```rust
comp.track("sampler")
    .sample_with_rate("kick", 1.5)   // Play 1.5x speed (higher pitch)
    .rest(0.5)
    .sample_with_rate("kick", 0.5);  // Play 0.5x speed (lower pitch)
```

---

## Templates

Save and reuse instrument configurations.

### Saving Templates

```rust
comp.instrument("my_lead", &Instrument::synth_lead())
    .waveform(Waveform::Sawtooth)
    .envelope(Envelope::pluck())
    .filter(Filter::low_pass(2000.0, 0.5))
    .reverb(Reverb::new(0.3, 0.4, 0.2))
    .delay(Delay::new(0.375, 0.3, 0.4))
    .save_template("lead_template");
```

### Using Templates

```rust
comp.from_template("lead_template", "melody")
    .notes(&[C4, E4, G4, C5], 0.5);

comp.from_template("lead_template", "harmony")
    .notes(&[E4, G4, C5, E5], 0.5);
```

Templates save all synthesis settings, effects, and mix parameters for reuse.

---

## Pattern Transformations

### Repeating Patterns

```rust
comp.track("pattern")
    .pattern_start()
    .notes(&[C4, E4, G4], 0.25)
    .repeat(3);  // Repeat the pattern 3 more times
```

### Reversing

```rust
comp.track("reverse")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.25)
    .reverse();  // Play C5, G4, E4, C4
```

### Speed Changes

```rust
comp.track("fast")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .speed(2.0);  // Play pattern at 2x speed
```

### Probabilistic Events

```rust
comp.track("random")
    .notes(&[C4, D4, E4, F4, G4, F4, E4, D4], 0.25)
    .probability(0.7);  // 70% chance each note plays
```

---

## Complete Examples

### Full Song Structure

```rust
use tunes::prelude::*;
use tunes::theory::core::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Define verse
    comp.section("verse")
        .instrument("bass", &Instrument::sub_bass())
        .notes(&[C2, C2, G2, F2], 0.5)
        .and()
        .track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

    // Define chorus
    comp.section("chorus")
        .instrument("lead", &Instrument::synth_lead())
        .filter(Filter::low_pass(2000.0, 0.5))
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .notes(&[C4, E4, G4, C5], 0.25)
        .and()
        .instrument("bass", &Instrument::sub_bass())
        .notes(&[C2, C2, G2, F2], 0.5)
        .and()
        .track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 2, 4, 6, 8, 10, 12, 14])
        .snare(&[4, 12]);

    // Arrange the song
    comp.arrange(&["verse", "verse", "chorus", "verse", "chorus", "chorus"]);

    // Play it
    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
```

### Generative Music

```rust
use tunes::prelude::*;
use tunes::theory::core::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(90.0));

    // Create a scale
    let scale = scale(C4, &ScalePattern::MINOR_PENTATONIC);

    // Ambient pad
    comp.instrument("pad", &Instrument::synth_pad())
        .volume(0.3)
        .reverb(Reverb::new(0.8, 0.6, 0.5))
        .chords(&[
            &[C3, E3, G3][..],
            &[A2, C3, E3][..],
            &[F3, A3, C4][..],
            &[G3, B3, D4][..],
        ], 4.0);

    // Melodic pattern
    comp.instrument("melody", &Instrument::bell())
        .volume(0.6)
        .delay(Delay::new(0.375, 0.4, 0.5))
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .arpeggiate(&scale, 0.25)
        .arpeggiate_reverse(&scale, 0.25);

    // Play
    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
```

---

## Method Chaining Philosophy

All builder methods return `self`, enabling fluent composition:

```rust
comp.instrument("complete", &Instrument::synth_lead())
    .waveform(Waveform::Sawtooth)
    .envelope(Envelope::pluck())
    .volume(0.7)
    .pan(0.3)
    .filter(Filter::low_pass(2000.0, 0.6))
    .distortion(Distortion::new(0.3, 0.5))
    .delay(Delay::new(0.375, 0.3, 0.4))
    .reverb(Reverb::new(0.5, 0.5, 0.3))
    .vibrato(5.0, 0.3)
    .save_template("my_sound")
    .notes(&[C4, E4, G4, C5], 0.25);
```

Build entire musical ideas in a single fluent chain.

---

**Next Steps:**
- [Mixer Layer](./mixer.md) - Understand audio rendering
- [AudioEngine Layer](./engine.md) - Play and control audio
- [Synthesis Basics](../synthesis/basics.md) - Deep dive into synthesis
- [Algorithmic Composition](../advanced/algorithmic.md) - Generative music techniques
