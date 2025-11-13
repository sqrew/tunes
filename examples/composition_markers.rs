use tunes::prelude::*;

/// Demonstrates the composition markers API for easy track synchronization
fn main() -> anyhow::Result<()> {
    println!("🎯 Composition Markers Demo\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(128.0));

    // === METHOD 1: Mark time points at composition level ===
    println!("Setting up song structure with markers...");
    comp.mark_at("intro", 0.0);
    comp.mark_at("verse", 8.0);
    comp.mark_at("drop", 16.0);
    comp.mark_at("outro", 32.0);

    // === METHOD 2: Markers created by TrackBuilder ===
    println!("Building intro...");
    comp.track("structure")
        .at_marker("intro")
        .wait(4.0)
        .mark("buildup")  // TrackBuilder can also create markers
        .wait(4.0);

    // === Use markers to sync tracks ===
    println!("Syncing tracks to markers...");

    // Intro - just hi-hats
    comp.track("hats")
        .at_marker("intro")
        .rhythm("x-x-x-x-x-x-x-x-", DrumType::HiHatClosed, 0.125);

    // Buildup - add kick
    comp.track("kick_buildup")
        .at_marker("buildup")
        .rhythm("x---x---x---x---", DrumType::Kick, 0.125);

    // Verse - full drums
    comp.track("verse_drums")
        .at_marker("verse")
        .rhythm("x-x-x-x-x-x-x-x-", DrumType::Kick808, 0.125)
        .rhythm("----x-------x---", DrumType::Snare, 0.125);

    // Drop - everything hits together!
    comp.track("drop_kick")
        .at_marker("drop")  // All start at exactly 16.0
        .rhythm("x-x-x-x-x-x-x-x-", DrumType::Kick808, 0.125);

    comp.track("drop_bass")
        .at_marker("drop")  // Synced to drop
        .notes(&[C1, C1, E2, C1, C1, G2, C1, C1], 0.5);

    comp.track("drop_lead")
        .at_marker("drop")  // Also synced to drop
        .notes(&[C5, E5, G5, C6], 0.25)
        .notes(&[C6, G5, E5, C5], 0.25);

    // Outro
    comp.track("outro_pad")
        .at_marker("outro")
        .notes(&[C3, E3, G3], 4.0);

    // === Query markers ===
    println!("\n📍 Markers:");
    let mut marker_list = comp.markers();
    marker_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    for (name, time) in marker_list {
        println!("  {} @ {:.1}s", name, time);
    }

    // === Check specific marker ===
    if let Some(drop_time) = comp.marker_time("drop") {
        println!("\n🔥 The drop hits at {}s!", drop_time);
    }

    println!("\n▶️  Playing composition...");
    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Done! Notice how all the drop elements hit together perfectly.");
    println!("   No manual time calculations needed!\n");

    Ok(())
}
