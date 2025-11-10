# Performance Optimizations

Tunes is designed for real-time audio performance with several key optimizations that allow it to handle hundreds of concurrent sounds with complex effect chains.

## Overview

The library achieves high performance through:
- **Block processing** - Process multiple samples at once instead of one at a time
- **Integer ID system** - O(1) lookups instead of string hash maps
- **Pre-allocated buffers** - Avoid allocations in the audio hot path
- **Quantized automation** - Update parameters every 64 samples instead of every sample
- **Optimized effect ordering** - Pre-computed priority sorting
- **Cache-friendly data structures** - Minimize memory indirection

---

## Block Processing

### What is Block Processing?

Instead of generating audio one sample at a time, Tunes processes entire buffers (blocks) of samples in batches. This dramatically reduces function call overhead and enables better CPU cache utilization.

**Sample-by-sample (older approach):**
```rust
// Called ~44,100 times per second at 44.1kHz
for each sample {
    call sample_at(time)
        -> call track.process()
           -> call effect.process()
              -> calculate one sample
              -> return
        <- return
    <- return
}
```

**Block processing (optimized):**
```rust
// Called ~689 times per second (64-sample blocks at 44.1kHz)
for each block of 64 samples {
    call process_block(buffer, ...)
        -> fill entire buffer at once
           -> process 64 samples through each effect
        <- return all 64 samples
    <- return full buffer
}
```

### Performance Impact

**Measured improvements:**
- **15-30% faster** rendering for typical compositions
- **40-50% fewer** function calls
- **Better CPU cache** locality (processing buffers sequentially)
- **Reduced overhead** from parameter checks and automation lookups

### Implementation Details

The Mixer uses dual-mode processing:

```rust
// Real-time playback (AudioEngine)
mixer.process_block(
    buffer,           // 64-2048 samples typically
    sample_rate,
    start_time,
    listener,
    spatial_params
);

// Legacy/export mode (still supported)
let (left, right) = mixer.sample_at(time, sample_rate);
```

**Key optimizations in block mode:**
- RMS envelope calculation averaged across the entire block
- Sidechain envelopes computed once per block
- Automation values updated every 64 samples (quantized)
- Effect state updated in batches

### When Block Processing is Used

- **AudioEngine playback**: Always uses block processing
- **Real-time control**: `play_mixer_realtime()` uses block mode
- **Export via AudioEngine**: `engine.export_wav()` uses block mode
- **Direct Mixer export**: Uses sample-by-sample for compatibility

---

## Integer ID System

### The Problem with Strings

Originally, tracks and buses were identified by strings (`HashMap<String, Track>`). This caused performance issues:
```rust
// OLD: String-based lookups
let track = tracks.get("kick");  // Hash "kick" string, lookup in HashMap
let bus = buses.get("drums");    // Hash "drums" string, lookup in HashMap
```

**Costs:**
- String hashing (computing hash from characters)
- HashMap lookup (O(1) average, but with overhead)
- String cloning when passing track names around
- Cache misses from pointer indirection

### The Solution: Integer IDs

Tracks and buses are now identified by small integer IDs (`TrackId`, `BusId`):

```rust
// NEW: Integer-based lookups
pub struct TrackId(usize);  // Just a number: 0, 1, 2, 3, ...
pub struct BusId(usize);    // Just a number: 0, 1, 2, 3, ...

// Vec-indexed by ID (direct array access)
let track = &tracks[track_id.0];  // O(1) direct index
let bus = &buses[bus_id.0];       // O(1) direct index
```

**Benefits:**
- **O(1) lookups** with no hashing overhead
- **No string clones** in the hot path
- **Cache-friendly** - sequential IDs = sequential memory
- **Smaller memory** footprint (8 bytes vs ~24+ bytes for String)

### How It Works

