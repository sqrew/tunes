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
    .reverb(Reverb::new(0.2, 0.4, 0.3))      // Add space
    .compressor(Compressor::new(0.6, 3.0, 0.005, 0.05, 1.2))  // Tighten dynamics
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
    .compressor(Compressor::new(0.65, 4.0, 0.01, 0.08, 1.1))
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

mixer.master_compressor(Compressor::new(0.55, 2.5, 0.01, 0.12, 1.0));  // Glue
mixer.master_saturation(Saturation::new(1.5, 0.15, 0.3));              // Warmth
mixer.master_limiter(Limiter::new(0.95, 0.05));                        // Protection
```

**Use cases:**
- Mastering: Final polish on complete mix
- Mix glue: Subtle compression to unify tracks
- Protection: Limiting to prevent clipping
- Creative: Tape saturation, lo-fi effects

---

## Two Ways to Apply Effects

Similar to generators and transformations, Tunes provides two syntaxes for applying effects: direct method calls and a cleaner namespaced API using `.effects()`.

### Direct Method Calls (Classic Syntax)

You can call effect methods directly on the track builder:

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

comp.track("direct")
    .filter(Filter::low_pass(1000.0, 0.7))     // Direct call
    .reverb(Reverb::new(0.8, 0.5, 0.3))        // Direct call
    .delay(Delay::new(0.25, 0.4, 0.5));        // Direct call
```

**Pros:**
- Familiar if you're used to the original API
- Slightly more concise for single effects

**Cons:**
- Clutters autocomplete with 17+ effect methods
- Less organized when applying multiple effects

### Effects Namespace (New Syntax)

Encapsulate effects in an `.effects()` closure for better organization:

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

comp.track("organized")
    .notes(&[C4, E4, G4], 0.5)
    .effects(|e| e  // Enter effects namespace
        .filter(Filter::low_pass(1000.0, 0.7))
        .reverb(Reverb::new(0.8, 0.5, 0.3))
        .delay(Delay::new(0.25, 0.4, 0.5))
    );              // Automatically exits namespace
```

**Pros:**
- Cleaner autocomplete - effects only appear inside `.effects()`
- Better organization - visual grouping of related processing
- More readable - clear boundaries for effect processing
- Easy to chain with `.generator()` and `.transform()`

**Cons:**
- Slightly more verbose for single effects

**Both syntaxes work and are fully compatible!** Choose whichever fits your workflow.

### Complete Example with All Three Namespaces

Here's how generators, transformations, and effects work together:

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

comp.track("complete")
    // 1. Generate musical patterns
    .generator(|g| g
        .chord(C4, &ChordPattern::MAJOR, 0.5)
        .arpeggiate(&[C5, E5, G5, C6], 0.125)
    )
    // 2. Transform the patterns
    .transform(|t| t
        .shift(7)
        .humanize(0.01, 0.05)
    )
    // 3. Apply effects
    .effects(|e| e
        .filter(Filter::low_pass(2000.0, 0.7))
        .compressor(Compressor::new(0.6, 3.0, 0.01, 0.1, 1.3))
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .delay(Delay::new(0.375, 0.3, 0.4))
    );
```

**Workflow:**
1. **Generate** musical material (chords, scales, arpeggios)
2. **Transform** the material (transpose, humanize, etc.)
3. **Apply effects** (filters, reverb, delay, etc.)

This creates a clear, organized signal flow that's easy to read and modify.

### Multiple Effect Blocks

You can use multiple `.effects()` blocks for organization:

```rust
comp.track("layered")
    .notes(&[C4, E4, G4], 0.5)
    .effects(|e| e  // Filtering and dynamics
        .filter(Filter::band_pass(1000.0, 0.7))
        .compressor(Compressor::new(0.6, 3.0, 0.01, 0.1, 1.2))
        .saturation(Saturation::new(1.5, 0.3, 0.5))
    )
    .effects(|e| e  // Modulation
        .chorus(Chorus::new(1.0, 5.0, 0.4))
        .phaser(Phaser::new(0.5, 0.7, 0.5, 4, 0.5))
    )
    .effects(|e| e  // Time-based effects
        .delay(Delay::new(0.375, 0.4, 0.5))
        .reverb(Reverb::new(0.6, 0.5, 0.4))
    );
```

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
        0.5,      // Threshold (0.0-1.0 amplitude, lower = more compression)
        4.0,      // Ratio (2.0 = gentle, 4.0 = medium, 10.0 = aggressive)
        0.01,     // Attack time (seconds, fast = 0.001, slow = 0.1)
        0.1,      // Release time (seconds)
        1.5       // Makeup gain (1.0 = unity, higher = louder output)
    ));
