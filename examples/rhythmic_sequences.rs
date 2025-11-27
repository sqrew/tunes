use tunes::prelude::*;
use tunes::sequences;

/// Showcase of rhythmic pattern generators
///
/// This example demonstrates rhythm-focused sequences:
/// - Euclidean Rhythms: Evenly distributed pulses (used worldwide!)
/// - Golden Ratio Rhythm: Non-periodic but balanced (Beatty sequence)
/// - Thue-Morse Sequence: Fair binary rhythms
/// - Cantor Set: Fractal rhythmic patterns
/// - Shepard Tone: Circular pitch for infinite rises/falls
fn main() -> anyhow::Result<()> {
    println!("\n🥁 Rhythmic Sequences for Music\n");
    println!("Mathematical patterns for drums, percussion, and timing\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // ===== EUCLIDEAN RHYTHMS - WORLD MUSIC =====
    println!("1. Euclidean Rhythms\n");
    println!("   Evenly distribute k pulses over n steps");
    println!("   Used in traditional music worldwide!\n");

    // Cuban Tresillo: 3 pulses in 8 steps
    println!("   • Tresillo (3, 8): Cuban son, salsa [x..x..x.]");
    let tresillo = sequences::euclidean::generate(3, 8);

    comp.track("tresillo")
        .drum_grid(8, 0.25, |g| g
            .sound(DrumType::Kick, &tresillo)
            .sound(DrumType::HiHatClosed, &sequences::euclidean::generate(8, 8)));

    // Cuban Cinquillo: 5 pulses in 8 steps
    println!("   • Cinquillo (5, 8): Cuban rumba [x.xx.xx.]");
    let cinquillo = sequences::euclidean::generate(5, 8);

    comp.track("cinquillo")
        .at(2.0)
        .drum_grid(8, 0.25, |g| g
            .sound(DrumType::Snare, &cinquillo));

    // Bossa Nova: 5 pulses in 16 steps
    println!("   • Bossa Nova (5, 16): Brazilian rhythm");
    let bossa = sequences::euclidean::generate(5, 16);

    comp.track("bossa")
        .at(4.0)
        .drum_grid(16, 0.125, |g| g
            .sound(DrumType::Kick, &bossa)
            .sound(DrumType::HiHatClosed, &sequences::euclidean::generate(8, 16)));

    // Soukous: 4 pulses in 16 steps
    println!("   • Soukous (4, 16): Central African dance");
    let soukous = sequences::euclidean::generate(4, 16);

    comp.track("soukous")
        .at(6.0)
        .drum_grid(16, 0.125, |g| g
            .sound(DrumType::Kick, &soukous)
            .sound(DrumType::Snare, &sequences::euclidean::generate(3, 16)));

    // Complex polyrhythm: 5 against 7 against 11
    println!("   • Polyrhythm (5, 7, 11 over 16): Complex groove\n");

    comp.track("polyrhythm")
        .at(8.0)
        .drum_grid(16, 0.125, |g| g
            .sound(DrumType::Kick, &sequences::euclidean::generate(5, 16))
            .sound(DrumType::Snare, &sequences::euclidean::generate(7, 16))
            .sound(DrumType::HiHatClosed, &sequences::euclidean::generate(11, 16)));

    // ===== GOLDEN RATIO RHYTHM =====
    println!("2. Golden Ratio Rhythm (Beatty Sequence)\n");
    println!("   Based on φ ≈ 1.618... (golden ratio)");
    println!("   Non-periodic rhythm - never quite repeats");
    println!("   Organic and flowing, used by Xenakis\n");

    let phi_rhythm = sequences::golden_ratio_rhythm::generate(32);

    comp.track("golden_rhythm")
        .at(10.0)
        .drum_grid(32, 0.125, |g| g
            .sound(DrumType::Kick, &phi_rhythm)
            .sound(DrumType::HiHatClosed, &sequences::euclidean::generate(16, 32)));

    // ===== THUE-MORSE SEQUENCE =====
    println!("3. Thue-Morse Sequence (Fair Division)\n");
    println!("   Self-similar binary sequence: 0,1,1,0,1,0,0,1...");
    println!("   Creates non-repetitive but balanced rhythms");
    println!("   No same-length consecutive runs\n");

    let thue_morse = sequences::thue_morse::generate(32);
    let tm_hits: Vec<usize> = thue_morse
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();

    comp.track("thue_morse")
        .at(14.0)
        .drum_grid(32, 0.125, |g| g
            .sound(DrumType::Kick, &tm_hits)
            .sound(DrumType::Snare, &sequences::euclidean::generate(5, 32)));

    // Use Thue-Morse for hi-hat open/closed pattern
    println!("   Using Thue-Morse for timbre alternation (open/closed hi-hat)\n");

    let closed_hits: Vec<usize> = thue_morse
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 0)
        .map(|(i, _)| i)
        .collect();
    let open_hits: Vec<usize> = thue_morse
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();

    comp.track("tm_hihats")
        .at(18.0)
        .drum_grid(32, 0.125, |g| g
            .sound(DrumType::HiHatClosed, &closed_hits)
            .sound(DrumType::HiHatOpen, &open_hits));

    // ===== CANTOR SET - FRACTAL RHYTHMS =====
    println!("4. Cantor Set (Fractal Rhythm)\n");
    println!("   Recursive subdivision: remove middle third");
    println!("   Creates self-similar fractal patterns");
    println!("   Sparse, distributed rhythms\n");

    let cantor = sequences::cantor_set::generate(4, 64); // 4 iterations, 64 steps

    // Cantor set returns 1s and 0s, extract positions of 1s
    let cantor_hits: Vec<usize> = cantor
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();

    comp.track("cantor_set")
        .at(22.0)
        .drum_grid(64, 0.0625, |g| g
            .sound(DrumType::Kick, &cantor_hits)
            .sound(DrumType::HiHatClosed, &sequences::euclidean::generate(32, 64)));

    // ===== SHEPARD TONE - CIRCULAR PITCH =====
    println!("5. Shepard Tone (Circular Pitch)\n");
    println!("   Illusion of infinitely rising or falling pitch");
    println!("   Smooth transitions create cyclical patterns\n");

    // Ascending Shepard tone
    let shepard_up = sequences::shepard_tone::generate(32, 12, true);
    let shepard_freqs = sequences::normalize(
        &shepard_up.iter().map(|&x| x as u32).collect::<Vec<_>>(),
        200.0,
        800.0,
    );

    comp.instrument("shepard", &Instrument::synth_lead())
        .at(26.0)
        .notes(&shepard_freqs, 0.125);

    // ===== RHYTHM DENSITY VARIATIONS =====
    println!("6. Rhythm Density Variations\n");
    println!("   Using Euclidean rhythms with increasing density\n");

    for density in [2, 3, 5, 8, 13] {
        let rhythm = sequences::euclidean::generate(density, 16);
        println!("   • Density {}/16", density);

        comp.track(&format!("density_{}", density))
            .at(30.0 + (density - 2) as f32 * 2.0)
            .drum_grid(16, 0.125, |g| g
                .sound(DrumType::Kick, &rhythm));
    }

    // ===== LAYERED POLYRHYTHMS =====
    println!("\n7. Complex Polyrhythmic Texture\n");
    println!("   Multiple Euclidean patterns layered together\n");

    comp.track("poly_layer1")
        .at(40.0)
        .drum_grid(16, 0.125, |g| g
            .sound(DrumType::Kick, &sequences::euclidean::generate(4, 16)));

    comp.track("poly_layer2")
        .at(40.0)
        .drum_grid(16, 0.125, |g| g
            .sound(DrumType::Snare, &sequences::euclidean::generate(5, 16)));

    comp.track("poly_layer3")
        .at(40.0)
        .drum_grid(16, 0.125, |g| g
            .sound(DrumType::HiHatClosed, &sequences::euclidean::generate(7, 16)));

    comp.track("poly_layer4")
        .at(40.0)
        .drum_grid(16, 0.125, |g| g
            .sound(DrumType::Tom, &sequences::euclidean::generate(3, 16)));

    // Golden ratio rhythm on top
    comp.track("poly_golden")
        .at(40.0)
        .drum_grid(16, 0.125, |g| g
            .sound(DrumType::Clap, &sequences::golden_ratio_rhythm::generate(16)));

    // ===== RHYTHMIC TRANSFORMATION =====
    println!("8. Rhythmic Transformation\n");
    println!("   Gradually transforming one pattern into another\n");

    // Start with simple (2, 8), evolve to complex (7, 8)
    for i in 2..=7 {
        let evolving = sequences::euclidean::generate(i, 8);
        comp.track(&format!("evolve_{}", i))
            .at(44.0 + (i - 2) as f32 * 1.0)
            .drum_grid(8, 0.25, |g| g
                .sound(DrumType::Kick, &evolving)
                .sound(DrumType::HiHatClosed, &sequences::euclidean::generate(8, 8)));
    }

    println!("\n▶️  Playing rhythmic sequences...\n");
    println!("    Duration: ~50 seconds\n");

    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Rhythmic Sequences Complete!\n");
    println!("💡 Key Takeaways:");
    println!("   • Euclidean rhythms distribute pulses evenly");
    println!("   • Used in traditional music worldwide (Cuba, Brazil, Africa)");
    println!("   • Golden ratio rhythm never quite repeats");
    println!("   • Thue-Morse creates balanced, non-repetitive patterns");
    println!("   • Cantor set creates fractal, self-similar rhythms");
    println!("   • Shepard tone creates circular pitch illusions\n");
    println!("🥁 Rhythmic Applications:");
    println!("   • Drum patterns and percussion");
    println!("   • Polyrhythmic textures");
    println!("   • Evolving groove patterns");
    println!("   • Non-repetitive but structured rhythms");
    println!("   • World music and traditional patterns\n");
    println!("📚 Common Euclidean Patterns:");
    println!("   E(3,8) = Cuban Tresillo [x..x..x.]");
    println!("   E(5,8) = Cuban Cinquillo [x.xx.xx.]");
    println!("   E(5,16) = Bossa Nova clave");
    println!("   E(7,12) = West African bell pattern");
    println!("   E(5,12) = Arabic rhythm");
    println!("   E(7,16) = Brazilian samba\n");
    println!("📚 Try Next:");
    println!("   cargo run --example generative_sequences");
    println!("   cargo run --example mathematical_sequences\n");

    Ok(())
}
