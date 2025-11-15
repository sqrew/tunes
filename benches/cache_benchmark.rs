use tunes::prelude::*;
use tunes::synthesis::fm_synthesis::FMParams;
use std::time::Instant;

/// Benchmark demonstrating sample cache performance
///
/// The cache is designed for scenarios where:
/// 1. The same sounds repeat many times (drums, patterns, loops)
/// 2. Sounds can be pre-rendered once (ideally on GPU) and played back
/// 3. You want to avoid live synthesis overhead during playback

fn main() -> anyhow::Result<()> {
    println!("\n🗂️  Sample Cache Benchmark\n");
    println!("Comparing: CPU-only, CPU+Cache, GPU+Cache\n");

    let engine = AudioEngine::new()?;

    // Test 1: Repetitive bass pattern
    println!("=== Test 1: Bass Pattern (5 notes × 100 repetitions) ===");
    println!("Simulates a repetitive bass line with complex FM synthesis\n");

    println!("Scenario 1: CPU live synthesis (no cache)");
    {
        let mut comp = Composition::new(Tempo::new(120.0));

        // 5 different bass notes, each repeated 100 times = 500 total
        let bass_notes = [C2, D2, E2, F2, G2];

        for i in 0..500 {
            let note = bass_notes[i % 5];
            comp.track("bass")
                .at((i as f32) * 0.1)
                .note(&[note], 0.15)
                .fm(FMParams::new(4.0, 6.0));  // Heavy FM modulation
        }

        let mut mixer = comp.into_mixer();
        let duration = mixer.total_duration();

        println!("  Composition: {:.1}s, 5 unique notes, 500 total hits", duration);

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Render time: {:.3}s ({:.1}x realtime)", render_time.as_secs_f32(), realtime_ratio);
    }

    println!("\nScenario 2: CPU with cache (CPU pre-render + CPU playback)");
    {
        let mut comp = Composition::new(Tempo::new(120.0));

        let bass_notes = [C2, D2, E2, F2, G2];

        for i in 0..500 {
            let note = bass_notes[i % 5];
            comp.track("bass")
                .at((i as f32) * 0.1)
                .note(&[note], 0.15)
                .fm(FMParams::new(4.0, 6.0));
        }

        let mut mixer = comp.into_mixer();
        mixer.enable_cache();

        let duration = mixer.total_duration();
        println!("  Composition: {:.1}s, 5 unique notes, 500 total hits", duration);

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Render time: {:.3}s ({:.1}x realtime)", render_time.as_secs_f32(), realtime_ratio);

        let stats = mixer.cache_stats().unwrap();
        println!("  Cache: {} hits, {} misses ({:.1}% hit rate)",
            stats.hits,
            stats.misses,
            stats.hit_rate() * 100.0);
    }

    #[cfg(feature = "gpu")]
    {
        println!("\nScenario 3: GPU+Cache (GPU pre-render + CPU playback)");
        {
            let mut comp = Composition::new(Tempo::new(120.0));

            let bass_notes = [C2, D2, E2, F2, G2];

            for i in 0..500 {
                let note = bass_notes[i % 5];
                comp.track("bass")
                    .at((i as f32) * 0.1)
                    .note(&[note], 0.15)
                    .fm(FMParams::new(4.0, 6.0));
            }

            let mut mixer = comp.into_mixer();
            mixer.enable_cache();
            mixer.enable_gpu();

            let duration = mixer.total_duration();
            println!("  Composition: {:.1}s, 5 unique notes, 500 total hits", duration);

            let start = Instant::now();
            let buffer = engine.render_to_buffer(&mut mixer);
            let render_time = start.elapsed();

            let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
            let realtime_ratio = audio_duration / render_time.as_secs_f32();

            println!("  Render time: {:.3}s ({:.1}x realtime)", render_time.as_secs_f32(), realtime_ratio);

            let stats = mixer.cache_stats().unwrap();
            println!("  Cache: {} hits, {} misses ({:.1}% hit rate)",
                stats.hits,
                stats.misses,
                stats.hit_rate() * 100.0);
        }
    }

    #[cfg(not(feature = "gpu"))]
    {
        println!("\nScenario 3: GPU+Cache - SKIPPED (GPU feature not enabled)");
        println!("  Run with: cargo bench --bench cache_benchmark --features gpu");
    }

    // Test 2: Chord progression with many repetitions
    println!("\n=== Test 2: Chord Progression (8 chords × 50 repetitions) ===");
    println!("Simulates a 30-second progression with complex FM synthesis\n");

    println!("Scenario 1: CPU live synthesis (no cache)");
    {
        let mut comp = Composition::new(Tempo::new(120.0));

        // 8 different chords, each repeated 50 times
        let chords = [
            vec![C4, E4, G4],      // C major
            vec![D4, F4, A4],      // Dm
            vec![E4, G4, B4],      // Em
            vec![F4, A4, C5],      // F major
            vec![G4, B4, D5],      // G major
            vec![A4, C5, E5],      // Am
            vec![B4, D5, F5],      // Bdim
            vec![C5, E5, G5],      // C major (octave up)
        ];

        for i in 0..400 {
            let chord = &chords[i % 8];
            comp.track("fm_pad")
                .at((i as f32) * 0.075)
                .note(chord, 0.5)
                .fm(FMParams::new(2.5, 4.0));
        }

        let mut mixer = comp.into_mixer();
        let duration = mixer.total_duration();

        println!("  Composition: {:.1}s, 8 unique chords, 400 total plays", duration);

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Render time: {:.3}s ({:.1}x realtime)", render_time.as_secs_f32(), realtime_ratio);
    }

    println!("\nScenario 2: CPU with cache (CPU pre-render + CPU playback)");
    {
        let mut comp = Composition::new(Tempo::new(120.0));

        let chords = [
            vec![C4, E4, G4],
            vec![D4, F4, A4],
            vec![E4, G4, B4],
            vec![F4, A4, C5],
            vec![G4, B4, D5],
            vec![A4, C5, E5],
            vec![B4, D5, F5],
            vec![C5, E5, G5],
        ];

        for i in 0..400 {
            let chord = &chords[i % 8];
            comp.track("fm_pad")
                .at((i as f32) * 0.075)
                .note(chord, 0.5)
                .fm(FMParams::new(2.5, 4.0));
        }

        let mut mixer = comp.into_mixer();
        mixer.enable_cache();

        let duration = mixer.total_duration();
        println!("  Composition: {:.1}s, 8 unique chords, 400 total plays", duration);

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Render time: {:.3}s ({:.1}x realtime)", render_time.as_secs_f32(), realtime_ratio);

        let stats = mixer.cache_stats().unwrap();
        println!("  Cache: {} hits, {} misses", stats.hits, stats.misses);
    }

    #[cfg(feature = "gpu")]
    {
        println!("\nScenario 3: GPU+Cache (GPU pre-render + CPU playback)");
        {
            let mut comp = Composition::new(Tempo::new(120.0));

            let chords = [
                vec![C4, E4, G4],
                vec![D4, F4, A4],
                vec![E4, G4, B4],
                vec![F4, A4, C5],
                vec![G4, B4, D5],
                vec![A4, C5, E5],
                vec![B4, D5, F5],
                vec![C5, E5, G5],
            ];

            for i in 0..400 {
                let chord = &chords[i % 8];
                comp.track("fm_pad")
                    .at((i as f32) * 0.075)
                    .note(chord, 0.5)
                    .fm(FMParams::new(2.5, 4.0));
            }

            let mut mixer = comp.into_mixer();
            mixer.enable_cache();
            mixer.enable_gpu();

            let duration = mixer.total_duration();
            println!("  Composition: {:.1}s, 8 unique chords, 400 total plays", duration);

            let start = Instant::now();
            let buffer = engine.render_to_buffer(&mut mixer);
            let render_time = start.elapsed();

            let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
            let realtime_ratio = audio_duration / render_time.as_secs_f32();

            println!("  Render time: {:.3}s ({:.1}x realtime)", render_time.as_secs_f32(), realtime_ratio);

            let stats = mixer.cache_stats().unwrap();
            println!("  Cache: {} hits, {} misses", stats.hits, stats.misses);
        }
    }

    #[cfg(not(feature = "gpu"))]
    {
        println!("\nScenario 3: GPU+Cache - SKIPPED (GPU feature not enabled)");
        println!("  Run with: cargo bench --bench cache_benchmark --features gpu");
    }

    println!("\n=== Summary ===");
    println!("The cache is designed for:");
    println!("  ✓ Pattern-heavy music (drums, loops, ostinatos)");
    println!("  ✓ Many repetitions of the same sounds");
    println!("  ✓ GPU pre-rendering (render all unique sounds at startup)");
    println!("  ✓ Avoiding live synthesis overhead during playback");
    println!("\nIf live synthesis is fast enough, caching may not help much.");
    println!("Cache shines when synthesis is expensive or repetitions are high.");

    Ok(())
}
