// Transform Namespace - New Closure-Based API
//
// Pattern transformations are now namespaced under `.transform()` for better
// API organization. Both the old direct-call syntax and new closure syntax work!

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let engine = AudioEngine::new()?;

    println!("🎯 Transform Namespace Demo\n");

    // OLD WAY (still works, backward compatible)
    println!("📜 Old syntax (direct calls - still works!):");
    comp.instrument("old_way", &Instrument::synth_lead())
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .shift(7)           // Direct method calls
        .humanize(0.01, 0.05)
        .rotate(1);

    // NEW WAY (cleaner, scoped, organized)
    println!("✨ New syntax (scoped with closure):");
    comp.instrument("new_way", &Instrument::synth_lead())
        .wait(2.0)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .transform(|t| t    // Enter transform namespace
            .shift(7)
            .humanize(0.01, 0.05)
            .rotate(1)
        )                   // Automatically exits namespace
        .wait(1.0);

    // CHAINING MULTIPLE TRANSFORM BLOCKS
    println!("🔗 Chaining multiple transform blocks:");
    comp.instrument("chained", &Instrument::electric_piano())
        .wait(4.0)
        .pattern_start()
        .notes(&[C4, D4, E4, F4, G4], 0.25)
        .transform(|t| t    // First transform: rhythm
            .stretch(0.5)
            .quantize(0.125)
        )
        .transform(|t| t    // Second transform: pitch
            .shift(12)
            .mutate(2)
        )
        .transform(|t| t    // Third transform: feel
            .humanize(0.01, 0.05)
            .shuffle()
        );

    // COMPLEX GENERATIVE EXAMPLE
    println!("🌀 Complex generative transformation:");
    comp.instrument("generative", &Instrument::synth_lead())
        .wait(6.0)
        .pattern_start()
        .note(&[C4], 1.0)
        .transform(|t| t
            .granularize(16)           // Break into grains
            .mutate(4)                 // Randomize pitches
            .thin(0.7)                 // Remove some grains
            .magnetize(&[C4, D4, E4, G4, A4])  // Force to pentatonic
            .gravity(E4, 0.3)          // Pull toward E4
            .ripple(0.02)              // Add cascading effect
            .shuffle()                 // Randomize order
            .humanize(0.01, 0.08)      // Add organic feel
        );

    // Play everything
    println!("\n▶️  Playing transform namespace demo...\n");
    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    println!("✅ Demo complete!\n");
    println!("💡 Benefits of the new syntax:");
    println!("   ✨ Cleaner autocomplete - only see transforms when in .transform()");
    println!("   📦 Organized namespace - all 18 transforms grouped together");
    println!("   🔍 Better discoverability - easy to explore what's available");
    println!("   📖 More readable - clear boundaries for transform operations");
    println!("   🔄 Backward compatible - old direct-call syntax still works!");
    println!("\n   Both syntaxes are valid - use whichever fits your workflow!");

    Ok(())
}
