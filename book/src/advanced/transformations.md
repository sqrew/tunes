# Pattern Transformations

Pattern transformations are powerful tools for manipulating musical patterns in creative and expressive ways. Tunes provides two syntaxes for applying transformations: direct method calls and a cleaner namespaced API using `.transform()`.

## Overview

Pattern transformations let you:
- **Modify pitch** - transpose, rotate, invert, or mutate notes
- **Modify timing** - stretch, compress, quantize, or add stuttering effects
- **Add variation** - humanize, shuffle, or add evolutionary mutations
- **Create echoes** - layer delayed repetitions with volume decay
- **Shape melodies** - compress/expand pitch ranges, smooth/exaggerate contours

All transformations are chainable and can be applied to any pattern created with `.pattern_start()`.

---

## Two Ways to Apply Transformations

### Direct Method Calls (Classic Syntax)

You can call transformation methods directly on the track builder:

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

comp.track("example")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.25)
    .shift(7)              // Direct method call
    .humanize(0.01, 0.05)  // Direct method call
    .rotate(1);            // Direct method call
```

**Pros:**
- Familiar if you're used to the original API
- Slightly more concise for single transformations

**Cons:**
- Clutters autocomplete with many transformation methods
- Less organized when applying multiple transformations

### Transform Namespace (New Syntax)

Encapsulate transformations in a `.transform()` closure for better organization:

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

comp.track("example")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.25)
    .transform(|t| t       // Enter transform namespace
        .shift(7)
        .humanize(0.01, 0.05)
        .rotate(1)
    );                     // Automatically exits namespace
```

**Pros:**
- Cleaner autocomplete - transformations only appear inside `.transform()`
- Better organization - visual grouping of related operations
- More readable - clear boundaries for transformation logic
- Easy to chain multiple transform blocks

**Cons:**
- Slightly more verbose for single transformations

**Both syntaxes work and are fully compatible!** Choose whichever fits your workflow.

---

## Common Transformations

### Shift - Transpose Patterns

Transpose a pattern up or down by semitones:

```rust
comp.track("transpose")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .shift(7)    // Up a perfect fifth
    );

// Or transpose down
comp.track("down")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .shift(-12)  // Down an octave
    );
```

**Parameters:**
- `semitones: i32` - Number of semitones to shift (positive = up, negative = down)

**Use cases:**
- Quick key changes in live coding
- Creating harmonic variations
- Building chord progressions

### Humanize - Add Organic Feel

Add subtle timing and velocity variations to make programmed patterns feel natural:

```rust
comp.track("humanized")
    .pattern_start()
    .notes(&[C4, C4, C4, C4], 0.25)
    .transform(|t| t
        .humanize(0.01, 0.05)  // ±10ms timing, ±5% velocity
    );

// Heavy humanization for drunk piano effect
comp.track("drunk")
    .pattern_start()
    .notes(&[C4, C4, C4, C4], 0.25)
    .transform(|t| t
        .humanize(0.05, 0.2)   // ±50ms timing, ±20% velocity
    );
```

**Parameters:**
- `timing_variance: f32` - Maximum timing offset in seconds (±)
- `velocity_variance: f32` - Maximum velocity change as fraction (±)

**What's happening:**
- Each note's start time is randomly offset within ±timing_variance
- Each note's velocity is randomly adjusted within ±velocity_variance
- Values of 0.01-0.02 for timing and 0.05-0.1 for velocity sound realistic

**Use cases:**
- Making sequences sound less robotic
- Adding realism to drum programming
- Creating organic-feeling generative music

### Range Dilation - Expand or Compress Pitch Range

Unified control for expanding or compressing the pitch range of a pattern:

```rust
// Compress: Make wide melody narrower
comp.track("compress")
    .pattern_start()
    .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25)  // 2 octave span
    .transform(|t| t
        .range_dilation(0.5)  // Compress to 1 octave
    );

// Expand: Make narrow melody wider
comp.track("expand")
    .pattern_start()
    .notes(&[C4, D4, E4, F4, G4], 0.25)  // Small range
    .transform(|t| t
        .range_dilation(2.0)  // Double the range
    );
```

**Parameters:**
- `factor: f32` - Dilation factor
  - `< 1.0` - Compress range (notes closer together)
  - `= 1.0` - No change
  - `> 1.0` - Expand range (notes further apart)

**What's happening:**
- Calculates the pattern's average pitch
- Scales each note's distance from the average by the factor
- Preserves the melodic center while expanding/compressing around it

