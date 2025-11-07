// Example demonstrating the new drum sounds added to tunes
//
// This showcases:
// - Simple percussion (Claves, Triangle, SideStick, WoodBlock)
// - 909 electronic drums (Kick909, Snare909)
// - Latin percussion (CongaHigh, CongaLow, BongoHigh, BongoLow)
// - Ride Bell

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let eighth = comp.tempo().eighth_note();

    println!("Playing new drum sounds...\n");

    // Section 1: 909 Electronic Drums
    println!("1. 909 Electronic Kit");
    comp.track("909_kit")
        .at(0.0)
        .drum(DrumType::Kick909)
        .wait(eighth)
        .drum(DrumType::Snare909)
        .wait(eighth)
        .drum(DrumType::Kick909)
        .wait(eighth)
        .drum(DrumType::Snare909);

    // Section 2: Latin Percussion
    println!("2. Latin Percussion");
    comp.track("latin")
        .at(2.0)
        .drum(DrumType::CongaLow)
        .wait(0.15)
        .drum(DrumType::CongaHigh)
        .wait(0.15)
        .drum(DrumType::CongaHigh)
        .wait(0.15)
        .drum(DrumType::BongoHigh)
        .wait(0.15)
        .drum(DrumType::BongoLow)
        .wait(0.15)
        .drum(DrumType::BongoHigh);

    // Section 3: Simple Percussion
    println!("3. Simple Percussion");
    comp.track("simple_perc")
        .at(4.0)
        .drum(DrumType::Claves)
        .wait(0.2)
        .drum(DrumType::Claves)
        .wait(0.2)
        .drum(DrumType::WoodBlock)
        .wait(0.2)
        .drum(DrumType::SideStick)
        .wait(0.2)
        .drum(DrumType::Triangle);

    // Section 4: Ride Bell accent pattern
    println!("4. Ride Bell Pattern");
    comp.track("ride_bell")
        .at(6.0)
        .drum(DrumType::RideBell)
        .wait(eighth)
        .drum(DrumType::RideBell)
        .wait(eighth)
        .drum(DrumType::RideBell)
        .wait(eighth)
        .drum(DrumType::RideBell);

    // Section 5: Full Latin groove combining everything
    println!("5. Complete Latin Groove");
    comp.track("full_groove")
        .at(8.0)
        .drum(DrumType::Kick909)
        .wait(0.0)
        .drum(DrumType::CongaLow)
        .wait(0.25)
        .drum(DrumType::Claves)
        .wait(0.25)
        .drum(DrumType::Snare909)
        .wait(0.0)
        .drum(DrumType::CongaHigh)
        .wait(0.25)
        .drum(DrumType::BongoHigh)
        .wait(0.25)
        .drum(DrumType::Kick909)
        .wait(0.0)
        .drum(DrumType::CongaLow)
        .wait(0.25)
        .drum(DrumType::RideBell)
        .wait(0.25)
        .drum(DrumType::Snare909)
        .wait(0.0)
        .drum(DrumType::BongoLow)
        .wait(0.25)
        .drum(DrumType::Triangle);

    println!("\nTotal new drums: 11");
    println!("- Simple percussion: Claves, Triangle, SideStick, WoodBlock");
    println!("- 909 drums: Kick909, Snare909");
    println!("- Latin percussion: CongaHigh, CongaLow, BongoHigh, BongoLow");
    println!("- Utility: RideBell\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
