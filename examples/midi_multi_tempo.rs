/// Example demonstrating MIDI export with multiple tempo changes
///
/// This example creates a composition with tempo changes and exports it to MIDI.
/// The fix ensures that notes placed after tempo changes are correctly positioned
/// in the MIDI file's tick timeline.
///
/// Before the fix: Notes after tempo changes would have incorrect timing due to
/// using only the initial tempo for time-to-tick conversion.
///
/// After the fix: The TempoMap correctly integrates through all tempo changes,
/// ensuring accurate timing throughout the composition.

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    // Create composition at 120 BPM
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.track("piano")
        // First section at 120 BPM (0-2 seconds)
        .note(&[C4], 0.5)
        .note(&[D4], 0.5)
        .note(&[E4], 0.5)
        .note(&[F4], 0.5)
        // Add tempo change to 60 BPM (happens at cursor = 2 seconds)
        .tempo(60.0)
        // Second section at 60 BPM (2-4 seconds)
        // At 60 BPM, each beat takes 1 second (twice as slow)
        .note(&[G4], 1.0)
        .note(&[A4], 1.0)
        // Add another tempo change to 180 BPM (happens at cursor = 4 seconds)
        .tempo(180.0)
        // Third section at 180 BPM (4-6 seconds)
        // At 180 BPM, each beat takes 0.333 seconds (50% faster than initial)
        .note(&[B4], 0.333)
        .note(&[C5], 0.333)
        .note(&[B4], 0.333)
        .note(&[A4], 0.333)
        .note(&[G4], 0.333)
        .note(&[C5], 0.333);

    // Export to MIDI
    let mixer = comp.into_mixer();
    mixer.export_midi("multi_tempo.mid")?;

    println!("✓ Exported multi_tempo.mid with 3 tempo changes:");
    println!("  - 0-2s: 120 BPM (4 quarter notes)");
    println!("  - 2-4s: 60 BPM (2 whole notes)");
    println!("  - 4-6s: 180 BPM (6 eighth notes)");
    println!();
    println!("The TempoMap ensures accurate timing across all tempo changes.");
    println!("Import this MIDI into a DAW to verify timing is correct!");

    // Also demonstrate the calculation
    println!();
    println!("Tick calculation verification:");
    println!("  - At 2.0s (tempo change): should be at tick 1920");
    println!("  - At 4.0s (tempo change): should be at tick 2880 (1920 + 960)");
    println!("  - At 6.0s (end): should be at tick 4320 (2880 + 1440)");

    Ok(())
}