**Use cases:**
- Taming overly wide melodic leaps
- Exaggerating small melodic variations
- Creating dynamic range shifts in generative music

### Shape Contour - Smooth or Exaggerate Melodic Intervals

Unified control for smoothing jagged melodies or exaggerating subtle ones:

```rust
// Smooth: Tame large jumps
comp.track("smooth")
    .pattern_start()
    .notes(&[C4, C6, C3, C5, C4], 0.4)  // Jagged leaps
    .transform(|t| t
        .shape_contour(0.4)  // Smooth out intervals
    );

// Exaggerate: Make small steps dramatic
comp.track("dramatic")
    .pattern_start()
    .notes(&[C4, D4, E4, F4, E4, D4, C4], 0.3)  // Step-wise
    .transform(|t| t
        .shape_contour(2.5)  // Exaggerate intervals
    );
```

**Parameters:**
- `factor: f32` - Contour shaping factor
  - `< 1.0` - Smooth (reduce interval sizes)
  - `= 1.0` - No change
  - `> 1.0` - Exaggerate (increase interval sizes)

**What's happening:**
- Analyzes intervals between consecutive notes
- Scales each interval by the factor
- Preserves the starting note while reshaping the melodic contour

**Use cases:**
- Making random melodies more singable
- Creating dramatic variations of simple melodies
- Controlling melodic complexity in algorithmic composition

### Echo - Delay Trail Effect

Create fading echoes of a pattern with configurable timing and decay:

```rust
comp.track("echo")
    .pattern_start()
    .notes(&[C5, E5, G5, C6], 0.4)
    .transform(|t| t
        .echo(0.35, 3, 0.6)  // 350ms delay, 3 repeats, 60% decay
    );
```

**Parameters:**
- `delay: f32` - Time between echoes in seconds
- `repeats: usize` - Number of echo repetitions
- `decay: f32` - Volume multiplier for each repeat (0.0-1.0)

**What's happening:**
- Each note in the pattern is duplicated `repeats` times
- Each repetition is delayed by `delay` seconds
- Volume decreases by `decay` factor each time (e.g., 0.6 = 60% of previous)

**Use cases:**
- Adding depth and space to melodic lines
- Creating rhythmic polyrhythms from simple patterns
- Ambient texture building

---

## Advanced Transformations

### Rotate - Cycle Pitch Positions

Shift pitch positions forward or backward while preserving timing:

```rust
// Original: C4, E4, G4, C5
comp.track("rotate")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .rotate(1)   // Result: E4, G4, C5, C4
    );

// Rotate backward
comp.track("backward")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .rotate(-1)  // Result: C5, C4, E4, G4
    );
```

**Parameters:**
- `positions: i32` - Number of positions to rotate (positive = forward, negative = backward)

**Use cases:**
- Quick melodic variations in live coding
- Creating inversions without retyping notes
- Exploring different melodic orderings

### Retrograde - Reverse Pitch Sequence

Classic compositional technique that reverses the order of pitches (timing unchanged):

```rust
// Original ascending: C4, D4, E4, F4, G4
comp.track("retrograde")
    .pattern_start()
    .notes(&[C4, D4, E4, F4, G4], 0.25)
    .transform(|t| t
        .retrograde()  // Result: G4, F4, E4, D4, C4
    );
```

**Use cases:**
- Classical composition techniques (fugues, canons)
- Creating mirror melodies
- Adding variation without new material

### Shuffle - Random Reordering

Randomly shuffle the order of pitches:

```rust
comp.track("shuffle")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.25)
    .transform(|t| t
        .shuffle()  // Random order each time
    );
```

**What's happening:**
- Pitches are randomly reordered
- Each call to `.shuffle()` produces a different random order
- Timing structure is preserved

**Use cases:**
- Generative music variations
- Breaking predictable patterns
- Creating controlled randomness

### Thin - Probabilistic Note Removal

Randomly remove notes based on probability:

```rust
// Keep 70% of notes (remove 30%)
comp.track("thin")
    .pattern_start()
    .notes(&[C4; 16], 0.125)  // Dense pattern
    .transform(|t| t
        .thin(0.7)  // 70% density
    );

// Sparse: Keep 30% of notes
comp.track("sparse")
    .pattern_start()
    .notes(&[C4; 16], 0.125)
    .transform(|t| t
        .thin(0.3)  // 30% density
    );
```

