use tunes::prelude::*;

/// Demonstrate additive synthesis - building sounds from sine wave partials
///
/// This example shows the API for creating complex timbres by adding together
/// simple sine waves with different frequency ratios and amplitudes.
fn main() -> anyhow::Result<()> {
    println!("\n🎵 Example: Additive Synthesis API\n");

    println!("Additive Synthesis - Building sounds from sine wave partials\n");

    // 1. Pure Sine Wave (single partial)
    println!("▶ Pure sine wave (fundamental only)");
    let mut pure_sine = AdditiveSynth::new(440.0, 44100.0)
        .add_partial(Partial::harmonic(1, 1.0));

    let samples = pure_sine.generate(1000);
    println!("  Generated {} samples", samples.len());

    // 2. Sawtooth-like (harmonic series with 1/n amplitude)
    println!("\n▶ Sawtooth-like sound (harmonic series)");
    let mut sawtooth = AdditiveSynth::new(440.0, 44100.0)
        .add_partial(Partial::harmonic(1, 1.0))
        .add_partial(Partial::harmonic(2, 0.5))
        .add_partial(Partial::harmonic(3, 0.33))
        .add_partial(Partial::harmonic(4, 0.25))
        .add_partial(Partial::harmonic(5, 0.2))
        .add_partial(Partial::harmonic(6, 0.166));

    let saw_samples = sawtooth.generate(1000);
    println!("  Generated {} samples with 6 harmonics", saw_samples.len());

    // 3. Square-like (odd harmonics only)
    println!("\n▶ Square-like sound (odd harmonics)");
    let mut square = AdditiveSynth::new(440.0, 44100.0)
        .add_partial(Partial::harmonic(1, 1.0))
        .add_partial(Partial::harmonic(3, 0.33))
        .add_partial(Partial::harmonic(5, 0.2))
        .add_partial(Partial::harmonic(7, 0.143))
        .add_partial(Partial::harmonic(9, 0.111));

    let square_samples = square.generate(1000);
    println!("  Generated {} samples with odd harmonics", square_samples.len());

    // 4. Organ Sound (selected harmonics for organ timbre)
    println!("\n▶ Organ sound (organ-like harmonic recipe)");
    let mut organ = AdditiveSynth::new(440.0, 44100.0)
        .add_partial(Partial::harmonic(1, 1.0))    // Fundamental
        .add_partial(Partial::harmonic(2, 0.5))    // Octave
        .add_partial(Partial::harmonic(3, 0.8))    // Fifth
        .add_partial(Partial::harmonic(4, 0.4))    // Double octave
        .add_partial(Partial::harmonic(5, 0.3))    // Major third
        .add_partial(Partial::harmonic(8, 0.2));   // Triple octave

    let organ_samples = organ.generate(1000);
    println!("  Generated {} samples with organ voicing", organ_samples.len());

    // 5. Bell Sound (inharmonic partials for metallic timbre)
    println!("\n▶ Bell sound (inharmonic partials)");
    let mut bell = AdditiveSynth::new(440.0, 44100.0)
        .add_partial(Partial::inharmonic(1.0, 1.0))
        .add_partial(Partial::inharmonic(2.76, 0.6))
        .add_partial(Partial::inharmonic(5.4, 0.4))
        .add_partial(Partial::inharmonic(8.93, 0.25))
        .add_partial(Partial::inharmonic(13.34, 0.15));

    let bell_samples = bell.generate(1000);
    println!("  Generated {} samples with inharmonic ratios", bell_samples.len());

    // 6. Using with_partials for bulk adding
    println!("\n▶ Using with_partials() for bulk configuration");
    let partials = vec![
        Partial::harmonic(1, 1.0),
        Partial::harmonic(2, 0.5),
        Partial::harmonic(3, 0.33),
    ];

    let mut bulk = AdditiveSynth::new(440.0, 44100.0)
        .with_partials(partials);

    let bulk_samples = bulk.generate(1000);
    println!("  Generated {} samples from vec of partials", bulk_samples.len());

    // 7. Dynamic frequency changes
    println!("\n▶ Dynamic frequency control");
    let mut dynamic = AdditiveSynth::new(440.0, 44100.0)
        .add_partial(Partial::harmonic(1, 1.0));

    let a440 = dynamic.generate(500);
    dynamic.set_frequency(880.0); // Change to A5
    let a880 = dynamic.generate(500);
    println!("  Generated at 440Hz ({} samples), then 880Hz ({} samples)", a440.len(), a880.len());

    // 8. Phase reset
    println!("\n▶ Phase reset for synchronized playback");
    let mut synth = AdditiveSynth::new(440.0, 44100.0)
        .add_partial(Partial::harmonic(1, 1.0));

    synth.generate(1000); // Advance phase
    synth.reset(); // Reset to initial state
    let reset_samples = synth.generate(1000);
    println!("  Reset phase and generated {} samples", reset_samples.len());

    println!("\n✓ Additive Synthesis Concepts:");
    println!("  • Harmonic series: Integer frequency ratios (1, 2, 3, 4...)");
    println!("  • Sawtooth: All harmonics with 1/n amplitude");
    println!("  • Square: Odd harmonics only");
    println!("  • Organ: Selected harmonics for warmth");
    println!("  • Inharmonic: Non-integer ratios create metallic/bell timbres");
    println!("  • Flexibility: Precise control over each frequency component");

    println!("\n✓ API Usage:");
    println!("  ```rust");
    println!("  // Create synthesizer");
    println!("  let mut synth = AdditiveSynth::new(440.0, 44100.0)");
    println!("      .add_partial(Partial::harmonic(1, 1.0))");
    println!("      .add_partial(Partial::harmonic(2, 0.5));");
    println!();
    println!("  // Generate samples");
    println!("  let samples = synth.generate(44100); // 1 second");
    println!();
    println!("  // Change frequency");
    println!("  synth.set_frequency(880.0);");
    println!();
    println!("  // Reset phase");
    println!("  synth.reset();");
    println!("  ```\n");

    Ok(())
}
