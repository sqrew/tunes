# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Granular Synthesis
- **Granular synthesis engine** - Break audio into tiny grains for texture creation and time manipulation
- **6 granular presets**: `texture()`, `time_stretch()`, `freeze()`, `glitch()`, `cloud()`, `default()`
- **`.granular()` method** on TrackBuilder for applying granular effects to samples
- **Hann window envelope** - Smooth grain edges to prevent clicks
- Time-stretching, spectral freezing, and glitch effects support

#### Noise Generators
- **White noise generator** - Equal energy at all frequencies for hi-hats, percussion, textures
- **Brown noise generator** - Low-frequency bias using random walk for bass rumbles and drones
- **`.noise()` method** on TrackBuilder - Add noise directly to compositions
- **NoiseGenerator trait** - Extensible system for custom noise implementations
- Seeded generators for deterministic noise patterns

#### Algorithmic Sequences
- **L-Systems (Lindenmayer Systems)** - Fractal pattern generation for organic melodies
- **Markov Chains** - Probabilistic sequence generation with weighted state transitions
- **Cantor Set** - Fractal rhythmic patterns through recursive subdivision
- **Scale Mapping** - Quantize sequences to musical scales with 12 scale types:
  - Major/Minor Pentatonic, Major, Minor, Harmonic Minor
  - Blues, Chromatic, Whole Tone
  - Dorian, Phrygian, Lydian, Mixolydian modes
- **Shepard Tone** - Circular pitch patterns for infinite rise/fall illusions

#### Sample Manipulation
- **`.slice(start, end)`** - Extract portions of audio samples
- **`.normalize()`** - Normalize sample amplitude to maximum level
- **`.reverse()`** - Reverse sample playback
- **`.fade_in(duration)` and `.fade_out(duration)`** - Smooth fade transitions
- **`.with_gain(gain)`** - Apply volume adjustment
- **`.from_mono()`** - Constructor for creating mono samples from raw data
- **Loop support** - `.with_loop(start, end)` for seamless sample looping

#### Filters
- **Moog ladder filter** - Classic analog filter with resonance and self-oscillation
- Four-pole cascade design with authentic Moog character
- Adjustable cutoff frequency and resonance

### Changed

#### Performance Optimizations
- **Effects optimization** - Improved Gate and Limiter implementations
- **Filter optimization** - Enhanced DSP efficiency across all filter types
- **Waveform generation** - Removed unnecessary modulo operations
- **Wavetable synthesis** - Pre-computed reciprocals in harmonic generation

### Fixed
- Noise duration bug - Duration parameter now correctly uses seconds (not beats)
- Cantor Set algorithm - Fixed incorrect subdivision logic
- Shepard Tone - Fixed out-of-range values in descending patterns
- Brown noise test - Made deterministic with seeded generator for reliability

## [0.1.0] - 2025-10-30

### Added

#### Core Features
- Initial release of musicrs music composition library
- Cross-platform audio engine using cpal for real-time playback
- Fluent composition DSL with method chaining

#### Music Theory
- Scale generation with 15+ scale types (Major, Minor, Pentatonic, Blues, etc.)
- Chord construction with 20+ chord types (Major, Minor, Diminished, Augmented, etc.)
- Chord progressions (I-IV-V, ii-V-I, etc.)
- Note transposition and sequence manipulation
- Scale degree functions

#### Composition & Sequencing
- Note and chord playback with configurable durations
- Pattern sequencing with repeat functionality
- Arpeggios with multiple directions (Up, Down, UpDown, Random)
- Musical time abstractions (quarter notes, eighth notes, bars, beats)
- Cursor-based timeline positioning
- Tempo-based duration calculations

#### Rhythm & Drums
- Comprehensive drum synthesis (kick, snare, hi-hat, toms, cymbals, percussion)
- Drum grid sequencer with step programming
- Euclidean rhythm generation
- Pattern-based drum sequencing
- Support for 30+ drum types

#### Tuplets & Timing
- Tuplet support (triplets, quintuplets, sextuplets, septuplets)
- Custom tuplet division
- Dotted note durations
- Precise timing control

#### Musical Patterns
- Ostinato (repeated patterns)
- Arpeggios with configurable speed and direction
- Tremolo effects
- Alberti bass patterns
- Waltz bass patterns
- Stride bass patterns
- Broken chord accompaniment
- Walking bass lines
- Pedal point

#### Ornaments
- Trills (upper and lower)
- Mordents (upper and lower)
- Turns and inverted turns
- Grace notes (single and multiple)
- Appoggiatura
- Acciaccatura
- Glissando

#### Portamento & Slides
- Smooth pitch transitions
- Linear, exponential, and logarithmic portamento curves
- Slide effects between notes

#### Instruments
- 15+ pre-configured instrument presets
- Synthesizer types: leads, pads, bass, plucks
- Acoustic simulations: piano, strings
- Custom waveform and envelope configuration
- ADSR envelope control

#### Effects
- Time-based: Delay, Reverb, Chorus, Flanger, Phaser
- Dynamics: Compressor, Saturation, Distortion
- Filters: Low-pass, High-pass, Band-pass with resonance
- EQ: 3-band parametric equalizer
- Bitcrusher for lo-fi effects
- Ring modulation

#### Modulation
- LFO (Low-Frequency Oscillator) system
- Multiple waveforms: Sine, Triangle, Sawtooth, Square, Random
- Modulation targets: Pitch, Volume, Filter Cutoff, Pan, Distortion
- Custom modulation routing

#### Expression & Dynamics
- Volume control per track
- Stereo panning (-1.0 to 1.0)
- Pitch bend (±24 semitones)
- Vibrato with configurable rate and depth
- Fade in/out transitions
- Per-note waveform and envelope control

#### Waveforms
- Sine, Square, Sawtooth, Triangle waves
- Pulse wave with variable width
- White noise
- Custom waveform support

#### Testing & Quality
- 607 unit tests covering all modules
- 178 documentation tests with examples
- Comprehensive test coverage for composition, drums, effects, synthesis, and theory

#### Examples
- 20+ complete working examples
- Demonstrations of all major features
- Classical technique examples
- Instrument and effect showcases
- Rhythm and pattern examples

[Unreleased]: https://github.com/sqrew/tunes/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/sqrew/tunes/releases/tag/v0.1.0
