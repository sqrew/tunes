use tunes::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n🌍  World Scales Showcase\n");
    println!("Demonstrating 55 scale patterns from around the world!\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(100.0));

    let note_duration = 0.25;
    let pause = 0.3;

    // Use a single track for better performance
    let mut track_builder = comp.instrument("showcase", &Instrument::warm_pad());

    // === JAPANESE SCALES ===
    println!("🎌 Japanese Scales:");

    println!("  • Hirajoshi - Traditional Japanese pentatonic");
    let hirajoshi = scale(C4, &ScalePattern::HIRAJOSHI);
    track_builder = track_builder.notes(&hirajoshi, note_duration).wait(pause);

    println!("  • In Sen - Japanese pentatonic with distinctive intervals");
    let in_sen = scale(D4, &ScalePattern::IN_SEN);
    track_builder = track_builder.notes(&in_sen, note_duration).wait(pause);

    println!("  • Iwato - Dark, mysterious Japanese scale");
    let iwato = scale(E4, &ScalePattern::IWATO);
    track_builder = track_builder.notes(&iwato, note_duration).wait(pause);

    // === MIDDLE EASTERN SCALES ===
    println!("\n🕌 Middle Eastern Scales:");

    println!("  • Hijaz - Maqam with characteristic augmented 2nd");
    let hijaz = scale(A3, &ScalePattern::HIJAZ);
    track_builder = track_builder.notes(&hijaz, note_duration).wait(pause);

    println!("  • Persian - Exotic Persian scale");
    let persian = scale(D4, &ScalePattern::PERSIAN);
    track_builder = track_builder.notes(&persian, note_duration).wait(pause);

    // === INDIAN RAGAS ===
    println!("\n🇮🇳 Indian Classical Ragas:");

    println!("  • Bhairav - Morning raga with distinctive character");
    let bhairav = scale(C4, &ScalePattern::BHAIRAV);
    track_builder = track_builder.notes(&bhairav, note_duration).wait(pause);

    println!("  • Marva - Evening raga with augmented 4th");
    let marva = scale(G3, &ScalePattern::MARVA);
    track_builder = track_builder.notes(&marva, note_duration).wait(pause);

    // === HUNGARIAN & GYPSY SCALES ===
    println!("\n🎻 Hungarian & Gypsy Scales:");

    println!("  • Hungarian Minor - Dramatic Gypsy sound");
    let hungarian_minor = scale(E4, &ScalePattern::HUNGARIAN_MINOR);
    track_builder = track_builder.notes(&hungarian_minor, note_duration).wait(pause);

    println!("  • Hungarian Major - Unique major variant");
    let hungarian_major = scale(A3, &ScalePattern::HUNGARIAN_MAJOR);
    track_builder = track_builder.notes(&hungarian_major, note_duration).wait(pause);

    // === SPANISH & FLAMENCO ===
    println!("\n💃 Spanish & Flamenco:");

    println!("  • Phrygian Dominant - Spanish/Flamenco character");
    let spanish = scale(E4, &ScalePattern::PHRYGIAN_DOMINANT);
    track_builder = track_builder.notes(&spanish, note_duration).wait(pause);

    // === JAZZ SCALES ===
    println!("\n🎷 Jazz Scales:");

    println!("  • Bebop Dominant - Classic jazz sound");
    let bebop = scale(C4, &ScalePattern::BEBOP_DOMINANT);
    track_builder = track_builder.notes(&bebop, note_duration * 0.8).wait(pause);

    println!("  • Altered Scale - For dominant 7 chords");
    let altered = scale(G3, &ScalePattern::ALTERED);
    track_builder = track_builder.notes(&altered, note_duration).wait(pause);

    // === EXOTIC & ENIGMATIC ===
    println!("\n✨ Exotic & Enigmatic:");

    println!("  • Enigmatic - Mysterious and unusual");
    let enigmatic = scale(C4, &ScalePattern::ENIGMATIC);
    track_builder = track_builder.notes(&enigmatic, note_duration).wait(pause);

    println!("  • Prometheus - Named after the Scriabin piece");
    let prometheus = scale(F4, &ScalePattern::PROMETHEUS);
    track_builder = track_builder.notes(&prometheus, note_duration).wait(pause);

    // === ASIAN PENTATONICS ===
    println!("\n🏯 Asian Pentatonic Scales:");

    println!("  • Chinese - Traditional Chinese pentatonic");
    let chinese = scale(C4, &ScalePattern::CHINESE);
    track_builder = track_builder.notes(&chinese, note_duration).wait(pause);

    println!("  • Mongolian - Mongolian folk scale");
    let mongolian = scale(D4, &ScalePattern::MONGOLIAN);
    let _ = track_builder.notes(&mongolian, note_duration);

    // Summary
    println!("\n▶ Playing world scales demonstration...\n");

    engine.play_mixer(&comp.into_mixer())?;

    println!("✅ Demo complete!\n");
    println!("📚 Available scale categories:");
    println!("   • Western scales (9) - Major, Minor, Pentatonic, Blues, etc.");
    println!("   • Church modes (7) - Ionian through Locrian");
    println!("   • Jazz & Bebop (6) - Bebop, Altered, Diminished");
    println!("   • Japanese (5) - Hirajoshi, In Sen, Iwato, Yo, Kumoi");
    println!("   • Middle Eastern (4) - Hijaz, Persian, Double Harmonic");
    println!("   • Indian Ragas (5) - Bhairav, Kafi, Marva, etc.");
    println!("   • Hungarian/Gypsy (3) - Hungarian Minor/Major, Gypsy");
    println!("   • Spanish (2) - Phrygian Dominant, Flamenco");
    println!("   • Exotic (6) - Enigmatic, Neapolitan, Prometheus, etc.");
    println!("   • Asian Pentatonic (3) - Chinese, Egyptian, Mongolian");
    println!("   • Modern/Experimental (5) - Lydian variants, Super Locrian\n");
    println!("💡 Total: 55 scales from around the world!");
    println!("   Use with: scale(root, &ScalePattern::SCALE_NAME)\n");

    Ok(())
}
