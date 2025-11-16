/// Live Audio Recording Demo
///
/// Demonstrates recording from microphone/line-in and using the recording
/// in a composition with the full effects pipeline.
///
/// Run with: cargo run --example live_recording_demo --release
///
/// What this demo does:
/// 1. Records from your microphone for 5 seconds
/// 2. Saves the recording as a WAV file
/// 3. Uses the recording in a composition with reverb
/// 4. Exports the final result

use tunes::prelude::*;
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("🎙️  Live Audio Recording Demo\n");

    // Step 1: Record from microphone
    println!("=== Step 1: Recording from Microphone ===");
    println!("Make sure your microphone is connected and enabled.");
    println!("\nStarting recording in 3 seconds...");
    thread::sleep(Duration::from_secs(3));

    let mut recorder = LiveInput::new()?;
    recorder.start_recording("live_recording.wav")?;

    println!("\n🔴 RECORDING - Speak into your microphone!");
    println!("Recording for 5 seconds...\n");

    // Record for 5 seconds
    thread::sleep(Duration::from_secs(5));

    recorder.stop()?;
    println!("\n✅ Recording complete! Saved to: live_recording.wav");

    // Step 2: Use the recording in a composition
    println!("\n=== Step 2: Processing Recording ===");
    println!("Adding reverb and effects to your recording...\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // Load the recording into composition
    comp.load_sample("recording", "live_recording.wav")?;

    // Use the recording and add effects
    comp.track("vocal")
        .sample("recording")
        .reverb(Reverb::new(0.6, 0.5, 0.4));  // Add reverb (room_size, damping, mix)

    // Export the processed result
    println!("Exporting processed audio...");
    comp.into_mixer().export_wav("live_recording_processed.wav", 44100)?;

    println!("\n✅ Demo complete!");
    println!("\nFiles created:");
    println!("  • live_recording.wav          - Your raw recording");
    println!("  • live_recording_processed.wav - With reverb applied");
    println!("\n💡 Try editing this example to:");
    println!("  • Add more effects (delay, eq, etc.)");
    println!("  • Record longer durations");
    println!("  • Combine multiple recordings");
    println!("  • Layer your recording with instruments");

    Ok(())
}
