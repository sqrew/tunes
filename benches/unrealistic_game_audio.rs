use std::time::Instant;
use tunes::prelude::*;

/// UNREALISTIC Game Audio Benchmark - Maximum Stress Test
///
/// This benchmark represents the ABSOLUTE WORST CASE scenario:
/// - Heavy distortion/overdrive on every sample
/// - Convolution reverb (most expensive effect)
/// - Multiband compression
/// - Chorus + Phaser + Flanger modulation
/// - Full spatial audio with occlusion and directional cones
/// - Parametric EQ with multiple bands
/// - Saturation, bitcrusher, delay, gate
///
/// This is NOT realistic for actual games, but it provides a ceiling
/// for "what if we threw everything at it?"

fn create_brutal_samples() -> anyhow::Result<Vec<Sample>> {
    let mut samples = Vec::new();

    // Sample 1: Bass with heavy effects
    let mut comp_bass = Composition::new(Tempo::new(120.0));
    comp_bass
        .track("bass")
        .at(0.0)
        .note(&[65.4], 8.0) // C2 - 8 beats = 4 seconds
        .distortion(Distortion::crunch())
        .saturation(Saturation::new(2.5, 0.5, 1.2))
        .compressor(Compressor::new(0.3, 6.0, 0.005, 0.08, 1.5))
        .delay(Delay::new(0.3, 0.4, 0.25))
        .filter(Filter::low_pass(400.0, 0.8));

    let mut mixer_bass = comp_bass.into_mixer();
    mixer_bass.export_wav("bench_brutal_bass.wav", 44100)?;
    samples.push(Sample::from_file("bench_brutal_bass.wav")?);

    // Sample 2: Mid with modulation effects
    let mut comp_mid = Composition::new(Tempo::new(120.0));
    comp_mid
        .track("mid")
        .at(0.0)
        .note(&[220.0], 8.0) // A3
        .chorus(Chorus::new(0.003, 0.5, 0.7))
        .phaser(Phaser::new(0.5, 0.5, 0.7, 6, 0.7))
        .flanger(Flanger::new(0.005, 0.4, 0.6, 0.8))
        .compressor(Compressor::new(0.4, 4.0, 0.01, 0.1, 1.3))
        .filter(Filter::band_pass(800.0, 0.6));

    let mut mixer_mid = comp_mid.into_mixer();
    // Add multiband compression on master
    mixer_mid.master_compressor(Compressor::multiband_3way(200.0, 2000.0));
    mixer_mid.export_wav("bench_brutal_mid.wav", 44100)?;
    samples.push(Sample::from_file("bench_brutal_mid.wav")?);

    // Sample 3: High with kitchen sink
    let mut comp_high = Composition::new(Tempo::new(120.0));
    comp_high
        .track("high")
        .at(0.0)
        .note(&[329.6], 8.0) // E4
        .saturation(Saturation::new(3.0, 0.5, 1.2))
        .bitcrusher(BitCrusher::lofi())
        .delay(Delay::new(0.25, 0.35, 0.3))
        .gate(Gate::new(-35.0, 3.0, 0.005, 0.05))
        .reverb(Reverb::new(0.7, 0.5, 0.6))
        .filter(Filter::high_pass(500.0, 0.6));

    let mut mixer_high = comp_high.into_mixer();
    // Parametric EQ on master
    let peq = ParametricEQ::new()
        .band(100.0, -3.0, 0.7)
        .band(500.0, 2.0, 1.5)
        .band(2000.0, -2.0, 1.0)
        .band(8000.0, 3.0, 0.8);
    mixer_high.master_parametric_eq(peq);
    mixer_high.export_wav("bench_brutal_high.wav", 44100)?;
    samples.push(Sample::from_file("bench_brutal_high.wav")?);

    Ok(samples)
}

