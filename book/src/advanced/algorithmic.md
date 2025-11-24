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
let fib = sequences::fibonacci::generate(8);

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
let fib = sequences::fibonacci::generate(10);
// Result: [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
```

**Musical use:** Natural-sounding growth patterns, phrase lengths, rhythm densities.

### Prime Numbers

**Pattern:** Numbers divisible only by 1 and themselves: 2, 3, 5, 7, 11, 13, 17...

```rust
let primes = sequences::primes::generate(10);
// Result: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]

let melody = sequences::normalize(&primes, 220.0, 880.0);
comp.track("primes").notes(&melody, 0.2);
```

**Musical use:** Irregular but deterministic patterns, non-repetitive rhythms.

### Collatz Sequence (3n+1 Problem)

**Pattern:** If even: divide by 2; if odd: multiply by 3 and add 1. Eventually reaches 1.

```rust
// Start at 27, generate up to 40 terms
let collatz = sequences::collatz::generate(27, 40);
// Result: [27, 82, 41, 124, 62, 31, 94, 47, 142, 71, ...]

let melody = sequences::normalize(&collatz, 150.0, 700.0);
comp.track("collatz").notes(&melody, 0.15);
```

**Musical use:** Chaotic wandering melodies that eventually converge.

### Other Mathematical Sequences

```rust
// Arithmetic: a, a+d, a+2d, ... (linear progression)
let arithmetic = sequences::arithmetic::generate(5, 3, 10);  // [5, 8, 11, 14, 17, 20, 23, 26, 29, 32]

// Geometric: a, ar, ar², ar³, ... (exponential growth)
let geometric = sequences::geometric::generate(2, 2, 8);  // [2, 4, 8, 16, 32, 64, 128, 256]

// Triangular: 1, 3, 6, 10, 15, 21... (sum of integers)
let triangular = sequences::triangular::generate(8);

// Powers of two: 1, 2, 4, 8, 16, 32...
let powers = sequences::powers_of_two::generate(8);
```

---

## Rhythmic Patterns

### Euclidean Rhythms

Distribute `k` pulses as evenly as possible across `n` steps using Bjorklund's algorithm. This creates mathematically optimal rhythms used in music traditions worldwide.

```rust
// Returns step indices where hits occur
let kick = sequences::euclidean::generate(4, 16);     // [0, 4, 8, 12] - Four-on-floor
let snare = sequences::euclidean::generate(3, 16);    // [0, 5, 11] - Syncopated
let hihat = sequences::euclidean::generate(7, 16);    // Complex pattern

comp.track("drums")
    .drum_grid(16, 0.125)
    .kick(&kick)
    .snare(&snare)
    .hihat(&hihat);
```

**Common patterns:**
- `euclidean::generate(3, 8)` - Cuban tresillo
- `euclidean::generate(5, 8)` - Cuban cinquillo
- `euclidean::generate(5, 16)` - Bossa nova clave
- `euclidean::generate(4, 16)` - Standard four-on-floor kick

**What's happening:** The algorithm spaces pulses as evenly as possible, creating the most balanced rhythm distribution mathematically.

### Golden Ratio Rhythm

Non-periodic rhythm based on the golden ratio (φ ≈ 1.618).

```rust
let phi_rhythm = sequences::golden_ratio_rhythm::generate(32);
// Returns indices following golden ratio spacing

comp.track("phi_drums")
    .drum_grid(32, 0.125)
    .kick(&phi_rhythm);
```

**Musical use:** Never quite repeats, sounds organic and natural.

### Shepard Tone Rhythm

Creates the illusion of an infinitely rising or falling rhythm.

```rust
let shepard = sequences::shepard_tone::generate(16, 4);
// Returns rhythm pattern with perceived ascending/descending quality

comp.track("shepard_drums")
    .drum_grid(16, 0.125)
    .kick(&shepard);
```

**Musical use:** Hypnotic, gradually intensifying rhythmic patterns.

### Circle Map

Uses the circle map equation to generate quasi-periodic rhythms based on rotation numbers.

```rust
// Generate circle map sequence
let circle = sequences::circle_map::generate(0.5, 0.2, 0.0, 32);

// Convert to hit indices
let hits = sequences::circle_map_to_hits(&circle, 16);

// Or create hocket pattern (complementary rhythms)
let (rhythm_a, rhythm_b) = sequences::circle_map_hocket(&circle, 16);

comp.track("circle_drums")
    .drum_grid(16, 0.125)
    .kick(&hits);
```

**Musical use:** Complex rhythmic patterns that hover between periodic and chaotic.

### Additive Meter

Traditional rhythms based on additive groupings (like 2+2+3 or 3+3+2).

```rust
// Create custom additive meter: groups of [2, 2, 3]
let meter = sequences::additive_meter::generate(&[2, 2, 3]);

