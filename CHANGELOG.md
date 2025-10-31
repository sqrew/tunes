# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
- 331 unit tests covering all modules
- 76 documentation tests with examples
- Comprehensive test coverage for composition, drums, effects, and theory

#### Examples
- 20+ complete working examples
- Demonstrations of all major features
- Classical technique examples
- Instrument and effect showcases
- Rhythm and pattern examples

[Unreleased]: https://github.com/sqrew/tunes/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/sqrew/tunes/releases/tag/v0.1.0
