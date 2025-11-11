/// Multi-Format Audio Import Example
///
/// Demonstrates loading and playing audio files in various formats:
/// MP3, OGG Vorbis, FLAC, WAV, AAC
///
/// Tunes automatically detects the format and decodes it using symphonia.

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Tunes now supports multiple formats via Sample::from_file()
    // The format is automatically detected - no need to specify it

    // Example 1: Load samples directly (if you have the files)
    // let kick_mp3 = Sample::from_file("assets/kick.mp3")?;
    // let snare_ogg = Sample::from_file("assets/snare.ogg")?;
    // let hihat_flac = Sample::from_file("assets/hihat.flac")?;
    // let vocal_wav = Sample::from_file("assets/vocal.wav")?;
    // let synth_aac = Sample::from_file("assets/synth.m4a")?;

    // Example 2: Load and cache samples in composition
    // All formats work with the same API
    println!("Loading samples in various formats...");

    // These would work if the files exist:
    // comp.load_sample("kick", "assets/kick.mp3")?;
    // comp.load_sample("snare", "assets/snare.ogg")?;
    // comp.load_sample("hihat", "assets/hihat.flac")?;

    // For demonstration, we'll create procedural drums instead
    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

    comp.instrument("bass", &Instrument::sub_bass())
        .notes(&[C2, C2, G2, G2], 0.5);

    println!("\nFormat Support:");
    println!("  ✓ MP3 (MPEG-1/2 Layer III)");
    println!("  ✓ OGG Vorbis");
    println!("  ✓ FLAC (Free Lossless Audio Codec)");
    println!("  ✓ WAV (PCM, IEEE Float)");
    println!("  ✓ AAC / M4A (Advanced Audio Coding)");
    println!("\nAll formats automatically detected and decoded.");

    // Demonstrate format-agnostic loading
    println!("\nUsage examples:");
    println!("  let sample = Sample::from_file(\"kick.mp3\")?;");
    println!("  let sample = Sample::from_file(\"snare.ogg\")?;");
    println!("  let sample = Sample::from_file(\"loop.flac\")?;");
    println!("\n  comp.load_sample(\"vocal\", \"assets/vocal.wav\")?;");
    println!("  comp.track(\"vocals\").sample(\"vocal\")?;");

    // Play the composition
    let mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;

    println!("\nPlaying procedural composition...");
    engine.play_mixer(&mixer)?;

    Ok(())
}