// Traditional Bulgarian rhythms:
let rachenitsa = sequences::rachenitsa();        // 2+2+3 (7/8)
let kopanitsa = sequences::kopanitsa();          // 2+2+2+3 (9/8)
let kalamatianos = sequences::kalamatianos();    // 3+2+2 (7/8)
let aksak = sequences::aksak_9_8();              // 2+2+2+3 (9/8)

comp.track("folk_drums")
    .drum_grid(7, 0.125)
    .kick(&rachenitsa);
```

**Musical use:** Folk rhythms from Balkans, Turkey, and other traditions.

### Phase Shifting

Steve Reich-style phasing where rhythms gradually shift out of sync.

```rust
// Create base pattern
let base_pattern = sequences::euclidean::generate(5, 12);

// Shift by 1 step
let shifted = sequences::phase_shift_by(&base_pattern, 1, 12);

// Or use timed phase shifting (for gradual phasing effect)
let phase_states = sequences::phase_shift_timed(&base_pattern, 12, 8);

// Classic "Clapping Music" pattern
let (a, b) = sequences::clapping_music();
```

**Musical use:** Minimalist techniques, creating complex patterns from simple material.

---

## Generative Algorithms

### Chaos Theory: Logistic Map

The logistic map demonstrates how simple equations can produce complex chaotic behavior:

**Formula:** `x(n+1) = r * x(n) * (1 - x(n))`

```rust
// r parameter controls behavior:
// r=2.5: Stable (converges to fixed point)
let stable = sequences::logistic_map::generate(2.5, 0.5, 16);

// r=3.9: Chaotic (unpredictable but deterministic)
let chaotic = sequences::logistic_map::generate(3.9, 0.5, 32);

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
let walk = sequences::random_walk::generate(440.0, 20.0, 20);
comp.track("walk").notes(&walk, 0.25);

// Bounded walk (constrained to range)
let bounded = sequences::bounded_walk::generate(440.0, 30.0, 220.0, 880.0, 32);
comp.track("bounded").notes(&bounded, 0.2);
```

**What's happening:** Each step moves up or down by a random amount (`step_size`), creating smooth melodic contours like a drunk person walking.

### Cellular Automaton

Generate patterns using rule-based evolution (like Conway's Game of Life but 1D).

```rust
// Rule 30 - chaotic patterns
let rule30 = sequences::cellular_automaton::generate(30, 8, 16, None);
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
let thue_morse = sequences::thue_morse::generate(32);  // [0,1,1,0,1,0,0,1,...]

// Recamán: Back-and-forth spiraling
let recaman = sequences::recaman::generate(24);

// Van der Corput: Quasi-random low-discrepancy
let quasi = sequences::van_der_corput::generate(32, 2);

// Tent Map: Simple chaotic map
let tent = sequences::tent_map::generate(0.9, 0.5, 32);

// Sine Map: Musical chaotic sequences
let sine = sequences::sine_map::generate(0.9, 0.5, 32);

// Hénon Map: 2D chaotic attractor
let henon = sequences::henon_map::generate(1.4, 0.3, 0.1, 0.1, 100);
let (x_vals, y_vals) = sequences::henon_x(1.4, 0.3, 0.1, 0.1, 100);

// Baker's Map: Fractal mixing and distribution
let bakers = sequences::bakers_map::generate(0.3, 0.7, 100);
let (x_vals, y_vals) = sequences::bakers_x(0.3, 0.7, 100);

// Rössler Attractor: 3D chaotic system with spiral structure
let rossler = sequences::rossler_attractor::generate(0.2, 0.2, 5.7, 0.1, 0.1, 0.1, 1000);
let spiral = sequences::rossler_spiral(0.2, 0.2, 5.7, 0.1, 0.1, 0.1, 1000);

// Clifford Attractor: Strange attractor with flowing patterns
let clifford = sequences::clifford_attractor::generate(-1.4, 1.6, 1.0, -0.7, 0.0, 0.0, 1000);
let (x_vals, y_vals) = sequences::clifford_x(-1.4, 1.6, 1.0, -0.7, 0.0, 0.0, 1000);
let flow = sequences::clifford_flow(-1.4, 1.6, 1.0, -0.7, 0.0, 0.0, 1000);

