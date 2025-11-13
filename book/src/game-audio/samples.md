# Working with Samples

**Want to just play a sound effect? Skip everything and jump to [Quick Start: Fire-and-Forget](#quick-start-fire-and-forget-sound-effects) below.**

---

## The Absolute Easiest Way to Play Audio

Seriously. One line of setup, one line per sound. That's it.

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    // That's it. You're done. Play sounds anywhere:
    engine.play_sample("explosion.wav")?;
    engine.play_sample("footstep.ogg")?;
    engine.play_sample("coin.mp3")?;

    // They all play immediately, non-blocking, concurrent
    // Your game loop keeps running. No configuration needed.

    Ok(())
}
```

**Why this matters:**

- 🎮 **Perfect for game jams** - Get audio working in 30 seconds
- 🚀 **Zero learning curve** - If you can call a function, you can play audio
- 🎯 **Indie dev friendly** - Prototype fast, optimize later
- 🎵 **Simpler than Kira, Rodio, odd-io** - Compare for yourself
- ⚡ **Non-blocking by default** - Won't freeze your game loop
- 🔊 **Concurrent playback built-in** - Play dozens of sounds at once, it just works
- ✨ **Automatic caching** - Repeated sounds are instant (no manual pre-loading needed!)

**The simplest audio API in Rust, with smart performance built-in.**

When you need more power (effects, timing, synthesis), the full Composition system is there. But for "I just want to play a sound," you're already done reading.

### ✨ Automatic Caching (No Performance Worries!)

**Good news:** `play_sample()` automatically caches samples by path. The first call loads from disk, all subsequent calls use the cached version (instant Arc clone).

```rust
let engine = AudioEngine::new()?;

// First call: loads from disk (~1-10ms)
engine.play_sample("footstep.wav")?;

// Subsequent calls: instant! (uses cache)
engine.play_sample("footstep.wav")?;  // ⚡ instant
engine.play_sample("footstep.wav")?;  // ⚡ instant
engine.play_sample("footstep.wav")?;  // ⚡ instant

// You can spam this in your game loop - it's fast!
for _ in 0..100 {
    engine.play_sample("footstep.wav")?;  // All instant after first load
}
```

**This means:**
- ✅ Spam footsteps? Fast!
- ✅ Machine gun fire? Fast!
- ✅ Rain drops? Fast!
- ✅ Repeated UI sounds? Fast!
- ✅ Zero manual cache management needed!
- ✅ SIMD-accelerated playback (4-8 samples processed simultaneously)

**Optional: Pre-load during initialization to eliminate first-load delay:**

```rust
// During game initialization
engine.preload_sample("footstep.wav")?;
engine.preload_sample("jump.wav")?;
engine.preload_sample("explosion.wav")?;

// Now ALL calls are instant (even first one)
engine.play_sample("footstep.wav")?;  // ⚡ instant
```

**Optional: Cache management for memory control:**

```rust
// Clear specific sample when done with it
engine.remove_cached_sample("level1_boss.wav")?;

// Clear all cached samples between levels
engine.clear_sample_cache()?;
```

**You don't need to worry about caching - it just works!**

---

## When to Use What

**Use `engine.play_sample("file.wav")?` when:**
- ✅ Game sound effects (footsteps, explosions, UI clicks, impacts)
- ✅ Any repeated sounds (automatic caching makes this fast!)
- ✅ Prototyping / game jams / rapid development
- ✅ You just want a sound to play RIGHT NOW
- ✅ Simplicity is priority #1

**Use the full Composition API when:**
- Complex timing and rhythms
- Applying effects (reverb, delay, distortion, etc.)
- Procedural/generative music
- Precise control over playback
- Pitch shifting via playback rate

Both are available. Start simple, grow as needed.

---

## Loading Your First Sample

Tunes supports multiple audio formats with automatic format detection:

```rust
use tunes::prelude::*;
use tunes::synthesis::Sample;

