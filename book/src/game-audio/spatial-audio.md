# Spatial Audio

Position sounds in 3D space using panning and volume attenuation. Tunes provides real-time control of sound positioning for immersive game audio.

## Built-in 3D Spatial Audio

Tunes includes a comprehensive 3D spatial audio system with automatic distance attenuation, azimuth-based panning, and listener orientation. This is ideal for games, VR/AR applications, and interactive installations.

**Important:** Spatial positioning applies at the **track level**. All events in a track share the same spatial position. For sounds at different positions, use separate tracks (see examples below).

### Composition-Time Positioning

Position sounds in 3D space when creating a composition:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Guitar 3 meters to the right, 5 meters forward
    comp.instrument("guitar", &Instrument::pluck())
        .spatial_position(3.0, 0.0, 5.0)
        .notes(&[C4, E4, G4, C5], 0.5);

    // Bass at center, 2 meters forward
    comp.instrument("bass", &Instrument::synth_bass())
        .spatial_position(0.0, 0.0, 2.0)
        .notes(&[C2, C2, G2, G2], 1.0);

    // Drums at listener position (origin)
    comp.track("drums")
        .spatial_position(0.0, 0.0, 0.0)
        .drum(DrumType::Kick)
        .wait(0.5)
        .drum(DrumType::Snare);

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

**Coordinate System:**
- **X-axis:** Left (negative) to Right (positive)
- **Y-axis:** Down (negative) to Up (positive)
- **Z-axis:** Behind (negative) to Forward (positive)
- **Listener default:** Position (0, 0, 0) facing +Z direction

### Real-Time Position Updates

Move sounds dynamically during playback.

**Note:** Runtime positioning with `set_sound_position()` **overrides** any composition-time position. This allows you to set a default position in the composition and then control it at runtime.

```rust
use tunes::prelude::*;
use std::thread;
use std::time::Duration;

fn moving_sound_demo() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    let mut comp = Composition::new(Tempo::new(120.0));
    comp.instrument("moving", &Instrument::synth_lead())
        .note(&[A4], 3.0);  // 3 second tone

    let sound_id = engine.play_mixer_realtime(&comp.into_mixer())?;

    // Move sound from left to right over 3 seconds
    // This overrides any composition-time position
    for i in 0..30 {
        let x = -5.0 + (i as f32 * 10.0 / 30.0);  // -5 to +5
        engine.set_sound_position(sound_id, x, 0.0, 5.0)?;
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
```

### Distance Attenuation

Sounds automatically get quieter with distance. Multiple attenuation models are available:

```rust
use tunes::prelude::*;
use tunes::synthesis::spatial::{SpatialParams, AttenuationModel};

fn configure_attenuation() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    let mut params = SpatialParams::default();
    params.attenuation_model = AttenuationModel::InverseSquare;  // Realistic physics
    params.max_distance = 50.0;  // Silent beyond 50 meters
    params.ref_distance = 1.0;   // Full volume within 1 meter
    params.rolloff = 1.0;        // Attenuation steepness

    engine.set_spatial_params(params)?;

    Ok(())
}
```

**Available Attenuation Models:**
- `None` - No distance attenuation (volume constant)
- `Linear` - Linear falloff with distance
- `Inverse` - 1/distance (realistic for sound pressure)
- `InverseSquare` - 1/distance² (default, realistic for sound intensity)
- `Exponential` - Exponential decay with configurable rolloff

### Listener Position and Orientation

Control where the listener is and which direction they're facing:

```rust
use tunes::prelude::*;

fn listener_control() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    // Move listener position (e.g., player moved to new location)
    engine.set_listener_position(5.0, 1.7, 10.0)?;  // x, y (standing height), z

    // Change which direction listener is facing
    // This affects which side sounds come from
    engine.set_listener_forward(1.0, 0.0, 0.0)?;  // Now facing +X (right)

    Ok(())
}
```

### Doppler Effect

Tunes includes a built-in doppler effect system that automatically pitch-shifts sounds based on their velocity relative to the listener. This creates realistic audio for fast-moving objects like cars, aircraft, and projectiles.

