# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **AudioEngine now silent by default** - No automatic terminal output on initialization
  - `AudioEngine::new()` and `AudioEngine::with_buffer_size()` no longer print to stdout
  - New `AudioEngine::print_info()` method for opt-in verbose initialization output
  - `.print_info()` displays: device name, sample rate, buffer size, latency, channels, **SIMD detection** (AVX2/SSE/NEON), and concurrent mixing status
  - Better library citizen behavior - respects downstream users' output preferences
  - Addresses user feedback about unwanted terminal output in production use cases
  - Example: `examples/print_info_demo.rs` demonstrates the opt-in verbose output

### Added
- **SIMD Acceleration** - 2-8x performance boost for DSP operations using portable SIMD (stable Rust):
  - Runtime CPU detection with lazy_static (detects once at startup, zero overhead after)
  - Hybrid dispatch: match statement (~3 CPU cycles) → generic monomorphized code (fully optimized)
  - Automatic fallback: AVX2 (8-wide) → SSE (4-wide) → Scalar
  - **Effects with SIMD:**
    - **Wavetable oscillators:** `Wavetable::fill_buffer_simd()` - ~1.5x faster (measured)
    - **Distortion:** Soft clipping with SIMD - 2-4x faster expected
    - **Saturation:** Soft/hard waveshaping with SIMD - 2-4x faster expected
    - **Tremolo:** LFO-based amplitude modulation with SIMD - 2-4x faster expected
    - **RingModulator:** Carrier-based modulation with SIMD - 2-4x faster expected
  - **Sample Playback with SIMD:** `Sample::fill_buffer_simd_mono()` - Block-based processing for concurrent samples
    - **Industry-first:** Likely the only Rust audio library with SIMD-accelerated sample playback
    - Processes 4-8 samples simultaneously with vectorized interpolation
    - **30-45x realtime** performance measured with 50 concurrent samples
    - Perfect for bullet-hell games, rhythm games, and sample-heavy applications
    - Mixer automatically uses SIMD for all sample events (zero API changes)
    - Example: `examples/simd_sample_playback_benchmark.rs` demonstrates performance
  - Infrastructure proven and ready for more effects
  - Uses `wide` crate (same as FunDSP, battle-tested, stable Rust)
  - Zero API changes - speedups happen transparently for all users
  - Pattern proven: write code once, compiler optimizes for 4-lane and 8-lane automatically
  - Example: `examples/simd_wavetable_demo.rs` demonstrates detection and benchmarking
- **Multiband Compression** - Professional mastering tool for frequency-specific dynamics control:
  - `CompressorBand::new(low_freq, high_freq, compressor)` - Define frequency bands with independent compression
  - `Compressor::with_multiband(band)` - Add single frequency band for multiband compression
  - `Compressor::with_multibands(vec![bands])` - Add multiple frequency bands at once
  - `Compressor::multiband_3way(low_mid_cross, mid_high_cross)` - Convenience constructor for 3-band split
  - `.with_band_low()`, `.with_band_mid()`, `.with_band_high()` - Adjust individual band settings
  - Built-in Butterworth bandpass filters for clean frequency separation
  - Reuses existing Compressor logic - elegant composition of functionality
  - Perfect for mastering: control bass independently from mids/highs
  - Example: `Compressor::multiband_3way(200.0, 2000.0).with_band_low(0.3, 4.0).with_band_mid(0.5, 2.5).with_band_high(0.6, 2.0)`
  - Added comprehensive example: `examples/multiband_compression_demo.rs`
  - Feature not available in Kira, Rodio, or SoLoud
- **Fire-and-Forget Sample Playback with Automatic Caching** - Simplest game audio API in Rust:
  - `AudioEngine::play_sample(path)` - Non-blocking, concurrent sample playback with smart caching
  - **Automatic caching:** First call loads from disk, subsequent calls use cached Arc (instant)
  - No manual cache management needed - just call `play_sample()` and it's fast!
  - `AudioEngine::preload_sample(path)` - Optional: pre-load samples during initialization
  - `AudioEngine::clear_sample_cache()` - Optional: clear cache between levels
  - `AudioEngine::remove_cached_sample(path)` - Optional: remove specific cached sample
  - Returns `SoundId` for optional volume/pan control
  - Perfect for rapid game development - simpler than Kira, Rodio, or odd-io
  - Example: `engine.play_sample("explosion.wav")?;` - That's it!
  - Spam-safe: Repeated sounds are instant after first load (Arc clone from cache)
  - Updated `examples/sample_playback_demo.rs` with demonstration
  - Added comprehensive documentation in book's samples section with automatic caching details
- **Sample Slicing System** - Comprehensive audio sample slicing with multiple techniques:
  - `Sample::slice_equal(n)` - Divide sample into N equal parts
  - `Sample::slice_at_times(times)` - Slice at specific time points
  - `Sample::slice_at_frames(frames)` - Slice at frame indices
  - `Sample::slice_at_beats(bpm, beats_per_slice)` - Rhythmic slicing based on tempo
  - `Sample::detect_transients(threshold, min_gap_ms)` - Energy-based onset detection
  - `Sample::slice_by_transients(threshold, min_gap_ms)` - Auto-slice at detected hits
- **SampleSlice** - Lightweight slice reference type that avoids copying audio data
  - Efficient Arc-based referencing to parent sample
  - Methods: `sample_at()`, `to_sample()`, `start_time()`, `end_time()`, `num_frames()`
- **Direct Sample & Slice Playback API** - Play samples and slices without caching workflow:
  - `TrackBuilder::play_sample(sample, playback_rate)` - Play `Sample` directly in composition
  - `TrackBuilder::play_slice(slice, playback_rate)` - Play `SampleSlice` directly in composition
  - Enables dynamic, generative sample playback without pre-loading
