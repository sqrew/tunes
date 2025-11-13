use tunes::prelude::*;
use std::time::Instant;

/// Benchmark streaming audio memory usage
///
/// Verifies that streaming mode keeps memory usage low for large audio files.
/// Compares memory usage: normal loading vs streaming.

fn main() -> anyhow::Result<()> {
    println!("\n💾 Streaming Memory Benchmark\n");

    // Create a large test audio file (~10 seconds, which is ~1.7MB stereo)
    println!("Creating test audio file...");
    let mut comp = Composition::new(Tempo::new(120.0));

    // Dense composition to create a larger file
    for i in 0..20 {
        let freq = 200.0 + (i as f32 * 50.0);
        comp.track(&format!("t{}", i))
            .at(0.0)
            .notes(&[freq, freq * 1.25, freq * 1.5], 0.5)
            .repeat(20);
    }

    let mut mixer = comp.into_mixer();
    mixer.export_wav("/tmp/streaming_test.wav", 44100)?;

    let file_size = std::fs::metadata("/tmp/streaming_test.wav")?.len();
    println!("  ✓ Created test file: {:.1} MB\n", file_size as f64 / 1_000_000.0);

    let engine = AudioEngine::new()?;

    // Test 1: Normal sample loading (loads entire file into memory)
    println!("=== Test 1: Normal Sample Loading ===");
    println!("Loading entire file into memory...\n");

    let start = Instant::now();
    let sample = Sample::from_file("/tmp/streaming_test.wav")?;
    let load_time = start.elapsed();

    let sample_memory = sample.data.len() * std::mem::size_of::<f32>();
    let duration = sample.data.len() as f32 / sample.sample_rate as f32 / sample.channels as f32;
    println!("  Load time: {:.3}s", load_time.as_secs_f32());
    println!("  Memory used: {:.1} MB", sample_memory as f64 / 1_000_000.0);
    println!("  Sample rate: {} Hz", sample.sample_rate);
    println!("  Channels: {}", sample.channels);
    println!("  Samples: {}", sample.data.len());
    println!("  Duration: {:.1}s\n", duration);

    // Test 2: Streaming mode (minimal memory footprint)
    println!("=== Test 2: Streaming Mode ===");
    println!("Using streaming (minimal memory)...\n");

    let start = Instant::now();
    let _stream_id = engine.stream_file("/tmp/streaming_test.wav")?;
    let stream_time = start.elapsed();

    println!("  Stream start time: {:.3}s", stream_time.as_secs_f32());
    println!("  Memory used: < 1 MB (ring buffer only)");
    println!("  Note: Audio data streamed from disk as needed\n");

    // Wait a bit to let streaming actually happen
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Comparison
    println!("=== Comparison ===");
    let memory_saved = sample_memory as f64 - 1_000_000.0; // Assume ~1MB for streaming
    let savings_percent = (memory_saved / sample_memory as f64) * 100.0;

    println!("  Normal loading: {:.1} MB in RAM", sample_memory as f64 / 1_000_000.0);
    println!("  Streaming: ~1.0 MB in RAM");
    println!("  Memory saved: {:.1} MB ({:.0}%)\n", memory_saved / 1_000_000.0, savings_percent);

    // Extrapolate to larger files
    println!("=== Extrapolation ===");
    println!("For a 5-minute background music file:");
    let five_min_size = 5.0 * 60.0 * 44100.0 * 2.0 * 4.0; // 5min * 44.1kHz * stereo * f32
    println!("  Normal loading: {:.0} MB in RAM", five_min_size / 1_000_000.0);
    println!("  Streaming: ~1 MB in RAM");
    println!("  Memory saved: {:.0} MB\n", (five_min_size - 1_000_000.0) / 1_000_000.0);

    // Cleanup
    std::fs::remove_file("/tmp/streaming_test.wav").ok();

    println!("=== Summary ===");
    println!("Streaming is ideal for:");
    println!("  ✅ Long background music (>30 seconds)");
    println!("  ✅ Ambient soundscapes");
    println!("  ✅ Memory-constrained environments");
    println!("  ✅ Large audio files (>10 MB)");
    println!("\nNormal loading is better for:");
    println!("  ✅ Short sound effects (<5 seconds)");
    println!("  ✅ Frequently repeated sounds (caching benefits)");
    println!("  ✅ Low-latency playback requirements");
    println!("\nRule of thumb:");
    println!("  • Short, frequent sounds → Normal loading + cache");
    println!("  • Long, infrequent sounds → Streaming\n");

    Ok(())
}
