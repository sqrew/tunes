# Algorithmic Composition with Sequences

Tunes provides a powerful collection of **sequence generators** for algorithmic composition. Instead of manually writing out every note, you can use mathematical patterns, chaos theory, cellular automata, and other algorithms to generate melodies, rhythms, and structures.

## Overview

The `sequences` module is organized into four categories:

1. **Mathematical Sequences** - Classic patterns like Fibonacci, primes, Collatz
2. **Rhythmic Patterns** - Euclidean rhythms, golden ratio rhythms, polyrhythms
3. **Generative Algorithms** - Chaos theory, random walks, L-systems, cellular automata
4. **Musical Transformations** - Map sequences to frequencies, scales, and ranges

All sequences live under `tunes::sequences` and can be imported with `use tunes::sequences;`

---

## Basic Workflow

The typical workflow for using sequences is:

1. **Generate** a numeric sequence
2. **Transform** it to musical parameters (frequencies, durations, rhythms)
3. **Use** it in your composition

### Example: Fibonacci Melody

```rust
use tunes::prelude::*;
use tunes::sequences;

let mut comp = Composition::new(Tempo::new(120.0));

// 1. Generate Fibonacci sequence: [1, 1, 2, 3, 5, 8, 13, 21]
let fib = sequences::fibonacci(8);

// 2. Transform to frequency range (200-800 Hz)
let melody = sequences::normalize(&fib, 200.0, 800.0);

// 3. Use as melody
comp.track("fibonacci")
    .notes(&melody, 0.25);
```

**What's happening:**
- `fibonacci(8)` generates 8 Fibonacci numbers: `[1, 1, 2, 3, 5, 8, 13, 21]`
- `normalize(&fib, 200.0, 800.0)` scales them proportionally to 200-800 Hz
- The smallest value (1) maps to 200 Hz, largest (21) maps to 800 Hz
- Result: A melody that follows Fibonacci growth but stays in a playable frequency range

---

## Mathematical Sequences

Classic number sequences that create interesting patterns.

### Fibonacci Sequence

**Pattern:** Each number is the sum of the previous two: 1, 1, 2, 3, 5, 8, 13, 21...

```rust
let fib = sequences::fibonacci(10);
// Result: [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
```

**Musical use:** Natural-sounding growth patterns, phrase lengths, rhythm densities.

### Prime Numbers

**Pattern:** Numbers divisible only by 1 and themselves: 2, 3, 5, 7, 11, 13, 17...

```rust
let primes = sequences::primes(10);
// Result: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]

let melody = sequences::normalize(&primes, 220.0, 880.0);
comp.track("primes").notes(&melody, 0.2);
```

**Musical use:** Irregular but deterministic patterns, non-repetitive rhythms.

### Collatz Sequence (3n+1 Problem)

**Pattern:** If even: divide by 2; if odd: multiply by 3 and add 1. Eventually reaches 1.

```rust
// Start at 27, generate up to 40 terms
let collatz = sequences::collatz(27, 40);
// Result: [27, 82, 41, 124, 62, 31, 94, 47, 142, 71, ...]

let melody = sequences::normalize(&collatz, 150.0, 700.0);
comp.track("collatz").notes(&melody, 0.15);
```

**Musical use:** Chaotic wandering melodies that eventually converge.

### Other Mathematical Sequences

```rust
// Arithmetic: a, a+d, a+2d, ... (linear progression)
let arithmetic = sequences::arithmetic(5, 3, 10);  // [5, 8, 11, 14, 17, 20, 23, 26, 29, 32]

// Geometric: a, ar, ar², ar³, ... (exponential growth)
let geometric = sequences::geometric(2, 2, 8);  // [2, 4, 8, 16, 32, 64, 128, 256]

// Triangular: 1, 3, 6, 10, 15, 21... (sum of integers)
let triangular = sequences::triangular(8);

// Powers of two: 1, 2, 4, 8, 16, 32...
let powers = sequences::powers_of_two(8);
```

---

## Rhythmic Patterns

### Euclidean Rhythms

Distribute `k` pulses as evenly as possible across `n` steps using Bjorklund's algorithm. This creates mathematically optimal rhythms used in music traditions worldwide.

```rust
// Returns step indices where hits occur
let kick = sequences::euclidean(4, 16);     // [0, 4, 8, 12] - Four-on-floor
let snare = sequences::euclidean(3, 16);    // [0, 5, 11] - Syncopated
let hihat = sequences::euclidean(7, 16);    // Complex pattern

comp.track("drums")
    .drum_grid(16, 0.125)
    .kick(&kick)
    .snare(&snare)
    .hihat(&hihat);
```

