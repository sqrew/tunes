use tunes::prelude::*;

/// Demonstrate sample playback functionality
///
/// This example shows how to load and play WAV file samples,
/// including pitch shifting via playback rate changes.
fn main() -> anyhow::Result<()> {
    println!("\n🎵 Sample Playback Demo\n");

    // First, create a simple test WAV file (a 440Hz tone)
    println!("Creating test sample (440Hz tone)...");
    create_test_sample("test_tone.wav", 440.0, 0.5)?;
    println!("  ✓ Created test_tone.wav\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // Load the sample
    println!("Loading sample into composition...");
    comp.load_sample("tone", "test_tone.wav")?;
    println!("  ✓ Loaded 'tone' sample\n");

    // Example 1: Basic sample playback
    println!("Example 1: Basic Playback");
    comp.track("basic")
        .at(0.0)
        .sample("tone") // Play at normal speed
        .sample("tone") // Play again
        .sample("tone"); // And once more
    println!("  ✓ Playing sample 3 times\n");

    // Example 2: Pitch shifting with playback rate
    println!("Example 2: Pitch Shifting");
    comp.track("pitched")
        .at(2.0)
        .sample_with_rate("tone", 1.0) // Normal (A4 = 440Hz)
        .sample_with_rate("tone", 1.5) // 1.5x speed (≈ perfect fifth up)
        .sample_with_rate("tone", 2.0) // 2x speed (octave up = A5 = 880Hz)
        .sample_with_rate("tone", 0.5); // 0.5x speed (octave down = A3 = 220Hz)
    println!("  ✓ Playing with different pitch shifts\n");

    // Example 3: Rhythmic pattern
    println!("Example 3: Rhythmic Pattern");
    comp.track("rhythm")
        .at(5.0)
        .sample_with_rate("tone", 1.0)
        .sample_with_rate("tone", 1.25)
        .sample_with_rate("tone", 1.5)
        .sample_with_rate("tone", 2.0)
        .sample_with_rate("tone", 1.5)
        .sample_with_rate("tone", 1.25)
        .sample_with_rate("tone", 1.0)
        .sample_with_rate("tone", 0.75);
    println!("  ✓ Playing melodic pattern with samples\n");

    println!("=== Playback ===");
    let engine = AudioEngine::new()?;
    let mixer = comp.into_mixer();

    println!(
        "Playing {:.1}s composition with samples...\n",
        mixer.total_duration()
    );
    engine.play_mixer(&mixer)?;

    println!("\n=== Convenience Method Demo ===\n");
    println!("🎮 NEW: play_sample() - Fire-and-forget sound effects!");
    println!("\nBefore play_sample() existed:");
    println!("  let mut comp = Composition::new(Tempo::new(120.0));");
    println!("  comp.track(\"sfx\").sample(\"boom.wav\");");
    println!("  engine.play_mixer_realtime(&comp.into_mixer())?;");
    println!("\nNow you can simply write:");
    println!("  engine.play_sample(\"boom.wav\")?;");
    println!("\nPerfect for game sound effects - non-blocking and concurrent!\n");

    // Create a quick test sound effect
    create_test_sample("sfx_beep.wav", 880.0, 0.1)?;
    create_test_sample("sfx_boop.wav", 440.0, 0.1)?;

    println!("Example 4: Fire-and-forget SFX");
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Game-like scenario - rapid concurrent sound effects
    println!("  [Player jumps]");
    engine.play_sample("sfx_beep.wav")?; // Returns immediately!
    std::thread::sleep(std::time::Duration::from_millis(200));

    println!("  [Collects coin]");
    engine.play_sample("sfx_boop.wav")?;
    std::thread::sleep(std::time::Duration::from_millis(150));

    println!("  [Another coin]");
    engine.play_sample("sfx_boop.wav")?;
    std::thread::sleep(std::time::Duration::from_millis(150));

    println!("  [Three beeps simultaneously!]");
    engine.play_sample("sfx_beep.wav")?;
    engine.play_sample("sfx_beep.wav")?;
    engine.play_sample("sfx_beep.wav")?;

    std::thread::sleep(std::time::Duration::from_millis(500));

    // Optional: Keep the ID for control
    println!("\n  You can still control sounds if needed:");
    let sfx_id = engine.play_sample("sfx_boop.wav")?;
    engine.set_volume(sfx_id, 0.3)?;
    println!("  Playing at 30% volume...");

    std::thread::sleep(std::time::Duration::from_secs(1));

    println!("\n✅ Demo complete!");
    println!("\nKey features demonstrated:");
    println!("  • Loading WAV files as samples");
    println!("  • Basic sample playback");
    println!("  • Pitch shifting via playback_rate");
    println!("  • Creating rhythmic/melodic patterns with samples");
    println!("  • Fire-and-forget sound effects with play_sample()");
    println!("  • Concurrent sound playback (no blocking!)");
    println!("\nYou can now use your own drum samples, vocals, or any WAV files!");

    // Clean up test files
    std::fs::remove_file("test_tone.wav").ok();
    std::fs::remove_file("sfx_beep.wav").ok();
    std::fs::remove_file("sfx_boop.wav").ok();

    Ok(())
}

/// Helper function to create a test WAV file with a pure tone
fn create_test_sample(path: &str, frequency: f32, duration: f32) -> anyhow::Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec)?;
    let sample_rate = 44100.0;
    let num_samples = (duration * sample_rate) as usize;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate;
        let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin();

        // Apply envelope to avoid clicks
        let envelope = if t < 0.01 {
            t / 0.01 // 10ms fade-in
        } else if t > duration - 0.01 {
            (duration - t) / 0.01 // 10ms fade-out
        } else {
            1.0
        };

        let amplitude = (sample * envelope * 32767.0) as i16;
        writer.write_sample(amplitude)?;
    }

    writer.finalize()?;
    Ok(())
}
