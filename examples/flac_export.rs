use tunes::prelude::*;

/// Demonstrate FLAC export
///
/// This example shows how to export compositions to FLAC format for lossless
/// compression. FLAC files are typically 50-60% smaller than WAV with no quality loss.
fn main() -> anyhow::Result<()> {
    println!("\n🎵 FLAC Export Demo\n");

    // Create a simple composition
    let mut comp = Composition::new(Tempo::new(120.0));

    // Add a melody
    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5, G4, E4, C4, C4], 0.5);

    // Add bass
    comp.instrument("bass", &Instrument::sub_bass())
        .at(0.0)
        .notes(&[C2, C2, G2, G2], 1.0);

    // Add drums
    comp.track("drums")
        .at(0.0)
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

    println!("=== Exporting to FLAC ===\n");

    let mut mixer = comp.into_mixer();

    // Export to FLAC (lossless compression)
    println!("Exporting to 'output.flac'...");
    mixer.export_flac("output.flac", 44100)?;

    println!("\n=== FLAC vs WAV Comparison ===\n");

    // Also export to WAV for comparison
    println!("Exporting to 'output.wav' for comparison...");
    mixer.export_wav("output.wav", 44100)?;

    // Compare file sizes
    use std::fs;
    let flac_size = fs::metadata("output.flac")?.len();
    let wav_size = fs::metadata("output.wav")?.len();

    let compression_ratio = 100.0 - (flac_size as f64 / wav_size as f64 * 100.0);

    println!("\nFile Size Comparison:");
    println!("  WAV:  {} bytes", wav_size);
    println!("  FLAC: {} bytes", flac_size);
    println!("  Space saved: {:.1}% ({} bytes)", compression_ratio, wav_size - flac_size);

    println!("\n=== FLAC Benefits ===\n");
    println!("✅ Lossless compression (~50-60% smaller than WAV)");
    println!("✅ Perfect audio quality (bit-perfect)");
    println!("✅ Widely supported by DAWs and audio tools");
    println!("✅ Great for archival and professional workflows");
    println!("✅ Faster uploads/downloads than WAV");
    println!("✅ Supports metadata (track info, album art, etc.)");

    println!("\n=== When to Use FLAC ===\n");
    println!("• Archiving compositions");
    println!("• Sharing high-quality audio online");
    println!("• Professional production workflows");
    println!("• When storage space matters");
    println!("• Final masters before distribution");

    println!("\n=== When to Use WAV Instead ===\n");
    println!("• Maximum compatibility (some old tools don't support FLAC)");
    println!("• Streaming applications (WAV has lower decode overhead)");
    println!("• Real-time processing (less CPU usage)");

    Ok(())
}
