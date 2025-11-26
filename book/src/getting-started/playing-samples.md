# Level 2: Playing Samples

Before diving into composition, this section covers basic sample playback. This approach is suitable for game audio, UI sounds, or any situation requiring audio file playback.

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;

    // Play any audio file
    engine.play_sample("explosion.wav")?;
    engine.play_sample("jump.wav")?;
    engine.play_sample("coin.wav")?;

    // All three play concurrently with automatic mixing
    Ok(())
}
```

This requires two lines of code for basic game audio playback.

## What's Happening

- **`AudioEngine::new()`** – Creates your audio system
- **`.play_sample()`** – Plays an audio file with automatic caching and SIMD acceleration
- **Automatic caching** – First call loads from disk, subsequent calls are instant
- **Concurrent playback** – All sounds play simultaneously, automatically mixed

## Supported Formats

Play any of these formats:
- **WAV** (`.wav`)
- **MP3** (`.mp3`)
- **OGG Vorbis** (`.ogg`)
- **FLAC** (`.flac`)
- **AAC / M4A** (`.aac`, `.m4a`)

No manual format handling required.

## Performance Characteristics

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;

    // First call: loads from disk (~1-10ms)
    engine.play_sample("footstep.wav")?;

    // All subsequent calls use cache
    engine.play_sample("footstep.wav")?;  // cached
    engine.play_sample("footstep.wav")?;  // cached
    engine.play_sample("footstep.wav")?;  // cached

    // Multiple concurrent sounds supported without performance degradation
    for _ in 0..50 {
        engine.play_sample("footstep.wav")?;  // SIMD-accelerated playback
    }

    Ok(())
}
```

**Implementation details:**
- Automatic caching by file path
- SIMD-accelerated playback (4-8 samples processed simultaneously)
- Concurrent mixing with no manual management
- Zero allocations in audio callback

## Use Cases

Suitable for:
- Sound effects (explosions, footsteps, UI clicks)
- Bullet hell games with hundreds of concurrent sounds
- Rapid prototyping without complex setup
- General audio file playback

See the [Game Engine Integration](../game-audio/game-engine-integration.md) chapter for how to use this in any Rust game engine (Bevy, ggez, macroquad, bracket-lib, etc).

---

This covers basic sample playback. Additional capabilities (synthesis, composition, effects) are available when needed.

**Next:** [Level 3: Making Music](./making-music.md) →