fn main() -> anyhow::Result<()> {
    // Load any supported audio file - format is automatically detected
    let kick = Sample::from_file("assets/audio/kick.mp3")?;
    let snare = Sample::from_file("assets/audio/snare.ogg")?;
    let hihat = Sample::from_file("assets/audio/hihat.flac")?;

    println!("Loaded sample:");
    println!("  Duration: {:.2}s", kick.duration);
    println!("  Sample rate: {}Hz", kick.sample_rate);
    println!("  Channels: {}", kick.channels);

    Ok(())
}
```

**Supported formats:**
- **MP3** (MPEG-1/2 Layer III)
- **OGG Vorbis** (`.ogg`)
- **FLAC** (Free Lossless Audio Codec, `.flac`)
- **WAV** (PCM, IEEE Float, `.wav`)
- **AAC / M4A** (Advanced Audio Coding, `.aac`, `.m4a`)
- **Sample rates:** Any (44.1kHz, 48kHz, 96kHz, etc.)
- **Bit depths:** 8/16/24/32-bit (int), 32/64-bit (float) - automatically converted to f32
- **Channels:** Mono or stereo

## Quick Reference: play_sample() API

**Playing samples:**
```rust
engine.play_sample("sound.wav")?;  // Auto-caches, returns SoundId
```

**Pre-loading (optional):**
```rust
engine.preload_sample("sound.wav")?;  // Warm cache during init
```

**Cache management (optional):**
```rust
engine.clear_sample_cache()?;              // Clear all
engine.remove_cached_sample("sound.wav")?; // Clear specific
```

**With control:**
```rust
let id = engine.play_sample("sound.wav")?;
engine.set_volume(id, 0.5)?;
engine.set_pan(id, -0.3)?;
```

See `examples/sample_playback_demo.rs` for a complete demonstration.

## Playing Samples in Compositions

Once loaded, you can play samples in your compositions. There are two main approaches:

### Method 1: Cached Samples (Recommended for Repeated Use)

Load samples once and reference them by name:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Load and cache samples
    comp.load_sample("kick", "assets/kick.wav")?;
    comp.load_sample("snare", "assets/snare.wav")?;
    comp.load_sample("hihat", "assets/hihat.wav")?;

    // Use them by name
    comp.track("drums")
        .sample("kick")?
        .wait(0.5)
        .sample("snare")?
        .wait(0.5)
        .sample("hihat")?
        .wait(0.25)
        .sample("hihat")?;

    Ok(())
}
```

**When to use:**
- Samples used multiple times (drums, common SFX)
- When you want centralized sample management
- For cleaner, more readable code

### Method 2: Direct Sample Playback

Play `Sample` objects directly without caching:

```rust
use tunes::prelude::*;
use tunes::synthesis::Sample;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Load samples directly
    let explosion = Sample::from_file("assets/explosion.wav")?;
    let impact = Sample::from_file("assets/impact.wav")?;

    // Play them directly
    comp.track("sfx")
        .play_sample(&explosion, 1.0)? // 1.0 = normal speed
        .at(2.0)
        .play_sample(&impact, 1.0)?;

    Ok(())
}
```

**When to use:**
- One-off samples or dynamically generated audio
- When you're manipulating samples on-the-fly
- For procedural/generative audio

## Sample Playback Control

### Playback Rate (Pitch & Speed)

The playback rate controls both pitch and speed together:

```rust
use tunes::prelude::*;
use tunes::synthesis::Sample;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let voice = Sample::from_file("assets/voice.wav")?;

    comp.track("examples")
        // Normal playback
        .play_sample(&voice, 1.0)?

        // Double speed (2x faster, higher pitch)
        .at(2.0).play_sample(&voice, 2.0)?

        // Half speed (0.5x slower, lower pitch)
        .at(4.0).play_sample(&voice, 0.5)?;

    Ok(())
}
```

### Sample Rate Conversion

Samples are automatically converted to match your output sample rate during playback. Load any sample rate, and Tunes handles the conversion.

## Streaming Audio for Long Files

For long audio files (background music, ambience, voice-over narration), loading the entire file into memory can be wasteful. Tunes provides **streaming audio** that decodes files on-the-fly without loading them entirely into RAM.

### When to Use Streaming vs. Loading

**Use `Sample::from_file()` for:**
- Sound effects (< 5 seconds typically)
- Samples you'll manipulate (pitch shift, time stretch, slice)
- Samples used repeatedly
- Samples you need to process before playback

**Use streaming (`AudioEngine::stream_file()`) for:**
- Background music (3-10+ minutes)
- Ambient soundscapes
- Voice-over narration
- Any long audio where you don't need real-time manipulation
- When memory usage is a concern