**ID Generation:**
```rust
pub struct TrackIdGenerator {
    next_id: usize,
}

impl TrackIdGenerator {
    pub fn next(&mut self) -> TrackId {
        let id = TrackId(self.next_id);
        self.next_id = self.next_id.wrapping_add(1);  // Overflow-safe increment
        id
    }
}
```

**Storage:**
```rust
// Sparse Vec: Some(bus) at bus.id index, None otherwise
buses: Vec<Option<Bus>>

// String mapping kept for user-facing API
bus_name_to_id: HashMap<String, BusId>
```

**Two-Layer Design:**
- **Cold path (user API)**: `mixer.bus("drums")` - uses HashMap for name → ID
- **Hot path (audio rendering)**: Uses integer ID directly for Vec indexing

### Performance Impact

**Measured improvements:**
- **Sidechain lookups**: 3-5x faster (critical for real-time ducking)
- **Track routing**: 2-3x faster (every sample)
- **Memory usage**: ~40% reduction in mixer overhead

---

## Pre-Allocated Buffers

### The Allocation Problem

Allocating memory during audio rendering causes:
- **Latency spikes** (allocation can take microseconds)
- **Fragmentation** over time
- **Cache pollution** (allocator thrashing)

### Buffer Pooling Strategy

Tunes pre-allocates buffers at mixer creation and reuses them:

```rust
pub struct Mixer {
    // Pre-allocated buffers (reused every process_block call)
    track_outputs: Vec<TrackOutput>,    // Track results
    bus_outputs: Vec<BusOutput>,        // Bus results
    envelope_cache: EnvelopeCache,      // RMS envelopes for sidechaining

    // Initialized with capacity
    pub fn new(tempo: Tempo) -> Self {
        Self {
            track_outputs: Vec::with_capacity(64),  // Room for 64 tracks
            bus_outputs: Vec::with_capacity(16),    // Room for 16 buses
            envelope_cache: EnvelopeCache::new(64, 16),
            // ...
        }
    }
}
```

**How it works:**
```rust
pub fn process_block(&mut self, buffer: &mut [f32], ...) {
    // Clear (doesn't deallocate, just sets len = 0)
    self.track_outputs.clear();
    self.envelope_cache.clear();

    // Reuse capacity - no allocations unless we exceed capacity
    self.track_outputs.push(track_output);  // Fast append
}
```

### Effect State Buffers

Effects maintain internal state buffers:
```rust
pub struct Delay {
    buffer: Vec<f32>,  // Allocated once, never resized
    // ...
}

pub struct Reverb {
    comb_buffers: [Vec<f32>; 4],  // 4 fixed buffers
    allpass_buffers: [Vec<f32>; 2],
    // ...
}
```

These buffers are allocated when the effect is created and reused throughout the mixer's lifetime.

---

## Quantized Automation

### The Problem

Updating automation parameters every single sample is expensive:

```rust
// EXPENSIVE: Called 44,100 times per second
for each sample {
    if let Some(auto) = &self.threshold_automation {
        self.threshold = auto.value_at(time);  // Interpolation math
    }
}
```

### The Solution: Batch Updates

Update parameters every 64 samples instead:

```rust
// OPTIMIZED: Called ~689 times per second
pub fn process(&mut self, input: f32, sample_rate: f32, time: f32, sample_count: u64) -> f32 {
    // Quantized automation lookups (every 64 samples = 1.45ms @ 44.1kHz)
    // Use bitwise AND instead of modulo for power-of-2
    if sample_count & 63 == 0 {
        if let Some(auto) = &self.threshold_automation {
            self.threshold = auto.value_at(time).clamp(0.0, 1.0);
        }
        // Update other parameters...
    }

    // Process with cached parameter values
    // ...
}
```

**Benefits:**
- **64x fewer** automation lookups
- **Inaudible difference** (1.45ms granularity at 44.1kHz)
- **Consistent performance** regardless of automation complexity

**When to use different quantization:**
- **64 samples**: Default (good for most parameters)
- **32 samples**: For very fast filter sweeps
- **128 samples**: For slow-moving parameters (reverb size, etc.)

