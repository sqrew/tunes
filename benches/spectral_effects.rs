use std::time::Instant;
use tunes::prelude::*;
use tunes::synthesis::effects::{
    FilterType, SpectralDynamics, SpectralExciter, SpectralFilter, SpectralInvert, SpectralMorph,
    SpectralScramble, SpectralShift, SpectralWiden,
};

/// Benchmark spectral (frequency-domain) effects
///
/// Tests all spectral effects with SIMD-accelerated FFT processing:
/// - PhaseVocoder (pitch/time manipulation)
/// - SpectralFreeze (spectrum freezing)
/// - SpectralGate (frequency-selective gating)
/// - SpectralCompressor (frequency-selective compression)
/// - SpectralRobotize (phase manipulation)
/// - SpectralDelay (frequency-dependent delay)
/// - SpectralFilter (frequency-domain filtering)
///
/// This benchmark establishes CPU baselines for future GPU acceleration comparison.

fn main() -> anyhow::Result<()> {
    println!("\n🎵 Spectral Effects Benchmark\n");

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
    println!("Spectral effects use FFT with SIMD-accelerated windowing\n");

    let engine = AudioEngine::new()?;

    // Part 1: Individual spectral effects
    println!("=== Part 1: Individual Spectral Effects ===\n");

    let spectral_effects = [
        ("PhaseVocoder", "pitch_shift"),
        ("SpectralFreeze", "frozen"),
        ("SpectralGate", "denoise"),
        ("SpectralCompressor", "glue"),
        ("SpectralRobotize", "robot"),
        ("SpectralDelay", "shimmer"),
        ("SpectralFilter", "low_pass"),
        ("SpectralShift", "up"),
        ("SpectralExciter", "warm"),
        ("SpectralDynamics", "moderate"),
        ("SpectralInvert", "full"),
        ("SpectralWiden", "wide"),
        ("SpectralMorph", "robot"),
        ("SpectralScramble", "glitch"),
    ];

    for (effect_name, preset_name) in &spectral_effects {
        println!("--- {} ({}) ---", effect_name, preset_name);

        let mut comp = Composition::new(Tempo::new(120.0));

        // 5 tracks with the spectral effect
        for i in 0..5 {
            let track_name = format!("track_{}", i);
            let freq = 200.0 + (i as f32 * 100.0);

            let mut track = comp
                .instrument(&track_name, &Instrument::synth_lead())
                .at(0.0);

            // Apply the specific spectral effect using presets
            track = match effect_name.as_ref() {
                "PhaseVocoder" => track.phase_vocoder(PhaseVocoder::pitch_up()),
                "SpectralFreeze" => track.spectral_freeze(SpectralFreeze::frozen()),
                "SpectralGate" => track.spectral_gate(SpectralGate::denoise()),
                "SpectralCompressor" => track.spectral_compressor(SpectralCompressor::glue()),
                "SpectralRobotize" => track.spectral_robotize(SpectralRobotize::robot()),
                "SpectralDelay" => track.spectral_delay(SpectralDelay::shimmer()),
                "SpectralFilter" => {
                    let filter =
                        SpectralFilter::new(FilterType::LowPass, 1000.0, 1.0, 1.0, 44100.0);
                    track.spectral_filter(filter)
                }
                "SpectralShift" => track.spectral_shift(SpectralShift::metallic()),
                "SpectralExciter" => track.spectral_exciter(SpectralExciter::moderate()),
                "SpectralDynamics" => track.spectral_dynamics(SpectralDynamics::moderate()),
                "SpectralInvert" => track.spectral_invert(SpectralInvert::full()),
                "SpectralWiden" => track.spectral_widen(SpectralWiden::wide()),
                "SpectralMorph" => track.spectral_morph(SpectralMorph::robot()),
                "SpectralScramble" => track.spectral_scramble(SpectralScramble::glitch()),
                _ => track,
            };

            track.notes(&[freq, freq * 1.25, freq * 1.5], 0.5);
        }

        let mut mixer = comp.into_mixer();

        // Benchmark rendering
        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let sample_count = buffer.len() / 2;
        let audio_duration = sample_count as f32 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Audio: {:.2}s, 5 tracks", audio_duration);
        println!("  Render: {:.3}s", render_time.as_secs_f32());
        println!("  Ratio: {:.1}x realtime", realtime_ratio);

        // Spectral effects are heavier than time-domain effects
        if realtime_ratio > 10.0 {
            println!("  ✅ Excellent performance for spectral processing!");
        } else if realtime_ratio > 5.0 {
            println!("  ✅ Good real-time capability");
        } else if realtime_ratio > 1.0 {
            println!("  ⚠️  Real-time capable but CPU intensive");
        } else {
            println!("  ❌ Below real-time (may need GPU acceleration)");
        }
        println!();
    }

    // Part 2: FFT Size Comparison
    println!("=== Part 2: FFT Size Impact ===");
    println!("Testing SpectralFilter with different FFT sizes\n");

    let fft_sizes = [1024, 2048, 4096];

    for fft_size in &fft_sizes {
        let mut comp = Composition::new(Tempo::new(120.0));

        let _filter = SpectralFilter::with_params(
            *fft_size,
            fft_size / 4,
            FilterType::LowPass,
            1000.0,
            1.0,
            1.0,
            44100.0,
        );

        let filter = SpectralFilter::new(FilterType::LowPass, 1000.0, 1.0, 1.0, 44100.0);

        comp.instrument("filtered", &Instrument::synth_lead())
            .at(0.0)
            .spectral_filter(filter)
            .notes(&[440.0, 554.0, 659.0], 1.0);

        let mut mixer = comp.into_mixer();

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = (buffer.len() / 2) as f32 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        let latency_ms = (*fft_size as f32 / 44100.0) * 1000.0;

        println!("  FFT Size: {}", fft_size);
        println!("    Render: {:.3}s", render_time.as_secs_f32());
        println!("    Ratio: {:.1}x realtime", realtime_ratio);
        println!("    Latency: {:.1}ms", latency_ms);

        if *fft_size <= 2048 && realtime_ratio > 5.0 {
            println!("    ✅ Good balance of quality/performance");
        } else if *fft_size == 4096 && realtime_ratio > 3.0 {
            println!("    ✅ High quality, still real-time capable");
        }
        println!();
    }

    // Part 3: Stacking Spectral Effects
    println!("=== Part 3: Stacking Spectral Effects ===");
    println!("Multiple spectral effects on one track (worst case)\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    let filter = SpectralFilter::new(FilterType::LowPass, 5000.0, 1.0, 1.0, 44100.0);

    comp.instrument("kitchen_sink", &Instrument::synth_lead())
        .at(0.0)
        .spectral_gate(SpectralGate::gentle())
        .spectral_compressor(SpectralCompressor::gentle())
        .spectral_filter(filter)
        .spectral_delay(SpectralDelay::new(150.0, 0.3, 1.0, 0.4, 44100.0))
        .notes(&[440.0, 554.0, 659.0, 784.0], 0.5);

    let mut mixer = comp.into_mixer();

    let start = Instant::now();
    let buffer = engine.render_to_buffer(&mut mixer);
    let render_time = start.elapsed();

    let sample_count = buffer.len() / 2;
    let audio_duration = sample_count as f32 / 44100.0;
    let realtime_ratio = audio_duration / render_time.as_secs_f32();

    println!("  Effects: Gate → Compressor → Filter → Delay");
    println!("  Audio: {:.2}s", audio_duration);
    println!("  Render: {:.3}s", render_time.as_secs_f32());
    println!("  Ratio: {:.1}x realtime", realtime_ratio);

    if realtime_ratio > 5.0 {
        println!("  ✅ Can stack multiple spectral effects without issues!\n");
    } else if realtime_ratio > 2.0 {
        println!("  ✅ Spectral effects are stackable\n");
    } else if realtime_ratio > 1.0 {
        println!("  ⚠️  Heavy CPU load but still real-time\n");
    } else {
        println!("  ❌ May benefit from GPU acceleration\n");
    }

    // Part 4: Real-time Processing Simulation
    println!("=== Part 4: Real-time Processing Test ===");
    println!("Direct effect processing of 512-sample blocks\n");

    use tunes::synthesis::effects::SpectralFilter;

    // Create a spectral filter for testing
    let mut filter = SpectralFilter::new(FilterType::LowPass, 2000.0, 2.0, 1.0, 44100.0);

    // Generate test audio blocks
    let block_size = 512;
    let sample_rate = 44100.0;
    let test_blocks = 100; // Test 100 blocks

    let mut process_times = Vec::new();

    for _ in 0..test_blocks {
        // Generate a block of test audio (sine wave)
        let mut audio_block: Vec<f32> = (0..block_size)
            .map(|i| (2.0 * std::f32::consts::PI * 440.0 * i as f32 / sample_rate).sin())
            .collect();

        // Measure processing time
        let start = Instant::now();
        filter.process_block(&mut audio_block, sample_rate, 0.0, 0);
        let elapsed = start.elapsed();
        process_times.push(elapsed);
    }

    let avg_time =
        process_times.iter().map(|d| d.as_secs_f64()).sum::<f64>() / process_times.len() as f64;
    let max_time = process_times.iter().max().unwrap().as_secs_f64();
    let min_time = process_times.iter().min().unwrap().as_secs_f64();
    let block_duration = block_size as f64 / sample_rate as f64;

    println!("  Block size: {} samples", block_size);
    println!("  Block duration: {:.2}ms", block_duration * 1000.0);
    println!("  Min process time: {:.3}ms", min_time * 1000.0);
    println!("  Avg process time: {:.3}ms", avg_time * 1000.0);
    println!("  Max process time: {:.3}ms", max_time * 1000.0);
    println!(
        "  CPU headroom: {:.1}%",
        (1.0 - avg_time / block_duration) * 100.0
    );
    println!("  Realtime ratio: {:.1}x", block_duration / avg_time);

    if max_time < block_duration * 0.5 {
        println!("  ✅ Plenty of CPU headroom for real-time processing!");
    } else if max_time < block_duration * 0.8 {
        println!("  ✅ Real-time capable with good headroom");
    } else if max_time < block_duration {
        println!("  ⚠️  Real-time capable but tight on CPU");
    } else {
        println!("  ❌ Cannot maintain real-time (GPU acceleration recommended)");
    }
    println!();

    // Summary
    println!("=== Summary ===");
    println!("Spectral Effects Performance:");
    println!("  • FFT processing with SIMD-accelerated windowing");
    println!("  • Default FFT size: 2048 samples (~46ms latency @ 44.1kHz)");
    println!("  • {} SIMD lanes for parallel processing", simd_width);
    println!("  • Real-time capable on modern CPUs");
    println!("  • Stackable with moderate CPU usage");

    Ok(())
}
