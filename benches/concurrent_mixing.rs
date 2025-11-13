use tunes::prelude::*;
use std::time::Instant;

/// Benchmark concurrent track mixing with realistic production effects
///
/// This simulates a typical music production scenario:
/// - Multiple tracks playing simultaneously
/// - Each track has reverb, delay, and compression
/// - Measures if realtime playback is achievable

fn main() -> anyhow::Result<()> {
    println!("\n🎚️  Concurrent Mixing Benchmark\n");

    let engine = AudioEngine::new()?;

    // Test configurations
    let track_counts = [5, 10, 20];

    for &num_tracks in &track_counts {
        println!("=== Testing {} Tracks ===", num_tracks);

        let mut comp = Composition::new(Tempo::new(120.0));

        // Create multiple tracks, each with heavy effects
        for i in 0..num_tracks {
            let track_name = format!("track_{}", i);
            let base_freq = 200.0 + (i as f32 * 50.0);

            let instrument = match i % 4 {
                0 => Instrument::synth_lead(),
                1 => Instrument::electric_piano(),
                2 => Instrument::warm_pad(),
                _ => Instrument::sub_bass(),
            };

            // Each track: 8 notes, heavy effects
            comp.instrument(&track_name, &instrument)
                .at(0.0)
                .reverb(Reverb::new(0.7, 0.5, 0.3))      // Large reverb
                .delay(Delay::new(0.25, 0.4, 0.3))       // Quarter note delay
                .compressor(Compressor::new(0.5, 4.0, 0.01, 0.1, 1.2))  // Compression
                .notes(&[
                    base_freq, base_freq * 1.125, base_freq * 1.25, base_freq * 1.5,
                    base_freq * 1.25, base_freq, base_freq * 0.875, base_freq
                ], 0.5);
        }

        let mut mixer = comp.into_mixer();
        let duration = mixer.total_duration();

        println!("  Composition: {:.1}s, {} tracks", duration, num_tracks);
        println!("  Effects per track: Reverb + Delay + Compressor");

        // Benchmark rendering
        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let sample_count = buffer.len() / 2; // Stereo
        let audio_duration = sample_count as f32 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Results:");
        println!("    Audio duration: {:.2}s", audio_duration);
        println!("    Render time: {:.3}s", render_time.as_secs_f32());
        println!("    Realtime ratio: {:.1}x", realtime_ratio);

        // Verdict
        if realtime_ratio > 10.0 {
            println!("    ✅ Excellent! Plenty of CPU headroom\n");
        } else if realtime_ratio > 3.0 {
            println!("    ✅ Good! Realtime playback achievable\n");
        } else if realtime_ratio > 1.0 {
            println!("    ⚠️  Tight! May glitch on slower systems\n");
        } else {
            println!("    ❌ Too slow for realtime playback\n");
        }
    }

    println!("=== Summary ===");
    println!("Typical production workflow (10 tracks with effects):");
    println!("  - If ratio > 3x: ✅ Safe for realtime playback");
    println!("  - If ratio > 10x: ✅ Can add more tracks/effects");
    println!("  - If ratio < 1x: ⚠️  Use offline rendering only\n");

    Ok(())
}