```

**Parameters:**
- **Threshold**: 0.0-1.0 amplitude (0.3 = aggressive, 0.5 = moderate, 0.7 = gentle)
- **Ratio**: Compression amount (2.0 = gentle, 4.0 = medium, 10.0 = limiting)
- **Attack**: How quickly compression kicks in (fast = snappy, slow = natural)
- **Release**: How quickly compression lets go
- **Makeup gain**: Output boost to compensate for compression (1.0-3.0 typical)

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

#### Stereo-Linked Compression

When processing stereo signals (buses and master), the compressor uses **stereo-linked** detection to prevent stereo image shifts. This is a professional technique used in all high-quality mixing software.

**How it works:**
```
1. Detect peak from MAX(left, right) - use the louder channel
2. Calculate gain reduction once based on this peak
3. Apply the SAME gain to both left and right channels
```

**Why this matters:**
- **Without linking**: Left and right channels compress independently, causing the stereo image to "wobble" or shift when compression kicks in
- **With linking**: Both channels get identical gain reduction, preserving the stereo image perfectly

**Automatic behavior:**
- **Track-level effects**: Process mono signal (no linking needed)
- **Bus-level effects**: Automatically stereo-linked
- **Master-level effects**: Automatically stereo-linked

```rust
// This compressor on a bus automatically uses stereo linking
mixer.bus("drums")
    .compressor(Compressor::new(0.65, 4.0, 0.005, 0.05, 1.2));
    // Detects from max(L, R), applies same gain to both channels
```

**Technical details:**
The implementation uses `process_stereo_linked()` internally for all bus and master compressors, ensuring professional-grade stereo image preservation. This is especially important for:
- Master bus compression (mix glue)
- Drum bus compression (maintains stereo width of overheads/rooms)
- Sidechained compression (EDM pump stays centered)

### 4. Gate

Noise gate - silences audio below a threshold.

```rust
comp.track("guitar")
    .gate(Gate::new(
        0.05,     // Threshold (signals below this are silenced)
        0.002,    // Attack (how fast gate opens)
        0.05      // Release (how fast gate closes)
    ));
```

**Use cases:** Remove background noise, clean up recordings, create rhythmic effects

### 5. Limiter

Brick-wall limiting - prevents audio from exceeding a ceiling.

```rust
// Prevent clipping on master
mixer.master_limiter(Limiter::new(
    0.95,   // Threshold in dB (max level = 0.95 = -0.4dB)
    0.05    // Release time in seconds
));
```

**Parameters:**
- **Threshold**: Maximum output level (0.95 = safe headroom, 1.0 = absolute max)
- **Release**: How quickly limiter recovers (0.05 = fast, 0.2 = slow)

**Stereo-Linked Limiting:**
Like the compressor, the limiter uses stereo-linked detection on buses and master. It detects peaks from max(left, right) and applies identical gain reduction to both channels, preventing stereo image distortion during limiting.

**Use cases:** Prevent digital clipping, maximize loudness, protect speakers

### 6. Saturation

Analog-style harmonic saturation.

```rust
comp.track("synth")
    .saturation(Saturation::new(
        1.5,   // Drive (1.0 = unity, higher = more saturation)
        0.3,   // Character (0.0-1.0, harmonic coloration)
        0.5    // Mix (0.0 = dry, 1.0 = fully saturated)
    ));
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
        0.5       // Mix
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
    .compressor(Compressor::new(0.6, 3.0, 0.005, 0.08, 1.3))  // Even dynamics
    .reverb(Reverb::new(0.3, 0.5, 0.3))         // Add space
    .delay(Delay::new(0.375, 0.25, 0.15));      // Subtle delay
```

### Bass Chain

```rust
comp.instrument("bass", &Instrument::sub_bass())
    .compressor(Compressor::new(0.5, 4.0, 0.01, 0.1, 1.4))  // Tight and consistent
    .saturation(Saturation::new(1.3, 0.2, 0.4))  // Add harmonics
    .eq(EQ::new(1.2, 1.0, 0.9, 250.0, 4000.0));  // Boost lows
```

### Mastering Chain

```rust
let mut mixer = comp.into_mixer();

// Professional mastering chain
mixer.master_parametric_eq(ParametricEQ::new()
    .band(60.0, -3.0, 0.7)      // Cut sub rumble
    .band(3000.0, 2.0, 1.5));   // Presence boost

mixer.master_compressor(Compressor::new(0.55, 2.5, 0.01, 0.12, 1.0));  // Gentle glue
mixer.master_saturation(Saturation::new(1.2, 0.1, 0.2));               // Analog warmth
mixer.master_limiter(Limiter::new(0.95, 0.05));                        // Prevent clipping
```

### Drum Bus

```rust
// Using the bus builder API (preferred)
mixer.bus("drums")
    .compressor(Compressor::new(0.65, 4.0, 0.005, 0.05, 1.2))  // Punch
    .saturation(Saturation::new(1.3, 0.15, 0.3))               // Warmth
    .reverb(Reverb::new(0.2, 0.4, 0.3));                       // Shared room
```

### Lo-Fi Effect

```rust
comp.track("melody")
    .bitcrusher(BitCrusher::new(8.0, 0.7))         // Digital degradation
    .saturation(Saturation::new(1.4, 0.3, 0.5))    // Analog warmth
    .eq(EQ::new(0.7, 1.0, 0.6, 300.0, 5000.0));    // Remove extremes
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
let compressor = Compressor::new(0.6, 3.0, 0.01, 0.1, 1.2)
    .with_priority(15);  // Run before default EQ (priority 10)
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
