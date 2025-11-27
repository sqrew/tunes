use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("🎵 Instrument Showcase - New Voices and Percussion\n");

    let mut comp = Composition::new(Tempo::new(110.0));

    // ===== REALISTIC INSTRUMENTS =====
    println!("Part 1: Realistic Instruments");

    // 1. Acoustic Piano
    println!("  • Acoustic Piano");
    comp.instrument("piano", &Instrument::acoustic_piano())
        .at(0.0)
        .notes(&[C4, E4, G4, C5, G4, E4, C4, G3], 0.35);

    // 2. Strings
    println!("  • Strings");
    comp.instrument("strings", &Instrument::strings())
        .at(3.0)
        .note(&[G3, B3, D4], 2.5);

    // 3. Brass
    println!("  • Brass");
    comp.instrument("brass", &Instrument::brass())
        .at(5.7)
        .notes(&[C4, D4, E4, G4], 0.4);

    // 4. Flute
    println!("  • Flute");
    comp.instrument("flute", &Instrument::flute())
        .at(7.5)
        .notes(&[G4, A4, B4, C5, B4, A4, G4, E4], 0.25);

    // 5. Clarinet
    println!("  • Clarinet");
    comp.instrument("clarinet", &Instrument::clarinet())
        .at(9.5)
        .notes(&[E4, G4, B4, E5, B4, G4], 0.3);

    // 6. Upright Bass
    println!("  • Upright Bass");
    comp.instrument("upright_bass", &Instrument::upright_bass())
        .at(11.5)
        .notes(&[C2, G2, C3, G2, C2, E2, G2, C3], 0.25);

    // ===== GENRE-SPECIFIC SYNTHS =====
    println!("\nPart 2: Genre-Specific Synths");

    // 7. Supersaw (Trance)
    println!("  • Supersaw");
    comp.instrument("supersaw", &Instrument::supersaw())
        .at(13.5)
        .note(&[C4, E4, G4], 2.0);

    // 8. FM Bells
    println!("  • FM Bells");
    comp.instrument("fm_bells", &Instrument::fm_bells())
        .at(15.7)
        .notes(&[C5, G4, C5, E5, G5], 0.5);

    // 9. Hoover (Rave)
    println!("  • Hoover");
    comp.instrument("hoover", &Instrument::hoover())
        .at(18.2)
        .note(&[G3], 2.0);

    // 10. Stab
    println!("  • Stab");
    comp.instrument("stab", &Instrument::stab())
        .at(20.5)
        .notes(&[C4, C4], 0.15)
        .wait(0.3)
        .notes(&[E4, E4], 0.15)
        .wait(0.3)
        .notes(&[G4, G4], 0.15);

    // 11. Chiptune (8-bit)
    println!("  • Chiptune");
    comp.instrument("chiptune", &Instrument::chiptune())
        .at(22.0)
        .notes(&[C5, E5, G5, C6, G5, E5, C5, G4], 0.15);

    // 12. Vocal Pad
    println!("  • Vocal Pad");
    comp.instrument("vocal_pad", &Instrument::vocal_pad())
        .at(24.0)
        .note(&[C3, E3, G3], 3.0);

    // 13. Mallet
    println!("  • Mallet");
    comp.instrument("mallet", &Instrument::mallet())
        .at(27.2)
        .notes(&[C5, D5, E5, G5, E5, D5, C5, G4], 0.25);

    // ===== EXPANDED PERCUSSION =====
    println!("\nPart 3: Expanded Percussion");

    // 14. Tom fills (high, mid, low)
    println!("  • Tom fills");
    comp.instrument("tom_fill", &Instrument::sub_bass())
        .at(29.5)
        .drum_grid(16, 0.125, |g| g
        .sound(DrumType::TomHigh, &[0, 1])
        .sound(DrumType::Tom, &[2, 3])
        .sound(DrumType::TomLow, &[4, 5, 6, 7]));

    // 15. Cymbal variety
    println!("  • Cymbal variety");
    comp.instrument("cymbals", &Instrument::sub_bass())
        .at(31.5)
        .drum_grid(16, 0.25, |g| g
        .sound(DrumType::Crash, &[0])
        .sound(DrumType::Ride, &[2, 4, 6])
        .sound(DrumType::China, &[8])
        .sound(DrumType::Splash, &[10, 11]));

    // 16. Shaker and Tambourine
    println!("  • Shaker and Tambourine");
    comp.instrument("perc", &Instrument::sub_bass())
        .at(35.5)
        .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Shaker, &[0, 2, 4, 6, 8, 10, 12, 14])
        .sound(DrumType::Tambourine, &[4, 12]))
        .repeat(1);

    // ===== MUSICAL EXAMPLE: COMBINING NEW INSTRUMENTS =====
    println!("\nPart 4: Musical Example - Orchestra meets Synth");

    // Orchestral chord progression
    comp.instrument("orch_strings", &Instrument::strings())
        .at(39.0)
        .note(&[C3, E3, G3], 2.0)
        .note(&[F3, A3, C4], 2.0)
        .note(&[G3, B3, D4], 2.0)
        .note(&[C3, E3, G3], 2.0);

    // Brass stabs
    comp.instrument("orch_brass", &Instrument::brass())
        .at(39.0)
        .notes(&[C4, E4], 0.3)
        .wait(1.7)
        .notes(&[F4, A4], 0.3)
        .wait(1.7)
        .notes(&[G4, B4], 0.3)
        .wait(1.7)
        .notes(&[C5, E5], 0.3);

    // Piano melody over chords
    comp.instrument("orch_piano", &Instrument::acoustic_piano())
        .at(39.5)
        .notes(&[G4, A4, B4, C5], 0.4)
        .notes(&[A4, B4, C5, D5], 0.4)
        .notes(&[B4, C5, D5, E5], 0.4)
        .notes(&[C5, B4, A4, G4], 0.4);

    // Synth layer
    comp.instrument("synth_layer", &Instrument::supersaw())
        .at(47.0)
        .note(&[C4, E4, G4], 2.0)
        .note(&[F3, A3, C4], 2.0)
        .note(&[G3, B3, D4], 2.0)
        .note(&[C4, E4, G4], 2.0);

    // Orchestral percussion
    comp.instrument("orch_perc", &Instrument::sub_bass())
        .at(47.0)
        .drum_grid(32, 0.25, |g| g
        .sound(DrumType::Kick, &[0, 8, 16, 24])
        .sound(DrumType::Snare, &[8, 24])
        .sound(DrumType::Crash, &[0, 16])
        .sound(DrumType::Tambourine, &[4, 12, 20, 28])
        .sound(DrumType::TomHigh, &[14, 15])
        .sound(DrumType::TomLow, &[30, 31]));

    // FM Bells outro
    comp.instrument("bells_outro", &Instrument::fm_bells())
        .at(55.0)
        .notes(&[C5, G4, E4, C4], 1.0);

    // ===== CLASSIC SYNTH INSTRUMENTS =====
    println!("\nPart 5: Classic Synth Instruments");

    // Bass synths
    println!("  • Bass Synths");
    comp.instrument("reese", &Instrument::reese_bass())
        .at(59.0)
        .note(&[C2], 0.8);

    comp.instrument("acid", &Instrument::acid_bass())
        .at(60.0)
        .note(&[C2], 0.8);

    comp.instrument("wobble", &Instrument::wobble_bass())
        .at(61.0)
        .note(&[C2], 0.8);

    // Lead synths
    println!("  • Lead Synths");
    comp.instrument("pluck_lead", &Instrument::pluck())
        .at(62.0)
        .notes(&[C5, E5, G5, C6], 0.2);

    comp.instrument("saw_lead_demo", &Instrument::saw_lead())
        .at(62.8)
        .notes(&[C5, E5, G5, C6], 0.2);

    comp.instrument("square_lead_demo", &Instrument::square_lead())
        .at(63.6)
        .notes(&[C5, E5, G5, C6], 0.2);

    comp.instrument("synth_lead_demo", &Instrument::synth_lead())
        .at(64.4)
        .notes(&[C5, E5, G5, C6], 0.2);

    // Pad synths
    println!("  • Pad Synths");
    comp.instrument("warm_pad_demo", &Instrument::warm_pad())
        .at(65.5)
        .note(&[C3, E3, G3], 2.0);

    comp.instrument("ambient_pad_demo", &Instrument::ambient_pad())
        .at(67.5)
        .note(&[C3, E3, G3, C4], 2.5);

    // Keys
    println!("  • Keys");
    comp.instrument("organ_demo", &Instrument::organ())
        .at(70.0)
        .notes(&[C4, E4, G4, C5], 0.3);

    comp.instrument("epiano_demo", &Instrument::electric_piano())
        .at(71.2)
        .notes(&[C4, E4, G4, C5, G4, E4], 0.25);

    // FX
    println!("  • FX");
    comp.instrument("riser_demo", &Instrument::riser())
        .at(72.8)
        .note(&[C3], 2.0);

    comp.instrument("impact_demo", &Instrument::impact())
        .at(75.0)
        .note(&[C2], 0.5);

    println!("\n▶️  Playing comprehensive instrument showcase...");
    println!("    Duration: ~76 seconds\n");
    println!("    💎 Instruments Demonstrated:");
    println!("       REALISTIC:");
    println!("       • Acoustic Piano, Strings, Brass");
    println!("       • Flute, Clarinet, Upright Bass");
    println!("       GENRE-SPECIFIC SYNTHS:");
    println!("       • Supersaw, FM Bells, Hoover, Stab");
    println!("       • Chiptune, Vocal Pad, Mallet");
    println!("       CLASSIC SYNTHS:");
    println!("       • Reese/Acid/Wobble Bass, Sub Bass");
    println!("       • Pluck, Saw/Square/Synth Leads");
    println!("       • Warm/Ambient Pads");
    println!("       • Organ, Electric Piano");
    println!("       • Riser, Impact");
    println!("       PERCUSSION:");
    println!("       • Tom High/Mid/Low, China, Splash");
    println!("       • Tambourine, Shaker");

    let mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;

    println!("\n✅ Showcase complete!");
    println!("   {} instrument presets total!", 26);
    println!("   {} percussion sounds!", 6);

    Ok(())
}
