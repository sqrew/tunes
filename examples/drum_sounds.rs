use tunes::prelude::*;

/// Demonstrate all drum sounds
fn main() -> anyhow::Result<()> {
    println!("\n🥁 Example: Drum Sounds\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.track("drums")
        .at(0.0)
        .drum(DrumType::Kick, 0.2)
        .drum(DrumType::Snare, 0.2)
        .drum(DrumType::Tom, 0.2)
        .drum(DrumType::HiHatClosed, 0.2)
        .drum(DrumType::HiHatOpen, 0.2)
        .drum(DrumType::Rimshot, 0.2)
        .drum(DrumType::Cowbell, 0.2)
        .drum(DrumType::Clap, 0.2)
        .drum(DrumType::Crash, 0.5)
        .drum(DrumType::Ride, 0.0);

    println!("✓ 10 drum types available:");
    println!("  - Kick, Snare, Tom");
    println!("  - HiHatClosed, HiHatOpen");
    println!("  - Rimshot, Cowbell, Clap");
    println!("  - Crash, Ride\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
