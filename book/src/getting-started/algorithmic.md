# Level 4: Algorithmic Music (The Collatz Hook)

Here's where Tunes gets interesting. Let's generate a melody from the **Collatz sequence** – a mathematical pattern that creates surprisingly musical results:

```rust
use tunes::prelude::*;
use tunes::sequences;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(140.0));

    // Generate melody from Collatz sequence
    let collatz_seq = sequences::collatz(27, 20);  // Start at 27, take 20 values
    let melody = sequences::map_to_scale(
        &collatz_seq,
        &sequences::Scale::minor_pentatonic(),
        C4,  // Root note
        2    // Two octaves
    );

    comp.instrument("lead", &Instrument::synth_lead())
        .notes(&melody, 0.25);

    // Add bass
    comp.instrument("bass", &Instrument::sub_bass())
        .notes(&[C2, C2, F2, G2], 1.0);

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

## The Magic

- **`sequences::collatz(27, 20)`** – Generates 20 numbers from the Collatz sequence starting at 27
- **`sequences::map_to_scale()`** – Maps those numbers to musical notes in C minor pentatonic across 2 octaves
- **`.notes(&melody, 0.25)`** – Plays each note for 0.25 seconds
- **The result?** A hauntingly beautiful melody that sounds composed, but emerged from mathematics

## The Collatz Sequence

Starting with any positive integer:
- If it's **even**, divide by 2
- If it's **odd**, multiply by 3 and add 1
- Repeat until you reach 1

For 27, the sequence begins: **27 → 82 → 41 → 124 → 62 → 31...**

This seemingly simple rule creates rhythmic patterns and melodic contours that are difficult to compose by hand. It's the essence of **algorithmic music** – mathematical patterns that resonate with human perception.

---

## Next Steps

Now that you've heard the power of algorithmic composition, you're ready to:

- Explore other sequence generators (Fibonacci, primes, chaos attractors)
- Combine multiple instruments and effects
- Build complete arrangements with sections and structure
- Export your creations to WAV/FLAC/MIDI

The rest of this guide will show you how.

---

**Next:** [Architecture Overview](../concepts/architecture.md) →
