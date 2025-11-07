// Expanded Percussion Library Demo
//
// Showcasing 15 new drums spanning orchestral, world, hand percussion, and effects!
// Total library: 61 drums (up from 46)

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(100.0));
    let quarter = comp.tempo().quarter_note();

    println!("Expanded Percussion Library - 61 Total Drums!\n");

    // Section 1: Orchestral Percussion
    println!("=== ORCHESTRAL PERCUSSION ===");
    println!("1. Timpani (tuned bass drum)");
    comp.track("timpani")
        .at(0.0)
        .drum(DrumType::Timpani)
        .wait(0.6)
        .drum(DrumType::Timpani)
        .wait(0.6)
        .drum(DrumType::Timpani);

    println!("2. Gong (deep metallic crash)");
    comp.track("gong")
        .at(2.5)
        .drum(DrumType::Gong);

    println!("3. Chimes (tubular bells)");
    comp.track("chimes")
        .at(6.5)
        .drum(DrumType::Chimes)
        .wait(0.5)
        .drum(DrumType::Chimes)
        .wait(0.5)
        .drum(DrumType::Chimes);

    // Section 2: World Percussion
    println!("\n=== WORLD PERCUSSION ===");
    println!("4. Djembe (West African hand drum)");
    comp.track("djembe")
        .at(8.5)
        .drum(DrumType::Djembe)
        .wait(0.25)
        .drum(DrumType::Djembe)
        .wait(0.25)
        .drum(DrumType::Djembe)
        .wait(0.25)
        .drum(DrumType::Djembe);

    println!("5. Tabla (Indian drums - Bayan & Dayan)");
    comp.track("tabla")
        .at(10.0)
        .drum(DrumType::TablaBayan)
        .wait(0.2)
        .drum(DrumType::TablaDayan)
        .wait(0.1)
        .drum(DrumType::TablaDayan)
        .wait(0.2)
        .drum(DrumType::TablaBayan)
        .wait(0.2)
        .drum(DrumType::TablaDayan)
        .wait(0.1)
        .drum(DrumType::TablaDayan);

    println!("6. Cajon (box drum)");
    comp.track("cajon")
        .at(11.5)
        .drum(DrumType::Cajon)
        .wait(quarter)
        .drum(DrumType::Cajon)
        .wait(quarter)
        .drum(DrumType::Cajon)
        .wait(quarter)
        .drum(DrumType::Cajon);

    // Section 3: Hand Percussion
    println!("\n=== HAND PERCUSSION ===");
    println!("7. Fingersnap");
    comp.track("fingersnap")
        .at(14.0)
        .drum(DrumType::Fingersnap)
        .wait(0.15)
        .drum(DrumType::Fingersnap)
        .wait(0.15)
        .drum(DrumType::Fingersnap)
        .wait(0.15)
        .drum(DrumType::Fingersnap);

    println!("8. Maracas");
    comp.track("maracas")
        .at(15.0)
        .drum(DrumType::Maracas)
        .wait(0.2)
        .drum(DrumType::Maracas)
        .wait(0.2)
        .drum(DrumType::Maracas)
        .wait(0.2)
        .drum(DrumType::Maracas);

    println!("9. Castanet");
    comp.track("castanet")
        .at(16.0)
        .drum(DrumType::Castanet)
        .wait(0.1)
        .drum(DrumType::Castanet)
        .wait(0.1)
        .drum(DrumType::Castanet)
        .wait(0.1)
        .drum(DrumType::Castanet)
        .wait(0.1)
        .drum(DrumType::Castanet);

    println!("10. Sleigh Bells");
    comp.track("sleigh_bells")
        .at(17.0)
        .drum(DrumType::SleighBells)
        .wait(0.5)
        .drum(DrumType::SleighBells);

    // Section 4: Electronic / Effects
    println!("\n=== ELECTRONIC / EFFECTS ===");
    println!("11. Laser Zap (sci-fi sound)");
    comp.track("laser")
        .at(18.5)
        .drum(DrumType::LaserZap)
        .wait(0.4)
        .drum(DrumType::LaserZap)
        .wait(0.4)
        .drum(DrumType::LaserZap);

    println!("12. Reverse Cymbal (buildup effect)");
    comp.track("reverse")
        .at(20.0)
        .drum(DrumType::ReverseCymbal);

    println!("13. White Noise Hit");
    comp.track("noise")
        .at(22.0)
        .drum(DrumType::WhiteNoiseHit)
        .wait(0.3)
        .drum(DrumType::WhiteNoiseHit)
        .wait(0.3)
        .drum(DrumType::WhiteNoiseHit);

    println!("14. Stick Click");
    comp.track("stick")
        .at(23.5)
        .drum(DrumType::StickClick)
        .wait(0.1)
        .drum(DrumType::StickClick)
        .wait(0.1)
        .drum(DrumType::StickClick)
        .wait(0.1)
        .drum(DrumType::StickClick);

    // Section 5: Full Groove Mix
    println!("\n15. Full World Music Groove");
    comp.track("world_groove")
        .at(24.5)
        // Bar 1
        .drum(DrumType::Djembe)
        .wait(0.0)
        .drum(DrumType::Fingersnap)
        .wait(quarter)
        .drum(DrumType::Cajon)
        .wait(0.0)
        .drum(DrumType::Maracas)
        .wait(quarter)
        .drum(DrumType::Djembe)
        .wait(quarter)
        .drum(DrumType::TablaDayan)
        .wait(0.0)
        .drum(DrumType::Castanet)
        .wait(quarter)
        // Bar 2
        .drum(DrumType::Cajon)
        .wait(0.0)
        .drum(DrumType::Maracas)
        .wait(quarter)
        .drum(DrumType::TablaBayan)
        .wait(quarter)
        .drum(DrumType::Djembe)
        .wait(0.0)
        .drum(DrumType::Fingersnap)
        .wait(quarter)
        .drum(DrumType::Cajon)
        .wait(0.0)
        .drum(DrumType::SleighBells)
        .wait(quarter);

    println!("\n=== SUMMARY ===");
    println!("Total drums: 61");
    println!("- Original library: 22");
    println!("- First expansion: +11 (909s, Latin, simple percussion)");
    println!("- Second expansion: +13 (MIDI gap filling)");
    println!("- Third expansion: +15 (Orchestral, World, Hand, Effects)");
    println!("\nCategories:");
    println!("- Kicks: 4 | Snares: 3 | Hi-hats: 5 | Claps: 2");
    println!("- Toms: 5 | Cymbals: 5 | Latin: 12 | Orchestral: 3");
    println!("- World: 4 | Hand Percussion: 8 | Effects: 10\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
