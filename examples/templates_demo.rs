use tunes::prelude::*;

/// Comprehensive demonstration of track templates for reusing settings
fn main() -> anyhow::Result<()> {
    println!("\n🎨 Track Templates Demo\n");
    println!("Showcasing the .save_template() and .from_template() methods\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(110.0));

    // ===== PART 1: BEFORE TEMPLATES - REPETITIVE CODE =====
    println!("Part 1: The Problem - Repetitive Configuration\n");
    println!("Without templates, configuring multiple similar tracks is tedious:");
    println!("  comp.instrument(\"lead1\", &Instrument::synth_lead())");
    println!("      .reverb(Reverb::new(0.5, 0.5, 0.3))");
    println!("      .delay(Delay::new(0.375, 0.3, 0.5))");
    println!("      .volume(0.7)");
    println!("      .notes(&[C4, E4, G4], 0.25);");
    println!();
    println!("  // Copy-paste and modify for each similar track!");
    println!("  comp.instrument(\"lead2\", &Instrument::synth_lead())");
    println!("      .reverb(Reverb::new(0.5, 0.5, 0.3))  // Same settings!");
    println!("      .delay(Delay::new(0.375, 0.3, 0.5))  // Same settings!");
    println!("      .volume(0.7)                          // Same settings!");
    println!("      .notes(&[G4, B4, D5], 0.25);\n");

    // ===== PART 2: BASIC TEMPLATE USAGE =====
    println!("Part 2: The Solution - Templates\n");
    println!("  ♪ Create a template once:");
    println!("    comp.instrument(\"lead1\", &Instrument::synth_lead())");
    println!("        .reverb(Reverb::new(0.5, 0.5, 0.3))");
    println!("        .delay(Delay::new(0.375, 0.3, 0.5))");
    println!("        .volume(0.7)");
    println!("        .save_template(\"lead_sound\")  // Save the template!");
    println!("        .notes(&[C4, E4, G4], 0.25);");
    println!();
    println!("  ♪ Reuse it instantly:");
    println!("    comp.from_template(\"lead_sound\", \"lead2\")");
    println!("        .notes(&[G4, B4, D5], 0.25);\n");

    comp.instrument("lead1", &Instrument::synth_lead())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .delay(Delay::new(0.375, 0.3, 0.5))
        .volume(0.7)
        .save_template("lead_sound")
        .notes(&[C4, E4, G4], 0.25);

    comp.from_template("lead_sound", "lead2")
        .at(1.0)
        .notes(&[G4, B4, D5], 0.25);

    comp.from_template("lead_sound", "lead3")
        .at(2.0)
        .notes(&[E4, G4, B4], 0.25);

    // ===== PART 3: TEMPLATE WITH MULTIPLE EFFECTS =====
    println!("Part 3: Templates Capture Everything\n");
    println!("  ♪ Templates save ALL settings:");
    println!("    • Effects (reverb, delay, chorus, distortion, etc.)");
    println!("    • Mix settings (volume, pan)");
    println!("    • Synthesis parameters (waveform, envelope, FM)");
    println!("    • Filter settings");
    println!("    • Everything!\n");

    comp.instrument("pad1", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .chorus(Chorus::new(0.3, 2.5, 0.3))
        .volume(0.5)
        .pan(-0.5)
        .save_template("pad_sound")
        .at(3.0)
        .note(&chord(C3, &ChordPattern::MAJOR), 2.0);

    comp.from_template("pad_sound", "pad2")
        .at(3.0)
        .pan(0.5) // Override pan for stereo width
        .note(&chord(F3, &ChordPattern::MAJOR), 2.0);

    comp.from_template("pad_sound", "pad3")
        .at(5.0)
        .note(&chord(G3, &ChordPattern::MAJOR), 2.0);

    // ===== PART 4: TEMPLATE VARIATIONS =====
    println!("Part 4: Template Variations\n");
    println!("  ♪ Start with a template, then customize:");

    comp.instrument("bass1", &Instrument::sub_bass())
        .distortion(Distortion::new(0.3, 0.5))
        .compressor(Compressor::new(-20.0, 4.0, 5.0, 50.0, 2.0))
        .volume(0.8)
        .save_template("bass_sound")
        .at(7.0)
        .notes(&[C2, C2, E2, G2], 0.5);

    // Use template as base, then add extra effects
    println!("    comp.from_template(\"bass_sound\", \"bass_filtered\")");
    println!("        .filter(Filter::low_pass(800.0, 0.7))  // Add filter!");
    println!("        .notes(&[A2, A2, C3, E3], 0.5);\n");

    comp.from_template("bass_sound", "bass_filtered")
        .at(9.0)
        .filter(Filter::low_pass(800.0, 0.7)) // Add filter on top of template
        .notes(&[A2, A2, C3, E3], 0.5);

    // Use template but override volume
    comp.from_template("bass_sound", "bass_quiet")
        .at(11.0)
        .volume(0.3) // Override template volume
        .notes(&[F2, F2, A2, C3], 0.5);

    // ===== PART 5: TEMPLATES WITH CUSTOM SYNTHESIS =====
    println!("Part 5: Templates with Custom Synthesis\n");
    println!("  ♪ Templates remember synthesis parameters:\n");

    comp.track("pluck1")
        .waveform(Waveform::Square)
        .envelope(Envelope::new(0.001, 0.05, 0.0, 0.3))
        .fm_custom(2.0, 1.2)
        .delay(Delay::new(0.375, 0.3, 0.6))
        .save_template("pluck_sound")
        .at(13.0)
        .notes(&scale(E4, &ScalePattern::MINOR_PENTATONIC), 0.15);

    comp.from_template("pluck_sound", "pluck2")
        .at(14.5)
        .notes(&scale(E5, &ScalePattern::MINOR_PENTATONIC), 0.15);

    // ===== PART 6: TEMPLATES IN SONG STRUCTURE =====
    println!("Part 6: Templates with Markers\n");
    println!("  ♪ Combine templates with markers for song structure:\n");

    // Setup song structure
    comp.track("structure")
        .mark("intro")
        .wait(4.0)
        .mark("verse")
        .wait(8.0)
        .mark("chorus");

    // Create different templates for different sections
    comp.instrument("intro_lead", &Instrument::synth_lead())
        .reverb(Reverb::new(0.8, 1.0, 0.6))
        .volume(0.4)
        .save_template("intro_sound")
        .at_mark("intro")
        .notes(&[C4, D4, E4, G4], 0.5);

    comp.instrument("verse_melody", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .save_template("verse_sound")
        .at_mark("verse")
        .notes(&[C4, E4, G4, B4, C5, B4, G4, E4], 0.5);

    comp.instrument("chorus_lead", &Instrument::synth_lead())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .chorus(Chorus::new(0.4, 3.0, 0.3))
        .save_template("chorus_sound")
        .at_mark("chorus")
        .notes(&[E4, G4, C5, B4], 1.0);

    // Reuse templates for harmonies
    comp.from_template("intro_sound", "intro_harmony")
        .at_mark("intro")
        .notes(&[E4, F4, G4, B4], 0.5);

    comp.from_template("verse_sound", "verse_harmony")
        .at_mark("verse")
        .pan(0.5)
        .notes(&[E4, G4, B4, D5, E5, D5, B4, G4], 0.5);

    // ===== PART 7: TEMPLATES WITH PROGRESSIONS =====
    println!("Part 7: Templates for Chord Progressions\n");
    println!("  ♪ Perfect for creating multiple chord layers:\n");

    comp.instrument("chords_main", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .volume(0.6)
        .save_template("chord_sound")
        .at(16.0)
        .progression(C4, &ScalePattern::MAJOR, &[1, 5, 6, 4], 1.5);

    // Add a harmony layer with same sound
    comp.from_template("chord_sound", "chords_harmony")
        .at(16.0)
        .volume(0.3)
        .pan(-0.5)
        .progression_7th(C5, &ScalePattern::MAJOR, &[1, 5, 6, 4], 1.5);

    // ===== PART 8: REAL-WORLD EXAMPLE - DRUM TEMPLATES =====
    println!("Part 8: Drum Templates\n");
    println!("  ♪ Create consistent drum sounds across patterns:\n");

    comp.track("drums_kit")
        .volume(0.9)
        .compressor(Compressor::new(-18.0, 3.0, 3.0, 100.0, 1.0))
        .save_template("drum_sound")
        .at(22.0)
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

    comp.from_template("drum_sound", "drums_variation")
        .at(24.0)
        .drum_grid(16, 0.125)
        .kick(&[0, 6, 10])
        .snare(&[4, 12])
        .hihat(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    // ===== PART 9: TEMPLATE LIBRARY PATTERN =====
    println!("Part 9: Building a Template Library\n");
    println!("  ♪ Create a library of sounds at the start of your composition:\n");

    // Create a library of templates
    comp.instrument("lib_bright", &Instrument::synth_lead())
        .reverb(Reverb::new(0.3, 0.3, 0.2))
        .delay(Delay::new(0.375, 0.2, 0.4))
        .save_template("bright_lead")
        .at(26.0);

    comp.instrument("lib_dark", &Instrument::synth_lead())
        .reverb(Reverb::new(0.8, 1.0, 0.7))
        .filter(Filter::low_pass(1000.0, 0.5))
        .save_template("dark_lead")
        .at(26.0);

    comp.instrument("lib_aggressive", &Instrument::synth_lead())
        .distortion(Distortion::new(0.5, 0.7))
        .save_template("aggressive_lead")
        .at(26.0);

    // Now use them throughout the composition
    comp.from_template("bright_lead", "melody1")
        .at(26.0)
        .notes(&[C4, D4, E4, F4], 0.25);

    comp.from_template("dark_lead", "melody2")
        .at(27.0)
        .notes(&[G4, A4, B4, C5], 0.25);

    comp.from_template("aggressive_lead", "melody3")
        .at(28.0)
        .notes(&[C5, B4, A4, G4], 0.25);

    println!("\n▶️  Playing templates demonstration...\n");
    println!("    Duration: ~29 seconds\n");
    println!("    💎 Features demonstrated:");
    println!("    • Basic template creation with .save_template()");
    println!("    • Template reuse with .from_template()");
    println!("    • Templates capture ALL settings (effects, synthesis, mix)");
    println!("    • Template variations (override settings after loading)");
    println!("    • Templates with custom synthesis parameters");
    println!("    • Templates with song structure (markers)");
    println!("    • Templates for chord progressions");
    println!("    • Drum templates for consistent patterns");
    println!("    • Building a template library for reuse\n");
    println!("    💡 Benefits:");
    println!("    • Eliminate repetitive configuration code");
    println!("    • Maintain consistency across similar tracks");
    println!("    • Easy to experiment with sound variations");
    println!("    • Build a reusable sound library");
    println!("    • Faster composition workflow\n");

    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Templates Demo Complete!\n");
    println!("💡 Key Methods:");
    println!("   • .save_template(name)");
    println!("     → Saves all current track settings as a template");
    println!("   • comp.from_template(template_name, new_track_name)");
    println!("     → Creates a new track with all template settings");
    println!("   • Override any setting after loading a template");
    println!("     → comp.from_template(\"name\", \"track\").volume(0.5)\n");
    println!("💡 What Gets Saved:");
    println!("   ✓ All effects (reverb, delay, chorus, etc.)");
    println!("   ✓ Mix settings (volume, pan)");
    println!("   ✓ Synthesis parameters (waveform, envelope, FM)");
    println!("   ✓ Filter settings");
    println!("   ✓ MIDI program");
    println!("   ✓ Everything except notes/timing!\n");

    Ok(())
}
