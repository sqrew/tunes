# Synthesis Basics

Synthesis is the art of creating sound from scratch using mathematical waveforms and signal processing. Unlike sample playback (playing recorded audio), synthesis gives you complete control over every aspect of a sound's timbre, envelope, and character.

## Why Synthesis?

**Advantages over samples:**
- **Small footprint** - No large audio files to ship
- **Infinite variation** - Every note can be slightly different
- **Real-time control** - Change any parameter during playback
- **Perfect pitch** - Play any frequency without artifacts
- **Procedural generation** - Create sounds algorithmically

**When to use synthesis:**
- Musical instruments (bass, leads, pads, plucks)
- Retro/chiptune aesthetics
- Sound effects (explosions, lasers, UI sounds)
- Generative/algorithmic audio
- When you need small binary sizes

---

## The Synthesis Signal Chain

Sound synthesis follows a signal path from generation to output:

```
1. Oscillator (Waveform)
   ↓
2. Envelope (ADSR)
   ↓
3. Filter (Subtractive)
   ↓
4. Effects (Reverb, Delay, etc.)
   ↓
Output
```

Let's explore each stage.

---

## 1. Oscillators & Waveforms

The oscillator generates the raw sound wave. Different waveforms have different harmonic content, which determines the timbre.

### Sine Wave

The purest waveform - a single frequency with no harmonics.

```rust
comp.track("sine")
    .waveform(Waveform::Sine)
    .note(&[440.0], 1.0);
```

**Sound:** Pure, smooth, mellow
**Use for:** Sub bass, flutes, pure tones, test signals
**Harmonics:** Fundamental only

### Square Wave

A hollow, clarinet-like sound with only odd harmonics.

```rust
comp.track("square")
    .waveform(Waveform::Square)
    .note(&[440.0], 1.0);
```

**Sound:** Hollow, nasal, video-game like
**Use for:** Chiptune leads, retro bass, clarinets, organ-like sounds
**Harmonics:** Odd harmonics (1st, 3rd, 5th, 7th...)

### Sawtooth Wave

The richest waveform - contains all harmonics.

```rust
comp.track("saw")
    .waveform(Waveform::Sawtooth)
    .note(&[440.0], 1.0);
```

**Sound:** Bright, buzzy, cutting, aggressive
**Use for:** Synth leads, strings, brass, aggressive bass
**Harmonics:** All harmonics (1st, 2nd, 3rd, 4th...)

### Triangle Wave

Softer than sawtooth, containing only odd harmonics but weaker.

```rust
comp.track("triangle")
    .waveform(Waveform::Triangle)
    .note(&[440.0], 1.0);
```

**Sound:** Soft, mellow, flute-like
**Use for:** Soft leads, flutes, ocarina-like sounds
**Harmonics:** Odd harmonics (weaker than square)

### Waveform Comparison Example

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Play each waveform for comparison
    comp.track("sine")
        .waveform(Waveform::Sine)
        .note(&[C4], 1.0);

    comp.track("square")
        .waveform(Waveform::Square)
        .at(1.0)
        .note(&[C4], 1.0);

    comp.track("saw")
        .waveform(Waveform::Sawtooth)
        .at(2.0)
        .note(&[C4], 1.0);

    comp.track("triangle")
        .waveform(Waveform::Triangle)
        .at(3.0)
        .note(&[C4], 1.0);

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

---

## 2. Envelopes (ADSR)

An envelope controls how a sound evolves over time. The classic **ADSR** envelope has four stages:

```
Volume
  ↑
  |     /\___________
  |    /  \          \
  |   /    \          \
  |  /      \          \___
  | /        \
  |/          \
  +--+--+-----+------------→ Time
     A  D  S        R

A = Attack   - Time to reach full volume
D = Decay    - Time to drop to sustain level
S = Sustain  - Held volume level (0.0-1.0)
R = Release  - Time to fade after note ends
```

### Basic Envelope

```rust
use tunes::synthesis::envelope::Envelope;

comp.track("synth")
    .waveform(Waveform::Sawtooth)
    .envelope(Envelope::new(
        0.1,   // Attack: 0.1s
        0.2,   // Decay: 0.2s
        0.7,   // Sustain: 70%
        0.3    // Release: 0.3s
    ))
    .note(&[C4], 1.0);
```

### Envelope Presets

Tunes provides common envelope shapes:

