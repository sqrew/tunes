# Tempo and Rhythm

Tempo is the heartbeat of music - it defines how musical time (beats, measures) translates to real time (seconds). Tunes provides powerful tempo management and a step sequencer-style drum grid for programming drum patterns.

## Overview

The tempo system handles:
- **BPM (Beats Per Minute)** - The speed of the music
- **Note durations** - Musical time values (whole, half, quarter notes, etc.)
- **Tempo changes** - Ritardando, accelerando, and multi-section tempos
- **Drum Grid** - Step sequencer-style drum pattern programming

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

// Using convenience methods
let quarter = tempo.quarter_note();    // 0.5s at 120 BPM
let eighth = tempo.eighth_note();      // 0.25s
let sixteenth = tempo.sixteenth_note(); // 0.125s
```

---

## Drum Grid

The `drum_grid()` method provides a step sequencer-style interface for programming drum patterns:

```rust
comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, "x---x---x---x---")
        .sound(DrumType::Snare, "----x-------x---")
        .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-"));
```

### Parameters

- **steps** - Number of steps in the grid (e.g., 16 for a bar of 16th notes)
- **step_duration** - Duration of each step in seconds (e.g., 0.125s for 16th notes at 120 BPM)
- **closure** - Builder function that returns the configured grid

### Pattern Syntax

Patterns use a simple character-based syntax:

- **Hit characters:** `x`, `X`, `1`, `*` (all trigger a hit)
- **Rest characters:** `-`, `_`, `.`, `~`, `0`, space (all are rests)

```rust
// These are all equivalent
.sound(DrumType::Kick, "x---x---")
.sound(DrumType::Kick, "x...x...")
.sound(DrumType::Kick, "1___1___")
.sound(DrumType::Kick, &[0, 4])  // Array syntax also works
```

### Common Patterns

**Four-on-the-floor:**
```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::Kick, "x---x---x---x---"))
```

**Backbeat:**
```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::Snare, "----x-------x---"))
```

**Eighth-note hi-hats:**
```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-"))
```

---

## Velocity and Dynamics

### Accent Patterns

Apply velocity patterns to create dynamic feel:

```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-")
    .accent("x---x---x---x---"))  // Accent downbeats
```

Accented steps get velocity 1.0, others get 0.5. Use `accent_with_levels()` for custom values:

```rust
.accent_with_levels("x---x---", 1.0, 0.4)  // Custom high/low
```

### Per-Step Velocity

Set explicit velocity for each step:

```rust
.drum_grid(8, 0.125, |g| g
    .sound(DrumType::HiHatClosed, "xxxxxxxx")
    .velocity(&[1.0, 0.5, 0.7, 0.5, 1.0, 0.5, 0.7, 0.5]))
```

### Crescendo / Decrescendo

Ramp velocity across the pattern:

```rust
// Works on any pattern (drums or notes)
comp.track("drums")
    .drum_grid(8, 0.125, |g| g
        .sound(DrumType::Snare, "x-x-x-x-"))
    .crescendo(0.3, 1.0);   // Build up

comp.track("drums")
    .drum_grid(8, 0.125, |g| g
        .sound(DrumType::Snare, "x-x-x-x-"))
    .decrescendo(1.0, 0.3); // Fade down
```

---

## Ghost Notes and Rudiments

### Ghost Notes

Add quiet hits for texture:

```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::Snare, "----x-------x---")      // Main hits
    .ghost(DrumType::Snare, "-x----x--x----x-", 0.3)) // Ghost notes at 30% velocity
```

### Flams

Grace note before the main hit:

```rust
.drum_grid(16, 0.125, |g| g
    .flam(DrumType::Snare, "----x-------x---", 0.03, 0.4))
    //                     pattern          offset  grace_velocity
```

### Rolls

Subdivide a step into rapid hits:

```rust
.drum_grid(16, 0.125, |g| g
    .roll(DrumType::Snare, "---------------x", 8))  // 8 hits in final step
```

---

## Probabilistic Patterns

### Maybe

Hits occur with a given probability:

```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::Kick, "x---x---x---x---")
    .maybe(DrumType::HiHatOpen, "x-x-x-x-x-x-x-x-", 0.3))  // 30% chance per step
```

---

## Pattern Repetition

Repeat the drum pattern:

```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::Kick, "x---x---x---x---")
    .sound(DrumType::Snare, "----x-------x---")
    .repeat(3))  // Play 4 times total (original + 3 repeats)
