use tunes::chords::*;
use tunes::composition::Composition;
use tunes::engine::AudioEngine;
use tunes::instruments::Instrument;
use tunes::notes::*;
use tunes::rhythm::Tempo;
use std::error::Error;

/// Comprehensive showcase of all ornamental and expressive techniques
fn main() -> Result<(), Box<dyn Error>> {
    println!("🎵 Ornaments and Expression Showcase\n");
    println!("Demonstrating all classical ornaments and expressive techniques.\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // ===== 1. SLIDE / GLISSANDO =====
    println!("1. Slide / Glissando - Smooth pitch transitions");

    comp.instrument("slide_up", &Instrument::synth_lead())
        .at(0.0)
        .slide(C4, C5, 0.5);

    comp.instrument("slide_down", &Instrument::synth_lead())
        .at(0.6)
        .slide(C5, C4, 0.5);

    // ===== 2. GRACE NOTES =====
    println!("\n2. Grace Notes - Quick ornamental notes before main notes");

    // Simple grace notes
    comp.instrument("grace_basic", &Instrument::acoustic_piano())
        .at(2.0)
        .grace(C4, B3, 0.05, 0.45)
        .grace(E4, D4, 0.05, 0.45)
        .grace(G4, FS4, 0.05, 0.45)
        .grace(C5, B4, 0.05, 0.45);

    // Acciaccatura (crushed, very short)
    comp.instrument("acciaccatura", &Instrument::harpsichord())
        .at(4.5)
        .grace(E4, DS4, 0.02, 0.48)
        .grace(G4, FS4, 0.02, 0.48)
        .grace(C5, B4, 0.02, 0.48);

    // Appoggiatura (leaning, emphasized)
    comp.instrument("appoggiatura", &Instrument::flute())
        .at(6.0)
        .grace(C4, D4, 0.15, 0.35)
        .grace(E4, F4, 0.15, 0.35)
        .grace(G4, A4, 0.15, 0.35);

    // ===== 3. TRILL =====
    println!("\n3. Trill - Rapid alternation between two notes");

    // Slow trill
    comp.instrument("trill_slow", &Instrument::acoustic_piano())
        .at(8.0)
        .trill(C4, D4, 8, 0.1);

    // Fast trill (classic)
    comp.instrument("trill_fast", &Instrument::acoustic_piano())
        .at(9.0)
        .trill(E4, F4, 16, 0.05);

    // Major third trill
    comp.instrument("trill_third", &Instrument::synth_lead())
        .at(10.2)
        .trill(G4, B4, 12, 0.08);

    // ===== 4. TREMOLO =====
    println!("\n4. Tremolo - Rapid repetition of same note");

    // Single note tremolo
    comp.instrument("tremolo_basic", &Instrument::strings())
        .at(12.0)
        .tremolo(C4, 16, 0.06);

    // Tremolo chord (multiple tracks)
    comp.instrument("tremolo_chord_1", &Instrument::organ())
        .at(13.2)
        .tremolo(C4, 12, 0.08);

    comp.instrument("tremolo_chord_2", &Instrument::organ())
        .at(13.2)
        .tremolo(E4, 12, 0.08);

    comp.instrument("tremolo_chord_3", &Instrument::organ())
        .at(13.2)
        .tremolo(G4, 12, 0.08);

    // ===== 5. MORDENT & TURN =====
    println!("\n5. Mordent & Turn - Classical ornaments");

    // Mordent (main-upper-main)
    comp.instrument("mordent", &Instrument::harpsichord())
        .at(15.0)
        .note(&[C4], 0.3)
        .mordent(E4, 0.15)
        .note(&[G4], 0.3)
        .mordent(C5, 0.15);

    // Inverted mordent (main-lower-main)
    comp.instrument("inv_mordent", &Instrument::harpsichord())
        .at(16.5)
        .note(&[C4], 0.3)
        .inverted_mordent(E4, 0.15)
        .note(&[G4], 0.3)
        .inverted_mordent(C5, 0.15);

    // Turn (upper-main-lower-main)
    comp.instrument("turn", &Instrument::harpsichord())
        .at(18.0)
        .note(&[C4], 0.3)
        .turn(E4, 0.2)
        .note(&[G4], 0.3)
        .turn(C5, 0.2);

    // Inverted turn (lower-main-upper-main)
    comp.instrument("inv_turn", &Instrument::harpsichord())
        .at(19.5)
        .note(&[C4], 0.3)
        .inverted_turn(E4, 0.2)
        .note(&[G4], 0.3)
        .inverted_turn(C5, 0.2);

    // ===== 6. CASCADE & STRUM =====
    println!("\n6. Cascade & Strum - Layered and staggered notes");

    // Harp-like cascade
    comp.instrument("cascade_harp", &Instrument::pluck())
        .at(21.0)
        .cascade(&[C3, E3, G3, C4, E4, G4, C5], 0.4, 0.04);

    // Slow atmospheric cascade
    comp.instrument("cascade_slow", &Instrument::warm_pad())
        .at(22.5)
        .cascade(&[E3, G3, B3, E4, G4], 0.8, 0.15);

    // Guitar strum
    comp.instrument("strum_guitar", &Instrument::pluck())
        .at(24.5)
        .strum(C4_MAJOR, 0.5, 0.03)
        .wait(0.1)
        .strum(G4_MAJOR, 0.5, 0.03)
        .wait(0.1)
        .strum(A4_MINOR, 0.5, 0.03)
        .wait(0.1)
        .strum(F4_MAJOR, 0.5, 0.03);

    // Reverse cascade (descending)
    comp.instrument("cascade_down", &Instrument::pluck())
        .at(26.5)
        .cascade(&[C5, B4, A4, G4, F4, E4, D4, C4], 0.25, 0.06);

    // ===== 7. VIBRATO & EXPRESSION =====
    println!("\n7. Vibrato & Expression - Dynamic control");

    // No vibrato (plain)
    comp.instrument("no_vibrato", &Instrument::synth_lead())
        .at(28.0)
        .note(&[A4], 1.5);

    // With vibrato
    comp.instrument("with_vibrato", &Instrument::synth_lead())
        .at(30.0)
        .vibrato(5.5, 0.4)
        .note(&[A4], 1.5);

    // Fade out example
    comp.instrument("fade_demo", &Instrument::warm_pad())
        .at(32.0)
        .note(&[C4, E4, G4], 2.0)
        .fade_to(0.0, 2.0)
        .wait(2.0);

    // ===== 8. MUSICAL EXAMPLES - COMBINING TECHNIQUES =====
    println!("\n8. Musical Examples - Combining multiple ornaments");

    // Baroque phrase with mordents and turns
    comp.instrument("baroque", &Instrument::harpsichord())
        .at(36.0)
        .note(&[C4], 0.25)
        .mordent(D4, 0.15)
        .note(&[E4], 0.25)
        .turn(F4, 0.2)
        .note(&[G4], 0.5)
        .trill(G4, A4, 8, 0.06)
        .note(&[C5], 1.0);

    // Expressive lead with multiple techniques
    comp.instrument("expressive_lead", &Instrument::synth_lead())
        .at(38.5)
        .vibrato(6.0, 0.3)
        .grace(E4, D4, 0.05, 0.45)
        .slide(E4, G4, 0.3)
        .note(&[G4], 0.4)
        .trill(G4, A4, 6, 0.06)
        .slide(G4, C5, 0.3)
        .note(&[C5], 1.0);

    // Classical piano passage
    comp.instrument("piano_passage", &Instrument::acoustic_piano())
        .at(41.5)
        .cascade(&[C4, E4, G4, C5], 0.3, 0.06)
        .mordent(C5, 0.15)
        .notes(&[B4, A4, G4], 0.2)
        .turn(F4, 0.2)
        .note(&[E4], 0.4)
        .trill(E4, F4, 8, 0.05)
        .note(&[C4], 1.0);

    // Strings with tremolo and vibrato
    comp.instrument("strings_expressive", &Instrument::strings())
        .at(44.5)
        .vibrato(5.0, 0.35)
        .tremolo_strings(&[C3, E3, G3], 2.0, 0.03)
        .wait(0.5)
        .note(&[G3, B3, D4], 2.0);

    // Jazz-style phrase with grace notes and slides
    comp.instrument("jazz_phrase", &Instrument::electric_piano())
        .at(49.0)
        .grace(C4, B3, 0.08, 0.42)
        .slide(C4, DS4, 0.15)
        .grace(E4, DS4, 0.08, 0.42)
        .note(&[E4], 0.3)
        .grace(G4, FS4, 0.08, 0.42)
        .slide(G4, AS4, 0.15)
        .note(&[B4], 0.5);

    // Final cascade chord
    comp.instrument("finale", &Instrument::warm_pad())
        .at(52.0)
        .cascade(&[C3, E3, G3, C4, E4, G4, C5], 0.5, 0.08)
        .wait(0.5)
        .note(&[C3, E3, G3, C4, E4, G4, C5], 3.0);

    println!("\n▶️  Playing comprehensive ornaments showcase...");
    println!("    Duration: ~56 seconds\n");
    println!("    💎 Techniques Demonstrated:");
    println!("       • Slide/Glissando - Smooth pitch transitions");
    println!("       • Grace Notes - Acciaccatura, Appoggiatura");
    println!("       • Trill - Rapid note alternation");
    println!("       • Tremolo - Rapid note repetition");
    println!("       • Mordent & Turn - Classical ornaments");
    println!("       • Inverted Mordent & Turn - Variations");
    println!("       • Cascade & Strum - Layered effects");
    println!("       • Vibrato & Expression - Dynamic control");
    println!("       • Musical combinations - Real-world usage");

    let mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;

    println!("\n✅ Showcase complete!");
    println!("   All ornamental techniques in one comprehensive example!");

    Ok(())
}
