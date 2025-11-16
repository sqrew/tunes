# Live Audio Recording

> **Status:** Basic recording functionality available. Advanced features (live monitoring, real-time processing) planned for future releases.

Tunes provides simple live audio recording from microphones and line inputs. Recordings are saved as WAV files and can be immediately processed through the full effects pipeline.

## Basic Recording

```rust
use tunes::prelude::*;
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // Create recorder (uses default input device)
    let mut recorder = LiveInput::new()?;

    // Start recording to WAV file
    recorder.start_recording("my_recording.wav")?;

    println!("Recording...");
    thread::sleep(Duration::from_secs(5));

    // Stop and finalize WAV file
    recorder.stop()?;

    println!("Recording saved to my_recording.wav");
    Ok(())
}
```

## Using Recordings in Compositions

Once recorded, WAV files can be used like any other sample:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Load the recording
    comp.load_sample("vocal", "my_recording.wav")?;

    // Process with effects
    comp.track("vocals")
        .sample("vocal")
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .delay(Delay::new(0.3, 0.5, 0.4))
        .eq(ParametricEQ::new().add_band(EQBand::high_shelf(8000.0, 0.7, 2.0)));

    // Mix with instruments
    comp.instrument("backing", &Instrument::acoustic_piano())
        .notes(&[C4, E4, G4], 0.5);

    comp.into_mixer().export_wav("final_mix.wav", 44100)?;
    Ok(())
}
```

## How It Works

The `LiveInput` type:
1. Opens the default audio input device (microphone/line-in)
2. Streams audio directly to a WAV file
3. Handles multiple sample formats (F32, I16, U16) automatically
4. Auto-detects sample rate and channels

**Simple architecture:**
- `cpal` provides cross-platform audio input
- `hound` writes WAV files
- Direct stream (no intermediate buffering needed)

## Examples

Run the included examples to see live recording in action:

```bash
# Basic recording and processing
cargo run --example live_recording_demo --release

# Recording with backing track
cargo run --example recording_with_backing_track --release
```

## Planned Features

Future releases will add:
- **Live monitoring** - Hear yourself while recording
- **Real-time effects** - Apply effects during recording
- **Multi-track recording** - Capture multiple inputs simultaneously
- **Streaming engine** - Full AudioEngine integration for live input + playback

For now, the recording workflow is: **Record → Save → Load → Process**. This keeps the implementation simple and leverages all existing sample playback and effects infrastructure.
