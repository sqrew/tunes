# tunes

A Rust library for music composition, synthesis, and audio generation. Build complex musical pieces with an intuitive, expressive API.

## Features

- **Music Theory**: Scales, chords, patterns, progressions, and transposition
- **Composition DSL**: Fluent API for building musical sequences
- **Rhythm & Drums**: Drum grids, euclidean rhythms, and pattern sequencing
- **Instruments**: Pre-configured synthesizers, bass, pads, leads, and more
- **Effects**: Delay, reverb, distortion, chorus, filters, and modulation
- **Musical Patterns**: Arpeggios, ornaments, tuplets, classical techniques
- **Tempo & Timing**: Musical time abstractions (quarter notes, bars, beats)
- **Real-time Playback**: Cross-platform audio output via cpal

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tunes = "0.1.0"
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

## Quick Start

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Play a simple C major arpeggio
    comp.track("piano")
        .note(&[C4], 0.5)
        .note(&[E4], 0.5)
        .note(&[G4], 0.5)
        .note(&[C5], 0.5);

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
```

## Documentation

Run `cargo doc --open` to view the full API documentation with detailed examples for each module.

## Project Structure

- `src/composition/` - Musical composition DSL and builders
- `src/instruments/` - Pre-configured instrument presets
- `src/effects/` - Audio effects (reverb, delay, filters, etc.)
- `src/drums/` - Drum synthesis and sequencing
- `src/theory.rs` - Music theory (scales, chords, progressions)
- `src/engine.rs` - Audio playback engine
- `examples/` - 20+ complete examples demonstrating features

## Testing

```bash
cargo test
```

The library includes 331 unit tests and 76 documentation tests ensuring reliability.

## Examples

Run the included examples to hear the library in action:

```bash
# Theory and scales
cargo run --example theory_demo

# Instrument showcase
cargo run --example instrument_showcase

# Classical techniques
cargo run --example classical_techniques

# Effects demonstration
cargo run --example effects_showcase

# Drum patterns
cargo run --example drum_grid

# And many more...
```

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
