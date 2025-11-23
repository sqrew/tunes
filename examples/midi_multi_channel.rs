/// Example demonstrating MIDI export with multiple channels
///
/// This example creates a composition with multiple melodic instruments plus drums.
/// Each melodic track is automatically assigned to a separate MIDI channel (0-8, 10-15).
/// Drums are always assigned to channel 10 (MIDI standard).
///
/// Channel allocation:
/// - Melodic tracks: Channels 1-9, 11-16 (0-8, 10-15 in 0-indexed)
/// - Drums: Channel 10 (9 in 0-indexed)
/// - If more than 15 melodic tracks: Channels wrap around (shared)

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Track 1: Piano (will be assigned to channel 1/0)
    comp.track("Piano")
        .notes(&[C4, E4, G4, C5], 0.5)
        .notes(&[C4, E4, G4, C5], 0.5);

    // Track 2: Bass (will be assigned to channel 2/1)
    comp.track("Bass")
        .note(&[C3], 1.0)
        .note(&[G3], 1.0)
        .note(&[F3], 1.0)
        .note(&[G3], 1.0);

    // Track 3: Strings (will be assigned to channel 3/2)
    comp.track("Strings")
        .note(&[E4, G4], 2.0)
        .note(&[D4, F4], 2.0);

    // Track 4: Lead (will be assigned to channel 4/3)
    comp.track("Lead")
        .wait(2.0)
        .notes(&[C5, D5, E5, F5, G5, A5, B5, C6], 0.25);

    // Track 5: Pad (will be assigned to channel 5/4)
    comp.track("Pad").note(&[C4, E4, G4, B4], 4.0);

    // Track 6: Drums (will be assigned to channel 10/9)
    comp.track("Drums")
        .drum_grid(16, 0.25)
        .kick(&[0, 8])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14])
        .repeat(2);

    // Track 7: Another melodic instrument (will be assigned to channel 6/5)
    comp.track("Organ")
        .notes(&[C4, D4, E4, F4], 0.5)
        .notes(&[G4, A4, B4, C5], 0.5);

    // Export to MIDI
    let mixer = comp.into_mixer();
    mixer.export_midi("multi_channel.mid")?;

    println!("✓ Exported multi_channel.mid with automatic channel allocation:");
    println!();
    println!("Channel assignments (0-indexed):");
    println!("  - Track 1 (Piano):   Channel 0");
    println!("  - Track 2 (Bass):    Channel 1");
    println!("  - Track 3 (Strings): Channel 2");
    println!("  - Track 4 (Lead):    Channel 3");
    println!("  - Track 5 (Pad):     Channel 4");
    println!("  - Track 6 (Drums):   Channel 9 (drums channel)");
    println!("  - Track 7 (Organ):   Channel 5");
    println!();
    println!("Import this MIDI into a DAW and you'll see:");
    println!("  ✓ Each melodic track on a separate channel");
    println!("  ✓ Drums on channel 10 (standard)");
    println!("  ✓ You can assign different instruments to each channel");
    println!();
    println!("Note: MIDI supports up to 15 melodic channels (0-8, 10-15).");
    println!("If you have more tracks, they'll share channels (wrap around).");

    Ok(())
}
