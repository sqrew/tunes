use tunes::composition::Composition;
use tunes::engine::AudioEngine;
use tunes::instruments::Instrument;
use tunes::notes::*;
use tunes::rhythm::Tempo;
use std::error::Error;

/// Demonstrate musical time notation (bars, beats, quarters, eighths, etc.)
fn main() -> Result<(), Box<dyn Error>> {
    println!("🎵 Musical Time Notation Demo\n");
    println!("Using tempo-aware musical notation instead of raw seconds.\n");

    let mut comp = Composition::new(Tempo::new(120.0)); // 120 BPM

    // ===== 1. BAR AND BEAT POSITIONING =====
    println!("1. Bar and Beat Positioning");

    comp.instrument("bars_demo", &Instrument::acoustic_piano())
        .at_bar(1)  // Start at bar 1
        .quarters(&[C4, D4, E4, F4])
        .at_bar(2)  // Jump to bar 2
        .quarters(&[G4, A4, B4, C5]);

    comp.instrument("beats_demo", &Instrument::acoustic_piano())
        .at_beat(9)  // Beat 9 (bar 3, beat 1 in 4/4)
        .quarters(&[C5, B4, A4, G4]);

    // ===== 2. MUSICAL NOTE DURATIONS =====
    println!("\n2. Musical Note Durations - Quarter, Eighth, Sixteenth Notes");

    comp.instrument("quarters", &Instrument::synth_lead())
        .at_bar(4)
        .quarters(&[C4, E4, G4, C5]); // Four quarter notes

    comp.instrument("eighths", &Instrument::synth_lead())
        .at_bar(5)
        .eighths(&[C4, D4, E4, F4, G4, A4, B4, C5]); // Eight eighth notes

    comp.instrument("sixteenths", &Instrument::synth_lead())
        .at_bar(6)
        .sixteenths(&[C5, B4, A4, G4, F4, E4, D4, C4, C4, D4, E4, F4, G4, A4, B4, C5]); // 16 sixteenths

    // ===== 3. LONGER NOTE VALUES =====
    println!("\n3. Longer Note Values - Halves and Wholes");

    comp.instrument("halves", &Instrument::warm_pad())
        .at_bar(7)
        .halves(&[C4, G4]); // Two half notes (2 beats each)

    comp.instrument("wholes", &Instrument::warm_pad())
        .at_bar(8)
        .wholes(&[C4]); // One whole note (4 beats)

    // ===== 4. DOTTED RHYTHMS =====
    println!("\n4. Dotted Rhythms - 1.5x duration");

    comp.instrument("dotted_quarters", &Instrument::pluck())
        .at_bar(9)
        .dotted_quarters(&[C4, E4])  // Two dotted quarters (1.5 beats each = 3 beats total)
        .quarter(&[G4]); // One quarter to complete the bar

    comp.instrument("dotted_eighths", &Instrument::pluck())
        .at_bar(10)
        .dotted_eighths(&[C4, D4, E4, F4]);

    // ===== 5. MIXING DURATIONS =====
    println!("\n5. Mixing Different Duration");

    comp.instrument("mixed", &Instrument::acoustic_piano())
        .at_bar(11)
        .half(&[C4])            // Half note (2 beats)
        .quarter(&[E4])         // Quarter note (1 beat)
        .eighth(&[G4])          // Eighth note (0.5 beats)
        .eighth(&[A4])          // Eighth note (0.5 beats)
        .whole(&[C5]);          // Whole note (4 beats) - continues into next bar

    // ===== 6. WAIT IN MUSICAL TIME =====
    println!("\n6. Waiting in Musical Time - .beats() and .bars()");

    comp.instrument("waits", &Instrument::synth_lead())
        .at_bar(13)
        .quarter(&[C4])
        .beats(2.0)             // Wait 2 quarter notes
        .quarter(&[E4])
        .bars(1.0)              // Wait 1 full bar
        .quarter(&[G4]);

    // ===== 7. CHORD PROGRESSION WITH MUSICAL TIME =====
    println!("\n7. Musical Example - Chord Progression");

    comp.instrument("chords", &Instrument::warm_pad())
        .at_bar(15)
        .whole(&[C3, E3, G3])   // C major - 1 bar
        .whole(&[F3, A3, C4])   // F major - 1 bar
        .whole(&[G3, B3, D4])   // G major - 1 bar
        .whole(&[C3, E3, G3]);  // C major - 1 bar

    // Bass line with mixed durations
    comp.instrument("bass", &Instrument::sub_bass())
        .at_bar(15)
        .half(&[C2])
        .quarter(&[C2])
        .quarter(&[G2])
        .half(&[F2])
        .quarter(&[F2])
        .quarter(&[C3])
        .half(&[G2])
        .quarter(&[G2])
        .quarter(&[D2])
        .whole(&[C2]);

    // Melody with various note values
    comp.instrument("melody", &Instrument::synth_lead())
        .at_bar(15)
        .eighth(&[C4])
        .eighth(&[D4])
        .quarter(&[E4])
        .quarter(&[G4])
        .half(&[C5])
        .quarters(&[A4, F4, D4, C4])
        .half(&[B3])
        .half(&[G3])
        .whole(&[C4]);

    // ===== 8. DRUM PATTERN WITH BEATS =====
    println!("\n8. Drums with Musical Time");

    comp.instrument("drums", &Instrument::sub_bass())
        .at_bar(19)
        .drum_grid(16, 0.125)  // Still using seconds for drum grid (tempo-aware grid would be next step)
        .kick(&[0, 8])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14])
        .repeat(3);

    // ===== 9. TEMPO HELPERS WITH ORNAMENTS =====
    println!("\n9. Using Tempo Helpers with Ornaments");

    // Option 1: Capture tempo values for readability
    let qtr = comp.tempo().quarter_note();
    let eighth = comp.tempo().eighth_note();
    let sixteenth = eighth / 2.0;

    comp.instrument("ornaments_tempo", &Instrument::harpsichord())
        .at_bar(41)
        .note(&[C4], qtr)
        .trill(C4, D4, 16, sixteenth)  // Sixteenth note trill
        .note(&[E4], qtr)
        .mordent(G4, eighth)           // Eighth note mordent
        .grace(C5, B4, sixteenth, qtr); // Quick grace into quarter

    // Option 2: Tempo returns a copy now, but still need to capture before builder chain
    let tempo = comp.tempo();  // Get tempo copy
    comp.instrument("inline_calc", &Instrument::acoustic_piano())
        .at_bar(42)
        .tremolo(C4, 8, tempo.eighth_note())
        .trill(C4, D4, 8, tempo.eighth_note() / 2.0);

    // ===== 10. COMPARISON: OLD VS NEW =====
    println!("\n10. Comparison - Raw seconds vs Musical time");

    // Old way (raw seconds)
    comp.instrument("old_way", &Instrument::pluck())
        .at(88.0)  // Have to calculate: at 120 BPM, bar 45 = ?
        .notes(&[C4, E4, G4, C5], 0.5); // Have to calculate: quarter note at 120 BPM = 0.5s

    // New way (musical time)
    comp.instrument("new_way", &Instrument::pluck())
        .at_bar(46) // Clear and musical!
        .quarters(&[C4, E4, G4, C5]); // Self-documenting!

    println!("\n▶️  Playing musical time demo...");
    println!("    Tempo: 120 BPM");
    println!("    Duration: ~92 seconds (~46 bars)\n");
    println!("    💎 New Features:");
    println!("       POSITIONING:");
    println!("       • .at_bar(n) - Position by bar number");
    println!("       • .at_beat(n) - Position by beat number");
    println!("       NOTE DURATIONS:");
    println!("       • .quarters(), .eighths(), .sixteenths() - Note arrays");
    println!("       • .quarter(), .eighth(), .half(), .whole() - Single notes/chords");
    println!("       • .dotted_quarters(), .dotted_eighths() - Dotted rhythms");
    println!("       WAITING:");
    println!("       • .beats(n), .bars(n) - Wait in musical time");
    println!("       TEMPO HELPERS (for ornaments):");
    println!("       • comp.tempo().quarter_note() - Get quarter note duration");
    println!("       • comp.tempo().eighth_note() - Get eighth note duration");
    println!("       • Use for tempo-aware ornaments, trills, grace notes, etc.");
    println!("\n    ✨ All automatically adjust to tempo!");
    println!("    ✨ Existing .at() and .notes() still work for precise control!");

    let mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;

    println!("\n✅ Demo complete!");
    println!("   Musical time notation makes compositions more readable and tempo-independent!");

    Ok(())
}
