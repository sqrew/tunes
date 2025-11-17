# Spectral Effects

Tunes provides 12 advanced spectral effects that operate in the frequency domain using FFT (Fast Fourier Transform). These effects manipulate individual frequency bins, enabling creative sound design impossible with traditional time-domain effects.

## What Are Spectral Effects?

**Traditional effects** work on the raw audio signal (time domain):
- Simple and fast
- Limited to amplitude/timing modifications

**Spectral effects** work on frequency components (frequency domain):
- Convert audio to frequencies using FFT
- Modify individual frequency bins
- Convert back to audio using inverse FFT
- Enable frequency-specific processing

**Trade-off:** More CPU intensive, but vastly more creative possibilities.

---

## Available Spectral Effects

| Effect | Category | Description | Use Cases |
|--------|----------|-------------|-----------|
| **SpectralFreeze** | Texture | Freezes frequency spectrum in time | Ambient pads, drones, textures |
| **SpectralBlur** | Texture | Smooths frequency bins over time | Soft pads, dreamlike atmospheres |
| **SpectralMorph** | Texture | Morphs between dry and processed sound | Evolving timbres, transitions |
| **SpectralInvert** | Transform | Inverts frequency spectrum | Metallic tones, special effects |
| **SpectralShift** | Pitch | Shifts all frequencies up/down | Pitch effects, harmonizers |
| **SpectralWiden** | Stereo | Widens stereo image per frequency | Immersive soundscapes, mixing |
| **SpectralExciter** | Enhancement | Adds harmonic excitement | Brightness, presence, air |
| **SpectralFilter** | Dynamics | Frequency-dependent filtering | Spectral gating, cleaning |
| **SpectralGate** | Dynamics | Gates frequencies below threshold | Noise reduction, clarity |
| **SpectralCompressor** | Dynamics | Compresses per-frequency band | Multiband compression |
| **SpectralDynamics** | Dynamics | Expands/compresses frequency ranges | Spectral shaping |
| **SpectralScramble** | Creative | Randomizes frequency bin order | Glitches, digital artifacts |

---

## Spectral Texture Effects

### SpectralFreeze

Captures and freezes the frequency spectrum at a moment in time, creating sustained, evolving textures.

**Parameters:**
- `freeze_amount` (0.0-1.0): How much to freeze (0.0 = normal, 1.0 = completely frozen)
- `mix` (0.0-1.0): Dry/wet blend

**Presets:**
```rust
SpectralFreeze::frozen()    // Full freeze effect
SpectralFreeze::shimmer()   // Subtle freezing with movement
SpectralFreeze::glitch()    // Aggressive frozen artifacts
```

**Example:**
```rust
comp.instrument("pad", &Instrument::warm_pad())
    .spectral_freeze(SpectralFreeze::shimmer())
    .note(&[C3, E3, G3], 4.0);
```

**Use cases:**
- Ambient pads that sustain indefinitely
- Drone textures from rhythmic material
- Creating "frozen" moments in time

---

### SpectralBlur

Smooths frequency content over time, creating soft, blurred textures by averaging frequency bins across frames.

**Parameters:**
- `blur_amount` (0.0-1.0): Amount of temporal smoothing
- `spatial_blur` (0.0-1.0): Frequency bin neighbor blurring
- `mix` (0.0-1.0): Dry/wet blend

**Presets:**
```rust
SpectralBlur::gentle()    // Subtle softening
SpectralBlur::dreamy()    // Medium blur for atmospheric sounds
SpectralBlur::heavy()     // Heavy blur for ambient textures
```

**Example:**
```rust
comp.instrument("strings", &Instrument::warm_pad())
    .spectral_blur(SpectralBlur::dreamy())
    .notes(&[A3, C4, E4], 2.0);
```

**Use cases:**
- Softening harsh transients
- Creating dreamlike atmospheres
- Smoothing out digital artifacts

---

### SpectralMorph

Morphs between the original signal and a spectrally processed version, with flexible control over transition depth.

**Parameters:**
- `morph_depth` (0.0-1.0): How much to morph the spectrum
- `brightness` (0.0-2.0): Tilt toward high frequencies
- `mix` (0.0-1.0): Dry/wet blend

**Presets:**
```rust
SpectralMorph::subtle()    // Gentle spectral changes
SpectralMorph::moderate()  // Noticeable transformation
SpectralMorph::extreme()   // Heavy morphing
```

