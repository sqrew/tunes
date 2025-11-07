# tunes

A standalone Rust library for music composition, synthesis, and audio generation. Build complex musical pieces with an intuitive, expressive API — no runtime dependencies required.

Perfect for algorithmic music, game audio, generative art, and interactive installations.

## Features

- **Music Theory**: Scales, chords, patterns, progressions, and transposition
- **Composition DSL**: Fluent API for building musical sequences
- **Sections & Arrangements**: Create reusable sections (verse, chorus, bridge) and arrange them
- **Synthesis**: FM synthesis, Granular synthesis, filter envelopes, wavetable oscillators
- **Sample Playback**: Load and play WAV files with pitch shifting
- **Rhythm & Drums**: Drum grids, euclidean rhythms, 808-style synthesis, and pattern sequencing
- **Instruments**: 100+ Pre-configured synthesizers, bass, pads, leads, guitars, percussion, brass, strings, woodwinds and more
- **Effects, Automation and Filters**: Delay, reverb, distortion, chorus, modulation, tremolo, autopan, gate, limiter, compressor, bitcrusher, eq, phaser, flanger, saturation, filters
- **Musical Patterns**: Arpeggios, ornaments, tuplets, classical techniques
- **Algorithmic Sequences**: Primes, Fib, 2^x, Markov, L-map, Collatz, Euclidean, Golden ratio, random/bounded walks, Thue-Morse, Recamán's, Van der Corput, L-System, Cantor, Shepherd, Cellular Automaton, and many more
- **Tempo & Timing**: Tempo changes, time signatures (3/4, 5/4, 7/8, etc.), key signatures with modal support
- **Key Signatures & Modes**: Major, minor, and all 7 Greek modes (Dorian, Phrygian, Lydian, etc.)
- **Real-time Playback**: Cross-platform audio output via cpal
- **Audio Export**: WAV (uncompressed), FLAC (lossless ~50-60% compression), STEM export
- **MIDI Import/Export**: Import Standard MIDI Files and export compositions to MIDI with proper metadata
- **Sample Import**: Load and manipulate WAV samples
- **Live Coding**: Hot-reload system - edit code and hear changes instantly

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tunes = "0.6.0"
```

### Platform Requirements

**Linux users** need ALSA development libraries:
```bash
# Debian/Ubuntu
sudo apt install libasound2-dev

# Fedora/RHEL
sudo dnf install alsa-lib-devel
```

**macOS and Windows** work out of the box with no additional dependencies.

## Quick Start: Super simple!!
### Real-time Playback

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));
    let eighth = comp.tempo().eighth_note();  //getting this value to use in the next example

    // This is where you can do everything. 
    // Notes and chords can be input as floats with frequencies in hz or using or by prelude constants
    // Durations can be input as a duration of seconds as a float or using durations inferred by tempo
    
    comp.instrument("piano", &Instrument::electric_piano())
        .note(&[C4], 0.5)    //plays a c4 for half a second
        .note(&[280.0], eighth); //plays 280.0 hz note for half a second
        //continue chaining methods after the second note if you want.
    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

### Export to WAV

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Create a melody with instruments and effects
    comp.instrument("lead", &Instrument::synth_lead())
        .filter(Filter::low_pass(1200.0, 0.6))
        .notes(&[C4, E4, G4, C5], 0.5);

    // Export to WAV file
    let mut mixer = comp.into_mixer();
    mixer.export_wav("my_song.wav", 44100)?;
    Ok(())
}
```

### Export to FLAC (Lossless Compression)

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("piano", &Instrument::electric_piano())
        .notes(&[C4, E4, G4, C5], 0.5);

    // Export to FLAC (50-60% smaller than WAV, lossless quality)
    let mut mixer = comp.into_mixer();
    mixer.export_flac("my_song.flac", 44100)?;
    Ok(())
}
```

### Sample Playback

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Load samples (drums, vocals, any WAV file)
    comp.load_sample("kick", "samples/kick.wav")?;
    comp.load_sample("snare", "samples/snare.wav")?;

    // Use samples in your composition
    comp.track("drums")
        .sample("kick")                    // Play at normal speed
        .sample("snare")
        .sample("kick")
        .sample_with_rate("snare", 2.0);   // Double speed (pitch up)

    // Mix samples with synthesis
    comp.instrument("bass", &Instrument::sub_bass())
        .notes(&[C2, C2, G2, G2], 0.5);

    let mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;
    Ok(())
}
```

### MIDI Import/Export

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    // Export: Create and export a composition to MIDI
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5], 0.5);
    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12]);

    let mixer = comp.into_mixer();
    mixer.export_midi("song.mid")?;

    // Import: Load a MIDI file and render to audio
    let mut imported = Mixer::import_midi("song.mid")?;
    imported.export_wav("output.wav", 44100)?;

    // Or play it directly
    let engine = AudioEngine::new()?;
    engine.play_mixer(&imported)?;
    Ok(())
}
```

### Live Coding (Hot Reload)

```bash
# 1. Copy the template
cp templates/live_template.rs my_live.rs

