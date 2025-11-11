/// Streaming Audio Demo - Memory-Efficient File Playback
///
/// Demonstrates Tunes' streaming audio capabilities for playing long audio files
/// without loading them entirely into memory. Perfect for:
/// - Background music
/// - Ambient sounds
/// - Voice-over narration
/// - Any long-form audio content

use tunes::prelude::*;
use std::thread::sleep;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("=== Tunes Streaming Audio Demo ===\n");

    // First, create a test audio file to stream
    println!("Creating test audio file for streaming demo...");
    create_test_audio_file()?;
    println!("  ✓ Created test_stream.wav (30 seconds of audio)\n");

    let engine = AudioEngine::new()?;

    // Demo 1: Basic Streaming
    println!("Demo 1: Basic Streaming");
    println!("-----------------------");
    println!("  Streaming test_stream.wav...");
    let stream_id = engine.stream_file("test_stream.wav")?;

    sleep(Duration::from_secs(5));
    println!("  Playing for 5 seconds...");

    engine.stop_stream(stream_id)?;
    println!("  ✓ Stopped stream\n");

    // Demo 2: Looping Stream
    println!("Demo 2: Looping Stream");
    println!("----------------------");
    println!("  Streaming in loop mode...");
    let loop_id = engine.stream_file_looping("test_stream.wav")?;

    sleep(Duration::from_secs(3));
    println!("  Looping for 3 seconds...");

    engine.stop_stream(loop_id)?;
    println!("  ✓ Stopped looping stream\n");

    // Demo 3: Volume Control
    println!("Demo 3: Volume Control");
    println!("----------------------");
    let volume_id = engine.stream_file("test_stream.wav")?;

    println!("  Starting at full volume...");
    sleep(Duration::from_secs(2));

    println!("  Reducing to 50% volume...");
    engine.set_stream_volume(volume_id, 0.5)?;
    sleep(Duration::from_secs(2));

    println!("  Reducing to 25% volume...");
    engine.set_stream_volume(volume_id, 0.25)?;
    sleep(Duration::from_secs(2));

    engine.stop_stream(volume_id)?;
    println!("  ✓ Volume control complete\n");

    // Demo 4: Pan Control
    println!("Demo 4: Pan Control");
    println!("-------------------");
    let pan_id = engine.stream_file("test_stream.wav")?;

    println!("  Starting at center...");
    sleep(Duration::from_secs(2));

    println!("  Panning to full left...");
    engine.set_stream_pan(pan_id, -1.0)?;
    sleep(Duration::from_secs(2));

    println!("  Panning to full right...");
    engine.set_stream_pan(pan_id, 1.0)?;
    sleep(Duration::from_secs(2));

    println!("  Back to center...");
    engine.set_stream_pan(pan_id, 0.0)?;
    sleep(Duration::from_secs(1));

    engine.stop_stream(pan_id)?;
    println!("  ✓ Pan control complete\n");

    // Demo 5: Pause and Resume
    println!("Demo 5: Pause and Resume");
    println!("------------------------");
    let pause_id = engine.stream_file("test_stream.wav")?;

    println!("  Playing...");
    sleep(Duration::from_secs(2));

    println!("  Pausing...");
    engine.pause_stream(pause_id)?;
    sleep(Duration::from_secs(2));

    println!("  Resuming...");
    engine.resume_stream(pause_id)?;
    sleep(Duration::from_secs(2));

    engine.stop_stream(pause_id)?;
    println!("  ✓ Pause/resume complete\n");

    // Demo 6: Multiple Concurrent Streams
    println!("Demo 6: Multiple Concurrent Streams");
    println!("------------------------------------");
    println!("  Starting 3 streams simultaneously...");

    let stream1 = engine.stream_file("test_stream.wav")?;
    engine.set_stream_volume(stream1, 0.3)?;
    engine.set_stream_pan(stream1, -0.7)?;

    let stream2 = engine.stream_file("test_stream.wav")?;
    engine.set_stream_volume(stream2, 0.3)?;
    engine.set_stream_pan(stream2, 0.0)?;

    let stream3 = engine.stream_file("test_stream.wav")?;
    engine.set_stream_volume(stream3, 0.3)?;
    engine.set_stream_pan(stream3, 0.7)?;

    println!("  Playing 3 streams at different pan positions...");
    sleep(Duration::from_secs(3));

    engine.stop_stream(stream1)?;
    engine.stop_stream(stream2)?;
    engine.stop_stream(stream3)?;
    println!("  ✓ Multiple streams complete\n");

    // Cleanup
    std::fs::remove_file("test_stream.wav")?;

    println!("=== Demo Complete ===");
    println!("\nStreaming Audio Features Summary:");
    println!("  ✓ Basic streaming (stream_file)");
    println!("  ✓ Looping streams (stream_file_looping)");
    println!("  ✓ Volume control (set_stream_volume)");
    println!("  ✓ Pan control (set_stream_pan)");
    println!("  ✓ Pause/Resume (pause_stream, resume_stream)");
    println!("  ✓ Multiple concurrent streams");
    println!("\nMemory Benefits:");
    println!("  - No need to load entire file into RAM");
    println!("  - Perfect for long background music (3-10 minutes+)");
    println!("  - Decode on-the-fly in background thread");
    println!("  - Lock-free ring buffer for smooth playback");
    println!("\nSupported Formats:");
    println!("  MP3, OGG, FLAC, WAV, AAC (via symphonia)");

    Ok(())
}

/// Create a test audio file for the demo
fn create_test_audio_file() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Create a pleasant 30-second ambient sound
    comp.instrument("pad", &Instrument::warm_pad())
        .notes(&[C3, E3, G3], 10.0) // 10 seconds
        .notes(&[D3, F3, A3], 10.0) // 10 seconds
        .notes(&[E3, G3, B3], 10.0); // 10 seconds

    // Add some bass movement
    comp.instrument("bass", &Instrument::sub_bass())
        .filter(Filter::low_pass(120.0, 0.3))
        .notes(&[C2, C2, C2, C2], 2.5)
        .notes(&[D2, D2, D2, D2], 2.5)
        .notes(&[E2, E2, E2, E2], 2.5)
        .notes(&[C2, C2, C2, C2], 2.5)
        .notes(&[D2, D2, D2, D2], 2.5)
        .notes(&[E2, E2, E2, E2], 2.5)
        .notes(&[C2, C2, C2, C2], 2.5)
        .notes(&[D2, D2, D2, D2], 2.5)
        .notes(&[E2, E2, E2, E2], 2.5)
        .notes(&[C2, C2, C2, C2], 2.5)
        .notes(&[D2, D2, D2, D2], 2.5)
        .notes(&[E2, E2, E2, E2], 2.5);

    let mut mixer = comp.into_mixer();
    mixer.export_wav("test_stream.wav", 44100)?;

    Ok(())
}
