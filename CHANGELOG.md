# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Karplus-Strong Physical Modeling Synthesis
- **New synthesis technique** - Physical modeling for realistic plucked string sounds
- **`KarplusStrong`** - Simulates plucked strings using delay line with feedback filtering
- **Key features:**
  - Minimal computational cost for authentic guitar, harp, and plucked instrument sounds
  - Configurable decay (sustain length) - 0.99=staccato, 0.996=realistic guitar, 0.999=long sustain
  - Configurable brightness (high-frequency content) - 0.0=dark/muffled, 0.5=balanced, 1.0=bright/metallic
  - `pluck()` method to re-excite without recreating the object
  - Deterministic output with `with_seed()` for reproducible sounds
- **How it works:**
  - Delay buffer filled with noise acts as initial pluck
  - Buffer feeds back through lowpass filter creating harmonic decay
  - Buffer length determines pitch, filter determines timbre
  - Natural-sounding attack and decay without explicit envelopes
- **Use cases:**
  - Acoustic guitar sounds
  - Harp and lute timbres
  - Pizzicato strings
  - Realistic plucked bass
  - Minimal-CPU string synthesis
- **API:**
  ```rust
  let mut string = KarplusStrong::new(440.0, 44100.0)
      .with_decay(0.996)      // Realistic sustain
      .with_brightness(0.5);  // Balanced tone

  let samples = string.generate(44100); // 1 second
  string.pluck(); // Re-trigger for new note
  ```
- **10 comprehensive tests** covering buffer creation, decay behavior, pluck re-excitation, brightness effects, and determinism
- **Exported** in `synthesis` module and prelude

#### Expanded Noise Generator Suite
- **4 new noise types** - Pink, Blue, Green, and Perlin noise (expanded from 2 to 6 total)
- **Comprehensive spectral coverage** - From low-frequency rumble to high-frequency sizzle
- **Coherent noise** - Perlin noise for smooth organic modulation

**Pink Noise** (1/f spectrum):
- **`PinkNoise`** - Equal energy per octave, balanced and natural-sounding
- **Implementation:** Voss-McCartney algorithm with 7 octaves for high-quality 1/f spectrum
- **Use cases:**
  - Audio testing (more representative of real-world sounds than white noise)
  - Ambient soundscapes and background textures
  - Gentle, non-fatiguing sound
  - Professional audio calibration

**Blue Noise** (High-frequency emphasis):
- **`BlueNoise`** - Increases energy at higher frequencies (+3dB per octave)
- **Implementation:** Differentiation of white noise (opposite of pink noise)
- **Use cases:**
  - Dithering in digital audio processing
  - High-frequency textures (sizzle, air, breath sounds)
  - Complementary to bass-heavy content
  - Crispy, bright sound effects

**Green Noise** (Midrange emphasis):
- **`GreenNoise`** - Emphasizes frequencies around 500Hz, tuned to human hearing
- **Implementation:** Pink noise with gentle highpass for midrange peak
- **Use cases:**
  - Nature sounds (rustling leaves, gentle rain)
  - Relaxation and meditation audio
  - Organic, natural-sounding textures
  - Pleasant, non-fatiguing background

**Perlin Noise** (Coherent gradient noise):
- **`PerlinNoise`** - Smooth, organic, continuous noise for modulation
- **Implementation:** 1D Perlin noise using same algorithm as `sequences::perlin_noise`
- **Key features:**
  - Configurable frequency (0.001-0.1 for different speeds of variation)
  - Deterministic with `with_seed()` for reproducibility
  - `set_frequency()` for runtime control
  - `reset()` to return to initial phase
- **Use cases:**
  - LFO modulation (more natural than sine/triangle waves)
  - Organic filter sweeps and automation
  - Smooth vibrato/tremolo depth variation
  - Evolving pad textures
  - Wind and breath sounds
  - Natural parameter modulation
- **API:**
  ```rust
  let mut perlin = PerlinNoise::new()
      .with_frequency(0.02);  // Smooth variation

  let modulation = perlin.generate(1000);
  ```

**Noise Type Enum:**
- **`NoiseType`** updated with Pink, Blue, Green, and Perlin variants
- All noise types implement `NoiseGenerator` trait
- Seeded constructors for deterministic output
- **Total coverage:** White (flat), Brown (1/f²), Pink (1/f), Blue (f), Green (midrange), Perlin (coherent)

**23 comprehensive tests** covering all noise types with range validation, determinism, and spectral characteristics
**New example:** `noise_generator_showcase.rs` - Demonstrates all 6 noise types with usage patterns

#### Additive Synthesis
- **New synthesis module** - Build complex sounds by summing sine wave partials
- **`AdditiveSynth`** - Generate timbres from scratch using frequency components
- **`Partial`** - Individual sine wave component with frequency ratio and amplitude

**Key Features:**
- **Harmonic synthesis:** `Partial::harmonic(n, amplitude)` - Integer ratios (1, 2, 3...) for musical sounds
- **Inharmonic synthesis:** `Partial::inharmonic(ratio, amplitude)` - Non-integer ratios for bells and metallic timbres
- **Flexible configuration:**
  - `.add_partial()` - Builder pattern for individual partials
  - `.with_partials(vec)` - Bulk configuration
  - `set_frequency()` - Dynamic frequency control
  - `reset()` - Reset phase accumulators
- **Phase control:** Optional phase offset for each partial (0.0-1.0)

