# Tempo and Rhythm

Tempo is the heartbeat of music - it defines how musical time (beats, measures) translates to real time (seconds). Tunes provides powerful tempo management, rhythm notation, and timing utilities.

## Overview

The tempo system handles:
- **BPM (Beats Per Minute)** - The speed of the music
- **Note durations** - Musical time values (whole, half, quarter notes, etc.)
- **Tempo changes** - Ritardando, accelerando, and multi-section tempos
- **Rhythm notation** - String-based drum pattern programming
- **Time conversion** - Musical beats ↔ seconds

---

## Creating a Tempo

Every composition starts with a tempo:

```rust
use tunes::prelude::*;

// Create composition at 120 BPM
let mut comp = Composition::new(Tempo::new(120.0));

// Different tempos
let slow = Tempo::new(60.0);      // Adagio
let moderate = Tempo::new(120.0); // Common tempo
let fast = Tempo::new(180.0);     // Presto
```

**BPM ranges:**
- **20-60 BPM** - Very slow (Largo, Adagio)
- **60-80 BPM** - Slow (Andante)
- **80-120 BPM** - Moderate (Moderato, Allegretto)
- **120-168 BPM** - Fast (Allegro, Vivace)
- **168-500 BPM** - Very fast (Presto, Prestissimo)

**Clamping:** BPM values are automatically clamped to 20-500 to prevent errors.

---

## Note Durations

The `NoteDuration` enum represents standard musical time values:

```rust
use tunes::composition::rhythm::NoteDuration;

// Basic note values
NoteDuration::Whole;          // 4 beats (whole note)
NoteDuration::Half;           // 2 beats
NoteDuration::Quarter;        // 1 beat
NoteDuration::Eighth;         // 0.5 beats
NoteDuration::Sixteenth;      // 0.25 beats
NoteDuration::ThirtySecond;   // 0.125 beats

// Dotted notes (1.5x duration)
NoteDuration::DottedHalf;     // 3 beats
NoteDuration::DottedQuarter;  // 1.5 beats
NoteDuration::DottedEighth;   // 0.75 beats

// Triplets (2/3 duration)
NoteDuration::QuarterTriplet; // 2/3 beat
NoteDuration::EighthTriplet;  // 1/3 beat
```

### Converting Durations to Seconds

Use the tempo to convert musical time to real time:

```rust
let tempo = Tempo::new(120.0);

// Using NoteDuration enum
let quarter = tempo.duration_to_seconds(NoteDuration::Quarter);
// At 120 BPM: 0.5 seconds

let dotted_eighth = tempo.duration_to_seconds(NoteDuration::DottedEighth);
// At 120 BPM: 0.375 seconds

// Using convenience methods (faster for common values)
let quarter = tempo.quarter_note();    // 0.5s at 120 BPM
let eighth = tempo.eighth_note();      // 0.25s
let sixteenth = tempo.sixteenth_note(); // 0.125s
let whole = tempo.whole_note();        // 2.0s
```

**Formula:** `duration_in_seconds = (beats / BPM) * 60`

**Examples at 120 BPM:**
- Quarter note (1 beat) = 0.5 seconds
- Eighth note (0.5 beats) = 0.25 seconds
- Dotted quarter (1.5 beats) = 0.75 seconds

**Examples at 60 BPM:**
- Quarter note (1 beat) = 1.0 second
- Eighth note (0.5 beats) = 0.5 seconds
- Whole note (4 beats) = 4.0 seconds

---

## Tempo Changes

Change tempo at any point in your composition for dynamic expression:

### Sudden Tempo Changes

```rust
let mut comp = Composition::new(Tempo::new(120.0));

comp.track("melody")
    // Fast section at 120 BPM
    .notes(&[C4, D4, E4, F4], 0.25)
    .notes(&[G4, A4, B4, C5], 0.25)
    .wait(0.5)
    // Suddenly slow to 80 BPM
    .tempo(80.0)
    .notes(&[C5, B4, A4, G4], 0.5)
    .notes(&[F4, E4, D4, C4], 0.5);
```

**What happens:**
- All durations after `.tempo()` use the new BPM
- The transition is immediate
- Applies to all tracks in the composition

### Ritardando (Gradual Slowdown)

Create a gradual slowdown by changing tempo in steps:

