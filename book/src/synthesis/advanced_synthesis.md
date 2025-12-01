# Advanced Synthesis

Tunes provides several synthesis techniques beyond basic waveforms. Each method has unique characteristics suited to different use cases.

---

## FM Synthesis

FM (Frequency Modulation) synthesis generates complex harmonic content by using one oscillator (modulator) to modulate the frequency of another oscillator (carrier).

### What It Is

FM synthesis varies the frequency of a carrier wave at audio rates, producing sidebands that create rich harmonic and inharmonic overtones. The modulation index controls the intensity of modulation, determining timbral complexity.

**Key parameters:**
- **Carrier frequency** - The base pitch
- **Modulator ratio** - Frequency relationship between modulator and carrier
- **Modulation index** - Intensity of frequency modulation (affects brightness)

### When to Use

- Electric piano and bell tones (harmonic ratios like 1:2, 1:3)
- Metallic and percussive sounds (inharmonic ratios like 1:2.1)
- Brass and reed instruments (moderate modulation index)
- Complex evolving timbres with few oscillators

### Usage

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

### Presets

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

---

## Additive Synthesis

Additive synthesis constructs complex waveforms by summing individual sine wave harmonics. Each harmonic's amplitude can be independently controlled to shape the timbre.

### What It Is

Sound is built from the ground up by combining sine waves at integer multiples of a fundamental frequency. Each harmonic contributes to the overall spectral shape. The amplitude of each harmonic determines its contribution to the final timbre.

**Harmonic series:**
- **1st harmonic** (fundamental) - Base frequency
- **2nd harmonic** - Octave above (2× frequency)
- **3rd harmonic** - Perfect fifth above octave (3× frequency)
- **4th harmonic** - Two octaves above (4× frequency)
- And so on...

### When to Use

- Organ and drawbar-style tones
- Precise timbral control
- Recreating acoustic instrument spectra
- Bright, pure tones with controllable overtones
- Educational purposes (visualizing harmonic content)

### Usage

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
```

### Common Harmonic Profiles

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

**Characteristics:**
- Precise spectral control (exact harmonic balance)
- CPU cost scales with number of active harmonics
- Produces static timbres (no automatic harmonic evolution)
- Suitable for pure, organ-like tones
- Combine with envelopes and filters for evolving sounds

---

## Granular Synthesis

Granular synthesis divides audio samples into small segments (grains) of 1-100ms and reassembles them with varied timing, pitch, and density to create textures and transformations.

### What It Is

Audio is segmented into overlapping grains which are individually processed and recombined. Each grain is windowed (typically with a Hann or Gaussian envelope) to prevent clicks. Grains can be played at different rates, pitches, and positions within the source material.

**Key parameters:**
- **Grain size** - Duration of each grain (10-100ms typical)
- **Grain density** - Overlap and spacing of grains (0.0-1.0)
- **Pitch shift** - Playback rate of grains (affects pitch without changing duration)
- **Position** - Playback position within source material

### When to Use

- Ambient textures and soundscapes
- Time-stretching without pitch change
- Pitch-shifting without time change
- Clouds and swarms of sound
- Frozen or stuttering textures
- Spectral manipulation of samples

### Usage

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

### Parameter Guidelines

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

---

## Karplus-Strong Synthesis

Karplus-Strong is a physical modeling technique that simulates plucked string instruments by filtering a burst of noise through a feedback delay line.

### What It Is

The algorithm generates a short noise burst, feeds it into a delay line tuned to the desired pitch, and applies a low-pass filter in the feedback path. This creates a naturally decaying tone with harmonic content similar to plucked strings.

**Process:**
1. Generate initial excitation (noise burst or impulse)
2. Feed signal through delay line (length determines pitch)
3. Apply low-pass filter to feedback (simulates string damping)
4. Sum delayed signal back into the delay line (sustains the tone)

The delay time is set to `sample_rate / frequency`, creating a periodic waveform at the target pitch. The filter progressively removes high frequencies, simulating natural string damping.

### When to Use

- Plucked string instruments (guitar, banjo, harp, sitar)
- Percussive, natural-sounding tones
- Low CPU cost synthesis with realistic character
- Ethnic string instruments
- Game audio (footsteps on wood, impacts)

### Usage

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

### Parameter Guidelines

**Feedback coefficient (damping):**
- **0.90-0.93** - Heavy damping, short decay (muted strings, dead strings)
- **0.94-0.97** - Moderate damping, natural decay (acoustic guitar)
- **0.98-0.995** - Light damping, long sustain (bright strings, electric guitar)
- **0.996-0.999** - Very light damping, extended sustain (sitar, resonant strings)

Higher feedback values produce longer, brighter tones. Lower values produce shorter, darker tones.

### Presets

```rust
// Default preset (balanced)
KarplusStrongParams::default()  // feedback = 0.995

// Custom feedback
KarplusStrongParams::new(0.97)  // Moderate decay
```

### Advanced Techniques

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
