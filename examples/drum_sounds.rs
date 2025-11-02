use tunes::prelude::*;

/// Demonstrate all drum sounds
fn main() -> anyhow::Result<()> {
    println!("\n🥁 Example: Drum Sounds\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.track("drums")
        .at(0.0)
        .drum(DrumType::Kick)
        .wait(0.2)
        .drum(DrumType::Snare)
        .wait(0.2)
        .drum(DrumType::Tom)
        .wait(0.2)
        .drum(DrumType::HiHatClosed)
        .wait(0.2)
        .drum(DrumType::HiHatOpen)
        .wait(0.2)
        .drum(DrumType::Rimshot)
        .wait(0.2)
        .drum(DrumType::Cowbell)
        .wait(0.2)
        .drum(DrumType::Clap)
        .wait(0.2)
        .drum(DrumType::Crash)
        .wait(0.5)
        .drum(DrumType::Ride);

    println!("✓ 10 drum types available:");
    println!("  - Kick, Snare, Tom");
    println!("  - HiHatClosed, HiHatOpen");
    println!("  - Rimshot, Cowbell, Clap");
    println!("  - Crash, Ride\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
