// Drum Variations Demo - 79 Total Drums!
//
// Showcasing 18 variations of the most commonly used percussion:
// - 4 kick variations
// - 4 snare variations
// - 2 hi-hat variations
// - 4 clap variations
// - 2 cymbal variations
// - 2 shaker variations

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let eighth = comp.tempo().eighth_note();
    let quarter = comp.tempo().quarter_note();

    println!("=== DRUM VARIATIONS LIBRARY ===");
    println!("Total: 79 drums (61 base + 18 variations)\n");

    // Section 1: Kick Variations
    println!("1. KICK VARIATIONS (4)");
    println!("   - Tight: Short, punchy");
    println!("   - Deep: Extended low-end");
    println!("   - Acoustic: Natural drum kit");
    println!("   - Click: Prominent beater attack");

    comp.track("kicks")
        .at(0.0)
        .drum(DrumType::KickTight)
        .wait(quarter)
        .drum(DrumType::KickDeep)
        .wait(quarter)
        .drum(DrumType::KickAcoustic)
        .wait(quarter)
        .drum(DrumType::KickClick);

    // Section 2: Snare Variations
    println!("\n2. SNARE VARIATIONS (4)");
    println!("   - Rim: Rim-focused");
    println!("   - Tight: Short, dry");
    println!("   - Loose: Longer ring");
    println!("   - Piccolo: High-pitched");

    comp.track("snares")
        .at(2.5)
        .drum(DrumType::SnareRim)
        .wait(quarter)
        .drum(DrumType::SnareTight)
        .wait(quarter)
        .drum(DrumType::SnareLoose)
        .wait(quarter)
        .drum(DrumType::SnarePiccolo);

    // Section 3: Hi-Hat Variations
    println!("\n3. HI-HAT VARIATIONS (2)");
    println!("   - Half-Open: Between closed and open");
    println!("   - Sizzle: Lots of high-frequency");

    comp.track("hihats")
        .at(5.0)
        .drum(DrumType::HiHatHalfOpen)
        .wait(eighth)
        .drum(DrumType::HiHatHalfOpen)
        .wait(eighth)
        .drum(DrumType::HiHatSizzle)
        .wait(eighth)
        .drum(DrumType::HiHatSizzle);

    // Section 4: Clap Variations
    println!("\n4. CLAP VARIATIONS (4)");
    println!("   - Dry: No reverb, tight");
    println!("   - Room: Natural ambience");
    println!("   - Group: Multiple claps");
    println!("   - Clap Snare: Hybrid sound");

    comp.track("claps")
        .at(6.5)
        .drum(DrumType::ClapDry)
        .wait(quarter)
        .drum(DrumType::ClapRoom)
        .wait(quarter)
        .drum(DrumType::ClapGroup)
        .wait(quarter)
        .drum(DrumType::ClapSnare);

    // Section 5: Cymbal Variations
    println!("\n5. CYMBAL VARIATIONS (2)");
    println!("   - Crash Short: Quick, gated");
    println!("   - Ride Tip: Bell-less ride");

    comp.track("cymbals")
        .at(9.0)
        .drum(DrumType::CrashShort)
        .wait(1.0)
        .drum(DrumType::RideTip);

    // Section 6: Shaker Variations
    println!("\n6. SHAKER VARIATIONS (2)");
    println!("   - Egg Shaker: Tight, short");
    println!("   - Tube Shaker: Longer, sustained");

    comp.track("shakers")
        .at(11.0)
        .drum(DrumType::EggShaker)
        .wait(0.3)
        .drum(DrumType::EggShaker)
        .wait(0.3)
        .drum(DrumType::TubeShaker)
        .wait(0.3)
        .drum(DrumType::TubeShaker);

    // Section 7: Full Groove Using Variations
    println!("\n7. FULL GROOVE (using all variations)");

    comp.track("groove")
        .at(13.0)
        // Bar 1
        .drum(DrumType::KickTight)
        .wait(0.0)
        .drum(DrumType::HiHatHalfOpen)
        .wait(eighth)
        .drum(DrumType::HiHatSizzle)
        .wait(eighth)
        .drum(DrumType::SnareTight)
        .wait(0.0)
        .drum(DrumType::EggShaker)
        .wait(eighth)
        .drum(DrumType::HiHatHalfOpen)
        .wait(eighth)
        // Bar 2
        .drum(DrumType::KickDeep)
        .wait(0.0)
        .drum(DrumType::HiHatSizzle)
        .wait(eighth)
        .drum(DrumType::HiHatHalfOpen)
        .wait(eighth)
        .drum(DrumType::ClapDry)
        .wait(0.0)
        .drum(DrumType::SnareRim)
        .wait(eighth)
        .drum(DrumType::HiHatSizzle)
        .wait(eighth)
        // Bar 3
        .drum(DrumType::KickAcoustic)
        .wait(0.0)
        .drum(DrumType::HiHatHalfOpen)
        .wait(eighth)
        .drum(DrumType::TubeShaker)
        .wait(eighth)
        .drum(DrumType::SnareLoose)
        .wait(0.0)
        .drum(DrumType::ClapRoom)
        .wait(eighth)
        .drum(DrumType::HiHatSizzle)
        .wait(eighth)
        // Bar 4
        .drum(DrumType::KickClick)
        .wait(0.0)
        .drum(DrumType::HiHatHalfOpen)
        .wait(eighth)
        .drum(DrumType::EggShaker)
        .wait(eighth)
        .drum(DrumType::SnarePiccolo)
        .wait(0.0)
        .drum(DrumType::ClapGroup)
        .wait(quarter)
        .drum(DrumType::CrashShort);

    println!("\n=== LIBRARY SUMMARY ===");
    println!("Kicks: 8 total (4 base + 4 variations)");
    println!("Snares: 7 total (3 base + 4 variations)");
    println!("Hi-Hats: 7 total (5 base + 2 variations)");
    println!("Claps: 6 total (2 base + 4 variations)");
    println!("Cymbals: 7 total (5 base + 2 variations)");
    println!("Shakers: 5 total (3 base + 2 variations)");
    println!("\n**Total: 79 drums!**\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
