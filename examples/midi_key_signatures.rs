/// Example demonstrating MIDI export with key signature changes
///
/// This example creates a composition with key signature changes and exports it to MIDI.
/// Key signatures are exported as MIDI meta events that are recognized by DAWs
/// and notation software.
///
/// MIDI key signature format:
/// - Sharps/flats: -7 to +7 (negative = flats, positive = sharps)
/// - Major/minor: false = major, true = minor

use tunes::prelude::*;
use tunes::theory::key_signature::{KeyMode, KeyRoot, KeySignature};

fn main() -> anyhow::Result<()> {
    // Create composition at 120 BPM
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.track("melody")
        // Start in C Major (no sharps or flats)
        .key_signature(KeySignature::new(KeyRoot::C, KeyMode::Major))
        .notes(&[C4, D4, E4, F4], 0.5)
        // Modulate to G Major (1 sharp: F#)
        .key_signature(KeySignature::new(KeyRoot::G, KeyMode::Major))
        .notes(&[G4, A4, B4, C5], 0.5)
        // Modulate to D Major (2 sharps: F#, C#)
        .key_signature(KeySignature::new(KeyRoot::D, KeyMode::Major))
        .notes(&[D5, E5, FS5, G5], 0.5)
        // Modulate to A minor (no sharps or flats, relative to C Major)
        .key_signature(KeySignature::new(KeyRoot::A, KeyMode::Minor))
        .notes(&[A4, B4, C5, D5], 0.5)
        // Modulate to F Major (1 flat: Bb)
        .key_signature(KeySignature::new(KeyRoot::F, KeyMode::Major))
        .notes(&[F4, G4, A4, BB4], 0.5)
        // Modulate to Bb Major (2 flats: Bb, Eb)
        .key_signature(KeySignature::new(KeyRoot::Bf, KeyMode::Major))
        .notes(&[BB4, C5, D5, EB5], 0.5);

    // Export to MIDI
    let mixer = comp.into_mixer();
    mixer.export_midi("key_signatures.mid")?;

    println!("✓ Exported key_signatures.mid with key signature changes:");
    println!("  - C Major (0 sharps/flats)");
    println!("  - G Major (1 sharp: F#)");
    println!("  - D Major (2 sharps: F#, C#)");
    println!("  - A minor (0 sharps/flats)");
    println!("  - F Major (1 flat: Bb)");
    println!("  - Bb Major (2 flats: Bb, Eb)");
    println!();
    println!("Import this MIDI into a DAW or notation software to verify");
    println!("key signatures are displayed correctly!");
    println!();
    println!("Note: Modal key signatures (Dorian, Phrygian, etc.) are also");
    println!("supported and will be exported using their parent major scale's");
    println!("key signature, which is the standard MIDI convention.");

    Ok(())
}
