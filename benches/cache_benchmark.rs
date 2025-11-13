use tunes::prelude::*;
use tunes::synthesis::fm_synthesis::FMParams;
use std::time::Instant;

/// Benchmark demonstrating sample cache performance
///
/// This tests how caching pre-rendered synthesis improves performance
/// when the same notes are played multiple times.

fn main() -> anyhow::Result<()> {
    println!("\n🗂️  Sample Cache Benchmark\n");

    let engine = AudioEngine::new()?;

    // Test 1: Simple sine waves (cheap synthesis)
    println!("=== Test 1: Simple Sine Waves (Baseline) ===");
    println!("Without cache:");
    {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Create a composition with repeated notes (same parameters)
        for i in 0..100 {
            comp.track("synth")
                .at((i as f32) * 0.1)
                .note(&[C4], 0.15);
        }

        let mut mixer = comp.into_mixer();
        let duration = mixer.total_duration();

        println!("  Composition: {:.1}s, 100 identical sine notes", duration);

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Render time: {:.3}s", render_time.as_secs_f32());
        println!("  Realtime ratio: {:.1}x", realtime_ratio);
    }

    println!("\nWith cache:");
    {
        let mut comp = Composition::new(Tempo::new(120.0));

        for i in 0..100 {
            comp.track("synth")
                .at((i as f32) * 0.1)
                .note(&[C4], 0.15);
        }

        let mut mixer = comp.into_mixer();
        mixer.enable_cache();

        let duration = mixer.total_duration();
        println!("  Composition: {:.1}s, 100 identical sine notes", duration);

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Render time: {:.3}s", render_time.as_secs_f32());
        println!("  Realtime ratio: {:.1}x", realtime_ratio);
        println!("  Cache: {} hits, {} misses ({:.1}% hit rate)",
            mixer.cache_stats().unwrap().hits,
            mixer.cache_stats().unwrap().misses,
            mixer.cache_stats().unwrap().hit_rate() * 100.0);
    }

    // Test 2: Complex FM synthesis (expensive)
    println!("\n=== Test 2: Complex FM Synthesis (Expensive) ===");
    println!("Without cache:");
    {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Create FM synthesis notes (much more expensive to compute)
        for i in 0..100 {
            comp.track("fm_bass")
                .at((i as f32) * 0.1)
                .note(&[C4], 0.15)
                .fm(FMParams::new(3.0, 5.0));  // Heavy FM modulation
        }

        let mut mixer = comp.into_mixer();
        let duration = mixer.total_duration();

        println!("  Composition: {:.1}s, 100 identical FM notes", duration);

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Render time: {:.3}s", render_time.as_secs_f32());
        println!("  Realtime ratio: {:.1}x", realtime_ratio);
    }

    println!("\nWith cache:");
    {
        let mut comp = Composition::new(Tempo::new(120.0));

        for i in 0..100 {
            comp.track("fm_bass")
                .at((i as f32) * 0.1)
                .note(&[C4], 0.15)
                .fm(FMParams::new(3.0, 5.0));
        }

        let mut mixer = comp.into_mixer();
        mixer.enable_cache();

        let duration = mixer.total_duration();
        println!("  Composition: {:.1}s, 100 identical FM notes", duration);

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Render time: {:.3}s", render_time.as_secs_f32());
        println!("  Realtime ratio: {:.1}x", realtime_ratio);

        println!();
        mixer.print_cache_stats();
    }

    println!("\n=== Summary ===");
    println!("Cache will speed up rendering when:");
    println!("  1. Same notes are used multiple times");
    println!("  2. Expensive synthesis (FM, complex envelopes)");
    println!("  3. Pattern-heavy compositions (drums, loops)");
    println!("\nNote: First render populates cache, subsequent renders benefit.");

    Ok(())
}
