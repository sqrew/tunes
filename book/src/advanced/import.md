# Importing MIDI

Import MIDI files to use as composition starting points or convert them to audio.

## Basic MIDI Import

Load a MIDI file into a `Mixer`:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    // Import MIDI file
    let mixer = Mixer::import_midi("song.mid")?;

    println!("Imported MIDI:");
    println!("  Tempo: {:.1} BPM", mixer.tempo.bpm);
    println!("  Tracks: {}", mixer.all_tracks().len());
    println!("  Duration: {:.2}s", mixer.total_duration());

    Ok(())
}
```

**What gets imported:**
- Note events (pitch, duration, velocity)
- Drum events (channel 10)
- Tempo changes
- Time signature changes
- Track names
- Program changes (as metadata)

**What's ignored:**
- Control Change (CC) messages
- Pitch bend (converted to static offsets)
- Polyphonic aftertouch
- SMPTE timecode

## Common Workflows

### Convert MIDI to WAV

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    // Import MIDI
    let mut mixer = Mixer::import_midi("input.mid")?;

    // Export to WAV
    mixer.export_wav("output.wav", 44100)?;

    println!("Converted MIDI to WAV!");
    Ok(())
}
```

### Play Imported MIDI

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    // Import and play
    let mixer = Mixer::import_midi("song.mid")?;
    engine.play_mixer(&mixer)?;

    Ok(())
}
```

### Round-Trip Testing

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    // Create composition
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4], 0.5);

    // Export to MIDI
    let mixer = comp.into_mixer();
    mixer.export_midi("test.mid")?;

    // Re-import
    let reimported = Mixer::import_midi("test.mid")?;

    // Export again to verify
    reimported.export_midi("test_roundtrip.mid")?;

    Ok(())
}
```

## Inspecting MIDI Data

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mixer = Mixer::import_midi("song.mid")?;

    // Examine each track
    for (i, track) in mixer.all_tracks().iter().enumerate() {
        let name = track.name.as_deref().unwrap_or("Untitled");
        let num_events = track.events.len();

        println!("Track {}: '{}' ({} events)", i + 1, name, num_events);

        // Check if track has drums
        let has_drums = track.events.iter().any(|e| {
            matches!(e, tunes::track::AudioEvent::Drum(_))
        });

        if has_drums {
            println!("  Contains drum events");
        }
    }

    Ok(())
}
```

## Limitations & Notes

### Instrument Mapping

Imported MIDI tracks use default instruments:
- **Melodic tracks** → Simple sine wave synthesis
- **Drum tracks (channel 10)** → Drum synthesis

To use custom instruments, you'll need to:
1. Import MIDI to understand structure
2. Recreate composition with desired instruments
3. Reference MIDI timing/notes

### Timing

MIDI timing is converted to Tunes' internal time format:
- **PPQ (Pulses Per Quarter)** → Seconds
- Tempo changes are applied
- Time signatures are preserved

### Velocity

MIDI velocity (0-127) is converted to amplitude (0.0-1.0):
```
amplitude = velocity / 127.0
```

## Use Cases

**MIDI Import is great for:**
- Converting MIDI to audio
- Using MIDI as composition reference
- Batch processing MIDI files
- Testing round-trip export/import
- Analyzing MIDI structure

**Not ideal for:**
- Complex MIDI editing (use a DAW)
- Real-time MIDI input (not supported)
- MIDI controller integration

## Example: Batch Convert MIDI to WAV

```rust
use tunes::prelude::*;
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    // Find all MIDI files in directory
    for entry in fs::read_dir("midi_files")? {
        let path = entry?.path();

        if path.extension().and_then(|s| s.to_str()) == Some("mid") {
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let output = format!("wav_output/{}.wav", stem);

            println!("Converting: {} → {}", path.display(), output);

            // Convert MIDI to WAV
            let mut mixer = Mixer::import_midi(&path)?;
            mixer.export_wav(&output, 44100)?;

            println!("  ✓ Done");
        }
    }

    Ok(())
}
```

## Quick Reference

```rust
// Import MIDI
let mixer = Mixer::import_midi("song.mid")?;

// Common operations after import
mixer.export_wav("output.wav", 44100)?;
mixer.export_flac("output.flac", 44100)?;
engine.play_mixer(&mixer)?;

// Inspect imported data
mixer.tempo.bpm                 // Get tempo
mixer.all_tracks().len()        // Track count
mixer.total_duration()          // Duration in seconds

// Round-trip
mixer.export_midi("output.mid")?;
```
