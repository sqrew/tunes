// Generator API - Complete Overview
//
// Demonstrates the new generator namespace for note-producing methods.
// Both old (direct) and new (namespaced) syntax work - use what fits your workflow!

use tunes::prelude::*;
use tunes::theory::core::ChordPattern;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let engine = AudioEngine::new()?;

    println!("🎯 Complete Generator API Demo\n");

    // ==================== GENERATOR NAMESPACE ====================
    println!("🎼 GENERATOR NAMESPACE (40+ methods total):");
    println!("   CHORDS: chord, chord_inverted, chord_voice_lead, chord_over_bass, chords, chords_from");
    println!("   SCALES: scale, scale_reverse, scale_updown, scale_downup");
    println!("   ARPEGGIOS: arpeggiate, arpeggiate_reverse, arpeggiate_updown, arpeggiate_downup");
    println!("   CLASSICAL: alberti_bass, waltz_bass, broken_chord, walking_bass, tremolo_strings, ostinato");
    println!("   ORNAMENTS: trill, cascade, tremolo_note, strum, mordent, inverted_mordent, turn, inverted_turn");
    println!("   TUPLETS: tuplet, triplet, quintuplet, sextuplet, septuplet");
    println!("   MUSICAL: octaves, pedal, sequence_from");
    println!("   PORTAMENTO: slide");
    println!("   TIME-BASED: wholes, halves, quarters, eighths, sixteenths\n");

    // ==================== CHORDS ====================
    comp.instrument("chords", &Instrument::electric_piano())
        .generator(|g| g
            .chord(C4, &ChordPattern::MAJOR, 0.5)
            .chord(F4, &ChordPattern::MAJOR, 0.5)
            .chord(G4, &ChordPattern::MAJOR, 0.5)
            .chord(C4, &ChordPattern::MAJOR, 0.5)
        );

    // ==================== ARPEGGIOS ====================
    comp.instrument("arpeggios", &Instrument::synth_lead())
        .wait(2.0)
        .generator(|g| g
            .arpeggiate(&[C5, E5, G5, C6], 0.125)
        );

    // ==================== CLASSICAL PATTERNS ====================
    comp.instrument("alberti", &Instrument::electric_piano())
        .wait(3.0)
        .generator(|g| g
            .alberti_bass(&[C4, E4, G4], 0.125)
        );

    // ==================== ORNAMENTS ====================
    comp.instrument("ornaments", &Instrument::synth_lead())
        .wait(4.0)
        .generator(|g| g
            .trill(C5, D5, 8, 0.0625)
        );

    // ==================== TUPLETS ====================
    comp.instrument("tuplets", &Instrument::synth_lead())
        .wait(5.0)
        .generator(|g| g
            .triplet(&[C5, E5, G5], 0.5)
        );

    // ==================== SCALES ====================
    comp.instrument("scales", &Instrument::electric_piano())
        .wait(6.0)
        .generator(|g| g
            .scale(&[C4, D4, E4, F4, G4, A4, B4, C5], 0.125)
        );

    // ==================== COMBINED WITH TRANSFORMS ====================
    println!("🔗 COMBINED - Generators + Transforms:");
    println!("   Chain generators with transforms for complete control\n");

    comp.instrument("combined", &Instrument::synth_lead())
        .wait(7.0)
        .generator(|g| g
            .chord(C5, &ChordPattern::MAJOR, 0.25)
            .chord(F5, &ChordPattern::MAJOR, 0.25)
        )
        .transform(|t| t
            .shift(12)
            .humanize(0.01, 0.05)
        );

    // ==================== CHAINING MULTIPLE GENERATOR BLOCKS ====================
    println!("🔁 CHAINING - Multiple generator blocks:");
    println!("   Create complex musical phrases in stages\n");

    comp.instrument("chained", &Instrument::electric_piano())
        .wait(8.0)
        // First: Generate a chord progression
        .generator(|g| g
            .chord(C4, &ChordPattern::MAJOR, 0.5)
            .chord(A4, &ChordPattern::MINOR, 0.5)
        )
        // Then: Add a melody on top
        .generator(|g| g
            .arpeggiate(&[E5, G5, B5], 0.125)
        );

    // ==================== BACKWARD COMPATIBILITY ====================
    println!("⏮️  OLD SYNTAX STILL WORKS:");
    println!("   Direct method calls for quick live coding\n");

    comp.instrument("old_style", &Instrument::electric_piano())
        .wait(9.0)
        // Old direct-call syntax (no namespace)
        .chord(G4, &ChordPattern::MAJOR7, 0.5)
        .arpeggiate(&[G4, B4, D5, F5], 0.125);

    // Play everything
    println!("\n▶️  Playing generator API demo...\n");
    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    println!("✅ Demo complete!\n");
    println!("💡 Benefits of generator namespace:");
    println!("   📦 Organized - 40+ note generators clearly grouped");
    println!("   🔍 Discoverable - Type .generator() to see all note-producing methods");
    println!("   🧹 Clean autocomplete - Only see generator methods in this namespace");
    println!("   📖 Readable - Visual boundaries clarify intent");
    println!("   🔄 Backward compatible - Old direct-call syntax still works!");
    println!("   🎵 Chainable - Combine with .transform() and .effects() for complete control");
    println!("\n   Use whichever style fits your workflow! 🎵");

    Ok(())
}
