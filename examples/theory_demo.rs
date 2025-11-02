use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("🎵 Music Theory Helpers Demo\n");
    println!("This demo shows how to use programmatic scale and chord generation");
    println!("instead of relying on hardcoded constants.\n");

    let mut comp = Composition::new(Tempo::new(100.0));

    // 1. Scale generation - C Major scale
    println!("1. Scale Generation - C Major");
    let c_major_scale = scale(C4, &ScalePattern::MAJOR);
    comp.instrument("c_major", &Instrument::pluck())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .notes(&c_major_scale, 0.3);

    // 2. Different scale types from the same root
    println!("2. A Minor Pentatonic scale");
    let a_minor_pent = scale(A3, &ScalePattern::MINOR_PENTATONIC);
    comp.instrument("a_minor_pent", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.4))
        .at(2.5)
        .notes(&a_minor_pent, 0.25);

    // 3. Blues scale
    println!("3. E Blues scale");
    let e_blues = scale(E3, &ScalePattern::BLUES);
    comp.instrument("e_blues", &Instrument::pluck())
        .at(4.0)
        .notes(&e_blues, 0.3);

    // 4. Modal scales - Dorian mode
    println!("4. D Dorian mode (jazz flavor)");
    let d_dorian = scale(D4, &ScalePattern::DORIAN);
    comp.instrument("d_dorian", &Instrument::synth_lead())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(6.0)
        .notes(&d_dorian, 0.25);

    // 5. Chord generation - Major, Minor, Dominant7
    println!("5. Chord Generation - C Major, A Minor, G Dominant7");
    comp.instrument("chords_demo", &Instrument::warm_pad())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(8.0)
        .note(&chord(C4, &ChordPattern::MAJOR), 1.0)
        .note(&chord(A3, &ChordPattern::MINOR), 1.0)
        .note(&chord(G3, &ChordPattern::DOMINANT7), 1.5);

    // 6. Extended chords
    println!("6. Extended chords - Major7, Minor7, Add9");
    comp.instrument("ext_chords", &Instrument::warm_pad())
        .chorus(Chorus::new(0.4, 3.0, 0.3))
        .at(11.5)
        .note(&chord(F3, &ChordPattern::MAJOR7), 1.2)
        .note(&chord(D3, &ChordPattern::MINOR7), 1.2)
        .note(&chord(C3, &ChordPattern::ADD9), 1.5);

    // 7. Chord progression - I-V-vi-IV (Pop progression)
    println!("7. Chord Progression - I-V-vi-IV in C Major (using .progression() method)");
    comp.instrument("pop_prog", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(15.5)
        .progression(C4, &ScalePattern::MAJOR, &[1, 5, 6, 4], 1.5);

    // 8. Jazz progression - ii-V-I with 7th chords
    println!("8. Jazz Progression - ii7-V7-Imaj7 in C Major (using .progression_7th() method)");
    comp.instrument("jazz_prog", &Instrument::warm_pad())
        .chorus(Chorus::new(0.3, 2.5, 0.4))
        .at(21.5)
        .progression_7th(C4, &ScalePattern::MAJOR, &[2, 5, 1], 2.0);

    // 9. Transposition - Same melody in different keys
    println!("9. Transposition - Melody in C, then D, then E");
    let melody = vec![C4, D4, E4, G4, E4, D4, C4, G3];

    comp.instrument("melody_c", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(27.5)
        .notes(&melody, 0.2);

    // Transpose up 2 semitones (to D)
    let melody_d = transpose_sequence(&melody, 2);
    comp.instrument("melody_d", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(29.1)
        .notes(&melody_d, 0.2);

    // Transpose up 4 semitones (to E)
    let melody_e = transpose_sequence(&melody, 4);
    comp.instrument("melody_e", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(30.7)
        .notes(&melody_e, 0.2);

    // 10. Musical example - Full progression with melody
    println!("10. Full Musical Example - Progression with melody in G Major");

    // Chord progression: I-vi-IV-V in G major (using .progression() method)
    comp.instrument("full_chords", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .chorus(Chorus::new(0.3, 2.5, 0.3))
        .at(33.0)
        .progression(G3, &ScalePattern::MAJOR, &[1, 6, 4, 5], 2.0);

    // Melody using G major scale
    let g_major = scale(G4, &ScalePattern::MAJOR);
    let scale_melody = vec![
        g_major[0], g_major[2], g_major[4], g_major[2], g_major[5], g_major[4], g_major[2],
        g_major[0],
    ];

    comp.instrument("full_melody", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(33.0)
        .notes(&scale_melody, 0.5)
        .wait(0.5)
        .notes(&scale_melody, 0.5);

    // 11. Power chords for rock/metal
    println!("11. Power Chords - E5, G5, A5");
    comp.instrument("power_chords", &Instrument::synth_lead())
        .at(41.0)
        .note(&chord(E3, &ChordPattern::POWER_OCTAVE), 0.8)
        .wait(0.2)
        .note(&chord(G3, &ChordPattern::POWER_OCTAVE), 0.8)
        .wait(0.2)
        .note(&chord(A3, &ChordPattern::POWER_OCTAVE), 1.2);

    // 12. Exotic scales - Phrygian mode for Spanish/Middle Eastern flavor
    println!("12. E Phrygian mode - Exotic flavor");
    let e_phrygian = scale(E4, &ScalePattern::PHRYGIAN);
    comp.instrument("phrygian", &Instrument::pluck())
        .reverb(Reverb::new(0.6, 1.0, 0.4))
        .at(43.5)
        .notes(&e_phrygian, 0.25)
        .wait(0.5)
        .notes(
            &[e_phrygian[7], e_phrygian[6], e_phrygian[5], e_phrygian[4]],
            0.3,
        );

    println!("\n▶️  Playing music theory demo...");
    println!("    Duration: ~47 seconds\n");
    println!("    💎 Features demonstrated:");
    println!("    • Programmatic scale generation (Major, Minor, Pentatonic, Blues, Modes)");
    println!("    • Chord generation (Triads, 7ths, Extended chords, Power chords)");
    println!("    • Chord progressions (Pop, Jazz, Rock)");
    println!("    • Transposition of melodies to different keys");
    println!("    • Complete musical examples combining all features");
    println!("\n    No hardcoded scale constants needed!");

    let mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;

    println!("\n✅ Demo complete!");
    println!("   Now you can generate any scale or chord from any root note!");
    println!("   Try experimenting with different ScalePatterns and ChordPatterns.");

    Ok(())
}
