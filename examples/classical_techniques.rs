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
    let mut time = 0.0;
    let pause = 0.5;

    // Use single track for better performance
    let mut track = comp.instrument("demo", &Instrument::acoustic_piano()).reverb(Reverb::new(0.5, 0.6, 0.4));

    // ===== 1. ALBERTI BASS =====
    println!("1. Alberti Bass - The quintessential Classical accompaniment (Mozart, Haydn)");

    track = track
        .at(time)
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[B2, D3, G3], 0.125)
        .alberti_bass(&[B2, D3, G3], 0.125);
    time += 2.0 + pause;

    // ===== 2. WALTZ BASS =====
    println!("\n2. Waltz Bass - Classic oom-pah-pah in 3/4 time");

    track = track
        .at(time)
        .waltz_bass(C2, &[C3, E3, G3], 0.5)
        .waltz_bass(F2, &[F3, A3, C4], 0.5)
        .waltz_bass(G2, &[G3, B3, D4], 0.5)
        .waltz_bass(C2, &[C3, E3, G3], 0.5);
    time += 6.0 + pause;

    // ===== 3. STRIDE PIANO =====
    println!("\n3. Stride Piano - Jazz piano technique with alternating bass");

    track = track
        .at(time)
        .stride_bass(C2, &[C3, E3, G3], 0.25)
        .stride_bass(C2, &[C3, E3, G3], 0.25);
    time += 1.0 + pause;

    // ===== 4. BROKEN CHORD ARPEGGIOS =====
    println!("\n4. Broken Chord Arpeggios - Flowing accompaniment");

    track = track
        .at(time)
        .broken_chord(&[C3, E3, G3, C4], 0.15)
        .broken_chord(&[F3, A3, C4, F4], 0.15)
        .broken_chord(&[G3, B3, D4, G4], 0.15)
        .broken_chord(&[C3, E3, G3, C4], 0.15);
    time += 2.4 + pause;

    // ===== 5. WALKING BASS =====
    println!("\n5. Walking Bass - Smooth chromatic movement");

    track = track
        .at(time)
        .walking_bass(&[C2, D2, E2, F2, G2, A2, B2, C3], 0.25);
    time += 2.0 + pause;

    // ===== 6. TREMOLO =====
    println!("\n6. Tremolo - Rapid repetition of a single note");

    track = track
        .at(time)
        .tremolo(C4, 0.05, 16)
        .wait(pause)
        .tremolo(E4, 0.05, 16);
    time += 1.6 + pause;

    // ===== 7. OSTINATO =====
    println!("\n7. Ostinato - Repeating rhythmic/melodic pattern");

    track = track
        .at(time)
        .pattern_start()
        .notes(&[C3, E3, G3, E3], 0.2)
        .repeat(3);
    time += 3.2 + pause;

    // ===== 8. FINALE =====
    println!("\n8. Finale - Alberti bass with melody");

    let _ = track
        .at(time)
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[C3, E3, G3], 0.125)
        .alberti_bass(&[F3, A3, C4], 0.125)
        .alberti_bass(&[F3, A3, C4], 0.125);

    println!("\n▶ Playing classical techniques demonstration...\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    println!("✅ Demo complete!");
    println!("\n📚 Techniques demonstrated:");
    println!("   1. Alberti Bass - Low-High-Middle-High pattern");
    println!("   2. Waltz Bass - Oom-pah-pah in 3/4 time");
    println!("   3. Stride Piano - Alternating bass and chords");
    println!("   4. Broken Chords - Arpeggiated harmonies");
    println!("   5. Walking Bass - Stepwise motion");
    println!("   6. Tremolo - Rapid note repetition");
    println!("   7. Ostinato - Repeating pattern");
    println!("   8. Finale - Combined techniques\n");

    Ok(())
}
