use tunes::prelude::*;
use std::time::Instant;

/// Benchmark maximum polyphony (simultaneous voices)
///
/// Tests how many notes can play simultaneously before performance degrades.
/// Important for:
/// - Dense orchestral arrangements
/// - Generative/algorithmic music with many voices
/// - Game audio with many concurrent sounds

fn main() -> anyhow::Result<()> {
    println!("\n🎹 Polyphony Benchmark\n");

    let engine = AudioEngine::new()?;

    // Test different polyphony levels
    let voice_counts = [10, 50, 100, 200, 500];

    println!("Testing simultaneous voice capacity...\n");

    for &num_voices in &voice_counts {
        println!("=== {} Simultaneous Voices ===", num_voices);

        let mut comp = Composition::new(Tempo::new(120.0));

        // Create many simultaneous notes (all starting at time 0.0)
        for i in 0..num_voices {
            let track_name = format!("voice_{}", i);
            let freq = 200.0 + (i as f32 * 5.0); // Spread across frequency range

            comp.track(&track_name)
                .at(0.0)
                .note(&[freq], 2.0); // 2-second note, all overlapping
        }

        let mut mixer = comp.into_mixer();
        let duration = mixer.total_duration();

        println!("  Configuration:");
        println!("    Voices: {}", num_voices);
        println!("    Duration: {:.1}s", duration);
        println!("    All notes overlapping");

        // Benchmark rendering
        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let sample_count = buffer.len() / 2;
        let audio_duration = sample_count as f32 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Performance:");
        println!("    Render time: {:.3}s", render_time.as_secs_f32());
        println!("    Realtime ratio: {:.1}x", realtime_ratio);

        // Calculate samples processed per second
        let samples_per_sec = (sample_count as f64 / render_time.as_secs_f64()) / 1_000_000.0;
        println!("    Throughput: {:.1} million samples/sec", samples_per_sec);

        // Verdict
        if realtime_ratio > 5.0 {
            println!("    ✅ Excellent! Can handle this polyphony easily\n");
        } else if realtime_ratio > 2.0 {
            println!("    ✅ Good! Realtime playback achievable\n");
        } else if realtime_ratio > 1.0 {
            println!("    ⚠️  Borderline - may glitch on slower systems\n");
        } else {
            println!("    ❌ Too slow - use fewer voices or offline rendering\n");
            break; // Don't test higher counts if we've already failed
        }
    }

    // Test polyphony with effects (more realistic)
    println!("=== Realistic Test: 100 Voices + Effects ===");

    let mut comp = Composition::new(Tempo::new(120.0));

    for i in 0..100 {
        let track_name = format!("voice_{}", i);
        let freq = 200.0 + (i as f32 * 5.0);

        comp.instrument(&track_name, &Instrument::synth_lead())
            .at(0.0)
            .filter(Filter::low_pass(2000.0, 0.6))
            .note(&[freq], 2.0);
    }

    // Add shared reverb
    comp.track("master")
        .at(0.0)
        .reverb(Reverb::new(0.6, 0.4, 0.3));

    let mut mixer = comp.into_mixer();

    let start = Instant::now();
    let buffer = engine.render_to_buffer(&mut mixer);
    let render_time = start.elapsed();

    let sample_count = buffer.len() / 2;
    let audio_duration = sample_count as f32 / 44100.0;
    let realtime_ratio = audio_duration / render_time.as_secs_f32();

    println!("  100 voices + filters + reverb:");
    println!("    Render time: {:.3}s", render_time.as_secs_f32());
    println!("    Realtime ratio: {:.1}x", realtime_ratio);

    if realtime_ratio > 2.0 {
        println!("    ✅ Can handle complex polyphonic arrangements!\n");
    } else {
        println!("    ⚠️  May need to reduce complexity for realtime\n");
    }

    println!("=== Summary ===");
    println!("Polyphony guidelines:");
    println!("  • 10-50 voices: ✅ Safe for any CPU");
    println!("  • 50-100 voices: ✅ Good for modern CPUs");
    println!("  • 100-200 voices: ⚠️  May need testing on target hardware");
    println!("  • 200+ voices: ⚠️  Consider offline rendering or optimization");
    println!("\nTips for high polyphony:");
    println!("  • Use simpler waveforms (sine > sawtooth)");
    println!("  • Reduce per-voice effects");
    println!("  • Use shared reverb on master track");
    println!("  • Enable SIMD (automatic on modern CPUs)\n");

    Ok(())
}
