use tunes::prelude::*;

/// Demonstrate time markers for easy track synchronization
fn main() -> anyhow::Result<()> {
    println!("\n⏱️  Time Markers Demo\n");
    println!("Markers make it easy to synchronize timing across multiple tracks");
    println!("without manual time calculations.\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // ===== BASIC MARKER USAGE =====
    println!("Part 1: Basic Markers");
    println!("  Creating markers at key song positions...\n");

    // Define the song structure with markers
    comp.track("structure")
        .mark("intro") // 0.0
        .wait(4.0)
        .mark("verse1") // 4.0
        .wait(8.0)
        .mark("chorus") // 12.0
        .wait(8.0)
        .mark("verse2") // 20.0
        .wait(8.0)
        .mark("outro") // 28.0
        .wait(4.0); // End at 32.0

    // Drums enter at verse 1
    comp.track("drums")
        .at_mark("verse1")
        .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, &[0, 4, 8, 12])
        .sound(DrumType::Snare, &[4, 12])
        .sound(DrumType::HiHatClosed, &[0, 2, 4, 6, 8, 10, 12, 14]))
        .repeat(3); // Plays through verse1, chorus, and part of verse2

    // Bass starts at chorus
    comp.instrument("bass", &Instrument::sub_bass())
        .at_mark("chorus")
        .pattern_start()
        .notes(&[C2, C2, G2, C3], 0.5)
        .repeat(7); // 8 bars total

    // Lead melody comes in at verse 2
    comp.instrument("lead", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at_mark("verse2")
        .notes(&[C4, E4, G4, C5, G4, E4, C4, G3], 0.25);

    // Pad fills out chorus and outro
    comp.instrument("pad", &Instrument::warm_pad())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at_mark("chorus")
        .note(&[C3, E3, G3], 8.0)
        .at_mark("outro")
        .note(&[F3, A3, C4], 4.0);

    println!("✓ Markers created:");
    println!("  • intro   @ 0.0s");
    println!("  • verse1  @ 4.0s");
    println!("  • chorus  @ 12.0s");
    println!("  • verse2  @ 20.0s");
    println!("  • outro   @ 28.0s");

    println!("\n✓ Tracks synchronized using markers:");
    println!("  • drums  → start at verse1");
    println!("  • bass   → start at chorus");
    println!("  • lead   → start at verse2");
    println!("  • pad    → play at chorus and outro");

    // ===== ADVANCED: USING PEEK_CURSOR =====
    println!("\n\nPart 2: Using .peek_cursor() for debugging");

    let mut comp2 = Composition::new(Tempo::new(140.0));

    let builder = comp2.track("debug_example").notes(&[C4, E4], 0.5);

    let pos1 = builder.peek_cursor();
    println!("  After 2 notes: cursor at {:.1}s", pos1);

    let builder = builder.wait(2.0);
    let pos2 = builder.peek_cursor();
    println!("  After wait(2.0): cursor at {:.1}s", pos2);

    let builder = builder.note(&[G4], 0.5).mark("checkpoint");

    let pos3 = builder.peek_cursor();
    println!("  After 1 more note: cursor at {:.1}s", pos3);
    println!("  Marker 'checkpoint' saved at {:.1}s\n", pos3);

    // ===== ADVANCED: MARKER REUSE =====
    println!("Part 3: Reusing markers across multiple tracks\n");

    let mut comp3 = Composition::new(Tempo::new(128.0));

    // Setup markers
    comp3
        .track("timeline")
        .wait(4.0)
        .mark("drop")
        .wait(8.0)
        .mark("breakdown")
        .wait(4.0)
        .mark("buildup");

    // Multiple tracks can all reference the same marker
    comp3
        .track("kick")
        .at_mark("drop")
        .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, &[0, 4, 8, 12]))
        .repeat(7);

    comp3
        .track("snare")
        .at_mark("drop")
        .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Snare, &[4, 12]))
        .repeat(7);

    comp3
        .instrument("wobble", &Instrument::wobble_bass())
        .at_mark("drop")
        .notes(&[C2, C2, C2, C2], 0.5);

    println!("✓ Three tracks all start at the 'drop' marker (4.0s)");
    println!("  No manual .at(4.0) needed - just .at_mark(\"drop\")!\n");

    println!("\n▶️  Playing demo...\n");
    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Markers Demo Complete!\n");
    println!("💡 Key Benefits:");
    println!("   • No manual time math - markers track positions for you");
    println!("   • Easy refactoring - change one marker, all tracks update");
    println!("   • Self-documenting - marker names describe song structure");
    println!("   • Cross-track sync - multiple tracks can use same marker");
    println!("\n💡 New Methods:");
    println!("   • .mark(\"name\")      - Save current cursor position");
    println!("   • .at_mark(\"name\")   - Jump to a saved marker");
    println!("   • .peek_cursor()     - Get current position (for debugging)\n");

    Ok(())
}
