use tunes::prelude::*;
use tunes::sequences::euclidean;

/// Demonstrate Euclidean rhythm patterns
fn main() -> anyhow::Result<()> {
    println!("\n🥁 Example: Euclidean Rhythms\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Classic patterns from world music

    // Tresillo (Cuban) - E(3,8)
    let tresillo = euclidean::generate(3, 8);
    comp.track("tresillo")
        .at(0.0)
        .drum_grid(8, 0.25)
        .kick(&tresillo);

    // Cinquillo (Cuban) - E(5,8)
    let cinquillo = euclidean::generate(5, 8);
    comp.track("cinquillo")
        .at(2.5)
        .drum_grid(8, 0.25)
        .snare(&cinquillo);

    // Four-on-floor - E(4,16)
    let four_floor = euclidean::generate(4, 16);
    comp.track("four_floor")
        .at(5.0)
        .drum_grid(16, 0.125)
        .kick(&four_floor);

    // Complex hi-hat - E(7,16)
    let complex_hh = euclidean::generate(7, 16);
    comp.track("complex_hh")
        .at(5.0)
        .drum_grid(16, 0.125)
        .hihat(&complex_hh);

    // Polyrhythmic pattern - multiple Euclidean rhythms layered
    comp.track("poly")
        .at(7.5)
        .drum_grid(16, 0.125)
        .kick(&euclidean::generate(5, 16))
        .snare(&euclidean::generate(3, 16))
        .hihat(&euclidean::generate(11, 16));

    println!("✓ Tresillo: E(3,8) - Classic Cuban pattern");
    println!("✓ Cinquillo: E(5,8) - Cuban dance rhythm");
    println!("✓ Four-on-floor: E(4,16) - Electronic music staple");
    println!("✓ Complex patterns: E(7,16), E(11,16)");
    println!("✓ Polyrhythms: Multiple Euclidean patterns layered\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
