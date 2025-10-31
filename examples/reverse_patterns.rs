use tunes::composition::Composition;
use tunes::engine::AudioEngine;
use tunes::instruments::Instrument;
use tunes::notes::*;
use tunes::rhythm::Tempo;
use tunes::scales::C4_MAJOR_SCALE;

/// Demonstrate pattern reversal
fn main() -> Result<(), anyhow::Error> {
    println!("\n⏮️  Example: Reverse Patterns\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Simple melody and its reverse
    comp.instrument("original", &Instrument::electric_piano())
        .at(0.0)
        .pattern_start()
        .notes(&[C4, D4, E4, F4, G4], 0.25)
        .repeat(0);

    comp.instrument("reversed", &Instrument::electric_piano())
        .at(1.5)
        .pattern_start()
        .notes(&[C4, D4, E4, F4, G4], 0.25)
        .reverse()
        .repeat(0);

    // Scale run forward and backward
    comp.instrument("scale_forward", &Instrument::pluck())
        .at(3.0)
        .pattern_start()
        .scale(&C4_MAJOR_SCALE, 0.15)
        .repeat(0);

    comp.instrument("scale_reversed", &Instrument::pluck())
        .at(4.5)
        .pattern_start()
        .scale(&C4_MAJOR_SCALE, 0.15)
        .reverse()
        .repeat(0);

    // Arpeggio and reverse
    comp.instrument("arp_forward", &Instrument::harpsichord())
        .at(6.0)
        .pattern_start()
        .notes(&[C4, E4, G4, C5, E5, G5], 0.15)
        .repeat(0);

    comp.instrument("arp_backward", &Instrument::harpsichord())
        .at(7.2)
        .pattern_start()
        .notes(&[C4, E4, G4, C5, E5, G5], 0.15)
        .reverse()
        .repeat(0);

    // Palindrome: forward + reverse
    comp.instrument("palindrome", &Instrument::pluck())
        .at(8.5)
        .pattern_start()
        .notes(&[C4, D4, E4, F4], 0.2)
        .repeat(0);

    comp.instrument("palindrome_2", &Instrument::pluck())
        .at(9.5)
        .pattern_start()
        .notes(&[C4, D4, E4, F4], 0.2)
        .reverse()
        .repeat(0);

    // Complex rhythm reversed
    comp.instrument("rhythm_original", &Instrument::pluck())
        .at(10.5)
        .pattern_start()
        .note(&[C4], 0.3)
        .note(&[E4], 0.2)
        .note(&[G4], 0.1)
        .note(&[C5], 0.4)
        .repeat(0);

    comp.instrument("rhythm_reversed", &Instrument::pluck())
        .at(11.5)
        .pattern_start()
        .note(&[C4], 0.3)
        .note(&[E4], 0.2)
        .note(&[G4], 0.1)
        .note(&[C5], 0.4)
        .reverse()
        .repeat(0);

    // Call and response (original and reverse)
    comp.instrument("call", &Instrument::bright_lead())
        .at(12.5)
        .pattern_start()
        .notes(&[E4, F4, G4, A4, G4, F4], 0.15)
        .repeat(0);

    comp.instrument("response", &Instrument::square_lead())
        .at(13.5)
        .pattern_start()
        .notes(&[E4, F4, G4, A4, G4, F4], 0.15)
        .reverse()
        .repeat(0);

    // Nested reverse with repeat
    comp.instrument("nested", &Instrument::arp_lead())
        .at(14.5)
        .pattern_start()
        .notes(&[C4, E4, G4], 0.15)
        .reverse()
        .repeat(2);

    // Reverse with different durations
    comp.instrument("varied_durations", &Instrument::electric_piano())
        .at(16.5)
        .pattern_start()
        .note(&[C4], 0.5)
        .note(&[D4], 0.25)
        .note(&[E4], 0.125)
        .note(&[F4], 0.25)
        .repeat(0);

    comp.instrument("varied_reversed", &Instrument::electric_piano())
        .at(18.0)
        .pattern_start()
        .note(&[C4], 0.5)
        .note(&[D4], 0.25)
        .note(&[E4], 0.125)
        .note(&[F4], 0.25)
        .reverse()
        .repeat(0);

    println!("✓ .reverse():");
    println!("  - Reverses the note order in the current pattern");
    println!("  - Maintains timing structure (durations preserved)");
    println!("  - Notes play in opposite sequence");
    println!("\n✓ Use with .pattern_start():");
    println!("  - Call .pattern_start() before building pattern");
    println!("  - Add notes with any methods");
    println!("  - Call .reverse() to flip the sequence");
    println!("\n✓ Musical applications:");
    println!("  - Retrograde (classical technique)");
    println!("  - Call and response");
    println!("  - Palindromic structures");
    println!("  - Creating variation");
    println!("\n✓ Works with scales, arpeggios, and custom patterns\\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