```rust
use tunes::prelude::*;
use std::thread;
use std::time::Duration;

fn car_passing_by() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    // Configure doppler effect
    use tunes::synthesis::spatial::SpatialParams;
    let mut params = SpatialParams::default();
    params.doppler_enabled = true;
    params.doppler_factor = 1.0;  // 1.0 = realistic, 2.0 = exaggerated
    engine.set_spatial_params(params)?;

    // Create engine sound
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.instrument("car", &Instrument::synth_bass())
        .filter(Filter::low_pass(400.0, 0.7))
        .note(&[110.0], 5.0);

    let sound_id = engine.play_looping(&comp.into_mixer())?;

    // Simulate car passing from left to right at 30 m/s (~67 mph)
    for i in 0..=50 {
        let t = i as f32 / 50.0;
        let x = -30.0 + (60.0 * t);  // -30m to +30m

        engine.set_sound_position(sound_id, x, 0.0, 5.0)?;
        engine.set_sound_velocity(sound_id, 30.0, 0.0, 0.0)?;  // Moving right

        thread::sleep(Duration::from_millis(100));
    }

    engine.stop(sound_id)?;
    Ok(())
}
```

**Physics:**
- **Approaching sounds** - Higher pitch (sound waves compressed)
- **Receding sounds** - Lower pitch (sound waves stretched)
- **Formula:** `pitch = speed_of_sound / (speed_of_sound - relative_velocity)`
- **Speed of sound:** 343 m/s (realistic physics)
- **Pitch range:** Clamped to 0.5x - 2.0x (one octave range)

**Doppler Configuration:**

```rust
use tunes::synthesis::spatial::SpatialParams;

let mut params = SpatialParams::default();
params.doppler_enabled = true;      // Enable/disable doppler
params.doppler_factor = 1.0;        // Strength multiplier:
                                     // 0.0 = disabled
                                     // 0.5 = subtle
                                     // 1.0 = realistic physics
                                     // 2.0 = exaggerated (arcade games)

engine.set_spatial_params(params)?;
```

**Setting Velocities:**

```rust
// Set sound source velocity (m/s)
engine.set_sound_velocity(sound_id, vx, vy, vz)?;

// Set listener velocity (for moving camera/player)
engine.set_listener_velocity(vx, vy, vz)?;
```

**Use Cases:**
- **Racing games** - Car engine sounds as they pass
- **Flight simulators** - Aircraft flyby effects
- **Action games** - Bullets, projectiles, rockets
- **Open world games** - Vehicles in the distance
- **VR experiences** - Immersive moving sound sources

**Example: Circular Motion**

```rust
// Race car circling the listener
let radius = 20.0;
let angular_velocity = std::f32::consts::PI;  // rad/s

for i in 0..60 {
    let t = i as f32 / 60.0;
    let angle = t * angular_velocity * 6.0;

    let x = radius * angle.cos();
    let z = radius * angle.sin();

    // Calculate tangential velocity
    let vx = -radius * angular_velocity * angle.sin();
    let vz = radius * angular_velocity * angle.cos();

    engine.set_sound_position(sound_id, x, 0.0, z)?;
    engine.set_sound_velocity(sound_id, vx, 0.0, vz)?;

    thread::sleep(Duration::from_millis(100));
}
```

**Complete Example:**

See `examples/doppler_effect_demo.rs` for a comprehensive demonstration with:
- Car passing by (left to right)
- Helicopter flyby (front to back)
- Racing car on circular track
- Doppler factor comparison (disabled/subtle/realistic/exaggerated)

Run it with: `cargo run --example doppler_effect_demo`

### Multi-Source Spatial Scene

Create complex 3D soundscapes with multiple positioned instruments.

**Note:** Each instrument is on a separate track with its own spatial position. All events within each track share that position.

