use tunes::prelude::*;

/// Comprehensive showcase of all audio effects
fn main() -> anyhow::Result<()> {
    println!("🎛️  Audio Effects Showcase\n");
    println!("Demonstrating all available audio effects and combinations.\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // ===== 1. CORE EFFECTS =====
    println!("1. Core Effects - Delay, Reverb, Distortion");

    // Clean (reference)
    comp.track("clean").at(0.0).notes(&[C4, E4, G4], 0.5);

    // Delay
    comp.track("delay_demo")
        .delay(Delay::new(0.3, 0.4, 0.5)) // time, feedback, mix
        .at(2.0)
        .notes(&[C4, E4, G4], 0.5);

    // Reverb
    comp.track("reverb_demo")
        .reverb(Reverb::new(0.5, 0.5, 0.3)) // room_size, damping, mix
        .at(4.0)
        .notes(&[C4, E4, G4], 0.5);

    // Distortion
    comp.track("distortion_demo")
        .distortion(Distortion::new(3.0, 0.7)) // drive, mix
        .at(6.0)
        .notes(&[C4, E4, G4], 0.5);

    // ===== 2. DYNAMIC EFFECTS =====
    println!("\n2. Dynamic Effects - Compressor, Saturation, EQ");

    // Compressor - Taming dynamics
    comp.instrument("compressor_demo", &Instrument::synth_lead())
        .compressor(Compressor::new(0.3, 4.0, 0.01, 0.1, 1.5)) // threshold, ratio, attack, release, makeup
        .at(8.5)
        .drum_grid(8, 0.2)
        .kick(&[0, 4])
        .snare(&[2, 6])
        .hihat(&[0, 1, 2, 3, 4, 5, 6, 7]);

    // Saturation - Analog warmth
    comp.instrument("saturation_demo", &Instrument::synth_lead())
        .saturation(Saturation::new(2.5, 0.5, 0.6)) // drive, character, mix
        .at(10.5)
        .notes(&[E4, G4, A4, B4, A4, G4, E4, D4], 0.2);

    // EQ - Frequency shaping
    comp.instrument("eq_demo", &Instrument::sub_bass())
        .eq(EQ::new(2.5, 1.0, 0.6, 250.0, 2500.0)) // low_gain, mid_gain, high_gain, low_freq, high_freq
        .at(12.5)
        .notes(&[C2, C2, E2, G2, C2, E2, G2, A2], 0.2);

    // ===== 3. MODULATION EFFECTS =====
    println!("\n3. Modulation Effects - Chorus, Phaser, Flanger");

    // Chorus - Rich doubling
    comp.instrument("chorus_demo", &Instrument::warm_pad())
        .chorus(Chorus::new(0.5, 0.003, 0.4)) // rate, depth, mix
        .at(14.5)
        .chords(
            &[&[C4, E4, G4], &[A3, C4, E4], &[F3, A3, C4], &[G3, B3, D4]],
            0.8,
        );

    // Phaser - Swoosh effect
    comp.instrument("phaser_demo", &Instrument::warm_pad())
        .phaser(Phaser::new(0.5, 0.7, 0.5, 0.7, 4)) // rate, depth, feedback, mix, stages
        .at(17.7)
        .note(&[E4, G4, B4], 2.5);

    // Flanger - Jet plane effect
    comp.instrument("flanger_demo", &Instrument::pluck())
        .flanger(Flanger::new(0.5, 3.0, 0.7, 0.7)) // rate, depth (ms), feedback, mix
        .at(20.5)
        .note(&[A3, C4, E4], 2.5);

    // ===== 4. LO-FI EFFECTS =====
    println!("\n4. Lo-Fi Effects - BitCrusher, Ring Modulator");

    // BitCrusher - Retro degradation
    comp.instrument("bitcrush_demo", &Instrument::synth_lead())
        .bitcrusher(BitCrusher::new(4.0, 8.0, 0.7)) // bit_depth, sample_rate_div, mix
        .at(23.5)
        .notes(&[C4, E4, G4, E4], 0.3);

    // Ring Modulator - Metallic/robotic
    comp.instrument("ringmod_demo", &Instrument::synth_lead())
        .ring_mod(RingModulator::new(440.0, 0.8)) // carrier_freq, mix
        .at(25.5)
        .notes(&[C4, E4, G4, C5], 0.5);

    // ===== 5. COMPARISON DEMONSTRATIONS =====
    println!("\n5. Comparison Demonstrations");

    // Compressor comparison
    println!("  • Compressor: Before and after");
    comp.instrument("uncomp", &Instrument::synth_lead())
        .at(28.0)
        .note(&[C4], 0.2)
        .note(&[E4], 0.2)
        .note(&[G4], 0.2)
        .note(&[C5], 0.2);

    comp.instrument("comp", &Instrument::synth_lead())
        .compressor(Compressor::new(0.2, 6.0, 0.005, 0.05, 2.0))
        .at(28.8)
        .note(&[C4], 0.2)
        .note(&[E4], 0.2)
        .note(&[G4], 0.2)
        .note(&[C5], 0.2);

    // EQ comparison
    println!("  • EQ: Bass boost, Mid boost, Treble boost");
    let eq_chord = &[C3, E3, G3, C4];

    comp.instrument("eq_bass_boost", &Instrument::warm_pad())
        .eq(EQ::new(3.0, 1.0, 0.5, 300.0, 2000.0))
        .at(30.0)
        .note(eq_chord, 1.0);

    comp.instrument("eq_mid_boost", &Instrument::warm_pad())
        .eq(EQ::new(0.7, 2.5, 0.7, 300.0, 2000.0))
        .at(31.2)
        .note(eq_chord, 1.0);

    comp.instrument("eq_treble_boost", &Instrument::warm_pad())
        .eq(EQ::new(0.6, 1.0, 2.5, 300.0, 2000.0))
        .at(32.4)
        .note(eq_chord, 1.0);

    // BitCrusher sweep
    println!("  • BitCrusher: Progressive degradation");
    for i in 0..6 {
        let bit_depth = 16.0 - (i as f32 * 2.0);
        comp.instrument(&format!("bitcrush_{}", i), &Instrument::synth_lead())
            .bitcrusher(BitCrusher::new(bit_depth, (i + 1) as f32 * 2.0, 0.8))
            .at(34.0 + (i as f32 * 0.3))
            .note(&[A4], 0.25);
    }

    // ===== 6. COMBINED EFFECTS =====
    println!("\n6. Combined Effects - Multiple effects working together");

    // Classic combo: Delay + Reverb
    comp.track("delay_reverb")
        .delay(Delay::new(0.25, 0.3, 0.4))
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .at(36.5)
        .notes(&[C4, E4, G4], 0.5);

    // Production combo: EQ + Compressor + Chorus
    comp.instrument("production_combo", &Instrument::warm_pad())
        .eq(EQ::new(0.9, 1.2, 0.8, 200.0, 3000.0))
        .compressor(Compressor::new(0.25, 3.0, 0.02, 0.15, 1.3))
        .chorus(Chorus::new(0.4, 0.002, 0.3))
        .at(38.5)
        .chords(
            &[
                &[C4, E4, G4, B4],
                &[F3, A3, C4, E4],
                &[G3, B3, D4, F4],
                &[C4, E4, G4, C5],
            ],
            0.8,
        );

    // Lo-fi combo: BitCrusher + Saturation
    comp.instrument("lofi_combo", &Instrument::synth_lead())
        .bitcrusher(BitCrusher::new(3.0, 12.0, 0.8))
        .saturation(Saturation::new(3.0, 0.7, 0.7))
        .at(42.0)
        .notes(&[C5, D5, E5, G5], 0.3);

    // Thick modulation: Phaser + Flanger
    comp.instrument("mod_combo", &Instrument::warm_pad())
        .phaser(Phaser::new(0.3, 0.6, 0.4, 0.5, 4))
        .flanger(Flanger::new(0.4, 2.5, 0.5, 0.4))
        .at(44.0)
        .note(&[D3, F3, A3, C4], 2.5);

    // All effects combined (extreme)
    comp.track("all_effects")
        .delay(Delay::new(0.25, 0.3, 0.4))
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .distortion(Distortion::new(2.0, 0.5))
        .at(47.0)
        .notes(&[C4, E4, G4], 0.5);

    // ===== 7. MUSICAL EXAMPLES =====
    println!("\n7. Musical Examples - Effects in context");

    // Dub delay melody
    comp.instrument("dub_delay", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.6, 0.5))
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(49.5)
        .notes(&[C4, E4, G4, A4, G4, E4, D4, C4], 0.25);

    // Phaser progression
    comp.instrument("phaser_progression", &Instrument::warm_pad())
        .phaser(Phaser::new(0.6, 0.7, 0.5, 0.6, 4))
        .at(51.5)
        .note(&[C4, E4, G4], 2.0)
        .note(&[F3, A3, C4], 2.0);

    // Ring mod drums (experimental)
    comp.instrument("ringmod_drums", &Instrument::sub_bass())
        .ring_mod(RingModulator::new(300.0, 0.6))
        .at(55.5)
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

    // Final chord with all modulation effects
    comp.instrument("finale", &Instrument::warm_pad())
        .chorus(Chorus::new(0.4, 0.003, 0.3))
        .phaser(Phaser::new(0.4, 0.6, 0.4, 0.4, 4))
        .flanger(Flanger::new(0.3, 2.0, 0.5, 0.3))
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(57.5)
        .note(&[C3, E3, G3, C4, E4, G4], 3.5);

    println!("\n▶️  Playing comprehensive effects showcase...");
    println!("    Duration: ~61 seconds\n");
    println!("    💎 Effects Demonstrated:");
    println!("       CORE EFFECTS:");
    println!("       • Delay, Reverb, Distortion");
    println!("       DYNAMIC EFFECTS:");
    println!("       • Compressor, Saturation, EQ");
    println!("       MODULATION EFFECTS:");
    println!("       • Chorus, Phaser, Flanger");
    println!("       LO-FI EFFECTS:");
    println!("       • BitCrusher, Ring Modulator");
    println!("       COMBINATIONS:");
    println!("       • Multiple effects working together");
    println!("       • Production-ready effect chains");

    let mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;

    println!("\n✅ Showcase complete!");
    println!("   All {} audio effects in one comprehensive example!", 11);
    println!("   Experiment with parameters to craft your unique sound!");

    Ok(())
}
