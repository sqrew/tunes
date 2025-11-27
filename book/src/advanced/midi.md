# MIDI

Import and export Standard MIDI Files (SMF) for interoperability with DAWs, notation software, and other music applications.

## Overview

The `midi` module provides:

- **File I/O**: Import MIDI files into playable `Mixer` objects, export compositions to `.mid` files
- **Conversion utilities**: Convert between MIDI values (note numbers, velocities, ticks) and Tunes internal representations (frequencies, volumes, seconds)
- **Tempo mapping**: Accurate time-to-tick conversion with support for tempo changes

## Importing MIDI Files

Load any Standard MIDI File and play it immediately:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    // Import a MIDI file
    let mixer = Mixer::import_midi("song.mid")?;

    // Play it
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;

    Ok(())
}
```

### What Gets Imported

| MIDI Feature | Tunes Mapping |
|--------------|---------------|
| Note On/Off events | `NoteEvent` with frequency from MIDI note number |
| Channel 10 drums | `DrumEvent` mapped from General MIDI percussion |
| Velocity (0-127) | Normalized to 0.0-1.0 volume |
| Tempo changes | `TempoChangeEvent` in the mixer |
| Time signatures | `TimeSignatureEvent` meta events |
| Track names | `track.name` property |
| Program changes | Mapped to appropriate `Instrument` presets |
| Pitch bend | Applied as static pitch offset to notes |
| CC7 (Volume) | Track volume (0.0-1.0) |
| CC10 (Pan) | Track pan (-1.0 to 1.0) |
| CC11 (Expression) | Track volume (alternative) |

### General MIDI Program Mapping

When importing MIDI files, program change messages automatically select appropriate instrument presets:

```rust
use tunes::midi::gm_program_to_instrument;

// GM Program 0 = Acoustic Grand Piano
let piano = gm_program_to_instrument(0);

// GM Program 33 = Electric Bass (finger)
let bass = gm_program_to_instrument(33);

// GM Program 73 = Flute
let flute = gm_program_to_instrument(73);
```

The mapping covers all 128 General MIDI programs across categories:

| Program Range | Category | Example Instruments |
|---------------|----------|---------------------|
| 0-7 | Piano | Acoustic Piano, Electric Piano, Harpsichord |
| 8-15 | Chromatic Percussion | Celesta, Glockenspiel, Vibraphone, Marimba |
| 16-23 | Organ | Hammond Organ, Church Organ, Accordion |
| 24-31 | Guitar | Acoustic Guitar, Electric Guitar (clean/distorted) |
| 32-39 | Bass | Upright Bass, Fingerstyle Bass, Synth Bass |
| 40-47 | Strings | Violin, Viola, Cello, Harp, Timpani |
| 48-55 | Ensemble | String Ensemble, Choir Aahs/Oohs |
| 56-63 | Brass | Trumpet, Trombone, Tuba, French Horn |
| 64-71 | Reed | Soprano/Alto/Tenor/Baritone Sax, Oboe, Clarinet |
| 72-79 | Pipe | Piccolo, Flute, Pan Flute, Shakuhachi |
| 80-87 | Synth Lead | Square Lead, Saw Lead, Supersaw |
| 88-95 | Synth Pad | Warm Pad, Ambient Pad, Shimmer Pad |
| 96-103 | Synth Effects | Cosmic Rays, Wind Chimes, Glitch |
| 104-111 | Ethnic | Sitar, Banjo, Koto, Kalimba, Bagpipes |
| 112-119 | Percussive | Steel Drums, Taiko, Djembe |
| 120-127 | Sound Effects | Laser, Impact, Riser |

### Import Workflow Example

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    // Import MIDI and convert to audio
    let mut mixer = Mixer::import_midi("classical_piece.mid")?;

    // Optionally adjust track properties
    for track in mixer.all_tracks_mut() {
        if track.name.as_deref() == Some("Bass") {
            track.volume = 0.8;
        }
    }

    // Export to WAV
    mixer.export_wav("rendered.wav", 44100)?;

    Ok(())
}
```

## Exporting to MIDI