```rust
comp.track("ritardando")
    .note(&[C3, E3, G3], 1.0)  // 120 BPM
    .tempo(110.0)
    .note(&[D3, F3, A3], 1.0)  // 110 BPM
    .tempo(100.0)
    .note(&[E3, G3, B3], 1.0)  // 100 BPM
    .tempo(80.0)
    .note(&[F3, A3, C4], 2.0); // 80 BPM - held longer
```

**Use cases:**
- Ending a piece naturally
- Transitioning between sections
- Creating dramatic tension

### Accelerando (Gradual Speedup)

The opposite of ritardando - gradually speed up:

```rust
comp.track("accelerando")
    .tempo(60.0)   // Start slow
    .notes(&[G4, G4], 0.5)
    .tempo(80.0)
    .notes(&[A4, A4], 0.4)
    .tempo(100.0)
    .notes(&[B4, B4], 0.3)
    .tempo(120.0)
    .notes(&[C5, C5], 0.25)
    .tempo(140.0)
    .notes(&[D5, D5, E5, F5], 0.2);
```

**Use cases:**
- Building excitement
- Racing toward a climax
- Dance music builds

### Multi-Section Compositions

Different sections with different tempos:

```rust
comp.track("multi_section")
    // Introduction - Moderate
    .tempo(90.0)
    .notes(&[C2, C2, G2, G2], 0.5)
    .wait(0.5)
    // Verse - Upbeat
    .tempo(120.0)
    .notes(&[C2, G2, A2, F2], 0.25)
    .notes(&[C2, G2, A2, F2], 0.25)
    .wait(0.5)
    // Chorus - Driving
    .tempo(140.0)
    .notes(&[C2, C2, E2, E2], 0.2)
    .notes(&[G2, G2, C3, C3], 0.2)
    .wait(0.5)
    // Outro - Slow down
    .tempo(100.0)
    .notes(&[A1, F2, C2], 0.75)
    .tempo(80.0)
    .note(&[C2], 2.0);
```

---

## Rhythm Notation

Tunes provides a string-based rhythm notation inspired by live coding languages (TidalCycles, Strudel):

### Basic Syntax

```rust
comp.track("drums")
    .rhythm("x--- x--- x--- x---", DrumType::Kick, 0.125);
    //      ^    ^    ^    ^
    //      Hits on beats 1, 5, 9, 13
```

**Characters:**
- **Hit characters:** `x`, `X`, `1`, `*` (all equivalent)
- **Rest characters:** `-`, `_`, `.`, `~`, `0`, ` ` (space)

### Common Drum Patterns

**Four-on-the-floor kick:**
```rust
comp.track("kick")
    .rhythm("x--- x--- x--- x---", DrumType::Kick, 0.125);
```

**Backbeat snare:**
```rust
comp.track("snare")
    .rhythm("---- x--- ---- x---", DrumType::Snare, 0.125);
```

**Eighth-note hi-hats:**
```rust
comp.track("hats")
    .rhythm("x-x- x-x- x-x- x-x-", DrumType::HiHatClosed, 0.125);
```

### Layering Patterns

Chain multiple `.rhythm()` calls for complete drum kits:

```rust
comp.track("drums")
    .rhythm("x--- x--- x--- x---", DrumType::Kick, 0.125)       // Kick
    .rhythm("---- x--- ---- x-x-", DrumType::Snare, 0.125)      // Snare
    .rhythm("xxxx xxxx xxxx xxxx", DrumType::HiHatClosed, 0.0625) // Hi-hats
    .rhythm("---x ---x ---x ---x", DrumType::Clap, 0.125);      // Claps
```

### Notation Styles

All of these are valid and equivalent:

```rust
// Using x and -
.rhythm("x-x- x-x-", DrumType::Kick, 0.125);

// Using . for rests
.rhythm("x.x. x.x.", DrumType::Kick, 0.125);

// Using _ for rests
.rhythm("x_x_ x_x_", DrumType::Kick, 0.125);

// Using numeric notation
.rhythm("1010 1010", DrumType::Kick, 0.125);

// Mixed (all work!)
.rhythm("x-*- 1-x-", DrumType::Kick, 0.125);
```

### Classic Rhythm Patterns

**Tresillo (Cuban pattern):**
```rust
comp.track("tresillo")
    .rhythm("x--x--x-", DrumType::Tom, 0.125);
```

**Son Clave (3-2):**
```rust
comp.track("clave")
    .rhythm("x--x--x---x-x---", DrumType::Rimshot, 0.125);
```

