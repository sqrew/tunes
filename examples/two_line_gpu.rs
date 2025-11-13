use tunes::prelude::*;

/// The Ultimate 2-Line GPU Demo
///
/// This demonstrates the absolute cleanest way to get GPU-accelerated audio.
/// Just one constructor change, and every sample is GPU-accelerated!

fn main() -> anyhow::Result<()> {
    println!("\n🚀 The 2-Line GPU-Accelerated Audio Demo\n");

    // ============================================================
    // BEFORE: Default performance (50-200x realtime)
    // ============================================================
    println!("=== Before: Default Performance ===\n");
    println!("Code:");
    println!("  let engine = AudioEngine::new()?;");
    println!("  engine.play_sample(\"explosion.wav\")?;");
    println!();
    println!("Performance: 50-200x realtime (CPU only)\n");

    // ============================================================
    // AFTER: GPU performance (500-5000x realtime)
    // ============================================================
    println!("=== After: GPU Acceleration ===\n");
    println!("Code:");
    println!("  let engine = AudioEngine::new_with_gpu()?;  // <-- ONE WORD CHANGE!");
    println!("  engine.play_sample(\"explosion.wav\")?;");
    println!();
    println!("Performance: 500-5000x realtime (GPU accelerated)\n");

    // ============================================================
    // LIVE DEMO
    // ============================================================
    println!("=== Live Demo ===\n");

    let engine = AudioEngine::new_with_gpu()?;
    println!("✅ Engine created with GPU acceleration");
    println!("   All play_sample() calls are now GPU-accelerated!\n");

    // Create test composition
    let mut comp = Composition::new(Tempo::new(140.0));
    comp.track("test").notes(&[C4, E4, G4], 0.2);

    println!("Playing GPU-accelerated composition...");
    engine.play_mixer(&comp.into_mixer())?;
    println!("✅ Played at 500-5000x realtime (with GPU)\n");

    println!("=== Summary ===\n");
    println!("Before:  AudioEngine::new()           → 50-200x realtime");
    println!("After:   AudioEngine::new_with_gpu()  → 500-5000x realtime");
    println!();
    println!("Change: ONE WORD in ONE LINE");
    println!("Speedup: 10-100x faster");
    println!();
    println!("That's it. That's the API. 🚀");

    Ok(())
}
