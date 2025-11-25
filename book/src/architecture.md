# Architecture

Understanding how Tunes is structured will help you make better decisions about when to use each component. The library has three core layers:

```
Composition  →  Mixer  →  AudioEngine
  (Musical)    (Audio)    (Playback)
```

Within the Mixer, audio flows through a professional bus architecture:

```
Tracks (individual instruments/parts)
   ↓ (with track-level effects)
Buses (groups of tracks)
   ↓ (with bus-level effects)
Master (final mix)
   ↓ (with master-level effects)
Output
```

---

## Mental Model

Think of Tunes like a professional recording studio:

- **Composition** = The sheet music and musical ideas
- **Track** = Individual instrument recordings
- **Bus** = Channel strips grouping related instruments (drum bus, vocal bus, etc.)
- **Master** = The master fader with final processing (EQ, compression, limiting)
- **Mixer** = The entire mixing console with its bus architecture
- **AudioEngine** = The speakers and monitoring system

You write music in `Composition`, it flows through the `Mixer`'s bus architecture (tracks → buses → master), and you hear it via `AudioEngine`.

---

## Data Flow: From Music to Sound

```rust
// 1. Create composition (musical layer)
let mut comp = Composition::new(Tempo::new(140.0));
comp.instrument("lead", &Instrument::synth_lead())
    .notes(&[C4, E4, G4], 0.25);

// 2. Convert to mixer (audio layer)
let mixer = comp.into_mixer();

// 3. Play or export (playback layer)
let engine = AudioEngine::new()?;
engine.play_mixer(&mixer)?;  // Blocking playback
// OR
let id = engine.play_mixer_realtime(&mixer)?;  // Non-blocking
```

Each layer has a clear responsibility:
- **Composition** → "What notes to play and when"
- **Mixer** → "How those notes sound as audio samples"
- **AudioEngine** → "Getting that audio to your speakers (or file)"

---

# Composition Layer

The `Composition` is where you think musically. It understands:

- **Tempo** – Beats per minute, note durations (quarter notes, eighths, etc.)
- **Musical Theory** – Scales, chords, progressions, harmonization
- **Structure** – Sections (verse, chorus), arrangements, repeats
- **Instruments** – Synthesis presets with configured sounds
- **Expression** – Volume, panning, pitch bends, vibrato
- **Effects** – Reverb, delay, filters, distortion, and more

## Basic Usage

```rust
use tunes::prelude::*;

let mut comp = Composition::new(Tempo::new(120.0));

// Add a melody
comp.track("melody")
    .notes(&[C4, E4, G4, C5], 0.5);

// Convert to audio
let mixer = comp.into_mixer();
```

## Tempo and Time

```rust
let comp = Composition::new(Tempo::new(120.0));  // 120 BPM

// Tempo-aware durations
let quarter = comp.tempo().quarter_note();
let eighth = comp.tempo().eighth_note();
let sixteenth = comp.tempo().sixteenth_note();
```

## Tracks vs Instruments

### `.track()` – Raw Audio

Use for samples, drums, and direct synthesis:

```rust
comp.track("drums")
    .drum_grid(16, 0.125)
    .kick(&[0, 4, 8, 12])
    .snare(&[4, 12]);
```

### `.instrument()` – Synthesis Presets

Use for melodic content with pre-configured sounds:

```rust
comp.instrument("piano", &Instrument::electric_piano())
    .notes(&[C4, E4, G4, C5], 0.5);
```

## Notes and Chords

```rust
// Single note
comp.track("lead").note(&[440.0], 0.5);

// Chord
comp.track("piano").note(&[C4, E4, G4], 1.0);

// Note sequences
comp.track("melody").notes(&[C4, D4, E4, F4, G4], 0.25);
```

**Note constants:** `C0` through `B8` with sharps (`CS4`, `DS4`) and flats (`DB4`, `EB4`, `AB4`, `BB4`).

## Drums

