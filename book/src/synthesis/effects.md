# Effects

Tunes provides a professional-grade effects system with 16 built-in effects that can be applied at track, bus, and master levels.

## EffectChain System

Every track, bus, and master fader has an `EffectChain` - a unified container for all 16 effects with automatic priority-based ordering.

### Priority-Based Effect Ordering

Effects are automatically sorted by priority (lower number = earlier in chain):

```
Priority 0-49: Early (EQ, filters)
Priority 50-99: Dynamics (Compressor, Gate, Limiter)
Priority 100-149: Saturation (Saturation, Distortion, Bitcrusher)
Priority 150-199: Modulation (Chorus, Phaser, Flanger, Ring Mod, Tremolo, AutoPan)
Priority 200-249: Time/Space (Delay, Reverb)
Priority 250+: Final (Reserved)
```

**Why this matters:** Effect order dramatically impacts sound. EQ before compression gives different results than compression before EQ. The priority system ensures professional signal flow automatically.

### Dual-Mode Processing

- **Mono mode (`process_mono`)**: For individual tracks
  - Input: Mono signal
  - Output: Mono signal with effects
  - Used by: Individual instrument tracks

- **Stereo mode (`process_stereo`)**: For buses and master
  - Input: Stereo (left + right) signals
  - Output: Processed stereo signals
  - Used by: Buses and master fader
  - Special: Compressor uses max(L, R) to preserve stereo image

---

## Three Levels of Effects

### Track-Level Effects

Apply effects to individual instruments:

```rust
comp.instrument("guitar", &Instrument::electric_guitar_clean())
    .reverb(Reverb::new(0.2, 0.4))           // Add space
    .compressor(Compressor::new(0.6, 3.0, 0.005, 0.05, 44100.0))  // Tighten dynamics
    .delay(Delay::new(0.375, 0.3, 0.5))      // Slapback delay
    .notes(&[E3, A3, B3], 0.5);
```

**Use cases:**
- Individual instrument character
- Specific timbral effects
- Each track needs different processing

### Bus-Level Effects

Apply effects to groups of tracks using the `BusBuilder` API:

```rust
let mut mixer = comp.into_mixer();

// Fluent API for bus effects
mixer.bus("drums")
    .reverb(Reverb::new(0.3, 0.4, 0.3))
    .compressor(Compressor::new(0.65, 4.0, 0.01, 0.08, 1.0))
    .volume(0.85);

// Sidechaining: Bass ducks when kick hits
mixer.bus("bass")
    .compressor(
        Compressor::new(0.6, 8.0, 0.001, 0.15, 1.2)
            .with_sidechain_track("kick")
    );
```

**Use cases:**
- Shared ambience (one reverb for all drums)
- Group processing (compress all vocals together)
- Sidechaining / ducking effects (see Compressor section)
- Efficient CPU usage (fewer effect instances)
- Professional mixing workflows

### Master-Level Effects

Apply effects to the final stereo mix:

```rust
// Professional mastering chain
mixer.master_parametric_eq(ParametricEQ::new()
    .band(60.0, -3.0, 0.7)      // Cut rumble
    .band(3000.0, 2.0, 1.5));   // Presence boost

mixer.master_compressor(Compressor::new(0.5, 2.0, 0.01, 0.1, 44100.0));  // Glue
mixer.master_saturation(Saturation::new(0.1));                           // Warmth
mixer.master_limiter(Limiter::new(0.95));                                // Protection
```

**Use cases:**
- Mastering: Final polish on complete mix
- Mix glue: Subtle compression to unify tracks
- Protection: Limiting to prevent clipping
- Creative: Tape saturation, lo-fi effects

---

## The 16 Effects

### 1. EQ (3-Band Equalizer)

Classic 3-band EQ with shelving filters.

```rust
use tunes::prelude::*;

comp.track("vocal")
    .eq(EQ::new(
        0.9,     // Low gain (slightly reduce bass)
        1.0,     // Mid gain (neutral)
        1.2,     // High gain (boost highs)
        250.0,   // Low/mid crossover (Hz)
        4000.0   // Mid/high crossover (Hz)
    ))
    .note(&[A4], 1.0);
```

**Parameters:**
- Gain values: 0.5 = -6dB, 1.0 = 0dB, 2.0 = +6dB
- Priority: 10 (very early in chain)

**Use cases:** Tone shaping, frequency balancing, removing mud

### 2. Parametric EQ

Surgical frequency control with multiple bands.

```rust
mixer.master_parametric_eq(ParametricEQ::new()
    .band(100.0, -6.0, 1.0)     // Cut rumble, wide Q
    .band(250.0, -3.0, 1.5)     // Reduce mud, moderate Q
    .band(3000.0, 4.0, 2.0)     // Presence boost, narrow Q
    .band(8000.0, -2.0, 1.5));  // Tame sibilance
```