### Basic Streaming

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    // Stream a long background music file without loading it all into RAM
    let music_id = engine.stream_file("assets/background_music.mp3")?;

    // Music plays in the background...
    std::thread::sleep(std::time::Duration::from_secs(60));

    // Stop when done
    engine.stop_stream(music_id)?;

    Ok(())
}
```

**How it works:**
- File is decoded in a background thread
- Decoded audio is buffered in a lock-free ring buffer (~5 seconds of audio)
- Audio callback reads from the buffer with zero allocations
- Minimal memory footprint - only buffered audio in RAM, not entire file

### Looping Background Music

Perfect for game music that needs to repeat continuously:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    // Loop background music indefinitely
    let music_id = engine.stream_file_looping("assets/music_loop.mp3")?;

    // Music loops until you stop it
    std::thread::sleep(std::time::Duration::from_secs(120));

    engine.stop_stream(music_id)?;

    Ok(())
}
```

The file will seamlessly restart from the beginning when it reaches the end.

### Controlling Streaming Playback

Streams support real-time control without allocations:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let music_id = engine.stream_file("assets/music.mp3")?;

    // Volume control
    engine.set_stream_volume(music_id, 0.5)?;  // 50% volume

    // Pan control
    engine.set_stream_pan(music_id, -0.5)?;    // Pan left

    // Pause and resume
    engine.pause_stream(music_id)?;
    std::thread::sleep(std::time::Duration::from_secs(2));
    engine.resume_stream(music_id)?;

    // Stop (cleans up decoder thread)
    engine.stop_stream(music_id)?;

    Ok(())
}
```

### Multiple Concurrent Streams

You can stream multiple files simultaneously:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    // Play multiple streams with independent control
    let music = engine.stream_file_looping("assets/music.mp3")?;
    let ambience = engine.stream_file_looping("assets/ambience.ogg")?;
    let narration = engine.stream_file("assets/voiceover.flac")?;

    // Control each independently
    engine.set_stream_volume(music, 0.6)?;
    engine.set_stream_volume(ambience, 0.3)?;
    engine.set_stream_volume(narration, 0.9)?;

    // Later: stop individual streams
    engine.stop_stream(music)?;
    engine.stop_stream(ambience)?;
    engine.stop_stream(narration)?;

    Ok(())
}
```

### Practical Game Audio Example: Dynamic Music System

```rust
use tunes::prelude::*;

struct MusicSystem {
    engine: AudioEngine,
    current_music: Option<SoundId>,
}

impl MusicSystem {
    fn new() -> anyhow::Result<Self> {
        Ok(Self {
            engine: AudioEngine::new()?,
            current_music: None,
        })
    }

    fn play_menu_music(&mut self) -> anyhow::Result<()> {
        // Stop any current music
        if let Some(id) = self.current_music {
            self.engine.stop_stream(id)?;
        }

        // Start new music
        let id = self.engine.stream_file_looping("assets/music/menu.mp3")?;
        self.engine.set_stream_volume(id, 0.7)?;
        self.current_music = Some(id);

        Ok(())
    }

    fn play_battle_music(&mut self) -> anyhow::Result<()> {
        if let Some(id) = self.current_music {
            self.engine.stop_stream(id)?;
        }

        let id = self.engine.stream_file_looping("assets/music/battle.mp3")?;
        self.engine.set_stream_volume(id, 0.8)?;
        self.current_music = Some(id);

        Ok(())
    }

    fn fade_out_music(&mut self, duration: f32) -> anyhow::Result<()> {
        if let Some(id) = self.current_music {
            // Gradually reduce volume
            for i in 0..10 {
                let volume = 1.0 - (i as f32 / 10.0);
                self.engine.set_stream_volume(id, volume)?;
                std::thread::sleep(std::time::Duration::from_secs_f32(duration / 10.0));
            }
            self.engine.stop_stream(id)?;
            self.current_music = None;
        }
        Ok(())
    }
}
```

### Streaming API Reference

```rust
// Start streaming
let id = engine.stream_file("path.mp3")?;           // Play once
let id = engine.stream_file_looping("path.mp3")?;  // Loop forever

// Control playback
engine.stop_stream(id)?;                            // Stop and cleanup
engine.pause_stream(id)?;                           // Pause decoding
engine.resume_stream(id)?;                          // Resume decoding

// Adjust parameters
engine.set_stream_volume(id, 0.5)?;                 // 0.0 to 1.0
engine.set_stream_pan(id, -0.5)?;                   // -1.0 (left) to 1.0 (right)
```

**Supported formats:** MP3, OGG Vorbis, FLAC, WAV, AAC (same as `Sample::from_file()`)

**Memory usage:** ~5 seconds of buffered audio (~900KB for stereo 44.1kHz), regardless of file length

**Performance:** Background decoding thread, lock-free ring buffer, zero allocations in audio callback

