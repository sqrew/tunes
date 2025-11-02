use tunes::prelude::*;

/// Demonstrate pattern_start() and repeat() for looping sections
fn main() -> anyhow::Result<()> {
    println!("\n🔁 Example: Pattern Repeat\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Simple pattern repeated
    comp.instrument("simple_repeat", &Instrument::pluck())
        .at(0.0)
        .pattern_start()
        .notes(&[C4, E4, G4], 0.2)
        .repeat(3);  // Play 3 more times (4 total)

    // Chord progression repeated
    comp.instrument("chord_loop", &Instrument::warm_pad())
        .at(2.5)
        .pattern_start()
        .chords(&[C4_MAJOR, F4_MAJOR, G4_MAJOR, C4_MAJOR], 0.5)
        .repeat(2);  // Play 2 more times

    // Complex pattern with nested repeats
    comp.instrument("nested", &Instrument::pluck())
        .at(7.0)
        .pattern_start()
        .note(&[C4], 0.2)
        .note(&[E4], 0.2)
        .repeat(1)  // C4-E4 twice
        .pattern_start()
        .note(&[G4], 0.4)
        .repeat(3);  // Then G4 four times

    // Build up a pattern then repeat it
    comp.instrument("buildup", &Instrument::arp_lead())
        .at(9.0)
        .pattern_start()
        .note(&[C4], 0.2)
        .note(&[D4], 0.2)
        .note(&[E4], 0.2)
        .note(&[F4], 0.2)
        .repeat(2);

    println!("✓ .pattern_start() marks the beginning of a repeatable section");
    println!("✓ .repeat(n) repeats the pattern n more times");
    println!("✓ Patterns can be nested");
    println!("✓ All note properties (waveform, envelope, etc) are preserved\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