**Sound Building Blocks:**
- **Sawtooth:** All harmonics with 1/n amplitude
- **Square:** Odd harmonics only (1, 3, 5, 7, 9...)
- **Organ sounds:** Selected harmonics (1, 2, 3, 4, 5, 8) for warm timbre
- **Bells:** Inharmonic partials (1.0, 2.76, 5.4, 8.93, 13.34)
- **Gongs:** Complex inharmonic spectrum for metallic resonance
- **Custom timbres:** Precise control over each frequency component

**Use Cases:**
- Organ and electric piano sounds
- Bell, chime, and gong synthesis
- Evolving pads with time-varying partials
- Spectral composition and sound design
- Educational tool for understanding harmonics
- Building waveforms from first principles

**API:**
```rust
// Create sawtooth-like sound
let mut synth = AdditiveSynth::new(440.0, 44100.0)
    .add_partial(Partial::harmonic(1, 1.0))
    .add_partial(Partial::harmonic(2, 0.5))
    .add_partial(Partial::harmonic(3, 0.33));

// Bell with inharmonic partials
let mut bell = AdditiveSynth::new(220.0, 44100.0)
    .add_partial(Partial::inharmonic(1.0, 1.0))
    .add_partial(Partial::inharmonic(2.76, 0.6))
    .add_partial(Partial::inharmonic(5.4, 0.4));

let samples = synth.generate(44100);
```

**13 comprehensive tests** covering partial creation, harmonic/inharmonic synthesis, phase control, and multi-partial summation
**New example:** `additive_synthesis_demo.rs` - Demonstrates building various timbres from partials
**Exported** in `synthesis` module and prelude

#### Parametric EQ - Professional Multi-Band Frequency Shaping
- **New effect** - Essential mixing and mastering tool for surgical frequency control
- **`ParametricEQ`** - Multi-band equalizer with peaking filters
- **`EQBand`** - Individual biquad peaking filter with frequency, gain, and Q control
- **`EQPreset`** - 5 professional presets for common scenarios

**Key Features:**
- **Surgical precision:** Boost or cut specific frequencies with adjustable bandwidth
- **Multi-band:** Add unlimited bands, each with independent control
- **Biquad filters:** Industry-standard peaking EQ implementation
- **Runtime control:**
  - `enable_band(index, bool)` - Enable/disable individual bands
  - `update_band(index, freq, gain_db, q, sample_rate)` - Dynamic parameter changes
  - `reset()` - Clear filter state
- **Parameters:**
  - **Frequency:** Center frequency to affect (Hz)
  - **Gain:** Boost (+) or cut (-) in dB
  - **Q (bandwidth):** 0.5=wide/gentle, 2.0=medium, 10.0=narrow/surgical

**Professional Presets:**
- **`EQPreset::VocalClarity`** - Cut rumble (100Hz, -6dB), reduce mud (250Hz, -3dB), boost presence (3kHz, +4dB), tame sibilance (8kHz, -2dB)
- **`EQPreset::BassBoost`** - Enhance sub (60Hz, +4dB) and bass (120Hz, +3dB), clean up mud (300Hz, -2dB)
- **`EQPreset::BrightAiry`** - Boost presence (5kHz, +3dB), air (10kHz, +4dB), and sparkle (15kHz, +2dB)
- **`EQPreset::Telephone`** - Lo-fi effect: cut lows (200Hz, -12dB), boost mids (1kHz, +6dB), cut highs (4kHz, -12dB)
- **`EQPreset::Warmth`** - Enhance low-mids (200Hz, +3dB), warmth (500Hz, +2dB), reduce harshness (8kHz, -2dB)

**Common Mixing Techniques:**
- Cut before boost (remove problems first)
- High-pass at 80-100Hz (remove rumble)
- Boost 3-5kHz (vocal presence and clarity)
- Cut 200-400Hz (reduce muddiness)
- Boost 10kHz+ (add air and sparkle)
- Narrow Q for surgical cuts, wide Q for gentle shaping

**Use Cases:**
- Vocal processing (clarity, de-essing, warmth)
- Instrument balancing in mixes
- Mastering and frequency correction
- Creative effects (telephone, lo-fi, radio)
- Removing problematic resonances
- Enhancing desired frequency characteristics

**API:**
```rust
// Custom EQ for vocals
let mut eq = ParametricEQ::new()
    .band(100.0, -6.0, 1.0)    // Cut rumble
    .band(250.0, -3.0, 1.5)    // Reduce mud
    .band(3000.0, 4.0, 2.0)    // Presence boost
    .band(8000.0, -2.0, 1.5);  // Tame harshness

// Or use preset
let mut eq = ParametricEQ::new()
    .preset(EQPreset::VocalClarity);

// Process audio
for sample in &mut audio_buffer {
    *sample = eq.process(*sample, 0.0, 0);
}
```

**8 comprehensive tests** covering band creation, processing, presets, enable/disable, and state management
**New example:** `parametric_eq_demo.rs` - Demonstrates all presets and custom EQ configurations
**Exported** in `synthesis::effects` and prelude

#### Extended Jazz and Altered Chord Patterns
- **10 new chord patterns** - Comprehensive jazz harmony support (expanded from 14 to 24 total patterns)
- **Professional chord library** - Complete coverage for jazz, classical, and contemporary music
- **Essential harmony extensions** - 6th chords, extended chords (11th, 13th), and altered dominants