## Sample Manipulation

Tunes provides powerful tools for transforming samples. All methods return new `Sample` objects, leaving the original unchanged.

### Time Stretching (Change Duration, Keep Pitch)

Stretch or compress duration without affecting pitch:

```rust
use tunes::synthesis::Sample;

fn main() -> anyhow::Result<()> {
    let impact = Sample::from_file("assets/impact.wav")?;

    // Slow motion: 2x longer, same pitch
    let slow_mo = impact.time_stretch(2.0);

    // Fast forward: 0.5x duration, same pitch
    let fast = impact.time_stretch(0.5);

    println!("Original: {:.2}s", impact.duration);
    println!("Stretched: {:.2}s", slow_mo.duration);
    println!("Compressed: {:.2}s", fast.duration);

    Ok(())
}
```

**Common use cases:**
- Slow-motion effects for impacts
- Speeding up dialog without "chipmunk" effect
- Time dilation in games
- Matching sample duration to beat timing

**Best results:** Stretch factors between 0.5x and 2.0x

### Pitch Shifting (Change Pitch, Keep Duration)

Shift pitch without changing duration:

```rust
use tunes::synthesis::Sample;

fn main() -> anyhow::Result<()> {
    let footstep = Sample::from_file("assets/footstep.wav")?;

    // Shift up by 5 semitones (higher pitch)
    let small_enemy = footstep.pitch_shift(5.0);

    // Shift down by 7 semitones (lower pitch)
    let large_enemy = footstep.pitch_shift(-7.0);

    // All have same duration
    assert!((small_enemy.duration - footstep.duration).abs() < 0.01);
    assert!((large_enemy.duration - footstep.duration).abs() < 0.01);

    Ok(())
}
```

**Common use cases:**
- Enemy size variations (low = big, high = small)
- Creating sample variations to reduce repetition
- Musical transposition
- Voice character changes

**Pitch intervals (semitones):**
- `12` = one octave up
- `-12` = one octave down
- `7` = perfect fifth up
- `5` = perfect fourth up
- `3` = minor third up

### Creating Variations for Games

Reduce repetitive audio by creating pitch variations:

```rust
use tunes::synthesis::Sample;

fn create_footstep_variations() -> anyhow::Result<Vec<Sample>> {
    let base = Sample::from_file("assets/footstep.wav")?;

    // Create 5 variations with slight pitch changes
    let variations = vec![
        base.pitch_shift(-2.0),  // Slightly lower
        base.pitch_shift(-1.0),  // A bit lower
        base.clone(),            // Original
        base.pitch_shift(1.0),   // A bit higher
        base.pitch_shift(2.0),   // Slightly higher
    ];

    Ok(variations)
}

// In your game, randomly pick one
fn play_random_footstep(variations: &[Sample]) -> &Sample {
    let idx = rand::random::<usize>() % variations.len();
    &variations[idx]
}
```

### Slicing Samples

Split samples into smaller pieces for creative manipulation:

```rust
use tunes::synthesis::Sample;

fn main() -> anyhow::Result<()> {
    let drum_loop = Sample::from_file("assets/drumloop.wav")?;

    // Split into 8 equal slices
    let slices = drum_loop.slice_equal(8)?;

    // Play slices in different order
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("drums")
        .play_slice(&slices[0], 1.0)? // Kick
        .wait(0.5)
        .play_slice(&slices[4], 1.0)? // Snare
        .wait(0.5)
        .play_slice(&slices[2], 1.0)? // Hat
        .wait(0.25)
        .play_slice(&slices[2], 1.0)?; // Hat again

    Ok(())
}
```

For more slicing techniques, see [Advanced: Sample Slicing](../advanced/samples.md).

### Detecting Transients (Onset Detection)

Automatically find hit points in drum loops or percussive samples:

```rust
use tunes::synthesis::Sample;

fn main() -> anyhow::Result<()> {
    let drum_loop = Sample::from_file("assets/drumloop.wav")?;

    // Detect transients (hits)
    // threshold: 0.3 (sensitivity), min_gap: 50ms (avoid double-triggers)
    let slices = drum_loop.slice_by_transients(0.3, 50.0)?;

    println!("Detected {} hits in drum loop", slices.len());

    // Play each detected hit
    let mut comp = Composition::new(Tempo::new(120.0));
    let mut track = comp.track("drums");

    for (i, slice) in slices.iter().enumerate() {
        track = track.play_slice(slice, 1.0)?;
        if i < slices.len() - 1 {
            track = track.wait(0.25);
        }
    }

    Ok(())
}
```

