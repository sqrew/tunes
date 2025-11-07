// Example demonstrating the complete MIDI percussion library (46 drums total!)
//
// This showcases all 13 newly added drums that fill in MIDI percussion gaps:
// - Floor toms (Low, High)
// - Pedal hi-hat
// - Crash cymbal 2
// - Vibraslap
// - Timbales (High, Low)
// - Agogo bells (High, Low)
// - Cabasa
// - Guiro (Short, Long)
// - Wood block (High)

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(110.0));
    let quarter = comp.tempo().quarter_note();
    let eighth = comp.tempo().eighth_note();

    println!("MIDI Percussion Library Demo - 46 Total Drums\n");

    // Section 1: Floor Toms
    println!("1. Floor Toms (Deep & Higher)");
    comp.track("floor_toms")
        .at(0.0)
        .drum(DrumType::FloorTomLow)
        .wait(0.3)
        .drum(DrumType::FloorTomLow)
        .wait(0.3)
        .drum(DrumType::FloorTomHigh)
        .wait(0.3)
        .drum(DrumType::FloorTomHigh);

    // Section 2: Hi-Hat with Pedal
    println!("2. Hi-Hat Pattern with Pedal Chicks");
    comp.track("hihat_pattern")
        .at(2.0)
        .drum(DrumType::HiHatClosed)
        .wait(eighth)
        .drum(DrumType::HiHatPedal)
        .wait(eighth)
        .drum(DrumType::HiHatClosed)
        .wait(eighth)
        .drum(DrumType::HiHatPedal)
        .wait(eighth);

    // Section 3: Crash Cymbals (1 & 2)
    println!("3. Crash Cymbal Variations");
    comp.track("crashes")
        .at(3.5)
        .drum(DrumType::Crash)
        .wait(1.0)
        .drum(DrumType::Crash2);

    // Section 4: Vibraslap
    println!("4. Vibraslap (Distinctive Rattle)");
    comp.track("vibraslap")
        .at(6.0)
        .drum(DrumType::Vibraslap)
        .wait(0.5)
        .drum(DrumType::Vibraslap)
        .wait(0.5)
        .drum(DrumType::Vibraslap);

    // Section 5: Timbales
    println!("5. Timbales (Metallic Shell Drums)");
    comp.track("timbales")
        .at(8.0)
        .drum(DrumType::TimbaleHigh)
        .wait(0.2)
        .drum(DrumType::TimbaleLow)
        .wait(0.2)
        .drum(DrumType::TimbaleHigh)
        .wait(0.2)
        .drum(DrumType::TimbaleLow)
        .wait(0.2)
        .drum(DrumType::TimbaleHigh)
        .wait(0.2)
        .drum(DrumType::TimbaleHigh);

    // Section 6: Agogo Bells
    println!("6. Agogo Bells (Brazilian)");
    comp.track("agogo")
        .at(10.0)
        .drum(DrumType::AgogoHigh)
        .wait(0.25)
        .drum(DrumType::AgogoLow)
        .wait(0.25)
        .drum(DrumType::AgogoHigh)
        .wait(0.25)
        .drum(DrumType::AgogoHigh)
        .wait(0.25)
        .drum(DrumType::AgogoLow);

    // Section 7: Cabasa
    println!("7. Cabasa (Textured Shaker)");
    comp.track("cabasa")
        .at(12.0)
        .drum(DrumType::Cabasa)
        .wait(0.3)
        .drum(DrumType::Cabasa)
        .wait(0.3)
        .drum(DrumType::Cabasa)
        .wait(0.3)
        .drum(DrumType::Cabasa);

    // Section 8: Guiro Scrapes
    println!("8. Guiro (Scraping Sounds)");
    comp.track("guiro")
        .at(14.0)
        .drum(DrumType::GuiroShort)
        .wait(0.15)
        .drum(DrumType::GuiroShort)
        .wait(0.4)
        .drum(DrumType::GuiroLong)
        .wait(0.4)
        .drum(DrumType::GuiroShort)
        .wait(0.15)
        .drum(DrumType::GuiroLong);

    // Section 9: Wood Blocks
    println!("9. Wood Blocks (High & Low)");
    comp.track("wood_blocks")
        .at(16.0)
        .drum(DrumType::WoodBlockHigh)
        .wait(0.15)
        .drum(DrumType::WoodBlock)
        .wait(0.15)
        .drum(DrumType::WoodBlockHigh)
        .wait(0.15)
        .drum(DrumType::WoodBlock)
        .wait(0.15)
        .drum(DrumType::WoodBlockHigh);

    // Section 10: Full Afro-Cuban Groove using all new percussion
    println!("10. Complete Afro-Cuban Groove");
    comp.track("full_groove")
        .at(18.0)
        // Measure 1
        .drum(DrumType::FloorTomLow)
        .wait(0.0)
        .drum(DrumType::TimbaleHigh)
        .wait(quarter)
        .drum(DrumType::AgogoHigh)
        .wait(0.0)
        .drum(DrumType::HiHatPedal)
        .wait(eighth)
        .drum(DrumType::Cabasa)
        .wait(eighth)
        .drum(DrumType::TimbaleHigh)
        .wait(quarter)
        // Measure 2
        .drum(DrumType::GuiroShort)
        .wait(eighth)
        .drum(DrumType::AgogoLow)
        .wait(eighth)
        .drum(DrumType::TimbaleLow)
        .wait(0.0)
        .drum(DrumType::HiHatPedal)
        .wait(quarter)
        .drum(DrumType::FloorTomHigh)
        .wait(0.0)
        .drum(DrumType::AgogoHigh)
        .wait(quarter)
        // Ending crash
        .drum(DrumType::Crash2);

    println!("\n=== Summary ===");
    println!("Total drums in library: 46");
    println!("- Original drums: 22");
    println!("- First expansion: 11 (Claves, Triangle, 909s, Congas, Bongos, etc.)");
    println!("- Second expansion: 13 (Floor toms, Timbales, Agogo, Guiro, etc.)");
    println!("\nMIDI compatibility: Covers most General MIDI percussion (notes 35-81)");
    println!("Perfect for MIDI import/export!\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
