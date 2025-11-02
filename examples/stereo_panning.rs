use tunes::prelude::*;

/// Demonstrate stereo panning
fn main() -> anyhow::Result<()> {
    println!("\n🎚️  Example: Stereo Panning\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Bass centered
    comp.instrument("bass", &Instrument::electric_piano())
        .pan(0.0) // Center
        .at(0.0)
        .notes(&[C2, C3, C4, C2, C3, C4, C2, C4], 0.2);

    // Chords panned left
    comp.track("chords_left")
        .pan(-0.7) // Left
        .at(0.0)
        .chords(&[C4_MAJOR, F4_MAJOR, G4_MAJOR, C4_MAJOR], 0.8);

    // Lead panned right
    comp.track("lead_right")
        .pan(0.7) // Right
        .at(0.0)
        .notes(&[E5, G5, B5, E6], 0.4);

    println!("✓ Pan values: -1.0 (left), 0.0 (center), 1.0 (right)");
    println!("✓ Creates stereo width and space\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