**Parameters:**
- `threshold` (0.0-1.0): Sensitivity (lower = more hits detected)
- `min_gap_ms`: Minimum time between hits in milliseconds

### Basic Sample Processing

#### Normalization

Scale sample to maximum volume without clipping:

```rust
let sample = Sample::from_file("quiet.wav")?;
let normalized = sample.normalize();
```

#### Gain (Volume)

Adjust volume by a multiplier:

```rust
let sample = Sample::from_file("loud.wav")?;

let quieter = sample.with_gain(0.5);   // Half volume
let louder = sample.with_gain(2.0);    // Double volume (may clip)
```

#### Reversing

Play sample backwards:

```rust
let sample = Sample::from_file("speech.wav")?;
let reversed = sample.reverse();
```

#### Fades

Apply fade in/out envelopes:

```rust
let sample = Sample::from_file("pad.wav")?;

let with_fadein = sample.with_fade_in(0.5);   // 0.5 second fade in
let with_fadeout = sample.with_fade_out(1.0); // 1.0 second fade out

// Chain them
let smooth = sample
    .with_fade_in(0.3)
    .with_fade_out(0.5);
```

## Practical Game Audio Examples

### Example 1: Enemy Footsteps with Variation

```rust
use tunes::prelude::*;
use tunes::synthesis::Sample;

struct FootstepSystem {
    variations: Vec<Sample>,
}

impl FootstepSystem {
    fn new() -> anyhow::Result<Self> {
        let base = Sample::from_file("assets/footstep.wav")?;

        // Create pitch variations for less repetition
        let variations = vec![
            base.pitch_shift(-2.0),
            base.pitch_shift(-1.0),
            base.clone(),
            base.pitch_shift(1.0),
            base.pitch_shift(2.0),
        ];

        Ok(Self { variations })
    }

    fn play_footstep(&self, comp: &mut Composition, time: f32) {
        // Pick random variation
        let idx = rand::random::<usize>() % self.variations.len();
        let sample = &self.variations[idx];

        comp.track("footsteps")
            .at(time)
            .play_sample(sample, 1.0);
    }
}
```

### Example 2: Impact Sound with Size Variation

```rust
use tunes::synthesis::Sample;

enum EnemySize {
    Small,
    Medium,
    Large,
}

fn play_impact_for_enemy(
    comp: &mut Composition,
    size: EnemySize,
    time: f32
) -> anyhow::Result<()> {
    let impact = Sample::from_file("assets/impact.wav")?;

    // Adjust pitch based on size
    let adjusted = match size {
        EnemySize::Small => impact.pitch_shift(5.0),   // Higher pitch
        EnemySize::Medium => impact,                   // Normal
        EnemySize::Large => impact.pitch_shift(-7.0),  // Lower pitch
    };

    comp.track("impacts")
        .at(time)
        .play_sample(&adjusted, 1.0);

    Ok(())
}
```

### Example 3: Slow Motion Effect

```rust
use tunes::synthesis::Sample;

fn create_slow_motion_sfx() -> anyhow::Result<Sample> {
    let explosion = Sample::from_file("assets/explosion.wav")?;

    // 2x slower for dramatic slow-mo
    let slow_mo = explosion.time_stretch(2.0);

    // Optionally pitch down slightly for extra drama
    let dramatic = slow_mo.pitch_shift(-5.0);

    Ok(dramatic)
}
```

### Example 4: Drum Loop Slicing & Rearrangement

```rust
use tunes::prelude::*;
use tunes::synthesis::Sample;

fn create_glitch_drums() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(140.0));

    // Load and slice drum loop
    let loop_sample = Sample::from_file("assets/drumloop.wav")?;
    let slices = loop_sample.slice_equal(16)?;

    // Create glitchy pattern by playing slices in random order
    let pattern = vec![0, 7, 2, 8, 4, 15, 1, 12]; // Custom pattern

    let mut track = comp.track("glitch");
    for &slice_idx in &pattern {
        track = track
            .play_slice(&slices[slice_idx], 1.0)?
            .wait(0.125); // 16th notes
    }

    Ok(())
}
```

### Example 5: Dynamic Sample Loading

