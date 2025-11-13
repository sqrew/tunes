use tunes::prelude::*;

/// Demonstrates multiband compression for mastering
fn main() -> anyhow::Result<()> {
    println!("🎚️  Multiband Compression Demo\n");

    let engine = AudioEngine::new()?;

    // Helper function to create test mix
    fn create_mix() -> Composition {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Bass: C1-C2 range (32-65 Hz)
        comp.instrument("bass", &Instrument::sub_bass())
            .notes(&[C1, C1, G1, C1, C1, E2, C1, C1], 0.5);

        // Mids: Pad in middle register (261-523 Hz)
        comp.instrument("pad", &Instrument::warm_pad())
            .at(0.0)
            .notes(&[C3, E3, G3], 4.0);

        // Highs: Lead melody (523-1046 Hz)
        comp.instrument("lead", &Instrument::synth_lead())
            .at(0.0)
            .notes(&[C5, E5, G5, C6, G5, E5, C5, G4], 0.5);

        comp
    }

    println!("Creating test mix with bass, mids, and highs...\n");

    println!("Example 1: Standard single-band compression\n");
    println!("  Applying uniform compression across all frequencies");
    println!("  Problem: Bass gets squashed same as highs\n");

    let mut mix1 = create_mix();
    mix1.track("master")
        .compressor(Compressor::new(0.4, 3.0, 0.01, 0.1, 1.2));

    engine.export_wav(&mut mix1.into_mixer(), "/tmp/single_band.wav")?;
    println!("  ✓ Exported: /tmp/single_band.wav\n");

    println!("Example 2: 3-band multiband compression\n");
    println!("  Bass (0-200Hz):   Tight control (threshold=0.3, ratio=4.0)");
    println!("  Mids (200-2kHz):  Gentle (threshold=0.5, ratio=2.5)");
    println!("  Highs (2k-20kHz): Transparent (threshold=0.6, ratio=2.0)\n");

    let mut mix2 = create_mix();
    mix2.track("master")
        .compressor(
            Compressor::multiband_3way(200.0, 2000.0)
                .with_band_low(0.3, 4.0)    // Tight bass control
                .with_band_mid(0.5, 2.5)    // Gentle mids
                .with_band_high(0.6, 2.0)   // Transparent highs
        );

    engine.export_wav(&mut mix2.into_mixer(), "/tmp/multiband_3way.wav")?;
    println!("  ✓ Exported: /tmp/multiband_3way.wav\n");

    println!("Example 3: Custom 5-band configuration\n");
    println!("  Sub bass (0-80Hz):    Very tight (ratio=5.0)");
    println!("  Bass (80-250Hz):      Moderate (ratio=3.5)");
    println!("  Mids (250-2kHz):      Gentle (ratio=2.5)");
    println!("  Presence (2k-8kHz):   Transparent (ratio=2.0)");
    println!("  Air (8k-20kHz):       Very gentle (ratio=1.5)\n");

    let bands = vec![
        CompressorBand::new(0.0, 80.0,
            Compressor::new(0.25, 5.0, 0.003, 0.03, 1.0)), // Sub bass - aggressive
        CompressorBand::new(80.0, 250.0,
            Compressor::new(0.35, 3.5, 0.005, 0.05, 1.0)), // Bass - moderate
        CompressorBand::new(250.0, 2000.0,
            Compressor::new(0.5, 2.5, 0.01, 0.1, 1.0)),    // Mids - gentle
        CompressorBand::new(2000.0, 8000.0,
            Compressor::new(0.6, 2.0, 0.01, 0.1, 1.0)),    // Presence - transparent
        CompressorBand::new(8000.0, 20000.0,
            Compressor::new(0.7, 1.5, 0.02, 0.15, 1.0)),   // Air - very gentle
    ];

    let mut mix3 = create_mix();
    mix3.track("master")
        .compressor(Compressor::new(0.5, 1.0, 0.01, 0.1, 1.0)
            .with_multibands(bands));

    engine.export_wav(&mut mix3.into_mixer(), "/tmp/multiband_5band.wav")?;
    println!("  ✓ Exported: /tmp/multiband_5band.wav\n");

    println!("Example 4: Targeted bass compression only\n");
    println!("  Only compressing 40-150Hz range");
    println!("  Rest of spectrum untouched\n");

    let mut mix4 = create_mix();
    mix4.track("master")
        .compressor(
            Compressor::new(0.5, 1.0, 0.01, 0.1, 1.0)
                .with_multiband(CompressorBand::new(
                    40.0, 150.0,
                    Compressor::new(0.3, 4.0, 0.005, 0.05, 1.0)
                ))
        );

    engine.export_wav(&mut mix4.into_mixer(), "/tmp/multiband_bass_only.wav")?;
    println!("  ✓ Exported: /tmp/multiband_bass_only.wav\n");

    println!("✅ Demo complete!\n");
    println!("📂 Compare the exported files:");
    println!("   /tmp/single_band.wav        - Standard compression");
    println!("   /tmp/multiband_3way.wav     - 3-band split");
    println!("   /tmp/multiband_5band.wav    - 5-band professional mastering");
    println!("   /tmp/multiband_bass_only.wav - Targeted bass control\n");

    println!("🎧 Listen for:");
    println!("   • Single-band: Pumping/breathing artifacts");
    println!("   • Multiband: Each frequency range controlled independently");
    println!("   • Bass-only: Punchy lows, unaffected highs\n");

    Ok(())
}
