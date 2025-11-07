// Algorithmic Symphony - A Showcase of Mathematical Music
//
// This composition demonstrates the comprehensive sequence library,
// weaving together chaos theory, fractal noise, polyrhythms, and
// classic mathematical sequences into a cohesive musical journey.
//
// Structure (with overlapping sections):
// I.   Emergence (0-18s)     - Lorenz attractor rises from silence
// II.  Evolution (12-48s)    - Perlin noise textures evolve (overlaps I and III)
// III. Complexity (42-74s)   - Polyrhythms and circle map patterns (overlaps II and IV)
// IV.  Resolution (68-100s)  - Classic sequences converge (overlaps III)

use tunes::prelude::*;
use tunes::sequences;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(100.0));

    println!("=== ALGORITHMIC SYMPHONY ===");
    println!("A journey through mathematical music");
    println!("Duration: ~100 seconds with overlapping sections\n");

    // ==================
    // I. EMERGENCE (0-18s) - Lorenz Attractor Rising
    // ==================
    println!("I. EMERGENCE (0-18s) - The butterfly emerges from chaos");

    let butterfly = sequences::lorenz_butterfly(150);

    // X coordinates for main melody (D minor)
    let x_vals: Vec<f32> = butterfly.iter().map(|(x, _, _)| *x).collect();
    let melody = sequences::map_to_scale_f32(&x_vals, &sequences::Scale::minor(), D4, 2);

    // Y coordinates for volume automation (breathing)
    let y_vals: Vec<f32> = butterfly.iter().map(|(_, y, _)| *y).collect();
    let volumes = sequences::normalize_f32(&y_vals, 0.2, 0.7);

    // Z coordinates for harmonic drone
    let z_vals: Vec<f32> = butterfly.iter().map(|(_, _, z)| *z).collect();
    let drone = sequences::normalize_f32(&z_vals, D3 * 0.5, D3 * 1.5);

    // Main Lorenz melody (0-12s)
    for i in 0..100 {
        comp.track("lorenz_lead")
            .at(i as f32 * 0.12)
            .volume(volumes[i])
            .note(&[melody[i]], 0.1);
    }

    // Evolving drone underneath (0-15s)
    for i in (0..125).step_by(4) {
        comp.track("lorenz_drone")
            .at(i as f32 * 0.12)
            .volume(0.15)
            .note(&[drone[i]], 0.5);
    }

    // Sparse Euclidean kick (4-18s) - extends into section II
    let kick_pattern = sequences::euclidean(7, 56);
    comp.track("emerge_kick")
        .at(4.0)
        .drum_grid(56, 0.25)
        .kick(&kick_pattern);

    // ==================
    // II. EVOLUTION (12-48s) - Perlin Noise Textures
    // ==================
    let section_2_start = 12.0;
    println!("\nII. EVOLUTION (12-48s) - Organic textures emerge");

    // Perlin noise for evolving pentatonic melody (12-30s)
    let perlin_melody = sequences::perlin_noise(777, 0.12, 4, 0.5, 144);
    let pentatonic = sequences::map_to_scale_f32(
        &perlin_melody,
        &sequences::Scale::minor_pentatonic(),
        A3,
        3
    );

    for i in 0..144 {
        comp.track("perlin_melody")
            .at(section_2_start + i as f32 * 0.125)
            .volume(0.4)
            .note(&[pentatonic[i]], 0.1);
    }

    // Second Perlin layer with different seed (harmony) (12-30s)
    let perlin_harmony = sequences::perlin_noise(888, 0.08, 3, 0.5, 144);
    let harmony = sequences::map_to_scale_f32(
        &perlin_harmony,
        &sequences::Scale::minor_pentatonic(),
        A4,
        2
    );

    for i in 0..144 {
        comp.track("perlin_harmony")
            .at(section_2_start + i as f32 * 0.125)
            .volume(0.25)
            .note(&[harmony[i]], 0.08);
    }

    // Perlin noise for stereo panning automation (18-36s)
    let pan_noise = sequences::perlin_noise_bipolar(555, 0.15, 2, 0.5, 72);
    for i in 0..72 {
        let freq = sequences::map_to_scale_f32(
            &vec![perlin_melody[i * 2]],
            &sequences::Scale::minor(),
            E5,
            1
        )[0];

        comp.track(&format!("perlin_pan_{}", i))
            .at(section_2_start + 6.0 + i as f32 * 0.25)
            .pan(pan_noise[i])
            .volume(0.3)
            .note(&[freq], 0.2);
    }

    // Fibonacci bass enters (22-42s) - extends into section III
    let fib = sequences::fibonacci(20);
    let fib_bass = sequences::map_to_scale(&fib, &sequences::Scale::minor_pentatonic(), F2, 2);

    for i in 0..20 {
        comp.track("fib_bass")
            .at(section_2_start + 10.0 + i as f32 * 1.0)
            .volume(0.5)
            .note(&[fib_bass[i]], 0.8);
    }

    // Euclidean hi-hat pattern (24-42s)
    let hihat_pattern = sequences::euclidean(11, 72);
    comp.track("evolution_hihat")
        .at(section_2_start + 12.0)
        .drum_grid(72, 0.25)
        .hihat(&hihat_pattern);

    // ==================
    // III. COMPLEXITY (42-74s) - Polyrhythms and Circle Map
    // ==================
    let section_3_start = 42.0;
    println!("\nIII. COMPLEXITY (42-74s) - Mathematical intricacy unfolds");

    // Circle map rhythm (golden ratio, quasi-periodic) (42-58s)
    let circle_hits = sequences::circle_map_to_hits(0.618, 1.5, 0.0, 128, 0.5);
    comp.track("circle_kick")
        .at(section_3_start)
        .drum_grid(128, 0.125)
        .kick(&circle_hits);

    // Circle map with different parameters for snare (42-58s)
    let circle_snare = sequences::circle_map_to_hits(0.4, 1.2, 0.0, 128, 0.6);
    comp.track("circle_snare")
        .at(section_3_start)
        .drum_grid(128, 0.125)
        .snare(&circle_snare);

    // Circle map melody (quantized to blues scale) (42-58s)
    let circle_phases = sequences::circle_map(0.42, 1.8, 0.0, 128);
    let blues_melody = sequences::map_to_scale_f32(
        &circle_phases,
        &sequences::Scale::blues(),
        C5,
        2
    );

    for i in 0..128 {
        comp.track("circle_blues")
            .at(section_3_start + i as f32 * 0.125)
            .volume(0.45)
            .note(&[blues_melody[i]], 0.1);
    }

    // Polyrhythm layers (3:4:5) (50-66s)
    let poly_345 = sequences::polyrhythm(&[3, 4, 5], 60);
    // Repeat the pattern 2 times for continuity
    for rep in 0..2 {
        comp.track(&format!("poly_drums_{}", rep))
            .at(section_3_start + 8.0 + rep as f32 * 8.0)
            .drum_grid(60, 0.133)
            .kick(&poly_345[0])     // 3 hits
            .snare(&poly_345[1])    // 4 hits
            .hihat(&poly_345[2]);   // 5 hits
    }

    // Complex 5:7 polyrhythm (58-70s) - repeated pattern
    let (poly_57, len_57) = sequences::polyrhythm_cycle(&[5, 7]);
    for rep in 0..4 {
        comp.track(&format!("poly_57_{}", rep))
            .at(section_3_start + 16.0 + rep as f32 * 3.0)
            .drum_grid(len_57, 0.08)
            .clap(&poly_57[0])
            .cowbell(&poly_57[1]);
    }

    // Collatz conjecture countermelody (46-62s)
    let collatz = sequences::collatz(27, 64);
    let collatz_melody = sequences::map_to_scale(
        &collatz,
        &sequences::Scale::dorian(),
        G4,
        2
    );

    for i in 0..64 {
        comp.track("collatz")
            .at(section_3_start + 4.0 + i as f32 * 0.25)
            .volume(0.35)
            .note(&[collatz_melody[i]], 0.2);
    }

    // ==================
    // IV. RESOLUTION (68-100s) - Classic Sequences Converge
    // ==================
    let section_4_start = 68.0;
    println!("\nIV. RESOLUTION (68-100s) - Order emerges from complexity");

    // Harmonic series drone enters first (68-88s) - foundational
    let harmonics = sequences::harmonic_series(C2, 8);
    comp.track("harmonic_resolution")
        .at(section_4_start)
        .volume(0.35)
        .note(&harmonics, 20.0);

    // Prime numbers in C major (ascending) (72-88s)
    let primes = sequences::primes(20);
    let prime_melody = sequences::map_to_scale(
        &primes,
        &sequences::Scale::major(),
        C4,
        3
    );

    for i in 0..20 {
        comp.track("primes")
            .at(section_4_start + 4.0 + i as f32 * 0.4)
            .volume(0.5)
            .note(&[prime_melody[i]], 0.35);
    }

    // Golden ratio harmony (76-88s)
    let golden = sequences::golden_ratio(16);
    let golden_harmony = sequences::normalize_f32(&golden, E4, E6);

    for i in 0..16 {
        comp.track("golden")
            .at(section_4_start + 8.0 + i as f32 * 0.5)
            .volume(0.3)
            .note(&[golden_harmony[i]], 0.45);
    }

    // Final Euclidean rhythm (7 in 16) (76-92s)
    let final_pattern = sequences::euclidean(7, 32);
    comp.track("final_rhythm")
        .at(section_4_start + 8.0)
        .drum_grid(32, 0.5)
        .kick(&final_pattern);

    // Descending pattern (resolution approach) (84-92s)
    let ascent = sequences::arithmetic(1, 1, 32);
    let descent: Vec<u32> = ascent.iter().rev().cloned().collect();
    let descent_freqs = sequences::normalize(&descent, 200.0, 500.0);

    for i in 0..32 {
        comp.track("descent")
            .at(section_4_start + 16.0 + i as f32 * 0.25)
            .volume(0.25)
            .note(&[descent_freqs[i]], 0.2);
    }

    // Final resolution chords (92-100s)
    // D minor chord (92-96s)
    comp.track("resolution_dm")
        .at(section_4_start + 24.0)
        .volume(0.5)
        .note(&[D3, F3, A3], 4.0);

    // D major chord (96-100s) - the resolution!
    comp.track("resolution_dmaj")
        .at(section_4_start + 28.0)
        .volume(0.6)
        .note(&[D3, FS3, A3], 4.0);

    // Final root note (98-100s)
    comp.track("final_root")
        .at(section_4_start + 30.0)
        .volume(0.4)
        .note(&[D2], 2.0);

    println!("\n=== COMPOSITION COMPLETE ===");
    println!("Total duration: ~100 seconds");
    println!("\nSequences featured:");
    println!("  • Lorenz attractor (continuous chaos)");
    println!("  • Perlin noise (organic modulation)");
    println!("  • Circle map (phase-locked rhythms)");
    println!("  • Polyrhythms (3:4:5, 5:7)");
    println!("  • Fibonacci, Collatz, Primes");
    println!("  • Golden ratio, Harmonic series");
    println!("  • Euclidean rhythms");
    println!("\nStructure: 4 overlapping movements creating continuous flow");
    println!("This is the power of mathematical music!\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
