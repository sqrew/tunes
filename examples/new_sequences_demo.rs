// New Sequences Demo - Lorenz Attractor, Circle Map, and Polyrhythms
//
// Demonstrates three new powerful sequence generators:
// 1. Lorenz Attractor - Smooth chaotic melodies from 3D continuous system
// 2. Circle Map - Phase-locking rhythms with Arnol'd tongues
// 3. Polyrhythm - Mathematical cross-rhythms (3:4, 5:7, etc.)

use tunes::prelude::*;
use tunes::sequences;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    println!("=== NEW SEQUENCE GENERATORS ===\n");

    // ==================
    // 1. LORENZ ATTRACTOR - Butterfly Melody
    // ==================
    println!("1. LORENZ ATTRACTOR");
    println!("   Continuous chaotic system creating smooth, flowing melodies");
    println!("   The iconic 'butterfly' attractor from chaos theory\n");

    // Generate Lorenz attractor path (classic parameters)
    let butterfly = sequences::lorenz_butterfly(200);

    // Extract X coordinates for melody and normalize to pitch range
    let x_vals: Vec<f32> = butterfly.iter().map(|(x, _, _)| *x).collect();
    let melody = sequences::normalize_f32(&x_vals, 220.0, 880.0);

    // Extract Y coordinates for volume automation
    let y_vals: Vec<f32> = butterfly.iter().map(|(_, y, _)| *y).collect();
    let volumes = sequences::normalize_f32(&y_vals, 0.4, 0.8);

    // Play the butterfly melody
    println!("   Playing {} notes following the butterfly attractor...", melody.len());
    for i in 0..melody.len() {
        comp.track("lorenz")
            .at(i as f32 * 0.125)
            .volume(volumes[i])
            .note(&[melody[i]], 0.1);
    }

    // ==================
    // 2. CIRCLE MAP - Phase-Locked Rhythms
    // ==================
    let circle_start = 26.0;
    println!("\n2. CIRCLE MAP (Arnol'd Tongue)");
    println!("   Phase-locking rhythm generator");
    println!("   Creates patterns that smoothly transition between locked and chaotic\n");

    // Example A: Pure rotation (K=0) - perfectly periodic
    println!("   A. Pure rotation (K=0) - 1:4 pattern");
    let rotation_hits = sequences::circle_map_to_hits(0.25, 0.0, 0.0, 16, 0.5);
    comp.track("circle_pure")
        .at(circle_start)
        .drum_grid(16, 0.25)
        .kick(&rotation_hits);

    // Example B: Critical coupling (K=1) - interesting mode-locking
    println!("   B. Critical coupling (K=1) - 1:3 with variation");
    let critical_hits = sequences::circle_map_to_hits(0.333, 1.0, 0.0, 16, 0.5);
    comp.track("circle_critical")
        .at(circle_start + 4.5)
        .drum_grid(16, 0.25)
        .snare(&critical_hits);

    // Example C: High coupling (K=2) - complex chaos
    println!("   C. High coupling (K=2) - golden ratio rhythm");
    let golden_hits = sequences::circle_map_to_hits(0.618, 2.0, 0.0, 32, 0.5);
    comp.track("circle_golden")
        .at(circle_start + 9.0)
        .drum_grid(32, 0.125)
        .hihat(&golden_hits);

    // Example D: Hocket pattern (complementary rhythms)
    println!("   D. Hocket pattern - kick/snare call-and-response");
    let (kick_hits, snare_hits) = sequences::circle_map_hocket(0.4, 1.5, 0.0, 16, 0.5);
    comp.track("circle_hocket")
        .at(circle_start + 13.0)
        .drum_grid(16, 0.25)
        .kick(&kick_hits)
        .snare(&snare_hits);

    // ==================
    // 3. POLYRHYTHMS - Mathematical Cross-Rhythms
    // ==================
    let poly_start = 44.0;
    println!("\n3. POLYRHYTHM GENERATOR");
    println!("   Mathematical polyrhythms - multiple simultaneous meters");
    println!("   Essential for complex rhythmic compositions\n");

    // Example A: Classic 3:4 (hemiola)
    println!("   A. Classic 3:4 polyrhythm over 12 steps");
    let patterns_34 = sequences::polyrhythm(&[3, 4], 12);
    comp.track("poly_34")
        .at(poly_start)
        .drum_grid(12, 0.25)
        .kick(&patterns_34[0])   // 3 hits
        .snare(&patterns_34[1]); // 4 hits

    // Example B: Complex 5:7 polyrhythm
    println!("   B. Complex 5:7 polyrhythm over 35 steps (LCM)");
    let (patterns_57, len_57) = sequences::polyrhythm_cycle(&[5, 7]);
    comp.track("poly_57")
        .at(poly_start + 3.5)
        .drum_grid(len_57, 0.125)
        .kick(&patterns_57[0])   // 5 hits
        .snare(&patterns_57[1]); // 7 hits

    // Example C: Triple polyrhythm 3:4:5
    println!("   C. Triple polyrhythm 3:4:5 over 60 steps");
    let patterns_345 = sequences::polyrhythm(&[3, 4, 5], 60);
    comp.track("poly_345")
        .at(poly_start + 8.0)
        .drum_grid(60, 0.1)
        .kick(&patterns_345[0])     // 3 hits
        .snare(&patterns_345[1])    // 4 hits
        .hihat(&patterns_345[2]);   // 5 hits

    // Example D: Using polyrhythm timings for melodic patterns
    println!("   D. Melodic polyrhythm - 4:5 over 2 beats");
    let timings = sequences::polyrhythm_timings(&[4, 5], 2.0);

    // Voice 1: 4 hits
    for &t in &timings[0] {
        comp.track("poly_melody_1")
            .at(poly_start + 14.0 + t)
            .note(&[C5], 0.08);
    }

    // Voice 2: 5 hits (different pitch)
    for &t in &timings[1] {
        comp.track("poly_melody_2")
            .at(poly_start + 14.0 + t)
            .note(&[E5], 0.08);
    }

    // ==================
    // 4. COMBINATION - Using all three together
    // ==================
    let combo_start = 62.0;
    println!("\n4. COMBINATION EXAMPLE");
    println!("   Lorenz melody + Circle map rhythm + Polyrhythm drums\n");

    // Lorenz melody (shorter version)
    let combo_butterfly = sequences::lorenz_butterfly(64);
    let combo_x: Vec<f32> = combo_butterfly.iter().map(|(x, _, _)| *x).collect();
    let combo_melody = sequences::normalize_f32(&combo_x, 330.0, 660.0);

    for i in 0..combo_melody.len() {
        comp.track("combo_melody")
            .at(combo_start + i as f32 * 0.125)
            .note(&[combo_melody[i]], 0.1);
    }

    // Circle map rhythm (kick pattern)
    let combo_circle = sequences::circle_map_to_hits(0.4, 1.2, 0.0, 32, 0.5);
    comp.track("combo_circle")
        .at(combo_start)
        .drum_grid(32, 0.25)
        .kick(&combo_circle);

    // Polyrhythm (hi-hats and snare)
    let combo_poly = sequences::polyrhythm(&[4, 3], 12);
    for i in 0..6 {
        // Repeat pattern 6 times
        let offset = combo_start + i as f32 * 3.0;
        comp.track(&format!("combo_poly_{}", i))
            .at(offset)
            .drum_grid(12, 0.25)
            .hihat(&combo_poly[0])
            .snare(&combo_poly[1]);
    }

    // ==================
    // Summary
    // ==================
    println!("\n=== SEQUENCE LIBRARY SUMMARY ===");
    println!("Lorenz Attractor:");
    println!("  - Smooth chaotic melodies from 3D continuous system");
    println!("  - Use X/Y/Z for pitch/volume/filter automation");
    println!("  - Perfect for ambient, experimental, generative music");
    println!();
    println!("Circle Map (Arnol'd Tongue):");
    println!("  - Phase-locking rhythm patterns");
    println!("  - K=0: pure rotation, K>1: mode-locked chaos");
    println!("  - Golden ratio (Ω=0.618) creates non-repeating patterns");
    println!();
    println!("Polyrhythm:");
    println!("  - Mathematical cross-rhythms (3:4, 5:7, 7:11, etc.)");
    println!("  - LCM calculation for complete cycles");
    println!("  - Essential for metric complexity");
    println!();
    println!("Total algorithmic sequences: 40+ generators!");
    println!("  - Mathematical: Fibonacci, primes, Collatz, etc.");
    println!("  - Chaotic: Logistic, tent, sine, Hénon, Baker's maps");
    println!("  - Fractal: L-systems, Cantor, cellular automata");
    println!("  - Rhythmic: Euclidean, golden ratio, Shepard, circle map");
    println!("  - Generative: Markov, random walks, Recamán");
    println!("  - Continuous: Lorenz attractor (NEW!)");
    println!("\nExplore the sequences module for endless musical possibilities!\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
