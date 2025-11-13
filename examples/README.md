# Tunes Examples

This directory contains **92 example programs** demonstrating all features of the Tunes audio library. Examples are organized from beginner to advanced topics.

**Note:** Benchmark programs have been moved to `benches/`. Manual test programs are in `tests/integration/`.

---

## 🎯 Getting Started

**Start here if you're new to Tunes!**

### Quick Start (2 Lines of Code!)

| Example | Description |
|---------|-------------|
| `sample_playback_demo.rs` | **START HERE** - Play audio files with automatic caching (game audio) |

### Basic Synthesis

| Example | Description |
|---------|-------------|
| `synthesis_demo.rs` | Introduction to synthesis: AM, FM, filters |
| `notes_and_chords.rs` | Play individual notes and chord arrays |
| `waveforms.rs` | Explore sine, square, sawtooth, and triangle waves |
| `envelopes.rs` | ADSR envelope shaping for dynamic sounds |
| `filters.rs` | Low-pass, high-pass, and band-pass filtering |
| `instrument_showcase.rs` | Tour of all 150+ built-in instrument presets |

---

## 🎮 Game Audio & Real-Time

**Perfect for game developers! Simple, fast, and powerful.**

| Example | Description |
|---------|-------------|
| `sample_playback_demo.rs` | **Fire-and-forget audio:** `engine.play_sample("boom.wav")?` |
| `concurrent_playback_demo.rs` | Multiple simultaneous audio streams (50+ concurrent samples) |
| `spatial_audio_demo.rs` | **3D positional audio** with distance attenuation and panning |
| `doppler_effect_demo.rs` | **Doppler effect** for moving sound sources (cars, bullets) |

**Key Features for Games:**
- ✅ **Automatic caching** - Load once, play many times (instant)
- ✅ **SIMD acceleration** - 4-8 samples processed simultaneously
- ✅ **Concurrent mixing** - Hundreds of sounds playing at once
- ✅ **Non-blocking** - Returns immediately, no game loop stalls
- ✅ **3D spatial audio** - Position sounds in game worlds
- ✅ **Bevy integration** - See [book/game-audio/bevy-integration.md](../book/src/game-audio/bevy-integration.md)

---

## 🎹 Synthesis & Sound Design

| Example | Description |
|---------|-------------|
| `synthesis_demo.rs` | AM, FM, and subtractive synthesis overview |
| `additive_synthesis_demo.rs` | Build complex timbres from sine wave harmonics |
| `wavetable_synthesis.rs` | Wavetable oscillators with morphing |
| `wavetable_demo.rs` | Additional wavetable synthesis examples |
| `noise_generator_showcase.rs` | White, pink, brown noise generators |
| `808_909_complete_demo.rs` | TR-808 and TR-909 drum machine sounds |
| `lfo_modulation.rs` | Low-frequency oscillators for parameter modulation |

---

## 🔢 Algorithmic Composition & Sequences

**Generate music from mathematical patterns and algorithms.**

| Example | Description |
|---------|-------------|
| `sequences_showcase.rs` | **Complete tour of all 50+ sequence generators** |
| `mathematical_sequences.rs` | Fibonacci, primes, Collatz, triangular numbers |
| `rhythmic_sequences.rs` | Euclidean rhythms, golden ratio, polyrhythms |
| `musical_sequences.rs` | Harmonic series, scales, musical transformations |
| `generative_sequences.rs` | Random walks, L-systems, Markov chains |
| `chaotic_sequences.rs` | Chaos theory: logistic map, Lorenz attractor |
| `algorithmic_patterns.rs` | Scales, arpeggios, trills, portamento, inversions |
| `euclidean_rhythms.rs` | Mathematically optimal rhythm distribution |
| `harmonic_series.rs` | Overtone-based melodies and spectral chords |
| `perlin_noise_demo.rs` | Smooth noise for organic parameter changes |

---

## 🥁 Drums & Percussion