- **Time Stretching & Pitch Shifting** - WSOLA-based audio manipulation for game audio variation:
  - `Sample::time_stretch(factor)` - Change duration without affecting pitch
    - Perfect for slow-motion effects, time dilation, dialog speed adjustment
    - Uses WSOLA (Waveform Similarity Overlap-Add) algorithm
    - Works best with stretch factors between 0.5x and 2.0x
    - Maintains pitch characteristics while changing duration
  - `Sample::pitch_shift(semitones)` - Change pitch without affecting duration
    - Ideal for enemy size variations, musical transposition, audio variety
    - Accepts semitone values (12 = octave up, -12 = octave down)
    - Combines resampling with time-stretching for duration preservation
    - Reduces repetitive audio in games by creating pitch variations
  - Implementation uses Hann windowing and cross-correlation for smooth grain matching
  - No external FFT dependencies required
- **Multi-Format Audio Import** - Industry-standard format support via symphonia:
  - `Sample::from_file(path)` - Universal audio loader with automatic format detection
  - Supported formats: MP3, OGG Vorbis, FLAC, WAV (PCM/Float), AAC, M4A
  - Automatic format detection from file extension and content analysis
  - Handles all sample formats (8/16/24/32-bit int, 32/64-bit float) with conversion to f32
  - Planar-to-interleaved audio conversion for efficient processing
  - Updated `Composition::load_sample()` to support all formats automatically
  - Simplified codebase: Removed `Sample::from_wav()` in favor of unified `from_file()` API
  - Clean separation: hound for WAV export (writing), symphonia for all import (reading)
- **Runtime Parameter Tweening** - Smooth interpolation for dynamic sound control:
  - `AudioEngine::tween_pan(id, target_pan, duration)` - Smoothly pan sounds left/right
  - `AudioEngine::tween_playback_rate(id, target_rate, duration)` - Smoothly change pitch/speed
  - Extends existing volume tweening (`fade_in`, `fade_out`) with pan and playback rate
- **Streaming Audio** - Memory-efficient playback for long audio files:
  - `AudioEngine::stream_file(path)` - Stream audio from disk without loading entire file into RAM
  - `AudioEngine::stream_file_looping(path)` - Stream audio in continuous loop
  - Full playback controls: `stop_stream()`, `pause_stream()`, `resume_stream()`
  - Real-time parameter control: `set_stream_volume()`, `set_stream_pan()`
  - Perfect for background music, ambient sounds, and voice-over narration (3-10+ minutes)
  - Supports all audio formats (MP3, OGG, FLAC, WAV, AAC) via symphonia decoder
  - Background decoding thread with lock-free ring buffer for smooth playback
  - Multiple concurrent streams supported with independent control
  - Automatic cleanup on stop (decoder thread terminates gracefully)
- **Doppler Effect** - Realistic pitch shifting for moving sound sources:
  - `AudioEngine::set_sound_velocity(id, vx, vy, vz)` - Set velocity for doppler effect on sounds
  - `AudioEngine::set_listener_velocity(vx, vy, vz)` - Set listener velocity for relative doppler
  - Physics-based pitch shift: higher pitch when approaching, lower when receding
  - Configurable doppler factor (0.0 = disabled, 1.0 = realistic, 2.0 = exaggerated)
  - Speed of sound = 343 m/s (realistic physics)
  - Perfect for racing games (car flyby), flight sims (aircraft), projectiles
  - **BUG FIX**: Doppler pitch was calculated but not applied to audio timeline - fixed by advancing `elapsed_time` with doppler-adjusted playback rate
- New examples:
  - `sample_slicing.rs` - Comprehensive demonstration of all slicing techniques
  - `slice_playback.rs` - Direct sample/slice playback in compositions
  - `time_pitch_manipulation.rs` - Time stretching and pitch shifting for game audio variations
  - `multiformat_import.rs` - Multi-format audio loading (MP3, OGG, FLAC, WAV, AAC)
  - `tweening_demo.rs` - Runtime parameter tweening for volume, pan, and playback rate
  - `streaming_demo.rs` - Memory-efficient audio streaming for long files
  - `doppler_effect_demo.rs` - Realistic doppler effect for moving sound sources (car passing, helicopter flyby, racing)
  - `new_transforms_demo.rs` - Demonstrations of new unified transforms (range_dilation, shape_contour, echo)
- Exported `Sample` and `SampleSlice` in prelude for convenience

### Performance

#### Massive Audio Engine Optimization - Production-Grade Real-Time Performance
- **512x reduction in function call overhead** - Stress test (2,500 events with heavy effects) now runs without underruns
- **Block-based audio processing** - Complete refactor from sample-by-sample to block processing throughout the entire audio stack
- **Allocation-free audio callback** - Pre-allocated buffers eliminate all allocations in real-time audio thread
- **Integer ID system** - Internal use of `BusId` and `TrackId` (integer IDs) instead of string lookups in hot path
- **Optimized binary search** - Event lookup reduced from 25,600 calls to 50 calls per audio callback (512-sample blocks, 50 tracks)

**Technical Details:**

**Block Processing (New `Mixer::process_block()` and `process_track_block()`):**
- **Before:** `Mixer::process_block()` called `sample_at()` 512 times per block (once per sample)
- **After:** True block processing - generate entire track buffers at once, then apply effects to blocks
- **Impact:** Binary search done once per track per block instead of 512 times
- **Example:** 50 tracks, 512-sample block → 25,600 function calls reduced to 50 calls
- **Methods:**
  - `process_track_block(track, buffer, sample_rate, start_time, start_sample_count)` - Mono track synthesis with block-based effects
  - `Mixer::process_block()` - Processes buses and master chain using block operations
  - `EffectChain::process_mono_block()` / `process_stereo_block()` - Block-based effect processing (already existed, now fully utilized)

