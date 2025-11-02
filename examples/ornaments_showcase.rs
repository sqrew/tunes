use tunes::prelude::*;
use std::error::Error;

/// Comprehensive showcase of all ornamental and expressive techniques
fn main() -> Result<(), Box<dyn Error>> {
    println!("🎵 Ornaments and Expression Showcase\n");
    println!("Demonstrating all classical ornaments and expressive techniques.\n");

    let mut comp = Composition::new(Tempo::new(120.0));
    let pause = 0.5;

    // Use single track for better performance
    let mut track = comp.instrument("demo", &Instrument::acoustic_piano());

    // ===== 1. SLIDE / GLISSANDO =====
    println!("1. Slide / Glissando - Smooth pitch transitions");

    track = track
        .slide(C4, C5, 0.5)
        .wait(0.1)
        .slide(C5, C4, 0.5)
        .wait(pause);

    // ===== 2. GRACE NOTES =====
    println!("\n2. Grace Notes - Quick ornamental notes before main notes");

    track = track
        .grace(C4, B3, 0.05, 0.45)
        .grace(E4, D4, 0.05, 0.45)
        .grace(G4, FS4, 0.05, 0.45)
        .grace(C5, B4, 0.05, 0.45)
        .wait(pause);

    // Acciaccatura (crushed, very short)
    track = track
        .grace(E4, DS4, 0.02, 0.48)
        .grace(G4, FS4, 0.02, 0.48)
        .grace(C5, B4, 0.02, 0.48)
        .wait(pause);

    // Appoggiatura (leaning, emphasized)
    track = track
        .grace(C4, D4, 0.15, 0.35)
        .grace(E4, F4, 0.15, 0.35)
        .grace(G4, A4, 0.15, 0.35)
        .wait(pause);

    // ===== 3. TRILL =====
    println!("\n3. Trill - Rapid alternation between two notes");

    // Slow trill
    track = track
        .trill(C4, D4, 8, 0.1)
        .wait(pause);

    // Fast trill (classic)
    track = track
        .trill(E4, F4, 16, 0.05)
        .wait(pause);

    // Major third trill
    track = track
        .trill(G4, B4, 12, 0.08)
        .wait(pause);

    // ===== 4. TREMOLO =====
    println!("\n4. Tremolo - Rapid repetition of same note");

    track = track
        .tremolo(C4, 16, 0.06)
        .wait(pause);

    // ===== 5. MORDENT =====
    println!("\n5. Mordent - Quick alternation: Main-Upper-Main");

    track = track
        .mordent(E4, 0.8)
        .mordent(G4, 0.8)
        .wait(pause);

    track = track
        .inverted_mordent(E4, 0.8)
        .inverted_mordent(G4, 0.8)
        .wait(pause);

    // ===== 6. TURN =====
    println!("\n6. Turn - Upper-Main-Lower-Main");

    track = track
        .turn(C4, 0.8)
        .turn(E4, 0.8)
        .wait(pause);

    track = track
        .inverted_turn(C4, 0.8)
        .inverted_turn(E4, 0.8)
        .wait(pause);

    // ===== 7. ARPEGGIO / CASCADE =====
    println!("\n7. Arpeggio / Cascade - Spreading notes of a chord");

    let c_major_chord = vec![C3, E3, G3, C4, E4];

    // Fast cascade up
    track = track
        .cascade(&c_major_chord, 1.0, 0.05)
        .wait(pause);

    // Slow cascade up
    track = track
        .cascade(&c_major_chord, 1.5, 0.15)
        .wait(pause);

    // Strum down and up
    track = track
        .strum(&c_major_chord, 1.0, 0.03)
        .wait(0.5);

    let c_major_reversed: Vec<f32> = c_major_chord.iter().rev().copied().collect();
    track = track
        .strum(&c_major_reversed, 1.0, 0.03)
        .wait(pause);

    // Cascade down (reversed)
    let _ = track
        .cascade(&c_major_reversed, 1.0, 0.06);

    println!("\n▶ Playing ornaments demonstration...\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    println!("✅ Demo complete!");
    println!("\n📚 Ornaments demonstrated:");
    println!("   • Slide/Glissando - Smooth pitch bends");
    println!("   • Grace Notes - Quick ornamental notes");
    println!("   • Acciaccatura - Crushed grace notes");
    println!("   • Appoggiatura - Emphasized grace notes");
    println!("   • Trill - Rapid alternation");
    println!("   • Tremolo - Rapid repetition");
    println!("   • Mordent - Main-Lower-Main");
    println!("   • Inverted Mordent - Main-Upper-Main");
    println!("   • Turn - Upper-Main-Lower-Main");
    println!("   • Inverted Turn - Lower-Main-Upper-Main");
    println!("   • Cascade/Arpeggio - Spreading chord notes");
    println!("   • Strum - Guitar-like chord articulation\n");

    Ok(())
}
