use tunes::prelude::*;

/// Demonstrate parametric EQ - surgical frequency shaping for mixing and sound design
///
/// This example shows how to use multi-band parametric EQ to:
/// - Fix frequency problems (mud, harshness, rumble)
/// - Enhance desired characteristics (clarity, warmth, air)
/// - Create creative effects (telephone, lo-fi)
fn main() -> anyhow::Result<()> {
    println!("\n🎚️ Example: Parametric EQ API\n");

    println!("Parametric EQ - Multi-band frequency shaping\n");

    // 1. Creating a basic EQ
    println!("▶ Creating a basic 2-band EQ");
    let eq1 = ParametricEQ::new()
        .band(100.0, -6.0, 1.0)    // Cut 100Hz by 6dB, Q=1.0
        .band(3000.0, 4.0, 2.0);   // Boost 3kHz by 4dB, Q=2.0

    println!("  Created EQ with {} bands", eq1.bands.len());

    // 2. Using presets
    println!("\n▶ Using EQ presets");

    let vocal = ParametricEQ::new().preset(EQPreset::VocalClarity);
    println!("  Vocal Clarity: {} bands", vocal.bands.len());
    println!("    - Cuts rumble and mud");
    println!("    - Boosts presence at 3kHz");
    println!("    - Tames sibilance at 8kHz");

    let bass = ParametricEQ::new().preset(EQPreset::BassBoost);
    println!("  Bass Boost: {} bands", bass.bands.len());
    println!("    - Enhances sub and bass");
    println!("    - Cleans up mud");

    let bright = ParametricEQ::new().preset(EQPreset::BrightAiry);
    println!("  Bright & Airy: {} bands", bright.bands.len());
    println!("    - Boosts presence and air");

    let telephone = ParametricEQ::new().preset(EQPreset::Telephone);
    println!("  Telephone: {} bands", telephone.bands.len());
    println!("    - Creates lo-fi effect");
    println!("    - Cuts lows and highs");

    let warmth = ParametricEQ::new().preset(EQPreset::Warmth);
    println!("  Warmth: {} bands", warmth.bands.len());
    println!("    - Enhances low-mids");

    // 3. Processing samples
    println!("\n▶ Processing samples through EQ");
    let mut eq = ParametricEQ::new()
        .band(1000.0, 3.0, 2.0); // Boost 1kHz

    let input_sample = 0.5;
    let output_sample = eq.process(input_sample, 0.0, 0);
    println!("  Input: {:.4}, Output: {:.4}", input_sample, output_sample);

    // 4. Dynamic band control
    println!("\n▶ Dynamic band control");
    let mut dynamic_eq = ParametricEQ::new()
        .band(500.0, 0.0, 1.0);  // Start with no boost/cut

    println!("  Initial: {} bands", dynamic_eq.bands.len());

    // Update band parameters at runtime
    dynamic_eq.update_band(0, 500.0, 6.0, 2.0, 44100.0);
    println!("  Updated band 0 to +6dB at 500Hz");

    // Enable/disable bands
    dynamic_eq.enable_band(0, false);
    println!("  Disabled band 0 (bypassed)");

    dynamic_eq.enable_band(0, true);
    println!("  Re-enabled band 0");

    // 5. Multi-band professional EQ
    println!("\n▶ Professional vocal EQ chain");
    let mut pro_eq = ParametricEQ::new()
        .band(80.0, -12.0, 0.7)    // High-pass: remove rumble
        .band(250.0, -3.0, 1.5)    // Cut mud
        .band(500.0, -1.5, 2.0)    // Slight cut for clarity
        .band(3000.0, 4.0, 2.0)    // Presence boost
        .band(6000.0, 2.0, 1.5)    // Brilliance
        .band(10000.0, -2.0, 1.0); // Tame harshness

    println!("  Created professional EQ with {} bands", pro_eq.bands.len());
    println!("  Processing chain:");
    for (i, band) in pro_eq.bands.iter().enumerate() {
        println!("    Band {}: {:.0}Hz {:+.1}dB (Q={:.1})",
            i + 1, band.frequency, band.gain_db, band.q);
    }

    // Process a batch of samples
    let test_samples = vec![0.5, 0.3, -0.2, 0.7, -0.4];
    println!("\n  Processing {} samples...", test_samples.len());

    let mut processed = test_samples.clone();
    for sample in &mut processed {
        *sample = pro_eq.process(*sample, 0.0, 0);
    }

    println!("  Original:  {:?}", test_samples.iter().map(|x| format!("{:.2}", x)).collect::<Vec<_>>());
    println!("  Processed: {:?}", processed.iter().map(|x| format!("{:.2}", x)).collect::<Vec<_>>());

    // 6. Reset functionality
    println!("\n▶ Resetting EQ state");
    pro_eq.reset();
    println!("  EQ filter state reset (cleared history)");

    println!("\n✓ Parametric EQ Concepts:");
    println!("  • Multi-band: Process multiple frequency ranges independently");
    println!("  • Surgical control: Precise frequency, gain, and bandwidth (Q)");
    println!("  • Biquad filters: Industry-standard peaking EQ algorithm");
    println!("  • Presets: Common EQ curves for typical scenarios");

    println!("\n✓ API Usage:");
    println!("  ```rust");
    println!("  // Create EQ with custom bands");
    println!("  let mut eq = ParametricEQ::new()");
    println!("      .band(100.0, -6.0, 1.0)    // freq, gain_db, Q");
    println!("      .band(3000.0, 4.0, 2.0);");
    println!();
    println!("  // Or use presets");
    println!("  let mut eq = ParametricEQ::new()");
    println!("      .preset(EQPreset::VocalClarity);");
    println!();
    println!("  // Process samples");
    println!("  for sample in &mut audio_buffer {{");
    println!("      *sample = eq.process(*sample, 0.0, 0);");
    println!("  }}");
    println!();
    println!("  // Dynamic control");
    println!("  eq.update_band(0, 500.0, 6.0, 2.0, 44100.0);");
    println!("  eq.enable_band(1, false);  // Bypass band");
    println!("  eq.reset();  // Clear filter state");
    println!("  ```");

    println!("\n✓ EQ Parameters:");
    println!("  • Frequency: Center frequency to boost/cut (Hz)");
    println!("  • Gain: Amount in dB (+ = boost, - = cut)");
    println!("  • Q: Bandwidth (0.5 = wide, 2.0 = medium, 10.0 = narrow)");

    println!("\n✓ Common Mixing Techniques:");
    println!("  • Cut before boost - Remove problems first");
    println!("  • High-pass at 80-100Hz - Remove rumble");
    println!("  • Boost 3-5kHz - Vocal presence/clarity");
    println!("  • Cut 200-400Hz - Reduce muddiness");
    println!("  • Boost 10kHz+ - Add air and sparkle");
    println!("  • Narrow Q for surgical cuts");
    println!("  • Wide Q for gentle tonal shaping\n");

    Ok(())
}
