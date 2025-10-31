use tunes::composition::Composition;
use tunes::effects::{Delay, Reverb};
use tunes::engine::AudioEngine;
use tunes::instruments::Instrument;
use tunes::notes::*;
use tunes::rhythm::Tempo;
use tunes::theory::{chord, ChordPattern};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎹 Classical Compositional Techniques Demo\n");
    println!("Showcasing common classical accompaniment patterns and techniques.\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // ===== 1. ALBERTI BASS =====
    println!("1. Alberti Bass - The quintessential Classical accompaniment");
    println!("   (Mozart, Haydn, Beethoven)");

    comp.instrument("alberti", &Instrument::acoustic_piano())
        .volume(0.6)
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .at(0.0)
        .alberti_bass(&[C3, E3, G3], 0.125) // C major
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[B2, D3, G3], 0.125) // G major
        .alberti_bass(&[B2, D3, G3], 0.125);

    // Simple melody over Alberti bass
    comp.instrument("melody1", &Instrument::acoustic_piano())
        .volume(0.7)
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .at(0.0)
        .notes(&[G4, A4, B4, C5], 0.5)
        .notes(&[B4, A4, G4, F4], 0.5);

    // ===== 2. WALTZ BASS =====
    println!("\n2. Waltz Bass - Classic oom-pah-pah in 3/4 time");

    comp.instrument("waltz", &Instrument::acoustic_piano())
        .volume(0.65)
        .reverb(Reverb::new(0.5, 0.6, 0.4))
        .at(4.5)
        .waltz_bass(C2, &[C3, E3, G3], 0.5)
        .waltz_bass(F2, &[F3, A3, C4], 0.5)
        .waltz_bass(G2, &[G3, B3, D4], 0.5)
        .waltz_bass(C2, &[C3, E3, G3], 0.5);

    // Waltz melody
    comp.instrument("waltz_melody", &Instrument::acoustic_piano())
        .volume(0.7)
        .reverb(Reverb::new(0.5, 0.6, 0.4))
        .at(5.0)
        .notes(&[E4, G4, C5], 1.0)
        .notes(&[F4, A4, C5], 1.0)
        .notes(&[D4, G4, B4], 1.0)
        .notes(&[C4, E4, C5], 1.0);

    // ===== 3. STRIDE PIANO =====
    println!("\n3. Stride Piano - Jazz/Ragtime boom-chick");

    comp.instrument("stride", &Instrument::acoustic_piano())
        .volume(0.7)
        .reverb(Reverb::new(0.3, 0.4, 0.25))
        .at(10.5)
        .stride_bass(C2, &[C3, E3, G3], 0.25, 2);

    // Stride melody (syncopated)
    comp.instrument("stride_melody", &Instrument::acoustic_piano())
        .volume(0.75)
        .reverb(Reverb::new(0.3, 0.4, 0.25))
        .at(10.5)
        .notes(&[C4, E4, G4, C5], 0.125)
        .notes(&[B4, G4, E4, D4], 0.125)
        .notes(&[C4, E4, G4, C5], 0.125)
        .notes(&[C5, B4, A4, G4], 0.125);

    // ===== 4. BROKEN CHORD PATTERNS =====
    println!("\n4. Broken Chord Patterns - Various arpeggiation styles");

    // Pattern 0: Up and back
    comp.instrument("broken1", &Instrument::acoustic_piano())
        .volume(0.6)
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .at(14.5)
        .broken_chord(&[C4, E4, G4], 0, 0.125)
        .broken_chord(&[C4, E4, G4], 0, 0.125);

    // Pattern 1: Down and back
    comp.instrument("broken2", &Instrument::acoustic_piano())
        .volume(0.6)
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .at(15.5)
        .broken_chord(&[F4, A4, C5], 1, 0.125)
        .broken_chord(&[F4, A4, C5], 1, 0.125);

    // Pattern 2: Up twice
    comp.instrument("broken3", &Instrument::acoustic_piano())
        .volume(0.6)
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .at(16.5)
        .broken_chord(&[G4, B4, D5], 2, 0.1);

    // Pattern 3: Ascending pairs
    comp.instrument("broken4", &Instrument::acoustic_piano())
        .volume(0.6)
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .at(17.5)
        .broken_chord(&[C4, E4, G4], 3, 0.15);

    // ===== 5. WALKING BASS =====
    println!("\n5. Walking Bass - Baroque stepwise bass movement");

    comp.instrument("walking", &Instrument::upright_bass())
        .volume(0.7)
        .reverb(Reverb::new(0.3, 0.4, 0.2))
        .at(19.0)
        .walking_bass(&[C2, D2, E2, F2, G2, F2, E2, D2], 0.25)
        .walking_bass(&[C2, B1, A1, G1, F1, G1, A1, B1], 0.25);

    // Baroque melody over walking bass
    comp.instrument("baroque_melody", &Instrument::flute())
        .volume(0.65)
        .reverb(Reverb::new(0.5, 0.5, 0.35))
        .at(19.0)
        .notes(&[C4, D4, E4, F4], 0.5)
        .notes(&[G4, F4, E4, D4], 0.5)
        .notes(&[C4, B3, A3, G3], 0.5)
        .notes(&[F3, G3, A3, B3], 0.5);

    // ===== 6. TREMOLO STRINGS =====
    println!("\n6. Tremolo Strings - Orchestral sustained tremolo");

    comp.instrument("tremolo", &Instrument::strings())
        .volume(0.6)
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(23.0)
        .tremolo_strings(&[C3, E3, G3], 2.0, 0.03) // Fast tremolo on C major chord
        .tremolo_strings(&[F3, A3, C4], 2.0, 0.03); // Fast tremolo on F major chord

    // Dramatic melody over tremolo
    comp.instrument("tremolo_melody", &Instrument::brass())
        .volume(0.7)
        .reverb(Reverb::new(0.5, 0.5, 0.35))
        .at(23.5)
        .notes(&[C5, B4, A4, G4], 0.5)
        .notes(&[F4, G4, A4, C5], 0.5);

    // ===== 7. OSTINATO =====
    println!("\n7. Ostinato - Repeating rhythmic/melodic pattern");

    comp.instrument("ostinato", &Instrument::pluck())
        .volume(0.6)
        .delay(Delay::new(0.375, 0.25, 0.3))
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .at(27.5)
        .ostinato(&[C4, E4, G4, E4], 0.125, 8); // Repeat pattern 8 times

    // Counter-melody over ostinato
    comp.instrument("counter", &Instrument::synth_lead())
        .volume(0.5)
        .delay(Delay::new(0.375, 0.2, 0.4))
        .reverb(Reverb::new(0.5, 0.5, 0.35))
        .at(27.5)
        .notes(&[G4, A4, B4, C5], 1.0)
        .notes(&[C5, B4, A4, G4], 1.0)
        .notes(&[F4, G4, A4, B4], 1.0)
        .notes(&[C5, D5, E5, C5], 1.0);

    // ===== 8. PEDAL POINT =====
    println!("\n8. Pedal Point - Sustained bass with changing harmonies");

    let c_major = chord(C3, &ChordPattern::MAJOR);
    let f_major = chord(F3, &ChordPattern::MAJOR);
    let g_major = chord(G3, &ChordPattern::MAJOR);
    let a_minor = chord(A3, &ChordPattern::MINOR);
    let c_major2 = chord(C3, &ChordPattern::MAJOR);

    comp.instrument("pedal", &Instrument::warm_pad())
        .volume(0.65)
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(31.5)
        .pedal_point(C2, &[c_major, f_major, g_major, a_minor, c_major2], 1.5);

    // Soaring melody over pedal point
    comp.instrument("pedal_melody", &Instrument::strings())
        .volume(0.7)
        .reverb(Reverb::new(0.6, 0.6, 0.45))
        .at(32.0)
        .notes(&[E4, F4, G4, A4], 0.75)
        .notes(&[G4, F4, E4, D4], 0.75)
        .notes(&[E4, G4, C5, E5], 0.75)
        .notes(&[D5, C5, B4, A4], 0.75)
        .note(&[G4], 1.5);

    // ===== MUSICAL EXAMPLE: COMBINING TECHNIQUES =====
    println!("\n9. Grand Finale - Combining multiple classical techniques");

    // Alberti bass foundation
    comp.instrument("finale_alberti", &Instrument::acoustic_piano())
        .volume(0.55)
        .reverb(Reverb::new(0.5, 0.6, 0.4))
        .at(39.0)
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[F3, A3, C4], 0.125)
        .alberti_bass(&[F3, A3, C4], 0.125)
        .alberti_bass(&[G3, B3, D4], 0.125)
        .alberti_bass(&[G3, B3, D4], 0.125)
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[C3, E3, G3], 0.125);

    // Melody with ornaments
    comp.instrument("finale_melody", &Instrument::flute())
        .volume(0.7)
        .delay(Delay::new(0.375, 0.2, 0.3))
        .reverb(Reverb::new(0.6, 0.6, 0.45))
        .at(39.0)
        .notes(&[C5, D5, E5, G5], 0.5)
        .trill(G5, A5, 8, 0.0625)
        .notes(&[F5, E5, D5, C5], 0.5)
        .notes(&[B4, C5, D5, E5], 0.5)
        .notes(&[G5, F5, E5, D5], 0.5)
        .note(&[C5], 2.0);

    // String pad
    comp.instrument("finale_strings", &Instrument::strings())
        .volume(0.5)
        .reverb(Reverb::new(0.7, 0.7, 0.5))
        .at(39.0)
        .note(&[C4, E4, G4], 4.0)
        .note(&[F4, A4, C5], 2.0)
        .note(&[G4, B4, D5], 2.0)
        .note(&[C4, E4, G4], 4.0);

    println!("\n▶️  Playing classical techniques demo...");
    println!("    Duration: ~51 seconds\n");
    println!("    💎 Techniques Demonstrated:");
    println!("       1. Alberti Bass (Mozart/Haydn style)");
    println!("       2. Waltz Bass (oom-pah-pah)");
    println!("       3. Stride Piano (Jazz/Ragtime)");
    println!("       4. Broken Chord Patterns (4 variations)");
    println!("       5. Walking Bass (Baroque)");
    println!("       6. Tremolo Strings (Orchestral)");
    println!("       7. Ostinato (Repeating patterns)");
    println!("       8. Pedal Point (Harmonic tension)");
    println!("       9. Combined Techniques (Grand finale)");

    let mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;

    println!("\n✅ Demo complete!");
    println!("   These patterns save enormous amounts of manual note entry!");

    Ok(())
}