Export compositions to Standard MIDI Files for use in DAWs and notation software:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5], 0.5);

    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12]);

    let mixer = comp.into_mixer();
    mixer.export_midi("song.mid")?;

    Ok(())
}
```

### What Gets Exported

| Tunes Feature | MIDI Output |
|---------------|-------------|
| `NoteEvent` | Note On/Off with frequency-to-MIDI conversion |
| `DrumEvent` | Channel 10 percussion notes (GM mapping) |
| Note velocity | MIDI velocity (0-127) |
| Pitch bend | MIDI pitch bend events (±2 semitone range) |
| Tempo changes | Tempo meta events |
| Time signatures | Time signature meta events |
| Key signatures | Key signature meta events |
| Track volume | CC7 (Volume) controller |
| Track pan | CC10 (Pan) controller |
| LFO modulation | Sampled as CC automation (Pitch→CC1, Volume→CC11, Pan→CC10) |

### Export Limitations

MIDI cannot represent:

- **Sample playback** - MIDI has no concept of audio samples
- **Effects** - Reverb, delay, filters are not in the MIDI spec
- **Synthesis parameters** - MIDI doesn't specify how notes sound
- **Custom waveforms** - Wavetable data cannot be exported

### Multi-Track Export

Each Tunes track becomes a separate MIDI track:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(140.0));

    // Track 1: Lead melody
    comp.instrument("lead", &Instrument::synth_lead())
        .notes(&[E5, G5, A5, B5], 0.25);

    // Track 2: Bass line
    comp.instrument("bass", &Instrument::sub_bass())
        .notes(&[E2, E2, A2, B2], 0.5);

    // Track 3: Drums (goes to MIDI channel 10)
    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat_closed(&[0, 2, 4, 6, 8, 10, 12, 14]);

    let mixer = comp.into_mixer();
    mixer.export_midi("multi_track.mid")?;

    Ok(())
}
```

The resulting MIDI file contains:
- Track 0: Tempo track (meta information)
- Track 1: "lead" on channel 0
- Track 2: "bass" on channel 1
- Track 3: "drums" on channel 10 (percussion)

### Tempo and Time Signature Changes

Export compositions with tempo variations:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Start at 120 BPM
    comp.track("melody")
        .notes(&[C4, E4, G4], 0.5)
        .tempo_change(140.0)  // Speed up to 140 BPM
        .notes(&[A4, B4, C5], 0.25);

    let mixer = comp.into_mixer();
    mixer.export_midi("tempo_changes.mid")?;

    Ok(())
}
```

## Conversion Utilities

The `midi::convert` module provides low-level conversion functions.

### Frequency and MIDI Notes

Convert between frequencies (Hz) and MIDI note numbers:

```rust
use tunes::midi::{frequency_to_midi_note, midi_note_to_frequency};

// A4 = 440 Hz = MIDI note 69
assert_eq!(frequency_to_midi_note(440.0), 69);
assert_eq!(midi_note_to_frequency(69), 440.0);

// C4 ≈ 261.63 Hz = MIDI note 60
assert_eq!(frequency_to_midi_note(261.63), 60);
```

Uses equal temperament tuning with A4 = 440 Hz as reference.

### Time and Ticks

Convert between seconds and MIDI ticks:

```rust
use tunes::midi::{seconds_to_ticks, ticks_to_seconds, PPQ};

// At 120 BPM with 480 PPQ (standard resolution)
// 1 beat = 0.5 seconds = 480 ticks
assert_eq!(seconds_to_ticks(0.5, 120.0, PPQ), 480);
assert_eq!(ticks_to_seconds(480, 120.0, PPQ), 0.5);

// 1 second = 2 beats = 960 ticks
assert_eq!(seconds_to_ticks(1.0, 120.0, PPQ), 960);
```

### TempoMap for Variable Tempo

When tempo changes occur mid-song, use `TempoMap` for accurate conversion:

```rust
use tunes::midi::{TempoMap, PPQ};

let mut tempo_map = TempoMap::new(120.0, PPQ);

// Add tempo change at 2 seconds (switch to 60 BPM)
tempo_map.add_change(2.0, 60.0);
tempo_map.finalize();

// First 2 seconds at 120 BPM = 1920 ticks
assert_eq!(tempo_map.seconds_to_ticks(2.0), 1920);

// Next 1 second at 60 BPM = 480 ticks
// Total at 3 seconds = 1920 + 480 = 2400 ticks
assert_eq!(tempo_map.seconds_to_ticks(3.0), 2400);
```

The `TempoMap` integrates through tempo segments to calculate accurate tick positions.

### Drum Mapping

Convert between Tunes `DrumType` and General MIDI percussion notes:

```rust
use tunes::midi::{drum_type_to_midi_note, midi_note_to_drum_type};
use tunes::instruments::drums::DrumType;

