/// Example demonstrating MIDI pitch bend import/export round-trip
///
/// This example:
/// 1. Creates notes with pitch bend (microtonal/expressive content)
/// 2. Exports to MIDI
/// 3. Imports the MIDI file
/// 4. Verifies pitch bend values are preserved
///
/// MIDI pitch bend:
/// - 14-bit value (0-16383, center=8192)
/// - Standard range: ±2 semitones
/// - Quantization: ~0.024 cents per step

use tunes::prelude::*;
use tunes::track::AudioEvent;

fn main() -> anyhow::Result<()> {
    println!("=== MIDI Pitch Bend Round-Trip Test ===");
    println!();

    // Step 1: Create composition with pitch-bent notes
    let mut comp = Composition::new(Tempo::new(120.0));

    // Notes with various pitch bends
    comp.track("Microtonal")
        .bend(0.5) // +0.5 semitones (quarter tone up)
        .note(&[C4], 1.0)
        .bend(-0.5) // -0.5 semitones (quarter tone down)
        .note(&[D4], 1.0)
        .bend(1.0) // +1.0 semitones (half step up)
        .note(&[E4], 1.0)
        .bend(-1.0) // -1.0 semitones (half step down)
        .note(&[F4], 1.0)
        .bend(0.0) // No bend (center)
        .note(&[G4], 1.0);

    let original_mixer = comp.into_mixer();

    println!("📤 Original pitch bend values:");
    for (i, track) in original_mixer.all_tracks().iter().enumerate() {
        println!("  Track {}:", i + 1);
        for (j, event) in track.events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                println!(
                    "    Note {}: {:.3} semitones",
                    j + 1,
                    note.pitch_bend_semitones
                );
            }
        }
    }
    println!();

    // Step 2: Export to MIDI
    println!("💾 Exporting to pitch_bend_test.mid...");
    original_mixer.export_midi("pitch_bend_test.mid")?;
    println!("   ✓ Exported with pitch bend messages");
    println!();

    // Step 3: Import the MIDI file
    println!("📥 Importing pitch_bend_test.mid...");
    let imported_mixer = Mixer::import_midi("pitch_bend_test.mid")?;
    println!("   ✓ Imported {} tracks", imported_mixer.all_tracks().len());
    println!();

    println!("📊 Imported pitch bend values:");
    for (i, track) in imported_mixer.all_tracks().iter().enumerate() {
        println!("  Track {}:", i + 1);
        for (j, event) in track.events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                println!(
                    "    Note {}: {:.3} semitones",
                    j + 1,
                    note.pitch_bend_semitones
                );
            }
        }
    }
    println!();

    // Step 4: Verify pitch bend values match
    println!("✅ Verification:");
    let mut all_match = true;
    let mut note_idx = 0;

    for track in original_mixer.all_tracks().iter() {
        for event in &track.events {
            if let AudioEvent::Note(original_note) = event {
                // Find corresponding imported note
                if let Some(imported_track) = imported_mixer.all_tracks().first() {
                    if let Some(AudioEvent::Note(imported_note)) =
                        imported_track.events.get(note_idx)
                    {
                        let bend_diff =
                            (original_note.pitch_bend_semitones
                                - imported_note.pitch_bend_semitones)
                                .abs();

                        // MIDI pitch bend is 14-bit (0-16383), so quantization is:
                        // Range: ±2 semitones = 4 semitones total
                        // Resolution: 4 / 16384 ≈ 0.000244 semitones per step
                        // Allow 1-step tolerance
                        let tolerance = 0.001; // ~4 MIDI steps

                        if bend_diff <= tolerance {
                            println!(
                                "  ✓ Note {}: Pitch bend preserved ({:.3} semitones)",
                                note_idx + 1,
                                imported_note.pitch_bend_semitones
                            );
                        } else {
                            println!(
                                "  ✗ Note {}: Mismatch - Expected {:.3}, Got {:.3} (diff: {:.6})",
                                note_idx + 1,
                                original_note.pitch_bend_semitones,
                                imported_note.pitch_bend_semitones,
                                bend_diff
                            );
                            all_match = false;
                        }
                    }
                }
                note_idx += 1;
            }
        }
    }
    println!();

    if all_match {
        println!("🎉 Success! All pitch bend values preserved in round-trip.");
        println!();
        println!("Notes:");
        println!("  - Pitch bend uses 14-bit MIDI values (very high resolution)");
        println!("  - Standard range is ±2 semitones");
        println!("  - Microtonal and expressive content is fully preserved");
        println!("  - Import the MIDI file in a DAW to verify pitch bend works");
    } else {
        println!("⚠️  Some pitch bend values did not match (within tolerance)");
    }

    Ok(())
}