**Techno 4/4:**
```rust
comp.track("techno")
    .rhythm("x-x-x-x-x-x-x-x-", DrumType::Kick808, 0.125)
    .rhythm("----x-------x---", DrumType::Snare, 0.125)
    .rhythm("x.x.x.x.x.x.x.x.", DrumType::HiHatClosed, 0.0625)
    .rhythm("-------x-------x", DrumType::HiHatOpen, 0.125);
```

**Breakbeat:**
```rust
comp.track("breakbeat")
    .rhythm("x-------x-------", DrumType::Kick, 0.0625)
    .rhythm("----x-------x---", DrumType::Snare, 0.0625)
    .rhythm("x-x-x-x-x-x-x-x-", DrumType::HiHatClosed, 0.0625);
```

### Polyrhythms

Create complex polyrhythmic patterns:

```rust
// 3 against 4
comp.track("poly_kick")
    .rhythm("x---x---x---", DrumType::Kick, 0.125);

comp.track("poly_snare")
    .rhythm("x--x--x--x--", DrumType::Snare, 0.125);
```

---

## Timing Calculations

### BPM to Seconds

```rust
let tempo = Tempo::new(120.0);

// Quarter note = 60 / BPM
let quarter_note = 60.0 / 120.0; // 0.5 seconds

// Eighth note = 30 / BPM
let eighth_note = 30.0 / 120.0;  // 0.25 seconds

// Whole note = 240 / BPM
let whole_note = 240.0 / 120.0;  // 2.0 seconds
```

**Formula reference:**
```
seconds_per_beat = 60.0 / BPM
duration_in_seconds = beats * seconds_per_beat
```

### Common BPM Conversions

| BPM | Quarter Note | Eighth Note | Sixteenth Note |
|-----|--------------|-------------|----------------|
| 60  | 1.0s         | 0.5s        | 0.25s          |
| 80  | 0.75s        | 0.375s      | 0.1875s        |
| 120 | 0.5s         | 0.25s       | 0.125s         |
| 140 | 0.429s       | 0.214s      | 0.107s         |
| 180 | 0.333s       | 0.167s      | 0.083s         |

### Syncing Effects to Tempo

Use tempo calculations for rhythmic delays and modulation:

```rust
let tempo = Tempo::new(120.0);

// Delay synced to quarter notes
let delay_time = tempo.quarter_note();
comp.track("synced")
    .notes(&[C4, E4, G4], 0.5)
    .delay(Delay::new(delay_time, 0.4, 0.5));

// Delay synced to dotted eighth
let dotted_eighth = tempo.duration_to_seconds(NoteDuration::DottedEighth);
comp.track("dotted")
    .notes(&[C4, E4, G4], 0.5)
    .delay(Delay::new(dotted_eighth, 0.4, 0.5));
```

**Common delay timings:**
- Quarter note delay: Creates rhythmic echoes on the beat
- Dotted eighth: Classic "U2 delay" sound
- Triplet delay: Shuffle groove echoes

---

## Complete Examples

### Example 1: Tempo-Synced Composition

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let tempo = Tempo::new(120.0);
    let mut comp = Composition::new(tempo);

    // Calculate tempo-synced timing
    let quarter = tempo.quarter_note();
    let eighth = tempo.eighth_note();

    // Melody using calculated durations
    comp.track("melody")
        .notes(&[C4, D4, E4, F4], quarter)
        .notes(&[G4, A4, B4, C5], eighth);

    // Tempo-synced delay
    comp.track("lead")
        .notes(&[E5, G5, C6], 0.5)
        .delay(Delay::new(quarter, 0.4, 0.5));

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

### Example 2: Dynamic Tempo Changes

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(100.0));

    comp.track("dynamic")
        // Section 1: Moderate
        .notes(&[C4, E4, G4], 0.5)
        .wait(0.5)
        // Section 2: Speed up
        .tempo(140.0)
        .notes(&[D4, F4, A4], 0.3)
        .wait(0.5)
        // Section 3: Slow down for ending
        .tempo(80.0)
        .notes(&[C4, E4, G4], 0.75)
        .note(&[C4], 2.0);

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

