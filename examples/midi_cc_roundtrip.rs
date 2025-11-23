/// Example demonstrating MIDI CC message import/export round-trip
///
/// This example:
/// 1. Creates a composition with track volume and pan settings
/// 2. Exports to MIDI (CC7 volume, CC10 pan)
/// 3. Imports the MIDI file
/// 4. Re-exports to verify CC values are preserved
///
/// Supported CCs:
/// - CC7: Volume (0-127)
/// - CC10: Pan (0-127, 64=center)
/// - CC11: Expression (treated as volume)

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("=== MIDI CC Round-Trip Test ===");
    println!();

    // Step 1: Create composition with different volumes and pans
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.track("Piano")
        .volume(0.8) // Will be exported as CC7 = 102
        .pan(-0.5) // Will be exported as CC10 = 32 (left)
        .notes(&[C4, E4, G4, C5], 0.5);

    comp.track("Bass")
        .volume(0.6) // Will be exported as CC7 = 76
        .pan(0.0) // Will be exported as CC10 = 64 (center)
        .note(&[C3], 2.0);

    comp.track("Lead")
        .volume(0.9) // Will be exported as CC7 = 114
        .pan(0.5) // Will be exported as CC10 = 96 (right)
        .wait(1.0)
        .notes(&[C5, D5, E5, F5], 0.25);

    // Get mixer for export
    let original_mixer = comp.into_mixer();

    println!("📤 Original track settings:");
    for (i, track) in original_mixer.all_tracks().iter().enumerate() {
        println!(
            "  Track {}: {} - Volume: {:.2}, Pan: {:.2}",
            i + 1,
            track.name.as_deref().unwrap_or("Unnamed"),
            track.volume,
            track.pan
        );
    }
    println!();

    // Step 2: Export to MIDI
    println!("💾 Exporting to cc_test.mid...");
    original_mixer.export_midi("cc_test.mid")?;
    println!("   ✓ Exported with CC7 (volume) and CC10 (pan)");
    println!();

    // Step 3: Import the MIDI file
    println!("📥 Importing cc_test.mid...");
    let imported_mixer = Mixer::import_midi("cc_test.mid")?;
    println!("   ✓ Imported {} tracks", imported_mixer.all_tracks().len());
    println!();

    println!("📊 Imported track settings:");
    for (i, track) in imported_mixer.all_tracks().iter().enumerate() {
        println!(
            "  Track {}: {} - Volume: {:.2}, Pan: {:.2}",
            i + 1,
            track.name.as_deref().unwrap_or("Unnamed"),
            track.volume,
            track.pan
        );
    }
    println!();

    // Step 4: Re-export to verify round-trip
    println!("💾 Re-exporting to cc_test_roundtrip.mid...");
    imported_mixer.export_midi("cc_test_roundtrip.mid")?;
    println!("   ✓ Re-exported");
    println!();

    // Step 5: Verify CC values match (with small tolerance for quantization)
    println!("✅ Verification:");
    let mut all_match = true;
    for (i, (original, imported)) in original_mixer
        .all_tracks()
        .iter()
        .zip(imported_mixer.all_tracks().iter())
        .enumerate()
    {
        let volume_diff = (original.volume - imported.volume).abs();
        let pan_diff = (original.pan - imported.pan).abs();

        // Allow small tolerance due to MIDI quantization (127 steps)
        let volume_tolerance = 1.0 / 127.0; // ~0.008
        let pan_tolerance = 2.0 / 127.0; // ~0.016

        let volume_matches = volume_diff <= volume_tolerance;
        let pan_matches = pan_diff <= pan_tolerance;

        if volume_matches && pan_matches {
            println!(
                "  ✓ Track {}: CC values preserved",
                i + 1
            );
        } else {
            println!(
                "  ✗ Track {}: Mismatch - Volume diff: {:.4}, Pan diff: {:.4}",
                i + 1,
                volume_diff,
                pan_diff
            );
            all_match = false;
        }
    }
    println!();

    if all_match {
        println!("🎉 Success! All CC values preserved in round-trip.");
        println!();
        println!("Notes:");
        println!("  - CC7 (Volume) and CC10 (Pan) are exported and imported");
        println!("  - Small differences (<1 MIDI step) are expected due to quantization");
        println!("  - Import both MIDI files in a DAW to verify visually");
    } else {
        println!("⚠️  Some CC values did not match (within tolerance)");
    }

    Ok(())
}
