// Orbit Generator Demo
//
// Demonstrates the orbit() generator which creates smooth sinusoidal pitch patterns
// around a center frequency, like planets orbiting a star.

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let engine = AudioEngine::new()?;

    println!("🌍 Orbit Generator Demo\n");

    // ==================== BASIC ORBIT ====================
    println!("🔄 Basic Orbit - Clockwise");
    println!("   Smooth oscillation ±7 semitones (perfect fifth) around C4\n");

    comp.instrument("basic_cw", &Instrument::synth_lead())
        .orbit(C4, 7.0, 16, 0.125, 1.0, true);  // Center, radius, steps_per_rotation, duration, rotations, clockwise

    // ==================== COUNTER-CLOCKWISE ====================
    println!("🔄 Counter-clockwise Orbit");
    println!("   Same pattern but starting by descending\n");

    comp.instrument("basic_ccw", &Instrument::synth_lead())
        .wait(2.0)
        .orbit(C4, 7.0, 16, 0.125, 1.0, false);  // false = counter-clockwise

    // ==================== MULTIPLE ROTATIONS ====================
    println!("🌀 Multiple Rotations");
    println!("   Two complete orbits create evolving patterns\n");

    comp.instrument("double", &Instrument::synth_lead())
        .wait(4.0)
        .orbit(G4, 5.0, 12, 0.0833, 2.0, true);  // 2 complete rotations

    // ==================== WIDE ORBIT ====================
    println!("🌌 Wide Orbit - Full Octave");
    println!("   ±12 semitones creates dramatic pitch sweeps\n");

    comp.instrument("wide", &Instrument::synth_lead())
        .wait(6.0)
        .orbit(A4, 12.0, 24, 0.0625, 1.0, true);  // Full octave range

    // ==================== TIGHT ORBIT ====================
    println!("🎯 Tight Orbit - Subtle Vibrato");
    println!("   Small radius creates vibrato-like effect\n");

    comp.instrument("tight", &Instrument::electric_piano())
        .wait(7.5)
        .orbit(E4, 2.0, 32, 0.0625, 1.0, true);  // Just ±2 semitones, many steps

    // ==================== FRACTIONAL ROTATIONS ====================
    println!("➗ Fractional Rotations");
    println!("   Half orbit creates ascending/descending phrases\n");

    comp.instrument("half", &Instrument::synth_lead())
        .wait(9.5)
        .orbit(D5, 7.0, 16, 0.0625, 0.5, true);  // Half rotation - smooth rise

    // ==================== NESTED ORBITS ====================
    println!("🪐 Nested Orbits");
    println!("   Multiple orbit patterns on different instruments\n");

    comp.instrument("outer", &Instrument::synth_lead())
        .wait(10.5)
        .orbit(C5, 12.0, 16, 0.125, 1.0, true);  // Outer orbit

    comp.instrument("inner", &Instrument::electric_piano())
        .wait(10.5)
        .orbit(C5, 5.0, 24, 0.0833, 1.0, false);  // Inner orbit, counter-clockwise

    // ==================== WITH GENERATOR NAMESPACE ====================
    println!("📦 Using .generator() Namespace");
    println!("   Clean, organized syntax\n");

    comp.instrument("namespace", &Instrument::synth_lead())
        .wait(12.5)
        .generator(|g| g
            .orbit(G4, 7.0, 12, 0.125, 1.0, true)
        );

    // ==================== COMBINED WITH TRANSFORMS ====================
    println!("🔗 Orbit + Transforms");
    println!("   Generate orbital pattern, then apply transformations\n");

    comp.instrument("transformed", &Instrument::synth_lead())
        .wait(14.0)
        .generator(|g| g
            .orbit(D4, 7.0, 16, 0.0625, 1.5, true)  // 1.5 rotations
        )
        .transform(|t| t
            .humanize(0.005, 0.02)  // Add organic timing
            .thin(0.7)               // Remove 30% of notes
        );

    // Play everything
    println!("\n▶️  Playing orbit demo...\n");
    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    println!("✅ Demo complete!\n");
    println!("💡 Orbit parameters:");
    println!("   center: The pitch to orbit around (e.g., C4, A4)");
    println!("   radius_semitones: Distance from center (7 = fifth, 12 = octave)");
    println!("   steps_per_rotation: Number of notes in one complete orbit (resolution)");
    println!("   step_duration: Length of each note in seconds");
    println!("   rotations: Number of complete orbits (1.0, 2.0, 0.5, etc.)");
    println!("   clockwise: true = start ascending, false = start descending");
    println!("\n🎵 Musical uses:");
    println!("   • Melodic contours and phrases");
    println!("   • Vibrato and pitch modulation effects (small radius, many steps)");
    println!("   • Ambient textures with nested orbits");
    println!("   • Generative composition patterns");
    println!("   • Fractional rotations for ascending/descending phrases");
    println!("   • Multiple rotations for evolving patterns");
    println!("   • Combine with .magnetize() to snap to scales!");

    Ok(())
}