**Parameters:**
- `keep_probability: f32` - Probability of keeping each note (0.0-1.0)

**Use cases:**
- Creating space in dense patterns
- Hi-hat variation in drums
- Controlling rhythmic density dynamically

### Stack - Harmonic Layering

Layer additional voices at intervals above or below the original:

```rust
// Stack octave above (classic doubling)
comp.track("octave")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .stack(12, 1)  // +12 semitones, 1 layer
    );

// Stack two octaves
comp.track("thick")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .stack(12, 2)  // 2 layers: +12 and +24 semitones
    );

// Stack perfect fifth
comp.track("harmony")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .stack(7, 1)   // +7 semitones
    );
```

**Parameters:**
- `semitones: i32` - Interval for each stack layer
- `count: usize` - Number of additional layers

**What's happening:**
- Original pattern plays normally
- Additional layers are added at `semitones`, `2*semitones`, `3*semitones`, etc.
- All layers play simultaneously

**Use cases:**
- Making sounds bigger and richer
- Creating instant harmonies
- Thickening lead lines

### Mutate - Evolutionary Pitch Variation

Randomly vary pitches within a range for organic evolution:

```rust
// Subtle mutation (±1 semitone)
comp.track("subtle")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .mutate(1)  // Each note ±0-1 semitone
    );

// Wild mutation (±7 semitones)
comp.track("wild")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .mutate(7)  // Each note ±0-7 semitones
    );
```

**Parameters:**
- `max_semitones: i32` - Maximum random shift per note (±0 to ±max)

**Use cases:**
- Generative music with controlled variation
- Evolving patterns over time
- Adding organic unpredictability

---

## Timing Transformations

### Stretch - Time Dilation

Speed up or slow down a pattern by a factor:

```rust
// Half speed (twice as long)
comp.track("slow")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.25)
    .transform(|t| t
        .stretch(2.0)  // 2x slower
    );

// Double speed (half duration)
comp.track("fast")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.25)
    .transform(|t| t
        .stretch(0.5)  // 2x faster
    );
```

**Parameters:**
- `factor: f32` - Time multiplier
  - `> 1.0` - Slower (stretched)
  - `< 1.0` - Faster (compressed)
  - `= 1.0` - No change

**Use cases:**
- Rhythmic variations
- Time stretching effects
- Matching patterns to specific tempos

### Compress - Target Duration

Fit a pattern to an exact duration (no ratio math required):

```rust
// Original: 2 seconds (4 notes × 0.5s each)
// Compress to 1 second (double speed)
comp.track("compress")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .compress(1.0)  // Fit into 1 second
    );

// Expand to 3 seconds (1.5x slower)
comp.track("expand")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .compress(3.0)  // Fit into 3 seconds
    );
```

**Parameters:**
- `target_duration: f32` - Desired total duration in seconds

**What's happening:**
- Calculates the pattern's current duration
- Computes the ratio: `target_duration / current_duration`
- Applies that ratio to all note timings

**Use cases:**
- Fitting patterns to exact measures
- No mental math for time ratios
- Dynamic pattern scaling

### Quantize - Snap to Grid

Clean up timing by snapping to a rhythmic grid:

```rust
// Sloppy humanized pattern
comp.track("quantize")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.25)
    .humanize(0.05, 0.0)  // Add timing jitter
    .transform(|t| t
        .quantize(0.25)   // Snap to 16th note grid
    );
```

**Parameters:**
- `grid: f32` - Grid size in beats (0.25 = 16th notes, 0.5 = 8th notes)

**Use cases:**
- Cleaning up after humanization
- Correcting timing drift
- Snapping random walk to rhythmic grid

### Palindrome - Mirror Pattern

Play pattern forward then backward (like a palindrome):

```rust
// Original: C4, D4, E4, F4
comp.track("palindrome")
    .pattern_start()
    .notes(&[C4, D4, E4, F4], 0.25)
    .transform(|t| t
        .palindrome()  // Result: C4, D4, E4, F4, F4, E4, D4, C4
    );
```

**Use cases:**
- Symmetrical phrases
- Classical composition techniques
- Creating balanced musical shapes

---

## Glitch and Texture Effects

### Stutter - Random Glitchy Repeats

Randomly repeat notes for glitch effects:

```rust
// 30% chance each note stutters 4 times
comp.track("stutter")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .stutter(0.3, 4)  // 30% probability, 4 repeats
    );

// Heavy glitch: 50% chance, 8x stutter
comp.track("heavy")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .stutter(0.5, 8)
    );
```

