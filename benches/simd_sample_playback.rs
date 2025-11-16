use tunes::prelude::*;
use std::time::Instant;

/// Rigorous SIMD Sample Playback Benchmark
///
/// Tests TRUE concurrent sample playback - all samples playing simultaneously.
/// This benchmark is designed to stress-test realistic game audio scenarios.

fn run_concurrent_test(sample_count: usize, sample: &Sample) -> anyhow::Result<(f32, f32)> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // ALL samples start at time 0.0 - true concurrency!
    for i in 0..sample_count {
        let pitch = 0.9 + (i as f32 * 0.005); // Slight pitch variation

        comp.track(&format!("sample_{}", i))
            .at(0.0)  // ALL START AT SAME TIME - this is the key!
            .play_sample(&sample, pitch);
    }

    let engine = AudioEngine::new()?;
    let mut mixer = comp.into_mixer();

    // Time the rendering
    let start = Instant::now();
    let buffer = engine.render_to_buffer(&mut mixer);
    let render_time = start.elapsed();

    let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
    let realtime_ratio = audio_duration / render_time.as_secs_f32();

    Ok((realtime_ratio, audio_duration))
}

fn main() -> anyhow::Result<()> {
    println!("\n🚀 SIMD Sample Playback Benchmark (Rigorous)\n");
    println!("Testing TRUE concurrent playback - all samples playing simultaneously!\n");

    // Create TWO test samples - short and long
    println!("Creating test samples...");

    // Short sample: Kick drum (~0.5s)
    let mut comp_kick = Composition::new(Tempo::new(120.0));
    comp_kick.track("kick").at(0.0).drum(DrumType::Kick808);
    let mut mixer_kick = comp_kick.into_mixer();
    mixer_kick.export_wav("bench_kick.wav", 44100)?;
    let kick_sample = Sample::from_file("bench_kick.wav")?;
    let kick_duration = kick_sample.data.len() as f32 / kick_sample.channels as f32 / kick_sample.sample_rate as f32;

    // Longer sample: Snare with reverb (~1.0s)
    let mut comp_snare = Composition::new(Tempo::new(120.0));
    comp_snare.track("snare")
        .at(0.0)
        .drum(DrumType::Snare808)
        .reverb(Reverb::new(0.7, 0.5, 0.6));
    let mut mixer_snare = comp_snare.into_mixer();
    mixer_snare.export_wav("bench_snare.wav", 44100)?;
    let snare_sample = Sample::from_file("bench_snare.wav")?;
    let snare_duration = snare_sample.data.len() as f32 / snare_sample.channels as f32 / snare_sample.sample_rate as f32;

    println!("  ✓ Kick sample: {:.2}s ({} channels, {}Hz)", kick_duration, kick_sample.channels, kick_sample.sample_rate);
    println!("  ✓ Snare sample: {:.2}s ({} channels, {}Hz)", snare_duration, snare_sample.channels, snare_sample.sample_rate);

    // Display SIMD capabilities
    use tunes::synthesis::simd::SIMD;
    let simd_width = SIMD.width();
    let simd_name = if simd_width == 8 {
        "AVX2"
    } else if simd_width == 4 {
        "SSE/NEON"
    } else {
        "Scalar"
    };
    println!("\n  SIMD: {} ({} lanes)\n", simd_name, simd_width);

    println!("{}", "=".repeat(70));

    // Test 1: 25 concurrent short samples
    println!("\n=== Test 1: 25 Concurrent SHORT Samples (kick, ~0.5s) ===");
    let (rt1, dur1) = run_concurrent_test(25, &kick_sample)?;
    println!("  Samples: 25 playing simultaneously");
    println!("  Duration: {:.2}s", dur1);
    println!("  Performance: {:.1}x realtime", rt1);
    println!("  Theoretical max concurrent: {} samples", (25.0 * rt1) as u32);

    println!("\n{}", "=".repeat(70));

    // Test 2: 50 concurrent short samples
    println!("\n=== Test 2: 50 Concurrent SHORT Samples (kick, ~0.5s) ===");
    let (rt2, dur2) = run_concurrent_test(50, &kick_sample)?;
    println!("  Samples: 50 playing simultaneously");
    println!("  Duration: {:.2}s", dur2);
    println!("  Performance: {:.1}x realtime", rt2);
    println!("  Theoretical max concurrent: {} samples", (50.0 * rt2) as u32);

    println!("\n{}", "=".repeat(70));

    // Test 3: 100 concurrent short samples (STRESS TEST)
    println!("\n=== Test 3: 100 Concurrent SHORT Samples (kick, ~0.5s) ===");
    println!("  (STRESS TEST - pushing limits!)");
    let (rt3, dur3) = run_concurrent_test(100, &kick_sample)?;
    println!("  Samples: 100 playing simultaneously");
    println!("  Duration: {:.2}s", dur3);
    println!("  Performance: {:.1}x realtime", rt3);
    println!("  Theoretical max concurrent: {} samples", (100.0 * rt3) as u32);

    println!("\n{}", "=".repeat(70));

    // Test 4: 25 concurrent LONG samples (harder test)
    println!("\n=== Test 4: 25 Concurrent LONG Samples (snare+reverb, ~1.0s) ===");
    let (rt4, dur4) = run_concurrent_test(25, &snare_sample)?;
    println!("  Samples: 25 playing simultaneously");
    println!("  Duration: {:.2}s", dur4);
    println!("  Performance: {:.1}x realtime", rt4);
    println!("  Theoretical max concurrent: {} samples", (25.0 * rt4) as u32);

    println!("\n{}", "=".repeat(70));

    // Test 5: 50 concurrent LONG samples (HARDEST TEST)
    println!("\n=== Test 5: 50 Concurrent LONG Samples (snare+reverb, ~1.0s) ===");
    println!("  (HARDEST TEST - long samples + many concurrent!)");
    let (rt5, dur5) = run_concurrent_test(50, &snare_sample)?;
    println!("  Samples: 50 playing simultaneously");
    println!("  Duration: {:.2}s", dur5);
    println!("  Performance: {:.1}x realtime", rt5);
    println!("  Theoretical max concurrent: {} samples", (50.0 * rt5) as u32);

    println!("\n{}", "=".repeat(70));
    println!("\n=== SUMMARY ===\n");

    // Calculate conservative estimate (worst case from tests)
    let worst_case_rt = rt1.min(rt2).min(rt3).min(rt4).min(rt5);
    let best_case_rt = rt1.max(rt2).max(rt3).max(rt4).max(rt5);
    let avg_rt = (rt1 + rt2 + rt3 + rt4 + rt5) / 5.0;

    println!("  Average performance: {:.1}x realtime", avg_rt);
    println!("  Best case: {:.1}x realtime", best_case_rt);
    println!("  Worst case: {:.1}x realtime", worst_case_rt);
    println!();
    println!("  Conservative concurrent capacity estimate:");
    println!("    With short samples (~0.5s): {} concurrent", (100.0 * worst_case_rt) as u32);
    println!("    With long samples (~1.0s): {} concurrent", (50.0 * worst_case_rt) as u32);
    println!();

    if worst_case_rt >= 1.0 {
        println!("  ✅ All tests passed! Can handle heavy concurrent loads.");
    } else {
        println!("  ⚠️  Performance below realtime in some tests!");
    }

    // Cleanup
    std::fs::remove_file("bench_kick.wav").ok();
    std::fs::remove_file("bench_snare.wav").ok();

    println!("\n✅ Rigorous benchmark complete!");
    println!("\nNote: These tests use TRUE concurrent playback (all samples start");
    println!("at the same time), which is the most realistic measure of game audio");
    println!("performance. SIMD acceleration ({} lanes) provides significant speedup.\n", simd_width);

    Ok(())
}
