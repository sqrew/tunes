use tunes::prelude::*;
use tunes::sequences;

/// Showcase of chaotic and fractal sequences for algorithmic composition
///
/// This example demonstrates chaos theory applied to music:
/// - Logistic Map: Classic 1D chaos (r, x_n, x_n+1)
/// - Tent Map: Simple triangular chaos
/// - Sine Map: Smooth chaotic dynamics (very musical!)
/// - Hénon Map: 2D chaotic attractor (x, y outputs)
/// - Baker's Map: Fractal mixing and distribution
/// - Random Walk: Brownian motion (smooth wandering)
/// - Bounded Walk: Constrained random motion
fn main() -> anyhow::Result<()> {
    println!("\n🌀 Chaotic & Fractal Sequences for Music\n");
    println!("Exploring chaos theory, fractals, and complex dynamics\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(110.0));

    // ===== LOGISTIC MAP - CLASSIC 1D CHAOS =====
    println!("1. Logistic Map: x(n+1) = r * x(n) * (1 - x(n))\n");
    println!("   The classic chaos theory equation");
    println!("   r < 3: Stable → r = 3.57: Chaos → r > 3.57: Full chaos\n");

    // Stable behavior (r=2.5)
    let stable_chaos = sequences::logistic_map(2.5, 0.5, 16);
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
        .notes(&stable_freqs, 0.3);

    // Chaotic behavior (r=3.9)
    let chaotic = sequences::logistic_map(3.9, 0.5, 32);
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
        .at(5.0)
        .notes(&chaotic_freqs, 0.15);

    // ===== TENT MAP - SIMPLE TRIANGULAR CHAOS =====
    println!("2. Tent Map: Triangular Chaos\n");
    println!("   Simpler than logistic map, easier to predict");
    println!("   μ = 2.0 for full chaos, creates clean patterns\n");

    let tent = sequences::tent_map(2.0, 0.3, 24);
    let tent_freqs: Vec<f32> = tent.iter().map(|&val| 220.0 + val * 660.0).collect();

    comp.instrument("tent_map", &Instrument::pluck())
        .reverb(Reverb::new(0.5, 0.5, 0.3))
        .at(10.0)
        .notes(&tent_freqs, 0.2);

    // ===== SINE MAP - SMOOTH MUSICAL CHAOS =====
    println!("3. Sine Map: x(n+1) = r * sin(π * x(n))\n");
    println!("   Based on sine waves - VERY musical!");
    println!("   r ∈ [2.5, 2.9] = sweet spot for musical chaos\n");

    let sine_chaos = sequences::sine_map(2.7, 0.4, 32);
    let sine_freqs: Vec<f32> = sine_chaos
        .iter()
        .map(|&val| 220.0 + (val / 3.0) * 660.0)
        .collect();

    comp.instrument("sine_map", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.6))
        .at(15.0)
        .notes(&sine_freqs, 0.2);

    // ===== HÉNON MAP - 2D CHAOTIC ATTRACTOR =====
    println!("4. Hénon Map: 2D Chaotic Attractor\n");
    println!("   x(n+1) = 1 - a*x(n)² + y(n)");
    println!("   y(n+1) = b*x(n)");
    println!("   Classic parameters: a=1.4, b=0.3\n");

    let (henon_x, henon_y) = sequences::henon_map(1.4, 0.3, 0.1, 0.1, 32);

    // Use x for melody
    let henon_melody: Vec<f32> = henon_x
        .iter()
        .map(|&x| 220.0 + (x + 1.5) / 3.0 * 660.0)
        .collect();

    comp.instrument("henon_melody", &Instrument::pluck())
        .chorus(Chorus::new(0.3, 2.5, 0.3))
        .at(20.0)
        .notes(&henon_melody, 0.15);

    // Use y for dynamics/rhythm
    let henon_rhythm: Vec<f32> = henon_y
        .iter()
        .map(|&y| 0.3 + (y + 1.5) / 3.0 * 0.7)
        .collect();

    // Apply dynamics to a drone
    for (i, &volume) in henon_rhythm.iter().take(16).enumerate() {
        comp.instrument("henon_drone", &Instrument::warm_pad())
            .volume(volume)
            .at(20.0 + i as f32 * 0.25)
            .note(&[330.0], 0.2);
    }

    // ===== BAKER'S MAP - FRACTAL MIXING =====
    println!("5. Baker's Map: Fractal Mixing (like kneading dough)\n");
    println!("   Stretching → Cutting → Stacking");
    println!("   Creates fractal distributions in [0, 1]\n");

    let (bakers_x, bakers_y) = sequences::bakers_map(0.3, 0.7, 32);

    // Use x for pitch (already in [0, 1])
    let bakers_pitches: Vec<f32> = bakers_x
        .iter()
        .map(|&x| 220.0 + x * 660.0)
        .collect();

    comp.instrument("bakers_map", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .at(24.0)
        .notes(&bakers_pitches, 0.2);

    // Use y for filter cutoff variation
    let bakers_velocities: Vec<f32> = bakers_y.iter().map(|&y| 0.4 + y * 0.6).collect();

    for (i, &vel) in bakers_velocities.iter().take(16).enumerate() {
        comp.instrument("bakers_texture", &Instrument::synth_lead())
            .velocity(vel)
            .at(28.0 + i as f32 * 0.25)
            .note(&[440.0], 0.2);
    }

    // ===== RANDOM WALK - BROWNIAN MOTION =====
    println!("6. Random Walk (Brownian Motion)\n");
    println!("   Smooth, organic wandering melodies");
    println!("   Like a drunk person walking or particle diffusion\n");

    let walk = sequences::random_walk(440.0, 20.0, 24);

    comp.instrument("random_walk", &Instrument::pluck())
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .at(32.0)
        .notes(&walk, 0.25);

    // ===== BOUNDED WALK - CONSTRAINED CHAOS =====
    println!("7. Bounded Random Walk\n");
    println!("   Random walk constrained to a range");
    println!("   Never escapes - perfect for controlled variation\n");

    let bounded = sequences::bounded_walk(440.0, 30.0, 220.0, 880.0, 32);

    comp.instrument("bounded_walk", &Instrument::synth_lead())
        .delay(Delay::new(0.375, 0.3, 0.6))
        .at(38.0)
        .notes(&bounded, 0.2);

    // ===== COMPARING CHAOS TYPES =====
    println!("8. Comparing Different Chaos Types\n");
    println!("   All starting from similar initial conditions\n");

    // Play them in parallel for comparison
    let compare_logistic = sequences::logistic_map(3.8, 0.5, 16);
    let compare_tent = sequences::tent_map(2.0, 0.5, 16);
    let compare_sine = sequences::sine_map(2.7, 0.5, 16);

    let logistic_freqs = sequences::normalize(
        &compare_logistic
            .iter()
            .map(|&x| (x * 100.0) as u32)
            .collect::<Vec<_>>(),
        300.0,
        600.0,
    );
    let tent_freqs: Vec<f32> = compare_tent.iter().map(|&x| 400.0 + x * 400.0).collect();
    let sine_freqs: Vec<f32> = compare_sine
        .iter()
        .map(|&x| 500.0 + (x / 3.0) * 400.0)
        .collect();

    comp.instrument("compare_logistic", &Instrument::pluck())
        .pan(-0.5)
        .at(45.0)
        .notes(&logistic_freqs, 0.25);

    comp.instrument("compare_tent", &Instrument::pluck())
        .pan(0.0)
        .at(45.0)
        .notes(&tent_freqs, 0.25);

    comp.instrument("compare_sine", &Instrument::pluck())
        .pan(0.5)
        .at(45.0)
        .notes(&sine_freqs, 0.25);

    println!("\n▶️  Playing chaotic sequences...\n");
    println!("    Duration: ~49 seconds\n");

    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✅ Chaotic Sequences Complete!\n");
    println!("💡 Key Takeaways:");
    println!("   • Logistic map: Classic chaos, parameter r controls behavior");
    println!("   • Tent map: Simpler chaos, triangular function");
    println!("   • Sine map: Smoothest chaos, very musical");
    println!("   • Hénon map: 2D chaos, two independent outputs");
    println!("   • Baker's map: Fractal mixing, creates distributions");
    println!("   • Random walks: Smooth wandering, organic variation");
    println!("   • All create non-repetitive but structured patterns\n");
    println!("🎵 Musical Applications:");
    println!("   • Evolving ambient textures");
    println!("   • Non-repetitive melodies with structure");
    println!("   • Dynamic parameter modulation");
    println!("   • Generative composition systems\n");
    println!("📚 Try Next:");
    println!("   cargo run --example musical_sequences");
    println!("   cargo run --example generative_sequences\n");

    Ok(())
}
