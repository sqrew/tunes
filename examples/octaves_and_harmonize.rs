use tunes::composition::Composition;
use tunes::engine::AudioEngine;
use tunes::instruments::Instrument;
use tunes::notes::*;
use tunes::rhythm::Tempo;

/// Demonstrate octave doubling and harmonization techniques
fn main() -> Result<(), anyhow::Error> {
    println!("\n🎼 Example: Octaves and Harmonization\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Single note vs octave doubled
    comp.instrument("single", &Instrument::electric_piano())
        .at(0.0)
        .notes(&[C4, D4, E4, F4], 0.4);

    comp.instrument("octave_doubled", &Instrument::electric_piano())
        .at(2.0)
        .octaves(&[C4, D4, E4, F4], 1, 0.4);  // Double one octave up

    // Two octaves up
    comp.instrument("two_octaves", &Instrument::electric_piano())
        .at(4.0)
        .octaves(&[C4, D4, E4, F4], 2, 0.4);

    // Octave down for bass
    comp.instrument("octave_bass", &Instrument::deep_bass())
        .at(6.0)
        .octaves(&[C4, D4, E4, F4], -1, 0.4);

    // Harmonization: Third above
    comp.instrument("melody", &Instrument::bright_lead())
        .pan(-0.3)
        .at(8.0)
        .notes(&[C4, D4, E4, F4, G4], 0.4);

    comp.instrument("third_harmony", &Instrument::bright_lead())
        .pan(0.3)
        .at(8.0)
        .harmonize(&[C4, D4, E4, F4, G4], 4, 0.4);  // Major third = 4 semitones

    // Fifth harmony
    comp.instrument("melody_2", &Instrument::square_lead())
        .pan(-0.3)
        .at(10.5)
        .notes(&[G4, F4, E4, D4], 0.4);

    comp.instrument("fifth_harmony", &Instrument::square_lead())
        .pan(0.3)
        .at(10.5)
        .harmonize(&[G4, F4, E4, D4], 7, 0.4);  // Perfect fifth = 7 semitones

    // Seventh harmony for jazz
    comp.instrument("jazz_melody", &Instrument::square_lead())
        .pan(-0.4)
        .at(12.5)
        .notes(&[C4, E4, G4, B4], 0.5);

    comp.instrument("seventh_harmony", &Instrument::square_lead())
        .pan(0.4)
        .at(12.5)
        .harmonize(&[C4, E4, G4, B4], 10, 0.5);  // Minor seventh = 10 semitones

    // Complex: Octave + harmony
    comp.instrument("thick_sound", &Instrument::warm_pad())
        .at(15.0)
        .notes(&[C4, D4, E4], 0.6);

    comp.instrument("thick_octave", &Instrument::warm_pad())
        .at(15.0)
        .octaves(&[C4, D4, E4], 1, 0.6);

    comp.instrument("thick_harmony", &Instrument::warm_pad())
        .at(15.0)
        .harmonize(&[C4, D4, E4], 7, 0.6);

    println!("✓ .octaves(notes, octave_offset, duration):");
    println!("  - Doubles notes at different octaves");
    println!("  - Positive offset = higher, negative = lower");
    println!("  - Creates thickness and power");
    println!("\n✓ .harmonize(notes, semitones, duration):");
    println!("  - Adds harmonic interval to each note");
    println!("  - Common intervals:");
    println!("    • 3 semitones = minor third");
    println!("    • 4 semitones = major third");
    println!("    • 5 semitones = perfect fourth");
    println!("    • 7 semitones = perfect fifth");
    println!("    • 10 semitones = minor seventh");
    println!("    • 12 semitones = octave");
    println!("\n✓ Combine for rich, layered sounds\\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
