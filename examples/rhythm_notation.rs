use tunes::prelude::*;

/// Demonstrates the compact rhythm notation feature
///
/// The .rhythm() method provides a simple string-based notation for drum patterns,
/// inspired by live coding languages like TidalCycles and Strudel.
fn main() -> anyhow::Result<()> {
    println!("\n🥁 Rhythm String Notation Demo\n");
    println!("Compact drum pattern programming with .rhythm()\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // ===== BASIC PATTERNS =====
    println!("Part 1: Basic Drum Patterns\n");
    println!("  x = hit, - = rest\n");

    // Four-on-the-floor kick
    comp.track("kicks")
        .rhythm("x--- x--- x--- x---", DrumType::Kick, 0.125);

    // Backbeat snare
    comp.track("snares")
        .rhythm("---- x--- ---- x---", DrumType::Snare, 0.125);

    // Hi-hat eighths
    comp.track("hats")
        .rhythm("x-x- x-x- x-x- x-x-", DrumType::HiHatClosed, 0.125);

    // ===== COMPACT NOTATION =====
    println!("Part 2: Compact Notation Styles\n");
    println!("  Different rest characters work the same\n");

    comp.track("compact1")
        .at(2.0)
        .rhythm("x.x.x.x.", DrumType::Kick, 0.125);

    comp.track("compact2")
        .at(2.0)
        .rhythm("__x___x_", DrumType::Snare, 0.125);

    comp.track("compact3")
        .at(2.0)
        .rhythm("x~x~x~x~", DrumType::HiHatClosed, 0.0625);

    // ===== MULTIPLE LAYERS =====
    println!("Part 3: Multiple Rhythm Layers\n");
    println!("  Chain multiple .rhythm() calls for complex patterns\n");

    comp.track("multi_drums")
        .at(4.0)
        .rhythm("x--- x--- x--- x---", DrumType::Kick, 0.125)
        .rhythm("---- x--- ---- x-x-", DrumType::Snare, 0.125)
        .rhythm("xxxx xxxx xxxx xxxx", DrumType::HiHatClosed, 0.0625)
        .rhythm("---x ---x ---x ---x", DrumType::Clap, 0.125);

    // ===== NUMERIC NOTATION =====
    println!("Part 4: Numeric Notation (1 = hit, 0 = rest)\n");

    comp.track("numeric")
        .at(6.0)
        .rhythm("10001000", DrumType::Kick808, 0.125)
        .rhythm("00100010", DrumType::Snare, 0.125);

    // ===== COMPLEX PATTERNS =====
    println!("Part 5: Complex Polyrhythmic Patterns\n");

    // 3 against 4
    comp.track("poly_kick")
        .at(8.0)
        .rhythm("x---x---x---", DrumType::Kick, 0.125);

    comp.track("poly_snare")
        .at(8.0)
        .rhythm("x--x--x--x--", DrumType::Snare, 0.125);

    // ===== MIXED NOTATION =====
    println!("Part 6: Mixed Character Styles\n");
    println!("  You can use any hit character: x, X, 1, *\n");

    comp.track("mixed")
        .at(10.0)
        .rhythm("x-*- 1-x- X-1- *-x-", DrumType::Cowbell, 0.125);

    // ===== LATIN RHYTHMS =====
    println!("Part 7: Classic Latin Rhythms\n");

    // Tresillo (Cuban pattern)
    comp.track("tresillo")
        .at(12.0)
        .rhythm("x--x--x-", DrumType::Tom, 0.125);

    // Son clave
    comp.track("clave")
        .at(12.0)
        .rhythm("x--x--x---x-x---", DrumType::Rimshot, 0.125);

    // ===== BREAKBEAT PATTERNS =====
    println!("Part 8: Classic Breakbeat\n");

    comp.track("break_kick")
        .at(14.0)
        .rhythm("x-------x-------", DrumType::Kick, 0.0625);

    comp.track("break_snare")
        .at(14.0)
        .rhythm("----x-------x---", DrumType::Snare, 0.0625);

    comp.track("break_hats")
        .at(14.0)
        .rhythm("x-x-x-x-x-x-x-x-", DrumType::HiHatClosed, 0.0625);

    // ===== TECHNO PATTERN =====
    println!("Part 9: Techno 4/4\n");

    comp.track("techno")
        .at(16.0)
        .rhythm("x-x-x-x-x-x-x-x-", DrumType::Kick808, 0.125)
        .rhythm("----x-------x---", DrumType::Snare, 0.125)
        .rhythm("x.x.x.x.x.x.x.x.", DrumType::HiHatClosed, 0.0625)
        .rhythm("-------x-------x", DrumType::HiHatOpen, 0.125);

    // ===== SYNCOPATED PATTERN =====
    println!("Part 10: Syncopated Groove\n");

    comp.track("synco")
        .at(18.0)
        .rhythm("x---x-----x-x---", DrumType::Kick, 0.125)
        .rhythm("----x--x----x---", DrumType::Snare, 0.125)
        .rhythm("x-xxx-xxx-xxx-xx", DrumType::HiHatClosed, 0.0625);

    println!("\n▶️  Playing rhythm notation examples...\n");
    println!("    Duration: ~20 seconds\n");
    println!("    💡 Benefits:\n");
    println!("    • Visual pattern representation\n");
    println!("    • Quick drum programming\n");
    println!("    • Familiar to live coders\n");
    println!("    • Still type-safe and expressive\n");
    println!("    • Easy to read and modify\n\n");

    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Rhythm Notation Demo Complete!\n");
    println!("💡 Usage Tips:\n");
    println!("   • x, X, 1, * = hit (all equivalent)\n");
    println!("   • -, _, ., ~, 0, space = rest\n");
    println!("   • Chain multiple .rhythm() calls for layers\n");
    println!("   • Works with all DrumType variants\n");
    println!("   • Use with .at() for positioning\n");
    println!("   • Combines well with existing drum_grid API\n\n");
    println!("📚 Examples:\n");
    println!("   .rhythm(\"x-x- x-x-\", DrumType::Kick, 0.125)\n");
    println!("   .rhythm(\"x.x.x.x.\", DrumType::HiHat, 0.125)\n");
    println!("   .rhythm(\"1001 1001\", DrumType::Snare, 0.125)\n");
    println!("   .rhythm(\"x--- x--- x--- x---\", DrumType::Kick808, 0.125)\n\n");

    Ok(())
}
