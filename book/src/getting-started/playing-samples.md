# Level 2: Playing Samples (Game Audio Made Easy)

Before we dive into composition, let's see the simplest possible use case: playing audio files. Perfect for game audio, UI sounds, or any situation where you just need to play a sound.

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;

    // That's it! Play any audio file
    engine.play_sample("explosion.wav")?;
    engine.play_sample("jump.wav")?;
    engine.play_sample("coin.wav")?;

    // All three play concurrently with automatic mixing
    Ok(())
}
```

**Two lines.** That's all you need for game audio.

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

No manual format handling - just pass the path.

## Why This Is Powerful

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;

    // First call: loads from disk (~1-10ms)
    engine.play_sample("footstep.wav")?;

    // All subsequent calls: instant! (uses cache)
    engine.play_sample("footstep.wav")?;  // ⚡ instant
    engine.play_sample("footstep.wav")?;  // ⚡ instant
    engine.play_sample("footstep.wav")?;  // ⚡ instant

    // You can spam sounds in game loops - no performance issues
    for _ in 0..50 {
        engine.play_sample("footstep.wav")?;  // All instant, SIMD-accelerated
    }

    Ok(())
}
```

**Behind the scenes:**
- ✅ Automatic caching by file path
- ✅ SIMD-accelerated playback (4-8 samples processed simultaneously)
- ✅ Concurrent mixing with no manual management
- ✅ Zero allocations in audio callback

## For Game Developers

This is perfect for:
- Sound effects (explosions, footsteps, UI clicks)
- Bullet hell games with hundreds of concurrent sounds
- Rapid prototyping without complex setup
- Any situation where you just need to play a sound

See the [Game Engine Integration](../game-audio/game-engine-integration.md) chapter for how to use this in any Rust game engine (Bevy, ggez, macroquad, bracket-lib, etc).

---

**This is the simplest audio API in Rust.** But when you need more power (synthesis, composition, effects), it's all there waiting for you.

**Next:** [Level 3: Making Music](./making-music.md) →
