# Granular Synthesis

Granular synthesis divides audio samples into small segments (grains) of 1-100ms and reassembles them with varied timing, pitch, and density to create textures and transformations.

## What It Is

Audio is segmented into overlapping grains which are individually processed and recombined. Each grain is windowed (typically with a Hann or Gaussian envelope) to prevent clicks. Grains can be played at different rates, pitches, and positions within the source material.

**Key parameters:**
- **Grain size** - Duration of each grain (10-100ms typical)
- **Grain density** - Overlap and spacing of grains (0.0-1.0)
- **Pitch shift** - Playback rate of grains (affects pitch without changing duration)
- **Position** - Playback position within source material

## When to Use

- Ambient textures and soundscapes
- Time-stretching without pitch change
- Pitch-shifting without time change
- Clouds and swarms of sound
- Frozen or stuttering textures
- Spectral manipulation of samples

## Usage

```rust
use tunes::prelude::*;
use tunes::synthesis::granular::GranularParams;

let mut comp = Composition::new(Tempo::new(120.0));

// Load source material
comp.load_sample("source", "sample.wav")?;

// Dense texture (many overlapping grains)
comp.track("texture")
    .granular(
        "source",
        GranularParams::new(0.05, 0.8, 1.0),  // 50ms grains, 80% density, normal pitch
        5.0  // Play for 5 seconds
    );

// Sparse, pitched down
comp.track("sparse")
    .granular(
        "source",
        GranularParams::new(0.08, 0.3, 0.5),  // 80ms grains, 30% density, octave down
        8.0
    );

// Cloud effect (small grains, high density, slight detune)
comp.track("cloud")
    .granular(
        "source",
        GranularParams::new(0.02, 0.9, 1.02),  // 20ms grains, 90% density, slight pitch up
        10.0
    );
```

## Parameter Guidelines

**Grain size:**
- **10-30ms** - Grainy, textural (individual grains audible)
- **30-60ms** - Smooth textures
- **60-100ms** - Closer to original audio character

**Density:**
- **0.1-0.3** - Sparse, stuttering
- **0.4-0.7** - Moderate texture
- **0.8-1.0** - Dense, smooth (original audio maintained)

**Pitch:**
- **0.5** - One octave down
- **1.0** - Original pitch
- **2.0** - One octave up
- **0.98-1.02** - Subtle detuning for chorus effects

**Characteristics:**
- Computationally intensive (many overlapping synthesis processes)
- Effective for radical transformations of source material
- Produces unique textures unattainable with other methods
- Requires source audio (samples)
