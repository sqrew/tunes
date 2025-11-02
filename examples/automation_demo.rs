use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Example 1: Reverb fade-in
    // Start completely dry, fade to 80% wet over 4 seconds
    comp.instrument("vocals", &Instrument::electric_piano())
        .reverb(
            Reverb::new(0.7, 0.6, 0.0)
                .with_mix_automation(Automation::linear(&[
                    (0.0, 0.0),   // Start: 0% reverb (completely dry)
                    (4.0, 0.8),   // 4s: 80% reverb (very wet)
                ]))
        )
        .notes(&[261.63, 293.66, 329.63, 349.23], 1.0);

    // Example 2: Dynamic room size (small → large → small)
    // Creates a morphing space effect
    comp.instrument("lead", &Instrument::synth_lead())
        .reverb(
            Reverb::new(0.3, 0.5, 0.6)
                .with_room_size_automation(Automation::smooth(&[
                    (4.0, 0.2),   // Small room
                    (6.0, 0.9),   // Large cathedral
                    (8.0, 0.2),   // Back to small
                ]))
        )
        .notes(&[440.0, 494.0, 523.25, 587.33], 1.0);

    // Example 3: Combined automation (mix + damping)
    // Fade in reverb while increasing brightness
    comp.instrument("pad", &Instrument::warm_pad())
        .reverb(
            Reverb::new(0.7, 0.8, 0.0)
                .with_mix_automation(Automation::linear(&[
                    (8.0, 0.0),    // Dry at start
                    (12.0, 0.9),   // Very wet at end
                ]))
                .with_damping_automation(Automation::smooth(&[
                    (8.0, 0.8),    // Dark/damped at start
                    (12.0, 0.2),   // Bright at end
                ]))
        )
        .note(&[C4, E4, G4], 4.0);

    println!("🎚️  Automation Demo");
    println!("=====================================");
    println!();
    println!("Example 1 (0-4s): Reverb fade-in");
    println!("  - Vocals start dry, gradually become wet");
    println!();
    println!("Example 2 (4-8s): Dynamic room size");
    println!("  - Lead synth morphs from small room → cathedral → small");
    println!();
    println!("Example 3 (8-12s): Combined automation");
    println!("  - Pad fades in reverb while getting brighter");
    println!();
    println!("Listen for the smooth parameter changes!");
    println!();

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
