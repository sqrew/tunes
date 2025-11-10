# Level 2: Making Music (Chord Progression)

Now let's make something musical. Instead of raw frequencies, we'll use note names and create a chord progression:

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("piano", &Instrument::electric_piano())
        .chord(C4, &ChordPattern::MAJOR, 0.5)    // C major
        .chord(F4, &ChordPattern::MAJOR, 0.5)    // F major
        .chord(G4, &ChordPattern::MAJOR, 0.5)    // G major
        .chord(C4, &ChordPattern::MAJOR, 0.5);   // C major

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

## What's New

- **Note names** like `C4`, `F4`, `G4` instead of raw frequencies
- **`ChordPattern::MAJOR`** uses music theory to build chords
- **`.chord()`** plays chords with a root note and pattern
- A recognizable **I-IV-V-I progression** in C major

This is the foundation of musical composition in Tunes – clear, readable code that maps directly to musical concepts.

---

**Next:** [Level 3: Algorithmic Music](./algorithmic.md) →