**Allocation-Free Audio Callback (Engine refactor):**
- **Before:** Audio callback allocated `vec![0.0; num_frames * 2]` and `Vec::new()` on every callback (every ~10ms)
- **After:** Pre-allocated buffers in `AudioCallbackState` struct, reused across all callbacks
- **Changes:**
  - New `AudioCallbackState` struct with `temp_buffer: Vec<f32>`, `finished_sounds: Vec<SoundId>`, `active_sounds: HashMap`
  - Audio callback destructures state to get separate mutable references (satisfies Rust borrow checker)
  - `mix_sounds()` signature changed to accept pre-allocated buffers instead of allocating internally
  - Buffers sized once on first use, then reused forever
- **Impact:** Eliminated primary cause of audio dropouts (allocation latency spikes)

**Integer ID System (Zero-cost string lookups):**
- **Before:** Buses and tracks identified by `String`, requiring `HashMap<String, Bus>` lookups during mixing
- **After:** Internal integer IDs (`BusId = u32`, `TrackId = u32`) with direct indexing
- **Implementation:**
  - `Mixer::buses: Vec<Option<Bus>>` - Sparse vector indexed by `BusId` for O(1) access
  - `Mixer::bus_order: Vec<BusId>` - Processing order using IDs
  - `Mixer::bus_name_to_id: HashMap<String, BusId>` - String lookups only for user-facing API
  - `BusIdGenerator` and `TrackIdGenerator` - Monotonic ID allocation with wraparound
  - Bus and Track structs store their own IDs for fast identification
- **API Impact:** User-facing API unchanged (still uses strings), internal mixing path uses integers

**Cache-Friendly Processing:**
- Effects already process entire buffers sequentially (good cache locality)
- Block processing keeps working sets small and cache-warm
- Pre-allocated buffers reduce memory allocator pressure

**Benchmark Results:**
- **Stress test:** 50 tracks, 50 events each (2,500 total), reverb + delay + chorus + filter on every track
- **Before optimization:** Heavy ALSA underruns (audio dropouts)
- **After optimization:** Zero underruns, smooth playback at 44.1kHz
- **Real-world headroom:** ~10x capacity for typical game audio / music production (10-20 simultaneous sounds)

**Files Modified:**
- `src/engine.rs` - AudioCallbackState, allocation-free mixing
- `src/track/mixer.rs` - Block-based processing, integer ID system
- `src/track/bus.rs` - BusId field
- `src/track/mod.rs` - TrackId field, ID generators
- `examples/stress_test.rs` - Performance validation (NEW)

**Backward Compatibility:**
- All public APIs unchanged
- All 972 unit tests + 338 doc tests passing
- User code requires zero modifications

### Added

#### Pattern Transformation Methods for Live Coding
- **Twenty-seven pattern manipulation methods** - Powerful tools for generative music and live coding workflows
- **NEW: `.transform()` namespace API** - Closure-based scoped access to all transformations
  - Example: `.transform(|t| t.shift(7).humanize(0.01, 0.05).rotate(1))`
  - Provides clean, organized namespace - only see transforms when you need them
  - Reduces autocomplete pollution - 27 methods are scoped within `.transform()`
  - Fully backward compatible - old direct-call syntax still works: `.shift(7).humanize(0.01, 0.05)`
  - Rust-idiomatic closure-based API with clear visual boundaries
  - Can chain multiple `.transform()` blocks for organized, readable code
  - See `examples/transform_namespace.rs` and `examples/namespace_api.rs` for usage
- **NEW: `.effects()` namespace API** - Closure-based scoped access to all 17 audio effects
  - Example: `.effects(|e| e.filter(...).reverb(...).delay(...))`
  - Organizes all effects (filter, delay, reverb, distortion, bitcrusher, compressor, chorus, eq, saturation, phaser, flanger, ring_mod, tremolo, autopan, gate, limiter, modulate)
  - Same closure pattern as `.transform()` for consistent API design
  - Fully backward compatible - effects can still be called directly
- **NEW: `.generator()` namespace API** - Closure-based scoped access to all 40+ note-generating methods
  - Example: `.generator(|g| g.chord(...).arpeggiate(...).trill(...))`
  - Organizes all generators: chords (6), scales (4), arpeggios (4), classical patterns (6), ornaments (8), tuplets (5), musical patterns (3), portamento (1), time-based (5)
  - Same closure pattern as `.transform()` and `.effects()` for consistent API design
  - Fully backward compatible - generators can still be called directly
  - See `examples/generator_namespace.rs` for complete usage guide
- **NEW: `.orbit()` generator** - Create sinusoidal pitch patterns around a center frequency
  - Example: `.orbit(C4, 7.0, 16, 0.125, 1.5, true)` - oscillate ±7 semitones around C4, 16 steps per rotation, 1.5 complete orbits
  - Parameters: center pitch, radius (semitones), steps per rotation, step duration, rotations (can be fractional), direction (clockwise/counter-clockwise)
  - **Rotations parameter** allows multiple complete orbits (2.0 = two cycles) or fractional (0.5 = half orbit for ascending/descending phrases)
  - Perfect for melodic contours, vibrato effects, ambient textures, and generative patterns
  - Combines beautifully with `.magnetize()` to snap to scales or `.gravity()` for tonal attraction
  - See `examples/orbit_demo.rs` for detailed demonstrations including multiple and fractional rotations
- **NEW: `.bounce()` generator** - Physics-based bouncing ball effect with damping
  - Example: `.bounce(440.0, 220.0, 0.5, 3, 8, 0.0625)` - drop from A4 to A3, bounce 3 times at 50% damping, 8 steps per segment
  - Parameters: start frequency, floor frequency, damping ratio (0.0-1.0), number of bounces, steps per segment, step duration
  - Simulates realistic bouncing with exponential decay - each bounce is `ratio` times the height of the previous bounce
  - **Zero bounces** creates a smooth pitch drop (portamento effect)
  - **Low ratio** (0.2-0.4) = tight, controlled bouncing that settles quickly
  - **High ratio** (0.7-0.9) = energetic bouncing that takes longer to settle
  - Perfect for percussive melodies, glitch effects, generative patterns, and sound design
  - Combine with `.magnetize()` to snap bounces to scale degrees
  - See `examples/bounce_demo.rs` for comprehensive demonstrations including layered bounces and reverse effects