**Jazz and Extended Chords (6 patterns):**
- **`ChordPattern::MAJOR6`** - Major 6th chord (R-M3-P5-M6) - Classic jazz voicing, stable and warm
- **`ChordPattern::MINOR6`** - Minor 6th chord (R-m3-P5-M6) - Jazz minor color, sophisticated sound
- **`ChordPattern::DOMINANT7SUS4`** - Dominant 7 sus4 (R-P4-P5-m7) - Suspended dominant for tension/resolution
- **`ChordPattern::MINOR_MAJOR7`** - Minor-major 7th (R-m3-P5-M7) - "James Bond chord", dramatic and tense
- **`ChordPattern::ELEVENTH`** - 11th chord (R-M3-P5-m7-M9-P11) - Rich extended harmony, complex voicing
- **`ChordPattern::THIRTEENTH`** - 13th chord (R-M3-P5-m7-M9-P11-M13) - Maximum extension, full spectrum harmony

**Altered Dominant Chords (4 patterns):**
- **`ChordPattern::DOMINANT7SHARP9`** - Dominant 7♯9 (R-M3-P5-m7-♯9) - "Hendrix chord", bluesy and tense
- **`ChordPattern::DOMINANT7FLAT9`** - Dominant 7♭9 (R-M3-P5-m7-♭9) - Dark altered dominant, strong resolution tendency
- **`ChordPattern::DOMINANT7SHARP5`** - Dominant 7♯5 (R-M3-♯5-m7) - Augmented dominant, whole-tone flavor
- **`ChordPattern::DOMINANT7FLAT5`** - Dominant 7♭5 (R-M3-♭5-m7) - Flat-five dominant, diminished flavor

**Complete Chord Pattern Library (24 total):**
- **Triads (5):** Major, Minor, Diminished, Augmented, Sus4
- **Seventh Chords (5):** Dominant7, Major7, Minor7, Diminished7, HalfDiminished7
- **Extended & Altered (4):** Dominant7Sus4, MinorMajor7, Eleventh, Thirteenth
- **Sixth Chords (2):** Major6, Minor6
- **Altered Dominants (4):** Dominant7Sharp9, Dominant7Flat9, Dominant7Sharp5, Dominant7Flat5
- **Power Chords (2):** Power5 (root+fifth), Power5Octave (root+fifth+octave)
- **Add Chords (2):** Add9 (major+9th), MinorAdd9 (minor+9th)

**Use Cases:**
- Jazz compositions and reharmonization
- Complex chord progressions (ii-V-I with alterations)
- Modal interchange and borrowed chords
- Film scoring and dramatic harmony
- Contemporary fusion and neo-soul progressions
- Blues and bebop vocabulary
- Chord melody and voicing studies

**Musical Context:**
- **6th chords:** Replace major/minor 7ths for lighter, more open sound
- **Extended chords:** Add color and complexity to progressions
- **Altered dominants:** Increase tension and harmonic interest in V-I resolutions
- **Sus4 chords:** Create suspension and anticipation before resolution
- **Minor-major 7:** Classic film noir and dramatic tension chord

**API Examples:**
```rust
use tunes::prelude::*;

// Jazz ii-V-I with extensions
let dm11 = chord(&[D4], ChordPattern::ELEVENTH);
let g13 = chord(&[G4], ChordPattern::THIRTEENTH);
let cmaj7 = chord(&[C4], ChordPattern::MAJOR7);

// Altered dominant resolution
let g7alt = chord(&[G4], ChordPattern::DOMINANT7SHARP9);
let cmaj = chord(&[C4], ChordPattern::MAJOR);

// Classic 6th chord substitution
let cmaj6 = chord(&[C4], ChordPattern::MAJOR6);  // Instead of Cmaj7

// Minor-major chord (James Bond)
let am_maj7 = chord(&[A3], ChordPattern::MINOR_MAJOR7);
```

**4 new comprehensive tests:**
- `test_all_chord_patterns_generate` - Verifies all 24 patterns generate valid notes
- `test_jazz_chord_intervals` - Tests 6th, 11th, 13th chord structures
- `test_altered_dominant_chords` - Verifies all 4 altered dominant voicings
- `test_extended_chords` - Tests extended harmony (9th, 11th, 13th intervals)

**Completeness:** Library now covers essential jazz harmony, classical extensions, and contemporary chord vocabulary

#### Advanced Music Theory - Intervals, Voicing, and Voice Leading
- **Professional music theory utilities** - Essential tools for composition, arranging, and harmonic analysis
- **Interval semantics** - Calculate and name musical intervals between pitches
- **Chord voicing** - Generate inversions, close/open voicings, and custom bass notes
- **Voice leading** - Smooth voice motion with minimal movement between chords
- **13 comprehensive functions** for professional music theory workflows

**Interval Functions:**
- **`interval_between(from, to)`** - Calculate semitone distance between two frequencies
  - Returns signed integer (positive = up, negative = down)
  - Example: `interval_between(C4, G4)` → 7 (perfect fifth up)
- **`interval_name(semitones)`** - Convert semitone distance to musical interval name
  - Returns: "Unison", "Minor second", "Major second", "Minor third", "Major third", "Perfect fourth", "Tritone", "Perfect fifth", "Minor sixth", "Major sixth", "Minor seventh", "Major seventh", "Octave"
  - Useful for analysis, debugging, and educational applications
- **`Interval` enum** - Type-safe interval representation with semantic names
  - Variants: Unison, MinorSecond, MajorSecond, MinorThird, MajorThird, PerfectFourth, Tritone, PerfectFifth, MinorSixth, MajorSixth, MinorSeventh, MajorSeventh, Octave
  - Convert to/from semitones: `Interval::PerfectFifth.semitones()` → 7