```rust
use tunes::prelude::*;
use tunes::synthesis::Sample;
use std::collections::HashMap;

struct SampleBank {
    samples: HashMap<String, Vec<Sample>>,
}

impl SampleBank {
    fn new() -> Self {
        Self {
            samples: HashMap::new(),
        }
    }

    fn load_variations(&mut self, name: &str, path: &str, count: usize) -> anyhow::Result<()> {
        let base = Sample::from_file(path)?;

        let mut variations = Vec::new();
        for i in 0..count {
            // Create pitch variations from -2 to +2 semitones
            let semitones = -2.0 + (4.0 * i as f32 / (count - 1) as f32);
            variations.push(base.pitch_shift(semitones));
        }

        self.samples.insert(name.to_string(), variations);
        Ok(())
    }

    fn play_random(&self, comp: &mut Composition, name: &str, time: f32) {
        if let Some(variations) = self.samples.get(name) {
            let idx = rand::random::<usize>() % variations.len();
            comp.track("sfx")
                .at(time)
                .play_sample(&variations[idx], 1.0);
        }
    }
}
```

## Sample Memory Management

Samples use `Arc<Vec<f32>>` internally, so cloning is cheap - it only increments a reference count rather than copying audio data.

```rust
let sample1 = Sample::from_file("big_file.wav")?;
let sample2 = sample1.clone(); // Cheap! No audio data copied

// Both share the same audio data in memory
```

**Performance tips:**
- Clone samples freely - it's just a pointer copy
- Manipulated samples (pitch shift, time stretch) create new data
- Cache manipulated samples if used repeatedly
- Use `play_sample()` for dynamic samples
- Use `load_sample()` + `sample()` for repeated samples

## Creating Samples from Code

You can also create samples programmatically:

```rust
use tunes::synthesis::Sample;

fn create_sine_wave(frequency: f32, duration: f32) -> Sample {
    let sample_rate = 44100;
    let num_samples = (sample_rate as f32 * duration) as usize;

    let data: Vec<f32> = (0..num_samples)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            (2.0 * std::f32::consts::PI * frequency * t).sin() * 0.5
        })
        .collect();

    Sample::from_mono(data, sample_rate)
}

// Use it
let test_tone = create_sine_wave(440.0, 1.0); // A4 for 1 second
```

This is useful for:
- Test signals
- Procedural sound generation
- Algorithmic sound design
- Audio unit tests

## Next Steps

- **[Spatial Audio](./spatial-audio.md)** - Position samples in 3D space
- **[Dynamic Music](./dynamic-music.md)** - Use samples in interactive music systems
- **[Advanced: MIDI Import](../advanced/midi.md)** - Trigger samples from MIDI
- **[Synthesis Basics](../synthesis/basics.md)** - Combine samples with synthesis

---

## Quick Reference

```rust
// Loading (supports MP3, OGG, FLAC, WAV, AAC)
Sample::from_file("kick.mp3")?
Sample::from_file("snare.ogg")?
Sample::from_file("loop.flac")?
Sample::from_mono(vec![0.0, 0.5, 1.0], 44100)  // Programmatic creation

// Quick fire-and-forget (NEW in v0.14)
engine.play_sample("boom.wav")?  // Non-blocking, concurrent, simple!

// Playing in compositions
comp.load_sample("name", "sample.mp3")?
comp.track("t").sample("name")?
comp.track("t").play_sample(&sample, 1.0)

// Streaming (for long files - background music, ambience)
let id = engine.stream_file("music.mp3")?         // Stream once
let id = engine.stream_file_looping("music.mp3")? // Loop forever
engine.set_stream_volume(id, 0.5)?                // Control volume
engine.set_stream_pan(id, -0.5)?                  // Control pan
engine.pause_stream(id)?                          // Pause
engine.resume_stream(id)?                         // Resume
engine.stop_stream(id)?                           // Stop & cleanup

// Manipulation
sample.time_stretch(1.5)        // 1.5x duration, same pitch
sample.pitch_shift(7.0)         // +7 semitones, same duration
sample.normalize()              // Scale to max volume
sample.with_gain(0.5)           // Adjust volume
sample.reverse()                // Play backwards
sample.with_fade_in(0.5)        // Fade in over 0.5s
sample.with_fade_out(1.0)       // Fade out over 1.0s

// Slicing
sample.slice_equal(8)?          // Split into 8 parts
sample.slice_by_transients(0.3, 50.0)?  // Auto-detect hits
sample.slice_at_times(&[0.5, 1.0, 1.5])?  // At specific times

// Info
sample.duration                 // Duration in seconds
sample.sample_rate              // Sample rate in Hz
sample.channels                 // 1 (mono) or 2 (stereo)
sample.num_frames()             // Number of frames
```
