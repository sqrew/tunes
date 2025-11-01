use tunes::prelude::*;

/// Demonstrate the synthesis capabilities: FM, filter envelopes, and AM
fn main() -> Result<(), anyhow::Error> {
    println!("\n🎹 Synthesis Showcase: AM, Subtractive, and FM Synthesis\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(100.0));

    // === AM (AMPLITUDE MODULATION) ===
    // AM is already available via the RingModulator effect
    println!("🔊 AM (Ring Modulation):");
    comp.instrument("am_demo", &Instrument::pluck())
        .at(0.0)
        .ring_mod(RingModulator::new(440.0, 0.7))  // 440 Hz carrier, 70% wet
        .note(&[C4], 0.5)
        .note(&[E4], 0.5)
        .note(&[G4], 0.5);
    println!("   ✓ RingModulator creates metallic/robotic tones\n");

    // === ENHANCED SUBTRACTIVE SYNTHESIS ===
    println!("🎛️  Enhanced Subtractive Synthesis:");

    // Classic analog filter sweep
    comp.instrument("classic_sweep", &Instrument::pluck())
        .at(3.0)
        .filter(Filter::low_pass(200.0, 0.7))  // Start with resonant filter
        .classic_filter()  // Add classic filter envelope
        .note(&[C3], 1.0)
        .note(&[G3], 1.0);
    println!("   ✓ Classic filter envelope: fast attack, medium decay");

    // Pluck-style filter
    comp.instrument("pluck_synth", &Instrument::warm_pad())
        .at(6.0)
        .filter(Filter::low_pass(300.0, 0.8))
        .pluck_filter()  // Quick decay for percussive sound
        .note(&[E3], 0.3)
        .note(&[G3], 0.3)
        .note(&[A3], 0.3)
        .note(&[C4], 0.3);
    println!("   ✓ Pluck filter: instant attack, fast decay");

    // Pad with slow filter sweep
    comp.instrument("evolving_pad", &Instrument::warm_pad())
        .at(8.5)
        .filter(Filter::low_pass(400.0, 0.5))
        .pad_filter()  // Slow evolution
        .note(&[C3, E3, G3], 3.0);
    println!("   ✓ Pad filter: slow attack and release for atmosphere\n");

    // === FM SYNTHESIS ===
    println!("⚡ FM Synthesis:");

    // Electric piano (classic DX7 sound)
    comp.instrument("fm_piano", &Instrument::pluck())
        .at(12.0)
        .fm_electric_piano()
        .notes(&[C4, E4, G4, C5, G4, E4], 0.3);
    println!("   ✓ Electric piano: mod_ratio=1.0, evolving brightness");

    // Bell sounds (inharmonic)
    comp.instrument("fm_bells", &Instrument::synth_lead())
        .at(14.5)
        .fm_bell()
        .note(&[C5], 0.5)
        .note(&[E5], 0.5)
        .note(&[G5], 0.5)
        .note(&[C6], 1.0);
    println!("   ✓ Bell: mod_ratio=3.5 (inharmonic), high index");

    // Brass
    comp.instrument("fm_brass", &Instrument::synth_lead())
        .at(17.0)
        .fm_brass()
        .notes(&[F3, A3, C4, F4], 0.6);
    println!("   ✓ Brass: high modulation index with envelope");

    // FM bass
    comp.instrument("fm_bass", &Instrument::pluck())
        .at(19.5)
        .fm_bass()
        .note(&[C2], 0.4)
        .note(&[C2], 0.4)
        .note(&[G2], 0.4)
        .note(&[C3], 0.4);
    println!("   ✓ FM bass: subtle harmonics, fundamental-heavy");

    // Growling bass (octave-down modulator)
    comp.instrument("growl_bass", &Instrument::pluck())
        .at(21.5)
        .fm_growl()
        .note(&[E2], 0.5)
        .note(&[E2], 0.5)
        .note(&[G2], 0.5)
        .note(&[A2], 0.5);
    println!("   ✓ Growl: mod_ratio=0.5 (sub-octave), aggressive");

    // Metallic pad
    comp.instrument("fm_pad", &Instrument::warm_pad())
        .at(24.0)
        .fm_metallic_pad()
        .note(&[A2, C3, E3], 3.0);
    println!("   ✓ Metallic pad: irrational ratio, evolving texture");

    // Custom FM sound
    comp.instrument("custom_fm", &Instrument::synth_lead())
        .at(28.0)
        .fm_custom(2.0, 4.0)  // Octave up, moderate index
        .notes(&[C4, D4, E4, G4, A4, C5], 0.25);
    println!("   ✓ Custom FM: mod_ratio=2.0, mod_index=4.0\n");

    // === COMBINED: FM + FILTER ENVELOPE ===
    println!("🌟 Combined FM + Filter Envelope:");
    comp.instrument("fm_filtered", &Instrument::synth_lead())
        .at(30.0)
        .filter(Filter::low_pass(500.0, 0.6))
        .fm_with_filter(
            FMParams::brass(),
            FilterEnvelope::classic()
        )
        .note(&[C3, E3, G3], 2.0);
    println!("   ✓ FM brass + filter sweep = rich, evolving timbre\n");

    println!("=== Summary ===");
    println!("✅ AM: RingModulator for metallic tones");
    println!("✅ Subtractive: Filter envelopes (classic, pluck, pad, bass)");
    println!("✅ FM: Electric piano, bells, brass, bass, growl, pads");
    println!("✅ Combined: FM + filter envelopes for maximum expressiveness\n");

    let mixer = comp.into_mixer();
    println!("Playing {:.1} seconds of synthesis examples...", mixer.total_duration());
    engine.play_mixer(&mixer)?;
    Ok(())
}
