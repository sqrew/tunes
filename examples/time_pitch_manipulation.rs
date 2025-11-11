/// Example: Time stretching and pitch shifting for game audio variation
///
/// Demonstrates how to use WSOLA-based time stretching and pitch shifting
/// to add variety to game audio without affecting duration or pitch.
///
/// Use cases:
/// - Randomize enemy footsteps by varying pitch slightly
/// - Time-stretch impact sounds for slow-motion effects
/// - Create variations of the same sound for less repetition
use anyhow::Result;
use tunes::prelude::*;
use tunes::synthesis::Sample;

fn main() -> Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    println!("Time Stretching and Pitch Shifting Demo");
    println!("========================================\n");

    // Example 1: Time stretching without pitch change
    println!("1. Time Stretching (duration changes, pitch stays the same)");
    example_time_stretch(&mut comp)?;

    // Example 2: Pitch shifting without time change
    println!("\n2. Pitch Shifting (pitch changes, duration stays the same)");
    example_pitch_shift(&mut comp)?;

    // Example 3: Game audio variations
    println!("\n3. Game Audio Variations");
    example_game_variations(&mut comp)?;

    // Example 4: Slow motion effect
    println!("\n4. Slow Motion Effect");
    example_slow_motion(&mut comp)?;

    println!("\n✓ Demo complete! Check the examples for code.");

    Ok(())
}

/// Example 1: Time stretching changes duration without affecting pitch
fn example_time_stretch(_comp: &mut Composition) -> Result<()> {
    println!("   Creating a test tone...");

    // Generate a 1-second 440Hz tone
    let sample_rate = 44100;
    let duration_secs = 1.0;
    let frequency = 440.0;

    let data: Vec<f32> = (0..(sample_rate as f32 * duration_secs) as usize)
        .map(|i| {
            (2.0 * std::f32::consts::PI * frequency * i as f32 / sample_rate as f32).sin() * 0.5
        })
        .collect();

    let sample = Sample::from_mono(data, sample_rate);

    println!("   Original duration: {:.2}s", sample.duration);

    // Stretch to 150% duration (slower playback, same pitch)
    let stretched = sample.time_stretch(1.5);
    println!("   Stretched (1.5x): {:.2}s - pitch unchanged", stretched.duration);

    // Compress to 50% duration (faster playback, same pitch)
    let compressed = sample.time_stretch(0.5);
    println!(
        "   Compressed (0.5x): {:.2}s - pitch unchanged",
        compressed.duration
    );

    // In a real game, you might use this for:
    // - Slow-motion effect on impacts (stretch time, keep pitch)
    // - Speed up dialog without chipmunk effect
    println!("   Use case: Slow-motion effects, time dilation");

    Ok(())
}

/// Example 2: Pitch shifting changes pitch without affecting duration
fn example_pitch_shift(_comp: &mut Composition) -> Result<()> {
    println!("   Creating a test tone...");

    // Generate a 1-second 440Hz (A4) tone
    let sample_rate = 44100;
    let duration_secs = 1.0;
    let frequency = 440.0;

    let data: Vec<f32> = (0..(sample_rate as f32 * duration_secs) as usize)
        .map(|i| {
            (2.0 * std::f32::consts::PI * frequency * i as f32 / sample_rate as f32).sin() * 0.5
        })
        .collect();

    let sample = Sample::from_mono(data, sample_rate);

    println!(
        "   Original: 440Hz (A4), duration: {:.2}s",
        sample.duration
    );

    // Shift up by 12 semitones (1 octave) -> 880Hz (A5)
    let octave_up = sample.pitch_shift(12.0);
    println!(
        "   +12 semitones (octave up): ~880Hz, duration: {:.2}s",
        octave_up.duration
    );

    // Shift up by 7 semitones (perfect fifth) -> ~659Hz (E5)
    let fifth_up = sample.pitch_shift(7.0);
    println!(
        "   +7 semitones (fifth up): ~659Hz, duration: {:.2}s",
        fifth_up.duration
    );

    // Shift down by 12 semitones (1 octave) -> 220Hz (A3)
    let octave_down = sample.pitch_shift(-12.0);
    println!(
        "   -12 semitones (octave down): ~220Hz, duration: {:.2}s",
        octave_down.duration
    );

    println!("   Use case: Enemy size variations, musical transposition");

    Ok(())
}