```rust
// Pluck - Fast attack, no sustain (guitar, piano)
comp.track("pluck")
    .waveform(Waveform::Triangle)
    .envelope(Envelope::pluck())
    .notes(&[C4, E4, G4], 0.25);

// Pad - Slow attack, long release (strings, pads)
comp.track("pad")
    .waveform(Waveform::Sawtooth)
    .envelope(Envelope::pad())
    .notes(&[C3, E3, G3], 4.0);

// Organ - Instant on/off (organ, accordion)
comp.track("organ")
    .waveform(Waveform::Sine)
    .envelope(Envelope::organ())
    .notes(&[C4, E4, G4], 0.5);
```

### Envelope Use Cases

**Fast Attack (0.001-0.05s):**
- Plucked instruments (guitar, harpsichord)
- Percussive sounds
- Stabs and hits

**Slow Attack (0.1-2.0s):**
- Pads and strings
- Ambient textures
- Swells and risers

**High Sustain (0.7-1.0):**
- Held notes (organ, sustained strings)
- Bass lines
- Pads

**Low/Zero Sustain (0.0-0.3):**
- Plucked sounds
- Percussive hits
- Decaying tones

---

## 3. Filters (Subtractive Synthesis)

Filters remove frequencies from the oscillator output, sculpting the timbre. This is called **subtractive synthesis**.

### Low-Pass Filter

Allows frequencies **below** the cutoff to pass through, removing highs.

```rust
comp.track("bass")
    .waveform(Waveform::Sawtooth)
    .filter(Filter::low_pass(300.0, 0.5))  // Cutoff: 300Hz, Resonance: 0.5
    .notes(&[C2, G2], 0.5);
```

**Sound:** Dark, muffled, bass-heavy
**Use for:** Bass sounds, warm pads, removing harshness

### High-Pass Filter

Allows frequencies **above** the cutoff to pass through, removing lows.

```rust
comp.track("bright")
    .waveform(Waveform::Sawtooth)
    .filter(Filter::high_pass(2000.0, 0.5))  // Cutoff: 2000Hz
    .notes(&[C4, E4], 0.5);
```

**Sound:** Thin, bright, airy
**Use for:** Hi-hats, air, removing rumble

### Band-Pass Filter

Allows frequencies in a **band** around the cutoff to pass through.

```rust
comp.track("telephone")
    .waveform(Waveform::Sawtooth)
    .filter(Filter::band_pass(1000.0, 1.5))  // Center: 1000Hz
    .notes(&[C4], 1.0);
```

**Sound:** Nasal, telephone-like, focused
**Use for:** Telephone effects, focused leads, vowel sounds

### Resonance

The second parameter is **resonance** (0.0-2.0+), which emphasizes frequencies near the cutoff.

```rust
// Low resonance - smooth rolloff
comp.track("smooth")
    .waveform(Waveform::Sawtooth)
    .filter(Filter::low_pass(800.0, 0.3))
    .note(&[C3], 1.0);

// High resonance - emphasized cutoff, "acid" sound
comp.track("acid")
    .waveform(Waveform::Sawtooth)
    .filter(Filter::low_pass(800.0, 2.0))
    .note(&[C3], 1.0);
```

**High resonance (1.5-3.0+):**
- Creates a peak at the cutoff frequency
- Classic "acid" bass sound (TB-303)
- Can self-oscillate at very high values
- Nasal, vowel-like quality

---

## 4. Putting It All Together

### Complete Synthesis Example

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Plucked bass
    comp.track("bass")
        .waveform(Waveform::Sawtooth)         // Rich harmonics
        .envelope(Envelope::pluck())           // Fast decay
        .filter(Filter::low_pass(400.0, 0.7))  // Remove highs
        .notes(&[C2, C2, G2, F2], 0.5);

    // Pad chords
    comp.track("pad")
        .waveform(Waveform::Sawtooth)         // Bright base
        .envelope(Envelope::pad())             // Slow swell
        .filter(Filter::low_pass(1200.0, 0.5)) // Warm tone
        .reverb(Reverb::new(0.6, 0.5, 0.4))    // Add space
        .notes(&[C4, E4, G4], 2.0);

    // Punchy lead
    comp.track("lead")
        .waveform(Waveform::Square)            // Hollow tone
        .envelope(Envelope::new(0.01, 0.1, 0.5, 0.2))
        .filter(Filter::band_pass(1800.0, 1.2)) // Focused
        .delay(Delay::new(0.375, 0.3, 0.4))     // Rhythm
        .notes(&[C5, D5, E5, G5], 0.25);

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

