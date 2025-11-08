use tunes::prelude::*;
use tunes::theory::core::ChordPattern;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(100.0));

    println!("Voicing & Voice Leading Demo\n");

    // ========================================================================
    // PART 1: Chord Inversions
    // ========================================================================
    println!("Part 1: Chord Inversions (Root, 1st, 2nd)");

    comp.section("inversions")
        .instrument("piano", &Instrument::electric_piano())
        // Root position (C-E-G)
        .chord(C4, &ChordPattern::MAJOR, 1.0)
        .wait(0.5)
        // First inversion (E-G-C)
        .chord_inverted(C4, &ChordPattern::MAJOR, 1, 1.0)
        .wait(0.5)
        // Second inversion (G-C-E)
        .chord_inverted(C4, &ChordPattern::MAJOR, 2, 1.0)
        .wait(0.5);

    // ========================================================================
    // PART 2: Smooth Voice Leading (I-V-vi-IV progression)
    // ========================================================================
    println!("Part 2: Smooth Voice Leading (I-V-vi-IV)");

    // Without voice leading (jumpy)
    comp.section("without_voice_leading")
        .instrument("strings_jumpy", &Instrument::synth_lead())
        .volume(0.6)
        .chord(C4, &ChordPattern::MAJOR, 1.5)  // I
        .chord(G4, &ChordPattern::MAJOR, 1.5)  // V (jumps to high G)
        .chord(A4, &ChordPattern::MINOR, 1.5)  // vi (jumps to high A)
        .chord(F4, &ChordPattern::MAJOR, 1.5); // IV (jumps back down)

    // With smooth voice leading (using chord_voice_lead)
    comp.section("with_voice_leading")
        .instrument("strings_smooth", &Instrument::synth_lead())
        .volume(0.6)
        .chord(C4, &ChordPattern::MAJOR, 1.5)        // I (first chord - no voice leading)
        .chord_voice_lead(G4, &ChordPattern::MAJOR, 1.5)  // V (smoothly voice led)
        .chord_voice_lead(A4, &ChordPattern::MINOR, 1.5)  // vi (smoothly voice led)
        .chord_voice_lead(F4, &ChordPattern::MAJOR, 1.5); // IV (smoothly voice led)

    // ========================================================================
    // PART 3: Slash Chords (C/E, F/A, etc.)
    // ========================================================================
    println!("Part 3: Slash Chords (Bass Note Changes)");

    comp.section("slash_chords")
        .instrument("bass_chords", &Instrument::electric_piano())
        .volume(0.7)
        // C major (regular root position)
        .chord(C4, &ChordPattern::MAJOR, 1.0)
        .wait(0.25)
        // C/E (C major over E bass)
        .chord_over_bass(C4, &ChordPattern::MAJOR, E3, 1.0)
        .wait(0.25)
        // C/G (C major over G bass)
        .chord_over_bass(C4, &ChordPattern::MAJOR, G3, 1.0)
        .wait(0.25);

    // ========================================================================
    // PART 4: Close vs Open Voicing
    // ========================================================================
    println!("Part 4: Close vs Open Voicings");

    // Note: close_voicing and open_voicing are transformations, not chord builders
    // So we still import them from theory::core for advanced voicing control
    use tunes::theory::core::{chord, close_voicing, open_voicing};

    let dominant7 = chord(G4, &ChordPattern::DOMINANT7); // G7

    comp.section("voicings")
        .instrument("voicing_demo", &Instrument::electric_piano())
        // Wide spacing (original)
        .note(&dominant7, 1.5)
        .wait(0.5)
        // Close voicing (all within one octave)
        .note(&close_voicing(&dominant7), 1.5)
        .wait(0.5)
        // Open voicing (drop-2)
        .note(&open_voicing(&dominant7), 1.5)
        .wait(0.5);

    // ========================================================================
    // PART 5: Jazz Voicing Example (ii-V-I)
    // ========================================================================
    println!("Part 5: Jazz ii-V-I Progression with Voice Leading");

    comp.section("jazz_progression")
        .instrument("jazz_piano", &Instrument::electric_piano())
        .volume(0.7)
        .reverb(Reverb::new(0.3, 0.5, 0.2))
        .chord(D4, &ChordPattern::MINOR7, 2.0)              // ii7
        .chord_voice_lead(G4, &ChordPattern::DOMINANT7, 2.0) // V7 (voice led)
        .chord_voice_lead(C4, &ChordPattern::MAJOR7, 3.0);   // Imaj7 (voice led)

    // ========================================================================
    // Arrange and play
    // ========================================================================
    comp.arrange(&[
        "inversions",
        "without_voice_leading",
        "with_voice_leading",
        "slash_chords",
        "voicings",
        "jazz_progression",
    ]);

    println!("\nPlaying voicing and voice leading examples...");
    engine.play_mixer(&comp.into_mixer())?;

    println!("\nDone! Notice:");
    println!("- Inversions change which note is in the bass");
    println!("- Voice leading makes progressions sound smoother");
    println!("- Slash chords create interesting bass movement");
    println!("- Different voicings change the character of chords");

    Ok(())
}
