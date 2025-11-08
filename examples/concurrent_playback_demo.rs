/// Demonstration of concurrent audio playback
///
/// This example shows the new concurrent mixing capabilities, where multiple
/// sounds can play simultaneously with independent volume and pan control.

use tunes::consts::*;
use tunes::composition::*;
use tunes::engine::AudioEngine;
use tunes::instruments::Instrument;

fn main() -> Result<(), anyhow::Error> {
    println!("=== Concurrent Playback Demo ===\n");

    let engine = AudioEngine::new()?;

    // Create three different compositions
    let mut drums = Composition::new(Tempo::new(120.0));
    drums
        .track("drums")
        .at(0.0).note(&[100.0], 0.1)  // Kick-like sound
        .at(0.5).note(&[300.0], 0.1)  // Snare-like sound
        .at(1.0).note(&[100.0], 0.1)
        .at(1.5).note(&[300.0], 0.1);

    let mut bass = Composition::new(Tempo::new(120.0));
    bass.instrument("bass", &Instrument::sub_bass())
        .note(&[A2], 0.25)
        .note(&[A2], 0.25)
        .note(&[C3], 0.25)
        .note(&[E3], 0.25)
        .note(&[D3], 0.5)
        .note(&[C3], 0.5);

    let mut melody = Composition::new(Tempo::new(120.0));
    melody
        .instrument("lead", &Instrument::synth_lead())
        .chords(&[C4_MAJOR], 1.0)
        .chords(&[F4_MAJOR], 1.0);

    println!("Playing three sounds concurrently...\n");

    // Trigger all three non-blocking - they play simultaneously!
    let drums_id = engine.play_mixer_realtime(&drums.into_mixer())?;
    let bass_id = engine.play_mixer_realtime(&bass.into_mixer())?;
    let melody_id = engine.play_mixer_realtime(&melody.into_mixer())?;

    println!("  Drums  ID: {}", drums_id);
    println!("  Bass   ID: {}", bass_id);
    println!("  Melody ID: {}", melody_id);
    println!();

    // Let them play for a bit
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Demonstrate real-time control
    println!("Reducing bass volume...");
    engine.set_volume(bass_id, 0.3)?;

    std::thread::sleep(std::time::Duration::from_millis(500));

    println!("Panning melody to the right...");
    engine.set_pan(melody_id, 0.7)?;

    // Wait for all sounds to finish
    while engine.is_playing(drums_id)
        || engine.is_playing(bass_id)
        || engine.is_playing(melody_id)
    {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("\n✓ All sounds finished!");
    println!("\nThis demonstrates:");
    println!("  • Multiple sounds playing simultaneously");
    println!("  • Non-blocking playback with SoundId");
    println!("  • Real-time volume control");
    println!("  • Real-time pan control");

    Ok(())
}
