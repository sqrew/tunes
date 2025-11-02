use tunes::prelude::*;

/// Demonstrate LFO (Low Frequency Oscillator) modulation
fn main() -> anyhow::Result<()> {
    println!("\n🌊 Example: LFO Modulation\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Filter cutoff modulation (wobble bass)
    comp.track("wobble_bass")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(800.0, 0.6))
        .modulate(ModRoute::new(
            LFO::new(Waveform::Sine, 2.0, 1.0), // 2 Hz sine wave
            ModTarget::FilterCutoff,
            0.8, // 80% modulation depth
        ))
        .at(0.0)
        .notes(&[C2, C2, C2, C2], 0.5);

    // Vibrato: Pitch modulation
    comp.instrument("vibrato", &Instrument::bright_lead())
        .modulate(ModRoute::new(
            LFO::fast_sine(0.5), // Fast sine at 5 Hz
            ModTarget::Pitch,
            0.3, // Subtle pitch variation
        ))
        .at(2.5)
        .notes(&[E4, F4, G4, A4], 0.8);

    // Tremolo: Volume modulation
    comp.instrument("tremolo", &Instrument::organ())
        .modulate(ModRoute::new(
            LFO::new(Waveform::Sine, 6.0, 1.0),
            ModTarget::Volume,
            0.7, // 70% volume modulation
        ))
        .at(6.0)
        .note(&[C4, E4, G4], 2.0);

    // Auto-pan: Stereo movement
    comp.instrument("auto_pan", &Instrument::synth_lead())
        .modulate(ModRoute::new(
            LFO::slow_sine(1.0),
            ModTarget::Pan,
            1.0, // Full pan sweep
        ))
        .at(8.5)
        .notes(&[C4, D4, E4, F4, G4, F4, E4, D4], 0.25);

    // Filter sweep with triangle wave
    comp.track("filter_sweep")
        .waveform(Waveform::Square)
        .filter(Filter::low_pass(400.0, 0.7))
        .modulate(ModRoute::new(
            LFO::triangle(0.5, 1.0), // Slow triangle
            ModTarget::FilterCutoff,
            1.0,
        ))
        .at(11.0)
        .notes(&[A2, A2, A2, A2], 0.8);

    // Stepped modulation (sample & hold effect)
    comp.track("stepped_filter")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(600.0, 0.5))
        .modulate(ModRoute::new(
            LFO::square(4.0, 1.0), // Stepped at 4 Hz
            ModTarget::FilterCutoff,
            0.7,
        ))
        .at(14.5)
        .notes(&[E3, E3, E3, E3, E3, E3, E3, E3], 0.25);

    // Fast filter wobble (dubstep style)
    comp.track("fast_wobble")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(1000.0, 0.8))
        .modulate(ModRoute::new(
            LFO::new(Waveform::Sine, 8.0, 1.0), // 8 Hz wobble
            ModTarget::FilterCutoff,
            0.9,
        ))
        .at(17.0)
        .notes(&[D2, D2, D2, D2], 0.5);

    // Multiple LFOs on one track
    comp.track("multi_mod")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(800.0, 0.5))
        .modulate(ModRoute::new(
            LFO::new(Waveform::Sine, 2.0, 0.8),
            ModTarget::FilterCutoff,
            0.6,
        ))
        .modulate(ModRoute::new(
            LFO::new(Waveform::Sine, 5.0, 0.5),
            ModTarget::Pitch,
            0.2, // Subtle vibrato
        ))
        .at(19.5)
        .notes(&[G3, G3, G3, G3], 0.5);

    // Resonance modulation
    comp.track("resonance_mod")
        .waveform(Waveform::Square)
        .filter(Filter::low_pass(1200.0, 0.3))
        .modulate(ModRoute::new(
            LFO::new(Waveform::Sine, 1.5, 1.0),
            ModTarget::FilterResonance,
            0.7,
        ))
        .at(22.0)
        .notes(&[C3, E3, G3, C4], 0.8);

    // Slow evolving pad
    comp.track("evolving_pad")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(600.0, 0.4))
        .modulate(ModRoute::new(
            LFO::new(Waveform::Triangle, 0.3, 1.0), // Very slow
            ModTarget::FilterCutoff,
            0.5,
        ))
        .at(25.5)
        .note(&[C3, E3, G3, B3], 4.0);

    println!("✓ LFO (Low Frequency Oscillator):");
    println!("  - Periodically modulates parameters over time");
    println!("  - Frequency: typically 0.1 to 20 Hz");
    println!("\n✓ Modulation targets:");
    println!("  - ModTarget::FilterCutoff - Wah/wobble effects");
    println!("  - ModTarget::FilterResonance - Resonance sweeps");
    println!("  - ModTarget::Volume - Tremolo effect");
    println!("  - ModTarget::Pitch - Vibrato effect");
    println!("  - ModTarget::Pan - Auto-pan/stereo movement");
    println!("\n✓ LFO waveforms:");
    println!("  - Sine: Smooth, natural modulation");
    println!("  - Triangle: Linear back-and-forth");
    println!("  - Square: Stepped/rhythmic changes");
    println!("  - Sawtooth: Ramp up or down");
    println!("\n✓ Usage:");
    println!("  .modulate(ModRoute::new(");
    println!("    LFO::new(Waveform::Sine, frequency, depth),");
    println!("    ModTarget::FilterCutoff,");
    println!("    amount");
    println!("  ))");
    println!("\n✓ Presets:");
    println!("  - LFO::slow_sine(depth) - 0.5 Hz sine");
    println!("  - LFO::fast_sine(depth) - 5 Hz sine (vibrato)");
    println!("  - LFO::triangle(freq, depth) - Triangle wave");
    println!("  - LFO::square(freq, depth) - Square wave\\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
