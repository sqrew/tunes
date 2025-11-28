# Drum Programming

Tunes provides a powerful drum programming system with 90+ synthesized drum sounds, a step sequencer-style API, and full transform support. No samples needed - every drum sound is generated in real-time.

## Table of Contents

- [Overview](#overview)
- [DrumGrid Basics](#drumgrid-basics)
- [Pattern Syntax](#pattern-syntax)
  - [String Patterns](#string-patterns)
  - [Array Patterns](#array-patterns)
- [DrumGrid Methods](#drumgrid-methods)
  - [sound - Basic Hits](#sound---basic-hits)
  - [ghost - Quiet Texture Notes](#ghost---quiet-texture-notes)
  - [flam - Grace Notes](#flam---grace-notes)
  - [drag - Trailing Grace Notes](#drag---trailing-grace-notes)
  - [ruff - Double Grace Notes](#ruff---double-grace-notes)
  - [diddle - Quick Double Stroke](#diddle---quick-double-stroke)
  - [buzz - Decaying Roll](#buzz---decaying-roll)
  - [double_flam - Two Trailing Grace Notes](#double_flam---two-trailing-grace-notes)
  - [roll - Rapid Subdivisions](#roll---rapid-subdivisions)
  - [maybe - Probabilistic Hits](#maybe---probabilistic-hits)
  - [accent - Velocity Patterns](#accent---velocity-patterns)
  - [velocity - Per-Step Control](#velocity---per-step-control)
  - [repeat - Pattern Repetition](#repeat---pattern-repetition)
- [Transforms on Drums](#transforms-on-drums)
  - [Timing Transforms](#timing-transforms)
  - [Velocity Transforms](#velocity-transforms)
  - [Pitch Transforms](#pitch-transforms)
  - [Event Iteration](#event-iteration)
- [Algorithmic Drum Patterns](#algorithmic-drum-patterns)
- [DrumType Reference](#drumtype-reference)

---

## Overview

The drum system in Tunes consists of:

- **DrumType** - 90+ synthesized drum sounds (kicks, snares, hi-hats, percussion, etc.)
- **DrumGrid** - Step sequencer-style pattern builder
- **DrumEvent** - The underlying event type with velocity and pitch_offset fields
- **Transforms** - All pattern transforms work on drums

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, "x---x---x---x---")
        .sound(DrumType::Snare, "----x-------x---")
        .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-"))
    .humanize(0.01, 0.05);
```

---

## DrumGrid Basics

The `drum_grid()` method creates a step sequencer-style grid:

```rust
.drum_grid(steps, step_duration, |g| g
    // ... add sounds here
)
```

**Parameters:**
- `steps` - Number of steps in the grid (e.g., 16 for a bar of 16th notes)
- `step_duration` - Duration of each step in seconds
- `closure` - Builder function to configure the grid

**Calculating step_duration:**

At 120 BPM, a quarter note is 0.5 seconds:
- 16th note: `0.5 / 4 = 0.125s`
- 8th note: `0.5 / 2 = 0.25s`

Or use the Tempo helper:
```rust
let tempo = Tempo::new(120.0);
let step = tempo.sixteenth_note();  // 0.125s
```

---

## Pattern Syntax

### String Patterns

Strings provide a visual representation of your beat:

```rust
.sound(DrumType::Kick, "x---x---x---x---")
```

**Hit characters:** `x`, `X`, `1`, `*` - all trigger a hit

**Rest characters:** `-`, `_`, `.`, `~`, `0`, space - all are rests

These are equivalent:
```rust
.sound(DrumType::Kick, "x---x---")
.sound(DrumType::Kick, "x...x...")
.sound(DrumType::Kick, "1___1___")
.sound(DrumType::Kick, "X   X   ")
```

### Array Patterns

Arrays specify step indices (0-indexed):

```rust
.sound(DrumType::Kick, &[0, 4, 8, 12])  // Same as "x---x---x---x---"
```

Arrays enable algorithmic patterns:
```rust
let kicks = euclidean_rhythm(4, 16);  // Generate pattern algorithmically
.sound(DrumType::Kick, &kicks)
```

---

## DrumGrid Methods

### sound - Basic Hits

Add drum hits at specified steps:

```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::Kick, "x---x---x---x---")
    .sound(DrumType::Snare, "----x-------x---")
    .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-"))
```

### ghost - Quiet Texture Notes

Add quiet hits for texture and groove:

```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::Snare, "----x-------x---")      // Main hits
    .ghost(DrumType::Snare, "-x----x--x----x-", 0.3)) // 30% velocity
```

**Parameters:**
- `drum_type` - Which drum sound
- `pattern` - Where to place ghost notes
- `velocity` - Volume (0.0-1.0, typically 0.2-0.4)

### flam - Grace Notes

Add a grace note before the main hit (drum rudiment):

```rust
.drum_grid(16, 0.125, |g| g
    .flam(DrumType::Snare, "----x-------x---", 0.03, 0.4))
```

**Parameters:**
- `drum_type` - Which drum sound
- `pattern` - Where to place flams
- `grace_offset` - Time before main hit (seconds, e.g., 0.03)
- `grace_velocity` - Grace note volume (0.0-1.0)

### drag - Trailing Grace Notes

Add a grace note after the main hit (opposite of flam):

```rust
.drum_grid(16, 0.125, |g| g
    .drag(DrumType::Snare, "----x-------x---", 0.03, 0.4))
```

**Parameters:**
- `drum_type` - Which drum sound
- `pattern` - Where to place drags
- `drag_offset` - Time after main hit (seconds, e.g., 0.03)
- `drag_velocity` - Trailing note volume (0.0-1.0)

### ruff - Double Grace Notes

Add two grace notes before the main hit (extends flam):

```rust
.drum_grid(16, 0.125, |g| g
    .ruff(DrumType::Snare, "----x-------x---", 0.05, 0.025, 0.35))
```

**Parameters:**
- `drum_type` - Which drum sound
- `pattern` - Where to place ruffs
- `first_offset` - Time before main hit for first grace note (e.g., 0.05)
- `second_offset` - Time before main hit for second grace note (e.g., 0.025)
- `grace_velocity` - Volume of both grace notes (0.0-1.0)

### diddle - Quick Double Stroke

Two hits in quick succession at equal velocity:

```rust
.drum_grid(16, 0.125, |g| g
    .diddle(DrumType::Snare, "----x-------x---", 0.03))
```

**Parameters:**
- `drum_type` - Which drum sound
- `pattern` - Where to place diddles
- `spacing` - Time between the two hits (seconds, e.g., 0.03)

### buzz - Decaying Roll

Rapid hits with decaying velocity (simulates stick bounce):

```rust
.drum_grid(16, 0.125, |g| g
    .buzz(DrumType::Snare, "---------------x", 6, 0.7))
```

**Parameters:**
- `drum_type` - Which drum sound
- `pattern` - Where to place buzz rolls
- `hits` - Number of hits in the buzz
- `decay` - Velocity multiplier per hit (e.g., 0.7 = each hit 70% of previous)

### double_flam - Two Trailing Grace Notes

Two grace notes after the main hit (opposite of ruff):

```rust
.drum_grid(16, 0.125, |g| g
    .double_flam(DrumType::Snare, "----x-------x---", 0.025, 0.05, 0.35))
```

**Parameters:**
- `drum_type` - Which drum sound
- `pattern` - Where to place double flams
- `first_offset` - Time after main hit for first grace note (e.g., 0.025)
- `second_offset` - Time after main hit for second grace note (e.g., 0.05)
- `grace_velocity` - Volume of both grace notes (0.0-1.0)

### roll - Rapid Subdivisions

Subdivide a step into rapid hits:

```rust
.drum_grid(16, 0.125, |g| g
    .roll(DrumType::Snare, "---------------x", 8))  // 8-hit roll on last step
```

**Parameters:**
- `drum_type` - Which drum sound
- `pattern` - Which steps get rolls
- `subdivisions` - Number of hits per step

### maybe - Probabilistic Hits

Hits occur with a given probability:

```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::Kick, "x---x---x---x---")
    .maybe(DrumType::HiHatOpen, "x-x-x-x-x-x-x-x-", 0.3))  // 30% chance
```

### accent - Velocity Patterns

Apply high/low velocity based on a pattern:

```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-")
    .accent("x---x---x---x---"))  // Accent downbeats
```

Default: accented = 1.0, non-accented = 0.5

Custom levels:
```rust
.accent_with_levels("x---x---", 1.0, 0.4)  // Custom high/low
```

### velocity - Per-Step Control

Set explicit velocity for each step:

```rust
.drum_grid(8, 0.125, |g| g
    .sound(DrumType::HiHatClosed, "xxxxxxxx")
    .velocity(&[1.0, 0.5, 0.7, 0.5, 1.0, 0.5, 0.7, 0.5]))
```

### repeat - Pattern Repetition

Repeat the entire drum pattern:

```rust
.drum_grid(16, 0.125, |g| g
    .sound(DrumType::Kick, "x---x---x---x---")
    .sound(DrumType::Snare, "----x-------x---")
    .repeat(3))  // Play 4 times total (original + 3 repeats)
```

---

## Transforms on Drums

All pattern transforms work on DrumEvents. Drums have `velocity` and `pitch_offset` fields that transforms can modify.

### Timing Transforms

```rust
comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-"))
    .humanize(0.01, 0.05)   // ±10ms timing, ±5% velocity
    .stretch(0.5);          // Double speed
```

### Velocity Transforms

```rust
comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Snare, "xxxxxxxxxxxxxxxx"))
    .crescendo(0.2, 1.0);   // Build from quiet to loud
```

Also: `.decrescendo()`, `.velocity_ramp()`

### Pitch Transforms

Drums have a `pitch_offset` field (in semitones):

```rust
comp.track("toms")
    .drum_grid(8, 0.125, |g| g
        .sound(DrumType::Tom, "x-x-x-x-"))
    .transform(|t| t.shift(5));  // Shift pitch up 5 semitones
```

### Event Iteration

Full control with closures:

```rust
use tunes::composition::generative::EventMut;

comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::HiHatClosed, "x-x-x-x-x-x-x-x-"))
    .every_nth_drum(4, |drum| {
        drum.velocity = 1.0;  // Accent every 4th hit
    });
```

Or with full access:
```rust
.for_each_event(|event, _note_count, drum_count| {
    if let EventMut::Drum(d) = event {
        if drum_count % 4 == 0 {
            d.velocity = 1.0;
            d.pitch_offset = 2.0;  // Raise pitch too
        }
    }
});
```

---

## Algorithmic Drum Patterns

Since `drum_grid` accepts arrays, you can generate patterns algorithmically:

```rust
// Euclidean rhythm: distribute n hits across k steps
fn euclidean_rhythm(hits: usize, steps: usize) -> Vec<usize> {
    let mut pattern = Vec::new();
    let mut bucket = 0.0;
    let increment = hits as f32 / steps as f32;

    for step in 0..steps {
        bucket += increment;
        if bucket >= 1.0 {
            pattern.push(step);
            bucket -= 1.0;
        }
    }
    pattern
}

let kicks = euclidean_rhythm(4, 16);   // [0, 4, 8, 12]
let snares = euclidean_rhythm(3, 16);  // [0, 5, 11]

comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, &kicks)
        .sound(DrumType::Snare, &snares));
```

You can also use sequences, random generators, or any code that produces `Vec<usize>`.

---

## DrumType Reference

All drum sounds are synthesized in real-time - no samples required.

### Kicks
| Type | Description |
|------|-------------|
| `Kick` | Standard kick drum |
| `Kick808` | Long, pitched 808 kick |
| `Kick909` | Punchier electronic kick |
| `SubKick` | Ultra-low sub kick |
| `KickTight` | Short, punchy kick |
| `KickDeep` | Extended low-end |
| `KickAcoustic` | Natural drum kit sound |
| `KickClick` | Prominent beater attack |

### Snares
| Type | Description |
|------|-------------|
| `Snare` | Standard snare |
| `Snare808` | 808 snare (dual triangle oscillators) |
| `Snare909` | Brighter electronic snare |
| `SnareRim` | Rim-focused |
| `SnareTight` | Short, dry |
| `SnareLoose` | Longer ring |
| `SnarePiccolo` | High-pitched, bright |

### Hi-Hats
| Type | Description |
|------|-------------|
| `HiHatClosed` | Standard closed hi-hat |
| `HiHatOpen` | Standard open hi-hat |
| `HiHat808Closed` | 808 closed (6 square oscillators) |
| `HiHat808Open` | 808 open (6 square oscillators) |
| `HiHatPedal` | Pedal hi-hat chick |
| `HiHatHalfOpen` | Between closed and open |
| `HiHatSizzle` | High-frequency content |

### Claps
| Type | Description |
|------|-------------|
| `Clap` | Standard clap |
| `Clap808` | 808 clap (multiple noise bursts) |
| `ClapDry` | No reverb, tight |
| `ClapRoom` | Natural room ambience |
| `ClapGroup` | Multiple claps layered |
| `ClapSnare` | Hybrid clap/snare |

### Toms
| Type | Description |
|------|-------------|
| `Tom` | Mid tom |
| `TomHigh` | High tom |
| `TomLow` | Low tom |
| `FloorTomLow` | Deep floor tom |
| `FloorTomHigh` | Higher floor tom |

### Cymbals
| Type | Description |
|------|-------------|
| `Crash` | Standard crash |
| `Crash2` | Second crash cymbal |
| `CrashShort` | Quick crash, gated |
| `Ride` | Standard ride |
| `RideBell` | Metallic ping |
| `RideTip` | Bell-less ride |
| `China` | China cymbal |
| `Splash` | Splash cymbal |

### Latin Percussion
| Type | Description |
|------|-------------|
| `CongaHigh` | Bright, high-pitched |
| `CongaLow` | Deep, resonant |
| `BongoHigh` | Sharp, articulate |
| `BongoLow` | Deeper bongo |
| `TimbaleHigh` | High timbale |
| `TimbaleLow` | Low timbale |
| `AgogoHigh` | High agogo bell |
| `AgogoLow` | Low agogo bell |
| `Cabasa` | Textured shaker/scraper |
| `GuiroShort` | Short scraping |
| `GuiroLong` | Long scraping |

### World Percussion
| Type | Description |
|------|-------------|
| `Djembe` | West African hand drum |
| `TablaBayan` | Indian bass drum (left hand) |
| `TablaDayan` | Indian treble drum (right hand) |
| `Cajon` | Box drum |

### Orchestral
| Type | Description |
|------|-------------|
| `Timpani` | Tuned orchestral bass drum |
| `Gong` | Deep metallic crash |
| `Chimes` | Tubular bells |

### Hand Percussion
| Type | Description |
|------|-------------|
| `Tambourine` | Standard tambourine |
| `Shaker` | Standard shaker |
| `Maracas` | Rattling shaker |
| `Fingersnap` | Fingersnap sound |
| `Castanet` | Spanish wooden clapper |
| `SleighBells` | Jingle bells |

### Wood/Metal
| Type | Description |
|------|-------------|
| `Claves` | Sharp wooden click |
| `WoodBlock` | Dry, pitched click |
| `WoodBlockHigh` | High-pitched wooden click |
| `Triangle` | Metallic ding |
| `Cowbell` | Standard cowbell |
| `SideStick` | Soft rim click |
| `Rimshot` | Standard rimshot |

### Electronic/Effects
| Type | Description |
|------|-------------|
| `BassDrop` | Dramatic bass drop impact |
| `Boom` | Deep cinematic boom |
| `LaserZap` | Sci-fi laser sound |
| `ReverseCymbal` | Reverse crash buildup |
| `WhiteNoiseHit` | Noise burst/clap |
| `StickClick` | Drumstick click |
| `Vibraslap` | Rattling/buzzing percussion |

---

**Next:** Learn about [Pattern Transformations](./transformations.md) for more ways to manipulate drum patterns.
