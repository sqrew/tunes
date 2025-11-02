use tunes::prelude::*;

/// Demonstrates the iterative workflow when composing with sections
///
/// This example shows the typical process:
/// 1. Define a section
/// 2. Export it to MIDI to review in your DAW
/// 3. Play it in isolation to hear it
/// 4. Refine and repeat
/// 5. Build up your full composition section by section
fn main() -> anyhow::Result<()> {
    println!("\n🎼 Section-Based Iterative Composition Workflow\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // ===== STEP 1: Compose a verse section =====
    println!("📝 Step 1: Composing 'verse' section...");
    comp.section("verse")
        .instrument("piano", &Instrument::acoustic_piano())
        .notes(&[C4, D4, E4, F4, G4, F4, E4, D4], 0.5)
        .and()
        .track("bass")
        .notes(&[C2, C2, G2, C2], 1.0);
    println!("   ✓ Verse section defined\n");

    // ===== STEP 2: Export to MIDI for review =====
    println!("📤 Step 2: Exporting 'verse' to MIDI for DAW review...");
    comp.export_section_midi("verse", "verse_draft.mid")?;
    println!("   ✓ verse_draft.mid created");
    println!("   → Open in MuseScore/Logic/Ableton to see notation\n");

    // ===== STEP 3: Play in isolation =====
    println!("▶  Step 3: Playing 'verse' in isolation...");
    let verse_mixer = comp.section_to_mixer("verse")?;
    engine.play_mixer(&verse_mixer)?;
    println!("   ✓ Verse playback complete\n");

    // ===== STEP 4: Add a chorus section =====
    println!("📝 Step 4: Composing 'chorus' section...");
    comp.section("chorus")
        .instrument("lead", &Instrument::synth_lead())
        .notes(&[E4, G4, C5, G4, E4, C4], 0.25)
        .and()
        .track("bass")
        .notes(&[C2, E2, G2, C3], 0.5);
    println!("   ✓ Chorus section defined\n");

    // ===== STEP 5: Export chorus to MIDI =====
    println!("📤 Step 5: Exporting 'chorus' to MIDI...");
    comp.export_section_midi("chorus", "chorus_draft.mid")?;
    println!("   ✓ chorus_draft.mid created\n");

    // ===== STEP 6: Play chorus in isolation =====
    println!("▶  Step 6: Playing 'chorus' in isolation...");
    let chorus_mixer = comp.section_to_mixer("chorus")?;
    engine.play_mixer(&chorus_mixer)?;
    println!("   ✓ Chorus playback complete\n");

    // ===== STEP 7: Arrange the full piece =====
    println!("🎹 Step 7: Arranging full composition...");
    comp.arrange(&["verse", "chorus", "verse", "chorus"]);
    println!("   ✓ Arrangement: V-C-V-C\n");

    // ===== STEP 8: Export full composition =====
    println!("📤 Step 8: Exporting complete arrangement to MIDI...");
    let full_mixer = comp.into_mixer();
    full_mixer.export_midi("full_composition.mid")?;
    println!("   ✓ full_composition.mid created\n");

    // ===== STEP 9: Play full composition =====
    println!("▶  Step 9: Playing full composition...");
    engine.play_mixer(&full_mixer)?;
    println!("   ✓ Full composition playback complete\n");

    println!("✅ Iterative Workflow Complete!\n");
    println!("📁 Files created:");
    println!("   • verse_draft.mid    - Review verse in your DAW");
    println!("   • chorus_draft.mid   - Review chorus in your DAW");
    println!("   • full_composition.mid - Complete arranged piece\n");

    println!("💡 Key Workflow Benefits:");
    println!("   • Work on one section at a time (reduces complexity)");
    println!("   • Test sections in isolation before arranging");
    println!("   • Export to DAW for detailed review/notation");
    println!("   • Iterate quickly without compiling full composition");
    println!("   • Build confidence in each section before moving on\n");

    println!("🎯 Next Steps:");
    println!("   • Open verse_draft.mid in your DAW to see the notation");
    println!("   • Refine individual sections based on what you hear");
    println!(
        "   • Export sections as WAV: comp.export_section_wav(\"verse\", \"verse.wav\", 44100)?"
    );
    println!("   • Loop a section: Play same mixer multiple times\n");

    Ok(())
}