---

## Optimized Effect Ordering

### Priority-Based Sorting

Effects are automatically sorted by priority when added, not on every sample:

```rust
pub struct EffectChain {
    // Effects (can be in any order)
    pub eq: Option<EQ>,
    pub compressor: Option<Compressor>,
    pub reverb: Option<Reverb>,
    // ...

    // Pre-computed processing order (sorted by priority)
    effect_order: Vec<u8>,  // [0, 1, 12, ...] = [EQ, Compressor, Reverb, ...]
}

impl EffectChain {
    fn rebuild_effect_order(&mut self) {
        // Called ONLY when effects are added/removed
        let mut priority_list = vec![];
        if let Some(ref eq) = self.eq {
            priority_list.push((eq.priority, 0));  // 0 = EQ effect ID
        }
        // ... add all effects ...

        // Sort by priority ONCE
        priority_list.sort_by_key(|(priority, _)| *priority);

        // Store just the effect IDs in order
        self.effect_order = priority_list.iter().map(|(_, id)| *id).collect();
    }
}
```

**Processing:**
```rust
pub fn process_stereo(&mut self, left: f32, right: f32, ...) -> (f32, f32) {
    let mut left_signal = left;
    let mut right_signal = right;

    // Iterate through pre-sorted effect order
    for &effect_id in &self.effect_order {
        match effect_id {
            0 => { /* EQ */ }
            1 => { /* Compressor */ }
            // ...
        }
    }

    (left_signal, right_signal)
}
```

**Performance:**
- **No sorting** in the hot path
- **Branch prediction** friendly (same order every time)
- **Cache-friendly** (sequential iteration)

---

## Event Lookup Optimizations

### Binary Search for Events

Musical events (notes, drums) are stored sorted by time and looked up using binary search:

```rust
pub struct Track {
    pub events: Vec<AudioEvent>,  // Sorted by time
    // ...
}

// Fast event lookup using binary search
let idx = events.binary_search_by(|event| {
    event.time.partial_cmp(&current_time).unwrap()
}).unwrap_or_else(|x| x);
```

**Complexity:**
- Linear scan: O(n) - check every event
- Binary search: O(log n) - halve the search space each step

**Impact:**
- **100 events**: ~7 comparisons instead of 100
- **1000 events**: ~10 comparisons instead of 1000
- **10000 events**: ~14 comparisons instead of 10000

### Time-Bounds Caching

Tracks cache their time bounds to skip processing when inactive:

```rust
pub struct Track {
    start_time: f32,  // Earliest event
    end_time: f32,    // Latest event + duration
    // ...
}

// Skip tracks outside the current time window
if time < track.start_time || time > track.end_time {
    continue;  // Don't process this track
}
```

**Benefits:**
- Tracks with no events at current time are **completely skipped**
- Especially effective for:
  - Sparse compositions (lots of silence)
  - Long compositions with section-based instruments
  - Game audio (many idle tracks)

---

## Stereo-Linked Processing

Compressor and Limiter use optimized stereo-linked detection:

```rust
pub fn process_stereo_linked(
    &mut self,
    left: f32,
    right: f32,
    // ...
) -> (f32, f32) {
    // Single detection calculation
    let peak = left.abs().max(right.abs());

    // Single gain reduction calculation
    let gain = calculate_gain_reduction(peak);

    // Apply same gain to both channels
    (left * gain, right * gain)
}
```

**vs. Independent Processing:**
```rust
// OLD: Process each channel separately (slower, causes image shift)
let left_gain = calculate_gain_reduction(left.abs());
let right_gain = calculate_gain_reduction(right.abs());
(left * left_gain, right * right_gain)
```

**Benefits:**
- **Fewer calculations** (one detection instead of two)
- **Better cache usage** (single path through calculation)
- **Preserves stereo image** (same gain on both channels)

---

## Practical Performance Tips