| Example | Description |
|---------|-------------|
| `drum_grid.rs` | Grid-based drum programming (kick, snare, hi-hat) |
| `drum_sounds.rs` | All available drum types and their sounds |
| `drum_808.rs` | Classic TR-808 drum patterns |
| `drum_variations_demo.rs` | Velocity, tuning, and timing variations |
| `expanded_percussion_demo.rs` | Extended percussion instruments |
| `midi_percussion_demo.rs` | MIDI percussion (General MIDI channel 10) |
| `808_909_complete_demo.rs` | Complete drum machine instrument suite |

---

## 🎛️ Effects & Processing

| Example | Description |
|---------|-------------|
| `effects_showcase.rs` | **All 16 effects demonstrated with audio** |
| `effect_presets_demo.rs` | Preset API: `Delay::quarter_note()`, `Reverb::hall()` |
| `parametric_eq_demo.rs` | Multi-band parametric EQ for precise frequency control |
| `multiband_compression_demo.rs` | **Frequency-specific dynamics control** |
| `sidechaining.rs` | Sidechain compression for pumping/ducking effects |
| `spatial_audio_demo.rs` | 3D positional audio and distance attenuation |
| `doppler_effect_demo.rs` | **Doppler pitch shifting** for moving sound sources |
| `stereo_panning.rs` | Stereo field positioning and width control |
| `automation_demo.rs` | Automate volume, pan, filter, effect parameters |

---

## 🎼 Music Theory

| Example | Description |
|---------|-------------|
| `theory_demo.rs` | Scales, chords, progressions, voice leading |
| `key_signatures.rs` | Working with different musical keys |
| `world_scales_demo.rs` | Global scales: pentatonic, blues, modes, ethnic |
| `microtonal_demo.rs` | Beyond 12-tone equal temperament |
| `progressions_demo.rs` | Common chord progressions (I-IV-V, etc.) |
| `voicing_and_voice_leading.rs` | Chord inversions and smooth voice transitions |
| `octaves_and_harmonize.rs` | Octave doubling and harmonization |

---

## 🎵 Composition Techniques

| Example | Description |
|---------|-------------|
| `arrangement_demo.rs` | Structure full songs with intro/verse/chorus |
| `section_workflow.rs` | Compose and arrange song sections |
| `templates_demo.rs` | Reusable composition templates |
| `classical_techniques.rs` | Canon, fugue, counterpoint, retrograde |
| `expressive_techniques.rs` | Dynamics, articulation, phrasing |
| `pattern_transformations.rs` | **21 pattern tools: shift, humanize, rotate, retrograde, and more** |
| `pattern_physics.rs` | **Physics-inspired transformations: magnetize, gravity, ripple** |
| `transform_namespace.rs` | **Closure-based `.transform()` API** |
| `sieve_demo.rs` | **Frequency-based filtering transforms** |
| `generator_namespace.rs` | **Closure-based `.generator()` API** |
| `orbit_demo.rs` | **Orbit generator** - sinusoidal pitch patterns |
| `bounce_demo.rs` | **Bounce generator** - physics-based bouncing |
| `sprinkle_demo.rs` | **Sprinkle generator** - random frequencies |
| `namespace_api.rs` | **Complete namespace API guide** |
| `pattern_modifiers.rs` | Transform patterns: reverse, invert, transpose |
| `pattern_repeat.rs` | Loop and repeat musical patterns |
| `reverse_patterns.rs` | Retrograde melodies and rhythms |

---

## ⏱️ Timing & Rhythm

| Example | Description |
|---------|-------------|
| `rhythm_notation.rs` | Note durations: whole, half, quarter, eighth |
| `time_signatures.rs` | 4/4, 3/4, 5/4, 7/8, and complex meters |
| `tempo_changes.rs` | Dynamic tempo modulation and automation |
| `swing_timing.rs` | Swing/shuffle rhythms (triplet feel) |
| `musical_time.rs` | Bars, beats, and timing helpers |
| `every_n_demo.rs` | Play events every N steps/bars |
| `markers_demo.rs` | Timeline markers for navigation |
| `tweening_demo.rs` | **Smooth parameter transitions** |

