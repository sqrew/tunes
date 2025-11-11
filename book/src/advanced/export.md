# Exporting Audio

Export your compositions to standard audio and MIDI formats for use in DAWs, games, videos, or distribution.

## WAV Export

Export to uncompressed WAV format - maximum compatibility and quality:

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

**When to use WAV:**
- Maximum compatibility
- Real-time applications (low decode overhead)
- When file size isn't a concern
- Final delivery format for games

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

**Benefits:**
- 50-60% smaller than WAV
- Bit-perfect quality (lossless)
- Widely supported by DAWs
- Great for archival

**When to use FLAC:**
- Archiving compositions
- Sharing online (faster uploads)
- Professional workflows
- When storage matters

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

**When to use MIDI:**
- Edit in DAW with your own instruments
- Share compositions as editable scores
- Use in notation software
- Collaboration workflows

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

**Recommended workflow:**
1. Create composition during development
2. Export to WAV for testing
3. Export to MIDI for editing in DAW
4. Export stems for professional mixing
5. Export to FLAC for archival
