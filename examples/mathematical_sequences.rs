use tunes::prelude::*;
use tunes::sequences;

/// Showcase of classic mathematical sequences for algorithmic composition
///
/// This example demonstrates fundamental mathematical sequences:
/// - Fibonacci: Natural growth patterns (1, 1, 2, 3, 5, 8...)
/// - Primes: Irregular but deterministic (2, 3, 5, 7, 11, 13...)
/// - Triangular Numbers: Sum sequences (1, 3, 6, 10, 15...)
/// - Powers of Two: Binary progression (1, 2, 4, 8, 16...)
/// - Arithmetic: Linear progressions (a, a+d, a+2d...)
/// - Geometric: Exponential growth (a, ar, ar²...)
/// - Collatz: The famous 3n+1 problem
fn main() -> anyhow::Result<()> {
    println!("\n🔢 Mathematical Sequences for Music\n");
    println!("Classic number theory sequences applied to melody and rhythm\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(100.0));

    // ===== FIBONACCI SEQUENCE =====
    println!("1. Fibonacci Sequence (1, 1, 2, 3, 5, 8, 13, 21...)\n");
    println!("   Each number is the sum of the previous two");
    println!("   Found everywhere in nature: shells, flowers, galaxies\n");

    let fib = sequences::fibonacci::generate(16);
    let fib_freqs = sequences::normalize(&fib, 220.0, 880.0);

    comp.instrument("fibonacci", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .notes(&fib_freqs, 0.2);

    // ===== PRIME NUMBERS =====
    println!("2. Prime Numbers (2, 3, 5, 7, 11, 13, 17, 19...)\n");
    println!("   Numbers divisible only by 1 and themselves");
    println!("   Creates angular, unpredictable melodies\n");

    let primes = sequences::primes::generate(16);
    let prime_freqs = sequences::normalize(&primes, 300.0, 900.0);

    comp.instrument("primes", &Instrument::synth_lead())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(3.5)
        .notes(&prime_freqs, 0.25);

    // ===== TRIANGULAR NUMBERS =====
    println!("3. Triangular Numbers (1, 3, 6, 10, 15, 21...)\n");
    println!("   T(n) = n(n+1)/2 - sum of first n integers");
    println!("   Creates smooth ascending melodic contours\n");

    let triangular = sequences::triangular::generate(12);
    let tri_freqs = sequences::normalize(&triangular, 200.0, 1000.0);

    comp.instrument("triangular", &Instrument::pluck())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(7.0)
        .notes(&tri_freqs, 0.2);

    // ===== POWERS OF TWO =====
    println!("4. Powers of Two (1, 2, 4, 8, 16, 32, 64...)\n");
    println!("   Fundamental to digital audio and binary systems");
    println!("   Creates octave relationships\n");

    let powers = sequences::powers_of_two::generate(8);
    let power_freqs = sequences::normalize(&powers, 110.0, 880.0);

    comp.instrument("powers_of_two", &Instrument::pluck())
        .at(9.5)
        .notes(&power_freqs, 0.3);

    // ===== ARITHMETIC SEQUENCE =====
    println!("5. Arithmetic Sequence (5, 8, 11, 14, 17...)\n");
    println!("   Linear progression: a, a+d, a+2d, a+3d...");
    println!("   Steady, predictable motion - like a scale\n");

    let arithmetic = sequences::arithmetic::generate(5, 3, 12);
    let arith_freqs = sequences::normalize(&arithmetic, 300.0, 700.0);

    comp.instrument("arithmetic", &Instrument::synth_lead())
        .at(12.0)
        .notes(&arith_freqs, 0.2);

    // ===== GEOMETRIC SEQUENCE =====
    println!("6. Geometric Sequence (2, 4, 8, 16, 32...)\n");
    println!("   Exponential growth: a, ar, ar², ar³...");
    println!("   Rapid expansion creates dramatic effect\n");

    let geometric = sequences::geometric::generate(2, 2, 8);
    let geo_freqs = sequences::normalize(&geometric, 150.0, 900.0);

    comp.instrument("geometric", &Instrument::pluck())
        .chorus(Chorus::new(0.3, 2.5, 0.3))
        .at(14.5)
        .notes(&geo_freqs, 0.25);

    // ===== COLLATZ CONJECTURE =====
    println!("7. Collatz Sequence (3n+1 Problem)\n");
    println!("   If even: divide by 2, if odd: multiply by 3 and add 1");
    println!("   Unsolved mathematical mystery - always reaches 1?\n");

    let collatz = sequences::collatz::generate(27, 40);
    let collatz_freqs = sequences::normalize(&collatz, 200.0, 800.0);

    comp.instrument("collatz", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.6))
        .at(17.0)
        .notes(&collatz_freqs, 0.15);

    // ===== COMBINING SEQUENCES =====
    println!("8. Combining Sequences for Complex Patterns\n");
    println!("   Fibonacci rhythm + Arithmetic melody\n");

    // Use Fibonacci to control rhythm density
    let fib_rhythm = sequences::fibonacci::generate(6);
    for (i, &density) in fib_rhythm.iter().take(4).enumerate() {
        let steps: Vec<usize> = (0..(density as usize).min(16))
            .map(|x| x * 16 / density as usize)
            .collect();

        comp.instrument(&format!("combined_{}", i), &Instrument::pluck())
            .at(23.0 + i as f32 * 2.0)
            .drum_grid(16, 0.125)
            .hihat(&steps);
    }

    // Arithmetic melody over the top
    let combined_melody = sequences::arithmetic::generate(440, 55, 8);
    let combined_freqs: Vec<f32> = combined_melody.iter().map(|&x| x as f32).collect();

    comp.instrument("combined_melody", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(23.0)
        .notes(&combined_freqs, 0.4);

    println!("\n▶️  Playing mathematical sequences...\n");
    println!("    Duration: ~31 seconds\n");

    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Mathematical Sequences Complete!\n");
    println!("💡 Key Takeaways:");
    println!("   • Fibonacci creates natural, organic growth");
    println!("   • Primes create unpredictable, angular melodies");
    println!("   • Triangular numbers create smooth contours");
    println!("   • Powers of two relate to octaves and binary rhythms");
    println!("   • Arithmetic = linear, Geometric = exponential");
    println!("   • Collatz shows ordered chaos\n");
    println!("📚 Try Next:");
    println!("   cargo run --example chaotic_sequences");
    println!("   cargo run --example musical_sequences\n");

    Ok(())
}