**Example:**
```rust
comp.instrument("vocal", &Instrument::synth_lead())
    .spectral_morph(SpectralMorph::moderate())
    .note(&[440.0], 2.0);
```

**Use cases:**
- Evolving timbres
- Transitions between sounds
- Adding movement to static tones

---

## Spectral Transform Effects

### SpectralInvert

Inverts the frequency spectrum, swapping low and high frequencies around a center point.

**Parameters:**
- `mix` (0.0-1.0): Dry/wet blend

**Presets:**
```rust
SpectralInvert::metallic()  // Full inversion for metallic tones
SpectralInvert::subtle()    // Partial inversion
```

**Example:**
```rust
comp.instrument("synth", &Instrument::synth_lead())
    .spectral_invert(SpectralInvert::metallic())
    .note(&[C4], 1.0);
```

**Use cases:**
- Metallic/robotic tones
- Otherworldly sound design
- Spectral effects

**Note:** Creates dramatic timbral changes - sounds completely different!

---

### SpectralShift

Shifts all frequencies up or down by a fixed amount (in Hz), different from pitch shifting which preserves harmonic relationships.

**Parameters:**
- `shift_hz` (Hz): Frequency shift amount (positive = up, negative = down)
- `mix` (0.0-1.0): Dry/wet blend

**Presets:**
```rust
SpectralShift::subtle()   // Small shift for detuning
SpectralShift::metallic() // Medium shift for metallic tones
SpectralShift::bell()     // Large shift for bell-like sounds
SpectralShift::alien()    // Extreme shift for weird tones
SpectralShift::down()     // Shift down for darker tones
```

**Example:**
```rust
comp.instrument("bell", &Instrument::pluck())
    .spectral_shift(SpectralShift::bell())
    .note(&[C5], 1.0);
```

**Use cases:**
- Inharmonic bell/metallic sounds
- Detuning/chorus effects
- Alien/robotic vocals
- Sound design

**How it differs from pitch shift:** Pitch shift maintains harmonic relationships (all harmonics move proportionally). Spectral shift moves all frequencies by the same Hz amount, creating inharmonic results.

---

## Spectral Stereo Effects

### SpectralWiden

Widens the stereo image on a per-frequency basis, creating immersive spatial effects.

**Parameters:**
- `width` (0.0-2.0): Stereo width (0.0 = mono, 1.0 = normal, 2.0 = ultra-wide)
- `low_freq` (Hz): Below this, keep narrow for bass
- `high_freq` (Hz): Above this, apply full widening
- `mix` (0.0-1.0): Dry/wet blend

**Presets:**
```rust
SpectralWiden::subtle()    // Gentle widening
SpectralWiden::moderate()  // Noticeable width
SpectralWiden::wide()      // Wide stereo field
SpectralWiden::ultra()     // Maximum width
```

**Example:**
```rust
comp.instrument("pad", &Instrument::warm_pad())
    .spectral_widen(SpectralWiden::wide())
    .note(&[C3, E3, G3], 4.0);
```

**Use cases:**
- Widening synth pads
- Creating immersive soundscapes
- Mixing: making sounds sit wider in the mix
- Stereo enhancement

**Best practice:** Keep bass frequencies narrow (< 200 Hz) for mono compatibility.

---

## Spectral Enhancement Effects

### SpectralExciter

Adds harmonic excitement and brightness by enhancing high-frequency content, similar to aural exciters used in mastering.

**Parameters:**
- `frequency` (Hz): Crossover frequency for enhancement
- `drive` (0.0-2.0): Amount of harmonic generation
- `harmonics` (0.0-1.0): Harmonic content amount
- `mix` (0.0-1.0): Dry/wet blend

**Presets:**
```rust
SpectralExciter::gentle()     // Subtle brightness
SpectralExciter::moderate()   // Noticeable enhancement
SpectralExciter::aggressive() // Heavy excitement
SpectralExciter::air()        // High-frequency sparkle
SpectralExciter::presence()   // Vocal presence boost
```

**Example:**
```rust
// Add air to vocals
comp.instrument("vocal", &Instrument::synth_lead())
    .spectral_exciter(SpectralExciter::air())
    .note(&[440.0], 2.0);

// Master bus enhancement
mixer.master_spectral_exciter(SpectralExciter::gentle());
```