**Parameters:**
- Frequency: Center frequency (Hz)
- Gain: Boost (+) or cut (-) in dB
- Q: Bandwidth (0.5 = wide/gentle, 2.0 = medium, 10.0 = narrow/surgical)

**Presets:** `VocalClarity`, `BassBoost`, `BrightAiry`, `Telephone`, `Warmth`

**Use cases:** Surgical frequency shaping, mastering, problem solving

### 3. Compressor

Dynamic range control - makes loud parts quieter and can make quiet parts louder.

```rust
comp.track("bass")
    .compressor(Compressor::new(
        0.5,      // Threshold (0.0-1.0, lower = more compression)
        4.0,      // Ratio (2.0 = gentle, 4.0 = medium, 10.0 = aggressive)
        0.01,     // Attack time (seconds, fast = 0.001, slow = 0.1)
        0.1,      // Release time (seconds)
        44100.0   // Sample rate
    ));
```

**How it works:**
- Reduces volume of signals above threshold
- Ratio determines how much reduction (4:1 = 4dB input becomes 1dB output)
- Attack: How quickly compression kicks in (fast = snappy, slow = natural)
- Release: How quickly compression lets go

**Use cases:**
- Even out dynamics (vocals, bass)
- Add punch (fast attack/release on drums)
- Glue tracks together (gentle 2:1 ratio on master)

#### Sidechaining / Ducking

The compressor supports **sidechaining** - using one signal to control compression of another. This is essential for EDM "pumping" effects and professional mixing.

```rust
let mut comp = Composition::new(Tempo::new(128.0));

// Create kick drum
comp.track("kick")
    .bus("drums")
    .drum(DrumType::Kick);

// Create bass that will duck
comp.track("bass")
    .bus("bass")
    .notes(&[C2, C2, G2, A2], 1.0);

let mut mixer = comp.into_mixer();

// Bass compresses when kick hits
mixer.bus("bass").compressor(
    Compressor::new(0.6, 8.0, 0.001, 0.15, 1.2)
        .with_sidechain_track("kick")  // Duck when "kick" plays
);
```

**How it works:**
- The compressor on the "bass" bus monitors the "kick" track's level
- When the kick hits (high level), the bass gets compressed (reduced in volume)
- Creates space for the kick and adds rhythmic pumping

**Two sidechaining modes:**
```rust
// Track-to-bus: Bass ducks when specific kick track plays
.with_sidechain_track("kick")

// Bus-to-bus: Entire synth bus ducks when entire drums bus plays
.with_sidechain_bus("drums")
```

**Parameter tuning:**
- **Aggressive pump** (EDM): Fast attack (0.001), slow release (0.3-0.5), high ratio (8.0+)
- **Subtle duck** (mixing): Slower attack (0.01), faster release (0.15), moderate ratio (3.0-4.0)
- **Threshold**: 0.5-0.7 (lower = more sensitive, more ducking)

**Common uses:**
- Kick ducking bass (EDM staple)
- Vocal ducking background music (podcasts, voiceovers)
- Drums ducking pads (creates rhythmic breathing)
- Creative rhythmic pumping effects

See the `examples/sidechaining.rs` for complete examples of different sidechaining styles.

### 4. Gate

Noise gate - silences audio below a threshold.

```rust
comp.track("guitar")
    .gate(Gate::new(
        0.05,     // Threshold (signals below this are silenced)
        0.002,    // Attack (how fast gate opens)
        0.05,     // Release (how fast gate closes)
        44100.0   // Sample rate
    ));
```

**Use cases:** Remove background noise, clean up recordings, create rhythmic effects

### 5. Limiter

Brick-wall limiting - prevents audio from exceeding a ceiling.

```rust
// Prevent clipping on master
mixer.master_limiter(Limiter::new(0.95));  // Max level = 0.95 (-0.4dB)
```

**Use cases:** Prevent digital clipping, maximize loudness, protect speakers

### 6. Saturation

Analog-style harmonic saturation.

```rust
comp.track("synth")
    .saturation(Saturation::new(0.3));  // Drive amount (0.0-1.0)
```

**Use cases:** Add warmth, analog character, subtle harmonics

### 7. Distortion

Aggressive harmonic distortion.

```rust
comp.track("lead")
    .distortion(Distortion::new(
        0.7,  // Drive (0.0-1.0)
        0.5   // Mix (0.0 = dry, 1.0 = fully distorted)
    ));
```

**Use cases:** Guitar-like distortion, aggressive synth sounds, lo-fi effects

### 8. Bitcrusher

