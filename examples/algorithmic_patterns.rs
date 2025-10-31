use tunes::chords::*;
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

    println!("✓ Scale runs: .scale() and .scale_reverse()");
    println!("✓ Arpeggios: .arpeggiate() and .arpeggiate_reverse()");
    println!("✓ Ornaments: .trill() and .tremolo()");
    println!("✓ Glides: .interpolated() and .portamento()");
    println!("✓ Sequences: .sequence_from() with Fibonacci");
    println!("✓ Transform: .reverse()\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