---

## 🎹 Performance Techniques

| Example | Description |
|---------|-------------|
| `arpeggios.rs` | Ascending/descending/alternating arpeggios |
| `ornaments_showcase.rs` | Trills, mordents, turns, grace notes |
| `interpolated.rs` | Smooth pitch glides between notes |
| `portamento.rs` | Scale-aware gliding between pitches |
| `pitch_bend.rs` | Continuous pitch bending automation |
| `pedal_tones.rs` | Sustained bass notes with changing harmony |

---

## 💿 MIDI & Import/Export

### MIDI

| Example | Description |
|---------|-------------|
| `midi_export.rs` | Export compositions as Standard MIDI Files |
| `midi_import.rs` | Import MIDI files and play them |
| `midi_to_flac.rs` | Convert MIDI files to high-quality FLAC audio |
| `midi_percussion_demo.rs` | MIDI drum mapping (General MIDI standard) |

### Audio Export

| Example | Description |
|---------|-------------|
| `wav_export_demo.rs` | Export compositions as WAV files |
| `flac_export.rs` | Export as lossless FLAC (50-60% smaller than WAV) |
| `multiformat_import.rs` | Import MP3, OGG, FLAC, WAV, AAC files |

---

## 🔪 Sample Manipulation

**Advanced audio sample slicing, processing, and playback techniques.**

| Example | Description |
|---------|-------------|
| `sample_playback_demo.rs` | **Simple sample playback** with automatic caching |
| `sample_slicing.rs` | All slicing techniques (equal, time, transient, beat-based) |
| `slice_playback.rs` | Direct sample and slice playback in compositions |
| `time_pitch_manipulation.rs` | **Time stretch and pitch shift** samples independently |
| `streaming_demo.rs` | **Memory-efficient streaming** for long audio files |

**Key Features:**
- **Simple API:** `engine.play_sample("file.wav")?` - automatic caching, SIMD-accelerated
- **Slicing:** Equal, time-based, transient detection, beat-based
- **Manipulation:** Time stretch, pitch shift (independent control)
- **Streaming:** Long files without loading entire audio into RAM
- **Zero-Copy:** Efficient Arc-based referencing avoids duplication

---

## 🚀 Advanced & Complete Demos

| Example | Description |
|---------|-------------|
| `master_feature_showcase.rs` | **Comprehensive demo of major features (19K)** |
| `sequences_showcase.rs` | **Complete tour of 50+ algorithmic sequences (19K)** |
| `pattern_transformations.rs` | **All 21 pattern transformation tools (13K)** |
| `effects_showcase.rs` | **All 16 audio effects with examples (12K)** |
| `claude_composition.rs` | Full composition example (Claude-generated) |
| `claude_composition_algorithms.rs` | Algorithmic composition example |

---

## 📊 Running Examples

Run any example with:

```bash
cargo run --example <name>

# Examples:
cargo run --example sample_playback_demo    # Start here!
cargo run --example synthesis_demo
cargo run --example effects_showcase
cargo run --example sequences_showcase
```

**For complex synthesis examples, use `--release` to avoid audio underruns:**

```bash
cargo run --release --example master_feature_showcase
```

Most examples will:
1. Play audio through your default audio output
2. Print information about what's happening
3. Exit when playback completes

---

## 🎓 Learning Path

### Beginner Track (Game Developers)
1. `sample_playback_demo.rs` - **Dead-simple game audio** (2 lines!)
2. `concurrent_playback_demo.rs` - Multiple sounds at once
3. `spatial_audio_demo.rs` - 3D positioned audio

### Beginner Track (Music Composers)
1. `notes_and_chords.rs` - Understand basic note playback
2. `waveforms.rs` - Hear different waveform types
3. `synthesis_demo.rs` - Learn synthesis basics
4. `drum_grid.rs` - Create simple drum patterns
5. `instrument_showcase.rs` - Explore 150+ instruments