Digital lo-fi effect - reduces bit depth and sample rate.

```rust
comp.track("drums")
    .bitcrusher(BitCrusher::new(
        8.0,      // Bit depth (16.0 = CD quality, 8.0 = lo-fi, 4.0 = very crunchy)
        0.5       // Mix (0.0-1.0)
    ));
```

**Use cases:** Retro video game sounds, lo-fi hip-hop, digital degradation

### 9. Reverb

Simulates acoustic spaces.

```rust
comp.track("vocal")
    .reverb(Reverb::new(
        0.4,  // Room size (0.0 = tiny, 1.0 = cathedral)
        0.5   // Damping (0.0 = bright, 1.0 = dark/muffled)
    ));
```

**Algorithm:** Freeverb (4 parallel comb filters + 2 all-pass filters)

**Use cases:** Add space and depth, ambient textures, create atmosphere

### 10. Delay

Echo/delay effect with feedback.

```rust
comp.track("guitar")
    .delay(Delay::new(
        0.375,  // Delay time (seconds) - 375ms
        0.4,    // Feedback (0.0-0.9, amount of repeats)
        0.5     // Mix (0.0 = dry, 1.0 = wet)
    ));
```

**Timing examples:**
- Slapback: 75-180ms, low feedback
- Rhythmic: Match tempo (e.g., eighth note = 0.25s at 120bpm)
- Ambient: 500ms+, high feedback

**Use cases:** Slapback echo, rhythmic delays, ambient soundscapes

### 11. Chorus

Thickening effect using modulated delays.

```rust
comp.track("pad")
    .chorus(Chorus::new(
        1.5,   // Rate (Hz, speed of modulation, try 0.5-3.0)
        5.0,   // Depth (ms, amount of pitch variation, try 2-10)
        0.5    // Mix
    ));
```

**How it works:** Creates detuned copies of the signal

**Use cases:** Thicken synths, classic 80s chorus, stereo width

### 12. Phaser

Sweeping notch filter effect.

```rust
comp.track("electric_piano")
    .phaser(Phaser::new(
        0.5,      // Rate (Hz, speed of sweep)
        0.7,      // Depth (0.0-1.0, intensity)
        0.5,      // Feedback (0.0-0.95, resonance)
        6,        // Stages (2-12, more = more notches)
        0.5       // Mix
    ));
```

**Use cases:** 70s electric piano, psychedelic sweeps, movement

### 13. Flanger

Extreme phasing with metallic character.

```rust
comp.track("synth")
    .flanger(Flanger::new(
        0.3,      // Rate (Hz)
        3.0,      // Depth (ms)
        0.7,      // Feedback (creates metallic resonance)
        0.5,      // Mix
        44100.0   // Sample rate
    ));
```

**Use cases:** Jet plane effect, metallic sweeps, experimental sounds

### 14. Ring Modulator

Metallic/robotic modulation effect.

```rust
comp.track("vocal")
    .ring_mod(RingModulator::new(
        200.0,  // Modulator frequency (Hz, try 50-1000)
        0.5     // Mix
    ));
```

**Use cases:** Robot voices, bell-like tones, alien sounds

### 15. Tremolo

Volume modulation (amplitude variation).

```rust
comp.track("guitar")
    .tremolo(Tremolo::new(
        4.0,  // Rate (Hz, 4.0 = 4 volume changes per second)
        0.5   // Depth (0.0-1.0, amount of volume variation)
    ));
```

**Use cases:** Classic guitar tremolo, rhythmic pulsing, texture

### 16. AutoPan

Automatic stereo panning.

```rust
comp.track("synth")
    .autopan(AutoPan::new(
        0.25  // Rate (Hz, 0.25 = one full L-R cycle every 4 seconds)
    ));
```

**Use cases:** Stereo movement, creating width, rhythmic panning

---

## Common Effect Chains

### Vocal Chain

```rust
comp.instrument("vocal", &Instrument::sine())
    .eq(EQ::new(0.8, 1.0, 1.1, 200.0, 4000.0))  // Cut lows, boost highs
    .compressor(Compressor::new(0.6, 3.0, 0.005, 0.08, 44100.0))  // Even dynamics
    .reverb(Reverb::new(0.3, 0.5))              // Add space
    .delay(Delay::new(0.375, 0.25, 0.15));      // Subtle delay
```

### Bass Chain

```rust
comp.instrument("bass", &Instrument::sub_bass())
    .compressor(Compressor::new(0.5, 4.0, 0.01, 0.1, 44100.0))  // Tight and consistent
    .saturation(Saturation::new(0.2))           // Add harmonics
    .eq(EQ::new(1.2, 1.0, 0.9, 250.0, 4000.0)); // Boost lows
```

