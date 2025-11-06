use std::collections::HashMap;
use tunes::prelude::*;
use tunes::sequences;

/// Showcase of advanced generative algorithms
///
/// This example demonstrates complex algorithmic sequence generators:
/// - Cellular Automaton: Rule-based evolution (Rule 30, Rule 90, etc.)
/// - L-Systems: Fractal growth patterns (Lindenmayer systems)
/// - Markov Chains: Probabilistic state transitions
/// - Recamán Sequence: Backward-looking spiraling patterns
/// - Van der Corput: Quasi-random low-discrepancy sequences
fn main() -> anyhow::Result<()> {
    println!("\n Generative Sequences: Advanced Algorithms\n");
    println!("Complex pattern generators for algorithmic composition\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(110.0));

    // ===== CELLULAR AUTOMATON - RULE 30 =====
    println!("1. Cellular Automaton - Rule 30 (Chaotic)\n");
    println!("   Simple rules create complex patterns");
    println!("   Used in Wolfram's random number generator!");
    println!("   Rule 30 = 00011110 in binary\n");

    let rule30 = sequences::cellular_automaton(30, 8, 16, None);

    // Play each generation as a rhythm
    for (gen_idx, generation) in rule30.iter().take(4).enumerate() {
        let rhythm: Vec<usize> = generation
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| i)
            .collect();

        println!("   Generation {}: {:?}", gen_idx + 1, generation);

        comp.instrument(&format!("ca30_gen{}", gen_idx), &Instrument::pluck())
            .at(gen_idx as f32 * 2.0)
            .drum_grid(16, 0.125)
            .kick(&rhythm);
    }

    // ===== CELLULAR AUTOMATON - RULE 90 =====
    println!("\n2. Cellular Automaton - Rule 90 (Sierpinski Triangle)\n");
    println!("   Creates fractal patterns!");
    println!("   Rule 90 = 01011010 in binary");
    println!("   Generates Sierpinski fractal structure\n");

    let rule90 = sequences::cellular_automaton(90, 8, 16, None);

    for (gen_idx, generation) in rule90.iter().take(4).enumerate() {
        let rhythm: Vec<usize> = generation
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| i)
            .collect();

        println!("   Generation {}: {:?}", gen_idx + 1, generation);

        comp.instrument(&format!("ca90_gen{}", gen_idx), &Instrument::synth_lead())
            .at(8.0 + gen_idx as f32 * 2.0)
            .drum_grid(16, 0.125)
            .snare(&rhythm);
    }

    // ===== L-SYSTEM - ALGAE GROWTH =====
    println!("\n3. L-System: Algae Growth Pattern\n");
    println!("   Lindenmayer system - fractal growth from rewriting rules");
    println!("   Axiom: 'A'");
    println!("   Rules: A → AB, B → A");
    println!("   Iterations: A → AB → ABA → ABAAB → ABAABABA...\n");

    let mut algae_rules = HashMap::new();
    algae_rules.insert('A', "AB".to_string());
    algae_rules.insert('B', "A".to_string());

    let lsystem_str = sequences::lsystem("A", &algae_rules, 5);

    println!("   Result: {}\n", lsystem_str);

    // Convert L-system to musical sequence (A=0, B=1 automatically)
    let lsystem_notes = sequences::lsystem_to_sequence(&lsystem_str);

    let lsystem_freqs: Vec<f32> = lsystem_notes
        .iter()
        .map(|&semitones| 440.0 * 2.0f32.powf(semitones as f32 / 12.0))
        .collect();

    comp.instrument("lsystem", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(16.0)
        .notes(&lsystem_freqs, 0.15);

    // ===== L-SYSTEM - CANTOR SET =====
    println!("4. L-System: Cantor Set (Fractal)\n");
    println!("   Axiom: 'A'");
    println!("   Rules: A → ABA, B → BBB");
    println!("   Creates fractal rhythmic patterns\n");

    let mut cantor_rules = HashMap::new();
    cantor_rules.insert('A', "ABA".to_string());
    cantor_rules.insert('B', "BBB".to_string());

    let cantor_lsys = sequences::lsystem("A", &cantor_rules, 3);

    let cantor_notes = sequences::lsystem_to_sequence(&cantor_lsys);

    // A=0, B=1, so filter for A's (0)
    let cantor_hits: Vec<usize> = cantor_notes
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 0)
        .map(|(i, _)| i)
        .collect();

    comp.track("lsystem_cantor")
        .at(19.0)
        .drum_grid(cantor_notes.len(), 0.125)
        .kick(&cantor_hits);

    // ===== MARKOV CHAIN =====
    println!("\n5. Markov Chain: Probabilistic Sequence Generation\n");
    println!("   Learning patterns from training data");
    println!("   Each state has probabilities for next states\n");

    // Simple melody pattern to learn from
    let training_melody = vec![0, 2, 4, 2, 0, 2, 4, 5, 4, 2, 0];
    println!("   Training data: {:?}", training_melody);

    // Build transition table (order 1 = first-order Markov chain)
    let transitions = sequences::build_markov_transitions(&training_melody, 1);

    // Generate new sequence based on learned patterns
    let markov_seq = sequences::markov_chain(&transitions, 0, 20);
    println!("   Generated: {:?}\n", markov_seq);

    let markov_freqs: Vec<f32> = markov_seq
        .iter()
        .map(|&semitones| 440.0 * 2.0f32.powf(semitones as f32 / 12.0))
        .collect();

    comp.instrument("markov", &Instrument::synth_lead())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(23.0)
        .notes(&markov_freqs, 0.25);

    // ===== RECAMÁN SEQUENCE =====
    println!("6. Recamán's Sequence (Spiraling Back-and-Forth)\n");
    println!("   a(0) = 0");
    println!("   a(n) = a(n-1) - n if positive and new, else a(n-1) + n");
    println!("   Creates beautiful melodic contours with memory\n");

    let recaman = sequences::recaman(24);
    println!("   First 24 terms: {:?}\n", &recaman[..12]);

    let recaman_freqs = sequences::normalize(&recaman, 220.0, 880.0);

    comp.instrument("recaman", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.6))
        .at(28.0)
        .notes(&recaman_freqs, 0.2);

    // ===== VAN DER CORPUT - QUASI-RANDOM =====
    println!("7. Van der Corput Sequence (Quasi-Random)\n");
    println!("   Low-discrepancy sequence - better than random");
    println!("   Fills space more evenly than random numbers");
    println!("   Used in computer graphics, Monte Carlo methods\n");

    let quasi_random = sequences::van_der_corput(32, 2); // Base 2
    println!("   First 16 values: {:?}\n", &quasi_random[..16]);

    // Use for note placement and pitch
    for (_i, &pos) in quasi_random.iter().take(32).enumerate() {
        let freq = 300.0 + pos * 500.0;
        let time = 33.0 + pos * 8.0; // Quasi-random timing

        comp.instrument("quasi", &Instrument::pluck())
            .at(time)
            .note(&[freq], 0.1);
    }

    // ===== COMBINING GENERATIVE ALGORITHMS =====
    println!("8. Full Generative Composition\n");
    println!("   Combining multiple algorithms together\n");

    // Bass line: Recamán (interesting contour)
    let bass_recaman = sequences::recaman(16);
    let bass_freqs = sequences::normalize(&bass_recaman, 55.0, 110.0);

    comp.instrument("gen_bass", &Instrument::sub_bass())
        .at(42.0)
        .notes(&bass_freqs, 0.5);

    // Melody: Markov chain
    let melody_markov = sequences::markov_chain(&transitions, 0, 16);
    let melody_freqs: Vec<f32> = melody_markov
        .iter()
        .map(|&s| 440.0 * 2.0f32.powf(s as f32 / 12.0))
        .collect();

    comp.instrument("gen_melody", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(42.0)
        .notes(&melody_freqs, 0.5);

    // Rhythm: Cellular Automaton Rule 30
    let rhythm_ca = sequences::cellular_automaton(30, 16, 16, None);
    let ca_rhythm: Vec<usize> = rhythm_ca[0]
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();

    comp.track("gen_rhythm")
        .at(42.0)
        .drum_grid(16, 0.125)
        .kick(&ca_rhythm)
        .snare(&sequences::euclidean(5, 16))
        .hihat(&sequences::euclidean(11, 16));

    // Texture: L-System
    let texture_lsys = sequences::lsystem("A", &algae_rules, 4);
    let texture_seq = sequences::lsystem_to_sequence(&texture_lsys);
    let texture_freqs: Vec<f32> = texture_seq
        .iter()
        .map(|&s| 880.0 * 2.0f32.powf(s as f32 / 12.0))
        .collect();

    comp.instrument("gen_texture", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(42.0)
        .notes(&texture_freqs, 0.25);

    println!("\n▶️  Playing generative sequences...\n");
    println!("    Duration: ~50 seconds\n");

    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Generative Sequences Complete!\n");
    println!("Key Takeaways:");
    println!("• Cellular automata: Simple rules → complex patterns");
    println!("• L-Systems: Fractal growth through string rewriting");
    println!("• Markov chains: Learn patterns from data");
    println!("• Recamán: Back-and-forth spiraling with memory");
    println!("• Van der Corput: Better distribution than random\n");
    println!("Generative Applications:");
    println!("• Algorithmic composition systems");
    println!("• Procedural game music");
    println!("• Evolving soundscapes");
    println!("• Non-repetitive background music");
    println!("• Interactive music systems\n");
    println!("Famous Rules:");
    println!("Rule 30: Chaotic, used in random generation");
    println!("Rule 90: Sierpinski triangle fractal");
    println!("Rule 110: Turing complete!");
    println!("Rule 184: Traffic flow simulation\n");
    println!("Try Next:");
    println!("cargo run --example mathematical_sequences");
    println!("cargo run --example chaotic_sequences\n");
    println!("Pro Tip:");
    println!("Combine these algorithms! Use CA for rhythm,");
    println!("Markov for melody, L-Systems for form,");
    println!("and Recamán for bass lines!\n");

    Ok(())
}
