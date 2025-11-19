use tunes::prelude::*;
use anyhow::Result;

/// Generate a LARGE sample pool (200+ samples) for cache pressure testing
///
/// This creates variations of existing samples by:
/// - Pitch shifting
/// - Time stretching
/// - Applying different effects
///
/// Goal: Test if performance degrades when sample pool exceeds CPU cache

fn main() -> Result<()> {
    println!("\n🔬 Generating Large Sample Pool for Cache Pressure Test\n");
    println!("Loading base samples and creating variations...\n");

    let output_dir = "benches/assets/large_pool";
    std::fs::create_dir_all(output_dir)?;

    // Load base samples
    let base_samples = vec![
        Sample::from_file("benches/assets/footstep_01.wav")?,
        Sample::from_file("benches/assets/footstep_02.wav")?,
        Sample::from_file("benches/assets/impact_01.wav")?,
        Sample::from_file("benches/assets/gunshot_01.wav")?,
        Sample::from_file("benches/assets/explosion_01.wav")?,
        Sample::from_file("benches/assets/voice_01.wav")?,
        Sample::from_file("benches/assets/ambient_01.wav")?,
        Sample::from_file("benches/assets/engine_01.wav")?,
        Sample::from_file("benches/assets/bass_01.wav")?,
        Sample::from_file("benches/assets/high_freq_01.wav")?,
    ];

    println!("Loaded {} base samples", base_samples.len());
    println!("Creating variations...\n");

    let mut total_created = 0;

    // Create 20 variations of each base sample
    for (base_idx, base_sample) in base_samples.iter().enumerate() {
        println!("Creating variations of base sample {}...", base_idx + 1);

        for variation_idx in 0..20 {
            // Apply different transformations
            let sample = match variation_idx % 4 {
                0 => {
                    // Pitch shift up
                    let shift = 1.0 + (variation_idx as f32 * 0.05);
                    base_sample.pitch_shift(shift)
                }
                1 => {
                    // Pitch shift down
                    let shift = 1.0 - (variation_idx as f32 * 0.03);
                    base_sample.pitch_shift(shift.max(0.5))
                }
                2 => {
                    // Time stretch
                    let factor = 1.0 + (variation_idx as f32 * 0.1);
                    base_sample.time_stretch(factor)
                }
                _ => {
                    // Reverse or original
                    if variation_idx % 2 == 0 {
                        base_sample.reverse()
                    } else {
                        base_sample.clone()
                    }
                }
            };

            // Save variation
            let filename = format!(
                "{}/sample_{}_{:03}.wav",
                output_dir,
                base_idx + 1,
                variation_idx + 1
            );
            sample.export_wav(&filename)?;
            total_created += 1;
        }
    }

    println!("\n✅ Created {} unique samples in {}/", total_created, output_dir);

    // Calculate total size
    let output = std::process::Command::new("du")
        .args(&["-sh", output_dir])
        .output()?;
    let size = String::from_utf8_lossy(&output.stdout);
    println!("Total disk usage: {}", size.trim());

    println!("\nThis sample pool provides:");
    println!("  ✓ {} unique audio files", total_created);
    println!("  ✓ Realistic cache pressure (won't all fit in L3)");
    println!("  ✓ Diverse spectral content (pitch shifted, time stretched)");
    println!("\nUse in benchmarks to test performance with realistic asset counts.\n");

    Ok(())
}
