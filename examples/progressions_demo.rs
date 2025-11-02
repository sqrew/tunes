use tunes::prelude::*;

/// Demonstrate the new progression builder methods
fn main() -> anyhow::Result<()> {
    println!("\n🎼 Chord Progressions Demo\n");
    println!("Showcasing the convenient .progression() and .progression_7th() methods\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(100.0));

    // ===== PART 1: OLD WAY VS NEW WAY =====
    println!("Part 1: Comparing old vs new API\n");

    println!("OLD WAY (still works):");
    println!("  let pop_prog = progression(C4, &ScalePattern::MAJOR, &[1, 5, 6, 4], ProgressionType::Triads);");
    println!("  comp.track(\"chords\").chords_from(&pop_prog, 1.5);\n");

    println!("NEW WAY (much cleaner):");
    println!("  comp.track(\"chords\")");
    println!("      .progression(C4, &ScalePattern::MAJOR, &[1, 5, 6, 4], 1.5);\n");

    // ===== PART 2: CLASSIC POP PROGRESSIONS =====
    println!("Part 2: Classic Pop Progressions\n");

    // I-V-vi-IV (most famous pop progression)
    println!("  ♪ I-V-vi-IV (\"Don't Stop Believin'\", \"Let It Be\", etc.)");
    comp.instrument("pop1", &Instrument::warm_pad())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(0.0)
        .progression(C4, &ScalePattern::MAJOR, &[1, 5, 6, 4], 1.5);

    // I-IV-V-IV (classic rock)
    println!("  ♪ I-IV-V-IV (Classic rock cadence)");
    comp.instrument("rock", &Instrument::warm_pad())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(6.5)
        .progression(C4, &ScalePattern::MAJOR, &[1, 4, 5, 4], 1.5);

    // vi-IV-I-V ("Axis of Awesome" 4-chord progression)
    println!("  ♪ vi-IV-I-V (\"Axis of Awesome\" progression)");
    comp.instrument("axis", &Instrument::warm_pad())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(13.0)
        .progression(C4, &ScalePattern::MAJOR, &[6, 4, 1, 5], 1.5);

    // ===== PART 3: JAZZ PROGRESSIONS WITH 7THS =====
    println!("\nPart 3: Jazz Progressions with 7th Chords\n");

    // ii-V-I (the most important jazz progression)
    println!("  ♪ ii7-V7-Imaj7 (The cornerstone of jazz)");
    comp.instrument("jazz1", &Instrument::electric_piano())
        .chorus(Chorus::new(0.3, 0.003, 0.3))
        .at(19.5)
        .progression_7th(C4, &ScalePattern::MAJOR, &[2, 5, 1], 2.0);

    // I-vi-ii-V (jazz turnaround)
    println!("  ♪ Imaj7-vi7-ii7-V7 (Jazz turnaround)");
    comp.instrument("turnaround", &Instrument::electric_piano())
        .chorus(Chorus::new(0.3, 0.003, 0.3))
        .at(25.5)
        .progression_7th(C4, &ScalePattern::MAJOR, &[1, 6, 2, 5], 1.5);

    // ===== PART 4: MINOR KEY PROGRESSIONS =====
    println!("\nPart 4: Minor Key Progressions\n");

    // i-VI-III-VII (minor key "Andalusian cadence")
    println!("  ♪ i-VI-III-VII in A minor (Andalusian cadence)");
    comp.instrument("andalusian", &Instrument::warm_pad())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(31.5)
        .progression(A3, &ScalePattern::MINOR, &[1, 6, 3, 7], 1.5);

    // i-iv-v (minor blues)
    println!("  ♪ i-iv-v in A minor (Minor blues)");
    comp.instrument("minor_blues", &Instrument::warm_pad())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(37.5)
        .progression(A3, &ScalePattern::MINOR, &[1, 4, 5], 2.0);

    // ===== PART 5: MODAL PROGRESSIONS =====
    println!("\nPart 5: Modal Progressions\n");

    // Dorian ii-I (jazzy modal sound)
    println!("  ♪ D Dorian: ii-I (Modal jazz vamp)");
    comp.instrument("dorian", &Instrument::warm_pad())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(43.5)
        .progression(D4, &ScalePattern::DORIAN, &[2, 1, 2, 1], 1.5);

    // Mixolydian I-bVII (rock modal)
    println!("  ♪ G Mixolydian: I-bVII (Classic rock modal)");
    comp.instrument("mixolydian", &Instrument::warm_pad())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(49.5)
        .progression(G3, &ScalePattern::MIXOLYDIAN, &[1, 7, 1, 7], 1.5);

    // ===== PART 6: COMBINING WITH OTHER METHODS =====
    println!("\nPart 6: Combining Progressions with Melodies\n");
    println!("  ♪ Chord progression with melody on top");

    // Chords
    comp.instrument("backing", &Instrument::warm_pad())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .volume(0.5)
        .at(55.5)
        .progression(C4, &ScalePattern::MAJOR, &[1, 5, 6, 4], 2.0);

    // Melody over the top
    comp.instrument("melody", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(55.5)
        .notes(&[E4, G4, C5, B4, A4, G4, F4, E4], 0.5)
        .wait(0.5)
        .notes(&[E4, G4, C5, B4, A4, G4, A4, B4], 0.5);

    // ===== PART 7: PROGRESSION VARIATIONS =====
    println!("\nPart 7: Same Progression, Different Rhythms\n");

    // Fast (quarter notes)
    println!("  ♪ Fast rhythm (0.5s per chord)");
    comp.instrument("fast", &Instrument::electric_piano())
        .at(63.5)
        .progression(G3, &ScalePattern::MAJOR, &[1, 5, 6, 4, 1, 5, 6, 4], 0.5);

    // Medium (half notes)
    println!("  ♪ Medium rhythm (1.0s per chord)");
    comp.instrument("medium", &Instrument::warm_pad())
        .volume(0.6)
        .at(67.5)
        .progression(G3, &ScalePattern::MAJOR, &[1, 5, 6, 4], 1.0);

    // Slow (whole notes)
    println!("  ♪ Slow rhythm (2.0s per chord)");
    comp.instrument("slow", &Instrument::warm_pad())
        .volume(0.7)
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(71.5)
        .progression(G3, &ScalePattern::MAJOR, &[1, 5, 6, 4], 2.0);

    // ===== PART 8: USING MARKERS WITH PROGRESSIONS =====
    println!("\nPart 8: Using Markers with Progressions\n");
    println!("  ♪ Combining markers with progression methods");

    // Setup song structure with markers
    comp.track("structure")
        .mark("intro")
        .wait(4.0)
        .mark("verse")
        .wait(8.0)
        .mark("chorus");

    // Different progressions at different sections
    comp.instrument("intro_chords", &Instrument::warm_pad())
        .at_mark("intro")
        .progression(F3, &ScalePattern::MAJOR, &[1, 5], 2.0);

    comp.instrument("verse_chords", &Instrument::electric_piano())
        .at_mark("verse")
        .progression(F3, &ScalePattern::MAJOR, &[1, 5, 6, 4], 2.0);

    comp.instrument("chorus_chords", &Instrument::warm_pad())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at_mark("chorus")
        .progression_7th(F3, &ScalePattern::MAJOR, &[1, 6, 4, 5], 2.0);

    println!("\n▶️  Playing demonstration...\n");
    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Progressions Demo Complete!\n");
    println!("💡 New Builder Methods:");
    println!("   • .progression(root, scale, degrees, duration)");
    println!("     → Plays triads (3-note chords)");
    println!("   • .progression_7th(root, scale, degrees, duration)");
    println!("     → Plays 7th chords (4-note chords)");
    println!("\n💡 Benefits:");
    println!("   ✓ No need to call progression() separately");
    println!("   ✓ One-liner for common chord progressions");
    println!("   ✓ Chains naturally with other builder methods");
    println!("   ✓ Works with markers and timing methods");
    println!("\n💡 Famous Progressions Shown:");
    println!("   • I-V-vi-IV    - Pop perfection");
    println!("   • ii7-V7-Imaj7 - Jazz standard");
    println!("   • i-VI-III-VII - Andalusian cadence");
    println!("   • I-bVII       - Rock modal sound\n");

    Ok(())
}
