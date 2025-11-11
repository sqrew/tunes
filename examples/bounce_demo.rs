// Bounce Generator Demo
//
// Demonstrates the bounce() generator which simulates a bouncing ball in pitch space
// with damping. Perfect for percussive melodies, generative patterns, and "settling" effects.

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let engine = AudioEngine::new()?;

    println!("🏀 Bounce Generator Demo\n");

    // ==================== BASIC BOUNCE ====================
    println!("⬇️  Basic Bounce");
    println!("   Drop from A4 (440Hz) to A3 (220Hz), bounce twice at 50% damping\n");

    comp.instrument("basic", &Instrument::synth_lead())
        .bounce(440.0, 220.0, 0.5, 2, 6, 0.0833);
        // start, stop, ratio, bounces, steps_per_segment, step_duration

    // ==================== HIGH DAMPING ====================
    println!("🎯 High Damping (ratio=0.3)");
    println!("   Bounces quickly settle - tight, controlled pattern\n");

    comp.instrument("tight", &Instrument::electric_piano())
        .wait(1.5)
        .bounce(523.25, 261.63, 0.3, 3, 4, 0.125);  // C5 to C4, high damping

    // ==================== LOW DAMPING ====================
    println!("🌊 Low Damping (ratio=0.8)");
    println!("   Bounces stay high - slower settling, more energetic\n");

    comp.instrument("bouncy", &Instrument::synth_lead())
        .wait(3.0)
        .bounce(392.0, 196.0, 0.8, 4, 8, 0.0625);  // G4 to G3, low damping

    // ==================== SINGLE DROP (no bounces) ====================
    println!("💧 Single Drop");
    println!("   Zero bounces - just a falling pitch (portamento effect)\n");

    comp.instrument("drop", &Instrument::synth_lead())
        .wait(5.5)
        .bounce(880.0, 440.0, 0.5, 0, 12, 0.0625);  // A5 to A4, no bounce

    // ==================== RAPID BOUNCES ====================
    println!("⚡ Rapid Bounces");
    println!("   Many bounces with few steps - quick, staccato feel\n");

    comp.instrument("rapid", &Instrument::pluck())
        .wait(6.5)
        .bounce(330.0, 165.0, 0.6, 5, 3, 0.05);  // E4 to E3, rapid

    // ==================== WITH GENERATOR NAMESPACE ====================
    println!("📦 Using .generator() Namespace");
    println!("   Clean, organized syntax\n");

    comp.instrument("namespace", &Instrument::synth_lead())
        .wait(8.0)
        .generator(|g| g
            .bounce(493.88, 246.94, 0.55, 3, 6, 0.08)  // B4 to B3
        );

    // ==================== COMBINED WITH TRANSFORMS ====================
    println!("🔗 Bounce + Transforms");
    println!("   Generate bouncing pattern, then apply effects\n");

    comp.instrument("transformed", &Instrument::electric_piano())
        .wait(10.0)
        .generator(|g| g
            .bounce(659.25, 329.63, 0.65, 4, 4, 0.0833)  // E5 to E4
        )
        .transform(|t| t
            .humanize(0.005, 0.03)   // Add organic feel
            .magnetize(&[E4, G4, A4, B4, D5, E5])  // Snap to E minor pentatonic
        );

    // ==================== LAYERED BOUNCES ====================
    println!("🎼 Layered Bounces");
    println!("   Multiple bounce patterns at different intervals\n");

    comp.instrument("layer1", &Instrument::synth_lead())
        .wait(12.0)
        .bounce(523.25, 261.63, 0.5, 3, 5, 0.1);  // C5 to C4

    comp.instrument("layer2", &Instrument::electric_piano())
        .wait(12.5)
        .bounce(392.0, 196.0, 0.6, 2, 6, 0.0833);  // G4 to G3

    // ==================== REVERSE BOUNCE (rise then settle) ====================
    println!("⬆️  Reverse Bounce");
    println!("   Start low, rise high, then bounce downward\n");

    comp.instrument("reverse", &Instrument::synth_lead())
        .wait(14.5)
        .bounce(220.0, 440.0, 0.5, 2, 8, 0.0625);  // Swap start/stop for reverse

    // Play everything
    println!("\n▶️  Playing bounce demo...\n");
    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    println!("✅ Demo complete!\n");
    println!("💡 Bounce parameters:");
    println!("   start: Starting frequency (e.g., 440.0 for A4)");
    println!("   stop: Floor frequency where bounces occur (e.g., 220.0 for A3)");
    println!("   ratio: Damping ratio (0.0-1.0, e.g., 0.5 = each bounce is 50% of previous)");
    println!("   bounces: Number of times to bounce back up (0 = just fall)");
    println!("   steps_per_segment: Number of notes in each rise/fall");
    println!("   step_duration: Length of each note in seconds");
    println!("\n🎵 Musical uses:");
    println!("   • Percussive melodic phrases");
    println!("   • Glitch/IDM style pitch drops");
    println!("   • Generative 'settling' patterns");
    println!("   • Sound effects (ball bounces, drops)");
    println!("   • Rhythmic pitch modulation");
    println!("   • Combine with .magnetize() to snap to scales!");
    println!("   • Layer multiple bounces for complexity");

    Ok(())
}
