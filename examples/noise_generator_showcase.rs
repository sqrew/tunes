use tunes::prelude::*;

/// Showcase all noise generator types
///
/// This example demonstrates the six types of noise generators available:
/// - White: Equal energy at all frequencies (static/hiss)
/// - Brown: Low-frequency emphasis (rumble/bass)
/// - Pink: Equal energy per octave (balanced)
/// - Blue: High-frequency emphasis (crispy/sizzle)
/// - Green: Midrange emphasis (natural/organic)
/// - Perlin: Smooth coherent noise (organic modulation)
fn main() -> anyhow::Result<()> {
    println!("\n🔊 Example: Noise Generator Showcase\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // White Noise - Equal energy at all frequencies
    println!("▶ White Noise - Static/hiss sound");
    comp.track("white")
        .at(0.0)
        .noise(NoiseType::White, 0.8, 0.3);

    // Brown Noise - Low-frequency emphasis
    println!("▶ Brown Noise - Deep rumble");
    comp.track("brown")
        .at(1.0)
        .noise(NoiseType::Brown, 0.8, 0.3);

    // Pink Noise - Equal energy per octave
    println!("▶ Pink Noise - Balanced, natural");
    comp.track("pink")
        .at(2.0)
        .noise(NoiseType::Pink, 0.8, 0.3);

    // Blue Noise - High-frequency emphasis
    println!("▶ Blue Noise - Crispy, bright");
    comp.track("blue")
        .at(3.0)
        .noise(NoiseType::Blue, 0.8, 0.3);

    // Green Noise - Midrange emphasis
    println!("▶ Green Noise - Organic, natural");
    comp.track("green")
        .at(4.0)
        .noise(NoiseType::Green, 0.8, 0.3);

    // Perlin Noise - Smooth coherent noise
    println!("▶ Perlin Noise - Smooth, flowing");
    comp.track("perlin")
        .at(5.0)
        .noise(NoiseType::Perlin, 0.8, 0.3);

    // Comparison: All noise types together in stereo
    println!("\n▶ All noise types playing together (stereo mix)");

    comp.track("white_stereo")
        .pan(-0.8)
        .at(6.5)
        .noise(NoiseType::White, 2.0, 0.15);

    comp.track("brown_stereo")
        .pan(-0.5)
        .at(6.5)
        .noise(NoiseType::Brown, 2.0, 0.15);

    comp.track("pink_stereo")
        .pan(-0.2)
        .at(6.5)
        .noise(NoiseType::Pink, 2.0, 0.15);

    comp.track("blue_stereo")
        .pan(0.2)
        .at(6.5)
        .noise(NoiseType::Blue, 2.0, 0.15);

    comp.track("green_stereo")
        .pan(0.5)
        .at(6.5)
        .noise(NoiseType::Green, 2.0, 0.15);

    comp.track("perlin_stereo")
        .pan(0.8)
        .at(6.5)
        .noise(NoiseType::Perlin, 2.0, 0.15);

    println!("\n✓ Noise types available:");
    println!("  • White   - Equal energy at all frequencies (static/hiss)");
    println!("  • Brown   - Low-frequency emphasis (rumble/bass)");
    println!("  • Pink    - Equal energy per octave (balanced, natural)");
    println!("  • Blue    - High-frequency emphasis (crispy/sizzle)");
    println!("  • Green   - Midrange emphasis (natural/organic)");
    println!("  • Perlin  - Smooth coherent noise (flowing modulation)");
    println!("\n✓ Common uses:");
    println!("  • White   - Hi-hats, snares, wind, rain");
    println!("  • Brown   - Ocean waves, thunder, bass textures");
    println!("  • Pink    - Audio testing, ambient soundscapes");
    println!("  • Blue    - Dithering, bright textures");
    println!("  • Green   - Nature sounds, relaxation");
    println!("  • Perlin  - LFO modulation, organic automation");
    println!("\n✓ Use .noise() to set the noise type");
    println!("✓ Combine with filters for more control\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