# 2. Start live coding mode
cargo run --bin tunes-live -- my_live.rs

# 3. Edit my_live.rs and save - hear changes instantly!
```

The live coding system watches your file and automatically:
- ✅ Recompiles when you save
- ✅ Stops the old version
- ✅ Starts playing the new version
- ✅ Shows compilation errors in real-time

Perfect for iterative composition, live performances, and experimentation!

```rust
// my_live.rs - edit and save to hear changes!
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(140.0));

    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12]);

    // Try changing notes here and saving!
    comp.instrument("lead", &Instrument::synth_lead())
        .notes(&[C4, E4, G4], 0.25);

    let mixer = comp.into_mixer();

    // 4096 samples = ~93ms latency - good balance for live coding
    let engine = AudioEngine::with_buffer_size(4096)?;

    // Loop playback (don't use .repeat() - creates too many events!)
    loop {
        engine.play_mixer(&mixer)?;
    }
}
```

**Important:**
- Don't use `.repeat(1000)` - it creates too many events for smooth real-time synthesis
- Instead, use a regular `loop` to play the mixer repeatedly
- Buffer size 4096 works well for most systems (increase to 8192 or 16384 if you hear glitches)


## Comparison with Other Music Programming Libraries

`tunes` occupies a unique position in the music programming landscape:

| Feature                  | SuperCollider | Sonic Pi        | Leipzig         | Strudel           | **tunes**          | Music21 |
|--------------------------|---------------|-----------------|-----------------|-------------------|--------------------|---------|
| **Type safety**          | No            | No              | No (Clojure)    | Partial (TS)      | **Yes (Rust)**     | No      |
| **Real-time audio**      | Yes           | Yes             | Yes (Overtone)  | Yes (Web Audio)   | **Yes**            | No      |
| **Sample playback**      | Yes           | Yes             | Yes (Overtone)  | Yes               | **Yes**            | No      |
| **WAV export**           | Yes (manual)  | No              | Via Overtone    | No (browser)      | **Yes (easy)**     | Yes     |
| **FLAC export**          | Yes (manual)  | No              | No              | No                | **Yes (easy)**     | No      |
| **MIDI import**          | Yes           | No              | No              | No                | **Yes**            | Yes     |
| **MIDI export**          | Yes           | No              | No              | No                | **Yes**            | Yes     |
| **Live coding**          | Yes           | Yes             | Partial         | Yes               | **Yes**            | No      |
| **Easy to learn**        | No            | Yes             | Medium          | Yes               | **Yes**            | Yes     |
| **No dependencies**      | No (needs SC) | No (needs Ruby) | No (Clojure+SC) | No (browser/Node) | **Yes**            | No      |
| **Algorithmic patterns** | Yes           | Yes             | Yes             | Yes               | **Yes**            | Yes     |
| **Music theory**         | Manual        | Manual          | Yes             | Some              | **Yes (built-in)** | Yes     |
| **Standalone binary**    | No            | No              | No              | No                | **Yes**            | No      |
| **Embeddable**           | No            | No              | No              | No                | **Yes**            | No      |

### When to use `tunes`

**tunes excels at:**
- Building Rust applications with music generation (games, art installations, tools)
- Algorithmic composition with type-safe APIs
- Offline music generation and batch processing
- Learning music programming without complex setup
- Prototyping musical ideas with immediate feedback

**Use alternatives if you need:**
- **SuperCollider**: Extreme synthesis flexibility and live coding ecosystem
- **Sonic Pi**: Classroom-ready live coding with visual feedback
- **Leipzig**: Functional composition with Clojure's elegance
- **Strudel**: Browser-based collaboration and live coding
- **Music21**: Academic music analysis and score manipulation

### tunes' unique position

`tunes` is the only **standalone, embeddable, type-safe** music library with synthesis + sample playback. It compiles to a single binary with no runtime dependencies, making it ideal for:
- Rust game developers (Bevy, ggez, etc.)
- Desktop music applications
- Command-line music tools
- Embedded audio systems

## Documentation

Run `cargo doc --open` to view the full API documentation with detailed examples for each module.

## Testing

```bash
cargo test
```

  * [ ] The library includes **882 comprehensive tests and 255 doc tests** ensuring reliability and correctness.

## Examples

Run the included **60+ examples** to hear the library in action:

```bash
# Sample playback (WAV file loading and playback)
cargo run --release --example sample_playback_demo

# Export to WAV file
cargo run --release --example wav_export_demo

# Synthesis showcase (FM, filters, envelopes)
cargo run --release --example synthesis_demo

# Theory and scales
cargo run --example theory_demo

# Effects and effect automation (dynamic parameter changes over time)
cargo run --example effects_showcase
cargo run --example automation_demo

# And many more...
cargo run -- example example-name-here
```

**Note:** Use `--release` for examples with very complex synthesis to avoid audio underruns.

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