// Ikeda Map: Complex dynamics from laser physics
let ikeda = sequences::ikeda_map::generate(0.9, 0.4, 6.0, 0.85, 0.85, 1000);
let (x_vals, y_vals) = sequences::ikeda_x(0.9, 0.4, 6.0, 0.85, 0.85, 1000);
let spiral = sequences::ikeda_spiral(0.9, 0.4, 6.0, 0.85, 0.85, 1000);
```

### L-Systems

Generate sequences using Lindenmayer systems (fractal growth patterns).

```rust
// Define an L-system with rewriting rules
let axiom = "A".to_string();
let rules = vec![
    ('A', "AB".to_string()),
    ('B', "A".to_string()),
];

// Generate iterations
let lsystem = sequences::lsystem::generate(&axiom, &rules, 6);

// Convert L-system string to numeric sequence
let sequence = sequences::lsystem_to_sequence(&lsystem);
```

**Musical use:** Fractal melodies and self-similar structures.

### Markov Chains

Generate probabilistic sequences learned from existing patterns.

```rust
// Learn from example melody
let training_data = vec![60, 62, 64, 62, 60, 64, 65, 67];

// Build transition probabilities
let transitions = sequences::build_markov_transitions(&training_data, 1);

// Generate new sequence following learned patterns
let markov_melody = sequences::markov::generate(&transitions, 60, 16);
```

**Musical use:** Style imitation, probabilistic variations, AI-assisted composition.

---

## Musical Transformations

### Normalize: Map to Ranges

Convert any sequence to a frequency, duration, or parameter range.

```rust
let seq = sequences::fibonacci::generate(8);

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
let fib = sequences::fibonacci::generate(16);

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
let chaos = sequences::logistic_map::generate(3.9, 0.5, 32);
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
    let recaman = sequences::recaman::generate(16);
    let bass_freqs = sequences::normalize(&recaman, 55.0, 110.0);

    comp.instrument("bass", &Instrument::sub_bass())
        .notes(&bass_freqs, 0.5);

    // === MELODY: Chaotic but in-scale ===
    let chaos = sequences::logistic_map::generate(3.7, 0.5, 32);
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
    let thue_morse = sequences::thue_morse::generate(16);
    let tm_hits: Vec<usize> = thue_morse
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();

    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&sequences::euclidean::generate(4, 16))  // Four-on-floor
        .snare(&tm_hits)                     // Non-repetitive
        .hihat(&sequences::euclidean::generate(7, 16));// Complex pattern

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
let fib = sequences::fibonacci::generate(8);
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
    .kick(&sequences::euclidean::generate(4, 16))    // Even kick
    .snare(&sequences::euclidean::generate(3, 16))   // Syncopated snare
    .hihat(&sequences::euclidean::generate(7, 16));  // Complex hi-hat
```

### 3. Combine Sequences for Complexity

Layer different sequences for rich patterns:

```rust
// Bass: Slow-moving Fibonacci
let fib_bass = sequences::normalize(&sequences::fibonacci::generate(8), 55.0, 110.0);

// Melody: Fast chaotic pattern in-scale
let chaos = sequences::logistic_map::generate(3.9, 0.5, 32);
let chaos_melody = sequences::map_to_scale_f32(&chaos, &sequences::Scale::minor(), C5, 2);

