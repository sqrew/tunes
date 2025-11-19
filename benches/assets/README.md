# Benchmark Audio Assets

This directory contains diverse audio samples for realistic performance benchmarking.

## Why These Samples?

The previous benchmarks used simple synthetic tones, which are unrealistic because:
- **Simple waveforms** are cache-friendly and predictable
- **Only 3 samples** reused meant unrealistic cache hit rates
- **All sustained tones** missed performance characteristics of transient sounds
- Real game audio has varied lengths, complex spectral content, and unpredictable memory patterns

These samples address those issues.

## Sample Inventory (46 files total)

### Short Transients (0.15-0.25s)
**Footsteps (8 files):**
- `footstep_01.wav` through `footstep_08.wav`
- Duration: ~0.15s each
- Characteristics: Sharp attack, quick decay, bandpass filtered (200-600Hz range)
- Simulates: Footsteps on various surfaces
- Spectral content: Complex harmonics at 80-185Hz fundamental

**Impacts (6 files):**
- `impact_01.wav` through `impact_06.wav`
- Duration: ~0.25s each
- Characteristics: Heavy transient, low-pass filtered (<800Hz), with distortion
- Simulates: Object impacts, collisions, hits
- Spectral content: Complex harmonics at 40-90Hz fundamental

### Medium Duration (0.8-1.5s)

**Gunshots (6 files):**
- `gunshot_01.wav` through `gunshot_06.wav`
- Duration: ~0.8s each
- Characteristics: Very sharp attack, high-pass filtered (>150Hz), crunch distortion, reverb tail
- Simulates: Various firearms
- Spectral content: Complex harmonics at 100-200Hz fundamental, bright overtones

**Explosions (5 files):**
- `explosion_01.wav` through `explosion_05.wav`
- Duration: ~1.5s each
- Characteristics: Complex broadband noise, low-pass filtered (<1200Hz), heavy distortion, reverb + delay
- Simulates: Explosions of various sizes
- Spectral content: 6+ harmonics from 30-70Hz fundamental

**Voice-like Sounds (6 files):**
- `voice_01.wav` through `voice_06.wav`
- Duration: ~0.8s each
- Characteristics: Harmonic series (5 partials), bandpass "formant" filtering, chorus/vibrato, reverb
- Simulates: Voice grunts, calls, vocalizations
- Spectral content: Natural harmonic series from 120-245Hz fundamental

**Bass-heavy (4 files):**
- `bass_01.wav` through `bass_04.wav`
- Duration: ~1.5s each
- Characteristics: Sub-bass focus, distortion, low-pass (<200Hz), heavy compression
- Simulates: Explosions, rumble, sub-bass impacts
- Spectral content: 40-95Hz fundamental with harmonics

**High-frequency (4 files):**
- `high_freq_01.wav` through `high_freq_04.wav`
- Duration: ~1.0s each
- Characteristics: Bright treble content, high-pass filtered (>800Hz), reverb
- Simulates: Glass, metal impacts, UI sounds
- Spectral content: 1000-2500Hz fundamental

### Sustained/Loops (3-4s)

**Ambient Loops (4 files):**
- `ambient_01.wav` through `ambient_04.wav`
- Duration: ~4s each
- Characteristics: Sustained pad texture, phaser modulation, heavy reverb, evolving
- Simulates: Wind, atmosphere, environmental ambience
- Spectral content: 150-550Hz fundamental with inharmonic partials

**Engine Sounds (3 files):**
- `engine_01.wav` through `engine_03.wav`
- Duration: ~3s each
- Characteristics: Rhythmic sustained, distortion, flanger (mechanical vibration)
- Simulates: Machinery, vehicles, motors
- Spectral content: 60-120Hz fundamental with strong harmonics

## Performance Characteristics

**Total sample pool size:** 46 unique samples
**Total disk usage:** ~2.5 MB (varies by system)
**L3 cache:** Typical CPUs have 8-16 MB L3 cache
**Cache pressure:** With 46 samples of varying sizes, not all will fit in L3 → realistic cache miss patterns

**Length distribution:**
- 14 samples < 0.5s (transients)
- 21 samples 0.5s-1.5s (medium)
- 11 samples > 2s (sustained)

**Spectral diversity:**
- 13 samples with sub-bass content (<100Hz)
- 19 samples mid-range (100Hz-1kHz)
- 14 samples with bright content (>1kHz)

**Effects complexity:**
- All samples have at least 2 effects applied
- 26 samples have reverb (expensive)
- 15 samples have distortion/saturation
- 10 samples have modulation effects (chorus/phaser/flanger)
- 5 samples have delay
- 4 samples have compression

## Usage in Benchmarks

Use these samples in your benchmarks by loading them:

```rust
let samples: Vec<Sample> = vec![
    Sample::from_file("benches/assets/footstep_01.wav")?,
    Sample::from_file("benches/assets/impact_01.wav")?,
    // ... load all 46 samples
];

// Then use in benchmark
for i in 0..sample_count {
    let sample = &samples[i % samples.len()]; // Cycle through all 46
    // ...
}
```

## Comparison to Previous Benchmarks

| Metric | Old (Synthetic) | New (These Assets) |
|--------|----------------|-------------------|
| Sample count | 3 | 46 |
| Unique waveforms | 3 simple tones | 46 complex sounds |
| Length variety | All 4s sustained | 0.15s - 4s varied |
| Spectral complexity | Simple harmonics | Complex, realistic |
| Cache behavior | Always hits | Realistic misses |
| Effects | Baked in export | Baked in export |

## Regenerating These Assets

If you need to regenerate these samples:

```bash
cargo run --example generate_benchmark_assets --release
```

This will overwrite all files in this directory.

## Git/Crate Inclusion

These files are currently excluded from git (see `.gitignore`) and from the published crate (see `Cargo.toml`).

**Options:**
1. **Commit to git** if you want benchmarks reproducible across machines
2. **Keep excluded** and regenerate on each machine (current setup)
3. **Ship in crate** if benchmark users need these files (would increase crate size by ~2.5MB)

Current recommendation: **Keep excluded** from published crate, but **commit to git** for reproducibility.
