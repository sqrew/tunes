use tunes::prelude::*;

/// Demonstrate notes and chords API
fn main() -> anyhow::Result<()> {
    println!("\n🎼 Example: Notes and Chords API\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Single notes played individually
    comp.track("single_notes")
        .at(0.0)
        .note(&[C4], 0.3)
        .note(&[E4], 0.3)
        .note(&[G4], 0.3);

    // Sequence of notes (more concise)
    comp.track("note_sequence")
        .at(1.2)
        .notes(&[C4, D4, E4, F4, G4], 0.2);

    // Chord progression
    comp.track("chord_progression")
        .at(2.5)
        .chords(&[C4_MAJOR, F4_MAJOR, G4_MAJOR, C4_MAJOR], 0.6);

    // Playing a single chord
    comp.track("single_chord")
        .at(5.0)
        .note(&[C4, E4, G4, C5], 1.0); // C major chord

    println!("✓ .note(&[freq], duration) - Single note or chord");
    println!("✓ .notes(&[...], duration) - Sequence of single notes");
    println!("✓ .chords(&[...], duration) - Sequence of chords");
    println!("✓ Use predefined chords from chords module\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
