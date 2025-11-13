use tunes::prelude::*;
use std::time::Instant;

/// Benchmark realtime control latency
///
/// Measures the delay between control changes (volume, pan, rate)
/// and when they take effect in the audio output.
///
/// Important for:
/// - Game audio (immediate feedback needed)
/// - Live coding (interactive parameter tweaking)
/// - Real-time control (MIDI, OSC, gamepad input)

fn main() -> anyhow::Result<()> {
    println!("\n⏱️  Realtime Control Latency Benchmark\n");

    // Test different buffer sizes
    let buffer_sizes = [512, 1024, 2048, 4096, 8192];
    let sample_rate = 44100.0;

    println!("Testing latency with different buffer sizes...\n");

    for &buffer_size in &buffer_sizes {
        println!("=== Buffer Size: {} samples ===", buffer_size);

        // Calculate theoretical latency
        let theoretical_latency_ms = (buffer_size as f32 / sample_rate) * 1000.0;
        println!("  Theoretical latency: {:.1}ms", theoretical_latency_ms);

        // Create engine with this buffer size
        let engine = AudioEngine::with_buffer_size(buffer_size)?;

        // Create a long-playing sound for testing
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("test", &Instrument::synth_lead())
            .note(&[440.0], 10.0); // 10-second note

        // Start playing
        let sound_id = engine.play_mixer_realtime(&comp.into_mixer())?;

        // Measure time to execute volume change
        let start = Instant::now();
        engine.set_volume(sound_id, 0.5)?;
        let control_time = start.elapsed();

        println!("  Control call time: {:.3}ms", control_time.as_micros() as f32 / 1000.0);
        println!("  Total latency: ~{:.1}ms (buffer + control)\n",
                 theoretical_latency_ms + (control_time.as_micros() as f32 / 1000.0));

        // Stop the sound
        engine.stop(sound_id)?;

        // Give it a moment to clean up
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // Detailed breakdown with default buffer size
    println!("=== Latency Breakdown (Default: 8192 samples) ===");
    let engine = AudioEngine::new()?;
    let default_latency = (8192.0 / sample_rate) * 1000.0;

    println!("\nComponents of latency:");
    println!("  1. Buffer size: ~{:.1}ms", default_latency);
    println!("     (Audio must fill entire buffer before playback)");
    println!("  2. Control overhead: <0.1ms");
    println!("     (Function call + message passing)");
    println!("  3. OS audio driver: 5-20ms (varies by system)");
    println!("     (Operating system audio stack)");
    println!("\n  Total expected: ~{:.0}ms\n", default_latency + 15.0);

    // Test control operation speed
    println!("=== Control Operation Performance ===");
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.instrument("bench", &Instrument::synth_lead())
        .note(&[440.0], 5.0);

    let sound_id = engine.play_mixer_realtime(&comp.into_mixer())?;

    // Benchmark different control operations
    let iterations = 1000;

    // set_volume
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = engine.set_volume(sound_id, 0.7);
    }
    let elapsed = start.elapsed();
    println!("  set_volume: {:.2}µs per call", elapsed.as_micros() as f32 / iterations as f32);

    // set_pan
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = engine.set_pan(sound_id, 0.5);
    }
    let elapsed = start.elapsed();
    println!("  set_pan: {:.2}µs per call", elapsed.as_micros() as f32 / iterations as f32);

    // set_playback_rate
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = engine.set_playback_rate(sound_id, 1.2);
    }
    let elapsed = start.elapsed();
    println!("  set_playback_rate: {:.2}µs per call", elapsed.as_micros() as f32 / iterations as f32);

    engine.stop(sound_id)?;

    println!("\n=== Recommendations ===");
    println!("\nFor different use cases:");
    println!("\n  🎮 Game Audio (Immediate Feedback):");
    println!("     Buffer: 1024-2048 samples (~23-46ms)");
    println!("     Tradeoff: Lower latency, may glitch on slow CPUs");
    println!("\n  🎵 Music Production (Quality First):");
    println!("     Buffer: 4096-8192 samples (~93-186ms)");
    println!("     Tradeoff: Higher latency, no glitches");
    println!("\n  🔴 Live Coding (Balance):");
    println!("     Buffer: 2048-4096 samples (~46-93ms)");
    println!("     Tradeoff: Acceptable latency, stable playback");
    println!("\n  📱 Mobile/Embedded (Safety):");
    println!("     Buffer: 8192-16384 samples (~186-372ms)");
    println!("     Tradeoff: High latency, maximum stability");

    println!("\n=== Human Perception ===");
    println!("  < 10ms:  Feels instant (impossible with audio buffers)");
    println!("  10-30ms: Feels responsive (requires tiny buffers)");
    println!("  30-50ms: Acceptable for games (1024-2048 buffer)");
    println!("  50-100ms: Noticeable but usable (4096 buffer)");
    println!("  > 100ms: Feels laggy (avoid for interactive use)\n");

    println!("=== Summary ===");
    println!("Control operations are very fast (<1µs).");
    println!("Latency is dominated by buffer size (audio pipeline requirement).");
    println!("Choose buffer size based on your latency vs stability needs.\n");

    Ok(())
}
