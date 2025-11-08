# Dynamic Music Systems

Dynamic music adapts to gameplay—tense during combat, calm during exploration, triumphant during victories. Tunes makes this easy with looping playback and real-time transitions.

## Basic Looping Background Music

The simplest approach is looping a track:

```rust
use tunes::prelude::*;

fn play_ambient_music(engine: &AudioEngine) -> anyhow::Result<SoundId> {
    let mut comp = Composition::new(Tempo::new(90.0));

    comp.instrument("pad", &Instrument::synth_pad())
        .notes(&[C3, E3, G3, C4], 2.0)
        .notes(&[A2, C3, E3, A3], 2.0);

    let loop_id = engine.play_looping(&comp.into_mixer())?;
    Ok(loop_id)
}
```

Stop it when transitioning to a new area:

```rust
engine.stop(loop_id)?;
```

## Layer-Based Music System

Build intensity by adding/removing layers:

```rust
use tunes::prelude::*;

struct LayeredMusic {
    engine: AudioEngine,
    ambient: Mixer,
    tension: Mixer,
    combat: Mixer,
    current_layers: Vec<SoundId>,
}

impl LayeredMusic {
    fn new() -> anyhow::Result<Self> {
        let engine = AudioEngine::new()?;

        Ok(Self {
            engine,
            ambient: Self::create_ambient(),
            tension: Self::create_tension(),
            combat: Self::create_combat(),
            current_layers: Vec::new(),
        })
    }

    fn set_intensity(&mut self, level: u8) -> anyhow::Result<()> {
        // Stop all current layers
        for id in &self.current_layers {
            self.engine.stop(*id)?;
        }
        self.current_layers.clear();

        // Start appropriate layers
        match level {
            0 => {
                // Silent
            }
            1 => {
                // Ambient only
                let id = self.engine.play_looping(&self.ambient)?;
                self.current_layers.push(id);
            }
            2 => {
                // Ambient + tension
                let id1 = self.engine.play_looping(&self.ambient)?;
                let id2 = self.engine.play_looping(&self.tension)?;
                self.current_layers.push(id1);
                self.current_layers.push(id2);
            }
            3 => {
                // Full combat music
                let id1 = self.engine.play_looping(&self.ambient)?;
                let id2 = self.engine.play_looping(&self.tension)?;
                let id3 = self.engine.play_looping(&self.combat)?;
                self.current_layers.push(id1);
                self.current_layers.push(id2);
                self.current_layers.push(id3);
            }
            _ => {}
        }

        Ok(())
    }

    fn create_ambient() -> Mixer {
        let mut comp = Composition::new(Tempo::new(80.0));
        comp.instrument("pad", &Instrument::synth_pad())
            .notes(&[C2, E2, G2], 4.0);
        comp.into_mixer()
    }

    fn create_tension() -> Mixer {
        let mut comp = Composition::new(Tempo::new(80.0));
        comp.instrument("strings", &Instrument::string_ensemble())
            .notes(&[C3, D3, E3, F3], 1.0);
        comp.into_mixer()
    }

    fn create_combat() -> Mixer {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("drums")
            .drum_grid(16, 0.125)
            .kick(&[0, 4, 8, 12])
            .snare(&[4, 12]);
        comp.into_mixer()
    }
}
```

Usage in your game loop:

```rust
let mut music = LayeredMusic::new()?;

// Exploration
music.set_intensity(1)?;

// Enemy spotted
music.set_intensity(2)?;

// Combat!
music.set_intensity(3)?;

// Back to safe
music.set_intensity(1)?;
```

## Crossfading Between Tracks

Smooth transitions by fading out one track while fading in another:

```rust
use tunes::prelude::*;
use std::thread;
use std::time::Duration;

fn crossfade(
    engine: &AudioEngine,
    old_id: SoundId,
    new_mixer: &Mixer,
    duration_ms: u64,
) -> anyhow::Result<SoundId> {
    // Start new track at 0 volume
    let new_id = engine.play_looping(new_mixer)?;
    engine.set_volume(new_id, 0.0)?;

    // Crossfade over duration
    let steps = 20;
    let step_duration = duration_ms / steps;

    for i in 0..=steps {
        let progress = i as f32 / steps as f32;
        engine.set_volume(old_id, 1.0 - progress)?;
        engine.set_volume(new_id, progress)?;
        thread::sleep(Duration::from_millis(step_duration));
    }

    // Stop old track
    engine.stop(old_id)?;

    Ok(new_id)
}
```

## Section-Based Music

Use `section()` and `arrange()` for structured music:

```rust
use tunes::prelude::*;

fn create_adaptive_music() -> anyhow::Result<Mixer> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Define sections
    comp.section("intro", |c| {
        c.instrument("piano", &Instrument::electric_piano())
            .notes(&[C4, E4, G4, C5], 0.5);
    });

    comp.section("explore", |c| {
        c.instrument("pad", &Instrument::synth_pad())
            .notes(&[C3, E3, G3], 2.0);
    });

    comp.section("combat", |c| {
        c.track("drums")
            .drum_grid(16, 0.125)
            .kick(&[0, 4, 8, 12])
            .snare(&[4, 12]);
        c.instrument("bass", &Instrument::sub_bass())
            .notes(&[C2, C2, G2, G2], 0.5);
    });

    comp.section("victory", |c| {
        c.instrument("trumpet", &Instrument::trumpet())
            .notes(&[C4, E4, G4, C5], 0.5);
    });

    // Arrange based on gameplay
    comp.arrange(&["intro", "explore", "explore", "combat", "victory"]);

    Ok(comp.into_mixer())
}
```

