use tunes::prelude::*;

/// Convert MIDI files to FLAC
///
/// This example demonstrates the full workflow: create a composition,
/// export to MIDI, import the MIDI, and export to FLAC.
fn main() -> anyhow::Result<()> {
    println!("\n🎼 MIDI → FLAC Conversion Demo\n");

    // Step 1: Create a composition
    println!("=== Step 1: Create Composition ===\n");
    let mut comp = Composition::new(Tempo::new(140.0));

    comp.instrument("lead", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5, G4, E4, C4], 0.25);

    comp.instrument("bass", &Instrument::sub_bass())
        .at(0.0)
        .notes(&[C2, G2, C2, G2], 0.5);

    comp.track("drums")
        .at(0.0)
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    println!("✓ Created composition with 3 tracks\n");

    // Step 2: Export to MIDI
    println!("=== Step 2: Export to MIDI ===\n");
    let mixer = comp.into_mixer();
    mixer.export_midi("demo.mid")?;
    println!("✓ Exported to 'demo.mid'\n");

    // Step 3: Import the MIDI file
    println!("=== Step 3: Import MIDI File ===\n");
    let mut imported_mixer = Mixer::import_midi("demo.mid")?;
    println!("✓ Imported 'demo.mid'");
    println!("  - Tracks: {}", imported_mixer.all_tracks().len());
    println!("  - Duration: {:.2}s", imported_mixer.total_duration());
    println!("  - Tempo: {:.1} BPM\n", imported_mixer.tempo.bpm);

    // Step 4: Export to FLAC
    println!("=== Step 4: Export to FLAC ===\n");
    imported_mixer.export_flac("demo.flac", 44100)?;

    // Step 5: Also export to WAV for comparison
    println!("\n=== Step 5: Export to WAV (for comparison) ===\n");
    imported_mixer.export_wav("demo.wav", 44100)?;

    // Compare file sizes
    use std::fs;
    let midi_size = fs::metadata("demo.mid")?.len();
    let flac_size = fs::metadata("demo.flac")?.len();
    let wav_size = fs::metadata("demo.wav")?.len();

    println!("\n=== File Size Comparison ===\n");
    println!("  MIDI: {:>8} bytes (notes only, no audio)", midi_size);
    println!("  WAV:  {:>8} bytes (uncompressed audio)", wav_size);
    println!("  FLAC: {:>8} bytes (lossless compressed)", flac_size);

    let compression_ratio = 100.0 - (flac_size as f64 / wav_size as f64 * 100.0);
    println!("\n  FLAC saves {:.1}% compared to WAV", compression_ratio);

    println!("\n=== Workflow Summary ===\n");
    println!("✅ Created composition in tunes");
    println!("✅ Exported to MIDI (portable, editable format)");
    println!("✅ Imported MIDI back into tunes");
    println!("✅ Exported to FLAC (high-quality audio, 50-60% smaller)");
    println!("✅ All files created successfully!");

    println!("\n=== Use Cases ===\n");
    println!("• Convert MIDI libraries to lossless audio");
    println!("• Create archival versions of MIDI compositions");
    println!("• Share MIDI compositions as high-quality audio");
    println!("• Build MIDI → audio conversion pipelines");

    Ok(())
}