- **NEW: `.scatter()` generator** - Generate random notes across a frequency range
  - Example: `.scatter(200.0, 800.0, 32, 0.0625)` - 32 random notes uniformly distributed between 200-800Hz
  - Parameters: minimum frequency, maximum frequency, note count, duration per note
  - Each note has a random pitch uniformly distributed within the range
  - Perfect for experimental music, glitch textures, generative soundscapes, and aleatoric composition
  - Combine with `.magnetize()` to constrain random notes to a scale
  - Combine with `.humanize()` for organic timing variations
  - Great for creating unpredictable melodic patterns and textural elements
- **NEW: `.stream()` generator** - Generate repeated notes at a single frequency
  - Example: `.stream(440.0, 16, 0.125)` - 16 repeated A4 notes, each 0.125 seconds long
  - Parameters: frequency, note count, duration per note
  - Creates uniform sequences of identical notes - perfect for drones and ostinatos
  - Ideal as a base pattern for transforms (`.shift()`, `.mutate()`, `.humanize()`, etc.)
  - Simple, efficient generator for repetitive patterns
  - Great starting point for generative manipulations
- **NEW: `.random_notes()` generator** - Randomly pick notes from a provided set
  - Example: `.random_notes(&[C4, E4, G4, C5], 16, 0.125)` - 16 random notes picked from C major triad
  - Parameters: array of frequencies, note count, duration per note
  - Each note randomly selected with equal probability from the provided array
  - Perfect for generative music constrained to a scale, chord, or custom note set
  - Different from `.scatter()` which uses continuous uniform distribution - this picks from discrete notes
- **NEW: `.sprinkle()` generator** - Generate completely random f32 frequencies within a range
  - Example: `.sprinkle(220.0, 440.0, 8, 0.125)` - 8 random frequencies between A3 and A4 (no snapping)
  - Parameters: min frequency, max frequency, note count, duration per note
  - Produces truly continuous pitch values with no quantization or snapping
  - Perfect for experimental, microtonal, or ambient textures
  - Different from discrete note selection - generates any f32 value within the range
  - Great for creating unpredictable, organic-sounding patterns
  - Great for algorithmic composition, random melodies within harmonic constraints
  - Combine with scales: `.random_notes(&C4_MAJOR_SCALE, 32, 0.0625)` for random scale-based melodies
- **Added 10 transform methods to namespace:**
  - `.reverse()` - Reverse timing of all events in pattern
  - `.invert(axis_freq)` - Invert pitches around an axis frequency (pitch mirroring)
  - `.invert_constrained(axis_freq, min, max)` - Pitch inversion with range constraints
  - `.sieve_inclusive(min_freq, max_freq)` - **NEW: Filter to keep only notes within frequency range**
  - `.sieve_exclusive(min_freq, max_freq)` - **NEW: Filter to remove notes within frequency range**
  - `.group(duration)` - **NEW: Collapse sequential notes into a single chord**
  - `.duplicate()` - **NEW: Duplicate pattern for layered transforms**
  - `.repeat(times)` - Repeat the pattern N times
  - `.harmonize(notes, semitones, duration)` - Play harmonized notes with interval above/below
  - `.every_n(n, drum)` - Play a drum every Nth event in the pattern
- **NEW: `.sieve_inclusive(min_freq, max_freq)` transform** - Frequency-based filtering to keep notes in range
  - Example: `.sieve_inclusive(150.0, 300.0)` keeps only notes between 150-300 Hz (bass range)
  - Removes all notes whose frequencies fall outside [min_freq, max_freq]
  - Perfect for isolating specific frequency bands (bass, midrange, treble)
  - Use cases: extract bass line, isolate melody, frequency-based arrangement
  - Chain with other sieves for complex frequency sculpting
  - Works with namespace: `.transform(|t| t.sieve_inclusive(20.0, 200.0))`
- **NEW: `.sieve_exclusive(min_freq, max_freq)` transform** - Frequency-based filtering to remove notes in range
  - Example: `.sieve_exclusive(200.0, 800.0)` removes muddy midrange, keeps bass and treble
  - Removes all notes whose frequencies fall within [min_freq, max_freq]
  - Perfect for removing problematic frequency ranges or creating spectral gaps
  - Use cases: remove muddy midrange, create frequency notches, experimental arrangements
  - Chain multiple exclusions: `.transform(|t| t.sieve_exclusive(100.0, 200.0).sieve_exclusive(400.0, 600.0))`
  - Opposite of `sieve_inclusive` - useful for subtractive frequency sculpting
- **NEW: `.group(duration)` transform** - Collapse sequential notes into a single chord
  - Example: `.notes(&[C4, E4, G4, C5], 0.25).group(2.0)` → single 4-note chord lasting 2 seconds
  - Takes all notes from a pattern and plays them simultaneously
  - Perfect for converting arpeggios/melodies into harmonic blocks
  - Use cases: arpeggio → chord, melodic line → pad, generative → harmony
  - Example: generate random notes then group them into experimental chords
  - Works with namespace: `.transform(|t| t.group(1.5))`
- **NEW: `.duplicate()` transform** - Duplicate pattern for layered transforms
  - Example: `.notes(&[C4, E4, G4], 0.25).duplicate().transform(|t| t.shift(12))` → melody + octave doubling
  - Creates a copy of all events and appends them after the pattern
  - Different from `.repeat()` - allows transforms to be applied to the duplicated copy
  - Perfect for harmony layers, echo effects, octave doubling
  - Use cases: instant harmonization, texture building, layered variations
  - Example: `.duplicate().transform(|t| t.shift(7).humanize(0.01, 0.05))` → add harmony with variation
  - Chain multiple: `.duplicate().shift(7).duplicate().shift(12)` → 3-part harmony
