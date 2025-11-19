# Karplus-Strong Synthesis

Karplus-Strong is a physical modeling technique that simulates plucked string instruments by filtering a burst of noise through a feedback delay line.

## What It Is

The algorithm generates a short noise burst, feeds it into a delay line tuned to the desired pitch, and applies a low-pass filter in the feedback path. This creates a naturally decaying tone with harmonic content similar to plucked strings.

**Process:**
1. Generate initial excitation (noise burst or impulse)
2. Feed signal through delay line (length determines pitch)
3. Apply low-pass filter to feedback (simulates string damping)
4. Sum delayed signal back into the delay line (sustains the tone)

The delay time is set to `sample_rate / frequency`, creating a periodic waveform at the target pitch. The filter progressively removes high frequencies, simulating natural string damping.

## When to Use

- Plucked string instruments (guitar, banjo, harp, sitar)
- Percussive, natural-sounding tones
- Low CPU cost synthesis with realistic character
- Ethnic string instruments
- Game audio (footsteps on wood, impacts)

## Usage

```rust
use tunes::prelude::*;
use tunes::synthesis::karplus_strong::KarplusStrongParams;

let mut comp = Composition::new(Tempo::new(120.0));

// Basic plucked string
comp.track("guitar")
    .karplus_strong(KarplusStrongParams::new(0.995))  // High feedback = longer sustain
    .notes(&[E2, A2, D3, G3, B3, E4], 0.5);

// Bright, metallic pluck
comp.track("bright")
    .karplus_strong(KarplusStrongParams::new(0.998))  // Very high feedback
    .notes(&[C4, E4, G4], 0.75);

// Damped, dead string
comp.track("muted")
    .karplus_strong(KarplusStrongParams::new(0.92))   // Lower feedback = fast decay
    .notes(&[C3, E3, G3], 0.3);

// Harp-like arpeggio
comp.track("harp")
    .karplus_strong(KarplusStrongParams::new(0.996))
    .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25);
```

## Parameter Guidelines

**Feedback coefficient (damping):**
- **0.90-0.93** - Heavy damping, short decay (muted strings, dead strings)
- **0.94-0.97** - Moderate damping, natural decay (acoustic guitar)
- **0.98-0.995** - Light damping, long sustain (bright strings, electric guitar)
- **0.996-0.999** - Very light damping, extended sustain (sitar, resonant strings)

Higher feedback values produce longer, brighter tones. Lower values produce shorter, darker tones.

## Presets

```rust
// Default preset (balanced)
KarplusStrongParams::default()  // feedback = 0.995

// Custom feedback
KarplusStrongParams::new(0.97)  // Moderate decay
```

## Advanced Techniques

**Combine with effects:**
```rust
comp.track("processed")
    .karplus_strong(KarplusStrongParams::new(0.994))
    .reverb(Reverb::new(0.5, 0.4, 0.3))  // Add room ambience
    .chorus(Chorus::new(0.5, 0.3, 0.4))  // Thicken the sound
    .notes(&[A3, C4, E4], 0.5);
```

**Strum patterns:**
```rust
// Downstroke (sequential timing)
comp.track("strum")
    .karplus_strong(KarplusStrongParams::new(0.996))
    .note(&[E2], 0.02)
    .note(&[A2], 0.02)
    .note(&[D3], 0.02)
    .note(&[G3], 0.02)
    .note(&[B3], 0.02)
    .note(&[E4], 0.02);
```

**Characteristics:**
- Extremely CPU-efficient (single delay line + filter per voice)
- Naturally decaying envelopes (no ADSR needed)
- Authentic plucked string character
- Limited timbral variation (string-like sounds only)
- No sustain control after onset (decay is inherent to the algorithm)
- Sensitive to pitch (lower pitches require longer delay lines)
