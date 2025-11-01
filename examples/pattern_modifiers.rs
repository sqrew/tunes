use tunes::composition::Composition;
use tunes::engine::AudioEngine;
use tunes::instruments::Instrument;
use tunes::notes::*;
use tunes::rhythm::Tempo;
use tunes::sequences::euclidean;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n🎛️  Pattern Modifiers Demo\n");
    println!("Demonstrating .speed() and .probability() pattern transformations\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(140.0));

    let sixteenth = comp.tempo().sixteenth_note();

    // Example 1: Speed modifier - Double-time melody
    println!("1. Speed Modifier - Normal vs 2x speed");
    comp.instrument("melody_normal", &Instrument::pluck())
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .repeat(1);  // Play twice at normal speed

    comp.instrument("melody_fast", &Instrument::synth_lead())
        .at(2.0)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .speed(2.0)  // Double-time!
        .repeat(1);

    // Example 2: Probability for variation
    println!("2. Probability - Creating variation in hi-hats");
    comp.track("hihat")
        .at(4.0)
        .pattern_start()
        .note(&[/* hihat freq */], sixteenth)
        .repeat(15)  // 16 hi-hats
        .probability(0.7);  // Each has 70% chance - creates human feel!

    // Example 3: Combining speed and probability
    println!("3. Combined - Fast + probabilistic arpeggio");
    comp.instrument("combined", &Instrument::synth_lead())
        .at(6.0)
        .pattern_start()
        .notes(&[C5, E5, G5, B5, D6, G6], 0.2)
        .speed(1.5)        // Speed up by 50%
        .probability(0.8); // Remove 20% for variation

    // Example 4: Generative drum pattern with euclidean rhythm
    println!("4. Euclidean rhythm pattern");
    let pattern = euclidean(7, 16);

    comp.track("drums")
        .at(8.0)
        .drum_grid(16, sixteenth)
        .kick(&pattern)
        .repeat(4);

    println!("\n▶ Playing composition with pattern modifiers...\n");
    engine.play_mixer(&comp.into_mixer())?;

    println!("✓ Speed modifier: Compress/expand timing");
    println!("✓ Probability: Add variation and humanization");
    println!("✓ Perfect for generative music!\n");

    Ok(())
}
