// Sprinkle Generator Demo
//
// Demonstrates the .sprinkle() generator for creating random continuous frequencies.
// Unlike discrete note selection, sprinkle generates any f32 value within a range.

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let engine = AudioEngine::new()?;

    println!("🎲 Sprinkle Generator Demo\n");
    println!("Generates completely random f32 frequencies with no snapping or quantization\n");

    // ==================== BASIC SPRINKLE ====================
    println!("🎯 BASIC - Random frequencies in a range:");
    comp.instrument("basic", &Instrument::synth_lead())
        .sprinkle(220.0, 440.0, 16, 0.125);  // A3 to A4 range

    // ==================== WIDE RANGE ====================
    println!("🌊 WIDE RANGE - Spanning multiple octaves:");
    comp.instrument("wide", &Instrument::electric_piano())
        .wait(2.0)
        .sprinkle(100.0, 1000.0, 24, 0.0833);  // Nearly 3.5 octaves

    // ==================== TIGHT RANGE (MICROTONAL) ====================
    println!("🎵 TIGHT RANGE - Microtonal variations:");
    comp.instrument("micro", &Instrument::synth_lead())
        .wait(4.0)
        .sprinkle(440.0, 450.0, 32, 0.0625);  // Just 10Hz range around A4

    // ==================== FAST CHAOTIC ====================
    println!("⚡ FAST - Rapid random notes:");
    comp.instrument("fast", &Instrument::pluck())
        .wait(6.0)
        .sprinkle(300.0, 900.0, 64, 0.03125);  // 64 rapid random notes

    // ==================== SLOW AMBIENT ====================
    println!("🌌 AMBIENT - Slow evolving textures:");
    comp.instrument("ambient", &Instrument::warm_pad())
        .wait(8.0)
        .sprinkle(80.0, 200.0, 8, 0.5);  // Slow, deep bass notes

    // ==================== WITH GENERATOR NAMESPACE ====================
    println!("📦 NAMESPACE - Using .generator() API:");
    comp.instrument("namespace", &Instrument::electric_piano())
        .wait(12.0)
        .generator(|g| g
            .sprinkle(261.63, 523.25, 16, 0.125)  // C4 to C5
        );

    // ==================== COMBINED WITH TRANSFORMS ====================
    println!("🔗 COMBINED - Sprinkle + Transforms:");
    comp.instrument("combined", &Instrument::synth_lead())
        .wait(14.0)
        .generator(|g| g
            .sprinkle(200.0, 600.0, 16, 0.125)
        )
        .transform(|t| t
            .humanize(0.02, 0.1)  // Add timing and velocity variation
            .quantize(50.0)       // Snap to 50Hz grid (optional organization)
        );

    // ==================== LAYERED SPRINKLES ====================
    println!("🎼 LAYERED - Multiple sprinkle patterns:");
    comp.instrument("layer1", &Instrument::synth_lead())
        .wait(16.0)
        .sprinkle(400.0, 800.0, 8, 0.25);  // High sparse notes

    comp.instrument("layer2", &Instrument::electric_piano())
        .wait(16.0)
        .sprinkle(150.0, 300.0, 16, 0.125);  // Low dense notes

    // ==================== EXPERIMENTAL TEXTURES ====================
    println!("🎨 EXPERIMENTAL - Dense random cloud:");
    comp.instrument("texture", &Instrument::ambient_pad())
        .wait(18.0)
        .sprinkle(100.0, 2000.0, 100, 0.02);  // Dense random texture

    // Play everything
    println!("\n▶️  Playing sprinkle demo...\n");
    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    println!("✅ Demo complete!\n");
    println!("💡 Sprinkle use cases:");
    println!("   🎲 Completely random frequencies (no snapping)");
    println!("   🎵 Microtonal and experimental music");
    println!("   🌊 Ambient textures with organic variation");
    println!("   ⚡ Chaotic, unpredictable patterns");
    println!("   🎨 Dense clouds of random pitches");
    println!("   🔬 Algorithmic composition with continuous pitch space");
    println!("\n   Compare with:");
    println!("   - .scatter() - uniform distribution across range");
    println!("   - .random_notes() - pick from discrete note array");
    println!("   - .orbit() - sinusoidal pitch motion");
    println!("   - .bounce() - physics-based pitch motion\n");

    Ok(())
}
