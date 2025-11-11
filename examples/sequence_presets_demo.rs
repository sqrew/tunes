use tunes::prelude::*;
use tunes::sequences;

/// Demonstrate the new sequence preset API
///
/// Shows how sequence presets make algorithmic music more discoverable
/// without breaking the existing API.
fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    println!("\n🎵 Sequence Presets Demo\n");

    // ========== COLLATZ PRESETS ==========
    println!("1. Collatz Presets:");
    println!("   Custom:  sequences::collatz::generate(27, 32)");
    println!("   Presets: sequences::collatz::dramatic()\n");

    comp.instrument("collatz_custom", &Instrument::music_box())
        .sequence_from(&sequences::collatz::generate(27, 32), C4_MAJOR_SCALE, 0.25)
        .reverb(Reverb::hall());

    comp.instrument("collatz_preset", &Instrument::pluck())
        .at(4.0)
        .sequence_from(&sequences::collatz::dramatic(), D4_MINOR_SCALE, 0.2)
        .delay(Delay::eighth_note());

    // ========== FIBONACCI PRESETS ==========
    println!("2. Fibonacci Presets:");
    println!("   Custom:  sequences::fibonacci::generate(12)");
    println!("   Presets: sequences::fibonacci::classic()\n");

    comp.instrument("fib_custom", &Instrument::synth_lead())
        .at(8.0)
        .sequence_from(&sequences::fibonacci::generate(8), A3_MINOR_SCALE, 0.3);

    comp.instrument("fib_preset", &Instrument::electric_piano())
        .at(12.0)
        .sequence_from(&sequences::fibonacci::classic(), E4_MINOR_PENTATONIC_SCALE, 0.25)
        .chorus(Chorus::classic());

    // ========== EUCLIDEAN PRESETS ==========
    println!("3. Euclidean Rhythm Presets:");
    println!("   Custom:  sequences::euclidean::generate(5, 16)");
    println!("   Presets: sequences::euclidean::kick_four_floor()\n");

    // Custom euclidean
    comp.track("drums_custom")
        .at(16.0)
        .drum_grid(16, 0.125)
        .kick(&sequences::euclidean::generate(4, 16))
        .snare(&sequences::euclidean::generate(3, 16))
        .hihat(&sequences::euclidean::generate(7, 16));

    // Preset euclidean
    comp.track("drums_preset")
        .at(20.0)
        .drum_grid(16, 0.125)
        .kick(&sequences::euclidean::kick_four_floor())
        .snare(&sequences::euclidean::snare_syncopated())
        .hihat(&sequences::euclidean::hihat_complex());

    // ========== TRADITIONAL PATTERNS ==========
    println!("4. Traditional Rhythm Presets:");
    println!("   sequences::euclidean::tresillo()      - Cuban 3/8");
    println!("   sequences::euclidean::cinquillo()     - Cuban 5/8");
    println!("   sequences::euclidean::bossa_nova()    - Bossa 7/16\n");

    comp.track("latin_drums")
        .at(24.0)
        .drum_grid(16, 0.125)
        .kick(&sequences::euclidean::tresillo())
        .clap(&sequences::euclidean::cinquillo());

    println!("════════════════════════════════════════");
    println!("\n✨ Benefits of the Preset API:\n");
    println!("1. Zero breaking changes - old code still works");
    println!("2. Discoverable - type 'sequences::collatz::' and see presets");
    println!("3. Namespaced - presets grouped with parent function");
    println!("4. No memory overhead - just functions in modules");
    println!("\nBoth APIs work:");
    println!("  sequences::collatz::generate(27, 32)   // Custom");
    println!("  sequences::collatz::dramatic()         // Preset");
    println!("\n════════════════════════════════════════\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
