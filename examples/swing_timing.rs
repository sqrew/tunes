use tunes::composition::Composition;
use tunes::drums::DrumType;
use tunes::engine::AudioEngine;
use tunes::rhythm::Tempo;

/// Demonstrate swing timing (straight vs swung)
fn main() -> Result<(), anyhow::Error> {
    println!("\n🎵 Example: Swing/Groove Timing\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Straight hi-hats (no swing)
    comp.track("hihat_straight")
        .pan(-0.5)
        .at(0.0)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed);

    // Swung hi-hats (triplet feel - 0.67)
    comp.track("hihat_swing")
        .pan(0.5)
        .swing(0.67)  // Triplet swing!
        .at(2.0)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed);

    // Heavy swing (0.75)
    comp.track("hihat_heavy")
        .pan(0.0)
        .swing(0.75)  // Heavy swing
        .at(4.0)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed)
        .drum(DrumType::HiHatClosed);

    println!("✓ Swing values:");
    println!("  - 0.5 (straight timing - no swing)");
    println!("  - 0.67 (triplet swing - jazzy)");
    println!("  - 0.75 (heavy swing - extreme)\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