## Procedural Music with Game State

Generate music based on game variables:

```rust
use tunes::prelude::*;
use tunes::sequences;

fn create_procedural_music(danger_level: f32, player_health: f32) -> Mixer {
    let mut comp = Composition::new(Tempo::new(60.0 + danger_level * 60.0));

    // Higher danger = more notes
    let note_count = (4.0 + danger_level * 12.0) as usize;

    // Lower health = lower pitch range
    let base_pitch = C3 + (player_health * 24.0);

    // Use euclidean rhythms for tension
    let rhythm = sequences::euclidean(note_count, 16);
    let pitches = sequences::map_to_scale(
        &rhythm,
        &sequences::Scale::minor_pentatonic(),
        base_pitch,
        2,
    );

    comp.instrument("lead", &Instrument::synth_lead())
        .notes(&pitches, 0.25);

    // Add drums when danger is high
    if danger_level > 0.5 {
        comp.track("drums")
            .drum_grid(16, 0.125)
            .kick(&[0, 8])
            .snare(&[4, 12]);
    }

    comp.into_mixer()
}
```

Usage:

```rust
// Update music as game state changes
let danger = calculate_danger_level(); // 0.0 - 1.0
let health = player.health / player.max_health; // 0.0 - 1.0

let music = create_procedural_music(danger, health);
engine.play_looping(&music)?;
```

## Stinger Events

Short musical accents for important events:

```rust
use tunes::prelude::*;

fn play_level_up_stinger(engine: &AudioEngine) -> anyhow::Result<SoundId> {
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("fanfare", &Instrument::trumpet())
        .note(&[C4], 0.2)
        .note(&[E4], 0.2)
        .note(&[G4], 0.2)
        .note(&[C5], 0.6);

    engine.play_mixer_realtime(&comp.into_mixer())
}

fn play_death_stinger(engine: &AudioEngine) -> anyhow::Result<SoundId> {
    let mut comp = Composition::new(Tempo::new(60.0));

    comp.instrument("ominous", &Instrument::synth_pad())
        .note(&[C2], 1.0)
        .fade_to(C1, 1.0)
        .reverb(Reverb::new(0.8, 0.7));

    engine.play_mixer_realtime(&comp.into_mixer())
}
```

## Complete Example: State-Based Music Manager

```rust
use tunes::prelude::*;

enum GameState {
    Menu,
    Exploration,
    Combat,
    Boss,
    Victory,
}

struct MusicManager {
    engine: AudioEngine,
    current_loop: Option<SoundId>,
    menu_music: Mixer,
    exploration_music: Mixer,
    combat_music: Mixer,
    boss_music: Mixer,
    victory_music: Mixer,
}

impl MusicManager {
    fn new() -> anyhow::Result<Self> {
        Ok(Self {
            engine: AudioEngine::new()?,
            current_loop: None,
            menu_music: Self::create_menu_music(),
            exploration_music: Self::create_exploration_music(),
            combat_music: Self::create_combat_music(),
            boss_music: Self::create_boss_music(),
            victory_music: Self::create_victory_music(),
        })
    }

    fn set_state(&mut self, state: GameState) -> anyhow::Result<()> {
        // Stop current music
        if let Some(id) = self.current_loop {
            self.engine.stop(id)?;
        }

        // Start new music based on state
        let mixer = match state {
            GameState::Menu => &self.menu_music,
            GameState::Exploration => &self.exploration_music,
            GameState::Combat => &self.combat_music,
            GameState::Boss => &self.boss_music,
            GameState::Victory => &self.victory_music,
        };

        self.current_loop = Some(self.engine.play_looping(mixer)?);
        Ok(())
    }

    fn create_menu_music() -> Mixer {
        let mut comp = Composition::new(Tempo::new(100.0));
        comp.instrument("piano", &Instrument::electric_piano())
            .notes(&[C4, E4, G4, E4], 1.0);
        comp.into_mixer()
    }

    fn create_exploration_music() -> Mixer {
        let mut comp = Composition::new(Tempo::new(80.0));
        comp.instrument("pad", &Instrument::synth_pad())
            .notes(&[C3, E3, G3], 4.0);
        comp.into_mixer()
    }

    fn create_combat_music() -> Mixer {
        let mut comp = Composition::new(Tempo::new(140.0));
        comp.track("drums")
            .drum_grid(16, 0.125)
            .kick(&[0, 4, 8, 12])
            .snare(&[4, 12]);
        comp.into_mixer()
    }

    fn create_boss_music() -> Mixer {
        let mut comp = Composition::new(Tempo::new(160.0));
        comp.track("drums")
            .drum_grid(16, 0.125)
            .kick(&[0, 2, 4, 6, 8, 10, 12, 14])
            .snare(&[4, 12]);
        comp.instrument("bass", &Instrument::sub_bass())
            .notes(&[C1, C1, C1, D1], 0.25)
            .distortion(Distortion::new(0.6));
        comp.into_mixer()
    }

    fn create_victory_music() -> Mixer {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("fanfare", &Instrument::trumpet())
            .notes(&[C4, E4, G4, C5], 0.5);
        comp.into_mixer()
    }
}
```

## Next Steps

- [Spatial Audio](./spatial-audio.md) - Position sounds in 3D space
- [Concurrent Sound Effects](./concurrent-sfx.md) - Playing multiple sounds at once