**Use cases:**
- Adding brightness without EQ
- Vocal presence
- Mixing: making instruments "pop"
- Mastering: final shine

---

## Spectral Dynamics Effects

### SpectralGate

Gates individual frequency bins below a threshold, removing quiet frequency content while preserving loud frequencies.

**Parameters:**
- `threshold` (0.0-1.0): Amplitude threshold for gating
- `attack` (seconds): How fast gate opens
- `release` (seconds): How fast gate closes
- `mix` (0.0-1.0): Dry/wet blend

**Presets:**
```rust
SpectralGate::gentle()      // Subtle gating
SpectralGate::aggressive()  // Strong gating
SpectralGate::denoise()     // Noise reduction
SpectralGate::tighten()     // Tighten transients
```

**Example:**
```rust
comp.instrument("vocal", &Instrument::synth_lead())
    .spectral_gate(SpectralGate::denoise())
    .note(&[440.0], 2.0);
```

**Use cases:**
- Noise reduction
- Removing room tone
- Cleaning up recordings
- Tightening instruments

---

### SpectralCompressor

Compresses each frequency band independently, like multiband compression but with hundreds of bands.

**Parameters:**
- `threshold` (0.0-1.0): Compression threshold
- `ratio` (1.0-20.0): Compression ratio
- `attack` (seconds): How fast compressor responds
- `release` (seconds): How fast compressor recovers

**Presets:**
```rust
SpectralCompressor::gentle()     // Subtle multiband compression
SpectralCompressor::aggressive() // Heavy compression
SpectralCompressor::glue()       // Mix glue compression
```

**Example:**
```rust
// Master bus spectral compression
mixer.master_spectral_compressor(SpectralCompressor::gentle());
```

**Use cases:**
- Multiband compression (but more bands!)
- Taming harsh frequencies
- Gluing mixes together
- Mastering

---

### SpectralDynamics

Expands or compresses frequency ranges based on their amplitude, for sophisticated spectral shaping.

**Parameters:**
- `threshold` (0.0-1.0): Dynamics threshold
- `ratio` (0.5-2.0): < 1.0 = expansion, > 1.0 = compression
- `attack` (seconds): Response time
- `release` (seconds): Recovery time

**Presets:**
```rust
SpectralDynamics::gentle()     // Subtle dynamics
SpectralDynamics::moderate()   // Noticeable processing
SpectralDynamics::aggressive() // Heavy dynamics
SpectralDynamics::expander()   // Expands dynamic range
SpectralDynamics::gate_like()  // Gate-like expansion
```

**Example:**
```rust
comp.instrument("drums", &Instrument::drum_kit())
    .spectral_dynamics(SpectralDynamics::aggressive())
    .drum(DrumType::Kick808);
```

**Use cases:**
- Spectral expansion/compression
- Dynamic frequency shaping
- Advanced mixing
- Sound design

---

### SpectralFilter

Frequency-dependent filtering that preserves or removes specific frequency ranges.

**Parameters:**
- `cutoff_low` (Hz): Low frequency cutoff
- `cutoff_high` (Hz): High frequency cutoff
- `resonance` (0.0-10.0): Filter resonance
- `mix` (0.0-1.0): Dry/wet blend

**Presets:**
```rust
SpectralFilter::low_pass()   // Remove highs
SpectralFilter::high_pass()  // Remove lows
SpectralFilter::band_pass()  // Isolate frequency range
SpectralFilter::telephone()  // Telephone effect
SpectralFilter::radio()      // AM radio effect
SpectralFilter::rumble_cut() // Remove low rumble
SpectralFilter::air_cut()    // Remove high hiss
```

**Example:**
```rust
comp.instrument("vocal", &Instrument::synth_lead())
    .spectral_filter(SpectralFilter::telephone())
    .note(&[440.0], 2.0);
```

**Use cases:**
- Creative filtering
- Spectral gating
- Sound design
- Special effects

---

## Spectral Creative Effects

### SpectralScramble

Randomizes the order of frequency bins, creating glitchy, digital artifacts and chaotic textures.

**Parameters:**
- `scramble_amount` (0.0-1.0): How much to scramble bins
- `probability` (0.0-1.0): Chance of scrambling each frame
- `seed` (integer): Random seed for reproducibility
- `mix` (0.0-1.0): Dry/wet blend