```rust
// Drum grid pattern
comp.track("drums")
    .drum_grid(16, 0.125)
    .kick(&[0, 4, 8, 12])
    .snare(&[4, 12])
    .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

// Rhythm strings
comp.track("drums")
    .rhythm("x-x- x-x-", DrumType::Kick, 0.125);  // 'x' = hit, '-' = rest
```

**Drum types:** `Kick`, `Snare`, `HiHat`, `ClosedHiHat`, `OpenHiHat`, `Tom`, `Clap`, `Rimshot`, `Cowbell`, `Crash`, `Ride`

## Musical Patterns

### Scales

```rust
use tunes::theory::core::{scale, ScalePattern};

let c_major = scale(C4, &ScalePattern::MAJOR);
comp.track("melody")
    .scale(&c_major, 0.25)           // Ascending
    .scale_reverse(&c_major, 0.25)   // Descending
    .scale_updown(&c_major, 0.25);   // Up then down
```

**Scales:** `MAJOR`, `MINOR`, `HARMONIC_MINOR`, `MELODIC_MINOR`, `MAJOR_PENTATONIC`, `MINOR_PENTATONIC`, `BLUES`, `DORIAN`, `PHRYGIAN`, `LYDIAN`, `MIXOLYDIAN`, and 40+ more.

### Arpeggios and Chords

```rust
use tunes::theory::core::{chord, ChordPattern};

let c_maj7 = chord(C4, &ChordPattern::MAJOR7);
comp.track("arp").arpeggiate(&c_maj7, 0.25);

// Chord progressions
comp.track("chords")
    .progression(C4, &ScalePattern::MAJOR, &[1, 5, 6, 4], 1.0);
```

**Chords:** `MAJOR`, `MINOR`, `DIMINISHED`, `AUGMENTED`, `MAJOR7`, `MINOR7`, `DOMINANT7`, `SUS2`, `SUS4`, `ADD9`, `NINTH`, `POWER`

## Synthesis

```rust
// Waveforms: Sine, Square, Sawtooth, Triangle, Noise
comp.track("synth")
    .waveform(Waveform::Sawtooth)
    .envelope(Envelope::new(0.001, 0.1, 0.0, 0.1))
    .notes(&[C4, E4, G4], 0.5);

// FM synthesis
comp.track("bell")
    .fm(FMParams::bell())
    .note(&[C5], 2.0);

// Additive synthesis
comp.track("saw")
    .additive_synth(&[1.0, 0.5, 0.33, 0.25, 0.2])
    .notes(&[C4, E4, G4], 0.5);
```

## Effects

```rust
comp.instrument("lead", &Instrument::synth_lead())
    .filter(Filter::low_pass(2000.0, 0.5))
    .distortion(Distortion::new(0.3, 0.5))
    .delay(Delay::new(0.375, 0.3, 0.4))
    .reverb(Reverb::new(0.5, 0.5, 0.3))
    .chorus(Chorus::new(0.5, 0.3, 0.5))
    .compressor(Compressor::new(-20.0, 4.0, 0.01, 0.1, 3.0))
    .notes(&[C4, E4, G4, C5], 0.5);
```

## Expression

```rust
comp.track("expressive")
    .volume(0.7)
    .pan(-0.5)           // Left
    .velocity(0.8)
    .bend(2.0)           // Bend up 2 semitones
    .vibrato(5.0, 0.5)   // rate (Hz), depth (semitones)
    .notes(&[C4, E4], 0.5);
```

## Timing

```rust
comp.track("melody")
    .at(0.0)       // Jump to specific time
    .note(&[C4], 0.5)
    .wait(1.0)     // Advance cursor
    .rest(0.25)    // Convenient wait alias
    .seek(-0.25)   // Relative positioning (can be negative)
    .note(&[E4], 0.5);

// Markers
comp.track("structure")
    .mark("verse_start")
    .notes(&[C4, E4, G4], 0.5)
    .at_mark("verse_start")  // Return to saved position
    .notes(&[C3, E3, G3], 0.5);  // Layered

// Swing
comp.track("groovy")
    .swing(0.67)  // Triplet feel (0.5 = straight, 0.75 = heavy)
    .notes(&[C4, D4, E4, F4], 0.125);
```

