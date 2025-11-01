use tunes::chords::*;
use tunes::composition::generative::{biased_random_walk_sequence, random_walk_sequence};
use tunes::composition::Composition;
use tunes::engine::AudioEngine;
use tunes::instruments::Instrument;
use tunes::notes::*;
use tunes::rhythm::Tempo;
use tunes::scales::*;
use tunes::sequences;

/// Demonstrate algorithmic pattern generation
fn main() -> Result<(), anyhow::Error> {
    println!("\n🎼 Example: Algorithmic Pattern Generation\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Scale runs
    comp.instrument("scale_up", &Instrument::pluck())
        .at(0.0)
        .scale(&C4_MAJOR_SCALE, 0.1);

    comp.instrument("scale_down", &Instrument::pluck())
        .at(1.0)
        .scale_reverse(&C4_MAJOR_SCALE, 0.1);

    // Arpeggios
    comp.instrument("arp_up", &Instrument::arp_lead())
        .at(2.0)
        .arpeggiate(C4_MAJOR, 0.125)
        .arpeggiate(F4_MAJOR, 0.125)
        .arpeggiate(G4_MAJOR, 0.125);

    comp.instrument("arp_down", &Instrument::arp_lead())
        .at(3.2)
        .arpeggiate_reverse(C4_MAJOR, 0.125)
        .arpeggiate_reverse(F4_MAJOR, 0.125)
        .arpeggiate_reverse(G4_MAJOR, 0.125);

    // Trills and tremolo
    comp.instrument("trill", &Instrument::pluck())
        .at(4.5)
        .trill(C5, D5, 16, 0.05);

    comp.instrument("tremolo", &Instrument::pluck())
        .at(5.5)
        .tremolo(E5, 16, 0.05);

    // Interpolation (smooth pitch glide)
    comp.instrument("glide", &Instrument::synth_lead())
        .at(6.5)
        .interpolated(C4, C5, 16, 0.05);

    // Portamento (scale-aware glide)
    comp.instrument("portamento", &Instrument::synth_lead())
        .at(7.5)
        .portamento(C4, C5, &C4_MAJOR_SCALE, 0.1);

    // Fibonacci sequence mapped to scale
    let fib = sequences::fibonacci(12);
    comp.instrument("fibonacci", &Instrument::pluck())
        .at(8.5)
        .sequence_from(&fib, &C4_MAJOR_SCALE, 0.1);

    // Pattern reversal
    comp.instrument("reversed", &Instrument::pluck())
        .at(10.0)
        .pattern_start()
        .notes(&[C4, D4, E4, F4, G4], 0.15)
        .reverse();

    // === GENERATIVE FEATURES ===

    // Random walk through a scale (unbiased)
    let walk = random_walk_sequence(3, 24, 0, 7); // Start at index 3, 24 steps
    comp.instrument("random_walk", &Instrument::synth_lead())
        .at(11.0)
        .sequence_from(&walk, &C4_MAJOR_SCALE, 0.125);

    // Biased random walk (tends upward for ascending melody)
    let ascending = biased_random_walk_sequence(0, 16, 0, 12, 0.7); // 70% up
    comp.instrument("ascending_walk", &Instrument::pluck())
        .at(14.0)
        .sequence_from(&ascending, &C4_MINOR_PENTATONIC_SCALE, 0.15);

    // Musical inversion (mirror a melody)
    comp.instrument("original", &Instrument::acoustic_piano())
        .at(17.0)
        .pattern_start()
        .notes(&[C4, E4, G4, A4, G4, F4], 0.2);

    comp.instrument("inverted", &Instrument::acoustic_piano())
        .at(18.5)
        .pattern_start()
        .notes(&[C4, E4, G4, A4, G4, F4], 0.2)
        .invert(C4); // Mirror around C4

    // Constrained inversion (keeps result playable)
    comp.instrument("inverted_constrained", &Instrument::warm_pad())
        .at(20.0)
        .pattern_start()
        .notes(&[C4, E4, G4, B4, C5], 0.3)
        .invert_constrained(C4, C3, C5); // Keep in playable range

    // Direct random walk method (walks through actual frequencies)
    comp.instrument("direct_walk", &Instrument::pluck())
        .at(22.0)
        .random_walk(C4, 24, 0.125, &C4_MAJOR_SCALE);

    println!("✓ Scale runs: .scale() and .scale_reverse()");
    println!("✓ Arpeggios: .arpeggiate() and .arpeggiate_reverse()");
    println!("✓ Ornaments: .trill() and .tremolo()");
    println!("✓ Glides: .interpolated() and .portamento()");
    println!("✓ Sequences: .sequence_from() with Fibonacci");
    println!("✓ Transform: .reverse()");
    println!("✓ Generative: random_walk_sequence() and biased_random_walk_sequence()");
    println!("✓ Generative: .invert() and .invert_constrained()");
    println!("✓ Generative: .random_walk() direct method\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
