# Mixer.rs Optimization Analysis

## Summary
Overall, the mixer is **already well-optimized** with good architecture:
- ✅ Vec-based indexing instead of HashMap (integer IDs)
- ✅ Pre-allocated buffers (track_outputs, bus_outputs, envelope_cache)
- ✅ SIMD in process_block (via Rayon + custom SIMD module)
- ✅ Binary search for active events
- ✅ Fast trig for panning
- ✅ Inline annotations on hot paths

However, there are **several opportunities** for further optimization:

---

## Critical Path: `sample_at()` Method (lines 883-1031)

### Issue 1: Double Bus Iteration ⚠️ HIGH IMPACT
**Location:** Lines 907-938 (Pass 1) and 943-1010 (Pass 2)

**Problem:**
```rust
// PASS 1: Iterate over all buses to process tracks
for bus_opt in self.buses.iter_mut() { ... }

// PASS 2: Iterate over all buses AGAIN to apply effects
for bus_opt in self.buses.iter_mut() { ... }
```

**Why it's slow:**
- Two separate iterations means bus cache misses
- Envelope cache is the only reason for two passes (sidechaining)
- But most buses don't use sidechaining!

**Optimization:**
```rust
// Option A: Single-pass for non-sidechained buses (90% case)
// Only do two-pass when sidechaining is detected

// Option B: Reorder bus processing to respect dependencies
// Process buses in dependency order (sidechain sources first)
```

**Expected gain:** 10-20% reduction in mixer overhead

---

### Issue 2: Linear Track Filtering ⚠️ MEDIUM IMPACT
**Location:** Lines 958-963

**Problem:**
```rust
for track_output in &self.track_outputs {
    if track_output.bus_id == bus_id {  // O(tracks) per bus!
        bus_left += track_output.left;
        bus_right += track_output.right;
    }
}
```

**Why it's slow:**
- With 100 tracks and 10 buses, this is 1,000 comparisons per sample
- Not cache-friendly (iterates through all tracks for each bus)

**Optimization:**
```rust
// Pre-group tracks by bus during PASS 1
// Use Vec<Vec<TrackOutput>> indexed by bus_id

struct BusAccumulator {
    left: f32,
    right: f32,
    track_outputs: Vec<TrackOutput>,  // Only this bus's tracks
}

// Then in PASS 2:
for bus in &mut buses {
    let (left, right) = bus_accumulator[bus.id].sum();  // Direct access!
}
```

**Expected gain:** 5-15% reduction with many tracks

---

### Issue 3: Pan Calculation Redundancy ⚠️ LOW IMPACT
**Location:** Lines 998-1002

**Problem:**
```rust
// Recalculated every sample even though bus.pan doesn't change!
let pan_left = if bus.pan <= 0.0 { 1.0 } else { 1.0 - bus.pan };
let pan_right = if bus.pan >= 0.0 { 1.0 } else { 1.0 + bus.pan };
```

**Optimization:**
```rust
// Cache pan gains in Bus struct
struct Bus {
    // ...
    cached_pan_left: f32,
    cached_pan_right: f32,
}

// Update cache when pan changes
impl Bus {
    fn set_pan(&mut self, pan: f32) {
        self.pan = pan;
        self.cached_pan_left = if pan <= 0.0 { 1.0 } else { 1.0 - pan };
        self.cached_pan_right = if pan >= 0.0 { 1.0 } else { 1.0 + pan };
    }
}
```

**Expected gain:** 1-2% (small but easy win)

---

## Hot Path: `process_track_static()` (lines 1672-1790)

### Issue 4: Polyphonic Frequency Loop Not Vectorized ⚠️ HIGH IMPACT
**Location:** Lines 1713-1738

**Problem:**
```rust
// Processes up to 16 frequencies SEQUENTIALLY
for i in 0..note_event.num_freqs {
    let base_freq = note_event.frequencies[i];
    // ... expensive calculations per frequency
    let sample = waveform.sample(phase);
    track_value += sample * envelope_amp;
}
```

**Why it's slow:**
- Modern CPUs can process 4-8 floats simultaneously with SIMD
- This is the innermost loop of synthesis - runs millions of times

**Optimization:**
```rust
// Use your existing SIMD module!
use crate::synthesis::simd::SIMD;

// Process 8 frequencies at once
let mut freq_chunks = note_event.frequencies[..note_event.num_freqs]
    .chunks_exact(8);

for freq_chunk in freq_chunks {
    // SIMD: compute 8 phases simultaneously
    let phases = SIMD.compute_phases(freq_chunk, time_in_note);

    // SIMD: sample 8 waveforms simultaneously
    let samples = SIMD.sample_waveforms(&phases, note_event.waveform);

    // SIMD: accumulate
    track_value += SIMD.horizontal_sum(&samples) * envelope_amp;
}

// Handle remainder (< 8 frequencies) with scalar code
```

**Expected gain:** 3-6x speedup for polyphonic synthesis (chords)

---

### Issue 5: Expensive `powf()` in Pitch Bend ⚠️ MEDIUM IMPACT
**Location:** Lines 1716-1720

**Problem:**
```rust
let bend_multiplier = 2.0f32.powf(
    (note_event.pitch_bend_semitones * bend_progress) / 12.0
);
```

