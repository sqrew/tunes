use tunes::chords::*;
use tunes::composition::Composition;
use tunes::engine::AudioEngine;
use tunes::instruments::Instrument;
use tunes::notes::*;
use tunes::rhythm::Tempo;

/// Demonstrate arpeggio techniques
fn main() -> Result<(), anyhow::Error> {
    println!("\n🎸 Example: Arpeggios\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Compare chord vs arpeggio
    comp.instrument("chord", &Instrument::electric_piano())
        .at(0.0)
        .note(C4_MAJOR, 1.0);

    comp.instrument("arpeggio", &Instrument::electric_piano())
        .at(1.2)
        .arpeggiate(C4_MAJOR, 0.2);

    // Fast arpeggio
    comp.instrument("fast_arp", &Instrument::arp_lead())
        .at(2.0)
        .arpeggiate(G4_MAJOR, 0.1);

    // Slow arpeggio
    comp.instrument("slow_arp", &Instrument::harpsichord())
        .at(2.5)
        .arpeggiate(D4_MINOR, 0.3);

    // Arpeggio sequence (chord progression)
    comp.instrument("progression", &Instrument::harpsichord())
        .at(3.5)
        .arpeggiate(C4_MAJOR, 0.15)
        .arpeggiate(A4_MINOR, 0.15)
        .arpeggiate(F4_MAJOR, 0.15)
        .arpeggiate(G4_MAJOR, 0.15);

    // Reverse arpeggio (descending)
    comp.instrument("reverse_arp", &Instrument::electric_piano())
        .at(5.5)
        .arpeggiate_reverse(C4_MAJOR, 0.15);

    // Compare ascending vs descending
    comp.instrument("up_then_down", &Instrument::pluck())
        .at(6.5)
        .arpeggiate(E4_MINOR, 0.12)
        .arpeggiate_reverse(E4_MINOR, 0.12);

    // Extended arpeggio pattern
    comp.instrument("extended", &Instrument::arp_lead())
        .at(8.0)
        .pattern_start()
        .arpeggiate(C4_MAJOR, 0.1)
        .arpeggiate_reverse(C4_MAJOR, 0.1)
        .repeat(1);

    // Broken chord pattern
    comp.instrument("broken_chord", &Instrument::electric_piano())
        .at(10.0)
        .notes(&[C4, E4, G4, E4], 0.2); // Classic pattern

    // Arpeggio with different instruments
    comp.instrument("harp_arp", &Instrument::harpsichord())
        .pan(-0.5)
        .at(11.5)
        .arpeggiate(D4_MAJOR, 0.12);

    comp.instrument("guitar_arp", &Instrument::harpsichord())
        .pan(0.0)
        .at(11.5)
        .arpeggiate(D4_MAJOR, 0.12);

    comp.instrument("synth_arp", &Instrument::arp_lead())
        .pan(0.5)
        .at(11.5)
        .arpeggiate(D4_MAJOR, 0.12);

    // Power chord arpeggios
    comp.instrument("power_arp", &Instrument::saw_lead())
        .at(13.0)
        .arpeggiate(C4_POWER, 0.1)
        .arpeggiate(A3_POWER, 0.1)
        .arpeggiate(F3_POWER, 0.1)
        .arpeggiate(G3_POWER, 0.1);

    // Alberti bass pattern (classical)
    comp.instrument("alberti", &Instrument::electric_piano())
        .at(14.5)
        .pattern_start()
        .notes(&[C3, G3, E3, G3], 0.15) // Low-High-Middle-High
        .repeat(3);

    // Arpeggio with octave spanning
    comp.instrument("octave_arp", &Instrument::electric_piano())
        .at(17.0)
        .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.12);

    // Jazz arpeggio (7th chords)
    let c7 = &[C4, E4, G4, AS4]; // C dominant 7th
    comp.instrument("jazz_arp", &Instrument::electric_piano())
        .at(18.5)
        .arpeggiate(c7, 0.15);

    println!("✓ .arpeggiate(chord, duration_per_note):");
    println!("  - Plays chord notes sequentially (ascending)");
    println!("  - Creates flowing, melodic feel");
    println!("\n✓ .arpeggiate_reverse(chord, duration_per_note):");
    println!("  - Plays chord notes descending");
    println!("  - Useful for variation");
    println!("\n✓ Common uses:");
    println!("  - Guitar fingerpicking");
    println!("  - Piano accompaniment");
    println!("  - Harp glissandos");
    println!("  - Synth sequences");
    println!("  - Alberti bass patterns");
    println!("\n✓ Arpeggios break chords into motion and time\\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