```

---

## Complete Drum Examples

### Classic Rock Beat

```rust
comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, "x---x---x---x---")
        .sound(DrumType::Snare, "----x-------x---")
        .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-")
        .accent("x---x---x---x---"));
```

### Funk Beat with Ghosts

```rust
comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, "x--x--x---x-x---")
        .sound(DrumType::Snare, "----x-------x---")
        .ghost(DrumType::Snare, "-xx---x--x----x-", 0.25)
        .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-")
        .accent("x---x---x---x---"));
```

### Electronic with Probability

```rust
comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick808, "x---x---x---x-x-")
        .sound(DrumType::Snare, "----x-------x---")
        .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-")
        .maybe(DrumType::HiHatOpen, "----x-------x---", 0.4)
        .ghost(DrumType::Rimshot, "x-x-x-x-x-x-x-x-", 0.2))
    .humanize(0.01, 0.05);  // Subtle humanization
```

### Snare Roll Buildup

```rust
comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, "x---------------")
        .roll(DrumType::Snare, "xxxxxxxxxxxxxxxx", 2))  // Continuous roll
    .crescendo(0.2, 1.0);  // Build intensity
```

---

## Tempo Changes

### Sudden Changes

```rust
comp.track("melody")
    .notes(&[C4, D4, E4, F4], 0.25)
    .tempo(80.0)  // Suddenly slow
    .notes(&[C5, B4, A4, G4], 0.5);
```

### Ritardando (Gradual Slowdown)

```rust
comp.track("outro")
    .tempo(120.0)
    .notes(&[C4, E4, G4], 0.5)
    .tempo(100.0)
    .notes(&[F4, A4, C5], 0.6)
    .tempo(80.0)
    .note(&[C4], 2.0);
```

### Accelerando (Gradual Speedup)

```rust
comp.track("buildup")
    .tempo(80.0)
    .notes(&[G4, G4], 0.5)
    .tempo(100.0)
    .notes(&[A4, A4], 0.4)
    .tempo(120.0)
    .notes(&[B4, B4], 0.3)
    .tempo(140.0)
    .notes(&[C5, C5], 0.25);
```

---

## Transforms on Drums

All pattern transforms work on drums:

```rust
comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-"))
    .humanize(0.01, 0.05)      // Timing + velocity variation
    .crescendo(0.5, 1.0);      // Velocity ramp
```

Since drums now have `pitch_offset` and `velocity` fields, pitch transforms also work:

```rust
comp.track("toms")
    .drum_grid(8, 0.125, |g| g
        .sound(DrumType::Tom, "x-x-x-x-"))
    .transform(|t| t.shift(5));  // Shift pitch up 5 semitones
```

---

## Timing Reference

### BPM to Step Duration

At 120 BPM:
- Quarter note: `60.0 / 120.0 = 0.5s`
- Eighth note: `0.25s`
- Sixteenth note: `0.125s` (common for 16-step patterns)

```rust
let tempo = Tempo::new(120.0);
let step_duration = tempo.sixteenth_note();  // 0.125s

comp.track("drums")
    .drum_grid(16, step_duration, |g| g
        .sound(DrumType::Kick, "x---x---x---x---"));
```

### Common BPM Conversions

| BPM | 16th Note Step Duration |
|-----|------------------------|
| 60  | 0.25s                  |
| 80  | 0.1875s                |
| 120 | 0.125s                 |
| 140 | 0.107s                 |
| 180 | 0.083s                 |

---

## Reference

### DrumGrid Methods

```rust
.sound(drum_type, pattern)           // Add hits
.ghost(drum_type, pattern, velocity) // Quiet ghost notes
.maybe(drum_type, pattern, prob)     // Probabilistic hits
.flam(drum_type, pattern, offset, vel) // Grace note + main
.roll(drum_type, pattern, subdivisions) // Rapid subdivision
.accent(pattern)                     // High/low velocity
.accent_with_levels(pattern, high, low)
.velocity(&[...])                    // Per-step velocity
.repeat(n)                           // Repeat pattern
```

### Tempo Methods

```rust
Tempo::new(bpm)
.duration_to_seconds(NoteDuration)
.quarter_note() / .eighth_note() / .sixteenth_note()
```

### Velocity Transforms

```rust
.crescendo(start_vel, end_vel)
.decrescendo(start_vel, end_vel)
.velocity_ramp(start_vel, end_vel)
```

---

**Next:** Explore [Pattern Transformations](../advanced/transformations.md) for more ways to manipulate patterns.
