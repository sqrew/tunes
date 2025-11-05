use tunes::prelude::*;
use tunes::sequences;

/// Showcase of musical ratio and tuning sequences
///
/// This example demonstrates sequences based on musical theory:
/// - Harmonic Series: Overtones (1, 2, 3, 4, 5... × fundamental)
/// - Undertone Series: Subharmonics (1, 1/2, 1/3, 1/4...)
/// - Circle of Fifths: Key relationships through perfect fifths
/// - Circle of Fourths: Reverse progression (perfect fourths)
/// - Pythagorean Tuning: Pure fifth-based tuning system
/// - Just Intonation: Pure harmonic ratios for scales
/// - Golden Ratio: φ (phi) for natural proportions
fn main() -> anyhow::Result<()> {
    println!("\n🎼 Musical Sequences: Ratios, Tuning & Harmony\n");
    println!("Exploring the mathematical foundations of music theory\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(100.0));

    // ===== HARMONIC (OVERTONE) SERIES =====
    println!("1. Harmonic Series (Overtones)\n");
    println!("   f, 2f, 3f, 4f, 5f, 6f, 7f, 8f...");
    println!("   The foundation of all musical timbre!");
    println!("   Natural frequencies that vibrate together\n");

    let harmonics = sequences::harmonic_series(110.0, 16);

    // Play first 8 harmonics as a chord
    comp.instrument("harmonics_chord", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .note(&harmonics[0..8], 3.0);

    // Play harmonics 4-12 as melody
    comp.instrument("harmonics_melody", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(3.5)
        .notes(&harmonics[3..12], 0.25);

    // ===== UNDERTONE SERIES =====
    println!("2. Undertone Series (Subharmonics)\n");
    println!("   f, f/2, f/3, f/4, f/5...");
    println!("   Mirror of harmonic series - darker, mysterious sound");
    println!("   Descending harmonic relationships\n");

    let undertones = sequences::undertone_series(12);
    let fundamental = 440.0; // A4
    let undertone_freqs: Vec<f32> = undertones
        .iter()
        .map(|&ratio| fundamental * ratio)
        .collect();

    comp.instrument("undertones", &Instrument::pluck())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(6.5)
        .notes(&undertone_freqs, 0.3);

    // ===== CIRCLE OF FIFTHS =====
    println!("3. Circle of Fifths\n");
    println!("   Moving up by perfect fifths (7 semitones)");
    println!("   C → G → D → A → E → B → F# → C# → ...");
    println!("   Foundation of key relationships in Western music\n");

    let fifths = sequences::circle_of_fifths(12, 0); // Starting from C
    let fifth_freqs: Vec<f32> = fifths
        .iter()
        .map(|&semitones| 261.63 * 2.0f32.powf(semitones as f32 / 12.0)) // C4
        .collect();

    comp.instrument("circle_fifths", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.6))
        .at(10.5)
        .notes(&fifth_freqs, 0.3);

    // ===== CIRCLE OF FOURTHS =====
    println!("4. Circle of Fourths\n");
    println!("   Moving up by perfect fourths (5 semitones)");
    println!("   C → F → Bb → Eb → Ab → Db → ...");
    println!("   Reverse of circle of fifths\n");

    let fourths = sequences::circle_of_fourths(12, 0);
    let fourth_freqs: Vec<f32> = fourths
        .iter()
        .map(|&semitones| 261.63 * 2.0f32.powf(semitones as f32 / 12.0))
        .collect();

    comp.instrument("circle_fourths", &Instrument::pluck())
        .chorus(Chorus::new(0.3, 2.5, 0.3))
        .at(14.5)
        .notes(&fourth_freqs, 0.3);

    // ===== PYTHAGOREAN TUNING =====
    println!("5. Pythagorean Tuning\n");
    println!("   Ancient tuning system based on stacking perfect fifths");
    println!("   Ratio of 3:2 creates pure, singing fifths");
    println!("   Different 'color' than equal temperament\n");

    let pyth_tuning = sequences::pythagorean_tuning(12);
    let pyth_root = 220.0; // A3
    let pyth_scale: Vec<f32> = pyth_tuning.iter().map(|&ratio| pyth_root * ratio).collect();

    comp.instrument("pythagorean", &Instrument::pluck())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(18.5)
        .notes(&pyth_scale, 0.25);

    // ===== JUST INTONATION - MAJOR SCALE =====
    println!("6. Just Intonation - Major Scale\n");
    println!("   Pure harmonic ratios: 1/1, 9/8, 5/4, 4/3, 3/2, 5/3, 15/8, 2/1");
    println!("   Perfectly tuned intervals - very consonant");
    println!("   Used in vocal and string ensembles\n");

    let just_major = sequences::just_intonation_major();
    let just_root = 264.0; // C4
    let just_major_scale: Vec<f32> = just_major.iter().map(|&ratio| just_root * ratio).collect();

    comp.instrument("just_major", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(21.5)
        .notes(&just_major_scale, 0.4);

    // ===== JUST INTONATION - MINOR SCALE =====
    println!("7. Just Intonation - Minor Scale\n");
    println!("   Pure minor intervals: 1/1, 9/8, 6/5, 4/3, 3/2, 8/5, 9/5, 2/1");
    println!("   Minor third at 6/5 ratio (slightly flatter than equal temp)\n");

    let just_minor = sequences::just_intonation_minor();
    let just_minor_scale: Vec<f32> = just_minor.iter().map(|&ratio| just_root * ratio).collect();

    comp.instrument("just_minor", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(25.0)
        .notes(&just_minor_scale, 0.4);

    // ===== GOLDEN RATIO POWERS =====
    println!("8. Golden Ratio (φ) Powers\n");
    println!("   φ ≈ 1.618... (the divine proportion)");
    println!("   φ^n: 1.0, 1.618, 2.618, 4.236, 6.854...");
    println!("   Found in nature: shells, flowers, galaxies\n");

    let phi_powers = sequences::golden_ratio(10);
    let phi_freqs = sequences::normalize(
        &phi_powers.iter().map(|&x| x as u32).collect::<Vec<_>>(),
        200.0,
        800.0,
    );

    comp.instrument("golden_ratio", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.6))
        .at(28.5)
        .notes(&phi_freqs, 0.25);

    // ===== GOLDEN SECTIONS =====
    println!("9. Golden Section Divisions\n");
    println!("   Dividing a value by φ recursively");
    println!("   Creates natural decay: 800, 494.4, 305.6, 188.9...");
    println!("   Used for form, timing, and proportions\n");

    let sections = sequences::golden_sections(800.0, 10);

    comp.instrument("golden_sections", &Instrument::pluck())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(31.0)
        .notes(&sections, 0.3);

    // ===== SPECTRAL HARMONY =====
    println!("10. Spectral Harmony (Using Harmonic Series as Chords)\n");
    println!("    Different harmonic clusters create rich textures\n");

    let spectral_base = sequences::harmonic_series(55.0, 16); // A1

    // Cluster 1: Harmonics 4-5-6 (major triad)
    comp.instrument("spectral1", &Instrument::warm_pad())
        .reverb(Reverb::new(0.8, 0.7, 0.6))
        .at(34.0)
        .note(&[spectral_base[3], spectral_base[4], spectral_base[5]], 2.5);

    // Cluster 2: Harmonics 7-8-9 (dissonant cluster)
    comp.instrument("spectral2", &Instrument::warm_pad())
        .reverb(Reverb::new(0.8, 0.7, 0.6))
        .at(36.5)
        .note(&[spectral_base[6], spectral_base[7], spectral_base[8]], 2.5);

    // Cluster 3: Harmonics 10-11-12 (complex cluster)
    comp.instrument("spectral3", &Instrument::warm_pad())
        .reverb(Reverb::new(0.8, 0.7, 0.6))
        .at(39.0)
        .note(
            &[spectral_base[9], spectral_base[10], spectral_base[11]],
            2.5,
        );

    // ===== COMBINING TUNING SYSTEMS =====
    println!("11. Comparing Equal Temperament vs Just Intonation\n");
    println!("    Playing same melody in both systems\n");

    // Equal temperament (standard tuning)
    let equal_temp_c_major = vec![
        264.0,
        296.33,
        329.63,
        352.0,
        396.0,
        440.0,
        493.88,
        528.0,
    ];

    comp.instrument("equal_temp", &Instrument::pluck())
        .pan(-0.5)
        .at(42.0)
        .notes(&equal_temp_c_major, 0.3);

    // Just intonation (pure ratios)
    comp.instrument("just_temp", &Instrument::pluck())
        .pan(0.5)
        .at(42.0)
        .notes(&just_major_scale, 0.3);

    println!("\n▶️  Playing musical sequences...\n");
    println!("    Duration: ~45 seconds\n");

    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Musical Sequences Complete!\n");
    println!("💡 Key Takeaways:");
    println!("   • Harmonic series = natural overtones (bright, rising)");
    println!("   • Undertone series = subharmonics (dark, descending)");
    println!("   • Circle of fifths/fourths = key relationships");
    println!("   • Pythagorean tuning = pure fifths (3:2 ratio)");
    println!("   • Just intonation = pure harmonic ratios");
    println!("   • Golden ratio = natural, organic proportions\n");
    println!("🎵 Musical Applications:");
    println!("   • Spectral music composition");
    println!("   • Microtonal and alternative tuning exploration");
    println!("   • Harmonic chord voicings");
    println!("   • Key modulation and progression planning");
    println!("   • Natural-sounding proportions and forms\n");
    println!("📚 Try Next:");
    println!("   cargo run --example rhythmic_sequences");
    println!("   cargo run --example generative_sequences\n");

    Ok(())
}
