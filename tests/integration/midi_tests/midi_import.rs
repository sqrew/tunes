use tunes::prelude::*;

/// Demonstrate MIDI file import
///
/// This example shows how to import MIDI files and work with them in tunes.
/// It creates a test MIDI file, imports it, and demonstrates various workflows.
fn main() -> anyhow::Result<()> {
    println!("\n🎹 MIDI Import Demo\n");

    // First, let's create a simple MIDI file to import
    println!("=== Creating Test MIDI File ===\n");
    create_test_midi_file()?;
    println!("✅ Created 'test_import.mid'\n");

    // Import the MIDI file
    println!("=== Importing MIDI File ===\n");
    println!("Loading 'test_import.mid'...");
    let mut mixer = Mixer::import_midi("test_import.mid")?;
    println!("✅ MIDI file imported successfully!\n");

    // Display information about the imported MIDI
    let tracks = mixer.all_tracks();
    println!("MIDI Import Details:");
    println!("  • Tempo: {:.1} BPM", mixer.tempo.bpm);
    println!("  • Number of tracks: {}", tracks.len());
    println!("  • Total duration: {:.2} seconds\n", mixer.total_duration());

    // Show track details
    for (i, track) in tracks.iter().enumerate() {
        let track_name = track.name.as_deref().unwrap_or("Untitled");
        let num_events = track.events.len();
        println!(
            "  Track {}: '{}' ({} events)",
            i + 1,
            track_name,
            num_events
        );
    }

    println!("\n=== Workflow Examples ===\n");

    // Workflow 1: Play the imported MIDI
    println!("1. Play imported MIDI (commented out - uncomment to hear)");
    println!("   let engine = AudioEngine::new()?;");
    println!("   engine.play_mixer(&mixer)?;\n");
    // Uncomment to play:
    // let engine = AudioEngine::new()?;
    // engine.play_mixer(&mixer)?;

    // Workflow 2: Export to WAV
    println!("2. Export imported MIDI to WAV");
    println!("   Exporting to 'imported.wav'...");
    mixer.export_wav("imported.wav", 44100)?;
    println!("   ✅ Exported to 'imported.wav'\n");

    // Workflow 3: Re-export to MIDI (round-trip test)
    println!("3. Re-export to MIDI (round-trip)");
    println!("   Exporting to 'reimported.mid'...");
    mixer.export_midi("reimported.mid")?;
    println!("   ✅ Exported to 'reimported.mid'\n");

    // Workflow 4: Import, modify, and export
    println!("4. Import MIDI, modify it, and export");
    println!("   (This would require converting Mixer back to Composition)");
    println!("   Currently not directly supported, but you can:");
    println!("   - Import MIDI to get note data");
    println!("   - Create new Composition with similar structure");
    println!("   - Apply effects, change instruments, etc.");
    println!("   - Export to WAV or MIDI\n");

    println!("=== Use Cases for MIDI Import ===\n");
    println!("• Load existing MIDI files for audio rendering");
    println!("• Convert MIDI to WAV");
    println!("• Use MIDI as composition starting points");
    println!("• Analyze MIDI file structure");
    println!("• Round-trip testing (export → import → export)");
    println!("• Extract note data from MIDI for further processing\n");

    println!("=== Supported MIDI Features ===\n");
    println!("✅ Note events (pitch, duration, velocity)");
    println!("✅ Drum events (channel 10 → DrumType)");
    println!("✅ Multiple tracks with names");
    println!("✅ Tempo changes");
    println!("✅ Time signature changes");
    println!("✅ Program changes (stored as metadata)\n");

    println!("=== Limitations ===\n");
    println!("❌ Control Change (CC) events are ignored");
    println!("❌ Pitch bend is converted to static offsets");
    println!("❌ No polyphonic aftertouch");
    println!("❌ SMPTE timecode not supported (only PPQ timing)\n");

    Ok(())
}

/// Create a test MIDI file for demonstration
fn create_test_midi_file() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Melody track
    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5, G4, E4, C4], 0.5);

    // Bass track
    comp.instrument("bass", &Instrument::sub_bass())
        .at(0.0)
        .notes(&[C2, C2, G2, G2, C2, C2, G2, G2], 0.5);

    // Drum track
    comp.track("drums")
        .at(0.0)
        .drum_grid(16, 0.125)
        .sound(DrumType::Kick, &[0, 4, 8, 12])
        .sound(DrumType::Snare, &[4, 12])
        .sound(DrumType::HiHatClosed, &[0, 2, 4, 6, 8, 10, 12, 14]);

    // Export to MIDI
    let mixer = comp.into_mixer();
    mixer.export_midi("test_import.mid")?;

    Ok(())
}
