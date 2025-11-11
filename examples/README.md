# Tunes Examples

This directory contains **80 example programs** demonstrating all features of the Tunes audio library. Examples are organized from beginner to advanced topics.

---

## 🎯 Getting Started

**Start here if you're new to Tunes!**

| Example | Description |
|---------|-------------|
| `notes_and_chords.rs` | Play individual notes and chord arrays |
| `waveforms.rs` | Explore sine, square, sawtooth, and triangle waves |
| `envelopes.rs` | ADSR envelope shaping for dynamic sounds |
| `filters.rs` | Low-pass, high-pass, and band-pass filtering |
| `synthesis_demo.rs` | Introduction to basic synthesis concepts |
| `instrument_showcase.rs` | Tour of all built-in instrument presets |

---

## 🎹 Synthesis & Sound Design

| Example | Description |
|---------|-------------|
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

---

## 🎸 Instruments & Presets

| Example | Description |
|---------|-------------|
| `instrument_showcase.rs` | All instrument presets with audio examples |
| `noise_generator_showcase.rs` | Noise-based instruments and textures |
| `808_909_complete_demo.rs` | Complete drum machine instrument suite |

---

## 🎛️ Effects & Processing

| Example | Description |
|---------|-------------|
| `effects_showcase.rs` | All 16 effects demonstrated with audio |
| `parametric_eq_demo.rs` | Multi-band parametric EQ for precise frequency control |
| `sidechaining.rs` | Sidechain compression for pumping/ducking effects |
| `spatial_audio_demo.rs` | 3D positional audio and Doppler effect |
| `stereo_panning.rs` | Stereo field positioning and width control |

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
| `pattern_transformations.rs` | **21 pattern tools: shift, humanize, rotate, retrograde, reverse, shuffle, thin, stack, mutate, stretch, compress, quantize, palindrome, stutter, stutter_every, granularize, magnetize, gravity, ripple, invert, invert_constrained** |
| `pattern_physics.rs` | **Physics-inspired transformations: magnetize, gravity, ripple** |
| `transform_namespace.rs` | **Closure-based `.transform()` API for organized pattern transformations** |
| `generator_namespace.rs` | **Closure-based `.generator()` API for organized note generation** |
| `namespace_api.rs` | **Complete namespace API guide - `.generator()`, `.transform()`, and `.effects()` closure patterns** |
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

## 🤖 Automation & Modulation

| Example | Description |
|---------|-------------|
| `automation_demo.rs` | Automate volume, pan, filter, effect parameters |
| `lfo_modulation.rs` | LFO-driven parameter modulation (vibrato, tremolo) |
| `pitch_bend.rs` | Pitch automation and bending |

---

## 💿 MIDI Import/Export

| Example | Description |
|---------|-------------|
| `midi_export.rs` | Export compositions as Standard MIDI Files |
| `midi_import.rs` | Import MIDI files and play them |
| `midi_to_flac.rs` | Convert MIDI files to high-quality FLAC audio |
| `midi_percussion_demo.rs` | MIDI drum mapping (General MIDI standard) |
| `test_roundtrip.rs` | Test MIDI export/import fidelity |

---

## 📁 Audio Export

| Example | Description |
|---------|-------------|
| `wav_export_demo.rs` | Export compositions as WAV files |
| `flac_export.rs` | Export as lossless FLAC (smaller file size) |
| `midi_to_flac.rs` | MIDI → Audio conversion pipeline |

---

## 🎮 Game Audio & Real-Time

| Example | Description |
|---------|-------------|
| `concurrent_playback_demo.rs` | Multiple simultaneous audio streams |
| `spatial_audio_demo.rs` | 3D positional audio for game worlds |
| `sample_playback_demo.rs` | Trigger audio samples dynamically |

---

## 🚀 Advanced & Complete Demos

| Example | Description |
|---------|-------------|
| `master_feature_showcase.rs` | **Comprehensive demo of major features** |
| `claude_composition.rs` | Full composition example (Claude-generated) |
| `claude_composition_algorithms.rs` | Algorithmic composition example |
| `stress_test.rs` | Performance test with many concurrent tracks |
| `volume_test.rs` | Audio level testing and calibration |

---

## 📊 Running Examples

Run any example with:

```bash
cargo run --example <name>

# Examples:
cargo run --example sequences_showcase
cargo run --example euclidean_rhythms
cargo run --example effects_showcase
```

Most examples will:
1. Play audio through your default audio output
2. Print information about what's happening
3. Exit when playback completes

---

## 🎓 Learning Path

### Beginner Track
1. `notes_and_chords.rs` - Understand basic note playback
2. `waveforms.rs` - Hear different waveform types
3. `drum_grid.rs` - Create simple drum patterns
4. `instrument_showcase.rs` - Explore available sounds
5. `effects_showcase.rs` - Learn about audio effects

### Intermediate Track
6. `theory_demo.rs` - Music theory concepts
7. `arrangement_demo.rs` - Structure complete songs
8. `automation_demo.rs` - Dynamic parameter changes
9. `euclidean_rhythms.rs` - Algorithmic rhythm patterns
10. `midi_export.rs` - Export to MIDI for DAWs

### Advanced Track
11. `sequences_showcase.rs` - Master algorithmic composition
12. `spatial_audio_demo.rs` - 3D audio positioning
13. `sidechaining.rs` - Professional mixing techniques
14. `master_feature_showcase.rs` - Everything together
15. `stress_test.rs` - Performance optimization

---

## 💡 Quick Reference

### By Feature

**Drums:** `drum_grid.rs`, `drum_808.rs`, `euclidean_rhythms.rs`
**Synthesis:** `synthesis_demo.rs`, `additive_synthesis_demo.rs`, `wavetable_synthesis.rs`
**Sequences:** `sequences_showcase.rs`, `mathematical_sequences.rs`, `chaotic_sequences.rs`
**Effects:** `effects_showcase.rs`, `sidechaining.rs`, `spatial_audio_demo.rs`
**Theory:** `theory_demo.rs`, `world_scales_demo.rs`, `progressions_demo.rs`
**MIDI:** `midi_export.rs`, `midi_import.rs`, `midi_to_flac.rs`
**Export:** `wav_export_demo.rs`, `flac_export.rs`

### By Use Case

**Game Audio:** `concurrent_playback_demo.rs`, `spatial_audio_demo.rs`, `sample_playback_demo.rs`
**Generative Music:** `sequences_showcase.rs`, `algorithmic_patterns.rs`, `chaotic_sequences.rs`, `pattern_transformations.rs`, `pattern_physics.rs`
**Electronic Music:** `drum_808.rs`, `sidechaining.rs`, `wavetable_synthesis.rs`
**Classical/Acoustic:** `classical_techniques.rs`, `voicing_and_voice_leading.rs`, `ornaments_showcase.rs`
**Learning:** `master_feature_showcase.rs`, `instrument_showcase.rs`, `effects_showcase.rs`

---

## 🤝 Contributing Examples

When adding new examples:

1. **Name clearly:** Use descriptive snake_case names
2. **Add comments:** Explain what's happening and why
3. **Print output:** Help users understand what they're hearing
4. **Keep focused:** One concept per example (usually)
5. **Update this README:** Add your example to the appropriate category

---

**Happy composing! 🎵**

For more information, see the [Tunes Documentation](../book).
