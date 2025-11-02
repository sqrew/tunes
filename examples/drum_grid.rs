use tunes::prelude::*;

/// Demonstrate step sequencer-style drum programming with drum_grid
fn main() -> anyhow::Result<()> {
    println!("\n🎛️  Example: Drum Grid (Step Sequencer)\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Basic 16-step pattern
    comp.track("basic_beat")
        .drum_grid(16, 0.125)  // 16 steps, each 1/16th note (0.125s at 120bpm)
        .kick(&[0, 4, 8, 12])           // Four-on-floor kick
        .snare(&[4, 12])                // Snare on 2 and 4
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);  // 8th note hi-hats

    // Complex pattern
    comp.track("complex_beat")
        .at(2.0)
        .drum_grid(16, 0.125)
        .kick(&[0, 3, 6, 10, 13])       // Syncopated kick
        .snare(&[4, 11])                // Off-beat snare
        .hihat(&[1, 2, 5, 7, 9, 10, 14, 15])  // Complex hi-hat
        .clap(&[8]);                     // Clap accent

    // Triplet feel (12 steps per bar)
    comp.track("triplet_beat")
        .at(4.0)
        .drum_grid(12, 0.167)  // 12 steps for triplet subdivision
        .kick(&[0, 6])
        .snare(&[3, 9])
        .hihat(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

    // Cowbell pattern (because more cowbell)
    comp.track("cowbell")
        .at(6.0)
        .drum_grid(8, 0.25)
        .cowbell(&[0, 2, 5, 7]);

    println!("✓ .drum_grid(steps, step_duration) creates step sequencer");
    println!("✓ Then use .kick(), .snare(), .hihat(), etc with step indices");
    println!("✓ Example: .kick(&[0, 4, 8, 12]) = quarter note kicks");
    println!("✓ Typical: 16 steps at 0.125s = one bar of 16th notes\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
