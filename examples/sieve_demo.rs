// Sieve Transform Demo
//
// Demonstrates the .sieve_inclusive() and .sieve_exclusive() transforms
// for frequency-based note filtering and spectral sculpting.

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let engine = AudioEngine::new()?;

    println!("🎚️  Sieve Transform Demo\n");
    println!("Frequency-based filtering to isolate or remove specific ranges\n");

    // ==================== BASELINE - NO FILTERING ====================
    println!("🎵 BASELINE - Full range (no filtering):");
    comp.instrument("baseline", &Instrument::electric_piano())
        .pattern_start()
        .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25);

    // ==================== SIEVE_INCLUSIVE - BASS ONLY ====================
    println!("🔉 INCLUSIVE - Keep only bass frequencies (< 200 Hz):");
    comp.instrument("bass_only", &Instrument::sub_bass())
        .wait(2.0)
        .pattern_start()
        .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25)
        .sieve_inclusive(20.0, 200.0);  // Only C3, E3, G3 remain

    // ==================== SIEVE_INCLUSIVE - MIDRANGE ONLY ====================
    println!("🔊 INCLUSIVE - Keep only midrange (200-400 Hz):");
    comp.instrument("mid_only", &Instrument::electric_piano())
        .wait(4.0)
        .pattern_start()
        .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25)
        .sieve_inclusive(200.0, 400.0);  // Only C4, E4, G4 remain

    // ==================== SIEVE_INCLUSIVE - TREBLE ONLY ====================
    println!("🔉 INCLUSIVE - Keep only treble (> 400 Hz):");
    comp.instrument("treble_only", &Instrument::synth_lead())
        .wait(6.0)
        .pattern_start()
        .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25)
        .sieve_inclusive(400.0, 2000.0);  // Only G4, C5 remain

    // ==================== SIEVE_EXCLUSIVE - REMOVE MIDRANGE ====================
    println!("🎛️  EXCLUSIVE - Remove muddy midrange:");
    comp.instrument("no_mid", &Instrument::electric_piano())
        .wait(8.0)
        .pattern_start()
        .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25)
        .sieve_exclusive(200.0, 400.0);  // Remove C4, E4, G4

    // ==================== SIEVE_EXCLUSIVE - REMOVE BASS ====================
    println!("🎚️  EXCLUSIVE - Remove bass frequencies:");
    comp.instrument("no_bass", &Instrument::electric_piano())
        .wait(10.0)
        .pattern_start()
        .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25)
        .sieve_exclusive(20.0, 200.0);  // Remove C3, E3, G3

    // ==================== WITH TRANSFORM NAMESPACE ====================
    println!("📦 NAMESPACE - Using .transform() API:");
    comp.instrument("namespace", &Instrument::electric_piano())
        .wait(12.0)
        .pattern_start()
        .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25)
        .transform(|t| t
            .sieve_inclusive(150.0, 450.0)  // Keep midrange
        );

    // ==================== CHAINING SIEVES ====================
    println!("🔗 CHAINING - Multiple sieve operations:");
    comp.instrument("chained", &Instrument::electric_piano())
        .wait(14.0)
        .pattern_start()
        .notes(&[C2, C3, E3, G3, C4, E4, G4, C5, C6], 0.125)
        .transform(|t| t
            .sieve_exclusive(20.0, 150.0)   // Remove very low bass (C2)
            .sieve_exclusive(450.0, 2000.0) // Remove high treble (C5, C6)
        );  // Only C3, E3, G3, C4, E4, G4 remain

    // ==================== NOTCH FILTER EFFECT ====================
    println!("🎚️  NOTCH - Create frequency gap (spectral hole):");
    comp.instrument("notch", &Instrument::electric_piano())
        .wait(16.0)
        .pattern_start()
        .notes(&[C3, D3, E3, F3, G3, A3, B3, C4, D4, E4], 0.125)
        .sieve_exclusive(200.0, 280.0);  // Remove notes around 240Hz

    // ==================== SPLIT INTO LAYERS ====================
    println!("🎼 LAYERS - Split melody into bass and treble:");
    let melody = &[C3, E3, G3, C4, E4, G4, C5, E5];

    // Bass layer
    comp.instrument("layer_bass", &Instrument::sub_bass())
        .wait(18.0)
        .pattern_start()
        .notes(melody, 0.125)
        .sieve_inclusive(20.0, 200.0);

    // Treble layer (different instrument)
    comp.instrument("layer_treble", &Instrument::synth_lead())
        .wait(18.0)
        .pattern_start()
        .notes(melody, 0.125)
        .sieve_inclusive(400.0, 2000.0);

    // ==================== EXPERIMENTAL - SPARSE SPECTRUM ====================
    println!("🎨 EXPERIMENTAL - Random notes with spectral gaps:");
    comp.instrument("sparse", &Instrument::synth_lead())
        .wait(20.0)
        .generator(|g| g
            .scatter(100.0, 1000.0, 32, 0.0625)  // Random notes
        )
        .transform(|t| t
            .sieve_exclusive(200.0, 250.0)   // Remove 200-250 Hz
            .sieve_exclusive(400.0, 450.0)   // Remove 400-450 Hz
            .sieve_exclusive(600.0, 650.0)   // Remove 600-650 Hz
        );

    // ==================== COMBINED WITH OTHER TRANSFORMS ====================
    println!("🔧 COMBINED - Sieve + shift + humanize:");
    comp.instrument("combined", &Instrument::electric_piano())
        .wait(22.0)
        .pattern_start()
        .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25)
        .transform(|t| t
            .sieve_inclusive(150.0, 400.0)  // Keep mid frequencies
            .shift(7)                        // Transpose up a fifth
            .humanize(0.01, 0.05)           // Add variation
        );

    // Play everything
    println!("\n▶️  Playing sieve demo...\n");
    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    println!("✅ Demo complete!\n");
    println!("💡 Sieve use cases:");
    println!("   🎚️  Frequency-based arrangement - separate bass/mid/treble");
    println!("   🔧 Spectral sculpting - remove problematic ranges");
    println!("   🎼 Layer splitting - extract frequency bands to different instruments");
    println!("   🎛️  Notch filtering - create spectral gaps");
    println!("   🎨 Experimental - complex frequency manipulation");
    println!("   🔊 Mix cleanup - remove muddy midrange or harsh highs");
    println!("\n   Compare with:");
    println!("   - .thin() - probabilistic note removal");
    println!("   - .filter() - audio-domain filtering (different from note filtering)");
    println!("   - .compress() - time domain, not frequency domain\n");

    Ok(())
}