**Common patterns:**
- `euclidean(3, 8)` - Cuban tresillo
- `euclidean(5, 8)` - Cuban cinquillo
- `euclidean(5, 16)` - Bossa nova clave
- `euclidean(4, 16)` - Standard four-on-floor kick

**What's happening:** The algorithm spaces pulses as evenly as possible, creating the most balanced rhythm distribution mathematically.

### Golden Ratio Rhythm

Non-periodic rhythm based on the golden ratio (φ ≈ 1.618).

```rust
let phi_rhythm = sequences::golden_ratio_rhythm(32);
// Returns indices following golden ratio spacing

comp.track("phi_drums")
    .drum_grid(32, 0.125)
    .kick(&phi_rhythm);
```

**Musical use:** Never quite repeats, sounds organic and natural.

---

## Generative Algorithms

### Chaos Theory: Logistic Map

The logistic map demonstrates how simple equations can produce complex chaotic behavior:

**Formula:** `x(n+1) = r * x(n) * (1 - x(n))`

```rust
// r parameter controls behavior:
// r=2.5: Stable (converges to fixed point)
let stable = sequences::logistic_map(2.5, 0.5, 16);

// r=3.9: Chaotic (unpredictable but deterministic)
let chaotic = sequences::logistic_map(3.9, 0.5, 32);

// Convert to frequencies
let melody = sequences::normalize(
    &chaotic.iter().map(|&x| (x * 100.0) as u32).collect::<Vec<_>>(),
    200.0, 800.0
);
```

**Musical use:** Smoothly transition from calm (low r) to intense (high r) music by adjusting the `r` parameter based on game state or intensity.

### Random Walk (Brownian Motion)

Smooth, organic wandering patterns.

```rust
// Unbounded walk (can go anywhere)
let walk = sequences::random_walk(440.0, 20.0, 20);
comp.track("walk").notes(&walk, 0.25);

// Bounded walk (constrained to range)
let bounded = sequences::bounded_walk(440.0, 30.0, 220.0, 880.0, 32);
comp.track("bounded").notes(&bounded, 0.2);
```

**What's happening:** Each step moves up or down by a random amount (`step_size`), creating smooth melodic contours like a drunk person walking.

### Cellular Automaton

