# Concurrent Sound Effects

Game audio requires playing multiple sounds simultaneously—footsteps, gunshots, ambient effects, and UI sounds all at once. Tunes provides a concurrent audio engine designed exactly for this use case.

**Quick Start:** For simple sample playback, just use `engine.play_sample("sound.wav")?` - it handles everything automatically. See [Working with Samples](./samples.md) for details.

## The Core Pattern: Realtime Playback

Unlike traditional music composition where you wait for playback to finish, games need **non-blocking** audio:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    // Create different sound effects
    let footstep = create_footstep();
    let jump = create_jump_sound();
    let coin = create_coin_sound();

    // Play them without blocking
    let step_id = engine.play_mixer_realtime(&footstep)?;
    let jump_id = engine.play_mixer_realtime(&jump)?;
    let coin_id = engine.play_mixer_realtime(&coin)?;

    // All three sounds play concurrently!
    // Game logic continues immediately...

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

fn create_coin_sound() -> Mixer {
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.instrument("coin", &Instrument::bell())
        .note(&[800.0], 0.15);
    comp.into_mixer()
}
```

## Pre-rendering Sounds

For frequently played sounds, **pre-render them once** and reuse:

```rust
use tunes::prelude::*;

struct GameAudio {
    engine: AudioEngine,
    // Pre-rendered sounds
    footstep: Mixer,
    gunshot: Mixer,
    explosion: Mixer,
}

impl GameAudio {
    fn new() -> anyhow::Result<Self> {
        let engine = AudioEngine::new()?;

        Ok(Self {
            engine,
            footstep: Self::create_footstep(),
            gunshot: Self::create_gunshot(),
            explosion: Self::create_explosion(),
        })
    }

    fn play_footstep(&self) -> anyhow::Result<SoundId> {
        self.engine.play_mixer_realtime(&self.footstep)
    }

    fn play_gunshot(&self) -> anyhow::Result<SoundId> {
        self.engine.play_mixer_realtime(&self.gunshot)
    }

    fn play_explosion(&self) -> anyhow::Result<SoundId> {
        self.engine.play_mixer_realtime(&self.explosion)
    }

    fn create_footstep() -> Mixer {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("step")
            .note(&[80.0], 0.05)
            .filter(Filter::low_pass(200.0, 0.3));
        comp.into_mixer()
    }

    fn create_gunshot() -> Mixer {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("gun")
            .note(&[150.0], 0.08)
            .filter(Filter::high_pass(100.0, 0.5))
            .distortion(Distortion::new(0.7));
        comp.into_mixer()
    }

    fn create_explosion() -> Mixer {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("boom")
            .note(&[60.0], 0.3)
            .distortion(Distortion::new(0.8))
            .reverb(Reverb::new(0.6, 0.4));
        comp.into_mixer()
    }
}
```

## Adding Variation

Prevent "machine gun effect" by randomizing sound parameters:

```rust
use tunes::prelude::*;
use rand::Rng;

fn play_footstep_varied(engine: &AudioEngine) -> anyhow::Result<SoundId> {
    let mut rng = rand::thread_rng();

    let mut comp = Composition::new(Tempo::new(120.0));

    // Randomize pitch (75-85 Hz)
    let pitch = rng.gen_range(75.0..85.0);

    // Randomize volume (0.7-1.0)
    let volume = rng.gen_range(0.7..1.0);

    comp.track("step")
        .note(&[pitch], 0.05)
        .filter(Filter::low_pass(200.0, 0.3))
        .volume(volume);

    let mixer = comp.into_mixer();
    let sound_id = engine.play_mixer_realtime(&mixer)?;

    // Also randomize playback rate (0.95-1.05)
    let rate = rng.gen_range(0.95..1.05);
    engine.set_playback_rate(sound_id, rate)?;

    Ok(sound_id)
}
```

## Using Samples for Realism

For realistic game audio, use WAV samples:

```rust
use tunes::prelude::*;

fn play_game_audio() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    // Play samples with automatic caching and SIMD acceleration
    engine.play_sample("assets/footstep_concrete.wav")?;  // Loads and caches
    std::thread::sleep(std::time::Duration::from_millis(300));

    engine.play_sample("assets/footstep_concrete.wav")?;  // Uses cache (instant)
    std::thread::sleep(std::time::Duration::from_millis(500));

    engine.play_sample("assets/door_open.wav")?;
    std::thread::sleep(std::time::Duration::from_secs(1));

    engine.play_sample("assets/door_close.wav")?;

    // All samples play concurrently with automatic mixing and SIMD
    Ok(())
}
```

**Note:** For precise timing in compositions, you can use the composition API (see [Composition chapter](../concepts/composition.md)).

## Controlling Active Sounds

You can control sounds after they start playing:

```rust
use tunes::prelude::*;
use std::time::Duration;
use std::thread;

fn controlled_playback_demo() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    let mut comp = Composition::new(Tempo::new(120.0));
    comp.instrument("synth", &Instrument::synth_lead())
        .note(&[440.0], 5.0); // Long note

    let sound_id = engine.play_mixer_realtime(&comp.into_mixer())?;

    // Fade out over time
    for i in 0..10 {
        let volume = 1.0 - (i as f32 * 0.1);
        engine.set_volume(sound_id, volume)?;
        thread::sleep(Duration::from_millis(100));
    }

    // Stop it
    engine.stop(sound_id)?;

    Ok(())
}
```

## Performance Tip: Buffer Size

For low-latency game audio, reduce buffer size:

```rust
use tunes::prelude::*;

fn low_latency_setup() -> anyhow::Result<AudioEngine> {
    // Default is 8192 samples (~185ms latency at 44.1kHz)
    // Reduce for faster response:
    let engine = AudioEngine::with_buffer_size(2048)?; // ~46ms latency

    // Trade-off: Lower = more responsive, but may glitch on slower CPUs
    // Common values: 1024 (23ms), 2048 (46ms), 4096 (93ms)

    Ok(engine)
}
```

## Next Steps

- [Dynamic Music Systems](./dynamic-music.md) - Adaptive music that responds to gameplay
- [Spatial Audio](./spatial-audio.md) - 3D positioned sound effects
