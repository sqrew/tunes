use tunes::prelude::*;
use tunes::synthesis::fm_synthesis::FMParams;
use std::time::Instant;

/// GPU vs CPU Performance Benchmark
///
/// This benchmark demonstrates the dramatic speedup from GPU-accelerated synthesis.
/// Expected results:
/// - CPU only: ~50-100x realtime
/// - CPU + cache: ~10-20x realtime (cache overhead)
/// - GPU + cache: ~500-5000x realtime (🚀 THE GOAL!)

fn main() -> anyhow::Result<()> {
    println!("\n🚀 GPU vs CPU Performance Benchmark\n");
    println!("Testing with 192 FM synthesis notes (16-bar drum pattern)\n");

    let engine = AudioEngine::new()?;

    // Test 1: CPU only (no cache, baseline)
    println!("=== Test 1: CPU Synthesis (No Cache) ===");
    {
        let mut comp = Composition::new(Tempo::new(140.0));

        for bar in 0..16 {
            let bar_start = bar as f32 * 4.0 * 0.428;

            // Kick
            comp.track("kick")
                .at(bar_start)
                .note(&[C2], 0.15)
                .fm(FMParams::new(2.0, 8.0));

            comp.track("kick")
                .at(bar_start + 2.0 * 0.428)
                .note(&[C2], 0.15)
                .fm(FMParams::new(2.0, 8.0));

            // Snare
            comp.track("snare")
                .at(bar_start + 1.0 * 0.428)
                .note(&[D2], 0.12)
                .fm(FMParams::new(3.5, 6.0));

            comp.track("snare")
                .at(bar_start + 3.0 * 0.428)
                .note(&[D2], 0.12)
                .fm(FMParams::new(3.5, 6.0));

            // Hihat (8 per bar)
            for eighth in 0..8 {
                comp.track("hihat")
                    .at(bar_start + (eighth as f32) * 0.214)
                    .note(&[FS2], 0.08)
                    .fm(FMParams::new(4.0, 3.0));
            }
        }

        let mut mixer = comp.into_mixer();
        let duration = mixer.total_duration();

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Duration: {:.1}s", duration);
        println!("  Render time: {:.3}s", render_time.as_secs_f32());
        println!("  Realtime ratio: {:.1}x", realtime_ratio);
    }

    // Test 2: CPU + Cache
    println!("\n=== Test 2: CPU Synthesis + Cache ===");
    {
        let mut comp = Composition::new(Tempo::new(140.0));

        for bar in 0..16 {
            let bar_start = bar as f32 * 4.0 * 0.428;

            comp.track("kick")
                .at(bar_start)
                .note(&[C2], 0.15)
                .fm(FMParams::new(2.0, 8.0));

            comp.track("kick")
                .at(bar_start + 2.0 * 0.428)
                .note(&[C2], 0.15)
                .fm(FMParams::new(2.0, 8.0));

            comp.track("snare")
                .at(bar_start + 1.0 * 0.428)
                .note(&[D2], 0.12)
                .fm(FMParams::new(3.5, 6.0));

            comp.track("snare")
                .at(bar_start + 3.0 * 0.428)
                .note(&[D2], 0.12)
                .fm(FMParams::new(3.5, 6.0));

            for eighth in 0..8 {
                comp.track("hihat")
                    .at(bar_start + (eighth as f32) * 0.214)
                    .note(&[FS2], 0.08)
                    .fm(FMParams::new(4.0, 3.0));
            }
        }

        let mut mixer = comp.into_mixer();
        mixer.enable_cache();

        let duration = mixer.total_duration();

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Duration: {:.1}s", duration);
        println!("  Render time: {:.3}s", render_time.as_secs_f32());
        println!("  Realtime ratio: {:.1}x", realtime_ratio);

        if let Some(stats) = mixer.cache_stats() {
            println!("  Cache: {} entries, {} hits, {} misses",
                stats.hits + stats.misses,
                stats.hits,
                stats.misses);
        }
    }

    // Test 3: GPU + Cache (THE BIG ONE!)
    println!("\n=== Test 3: GPU Synthesis + Cache 🚀 ===");
    {
        let mut comp = Composition::new(Tempo::new(140.0));

        for bar in 0..16 {
            let bar_start = bar as f32 * 4.0 * 0.428;

            comp.track("kick")
                .at(bar_start)
                .note(&[C2], 0.15)
                .fm(FMParams::new(2.0, 8.0));

            comp.track("kick")
                .at(bar_start + 2.0 * 0.428)
                .note(&[C2], 0.15)
                .fm(FMParams::new(2.0, 8.0));

            comp.track("snare")
                .at(bar_start + 1.0 * 0.428)
                .note(&[D2], 0.12)
                .fm(FMParams::new(3.5, 6.0));

            comp.track("snare")
                .at(bar_start + 3.0 * 0.428)
                .note(&[D2], 0.12)
                .fm(FMParams::new(3.5, 6.0));

            for eighth in 0..8 {
                comp.track("hihat")
                    .at(bar_start + (eighth as f32) * 0.214)
                    .note(&[FS2], 0.08)
                    .fm(FMParams::new(4.0, 3.0));
            }
        }

        let mut mixer = comp.into_mixer();
        mixer.enable_cache();
        mixer.enable_gpu();  // 🚀 GPU ACCELERATION!

        let duration = mixer.total_duration();
        println!("  GPU enabled: {}", mixer.gpu_enabled());

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Duration: {:.1}s", duration);
        println!("  Render time: {:.3}s", render_time.as_secs_f32());
        println!("  Realtime ratio: {:.1}x 🚀", realtime_ratio);

        if let Some(stats) = mixer.cache_stats() {
            println!("  Cache: {} entries, {} hits, {} misses",
                stats.hits + stats.misses,
                stats.hits,
                stats.misses);
        }
    }

    println!("\n=== Summary ===");
    println!("GPU compute shaders accelerate synthesis by rendering complete");
    println!("notes instantly on the GPU, then caching for instant playback.");
    println!("\nExpected speedup: 50-500x faster than CPU-only rendering!");
    println!("\nIf GPU is not available, the library automatically falls back");
    println!("to fast CPU synthesis. Either way, you get great performance!");

    Ok(())
}
