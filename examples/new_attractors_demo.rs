/// Demo showcasing the three new chaotic attractor sequence generators:
/// - Rössler Attractor: 3D spiral chaotic system
/// - Clifford Attractor: 2D flowing organic patterns
/// - Ikeda Map: 2D spiral patterns from laser physics
///
/// Each generator creates unique, never-repeating melodic patterns perfect for
/// algorithmic composition, ambient music, and generative art.
use tunes::prelude::*;
use tunes::sequences;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(100.0));
    let quarter = comp.tempo().quarter_note();

    println!("\n🎵 New Chaotic Attractors Demo 🎵\n");
    println!("Demonstrating three new sequence generators:");
    println!("1. Rössler Attractor - Smooth spiral melodies");
    println!("2. Clifford Attractor - Organic flowing patterns");
    println!("3. Ikeda Map - Laser physics inspired sequences\n");

    // ========================================
    // 1. RÖSSLER ATTRACTOR
    // ========================================
    // The Rössler attractor creates a single-banded spiral with smooth,
    // flowing trajectories. Perfect for melodic lines with structure.
    println!("▶ Part 1: Rössler Attractor (0-16 seconds)");

    // Generate classic Rössler spiral
    let rossler_path = sequences::rossler_spiral(64);
    let rossler_x: Vec<f32> = rossler_path.iter().map(|(x, _, _)| *x).collect();
    let rossler_z: Vec<f32> = rossler_path.iter().map(|(_, _, z)| *z).collect();

    // Map X to melody - just use normalized values directly
    let rossler_melody = sequences::normalize_f32(&rossler_x, 220.0, 880.0);

    // Map Z to note durations for rhythmic variation
    let rossler_durations = sequences::normalize_f32(&rossler_z, 0.08, 0.25);

    comp.instrument("rossler_lead", &Instrument::warm_pad())
        .filter(Filter::low_pass(1800.0, 0.5))
        .reverb(Reverb::new(0.3, 0.5, 0.4));

    let mut time = 0.0;
    for i in 0..32 {
        comp.track("rossler_lead")
            .at(time)
            .note(&[rossler_melody[i]], rossler_durations[i]);
        time += rossler_durations[i];
    }

    // ========================================
    // 2. CLIFFORD ATTRACTOR
    // ========================================
    // The Clifford attractor produces beautiful organic curves.
    // We'll use X and Y coordinates for two complementary voices.
    println!("▶ Part 2: Clifford Attractor (16-32 seconds)");

    // Generate Clifford with classic "flow" parameters
    let clifford_path = sequences::clifford_flow(64);

    // Separate X and Y coordinates
    let clifford_x: Vec<f32> = clifford_path.iter().map(|(x, _)| *x).collect();
    let clifford_y: Vec<f32> = clifford_path.iter().map(|(_, y)| *y).collect();

    // X → higher melody, Y → lower harmony
    let clifford_melody = sequences::normalize_f32(&clifford_x, 330.0, 880.0);
    let clifford_harmony = sequences::normalize_f32(&clifford_y, 165.0, 440.0);

    comp.instrument("clifford_high", &Instrument::pluck())
        .filter(Filter::band_pass(1200.0, 0.4))
        .delay(Delay::new(quarter, 0.3, 0.5))
        .reverb(Reverb::new(0.4, 0.5, 0.3));

    comp.instrument("clifford_low", &Instrument::sub_bass())
        .filter(Filter::low_pass(600.0, 0.6));

    for i in 0..32 {
        let t = 16.0 + i as f32 * quarter;
        comp.track("clifford_high")
            .at(t)
            .note(&[clifford_melody[i]], quarter * 0.8);

        comp.track("clifford_low")
            .at(t)
            .volume(0.5)
            .note(&[clifford_harmony[i]], quarter);
    }

    // ========================================
    // 3. IKEDA MAP
    // ========================================
    // The Ikeda map from laser physics creates stunning spirals with
    // bursts of dense activity. Great for rhythmic and spatial sequences.
    println!("▶ Part 3: Ikeda Map (32-48 seconds)");

    // Generate Ikeda spiral
    let ikeda_path = sequences::ikeda_spiral(128);

    let ikeda_x: Vec<f32> = ikeda_path.iter().map(|(x, _)| *x).collect();
    let ikeda_y: Vec<f32> = ikeda_path.iter().map(|(_, y)| *y).collect();

    // X → melody, Y → stereo panning
    let ikeda_melody = sequences::normalize_f32(&ikeda_x, 220.0, 660.0);
    let ikeda_panning = sequences::normalize_f32(&ikeda_y, -0.8, 0.8);

    comp.instrument("ikeda_spiral", &Instrument::electric_piano())
        .delay(Delay::new(quarter * 0.5, 0.25, 0.4))
        .reverb(Reverb::new(0.5, 0.5, 0.4));

    for i in 0..64 {
        let t = 32.0 + i as f32 * quarter * 0.5;
        comp.track("ikeda_spiral")
            .at(t)
            .pan(ikeda_panning[i])
            .volume(0.7)
            .note(&[ikeda_melody[i]], quarter * 0.4);
    }

    // Add subtle drums using Euclidean rhythm
    println!("▶ Adding rhythmic foundation");
    comp.track("drums")
        .drum_grid(16, quarter)
        .kick(&sequences::euclidean(4, 16))
        .snare(&sequences::euclidean(3, 16))
        .hihat(&sequences::euclidean(7, 16))
        .repeat(12); // 48 seconds total

    // ========================================
    // PLAYBACK
    // ========================================
    println!("\n🎼 Playing 48-second composition...\n");
    println!("Listen for:");
    println!("  • Rössler's smooth spiral melody (0-16s)");
    println!("  • Clifford's dual-voice harmony (16-32s)");
    println!("  • Ikeda's spatial spiraling patterns (32-48s)");
    println!("\nPress Ctrl+C to stop playback.\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    println!("\n✨ Demo complete! Try the generators in your own compositions:");
    println!("  • sequences::rossler_attractor(a, b, c, initial, dt, steps)");
    println!("  • sequences::clifford_attractor(a, b, c, d, initial, n)");
    println!("  • sequences::ikeda_map(a, b, c, d, initial, n)");

    Ok(())
}
