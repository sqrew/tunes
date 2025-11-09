# Architecture Overview

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

## The Three Layers

### 1. Composition – The Musical Layer

`Composition` is where you think musically. It understands:
- **Tempo** – Beats per minute, note durations (quarter notes, eighths, etc.)
- **Music Theory** – Scales, chords, key signatures, modes
- **Structure** – Sections (verse, chorus), arrangements, repeats
- **Instruments** – Named instruments with synthesis parameters

```rust
let mut comp = Composition::new(Tempo::new(120.0));
let quarter = comp.tempo().quarter_note();  // Tempo-aware duration

comp.instrument("piano", &Instrument::electric_piano())
    .notes(&[C4, E4, G4, C5], quarter);  // Think in musical terms
```

**Use Composition when:** You're thinking about music – melodies, harmonies, structure, timing.

---

### 2. Mixer – The Audio Rendering Layer

`Mixer` doesn't know about tempo or music theory. It's pure audio:
- Organizes tracks into buses (groups)
- Applies effects at three levels: track, bus, and master
- Renders all audio to stereo output
- Handles sample-accurate timing
- Can be exported or played

```rust
let mixer = comp.into_mixer();  // Convert musical -> audio
```

The `Mixer` is where your musical ideas become audio data. It's the rendering engine with professional bus architecture.

**Signal flow inside Mixer:**
1. **Tracks** - Individual instruments with track-level effects
2. **Buses** - Groups of tracks (e.g., "drums", "vocals") with bus-level effects
3. **Master** - Final stereo mix with master-level effects (EQ, compression, limiting)
4. **Output** - Soft-clipped audio ready for playback or export

**Use Mixer directly when:** You need low-level audio control, custom sample rates, master effects, or offline rendering without an AudioEngine.

---

### 3. AudioEngine – The Playback Layer

`AudioEngine` manages audio output and real-time control:
- Creates persistent audio stream (one per engine)
- Mixes multiple sounds concurrently
- Provides real-time control (volume, pan, playback rate)
- Handles export with automatic sample rate matching

```rust
let engine = AudioEngine::new()?;

// Concurrent playback
let sound1 = engine.play_mixer_realtime(&mixer1)?;
let sound2 = engine.play_mixer_realtime(&mixer2)?;

// Real-time control
engine.set_volume(sound1, 0.5)?;
engine.set_pan(sound2, -0.8)?;  // Pan left
```

**Use AudioEngine when:** You need real-time playback, concurrent sounds, or dynamic audio control.

---

## Data Flow: From Music to Sound

Here's the typical flow for creating and playing music:

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

## Key Design Decisions

### Blocking vs Non-Blocking Playback

```rust
// Blocking - waits until audio finishes
engine.play_mixer(&mixer)?;
println!("Audio finished!");  // Only prints after playback

// Non-blocking - returns immediately with SoundId
let id = engine.play_mixer_realtime(&mixer)?;
engine.set_volume(id, 0.8)?;  // Control while playing
println!("Audio started!");  // Prints immediately
```

**When to use blocking (`play_mixer`):**
- Simple scripts and examples
- Sequential playback (one sound after another)
- When you want to wait for audio to finish

**When to use non-blocking (`play_mixer_realtime`):**
- Games and interactive applications
- Concurrent sound effects
- Dynamic audio control
- Real-time parameter changes

---

### Sample Rate Considerations

The `AudioEngine` automatically uses your system's native sample rate (typically 44100 or 48000 Hz). All rendering happens at this rate.

```rust
let engine = AudioEngine::new()?;

// Engine export - uses engine's sample rate automatically
engine.export_wav(&mut mixer, "output.wav")?;

// Mixer export - you choose the sample rate
mixer.export_wav("output.wav", 48000)?;  // Explicit control
```

**Prefer engine exports** unless you need a specific sample rate. The engine ensures playback and export match perfectly.

---

### Track vs Instrument

You'll see both `.track()` and `.instrument()` in the API:

```rust
// instrument() - For synthesis and MIDI notes
comp.instrument("piano", &Instrument::electric_piano())
    .notes(&[C4, E4, G4], 0.5);

// track() - For samples, drums, and raw audio
comp.track("drums")
    .drum_grid(16, 0.125)
    .kick(&[0, 4, 8, 12]);
```

**The difference:**
- `.instrument()` – Uses synthesizers, applies MIDI-like parameters (pitch, velocity)
- `.track()` – Plays samples directly, drum synthesis, raw audio events

Both end up in the same `Mixer`, but the API reflects their different purposes.

---

### Concurrent Mixing (v0.7.0+)

The `AudioEngine` uses a persistent audio stream with lock-free command channels:

```rust
let engine = AudioEngine::new()?;  // Creates ONE stream

// These play concurrently, mixed in real-time
let drums = engine.play_mixer_realtime(&drum_mixer)?;
let bass = engine.play_mixer_realtime(&bass_mixer)?;
let melody = engine.play_mixer_realtime(&melody_mixer)?;

// All three are mixed together and sent to your speakers
```

**How it works:**
1. Each `play_mixer_realtime()` sends a command to the audio thread
2. The audio thread maintains a collection of active sounds
3. Every audio frame, all active sounds are mixed together
4. Control commands (volume, pan, stop) are processed lock-free

This architecture enables:
- Hundreds of concurrent sounds (limited only by CPU)
- Sub-millisecond control latency
- No audio dropouts from starting new sounds
- Perfect synchronization between sounds

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

For most use cases, you'll work primarily with `Composition` and `AudioEngine`, letting the `Mixer` handle the audio rendering and routing behind the scenes. The bus system becomes important when you want professional mixing workflows or need to apply effects to groups of tracks.

---

**Next:** Explore each layer in detail:
- [Composition Layer](./composition.md) - Build musical ideas
- [Mixer Layer](./mixer.md) - Understand audio rendering
- [AudioEngine Layer](./engine.md) - Play and control audio

Or jump to [Game Audio Patterns](../game-audio/concurrent-sfx.md) for practical examples →