- **`.shift(semitones)`** - Transpose entire patterns up or down by semitones
  - Example: `.shift(7)` transposes up a perfect fifth, `.shift(-12)` down an octave
  - Works within `pattern_start()` boundaries, preserves timing and all note parameters
- **`.humanize(timing_variance, velocity_variance)`** - Add organic feel to programmed sequences
  - Example: `.humanize(0.02, 0.1)` adds ±20ms timing jitter and ±10% velocity variation
  - Affects both notes and drums, makes mechanical patterns feel natural
- **`.rotate(positions)`** - Cycle pitch sequence while keeping timing intact
  - Example: `.rotate(1)` shifts pitches forward, `.rotate(-1)` shifts backward
  - Original: C4, E4, G4, C5 → After `.rotate(1)`: E4, G4, C5, C4
- **`.retrograde()`** - Classic compositional technique, reverses pitch order
  - Different from `.reverse()` which reverses time - retrograde only reverses pitches
  - Example: C4, E4, G4 becomes G4, E4, C4 at the same time positions
- **`.shuffle()`** - Randomly reorder pitches while maintaining rhythm
  - Each call produces a different random ordering using Fisher-Yates algorithm
  - Perfect for generative variations and exploration
- **`.thin(keep_probability)`** - Probabilistically remove notes to reduce density
  - Example: `.thin(0.7)` keeps ~70% of notes, removes ~30%
  - Great for hi-hat variations, creating space, and sparse textures
- **`.stack(semitones, count)`** - Layer harmonic voices on each note (octave doubling, vocal stacking)
  - Example: `.stack(12, 1)` adds octave above, `.stack(12, 2)` adds two octaves (three-voice stack)
  - Classic production technique for making sounds bigger and richer
  - Can stack any interval: `.stack(7, 1)` adds perfect fifth, `.stack(-12, 1)` bass reinforcement
- **`.mutate(max_semitones)`** - Evolutionary pitch variation for generative music
  - Example: `.mutate(2)` randomly shifts each note by -2 to +2 semitones
  - Creates organic variations while maintaining overall melodic shape
  - Perfect for generative systems and algorithmic composition
- **`.stretch(factor)`** - Time manipulation (speed up/slow down patterns)
  - Example: `.stretch(2.0)` plays at half speed (twice as long), `.stretch(0.5)` at double speed
  - Stretches both note timings AND durations proportionally
  - Great for rhythmic variations and time-based effects
- **`.compress(target_duration)`** - Ergonomic time compression to exact duration (no ratio math!)
  - Example: `.compress(1.0)` fits pattern to exactly 1 second, `.compress(2.5)` to 2.5 seconds
  - Automatically calculates stretch factor internally: `target_duration / current_duration`
  - Wrapper around `.stretch()` for more intuitive live coding workflow
- **`.quantize(grid)`** - Snap note timings to rhythmic grid
  - Example: `.quantize(0.25)` snaps to 16th notes, `.quantize(0.5)` to 8th notes
  - Perfect for cleaning up timing after humanization or fixing sloppy MIDI imports
  - Ensures tight rhythmic accuracy for electronic music production
- **`.palindrome()`** - Mirror pattern forward then backward (symmetrical phrases)
  - Example: `.notes(&[C4, E4, G4], 0.25).palindrome()` becomes C4, E4, G4, G4, E4, C4
  - Creates beautiful symmetrical musical phrases
  - Classic compositional technique for balance and structure
- **`.stutter(probability, repeats)`** - Random glitchy stuttering effect
  - Example: `.stutter(0.5, 4)` gives each note 50% chance to rapidly repeat 4 times
  - Popular in electronic music, glitch, and trap production
  - Creates unpredictable rhythmic complexity
- **`.stutter_every(nth, repeats)`** - Deterministic stuttering (every Nth note)
  - Example: `.stutter_every(4, 8)` makes every 4th note roll 8 times rapidly
  - Perfect for trap hi-hat rolls and rhythmic patterns
  - Predictable unlike `.stutter()` - great for consistent rhythmic effects
- **`.granularize(divisions)`** - Break notes into micro-fragments
  - Example: `.granularize(20)` splits each note into 20 tiny grains
  - Creates shimmering, glitchy, ambient textures
  - Combine with `.mutate()` for insane generative results: `.granularize(20).mutate(3)`
- **`.magnetize(scale_notes)`** - Snap pitches to nearest scale degree (pitch quantization)
  - Example: `.magnetize(&[C4, D4, E4, G4, A4])` forces chromatic melody into C major pentatonic
  - Perfect for generative music - ensures all notes are in-scale
  - Great for modal jazz, fixing off-key notes, or forcing melodies into specific tonalities
- **`.gravity(center_pitch, strength)`** - Apply gravitational pull toward/away from tonal center
  - Example: `.gravity(C4, 0.5)` pulls notes 50% closer to middle C
  - Negative strength repels: `.gravity(C4, -0.3)` pushes notes away from center
  - Creates organic pitch compression/expansion and tonal magnetism effects
- **`.ripple(intensity)`** - Cascading effects where each note influences subsequent notes
  - Example: `.ripple(0.02)` creates water droplet-like cascading timing and pitch shifts
  - Positive intensity pushes notes forward in time and up in pitch
  - Effect decays over time (70% per note) for natural-sounding cascades
- **`.range_dilation(factor)`** - **NEW: Unified pitch range expansion/compression** around pattern's center pitch
  - Example: `.range_dilation(0.5)` compresses range to 50% (tighten melodic contour)
  - Example: `.range_dilation(2.0)` expands range to 200% (exaggerate melodic motion)
  - **Unified design:** `factor < 1.0` compress, `factor = 1.0` no change, `factor > 1.0` expand
  - Replaces need for separate compress/expand functions with single intuitive parameter
  - Calculates geometric mean as center, scales all pitches' distances from center
  - Perfect for generative music: compress wild random patterns or expand subtle variations
  - Great for dynamic control: modulate factor over time for breathing melodies
  - See `examples/new_transforms_demo.rs` for comprehensive demonstrations
