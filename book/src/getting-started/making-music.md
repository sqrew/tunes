# Level 3: Making Music (Chord Progression)

Now let's make something musical. Instead of raw frequencies, we'll use note names and create a chord progression:

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("piano", &Instrument::electric_piano())
        .chords(&[C4_MAJOR, F4_MAJOR, G4_MAJOR, C4_MAJOR], 0.5);

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

## What's New

- **Note names** and **chord constants** like `C4_MAJOR`, `F4_MAJOR` instead of raw frequencies
- **`.chords()`** plays a sequence of chord constants with specified duration
- A recognizable **I-IV-V-I progression** in C major

This is the foundation of musical composition in Tunes – clear, readable code that maps directly to musical concepts.

---

**Next:** [Level 4: Algorithmic Music](./algorithmic.md) →
