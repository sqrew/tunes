# Scoped Builders

Tunes uses a "scoped builder" pattern in several places. This page explains what it is and why it matters.

## The Pattern

You'll see closures like this throughout the API:

```rust
comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, "x---x---x---x---")
        .sound(DrumType::Snare, "----x-------x---"))
    .humanize(0.01, 0.05);
```

The `|g| g.sound(...)` is a closure that receives a specialized builder (`g`). Methods like `.sound()` only exist on that builder - you can't call them directly on the track.

## Why Closures?

The closure creates a **scope** where certain methods make sense:

- `.sound()` only makes sense inside a drum grid (needs step count, timing)
- `.delay()` inside `.effects()` applies to the track's effect chain
- `.shift()` inside `.transform()` operates on the pattern

Without scoping, you'd have one massive builder with 100+ methods, making autocomplete useless and the API confusing.

## Where It's Used

### drum_grid

```rust
.drum_grid(steps, duration, |g| g
    .sound(DrumType::Kick, "x---")
    .ghost(DrumType::HiHat, "x-x-")
    .accent("x---x---"))
```

**Inside the closure:** `.sound()`, `.ghost()`, `.flam()`, `.roll()`, `.accent()`, `.velocity()`, `.repeat()`

### effects

```rust
.effects(|e| e
    .delay(Delay::new(0.25, 0.4, 0.5))
    .reverb(Reverb::new(0.5, 0.5, 0.3))
    .filter(Filter::low_pass(2000.0, 0.7)))
```

**Inside the closure:** `.delay()`, `.reverb()`, `.filter()`, `.distortion()`, `.chorus()`, `.compressor()`, etc.

### synthesis

```rust
.synthesis(|s| s
    .oscillator(Waveform::Saw)
    .filter(FilterType::LowPass, 2000.0)
    .resonance(4.0)
    .adsr(0.01, 0.1, 0.7, 0.3))
.notes(&[C4, E4, G4], 0.5)
```

**Inside the closure:** `.oscillator()`, `.filter()`, `.resonance()`, `.adsr()`, `.envelope()`, `.fm()`, `.lfo()`, `.supersaw()`, `.unison()`, `.additive()`, `.wavetable()`

### transform

```rust
.notes(&[C4, E4, G4, C5], 0.25)
.transform(|t| t
    .shift(12)
    .reverse()
    .stutter(2))
```

**Inside the closure:** `.shift()`, `.reverse()`, `.stutter()`, `.quantize()`, `.humanize()`, `.retrograde()`, etc.

## Reading the Chain

```rust
comp.track("synth")           // Returns TrackBuilder
    .synthesis(|s| s          // Opens SynthesisBuilder scope
        .oscillator(Waveform::Saw)
        .filter(FilterType::LowPass, 2000.0))
    .effects(|e| e            // Opens EffectsBuilder scope
        .delay(...)           //   EffectsBuilder method
        .reverb(...))         //   EffectsBuilder method
    .notes(&[C4], 0.5)        // Back to TrackBuilder
    .transform(|t| t          // Opens TransformBuilder scope
        .shift(7))            //   TransformBuilder method
    .volume(0.8);             // Back to TrackBuilder
```

After a closure ends, you're back to the original builder. Chain as many scopes as you need.

## Quick Reference

| Method                              | Builder Inside   | Purpose                             |
|-------------------------------------|------------------|-------------------------------------|
| `.drum_grid(steps, dur, \|g\| ...)` | DrumGrid         | Step sequencer patterns             |
| `.effects(\|e\| ...)`               | EffectsBuilder   | Track effect chain                  |
| `.synthesis(\|s\| ...)`             | SynthesisBuilder | Oscillator, filter, envelope config |
| `.transform(\|t\| ...)`             | TransformBuilder | Pattern manipulation                |

If a method isn't showing up in autocomplete, you're probably in the wrong scope - check if you need to be inside a closure (or outside one).
