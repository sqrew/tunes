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
        .phaser(Phaser::new(0.5, 0.7, 0.5, 4, 0.7)) // rate, depth, feedback, stages, mix
        .at(17.7)
        .note(&[E4, G4, B4], 2.5);

    // Flanger - Jet plane effect
    comp.instrument("flanger_demo", &Instrument::pluck())
        .flanger(Flanger::new(0.5, 3.0, 0.7, 0.7)) // rate, depth (ms), feedback, mix
        .at(20.5)
        .note(&[A3, C4, E4], 2.5);

    // Tremolo - Amplitude modulation
    comp.instrument("tremolo_demo", &Instrument::warm_pad())
        .tremolo(Tremolo::new(5.0, 0.8)) // rate (Hz), depth
        .at(23.5)
        .note(&[C4, E4, G4], 2.5);

    // AutoPan - Stereo movement
    comp.instrument("autopan_demo", &Instrument::warm_pad())
        .autopan(AutoPan::new(0.5, 0.9)) // rate (Hz), depth
        .at(26.5)
        .note(&[G3, B3, D4], 2.5);

    // ===== 4. DYNAMIC CONTROL =====
    println!("\n4. Dynamic Control - Gate, Limiter");

    // Gate - Noise reduction and gating
    comp.instrument("gate_demo", &Instrument::synth_lead())
        .gate(Gate::new(-30.0, 10.0, 0.001, 0.05)) // threshold (dB), ratio, attack, release
        .at(29.5)
        .notes(&[C4, 0.0, E4, 0.0, G4, 0.0, C5], 0.15); // 0.0 = silence to show gating

    // Limiter - Peak control (subtle effect, prevents clipping)
    comp.instrument("limiter_demo", &Instrument::synth_lead())
        .limiter(Limiter::new(-3.0, 0.05)) // threshold (dB), release
        .at(31.5)
        .notes(&[C4, E4, G4, C5], 0.3);

    // ===== 5. LO-FI EFFECTS =====
    println!("\n5. Lo-Fi Effects - BitCrusher, Ring Modulator");

    // BitCrusher - Retro degradation
    comp.instrument("bitcrush_demo", &Instrument::synth_lead())
        .bitcrusher(BitCrusher::new(4.0, 8.0, 0.7)) // bit_depth, sample_rate_div, mix
        .at(33.5)
        .notes(&[C4, E4, G4, E4], 0.3);

    // Ring Modulator - Metallic/robotic
    comp.instrument("ringmod_demo", &Instrument::synth_lead())
        .ring_mod(RingModulator::new(440.0, 0.8)) // carrier_freq, mix
        .at(35.5)
        .notes(&[C4, E4, G4, C5], 0.5);

    // ===== 6. COMPARISON DEMONSTRATIONS =====
    println!("\n6. Comparison Demonstrations");

    // Compressor comparison
    println!("  • Compressor: Before and after");
    comp.instrument("uncomp", &Instrument::synth_lead())
        .at(38.0)
        .note(&[C4], 0.2)
        .note(&[E4], 0.2)
        .note(&[G4], 0.2)
        .note(&[C5], 0.2);

    comp.instrument("comp", &Instrument::synth_lead())
        .compressor(Compressor::new(0.2, 6.0, 0.005, 0.05, 2.0))
        .at(38.8)
        .note(&[C4], 0.2)
        .note(&[E4], 0.2)
        .note(&[G4], 0.2)
        .note(&[C5], 0.2);

    // Tremolo rate comparison
    println!("  • Tremolo: Slow vs Fast");
    comp.instrument("tremolo_slow", &Instrument::warm_pad())
        .tremolo(Tremolo::new(2.0, 0.7)) // Slow 2Hz
        .at(40.0)
        .note(&[C4, E4, G4], 2.0);

    comp.instrument("tremolo_fast", &Instrument::warm_pad())
        .tremolo(Tremolo::new(8.0, 0.7)) // Fast 8Hz
        .at(42.2)
        .note(&[C4, E4, G4], 2.0);

    // AutoPan comparison
    println!("  • AutoPan: Width variations");
    comp.instrument("autopan_narrow", &Instrument::warm_pad())
        .autopan(AutoPan::new(1.0, 0.3)) // Narrow panning
        .at(44.5)
        .note(&[E3, G3, B3], 1.5);

    comp.instrument("autopan_wide", &Instrument::warm_pad())
        .autopan(AutoPan::new(1.0, 0.9)) // Wide panning
        .at(46.2)
        .note(&[E3, G3, B3], 1.5);

    // EQ comparison
    println!("  • EQ: Bass boost, Mid boost, Treble boost");
    let eq_chord = &[C3, E3, G3, C4];

    comp.instrument("eq_bass_boost", &Instrument::warm_pad())
        .eq(EQ::new(3.0, 1.0, 0.5, 300.0, 2000.0))
        .at(48.0)
        .note(eq_chord, 1.0);

    comp.instrument("eq_mid_boost", &Instrument::warm_pad())
        .eq(EQ::new(0.7, 2.5, 0.7, 300.0, 2000.0))
        .at(49.2)
        .note(eq_chord, 1.0);

    comp.instrument("eq_treble_boost", &Instrument::warm_pad())
        .eq(EQ::new(0.6, 1.0, 2.5, 300.0, 2000.0))
        .at(50.4)
        .note(eq_chord, 1.0);

    // BitCrusher sweep
    println!("  • BitCrusher: Progressive degradation");
    for i in 0..6 {
        let bit_depth = 16.0 - (i as f32 * 2.0);
        comp.instrument(&format!("bitcrush_{}", i), &Instrument::synth_lead())
            .bitcrusher(BitCrusher::new(bit_depth, (i + 1) as f32 * 2.0, 0.8))
            .at(52.0 + (i as f32 * 0.3))
            .note(&[A4], 0.25);
    }

    // ===== 7. COMBINED EFFECTS =====
    println!("\n7. Combined Effects - Multiple effects working together");

    // Classic combo: Delay + Reverb
    comp.track("delay_reverb")
        .delay(Delay::new(0.25, 0.3, 0.4))
        .reverb(Reverb::new(0.4, 0.5, 0.3))
        .at(54.5)
        .notes(&[C4, E4, G4], 0.5);

    // Production combo: EQ + Compressor + Chorus
    comp.instrument("production_combo", &Instrument::warm_pad())
        .eq(EQ::new(0.9, 1.2, 0.8, 200.0, 3000.0))
        .compressor(Compressor::new(0.25, 3.0, 0.02, 0.15, 1.3))
        .chorus(Chorus::new(0.4, 0.002, 0.3))
        .at(56.5)
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
        .at(60.0)
        .notes(&[C5, D5, E5, G5], 0.3);

    // Thick modulation: Phaser + Flanger + Tremolo
    comp.instrument("mod_combo", &Instrument::warm_pad())
        .phaser(Phaser::new(0.3, 0.6, 0.4, 4, 0.5))
        .flanger(Flanger::new(0.4, 2.5, 0.5, 0.4))
        .tremolo(Tremolo::new(4.0, 0.5))
        .at(62.0)
        .note(&[D3, F3, A3, C4], 2.5);

    // Stereo dynamics: AutoPan + Chorus + Reverb
    comp.instrument("stereo_combo", &Instrument::warm_pad())
        .autopan(AutoPan::new(0.3, 0.7))
        .chorus(Chorus::new(0.5, 0.003, 0.3))
        .reverb(Reverb::new(0.5, 0.5, 0.4))
        .at(65.0)
        .note(&[E3, G3, B3, D4], 2.5);

    // Clean production chain: Gate + Compressor + EQ + Limiter
    comp.instrument("production_chain", &Instrument::synth_lead())
        .gate(Gate::new(-35.0, 10.0, 0.001, 0.05))
        .compressor(Compressor::new(0.25, 4.0, 0.01, 0.1, 1.5))
        .eq(EQ::new(1.2, 1.0, 0.9, 200.0, 3000.0))
        .limiter(Limiter::new(-1.0, 0.05))
        .at(68.0)
        .notes(&[C4, E4, G4, C5, G4, E4], 0.3);

    // ===== 8. MUSICAL EXAMPLES =====
    println!("\n8. Musical Examples - Effects in context");

    // Dub delay melody
    comp.instrument("dub_delay", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.6, 0.5))
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(70.5)
        .notes(&[C4, E4, G4, A4, G4, E4, D4, C4], 0.25);

    // Phaser progression
    comp.instrument("phaser_progression", &Instrument::warm_pad())
        .phaser(Phaser::new(0.6, 0.7, 0.5, 4, 0.6))
        .at(72.5)
        .note(&[C4, E4, G4], 2.0)
        .note(&[F3, A3, C4], 2.0);

    // Tremolo melody with delay
    comp.instrument("tremolo_melody", &Instrument::synth_lead())
        .tremolo(Tremolo::new(6.0, 0.6))
        .delay(Delay::new(0.25, 0.4, 0.3))
        .at(76.5)
        .notes(&[E4, G4, A4, B4, A4, G4, E4], 0.25);

    // AutoPan with reverb (spacious)
    comp.instrument("autopan_space", &Instrument::warm_pad())
        .autopan(AutoPan::new(0.4, 0.8))
        .reverb(Reverb::new(0.6, 0.6, 0.4))
        .at(78.5)
        .chords(&[&[F3, A3, C4], &[G3, B3, D4], &[E3, G3, B3]], 1.5);

    // Ring mod drums (experimental)
    comp.instrument("ringmod_drums", &Instrument::sub_bass())
        .ring_mod(RingModulator::new(300.0, 0.6))
        .at(83.0)
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

    // Final chord with all modulation effects
    comp.instrument("finale", &Instrument::warm_pad())
        .chorus(Chorus::new(0.4, 0.003, 0.3))
        .phaser(Phaser::new(0.4, 0.6, 0.4, 4, 0.4))
        .flanger(Flanger::new(0.3, 2.0, 0.5, 0.3))
        .tremolo(Tremolo::new(3.0, 0.4))
        .autopan(AutoPan::new(0.25, 0.5))
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(85.0)
        .note(&[C3, E3, G3, C4, E4, G4], 4.0);

    println!("\n▶️  Playing comprehensive effects showcase...");
    println!("    Duration: ~89 seconds\n");
    println!("    💎 Effects Demonstrated:");
    println!("       CORE EFFECTS:");
    println!("       • Delay, Reverb, Distortion");
    println!("       DYNAMIC EFFECTS:");
    println!("       • Compressor, Saturation, EQ");
    println!("       MODULATION EFFECTS:");
    println!("       • Chorus, Phaser, Flanger, Tremolo, AutoPan");
    println!("       DYNAMIC CONTROL:");
    println!("       • Gate, Limiter");
    println!("       LO-FI EFFECTS:");
    println!("       • BitCrusher, Ring Modulator");
    println!("       COMBINATIONS:");
    println!("       • Multiple effects working together");
    println!("       • Production-ready effect chains");

    let mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;

    println!("\n✅ Showcase complete!");
    println!("   All {} audio effects in one comprehensive example!", 15);
    println!("   Experiment with parameters to craft your unique sound!");

    Ok(())
}