```rust
use tunes::prelude::*;

fn spatial_scene() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut scene = Composition::new(Tempo::new(140.0));

    // Left side: Piano melody (separate track)
    scene.instrument("piano-left", &Instrument::electric_piano())
        .spatial_position(-4.0, 0.0, 6.0)
        .notes(&[C4, E4, G4, E4], 0.375);

    // Right side: Synth harmony (separate track)
    scene.instrument("synth-right", &Instrument::warm_pad())
        .spatial_position(4.0, 0.0, 6.0)
        .note(&[G3, B3, D4], 3.0);

    // Center front: Lead (separate track)
    scene.instrument("lead-center", &Instrument::synth_lead())
        .spatial_position(0.0, 1.0, 3.0)  // Slightly above listener
        .notes(&[E5, G5, B5, D5], 0.375);

    // Behind listener: Ambient pad (separate track)
    scene.instrument("ambient-back", &Instrument::warm_pad())
        .spatial_position(0.0, 0.0, -5.0)  // Negative Z = behind
        .note(&[C3, E3, G3], 3.0);

    engine.play_mixer(&scene.into_mixer())?;
    Ok(())
}
```

### Game Integration Example

Use spatial audio in a game loop:

```rust
use tunes::prelude::*;

struct GameObject {
    position: (f32, f32, f32),
    sound_id: Option<SoundId>,
}

struct Game {
    engine: AudioEngine,
    player_pos: (f32, f32, f32),
    objects: Vec<GameObject>,
}

impl Game {
    fn update_audio(&mut self) -> anyhow::Result<()> {
        // Update listener to player position
        let (x, y, z) = self.player_pos;
        self.engine.set_listener_position(x, y, z)?;

        // Update all spatial sound sources
        for obj in &self.objects {
            if let Some(sound_id) = obj.sound_id {
                let (x, y, z) = obj.position;
                self.engine.set_sound_position(sound_id, x, y, z)?;
            }
        }

        Ok(())
    }

    fn spawn_sound_at_position(&self, pos: (f32, f32, f32)) -> anyhow::Result<SoundId> {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("sfx", &Instrument::synth_lead())
            .spatial_position(pos.0, pos.1, pos.2)
            .note(&[440.0], 0.5);

        self.engine.play_mixer_realtime(&comp.into_mixer())
    }
}
```

### Complete Example

See `examples/spatial_audio_demo.rs` for a comprehensive demonstration showing:
- Static spatial composition
- Moving sounds in real-time
- Distance attenuation
- Listener rotation
- Custom spatial parameters
- Multi-source spatial scenes

Run it with: `cargo run --example spatial_audio_demo`

---

## Manual Spatial Audio Implementation

The following sections show how to implement spatial audio manually using basic pan and volume controls. This is useful for 2D games or custom spatial audio logic.

## Basic Panning

Pan sounds left (-1.0) to right (+1.0):

```rust
use tunes::prelude::*;

fn play_positioned_sound(engine: &AudioEngine, pan: f32) -> anyhow::Result<SoundId> {
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("sfx").note(&[440.0], 1.0);

    let sound_id = engine.play_mixer_realtime(&comp.into_mixer())?;
    engine.set_pan(sound_id, pan)?; // -1.0 (left) to 1.0 (right)

    Ok(sound_id)
}
```

## 2D Spatial Positioning

Convert 2D game coordinates to stereo pan:

```rust
use tunes::prelude::*;

struct Listener {
    x: f32,
}

struct SoundSource {
    x: f32,
    y: f32,
}

fn calculate_pan(listener: &Listener, source: &SoundSource) -> f32 {
    // Calculate relative x position
    let relative_x = source.x - listener.x;

    // Convert to pan (-1.0 to 1.0)
    // Clamp to reasonable range (e.g., 10 units = full stereo width)
    let pan = (relative_x / 10.0).clamp(-1.0, 1.0);

    pan
}

fn play_spatial_2d(
    engine: &AudioEngine,
    listener: &Listener,
    source: &SoundSource,
) -> anyhow::Result<SoundId> {
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("sfx").note(&[200.0], 0.5);

    let sound_id = engine.play_mixer_realtime(&comp.into_mixer())?;

    let pan = calculate_pan(listener, source);
    engine.set_pan(sound_id, pan)?;

    Ok(sound_id)
}
```

## Distance Attenuation

Reduce volume based on distance:

