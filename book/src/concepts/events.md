# Audio Events

Audio events are the fundamental building blocks of a track. Every sound in a composition is represented as an event with a start time and type-specific parameters.

## The AudioEvent Enum

```rust
pub enum AudioEvent {
    Note(NoteEvent),           // Synthesized notes
    Drum(DrumEvent),           // Drum hits
    Sample(SampleEvent),       // WAV sample playback
    TempoChange(TempoChangeEvent),
    TimeSignature(TimeSignatureEvent),
    KeySignature(KeySignatureEvent),
}
```

---

## NoteEvent

Synthesized notes with full control over pitch, timbre, and expression.

| Field | Type | Description |
|-------|------|-------------|
| `frequencies` | `[f32; 8]` | Up to 8 frequencies for chords |
| `num_freqs` | `usize` | Number of active frequencies |
| `start_time` | `f32` | Start time in seconds |
| `duration` | `f32` | Sustain duration in seconds |
| `waveform` | `Waveform` | Sine, Square, Saw, Triangle |
| `envelope` | `Envelope` | ADSR amplitude envelope |
| `filter_envelope` | `FilterEnvelope` | ADSR filter modulation |
| `fm_params` | `FMParams` | FM synthesis parameters |
| `pitch_bend_semitones` | `f32` | Pitch bend amount |
| `custom_wavetable` | `Option<Wavetable>` | Custom waveform |
| `velocity` | `f32` | 0.0-1.0, affects dynamics |
| `spatial_position` | `Option<SpatialPosition>` | 3D audio position |

**Created via TrackBuilder:**
```rust
comp.track("melody")
    .note(C4, 0.5)                    // Simple note
    .notes(&[C4, E4, G4], 0.5)        // Chord
    .note_with_envelope(C4, 0.5, envelope);
```

---

## DrumEvent

Drum hits using built-in synthesis. Over 90 drum types available.

| Field | Type | Description |
|-------|------|-------------|
| `drum_type` | `DrumType` | Kick, Snare, HiHat, etc. |
| `start_time` | `f32` | Start time in seconds |
| `pitch_offset` | `f32` | Semitones offset (0.0 = default) |
| `velocity` | `f32` | 0.0-1.0, affects volume |
| `spatial_position` | `Option<SpatialPosition>` | 3D audio position |

**Created via DrumGrid:**
```rust
comp.track("drums")
    .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, "x---x---x---x---")
        .sound(DrumType::Snare, "----x-------x---"));
```

**Or directly:**
```rust
comp.track("drums")
    .drum(DrumType::Kick, 0.25);
```

---

## SampleEvent

Playback of loaded WAV samples with pitch shifting.

| Field | Type | Description |
|-------|------|-------------|
| `sample` | `Sample` | The loaded audio sample |
| `start_time` | `f32` | Start time in seconds |
| `playback_rate` | `f32` | Speed/pitch (1.0 = normal) |
| `volume` | `f32` | 0.0-1.0 |
| `spatial_position` | `Option<SpatialPosition>` | 3D audio position |

**Created via TrackBuilder:**
```rust
let sample = Sample::from_file("sound.wav")?;
comp.track("sfx")
    .sample(sample.clone(), 0.0)
    .sample_with_rate(sample, 2.0, 0.5);  // Octave up
```

---

## Metadata Events

These events don't produce sound but affect playback and export.

### TempoChangeEvent

```rust
pub struct TempoChangeEvent {
    pub start_time: f32,
    pub bpm: f32,
}
```

**Usage:**
```rust
comp.track("melody")
    .notes(&[C4, D4, E4], 0.5)
    .tempo(80.0)  // Slow down
    .notes(&[F4, G4, A4], 0.5);
```

### TimeSignatureEvent

```rust
pub struct TimeSignatureEvent {
    pub start_time: f32,
    pub numerator: u8,    // e.g., 3 in 3/4
    pub denominator: u8,  // e.g., 4 in 3/4
}
```

### KeySignatureEvent

```rust
pub struct KeySignatureEvent {
    pub start_time: f32,
    pub key_signature: KeySignature,
}
```

Used primarily for MIDI export to indicate key changes.

---

## Event Timing

All events have `start_time()` and `end_time()` methods:

```rust
impl AudioEvent {
    fn start_time(&self) -> f32;
    fn end_time(&self) -> f32;  // Includes release time for notes
}
```

**End time calculation:**
- **Notes:** `start_time + envelope.total_duration(duration)`
- **Drums:** `start_time + drum_type.duration() / pitch_ratio`
- **Samples:** `start_time + sample.duration / playback_rate`
- **Metadata:** Same as start time (instantaneous)

---

## Transforms and Events

Pattern transforms operate on events within the `[pattern_start, cursor)` range:

```rust
comp.track("melody")
    .pattern_start()           // Mark start
    .notes(&[C4, D4, E4], 0.5) // Events created here
    .humanize(0.01, 0.05);     // Transform affects those events
```

Both `NoteEvent` and `DrumEvent` support:
- Timing transforms (humanize, swing, retrograde)
- Pitch transforms (shift, mutate, invert) - uses `pitch_offset` for drums
- Velocity transforms (crescendo, decrescendo)

---

## Direct Event Access

For advanced use, access events directly on a Track:

```rust
let mixer = comp.into_mixer();
for track in mixer.tracks() {
    for event in &track.events {
        match event {
            AudioEvent::Note(n) => println!("Note at {}", n.start_time),
            AudioEvent::Drum(d) => println!("Drum at {}", d.start_time),
            _ => {}
        }
    }
}
```

---

**Next:** Learn about [Tempo and Rhythm](./tempo-rhythm.md) for timing and drum patterns.
