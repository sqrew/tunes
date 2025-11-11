// Pattern Physics: magnetize, gravity, and ripple
//
// Three unique pattern transformation methods inspired by physical forces:
// - magnetize() - Snap notes to scale degrees (pitch quantization)
// - gravity() - Pull notes toward or away from a tonal center
// - ripple() - Cascading effects through time and pitch

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let engine = AudioEngine::new()?;

    println!("🌀 Pattern Physics Demo\n");

    // 1. MAGNETIZE - Snap to scale (pitch quantization)
    println!("🧲 MAGNETIZE - Snap chromatic notes to C major pentatonic");
    comp.instrument("magnetize_demo", &Instrument::synth_lead())
        // Chromatic melody (all 12 semitones)
        .pattern_start()
        .notes(&[C4, CS4, D4, DS4, E4, F4, FS4, G4, GS4, A4, AS4, B4], 0.25)
        .wait(0.5)
        // Same melody snapped to C major pentatonic (C, D, E, G, A)
        .pattern_start()
        .notes(&[C4, CS4, D4, DS4, E4, F4, FS4, G4, GS4, A4, AS4, B4], 0.25)
        .magnetize(&[C4, D4, E4, G4, A4, C5])
        .wait(0.5)
        // Snapped to C minor pentatonic (C, Eb, F, G, Bb)
        .pattern_start()
        .notes(&[C4, CS4, D4, DS4, E4, F4, FS4, G4, GS4, A4, AS4, B4], 0.25)
        .magnetize(&[C4, DS4, F4, G4, AS4, C5]);

    // 2. GRAVITY - Tonal center attraction/repulsion
    println!("⚛️  GRAVITY - Pull notes toward or away from center pitch");
    comp.instrument("gravity_demo", &Instrument::synth_lead())
        .wait(16.0)
        // Wide spread melody (low and high notes)
        .pattern_start()
        .notes(&[C3, G3, C4, G4, C5, G5], 0.5)
        .wait(0.5)
        // Pull 50% toward C4 (compress range)
        .pattern_start()
        .notes(&[C3, G3, C4, G4, C5, G5], 0.5)
        .gravity(C4, 0.5)
        .wait(0.5)
        // Repel from C4 (expand range)
        .pattern_start()
        .notes(&[C3, G3, C4, G4, C5, G5], 0.5)
        .gravity(C4, -0.3);

    // 3. RIPPLE - Cascading time and pitch effects
    println!("🌊 RIPPLE - Cascading effects through the pattern");
    comp.instrument("ripple_demo", &Instrument::synth_lead())
        .wait(32.0)
        // Regular rhythm (no ripple)
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4], 0.25)
        .wait(0.5)
        // Positive ripple (pushes forward in time and up in pitch)
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4], 0.25)
        .ripple(0.03)
        .wait(0.5)
        // Negative ripple (pulls back in time and down in pitch)
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4], 0.25)
        .ripple(-0.02);

    // COMBINED EXAMPLE - Use all three together
    println!("🔗 COMBINED - Magnetize + Gravity + Ripple\n");
    comp.instrument("combined_demo", &Instrument::electric_piano())
        .wait(48.0)
        .pattern_start()
        .notes(&[C4, CS4, D4, DS4, E4, F4, FS4, G4], 0.25)
        .magnetize(&[C4, E4, G4, C5])   // Force to C major arpeggio
        .gravity(E4, 0.3)                // Pull toward E4
        .ripple(0.02);                   // Add cascading effect

    // Play composition
    println!("▶️  Playing pattern physics demo...\n");

    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    println!("✅ Demo complete!\n");
    println!("💡 Use cases:");
    println!("   🧲 .magnetize() - Force melodies into specific scales/modes");
    println!("      Great for: Generative music, fixing off-key notes, modal jazz");
    println!("   ⚛️  .gravity() - Create tonal centers, compress/expand melodic range");
    println!("      Great for: Organic pitch variation, tonal magnetism effects");
    println!("   🌊 .ripple() - Cascading delays and pitch shifts");
    println!("      Great for: Water droplet effects, delay-line textures");
    println!("\n   Chain them for unique generative algorithms!");

    Ok(())
}
