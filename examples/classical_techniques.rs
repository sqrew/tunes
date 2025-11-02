use tunes::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎹 Classical Compositional Techniques Demo\n");
    println!("Demonstrating:");
    println!("1. Alberti Bass - The quintessential Classical accompaniment (Mozart, Haydn)");
    println!("2. Waltz Bass - Classic oom-pah-pah in 3/4 time");
    println!("3. Stride Piano - Jazz piano technique with alternating bass");
    println!("4. Broken Chord Arpeggios - Flowing accompaniment");
    println!("5. Walking Bass - Smooth chromatic movement");
    println!("6. Tremolo - Rapid repetition of a single note");
    println!("7. Ostinato - Repeating rhythmic/melodic pattern");
    println!("8. Finale - Combined techniques\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // Use single track for better performance
    // The builder pattern tracks cursor position automatically - no manual time tracking needed!
    comp.instrument("demo", &Instrument::acoustic_piano())
        .reverb(Reverb::new(0.5, 0.6, 0.4))

        // ===== 1. ALBERTI BASS =====
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[B2, D3, G3], 0.125)
        .alberti_bass(&[B2, D3, G3], 0.125)
        .wait(0.5) // Pause between techniques

        // ===== 2. WALTZ BASS =====
        .waltz_bass(C2, &[C3, E3, G3], 0.5)
        .waltz_bass(F2, &[F3, A3, C4], 0.5)
        .waltz_bass(G2, &[G3, B3, D4], 0.5)
        .waltz_bass(C2, &[C3, E3, G3], 0.5)
        .wait(0.5)

        // ===== 3. STRIDE PIANO =====
        .stride_bass(C2, &[C3, E3, G3], 0.25, 1)
        .stride_bass(C2, &[C3, E3, G3], 0.25, 1)
        .wait(0.5)

        // ===== 4. BROKEN CHORD ARPEGGIOS =====
        .broken_chord(&[C3, E3, G3, C4], 0, 0.15)
        .broken_chord(&[F3, A3, C4, F4], 0, 0.15)
        .broken_chord(&[G3, B3, D4, G4], 0, 0.15)
        .broken_chord(&[C3, E3, G3, C4], 0, 0.15)
        .wait(0.5)

        // ===== 5. WALKING BASS =====
        .walking_bass(&[C2, D2, E2, F2, G2, A2, B2, C3], 0.25)
        .wait(0.5)

        // ===== 6. TREMOLO =====
        .tremolo(C4, 16, 0.05)
        .wait(0.5)
        .tremolo(E4, 16, 0.05)
        .wait(0.5)

        // ===== 7. OSTINATO =====
        .pattern_start()
        .notes(&[C3, E3, G3, E3], 0.2)
        .repeat(3)
        .wait(0.5)

        // ===== 8. FINALE =====
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[F3, A3, C4], 0.125)
        .alberti_bass(&[F3, A3, C4], 0.125);

    println!("▶ Playing demonstration...\n");
    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;
    println!("✅ Demo complete!\n");

    Ok(())
}