**Why it's slow:**
- `powf()` is very expensive (~20-30 cycles)
- Called for every frequency, every sample

**Optimization A: Fast approximation**
```rust
// Use fast pow2 approximation (your SIMD module might have this)
let exponent = (note_event.pitch_bend_semitones * bend_progress) / 12.0;
let bend_multiplier = fast_pow2(exponent);

// OR use lookup table for common semitone values
static SEMITONE_TABLE: [f32; 25] = [
    // Pre-computed 2^(n/12) for n = -12..12
    0.5, 0.529, 0.561, ..., 2.0
];
```

**Optimization B: Cache when bend_progress doesn't change**
```rust
// If using block processing, bend multiplier is constant across the block
// Compute once outside the sample loop
```

**Expected gain:** 5-10% for pitch-bent notes

---

### Issue 6: Modulo for Phase Calculation ⚠️ LOW IMPACT
**Location:** Lines 1730, 1734

**Problem:**
```rust
let phase = (time_in_note * freq) % 1.0;  // Modulo is slow
```

**Optimization:**
```rust
// If you can guarantee phase accumulation (in block processing):
// Use fractional part instead
let phase = (time_in_note * freq).fract();  // Faster than %

// OR track phase incrementally (even faster)
phase += freq / sample_rate;
if phase >= 1.0 { phase -= 1.0; }  // Simple comparison, no division
```

**Expected gain:** 2-3% (minor but common operation)

---

### Issue 7: Sample Event Stereo→Mono Conversion ⚠️ LOW IMPACT
**Location:** Line 1760

**Problem:**
```rust
// Computed every sample even though it could be pre-averaged
track_value += (sample_left + sample_right) * 0.5 * sample_event.volume;
```

**Optimization:**
```rust
// Pre-compute the mono value in sample_at_interpolated
// OR keep track stereo and do panning properly
// OR cache volume multiplier
let volume_scale = sample_event.volume * 0.5;
track_value += (sample_left + sample_right) * volume_scale;
```

**Expected gain:** <1% (minimal, but cleaner)

---

## Block Processing is Already Optimized ✅

The `process_block()` method (lines 1045+) is **already well-optimized**:
- ✅ Uses Rayon for parallel bus processing
- ✅ Uses SIMD for envelope calculations (line 1119)
- ✅ Uses SIMD for stereo mixing (line 1139)
- ✅ Pre-allocates buffers (line 1087)

**This is your fast path!** Make sure users call this instead of sample_at() for real-time audio.

---

## Recommended Priority

### High Priority (Do These First):
1. **Vectorize polyphonic frequency loop** → 3-6x speedup on chords
2. **Single-pass bus processing** (when no sidechaining) → 10-20% overall
3. **Fast pow2 for pitch bend** → 5-10% for pitch-bent notes

### Medium Priority (Easy Wins):
4. **Pre-group tracks by bus** → 5-15% with many tracks
5. **Cache bus pan gains** → 1-2% free performance

### Low Priority (Micro-opts):
6. **Use fract() instead of % for phase** → 2-3%
7. **Pre-compute sample volume scale** → <1%

---

## Profiling Recommendations

Before optimizing, **profile real-world usage**:

```bash
# Profile with a complex composition (100+ tracks, chords, effects)
cargo build --release
perf record --call-graph=dwarf ./target/release/benchmark
perf report

# Or use cargo-flamegraph
cargo install flamegraph
cargo flamegraph --bin benchmark
```

**Look for:**
- Time spent in `sample_at()` vs `process_block()`
- Time spent in frequency loop (if high → vectorize first)
- Time spent in bus iteration (if high → optimize bus grouping)

---

## Notes

**Don't over-optimize:**
- Your block processing is already excellent (SIMD + Rayon)
- `sample_at()` is probably only used for previews/simple playback
- Focus optimization effort where users actually spend time

**Keep the architecture:**
- Integer IDs are great
- Pre-allocated buffers are great
- Separation of concerns is great

**Consider:**
- Exposing `process_block()` as the primary API
- Deprecating `sample_at()` for performance-critical code
- Adding a "compile-time" option to skip sidechaining checks

---

## Estimated Total Gain

If you implement the **high-priority optimizations**:
- **Chords/polyphony:** 3-6x faster
- **Overall mixer:** 15-30% faster
- **Pitch bends:** 5-10% faster for those notes

**But remember:** Your `process_block()` with SIMD is already at ~100x realtime. These optimizations matter most for:
1. Very complex compositions (500+ concurrent notes)
2. Real-time synthesis on weak CPUs (integrated graphics machines)
3. Embedded/WASM targets

---

## Questions to Answer (via profiling)

1. **What % of time is spent in `sample_at()` vs `process_block()`?**
   - If <10% in `sample_at()`, don't bother optimizing it

2. **How many tracks have >1 frequency (polyphony)?**
   - If rare, vectorizing frequency loop won't help much

3. **Do buses have sidechaining enabled?**
   - If no, single-pass processing is a huge win
   - If yes, two-pass is necessary

4. **Are there bus routing bottlenecks?**
   - Profile the track→bus grouping loop

---

**Recommendation:** Run benchmarks with `criterion`, profile with `perf`, then tackle the high-priority items. Your architecture is already solid! 🚀