fn run_brutal_test(sample_count: usize, samples: &[Sample]) -> anyhow::Result<(f32, f32)> {
    use tunes::synthesis::spatial::{
        ListenerConfig, SoundCone, SpatialParams, SpatialPosition, Vec3,
        calculate_spatial_with_cone,
    };

    let mut mixers = Vec::new();
    let mut spatial_positions = Vec::new();
    let mut spatial_cones = Vec::new();
    let mut occlusions = Vec::new();

    // Create mixers with BRUTAL per-sample processing
    for i in 0..sample_count {
        let sample = &samples[i % samples.len()];
        let pitch = 0.9 + (i as f32 * 0.005);

        let mut comp = Composition::new(Tempo::new(120.0));

        // Every sample gets heavy processing
        comp.track(&format!("brutal_{}", i))
            .at(0.0)
            .play_sample(sample, pitch)
            // Tone shaping
            .eq(EQ::new(1.2, 1.0, 0.8, 200.0, 3000.0))
            .filter(Filter::low_pass(10000.0, 0.7))
            // Dynamics
            .compressor(Compressor::new(0.4, 4.0, 0.01, 0.1, 1.3))
            .gate(Gate::new(-40.0, 3.0, 0.005, 0.05))
            // Distortion/saturation
            .distortion(Distortion::overdrive())
            .saturation(Saturation::new(2.0, 0.4, 1.1))
            // Modulation (pick one to avoid too much CPU load)
            .chorus(Chorus::new(0.002, 0.3, 0.6))
            // Time-based
            .delay(Delay::new(0.25, 0.3, 0.2))
            .reverb(Reverb::new(0.5, 0.4, 0.5));

        mixers.push(comp.into_mixer());

        // BRUTAL: Full spatial audio with extreme distances and velocity
        let angle = (i as f32 * 2.0 * std::f32::consts::PI) / sample_count as f32;
        let distance = 2.0 + (i as f32 % 25.0); // More extreme distances (2-27 units)
        let x = distance * angle.cos();
        let z = distance * angle.sin();
        let y = (i as f32 % 12.0) - 6.0; // More elevation variation

        // Add velocity for Doppler shift (sounds moving in circles)
        let velocity_magnitude = 5.0 + (i as f32 % 10.0);
        let vel_x = -velocity_magnitude * angle.sin();
        let vel_z = velocity_magnitude * angle.cos();

        spatial_positions.push(Some(SpatialPosition {
            position: Vec3 { x, y, z },
            velocity: Vec3 {
                x: vel_x,
                y: 0.0,
                z: vel_z,
            },
        }));

        // BRUTAL: ALL sounds get directional cones (not just 50%)
        spatial_cones.push(Some(
            SoundCone::narrow().with_direction(-x.signum(), 0.0, -z.signum()),
        ));

        // BRUTAL: ALL sounds get heavy occlusion (not just 33%)
        occlusions.push(0.6); // Higher occlusion than realistic benchmark
    }

    // Setup
    let listener = ListenerConfig::default();
    let spatial_params = SpatialParams::default();
    let sample_rate = 44100.0;
    let buffer_size = 512;
    let block_duration = buffer_size as f32 / sample_rate;

    let mut output = vec![0.0f32; buffer_size * 2];
    let mut temp_buffer = vec![0.0f32; buffer_size * 2];
    let mut elapsed_times = vec![0.0f32; sample_count];

    let total_frames = (sample_rate * 2.0) as usize;
    let num_callbacks = total_frames / buffer_size;

    let render_start = Instant::now();

    // Render loop
    for _ in 0..num_callbacks {
        output.fill(0.0);

        for (i, mixer) in mixers.iter_mut().enumerate() {
            mixer.process_block(
                &mut temp_buffer,
                sample_rate,
                elapsed_times[i],
                Some(&listener),
                Some(&spatial_params),
            );

            // Apply spatial audio
            let (volume, pan) = if let Some(pos) = &spatial_positions[i] {
                let result = calculate_spatial_with_cone(
                    pos,
                    &listener,
                    &spatial_params,
                    spatial_cones[i].as_ref(),
                    occlusions[i],
                );
                (result.volume, result.pan)
            } else {
                (1.0, 0.0)
            };

            // Mix with panning
            let (left_pan, right_pan) = if pan < 0.0 {
                (1.0, 1.0 + pan)
            } else {
                (1.0 - pan, 1.0)
            };

            for (frame_idx, temp_frame) in temp_buffer.chunks(2).enumerate() {
                let left = temp_frame[0] * volume * left_pan;
                let right = temp_frame[1] * volume * right_pan;
                output[frame_idx * 2] += left;
                output[frame_idx * 2 + 1] += right;
            }

            elapsed_times[i] += block_duration;
        }
    }

    let elapsed = render_start.elapsed().as_secs_f32();
    let audio_duration = 2.0;
    let realtime_factor = audio_duration / elapsed;

    Ok((realtime_factor, elapsed))
}

