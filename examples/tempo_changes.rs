use tunes::prelude::*;

/// Demonstrate tempo changes: ritardando, accelerando, and multi-section tempos
fn main() -> anyhow::Result<()> {
    println!("\n🎵 Tempo Changes Demonstration\n");
    println!("Showcasing how to change tempo during a composition.");
    println!("Tempo changes affect MIDI export timing.\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // ===== 1. SUDDEN TEMPO CHANGE =====
    println!("1. Sudden Tempo Change - Fast to Slow");

    comp.instrument("melody", &Instrument::acoustic_piano())
        .at(0.0)
        // Fast section (120 BPM - default)
        .notes(&[C4, D4, E4, F4], 0.25)
        .notes(&[G4, A4, B4, C5], 0.25)
        .wait(0.5)
        // Sudden change to slow (80 BPM)
        .tempo(80.0)
        .notes(&[C5, B4, A4, G4], 0.5)
        .notes(&[F4, E4, D4, C4], 0.5);

    // ===== 2. RITARDANDO (GRADUAL SLOWDOWN) =====
    println!("2. Ritardando - Gradual Slowdown");

    comp.instrument("ritardando", &Instrument::warm_pad())
        .at(6.0)
        .note(&[C3, E3, G3], 1.0) // 80 BPM (from previous section)
        .tempo(75.0)
        .note(&[D3, F3, A3], 1.0)
        .tempo(70.0)
        .note(&[E3, G3, B3], 1.0)
        .tempo(60.0)
        .note(&[F3, A3, C4], 2.0); // Final chord held longer

    // ===== 3. ACCELERANDO (GRADUAL SPEEDUP) =====
    println!("3. Accelerando - Gradual Speedup");

    comp.instrument("accelerando", &Instrument::synth_lead())
        .at(12.0)
        .tempo(60.0) // Start slow
        .notes(&[G4, G4], 0.5)
        .tempo(80.0)
        .notes(&[A4, A4], 0.4)
        .tempo(100.0)
        .notes(&[B4, B4], 0.3)
        .tempo(120.0)
        .notes(&[C5, C5], 0.25)
        .tempo(140.0)
        .notes(&[D5, D5, E5, F5], 0.2);

    // ===== 4. MULTI-SECTION WITH DIFFERENT TEMPOS =====
    println!("4. Multi-Section Piece with Different Tempos");

    comp.instrument("bass", &Instrument::pluck())
        .at(16.0)
        // Introduction - Moderate
        .tempo(90.0)
        .notes(&[C2, C2, G2, G2], 0.5)
        .wait(0.5)
        // Verse - Upbeat
        .tempo(120.0)
        .notes(&[C2, G2, A2, F2], 0.25)
        .notes(&[C2, G2, A2, F2], 0.25)
        .wait(0.5)
        // Chorus - Driving
        .tempo(140.0)
        .notes(&[C2, C2, E2, E2], 0.2)
        .notes(&[G2, G2, C3, C3], 0.2)
        .wait(0.5)
        // Outro - Slow down
        .tempo(100.0)
        .notes(&[A1, F2, C2], 0.75)
        .tempo(80.0)
        .note(&[C2], 2.0);

    // ===== 5. TEMPO VARIATIONS IN PARALLEL TRACKS =====
    println!("5. Tempo Variations Across Parallel Tracks");

    // All tracks will follow the same tempo changes
    comp.instrument("harmony", &Instrument::warm_pad())
        .at(26.0)
        .tempo(100.0)
        .note(&[E3, G3, C4], 2.0)
        .tempo(120.0)
        .note(&[F3, A3, D4], 1.0)
        .tempo(90.0)
        .note(&[G3, B3, E4], 2.0);

    let mixer = comp.into_mixer();
    let duration = mixer.total_duration();

    println!("\n=== Summary ===");
    println!("✅ Sudden tempo changes: 120 → 80 BPM");
    println!("✅ Ritardando: Gradually slowing from 80 → 60 BPM");
    println!("✅ Accelerando: Gradually speeding from 60 → 140 BPM");
    println!("✅ Multi-section: 90 → 120 → 140 → 100 → 80 BPM");
    println!("✅ Parallel tracks: All follow same tempo changes\n");

    println!("💡 Tip: Tempo changes affect MIDI export but not audio rendering.");
    println!("   Use them to create more expressive MIDI files!\n");

    println!("Playing {:.1} seconds...", duration);
    engine.play_mixer(&mixer)?;

    println!("\n✅ Done! Export to MIDI with:");
    println!("   mixer.export_midi(\"tempo_demo.mid\", Tempo::new(120.0))?;\n");

    Ok(())
}
