use tunes::composition::Composition;
use tunes::engine::AudioEngine;
use tunes::instruments::Instrument;
use tunes::microtonal::*;
use tunes::notes::*;
use tunes::rhythm::Tempo;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n🎵 Microtonal Music Demonstration\n");
    println!("Exploring alternative tuning systems beyond 12-tone equal temperament.\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(100.0));

    let note_duration = 0.4;
    let pause = 0.5;

    let mut track = comp.instrument("microtonal", &Instrument::warm_pad());

    // ===== 1. STANDARD 12-TET FOR COMPARISON =====
    println!("1. Standard 12-TET (for reference)");
    let edo12_scale = EDO12.octave(C4);
    track = track.notes(&edo12_scale, note_duration * 0.8).wait(pause);

    // ===== 2. QUARTER TONES (24-TET) =====
    println!("\n2. Quarter Tones (24-TET) - Used in Arabic music");
    println!("   Demonstrates notes between the standard semitones");

    // Ascending quarter-tone scale
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
    track = track.notes(&quarter_tone_scale, note_duration).wait(pause);

    // ===== 3. 19-TET =====
    println!("\n3. 19-Tone Equal Temperament");
    println!("   Historical tuning with better thirds than 12-TET");

    let edo19 = Edo::new(19);
    let edo19_octave = edo19.octave(D4);
    track = track.notes(&edo19_octave, note_duration * 0.7).wait(pause);

    // ===== 4. 31-TET =====
    println!("\n4. 31-Tone Equal Temperament");
    println!("   Approximates meantone temperament very well");

    let edo31 = Edo::new(31);
    // Play just the first 16 notes for brevity
    let edo31_partial: Vec<f32> = (0..16).map(|i| edo31.step(E4, i)).collect();
    track = track.notes(&edo31_partial, note_duration * 0.6).wait(pause);

    // ===== 5. JUST INTONATION =====
    println!("\n5. Just Intonation - Pure Major Scale");
    println!("   Based on simple integer ratios (no beating)");

    let just_major = just_major_scale(C4);
    track = track.notes(&just_major, note_duration).wait(pause);

    // ===== 6. JUST INTONATION CHORD PROGRESSION =====
    println!("\n6. Just Intonation - Pure Intervals");
    println!("   Demonstrating pure perfect fifth (3:2) and major third (5:4)");

    let c_just = C4;
    let e_just = just_ratio(C4, 5, 4);   // Pure major third
    let g_just = just_ratio(C4, 3, 2);   // Pure perfect fifth
    let c_octave = just_ratio(C4, 2, 1); // Octave

    track = track
        .note(&[c_just], note_duration)
        .note(&[e_just], note_duration)
        .note(&[g_just], note_duration)
        .note(&[c_octave], note_duration)
        .wait(pause);

    // ===== 7. PYTHAGOREAN TUNING =====
    println!("\n7. Pythagorean Tuning");
    println!("   Based on stacking pure perfect fifths");

    let pythagorean = pythagorean_scale(D4);
    track = track.notes(&pythagorean, note_duration * 0.7).wait(pause);

    // ===== 8. 53-TET =====
    println!("\n8. 53-Tone Equal Temperament");
    println!("   Historical system with very small steps");

    let edo53 = Edo::new(53);
    // Play first 27 notes (about half an octave)
    let edo53_partial: Vec<f32> = (0..27).map(|i| edo53.step(F4, i)).collect();
    track = track.notes(&edo53_partial, note_duration * 0.5).wait(pause);

    // ===== 9. CENTS-BASED DETUNING =====
    println!("\n9. Cents-Based Microtonal Adjustments");
    println!("   Creating custom intervals with cent offsets");

    // Create a scale with custom cent offsets
    let custom_scale = vec![
        C4,
        freq_from_cents(C4, 150.0),  // Between C# and D
        freq_from_cents(C4, 350.0),  // Between D# and E
        freq_from_cents(C4, 550.0),  // Between F and F#
        freq_from_cents(C4, 700.0),  // Perfect fifth (G)
        freq_from_cents(C4, 900.0),  // Between G# and A
        freq_from_cents(C4, 1100.0), // Between A# and B
        freq_from_cents(C4, 1200.0), // Octave
    ];
    track = track.notes(&custom_scale, note_duration).wait(pause);

    // ===== 10. COMPARING TUNING SYSTEMS =====
    println!("\n10. Major Third in Different Tunings");
    println!("    Comparing the same interval in different systems");

    // Play major third in different tunings
    track = track
        .note(&[C4], note_duration * 0.5)
        .note(&[EDO12.step(C4, 4)], note_duration * 0.5)   // 12-TET (400 cents)
        .wait(0.2)
        .note(&[C4], note_duration * 0.5)
        .note(&[just_ratio(C4, 5, 4)], note_duration * 0.5) // Just (386 cents)
        .wait(0.2)
        .note(&[C4], note_duration * 0.5)
        .note(&[EDO19.step(C4, 6)], note_duration * 0.5)   // 19-TET (~379 cents)
        .wait(pause);

    // ===== 11. HARMONIC SERIES =====
    println!("\n11. Natural Harmonic Series");
    println!("    The pure overtones of a fundamental frequency");

    // First 12 harmonics of C2
    let harmonics: Vec<f32> = (1..=12).map(|n| C2 * n as f32).collect();
    let _ = track.notes(&harmonics, note_duration * 0.8);

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
