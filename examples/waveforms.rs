use tunes::prelude::*;

/// Demonstrate different waveform types
fn main() -> anyhow::Result<()> {
    println!("\n🌊 Example: Waveforms\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Sine wave - pure, fundamental tone
    comp.track("sine")
        .waveform(Waveform::Sine)
        .at(0.0)
        .note(&[A3], 0.5);

    // Square wave - hollow, retro video game sound
    comp.track("square")
        .waveform(Waveform::Square)
        .at(0.6)
        .note(&[A3], 0.5);

    // Sawtooth - bright, rich in harmonics
    comp.track("sawtooth")
        .waveform(Waveform::Sawtooth)
        .at(1.2)
        .note(&[A3], 0.5);

    // Triangle - warm, mellow
    comp.track("triangle")
        .waveform(Waveform::Triangle)
        .at(1.8)
        .note(&[A3], 0.5);

    // Compare them as a chord
    comp.track("sine_chord")
        .waveform(Waveform::Sine)
        .pan(-0.7)
        .at(2.5)
        .note(&[C4, E4, G4], 1.0);

    comp.track("square_chord")
        .waveform(Waveform::Square)
        .pan(-0.2)
        .at(2.5)
        .note(&[C4, E4, G4], 1.0);

    comp.track("sawtooth_chord")
        .waveform(Waveform::Sawtooth)
        .pan(0.2)
        .at(2.5)
        .note(&[C4, E4, G4], 1.0);

    comp.track("triangle_chord")
        .waveform(Waveform::Triangle)
        .pan(0.7)
        .at(2.5)
        .note(&[C4, E4, G4], 1.0);

    println!("✓ Waveform types available:");
    println!("  - Sine: Pure, smooth, fundamental");
    println!("  - Square: Hollow, retro, video game-like");
    println!("  - Sawtooth: Bright, rich, lots of harmonics");
    println!("  - Triangle: Warm, mellow, soft");
    println!("\n✓ Use .waveform() to set the oscillator type");
    println!("✓ Different waveforms have different harmonic content\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
