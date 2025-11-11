use tunes::prelude::*;

/// Demonstrate the new effect presets API
///
/// This example shows how effect presets make it easy to apply
/// musically-appropriate effects without memorizing parameter values.
fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    println!("\n🎛️  Effect Presets Demo\n");
    println!("Showing the new ergonomic preset API for effects\n");

    // Example 1: Delay presets (timing-based)
    println!("1. Delay Presets:");
    comp.instrument("delay_example", &Instrument::synth_lead())
        .notes(&[C4, E4, G4], 0.5)
        .delay(Delay::quarter_note());  // Clean!
    println!("   ✓ Delay::quarter_note() - rhythmic delay at 120 BPM");

    // Example 2: Reverb presets (space-based)
    println!("\n2. Reverb Presets:");
    comp.instrument("reverb_example", &Instrument::electric_piano())
        .at(2.0)
        .notes(&[A4, C5, E5], 0.5)
        .reverb(Reverb::hall());  // So much better!
    println!("   ✓ Reverb::hall() - large concert hall space");

    // Example 3: Distortion presets (character-based)
    println!("\n3. Distortion Presets:");
    comp.instrument("distortion_example", &Instrument::electric_piano())
        .at(4.0)
        .notes(&[E3, G3, B3], 0.5)
        .distortion(Distortion::crunch());  // Instantly recognizable!
    println!("   ✓ Distortion::crunch() - classic rock distortion");

    // Example 4: Compressor presets (use-case based)
    println!("\n4. Compressor Presets:");
    comp.instrument("compressor_example", &Instrument::sub_bass())
        .at(6.0)
        .notes(&[C2, G2, C3], 0.5)
        .compressor(Compressor::bass());  // Perfect for bass!
    println!("   ✓ Compressor::bass() - evens out bass notes");

    // Example 5: Chorus presets (intensity-based)
    println!("\n5. Chorus Presets:");
    comp.instrument("chorus_example", &Instrument::warm_pad())
        .at(8.0)
        .note(&[C4, E4, G4], 2.0)
        .chorus(Chorus::classic());  // 80s vibes!
    println!("   ✓ Chorus::classic() - 80s style chorus");

    // Example 6: Phaser presets (speed-based)
    println!("\n6. Phaser Presets:");
    comp.instrument("phaser_example", &Instrument::synth_lead())
        .at(10.0)
        .notes(&[D4, F4, A4], 0.5)
        .phaser(Phaser::classic());  // 70s style!
    println!("   ✓ Phaser::classic() - 70s style phasing");

    // Example 7: Tremolo presets (speed-based)
    println!("\n7. Tremolo Presets:");
    comp.instrument("tremolo_example", &Instrument::electric_piano())
        .at(12.0)
        .notes(&[G4, B4, D5], 0.5)
        .tremolo(Tremolo::classic());  // Classic trem!
    println!("   ✓ Tremolo::classic() - standard rock/blues tremolo");

    // Example 8: BitCrusher presets (character-based)
    println!("\n8. BitCrusher Presets:");
    comp.instrument("bitcrusher_example", &Instrument::synth_lead())
        .at(14.0)
        .notes(&[C5, E5, G5], 0.5)
        .bitcrusher(BitCrusher::gameboy());  // Retro!
    println!("   ✓ BitCrusher::gameboy() - classic 4-bit handheld sound");

    // Example 9: Chaining presets
    println!("\n9. Chaining Multiple Presets:");
    comp.instrument("chain_example", &Instrument::electric_piano())
        .at(16.0)
        .notes(&[A3, C4, E4, A4], 0.5)
        .reverb(Reverb::plate())
        .delay(Delay::dotted_eighth())
        .chorus(Chorus::subtle());
    println!("   ✓ Reverb::plate() + Delay::dotted_eighth() + Chorus::subtle()");

    // Example 10: EQ presets
    println!("\n10. EQ Presets:");
    comp.instrument("eq_example", &Instrument::synth_lead())
        .at(18.0)
        .notes(&[F4, A4, C5], 0.5)
        .eq(EQ::presence());  // Boost mids for clarity!
    println!("   ✓ EQ::presence() - boost mids for vocal clarity");

    // Example 11: Saturation presets
    println!("\n11. Saturation Presets:");
    comp.instrument("saturation_example", &Instrument::electric_piano())
        .at(20.0)
        .notes(&[C4, E4, G4], 0.5)
        .saturation(Saturation::tape());  // Analog warmth!
    println!("   ✓ Saturation::tape() - warm analog tape character");

    // Example 12: RingModulator presets
    println!("\n12. RingModulator Presets:");
    comp.instrument("ringmod_example", &Instrument::synth_lead())
        .at(22.0)
        .notes(&[A3, C4, E4], 0.5)
        .ring_mod(RingModulator::robotic());  // Sci-fi tones!
    println!("   ✓ RingModulator::robotic() - mid-range carrier for voice effects");

    // Example 13: Gate presets
    println!("\n13. Gate Presets:");
    comp.instrument("gate_example", &Instrument::sub_bass())
        .at(24.0)
        .notes(&[C2, E2, G2], 0.5)
        .gate(Gate::drum());  // Fast gating for drums!
    println!("   ✓ Gate::drum() - fast attack/release for drums");

    // Example 14: Limiter presets
    println!("\n14. Limiter Presets:");
    comp.instrument("limiter_example", &Instrument::synth_lead())
        .at(26.0)
        .notes(&[G4, B4, D5], 0.5)
        .limiter(Limiter::mastering());  // Professional limiting!
    println!("   ✓ Limiter::mastering() - professional mastering limiter");

    // Example 15: You can still customize!
    println!("\n15. Presets Can Still Be Customized:");
    comp.instrument("custom_example", &Instrument::synth_lead())
        .at(28.0)
        .notes(&[E4, G4, B4], 0.5)
        .delay(Delay::quarter_note().with_priority(100));  // Preset + customization!
    println!("   ✓ Delay::quarter_note().with_priority(100) - preset with custom priority");

    println!("\n════════════════════════════════════════");
    println!("\nBefore (verbose, unclear):");
    println!("  .delay(Delay::new(0.25, 0.4, 0.35))");
    println!("  What do those numbers mean? 🤔");
    println!("\nAfter (clean, obvious):");
    println!("  .delay(Delay::quarter_note())");
    println!("  Instantly understandable! ✨");
    println!("\n════════════════════════════════════════\n");

    // Play the composition
    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
