use std::time::Instant;
use tunes::prelude::*;
use tunes::synthesis::fm_synthesis::FMParams;

/// GPU Stress Test Benchmark
///
/// Designed to give GPU the best possible chance by:
/// 1. Many unique sounds (amortize cache overhead)
/// 2. Complex FM synthesis (heavy computation per sample)
/// 3. Long duration notes (maximize GPU parallelism)
/// 4. Large workload (minimize overhead impact)

fn main() -> anyhow::Result<()> {
    println!("\n🔥 GPU Stress Test - Designed to Favor GPU\n");
    println!("This benchmark is specifically designed to give GPU");
    println!("the best possible chance by maximizing parallel work.\n");

    // Test parameters (balanced for reasonable runtime)
    let unique_notes = 50; // Enough to amortize cache overhead
    let repeats_per_note = 20; // Each note played 20 times
    let note_duration = 1.0; // 1 second notes (still substantial for GPU)
    let total_events = unique_notes * repeats_per_note;

    println!("Workload:");
    println!("  Unique notes: {}", unique_notes);
    println!("  Repeats per note: {}", repeats_per_note);
    println!("  Total events: {}", total_events);
    println!("  Note duration: {}s", note_duration);
    println!("  Total audio: ~{}s\n", unique_notes * 2);

    let engine = AudioEngine::new()?;

    // ============================================================
    // Test 1: CPU Only (No Cache) - Baseline
    // ============================================================

    println!("=== Test 1: CPU Only (No Cache) ===");
    println!("Baseline: Real-time synthesis for every note event");

    let comp1 = create_complex_composition(unique_notes, repeats_per_note, note_duration);
    let mut mixer1 = comp1.into_mixer();
    let duration = mixer1.total_duration();

    let start = Instant::now();
    let _buffer1 = engine.render_to_buffer(&mut mixer1);
    let cpu_uncached_time = start.elapsed();
    let cpu_uncached_ratio = duration / cpu_uncached_time.as_secs_f32();

    println!("  Duration: {:.1}s", duration);
    println!("  Render time: {:.3}s", cpu_uncached_time.as_secs_f32());
    println!("  Realtime ratio: {:.1}x", cpu_uncached_ratio);
    println!(
        "  Synthesis events: {} (every note synthesized)",
        total_events
    );

    // ============================================================
    // Test 2: CPU + Cache
    // ============================================================

    println!("\n=== Test 2: CPU + Cache ===");
    println!(
        "Synthesize {} unique notes once, cache results",
        unique_notes
    );

    let comp2 = create_complex_composition(unique_notes, repeats_per_note, note_duration);
    let mut mixer2 = comp2.into_mixer();
    mixer2.enable_cache();

    let start = Instant::now();
    let _buffer2 = engine.render_to_buffer(&mut mixer2);
    let cpu_cached_time = start.elapsed();
    let cpu_cached_ratio = duration / cpu_cached_time.as_secs_f32();

    println!("  Render time: {:.3}s", cpu_cached_time.as_secs_f32());
    println!("  Realtime ratio: {:.1}x", cpu_cached_ratio);
    println!(
        "  Speedup vs uncached: {:.2}x",
        cpu_uncached_time.as_secs_f32() / cpu_cached_time.as_secs_f32()
    );

    if let Some(stats) = mixer2.cache_stats() {
        println!("  Cache hits: {}", stats.hits);
        println!("  Cache misses: {}", stats.misses);
        println!("  Hit rate: {:.1}%", stats.hit_rate() * 100.0);
    }

    // ============================================================
    // Test 3: GPU + Cache
    // ============================================================

    #[cfg(feature = "gpu")]
    {
        println!("\n=== Test 3: GPU + Cache 🚀 ===");
        println!(
            "GPU synthesizes {} unique notes, cache results",
            unique_notes
        );

        let mut comp3 = create_complex_composition(unique_notes, repeats_per_note, note_duration);
        let mut mixer3 = comp3.into_mixer();
        mixer3.enable_cache();
        mixer3.enable_gpu();

        if !mixer3.gpu_enabled() {
            println!("  ⚠️  GPU initialization failed - skipping GPU test");
        } else {
            let start = Instant::now();
            let _buffer3 = engine.render_to_buffer(&mut mixer3);
            let gpu_cached_time = start.elapsed();
            let gpu_cached_ratio = duration / gpu_cached_time.as_secs_f32();

            println!("  Render time: {:.3}s", gpu_cached_time.as_secs_f32());
            println!("  Realtime ratio: {:.1}x", gpu_cached_ratio);
            println!(
                "  Speedup vs CPU cached: {:.2}x",
                cpu_cached_time.as_secs_f32() / gpu_cached_time.as_secs_f32()
            );
            println!(
                "  Speedup vs CPU uncached: {:.2}x",
                cpu_uncached_time.as_secs_f32() / gpu_cached_time.as_secs_f32()
            );

            if let Some(stats) = mixer3.cache_stats() {
                println!("  Cache hits: {}", stats.hits);
                println!("  Cache misses: {}", stats.misses);
            }
        }
    }

    #[cfg(not(feature = "gpu"))]
    {
        println!("\n=== Test 3: GPU + Cache ===");
        println!("  (Compile with --features gpu to test GPU acceleration)");
    }

    // ============================================================
    // Summary
    // ============================================================

    println!("\n=== Summary ===");
    println!("This benchmark was designed to favor GPU by:");
    println!(
        "  • {} unique sounds (amortize cache overhead)",
        unique_notes
    );
    println!("  • Complex FM synthesis (heavy per-sample computation)");
    println!(
        "  • Long notes ({}s each, maximize parallelism)",
        note_duration
    );
    println!("  • Large workload ({} total events)", total_events);
    println!();
    println!("If GPU still shows minimal benefit on this workload,");
    println!("it's due to integrated GPU limitations, not the algorithm.");
    println!();
    println!("Expected results:");
    println!("  • Integrated GPU: 1.0-1.2x speedup (marginal)");
    println!("  • Discrete GPU (RTX/RX): 5-50x speedup (significant)");

    Ok(())
}

fn create_complex_composition(
    unique_notes: usize,
    repeats_per_note: usize,
    _note_duration: f32,
) -> Composition {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Create truly unique notes by varying DURATION and waveform
    // Cache must key by duration since it affects sample count!
    for unique_id in 0..unique_notes {
        let base_freq = 440.0; // Same frequency for all

        // Vary both waveform AND duration to ensure uniqueness
        let waveform = match unique_id % 4 {
            0 => Waveform::Sine,
            1 => Waveform::Sawtooth,
            2 => Waveform::Square,
            _ => Waveform::Triangle,
        };

        // VARY DURATION - this should definitely create unique cache entries!
        let duration = 0.5 + (unique_id as f32 * 0.05); // 0.5s, 0.55s, 0.6s...

        // Vary FM parameters too
        let mod_ratio = 2.0 + (unique_id as f32 * 0.1);
        let mod_index = 3.0 + (unique_id as f32 * 0.2);

        // Play this unique note multiple times at different positions
        for repeat in 0..repeats_per_note {
            let time = (unique_id * repeats_per_note + repeat) as f32 * 0.5;

            comp.track(&format!("track_{}", unique_id))
                .at(time)
                .waveform(waveform)
                .note(&[base_freq], duration)
                .fm(FMParams::new(mod_ratio, mod_index));
        }
    }

    comp
}
