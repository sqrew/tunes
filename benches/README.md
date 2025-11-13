# Tunes Benchmarks

Performance benchmarks for the Tunes audio library. These measure real-world performance characteristics to help users verify their use cases.

## Available Benchmarks

### SIMD Performance

| Benchmark | Description | What It Measures |
|-----------|-------------|------------------|
| `simd_sample_playback` | Sample playback with SIMD acceleration | Renders 50 concurrent samples, measures realtime ratio (30-45x expected) |
| `simd_wavetable` | Wavetable oscillator SIMD vs scalar | Compares scalar vs SIMD wavetable lookups (1.5x speedup expected) |
| `simd_effects` | **NEW** SIMD effects (Distortion, Saturation, Tremolo, Ring Mod) | Tests SIMD acceleration for effects processing |

### Real-World Performance

| Benchmark | Description | What It Measures |
|-----------|-------------|------------------|
| `concurrent_mixing` | Multiple tracks with effects | 5/10/20 tracks with reverb + delay + compression - realistic production scenario |
| `polyphony` | Maximum simultaneous voices | Tests 10, 50, 100, 200, 500 simultaneous notes - finds the breaking point |
| `export_speed` | Offline rendering speed | 30-second composition export to WAV/FLAC - batch processing capability |

### Memory & Latency

| Benchmark | Description | What It Measures |
|-----------|-------------|------------------|
| `sample_cache` | Automatic sample caching | First load (disk I/O) vs subsequent loads (cache hits) - 10-100x speedup |
| `streaming_memory` | Streaming vs normal loading | Memory usage comparison for large audio files - proves streaming works |
| `realtime_latency` | Control responsiveness | Measures volume/pan/rate control latency with different buffer sizes |

## Running Benchmarks

Run any benchmark with:

```bash
cargo run --release --bin <name>

# SIMD Performance:
cargo run --release --bin simd_sample_playback
cargo run --release --bin simd_wavetable
cargo run --release --bin simd_effects

# Real-World Performance:
cargo run --release --bin concurrent_mixing
cargo run --release --bin polyphony
cargo run --release --bin export_speed

# Memory & Latency:
cargo run --release --bin sample_cache
cargo run --release --bin streaming_memory
cargo run --release --bin realtime_latency
```

**Important:** Always use `--release` for accurate performance measurements.

## Expected Results

### SIMD Performance

**SIMD Sample Playback:**
- Realtime ratio: 30-45x (with AVX2)
- Test: 50 concurrent samples, ~1.4s audio
- Render time: ~30-45ms

**SIMD Wavetable:**
- Speedup: 1.5-2x (AVX2 vs scalar)
- Test: 64KB buffer, 1000 iterations

**SIMD Effects:**
- Realtime ratio: 20-40x (10 tracks with effects)
- Effects tested: Distortion, Saturation, Tremolo, Ring Modulator
- Speedup: 4-8x per effect

### Real-World Performance

**Concurrent Mixing:**
- 5 tracks: >20x realtime (excellent)
- 10 tracks: >10x realtime (production ready)
- 20 tracks: >5x realtime (good)

**Polyphony:**
- 10-50 voices: >10x realtime (safe for any CPU)
- 100 voices: >5x realtime (modern CPUs)
- 200+ voices: >2x realtime (may need optimization)

**Export Speed:**
- WAV export: 50-100x realtime
- FLAC export: 30-60x realtime (compression overhead)
- 30-second composition: ~0.3-0.5s render time

### Memory & Latency

**Sample Cache:**
- First load: 100-1000µs (disk I/O + decode)
- Cache hit: <10µs (memory copy)
- Speedup: 10-100x faster on cache hits

**Streaming Memory:**
- Normal loading: ~2 MB per minute of audio
- Streaming: ~1 MB fixed (ring buffer)
- Savings: 50-90% memory reduction

**Realtime Latency:**
- Buffer 512: ~12ms (low latency, may glitch)
- Buffer 2048: ~46ms (game audio sweet spot)
- Buffer 8192: ~186ms (default, stable)
- Control overhead: <0.1ms per operation

## System Requirements

**For best results:**
- AVX2-capable CPU (Intel Haswell 2013+, AMD Excavator 2015+)
- Release mode (`--release`)
- Sufficient CPU headroom (close other applications)

**SIMD detection:**
- AVX2: 8 lanes (best performance)
- SSE/NEON: 4 lanes (good performance)
- Scalar: 1 lane (fallback)

## What These Benchmarks Prove

✅ **SIMD acceleration is real** - 30-45x realtime for sample playback, effects accelerated 4-8x
✅ **Production-ready mixing** - 10+ tracks with heavy effects runs at >10x realtime
✅ **High polyphony** - 100+ simultaneous voices achievable on modern CPUs
✅ **Fast offline rendering** - Export at 50-100x realtime for batch processing
✅ **Efficient caching** - 10-100x faster on repeated sample plays
✅ **Memory-efficient streaming** - 50-90% memory savings for large files
✅ **Low control latency** - <1ms control overhead, adjustable buffer for latency vs stability
✅ **Game audio viable** - Can handle 50+ concurrent sounds with spatial audio
✅ **No external dependencies** - Rust compiles optimized SIMD automatically

## Performance Comparisons

See [book/src/comparisons.md](../book/src/comparisons.md) for detailed comparisons with other audio libraries.

---

**Note:** For automated integration tests, see `tests/`. For user-facing examples, see `examples/`.
