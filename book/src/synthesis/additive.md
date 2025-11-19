# Additive Synthesis

Additive synthesis constructs complex waveforms by summing individual sine wave harmonics. Each harmonic's amplitude can be independently controlled to shape the timbre.

## What It Is

Sound is built from the ground up by combining sine waves at integer multiples of a fundamental frequency. Each harmonic contributes to the overall spectral shape. The amplitude of each harmonic determines its contribution to the final timbre.

**Harmonic series:**
- **1st harmonic** (fundamental) - Base frequency
- **2nd harmonic** - Octave above (2× frequency)
- **3rd harmonic** - Perfect fifth above octave (3× frequency)
- **4th harmonic** - Two octaves above (4× frequency)
- And so on...

## When to Use

- Organ and drawbar-style tones
- Precise timbral control
- Recreating acoustic instrument spectra
- Bright, pure tones with controllable overtones
- Educational purposes (visualizing harmonic content)

## Usage

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

// Sawtooth-like (descending harmonic amplitudes)
comp.track("saw")
    .additive_synth(&[1.0, 0.5, 0.33, 0.25, 0.2, 0.17, 0.14])
    .notes(&[C4, E4, G4], 0.5);

// Organ (odd harmonics only, like a square wave)
comp.track("organ")
    .additive_synth(&[1.0, 0.0, 0.33, 0.0, 0.2, 0.0, 0.14])
    .notes(&[C3, E3, G3], 1.0);

// Bright tone (emphasize upper harmonics)
comp.track("bright")
    .additive_synth(&[0.5, 0.6, 0.7, 0.8, 0.9, 1.0])
    .notes(&[A4], 2.0);

// Pure tone (fundamental only)
comp.track("pure")
    .additive_synth(&[1.0])
    .notes(&[C5], 1.0);

// Custom spectrum (arbitrary harmonic balance)
comp.track("custom")
    .additive_synth(&[1.0, 0.3, 0.0, 0.5, 0.0, 0.2, 0.1, 0.4])
    .notes(&[E4, G4, B4], 0.5);
```

## Common Harmonic Profiles

**Sawtooth approximation:**
```rust
.additive_synth(&[1.0, 0.5, 0.33, 0.25, 0.2, 0.17, 0.14, 0.125])
```
All harmonics, amplitude = 1/n

**Square wave approximation:**
```rust
.additive_synth(&[1.0, 0.0, 0.33, 0.0, 0.2, 0.0, 0.14, 0.0])
```
Odd harmonics only, amplitude = 1/n

**Triangle wave approximation:**
```rust
.additive_synth(&[1.0, 0.0, 0.11, 0.0, 0.04, 0.0, 0.02, 0.0])
```
Odd harmonics, amplitude = 1/(n²)

**Clarinet-like:**
```rust
.additive_synth(&[1.0, 0.0, 0.75, 0.0, 0.5, 0.0, 0.14, 0.0])
```
Strong odd harmonics (hollow sound)

**Trumpet-like:**
```rust
.additive_synth(&[1.0, 0.9, 0.8, 0.6, 0.5, 0.4, 0.3, 0.2])
```
Strong presence of all harmonics (bright, brassy)

## Parameter Notes

- **Array length** - Number of harmonics (more = richer timbre, higher CPU cost)
- **Zero values** - Harmonics with amplitude 0.0 are skipped (efficient)
- **Amplitude range** - Typically 0.0-1.0 per harmonic
- **Normalization** - Output is not automatically normalized; reduce amplitudes if clipping occurs

**Characteristics:**
- Precise spectral control (exact harmonic balance)
- CPU cost scales with number of active harmonics
- Produces static timbres (no automatic harmonic evolution)
- Suitable for pure, organ-like tones
- Combine with envelopes and filters for evolving sounds