- **`.shape_contour(factor)`** - **NEW: Unified melodic interval smoothing/exaggeration**
  - Example: `.shape_contour(0.4)` smooths large jumps to 40% (stepwise motion)
  - Example: `.shape_contour(2.5)` exaggerates intervals to 250% (dramatic leaps)
  - **Unified design:** `factor < 1.0` smooth, `factor = 1.0` no change, `factor > 1.0` exaggerate
  - Replaces need for separate smooth/exaggerate functions with single intuitive parameter
  - Scales melodic intervals relative to first note (anchor point)
  - Perfect for melodic control: tame wild generative patterns or add drama to simple melodies
  - Use cases: fix awkward leaps, create singable melodies, add expressiveness
  - Combine with `.magnetize()` for scale-constrained smoothing
  - See `examples/new_transforms_demo.rs` for smooth/exaggerated contour examples
- **`.echo(delay, repeats, decay)`** - **NEW: Create delay/echo trails with volume decay**
  - Example: `.echo(0.5, 3, 0.7)` creates 3 echoes, 0.5s apart, each 70% volume of previous
  - Parameters: delay time (seconds), number of repetitions, decay factor per echo (0.0-1.0)
  - Duplicates entire pattern multiple times with time offset and exponential volume decay
  - Perfect for ambient effects, spacious textures, rhythmic delays, dub-style echoes
  - Works with notes, drums, and samples - all event types get echoed
  - Decay calculation: echo N is at `decay^N` volume (0.7^3 = 0.343 = 34.3% for 3rd echo)
  - Combine with other transforms: `.shape_contour(0.6).echo(0.3, 2, 0.5)` for smooth + echo
  - See `examples/new_transforms_demo.rs` for echo demonstrations
- **All methods are chainable** and work seamlessly with existing pattern operations
  - Example: `.pattern_start().note(&[C4], 1.0).granularize(16).mutate(4).thin(0.7).shuffle().humanize(0.01, 0.08)`
  - Example: `.transform(|t| t.range_dilation(0.7).shape_contour(1.3).echo(0.4, 2, 0.6))` combines new transforms
- **Fully tested** with 69 new unit tests ensuring correctness and edge cases (includes 2 tests for `.transform()` namespace API, 10 tests for new transforms)

#### Professional Bus Architecture and Master Effects Chain
- **Major architectural refactor** - Complete bus-based mixing system with master effects chain
- **`Bus` struct** - Intermediate mixing layer between tracks and master output
- **Signal flow:** `Tracks → Bus (with effects) → Master (with effects) → Output`
- **Full backward compatibility** - Automatic "default" bus for existing code, no migration needed

**EffectChain - Unified Effect System:**
- **`EffectChain` struct** - Centralized container for all 16 effects with priority-based ordering
- **Dual-mode processing:**
  - `process_mono()` - For individual tracks (mono → mono with effects)
  - `process_stereo()` - For buses and master (stereo → stereo with effects)
- **Priority-based effect ordering** - Effects automatically sorted by priority (lower = earlier in chain)
- **Refactored Track** - Now uses `EffectChain` instead of 16 individual effect fields
- **Consistent API** - Same effect methods work on tracks, buses, and master

**Bus System:**
- **`Bus`** - Groups tracks together with dedicated effects, volume, pan, and solo/mute
- **Flexible routing** - Each track belongs to a named bus (e.g., "drums", "vocals", "synths")
- **Bus-level effects** - Apply reverb, compression, EQ to entire groups of tracks
- **Bus-level mixing** - Volume and pan controls affect all tracks in the bus
- **Solo/mute** - Toggle entire buses on/off for mixing workflows
- **API:**
  ```rust
  // Buses created automatically when adding tracks
  let mut mixer = Mixer::new(Tempo::new(120.0));
  mixer.add_track("kick", track1, "drums");    // Add to "drums" bus
  mixer.add_track("snare", track2, "drums");   // Same bus
  mixer.add_track("bass", track3, "bass");     // Different bus

  // Access buses for configuration
  if let Some(drums_bus) = mixer.buses.get_mut("drums") {
      drums_bus.volume = 0.8;
      drums_bus.pan = 0.1;  // Slight right
      drums_bus.effects.reverb = Some(Reverb::new(0.3, 0.5));
  }
  ```

**Master Effects Chain (16 effects available on Mixer):**
- **Master-level processing** - Apply effects to the final stereo mix before output
- **All 16 effects supported:** EQ, Compressor, Gate, Saturation, Bitcrusher, Distortion, Chorus, Phaser, Flanger, Ring Modulator, Tremolo, AutoPan, Delay, Reverb, Limiter, Parametric EQ
- **Stereo processing** - Master effects process left and right channels independently
- **Stereo-linked compression** - Master compressor uses max(L, R) for natural stereo image preservation
- **Master methods on Mixer:**
  - `.master_eq(eq)` - 3-band EQ for frequency balancing
  - `.master_compressor(compressor)` - Stereo-linked dynamics control
  - `.master_reverb(reverb)` - Global ambience and space
  - `.master_limiter(limiter)` - Prevent clipping on final output
  - `.master_delay(delay)` - Stereo delay effects
  - `.master_parametric_eq(eq)` - Surgical frequency shaping
  - ... (all 16 effects available)
- **Use cases:**
  - Mastering: Apply gentle compression, EQ, and limiting to final mix
  - Creative effects: Reverb wash, tape saturation, bitcrushed lo-fi
  - Mix glue: Subtle compression to "glue" tracks together
  - Protection: Limiter to prevent clipping on final output
- **Example:**
  ```rust
  mixer.master_compressor(Compressor::new(0.5, 3.0, 0.01, 0.1, 44100.0));
  mixer.master_eq(EQ::new(1.0, 0.9, 1.1, 250.0, 4000.0));
  mixer.master_limiter(Limiter::new(0.95));
  ```

