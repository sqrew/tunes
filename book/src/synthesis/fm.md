# FM Synthesis

FM (Frequency Modulation) synthesis generates complex harmonic content by using one oscillator (modulator) to modulate the frequency of another oscillator (carrier).

## What It Is

FM synthesis varies the frequency of a carrier wave at audio rates, producing sidebands that create rich harmonic and inharmonic overtones. The modulation index controls the intensity of modulation, determining timbral complexity.

**Key parameters:**
- **Carrier frequency** - The base pitch
- **Modulator ratio** - Frequency relationship between modulator and carrier
- **Modulation index** - Intensity of frequency modulation (affects brightness)

## When to Use

- Electric piano and bell tones (harmonic ratios like 1:2, 1:3)
- Metallic and percussive sounds (inharmonic ratios like 1:2.1)
- Brass and reed instruments (moderate modulation index)
- Complex evolving timbres with few oscillators

## Usage

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

// Bell-like sound (high harmonic ratio)
comp.track("bell")
    .fm_custom(3.5, 4.0)  // ratio: 3.5, index: 4.0
    .notes(&[C5, E5, G5], 1.0);

// Electric piano (moderate modulation)
comp.track("epiano")
    .fm_custom(2.0, 2.5)
    .envelope(Envelope::new(0.001, 0.1, 0.3, 0.2))
    .notes(&[C4, E4, G4], 0.5);

// Brass (low modulation index)
comp.track("brass")
    .fm_custom(1.0, 1.5)
    .envelope(Envelope::new(0.05, 0.1, 0.7, 0.2))
    .notes(&[C3, E3, G3], 1.0);
```

## Presets

```rust
use tunes::synthesis::fm::FMParams;

comp.track("preset")
    .fm(FMParams::bell())           // Metallic bell tone
    .fm(FMParams::electric_piano()) // EP sound
    .fm(FMParams::brass())          // Brass texture
    .fm(FMParams::bass());          // FM bass
```

**Characteristics:**
- CPU-efficient (generates complex spectra with two oscillators)
- Difficult to predict exact harmonic content
- Excellent for percussive and metallic timbres
- Responsive to envelope modulation