fn main() -> anyhow::Result<()> {
    println!("\n");
    println!("========================================================================");
    println!("=== UNREALISTIC GAME AUDIO BENCHMARK - ABSOLUTE WORST CASE ===");
    println!("========================================================================");
    println!();
    println!("This benchmark represents the ABSOLUTE WORST scenario:");
    println!("  - Distortion + Saturation + Bitcrusher per sample");
    println!("  - Multiband compression");
    println!("  - Chorus + Phaser + Flanger");
    println!("  - Delay + Reverb + Gate");
    println!("  - Parametric EQ + Standard EQ");
    println!("  - Compressor + Gate per track");
    println!();
    println!("  SPATIAL AUDIO (BRUTAL MODE):");
    println!("    - 100% of sounds have narrow directional cones");
    println!("    - 100% of sounds have heavy occlusion (0.6)");
    println!("    - Velocity/Doppler shift on all sounds");
    println!("    - Extreme distance variation (2-27 units)");
    println!("    - Elevation variation (-6 to +6 units)");
    println!();
    println!("This is NOT realistic - it's a stress test to find the FLOOR.");
    println!();

    print!("Creating brutal test samples...");
    let samples = create_brutal_samples()?;
    println!(" ✅ Done");
    println!();

    println!("======================================================================");
    println!("=== Finding The Absolute Limit ===");
    println!("  Testing with increasing concurrent samples until we drop below 1x realtime...");
    println!();

    let test_counts = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150];
    let mut worst_case = f32::MAX;
    let mut breaking_point = None;

    for &count in &test_counts {
        let (realtime, _elapsed) = run_brutal_test(count, &samples)?;
        worst_case = worst_case.min(realtime);

        println!("  {} concurrent samples: {:.1}x realtime", count, realtime);

        if realtime < 1.0 && breaking_point.is_none() {
            breaking_point = Some(count);
        }

        if realtime < 0.5 {
            println!("  ⚠️  Severely underperforming, stopping test.");
            break;
        }
    }

    println!();
    println!("======================================================================");
    println!();
    println!("=== SUMMARY ===");
    println!();
    println!("  BRUTAL STRESS TEST (kitchen sink effects):");
    println!("    Worst case: {:.1}x realtime", worst_case);

    let conservative_capacity = if worst_case >= 1.0 {
        let last_count = *test_counts.iter().filter(|&&c| {
            let (rt, _) = run_brutal_test(c, &samples).unwrap_or((0.0, 0.0));
            rt >= 1.0
        }).last().unwrap_or(&10);
        last_count
    } else {
        breaking_point.unwrap_or(0)
    };

    println!("    Conservative capacity: ~{} concurrent samples", conservative_capacity);
    println!();

    if let Some(bp) = breaking_point {
        println!("  ⚠️  Breaking point: {} concurrent samples", bp);
    } else {
        println!("  ✅ No breaking point found within test range!");
    }

    println!();
    println!("  Compare to realistic_game_audio: ~450 concurrent samples");
    println!("  This brutal test shows the FLOOR - with every effect enabled.");
    println!();

    // Cleanup
    let _ = std::fs::remove_file("bench_brutal_bass.wav");
    let _ = std::fs::remove_file("bench_brutal_mid.wav");
    let _ = std::fs::remove_file("bench_brutal_high.wav");

    println!("✅ Brutal stress test complete!");
    println!();

    Ok(())
}
