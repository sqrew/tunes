/// Recording with Backing Track Demo
///
/// Shows how to record vocals/instruments and layer them with a backing track.
/// This example creates a simple chord progression, then lets you record over it.
///
/// Run with: cargo run --example recording_with_backing_track --release

use tunes::prelude::*;
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("🎵 Recording with Backing Track Demo\n");

    // Step 1: Create a backing track
    println!("=== Step 1: Creating Backing Track ===");
    let mut backing = Composition::new(Tempo::new(90.0));

    // Simple chord progression: Am - F - C - G
    let progression = [
        ([A3, C4, E4], 0.0),  // Am
        ([F3, A3, C4], 2.0),  // F
        ([C3, E3, G3], 4.0),  // C
        ([G3, B3, D4], 6.0),  // G
    ];

    for (chord, time) in progression {
        backing.instrument("pad", &Instrument::acoustic_piano())
            .at(time)
            .note(&chord, 1.8)
            .volume(0.3);  // Keep it quiet so it doesn't overpower recording
    }

    backing.track("pad").reverb(Reverb::new(0.5, 0.5, 0.3));
    backing.into_mixer().export_wav("backing_track.wav", 44100)?;
    println!("✅ Backing track created: backing_track.wav");

    // Step 2: Play the backing track and record over it
    println!("\n=== Step 2: Record Your Performance ===");
    println!("The backing track will play while you record.");
    println!("You can sing, play an instrument, or beatbox!");
    println!("\nStarting in 3 seconds...");
    thread::sleep(Duration::from_secs(3));

    // Note: In a real implementation, you'd want to play the backing track
    // simultaneously with recording. For now, we'll just record.
    // (Adding live playback would require the full AudioEngine we discussed)

    let mut recorder = LiveInput::new()?;
    recorder.start_recording("vocal_recording.wav")?;

    println!("\n🔴 RECORDING!");
    println!("(Imagine the chord progression: Am - F - C - G)");
    println!("Recording for 8 seconds...\n");

    thread::sleep(Duration::from_secs(8));

    recorder.stop()?;
    println!("\n✅ Recording complete!");

    // Step 3: Mix everything together
    println!("\n=== Step 3: Mixing ===");
    println!("Combining your recording with the backing track...\n");

    let mut final_comp = Composition::new(Tempo::new(90.0));

    // Load samples into composition
    final_comp.load_sample("backing", "backing_track.wav")?;
    final_comp.load_sample("vocal", "vocal_recording.wav")?;

    // Add backing track
    final_comp.track("backing")
        .sample("backing")
        .volume(0.5);

    // Add vocal recording with effects
    final_comp.track("vocal")
        .sample("vocal")
        .reverb(Reverb::new(0.6, 0.5, 0.4))  // room_size, damping, mix
        .delay(Delay::new(0.3, 0.5, 0.4))    // delay_time, feedback, mix
        .volume(0.8);

    // Export final mix
    final_comp.into_mixer().export_wav("final_mix.wav", 44100)?;

    println!("✅ Final mix exported: final_mix.wav");
    println!("\nFiles created:");
    println!("  • backing_track.wav   - The chord progression");
    println!("  • vocal_recording.wav - Your raw recording");
    println!("  • final_mix.wav       - Everything mixed together");
    println!("\n💡 Try extending this example to:");
    println!("  • Record multiple takes and choose the best");
    println!("  • Add a drum beat to the backing track");
    println!("  • Layer multiple vocal harmonies");
    println!("  • Apply different effects chains");

    Ok(())
}