**Chord Inversion:**
- **`chord_inversion(chord, inversion)`** - Generate chord inversions
  - `inversion = 0` → Root position (e.g., C-E-G)
  - `inversion = 1` → First inversion (e.g., E-G-C)
  - `inversion = 2` → Second inversion (e.g., G-C-E)
  - Automatically octave-transposes to maintain ascending order
  - Essential for voice leading and smooth bass lines
- **`chord_over_bass(chord, bass_note)`** - Custom bass note (slash chords)
  - Example: `chord_over_bass(G_maj, D4)` → G/D chord
  - Useful for modal harmony and pedal points

**Chord Voicing:**
- **`close_voicing(chord)`** - Compact voicing within one octave
  - Moves notes down to fit within 12 semitones
  - Example: C-E-G-C (wide) → C-E-G (close)
  - Useful for piano voicings and tight harmony
- **`open_voicing(chord, spread)`** - Spread chord across multiple octaves
  - `spread` parameter controls width (typically 1-3)
  - Creates spacious, orchestral voicings
  - Useful for strings, brass sections, and pad sounds

**Voice Leading:**
- **`voice_lead(from_chord, to_chord)`** - Smart voice leading with minimal movement
  - Finds the voicing of `to_chord` closest to `from_chord`
  - Minimizes total distance traveled by all voices
  - Creates smooth, connected harmonic motion
  - Essential for professional chord progressions
  - Example:
    ```rust
    let c_maj = chord(&[C4], ChordPattern::MAJOR);  // [C4, E4, G4]
    let f_maj = chord(&[F4], ChordPattern::MAJOR);  // [F4, A4, C5]
    let voiced = voice_lead(&c_maj, &f_maj);        // [C4, F4, A4]
    // Result: Minimal movement - C stays, E→F (up 1), G→A (up 2)
    ```
- **`voice_leading_distance(from, to)`** - Calculate total movement distance
  - Returns total Hz movement across all voices
  - Lower values = smoother voice leading
  - Useful for analyzing progressions and choosing optimal voicings

**Use Cases:**
- **Composition:** Create smooth chord progressions with voice_lead()
- **Arranging:** Generate inversions and voicings for different instruments
- **Analysis:** Calculate intervals and analyze harmonic relationships
- **Jazz/Classical:** Professional voice leading for ii-V-I progressions
- **Film scoring:** Smooth modulations and reharmonization
- **Education:** Teach interval recognition and voice leading principles
- **Bass line writing:** Use inversions to create walking bass patterns

**API Examples:**
```rust
use tunes::prelude::*;

// Interval analysis
let interval = interval_between(C4, G4);  // 7 semitones
let name = interval_name(interval);        // "Perfect fifth"

// Chord inversions for smooth bass line
let c_root = chord_inversion(&c_maj, 0);   // C-E-G (root position)
let c_first = chord_inversion(&c_maj, 1);  // E-G-C (first inversion)
let c_second = chord_inversion(&c_maj, 2); // G-C-E (second inversion)

// Slash chord (G major over D bass)
let g_over_d = chord_over_bass(&g_maj, D3);

// Close voicing for piano
let piano_voicing = close_voicing(&chord);

// Smooth voice leading for progression
let dm7 = chord(&[D4], ChordPattern::MINOR7);
let g7 = chord(&[G4], ChordPattern::DOMINANT7);
let cmaj7 = chord(&[C4], ChordPattern::MAJOR7);

// Voice lead through ii-V-I
let g7_voiced = voice_lead(&dm7, &g7);
let cmaj7_voiced = voice_lead(&g7_voiced, &cmaj7);
// Result: Smooth, minimal movement between all chords
```

**Musical Context:**
- **Inversions:** Essential for creating interesting bass lines and avoiding repeated root notes
- **Voice leading:** The foundation of counterpoint, classical harmony, and jazz arranging
- **Close voicing:** Standard for piano, guitar, and compact ensemble writing
- **Open voicing:** Used in orchestration for clarity and resonance
- **Minimal movement:** Professional arrangers always minimize voice motion for smooth sound

**Exported functions:**
- Core module: `chord_inversion`, `chord_over_bass`, `voice_lead`, `close_voicing`, `open_voicing`, `voice_leading_distance`
- Interval module: `interval_between`, `interval_name`, `Interval` enum
- All available in prelude for convenient access

#### Concurrent Audio Engine with Real-Time Mixing
- **Major refactor to concurrent architecture** - AudioEngine now maintains a persistent audio stream
- **Multi-sound playback** - Play unlimited sounds simultaneously with automatic mixing
- **Zero overhead triggering** - Eliminated 5-20ms stream creation overhead per sound
- **Real-time control** - New methods for controlling sounds during playback:
  - `set_volume(id, volume)` - Adjust volume (0.0 to 1.0)
  - `set_pan(id, pan)` - Stereo panning (-1.0=left, 0.0=center, 1.0=right)
  - `set_playback_rate(id, rate)` - Change speed and pitch (0.5=half speed/octave down, 2.0=double speed/octave up)
  - `pause(id)` / `resume(id)` - Pause and resume playback
  - `stop(id)` - Stop a sound immediately
  - `play_looping(mixer)` - Play a composition in a loop
  - `is_playing(id)` - Check if a sound is active
- **Export convenience methods** - AudioEngine now has export methods that automatically use the engine's sample rate:
  - `export_wav(&mut mixer, path)` - Export to WAV using engine's sample rate
  - `export_flac(&mut mixer, path)` - Export to FLAC using engine's sample rate
  - `render_to_buffer(&mut mixer)` - Render to in-memory buffer
  - **Why:** Prevents sample rate mismatches between playback and export
  - **Note:** Mixer-level methods (`mixer.export_wav(path, sample_rate)`) still available for standalone use
