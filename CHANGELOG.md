# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Massive Instrument Library Expansion - 105 Total Instruments!
- **Bass category** - `bass_808()`, `slap_bass()`, `synth_bass()` (+3 presets)
- **Lead category** - `laser_lead()`, `detuned_lead()`, `scream_lead()` (+3 presets)
- **Pad category** - `dark_pad()`, `shimmer_pad()`, `string_pad()` (+3 presets)
- **Orchestral category** - `oboe()`, `bassoon()`, `french_horn()`, `harp()`, `alto_sax()`, `tenor_sax()`, `soprano_sax()`, `baritone_sax()`, `trombone()`, `tuba()`, `piccolo()`, `english_horn()` (+12 presets)
- **Keys category** - `clavinet()`, `wurlitzer()`, `toy_piano()`, `hammond_organ()`, `church_organ()`, `reed_organ()`, `accordion()` (+7 presets)
- **Synth category** - `acid_synth()`, `trance_synth()`, `analog_brass()`, `fm_bass()`, `pwm_bass()`, `pluck_bass()` (+6 presets)
- **NEW: Strings category** - `violin()`, `viola()`, `cello()`, `double_bass()`, `pizzicato_strings()`, `tremolo_strings()`, `slow_strings()` (+7 presets)
- **NEW: Vocal category** - `choir_aahs()`, `choir_oohs()`, `synth_voice()` (+3 presets)
- **NEW: Ethnic/World category** - `sitar()`, `pan_flute()`, `didgeridoo()`, `shamisen()`, `bagpipes()`, `kalimba()`, `koto()`, `banjo()`, `tabla()`, `erhu()`, `dulcimer()` (+11 presets)
- **NEW: Percussion category** - `vibraphone()`, `glockenspiel()`, `tubular_bells()`, `steel_drums()`, `music_box()`, `celesta()`, `xylophone()`, `marimba()`, `bells()`, `cowbell()`, `timpani()`, `taiko_drum()` (+12 presets)
- **NEW: Guitars category** - `acoustic_guitar()`, `electric_guitar_clean()`, `electric_guitar_distorted()`, `guitar_12_string()` (+4 presets)
- **Total instrument presets: 33 → 105** (+72 new instruments, +218% growth!)
- **Now includes 13 instrument categories**: bass, leads, pads, keys, orchestral, fx, synths, strings, vocal, ethnic, percussion, guitars

### Fixed

#### Filter Cutoff Modulation Bug
- **Critical bug fix** - Fixed filter cutoff LFO modulation producing distorted static/buzzing sounds
- **Root cause** - Filter modulation was compounding every audio sample, causing cutoff frequency to spiral exponentially
- **Solution** - Store and restore base filter parameters each sample to prevent modulation compounding
- **Impact** - All instruments with FilterCutoff modulation now work correctly (affects ~30 presets including brass, pads, leads, synth bass)
- **Additional improvements**:
  - Reduced filter parameter smoothing from 0.999 to 0.95 for better modulation response
  - Changed filter stability checks to clamp state values instead of resetting (prevents audio glitches)
  - Filter modulation now sounds smooth and musical as intended

### Changed

#### Project Structure Refactoring
- **Reorganized codebase** into logical module hierarchy for improved discoverability and maintainability
- **`src/consts/`** - Musical constants (notes, scales, chords) now grouped together
- **`src/instruments/`** - Instrument presets organized by category (bass, leads, pads, keys, orchestral, fx, synths)
- **`src/synthesis/`** - All synthesis modules unified (waveform, envelope, lfo, filter, noise, automation, sample, effects, fm_synthesis, granular, wavetable, filter_envelope)
- **`src/theory/`** - Music theory modules grouped (core theory, microtonal systems, key signatures)
- **`src/composition/`** - Composition tools consolidated (drums, drum_grid, rhythm, patterns, sections)
- **`src/track/`** - Track system modularized (events, track, mixer, export)
- **`src/sequences/`** - Sequences showcased in organized categories:
  - `mathematical/` - Number theory sequences (fibonacci, primes, arithmetic, geometric, triangular, powers_of_two, collatz)
  - `rhythmic/` - Rhythmic patterns (euclidean, golden_ratio_rhythm, shepard_tone)
  - `generative/` - Algorithmic generation (random_walk, logistic_map, cellular_automaton, lsystem, markov, cantor_set, and more)
  - `musical/` - Musical transformations (harmonic_series, golden_ratio, normalize, map_to_scale)
- All public APIs remain unchanged - fully backward compatible

## [0.2.0] - 2025-10-31

### Added

#### Stem Export
- **`.export_stems()`** - Export individual tracks as separate WAV files for external mixing
- **`.export_stems_with_master()`** - Export stems plus master mix in one operation
- Automatic directory creation and filename sanitization
- Each stem preserves all track effects, filters, panning, and processing chain
- Perfect for professional production workflows, remixing, and collaboration

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
