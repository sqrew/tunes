# Mixer Layer

The `Mixer` is the audio rendering layer. It takes your musical composition and renders it into actual audio samples. Think of it as the bridge between musical ideas (Composition) and sound output (AudioEngine).

## What is Mixer?

`Mixer` handles:

- Rendering all tracks to audio samples
- Applying effects and mixing multiple tracks together
- Exporting to various formats (WAV, FLAC, MIDI, stems)
- Sample-accurate timing and playback

**When to use it directly:** When you need to export audio or need fine control over rendering without an AudioEngine.

---

## Creating a Mixer

### From Composition (Most Common)

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));
comp.track("piano").notes(&[C4, E4, G4, C5], 0.5);

let mixer = comp.into_mixer();  // Converts composition to audio
```

This is the standard workflow: build your music in `Composition`, then convert to `Mixer` for playback or export.

### From Section

Export just one section of your composition:

```rust
let verse_mixer = comp.section_to_mixer("verse")?;
verse_mixer.export_wav("verse.wav", 44100)?;
```

### Direct Construction

For advanced use cases, create an empty mixer:

```rust
let mixer = Mixer::new(Tempo::new(120.0));
// Manually add tracks if needed
```

---

## Query Methods

### Check Duration

```rust
let duration = mixer.total_duration();  // Duration in seconds
println!("Song is {} seconds long", duration);
```

### Check if Empty

```rust
if mixer.is_empty() {
    println!("Warning: No audio to play!");
}
```

**Use case:** Avoid playing silent compositions.

---

## Export Formats

Mixer supports multiple export formats, each with different use cases.

### WAV Export

Standard uncompressed audio format:

```rust
mixer.export_wav("output.wav", 44100)?;
```

**Parameters:**
- Path to output file
- Sample rate (44100 = CD quality, 48000 = professional)

**When to use:** Universal compatibility, lossless quality, maximum compatibility with DAWs.

### FLAC Export

Lossless compression format:

```rust
mixer.export_flac("output.flac", 44100)?;
```

**Benefits:**
- 50-60% smaller than WAV
- Lossless quality (24-bit)
- Better for archival and distribution

**When to use:** Save disk space while maintaining perfect quality.

### MIDI Export

Export as Standard MIDI File:

```rust
mixer.export_midi("song.mid")?;
```

**What's exported:**
- Note pitches, velocities, and durations
- Drum patterns (General MIDI channel 10)
- Tempo and time signature changes
- Multiple tracks

**What's NOT exported:**
- Audio samples
- Effects (reverb, delay, filters)
- Synthesis parameters

**When to use:** Share music with DAWs, notation software, or other MIDI-compatible tools.

### Stems Export

Export individual tracks as separate files:

```rust
mixer.export_stems("output/stems/", 44100)?;
// Creates: output/stems/drums.wav, output/stems/bass.wav, etc.
```

Or export stems plus a master mix:

```rust
mixer.export_stems_with_master("output/", 44100)?;
// Creates tracks + output/_master.wav
```

**When to use:** Professional mixing in DAWs, remixing, or collaborative production.

---

## MIDI Import

Import existing MIDI files:

```rust
let mixer = Mixer::import_midi("song.mid")?;

// Play it
let engine = AudioEngine::new()?;
engine.play_mixer(&mixer)?;

// Or export to audio
mixer.export_wav("converted.wav", 44100)?;
```

**Supported:**
- Note events with pitch and velocity
- Drum patterns
- Tempo changes
- Multiple tracks

**Limitations:**
- Uses default synthesis (sine waves)
- No effects imported
- Static pitch bends (not continuous)

---

## Manipulation

### Repeat

Repeat the entire composition:

```rust
let mixer = comp.into_mixer().repeat(3);  // Plays 4 times total
engine.play_mixer(&mixer)?;
```

**Use case:** Creating loops or extended versions without rebuilding the composition.

---

## Engine vs Mixer Exports

You have two options for exporting audio:

### Option 1: AudioEngine Export

```rust
let engine = AudioEngine::new()?;
engine.export_wav(&mut mixer, "output.wav")?;
```

**Benefits:**
- Automatically uses engine's sample rate
- Guaranteed to match playback exactly
- Simple and convenient

**Use when:** You have an AudioEngine and want exports to match playback.

### Option 2: Mixer Export

```rust
mixer.export_wav("output.wav", 48000)?;  // Explicit sample rate
```

**Benefits:**
- No AudioEngine required
- Full control over sample rate
- Works in headless environments (CI, servers)

**Use when:** No audio device available, or you need specific sample rates.

---

## Complete Example

Here's a complete workflow from composition to multiple exports:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    // 1. Create composition
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12]);

    comp.instrument("bass", &Instrument::sub_bass())
        .notes(&[C2, C2, G2, F2], 0.5);

    comp.instrument("lead", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, E4], 0.25);

    // 2. Convert to mixer
    let mut mixer = comp.into_mixer();

    // 3. Check before exporting
    if mixer.is_empty() {
        println!("No audio to export!");
        return Ok(());
    }

    println!("Duration: {:.2} seconds", mixer.total_duration());

    // 4. Export to multiple formats
    mixer.export_wav("output.wav", 44100)?;
    mixer.export_flac("output.flac", 48000)?;
    mixer.export_midi("output.mid")?;
    mixer.export_stems("stems/", 44100)?;

    // 5. Create a looped version
    let looped = mixer.clone().repeat(3);
    looped.export_wav("output_looped.wav", 44100)?;

    println!("Exports complete!");

    Ok(())
}
```

---

## How It Works

The `Mixer` renders audio through these steps:

1. **Track Collection** - Holds multiple tracks, each with their own events and effects
2. **Sample Generation** - For each audio frame, synthesizes sound for all active notes/drums
3. **Effects Processing** - Applies filters, reverb, delay, etc. to each track
4. **Stereo Mixing** - Combines all tracks with panning into a stereo output
5. **Soft Clipping** - Prevents harsh distortion using smooth saturation

**Performance optimizations:**
- Binary search for event lookups (fast with many events)
- Time-bounds caching (skips inactive tracks)
- Pre-computed effect ordering

The `Mixer` is sample-rate agnostic - you specify the sample rate at render/export time.

---

**Next Steps:**
- [Composition Layer](./composition.md) - Build complex musical compositions
- [AudioEngine Layer](./engine.md) - Play and control audio in real-time
- [Game Audio Patterns](../game-audio/concurrent-sfx.md) - Apply these concepts to games