### Example 3: Rhythm Notation Drums

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(128.0));

    // Complete drum pattern using rhythm notation
    comp.track("drums")
        .rhythm("x-x-x-x-x-x-x-x-", DrumType::Kick808, 0.125)
        .rhythm("----x-------x---", DrumType::Snare, 0.125)
        .rhythm("x.x.x.x.x.x.x.x.", DrumType::HiHatClosed, 0.0625)
        .rhythm("-------x-------x", DrumType::HiHatOpen, 0.125)
        .rhythm("x---------------", DrumType::Clap, 0.125);

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

---

## Tips and Best Practices

### 1. Choose Appropriate BPM

Match your tempo to the genre:
- **Ambient/Drone:** 60-80 BPM
- **Hip-Hop:** 80-100 BPM
- **House/Techno:** 120-130 BPM
- **Drum & Bass:** 160-180 BPM
- **Hardcore/Gabber:** 180-200+ BPM

### 2. Use Note Durations for Clarity

```rust
// ✅ GOOD: Clear musical intent
let tempo = Tempo::new(120.0);
let quarter = tempo.duration_to_seconds(NoteDuration::Quarter);
comp.track("melody").notes(&[C4, E4, G4], quarter);

// ❌ BAD: Magic number
comp.track("melody").notes(&[C4, E4, G4], 0.5);
```

### 3. Sync Effects to Tempo

Create cohesive, rhythmic effects:

```rust
let tempo = Tempo::new(120.0);

// Delay synced to tempo
let delay_time = tempo.quarter_note();
comp.track("synced")
    .notes(&[C4, E4, G4], 0.5)
    .delay(Delay::new(delay_time, 0.4, 0.5));

// Tremolo synced to tempo (4 Hz at 120 BPM)
let tremolo_rate = tempo.bpm / 60.0 / 2.0; // Quarter note rate
comp.track("tremolo")
    .notes(&[C4, E4, G4], 2.0)
    .tremolo(Tremolo::new(tremolo_rate, 0.6));
```

### 4. Use Rhythm Notation for Quick Prototyping

```rust
// Fast iteration with rhythm strings
comp.track("drums")
    .rhythm("x-x- x-x-", DrumType::Kick, 0.125)
    .rhythm("--x- --x-", DrumType::Snare, 0.125);

// Can easily experiment:
// "x-x- x-x-" → "x--- x---" (four-on-floor)
// "x-x- x-x-" → "x-x- x-xx" (add syncopation)
```

### 5. Gradual Tempo Changes for Expression

Create natural-sounding endings:

```rust
comp.track("outro")
    .tempo(120.0)
    .notes(&[C4, E4, G4], 0.5)
    .tempo(110.0)  // Start slowing
    .notes(&[F4, A4, C5], 0.5)
    .tempo(90.0)   // Slower
    .notes(&[E4, G4, B4], 0.75)
    .tempo(70.0)   // Almost stopping
    .note(&[C4], 2.0); // Final chord
```

---

## Tempo and MIDI Export

**Important:** Tempo changes are preserved in MIDI export:

```rust
let mixer = comp.into_mixer();
mixer.export_midi("song.mid", Tempo::new(120.0))?;
```

- MIDI files include tempo change events
- DAWs will respect tempo changes when importing
- Useful for creating expressive MIDI compositions

**Note:** Tempo changes affect MIDI timing but not audio rendering directly (audio durations are always in real time).

---

## Reference

### Tempo Methods

```rust
Tempo::new(bpm: f32) -> Self
.duration_to_seconds(duration: NoteDuration) -> f32
.quarter_note() -> f32
.eighth_note() -> f32
.sixteenth_note() -> f32
.whole_note() -> f32
```

### NoteDuration Variants

```rust
NoteDuration::Whole           // 4 beats
NoteDuration::Half            // 2 beats
NoteDuration::Quarter         // 1 beat
NoteDuration::Eighth          // 0.5 beats
NoteDuration::Sixteenth       // 0.25 beats
NoteDuration::ThirtySecond    // 0.125 beats
NoteDuration::DottedHalf      // 3 beats
NoteDuration::DottedQuarter   // 1.5 beats
NoteDuration::DottedEighth    // 0.75 beats
NoteDuration::QuarterTriplet  // 2/3 beat
NoteDuration::EighthTriplet   // 1/3 beat
```

### TrackBuilder Methods

```rust
.tempo(bpm: f32) -> Self
.rhythm(pattern: &str, drum_type: DrumType, duration: f32) -> Self
```

---

**Next:** Explore [Mixer Layer](./mixer.md) to understand how tracks are combined and processed →
