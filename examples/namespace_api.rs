// Namespace API - Complete Overview
//
// Demonstrates the new namespace organization for generators, transforms, and effects.
// Both old (direct) and new (namespaced) syntax work - use what fits your workflow!

use tunes::prelude::*;
use tunes::theory::core::ChordPattern;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let engine = AudioEngine::new()?;

    println!("🎯 Complete Namespace API Demo\n");

    // ==================== TRANSFORM NAMESPACE ====================
    println!("🔄 TRANSFORM NAMESPACE (24 methods total):");
    println!("   shift, humanize, rotate, retrograde, reverse, shuffle,");
    println!("   thin, stack, mutate, stretch, compress, quantize,");
    println!("   palindrome, stutter, stutter_every, granularize,");
    println!("   magnetize, gravity, ripple, invert, invert_constrained,");
    println!("   repeat, harmonize, every_n\n");

    comp.instrument("transforms", &Instrument::synth_lead())
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .transform(|t| t
            // Pitch transformations
            .shift(7)                     // Transpose up a fifth
            .rotate(1)                    // Cycle pitches
            .invert(G4)                   // Invert around G4

            // Rhythm transformations
            .stretch(1.2)                 // Slow down 20%
            .humanize(0.01, 0.05)         // Add organic feel

            // Effects transformations
            .mutate(2)                    // Random pitch variation
            .thin(0.8)                    // Remove 20% of notes
        );

    // ==================== EFFECTS NAMESPACE ====================
    println!("🎚️  EFFECTS NAMESPACE (17 methods total):");
    println!("   filter, delay, reverb, distortion, bitcrusher, compressor,");
    println!("   chorus, eq, saturation, phaser, flanger, ring_mod,");
    println!("   tremolo, autopan, gate, limiter, modulate\n");

    comp.instrument("effects", &Instrument::electric_piano())
        .wait(2.0)
        .note(&[C4], 1.0)
        .effects(|e| e
            .filter(Filter::low_pass(2000.0, 0.7))
        );

    // ==================== GENERATOR NAMESPACE ====================
    println!("🎼 GENERATOR NAMESPACE (40+ methods total):");
    println!("   chord, arpeggiate, trill, alberti_bass, triplet, scale,");
    println!("   cascade, tremolo_note, strum, mordent, turn, octaves,");
    println!("   waltz_bass, broken_chord, walking_bass, ostinato, slide,");
    println!("   wholes, halves, quarters, eighths, sixteenths, and more...\n");

    comp.instrument("generators", &Instrument::electric_piano())
        .wait(4.0)
        .generator(|g| g
            .chord(C4, &ChordPattern::MAJOR, 0.5)
            .arpeggiate(&[E5, G5, C6], 0.125)
        );

    // ==================== COMBINED EXAMPLE ====================
    println!("🔗 COMBINED - Generators + Transforms + Effects:");
    println!("   Chain all three namespaces for complete control\n");

    comp.instrument("combined", &Instrument::synth_lead())
        .wait(6.0)
        .generator(|g| g
            .chord(C5, &ChordPattern::MAJOR, 0.5)
        )
        .transform(|t| t
            .granularize(20)              // Break into grains
            .magnetize(&[C4, D4, E4, G4, A4])  // Snap to pentatonic
            .gravity(E4, 0.3)             // Pull toward E4
            .shuffle()                    // Randomize
            .humanize(0.01, 0.08)         // Organic feel
        )
        .effects(|e| e
            .filter(Filter::band_pass(1000.0, 0.5))
        );

    // ==================== CHAINING MULTIPLE BLOCKS ====================
    println!("🔁 CHAINING - Multiple transform/effect blocks:");
    println!("   Organize complex processing into logical stages\n");

    comp.instrument("chained", &Instrument::synth_lead())
        .wait(8.0)
        .pattern_start()
        .notes(&[C4, D4, E4, F4, G4], 0.2)
        // First transformation: rhythm
        .transform(|t| t
            .compress(1.0)                // Fit to 1 second
            .quantize(0.125)              // Snap to 16ths
        )
        // Second transformation: pitch
        .transform(|t| t
            .shift(12)                    // Up an octave
            .rotate(2)                    // Rotate pitches
            .magnetize(&[C5, E5, G5])     // Force to C major triad
        )
        // Third transformation: feel
        .transform(|t| t
            .humanize(0.005, 0.03)
            .ripple(0.015)                // Cascading effect
        )
        // Effects chain
        .effects(|e| e
            .filter(Filter::low_pass(2000.0, 0.6))
        );

    // ==================== BACKWARD COMPATIBILITY ====================
    println!("⏮️  OLD SYNTAX STILL WORKS:");
    println!("   Direct method calls for quick live coding\n");

    comp.instrument("old_style", &Instrument::synth_lead())
        .wait(10.0)
        .pattern_start()
        .notes(&[C4, E4, G4], 0.25)
        // Old direct-call syntax (no namespace)
        .shift(5)
        .humanize(0.01, 0.05)
        .rotate(1)
        // Mix with generators and effects
        .chord(C5, &ChordPattern::MINOR, 0.5)
        .filter(Filter::high_pass(1000.0, 0.7));

    // Play everything
    println!("\n▶️  Playing namespace API demo...\n");
    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    println!("✅ Demo complete!\n");
    println!("💡 Benefits of namespaced API:");
    println!("   📦 Organized - 40+ generators + 24 transforms + 17 effects clearly separated");
    println!("   🔍 Discoverable - Type .generator(), .transform(), or .effects() to see all options");
    println!("   🧹 Clean autocomplete - Only see relevant methods in each namespace");
    println!("   📖 Readable - Visual boundaries make code intent clear");
    println!("   🎯 Logical grouping - generators create, transforms modify, effects process");
    println!("   🔄 Backward compatible - Old direct-call syntax still works!");
    println!("\n   Use whichever style fits your workflow! 🎵");

    Ok(())
}
