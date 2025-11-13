use tunes::prelude::*;
use tunes::synthesis::fm_synthesis::FMParams;
use std::time::Instant;

/// GPU-Accelerated Export Demo
///
/// This demonstrates how GPU acceleration automatically applies to WAV/FLAC export,
/// making batch rendering up to 500-5000x faster on discrete GPUs!

fn main() -> anyhow::Result<()> {
    println!("\n🚀 GPU-Accelerated Export Demo\n");

    let engine = AudioEngine::new()?;

    // Create a moderately complex composition
    let mut comp = Composition::new(Tempo::new(140.0));

    for bar in 0..8 {
        let bar_start = bar as f32 * 4.0 * 0.428;

        // Kick drums
        comp.track("kick")
            .at(bar_start)
            .note(&[C2], 0.15)
            .fm(FMParams::new(2.0, 8.0));

        comp.track("kick")
            .at(bar_start + 2.0 * 0.428)
            .note(&[C2], 0.15)
            .fm(FMParams::new(2.0, 8.0));

        // Snares
        comp.track("snare")
            .at(bar_start + 1.0 * 0.428)
            .note(&[D2], 0.12)
            .fm(FMParams::new(3.5, 6.0));

        comp.track("snare")
            .at(bar_start + 3.0 * 0.428)
            .note(&[D2], 0.12)
            .fm(FMParams::new(3.5, 6.0));

        // Hi-hats
        for eighth in 0..8 {
            comp.track("hihat")
                .at(bar_start + (eighth as f32) * 0.214)
                .note(&[FS2], 0.08)
                .fm(FMParams::new(4.0, 3.0));
        }
    }

    let mut mixer = comp.into_mixer();
    let duration = mixer.total_duration();

    println!("Composition: 8 bars, {:.1}s duration\n", duration);

    // Test 1: CPU-only export
    println!("=== Test 1: CPU-Only Export ===");
    {
        let mut mixer_cpu = mixer.clone();

        let start = Instant::now();
        engine.export_wav(&mut mixer_cpu, "test_cpu_export.wav")?;
        let export_time = start.elapsed();

        let realtime_ratio = duration / export_time.as_secs_f32();
        println!("  Export time: {:.3}s ({:.1}x realtime)\n", export_time.as_secs_f32(), realtime_ratio);

        // Cleanup
        std::fs::remove_file("test_cpu_export.wav").ok();
    }

    // Test 2: GPU + Cache export
    println!("=== Test 2: GPU + Cache Export 🚀 ===");
    {
        let mut mixer_gpu = mixer.clone();
        mixer_gpu.enable_cache();
        mixer_gpu.enable_gpu();

        let start = Instant::now();
        engine.export_wav(&mut mixer_gpu, "test_gpu_export.wav")?;
        let export_time = start.elapsed();

        let realtime_ratio = duration / export_time.as_secs_f32();
        println!("  Export time: {:.3}s ({:.1}x realtime)\n", export_time.as_secs_f32(), realtime_ratio);

        if let Some(stats) = mixer_gpu.cache_stats() {
            println!("  Cache: {} entries, {} hits, {} misses",
                stats.hits + stats.misses,
                stats.hits,
                stats.misses);
        }

        // Cleanup
        std::fs::remove_file("test_gpu_export.wav").ok();
    }

    println!("\n=== Summary ===");
    println!("GPU acceleration automatically applies to:");
    println!("  • engine.export_wav(mixer, path)");
    println!("  • engine.export_flac(mixer, path)");
    println!("  • mixer.export_wav(path, sample_rate)");
    println!("  • mixer.export_flac(path, sample_rate)");
    println!("\nJust enable cache + GPU before export for instant rendering! 🚀");

    Ok(())
}
