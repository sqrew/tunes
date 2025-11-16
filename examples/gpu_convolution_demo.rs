/// GPU-Accelerated Convolution Reverb Demo
///
/// Demonstrates the performance improvement of GPU-accelerated convolution reverb.
/// Compares CPU and GPU processing times for the same reverb effect.
///
/// Run with: cargo run --example gpu_convolution_demo --features gpu --release

use tunes::prelude::*;
use tunes::synthesis::effects::convolution;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    println!("🚀 GPU-Accelerated Convolution Reverb Demo\n");

    println!("Creating test composition (arpeggios with convolution reverb)...");
    println!("Composition: ~4 seconds, 64 notes\n");

    // Variables to store timings
    let cpu_time;

    // === Test 1: CPU Convolution ===
    println!("=== Test 1: CPU Convolution (Cathedral) ===");
    println!("Note: Using cathedral preset (long IR ~500k samples)");
    println!("      GPU uses partitioned convolution to handle any IR size\n");
    {
        // Create composition with many notes to process
        let mut comp_cpu = Composition::new(Tempo::new(120.0));

        // Add multiple arpeggios to stress-test the convolution
        let pattern = [C4, E4, G4, B4, C5, B4, G4, E4];
        for i in 0..8 {
            let start_time = i as f32 * 0.5;
            for (j, &note) in pattern.iter().enumerate() {
                comp_cpu.instrument("piano", &Instrument::acoustic_piano())
                    .at(start_time + j as f32 * 0.125)
                    .note(&[note], 0.2);
            }
        }

        // Apply convolution reverb (CPU) - using cathedral (long IR)
        let reverb = convolution::presets::cathedral(0.7)?;
        comp_cpu.track("piano").convolution_reverb(reverb);

        let mut mixer = comp_cpu.into_mixer();

        println!("⏱️  Rendering with CPU convolution...");
        let start = Instant::now();
        mixer.export_wav("gpu_convolution_cpu.wav", 44100)?;
        cpu_time = start.elapsed();

        println!("   CPU render time: {:.3}s", cpu_time.as_secs_f32());
        println!("   Output: gpu_convolution_cpu.wav");
    }

    // === Test 2: GPU Convolution ===
    #[cfg(feature = "gpu")]
    {
        println!("\n=== Test 2: GPU Convolution (Cathedral) ===");

        // Create same composition for GPU test
        let mut comp_gpu = Composition::new(Tempo::new(120.0));
        let pattern = [C4, E4, G4, B4, C5, B4, G4, E4];
        for i in 0..8 {
            let start_time = i as f32 * 0.5;
            for (j, &note) in pattern.iter().enumerate() {
                comp_gpu.instrument("piano", &Instrument::acoustic_piano())
                    .at(start_time + j as f32 * 0.125)
                    .note(&[note], 0.2);
            }
        }

        // Apply convolution reverb and enable GPU
        let mut reverb = convolution::presets::cathedral(0.7)?;

        print!("🎮 Initializing GPU...");
        match reverb.enable_gpu() {
            Ok(_) => println!(" ✅ GPU enabled!"),
            Err(e) => {
                println!(" ❌ GPU initialization failed: {}", e);
                println!("   Falling back to CPU");
            }
        }

        comp_gpu.track("piano").convolution_reverb(reverb);

        let mut mixer = comp_gpu.into_mixer();

        println!("⏱️  Rendering with GPU convolution...");
        let start = Instant::now();
        mixer.export_wav("gpu_convolution_gpu.wav", 44100)?;
        let gpu_time = start.elapsed();

        println!("   GPU render time: {:.3}s", gpu_time.as_secs_f32());
        println!("   Output: gpu_convolution_gpu.wav");

        // Calculate speedup
        let speedup = cpu_time.as_secs_f32() / gpu_time.as_secs_f32();
        println!("\n📊 Performance Comparison:");
        println!("   CPU:  {:.3}s", cpu_time.as_secs_f32());
        println!("   GPU:  {:.3}s", gpu_time.as_secs_f32());
        println!("   Speedup: {:.2}x faster", speedup);

        if speedup > 1.0 {
            println!("\n🎯 GPU acceleration successful! {:.1}% faster", (speedup - 1.0) * 100.0);
        } else {
            println!("\n⚠️  Note: On integrated GPUs, speedup may be minimal.");
            println!("   Discrete GPUs typically show 5-50x speedups.");
        }

        println!("\n📝 GPU Features:");
        println!("   • Partitioned convolution: Handles any IR size (no limits!)");
        println!("   • Parallel processing: All partitions processed simultaneously");
        println!("   • Long reverbs (cathedral, concert hall) fully supported");
    }

    #[cfg(not(feature = "gpu"))]
    {
        println!("\n=== Test 2: GPU Convolution - SKIPPED ===");
        println!("   GPU feature not enabled");
        println!("   Run with: cargo run --example gpu_convolution_demo --features gpu --release");
    }

    println!("\n✅ Demo complete!");

    Ok(())
}