/// Example 3: Creating variations for game audio
fn example_game_variations(_comp: &mut Composition) -> Result<()> {
    println!("   Simulating enemy footstep variations...");

    // Simulate a footstep sample (short impact)
    let sample_rate = 44100;
    let footstep_duration = 0.3; // 300ms footstep

    // Create a simple impact sound (decay envelope with noise)
    let data: Vec<f32> = (0..(sample_rate as f32 * footstep_duration) as usize)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            let decay = (-t * 8.0).exp(); // Fast decay
            let noise = (rand::random::<f32>() - 0.5) * 2.0;
            noise * decay * 0.3
        })
        .collect();

    let footstep = Sample::from_mono(data, sample_rate);

    println!("   Original footstep: duration {:.2}s", footstep.duration);

    // Create 3 variations by slightly shifting pitch
    // This makes repetitive sounds less noticeable
    let variations = vec![
        footstep.pitch_shift(-2.0),  // Slightly lower (larger creature)
        footstep.clone(),             // Original
        footstep.pitch_shift(2.0),    // Slightly higher (smaller creature)
        footstep.pitch_shift(-1.0),   // Another variation
        footstep.pitch_shift(1.5),    // Another variation
    ];

    println!(
        "   Created {} footstep variations",
        variations.len()
    );
    println!("   Each has slightly different pitch but same duration");
    println!("   Reduces audio repetition in games!");

    // In a real game, you'd randomly pick from these variations
    println!("   Usage: footsteps[rand() % 5].play()");

    Ok(())
}

/// Example 4: Slow motion effect
fn example_slow_motion(_comp: &mut Composition) -> Result<()> {
    println!("   Creating slow-motion impact effect...");

    // Simulate an impact/explosion sound
    let sample_rate = 44100;
    let impact_duration = 0.5;

    // Create impact: quick attack with rumble
    let data: Vec<f32> = (0..(sample_rate as f32 * impact_duration) as usize)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            let envelope = (-t * 5.0).exp();

            // Mix of frequencies for impact
            let low_freq = (2.0 * std::f32::consts::PI * 60.0 * t).sin();
            let mid_freq = (2.0 * std::f32::consts::PI * 150.0 * t).sin();
            let noise = (rand::random::<f32>() - 0.5) * 2.0;

            (low_freq * 0.4 + mid_freq * 0.3 + noise * 0.3) * envelope * 0.4
        })
        .collect();

    let impact = Sample::from_mono(data, sample_rate);

    println!("   Original impact: duration {:.2}s", impact.duration);

    // Slow-motion effect: 50% speed (2x duration) but keep pitch low
    // Time-stretch maintains the deep "boom" character
    let slow_mo = impact.time_stretch(2.0);

    println!(
        "   Slow-motion (2x slower): duration {:.2}s",
        slow_mo.duration
    );
    println!("   Pitch unchanged - maintains deep impact feel");
    println!("   Perfect for cinematic slow-mo sequences!");

    // For dramatic effect, could also pitch down slightly
    let dramatic_slow_mo = impact.time_stretch(2.0).pitch_shift(-5.0);
    println!(
        "   Dramatic slow-mo (stretched + pitched down): {:.2}s",
        dramatic_slow_mo.duration
    );

    Ok(())
}

// Additional helper function showing how to use this in a composition
#[allow(dead_code)]
fn create_rhythm_with_variations() -> Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // In a real scenario, you'd load an actual drum sample
    // let kick = Sample::from_wav("kick.wav")?;

    // For this example, we'll create a synthetic kick
    let sample_rate = 44100;
    let kick_duration = 0.15;

    let kick_data: Vec<f32> = (0..(sample_rate as f32 * kick_duration) as usize)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            let envelope = (-t * 40.0).exp();
            let freq = 60.0 * (-t * 30.0).exp(); // Pitch drop
            (2.0 * std::f32::consts::PI * freq * t).sin() * envelope * 0.6
        })
        .collect();

    let kick = Sample::from_mono(kick_data, sample_rate);

    // Create variations by pitch shifting
    let kick_heavy = kick.pitch_shift(-3.0); // Heavier kick (lower pitch)
    let kick_tight = kick.pitch_shift(2.0);  // Tighter kick (higher pitch)

    // Use in a pattern (using play_sample for direct Sample playback)
    // Play each variation at different times
    comp.track("drums")
        .play_sample(&kick, 1.0)
        .at(1.0)
        .play_sample(&kick_heavy, 1.0)
        .at(2.0)
        .play_sample(&kick, 1.0)
        .at(3.0)
        .play_sample(&kick_tight, 1.0);

    println!("Created varied drum pattern with 3 pitch variations!");
    println!("Each variation has the same duration but different pitch!");

    Ok(())
}