## Sections and Arrangement

```rust
// Define sections
comp.section("verse")
    .instrument("bass", &Instrument::sub_bass())
    .notes(&[C2, C2, G2, F2], 0.5)
    .and()  // Switch to another track
    .track("drums")
    .drum_grid(16, 0.125)
    .kick(&[0, 4, 8, 12]);

comp.section("chorus")
    .instrument("lead", &Instrument::synth_lead())
    .notes(&[C4, E4, G4, C5], 0.25);

// Arrange the song
comp.arrange(&["verse", "verse", "chorus", "verse", "chorus"]);
```

## Templates

```rust
// Save reusable configurations
comp.instrument("my_lead", &Instrument::synth_lead())
    .waveform(Waveform::Sawtooth)
    .filter(Filter::low_pass(2000.0, 0.5))
    .reverb(Reverb::new(0.3, 0.4, 0.2))
    .save_template("lead_template");

// Reuse
comp.from_template("lead_template", "melody")
    .notes(&[C4, E4, G4], 0.5);
```

---

# Mixer Layer

The `Mixer` is the audio rendering layer. It takes your musical composition and renders it into actual audio samples.

## Creating a Mixer

```rust
// From Composition (most common)
let mixer = comp.into_mixer();

// From a specific section
let verse_mixer = comp.section_to_mixer("verse")?;

// Direct construction
let mixer = Mixer::new(Tempo::new(120.0));
```

## Query Methods

```rust
let duration = mixer.total_duration();  // Duration in seconds
if mixer.is_empty() {
    println!("No audio to play!");
}
```

## Export Formats

```rust
// WAV (uncompressed)
mixer.export_wav("output.wav", 44100)?;

// FLAC (lossless compression, 50-60% smaller)
mixer.export_flac("output.flac", 44100)?;

// MIDI (notes only, no audio)
mixer.export_midi("song.mid")?;

// Stems (individual tracks)
mixer.export_stems("output/stems/", 44100)?;
mixer.export_stems_with_master("output/", 44100)?;
```

## MIDI Import

```rust
let mixer = Mixer::import_midi("song.mid")?;
engine.play_mixer(&mixer)?;
```

## Manipulation

```rust
let mixer = comp.into_mixer().repeat(3);  // Plays 4 times total
```

## Signal Flow

```
1. Tracks (individual instruments)
   ├─ Sample Generation
   ├─ Track Effects (EffectChain)
   └─ Output: Mono signal

2. Buses (groups of tracks)
   ├─ Sum all tracks in the bus
   ├─ Bus Effects (EffectChain)
   ├─ Bus Volume & Pan
   └─ Output: Stereo signal

3. Master (final mix)
   ├─ Sum all buses
   ├─ Master Effects (EffectChain)
   └─ Output: Stereo signal

4. Soft Clipping (prevent distortion)
```

## Bus System

```rust
// Assign tracks to buses during composition
comp.track("kick").bus("drums").drum(DrumType::Kick);
comp.track("snare").bus("drums").drum(DrumType::Snare);
comp.track("bass").bus("bass").notes(&[C2, G2], 0.5);

// Apply bus-level effects
let mut mixer = comp.into_mixer();
mixer.bus("drums")
    .reverb(Reverb::new(0.2, 0.3, 0.4))
    .compressor(Compressor::new(0.65, 4.0, 0.005, 0.05, 1.2))
    .volume(0.9);
```

## Master Effects

```rust
mixer.master_parametric_eq(ParametricEQ::new()
    .band(60.0, -3.0, 0.7)
    .band(3000.0, 2.0, 1.5));
mixer.master_compressor(Compressor::new(0.55, 2.5, 0.01, 0.12, 1.0));
mixer.master_limiter(Limiter::new(0.95, 0.05));
```