**Parameters:**
- `probability: f32` - Chance each note stutters (0.0-1.0)
- `repeats: usize` - Number of times to repeat when stuttering

**Use cases:**
- IDM and glitch hop effects
- Controlled randomness
- Rhythmic interest

### Stutter Every - Deterministic Rolls

Stutter every Nth note (trap-style hi-hat rolls):

```rust
// Every 4th note rolls 8 times
comp.track("trap")
    .pattern_start()
    .notes(&[C4; 16], 0.25)
    .transform(|t| t
        .stutter_every(4, 8)  // Every 4th, repeat 8x
    );
```

**Parameters:**
- `nth: usize` - Stutter every Nth note (1-indexed)
- `repeats: usize` - Number of repetitions

**Use cases:**
- Trap-style hi-hat rolls
- Predictable rhythmic embellishments
- EDM build-ups

### Granularize - Micro-Fragments

Break notes into tiny grains for shimmering textures:

```rust
// Break 1-second note into 10 grains
comp.track("granular")
    .pattern_start()
    .note(&[C4], 1.0)
    .transform(|t| t
        .granularize(10)  // 10 micro-grains
    );

// Granular shimmer with pitch mutation
comp.track("shimmer")
    .pattern_start()
    .note(&[C4], 1.0)
    .transform(|t| t
        .granularize(20)  // 20 grains
        .mutate(3)        // Vary pitches
    );
```

**Parameters:**
- `divisions: usize` - Number of grains to create

**What's happening:**
- Each note is divided into `divisions` equal micro-notes
- Original duration is preserved
- Combine with `.mutate()` for pitch-varied textures

**Use cases:**
- Ambient shimmer effects
- Granular synthesis-style textures
- Creating clouds of sound

---

## Advanced Pitch Manipulation

### Magnetize - Snap to Scale

Pull pitches toward the nearest notes in a scale:

```rust
// Force random notes to pentatonic scale
comp.track("magnetize")
    .pattern_start()
    .notes(&[220.0, 315.7, 428.3, 567.1], 0.5)  // Random frequencies
    .transform(|t| t
        .magnetize(&[C4, D4, E4, G4, A4])  // C pentatonic
    );
```

**Parameters:**
- `scale_notes: &[f32]` - Array of allowed frequencies

**Use cases:**
- Correcting out-of-scale notes
- Quantizing random walks to scales
- Creating modal constraints

### Gravity - Pull Toward Center Pitch

Gently pull pitches toward a target frequency:

```rust
comp.track("gravity")
    .pattern_start()
    .notes(&[C3, G3, C4, G4, C5], 0.3)
    .transform(|t| t
        .gravity(E4, 0.3)  // Pull 30% toward E4
    );
```

**Parameters:**
- `center_pitch: f32` - Target frequency to pull toward
- `strength: f32` - Pull strength (0.0-1.0, where 1.0 = complete collapse)

**Use cases:**
- Subtle melodic centering
- Dynamic pitch attraction
- Creating pitch orbits

### Ripple - Cascading Micro-Delays

Add tiny staggered delays for a cascading effect:

```rust
comp.track("ripple")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.5)
    .transform(|t| t
        .ripple(0.02)  // 20ms cascade between notes
    );
```

**Parameters:**
- `intensity: f32` - Delay increment per note in seconds

**Use cases:**
- String section humanization
- Harp-like cascades
- Adding subtle motion

---

## Chaining Multiple Transformations

### Combining Transforms in One Block

You can chain multiple transformations within a single `.transform()` call:

```rust
comp.track("complex")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.25)
    .transform(|t| t
        .shift(7)              // Transpose up
        .humanize(0.01, 0.05)  // Add realism
        .rotate(1)             // Rotate pitches
        .echo(0.25, 2, 0.5)    // Add echo
    );
```

### Multiple Transform Blocks

You can also use multiple `.transform()` blocks for organization:

```rust
comp.track("organized")
    .pattern_start()
    .notes(&[C4, D4, E4, F4, G4], 0.25)
    .transform(|t| t  // Rhythm transformations
        .stretch(0.5)
        .quantize(0.125)
    )
    .transform(|t| t  // Pitch transformations
        .shift(12)
        .mutate(2)
    )
    .transform(|t| t  // Feel transformations
        .humanize(0.01, 0.05)
        .shuffle()
    );
```

