/// Demonstration of custom wavetable synthesis
///
/// This example shows how to create and use custom wavetables for unique timbres.
/// Run with: cargo run --example wavetable_demo

use tunes::prelude::*;
use tunes::wavetable::{Wavetable, DEFAULT_TABLE_SIZE};

fn main() -> anyhow::Result<()> {
    println!("Custom Wavetable Synthesis Demo");
    println!("================================\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // Example 1: Organ sound using odd harmonics
    println!("1. Creating organ sound with odd harmonics...");
    let organ_wt = Wavetable::from_harmonics(
        DEFAULT_TABLE_SIZE,
        &[(1, 1.0), (3, 0.5), (5, 0.3), (7, 0.2), (9, 0.1)],
    );

    comp.track("organ")
        .custom_waveform(organ_wt)
        .notes(&[C4, E4, G4, C5], 0.5);

    // Example 2: Bright synth using all harmonics
    println!("2. Creating bright synth with all harmonics...");
    let bright_wt = Wavetable::from_harmonics(
        DEFAULT_TABLE_SIZE,
        &[
            (1, 1.0),
            (2, 0.8),
            (3, 0.6),
            (4, 0.4),
            (5, 0.3),
            (6, 0.2),
            (7, 0.15),
            (8, 0.1),
        ],
    );

    comp.track("bright")
        .custom_waveform(bright_wt)
        .notes(&[G4, B4, D5, G5], 0.5);

    // Example 3: Custom waveform from function
    println!("3. Creating custom waveform from function...");
    let custom_wt = Wavetable::from_fn(DEFAULT_TABLE_SIZE, |phase| {
        // Combine multiple shapes for unique timbre
        let sine = (phase * std::f32::consts::TAU).sin();
        let pulse = if phase < 0.3 { 1.0 } else { -0.5 };
        sine * 0.6 + pulse * 0.4
    });

    comp.track("custom")
        .custom_waveform(custom_wt)
        .notes(&[E3, G3, B3, E4], 0.5);

    // Example 4: Using built-in band-limited waveforms
    println!("4. Using band-limited sawtooth...");
    comp.track("saw")
        .custom_waveform(Wavetable::saw_bandlimited())
        .notes(&[A3, C4, E4, A4], 0.5);

    // Example 5: PWM (Pulse Width Modulation) with different duty cycles
    println!("5. PWM with 25% duty cycle...");
    comp.track("pwm")
        .custom_waveform(Wavetable::pwm(0.25))
        .notes(&[F3, A3, C4, F4], 0.5);

    // Render to file
    println!("\nRendering composition to wavetable_demo.wav...");
    let mut mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;
    engine.render_to_wav(&mut mixer, "wavetable_demo.wav")?;

    println!("✓ Done! Play wavetable_demo.wav to hear the custom wavetables.");
    println!("\nNotice how each track has a distinct timbre from its custom wavetable!");

    Ok(())
}
