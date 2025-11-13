# Bevy Integration

Integrating tunes with Bevy is straightforward - just add `AudioEngine` as a resource and call `play_sample()` from your systems.

## Quick Setup

**1. Add dependencies to `Cargo.toml`:**

```toml
[dependencies]
bevy = "0.14"
tunes = "0.15"
```

**2. Create the AudioEngine resource:**

```rust
use bevy::prelude::*;
use tunes::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_audio)
        .add_systems(Update, game_audio_system)
        .run();
}

fn setup_audio(mut commands: Commands) {
    // Create AudioEngine and add as resource
    let engine = AudioEngine::new().expect("Failed to initialize audio");
    commands.insert_resource(engine);
}
```

**3. Play audio from any system:**

```rust
fn game_audio_system(
    engine: Res<AudioEngine>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Play samples in response to events
    if keyboard.just_pressed(KeyCode::Space) {
        engine.play_sample("assets/audio/jump.wav")
            .expect("Failed to play jump sound");
    }

    if keyboard.just_pressed(KeyCode::KeyF) {
        engine.play_sample("assets/audio/shoot.wav")
            .expect("Failed to play shoot sound");
    }
}
```

That's it! The AudioEngine automatically handles:
- ✅ Caching (repeated sounds are instant)
- ✅ Concurrent playback
- ✅ SIMD acceleration
- ✅ Automatic mixing

## Complete Example

```rust
use bevy::prelude::*;
use tunes::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_audio, setup_game))
        .add_systems(Update, (
            player_movement,
            collision_system,
        ))
        .run();
}

fn setup_audio(mut commands: Commands) {
    let engine = AudioEngine::new().expect("Failed to initialize audio");

    // Optional: Pre-load sounds during startup
    engine.preload_sample("assets/audio/footstep.wav").ok();
    engine.preload_sample("assets/audio/jump.wav").ok();
    engine.preload_sample("assets/audio/coin.wav").ok();

    commands.insert_resource(engine);
}

fn setup_game(mut commands: Commands) {
    // Spawn player
    commands.spawn((
        Player,
        SpriteBundle::default(),
    ));
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    engine: Res<AudioEngine>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let speed = 5.0;

        if keyboard.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= speed;
            if keyboard.just_pressed(KeyCode::ArrowLeft) {
                engine.play_sample("assets/audio/footstep.wav").ok();
            }
        }

        if keyboard.just_pressed(KeyCode::Space) {
            // Jump!
            engine.play_sample("assets/audio/jump.wav").ok();
        }
    }
}

fn collision_system(
    engine: Res<AudioEngine>,
    // ... your collision logic here
) {
    // Play sound on collision
    engine.play_sample("assets/audio/coin.wav").ok();
}
```

## Tips

### Pre-loading Sounds

Pre-load frequently-used sounds during `Startup` to avoid first-play delay:

```rust
fn setup_audio(mut commands: Commands) {
    let engine = AudioEngine::new().unwrap();

    // Pre-load common sounds
    engine.preload_sample("assets/audio/footstep.wav").ok();
    engine.preload_sample("assets/audio/jump.wav").ok();
    engine.preload_sample("assets/audio/hurt.wav").ok();

    commands.insert_resource(engine);
}
```

### Volume Control

Store `SoundId` if you need to control sounds after playing:

```rust
#[derive(Resource)]
struct MusicHandle(SoundId);

fn play_music(mut commands: Commands, engine: Res<AudioEngine>) {
    let id = engine.play_sample("assets/audio/music.wav")
        .expect("Failed to play music");

    // Store handle for later control
    commands.insert_resource(MusicHandle(id));
}

fn adjust_music_volume(
    engine: Res<AudioEngine>,
    handle: Res<MusicHandle>,
) {
    engine.set_volume(handle.0, 0.5).ok(); // 50% volume
}
```

### Stopping Sounds

```rust
fn stop_all_audio(engine: Res<AudioEngine>) {
    engine.stop_all();
}

fn stop_specific_sound(
    engine: Res<AudioEngine>,
    handle: Res<MusicHandle>,
) {
    engine.stop(handle.0);
}
```

## Performance

Tunes is designed for real-time game audio:
- **30-45x realtime** with 50 concurrent samples (measured)
- SIMD-accelerated sample playback
- Zero allocations in audio callback
- Automatic cache management

Your game loop won't be blocked by audio playback.

## Advanced: Using Composition System

For dynamic music or complex audio, use the composition system:

```rust
fn play_dynamic_music(
    mut commands: Commands,
    engine: Res<AudioEngine>,
) {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Create adaptive music
    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5], 0.5);

    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12]);

    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer).ok();
}
```

See the [Composition chapter](../concepts/composition.md) for more details.

---

**That's it!** Tunes integrates with Bevy in just a few lines. No complex setup, no audio thread management - just call `play_sample()` and go.
