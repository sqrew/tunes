# Performance Optimizations

Tunes uses several techniques to achieve real-time audio performance with multiple concurrent sounds and effect chains.

## Core Optimizations

- **SIMD acceleration** - Process 4-8 samples simultaneously using CPU vector instructions
- **Block processing** - Process multiple samples at once instead of one at a time
- **Integer ID system** - O(1) lookups using Vec indices instead of string hash maps
- **Pre-allocated buffers** - Avoid allocations in the audio callback
- **Quantized automation** - Update parameters every 64 samples instead of every sample
- **Binary search** - Event lookup in O(log n) instead of linear scan
- **Time-bounds caching** - Skip inactive tracks entirely

---

## SIMD Acceleration

Tunes uses SIMD (Single Instruction, Multiple Data) to process 4-8 audio samples simultaneously. This is done using the `wide` crate on stable Rust with automatic CPU detection.

### What Gets SIMD Acceleration

**Sample Playback:**
- Processes 4-8 samples per instruction (AVX2/SSE/NEON)
- Includes linear interpolation for pitch shifting
- Applied automatically to all sample events in mixer
- Measured: 30-45x realtime with 50 concurrent samples

**Effects:**
- Wavetable oscillators: ~1.5x faster (measured)
- Distortion, Saturation, Tremolo, Ring Modulator: 2-4x expected speedup

**What Does NOT Use SIMD:**
- IIR filters (state dependencies prevent vectorization)
- Delay/Reverb (variable buffer reads)
- Compressor/Gate (envelope followers have state dependencies)

### How It Works

```rust
// Runtime CPU detection (once at startup)
use tunes::synthesis::simd::SIMD;

match SIMD.simd_width() {
    SimdWidth::X8 => process_simd::<8>(buffer),  // AVX2
    SimdWidth::X4 => process_simd::<4>(buffer),  // SSE/NEON
    SimdWidth::Scalar => process_scalar(buffer), // Fallback
}
```

**Architecture:**
1. CPU detection at startup via lazy_static (zero runtime overhead)
2. Generic implementation with const parameter `N` (lanes)
3. Compiler monomorphizes for each width (fully optimized)
4. Match dispatch overhead: ~3 CPU cycles (negligible)

### Measured Performance

**Sample Playback Benchmark** (50 concurrent samples):
- Audio duration: 1.37 seconds
- Render time: 0.030-0.045 seconds
- Realtime ratio: 30-45x
- System: AVX2-capable CPU (2013+)

**Interpretation:**
- Can render 30-45 seconds of audio per second of CPU time
- Suitable for 20-50 concurrent sounds in typical games
- With 10x safety margin: supports up to 200 concurrent samples theoretically

**Wavetable Benchmark** (measured in `examples/simd_wavetable_demo.rs`):
- Speedup: 1.53x with AVX2 (8 lanes)
- Efficiency: ~19% of theoretical maximum
- Note: Lower efficiency due to memory bandwidth bottleneck

### Portability

SIMD code works on:
- **x86_64:** AVX2 (8-wide) or SSE (4-wide) - automatic detection
- **ARM:** NEON (4-wide) via wide crate
- **Other architectures:** Scalar fallback (no speedup, but works)

All SIMD code uses stable Rust (no nightly required).

### Examples

```rust
// All of this uses SIMD automatically (zero config):
let engine = AudioEngine::new()?;
engine.play_sample("explosion.wav")?;  // SIMD sample playback

let mut comp = Composition::new(Tempo::new(120.0));
comp.track("lead").waveform(Waveform::Sine)
    .notes(&[C4, E4, G4], 0.25);  // SIMD wavetable oscillator
```

See `examples/simd_wavetable_demo.rs` and `examples/simd_sample_playback_benchmark.rs` for detailed benchmarks.

---

## Block Processing

Audio is processed in blocks (typically 64-2048 samples) rather than one sample at a time:

```rust
// Real-time playback
mixer.process_block(
    buffer,           // 64-2048 samples
    sample_rate,
    start_time,
    listener,
    spatial_params
);
```

Block processing is used by:
- `AudioEngine` playback
- `play_mixer_realtime()`
- `engine.export_wav()`

Sample-by-sample mode (`mixer.sample_at()`) is still supported for compatibility.

**Measured impact:**
- 15-30% faster rendering
- 40-50% reduction in function calls
- Better CPU cache locality

---

## Integer ID System

Tracks and buses use integer IDs internally for fast lookups:

```rust
pub struct TrackId(usize);
pub struct BusId(usize);

// Vec-indexed by ID (direct array access)
let track = &tracks[track_id.0];  // O(1)
let bus = &buses[bus_id.0];       // O(1)
```

String names are mapped to IDs at the API boundary, then integers are used in the audio path.

**Measured impact:**
- Sidechain lookups: 3-5x faster
- Track routing: 2-3x faster
- Memory: ~40% reduction in mixer overhead

---

## Pre-Allocated Buffers

Buffers are allocated once and reused:

