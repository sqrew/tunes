use tunes::prelude::*;
use tunes::sequences;

/// Comprehensive showcase of all mathematical sequences for generative composition
fn main() -> anyhow::Result<()> {
    println!("\n🔢 Mathematical Sequences Showcase\n");
    println!("Exploring algorithmic composition with mathematical sequences\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(100.0));

    // ===== PART 1: FIBONACCI SEQUENCE =====
    println!("Part 1: Fibonacci Sequence\n");
    println!("  The classic recursive sequence: 1, 1, 2, 3, 5, 8, 13, 21...");
    println!("  Each number is the sum of the previous two\n");

    let fib = sequences::fibonacci::generate(16);
    let fib_freqs = sequences::normalize(&fib, 200.0, 800.0);

    comp.instrument("fibonacci", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(0.0)
        .notes(&fib_freqs, 0.2);

    // ===== PART 2: PRIME NUMBERS =====
    println!("Part 2: Prime Numbers\n");
    println!("  Numbers divisible only by 1 and themselves: 2, 3, 5, 7, 11, 13...");
    println!("  Creates angular, unpredictable melodies\n");

    let primes = sequences::primes::generate(12);
    let prime_freqs = sequences::normalize(&primes, 300.0, 900.0);

    comp.instrument("primes", &Instrument::synth_lead())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(3.5)
        .notes(&prime_freqs, 0.25);

    // ===== PART 3: COLLATZ SEQUENCE =====
    println!("Part 3: Collatz Sequence (3n+1 Problem)\n");
    println!("  If even: divide by 2, if odd: 3n+1");
    println!("  Creates chaotic but eventually converging patterns\n");

    let collatz = sequences::collatz::generate(27, 40);
    let collatz_freqs = sequences::normalize(&collatz, 150.0, 700.0);

    comp.instrument("collatz", &Instrument::pluck())
        .chorus(Chorus::new(0.3, 2.5, 0.3))
        .at(6.5)
        .notes(&collatz_freqs, 0.15);

    // ===== PART 4: HARMONIC SERIES =====
    println!("Part 4: Harmonic (Overtone) Series\n");
    println!("  Natural integer multiples: f, 2f, 3f, 4f, 5f...");
    println!("  The foundation of all musical timbre\n");

    let harmonics = harmonic_series(110.0, 12);

    comp.instrument("harmonics", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(9.0)
        .note(&harmonics[3..8], 2.0); // Harmonics 4-8 as a cluster

    // ===== PART 5: GOLDEN RATIO POWERS =====
    println!("Part 5: Golden Ratio (φ) Powers\n");
    println!("  φ^n where φ ≈ 1.618: 1.0, 1.618, 2.618, 4.236...");
    println!("  Natural, organic-sounding proportions\n");

    let phi_seq = golden_ratio(10);
    let phi_freqs = sequences::normalize(
        &phi_seq.iter().map(|&x| x as u32).collect::<Vec<_>>(),
        250.0,
        750.0,
    );

    comp.instrument("golden_ratio", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.6))
        .at(11.5)
        .notes(&phi_freqs, 0.2);

    // ===== PART 6: GOLDEN RATIO RHYTHM =====
    println!("Part 6: Golden Ratio Rhythm (Beatty Sequence)\n");
    println!("  Non-periodic rhythm based on φ spacing");
    println!("  Never quite repeats - organic and flowing\n");

    let phi_rhythm = golden_ratio_rhythm::generate(32);

    comp.track("phi_drums")
        .at(13.5)
        .drum_grid(32, 0.125)
        .kick(&phi_rhythm)
        .hihat(&sequences::euclidean::generate(16, 32)); // Compare with Euclidean

    // ===== PART 7: EUCLIDEAN RHYTHMS =====
    println!("Part 7: Euclidean Rhythms\n");
    println!("  Evenly distribute k pulses over n steps");
    println!("  Used in music traditions worldwide\n");

    // Classic patterns
    let tresillo = sequences::euclidean::generate(3, 8); // Cuban tresillo
    let cinquillo = sequences::euclidean::generate(5, 8); // Cuban cinquillo
    let bossa = sequences::euclidean::generate(5, 16); // Bossa nova clave

    comp.track("euclidean")
        .at(17.5)
        .drum_grid(16, 0.125)
        .kick(&tresillo)
        .snare(&cinquillo)
        .hihat(&bossa);

    // ===== PART 8: TRIANGULAR NUMBERS =====
    println!("Part 8: Triangular Numbers\n");
    println!("  Sum of integers: T(n) = n(n+1)/2");
    println!("  Creates ascending melodic contours: 1, 3, 6, 10, 15, 21...\n");

    let triangular = sequences::triangular::generate(12);
    let tri_freqs = sequences::normalize(&triangular, 200.0, 1000.0);

    comp.instrument("triangular", &Instrument::pluck())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(19.5)
        .notes(&tri_freqs, 0.2);

    // ===== PART 9: POWERS OF TWO =====
    println!("Part 9: Powers of Two\n");
    println!("  Exponential growth: 1, 2, 4, 8, 16, 32...");
    println!("  Creates octave relationships when used as frequencies\n");

    let powers = sequences::powers_of_two::generate(8);
    let power_freqs = sequences::normalize(&powers, 110.0, 880.0);

    comp.instrument("powers_of_two", &Instrument::pluck())
        .at(22.0)
        .notes(&power_freqs, 0.3);

    // ===== PART 10: ARITHMETIC SEQUENCE =====
    println!("Part 10: Arithmetic Sequence\n");
    println!("  Linear progression: a, a+d, a+2d, a+3d...");
    println!("  Steady, predictable motion\n");

    let arithmetic = sequences::arithmetic::generate(5, 3, 10);
    let arith_freqs = sequences::normalize(&arithmetic, 300.0, 700.0);

    comp.instrument("arithmetic", &Instrument::synth_lead())
        .at(24.5)
        .notes(&arith_freqs, 0.2);

    // ===== PART 11: GEOMETRIC SEQUENCE =====
    println!("Part 11: Geometric Sequence\n");
    println!("  Exponential growth: a, ar, ar², ar³...");
    println!("  Rapid expansion or contraction\n");

    let geometric = sequences::geometric::generate(2, 2, 8);
    let geo_freqs = sequences::normalize(&geometric, 150.0, 900.0);

    comp.instrument("geometric", &Instrument::pluck())
        .chorus(Chorus::new(0.3, 2.5, 0.3))
        .at(26.5)
        .notes(&geo_freqs, 0.25);

    // ===== PART 12: GOLDEN SECTIONS =====
    println!("Part 12: Golden Section Divisions\n");
    println!("  Dividing values by φ recursively");
    println!("  Natural decay/diminishment\n");

    let sections = golden_sections(800.0, 8);

    comp.instrument("golden_sections", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(28.5)
        .notes(&sections, 0.3);

    // ===== PART 13: COMBINING SEQUENCES =====
    println!("Part 13: Combining Sequences\n");
    println!("  Using multiple sequences together for complex patterns\n");

    // Fibonacci rhythm with harmonic melody
    let fib_rhythm = sequences::fibonacci::generate(8);
    let fib_steps: Vec<usize> = fib_rhythm.iter().map(|&x| (x % 16) as usize).collect();

    let harm_melody = harmonic_series(220.0, 6); // Reduced from 8 to 6 - less harsh

    comp.instrument("combined", &Instrument::pluck())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(31.0)
        .drum_grid(16, 0.125)
        .hit(DrumType::Kick, &fib_steps);

    comp.instrument("combined_melody", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.6))
        .at(31.0)
        .notes(&harm_melody, 0.4); // Slower to hear each harmonic

    // ===== PART 14: SEQUENCE AS RHYTHM DENSITY =====
    println!("Part 14: Using Sequences to Control Rhythm Density\n");
    println!("  Fibonacci numbers determine how many hits per measure\n");

    let densities = sequences::fibonacci::generate(6);

    for (i, &density) in densities.iter().take(4).enumerate() {
        let steps: Vec<usize> = (0..(density as usize).min(16))
            .map(|x| x * 16 / density as usize)
            .collect();
        comp.instrument(&format!("density_{}", i), &Instrument::pluck())
            .at(33.0 + i as f32 * 2.0)
            .drum_grid(16, 0.125)
            .hihat(&steps);
    }

    // ===== PART 15: SPECTRAL MUSIC WITH HARMONICS =====
    println!("Part 15: Spectral Chords from Harmonic Series\n");
    println!("  Using different harmonics as chord tones\n");

    let spectral_base = harmonic_series(55.0, 20);

    // Different harmonic clusters
    comp.instrument("spectral1", &Instrument::warm_pad())
        .reverb(Reverb::new(0.8, 0.7, 0.6))
        .at(41.0)
        .note(&[spectral_base[3], spectral_base[4], spectral_base[5]], 2.0) // 4-5-6: major triad
        .note(&[spectral_base[6], spectral_base[7], spectral_base[8]], 2.0) // Mid cluster
        .note(
            &[spectral_base[9], spectral_base[10], spectral_base[11]],
            2.0,
        ); // Higher cluster

    // ===== PART 16: LOGISTIC MAP (CHAOS THEORY) =====
    println!("Part 16: Logistic Map - Chaos Theory\n");
    println!("  x(n+1) = r * x(n) * (1 - x(n))");
    println!("  Smooth transition from stable to chaotic behavior\n");

    // Stable behavior (r=2.5)
    let stable_chaos = sequences::logistic_map::generate(2.5, 0.5, 16);
    let stable_freqs = sequences::normalize(
        &stable_chaos
            .iter()
            .map(|&x| (x * 100.0) as u32)
            .collect::<Vec<_>>(),
        300.0,
        600.0,
    );

    comp.instrument("chaos_stable", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(47.0)
        .notes(&stable_freqs, 0.3);

    // Chaotic behavior (r=3.9)
    let chaotic = sequences::logistic_map::generate(3.9, 0.5, 32);
    let chaotic_freqs = sequences::normalize(
        &chaotic
            .iter()
            .map(|&x| (x * 100.0) as u32)
            .collect::<Vec<_>>(),
        200.0,
        800.0,
    );

    comp.instrument("chaos_intense", &Instrument::pluck())
        .chorus(Chorus::new(0.3, 2.5, 0.3))
        .at(52.0)
        .notes(&chaotic_freqs, 0.15);

    // ===== PART 17: RANDOM WALK (BROWNIAN MOTION) =====
    println!("Part 17: Random Walk (Brownian Motion)\n");
    println!("  Smooth, organic wandering melodies");
    println!("  Like a drunk person walking or particle diffusion\n");

    // Unbounded walk
    let walk = sequences::random_walk::generate(440.0, 20.0, 20);

    comp.instrument("random_walk", &Instrument::pluck())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(57.0)
        .notes(&walk, 0.25);

    // Bounded walk (constrained to range)
    let bounded = sequences::bounded_walk::generate(440.0, 30.0, 220.0, 880.0, 32);

    comp.instrument("bounded_walk", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.6))
        .at(62.0)
        .notes(&bounded, 0.2);

    // ===== PART 18: THUE-MORSE SEQUENCE =====
    println!("Part 18: Thue-Morse Sequence (Fair Division)\n");
    println!("  Self-similar binary sequence: 0,1,1,0,1,0,0,1...");
    println!("  Non-repetitive rhythms that sound organic\n");

    let thue_morse = sequences::thue_morse::generate(32);
    let tm_hits: Vec<usize> = thue_morse
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();

    comp.track("thue_morse_drums")
        .at(68.5)
        .drum_grid(32, 0.125)
        .kick(&tm_hits)
        .snare(&sequences::euclidean::generate(5, 32)); // Compare with Euclidean

    // Use Thue-Morse for timbral alternation
    let tm_melody = sequences::thue_morse::generate(16);
    for (i, &val) in tm_melody.iter().enumerate() {
        let freq = if val == 0 { 440.0 } else { 554.37 };
        comp.instrument("tm_alternating", &Instrument::pluck())
            .at(72.5 + i as f32 * 0.25)
            .note(&[freq], 0.2);
    }

    // ===== PART 19: RECAMÁN'S SEQUENCE =====
    println!("\nPart 19: Recamán's Sequence\n");
    println!("  Back-and-forth spiraling pattern");
    println!("  Beautiful melodic contours with memory\n");

    let recaman_seq = sequences::recaman::generate(24);
    let recaman_freqs = sequences::normalize(&recaman_seq, 220.0, 880.0);

    comp.instrument("recaman", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(77.0)
        .notes(&recaman_freqs, 0.2);

    // ===== PART 20: VAN DER CORPUT (QUASI-RANDOM) =====
    println!("Part 20: Van der Corput Sequence\n");
    println!("  Quasi-random low-discrepancy sequence");
    println!("  Better distribution than pure random\n");

    // Use for note placement
    let quasi_positions = sequences::van_der_corput::generate(32, 2);
    for (i, &pos) in quasi_positions.iter().enumerate() {
        let freq = 300.0 + pos * 500.0;
        comp.instrument("quasi", &Instrument::synth_lead())
            .at(82.0 + i as f32 * 0.125)
            .note(&[freq], 0.1);
    }

    // ===== PART 21: CELLULAR AUTOMATON =====
    println!("Part 21: Cellular Automaton (Rule 30)\n");
    println!("  Chaotic patterns from simple rules");
    println!("  Used in Mathematica's random number generator!\n");

    // Rule 30 - chaotic
    let rule30 = sequences::cellular_automaton::generate(30, 8, 16, None);
    for (gen_idx, generation) in rule30.iter().take(4).enumerate() {
        let rhythm: Vec<usize> = generation
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| i)
            .collect();

        comp.instrument(&format!("ca30_{}", gen_idx), &Instrument::pluck())
            .at(90.5 + gen_idx as f32 * 2.0)
            .drum_grid(16, 0.125)
            .kick(&rhythm);
    }

    // Rule 90 - Sierpinski fractal
    let rule90 = sequences::cellular_automaton::generate(90, 4, 16, None);
    let sierpinski_rhythm: Vec<usize> = rule90[3]
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();

    comp.track("ca90_fractal")
        .at(98.5)
        .drum_grid(16, 0.125)
        .snare(&sierpinski_rhythm)
        .hihat(&sequences::euclidean::generate(13, 16));

    // ===== PART 22: GENERATIVE COMPOSITION =====
    println!("\nPart 22: Full Generative Piece\n");
    println!("  Combining all sequences into a complete composition\n");

    // Bass: Recamán (interesting contour)
    let bass_recaman = sequences::recaman::generate(16);
    let bass_freqs = sequences::normalize(&bass_recaman, 55.0, 110.0);

    comp.instrument("gen_bass", &Instrument::sub_bass())
        .at(102.0)
        .notes(&bass_freqs, 0.5);

    // Melody: Random walk (organic variation)
    let melody_walk = sequences::bounded_walk::generate(550.0, 40.0, 440.0, 880.0, 16);

    comp.instrument("gen_melody", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(102.0)
        .notes(&melody_walk, 0.4);

    // Chords: Harmonic series
    let chord_harmonics = harmonic_series(82.41, 12); // E2

    comp.instrument("gen_chords", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .chorus(Chorus::new(0.3, 2.5, 0.3))
        .at(102.0)
        .note(&chord_harmonics[3..6], 4.0)
        .note(&chord_harmonics[4..7], 4.0);

    // Rhythm: Thue-Morse for non-repetitive patterns
    let tm_rhythm = sequences::thue_morse::generate(16);
    let tm_rhythm_hits: Vec<usize> = tm_rhythm
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();

    comp.instrument("gen_drums", &Instrument::pluck())
        .at(102.0)
        .drum_grid(16, 0.125)
        .kick(&sequences::euclidean::generate(4, 16))
        .snare(&tm_rhythm_hits)
        .hihat(&sequences::euclidean::generate(13, 16));

    println!("\n▶️  Playing sequences showcase...\n");
    println!("    Duration: ~110 seconds\n");
    println!("    📊 Sequences Demonstrated:");
    println!("    1. Fibonacci (1, 1, 2, 3, 5, 8...)");
    println!("    2. Primes (2, 3, 5, 7, 11, 13...)");
    println!("    3. Collatz (3n+1 problem)");
    println!("    4. Harmonic series (f, 2f, 3f...)");
    println!("    5. Golden ratio powers (φ^n)");
    println!("    6. Golden ratio rhythm (Beatty sequence)");
    println!("    7. Euclidean rhythms");
    println!("    8. Triangular numbers (1, 3, 6, 10...)");
    println!("    9. Powers of two (1, 2, 4, 8...)");
    println!("    10. Arithmetic sequences");
    println!("    11. Geometric sequences");
    println!("    12. Golden sections");
    println!("    13. Combined sequences");
    println!("    14. Sequence-driven rhythm density");
    println!("    15. Spectral chords from harmonics");
    println!("    16. Logistic map (chaos theory)");
    println!("    17. Random walk (Brownian motion)");
    println!("    18. Thue-Morse sequence (fair division)");
    println!("    19. Recamán's sequence (spiraling back-and-forth)");
    println!("    20. Van der Corput (quasi-random low-discrepancy)");
    println!("    21. Cellular Automaton (Rule 30 & 90)");
    println!("    22. Complete generative composition\n");
    println!("    💡 Applications:");
    println!("    • Algorithmic composition");
    println!("    • Generative music systems");
    println!("    • Procedural game soundtracks");
    println!("    • Mathematical music exploration");
    println!("    • Educational demonstrations\n");

    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Sequences Showcase Complete!\n");
    println!("💡 Next Steps:");
    println!("   • Combine sequences in new ways");
    println!("   • Use normalize() to map to different ranges");
    println!("   • Apply sequences to rhythm, melody, harmony, and form");
    println!("   • Explore chaos with Collatz sequences");
    println!("   • Build spectral music with harmonic series");
    println!("   • Create natural-sounding patterns with golden ratio\n");
    println!("📚 Available Functions:");
    println!("   sequences::fibonacci::generate(n)");
    println!("   sequences::primes::generate(n)");
    println!("   sequences::collatz::generate(start, max_terms)");
    println!("   sequences::harmonic_series(fundamental, n)");
    println!("   sequences::golden_ratio(n)");
    println!("   sequences::golden_ratio_rhythm::generate(steps)");
    println!("   sequences::golden_sections(value, divisions)");
    println!("   sequences::euclidean::generate(pulses, steps)");
    println!("   sequences::triangular::generate(n)");
    println!("   sequences::powers_of_two::generate(n)");
    println!("   sequences::arithmetic::generate(start, step, n)");
    println!("   sequences::geometric::generate(start, ratio, n)");
    println!("   sequences::logistic_map::generate(r, initial, n)");
    println!("   sequences::random_walk::generate(start, step_size, steps)");
    println!("   sequences::bounded_walk::generate(start, step, min, max, steps)");
    println!("   sequences::thue_morse::generate(n)");
    println!("   sequences::recaman::generate(n)");
    println!("   sequences::van_der_corput::generate(n, base)");
    println!("   sequences::cellular_automaton::generate(rule, steps, width, initial_state)");
    println!("   sequences::normalize(seq, min, max)\n");

    Ok(())
}
