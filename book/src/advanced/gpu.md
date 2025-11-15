# GPU Acceleration Guide

This guide provides a comprehensive overview of GPU acceleration in Tunes - what it does, where it's used, why it exists, when to use it, and how to enable it.

## Table of Contents

1. [Overview](#overview)
2. [What GPU Acceleration Does](#what-gpu-acceleration-does)
3. [Where It's Used](#where-its-used)
4. [Why Use GPU Acceleration](#why-use-gpu-acceleration)
5. [When to Use GPU](#when-to-use-gpu)
6. [How to Enable GPU](#how-to-enable-gpu)
7. [Technical Architecture](#technical-architecture)
8. [Performance Characteristics](#performance-characteristics)
9. [Benchmarking](#benchmarking)
10. [Troubleshooting](#troubleshooting)

---

## Overview

GPU acceleration in Tunes uses compute shaders to synthesize audio on the graphics card instead of the CPU. This is implemented using wgpu (WebGPU) and WGSL (WebGPU Shading Language) compute shaders.

**Key Points:**
- Optional feature (requires `gpu` feature flag)
- Transparent API integration
- Automatic hardware detection
- Cross-platform (Vulkan, Metal, DX12, WebGPU)
- Automatic CPU fallback

**Performance Reality:**
- Integrated GPUs: 1.0-1.2x speedup (marginal benefit)
- Discrete GPUs: Performance scales with hardware capabilities
- CPU baseline: 70x realtime (already fast)

---

## What GPU Acceleration Does

GPU acceleration moves the synthesis workload from CPU to GPU. Specifically:

### Synthesis on GPU

The GPU renders complete audio waveforms in parallel:

```text
Input: Waveform type, frequency, duration, ADSR envelope, FM parameters
       ↓
GPU Compute Shader (WGSL)
       ↓
Output: Complete audio buffer (f32 samples)
```

**What the GPU does:**
1. Generates waveforms (sine, saw, square, triangle)
2. Applies ADSR envelopes
3. Performs FM synthesis (modulator + carrier)
4. Handles velocity and pitch bend
5. Outputs complete rendered audio

**What remains on CPU:**
- Effects (reverb, delay, filters)
- Mixing multiple tracks
- Sample playback
- Real-time parameter changes

### The Two-Stage Pipeline

GPU acceleration enables a two-stage performance pipeline:

**Stage 1: Synthesis → Export**
```rust
let engine = AudioEngine::new_with_gpu()?;
engine.export_wav(&mut comp.into_mixer(), "complex_sound.wav")?;
// GPU renders synthesis 1.0-1.2x faster (integrated) or more (discrete)
```

**Stage 2: Sample Playback**
```rust
engine.play_sample("complex_sound.wav")?;
// Already fast on CPU, but offloads work
```

This makes unlimited complexity synthesis feel real-time by pre-rendering to cache.

---

## Where It's Used

GPU acceleration automatically activates in three scenarios when enabled:

### 1. Export Operations

```rust
let engine = AudioEngine::new_with_gpu()?;

// WAV export uses GPU
engine.export_wav(&mut mixer, "output.wav")?;

// FLAC export uses GPU
engine.export_flac(&mut mixer, "output.flac")?;
```

**What happens:**
- Mixer automatically enables GPU if engine has `enable_gpu_for_samples` flag
- Synthesis operations run on GPU
- Export to disk happens on CPU (IO operation)

### 2. Real-Time Playback

```rust
let engine = AudioEngine::new_with_gpu()?;

// Real-time playback uses GPU
engine.play_mixer_realtime(&mixer)?;

// Looping playback uses GPU
engine.play_looping(&mixer)?;
```

**What happens:**
- Mixer is cloned and GPU-enabled automatically
- Synthesis runs on GPU
- Mixing and output happen on CPU

### 3. Sample Playback (Manual)

```rust
let engine = AudioEngine::new_with_gpu()?;

// play_sample() uses GPU when engine was created with GPU support
engine.play_sample("laser.wav")?;
```

**What happens:**
- Composition created internally
- Mixer has GPU auto-enabled
- Sample playback happens

---

## Why Use GPU Acceleration

### The Case For GPU

**Resource Distribution:**
- Integrated GPUs use fraction of system power to match CPU performance
- Offloads synthesis work from CPU to GPU
- Frees CPU for game logic, physics, AI

**Scaling Potential:**
- Discrete GPUs have 100-150x more compute units than integrated
- Performance scales with hardware capabilities
- Future-proof architecture

**Parallel Architecture:**
- GPUs excel at parallel workload (thousands of audio samples)
- SIMD-like operations at massive scale
- Natural fit for synthesis algorithms

### The Case Against GPU

**Reality Check:**
- CPU performance is already excellent (70x realtime)
- Integrated GPUs show minimal improvement (1.0-1.2x)
- Adds complexity and dependencies

**When CPU is Better:**
- Simple synthesis (CPU is faster)
- Small workloads (GPU overhead not worth it)
- Integrated graphics (CPU optimizations beat GPU)

**Overhead Costs:**
- GPU initialization time
- Data transfer (CPU ↔ GPU)
- Cache management complexity

### Honest Assessment

For most users, CPU synthesis is sufficient. GPU acceleration is:
- **Not necessary** for typical use cases
- **Experimental** on discrete hardware
- **Marginal benefit** on integrated graphics
- **Future-looking** for scaling potential

---

## When to Use GPU

### Hardware Considerations

**Integrated GPUs (Intel HD, AMD Vega integrated):**
```text
Performance: 1.0-1.2x vs CPU
Recommendation: Skip GPU, use CPU
Reason: Marginal benefit, added complexity
```

**Discrete GPUs (RTX 30/40 series, RX 6000/7000 series):**
```text
Performance: Scales with compute units and memory bandwidth
Recommendation: Experiment with GPU feature
Reason: Potential for significant speedup
```

**No GPU / Old GPUs:**
```text
Performance: N/A (automatic CPU fallback)
Recommendation: CPU only
Reason: No GPU available
```

### Workload Considerations

**Simple Synthesis (sine/square waves, basic ADSR):**
- **CPU:** 70x realtime
- **GPU (integrated):** 15-20x realtime (slower)
- **Recommendation:** CPU only

**Complex Synthesis (FM, multiple oscillators, long envelopes):**
- **CPU:** 18-22x realtime
- **GPU (integrated):** 15-20x realtime (similar)
- **GPU (discrete):** Potentially faster
- **Recommendation:** Test both

**Export Workflows (batch rendering many files):**
- **CPU:** 7.6x realtime (WAV), 4.1x realtime (FLAC)
- **GPU (integrated):** 8.4x realtime (WAV, 1.1x speedup)
- **Recommendation:** Marginal benefit unless discrete GPU

### Use Case Matrix

| Use Case | Integrated GPU | Discrete GPU | CPU Only |
|----------|----------------|--------------|----------|
| Game audio (simple) | ❌ Slower | ✅ Experiment | ✅ Recommended |
| Game audio (complex) | ⚠️ Marginal | ✅ Test it | ✅ Good |
| Music production | ❌ Skip | ⚠️ Maybe | ✅ Recommended |
| Batch export | ⚠️ Marginal (1.1x) | ✅ Worth trying | ✅ Good |
| Live coding | ❌ Skip | ❌ Skip | ✅ Recommended |
| Web/mobile | ❌ N/A | ❌ N/A | ✅ Only option |

---

## How to Enable GPU

### Prerequisites

**1. Enable the `gpu` feature in Cargo.toml:**
```toml
[dependencies]
tunes = { version = "0.16.0", features = ["gpu"] }
```

**2. Ensure wgpu-compatible graphics drivers:**
- **Windows:** DirectX 12 drivers
- **macOS:** Metal (automatic)
- **Linux:** Vulkan drivers
  ```bash
  # Check Vulkan support
  vulkaninfo | head -20
  ```

### Method 1: Transparent API (Recommended)

The simplest way to enable GPU:

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    // Create engine with GPU enabled
    let engine = AudioEngine::new_with_gpu()?;

    // All operations automatically use GPU when possible
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("lead").sine(440.0, 1.0);

    // Export automatically uses GPU
    engine.export_wav(&mut comp.into_mixer(), "output.wav")?;

    // Playback automatically uses GPU
    engine.play_mixer_realtime(&comp.into_mixer())?;

    Ok(())
}
```

**What happens:**
- `AudioEngine::new_with_gpu()` sets internal `enable_gpu_for_samples` flag
- All `export_*()` methods check flag and auto-enable GPU on mixer
- All `play_*()` methods check flag and auto-enable GPU on mixer clone
- No manual `enable_gpu()` calls needed

### Method 2: Manual Control

For fine-grained control:

```rust
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new()?;  // Standard engine
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("lead").sine(440.0, 1.0);

    let mut mixer = comp.into_mixer();

    // Enable caching (required for GPU)
    mixer.enable_cache();

    // Explicitly enable GPU
    mixer.enable_gpu();

    // Check if GPU is actually enabled
    if mixer.gpu_enabled() {
        println!("GPU acceleration active!");
    }

    // Render with GPU
    let buffer = engine.render_to_buffer(&mut mixer);

    // Disable GPU for next render
    mixer.disable_gpu();

    Ok(())
}
```

**Use manual control when:**
- Need to enable/disable GPU dynamically
- Want to compare CPU vs GPU performance
- Debugging or profiling

### Method 3: Conditional Compilation

For code that works with and without GPU:

```rust
#[cfg(feature = "gpu")]
use gpu_enabled_code;

fn create_engine() -> Result<AudioEngine, anyhow::Error> {
    #[cfg(feature = "gpu")]
    {
        println!("GPU feature enabled, using GPU acceleration");
        AudioEngine::new_with_gpu()
    }

    #[cfg(not(feature = "gpu"))]
    {
        println!("GPU feature disabled, using CPU");
        AudioEngine::new()
    }
}
```

---

## Technical Architecture

### Overview

```text
┌─────────────────────────────────────────────────────┐
│                  Tunes Library                      │
├─────────────────────────────────────────────────────┤
│  Composition → Mixer → GPU Synthesis → Cache        │
│                  ↓                        ↓          │
│              enable_gpu()            Arc<Sample>    │
├─────────────────────────────────────────────────────┤
│                   wgpu Layer                        │
├─────────────────────────────────────────────────────┤
│   GpuDevice → GpuSynthesizer → WGSL Compute Shader │
├─────────────────────────────────────────────────────┤
│        Vulkan / Metal / DX12 / WebGPU              │
└─────────────────────────────────────────────────────┘
```

### Components

**1. GpuDevice (src/gpu/device.rs)**

Manages wgpu device and queue:

```rust
pub struct GpuDevice {
    device: Arc<Device>,
    queue: Arc<Queue>,
    adapter_info: AdapterInfo,
}
```

**Responsibilities:**
- Initialize wgpu instance
- Select GPU adapter (integrated vs discrete detection)
- Create logical device and command queue
- Provide device info for warnings

**2. GpuSynthesizer (src/gpu/synthesis.rs)**

Manages compute pipeline and synthesis:

```rust
pub struct GpuSynthesizer {
    device: Arc<GpuDevice>,
    pipeline: ComputePipeline,
    bind_group_layout: BindGroupLayout,
}
```

**Responsibilities:**
- Load and compile WGSL compute shader
- Create compute pipeline
- Manage GPU buffers (params, output)
- Execute synthesis on GPU
- Download results to CPU

**3. WGSL Compute Shader (src/gpu/synthesis.wgsl)**

The actual GPU code:

```wgsl
@group(0) @binding(0) var<storage, read> params: NoteParams;
@group(0) @binding(1) var<storage, read_write> output: array<f32>;

@compute @workgroup_size(256)
fn synthesize(@builtin(global_invocation_id) id: vec3<u32>) {
    let sample_idx = id.x;

    // Generate waveform
    let phase = params.frequency * f32(sample_idx) / params.sample_rate;
    var sample = generate_waveform(phase, params.waveform_type);

    // Apply ADSR envelope
    sample = sample * calculate_envelope(sample_idx, params);

    // Write to output buffer
    output[sample_idx] = sample;
}
```

**GPU Execution:**
- Dispatched with workgroups of 256 threads
- Each thread processes one audio sample
- Fully parallel execution across thousands of samples

### Data Flow

**1. Synthesis Request:**
```rust
mixer.enable_gpu();  // Sets up GPU synthesizer
```

**2. Pre-rendering (Before Playback):**
```rust
// Scans all tracks for unique note parameters
let unique_notes = find_unique_notes(tracks);

// For each unique note:
for note in unique_notes {
    // 1. Upload parameters to GPU
    let param_buffer = gpu.create_buffer(note.params);

    // 2. Execute compute shader
    gpu.dispatch_compute(workgroups);

    // 3. Download results
    let samples = gpu.read_buffer_async().await;

    // 4. Store in cache
    cache.insert(note.hash(), samples);
}
```

**3. Playback (Real-Time):**
```rust
// Look up pre-rendered samples from cache
let samples = cache.get(note.hash());

// Mix into output buffer (CPU, zero-copy via Arc)
mix_samples(output, samples, volume, pan);
```

### GPU vs CPU Path

**CPU Path (Default):**
```text
Note Event → Oscillator (CPU) → ADSR (CPU) → Buffer → Mix
```

**GPU Path (Enabled):**
```text
Note Event → GPU Upload → Compute Shader → Download → Cache → Buffer → Mix
                   ↑
            (One-time cost)
```

**Key Difference:**
- CPU: Synthesize every time note plays
- GPU: Synthesize once, cache result, reuse millions of times

---

## Performance Characteristics

### Measured Performance

**Test System:**
- CPU: Intel i5-6500 @ 3.2GHz (4 cores, 2013 architecture)
- GPU: Intel HD Graphics 530 (integrated, 24 compute units)
- RAM: 16 GB DDR4
- OS: Linux (Manjaro)

**Benchmark Results:**

| Scenario | CPU Performance | GPU Performance | Speedup |
|----------|-----------------|-----------------|---------|
| Uncached synthesis | 70.2x realtime | 15.9x realtime | 0.23x (slower) |
| Cached synthesis | 18.3x realtime | 15.9x realtime | 0.87x (slower) |
| WAV export (30s) | 7.6x realtime | 8.4x realtime | 1.1x (faster) |
| Simple patterns | 73.9x realtime (CPU baseline) | 73.9x realtime | 1.0x (same) |

**Key Findings:**
1. Integrated GPU provides marginal benefit (1.0-1.2x)
2. CPU optimizations (SIMD + Rayon) are highly effective
3. Cache overhead dominates on small workloads
4. GPU shows slight advantage only in export scenarios

### Hardware Scaling

**Intel HD 530 (Measured):**
- Compute units: 24
- Performance: 1.0-1.2x vs CPU
- Recommendation: Use CPU

**Expected Discrete GPU Performance:**

| GPU | Compute Units | Expected Scaling |
|-----|---------------|------------------|
| RTX 3060 | 3584 | 150x more than HD 530 |
| RTX 4070 | 5888 | 245x more than HD 530 |
| RX 6700 XT | 2560 | 107x more than HD 530 |

**Note:** These are theoretical maximums based on compute unit count. Actual performance depends on memory bandwidth, shader occupancy, and workload characteristics.

### Performance Trade-offs

**GPU Overhead:**
- Initialization: ~100-200ms (one-time)
- Parameter upload: ~1-5μs per note
- Compute dispatch: ~10-50μs
- Download results: ~50-200μs per note
- Total per-note overhead: ~60-250μs

**When Overhead Dominates (GPU Slower):**
- Simple waveforms (< 1000 samples)
- Few unique notes (< 10)
- Integrated graphics

**When GPU Wins (GPU Faster):**
- Complex synthesis (FM, multiple oscillators)
- Many unique notes (> 100)
- Discrete graphics with high compute

### Memory Usage

**CPU Synthesis:**
- Stack allocation for oscillators
- Minimal heap (< 1 MB typically)

**GPU Synthesis:**
- Parameter buffers: ~256 bytes per note
- Output buffers: samples * 4 bytes (f32)
- VRAM usage: < 100 MB for typical workloads
- Cache: 500 MB default (configurable)

---

## Benchmarking

### Running Built-in Benchmarks

**1. GPU vs CPU comparison:**
```bash
cargo run --release --features gpu --bin gpu_benchmark
```

**Output:**
```
=== Test 1: CPU Synthesis (No Cache) ===
  Render time: 0.388s
  Realtime ratio: 70.2x

=== Test 2: CPU Synthesis + Cache ===
  Render time: 1.492s
  Realtime ratio: 18.3x

=== Test 3: GPU Synthesis + Cache ===
🎮 GPU Device: Intel(R) HD Graphics 530 (Vulkan)
   Type: Integrated
   ⚠️  Integrated GPU detected - may be slower than CPU synthesis
  Render time: 1.710s
  Realtime ratio: 15.9x

=== Test 4: Transparent GPU API with AudioEngine ===
  Export time: 0.369s
  Realtime ratio: 73.9x
```

**2. Export performance:**
```bash
cargo run --release --features gpu --bin export_speed
```

**3. Two-stage pipeline:**
```bash
cargo run --release --features gpu --bin pipeline_benchmark
```

### Custom Benchmarks

**Measure your workload:**

```rust
use std::time::Instant;
use tunes::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let engine = AudioEngine::new_with_gpu()?;

    let mut comp = Composition::new(Tempo::new(120.0));
    // ... build your composition ...

    let mut mixer = comp.into_mixer();
    let duration = mixer.total_duration();

    // Benchmark
    let start = Instant::now();
    let buffer = engine.render_to_buffer(&mut mixer);
    let elapsed = start.elapsed();

    let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
    let realtime_ratio = audio_duration / elapsed.as_secs_f32();

    println!("Duration: {:.1}s", duration);
    println!("Render time: {:.3}s", elapsed.as_secs_f32());
    println!("Realtime ratio: {:.1}x", realtime_ratio);

    Ok(())
}
```

**Compare CPU vs GPU:**

```rust
// CPU baseline
let engine_cpu = AudioEngine::new()?;
let mut mixer_cpu = comp.into_mixer();
let start = Instant::now();
let buffer_cpu = engine_cpu.render_to_buffer(&mut mixer_cpu);
let cpu_time = start.elapsed();

// GPU
let engine_gpu = AudioEngine::new_with_gpu()?;
let mut mixer_gpu = comp.into_mixer();
let start = Instant::now();
let buffer_gpu = engine_gpu.render_to_buffer(&mut mixer_gpu);
let gpu_time = start.elapsed();

let speedup = cpu_time.as_secs_f32() / gpu_time.as_secs_f32();
println!("GPU speedup: {:.2}x", speedup);
```

---

## Troubleshooting

### GPU Not Detected

**Problem:** GPU acceleration doesn't activate

**Solutions:**

1. **Check feature flag:**
   ```bash
   cargo build --features gpu
   ```

2. **Verify GPU drivers:**
   ```bash
   # Linux: Check Vulkan
   vulkaninfo | head -20

   # Windows: Ensure DirectX 12 drivers installed
   # macOS: Metal is automatic, ensure OS is up to date
   ```

3. **Check wgpu initialization:**
   ```rust
   mixer.enable_gpu();
   if !mixer.gpu_enabled() {
       println!("GPU failed to initialize - using CPU fallback");
   }
   ```

### Performance Slower Than CPU

**Problem:** GPU is slower than CPU

**Likely Causes:**

1. **Integrated GPU:**
   - Solution: Use CPU instead
   - Check: Look for "⚠️ Integrated GPU detected" warning

2. **Small workload:**
   - Solution: GPU overhead dominates, use CPU
   - Check: Fewer than 10 unique notes

3. **Simple synthesis:**
   - Solution: CPU SIMD is faster for simple waveforms
   - Check: Only sine/saw waves with basic ADSR

**Verification:**
```rust
// Disable GPU and compare
mixer.disable_gpu();
let cpu_buffer = engine.render_to_buffer(&mut mixer);

mixer.enable_gpu();
let gpu_buffer = engine.render_to_buffer(&mut mixer);

// Time both and compare
```

### Compilation Errors

**Problem:** `gpu` feature doesn't compile

**Solutions:**

1. **Update Rust:**
   ```bash
   rustup update stable
   ```

2. **Check wgpu version:**
   - Ensure `wgpu = "23.0"` in Cargo.toml

3. **Platform-specific issues:**
   - **Linux:** Install Vulkan development libraries
     ```bash
     # Debian/Ubuntu
     sudo apt install libvulkan-dev

     # Arch/Manjaro
     sudo pacman -S vulkan-headers
     ```
   - **Windows:** Ensure Windows 10 or later (DirectX 12)
   - **macOS:** Ensure macOS 10.14+ (Metal support)

### Cache Not Working

**Problem:** GPU synthesis happening every frame

**Solution:** Enable caching first

```rust
mixer.enable_cache();  // Must call before enable_gpu()
mixer.enable_gpu();
```

**Verify caching:**
```rust
mixer.print_cache_stats();
// Should show high hit rate (> 99%)
```

### Audio Artifacts

**Problem:** Clicks, pops, or distortion with GPU

**Likely Causes:**

1. **Buffer underrun** (GPU too slow for real-time)
   - Solution: Increase buffer size
     ```rust
     let engine = AudioEngine::with_buffer_size(8192)?;
     ```

2. **Precision issues** (rare)
   - Solution: Disable GPU for that specific composition

**Debugging:**
```rust
// Export both CPU and GPU renders
engine.export_wav(&mut mixer_cpu, "cpu.wav")?;
engine.export_wav(&mut mixer_gpu, "gpu.wav")?;

// Compare waveforms in audio editor
```

---

## Summary

**GPU acceleration in Tunes:**
- Provides transparent GPU compute shader synthesis
- Shows 1.0-1.2x improvement on integrated GPUs
- Scales with discrete GPU hardware
- Requires `gpu` feature flag
- Automatic CPU fallback

**Recommendations:**
- **Most users:** Use CPU (70x realtime is fast)
- **Discrete GPU owners:** Experiment with GPU feature
- **Integrated GPU users:** Skip GPU, CPU is faster
- **Benchmarking:** Always test your specific workload

**The transparent API makes it easy to try:**
```rust
let engine = AudioEngine::new_with_gpu()?;
// Everything else stays the same!
```

**Next Steps:**
- [Performance Guide](./performance.md) - Overall performance optimizations
- [Benchmarks](../../README.md#benchmarks) - Run benchmarks on your hardware
- [Examples](../../examples/) - GPU usage examples
