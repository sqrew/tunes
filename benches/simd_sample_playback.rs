use tunes::prelude::*;
use std::time::Instant;

/// Benchmark SIMD-accelerated sample playback
///
/// This example demonstrates the performance benefits of SIMD sample playback
/// by playing many concurrent samples. SIMD processes 4-8 samples at once,
/// making it significantly faster for sample-heavy applications (games, music production).

fn main() -> anyhow::Result<()> {
    println!("\n🚀 SIMD Sample Playback Benchmark\n");

    // Create a test sample (short kick drum sound)
    println!("Creating test samples...");
    let mut comp_setup = Composition::new(Tempo::new(120.0));
    comp_setup.track("kick").at(0.0).drum(DrumType::Kick808);
    let mut mixer_temp = comp_setup.into_mixer();

    // Export to WAV for reloading as sample
    mixer_temp.export_wav("bench_kick.wav", 44100)?;

    let sample = Sample::from_file("bench_kick.wav")?;
    println!("  ✓ Sample created ({} channels, {}Hz)\n", sample.channels, sample.sample_rate);

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
    println!("  SIMD: {} ({} lanes)", simd_name, simd_width);
    println!("  Expected speedup: {}x for sample-heavy workloads\n", simd_width);

    // Benchmark: Many concurrent samples
    println!("=== Benchmark: 50 Concurrent Samples ===");
    let mut comp = Composition::new(Tempo::new(120.0));

    // Add 50 samples playing concurrently at different times/pitches
    for i in 0..50 {
        let start_time = (i as f32 * 0.02) % 2.0; // Stagger starts
        let pitch = 0.8 + (i as f32 * 0.01); // Vary pitch

        comp.track(&format!("sample_{}", i))
            .at(start_time)
            .play_sample(&sample, pitch);
    }

    let engine = AudioEngine::new()?;
    let mut mixer = comp.into_mixer();

    println!("  50 samples configured");
    println!("  Total duration: {:.2}s", mixer.total_duration());
    println!("  Rendering with SIMD sample playback...\n");

    // Time the rendering
    let start = Instant::now();
    let buffer = engine.render_to_buffer(&mut mixer);
    let render_time = start.elapsed();

    let sample_count = buffer.len() / 2; // Stereo
    let duration = sample_count as f32 / 44100.0;
    let realtime_ratio = duration / render_time.as_secs_f32();

    println!("=== Results ===");
    println!("  Rendered: {:.2}s of audio", duration);
    println!("  Render time: {:.3}s", render_time.as_secs_f32());
    println!("  Realtime ratio: {:.1}x realtime", realtime_ratio);
    println!("  Samples processed: {} ({} concurrent)", sample_count, 50);

    if realtime_ratio > 40.0 {
        println!("\n  ✅ Excellent! SIMD acceleration is working great!");
        println!("  Can process {} samples in parallel without lag!", (realtime_ratio as u32));
    } else if realtime_ratio > 20.0 {
        println!("\n  ✅ Good performance with SIMD!");
    } else {
        println!("\n  ⚠ Performance lower than expected");
    }

    println!("\n=== Playback Test ===");
    println!("Playing back rendered audio...\n");
    engine.play_mixer(&mixer)?;

    // Cleanup
    std::fs::remove_file("bench_kick.wav").ok();

    println!("\n✅ Benchmark complete!");
    println!("\nNote: SIMD sample playback processes {}-samples at once,", simd_width);
    println!("providing significant speedup for games and applications");
    println!("with many concurrent sound effects.\n");

    Ok(())
}
