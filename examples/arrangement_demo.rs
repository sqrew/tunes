use tunes::prelude::*;


fn main() -> anyhow::Result<()> {
    println!("\n🎼  Arrangement System Demo\n");
    println!("Demonstrating section-based composition and arrangement\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(128.0));

    // === INTRO SECTION ===
    println!("📝 Defining sections...\n");
    println!("  • Intro - Atmospheric pad and simple drums");

    comp.section("intro")
        .instrument("pad", &Instrument::warm_pad())
        .notes(&[C3, E3, G3], 2.0)
        .and()
        .track("drums")
        .drum(DrumType::Kick)
        .wait(0.5)
        .drum(DrumType::Snare)
        .wait(0.5);

    // === VERSE SECTION ===
    println!("  • Verse - Bass line + drums pattern");

    comp.section("verse")
        .instrument("bass", &Instrument::pluck())
        .envelope(Envelope::new(0.01, 0.1, 0.8, 0.2))
        .pattern_start()
        .notes(&[C2, C2, G2, C2], 0.5)
        .repeat(1) // Play twice
        .and()
        .track("drums")
        .pattern_start()
        .drum(DrumType::Kick)
        .wait(0.25)
        .drum(DrumType::HiHatClosed)
        .wait(0.25)
        .drum(DrumType::Snare)
        .wait(0.25)
        .drum(DrumType::HiHatClosed)
        .wait(0.25)
        .repeat(3); // 4 bars total

    // === CHORUS SECTION ===
    println!("  • Chorus - Uplifting melody + full instrumentation");

    comp.section("chorus")
        .instrument("lead", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5, G4, E4, C4, G3], 0.25)
        .and()
        .instrument("bass", &Instrument::pluck())
        .envelope(Envelope::new(0.01, 0.1, 0.8, 0.2))
        .notes(&[C2, E2, G2, C3], 0.5)
        .and()
        .track("drums")
        .pattern_start()
        .drum(DrumType::Kick)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::Snare)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .repeat(0); // 1 bar

    // === BRIDGE SECTION ===
    println!("  • Bridge - Breakdown with sparse drums");

    comp.section("bridge")
        .instrument("synth", &Instrument::synth_lead())
        .notes(&[A3, C4, E4, A4, E4, C4], 0.5)
        .and()
        .track("drums")
        .drum(DrumType::Snare)
        .wait(1.5);

    // === OUTRO SECTION ===
    println!("  • Outro - Fade out with pad");

    comp.section("outro")
        .instrument("pad", &Instrument::warm_pad())
        .notes(&[C3, E3, G3], 2.0);

    // === ARRANGEMENT ===
    println!("\n🎹 Arranging composition:\n");
    println!("  Structure: Intro → Verse → Chorus → Verse → Chorus → Bridge → Chorus → Outro");
    println!("");

    comp.arrange(&[
        "intro",  // 2s
        "verse",  // 4s
        "chorus", // 2s
        "verse",  // 4s
        "chorus", // 2s
        "bridge", // 3s
        "chorus", // 2s
        "outro",  // 2s
    ]);

    println!("✓ Total duration: ~21 seconds");
    println!("✓ Sections can be reused (verse and chorus each appear twice)");
    println!("✓ Each section maintains its own timing and instrumentation\n");

    // === SECTION ISOLATION DEMO ===
    println!("🔍 Section Isolation Feature:\n");
    println!("  During composition, you can work on individual sections!");
    println!("  Let's export and play just the 'chorus' section:\n");

    // Export individual sections for DAW review
    println!("  📤 Exporting sections to MIDI files...");
    comp.export_section_midi("verse", "verse.mid")?;
    comp.export_section_midi("chorus", "chorus.mid")?;
    comp.export_section_midi("bridge", "bridge.mid")?;
    println!("     ✓ verse.mid");
    println!("     ✓ chorus.mid");
    println!("     ✓ bridge.mid");
    println!("     → Open these in your DAW to review individual sections!\n");

    // Play just one section for testing
    println!("  ▶ Playing ONLY the chorus section (for iteration)...");
    let chorus_mixer = comp.section_to_mixer("chorus")?;
    engine.play_mixer(&chorus_mixer)?;
    println!("     ✓ Chorus section played in isolation\n");

    println!("  💡 Iterative workflow:");
    println!("     1. Define a section");
    println!("     2. Play it in isolation → comp.section_to_mixer(\"name\")");
    println!("     3. Export to MIDI for review → comp.export_section_midi()");
    println!("     4. Refine and repeat");
    println!("     5. Arrange all sections when ready!\n");

    println!("▶ Now playing full arranged composition...\n");
    engine.play_mixer(&comp.into_mixer())?;

    println!("✅ Arrangement demo complete!\n");
    println!("💡 Key features:");
    println!("   • Define sections once, use them multiple times");
    println!("   • .and() chains multiple tracks within a section");
    println!("   • .arrange() sequences sections in any order");
    println!("   • .section_to_mixer() - Test individual sections");
    println!("   • .export_section_midi() - Export sections to DAW");
    println!("   • .export_section_wav() - Export sections as audio");
    println!("   • Perfect for song structures (verse/chorus/bridge)");
    println!("   • Sections maintain timing consistency\n");

    Ok(())
}