```rust
use tunes::prelude::*;

fn calculate_volume_from_distance(distance: f32, max_distance: f32) -> f32 {
    if distance >= max_distance {
        return 0.0;
    }

    // Linear falloff
    1.0 - (distance / max_distance)

    // Or use inverse square law for more realism:
    // (1.0 / (1.0 + distance * distance)).min(1.0)
}

fn play_with_distance(
    engine: &AudioEngine,
    distance: f32,
) -> anyhow::Result<SoundId> {
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("sfx").note(&[440.0], 1.0);

    let sound_id = engine.play_mixer_realtime(&comp.into_mixer())?;

    let volume = calculate_volume_from_distance(distance, 50.0);
    engine.set_volume(sound_id, volume)?;

    Ok(sound_id)
}
```

## Complete 2D Spatial System

Combine panning and distance attenuation:

```rust
use tunes::prelude::*;

struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn distance_to(&self, other: &Vec2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

struct SpatialAudio {
    engine: AudioEngine,
    listener_pos: Vec2,
    max_hear_distance: f32,
}

impl SpatialAudio {
    fn new() -> anyhow::Result<Self> {
        Ok(Self {
            engine: AudioEngine::new()?,
            listener_pos: Vec2 { x: 0.0, y: 0.0 },
            max_hear_distance: 50.0,
        })
    }

    fn set_listener_position(&mut self, pos: Vec2) {
        self.listener_pos = pos;
    }

    fn play_at_position(
        &self,
        mixer: &Mixer,
        source_pos: Vec2,
    ) -> anyhow::Result<SoundId> {
        let distance = self.listener_pos.distance_to(&source_pos);

        // Don't play if too far
        if distance > self.max_hear_distance {
            return Ok(0); // Return dummy ID
        }

        // Calculate pan based on relative x position
        let relative_x = source_pos.x - self.listener_pos.x;
        let pan = (relative_x / 10.0).clamp(-1.0, 1.0);

        // Calculate volume based on distance
        let volume = calculate_volume_from_distance(distance, self.max_hear_distance);

        // Play with spatial positioning
        let sound_id = self.engine.play_mixer_realtime(mixer)?;
        self.engine.set_pan(sound_id, pan)?;
        self.engine.set_volume(sound_id, volume)?;

        Ok(sound_id)
    }

    fn update_sound_position(
        &self,
        sound_id: SoundId,
        source_pos: Vec2,
    ) -> anyhow::Result<()> {
        let distance = self.listener_pos.distance_to(&source_pos);

        // Stop if moved too far
        if distance > self.max_hear_distance {
            self.engine.stop(sound_id)?;
            return Ok(());
        }

        // Update pan
        let relative_x = source_pos.x - self.listener_pos.x;
        let pan = (relative_x / 10.0).clamp(-1.0, 1.0);
        self.engine.set_pan(sound_id, pan)?;

        // Update volume
        let volume = calculate_volume_from_distance(distance, self.max_hear_distance);
        self.engine.set_volume(sound_id, volume)?;

        Ok(())
    }
}

fn calculate_volume_from_distance(distance: f32, max_distance: f32) -> f32 {
    if distance >= max_distance {
        return 0.0;
    }
    1.0 - (distance / max_distance)
}
```

Usage in a game loop:

```rust
let mut spatial = SpatialAudio::new()?;

// Player moves
spatial.set_listener_position(Vec2 { x: 10.0, y: 5.0 });

// Enemy footstep at position
let enemy_pos = Vec2 { x: 15.0, y: 5.0 };
let footstep = create_footstep_sound();
let sound_id = spatial.play_at_position(&footstep, enemy_pos)?;

// Later: enemy moves
let new_enemy_pos = Vec2 { x: 20.0, y: 10.0 };
spatial.update_sound_position(sound_id, new_enemy_pos)?;
```

## 3D Spatial Positioning

For 3D games, project to stereo:

```rust
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn distance_to(&self, other: &Vec3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

struct Camera {
    position: Vec3,
    forward: Vec3,
    right: Vec3,
}

fn calculate_3d_spatial(
    camera: &Camera,
    source_pos: &Vec3,
) -> (f32, f32) {
    // Vector from camera to source
    let to_source = Vec3 {
        x: source_pos.x - camera.position.x,
        y: source_pos.y - camera.position.y,
        z: source_pos.z - camera.position.z,
    };

    // Dot product with camera's right vector to get left/right
    let pan_factor = to_source.x * camera.right.x
        + to_source.y * camera.right.y
        + to_source.z * camera.right.z;

    let pan = (pan_factor / 10.0).clamp(-1.0, 1.0);

    // Distance for volume
    let distance = camera.position.distance_to(source_pos);

    (pan, distance)
}
```

## Moving Sound Sources

For continuous sounds from moving objects:

```rust
use tunes::prelude::*;
use std::time::Duration;
use std::thread;

struct MovingSoundSource {
    sound_id: SoundId,
    position: Vec2,
}

impl MovingSoundSource {
    fn new(
        engine: &AudioEngine,
        mixer: &Mixer,
        start_pos: Vec2,
    ) -> anyhow::Result<Self> {
        let sound_id = engine.play_looping(mixer)?;
        Ok(Self {
            sound_id,
            position: start_pos,
        })
    }

    fn update(
        &mut self,
        engine: &AudioEngine,
        listener_pos: &Vec2,
        new_pos: Vec2,
    ) -> anyhow::Result<()> {
        self.position = new_pos;

        let distance = listener_pos.distance_to(&self.position);
        let max_distance = 50.0;

        if distance > max_distance {
            engine.stop(self.sound_id)?;
            return Ok(());
        }

        // Update pan
        let relative_x = self.position.x - listener_pos.x;
        let pan = (relative_x / 10.0).clamp(-1.0, 1.0);
        engine.set_pan(self.sound_id, pan)?;

        // Update volume
        let volume = 1.0 - (distance / max_distance);
        engine.set_volume(self.sound_id, volume)?;

        Ok(())
    }
}
```

## Doppler Effect (Simple)

Simulate pitch shift based on relative velocity:

```rust
use tunes::prelude::*;

fn update_with_doppler(
    engine: &AudioEngine,
    sound_id: SoundId,
    relative_velocity: f32,
) -> anyhow::Result<()> {
    // Speed of sound ~343 m/s
    // Simplified doppler: f' = f * (1 + v/c)
    let speed_of_sound = 343.0;
    let doppler_factor = 1.0 + (relative_velocity / speed_of_sound);

    // Clamp to reasonable range
    let playback_rate = doppler_factor.clamp(0.5, 2.0);

    engine.set_playback_rate(sound_id, playback_rate)?;

    Ok(())
}
```

## Performance Consideration

For many spatial sounds, cull distant ones:

```rust
const MAX_SPATIAL_SOUNDS: usize = 32;
const CULL_DISTANCE: f32 = 100.0;

fn cull_distant_sounds(
    sounds: &mut Vec<(SoundId, Vec2)>,
    listener_pos: &Vec2,
    engine: &AudioEngine,
) -> anyhow::Result<()> {
    // Sort by distance
    sounds.sort_by(|a, b| {
        let dist_a = listener_pos.distance_to(&a.1);
        let dist_b = listener_pos.distance_to(&b.1);
        dist_a.partial_cmp(&dist_b).unwrap()
    });

    // Stop sounds beyond max count or cull distance
    for (i, (sound_id, pos)) in sounds.iter().enumerate() {
        let distance = listener_pos.distance_to(pos);

        if i >= MAX_SPATIAL_SOUNDS || distance > CULL_DISTANCE {
            engine.stop(*sound_id)?;
        }
    }

    // Remove stopped sounds
    sounds.retain(|(_, pos)| {
        listener_pos.distance_to(pos) <= CULL_DISTANCE
    });
    sounds.truncate(MAX_SPATIAL_SOUNDS);

    Ok(())
}
```

## Next Steps

- [Concurrent Sound Effects](./concurrent-sfx.md) - Managing multiple simultaneous sounds
- [Dynamic Music Systems](./dynamic-music.md) - Adaptive background music