All 16 master effect methods available: `master_eq()`, `master_compressor()`, `master_reverb()`, `master_delay()`, `master_limiter()`, etc.

---

# AudioEngine Layer

The `AudioEngine` manages audio playback and real-time control.

## Creating an Engine

```rust
let engine = AudioEngine::new()?;  // Default buffer (4096 samples, ~93ms)

// Low latency for games
let engine = AudioEngine::with_buffer_size(1024)?;  // ~23ms latency
```

**Buffer size guidelines:**
- **512-1024**: Low latency for games, may glitch on slower CPUs
- **2048-4096**: Balanced for most applications (default: 4096)
- **8192+**: Very stable, higher latency

## Playback

### Blocking

```rust
engine.play_mixer(&mixer)?;  // Waits until playback finishes
```

### Non-blocking

```rust
let id = engine.play_mixer_realtime(&mixer)?;  // Returns immediately
engine.set_volume(id, 0.5)?;  // Control while playing
```

### Concurrent Sounds

```rust
let footstep_id = engine.play_mixer_realtime(&footstep)?;
let gunshot_id = engine.play_mixer_realtime(&gunshot)?;
let music_id = engine.play_mixer_realtime(&music)?;
// All three mix in real-time
```

### Looping

```rust
let loop_id = engine.play_looping(&background_music)?;
// ...
engine.stop(loop_id)?;
```

## Real-Time Control

```rust
let id = engine.play_mixer_realtime(&mixer)?;

engine.set_volume(id, 0.5)?;        // 0.0-1.0
engine.set_pan(id, -1.0)?;          // -1.0 (left) to 1.0 (right)
engine.set_playback_rate(id, 1.5)?; // Speed + pitch

engine.pause(id)?;
engine.resume(id)?;
engine.stop(id)?;

if engine.is_playing(id) { /* ... */ }
```

## Export

```rust
// Uses engine's sample rate (matches playback)
engine.export_wav(&mut mixer, "output.wav")?;
engine.export_flac(&mut mixer, "output.flac")?;
```

---

# Key Design Decisions

## Blocking vs Non-Blocking Playback

**Blocking (`play_mixer`):**
- Simple scripts and examples
- Sequential playback
- Wait for completion

**Non-blocking (`play_mixer_realtime`):**
- Games and interactive applications
- Concurrent sound effects
- Dynamic control

## Sample Rate Considerations

The `AudioEngine` automatically uses your system's native sample rate (typically 44100 or 48000 Hz).

```rust
// Engine export - uses engine's sample rate automatically
engine.export_wav(&mut mixer, "output.wav")?;

// Mixer export - you choose the sample rate
mixer.export_wav("output.wav", 48000)?;
```

**Prefer engine exports** unless you need a specific sample rate.

## Track vs Instrument

```rust
// instrument() - For synthesis and MIDI notes
comp.instrument("piano", &Instrument::electric_piano())
    .notes(&[C4, E4, G4], 0.5);

// track() - For samples, drums, and raw audio
comp.track("drums")
    .drum_grid(16, 0.125)
    .kick(&[0, 4, 8, 12]);
```

Both end up in the same `Mixer`, but the API reflects their different purposes.

---

# Complete Example

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Define verse
    comp.section("verse")
        .instrument("bass", &Instrument::sub_bass())
        .notes(&[C2, C2, G2, F2], 0.5)
        .and()
        .track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

    // Define chorus
    comp.section("chorus")
        .instrument("lead", &Instrument::synth_lead())
        .filter(Filter::low_pass(2000.0, 0.5))
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .notes(&[C4, E4, G4, C5], 0.25)
        .and()
        .instrument("bass", &Instrument::sub_bass())
        .notes(&[C2, C2, G2, F2], 0.5)
        .and()
        .track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 2, 4, 6, 8, 10, 12, 14])
        .snare(&[4, 12]);

    // Arrange and play
    comp.arrange(&["verse", "verse", "chorus", "verse", "chorus", "chorus"]);

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
```
