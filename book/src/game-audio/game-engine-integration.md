# Game Engine Integration

Tunes is **framework-agnostic** and integrates trivially with any Rust game engine. No wrapper crate needed - just store `AudioEngine` in your game state and call `play_sample()`.

> **🎮 Universal Pattern:** All Rust game engines follow the same pattern: create `AudioEngine`, store it, call methods. That's it.

## Integration Pattern

The integration pattern is identical across all engines:

1. **Add dependency:** `tunes = "0.16.0"` in `Cargo.toml`
2. **Create engine:** `AudioEngine::new()` or `AudioEngine::new_with_gpu()`
3. **Store in game state:** Engine resource, struct field, or global
4. **Call from game logic:** `engine.play_sample("sound.wav")`

Choose your engine below for specific examples:
- [Bevy](#bevy)
- [ggez](#ggez)
- [macroquad](#macroquad)
- [bracket-lib](#bracket-lib)
- [Custom Engine](#custom-engine)

---

## Bevy

Bevy integration uses the ECS resource system.

**1. Add dependencies to `Cargo.toml`:**

```toml
[dependencies]
bevy = "0.14"
tunes = "0.16.0"
```

**2. Create the AudioEngine resource:**

**Option A: Simple (Recommended)**

```rust
use bevy::prelude::*;
use tunes::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AudioEngine::new().unwrap())  // That's it!
        .add_systems(Update, game_audio_system)
        .run();
}
```

**Option B: With Startup System**

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_audio)
        .add_systems(Update, game_audio_system)
        .run();
}

fn setup_audio(mut commands: Commands) {
    let engine = AudioEngine::new().expect("Failed to initialize audio");
    commands.insert_resource(engine);
}
```

**Option C: With GPU Acceleration (500-5000x faster)**

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AudioEngine::new_with_gpu().unwrap())  // GPU enabled!
        .add_systems(Update, game_audio_system)
        .run();
}
```

> **💡 GPU Tip:** Use `new_with_gpu()` if you have a discrete GPU (RTX, RX series). Tunes will automatically detect integrated GPUs and warn if they're slower than CPU.

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
- ✅ Caching (repeated sounds are instant, Arc-based sharing)
- ✅ Concurrent playback (100+ sounds simultaneously)
- ✅ SIMD acceleration (47x realtime measured)
- ✅ Multi-core parallelism (Rayon, 54x realtime measured)
- ✅ Automatic mixing (zero-copy where possible)
- ✅ Optional GPU acceleration (500-5000x realtime with discrete GPUs)

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

Tunes is designed for exceptional real-time game audio performance:

**Measured on i5-6500 @ 3.2GHz:**
- **47x realtime** with 50 concurrent samples (SIMD acceleration)
- **54x realtime** with multi-core parallelism (Rayon)
- **81x realtime** for CPU synthesis baseline
- **500-5000x realtime** with GPU acceleration (discrete GPUs - projected)

**Technical details:**
- Zero allocations in audio callback
- SIMD vectorization (AVX2/SSE/NEON) automatic
- Block processing (512-sample chunks)
- Lock-free sample playback where possible
- Automatic cache management (LRU eviction)

**Your game loop won't be blocked by audio playback.** Audio rendering happens on a dedicated thread with lock-free communication.

## GPU Acceleration for Games

If you have a discrete GPU (RTX, RX series), enable GPU acceleration for massive performance gains:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AudioEngine::new_with_gpu().unwrap())
        .add_systems(Update, game_audio)
        .run();
}
```

**What this enables:**
- Pre-render 1,000+ unique sound variations at startup in ~100ms
- Procedural sound generation without performance cost
- Unique audio for every game object (bullets, enemies, projectiles)

**Example: Bullet hell game**

```rust
use tunes::synthesis::fm_synthesis::FMParams;

fn spawn_projectiles(
    mut commands: Commands,
    engine: Res<AudioEngine>,
) {
    for i in 0..1000 {
        // Each projectile gets a unique synthesized sound
        let freq = 200.0 + (i as f32 * 5.0);

        // GPU renders this instantly (projected: 10,000-30,000 notes/sec)
        let mut comp = Composition::new(Tempo::new(140.0));
        comp.track("laser")
            .note(&[freq], 0.05)
            .fm(FMParams::new(3.0, 8.0));

        engine.play_mixer(&comp.into_mixer_with_gpu()).ok();

        commands.spawn(ProjectileBundle { /* ... */ });
    }
}
```

**Performance comparison:**
- CPU: ~1,500 notes/second (still fast!)
- GPU (discrete): ~10,000-30,000 notes/second (projected)
- Result: 1,000 unique sounds generated in 50-100ms

**Note:** Integrated GPUs (Intel HD, AMD Vega) may be slower than CPU. Tunes detects this and displays a warning automatically.

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

## ggez

ggez integration stores `AudioEngine` in your game state struct.

**Setup:**

```toml
[dependencies]
ggez = "0.9"
tunes = "0.16.0"
```

**Basic Integration:**

```rust
use ggez::event::{self, EventHandler};
use ggez::{Context, GameResult};
use tunes::prelude::*;

struct GameState {
    audio: AudioEngine,
    // ... other game state ...
}

impl GameState {
    fn new() -> GameResult<GameState> {
        Ok(GameState {
            audio: AudioEngine::new_with_gpu().unwrap(),
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Play audio from game logic
        if some_collision {
            self.audio.play_sample("assets/explosion.wav").ok();
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("game", "author")
        .build()?;
    let state = GameState::new()?;
    event::run(ctx, event_loop, state)
}
```

**Key Points:**
- Store `AudioEngine` in `GameState` struct
- Create in `new()` with `AudioEngine::new()` or `new_with_gpu()`
- Call `play_sample()` from `update()` or `draw()`

---

## macroquad

macroquad integration is the simplest - just create `AudioEngine` at the start of `main()`.

**Setup:**

```toml
[dependencies]
macroquad = "0.4"
tunes = "0.16.0"
```

**Basic Integration:**

```rust
use macroquad::prelude::*;
use tunes::prelude::*;

#[macroquad::main("Game")]
async fn main() {
    // Create audio engine (GPU-accelerated for max performance)
    let audio = AudioEngine::new_with_gpu().unwrap();

    // Pre-load common sounds
    audio.preload_sample("assets/jump.wav").ok();
    audio.preload_sample("assets/coin.wav").ok();

    loop {
        // Game logic
        if is_key_pressed(KeyCode::Space) {
            audio.play_sample("assets/jump.wav").ok();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            audio.play_sample("assets/shoot.wav").ok();
        }

        // Draw
        clear_background(BLACK);
        draw_text("Press SPACE to jump", 10.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
}
```

**Key Points:**
- Create `AudioEngine` once at start of `main()`
- Use directly in game loop (no state struct needed)
- Works seamlessly with macroquad's async model

---

## bracket-lib

bracket-lib (formerly RLTK) integration uses the state pattern.

**Setup:**

```toml
[dependencies]
bracket-lib = "0.8"
tunes = "0.16.0"
```

**Basic Integration:**

```rust
use bracket_lib::prelude::*;
use tunes::prelude::*;

struct State {
    audio: AudioEngine,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Game logic
        if ctx.key == Some(VirtualKeyCode::Space) {
            self.audio.play_sample("assets/attack.wav").ok();
        }

        // Rendering...
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike")
        .build()?;

    let gs = State {
        audio: AudioEngine::new_with_gpu().unwrap(),
    };

    main_loop(context, gs)
}
```

**Key Points:**
- Store `AudioEngine` in your `State` struct
- Create during state initialization
- Call from `tick()` method

---

## Custom Engine

If you're building your own engine, the pattern is the same:

```rust
use tunes::prelude::*;

struct GameEngine {
    audio: AudioEngine,
    // ... your engine state ...
}

impl GameEngine {
    fn new() -> Self {
        Self {
            audio: AudioEngine::new_with_gpu().unwrap(),
        }
    }

    fn update(&mut self) {
        // Game logic triggers audio
        if self.player_jumped() {
            self.audio.play_sample("jump.wav").ok();
        }
    }
}

fn main() {
    let mut game = GameEngine::new();

    loop {
        game.update();
        game.render();
    }
}
```

**Key Points:**
- Store `AudioEngine` in your main game/engine struct
- Initialize once during engine creation
- Call methods from your game loop

---

## Performance Across All Engines

Performance is identical across all engines:

**Measured on i5-6500 @ 3.2GHz:**
- **47x realtime** with 50 concurrent samples (SIMD acceleration)
- **54x realtime** with multi-core parallelism (Rayon)
- **81x realtime** for CPU synthesis baseline
- **500-5000x realtime** with GPU acceleration (discrete GPUs - projected)

**Your choice of engine doesn't affect Tunes performance.** The audio runs on a dedicated thread with lock-free communication regardless of which engine you use.

---

**That's it!** Tunes integrates with any Rust game engine in just a few lines. No complex setup, no audio thread management, no wrapper crates - just call `play_sample()` and go.

**Need synthesis or composition?** See the [Composition chapter](../concepts/composition.md) for dynamic music and procedural audio.