// Kick drum = MIDI note 36
assert_eq!(drum_type_to_midi_note(DrumType::Kick), 36);

// MIDI note 38 = Snare
assert_eq!(midi_note_to_drum_type(38), Some(DrumType::Snare));

// MIDI note 42 = Closed Hi-Hat
assert_eq!(midi_note_to_drum_type(42), Some(DrumType::HiHatClosed));
```

Full mapping covers 80+ drum types across these General MIDI percussion categories:

| MIDI Note Range | Category |
|-----------------|----------|
| 35-36 | Kick drums |
| 37-40 | Snare, rimshot, clap |
| 41-48 | Toms |
| 42, 44, 46 | Hi-hats |
| 49-55, 57 | Cymbals (crash, ride, splash, china) |
| 56 | Cowbell |
| 60-64 | Bongos and congas |
| 65-68 | Timbales and agogos |
| 70, 73-74 | Shakers and guiros |
| 75-77 | Claves and wood blocks |
| 81 | Triangle |

### Velocity and Volume

Convert between MIDI velocity (0-127) and Tunes volume (0.0-1.0):

```rust
use tunes::midi::{volume_to_velocity, velocity_to_volume};

// Full volume = velocity 127
assert_eq!(volume_to_velocity(1.0), 127);

// Half volume = velocity 64
assert_eq!(volume_to_velocity(0.5), 64);

// Velocity 64 = ~0.5 volume
assert!((velocity_to_volume(64) - 0.5).abs() < 0.01);
```

### Pitch Bend

Convert between semitones and MIDI pitch bend values:

```rust
use tunes::midi::semitones_to_pitch_bend;

// Center (no bend) = 8192
assert_eq!(semitones_to_pitch_bend(0.0, 2.0), 8192);

// +2 semitones (full range up) = 16383
assert_eq!(semitones_to_pitch_bend(2.0, 2.0), 16383);

// -2 semitones (full range down) = 0
assert_eq!(semitones_to_pitch_bend(-2.0, 2.0), 0);

// +1 semitone (half range) = 12288
assert_eq!(semitones_to_pitch_bend(1.0, 2.0), 12288);
```

The second parameter is the pitch bend range (standard is ±2 semitones, but some synths use ±12).

## Complete Roundtrip Example

Import, modify, and re-export:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    // Import existing MIDI
    let mut mixer = Mixer::import_midi("original.mid")?;

    // Play to verify
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;

    // Export to different formats
    mixer.export_midi("copy.mid")?;        // Re-export as MIDI
    mixer.export_wav("rendered.wav", 44100)?;  // Render to audio

    Ok(())
}
```

## Quick Reference

```rust
// Import
let mixer = Mixer::import_midi("file.mid")?;

// Export
mixer.export_midi("file.mid")?;

// Conversion utilities
use tunes::midi::{
    frequency_to_midi_note,     // Hz → MIDI note (0-127)
    midi_note_to_frequency,     // MIDI note → Hz
    seconds_to_ticks,           // seconds → MIDI ticks
    ticks_to_seconds,           // MIDI ticks → seconds
    drum_type_to_midi_note,     // DrumType → GM percussion note
    midi_note_to_drum_type,     // GM percussion note → DrumType
    volume_to_velocity,         // 0.0-1.0 → 0-127
    velocity_to_volume,         // 0-127 → 0.0-1.0
    semitones_to_pitch_bend,    // semitones → 14-bit pitch bend
    gm_program_to_instrument,   // GM program → Instrument preset
    TempoMap,                   // Variable tempo time→tick conversion
    PPQ,                        // 480 ticks per quarter note
    DEFAULT_VELOCITY,           // 80 (standard velocity)
};
```

## Technical Details

### File Format

Tunes uses the `midly` crate for MIDI parsing and writing. Exported files are:
- Format: Type 1 (multiple tracks, parallel)
- Resolution: 480 PPQ (pulses per quarter note)
- Timing: Metrical (not SMPTE timecode)

### Channel Allocation

- Melodic tracks: Channels 0-8, 10-15 (15 available)
- Drum tracks: Channel 9 (always, per GM spec)
- If more than 15 melodic tracks exist, channels wrap around

### Pitch Bend Behavior

On export:
- Pitch bend events are inserted before notes with non-zero pitch offset
- Pitch bend resets to center after the note ends

On import:
- Pitch bend state is tracked per channel
- Notes capture the current pitch bend value at Note On time
- Continuous pitch bend gestures become static offsets (one value per note)