---

## Common Synthesis Patterns

### Sub Bass

Pure low-end foundation:

```rust
comp.track("sub")
    .waveform(Waveform::Sine)         // Pure fundamental
    .envelope(Envelope::new(0.01, 0.1, 0.8, 0.2))
    .notes(&[C1, C1, G1, F1], 1.0);
```

### Acid Bass

Classic TB-303 style:

```rust
comp.track("acid")
    .waveform(Waveform::Sawtooth)
    .envelope(Envelope::new(0.001, 0.05, 0.0, 0.1))
    .filter(Filter::low_pass(200.0, 2.5))  // High resonance!
    .notes(&[C2, C2, DS2, C2, F2, C2, G2, C2], 0.125);
```

### Synth Lead

Cutting through the mix:

```rust
comp.track("lead")
    .waveform(Waveform::Sawtooth)
    .envelope(Envelope::new(0.01, 0.1, 0.6, 0.2))
    .filter(Filter::low_pass(2400.0, 0.8))
    .notes(&[C4, D4, E4, G4, E4, D4, C4], 0.25);
```

### Warm Pad

Lush background texture:

```rust
comp.track("pad")
    .waveform(Waveform::Sawtooth)
    .envelope(Envelope::new(0.5, 0.3, 0.8, 1.5))  // Slow attack/release
    .filter(Filter::low_pass(1000.0, 0.4))
    .reverb(Reverb::new(0.8, 0.6, 0.5))
    .chorus(Chorus::new(0.4, 0.3, 0.3))  // Add width
    .notes(&[C3, E3, G3, B3], 4.0);
```

### Pluck

Guitar/harp-like attack:

```rust
comp.track("pluck")
    .waveform(Waveform::Triangle)
    .envelope(Envelope::new(0.001, 0.15, 0.0, 0.1))  // Fast decay, no sustain
    .filter(Filter::low_pass(3000.0, 0.3))
    .notes(&[C4, E4, G4, C5, E5], 0.2);
```

---

## Advanced: Custom Wavetables

For more complex timbres, use custom wavetables built from harmonics:

```rust
use tunes::synthesis::wavetable::{Wavetable, DEFAULT_TABLE_SIZE};

// Create organ sound (odd harmonics)
let organ_wt = Wavetable::from_harmonics(
    DEFAULT_TABLE_SIZE,
    &[(1, 1.0), (3, 0.5), (5, 0.3), (7, 0.2)]
);

comp.track("organ")
    .custom_waveform(organ_wt)
    .notes(&[C3, E3, G3], 1.0);
```

Or use the convenient `.additive_synth()` method:

```rust
// Sawtooth-like (all harmonics)
comp.track("bright")
    .additive_synth(&[1.0, 0.5, 0.33, 0.25, 0.2, 0.16])
    .notes(&[C4, E4, G4], 0.5);

// Hollow (odd harmonics only)
comp.track("hollow")
    .additive_synth(&[1.0, 0.0, 0.5, 0.0, 0.3])
    .notes(&[C4, E4, G4], 0.5);
```

---

## Synthesis vs Samples: When to Use Each

| Synthesis | Samples |
|-----------|---------|
| Small memory footprint | Realistic acoustic sounds |
| Perfect at any pitch | Natural timbre and character |
| Real-time parameter control | Quick to implement |
| Infinite variation | No CPU overhead |
| Retro/electronic aesthetic | Professional sound libraries |

**Best practice:** Use synthesis for synths, bass, and leads. Use samples for drums and acoustic instruments. Combine both for maximum flexibility.

---

## Next Steps

Now that you understand synthesis basics, explore:

- **[FM Synthesis](./fm.md)** - Complex timbres from frequency modulation
- **[Granular Synthesis](./granular.md)** - Textures from tiny audio grains
- **[Effects Chain](./effects.md)** - Polish your sounds with effects
- **[Composition Layer](../concepts/composition.md)** - Full API reference

---

**Experiment!** Synthesis is learned by doing. Try different waveforms, envelopes, and filters. Listen to how each parameter changes the sound. There's no "wrong" sound in synthesis—only discovery.
