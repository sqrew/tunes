use tunes::prelude::*;
use tunes::sequences;

/// Demonstrates the .every_n() method for adding variation to repeated patterns
fn main() -> anyhow::Result<()> {
    println!("\n🔄 Every N Pattern Variation Demo\n");
    println!("Adding predetermined variation to loops with .every_n()\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // ===== PART 1: BASIC ACCENT PATTERN =====
    println!("Part 1: Crash Every 4 Beats\n");
    println!("  Adding crashes on downbeats (4th, 8th, 12th, 16th)\n");

    comp.track("basic_accent")
        .pattern_start()
        .drum(DrumType::Kick, 0.25)
        .repeat(7) // 16 kicks total
        .every_n(4, DrumType::Crash);

    // ===== PART 2: MULTIPLE VARIATIONS =====
    println!("Part 2: Multiple Layers of Variation\n");
    println!("  Crash every 4, ride every 8\n");

    comp.track("multi_variation")
        .at(2.0)
        .pattern_start()
        .drum(DrumType::HiHatClosed, 0.25)
        .drum(DrumType::HiHatClosed, 0.25)
        .repeat(7) // 32 hihats
        .every_n(4, DrumType::Crash) // Crash every 4th
        .every_n(8, DrumType::Ride); // Ride every 8th

    // ===== PART 3: MELODIC ACCENTS =====
    println!("Part 3: Melodic Pattern with Percussion Accents\n");
    println!("  Add cowbell every 3rd note for syncopation\n");

    comp.track("melody_accent")
        .at(4.0)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.2)
        .repeat(5) // 24 notes total
        .every_n(3, DrumType::Cowbell); // Every 3rd gets cowbell

    // ===== PART 4: DRUM PATTERN VARIATION =====
    println!("Part 4: Adding Variation to Drum Loops\n");
    println!("  Basic beat with snare fills every 8 hits\n");

    comp.track("drum_variation")
        .at(9.0)
        .pattern_start()
        .drum(DrumType::Kick, 0.25)
        .repeat(7) // 16 kicks
        .every_n(8, DrumType::Snare); // Fill on 8th, 16th

    // ===== PART 5: COMPLEX POLYRHYTHM =====
    println!("Part 5: Polyrhythmic Accents\n");
    println!("  Every 3 and every 5 create interesting patterns\n");

    comp.track("polyrhythm")
        .at(11.0)
        .pattern_start()
        .drum(DrumType::HiHatClosed, 0.0)
        .repeat(29) // 30 hits
        .every_n(3, DrumType::Clap) // Every 3rd
        .every_n(5, DrumType::Rimshot); // Every 5th

    // ===== PART 6: RHYTHMIC HIHAT PATTERN =====
    println!("Part 6: Hi-Hat Pattern with Open Hats\n");
    println!("  Open hihat every 6th for groove\n");

    comp.track("hihat_groove")
        .at(13.0)
        .pattern_start()
        .drum(DrumType::HiHatClosed, 0.25)
        .drum(DrumType::HiHatClosed, 0.25)
        .repeat(7) // 32 closed hihats
        .every_n(6, DrumType::HiHatOpen); // Open every 6th

    // ===== PART 7: PROGRESSIVE DENSITY =====
    println!("Part 7: Building Intensity\n");
    println!("  Repeated pattern with gradually added percussion\n");

    // Base pattern
    comp.track("progressive")
        .at(15.0)
        .pattern_start()
        .drum(DrumType::Kick808, 0.0);

    // Layer 1: Every 2nd
    comp.track("progressive")
        .at(15.0)
        .pattern_start()
        .drum(DrumType::Kick808, 0.0)
        .repeat(3)
        .every_n(2, DrumType::Snare);

    // Layer 2: Every 4th
    comp.track("progressive2")
        .at(17.0)
        .pattern_start()
        .drum(DrumType::Kick808, 0.0)
        .repeat(7)
        .every_n(2, DrumType::Snare)
        .every_n(4, DrumType::Crash);

    // ===== PART 8: EUCLIDEAN WITH ACCENTS =====
    println!("Part 8: Euclidean Rhythm with Accents\n");
    println!("  Euclidean pattern + crash every 4th hit\n");

    let euclidean = sequences::euclidean::generate(5, 16);

    comp.track("euclidean_accent")
        .at(19.0)
        .drum_grid(16, 0.125, |g| g
        .sound(DrumType::Kick, &euclidean));

    // Add crashes every 4 steps in grid
    for i in (3..16).step_by(4) {
        comp.track("euclidean_accent")
            .at(19.0 + i as f32 * 0.125)
            .drum(DrumType::Crash, 0.0);
    }

    // ===== PART 9: CALL AND RESPONSE =====
    println!("Part 9: Call and Response Pattern\n");
    println!("  Main pattern with answering hits\n");

    comp.track("call_response")
        .at(21.0)
        .pattern_start()
        .drum(DrumType::Tom, 0.25)
        .repeat(7) // 16 toms
        .every_n(4, DrumType::TomLow) // Response on 4s
        .every_n(8, DrumType::Crash); // Accent on 8s

    println!("\n▶️  Playing every_n() demonstration...\n");
    println!("    Duration: ~23 seconds\n");
    println!("    💡 Use Cases:\n");
    println!("    • Adding crashes on downbeats\n");
    println!("    • Accentuating specific beats\n");
    println!("    • Creating call-and-response patterns\n");
    println!("    • Building polyrhythmic textures\n");
    println!("    • Adding variation to loops\n");
    println!("    • Progressive intensity building\n\n");

    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Every N Demo Complete!\n");
    println!("💡 Usage Tips:\n");
    println!("   • Use .every_n(4, DrumType::Crash) for downbeat crashes\n");
    println!("   • Chain multiple .every_n() calls for layered variation\n");
    println!("   • Works with .repeat() for longer patterns\n");
    println!("   • Combines with .drum_grid() for complex grooves\n");
    println!("   • Try prime numbers (3, 5, 7) for polyrhythms\n\n");
    println!("📚 Common Patterns:\n");
    println!("   .every_n(4, DrumType::Crash)    // Downbeats\n");
    println!("   .every_n(8, DrumType::Ride)     // Every 2 bars\n");
    println!("   .every_n(2, DrumType::Snare)    // Backbeat\n");
    println!("   .every_n(3, DrumType::Clap)     // Triplet feel\n\n");

    Ok(())
}
