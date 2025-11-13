// Volume Test - Demonstrates soft clipping improvement
//
// This example has 20 tracks, but only ONE plays at a time.
// Old behavior: Volume divided by 20 = 5% volume (too quiet!)
// New behavior: Full volume with soft clipping (much better!)

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    println!("=== VOLUME TEST ===");
    println!("20 tracks, but only ONE plays at a time");
    println!("Old method: 1/20 = 5% volume (way too quiet)");
    println!("New method: Full volume with soft clipping\n");

    // Create 20 tracks, each playing a single drum hit sequentially
    for i in 0..20 {
        let start_time = i as f32 * 0.2; // 200ms apart
        let track_name = format!("track_{}", i);

        comp.track(&track_name)
            .at(start_time)
            .drum(DrumType::Kick);
    }

    println!("Playing 20 sequential kick drums (one per track)...\n");
    println!("Listen for consistent volume across all hits.");
    println!("With old method, these would be barely audible!\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