### Intermediate Track
6. `theory_demo.rs` - Music theory concepts
7. `arrangement_demo.rs` - Structure complete songs
8. `automation_demo.rs` - Dynamic parameter changes
9. `effects_showcase.rs` - All audio effects
10. `euclidean_rhythms.rs` - Algorithmic rhythm patterns

### Advanced Track
11. `sequences_showcase.rs` - Master algorithmic composition
12. `pattern_transformations.rs` - 21 pattern manipulation tools
13. `sidechaining.rs` - Professional mixing techniques
14. `multiband_compression_demo.rs` - Frequency-specific dynamics
15. `master_feature_showcase.rs` - Everything together

---

## 💡 Quick Reference

### By Use Case

**Game Audio (Start Here!):**
- Simple: `sample_playback_demo.rs` - Just 2 lines of code!
- Concurrent: `concurrent_playback_demo.rs` - Many sounds at once
- 3D Audio: `spatial_audio_demo.rs`, `doppler_effect_demo.rs`
- Integration: See [book/game-audio/bevy-integration.md](../book/src/game-audio/bevy-integration.md)

**Generative Music:**
- Sequences: `sequences_showcase.rs` (50+ algorithms)
- Patterns: `pattern_transformations.rs` (21 tools)
- Chaos: `chaotic_sequences.rs` (Lorenz, logistic map)
- Physics: `pattern_physics.rs` (magnetize, gravity, ripple)

**Electronic Music:**
- Drums: `drum_808.rs`, `808_909_complete_demo.rs`
- Mixing: `sidechaining.rs`, `multiband_compression_demo.rs`
- Synthesis: `wavetable_synthesis.rs`, `additive_synthesis_demo.rs`
- Samples: `sample_slicing.rs`, `time_pitch_manipulation.rs`

**Classical/Acoustic:**
- Theory: `classical_techniques.rs`, `voicing_and_voice_leading.rs`
- Expression: `ornaments_showcase.rs`, `expressive_techniques.rs`
- Instruments: `instrument_showcase.rs` (150+ presets)

**Learning:**
- Start: `sample_playback_demo.rs` (simplest), `synthesis_demo.rs`
- Complete: `master_feature_showcase.rs` (everything)
- Reference: `effects_showcase.rs`, `sequences_showcase.rs`

### By Feature

**Synthesis:** `synthesis_demo.rs`, `additive_synthesis_demo.rs`, `wavetable_synthesis.rs`
**Effects:** `effects_showcase.rs`, `effect_presets_demo.rs`, `sidechaining.rs`, `spatial_audio_demo.rs`
**Drums:** `drum_grid.rs`, `drum_808.rs`, `euclidean_rhythms.rs`, `808_909_complete_demo.rs`
**Sequences:** `sequences_showcase.rs`, `mathematical_sequences.rs`, `chaotic_sequences.rs`
**Theory:** `theory_demo.rs`, `world_scales_demo.rs`, `progressions_demo.rs`
**Samples:** `sample_playback_demo.rs`, `sample_slicing.rs`, `time_pitch_manipulation.rs`
**MIDI:** `midi_export.rs`, `midi_import.rs`, `midi_to_flac.rs`
**Export:** `wav_export_demo.rs`, `flac_export.rs`

---

## 🤝 Contributing Examples

When adding new examples:

1. **Name clearly:** Use descriptive snake_case names
2. **Add comments:** Explain what's happening and why
3. **Print output:** Help users understand what they're hearing
4. **Keep focused:** One concept per example (usually)
5. **Update this README:** Add your example to the appropriate category
6. **Test with `--release`:** Complex synthesis should run smoothly

---

**Happy composing! 🎵**

For more information, see the [Tunes Documentation Book](../book).

**New to Tunes?** Start with `sample_playback_demo.rs` (game audio) or `synthesis_demo.rs` (music composition).
