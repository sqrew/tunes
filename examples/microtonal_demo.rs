use tunes::prelude::*;


fn main() -> anyhow::Result<()> {
    println!("\n🎵 Microtonal Music Demonstration\n");
    println!("Exploring alternative tuning systems beyond 12-tone equal temperament.\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(100.0));

    let note_duration = 0.4;
    let pause = 0.5;

    // Prepare all scales before building the track
    println!("1. Standard 12-TET (for reference)");
    let edo12_scale = EDO12.octave(C4);

    println!("\n2. Quarter Tones (24-TET) - Used in Arabic music");
    println!("   Demonstrates notes between the standard semitones");
    let quarter_tone_scale = vec![
        C4,
        quarter_sharp(C4),   // C quarter-sharp
        CS4,                 // C sharp (half-sharp)
        quarter_sharp(CS4),  // C three-quarter-sharp
        D4,
        quarter_sharp(D4),
        DS4,
        quarter_sharp(DS4),
        E4,
    ];

    println!("\n3. 19-Tone Equal Temperament");
    println!("   Historical tuning with better thirds than 12-TET");
    let edo19 = Edo::new(19);
    let edo19_octave = edo19.octave(D4);

    println!("\n4. 31-Tone Equal Temperament");
    println!("   Approximates meantone temperament very well");
    let edo31 = Edo::new(31);
    let edo31_partial: Vec<f32> = (0..16).map(|i| edo31.step(E4, i)).collect();

    println!("\n5. Just Intonation - Pure Major Scale");
    println!("   Based on simple integer ratios (no beating)");
    let just_major = just_major_scale(C4);

    comp.instrument("microtonal", &Instrument::warm_pad())
        // 1. Standard 12-TET
        .notes(&edo12_scale, note_duration * 0.8)
        .wait(pause)
        // 2. Quarter tones
        .notes(&quarter_tone_scale, note_duration)
        .wait(pause)
        // 3. 19-TET
        .notes(&edo19_octave, note_duration * 0.7)
        .wait(pause)
        // 4. 31-TET
        .notes(&edo31_partial, note_duration * 0.6)
        .wait(pause)
        // 5. Just intonation
        .notes(&just_major, note_duration)
        .wait(pause)

        // 6. Just intonation chord progression
        .note(&[C4], note_duration)
        .note(&[just_ratio(C4, 5, 4)], note_duration)   // Pure major third
        .note(&[just_ratio(C4, 3, 2)], note_duration)   // Pure perfect fifth
        .note(&[just_ratio(C4, 2, 1)], note_duration)   // Octave
        .wait(pause)
        // 7. Pythagorean tuning
        .notes(&pythagorean_scale(D4), note_duration * 0.7)
        .wait(pause)
        // 8. 53-TET
        .notes(&(0..27).map(|i| Edo::new(53).step(F4, i)).collect::<Vec<_>>(), note_duration * 0.5)
        .wait(pause)
        // 9. Cents-based detuning
        .notes(&vec![
            C4,
            freq_from_cents(C4, 150.0),
            freq_from_cents(C4, 350.0),
            freq_from_cents(C4, 550.0),
            freq_from_cents(C4, 700.0),
            freq_from_cents(C4, 900.0),
            freq_from_cents(C4, 1100.0),
            freq_from_cents(C4, 1200.0),
        ], note_duration)
        .wait(pause)
        // 10. Comparing tuning systems
        .note(&[C4], note_duration * 0.5)
        .note(&[EDO12.step(C4, 4)], note_duration * 0.5)
        .wait(0.2)
        .note(&[C4], note_duration * 0.5)
        .note(&[just_ratio(C4, 5, 4)], note_duration * 0.5)
        .wait(0.2)
        .note(&[C4], note_duration * 0.5)
        .note(&[EDO19.step(C4, 6)], note_duration * 0.5)
        .wait(pause)
        // 11. Harmonic series
        .notes(&(1..=12).map(|n| C2 * n as f32).collect::<Vec<_>>(), note_duration * 0.8);

    println!("\n6. Just Intonation - Pure Intervals");
    println!("   Demonstrating pure perfect fifth (3:2) and major third (5:4)");
    println!("\n7. Pythagorean Tuning");
    println!("   Based on stacking pure perfect fifths");
    println!("\n8. 53-Tone Equal Temperament");
    println!("   Historical system with very small steps");
    println!("\n9. Cents-Based Microtonal Adjustments");
    println!("   Creating custom intervals with cent offsets");
    println!("\n10. Major Third in Different Tunings");
    println!("    Comparing the same interval in different systems");
    println!("\n11. Natural Harmonic Series");
    println!("    The pure overtones of a fundamental frequency");

    println!("\n▶ Playing microtonal demonstration...\n");
    engine.play_mixer(&comp.into_mixer())?;

    println!("✅ Demo complete!\n");
    println!("📚 Tuning systems demonstrated:");
    println!("   • 12-TET - Standard Western tuning (reference)");
    println!("   • 24-TET - Quarter tones (Arabic, contemporary)");
    println!("   • 19-TET - Historical alternative with better thirds");
    println!("   • 31-TET - Approximates meantone temperament");
    println!("   • 53-TET - Mercator's comma, very fine divisions");
    println!("   • Just Intonation - Pure integer ratios (5:4, 3:2, etc.)");
    println!("   • Pythagorean Tuning - Based on stacking pure fifths");
    println!("   • Cents-based - Custom detuning in cents");
    println!("   • Harmonic Series - Natural overtones\n");

    println!("💡 Available functions:");
    println!("   • Edo::new(n) - Create any equal temperament");
    println!("   • EDO19, EDO24, EDO31, EDO53 - Common systems");
    println!("   • just_ratio(freq, num, den) - Pure intervals");
    println!("   • just_major_scale(), just_minor_scale()");
    println!("   • pythagorean_scale()");
    println!("   • quarter_sharp(), quarter_flat() - Quarter tones");
    println!("   • cents_to_ratio(), freq_from_cents() - Cent calculations\n");

    Ok(())
}
