use tunes::prelude::*;

/// Demonstrate key signatures: major, minor, and all 7 Greek modes
///
/// This example showcases:
/// - Major and minor key signatures
/// - All 7 Greek modes (Ionian, Dorian, Phrygian, Lydian, Mixolydian, Aeolian, Locrian)
/// - How key signatures affect MIDI export
/// - Modal music theory in practice
fn main() -> anyhow::Result<()> {
    println!("\n🎼 Key Signatures & Modal Music Demonstration\n");
    println!("Showcasing major, minor, and modal key signatures.");
    println!("Each section demonstrates a different mode or key.\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // ===== MAJOR KEY SIGNATURES =====
    println!("1. Major Key Signatures");
    println!("   Playing C Major scale (no sharps/flats)");

    comp.section("c_major")
        .instrument("piano", &Instrument::acoustic_piano())
        .key_signature(KeySignature::C_MAJOR)
        .notes(&[C4, D4, E4, F4, G4, A4, B4, C5], 0.25);

    println!("   Playing G Major scale (1 sharp: F#)");

    comp.section("g_major")
        .instrument("piano", &Instrument::acoustic_piano())
        .key_signature(KeySignature::G_MAJOR)
        .notes(&[G4, A4, B4, C5, D5, E5, FS5, G5], 0.25);

    println!("   Playing F Major scale (1 flat: Bb)");

    comp.section("f_major")
        .instrument("piano", &Instrument::acoustic_piano())
        .key_signature(KeySignature::F_MAJOR)
        .notes(&[F4, G4, A4, AS4, C5, D5, E5, F5], 0.25);

    // ===== MINOR KEY SIGNATURES =====
    println!("\n2. Minor Key Signatures (Natural Minor)");
    println!("   Playing A minor scale (no sharps/flats, relative to C Major)");

    comp.section("a_minor")
        .instrument("piano", &Instrument::acoustic_piano())
        .key_signature(KeySignature::A_MINOR)
        .notes(&[A4, B4, C5, D5, E5, F5, G5, A5], 0.25);

    println!("   Playing E minor scale (1 sharp: F#)");

    comp.section("e_minor")
        .instrument("piano", &Instrument::acoustic_piano())
        .key_signature(KeySignature::E_MINOR)
        .notes(&[E4, FS4, G4, A4, B4, C5, D5, E5], 0.25);

    // ===== MODAL KEY SIGNATURES =====
    println!("\n3. Greek Modes - The Seven Modes of the Major Scale");
    println!("   All modes below share the same key signature (C Major / A minor)");
    println!("   but start on different degrees, creating unique flavors.\n");

    // Mode 1: Ionian (Major)
    println!("   • Ionian (Major) - Bright, happy: C-D-E-F-G-A-B");
    comp.section("ionian")
        .instrument("strings", &Instrument::warm_pad())
        .key_signature(KeySignature::C_MAJOR)  // C Ionian = C Major
        .notes(&[C4, D4, E4, F4, G4, A4, B4, C5], 0.25);

    // Mode 2: Dorian
    println!("   • Dorian - Minor with raised 6th, jazzy: D-E-F-G-A-B-C");
    comp.section("dorian")
        .instrument("strings", &Instrument::warm_pad())
        .key_signature(KeySignature::D_DORIAN)
        .notes(&[D4, E4, F4, G4, A4, B4, C5, D5], 0.25);

    // Mode 3: Phrygian
    println!("   • Phrygian - Dark, Spanish: E-F-G-A-B-C-D");
    comp.section("phrygian")
        .instrument("strings", &Instrument::warm_pad())
        .key_signature(KeySignature::E_PHRYGIAN)
        .notes(&[E4, F4, G4, A4, B4, C5, D5, E5], 0.25);

    // Mode 4: Lydian
    println!("   • Lydian - Bright, dreamy: F-G-A-B-C-D-E");
    comp.section("lydian")
        .instrument("strings", &Instrument::warm_pad())
        .key_signature(KeySignature::F_LYDIAN)
        .notes(&[F4, G4, A4, B4, C5, D5, E5, F5], 0.25);

    // Mode 5: Mixolydian
    println!("   • Mixolydian - Bluesy, rock: G-A-B-C-D-E-F");
    comp.section("mixolydian")
        .instrument("strings", &Instrument::warm_pad())
        .key_signature(KeySignature::G_MIXOLYDIAN)
        .notes(&[G4, A4, B4, C5, D5, E5, F5, G5], 0.25);

    // Mode 6: Aeolian (Natural Minor)
    println!("   • Aeolian (Natural Minor) - Melancholic: A-B-C-D-E-F-G");
    comp.section("aeolian")
        .instrument("strings", &Instrument::warm_pad())
        .key_signature(KeySignature::A_AEOLIAN)  // A Aeolian = A Natural Minor
        .notes(&[A4, B4, C5, D5, E5, F5, G5, A5], 0.25);

    // Mode 7: Locrian
    println!("   • Locrian - Unstable, diminished: B-C-D-E-F-G-A");
    comp.section("locrian")
        .instrument("strings", &Instrument::warm_pad())
        .key_signature(KeySignature::B_LOCRIAN)
        .notes(&[B4, C5, D5, E5, F5, G5, A5, B5], 0.25);

    // ===== MODAL MELODY EXAMPLE =====
    println!("\n4. Modal Melody - Same Melody in Different Modes");
    println!("   Playing the same melodic pattern in Dorian, Phrygian, and Lydian");
    println!("   to demonstrate how mode choice affects emotional color.\n");

    // Simple melodic pattern transposed to different modes
    // In D Dorian - jazzy, minor but bright
    let pattern_d = [D4, E4, F4, D4, F4, E4, D4];
    comp.section("melody_dorian")
        .instrument("lead", &Instrument::synth_lead())
        .key_signature(KeySignature::D_DORIAN)
        .at(0.0)
        .notes(&pattern_d, 0.3);

    // In E Phrygian - dark, exotic
    let pattern_e = [E4, F4, G4, E4, G4, F4, E4];
    comp.section("melody_phrygian")
        .instrument("lead", &Instrument::synth_lead())
        .key_signature(KeySignature::E_PHRYGIAN)
        .at(0.0)
        .notes(&pattern_e, 0.3);

    // In F Lydian - bright, ethereal
    let pattern_f = [F4, G4, A4, F4, A4, G4, F4];
    comp.section("melody_lydian")
        .instrument("lead", &Instrument::synth_lead())
        .key_signature(KeySignature::F_LYDIAN)
        .at(0.0)
        .notes(&pattern_f, 0.3);

    // ===== ARRANGEMENT =====
    println!("5. Arranging Sections");
    println!("   Creating a journey through different keys and modes\n");

    comp.arrange(&[
        "c_major",      // Start bright
        "a_minor",      // Move to relative minor
        "dorian",       // Jazzy mode
        "phrygian",     // Dark mode
        "lydian",       // Dreamy mode
        "mixolydian",   // Bluesy mode
        "aeolian",      // Natural minor
        "c_major",      // Return home
    ]);

    // ===== PLAYBACK =====
    println!("\nPlaying modal demonstration...");
    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    // ===== MIDI EXPORT =====
    println!("\nExporting to MIDI with key signature metadata...");
    mixer.export_midi("key_signatures_demo.mid")?;
    println!("✅ Exported to key_signatures_demo.mid");
    println!("   Open in your DAW to see key signatures for each section!");

    println!("\n✨ Demonstration complete!");
    println!("\n📚 Modal Music Theory Recap:");
    println!("   • Modes are scales derived from a parent major scale");
    println!("   • Each mode starts on a different degree of the parent scale");
    println!("   • All modes of C major have 0 sharps/flats but sound different");
    println!("   • Modes create different emotional colors:");
    println!("     - Ionian (Major): Happy, bright");
    println!("     - Dorian: Jazzy, minor with edge");
    println!("     - Phrygian: Dark, Spanish/Middle Eastern");
    println!("     - Lydian: Ethereal, dreamy");
    println!("     - Mixolydian: Bluesy, rock");
    println!("     - Aeolian (Minor): Sad, introspective");
    println!("     - Locrian: Unstable, rarely used");

    Ok(())
}
