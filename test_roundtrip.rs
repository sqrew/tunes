use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("Testing MIDI round-trip...\n");
    
    // Import the MIDI file we just exported
    println!("1. Importing 'output.mid'...");
    let mut mixer = Mixer::import_midi("output.mid")?;
    println!("   ✓ Imported successfully");
    println!("   - Tracks: {}", mixer.tracks.len());
    println!("   - Duration: {:.2}s", mixer.total_duration());
    println!("   - Tempo: {:.1} BPM\n", mixer.tempo.bpm);
    
    // Show track details
    for (i, track) in mixer.tracks.iter().enumerate() {
        println!("   Track {}: {} ({} events)", 
            i + 1,
            track.name.as_deref().unwrap_or("Untitled"),
            track.events.len()
        );
    }
    
    // Re-export to test round-trip
    println!("\n2. Re-exporting to 'roundtrip.mid'...");
    mixer.export_midi("roundtrip.mid")?;
    println!("   ✓ Re-exported successfully\n");
    
    // Export to WAV to verify audio rendering works
    println!("3. Exporting to WAV 'roundtrip.wav'...");
    mixer.export_wav("roundtrip.wav", 44100)?;
    println!("   ✓ WAV export successful\n");
    
    println!("✅ Round-trip test passed!");
    
    Ok(())
}