**Presets:**
```rust
SpectralScramble::subtle()   // Occasional glitches
SpectralScramble::moderate() // Noticeable scrambling
SpectralScramble::chaos()    // Chaotic glitching
SpectralScramble::glitch()   // Heavy glitch effect
SpectralScramble::digital()  // Digital artifacts
```

**Example:**
```rust
comp.instrument("glitch", &Instrument::synth_lead())
    .spectral_scramble(SpectralScramble::glitch())
    .note(&[C4], 2.0);
```

**Use cases:**
- Glitch effects
- Digital artifacts
- Experimental music
- Sound design
- IDM/electronic music

---

## Performance Considerations

### CPU Usage

Spectral effects are **more CPU intensive** than traditional effects:

| Effect | Relative Cost | FFT Size |
|--------|---------------|----------|
| Traditional Filter | 1x (baseline) | None |
| SpectralBlur | ~5-10x | 2048-4096 |
| SpectralShift | ~5-10x | 2048-4096 |
| SpectralScramble | ~5-10x | 2048-4096 |

**Why?** FFT requires transforming audio to frequency domain and back for every frame.

### Optimization Tips

1. **Use sparingly:** Apply to key instruments, not everything
2. **Smaller FFT sizes:** Use 2048 instead of 4096 for less latency/CPU
3. **Render to file:** For production, render with spectral effects then use the rendered file
4. **Master bus only:** Apply spectral effects to final mix, not individual tracks

### Latency

Spectral effects introduce latency due to FFT window size:
- FFT size 2048 @ 44.1kHz = ~46ms latency
- FFT size 4096 @ 44.1kHz = ~93ms latency

**Not suitable for:** Live performance with monitoring
**Fine for:** Studio production, sound design, post-processing

---

## Combining Spectral Effects

Spectral effects can be chained, but **order matters**:

```rust
comp.instrument("experimental", &Instrument::warm_pad())
    .spectral_freeze(SpectralFreeze::shimmer())  // First: freeze spectrum
    .spectral_shift(SpectralShift::bell())       // Then: shift frequencies
    .spectral_widen(SpectralWiden::wide())       // Finally: widen result
    .note(&[C3], 4.0);
```

**Best practices:**
1. **Texture effects first:** Freeze/Blur create the base texture
2. **Transform effects:** Shift/Invert/Morph shape the spectrum
3. **Enhancement last:** Exciter/Widen polish the result

---

## Example: Complete Spectral Processing Chain

```rust
use tunes::prelude::*;
use tunes::synthesis::effects::*;

let mut comp = Composition::new(Tempo::new(120.0));

// Ambient pad with full spectral treatment
comp.instrument("spectral_pad", &Instrument::warm_pad())
    // Start with frozen texture
    .spectral_freeze(SpectralFreeze::shimmer())

    // Add blur for dreaminess
    .spectral_blur(SpectralBlur::gentle())

    // Shift frequencies for inharmonic quality
    .spectral_shift(SpectralShift::subtle())

    // Widen for immersive stereo
    .spectral_widen(SpectralWiden::wide())

    // Final brightness
    .spectral_exciter(SpectralExciter::air())

    .note(&[C2, E2, G2], 8.0);

// Master spectral enhancement
let mut mixer = comp.into_mixer();
mixer.master_spectral_exciter(SpectralExciter::gentle());
mixer.master_spectral_widen(SpectralWiden::moderate());
```

---

## Spectral Effects vs Traditional Effects

| Aspect     | Traditional    | Spectral     |
|------------|----------------|--------------|
| Domain     | Time           | Frequency    |
| CPU        | Low            | High         |
| Latency    | Minimal        | ~50-100ms    |
| Precision  | Good           | Excellent    |
| Creativity | Limited        | Vast         |
| Use case   | General mixing | Sound design |

**When to use traditional:** Mixing, live performance, CPU-limited scenarios
**When to use spectral:** Creative sound design, mastering, post-production

---

## Further Reading

- [Effects Overview](./effects.md) - Traditional effects
- [Synthesis](./synthesis.md) - Sound synthesis techniques
- [Examples](../../examples/) - See `spectral_effects_demo.rs`

---

**Pro Tip:** Start with presets! Every spectral effect has carefully tuned presets (`::gentle()`, `::moderate()`, `::aggressive()`, etc.) that provide professional starting points. Experiment from there.