### Generative Example

Here's a complex generative pattern using many transformations:

```rust
comp.track("generative")
    .pattern_start()
    .note(&[C4], 1.0)
    .transform(|t| t
        .granularize(16)                        // Break into grains
        .mutate(4)                              // Randomize pitches
        .thin(0.7)                              // Remove some grains
        .magnetize(&[C4, D4, E4, G4, A4])      // Force to pentatonic
        .gravity(E4, 0.3)                       // Pull toward E4
        .ripple(0.02)                           // Add cascade
        .shuffle()                              // Randomize order
        .humanize(0.01, 0.08)                   // Add organic feel
    );
```

**What's happening:**
1. Single note broken into 16 grains
2. Each grain's pitch randomly varied (±4 semitones)
3. 30% of grains removed for space
4. Remaining grains snapped to C pentatonic
5. All pitches pulled 30% toward E4
6. Tiny cascading delays added
7. Order randomized
8. Timing and velocity humanized

**Result:** A shimmering, evolving texture from a single note!

---

## Full Transformation Reference

### Pitch Transformations
- `.shift(semitones)` - Transpose up/down
- `.rotate(positions)` - Cycle pitch positions
- `.retrograde()` - Reverse pitch sequence
- `.shuffle()` - Random pitch reordering
- `.mutate(max_semitones)` - Random pitch variation
- `.stack(semitones, count)` - Layer harmonic voices
- `.magnetize(scale_notes)` - Snap to scale
- `.gravity(center_pitch, strength)` - Pull toward pitch
- `.range_dilation(factor)` - Expand/compress pitch range
- `.shape_contour(factor)` - Smooth/exaggerate intervals

### Timing Transformations
- `.stretch(factor)` - Speed up/slow down
- `.compress(target_duration)` - Fit to exact duration
- `.quantize(grid)` - Snap to rhythmic grid
- `.palindrome()` - Play forward then backward
- `.ripple(intensity)` - Cascading micro-delays
- `.echo(delay, repeats, decay)` - Delay trail effect

### Density Transformations
- `.thin(keep_probability)` - Probabilistically remove notes
- `.stutter(probability, repeats)` - Random glitchy repeats
- `.stutter_every(nth, repeats)` - Deterministic stuttering
- `.granularize(divisions)` - Break into micro-fragments

### Feel Transformations
- `.humanize(timing_variance, velocity_variance)` - Add organic variations

---

## Tips and Best Practices

### 1. Start Simple, Then Transform

Build your pattern first, then apply transformations:

```rust
// ✅ GOOD: Clear structure
comp.track("melody")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.25)  // Base pattern
    .transform(|t| t                  // Then transform
        .shift(7)
        .humanize(0.01, 0.05)
    );

// ❌ BAD: Hard to understand what the original was
comp.track("confusing")
    .pattern_start()
    .notes(&[C4, E4, G4, C5], 0.25)
    .shift(7).humanize(0.01, 0.05).rotate(1).mutate(2);
```

### 2. Use Transform Blocks for Organization

Group related transformations:

```rust
comp.track("organized")
    .pattern_start()
    .notes(&[C4, E4, G4], 0.25)
    .transform(|t| t  // Pitch
        .shift(12)
        .mutate(2)
    )
    .transform(|t| t  // Rhythm
        .stretch(0.5)
        .humanize(0.01, 0.0)
    );
```

### 3. Combine Complementary Transforms

Some transformations work great together:

- **Granularize + Mutate** - Shimmering textures
- **Range Dilation + Shape Contour** - Fine-tune melodic shape
- **Shuffle + Magnetize** - Controlled randomness
- **Humanize + Quantize** - Realistic but tight timing
- **Echo + Range Dilation** - Evolving delay trails

### 4. Order Matters

Transformations apply in sequence, so order affects the result:

```rust
// Different results!
.transform(|t| t.granularize(10).mutate(3))  // Mutate grains
.transform(|t| t.mutate(3).granularize(10))  // Granularize mutated notes
```

### 5. Use Subtle Values for Realism

Less is often more for natural-sounding results:

```rust
// ✅ GOOD: Subtle humanization
.humanize(0.01, 0.05)  // Realistic

// ❌ BAD: Overly extreme
.humanize(0.1, 0.5)    // Sounds broken
```

---

**Next:** Explore [Live Coding](./live-coding.md) to use transformations in real-time performance →
