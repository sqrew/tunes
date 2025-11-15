use tunes::prelude::*;
use std::time::Instant;

/// Benchmark export/offline rendering speed
///
/// Measures how fast compositions can be rendered to disk.
/// Important for batch processing, automation, and offline workflows.

fn main() -> anyhow::Result<()> {
    println!("\n💾 Export Speed Benchmark\n");

    // Create a 30-second composition with realistic complexity
    let mut comp = Composition::new(Tempo::new(120.0));

    println!("Creating 30-second composition...");

    // 5 tracks with various instruments and effects
    comp.instrument("lead", &Instrument::synth_lead())
        .at(0.0)
        .filter(Filter::low_pass(2000.0, 0.7))
        .reverb(Reverb::new(0.5, 0.4, 0.3))
        .notes(&[440.0, 494.0, 523.0, 587.0], 0.5)
        .repeat(60); // 30 seconds

    comp.instrument("bass", &Instrument::sub_bass())
        .at(0.0)
        .compressor(Compressor::new(0.5, 4.0, 0.01, 0.1, 1.2))
        .notes(&[110.0, 123.0, 131.0, 147.0], 1.0)
        .repeat(30);

    comp.instrument("pad", &Instrument::warm_pad())
        .at(0.0)
        .filter(Filter::low_pass(1000.0, 0.6))
        .reverb(Reverb::new(0.8, 0.5, 0.4))
        .note(&[220.0, 277.0, 330.0], 30.0);

    comp.track("drums")
        .at(0.0)
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14])
        .repeat(60);

    comp.instrument("fx", &Instrument::pluck())
        .at(0.0)
        .delay(Delay::new(0.25, 0.5, 0.4))
        .notes(&[880.0, 988.0, 1047.0], 0.25)
        .repeat(120);

    let mut mixer = comp.into_mixer();
    let duration = mixer.total_duration();

    println!("  ✓ Created {:.1}s composition with 5 tracks\n", duration);

    // Benchmark WAV export
    println!("=== WAV Export ===");
    let start = Instant::now();
    mixer.export_wav("/tmp/bench_export.wav", 44100)?;
    let wav_time = start.elapsed();
    let wav_ratio = duration / wav_time.as_secs_f32();

    println!("  Export time: {:.3}s", wav_time.as_secs_f32());
    println!("  Realtime ratio: {:.1}x", wav_ratio);
    println!("  Equivalent: {:.0} minutes of audio per second\n", wav_ratio / 60.0);

    // Benchmark FLAC export
    println!("=== FLAC Export ===");
    let start = Instant::now();
    mixer.export_flac("/tmp/bench_export.flac", 44100)?;
    let flac_time = start.elapsed();
    let flac_ratio = duration / flac_time.as_secs_f32();

    println!("  Export time: {:.3}s", flac_time.as_secs_f32());
    println!("  Realtime ratio: {:.1}x", flac_ratio);
    println!("  Equivalent: {:.0} minutes of audio per second\n", flac_ratio / 60.0);

    // GPU-accelerated export (if available)
    #[cfg(feature = "gpu")]
    {
        println!("=== GPU-Accelerated WAV Export 🚀 ===");

        // Create same composition again
        let mut comp_gpu = Composition::new(Tempo::new(120.0));
        comp_gpu.instrument("lead", &Instrument::synth_lead())
            .at(0.0)
            .filter(Filter::low_pass(2000.0, 0.7))
            .reverb(Reverb::new(0.5, 0.4, 0.3))
            .notes(&[440.0, 494.0, 523.0, 587.0], 0.5)
            .repeat(60);
        comp_gpu.instrument("bass", &Instrument::sub_bass())
            .at(0.0)
            .compressor(Compressor::new(0.5, 4.0, 0.01, 0.1, 1.2))
            .notes(&[110.0, 123.0, 131.0, 147.0], 1.0)
            .repeat(30);
        comp_gpu.instrument("pad", &Instrument::warm_pad())
            .at(0.0)
            .filter(Filter::low_pass(1000.0, 0.6))
            .reverb(Reverb::new(0.8, 0.5, 0.4))
            .note(&[220.0, 277.0, 330.0], 30.0);
        comp_gpu.track("drums")
            .at(0.0)
            .drum_grid(16, 0.125)
            .kick(&[0, 4, 8, 12])
            .snare(&[4, 12])
            .hihat(&[0, 2, 4, 6, 8, 10, 12, 14])
            .repeat(60);
        comp_gpu.instrument("fx", &Instrument::pluck())
            .at(0.0)
            .delay(Delay::new(0.25, 0.5, 0.4))
            .notes(&[880.0, 988.0, 1047.0], 0.25)
            .repeat(120);

        let mut mixer_gpu = comp_gpu.into_mixer();
        mixer_gpu.enable_gpu();

        let start = Instant::now();
        mixer_gpu.export_wav("/tmp/bench_export_gpu.wav", 44100)?;
        let gpu_time = start.elapsed();
        let gpu_ratio = duration / gpu_time.as_secs_f32();
        let speedup = gpu_ratio / wav_ratio;

        println!("  Export time: {:.3}s", gpu_time.as_secs_f32());
        println!("  Realtime ratio: {:.1}x", gpu_ratio);
        println!("  Equivalent: {:.0} minutes of audio per second", gpu_ratio / 60.0);
        println!("  🚀 Speedup vs CPU: {:.1}x faster\n", speedup);

        fs::remove_file("/tmp/bench_export_gpu.wav").ok();
    }

    // Compare file sizes
    use std::fs;
    let wav_size = fs::metadata("/tmp/bench_export.wav")?.len();
    let flac_size = fs::metadata("/tmp/bench_export.flac")?.len();
    let compression = 100.0 - (flac_size as f64 / wav_size as f64 * 100.0);

    println!("=== File Size Comparison ===");
    println!("  WAV:  {:.1} MB", wav_size as f64 / 1_000_000.0);
    println!("  FLAC: {:.1} MB", flac_size as f64 / 1_000_000.0);
    println!("  Space saved: {:.1}%\n", compression);

    // Cleanup
    fs::remove_file("/tmp/bench_export.wav").ok();
    fs::remove_file("/tmp/bench_export.flac").ok();

    println!("=== Summary ===");
    if wav_ratio > 100.0 {
        println!("✅ Excellent export speed! Can batch process many files quickly");
    } else if wav_ratio > 50.0 {
        println!("✅ Good export speed for most workflows");
    } else if wav_ratio > 10.0 {
        println!("✅ Acceptable for offline rendering");
    } else {
        println!("⚠️  Slow export - may want to optimize composition complexity");
    }

    println!("\nUse cases:");
    println!("  • Batch rendering: Generate 100 files overnight");
    println!("  • Algorithmic music: Create variations in seconds");
    println!("  • Game audio: Pre-render dynamic music stems\n");

    Ok(())
}
