use tunes::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
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

    println!("▶ Playing arranged composition...\n");
    engine.play_mixer(&comp.into_mixer())?;

    println!("✅ Arrangement demo complete!\n");
    println!("💡 Key features:");
    println!("   • Define sections once, use them multiple times");
    println!("   • .and() chains multiple tracks within a section");
    println!("   • .arrange() sequences sections in any order");
    println!("   • Perfect for song structures (verse/chorus/bridge)");
    println!("   • Sections maintain timing consistency\n");

    Ok(())
}
