use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(110.0));

    // Example 1: Reverb fade-in (0-4s)
    // Start completely dry, fade to 80% wet over 4 seconds
    comp.instrument("vocals", &Instrument::electric_piano())
        .reverb(
            Reverb::new(0.7, 0.6, 0.0).with_mix_automation(Automation::linear(&[
                (0.0, 0.0), // Start: 0% reverb (completely dry)
                (4.0, 0.8), // 4s: 80% reverb (very wet)
            ])),
        )
        .notes(&[261.63, 293.66, 329.63, 349.23], 1.0);

    // Example 2: Delay feedback sweep (4-8s)
    comp.instrument("delay_demo", &Instrument::synth_lead())
        .delay(
            Delay::new(0.25, 0.0, 0.7).with_feedback_automation(Automation::smooth(&[
                (4.0, 0.0), // No feedback
                (6.0, 0.6), // Build up
                (8.0, 0.0), // Fade out
            ])),
        )
        .at(4.0)
        .notes(&[440.0, 494.0, 523.25], 1.2);

    // Example 3: Distortion drive increase (8-12s)
    comp.instrument("distortion_demo", &Instrument::electric_piano())
        .distortion(
            Distortion::new(1.0, 1.0).with_drive_automation(Automation::linear(&[
                (8.0, 1.0),  // Clean
                (12.0, 8.0), // Heavy distortion
            ])),
        )
        .at(8.0)
        .notes(&[261.63, 329.63, 392.0], 1.0);

    // Example 4: Chorus rate modulation (12-16s)
    comp.instrument("chorus_demo", &Instrument::warm_pad())
        .chorus(
            Chorus::new(0.5, 5.0, 0.8).with_rate_automation(Automation::smooth(&[
                (12.0, 0.3), // Slow
                (14.0, 3.0), // Fast
                (16.0, 0.5), // Slow again
            ])),
        )
        .at(12.0)
        .note(&[C4, E4, G4], 4.0);

    // Example 5: Phaser depth sweep (16-20s)
    comp.instrument("phaser_demo", &Instrument::synth_lead())
        .phaser(
            Phaser::new(0.5, 0.0, 0.6, 4, 0.8).with_depth_automation(Automation::smooth(&[
                (16.0, 0.0), // No effect
                (18.0, 1.0), // Full depth
                (20.0, 0.0), // Fade out
            ])),
        )
        .at(16.0)
        .notes(&[330.0, 370.0, 415.0, 466.0], 1.0);

    // Example 6: Multiple effects with automation on same instrument (20-26s)
    // Distortion builds, then delay feedback increases, while reverb fades in
    comp.instrument("multi_fx", &Instrument::synth_lead())
        .distortion(
            Distortion::new(1.0, 1.0).with_drive_automation(Automation::smooth(&[
                (20.0, 1.0), // Clean
                (22.0, 6.0), // Build distortion
                (26.0, 6.0), // Hold
            ])),
        )
        .delay(
            Delay::new(0.375, 0.0, 0.5).with_feedback_automation(Automation::smooth(&[
                (20.0, 0.0), // No feedback
                (22.0, 0.0), // Wait
                (24.0, 0.7), // Add feedback
                (26.0, 0.7), // Hold
            ])),
        )
        .reverb(
            Reverb::new(0.6, 0.5, 0.0).with_mix_automation(Automation::smooth(&[
                (20.0, 0.0), // Dry
                (24.0, 0.0), // Wait
                (26.0, 0.8), // Fade in reverb
            ])),
        )
        .at(20.0)
        .notes(&[392.0, 440.0, 523.25, 587.33, 659.25], 1.2);

    println!("🎚️  Automation Demo - Dynamic Parameter Control");
    println!("=================================================");
    println!();
    println!("Example 1 (0-4s):   Reverb fade-in");
    println!("  └─ Vocals start dry, gradually become wet");
    println!();
    println!("Example 2 (4-8s):   Delay feedback sweep");
    println!("  └─ Feedback builds up then fades out");
    println!();
    println!("Example 3 (8-12s):  Distortion drive increase");
    println!("  └─ Clean → heavily distorted");
    println!();
    println!("Example 4 (12-16s): Chorus rate modulation");
    println!("  └─ LFO speed: slow → fast → slow");
    println!();
    println!("Example 5 (16-20s): Phaser depth sweep");
    println!("  └─ No effect → full depth → fade out");
    println!();
    println!("Example 6 (20-26s): Multiple effects automated together!");
    println!("  └─ Distortion + delay + reverb building sequentially");
    println!();
    println!("Listen for the smooth parameter changes over time!");
    println!();

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
