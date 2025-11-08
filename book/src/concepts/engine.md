# AudioEngine Layer

The `AudioEngine` manages audio playback and real-time control. It's your interface to speakers (or files), handling concurrent sound mixing and dynamic parameter changes.

## What is AudioEngine?

`AudioEngine` takes `Mixer` objects and plays them through your audio device. It:

- Manages a persistent audio stream (one per engine)
- Plays multiple sounds concurrently
- Provides real-time control (volume, pan, playback rate)
- Handles looping music
- Exports audio at the correct sample rate

**When to use it:** Any time you need to hear audio or control it dynamically.

---

## Creating an Engine

### Basic Creation

```rust
use tunes::prelude::*;

let engine = AudioEngine::new()?;
```

This creates an engine with:
- Default buffer size: 4096 samples (~93ms latency at 44.1kHz)
- System default audio device
- Automatic sample rate detection

### Custom Buffer Size

For games or interactive apps, reduce latency:

```rust
let engine = AudioEngine::with_buffer_size(1024)?;  // ~23ms latency
```

**Buffer size guidelines:**
- **512-1024**: Low latency for games, may glitch on slower CPUs
- **2048-4096**: Balanced for most applications (default: 4096)
- **8192+**: Very stable, higher latency

The engine prints initialization info:
```
Audio Engine initialized:
  Device: Default Audio Device
  Sample rate: 44100
  Buffer size: 4096 samples (93.0ms latency)
  Concurrent mixing: enabled
```

---

## Playing Audio

### Blocking Playback

`play_mixer()` waits until playback finishes:

```rust
let mut comp = Composition::new(Tempo::new(120.0));
comp.track("piano").notes(&[C4, E4, G4, C5], 0.5);

engine.play_mixer(&comp.into_mixer())?;
println!("Playback finished!");  // Prints after audio completes
```

**Use when:**
- Writing simple scripts or examples
- Playing one sound at a time
- You want to wait for completion

### Non-blocking Playback

`play_mixer_realtime()` returns immediately with a `SoundId`:

```rust
let id = engine.play_mixer_realtime(&mixer)?;
println!("Audio started!");  // Prints immediately

// Continue doing other work...
engine.set_volume(id, 0.5)?;  // Control while playing
```

**Use when:**
- Building games or interactive applications
- Playing multiple sounds concurrently
- Need dynamic control during playback

### Concurrent Sounds

Play multiple sounds simultaneously:

```rust
let footstep_id = engine.play_mixer_realtime(&footstep)?;
let gunshot_id = engine.play_mixer_realtime(&gunshot)?;
let music_id = engine.play_mixer_realtime(&music)?;

// All three sounds play together, mixed in real-time
```

The engine automatically mixes all active sounds into a single output stream.

---

## Looping Music

Use `play_looping()` for background music:

```rust
let loop_id = engine.play_looping(&background_music)?;

// Music loops infinitely...

// Stop it when done
engine.stop(loop_id)?;
```

**Common pattern - dynamic music:**

```rust
// Exploration
let ambient_id = engine.play_looping(&ambient)?;

// Enemy appears - switch music
engine.stop(ambient_id)?;
let combat_id = engine.play_looping(&combat)?;

// Back to safe
engine.stop(combat_id)?;
let ambient_id = engine.play_looping(&ambient)?;
```

---

## Real-Time Control

Control sounds after they start playing using the `SoundId`:

### Volume

```rust
let id = engine.play_mixer_realtime(&mixer)?;
engine.set_volume(id, 0.5)?;  // 50% volume (range: 0.0-1.0)
```

### Stereo Panning

```rust
engine.set_pan(id, -1.0)?;  // Full left
engine.set_pan(id, 0.0)?;   // Center
engine.set_pan(id, 1.0)?;   // Full right
```

### Playback Rate (Speed + Pitch)

```rust
engine.set_playback_rate(id, 2.0)?;  // 2x speed, one octave higher
engine.set_playback_rate(id, 0.5)?;  // Half speed, one octave lower
```

**Use cases:**
- Randomize footstep sounds (0.95-1.05)
- Doppler effect simulation
- Impact sounds based on velocity

### Pause and Resume

```rust
engine.pause(id)?;
// ... time passes ...
engine.resume(id)?;  // Continues from where it paused
```

### Stop

```rust
engine.stop(id)?;  // Stops and removes the sound (cannot resume)
```

### Check if Playing

```rust
if engine.is_playing(id) {
    engine.stop(id)?;
}
```

---

## Export Methods

Export audio using the engine's sample rate:

```rust
let mut mixer = comp.into_mixer();

// WAV export (16-bit)
engine.export_wav(&mut mixer, "output.wav")?;

// FLAC export (24-bit, lossless compression)
engine.export_flac(&mut mixer, "output.flac")?;
```

**Why use engine exports?** The engine automatically uses the same sample rate as playback, ensuring exported audio matches what you hear.

For other formats or explicit sample rate control, use `Mixer` export methods directly.

---

## Complete Example

Here's a practical game audio example:

```rust
use tunes::prelude::*;
use std::time::Duration;
use std::thread;

fn main() -> anyhow::Result<()> {
    // Create engine with low latency
    let engine = AudioEngine::with_buffer_size(1024)?;

    // Pre-create sound effects
    let footstep = create_footstep();
    let jump = create_jump_sound();
    let background = create_ambient_music();

    // Start looping background music
    let music_id = engine.play_looping(&background)?;
    engine.set_volume(music_id, 0.3)?;  // Quiet background music

    // Play sound effects concurrently
    let step_id = engine.play_mixer_realtime(&footstep)?;
    thread::sleep(Duration::from_millis(300));

    let jump_id = engine.play_mixer_realtime(&jump)?;
    thread::sleep(Duration::from_millis(500));

    let step2_id = engine.play_mixer_realtime(&footstep)?;

    // Randomize the second footstep
    engine.set_playback_rate(step2_id, 1.05)?;
    engine.set_pan(step2_id, 0.3)?;

    // Wait for sound effects to finish
    while engine.is_playing(step_id) || engine.is_playing(jump_id) {
        thread::sleep(Duration::from_millis(10));
    }

    // Stop background music
    engine.stop(music_id)?;

    Ok(())
}

fn create_footstep() -> Mixer {
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("step")
        .note(&[80.0], 0.05)
        .filter(Filter::low_pass(200.0, 0.3));
    comp.into_mixer()
}

fn create_jump_sound() -> Mixer {
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("jump")
        .note(&[200.0], 0.1)
        .fade_to(400.0, 0.1);
    comp.into_mixer()
}

fn create_ambient_music() -> Mixer {
    let mut comp = Composition::new(Tempo::new(80.0));
    comp.instrument("pad", &Instrument::synth_pad())
        .notes(&[C3, E3, G3], 2.0);
    comp.into_mixer()
}
```

---

## How It Works

Under the hood, `AudioEngine` uses a persistent audio stream with lock-free communication:

1. **Main thread** sends commands (`play_mixer_realtime()`, `set_volume()`, etc.)
2. **Audio thread** receives commands and mixes all active sounds
3. **Concurrent mixing** combines multiple sounds in real-time
4. **Real-time control** updates parameters with sub-millisecond latency

This architecture enables hundreds of concurrent sounds with zero audio dropouts.

---

**Next Steps:**
- [Composition Layer](./composition.md) - Build musical ideas
- [Mixer Layer](./mixer.md) - Understand audio rendering
- [Game Audio Patterns](../game-audio/concurrent-sfx.md) - Practical game audio techniques