**BusBuilder - Fluent API for Bus Configuration:**
- **`BusBuilder`** - Ergonomic fluent interface for bus-level effects and mixing
- **Easy bus access** - `mixer.bus("drums")` returns BusBuilder for chaining
- **All 16 effects supported** - Same fluent API as tracks (`.reverb()`, `.compressor()`, etc.)
- **Example:**
  ```rust
  mixer.bus("drums")
      .reverb(Reverb::new(0.3, 0.4, 0.3))
      .compressor(Compressor::new(0.65, 4.0, 0.01, 0.08, 1.0))
      .volume(0.85)
      .pan(-0.1);
  ```

**Sidechaining / Ducking:**
- **Track-to-bus sidechaining** - Compress one track based on another track's level
- **Bus-to-bus sidechaining** - Compress entire bus based on another bus's level
- **`.with_sidechain_track(name)`** - Configure compressor to duck when specific track plays
- **`.with_sidechain_bus(name)`** - Configure compressor to duck when bus plays
- **Two-pass rendering** - Envelope caching system for efficient sidechain processing
- **Common uses:**
  - EDM kick ducking bass (classic pumping effect)
  - Vocal ducking background music (podcasts, voiceovers)
  - Drums ducking pads (rhythmic breathing)
  - Creative rhythmic pumping effects
- **Example:**
  ```rust
  // Bass ducks when kick drum hits (classic EDM)
  mixer.bus("bass").compressor(
      Compressor::new(0.6, 8.0, 0.001, 0.15, 1.2)
          .with_sidechain_track("kick")
  );
  ```
- **See:** `examples/sidechaining.rs` for complete examples

**Signal Flow Architecture:**
```
Composition
    ↓
Multiple Tracks (each with EffectChain)
    ↓
Buses (grouped tracks, each with EffectChain)
    ↓
Master (final mix with EffectChain)
    ↓
Output (soft clipped to prevent distortion)
```

**Key Features:**
- **Effect priority system** - Automatic ordering: EQ → Dynamics → Saturation → Modulation → Time/Space → Limiting
- **Per-stage effects** - Different effects at track, bus, and master levels
- **Professional workflows** - Matches industry-standard DAW architecture (ProTools, Logic, Ableton)
- **Backward compatible** - Existing code works unchanged via automatic "default" bus
- **Flexible routing** - Easy to reorganize tracks into different buses
- **Performance** - Minimal overhead from effect ordering and priority system

**Technical Implementation:**
- `Mixer::buses: HashMap<String, Bus>` - Named buses for flexible organization
- `Bus::effects: EffectChain` - Bus-level effect processing
- `Mixer::master: EffectChain` - Master-level effect processing
- `EffectChain::effect_order: Vec<u8>` - Cached priority-sorted effect indices
- Static `process_track_static()` - Avoids borrow checker issues during mixing
- Helper methods `all_tracks()` and `tracks()` - Backward compatibility for tests/examples

**Migration Guide:**
- **No changes required** - All tracks go to "default" bus automatically
- **To use buses:**
  ```rust
  // Old way (still works):
  mixer.add_track("kick", kick_track);

  // New way (with buses):
  mixer.add_track("kick", kick_track, "drums");
  mixer.add_track("bass", bass_track, "bass");
  ```
- **Direct field access:** `.tracks` field removed, use `.all_tracks()` method instead

**Benefits:**
- **Industry standard** - Matches professional DAW workflows
- **Better organization** - Group related tracks (drums, vocals, synths)
- **Efficient effects** - Apply one reverb to a bus instead of per-track
- **Mastering ready** - Professional master chain for final output
- **Flexible mixing** - Solo/mute entire instrument groups
- **Future-proof** - Ready for sidechaining, aux sends, and advanced routing

**Breaking Changes:**
- `Mixer.tracks` field removed - use `.all_tracks()` method to access all tracks
- Track struct: Individual effect fields replaced with `effects: EffectChain`
- Internal: `Track` field access patterns changed from `track.delay` to `track.effects.delay`

**Tests:** All 967 tests passing with new architecture
**Examples:** All 73 examples updated and working
**Documentation:** Architecture, mixer, and effects docs updated

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

#### 3D Spatial Audio System
- **Comprehensive 3D spatial audio** - Built-in positioning, distance attenuation, and listener orientation
- **Game-ready implementation** - Perfect for 3D games, VR/AR, and interactive installations
- **Zero-configuration basics** - Automatic distance attenuation and azimuth-based panning out of the box
- **Real-time control** - Move sounds and listener dynamically during playback

**Core Spatial Audio Module (`synthesis::spatial`):**
- **`Vec3`** - 3D vector with standard operations (dot product, cross product, normalization, distance)
- **`SpatialPosition`** - Sound source position and velocity in 3D space
- **`ListenerConfig`** - Listener position, orientation (forward/up vectors), and velocity
- **`SpatialParams`** - Global spatial audio configuration (attenuation model, distances, doppler)
- **`SpatialResult`** - Calculated volume attenuation, pan value, and pitch adjustment
- **`calculate_spatial()`** - Core function computing all spatial audio effects

**Attenuation Models:**
- **`AttenuationModel::None`** - No distance attenuation (constant volume)
- **`AttenuationModel::Linear`** - Linear falloff with distance
- **`AttenuationModel::Inverse`** - 1/distance (realistic for sound pressure)
- **`AttenuationModel::InverseSquare`** - 1/distance² (default, realistic for sound intensity)
- **`AttenuationModel::Exponential`** - Exponential decay with configurable rolloff factor

**Composition-Time Positioning:**
- **`.spatial_position(x, y, z)`** - Set 3D position when composing tracks
- **Track-level positioning** - Spatial position applies to the entire track, not individual events
- **Important:** All events in a track must have the same spatial position (validation enforces this at `into_mixer()`)
- **For different positions:** Use separate tracks or runtime `set_sound_position()` for moving sounds
- **Coordinate system:**
  - X-axis: Left (negative) to Right (positive)
  - Y-axis: Down (negative) to Up (positive)
  - Z-axis: Behind (negative) to Forward (positive)
  - Listener default: Position (0, 0, 0) facing +Z direction
