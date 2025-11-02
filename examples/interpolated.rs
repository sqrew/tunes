use tunes::prelude::*;

/// Demonstrate smooth pitch interpolation (glissando/portamento)
fn main() -> anyhow::Result<()> {
    println!("\n〰️  Example: Interpolated Pitch Slides\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Simple upward glissando
    comp.instrument("slide_up", &Instrument::synth_lead())
        .at(0.0)
        .interpolated(C3, C5, 20, 0.05);  // 20 steps, 0.05s each = 1 second total

    // Downward glissando
    comp.instrument("slide_down", &Instrument::synth_lead())
        .at(1.2)
        .interpolated(C5, C3, 20, 0.05);

    // Fast slide (swoosh effect)
    comp.instrument("fast_slide", &Instrument::synth_lead())
        .at(2.4)
        .interpolated(A2, A5, 30, 0.02);  // 30 steps in 0.6s

    // Slow smooth glide
    comp.instrument("slow_glide", &Instrument::warm_pad())
        .at(3.2)
        .interpolated(E3, E4, 24, 0.08);  // Slow and smooth

    // Trombone glissando
    comp.instrument("trombone", &Instrument::square_lead())
        .at(5.2)
        .interpolated(F3, B4, 28, 0.04);

    // Theremin-style wobbly slides
    comp.instrument("theremin_1", &Instrument::synth_lead())
        .at(6.5)
        .interpolated(C4, G4, 12, 0.06);

    comp.instrument("theremin_2", &Instrument::synth_lead())
        .at(7.3)
        .interpolated(G4, E4, 8, 0.05);

    comp.instrument("theremin_3", &Instrument::synth_lead())
        .at(7.8)
        .interpolated(E4, C5, 16, 0.04);

    // Siren effect (up and down)
    comp.instrument("siren_up", &Instrument::synth_lead())
        .at(9.0)
        .pattern_start()
        .interpolated(A3, A4, 16, 0.03)
        .repeat(0);

    comp.instrument("siren_down", &Instrument::synth_lead())
        .at(9.5)
        .pattern_start()
        .interpolated(A4, A3, 16, 0.03)
        .repeat(0);

    // Pitch bend effect on melody
    comp.instrument("bend_melody", &Instrument::arp_lead())
        .at(11.0)
        .note(&[C4], 0.3)
        .interpolated(C4, D4, 6, 0.05)
        .note(&[E4], 0.3)
        .interpolated(E4, F4, 6, 0.05)
        .note(&[G4], 0.6);

    // Sci-fi laser effect
    comp.instrument("laser", &Instrument::synth_lead())
        .at(13.0)
        .interpolated(C6, C2, 40, 0.015);  // High to low, very fast

    // Smooth chromatic transition
    comp.instrument("chromatic", &Instrument::electric_piano())
        .at(14.0)
        .interpolated(C4, C5, 12, 0.08);  // One octave = 12 semitones

    println!("✓ .interpolated(start_note, end_note, steps, duration_per_step):");
    println!("  - Creates smooth pitch transitions (glissando)");
    println!("  - Linearly interpolates frequency between notes");
    println!("  - More steps = smoother glide");
    println!("\n✓ Use cases:");
    println!("  - Trombone/synth slides");
    println!("  - Theremin effects");
    println!("  - Siren sounds");
    println!("  - Sci-fi effects");
    println!("  - Expressive pitch bends");
    println!("\n✓ Tip: Adjust steps and duration for different feels:");
    println!("  - Fast + many steps = smooth swoosh");
    println!("  - Slow + many steps = liquid glide");
    println!("  - Fast + few steps = chromatic run\\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
