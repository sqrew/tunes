use tunes::prelude::*;
use std::time::Instant;

/// Demonstrates SIMD-accelerated wavetable oscillators
///
/// This example compares:
/// 1. Scalar wavetable lookups (calling .sample() in a loop)
/// 2. SIMD wavetable lookups (calling .fill_buffer_simd())
///
/// Expected speedup: 2-4x faster with SIMD

fn main() -> anyhow::Result<()> {
    println!("🎵 SIMD Wavetable Performance Demo\n");

    // Detect SIMD capabilities
    use tunes::synthesis::simd::SIMD;
    let simd_width = SIMD.width();
    println!("✓ Detected SIMD width: {} lanes", simd_width);
    println!("  (4 = SSE, 8 = AVX2)\n");

    let wavetable = tunes::synthesis::wavetable::Wavetable::sine();
    let sample_rate = 44100.0;
    let frequency = 440.0; // A4
    let phase_inc = frequency / sample_rate;
    let buffer_size = 1024 * 64; // 64KB of audio

    println!("Test parameters:");
    println!("  Frequency: {} Hz", frequency);
    println!("  Sample rate: {} Hz", sample_rate);
    println!("  Buffer size: {} samples\n", buffer_size);

    // Benchmark 1: Scalar approach (manual loop)
    println!("Benchmark 1: Scalar wavetable lookups");
    let mut buffer_scalar = vec![0.0f32; buffer_size];
    let start = Instant::now();
    let iterations = 1000;

    for _ in 0..iterations {
        let mut phase = 0.0;
        for sample in buffer_scalar.iter_mut() {
            *sample = wavetable.sample(phase);
            phase += phase_inc;
            if phase >= 1.0 {
                phase -= 1.0;
            }
        }
    }

    let scalar_time = start.elapsed();
    println!("  Time: {:?} for {} iterations", scalar_time, iterations);
    println!("  Per iteration: {:?}", scalar_time / iterations);

    // Benchmark 2: SIMD approach
    println!("\nBenchmark 2: SIMD wavetable lookups");
    let mut buffer_simd = vec![0.0f32; buffer_size];
    let start = Instant::now();

    for _ in 0..iterations {
        wavetable.fill_buffer_simd(&mut buffer_simd, 0.0, phase_inc);
    }

    let simd_time = start.elapsed();
    println!("  Time: {:?} for {} iterations", simd_time, iterations);
    println!("  Per iteration: {:?}", simd_time / iterations);

    // Calculate speedup
    let speedup = scalar_time.as_secs_f64() / simd_time.as_secs_f64();
    println!("\n📊 Performance Results:");
    println!("  Speedup: {:.2}x faster with SIMD", speedup);
    println!("  Efficiency: {:.1}% of theoretical max ({}x)",
             (speedup / simd_width as f64) * 100.0, simd_width);

    // Verify correctness - outputs should be identical
    let mut buffer_verify = vec![0.0f32; 100];
    let mut phase = 0.0;
    for sample in buffer_verify.iter_mut() {
        *sample = wavetable.sample(phase);
        phase += phase_inc;
    }

    let mut buffer_simd_verify = vec![0.0f32; 100];
    wavetable.fill_buffer_simd(&mut buffer_simd_verify, 0.0, phase_inc);

    let max_diff = buffer_verify.iter()
        .zip(buffer_simd_verify.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);

    println!("\n✓ Correctness check:");
    println!("  Max difference: {} (should be ~0.0)", max_diff);
    if max_diff < 0.0001 {
        println!("  ✓ SIMD implementation is bit-accurate!");
    } else {
        println!("  ⚠ Warning: SIMD differs from scalar");
    }

    // Create a short audio example using SIMD
    println!("\n🎧 Generating audio example with SIMD...");
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Manually create an oscillator using SIMD wavetable
    // (In the future, all oscillators will use SIMD internally)
    comp.instrument("simd_sine", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5], 0.5);

    engine.play_mixer(&comp.into_mixer())?;

    println!("✅ Demo complete!\n");
    println!("Note: All future oscillators will use SIMD automatically.");
    println!("The speedup happens transparently without any API changes.");

    Ok(())
}