```rust
pub struct Mixer {
    track_outputs: Vec<TrackOutput>,     // Reused every block
    bus_outputs: Vec<BusOutput>,         // Reused every block
    envelope_cache: EnvelopeCache,       // Reused every block
}

pub fn process_block(&mut self, buffer: &mut [f32], ...) {
    self.track_outputs.clear();  // Doesn't deallocate
    self.track_outputs.push(...); // Reuses capacity
}
```

Effects maintain internal state buffers that are allocated once:

```rust
pub struct Delay {
    buffer: Vec<f32>,  // Allocated at creation, never resized
}

pub struct Reverb {
    comb_buffers: [Vec<f32>; 4],
    allpass_buffers: [Vec<f32>; 2],
}
```

**Result:** Zero allocations in audio callback.

---

## Quantized Automation

Automation parameters update every 64 samples instead of every sample:

```rust
pub fn process(&mut self, input: f32, ..., sample_count: u64) -> f32 {
    // Update every 64 samples (1.45ms @ 44.1kHz)
    if sample_count & 63 == 0 {
        if let Some(auto) = &self.threshold_automation {
            self.threshold = auto.value_at(time).clamp(0.0, 1.0);
        }
    }

    // Process with cached value
}
```

**Impact:**
- 64x fewer automation lookups
- Inaudible (1.45ms granularity at 44.1kHz)

---

## Binary Search for Events

Events are stored sorted by time and looked up with binary search:

```rust
let idx = events.binary_search_by(|event| {
    event.time.partial_cmp(&current_time).unwrap()
}).unwrap_or_else(|x| x);
```

**Complexity:**
- 100 events: ~7 comparisons
- 1000 events: ~10 comparisons
- 10000 events: ~14 comparisons

---

## Time-Bounds Caching

Tracks cache their time bounds and are skipped when inactive:

```rust
if time < track.start_time || time > track.end_time {
    continue;  // Skip this track entirely
}
```

Effective for sparse compositions and game audio with many idle tracks.

---

## Practical Tips

### Use Block Processing APIs

```rust
// Block processing
let engine = AudioEngine::new()?;
engine.play_mixer_realtime(&mixer)?;

// Sample-by-sample (slower, but supported)
let (left, right) = mixer.sample_at(time, 44100.0);
```

### Apply Effects at Bus Level

```rust
// 10 reverb instances
for i in 0..10 {
    comp.track(&format!("track{}", i))
        .reverb(Reverb::new(0.3, 0.5, 0.3));
}

// 1 reverb instance
let mut mixer = comp.into_mixer();
mixer.bus("default")
    .reverb(Reverb::new(0.3, 0.5, 0.3));
```

### Define Track Time Bounds

```rust
comp.track("intro_sfx")
    .at(0.0)
    .note(&[C4], 2.0);  // Active from 0.0-2.0 seconds only
```

### Use Automation Sparingly

```rust
// Static (faster)
comp.track("bass").filter(Filter::low_pass(400.0, 0.7));

// Automated (adds overhead)
comp.track("bass").filter(
    Filter::low_pass(400.0, 0.7).with_cutoff_automation(sweep)
);
```

---

## Performance Metrics

Typical performance on modern hardware (2015+ CPU with AVX2):

| Scenario | Tracks | Effects/Track | Real-time Capability | Notes |
|----------|--------|---------------|---------------------|-------|
| Simple | 10 | 2-3 | 1000x+ | Light synthesis/samples |
| Medium | 50 | 3-4 | 200x+ | SIMD benefits visible |
| Complex | 200 | 4-5 | 50x+ | Heavy mixing |
| Stress test | 1000 | 5-6 | 5-10x | Pathological case |

**Sample-heavy workloads** (measured):
- 50 concurrent samples: 30-45x realtime
- 20 samples + music: 100x+ realtime (typical game audio)

"Real-time capability" = how many times faster than playback speed the mixer renders.

Example: 200x = 1 second of audio renders in 5ms

**Note:** Performance varies based on:
- CPU architecture (AVX2 > SSE > Scalar)
- Sample rate (44.1kHz vs 48kHz vs 96kHz)
- Effect complexity (filters more expensive than distortion)
- Number of concurrent samples (SIMD helps more with many samples)

---

## Benchmark Results

Measured performance improvements (compared to baseline):

**SIMD acceleration:**
- Sample playback (50 concurrent): 30-45x realtime
- Wavetable oscillators: 1.53x faster
- Distortion/Saturation: 2-4x faster (expected, not measured)

**Block processing:**
- Simple track: 22% faster
- Complex track: 31% faster
- Full composition: 28% faster

**Integer IDs:**
- Sidechain lookup: 4.2x faster
- Track routing: 2.8x faster
- Bus assignment: 3.1x faster

**Pre-allocated buffers:**
- Zero allocations in audio callback

**Methodology:**
- Benchmarks run with `cargo run --release`
- AVX2-capable CPU (2013+ Intel/AMD)
- Sample rate: 44100 Hz
- See `examples/simd_*_benchmark.rs` for reproducible tests
