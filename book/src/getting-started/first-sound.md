# Level 1: Your First Sound (Proof of Life)

Let's start with the absolute minimum – a simple 440Hz tone (concert A):

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.track("tone")
        .waveform(Waveform::Sine)
        .note(&[440.0], 1.0);  // 440Hz for 1 second

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

Run it with `cargo run`. You should hear a pure tone for one second.

## What's Happening

- **`AudioEngine::new()`** – Creates your audio output
- **`Composition`** – The container for your musical ideas
- **`.track()`** – Creates a track to hold audio events
- **`.waveform()`** – Sets the oscillator type (sine wave for pure tone)
- **`.note()`** – Plays a frequency (440Hz) for a duration (1 second)
- **`.play_mixer()`** – Renders and plays the audio

Simple, but you've just synthesized sound from scratch in Rust!

---

**Next:** [Level 2: Playing Samples](./playing-samples.md) →
