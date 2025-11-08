# Spatial Audio

Position sounds in 3D space using panning and volume attenuation. Tunes provides real-time control of sound positioning for immersive game audio.

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
