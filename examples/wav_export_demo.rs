use tunes::prelude::*;

/// Demonstrate WAV file export functionality
///
/// This example creates a short composition and exports it to a WAV file,
/// showing both the direct mixer method and the AudioEngine convenience method.
fn main() -> anyhow::Result<()> {
    println!("\n🎵 WAV Export Demo\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // Create a simple melodic sequence
    println!("Creating composition...");

    comp.instrument("melody", &Instrument::synth_lead())
        .at(0.0)
        .filter(Filter::low_pass(1200.0, 0.6))
        .notes(&[C4, E4, G4, C5], 0.4)
        .notes(&[B4, G4, E4, C4], 0.4);

    comp.instrument("bass", &Instrument::sub_bass())
        .at(0.0)
        .notes(&[C2, C2, G2, G2], 0.8);

    comp.instrument("chords", &Instrument::warm_pad())
        .at(0.0)
        .filter(Filter::low_pass(800.0, 0.5))
        .note(&[C3, E3, G3], 1.6)
        .note(&[C3, E3, G3], 1.6);

    println!("  ✓ Created 3 tracks");

    // Export to WAV using the mixer
    println!("\n📁 Exporting to WAV...");
    let mut mixer = comp.into_mixer();

    // Export using the mixer's direct method
    mixer.export_wav("output.wav", 44100)?;

    println!();
    println!("You can now:");
    println!("  • Import output.wav into your DAW");
    println!("  • Share your compositions");
    println!("  • Use in video/game projects");
    println!("  • Archive your algorithmic music");

    Ok(())
}
