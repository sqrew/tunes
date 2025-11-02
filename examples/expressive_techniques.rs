use tunes::prelude::*;

/// Demonstrate expressive performance techniques
fn main() -> anyhow::Result<()> {
    println!("\n🎸 Example: Expressive Techniques\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Octave doubling for thickness
    comp.instrument("thicc", &Instrument::saw_lead())
        .at(0.0)
        .octaves(&[C4, D4, E4, F4, G4], -1, 0.25);

    // Harmonization (adding fifths)
    comp.instrument("harmony", &Instrument::pluck())
        .at(1.5)
        .harmonize(&[C4, D4, E4, F4, G4], 7, 0.25);

    // Guitar strum effect
    comp.instrument("strum", &Instrument::pluck())
        .at(3.0)
        .strum(C4_MAJOR, 0.8, 0.03)
        .wait(0.2)
        .strum(F4_MAJOR, 0.8, 0.03)
        .wait(0.2)
        .strum(G4_MAJOR, 0.8, 0.03);

    // Grace notes
    comp.instrument("ornament", &Instrument::pluck())
        .at(5.5)
        .grace(C5, B4, 0.05, 0.4)
        .grace(D5, C5, 0.05, 0.4)
        .grace(E5, D5, 0.05, 0.4);

    // Cascade effect (waterfall)
    comp.instrument("cascade", &Instrument::warm_pad())
        .at(7.0)
        .cascade(&[C4, E4, G4, C5], 2.0, 0.15);

    // Pedal tone (bass sustain under melody)
    comp.instrument("pedal", &Instrument::sub_bass())
        .at(9.5)
        .pedal(C2, &[E4, G4, B4, C5, B4, G4], 0.3);

    println!("✓ Thickness: .octaves() and .harmonize()");
    println!("✓ Guitar: .strum()");
    println!("✓ Ornaments: .grace()");
    println!("✓ Layering: .cascade()");
    println!("✓ Bass: .pedal()\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