- **Game-ready** - Perfect for Bevy integration, UI sounds, real-time audio
- **Backward compatible** - `play_mixer()` and `play_mixer_prerender()` unchanged
- **New example:** `concurrent_playback_demo.rs` - Demonstrates overlapping sounds with real-time control
- **Dependencies:** Added `crossbeam` for thread-safe channels

#### Improved Mixer Volume Handling with Soft Clipping
- **Smart volume management** - Replaced naive track count division with soft clipping
- **Problem solved:** Previous behavior divided volume by number of tracks, causing unnecessarily quiet output when tracks don't play simultaneously
- **Solution:** Uses `tanh()` soft clipping for smooth saturation
- **Benefits:**
  - Maintains full volume when tracks play sequentially
  - Prevents harsh clipping when tracks overlap
  - Industry-standard approach used in professional audio software
  - Adds subtle warmth/saturation only when signals exceed 1.0
  - No more artificially quiet mixes with many tracks!
- **Example:** 20 tracks playing one at a time now have full volume instead of 5% volume
- New example: `volume_test.rs` demonstrating the improvement

#### New Algorithmic Sequence Generators
- **4 powerful new generators** - Lorenz Attractor, Circle Map, Polyrhythm, and Perlin Noise
- **Fills critical gaps** - Continuous chaotic system, phase-locking rhythms, mathematical cross-rhythms, and smooth organic modulation
- **44+ total generators** - Comprehensive algorithmic composition toolkit

**Lorenz Attractor** (Continuous Chaotic System):
- **`lorenz_attractor(σ, ρ, β, initial, dt, steps)`** - Generate smooth 3D chaotic trajectories
- **`lorenz_butterfly(steps)`** - Convenience function with classic parameters (σ=10, ρ=28, β=8/3)
- **Returns:** Vec<(f32, f32, f32)> - X/Y/Z coordinates tracing the butterfly attractor
- **Implementation:** Runge-Kutta 4th order integration for accuracy
- **Use cases:**
  - Smooth, flowing melodies without jumps (unlike discrete maps)
  - Parameter automation: X→pitch, Y→volume, Z→filter cutoff
  - Ambient textures, generative music, binaural effects
  - Never-repeating but bounded patterns
- **Key feature:** First continuous (not discrete) chaotic system in the library
- Discards first 100 transient steps for stable attractor behavior
- Coordinates span approximately -20 to 20, normalize to musical ranges

