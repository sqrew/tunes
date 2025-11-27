# Quick Reference

A concise guide to common Tunes operations. For detailed explanations, see the full documentation.

## Table of Contents

- [Installation](#installation)
- [Basic Setup](#basic-setup)
- [Playing Notes & Chords](#playing-notes--chords)
- [Instruments & Synthesis](#instruments--synthesis)
- [Drums & Rhythm](#drums--rhythm)
- [Effects](#effects)
- [Scales & Music Theory](#scales--music-theory)
- [Sample Playback (Game Audio)](#sample-playback-game-audio)
- [Streaming (Long Audio Files)](#streaming-long-audio-files)
- [Spatial Audio (3D Sound)](#spatial-audio-3d-sound)
- [Export Audio](#export-audio)
- [Import MIDI](#import-midi)
- [Real-time Control](#real-time-control)
- [Algorithmic Patterns](#algorithmic-patterns)
- [Sections & Arrangements](#sections--arrangements)
- [Live Coding (Hot Reload)](#live-coding-hot-reload)
- [Game Engine Integration](#game-engine-integration)
  - [Bevy](#bevy)
  - [ggez](#ggez)
  - [macroquad](#macroquad)
- [GPU Acceleration](#gpu-acceleration)
- [WebAssembly](#webassembly)
- [Common Patterns](#common-patterns)
  - [Dynamic Music System](#dynamic-music-system)
  - [Procedural Sound Effects](#procedural-sound-effects)
- [Performance Tips](#performance-tips)
- [Full Documentation](#full-documentation)

---

## Installation

```toml
[dependencies]
tunes = "1.0.2"

# Optional features
tunes = { version = "1.0.2", features = ["gpu"] }    # GPU acceleration
tunes = { version = "1.0.2", features = ["web"] }    # WebAssembly support
```

## Basic Setup

```rust
use tunes::prelude::*;

let engine = AudioEngine::new()?;                    // Standard
let engine = AudioEngine::new_with_gpu()?;           // With GPU (requires "gpu" feature)
let engine = AudioEngine::with_buffer_size(4096)?;   // Custom buffer size
```

## Playing Notes & Chords

```rust
let mut comp = Composition::new(Tempo::new(120.0));

// Single note
comp.track("melody").note(&[C4], 0.5);

// Multiple notes (sequence)
comp.track("melody").notes(&[C4, E4, G4, C5], 0.5);

// Chord (simultaneous)
comp.track("harmony").note(&[C4, E4, G4], 1.0);

// Play it
engine.play_mixer(&comp.into_mixer())?;
```

## Instruments & Synthesis

```rust
// Use preset instruments
comp.instrument("piano", &Instrument::electric_piano())
    .notes(&[C4, E4, G4], 0.5);

comp.instrument("bass", &Instrument::sub_bass())
    .notes(&[C2, G2], 1.0);

// Raw synthesis
comp.track("synth")
    .sine(440.0, 1.0)           // Frequency, duration
    .sawtooth(880.0, 0.5)
    .square(220.0, 0.5);
```

## Drums & Rhythm

```rust
// Drum grid (closure-based API)
comp.track("drums")
    .drum_grid(16, 0.125, |g| g  // 16 steps, 1/16th notes
        .sound(DrumType::Kick, &[0, 4, 8, 12])       // Kick on steps 0, 4, 8, 12
        .sound(DrumType::Snare, &[4, 12])            // Snare on steps 4, 12
        .sound(DrumType::HiHatClosed, &[0, 2, 4, 6, 8, 10, 12, 14]));  // Hi-hats

// Euclidean rhythms (using sequences)
let pattern = sequences::euclidean::generate(5, 8);  // 5 hits over 8 steps
comp.track("perc")
    .drum_grid(8, 0.125, |g| g
        .sound(DrumType::Rimshot, &pattern));
```

## Effects

```rust
// Single effect
comp.track("melody")
    .notes(&[C4, E4, G4], 0.5)
    .reverb(Reverb::new(0.8, 0.5, 0.3));  // room_size, damping, mix

// Effect chain
comp.track("lead")
    .notes(&[C5, E5, G5], 0.5)
    .filter(Filter::low_pass(1200.0, 0.7))
    .delay(Delay::new(0.25, 0.5, 0.3))     // time, feedback, mix
    .distortion(Distortion::new(0.5, 0.7))  // drive, mix
    .reverb(Reverb::new(0.5, 0.6, 0.2));

// Filter sweep (automation)
comp.track("sweep")
    .notes(&[C4, E4, G4], 2.0)
    .filter(Filter::low_pass(200.0, 0.5))
    .filter_sweep(2000.0, 2.0);            // target_cutoff, duration
```

## Scales & Music Theory

```rust
use tunes::sequences;

// Map sequence to musical scale
let fib = sequences::fibonacci::generate(8);
let melody = sequences::map_to_scale(&fib, &sequences::Scale::major_pentatonic(), C4, 2);
comp.track("melody").notes(&melody, 0.5);

// Available scales
let major = sequences::Scale::major();
let minor = sequences::Scale::minor();
let blues = sequences::Scale::blues();
let pentatonic = sequences::Scale::major_pentatonic();

// Map to minor scale
let primes = sequences::primes::generate(10);
let dark_melody = sequences::map_to_scale(&primes, &sequences::Scale::minor(), A4, 2);
comp.track("dark").notes(&dark_melody, 0.5);

// Chord progressions (direct notes)
comp.track("chords")
    .note(&[C4, E4, G4], 1.0)      // C major
    .note(&[G4, B4, D5], 1.0)      // G major
    .note(&[A4, C5, E5], 1.0)      // A minor
    .note(&[F4, A4, C5], 1.0);     // F major
```

## Sample Playback (Game Audio)

```rust
// Fire-and-forget (simple)
engine.play_sample("explosion.wav");   // plays sample, caches
play_sample!(engine, "explosion.wav"); // does the same but also has runtime startup file validation and compile time path resolution

// Pre-load for instant playback
engine.preload_sample("jump.wav")?;
engine.play_sample("jump.wav");              // Instant! Already cached

// Sample playback in compositions (precise timing)
comp.load_sample("kick", "samples/kick.wav")?;
comp.track("drums")
    .sample("kick")
    .sample_with_rate("kick", 1.5);           // 1.5x speed (pitch up)
```

## Streaming (Long Audio Files)

```rust
// Stream background music without loading entire file
let music_id = engine.stream_file("background_music.mp3")?;

// Loop forever
let music_id = engine.stream_file_looping("music_loop.mp3")?;

// Control streaming
engine.set_stream_volume(music_id, 0.5)?;     // 50% volume
engine.set_stream_pan(music_id, -0.5)?;       // Pan left
engine.pause_stream(music_id)?;
engine.resume_stream(music_id)?;
engine.stop_stream(music_id)?;
```

## Spatial Audio (3D Sound)

```rust
// Position sound in 3D space (x, y, z)
engine.play_sample("footstep.wav")
    .spatial(5.0, 0.0, 3.0)      // 5m right, 0m up, 3m forward
    .volume(0.8);

// Multiple positioned sounds
engine.play_sample("ambient.wav")
    .spatial(-10.0, 2.0, 0.0)    // Left and slightly above
    .volume(0.5);

// Set listener position (for proper 3D audio calculation)
engine.set_listener_position(0.0, 0.0, 0.0);  // Player at origin
```

## Export Audio

```rust
let mut mixer = comp.into_mixer();

// WAV (uncompressed)
mixer.export_wav("output.wav", 44100)?;

// FLAC (lossless, ~50% smaller)
mixer.export_flac("output.flac", 44100)?;

// MIDI
mixer.export_midi("output.mid")?;

// STEM export (separate tracks)
mixer.export_stems("stems/", 44100)?;
```

## Import MIDI

```rust
// Import and play
let mut mixer = Mixer::import_midi("song.mid")?;
engine.play_mixer(&mixer)?;

// Import and export to audio
let mut mixer = Mixer::import_midi("song.mid")?;
mixer.export_wav("output.wav", 44100)?;
```

## Real-time Control

```rust
// Non-blocking playback (returns SoundId)
let id = engine.play_mixer_realtime(&comp.into_mixer())?;

// Control while playing
engine.set_volume(id, 0.5)?;                  // 50% volume
engine.set_pan(id, -0.5)?;                    // Pan left
engine.set_playback_rate(id, 1.5)?;           // 1.5x speed
engine.stop(id)?;                             // Stop

// Looping playback
let loop_id = engine.play_looping(&comp.into_mixer())?;
engine.stop(loop_id)?;
```

## Algorithmic Patterns

```rust
use tunes::sequences;

// Fibonacci sequence
let fib = sequences::fibonacci::generate(8);
let melody = sequences::normalize(&fib, 200.0, 800.0);
comp.track("fib").notes(&melody, 0.25);

// Prime numbers
let primes = sequences::primes::generate(10);
let melody = sequences::normalize(&primes, 220.0, 880.0);
comp.track("primes").notes(&melody, 0.25);

// Collatz conjecture
let collatz = sequences::collatz::generate(27, 40);
let melody = sequences::normalize(&collatz, 150.0, 700.0);
comp.track("collatz").notes(&melody, 0.15);

// Euclidean rhythm (for drums)
let kick = sequences::euclidean::generate(4, 16);     // [0, 4, 8, 12]
comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, &kick));

// Random walk
let walk = sequences::random_walk::generate(440.0, 20.0, 20);
comp.track("walk").notes(&walk, 0.25);

// Chaos theory (Logistic Map)
let chaotic = sequences::logistic_map::generate(3.9, 0.5, 32);
let melody = sequences::normalize(
    &chaotic.iter().map(|&x| (x * 100.0) as u32).collect::<Vec<_>>(),
    200.0, 800.0
);
comp.track("chaos").notes(&melody, 0.2);
```

## Sections & Arrangements

```rust
// Define sections
comp.section("verse")
    .instrument("bass", &Instrument::pluck())
    .notes(&[C2, C2, G2, C2], 0.5);

comp.section("chorus")
    .instrument("lead", &Instrument::synth_lead())
    .notes(&[C5, E5, G5], 0.5);

// Arrange sections
comp.arrange(&["verse", "verse", "chorus", "verse", "chorus", "chorus"]);
```

## Live Coding (Hot Reload)

```bash
# Start live coding mode
cargo run --bin tunes-live -- my_composition.rs

# Edit my_composition.rs and save - changes play instantly!
```

```rust
// my_composition.rs
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(140.0));

    comp.track("drums")
        .drum_grid(16, 0.125, |g| g
            .sound(DrumType::Kick, &[0, 4, 8, 12]));

    let mixer = comp.into_mixer();
    let engine = AudioEngine::with_buffer_size(4096)?;
    let loop_id = engine.play_looping(&mixer)?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
```

## Game Engine Integration

### Bevy

```rust
use bevy::prelude::*;
use tunes::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AudioEngine::new().unwrap())
        .add_systems(Update, game_audio)
        .run();
}

fn game_audio(engine: Res<AudioEngine>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        engine.play_sample("jump.wav").ok();
    }
}
```

### ggez

```rust
use ggez::event::{self, EventHandler};
use tunes::prelude::*;

struct GameState {
    audio: AudioEngine,
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if some_collision {
            self.audio.play_sample("explosion.wav")?;
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult { Ok(()) }
}
```

### macroquad

```rust
use macroquad::prelude::*;
use tunes::prelude::*;

#[macroquad::main("Game")]
async fn main() {
    let audio = AudioEngine::new().unwrap();

    loop {
        if is_key_pressed(KeyCode::Space) {
            audio.play_sample("jump.wav").ok();
        }

        next_frame().await
    }
}
```

## GPU Acceleration

```rust
// Enable at engine creation (transparent API)
let engine = AudioEngine::new_with_gpu()?;

// All operations automatically GPU-accelerated
engine.export_wav(&mut comp.into_mixer(), "output.wav")?;
engine.play_mixer_realtime(&mixer)?;

// Or enable per-composition
let mixer = comp.into_mixer_with_gpu();
engine.play_mixer(&mixer)?;
```

## WebAssembly

```toml
[dependencies]
tunes = { version = "1.0.2", features = ["web"] }
```

```bash
wasm-pack build --target web --features web
```

```rust
use wasm_bindgen::prelude::*;
use tunes::prelude::*;

#[wasm_bindgen]
pub fn play_music() -> Result<(), JsValue> {
    let engine = AudioEngine::new()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("melody").notes(&[C4, E4, G4, C5], 0.5);

    engine.play_mixer(&comp.into_mixer())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}
```

## Common Patterns

### Dynamic Music System

```rust
enum GameState { Menu, Playing, BossFight }

fn play_music_for_state(engine: &AudioEngine, state: GameState) -> Result<SoundId> {
    let mut comp = Composition::new(Tempo::new(120.0));

    match state {
        GameState::Menu => {
            comp.instrument("pad", &Instrument::synth_pad())
                .notes(&[C4, E4, G4], 2.0);
        }
        GameState::Playing => {
            comp.track("drums")
                .drum_grid(16, 0.125)
                .kick(&[0, 4, 8, 12]);
        }
        GameState::BossFight => {
            comp.instrument("bass", &Instrument::sub_bass())
                .notes(&[C2, C2, D2, C2], 0.25);
        }
    }

    engine.play_looping(&comp.into_mixer())
}
```

### Procedural Sound Effects

```rust
fn laser_sound(frequency: f32) -> Composition {
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("laser")
        .note(&[frequency], 0.05)
        .filter(Filter::low_pass(frequency * 2.0, 0.5))
        .filter_sweep(frequency * 0.5, 0.05);
    comp
}

// Generate unique laser sounds
engine.play_mixer(&laser_sound(440.0).into_mixer())?;
engine.play_mixer(&laser_sound(880.0).into_mixer())?;
```

---

## Performance Tips

- Pre-load frequently used samples with `preload_sample() to avoid first-time lookup (caches sample)`
- Use `play_sample!()` for fire-and-forget game audio, compile time path resolution and startup file validation
- Use `stream_file()` for long audio files (music, ambience)
- SIMD and multi-core parallelism are automatic
- GPU acceleration helps with heavy synthesis workloads (if you have a discrete GPU) or wav exporting

## Full Documentation

- [Getting Started Guide](./getting-started/installation.md)
- [Core Concepts](./concepts/architecture.md)
- [Game Audio Patterns](./game-audio/samples.md)
- [Synthesis & Effects](./synthesis/basics.md)
- [API Reference](./api-reference.md)
