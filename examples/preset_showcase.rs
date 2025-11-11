// Comprehensive showcase of all sequence presets in tunes
//
// This example demonstrates the new preset system that makes algorithmic
// composition instantly accessible. Instead of needing to know all the parameters,
// you can now use musically-meaningful presets across all 44 sequence algorithms!
//
// Run with: cargo run --example preset_showcase

use tunes::prelude::*;
use tunes::sequences;

fn main() {
    let mut comp = Composition::new(Tempo::new(120.0));

    // ========== MATHEMATICAL SEQUENCES ==========
    println!("Mathematical sequences with presets:");

    // Fibonacci - classic growth pattern
    let fib = sequences::fibonacci::classic();
    println!("Fibonacci: {:?}", &fib[..8]);
    let fib_melody = sequences::normalize(&fib[..12], 220.0, 440.0);
    comp.instrument("fibonacci", &Instrument::pluck())
        .at(0.0)
        .notes(&fib_melody, 0.25);

    // Primes - for polyrhythmic complexity
    let primes = sequences::primes::polyrhythm();
    println!("Primes: {:?}", primes);

    // Collatz - dramatic journey
    let collatz = sequences::collatz::dramatic();
    println!("Collatz sequence has {} steps", collatz.len());

    // ========== RHYTHMIC SEQUENCES ==========
    println!("\nRhythmic sequences with presets:");

    // Euclidean rhythms - classic drum patterns
    let kick = sequences::euclidean::kick_four_floor();
    let snare = sequences::euclidean::snare_syncopated();
    let hihat = sequences::euclidean::hihat_complex();

    println!("Kick pattern: {:?}", kick);
    println!("Snare pattern: {:?}", snare);
    println!("Hihat pattern: {:?}", hihat);

    comp.track("drums")
        .at(4.0)
        .drum_grid(16, 0.125)
        .kick(&kick)
        .snare(&snare)
        .hihat(&hihat);

    // Golden ratio rhythm
    let golden = sequences::golden_ratio_rhythm::classic();
    println!("Golden ratio rhythm: {:?}", golden);

    // Polyrhythm - 3 against 2 (hemiola)
    let poly = sequences::polyrhythm::three_two();
    println!("Polyrhythm 3:2 has {} voices", poly.len());

    // ========== CHAOS SEQUENCES ==========
    println!("\nChaos sequences with presets:");

    // Logistic map - classic chaos
    let logistic_chaos = sequences::logistic_map::chaotic();
    println!("Logistic map (chaotic): {} values", logistic_chaos.len());
    let chaos_melody = sequences::normalize_f32(&logistic_chaos[..16], 330.0, 660.0);
    comp.instrument("chaos", &Instrument::synth_lead())
        .at(8.0)
        .notes(&chaos_melody, 0.25);

    // Tent map
    let tent = sequences::tent_map::generate(2.0, 0.3, 48);
    println!("Tent map: {} values", tent.len());

    // Sine map
    let sine = sequences::sine_map::generate(3.14159, 0.5, 48); // π for edge of chaos
    println!("Sine map (edge of chaos): {} values", sine.len());

    // ========== ATTRACTORS ==========
    println!("\nAttractor sequences with presets:");

    // Lorenz butterfly
    let lorenz = sequences::lorenz_attractor::classic();
    println!("Lorenz attractor: {} points", lorenz.len());
    let lorenz_x: Vec<f32> = lorenz[..20].iter().map(|(x, _, _)| *x).collect();
    let lorenz_melody = sequences::normalize_f32(&lorenz_x, 220.0, 880.0);
    comp.instrument("lorenz", &Instrument::synth_lead())
        .at(12.0)
        .notes(&lorenz_melody, 0.5);

    // Clifford attractor
    let clifford = sequences::clifford_attractor::classic();
    println!("Clifford attractor: {} points", clifford.len());

    // Rössler attractor
    let rossler = sequences::rossler_attractor::classic();
    println!("Rössler attractor: {} points", rossler.len());

    // ========== 2D MAPS ==========
    println!("\n2D Map sequences with presets:");

    // Hénon map - returns (Vec<f32>, Vec<f32>)
    let (henon_x, henon_y) = sequences::henon_map::generate(1.4, 0.3, 0.1, 0.1, 48);
    println!(
        "Hénon map: {} x-values, {} y-values",
        henon_x.len(),
        henon_y.len()
    );
    let henon_melody = sequences::normalize_f32(&henon_x[..16], 440.0, 880.0);
    comp.instrument("henon", &Instrument::synth_lead())
        .at(16.0)
        .notes(&henon_melody, 0.25);

    // Baker's map
    let (baker_x, _baker_y) = sequences::bakers_map::classic();
    println!("Baker's map: {} values", baker_x.len());

    // Ikeda map
    let ikeda = sequences::ikeda_map::classic();
    println!("Ikeda map: {} points", ikeda.len());

    // ========== RANDOM WALKS ==========
    println!("\nRandom walk sequences with presets:");

    // Random walk
    let walk = sequences::random_walk::classic();
    println!("Random walk: {:?}", &walk[..8]);

    // Bounded walk
    let bounded = sequences::bounded_walk::narrow();
    println!("Bounded walk: {:?}", &bounded[..8]);

    // ========== OTHER GENERATIVE ==========
    println!("\nOther generative sequences with presets:");

    // Thue-Morse - fair division
    let thue = sequences::thue_morse::classic();
    println!("Thue-Morse: {:?}", &thue[..16]);
    let thue_hits: Vec<usize> = thue
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .take(16)
        .collect();
    comp.track("thue_drums")
        .at(20.0)
        .drum_grid(32, 0.125)
        .kick(&thue_hits);

    // Recamán sequence
    let recaman = sequences::recaman::classic();
    println!("Recamán: {:?}", &recaman[..10]);

    // Van der Corput - quasi-random
    let vdc = sequences::van_der_corput::classic();
    println!("Van der Corput: {:?}", &vdc[..8]);

    // Perlin noise - smooth randomness
    let perlin = sequences::perlin_noise::classic();
    println!("Perlin noise: {} smooth values", perlin.len());

    // Cellular automaton - Rule 30 chaos
    let ca_rule30 = sequences::cellular_automaton::rule30();
    println!("CA Rule 30: {} generations", ca_rule30.len());

    // L-System - Fibonacci pattern
    let lsys = sequences::lsystem::fibonacci();
    println!("L-System (Fibonacci): {}", lsys);

    // Markov chain - C major melody
    let markov = sequences::markov::c_major_melody();
    println!("Markov melody: {:?}", &markov[..10]);

    // Cantor set - fractal rhythm
    let cantor = sequences::cantor_set::generate(3, 27);
    println!("Cantor set: {:?}", &cantor[..12]);

    // ========== SUMMARY ==========
    println!("\n🎉 All 44 sequence algorithms now have instant-access presets!");
    println!("   ✓ Mathematical (10): fibonacci, primes, collatz, arithmetic, etc.");
    println!("   ✓ Rhythmic (7): euclidean, golden_ratio, polyrhythm, etc.");
    println!("   ✓ Chaos (3): logistic_map, tent_map, sine_map");
    println!("   ✓ Attractors (3): lorenz, clifford, rossler");
    println!("   ✓ 2D Maps (3): henon, bakers, ikeda");
    println!("   ✓ Walks (3): random_walk, bounded_walk, thue_morse");
    println!("   ✓ Other Generative (10): perlin, recaman, van_der_corput, etc.");
    println!("\n💡 Try exploring variations:");
    println!("   - Use ::classic(), ::short(), ::chaotic(), etc. for quick access");
    println!("   - Or call ::generate() with custom parameters for full control");
    println!("   - All presets are musically meaningful and ready to use!");

    // The composition is ready - examples typically don't render
    // but you can add .export() or other output methods here
    println!("\n📝 Composition created successfully!");
    println!("   Preset showcase complete. Happy composing! 🎶");
}
