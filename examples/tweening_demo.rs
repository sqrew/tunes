/// Tweening Demo - Smooth Parameter Interpolation
///
/// Demonstrates Tunes' runtime tweening capabilities for creating
/// smooth, dynamic audio effects:
/// - Volume tweening (fades)
/// - Pan tweening (left-to-right movement)
/// - Playback rate tweening (pitch/speed ramping)

use tunes::prelude::*;
use std::thread::sleep;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("=== Tunes Tweening Demo ===\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // Create a simple sustained pad sound for tweening demos
    comp.instrument("pad", &Instrument::warm_pad())
        .notes(&[C3, E3, G3], 10.0); // Long sustained chord

    let mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;

    // Demo 1: Volume Tweening (Fade In/Out)
    println!("Demo 1: Volume Tweening");
    println!("------------------------");
    let id = engine.play_looping(&mixer)?;

    engine.set_volume(id, 0.0)?; // Start silent
    println!("  Starting silent...");
    sleep(Duration::from_secs(1));

    println!("  Fading in to 80% over 3 seconds...");
    engine.fade_in(id, 3.0, 0.8)?;
    sleep(Duration::from_secs(4));

    println!("  Fading out to 0% over 2 seconds...");
    engine.fade_out(id, 2.0)?;
    sleep(Duration::from_secs(3));

    engine.stop(id)?;
    println!("  ✓ Volume tweening complete\n");

    // Demo 2: Pan Tweening (Left-to-Right Movement)
    println!("Demo 2: Pan Tweening");
    println!("--------------------");
    let id = engine.play_looping(&mixer)?;

    engine.set_pan(id, -1.0)?; // Start at full left
    println!("  Starting at full left (-1.0)...");
    sleep(Duration::from_secs(1));

    println!("  Panning to full right (1.0) over 4 seconds...");
    engine.tween_pan(id, 1.0, 4.0)?;
    sleep(Duration::from_secs(5));

    println!("  Panning back to center (0.0) over 2 seconds...");
    engine.tween_pan(id, 0.0, 2.0)?;
    sleep(Duration::from_secs(3));

    engine.stop(id)?;
    println!("  ✓ Pan tweening complete\n");

    // Demo 3: Playback Rate Tweening (Pitch/Speed Ramping)
    println!("Demo 3: Playback Rate Tweening");
    println!("-------------------------------");
    let id = engine.play_looping(&mixer)?;

    println!("  Starting at normal speed (1.0)...");
    sleep(Duration::from_secs(1));

    println!("  Ramping up to 2x speed/pitch over 3 seconds...");
    engine.tween_playback_rate(id, 2.0, 3.0)?;
    sleep(Duration::from_secs(4));

    println!("  Ramping down to 0.5x speed/pitch over 3 seconds...");
    engine.tween_playback_rate(id, 0.5, 3.0)?;
    sleep(Duration::from_secs(4));

    println!("  Returning to normal (1.0) over 2 seconds...");
    engine.tween_playback_rate(id, 1.0, 2.0)?;
    sleep(Duration::from_secs(3));

    engine.stop(id)?;
    println!("  ✓ Playback rate tweening complete\n");

    // Demo 4: Combined Tweening (Multiple Parameters)
    println!("Demo 4: Combined Tweening");
    println!("-------------------------");
    let id = engine.play_looping(&mixer)?;

    println!("  Simultaneously tweening volume, pan, and playback rate...");
    engine.set_volume(id, 0.0)?;
    engine.set_pan(id, -1.0)?;
    engine.set_playback_rate(id, 0.5)?;

    sleep(Duration::from_secs(1));

    println!("  Volume: 0.0 → 1.0 (4s)");
    println!("  Pan: -1.0 → 1.0 (4s)");
    println!("  Rate: 0.5 → 2.0 (4s)");

    engine.fade_in(id, 4.0, 1.0)?;
    engine.tween_pan(id, 1.0, 4.0)?;
    engine.tween_playback_rate(id, 2.0, 4.0)?;

    sleep(Duration::from_secs(5));

    engine.stop(id)?;
    println!("  ✓ Combined tweening complete\n");

    println!("=== Demo Complete ===");
    println!("\nTweening Features Summary:");
    println!("  ✓ Volume tweening (fade_in, fade_out)");
    println!("  ✓ Pan tweening (tween_pan)");
    println!("  ✓ Playback rate tweening (tween_playback_rate)");
    println!("  ✓ All tweens use linear interpolation");
    println!("  ✓ Multiple tweens can run simultaneously");
    println!("\nPerfect for:");
    println!("  - Smooth audio transitions");
    println!("  - Dynamic game audio (engine sounds, doppler effects)");
    println!("  - Spatial audio movement (helicopter flyby, car passing)");
    println!("  - Time-based effects (slow motion, acceleration)");

    Ok(())
}