Generate patterns using rule-based evolution (like Conway's Game of Life but 1D).

```rust
// Rule 30 - chaotic patterns
let rule30 = sequences::cellular_automaton(30, 8, 16, None);
// Returns 8 generations, each with 16 cells (0 or 1)

for (gen_idx, generation) in rule30.iter().take(4).enumerate() {
    let rhythm: Vec<usize> = generation
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)  // Find cells with value 1
        .map(|(i, _)| i)            // Get their indices
        .collect();

    comp.track(&format!("ca_{}", gen_idx))
        .drum_grid(16, 0.125)
        .kick(&rhythm);
}
```

**What's happening:** Each generation evolves from the previous one using simple rules. Rule 30 creates chaotic patterns, while Rule 90 creates fractal Sierpinski triangles.

### Other Generative Algorithms

```rust
// Thue-Morse: Binary sequence avoiding repetition
let thue_morse = sequences::thue_morse(32);  // [0,1,1,0,1,0,0,1,...]

// Recamán: Back-and-forth spiraling
let recaman = sequences::recaman(24);

// Van der Corput: Quasi-random low-discrepancy
let quasi = sequences::van_der_corput(32, 2);

// Tent Map: Simple chaotic map
let tent = sequences::tent_map(0.9, 0.5, 32);

// Sine Map: Musical chaotic sequences
let sine = sequences::sine_map(0.9, 0.5, 32);

// Hénon Map: 2D chaotic attractor
let (x_vals, y_vals) = sequences::henon_x(1.4, 0.3, 0.1, 0.1, 100);
```

---

## Musical Transformations

### Normalize: Map to Ranges

Convert any sequence to a frequency, duration, or parameter range.

```rust
let seq = sequences::fibonacci(8);

// Map to frequency range (melody)
let melody = sequences::normalize(&seq, 220.0, 880.0);

// Map to note durations (rhythm)
let durations = sequences::normalize(&seq, 0.125, 1.0);

// Map to volume levels
let volumes = sequences::normalize(&seq, 0.3, 0.9);
```

**Formula:** Linear min-max scaling preserving proportions.

### Map to Scale: Quantize to Musical Keys

Convert sequences to notes in a specific musical scale.

```rust
let fib = sequences::fibonacci(16);

// Map to C major pentatonic, spanning 2 octaves
let melody = sequences::map_to_scale(&fib, &sequences::Scale::major_pentatonic(), C4, 2);

// Use directly - returns frequencies!
comp.track("scale_melody").notes(&melody, 0.25);
```

**Available scales:**
- `Scale::major()` - C D E F G A B
- `Scale::minor()` - C D Eb F G Ab Bb
- `Scale::major_pentatonic()` - C D E G A
- `Scale::minor_pentatonic()` - C Eb F G Bb
- `Scale::blues()` - C Eb F F# G Bb
- `Scale::harmonic_minor()` - C D Eb F G Ab B
- `Scale::chromatic()` - All 12 semitones
- `Scale::whole_tone()` - C D E F# G# A#
- `Scale::dorian()`, `Scale::phrygian()`, `Scale::lydian()`, `Scale::mixolydian()`

**For continuous (f32) sequences:**

```rust
// Chaos theory, Perlin noise, Lorenz attractor, etc.
let chaos = sequences::logistic_map(3.9, 0.5, 32);
let melody = sequences::map_to_scale_f32(&chaos, &sequences::Scale::minor(), D4, 2);
```

### Harmonic Series

Generate overtone frequencies - the foundation of musical timbre.

```rust
let harmonics = sequences::harmonic_series(110.0, 12);
// Result: [110, 220, 330, 440, 550, 660, 770, 880, 990, 1100, 1210, 1320]
// Formula: f, 2f, 3f, 4f, 5f, ...

// Use for spectral chords
comp.track("spectral")
    .note(&harmonics[3..6], 2.0);  // Harmonics 4-6 form a major triad
```

**Musical use:** Spectral music, overtone-based harmony, natural timbre.

### Golden Ratio

Powers of φ (phi ≈ 1.618) for natural proportions.

```rust
let phi = sequences::golden_ratio(8);
// Result: [1.0, 1.618, 2.618, 4.236, 6.854, ...]

// Golden sections: divide values recursively
let sections = sequences::golden_sections(800.0, 6);
// Result: [800.0, 494.4, 305.6, 188.9, 116.7, 72.1]
```

---

## Complete Example: Generative Composition

Combining multiple sequences for a full algorithmic piece:

```rust
use tunes::prelude::*;
use tunes::sequences;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // === BASS: Recamán sequence (interesting contour) ===
    let recaman = sequences::recaman(16);
    let bass_freqs = sequences::normalize(&recaman, 55.0, 110.0);

    comp.instrument("bass", &Instrument::sub_bass())
        .notes(&bass_freqs, 0.5);

    // === MELODY: Chaotic but in-scale ===
    let chaos = sequences::logistic_map(3.7, 0.5, 32);
    let melody = sequences::map_to_scale_f32(
        &chaos,
        &sequences::Scale::minor_pentatonic(),
        C5,
        2
    );

    comp.instrument("lead", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .notes(&melody, 0.25);

    // === CHORDS: Harmonic series ===
    let harmonics = sequences::harmonic_series(82.41, 12);  // E2

    comp.instrument("pad", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .note(&harmonics[3..6], 4.0)    // Major triad
        .note(&harmonics[4..7], 4.0);

    // === DRUMS: Euclidean + Thue-Morse ===
    let thue_morse = sequences::thue_morse(16);
    let tm_hits: Vec<usize> = thue_morse
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();

    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&sequences::euclidean(4, 16))  // Four-on-floor
        .snare(&tm_hits)                     // Non-repetitive
        .hihat(&sequences::euclidean(7, 16));// Complex pattern

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

**What's happening:**
1. **Bass** follows Recamán's spiraling pattern
2. **Melody** uses chaotic logistic map but quantized to minor pentatonic scale
3. **Chords** use natural harmonic series for pure intervals
4. **Drums** combine Euclidean (even distribution) with Thue-Morse (non-repetitive)

---

## Tips and Best Practices

### 1. Always Normalize or Map to Scale

Raw sequences like Fibonacci produce unusable frequency values (e.g., 13 Hz is too low). Always transform them:

```rust
// ❌ BAD: Raw Fibonacci as frequencies
let fib = sequences::fibonacci(8);
comp.track("bad").notes(&fib.iter().map(|&x| x as f32).collect::<Vec<_>>(), 0.25);

// ✅ GOOD: Normalized to playable range
let melody = sequences::normalize(&fib, 220.0, 880.0);
comp.track("good").notes(&melody, 0.25);

// ✅ GOOD: Quantized to musical scale
let scale_melody = sequences::map_to_scale(&fib, &sequences::Scale::major(), C4, 2);
comp.track("better").notes(&scale_melody, 0.25);
```

### 2. Use Euclidean Rhythms for Drums

Euclidean rhythms are perfect for drum patterns because they're mathematically optimal:

```rust
comp.track("drums")
    .drum_grid(16, 0.125)
    .kick(&sequences::euclidean(4, 16))    // Even kick
    .snare(&sequences::euclidean(3, 16))   // Syncopated snare
    .hihat(&sequences::euclidean(7, 16));  // Complex hi-hat
```

### 3. Combine Sequences for Complexity

Layer different sequences for rich patterns:

```rust
// Bass: Slow-moving Fibonacci
let fib_bass = sequences::normalize(&sequences::fibonacci(8), 55.0, 110.0);

// Melody: Fast chaotic pattern in-scale
let chaos = sequences::logistic_map(3.9, 0.5, 32);
let chaos_melody = sequences::map_to_scale_f32(&chaos, &sequences::Scale::minor(), C5, 2);

// Rhythm: Euclidean with cellular automaton variation
let base_rhythm = sequences::euclidean(5, 16);
let ca_variation = sequences::cellular_automaton(30, 4, 16, None);
```

### 4. Use Chaos Theory for Dynamic Intensity

Map game state or intensity to the `r` parameter in logistic map:

```rust
fn generate_melody_for_intensity(intensity: f32) -> Vec<f32> {
    // intensity: 0.0 (calm) to 1.0 (chaotic)
    let r = 2.5 + intensity * 1.5;  // r ranges from 2.5 (stable) to 4.0 (chaos)
    let chaos = sequences::logistic_map(r, 0.5, 32);
    sequences::normalize(
        &chaos.iter().map(|&x| (x * 100.0) as u32).collect::<Vec<_>>(),
        220.0,
        880.0
    )
}
```

### 5. Explore All Categories

Don't just stick to one type - combine mathematical, rhythmic, and generative sequences:

- **Structure:** Fibonacci for phrase lengths
- **Melody:** Chaotic patterns mapped to scale
- **Harmony:** Harmonic series for chords
- **Rhythm:** Euclidean patterns
- **Variation:** Cellular automaton for evolving patterns

---

## Full Sequence Reference

### Mathematical
- `fibonacci(n)` - Fibonacci sequence
- `primes(n)` - Prime numbers
- `arithmetic(start, step, n)` - Linear progression
- `geometric(start, ratio, n)` - Exponential growth
- `triangular(n)` - Triangular numbers
- `powers_of_two(n)` - Powers of 2
- `collatz(start, max)` - 3n+1 problem
- `lucas(n)`, `catalan(n)`, `padovan(n)`, `pell(n)`, `pentagonal(n)` - Other sequences

### Rhythmic
- `euclidean(pulses, steps)` - Optimal beat distribution
- `euclidean_pattern(pulses, steps)` - Full binary pattern
- `golden_ratio_rhythm(steps)` - Non-periodic rhythm
- `polyrhythm(a, b, cycles)` - Layered rhythms
- `son_clave_3_2()`, `rumba_clave_3_2()`, `bossa_clave()` - Traditional claves

### Generative
- `logistic_map(r, initial, n)` - Chaos theory
- `random_walk(start, step, n)` - Brownian motion
- `bounded_walk(start, step, min, max, n)` - Constrained walk
- `tent_map(r, initial, n)` - Simple chaotic map
- `sine_map(r, initial, n)` - Musical chaotic sequences
- `henon_map(a, b, x0, y0, n)` - 2D attractor
- `thue_morse(n)` - Fair binary sequences
- `recaman(n)` - Spiraling back-and-forth
- `van_der_corput(n, base)` - Quasi-random
- `cellular_automaton(rule, gens, width, initial)` - Rule-based evolution
- `cantor_set(depth, steps)` - Fractal rhythms
- `lorenz_butterfly(n)` - 3D chaotic attractor
- `perlin_noise(seed, freq, octaves, persistence, n)` - Smooth noise

### Musical Transformations
- `normalize(seq, min, max)` - Map to range
- `normalize_f32(seq, min, max)` - Map f32 sequence
- `map_to_scale(seq, scale, root, octaves)` - Quantize to scale
- `map_to_scale_f32(seq, scale, root, octaves)` - Quantize f32 to scale
- `harmonic_series(fundamental, n)` - Overtone frequencies
- `undertone_series(fundamental, n)` - Mirror of harmonics
- `golden_ratio(n)` - Powers of φ
- `golden_sections(value, divisions)` - Divide by φ recursively
- `circle_of_fifths(root, n)` - Key relationships
- `pythagorean_tuning(root, n)` - Pure fifth tuning
- `just_intonation_major(root)` - Pure harmonic ratios

---

**Next:** Explore [MIDI Import/Export](./midi.md) to bring external MIDI files into your algorithmic compositions →
