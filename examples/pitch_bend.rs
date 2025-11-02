use tunes::prelude::*;

/// Demonstrate pitch bend
fn main() -> anyhow::Result<()> {
    println!("\n🎸 Example: Pitch Bend\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Normal notes (no bend)
    comp.track("no_bend")
        .pan(-0.5)
        .at(0.0)
        .notes(&[C4, E4, G4], 0.5);

    // Bend up 2 semitones
    comp.track("bend_up")
        .pan(0.0)
        .bend(2.0)
        .at(2.0)
        .notes(&[C4, E4, G4], 0.5);

    // Bend down 3 semitones
    comp.track("bend_down")
        .pan(0.5)
        .bend(-3.0)
        .at(4.0)
        .notes(&[C4, E4, G4], 0.5);

    // Dramatic octave bend
    comp.track("bend_octave")
        .bend(12.0)  // Full octave up!
        .at(6.0)
        .note(&[C3], 1.0);

    println!("✓ Pitch bend in semitones");
    println!("✓ Positive values: bend up");
    println!("✓ Negative values: bend down");
    println!("✓ Range: -24 to +24 semitones (2 octaves)");
    println!("✓ Bend is linear over note duration\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