### 1. Use Block Processing APIs

```rust
// GOOD: Uses block processing
let engine = AudioEngine::new()?;
engine.play_mixer_realtime(&mixer)?;

// SLOWER: Sample-by-sample (but still supported)
let (left, right) = mixer.sample_at(time, 44100.0);
```

### 2. Apply Effects at Bus Level

```rust
// SLOW: Reverb on every track (10 reverb instances)
for i in 0..10 {
    comp.track(&format!("track{}", i))
        .reverb(Reverb::new(0.3, 0.5, 0.3));
}

// FAST: One reverb on the bus (1 reverb instance)
let mut mixer = comp.into_mixer();
mixer.bus("default")
    .reverb(Reverb::new(0.3, 0.5, 0.3));
```

**Impact:** 10x fewer reverb calculations

### 3. Use Appropriate Buffer Sizes

AudioEngine automatically uses optimal buffer sizes (typically 512-2048 samples), but you can tune if needed:
- **Smaller buffers** (256-512): Lower latency, higher CPU usage
- **Larger buffers** (2048-4096): Lower CPU usage, higher latency

For offline rendering (export), larger buffers are always better.

### 4. Minimize Active Tracks

Use time-bounds to your advantage:
```rust
// Define clear start/end times
comp.track("intro_sfx")
    .at(0.0)  // Start at beginning
    .note(&[C4], 2.0);  // Ends at 2 seconds

// Track is automatically skipped outside 0.0-2.0 second range
```

### 5. Prefer Simple Automation

```rust
// FAST: Static values
comp.track("bass")
    .filter(Filter::low_pass(400.0, 0.7));

// SLOWER: Automation (still fast, but adds overhead)
comp.track("bass")
    .filter(Filter::low_pass(400.0, 0.7)
        .with_cutoff_automation(sweep));
```

Only use automation when you need it.

---

## Performance Metrics

Typical performance on modern hardware (2020+ CPU):

| Scenario | Tracks | Effects/Track | Real-time Capability |
|----------|--------|---------------|---------------------|
| Simple composition | 10 | 2-3 | 1000x real-time |
| Medium composition | 50 | 3-4 | 200x real-time |
| Complex composition | 200 | 4-5 | 50x real-time |
| Stress test | 1000 | 5-6 | 5-10x real-time |

**"Real-time capability"** means how many times faster than playback speed the mixer can render.

**Example:** 200x real-time = 1 second of audio renders in 5ms

---

## Benchmark Comparisons

Internal benchmarks show:

**Block processing vs. sample-by-sample:**
- Simple track (sine wave + reverb): **22% faster**
- Complex track (synth + 5 effects): **31% faster**
- Full composition (50 tracks): **28% faster**

**Integer IDs vs. String lookups:**
- Sidechain lookup: **4.2x faster**
- Track routing in mixer: **2.8x faster**
- Bus assignment: **3.1x faster**

**Pre-allocated buffers:**
- Zero allocations in hot path (measured with `cargo flamegraph`)
- Eliminates GC-like pauses seen in earlier versions

---

## Future Optimizations

Planned but not yet implemented:

1. **SIMD** (Single Instruction, Multiple Data)
   - Process 4-8 samples at once using CPU vector instructions
   - Estimated 2-4x speedup for arithmetic-heavy effects

2. **Multi-threaded mixing**
   - Process buses in parallel
   - Requires careful synchronization for sidechain

3. **Effect-specific optimizations**
   - IIR filter state-space forms
   - FFT-based convolution reverb
   - Vectorized delay buffer access

4. **GPU acceleration** (stretch goal)
   - Offload reverb/convolution to GPU
   - Experimental; latency concerns

---

**Key Takeaway:** Tunes is optimized for real-time audio with intelligent trade-offs. Use block processing APIs, group tracks into buses, and trust the built-in optimizations to handle the rest.

---

**Next:** Learn about [MIDI Import/Export](./midi.md) →
