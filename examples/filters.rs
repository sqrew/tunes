use tunes::prelude::*;

/// Demonstrate filter types and parameters
fn main() -> anyhow::Result<()> {
    println!("\n🎛️  Example: Filters\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Compare unfiltered vs filtered sawtooth wave
    comp.track("unfiltered")
        .waveform(Waveform::Sawtooth)
        .at(0.0)
        .notes(&[C3, E3, G3, C4], 0.5);

    // Low-pass filter: Removes high frequencies (muffled sound)
    comp.track("low_pass")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(800.0, 0.3))
        .at(2.5)
        .notes(&[C3, E3, G3, C4], 0.5);

    // Low-pass with high resonance (emphasizes cutoff frequency)
    comp.track("low_pass_resonant")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(1200.0, 0.8))
        .at(5.0)
        .notes(&[C3, E3, G3, C4], 0.5);

    // High-pass filter: Removes low frequencies (thin sound)
    comp.track("high_pass")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::high_pass(1500.0, 0.3))
        .at(7.5)
        .notes(&[C3, E3, G3, C4], 0.5);

    // High-pass with resonance (telephone effect)
    comp.track("high_pass_resonant")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::high_pass(2000.0, 0.7))
        .at(10.0)
        .notes(&[C3, E3, G3, C4], 0.5);

    // Band-pass filter: Only middle frequencies pass through
    comp.track("band_pass")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::band_pass(1000.0, 0.5))
        .at(12.5)
        .notes(&[C3, E3, G3, C4], 0.5);

    // Band-pass with high resonance (vocal formant)
    comp.track("band_pass_resonant")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::band_pass(1500.0, 0.85))
        .at(15.0)
        .notes(&[C3, E3, G3, C4], 0.5);

    // Classic synth bass: Low-pass with square wave
    comp.track("synth_bass")
        .waveform(Waveform::Square)
        .filter(Filter::low_pass(300.0, 0.4))
        .at(17.5)
        .notes(&[C2, C2, G2, AS2], 0.4);

    // Pad sound: Low-pass sawtooth
    comp.track("pad_sound")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(600.0, 0.3))
        .at(19.5)
        .notes(&[C3, E3, G3, C4, E4, G4], 0.8);

    // Lead sound: Band-pass with resonance
    comp.track("lead_sound")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::band_pass(2000.0, 0.75))
        .at(22.0)
        .notes(&[C4, D4, E4, F4, G4, F4, E4, D4], 0.25);

    // Progressive cutoff demonstration
    comp.track("cutoff_100")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(100.0, 0.5))
        .at(24.5)
        .note(&[A3], 0.5);

    comp.track("cutoff_400")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(400.0, 0.5))
        .at(25.2)
        .note(&[A3], 0.5);

    comp.track("cutoff_1600")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(1600.0, 0.5))
        .at(26.0)
        .note(&[A3], 0.5);

    comp.track("cutoff_6400")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(6400.0, 0.5))
        .at(26.8)
        .note(&[A3], 0.5);

    // Resonance demonstration
    comp.track("resonance_0")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(1000.0, 0.0))
        .at(28.0)
        .note(&[C4], 0.5);

    comp.track("resonance_50")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(1000.0, 0.5))
        .at(28.7)
        .note(&[C4], 0.5);

    comp.track("resonance_90")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(1000.0, 0.9))
        .at(29.4)
        .note(&[C4], 0.5);

    println!("✓ Filter types available:");
    println!("  - Filter::low_pass(cutoff, resonance)");
    println!("  - Filter::high_pass(cutoff, resonance)");
    println!("  - Filter::band_pass(cutoff, resonance)");
    println!("\n✓ Parameters:");
    println!("  - cutoff: Frequency in Hz (20-20000)");
    println!("    • Low values = darker, muffled");
    println!("    • High values = brighter, more harmonics");
    println!("  - resonance: Emphasis at cutoff (0.0-0.99)");
    println!("    • 0.0 = gentle slope");
    println!("    • 0.9 = sharp peak, self-oscillation");
    println!("\n✓ Filter characteristics:");
    println!("  - Low-pass: Warm bass, pads, removes brightness");
    println!("  - High-pass: Thin sound, removes mud, telephone effect");
    println!("  - Band-pass: Vocal formants, nasal sounds");
    println!("\n✓ Classic synth sounds:");
    println!("  - Bass: Square/Saw + low-pass (200-500 Hz)");
    println!("  - Pad: Sawtooth + low-pass (400-800 Hz)");
    println!("  - Lead: Saw + band-pass with resonance");
    println!("\n✓ Use .filter() on any track\\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