// Rhythm: Euclidean with cellular automaton variation
let base_rhythm = sequences::euclidean::generate(5, 16);
let ca_variation = sequences::cellular_automaton::generate(30, 4, 16, None);
```

### 4. Use Chaos Theory for Dynamic Intensity

Map game state or intensity to the `r` parameter in logistic map:

```rust
fn generate_melody_for_intensity(intensity: f32) -> Vec<f32> {
    // intensity: 0.0 (calm) to 1.0 (chaotic)
    let r = 2.5 + intensity * 1.5;  // r ranges from 2.5 (stable) to 4.0 (chaos)
    let chaos = sequences::logistic_map::generate(r, 0.5, 32);
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
- `fibonacci::generate(n)` - Fibonacci sequence
- `primes::generate(n)` - Prime numbers
- `arithmetic::generate(start, step, n)` - Linear progression
- `geometric::generate(start, ratio, n)` - Exponential growth
- `triangular::generate(n)` - Triangular numbers
- `powers_of_two::generate(n)` - Powers of 2
- `collatz::generate(start, max)` - 3n+1 problem
- `lucas::generate(n)`, `catalan::generate(n)`, `padovan::generate(n)`, `pell::generate(n)`, `pentagonal::generate(n)` - Other sequences

### Rhythmic
- `euclidean::generate(pulses, steps)` - Optimal beat distribution
- `euclidean::pattern(pulses, steps)` - Full binary pattern
- `golden_ratio_rhythm::generate(steps)` - Non-periodic rhythm
- `shepard_tone::generate(steps, layers)` - Infinitely rising/falling rhythm illusion
- `circle_map::generate(omega, k, theta, n)` - Quasi-periodic rhythms
- `circle_map_to_hits(seq, steps)`, `circle_map_hocket(seq, steps)` - Convert circle map to rhythms
- `polyrhythm::generate(a, b, cycles)` - Layered rhythms
- `polyrhythm_cycle(a, b)`, `polyrhythm_timings(a, b, cycles)` - Polyrhythm helpers
- `son_clave_3_2()`, `son_clave_2_3()`, `rumba_clave_3_2()`, `rumba_clave_2_3()`, `bossa_clave()` - Traditional claves
- `additive_meter::generate(groups)` - Custom additive groupings
- `rachenitsa()`, `kopanitsa()`, `kalamatianos()`, `aksak_9_8()` - Traditional folk rhythms
- `phase_shift_by(pattern, shift, steps)` - Shift rhythm by n steps
- `phase_shift_timed(pattern, steps, phases)` - Gradual phase shifting
- `phase_relationship(a, b)`, `clapping_music()` - Phase relationship helpers

### Generative
- `logistic_map::generate(r, initial, n)` - Chaos theory
- `random_walk::generate(start, step, n)` - Brownian motion
- `bounded_walk::generate(start, step, min, max, n)` - Constrained walk
- `tent_map::generate(r, initial, n)` - Simple chaotic map
- `sine_map::generate(r, initial, n)` - Musical chaotic sequences
- `henon_map::generate(a, b, x0, y0, n)` - 2D attractor
- `henon_x(a, b, x0, y0, n)`, `henon_y(a, b, x0, y0, n)` - Extract x/y coordinates
- `bakers_map::generate(x0, y0, n)` - Fractal mixing and distribution
- `bakers_x(x0, y0, n)`, `bakers_y(x0, y0, n)` - Extract x/y coordinates
- `thue_morse::generate(n)` - Fair binary sequences
- `recaman::generate(n)` - Spiraling back-and-forth
- `van_der_corput::generate(n, base)` - Quasi-random
- `cellular_automaton::generate(rule, gens, width, initial)` - Rule-based evolution
- `lsystem::generate(axiom, rules, iterations)` - L-system fractal growth
- `lsystem_to_sequence(lsystem)` - Convert L-system to numbers
- `markov::generate(transitions, start, n)` - Probabilistic sequences
- `build_markov_transitions(data, order)` - Learn from data
- `cantor_set::generate(depth, steps)` - Fractal rhythms
- `lorenz_attractor::generate(sigma, rho, beta, x0, y0, z0, n)` - 3D chaotic attractor
- `lorenz_butterfly(n)` - Lorenz attractor with default parameters
- `perlin_noise::generate(seed, freq, octaves, persistence, n)` - Smooth noise
- `perlin_noise_bipolar(seed, freq, octaves, persistence, n)` - Perlin in [-1, 1] range
- `rossler_attractor::generate(a, b, c, x0, y0, z0, n)` - 3D spiral attractor
- `rossler_spiral(a, b, c, x0, y0, z0, n)` - Rössler with default view
- `clifford_attractor::generate(a, b, c, d, x0, y0, n)` - Strange attractor
- `clifford_x(a, b, c, d, x0, y0, n)`, `clifford_y(...)` - Extract coordinates
- `clifford_flow(a, b, c, d, x0, y0, n)` - Flowing pattern variant
- `ikeda_map::generate(u, rho, c, x0, y0, n)` - Complex dynamics from laser physics
- `ikeda_x(u, rho, c, x0, y0, n)`, `ikeda_y(...)` - Extract coordinates
- `ikeda_spiral(u, rho, c, x0, y0, n)` - Spiral pattern variant

### Musical Transformations
- `normalize(seq, min, max)` - Map to range
- `normalize_f32(seq, min, max)` - Map f32 sequence
- `map_to_scale(seq, scale, root, octaves)` - Quantize to scale
- `map_to_scale_f32(seq, scale, root, octaves)` - Quantize f32 to scale
- `harmonic_series(fundamental, n)` - Overtone frequencies
- `undertone_series(fundamental, n)` - Mirror of harmonics
- `golden_ratio(n)` - Powers of φ
- `golden_sections(value, divisions)` - Divide by φ recursively
- `circle_of_fifths(root, n)` - Key relationships (ascending fifths)
- `circle_of_fourths(root, n)` - Key relationships (ascending fourths)
- `pythagorean_tuning(root, n)` - Pure fifth tuning
- `just_intonation_major(root)` - Pure harmonic ratios for major scale
- `just_intonation_minor(root)` - Pure harmonic ratios for minor scale

---

**Next:** Explore [MIDI Import/Export](./midi.md) to bring external MIDI files into your algorithmic compositions →