### Mastering Chain

```rust
let mut mixer = comp.into_mixer();

// Professional mastering chain
mixer.master_parametric_eq(ParametricEQ::new()
    .band(60.0, -3.0, 0.7)      // Cut sub rumble
    .band(3000.0, 2.0, 1.5));   // Presence boost

mixer.master_compressor(Compressor::new(0.5, 2.0, 0.01, 0.1, 44100.0));  // Gentle glue
mixer.master_saturation(Saturation::new(0.05));                          // Analog warmth
mixer.master_limiter(Limiter::new(0.95));                                // Prevent clipping
```

### Drum Bus

```rust
if let Some(drums) = mixer.buses.get_mut("drums") {
    drums.effects.compressor = Some(Compressor::new(0.65, 4.0, 0.005, 0.05, 44100.0));  // Punch
    drums.effects.saturation = Some(Saturation::new(0.1));   // Warmth
    drums.effects.reverb = Some(Reverb::new(0.2, 0.4));     // Shared room
}
```

### Lo-Fi Effect

```rust
comp.track("melody")
    .bitcrusher(BitCrusher::new(8.0, 0.7))      // Digital degradation
    .saturation(Saturation::new(0.3))           // Analog warmth
    .eq(EQ::new(0.7, 1.0, 0.6, 300.0, 5000.0)); // Remove extremes
```

---

## Effect Automation

Many effects support parameter automation over time:

```rust
use tunes::synthesis::Automation;

let filter_sweep = Automation::new(vec![
    (0.0, 200.0),   // Start at 200Hz
    (2.0, 2000.0),  // Sweep to 2000Hz over 2 seconds
    (4.0, 200.0),   // Back down
]);

comp.track("bass")
    .filter(Filter::lowpass(200.0, 0.7, 44100.0)
        .with_cutoff_automation(filter_sweep));
```

**Supported on:**
- EQ (gain automation for each band)
- Compressor (threshold, ratio, attack, release)
- All modulation effects (rate, depth, mix)
- Delays (time, feedback, mix)

---

## Best Practices

### Effect Ordering

The automatic priority system handles most cases, but you can override priorities:

```rust
let mut compressor = Compressor::new(0.6, 3.0, 0.01, 0.1, 44100.0);
compressor.priority = 15;  // Run before default EQ (priority 10)
```

**General rules:**
1. **EQ first** - Shape tone before dynamics
2. **Dynamics second** - Compressor, gate, limiter
3. **Saturation/distortion** - Add harmonics
4. **Modulation** - Chorus, phaser, flanger
5. **Time-based last** - Delay and reverb

### CPU Management

**Use buses for shared effects:**
```rust
// BAD: Reverb on every track (10 reverb instances)
for i in 0..10 {
    comp.track(&format!("track{}", i))
        .reverb(Reverb::new(0.3, 0.5));
}

// GOOD: One reverb on the bus (1 reverb instance)
let mut mixer = comp.into_mixer();
if let Some(bus) = mixer.buses.get_mut("default") {
    bus.effects.reverb = Some(Reverb::new(0.3, 0.5));
}
```

### Mix Balance

**Typical mix levels:**
- Track effects: Moderate (reverb mix 0.2-0.4, delay mix 0.2-0.3)
- Bus effects: Shared processing (reverb mix 0.3-0.5)
- Master effects: Subtle (compression ratio 1.5-2.5, limiter at 0.95)

**Don't overdo it:** Start with small amounts and increase gradually.

---

## Effect Reference Table

| Effect | Type | Priority | Typical Use |
|--------|------|----------|-------------|
| EQ | Filter | 10 | Tone shaping |
| Parametric EQ | Filter | 10 | Surgical frequency control |
| Compressor | Dynamics | 60 | Even dynamics, punch |
| Gate | Dynamics | 55 | Noise removal |
| Limiter | Dynamics | 70 | Prevent clipping |
| Saturation | Harmonic | 120 | Analog warmth |
| Distortion | Harmonic | 110 | Aggressive overdrive |
| Bitcrusher | Harmonic | 105 | Lo-fi digital effect |
| Chorus | Modulation | 160 | Thickening, width |
| Phaser | Modulation | 165 | Sweeping notches |
| Flanger | Modulation | 170 | Jet plane effect |
| Ring Mod | Modulation | 155 | Metallic/robotic |
| Tremolo | Modulation | 175 | Volume modulation |
| AutoPan | Modulation | 180 | Stereo movement |
| Delay | Time-based | 210 | Echo, rhythmic repeats |
| Reverb | Time-based | 220 | Spatial depth |

---

**Next:** Learn about synthesis techniques in [Synthesis Basics](./basics.md) →
