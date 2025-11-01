use tunes::prelude::*;

/// Demonstrate the 808 drum machine sounds
///
/// This example showcases the TR-808-inspired drum synthesis,
/// including authentic algorithms for kick, snare, hi-hat, and clap.
fn main() -> Result<(), anyhow::Error> {
    println!("\n🥁 808 Drum Machine Demo\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(100.0));

    // Example 1: Individual 808 drum sounds
    println!("Part 1: Individual 808 Sounds");
    println!("  - Kick808: Long, pitched sub-bass kick");
    println!("  - Snare808: Dual triangle oscillators + noise burst");
    println!("  - HiHat808: 6 square wave oscillators (metallic)");
    println!("  - Clap808: Multiple tightly-timed noise bursts\n");

    comp.track("demo")
        .at(0.0)
        .drum(DrumType::Kick808)
        .wait(0.6)
        .drum(DrumType::Snare808)
        .wait(0.3)
        .drum(DrumType::HiHat808Closed)
        .wait(0.1)
        .drum(DrumType::HiHat808Open)
        .wait(0.3)
        .drum(DrumType::Clap808)
        .wait(0.3);

    // Example 2: Classic 808 beat pattern
    println!("Part 2: Classic 808 Beat Pattern");
    comp.track("808_beat")
        .at(3.0)
        .drum_grid(16, 0.15)
        .kick(&[0, 6, 10])
        .snare(&[4, 12])
        .hit(DrumType::Snare808, &[4, 12])
        .hit(DrumType::HiHat808Closed, &[0, 2, 4, 6, 8, 10, 12, 14])
        .hit(DrumType::HiHat808Open, &[3, 11])
        .hit(DrumType::Clap808, &[4, 12]);

    // Example 3: 808-style hip-hop beat
    println!("Part 3: 808 Hip-Hop Pattern\n");
    comp.track("hiphop")
        .at(5.5)
        .drum_grid(16, 0.15)
        .hit(DrumType::Kick808, &[0, 3, 6, 10])
        .hit(DrumType::Snare808, &[4, 12])
        .hit(DrumType::HiHat808Closed, &[1, 2, 5, 7, 9, 11, 13, 14])
        .hit(DrumType::HiHat808Open, &[15])
        .hit(DrumType::Clap808, &[8]);

    println!("=== Playback ===");
    let mixer = comp.into_mixer();
    println!("Playing {:.1}s composition with 808 drums...\n", mixer.total_duration());

    engine.play_mixer(&mixer)?;

    println!("\n✅ 808 Demo complete!\n");
    println!("Technical details:");
    println!("  • Kick808: 200Hz → 30Hz pitch drop, pure sine wave");
    println!("  • Snare808: 180Hz + 240Hz triangle waves + short noise burst");
    println!("  • HiHat808: 6 square oscillators at 3.5-8.4kHz (authentic ratios)");
    println!("  • Clap808: 3 noise bursts at 0ms, 8ms, 16ms + band-pass filter");
    println!("\nThe TR-808 drum machine (1980) used analog synthesis.");
    println!("These algorithms recreate the classic sound using DSP.\n");

    Ok(())
}