**Circle Map** (Arnol'd Tongue / Phase-Locking):
- **`circle_map(ω, k, initial, iterations)`** - Generate phase angles on unit circle
- **`circle_map_to_hits(ω, k, initial, iterations, threshold)`** - Convert to rhythm hits
- **`circle_map_hocket(ω, k, initial, iterations, threshold)`** - Generate complementary rhythms
- **Parameters:**
  - ω (omega): Driving frequency ratio (0.0-1.0). Rational = mode-locked, φ=0.618 = never locks
  - K: Coupling strength (0=pure rotation, 1=critical, >1=strong locking)
- **Use cases:**
  - Polyrhythmic patterns with smooth transitions between locked/chaotic
  - Metric modulation, phasing effects, groove generation
  - Golden ratio rhythms (ω=0.618) for non-repeating patterns
  - Hocket patterns (call-and-response)
- **Key feature:** Models phase-locking in oscillators, specifically designed for rhythm
- Exhibits Arnol'd tongues (triangular mode-locked regions in parameter space)

**Polyrhythm Generator** (Mathematical Cross-Rhythms):
- **`polyrhythm(ratios, total_length)`** - Generate multiple simultaneous subdivisions
- **`polyrhythm_cycle(ratios)`** - Auto-calculate LCM for complete cycle
- **`polyrhythm_timings(ratios, cycle_duration)`** - Get exact timing in beats
- **`lcm(numbers)`** - Calculate least common multiple for pattern lengths
- **Use cases:**
  - Classic polyrhythms: 3:4 (hemiola), 5:7, 7:11
  - Triple/quad polyrhythms: 3:4:5, 5:6:7:11
  - Metric modulation, phasing, Steve Reich-style patterns
  - Essential for rhythmic complexity in composition
- **Key feature:** Surprisingly missing from library, now fills critical gap
- Returns hit indices for each voice (easy integration with drum_grid)
- LCM calculation ensures complete pattern cycles

**Perlin Noise** (Smooth Organic Modulation):
- **`perlin_noise(seed, frequency, octaves, persistence, length)`** - Smooth continuous pseudo-random sequences
- **`perlin_noise_bipolar(seed, frequency, octaves, persistence, length)`** - Bipolar version in [-1, 1]
- **Returns:** Vec<f32> with controllable smooth randomness
- **Implementation:** Classic Perlin noise with Ken Perlin's improved fade function (6t^5 - 15t^4 + 10t^3)
- **Parameters:**
  - frequency: Speed of variation (0.01=slow drift, 0.5=fast changes)
  - octaves: Number of detail layers (1-8, Fractal Brownian Motion)
  - persistence: How much each octave contributes (typical: 0.5)
- **Use cases:**
  - Smooth filter sweeps (organic cutoff automation)
  - Volume automation (breathing pads, natural swells)
  - Vibrato/tremolo depth variation
  - Stereo panning (smooth movement)
  - Timbre evolution (overtone weight changes)
  - Rhythm humanization (subtle timing/velocity drift)
  - Pitch detune (natural variation)
- **Key feature:** Fills the gap between mechanical (sine) and jumpy (random walk) - controllable smooth randomness
- **Why important:** The "secret sauce" in modern synthesizers (Serum, Massive, Omnisphere all use Perlin for LFO modulation)
- Multi-octave support (FBM) adds natural texture at different scales
- Deterministic (same seed = same output) for reproducibility

**Technical improvements:**
- All generators include comprehensive tests (chaos verification, boundary checks, smoothness, determinism)
- Fully documented with musical applications and parameter exploration guides
- Integrated into existing sequences module hierarchy
- New examples: `new_sequences_demo.rs` (Lorenz/circle map/polyrhythm), `perlin_noise_demo.rs` (organic modulation)

**API Enhancement - Float Sequence Operations:**
- **`normalize_f32(sequence, min, max)`** - Normalize f32 sequences to a target range
  - Complements existing `normalize()` - Now have both u32 and f32 normalization
  - Map Lorenz coordinates to pitch/volume ranges, Perlin noise to filter cutoffs, etc.
- **`map_to_scale()` & `map_to_scale_f32()` - BREAKING CHANGE: Now return frequencies directly!**
  - **Both functions now return `Vec<f32>` (frequencies) instead of MIDI notes**
  - **Both accept `root: f32`** - Use note constants like `C4`, `D4` instead of MIDI numbers
  - **Direct path from any sequence to in-key melodies** - no conversion needed!
  - `map_to_scale()` - for integer sequences (Fibonacci, Collatz, primes, etc.)
  - `map_to_scale_f32()` - for continuous sequences (Lorenz, Perlin, circle maps, etc.)
  - Automatically normalizes input range (f32 version only) or wraps (u32 version)
- **Why needed:** Lorenz attractor, Perlin noise, circle maps, and other continuous generators return f32 values
- **Before/After comparison:**
  ```rust
  // OLD: MIDI numbers + manual conversion
  let fib = sequences::fibonacci(16);
  let midi = sequences::map_to_scale(&fib, &Scale::major(), 60, 2);  // u32 → Vec<u32>
  let freqs: Vec<f32> = midi.iter()
      .map(|&m| 440.0 * 2_f32.powf((m as f32 - 69.0) / 12.0))
      .collect();
  comp.track("melody").notes(&freqs, 0.25);

  // NEW: Note constants, direct frequencies!
  let fib = sequences::fibonacci(16);
  let melody = sequences::map_to_scale(&fib, &Scale::major(), C4, 2);  // f32 → Vec<f32>
  comp.track("melody").notes(&melody, 0.25);

  // Works for continuous sequences too!
  let phases = sequences::circle_map(0.618, 1.5, 0.0, 32);
  let melody = sequences::map_to_scale_f32(&phases, &Scale::minor(), C4, 2);
  comp.track("chaos").notes(&melody, 0.25);
  ```
- **Use cases:**
  - Lorenz attractor melodies that stay in key (D minor butterfly!)
  - Perlin noise for evolving pentatonic patterns
  - Circle map phases quantized to blues scale
  - Any continuous sequence → musical scale → ready to play!
- **Examples updated** - All examples and doc tests demonstrate the new functions for API discoverability
- Makes working with continuous sequences as easy as discrete ones

**Total sequence library: 44 generators:**
- Mathematical (7): Fibonacci, primes, Collatz, arithmetic, geometric, triangular, powers of two
- Chaotic maps (8): Logistic, tent, sine, Hénon, Baker's, Lorenz (NEW!)
- Fractal/recursive (6): L-systems, Thue-Morse, Cantor set, Recamán, van der Corput, cellular automata
- Rhythmic (5): Euclidean, golden ratio, Shepard tone, circle map (NEW!), polyrhythm (NEW!)
- Smooth noise (2): Random walk, bounded walk, **Perlin noise (NEW!)**
- Musical transformations (14): Harmonic series, undertone series, circle of fifths/fourths, Pythagorean tuning, just intonation, golden ratio, normalize, map to scale, etc.
- Stochastic: Markov chains

#### Massively Expanded Drum Library
- **69 new drum sounds** - Expanded from 22 to 91 total percussion instruments (4.1x increase!)
- **Comprehensive coverage** - MIDI percussion, orchestral, world music, hand percussion, electronic effects, variations, and legendary drum machines
- **Professional-grade variety** - Multiple variations of commonly-used drums for diverse sonic palettes
- **Complete 808 & 909 kits** - Iconic drum machine sounds from Roland's TR-808 and TR-909
- **Production-ready** - Covers acoustic, electronic, world music, experimental, and modern production needs

**First Expansion (11 drums):**
- **Simple Percussion:**
  - `DrumType::Claves` - Sharp wooden click (2500Hz, 20ms duration)
  - `DrumType::Triangle` - Metallic ding with odd harmonics (1.5s sustain)
  - `DrumType::SideStick` - Soft rim click (less aggressive than rimshot)
  - `DrumType::WoodBlock` - Dry, pitched click (1500Hz)
- **909 Electronic Drums:**
  - `DrumType::Kick909` - Punchier electronic kick with tanh() distortion
  - `DrumType::Snare909` - Brighter electronic snare (85% noise / 15% tone)
- **Latin Percussion:**
  - `DrumType::CongaHigh` - Bright, high-pitched hand drum (400Hz → 320Hz drop)
  - `DrumType::CongaLow` - Deep, resonant bass conga (180Hz → 140Hz drop)
  - `DrumType::BongoHigh` - Sharp, articulate bongo (500Hz → 420Hz drop)
  - `DrumType::BongoLow` - Deeper bongo with warmer tone (300Hz → 250Hz drop)
- **Utility:**
  - `DrumType::RideBell` - Metallic ping with inharmonic partials (4000Hz)

**Second Expansion (13 drums - MIDI percussion gap filling):**
- **Additional Toms:**
  - `DrumType::FloorTomLow` - Deep floor tom (80Hz → 70Hz)
  - `DrumType::FloorTomHigh` - Higher floor tom (110Hz → 95Hz)
- **Additional Hi-Hat:**
  - `DrumType::HiHatPedal` - Pedal hi-hat "chick" sound (GM #44)
- **Additional Cymbals:**
  - `DrumType::Crash2` - Second crash variation with slower decay
- **Special Effects:**
  - `DrumType::Vibraslap` - Distinctive rattling/buzzing percussion
- **Additional Latin Percussion:**
  - `DrumType::TimbaleHigh` - High timbale, metallic shell drum (850Hz)
  - `DrumType::TimbaleLow` - Low timbale (550Hz)
  - `DrumType::AgogoHigh` - High agogo bell, Brazilian (3500Hz)
  - `DrumType::AgogoLow` - Low agogo bell (2500Hz)
- **Additional Shakers/Scrapers:**
  - `DrumType::Cabasa` - Textured shaker/scraper hybrid
  - `DrumType::GuiroShort` - Short scraping sound (80ms)
  - `DrumType::GuiroLong` - Long scraping sound (200ms)
- **Additional Wood Percussion:**
  - `DrumType::WoodBlockHigh` - High-pitched wooden click (2500Hz)

**Third Expansion (15 drums - Orchestral, World, Hand Percussion, Effects):**
- **Orchestral Percussion:**
  - `DrumType::Timpani` - Tuned orchestral bass drum (80Hz, rich harmonics)
  - `DrumType::Gong` - Deep metallic crash with long decay (3.5s)
  - `DrumType::Chimes` - Tubular bells with bell-like inharmonic partials
- **World Percussion:**
  - `DrumType::Djembe` - West African hand drum with slap attack
  - `DrumType::TablaBayan` - Indian bass drum with pitch bend (150Hz → 100Hz)
  - `DrumType::TablaDayan` - Indian treble drum, bright ringing tone (400Hz)
  - `DrumType::Cajon` - Box drum with internal wire buzz, very popular
- **Hand Percussion:**
  - `DrumType::Fingersnap` - Clean snap sound with high-frequency click
  - `DrumType::Maracas` - Bright rattling shaker (distinct from generic shaker)
  - `DrumType::Castanet` - Spanish wooden clapper with sharp attack
  - `DrumType::SleighBells` - Jingle bells cluster with shimmer
- **Electronic / Effects:**
  - `DrumType::LaserZap` - Sci-fi pitch sweep (2000Hz → 80Hz)
  - `DrumType::ReverseCymbal` - Reversed crash buildup effect
  - `DrumType::WhiteNoiseHit` - Pure noise burst/clap effect
  - `DrumType::StickClick` - Drumstick click sound

**Fourth Expansion (18 drums - Variations of commonly-used percussion):**
- **Kick Variations (4):**
  - `DrumType::KickTight` - Short, punchy kick for electronic music (60ms)
  - `DrumType::KickDeep` - Extended low-end, longer decay (500ms)
  - `DrumType::KickAcoustic` - Natural drum kit sound with harmonics
  - `DrumType::KickClick` - Prominent beater attack for clarity
- **Snare Variations (4):**
  - `DrumType::SnareRim` - Rim-focused, minimal body (80ms)
  - `DrumType::SnareTight` - Short, dry, minimal resonance (70ms)
  - `DrumType::SnareLoose` - Longer ring, more wire buzz (180ms)
  - `DrumType::SnarePiccolo` - High-pitched, bright (350Hz)
- **Hi-Hat Variations (2):**
  - `DrumType::HiHatHalfOpen` - Between closed and open (100ms)
  - `DrumType::HiHatSizzle` - Lots of high-frequency content (200ms)
- **Clap Variations (4):**
  - `DrumType::ClapDry` - No reverb, tight (50ms)
  - `DrumType::ClapRoom` - Natural room ambience with tail
  - `DrumType::ClapGroup` - Multiple hand claps layered
  - `DrumType::ClapSnare` - Hybrid clap/snare sound
- **Cymbal Variations (2):**
  - `DrumType::CrashShort` - Quick crash, gated (500ms)
  - `DrumType::RideTip` - Bell-less ride, stick tip sound (600ms)
- **Shaker Variations (2):**
  - `DrumType::EggShaker` - Tight, short shake (80ms)
  - `DrumType::TubeShaker` - Longer, more sustained (250ms)

**Fifth Expansion (12 drums - 808/909 Kit Completion + Transition Effects):**
- **808 Kit Completion (5):**
  - `DrumType::Tom808Low` - Deep 808 tom (105Hz → 65Hz, triangle oscillators, 400ms)
  - `DrumType::Tom808Mid` - Mid 808 tom (145Hz → 90Hz, triangle oscillators, 350ms)
  - `DrumType::Tom808High` - High 808 tom (220Hz → 140Hz, triangle oscillators, 300ms)
  - `DrumType::Cowbell808` - Iconic 808 cowbell (540Hz + 800Hz square waves, 300ms)
  - `DrumType::Clave808` - Sharp 808 clave (2500Hz + 5000Hz sine, 25ms)
- **909 Kit Completion (5):**
  - `DrumType::HiHat909Closed` - Bright 909 closed hat (12kHz noise + 10.5kHz metallic, 50ms)
  - `DrumType::HiHat909Open` - Sustained 909 open hat (12kHz noise + metallic, 180ms)
  - `DrumType::Clap909` - Classic 909 clap (multiple noise bursts with offsets, 100ms)
  - `DrumType::Cowbell909` - Sharp 909 cowbell (587Hz + 845Hz triangle waves, 250ms)
  - `DrumType::Rim909` - 909 rim shot (1950Hz triangle + filtered noise, 60ms)
- **Transition Effects (2):**
  - `DrumType::ReverseSnare` - Snare buildup effect (reverse envelope, 1.2s)
  - `DrumType::CymbalSwell` - Building cymbal wash (gradual buildup then fade, 2.0s)
- **Technical improvements:**
  - Added `triangle_wave()` helper function for authentic 808 synthesis
  - 808 toms use dual triangle oscillators with pitch drops (characteristic TR-808 sound)
  - 808 cowbell uses square waves at specific harmonic ratios
  - 909 hi-hats use high-frequency noise with metallic overtones
  - 909 clap uses multiple time-offset noise bursts for realistic hand clap
  - Transition effects use reverse/building envelopes for modern production

- All 69 new drums have proper MIDI mappings for import/export compatibility
- New examples: `new_drums_demo.rs`, `midi_percussion_demo.rs`, `expanded_percussion_demo.rs`, `drum_variations_demo.rs`, `808_909_complete_demo.rs`
- **Final Breakdown:** 12 kicks, 11 snares, 9 hi-hats, 10 claps, 9 cymbals, 7 shakers, 8 toms, 3 cowbells
- **Complete 808 kit:** Kick, Snare, 2 HiHats, Clap, 3 Toms, Cowbell, Clave (12 total)
- **Complete 909 kit:** Kick, Snare, 2 HiHats, Clap, Cowbell, Rim (7 total)

#### Live Coding / Hot Reload System
- **`tunes-live` binary** - Watch and auto-reload composition code
- Edit your composition in Rust and hear changes instantly
- Automatic recompilation on file save
- Graceful restart of audio playback
- Real-time compilation error display
- Template file for quick start (`templates/live_template.rs`)
- Perfect for:
  - Iterative composition workflow
  - Live performances
  - Experimentation and learning
  - Rapid prototyping
- Simple workflow: `cargo run --bin tunes-live -- my_composition.rs`
- Uses file watching (`notify` crate) and process management
- No complex serialization - just recompiles and restarts

#### FLAC Export Support
- **`Mixer::export_flac(path, sample_rate)`** - Export compositions to FLAC format
- Lossless compression typically achieves 50-60% file size reduction compared to WAV
- Uses 24-bit depth for excellent audio quality
- Pure Rust implementation via `flacenc` crate (no system dependencies)
- Perfect for archival, sharing, and professional workflows
- Widely supported by DAWs, media players, and audio tools
- New example: `flac_export.rs` demonstrating FLAC export and size comparison

#### MIDI Import Support
- **`Mixer::import_midi(path)`** - Import Standard MIDI Files into tunes
- Converts MIDI notes to NoteEvent with proper frequency conversion
- Maps MIDI channel 10 (percussion) to DrumType automatically
- Supports tempo changes and time signature changes
- Preserves track names and MIDI program numbers
- Enables new workflows:
  - **MIDI to WAV conversion** - Import MIDI files and render as audio
  - **Round-trip testing** - Export to MIDI and import back
  - **MIDI analysis** - Extract note data from MIDI files
  - **Direct playback** - Play imported MIDI files through tunes engine
- New public helper functions:
  - `midi_note_to_frequency()` - Convert MIDI note numbers (0-127) to Hz
  - `midi_note_to_drum_type()` - Map General MIDI percussion notes to DrumType
- DrumType now derives `PartialEq` and `Eq` for comparison
- New example: `midi_import.rs` demonstrating all MIDI import workflows

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

#### AudioEngine API - BREAKING CHANGE
- **`play_mixer_realtime(&Mixer)` now returns `Result<SoundId>` instead of `Result<()>`**
  - **Breaking change** for code using `play_mixer_realtime()`
  - Enables non-blocking playback and real-time control
  - Migration: Store the returned `SoundId` to control the sound
  - **Before:** `engine.play_mixer_realtime(&mixer)?;`
  - **After:** `let sound_id = engine.play_mixer_realtime(&mixer)?;`
- **`play_mixer()` and `play_mixer_prerender()` are unchanged** (backward compatible)
- **Removed legacy methods** (unused by any examples or internal code):
  - `play()` - Play raw frequencies
  - `play_tempo()` - Play with tempo-based duration
  - `play_interpolated()` - Play frequency sweeps
  - `play_drum()` - Play drum sounds directly
  - All internal `run*()` methods
- These methods were superseded by the Composition → Mixer → AudioEngine pattern

### Changed (Continued)

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
- 942 unit tests covering all modules
- 299 documentation tests with examples
- Comprehensive test coverage for composition, drums, effects, synthesis, and theory

#### Examples
- 70+ complete working examples
- Demonstrations of all major features
- Classical technique examples
- Instrument and effect showcases
- Rhythm and pattern examples

[Unreleased]: https://github.com/sqrew/tunes/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/sqrew/tunes/releases/tag/v0.1.0
