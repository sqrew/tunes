use tunes::prelude::*;
use std::time::Instant;

/// Demonstrates automatic sample caching performance
fn main() -> anyhow::Result<()> {
    println!("🚀 Sample Cache Performance Demo\n");

    let engine = AudioEngine::new()?;

    // Create a test sample
    println!("Creating test sample...");
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("tone").note(&[440.0], 0.5);
    engine.export_wav(&mut comp.into_mixer(), "/tmp/test_cache.wav")?;
    println!("  ✓ Created /tmp/test_cache.wav\n");

    // Test 1: First load (should be slow - disk I/O)
    println!("Test 1: First call (loads from disk)");
    let start = Instant::now();
    engine.play_sample("/tmp/test_cache.wav")?;
    let first_duration = start.elapsed();
    println!("  Time: {:?}\n", first_duration);

    // Test 2: Second load (should be instant - cached)
    println!("Test 2: Second call (uses cache)");
    let start = Instant::now();
    engine.play_sample("/tmp/test_cache.wav")?;
    let second_duration = start.elapsed();
    println!("  Time: {:?}", second_duration);

    if second_duration < first_duration / 10 {
        println!("  ✅ Cache hit! Second call was {}x faster!\n",
                 first_duration.as_micros() / second_duration.as_micros().max(1));
    } else {
        println!("  ⚠️  Unexpected: Second call should be much faster\n");
    }

    // Test 3: Spam test (should all be instant)
    println!("Test 3: Spam 100 calls");
    let start = Instant::now();
    for _ in 0..100 {
        engine.play_sample("/tmp/test_cache.wav")?;
    }
    let spam_duration = start.elapsed();
    println!("  Total time for 100 calls: {:?}", spam_duration);
    println!("  Average per call: {:?}\n", spam_duration / 100);

    // Test 4: Preload performance
    println!("Test 4: Preload new sample");
    let mut comp2 = Composition::new(Tempo::new(120.0));
    comp2.track("tone").note(&[880.0], 0.5);
    engine.export_wav(&mut comp2.into_mixer(), "/tmp/test_cache2.wav")?;

    let start = Instant::now();
    engine.preload_sample("/tmp/test_cache2.wav")?;
    let preload_duration = start.elapsed();
    println!("  Preload time: {:?}", preload_duration);

    let start = Instant::now();
    engine.play_sample("/tmp/test_cache2.wav")?;
    let play_duration = start.elapsed();
    println!("  First play time (pre-loaded): {:?}", play_duration);

    if play_duration < preload_duration / 5 {
        println!("  ✅ Preload worked! Play was instant.\n");
    }

    // Test 5: Cache management
    println!("Test 5: Cache management");
    engine.remove_cached_sample("/tmp/test_cache.wav")?;
    println!("  ✓ Removed /tmp/test_cache.wav from cache");

    let start = Instant::now();
    engine.play_sample("/tmp/test_cache.wav")?;
    let reload_duration = start.elapsed();
    println!("  Time to play after removal: {:?}", reload_duration);

    if reload_duration > second_duration * 5 {
        println!("  ✅ Cache was actually cleared (had to reload)\n");
    }

    engine.clear_sample_cache()?;
    println!("  ✓ Cleared entire cache\n");

    // Clean up
    std::fs::remove_file("/tmp/test_cache.wav").ok();
    std::fs::remove_file("/tmp/test_cache2.wav").ok();

    println!("✅ Demo complete!");
    println!("\n📊 Summary:");
    println!("  • Automatic caching makes repeated sounds instant");
    println!("  • No manual cache management needed");
    println!("  • Optional preload for zero first-play latency");
    println!("  • Optional cache clearing for memory management\n");

    Ok(())
}
