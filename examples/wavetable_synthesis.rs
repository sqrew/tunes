use tunes::synthesis::wavetable::{DEFAULT_TABLE_SIZE, Wavetable};

/// Demonstrate user-definable wavetable synthesis
///
/// This example shows how to create custom waveforms using:
/// - Functions (procedural waveforms)
/// - Harmonics (additive synthesis)
/// - Samples (arbitrary waveform data)
/// - Presets (band-limited waveforms)
///
/// Note: This demonstrates the wavetable API. Future versions will
/// integrate custom wavetables directly into the Composition API.
fn main() -> anyhow::Result<()> {
    println!("\n🌊 Wavetable Synthesis API Demo\n");

    // Example 1: Band-limited waveforms (reduce aliasing)
    println!("Example 1: Band-Limited Waveforms");
    println!("  These use additive synthesis to reduce aliasing\n");

    let saw_wt = Wavetable::saw_bandlimited();
    let square_wt = Wavetable::square_bandlimited();

    println!("  Saw wavetable samples:");
    for i in 0..8 {
        let phase = i as f32 / 8.0;
        print!("    phase {:.2}: {:.3}  ", phase, saw_wt.sample(phase));
        if (i + 1) % 2 == 0 {
            println!();
        }
    }
    println!();

    println!("  Square wavetable samples:");
    for i in 0..8 {
        let phase = i as f32 / 8.0;
        print!("    phase {:.2}: {:.3}  ", phase, square_wt.sample(phase));
        if (i + 1) % 2 == 0 {
            println!();
        }
    }
    println!();

    // Example 2: Custom wavetables from functions
    println!("Example 2: Wavetables from Functions");
    println!("  Create waveforms using any mathematical function\n");

    // Create a warped sine wave
    let warped_wt = Wavetable::from_fn(DEFAULT_TABLE_SIZE, |phase| {
        let warped_phase = phase.powf(1.5); // Warp the phase
        (warped_phase * 2.0 * std::f32::consts::PI).sin()
    });

    println!("  Warped sine wave:");
    for i in 0..8 {
        let phase = i as f32 / 8.0;
        print!("    phase {:.2}: {:.3}  ", phase, warped_wt.sample(phase));
        if (i + 1) % 2 == 0 {
            println!();
        }
    }
    println!();

    // Example 3: Harmonics (additive synthesis)
    println!("Example 3: Additive Synthesis with Harmonics");
    println!("  Build complex waveforms by adding sine wave harmonics\n");

    // Organ-like sound (multiple harmonics)
    let organ_wt = Wavetable::from_harmonics(
        DEFAULT_TABLE_SIZE,
        &[
            (1, 1.0),   // Fundamental
            (2, 0.5),   // Octave
            (3, 0.33),  // Fifth
            (4, 0.25),  // Octave + Fifth
            (5, 0.2),   // Third harmonic
            (8, 0.125), // Three octaves up
        ],
    );

    println!("  Organ waveform (6 harmonics):");
    for i in 0..8 {
        let phase = i as f32 / 8.0;
        print!("    phase {:.2}: {:.3}  ", phase, organ_wt.sample(phase));
        if (i + 1) % 2 == 0 {
            println!();
        }
    }
    println!();

    // Example 4: PWM (Pulse Width Modulation)
    println!("Example 4: Pulse Width Modulation");
    println!("  Create pulse waves with different duty cycles\n");

    let pwm_25 = Wavetable::pwm(0.25); // 25% duty cycle
    let pwm_75 = Wavetable::pwm(0.75); // 75% duty cycle

    println!("  PWM 25% duty cycle:");
    for i in 0..8 {
        let phase = i as f32 / 8.0;
        print!("    phase {:.2}: {:.2}  ", phase, pwm_25.sample(phase));
        if (i + 1) % 2 == 0 {
            println!();
        }
    }
    println!();

    println!("  PWM 75% duty cycle:");
    for i in 0..8 {
        let phase = i as f32 / 8.0;
        print!("    phase {:.2}: {:.2}  ", phase, pwm_75.sample(phase));
        if (i + 1) % 2 == 0 {
            println!();
        }
    }
    println!();

    // Example 5: Custom samples
    println!("Example 5: Wavetables from Sample Data");
    println!("  Load arbitrary waveform data\n");

    // Create a simple stepped waveform
    let steps = vec![
        0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 0.8, 0.6, 0.4, 0.2, 0.0, -0.2, -0.4, -0.6, -0.8, -1.0, -0.8,
        -0.6, -0.4, -0.2,
    ];
    let _stepped_wt = Wavetable::from_samples(steps.clone());

    println!("  Stepped waveform ({} samples):", steps.len());
    for i in 0..steps.len() {
        print!("    {:.2}  ", steps[i]);
        if (i + 1) % 5 == 0 {
            println!();
        }
    }
    println!("\n");

    // Example 6: Performance comparison
    println!("Example 6: Performance");
    println!("  Wavetable lookups are ~10-100x faster than calling sin()\n");

    let sine_wt = Wavetable::sine();

    println!("  Sampling at 440Hz over 1 second:");
    let sample_rate = 44100.0;
    let freq = 440.0;

    let start = std::time::Instant::now();
    for i in 0..44100 {
        let time = i as f32 / sample_rate;
        let _sample = sine_wt.sample_at(freq, time);
    }
    let wavetable_time = start.elapsed();

    let start = std::time::Instant::now();
    for i in 0..44100 {
        let time = i as f32 / sample_rate;
        let phase = time * freq;
        let _sample = (phase * 2.0 * std::f32::consts::PI).sin();
    }
    let direct_time = start.elapsed();

    println!("    Wavetable: {:.2?}", wavetable_time);
    println!("    Direct sin(): {:.2?}", direct_time);
    println!(
        "    Speedup: {:.1}x\n",
        direct_time.as_secs_f64() / wavetable_time.as_secs_f64()
    );

    println!("✅ Wavetable API Demo complete!\n");
    println!("Key features:");
    println!("  • from_fn(): Create waveforms from mathematical functions");
    println!("  • from_harmonics(): Additive synthesis with harmonics");
    println!("  • from_samples(): Load arbitrary waveform data");
    println!("  • saw_bandlimited(), square_bandlimited(): Anti-aliased waveforms");
    println!("  • pwm(): Pulse width modulation");
    println!("  • sample(): Fast wavetable lookup with linear interpolation");
    println!("\nWavetable synthesis is essential for real-time audio performance!");
    println!("Use these building blocks to create unique, CPU-efficient sounds.\n");

    Ok(())
}
