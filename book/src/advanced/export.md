# Exporting Audio

Export your compositions to standard audio and MIDI formats for use in DAWs, games, videos, or distribution.

## WAV Export

Export to uncompressed WAV format:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5], 0.5);

    // Export to WAV
    let mixer = comp.into_mixer();
    mixer.export_wav("output.wav", 44100)?;

    Ok(())
}
```

**Sample rates:**
- `44100` - CD quality (standard)
- `48000` - Professional audio/video
- `96000` - High-resolution audio

**Characteristics:**
- Uncompressed PCM audio
- No decode overhead
- Larger file sizes than compressed formats
- Universal format support

## FLAC Export

Export to FLAC for lossless compression (~50-60% smaller than WAV):

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("drums", &Instrument::drums())
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12]);

    // Export to FLAC
    let mixer = comp.into_mixer();
    mixer.export_flac("output.flac", 44100)?;

    Ok(())
}
```

**Characteristics:**
- Lossless compression (typically 50-60% of WAV size)
- Bit-perfect audio reproduction
- Decode overhead required for playback
- Supported by most DAWs and audio software

**Use cases:**
- Storage-constrained environments
- Network transfer of lossless audio
- Archival storage

## MIDI Export

Export note data to MIDI for use in DAWs and notation software:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Create tracks with note events
    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5], 0.5);

    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12]);

    // Export to MIDI
    let mixer = comp.into_mixer();
    mixer.export_midi("output.mid")?;

    Ok(())
}
```

**What's exported:**
- Note pitches and durations
- Drum hits (General MIDI)
- Tempo information
- Track separation
- Time signatures

**What's NOT exported:**
- Sample playback
- Effects (reverb, delay, filters)
- Synthesis parameters
- Custom waveforms

**Use cases:**
- DAW import for re-instrumentation
- Notation software input
- Collaborative editing workflows

## Exporting Sections

Export specific sections of your composition:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Create multiple sections
    comp.section("intro")
        .instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4], 0.5);

    comp.section("drop")
        .instrument("bass", &Instrument::sub_bass())
        .notes(&[C2, C2, G2], 1.0);

    // Export just the "intro" section
    comp.export_section_wav("intro", "intro.wav", 44100)?;

    // Export just the "drop" section
    comp.export_section_midi("drop", "drop.mid")?;

    Ok(())
}
```

## Using the AudioEngine

You can also export using the AudioEngine for more control:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4], 0.5);

    let mixer = comp.into_mixer();

    // Export via AudioEngine
    engine.export_wav(&mixer, "output.wav", 44100)?;
    engine.export_flac(&mixer, "output.flac", 48000)?;

    Ok(())
}
```

## GPU-Accelerated Export

Enable GPU acceleration for faster export processing on large compositions:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    // Initialize AudioEngine with GPU acceleration
    let engine = AudioEngine::new_with_gpu()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4], 0.5);

    let mixer = comp.into_mixer();

    // Export operations will use GPU when available
    engine.export_wav(&mixer, "output.wav", 44100)?;
    engine.export_flac(&mixer, "output.flac", 48000)?;

    Ok(())
}
```

**Performance characteristics:**

GPU acceleration offloads audio rendering computations to the graphics processor. This provides performance improvements through:

- Parallel processing of multiple audio tracks simultaneously
- SIMD operations for sample-level computations (mixing, effects)
- Reduced CPU utilization during export
- Faster processing of effects chains (reverb, delay, filters)

**When GPU acceleration provides benefit:**

- Compositions with 8+ concurrent tracks
- Heavy use of real-time effects (reverb, convolution)
- High sample rates (96kHz, 192kHz)
- Batch export operations (stems, multiple sections)
- Long-duration compositions (>3 minutes)

**Limitations:**

- Requires compatible GPU (OpenCL or CUDA support)
- Falls back to CPU if GPU is unavailable or initialization fails
- Initial overhead (~50-100ms) for GPU context setup
- May not improve performance on simple compositions (<4 tracks, minimal effects)

## Stems Export

Export individual tracks as separate files for mixing:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4], 0.5);

    comp.instrument("bass", &Instrument::sub_bass())
        .notes(&[C2, C2], 1.0);

    let mut mixer = comp.into_mixer();

    // Export stems (one file per track)
    mixer.export_stems("stems", 44100)?;
    // Creates: stems/melody.wav, stems/bass.wav

    // Or include a master mix too
    mixer.export_stems_with_master("stems", 44100)?;
    // Creates: stems/melody.wav, stems/bass.wav, stems/master.wav

    Ok(())
}
```

## Quick Reference

```rust
// WAV export
mixer.export_wav("file.wav", 44100)?;
comp.export_section_wav("intro", "intro.wav", 44100)?;

// FLAC export
mixer.export_flac("file.flac", 44100)?;

// MIDI export
mixer.export_midi("file.mid")?;
comp.export_section_midi("verse", "verse.mid")?;

// Stems
mixer.export_stems("output_folder", 44100)?;
mixer.export_stems_with_master("output_folder", 44100)?;

// Via AudioEngine
engine.export_wav(&mixer, "file.wav", 44100)?;
engine.export_flac(&mixer, "file.flac", 48000)?;
```

**Common workflow patterns:**
1. Iterative development with WAV exports for verification
2. MIDI export for DAW-based editing
3. Stems export for multi-track mixing
4. FLAC export for archival storage