- **Example:**
  ```rust
  // Each instrument on separate track with its own position
  comp.instrument("guitar", &Instrument::pluck())
      .spatial_position(3.0, 0.0, 5.0)  // 3m right, 5m forward
      .notes(&[C4, E4, G4], 0.5);

  comp.instrument("bass", &Instrument::synth_bass())
      .spatial_position(-3.0, 0.0, 2.0)  // 3m left, 2m forward
      .notes(&[C2, G2], 1.0);
  ```

**Real-Time Spatial Control (AudioEngine methods):**
- **`set_sound_position(id, x, y, z)`** - Move a playing sound in real-time
- **`set_listener_position(x, y, z)`** - Update listener position (e.g., player moved)
- **`set_listener_forward(x, y, z)`** - Change listener orientation (which way they're facing)
- **`set_spatial_params(params)`** - Update global spatial audio settings
- **Runtime overrides composition:** If you call `set_sound_position()`, it overrides any composition-time position
- **Use case:** Game loop updates - move sounds with game objects, track player position

**Spatial Audio Features:**
- **Distance attenuation** - Sounds get quieter automatically based on distance
- **Azimuth-based panning** - Horizontal angle determines left/right stereo placement
- **Listener orientation** - Sounds pan correctly relative to which way listener is facing
- **Configurable parameters:**
  - `ref_distance` - Distance where attenuation starts (default: 1.0m)
  - `max_distance` - Distance where sound becomes inaudible (default: 100.0m)
  - `rolloff` - Attenuation curve steepness (default: 1.0)
  - `speed_of_sound` - For doppler calculations (default: 343.0 m/s)
  - `doppler_enabled` - Enable/disable doppler effect (default: false)
  - `doppler_factor` - Doppler effect strength (default: 1.0)

**Key Spatial Concepts:**
- **Automatic mixing** - Spatial calculations integrated into AudioEngine render callback
- **Per-sound positions** - Each sound maintains independent 3D position
- **Global listener** - Single listener configuration affects all spatial sounds
- **Optional positioning** - Sounds without spatial_position use normal pan/volume
- **Multi-source scenes** - Create complex 3D soundscapes with many positioned sounds

**Technical Implementation:**
- **Event-level spatial data storage** - `NoteEvent`, `DrumEvent`, and `SampleEvent` store `spatial_position: Option<SpatialPosition>`
- **Track-level spatial processing** - Spatial audio calculations applied at track level during rendering (mono to stereo conversion)
- **Validation at composition** - `into_mixer()` validates that all events in a track have the same position (panics with clear error if violated)
- **TrackBuilder support** - `.spatial_position()` method sets position for all subsequent events in the builder
- **Lock-free updates** - Command queue for updating sound positions during playback
- **Efficient calculation** - Spatial processing only when `spatial_position` is present
- **Vector math** - Proper 3D mathematics for accurate positioning

**Use Cases:**
- **Game audio:** Footsteps, gunshots, ambient sounds positioned in 3D world
- **VR/AR applications:** Immersive spatial soundscapes
- **Music production:** Creative stereo placement beyond simple panning
- **Cinematic audio:** Dialog, effects, and ambience in 3D space
- **Interactive installations:** Sound that responds to user movement

**Example - Game Integration:**
```rust
// In game loop
let (x, y, z) = player.position;
engine.set_listener_position(x, y, z)?;

for enemy in &enemies {
    if let Some(sound_id) = enemy.sound_id {
        let (ex, ey, ez) = enemy.position;
        engine.set_sound_position(sound_id, ex, ey, ez)?;
    }
}
```

**Example - Multi-Source Spatial Scene:**
```rust
let mut scene = Composition::new(Tempo::new(140.0));

// Left side: Piano
scene.instrument("piano-left", &Instrument::electric_piano())
    .spatial_position(-4.0, 0.0, 6.0)
    .notes(&[C4, E4, G4, E4], 0.375);

// Right side: Synth
scene.instrument("synth-right", &Instrument::warm_pad())
    .spatial_position(4.0, 0.0, 6.0)
    .note(&[G3, B3, D4], 3.0);

// Behind listener: Ambient
scene.instrument("ambient-back", &Instrument::warm_pad())
    .spatial_position(0.0, 0.0, -5.0)
    .note(&[C3, E3, G3], 3.0);
```

**16 comprehensive tests** covering:
- Vector operations (dot, cross, normalization, distance)
- All attenuation models with edge cases
- Azimuth calculation for stereo panning
- Doppler effect calculations
- Spatial result accuracy

**New example:** `spatial_audio_demo.rs` - Comprehensive demonstration with 6 scenarios:
1. Static spatial composition
2. Moving sound in real-time (left-to-right pan)
3. Distance attenuation (approaching sound)
4. Listener movement (rotation around stationary sound)
5. Custom spatial parameters
6. Multi-source spatial scene

**Documentation:** Complete spatial audio guide in `book/src/game-audio/spatial-audio.md` with:
- Built-in 3D spatial audio tutorial
- Composition-time positioning examples
- Real-time control patterns
- Game integration examples
- Manual 2D/3D implementation (for custom logic)

**Exported:** All spatial types in `synthesis::spatial` module and relevant items in prelude

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
- 957 unit tests covering all modules
- 304 documentation tests with examples
- Comprehensive test coverage for composition, drums, effects, synthesis, and theory

#### Examples
- 70+ complete working examples
- Demonstrations of all major features
- Classical technique examples
- Instrument and effect showcases
- Rhythm and pattern examples

[Unreleased]: https://github.com/sqrew/tunes/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/sqrew/tunes/releases/tag/v0.1.0
