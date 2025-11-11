# Working with Samples

Samples are pre-recorded audio files (like `.wav` files) that you can load, manipulate, and play back in your compositions. They're essential for game audio - sound effects, voice lines, drum loops, and realistic instruments all come from samples.

## Loading Your First Sample

The most common way to use samples is loading them from WAV files:

```rust
use tunes::prelude::*;
use tunes::synthesis::Sample;

fn main() -> anyhow::Result<()> {
    // Load a WAV file
    let kick = Sample::from_wav("assets/audio/kick.wav")?;

    println!("Loaded sample:");
    println!("  Duration: {:.2}s", kick.duration);
    println!("  Sample rate: {}Hz", kick.sample_rate);
    println!("  Channels: {}", kick.channels);

    Ok(())
}
```

**Supported formats:**
- WAV files (`.wav`)
- Sample rates: Any (44.1kHz, 48kHz, etc.)
- Bit depths: 16-bit, 24-bit, 32-bit (int or float)
- Channels: Mono or stereo

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
    let explosion = Sample::from_wav("assets/explosion.wav")?;
    let impact = Sample::from_wav("assets/impact.wav")?;

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
    let voice = Sample::from_wav("assets/voice.wav")?;

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

## Sample Manipulation

Tunes provides powerful tools for transforming samples. All methods return new `Sample` objects, leaving the original unchanged.

### Time Stretching (Change Duration, Keep Pitch)

Stretch or compress duration without affecting pitch:

```rust
use tunes::synthesis::Sample;

fn main() -> anyhow::Result<()> {
    let impact = Sample::from_wav("assets/impact.wav")?;

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
    let footstep = Sample::from_wav("assets/footstep.wav")?;

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
    let base = Sample::from_wav("assets/footstep.wav")?;

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
    let drum_loop = Sample::from_wav("assets/drumloop.wav")?;

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
    let drum_loop = Sample::from_wav("assets/drumloop.wav")?;

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
let sample = Sample::from_wav("quiet.wav")?;
let normalized = sample.normalize();
```

#### Gain (Volume)

Adjust volume by a multiplier:

```rust
let sample = Sample::from_wav("loud.wav")?;

let quieter = sample.with_gain(0.5);   // Half volume
let louder = sample.with_gain(2.0);    // Double volume (may clip)
```

#### Reversing

Play sample backwards:

```rust
let sample = Sample::from_wav("speech.wav")?;
let reversed = sample.reverse();
```

#### Fades

Apply fade in/out envelopes:

```rust
let sample = Sample::from_wav("pad.wav")?;

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
        let base = Sample::from_wav("assets/footstep.wav")?;

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
    let impact = Sample::from_wav("assets/impact.wav")?;

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
    let explosion = Sample::from_wav("assets/explosion.wav")?;

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
    let loop_sample = Sample::from_wav("assets/drumloop.wav")?;
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
        let base = Sample::from_wav(path)?;

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
let sample1 = Sample::from_wav("big_file.wav")?;
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
// Loading
Sample::from_wav("path.wav")?
Sample::from_mono(vec![0.0, 0.5, 1.0], 44100)

// Playing
comp.load_sample("name", "path.wav")?
comp.track("t").sample("name")?
comp.track("t").play_sample(&sample, 1.0)

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
