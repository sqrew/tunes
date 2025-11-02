use tunes::prelude::*;

/// Demonstration of the harmonic series for spectral and additive synthesis
fn main() -> anyhow::Result<()> {
    println!("\n🎵 Harmonic Series Demo\n");
    println!("Exploring the overtone series - the foundation of musical timbre\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(80.0));

    // ===== PART 1: BASIC HARMONIC SERIES =====
    println!("Part 1: The Natural Overtone Series\n");
    println!("  Playing the first 16 harmonics of A1 (55 Hz)");
    println!("  You'll hear: fundamental, octave, fifth, octave, third, fifth, seventh...\n");

    let harmonics = harmonic_series(55.0, 16);

    comp.instrument("overtones", &Instrument::pluck())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .notes(&harmonics, 0.5);

    // ===== PART 2: HARMONIC CHORD CLUSTERS =====
    println!("Part 2: Spectral Chord Clusters\n");
    println!("  Playing harmonics 8-16 as a chord (spectral music technique)\n");

    let high_harmonics = harmonic_series(55.0, 16);
    let chord_cluster = &high_harmonics[7..16]; // Harmonics 8-16

    comp.instrument("cluster", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(8.5)
        .note(chord_cluster, 3.0);

    // ===== PART 3: JUST INTONATION CHORDS =====
    println!("Part 3: Just Intonation from Harmonics\n");
    println!("  Harmonics 4-5-6 create a perfectly tuned major triad\n");

    // Major triad from harmonics (C major in this case)
    let c_fund = 65.41; // C2
    let c_harmonics = harmonic_series(c_fund, 6);
    let c_major_just = vec![c_harmonics[3], c_harmonics[4], c_harmonics[5]]; // 4th, 5th, 6th harmonics

    comp.instrument("just_major", &Instrument::warm_pad())
        .chorus(Chorus::new(0.3, 2.5, 0.3))
        .at(12.0)
        .note(&c_major_just, 2.0);

    // Compare with equal temperament (slightly out of tune)
    comp.instrument("equal_temp", &Instrument::warm_pad())
        .chorus(Chorus::new(0.3, 2.5, 0.3))
        .at(14.5)
        .note(&chord(C4, &ChordPattern::MAJOR), 2.0);

    // ===== PART 4: ADDITIVE SYNTHESIS =====
    println!("Part 4: Additive Synthesis - Building Timbre\n");
    println!("  Creating a sawtooth-like wave from 32 harmonics\n");

    let sawtooth_harmonics = harmonic_series(110.0, 32);

    comp.track("sawtooth")
        .at(17.0)
        .note(&sawtooth_harmonics, 2.0);

    // ===== PART 5: SUBHARMONICS (REVERSE OVERTONES) =====
    println!("Part 5: Exploring Different Fundamentals\n");
    println!("  Same harmonic relationships from different roots\n");

    // Low fundamental
    let low_series = harmonic_series(55.0, 8);
    comp.instrument("low_series", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(19.5)
        .notes(&low_series, 0.3);

    // High fundamental
    let high_series = harmonic_series(220.0, 8);
    comp.instrument("high_series", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(22.0)
        .notes(&high_series, 0.3);

    // ===== PART 6: THE FAMOUS 7TH HARMONIC "BLUE NOTE" =====
    println!("Part 6: The 7th Harmonic - The Natural 'Blue Note'\n");
    println!("  The 7th harmonic is a very flat minor seventh");
    println!("  This is where blues notes come from in the overtone series!\n");

    let blues_fundamentals = vec![110.0, 146.83, 164.81, 220.0]; // A2, D3, E3, A3

    for (i, &fundamental) in blues_fundamentals.iter().enumerate() {
        let h = harmonic_series(fundamental, 7);
        let seventh_harmonic = h[6]; // 7th harmonic

        comp.instrument(&format!("blues_{}", i), &Instrument::synth_lead())
            .reverb(Reverb::new(0.5, 0.5, 0.3))
            .at(24.5 + i as f32 * 0.8)
            .note(&[fundamental], 0.3)
            .note(&[seventh_harmonic], 0.5);
    }

    // ===== PART 7: SPECTRAL MELODY =====
    println!("Part 7: Spectral Melody\n");
    println!("  Creating a melody by walking through harmonics of changing fundamentals\n");

    let melody_fundamentals = vec![
        110.0, // A2
        98.0,  // G2
        82.41, // E2
        73.42, // D2
    ];

    let mut spectral_melody = Vec::new();
    for &fund in &melody_fundamentals {
        let h = harmonic_series(fund, 12);
        spectral_melody.extend_from_slice(&h[4..8]); // Use harmonics 5-8
    }

    comp.instrument("spectral_melody", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.6))
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(28.0)
        .notes(&spectral_melody, 0.15);

    // ===== PART 8: HARMONIC PROGRESSION =====
    println!("Part 8: Chord Progression Using Harmonic Subsets\n");
    println!("  Each chord uses different harmonics from the same series\n");

    let prog_fund = 55.0; // A1
    let full_harmonics = harmonic_series(prog_fund, 16);

    // Different harmonic subsets create different chord qualities
    comp.instrument("harm_chord1", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(30.5)
        .note(
            &[full_harmonics[3], full_harmonics[4], full_harmonics[5]],
            2.0,
        ) // Major
        .note(
            &[full_harmonics[5], full_harmonics[6], full_harmonics[7]],
            2.0,
        ) // Different voicing
        .note(
            &[full_harmonics[7], full_harmonics[9], full_harmonics[11]],
            2.0,
        ) // Dissonant
        .note(
            &[full_harmonics[3], full_harmonics[4], full_harmonics[5]],
            2.0,
        ); // Return home

    // ===== PART 9: BELL-LIKE INHARMONIC TONES =====
    println!("Part 9: Comparing Harmonic vs Detuned (Bell-like)\n");
    println!("  Pure harmonics vs slightly detuned (inharmonic) for bell tones\n");

    // Pure harmonic (will sound very stable)
    let pure = harmonic_series(220.0, 8);
    comp.track("pure_harmonics").at(39.0).note(&pure, 2.0);

    // Detuned slightly (bell-like, metallic)
    let mut bell_like = harmonic_series(220.0, 8);
    for (i, freq) in bell_like.iter_mut().enumerate() {
        *freq *= 1.0 + (i as f32 * 0.03); // Progressive detuning
    }
    comp.track("bell_like").at(41.5).note(&bell_like, 2.0);

    println!("\n▶️  Playing harmonic series demonstration...\n");
    println!("    Duration: ~44 seconds\n");
    println!("    💎 Concepts demonstrated:");
    println!("    • Natural overtone series (1f, 2f, 3f, 4f...)");
    println!("    • Spectral chord clusters");
    println!("    • Just intonation vs equal temperament");
    println!("    • Additive synthesis from harmonics");
    println!("    • The 7th harmonic 'blue note'");
    println!("    • Spectral melody construction");
    println!("    • Harmonic chord progressions");
    println!("    • Pure vs inharmonic tones\n");
    println!("    📚 Music Theory:");
    println!("    • Harmonics 4-5-6 = major triad (1.0 : 1.25 : 1.5)");
    println!("    • Harmonic 7 = natural minor seventh (flat!)");
    println!("    • Powers of 2 (harmonics 2,4,8,16) = octaves");
    println!("    • The overtone series is the basis of all timbre\n");

    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Harmonic Series Demo Complete!\n");
    println!("💡 Try experimenting with:");
    println!("   • Different fundamentals (low frequencies = more audible harmonics)");
    println!("   • Playing higher harmonics (8-32) for dissonant clusters");
    println!("   • Using sequences::normalize() to map harmonics to scale degrees");
    println!("   • Combining with FM synthesis for complex spectra\n");

    Ok(())
}
