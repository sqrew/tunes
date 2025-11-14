use std::time::Instant;
use tunes::prelude::*;

/// Benchmark SIMD-accelerated effects
///
/// Tests the SIMD implementations of:
/// - Distortion
/// - Saturation
/// - Tremolo
/// - Ring Modulator
///
/// Compares rendering time with heavy effect processing

fn main() -> anyhow::Result<()> {
    println!("\n⚡ SIMD Effects Benchmark\n");

    // Display SIMD capabilities
    use tunes::synthesis::simd::SIMD;
    let simd_width = SIMD.width();
    let simd_name = if simd_width == 8 {
        "AVX2"
    } else if simd_width == 4 {
        "SSE/NEON"
    } else {
        "Scalar"
    };

    println!("Detected SIMD: {} ({} lanes)", simd_name, simd_width);
    println!("Expected speedup: {}x for SIMD effects\n", simd_width);

    let engine = AudioEngine::new()?;

    // Test each SIMD-accelerated effect
    let effects_configs = [
        ("Distortion", vec![("heavy", 0.9, 0.0, 0.0)]),
        (
            "Saturation",
            vec![("soft", 2.0, 0.3, 0.5), ("hard", 3.5, 0.7, 0.7)],
        ),
        (
            "Tremolo",
            vec![("slow", 4.0, 0.0, 0.0), ("fast", 8.0, 0.0, 0.0)],
        ),
        (
            "Ring Modulator",
            vec![
                ("harmonic", 440.0, 0.0, 0.0),
                ("inharmonic", 333.0, 0.0, 0.0),
            ],
        ),
    ];

    for (effect_name, configs) in &effects_configs {
        println!("=== {} ===", effect_name);

        for (variant, param1, param2, param3) in configs {
            // Create a composition with heavy use of this effect
            let mut comp = Composition::new(Tempo::new(120.0));

            // 10 tracks, each with the effect applied
            for i in 0..10 {
                let track_name = format!("track_{}", i);
                let freq = 200.0 + (i as f32 * 50.0);

                let mut track = comp
                    .instrument(&track_name, &Instrument::synth_lead())
                    .at(0.0);

                // Apply the specific effect
                track = match effect_name.as_ref() {
                    "Distortion" => track.distortion(Distortion::new(*param1, 1.0)),
                    "Saturation" => track.saturation(Saturation::new(*param1, *param2, *param3)),
                    "Tremolo" => track.tremolo(Tremolo::new(*param1, 0.8)),
                    "Ring Modulator" => track.ring_mod(RingModulator::new(*param1, 1.0)),
                    _ => track,
                };

                track.notes(&[freq, freq * 1.25, freq * 1.5, freq * 2.0], 0.25);
            }

            let mut mixer = comp.into_mixer();

            // Benchmark rendering
            let start = Instant::now();
            let buffer = engine.render_to_buffer(&mut mixer);
            let render_time = start.elapsed();

            let sample_count = buffer.len() / 2;
            let audio_duration = sample_count as f32 / 44100.0;
            let realtime_ratio = audio_duration / render_time.as_secs_f32();

            println!("  {} variant:", variant);
            println!("    Audio: {:.1}s, 10 tracks", audio_duration);
            println!("    Render: {:.3}s", render_time.as_secs_f32());
            println!("    Ratio: {:.1}x realtime", realtime_ratio);

            if realtime_ratio > 20.0 {
                println!("    ✅ Excellent SIMD performance!");
            } else if realtime_ratio > 10.0 {
                println!("    ✅ Good SIMD acceleration");
            } else {
                println!("    ⚠️  Lower than expected");
            }
        }
        println!();
    }

    // Combined stress test: All SIMD effects at once
    println!("=== Combined Stress Test ===");
    println!("All SIMD effects on one track (worst case)\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("kitchen_sink", &Instrument::synth_lead())
        .at(0.0)
        .distortion(Distortion::new(0.8, 1.0))
        .saturation(Saturation::new(2.5, 0.5, 0.7))
        .tremolo(Tremolo::new(6.0, 0.5))
        .ring_mod(RingModulator::new(440.0, 0.8))
        .notes(
            &[440.0, 494.0, 523.0, 587.0, 659.0, 698.0, 784.0, 880.0],
            0.5,
        );

    let mut mixer = comp.into_mixer();

    let start = Instant::now();
    let buffer = engine.render_to_buffer(&mut mixer);
    let render_time = start.elapsed();

    let sample_count = buffer.len() / 2;
    let audio_duration = sample_count as f32 / 44100.0;
    let realtime_ratio = audio_duration / render_time.as_secs_f32();

    println!("  Audio duration: {:.1}s", audio_duration);
    println!("  Render time: {:.3}s", render_time.as_secs_f32());
    println!("  Realtime ratio: {:.1}x", realtime_ratio);

    if realtime_ratio > 10.0 {
        println!("  ✅ Can stack multiple SIMD effects without issues!\n");
    } else if realtime_ratio > 5.0 {
        println!("  ✅ SIMD effects are stackable\n");
    } else {
        println!("  ⚠️  May struggle with many stacked effects\n");
    }

    println!("=== Summary ===");
    println!("SIMD effects (Distortion, Saturation, Tremolo, Ring Mod):");
    println!("  • Process {}-samples per instruction", simd_width);
    println!("  • Significant speedup on AVX2/SSE/NEON CPUs");
    println!("  • Can be freely stacked in production\n");

    Ok(())
}
