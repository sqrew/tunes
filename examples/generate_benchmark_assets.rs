use tunes::prelude::*;
use anyhow::Result;

/// Generates diverse, realistic audio samples for benchmarking
///
/// Creates 50+ samples with varying characteristics:
/// - Short transients (footsteps, impacts, gunshots)
/// - Medium sounds (explosions, voices)
/// - Sustained sounds (ambient loops, drones)
/// - Varied spectral content (bass, mid, high)
/// - Complex waveforms (not just simple tones)

fn main() -> Result<()> {
    println!("\n🎵 Generating Benchmark Assets\n");
    println!("Creating diverse audio samples for realistic benchmarking...\n");

    let output_dir = "benches/assets";
    std::fs::create_dir_all(output_dir)?;

    let mut count = 0;

    // ============================================================================
    // SHORT TRANSIENTS (0.1-0.5s) - Footsteps, Impacts, UI sounds
    // ============================================================================
    println!("📦 Generating short transients (footsteps, impacts)...");

    // Footsteps - 8 variations
    for i in 0..8 {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Sharp attack, quick decay - like a footstep
        let freq = 80.0 + (i as f32 * 15.0);
        comp.instrument("impact", &Instrument::pluck())
            .at(0.0)
            .note(&[freq, freq * 1.5, freq * 2.3], 0.15) // Complex harmonic content
            .filter(Filter::band_pass(200.0 + (i as f32 * 50.0), 0.8))
            .volume(0.7);

        let mut mixer = comp.into_mixer();
        mixer.export_wav(&format!("{}/footstep_{:02}.wav", output_dir, i + 1), 44100)?;
        count += 1;
    }

    // Impacts/Hits - 6 variations (heavier than footsteps)
    for i in 0..6 {
        let mut comp = Composition::new(Tempo::new(120.0));

        let freq = 40.0 + (i as f32 * 10.0);
        comp.instrument("hit", &Instrument::pluck())
            .at(0.0)
            .note(&[freq, freq * 2.1, freq * 3.7, freq * 5.2], 0.25)
            .distortion(Distortion::new(0.3, 0.5))
            .filter(Filter::low_pass(800.0, 0.6))
            .volume(0.8);

        let mut mixer = comp.into_mixer();
        mixer.export_wav(&format!("{}/impact_{:02}.wav", output_dir, i + 1), 44100)?;
        count += 1;
    }

    // ============================================================================
    // GUNSHOTS/EXPLOSIONS (0.5-2s) - Sharp attack, reverb tail
    // ============================================================================
    println!("💥 Generating gunshots and explosions...");

    // Gunshots - 6 variations
    for i in 0..6 {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Very sharp attack with noise-like character
        let base_freq = 100.0 + (i as f32 * 20.0);
        comp.instrument("shot", &Instrument::synth_lead())
            .at(0.0)
            .note(&[base_freq, base_freq * 2.3, base_freq * 4.7, base_freq * 7.1], 0.8)
            .distortion(Distortion::crunch())
            .filter(Filter::high_pass(150.0, 0.7))
            .reverb(Reverb::new(0.3, 0.2, 0.4))
            .volume(0.9);

        let mut mixer = comp.into_mixer();
        mixer.export_wav(&format!("{}/gunshot_{:02}.wav", output_dir, i + 1), 44100)?;
        count += 1;
    }

    // Explosions - 5 variations (longer, more complex)
    for i in 0..5 {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Complex broadband noise with low-end punch
        let freq = 30.0 + (i as f32 * 8.0);
        comp.instrument("explosion", &Instrument::sub_bass())
            .at(0.0)
            .note(&[freq, freq * 1.7, freq * 2.9, freq * 4.3, freq * 6.1, freq * 8.7], 1.5)
            .distortion(Distortion::overdrive())
            .filter(Filter::low_pass(1200.0, 0.5))
            .reverb(Reverb::new(0.7, 0.6, 0.5))
            .delay(Delay::new(0.15, 0.25, 0.3))
            .volume(0.85);

        let mut mixer = comp.into_mixer();
        mixer.export_wav(&format!("{}/explosion_{:02}.wav", output_dir, i + 1), 44100)?;
        count += 1;
    }

    // ============================================================================
    // VOICE-LIKE SOUNDS (0.3-1.5s) - Complex harmonics, formants
    // ============================================================================
    println!("🗣️  Generating voice-like sounds...");

    for i in 0..6 {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Fundamental + harmonic series (like a voice)
        let fundamental = 120.0 + (i as f32 * 25.0);
        comp.instrument("voice", &Instrument::warm_pad())
            .at(0.0)
            .note(&[
                fundamental,
                fundamental * 2.0,
                fundamental * 3.0,
                fundamental * 4.0,
                fundamental * 5.0,
            ], 0.8)
            // Formant-like filtering (vowel coloration)
            .filter(Filter::band_pass(500.0 + (i as f32 * 200.0), 1.2))
            .chorus(Chorus::new(0.002, 0.3, 0.5)) // Natural vibrato
            .reverb(Reverb::new(0.2, 0.3, 0.4))
            .volume(0.6);

        let mut mixer = comp.into_mixer();
        mixer.export_wav(&format!("{}/voice_{:02}.wav", output_dir, i + 1), 44100)?;
        count += 1;
    }

    // ============================================================================
    // AMBIENT LOOPS (2-4s) - Sustained, evolving textures
    // ============================================================================
    println!("🌫️  Generating ambient loops...");

    // Wind/Atmosphere - 4 variations
    for i in 0..4 {
        let mut comp = Composition::new(Tempo::new(60.0)); // Slower for ambient

        let freq = 150.0 + (i as f32 * 100.0);
        comp.instrument("ambient", &Instrument::warm_pad())
            .at(0.0)
            .note(&[freq, freq * 1.3, freq * 1.7, freq * 2.1], 4.0)
            .phaser(Phaser::new(0.3, 0.4, 0.6, 4, 0.5)) // Slow movement
            .reverb(Reverb::new(0.8, 0.7, 0.6))
            .filter(Filter::high_pass(100.0, 0.5))
            .volume(0.4);

        let mut mixer = comp.into_mixer();
        mixer.export_wav(&format!("{}/ambient_{:02}.wav", output_dir, i + 1), 44100)?;
        count += 1;
    }

    // Machine/Engine sounds - 3 variations (rhythmic, sustained)
    for i in 0..3 {
        let mut comp = Composition::new(Tempo::new(120.0));

        let freq = 60.0 + (i as f32 * 20.0);
        comp.instrument("engine", &Instrument::sub_bass())
            .at(0.0)
            .note(&[freq, freq * 2.0, freq * 3.0], 3.0)
            .distortion(Distortion::overdrive())
            .filter(Filter::low_pass(400.0 + (i as f32 * 100.0), 0.8))
            .flanger(Flanger::new(0.004, 0.5, 0.7, 0.6)) // Mechanical vibration
            .volume(0.7);

        let mut mixer = comp.into_mixer();
        mixer.export_wav(&format!("{}/engine_{:02}.wav", output_dir, i + 1), 44100)?;
        count += 1;
    }

    // ============================================================================
    // BASS-HEAVY (sub-bass to mid) - For low-frequency performance testing
    // ============================================================================
    println!("🔊 Generating bass-heavy sounds...");

    for i in 0..4 {
        let mut comp = Composition::new(Tempo::new(120.0));

        let freq = 40.0 + (i as f32 * 15.0);
        comp.instrument("bass", &Instrument::sub_bass())
            .at(0.0)
            .note(&[freq, freq * 2.0], 1.5)
            .distortion(Distortion::overdrive())
            .filter(Filter::low_pass(200.0, 0.9))
            .compressor(Compressor::new(0.3, 6.0, 0.01, 0.1, 1.5))
            .volume(0.9);

        let mut mixer = comp.into_mixer();
        mixer.export_wav(&format!("{}/bass_{:02}.wav", output_dir, i + 1), 44100)?;
        count += 1;
    }

    // ============================================================================
    // HIGH-FREQUENCY - For treble/bright performance testing
    // ============================================================================
    println!("✨ Generating high-frequency sounds...");

    for i in 0..4 {
        let mut comp = Composition::new(Tempo::new(120.0));

        let freq = 1000.0 + (i as f32 * 500.0);
        comp.instrument("high", &Instrument::synth_lead())
            .at(0.0)
            .note(&[freq, freq * 1.5, freq * 2.0], 1.0)
            .filter(Filter::high_pass(800.0, 0.6))
            .reverb(Reverb::new(0.5, 0.4, 0.5))
            .volume(0.5);

        let mut mixer = comp.into_mixer();
        mixer.export_wav(&format!("{}/high_freq_{:02}.wav", output_dir, i + 1), 44100)?;
        count += 1;
    }

    println!("\n✅ Generated {} diverse audio samples in {}/", count, output_dir);
    println!("\nSample breakdown:");
    println!("  • 8 footsteps (0.15s, transient)");
    println!("  • 6 impacts (0.25s, transient)");
    println!("  • 6 gunshots (0.8s, sharp attack + reverb)");
    println!("  • 5 explosions (1.5s, complex broadband)");
    println!("  • 6 voice-like (0.8s, harmonic + formants)");
    println!("  • 4 ambient loops (4s, sustained texture)");
    println!("  • 3 engine sounds (3s, rhythmic sustained)");
    println!("  • 4 bass-heavy (1.5s, sub-bass focus)");
    println!("  • 4 high-frequency (1s, treble focus)");
    println!("\nThese samples provide:");
    println!("  ✓ Varied lengths (0.15s - 4s)");
    println!("  ✓ Complex spectral content (not simple tones)");
    println!("  ✓ Different envelope shapes (transient vs sustained)");
    println!("  ✓ Diverse frequency ranges (40Hz - 4kHz fundamentals)");
    println!("  ✓ Cache pressure (46 unique samples won't all fit in L3)");
    println!("\nUse these in benchmarks for more realistic performance testing!\n");

    Ok(())
}
