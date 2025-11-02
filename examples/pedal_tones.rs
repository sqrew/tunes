use tunes::prelude::*;

/// Demonstrate pedal tones (sustained bass note under changing melody)
fn main() -> anyhow::Result<()> {
    println!("\n🎹 Example: Pedal Tones\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Simple pedal: C bass, melody moving above
    comp.instrument("simple_pedal", &Instrument::organ())
        .at(0.0)
        .pedal(C3, &[C4, D4, E4, F4, G4, F4, E4, D4], 0.3);

    // Pedal with chord tones
    comp.instrument("chord_pedal", &Instrument::deep_bass())
        .at(2.5)
        .pedal(G2, &[G4, B4, D5, G5, D5, B4], 0.4);

    // Bagpipe drone effect
    comp.instrument("drone", &Instrument::organ())
        .at(5.0)
        .pedal(A2, &[A4, B4, CS5, D5, E5, D5, CS5, B4, A4], 0.35);

    // Inverted pedal (high sustained note, low melody)
    comp.instrument("inverted_pedal", &Instrument::warm_pad())
        .at(8.5)
        .pedal(G5, &[C3, D3, E3, F3, G3, F3, E3, D3], 0.35);

    // Double pedal (bass + treble sustained)
    comp.instrument("bass_pedal", &Instrument::deep_bass())
        .at(12.0)
        .pedal(C2, &[C4, E4, G4, C5], 0.5);

    comp.instrument("treble_pedal", &Instrument::warm_pad())
        .at(12.0)
        .pedal(G5, &[C4, E4, G4, C5], 0.5);

    // Organ pedal point
    comp.instrument("organ_pedal", &Instrument::organ())
        .at(14.5)
        .pedal(D3, &[D4, E4, FS4, G4, A4, G4, FS4, E4], 0.25);

    // Pedal with faster melody
    comp.instrument("fast_melody_pedal", &Instrument::warm_pad())
        .at(17.0)
        .pedal(C3, &[E4, F4, G4, A4, G4, F4, E4, D4, C4, D4, E4, F4], 0.2);

    // Pedal creating tension (dissonance)
    comp.instrument("tension_pedal", &Instrument::square_lead())
        .at(19.5)
        .pedal(C3, &[CS4, D4, DS4, E4, F4, FS4], 0.4);

    // Pedal resolving to tonic
    comp.instrument("resolving_pedal", &Instrument::organ())
        .at(22.0)
        .pedal(G2, &[D4, E4, F4, G4, A4, B4], 0.3)
        .at(22.0)
        .note(&[C4], 1.0); // Resolution

    println!("✓ .pedal(pedal_note, melody_notes, duration_per_note):");
    println!("  - Sustains bass note while melody plays above");
    println!("  - Creates harmonic foundation");
    println!("  - Common in organ music, bagpipes, drones");
    println!("\n✓ Types of pedals:");
    println!("  - Tonic pedal: Root note sustained");
    println!("  - Dominant pedal: 5th scale degree sustained");
    println!("  - Inverted pedal: High note sustained");
    println!("  - Double pedal: Both bass and treble sustained");
    println!("\n✓ Musical effects:");
    println!("  - Creates stability/grounding");
    println!("  - Can build tension when melody uses non-chord tones");
    println!("  - Common in classical, organ, and drone music");
    println!("\n✓ The pedal note plays for the entire melody duration\\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
