/// Doppler Effect Demo - Realistic Moving Sound Sources
///
/// Demonstrates Tunes' spatial audio doppler effect for moving sounds.
/// The doppler effect simulates the pitch change you hear when a sound source
/// moves relative to the listener - like a car passing by or a helicopter flyby.

use tunes::prelude::*;
use tunes::synthesis::spatial::SpatialParams;
use std::thread::sleep;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("=== Tunes Doppler Effect Demo ===\n");

    let engine = AudioEngine::new()?;

    // Configure spatial audio for more audible sounds at distance
    // Default: InverseSquare with ref_distance=1.0 (realistic physics but too quiet for demo)
    // - A car at 50m distance would be at (1/50)² = 0.04% volume (essentially silent!)
    // Solution: Use Linear attenuation with larger ref_distance for game-friendly audio
    let mut params = SpatialParams::default();
    params.attenuation_model = tunes::synthesis::spatial::AttenuationModel::Linear;
    params.ref_distance = 10.0;  // Full volume within 10m
    params.max_distance = 100.0; // Audible up to 100m
    engine.set_spatial_params(params)?;

    // Demo 1: Car Passing By (Left to Right)
    println!("Demo 1: Car Passing By");
    println!("----------------------");
    println!("  A car drives past you from left to right at 30 m/s (~67 mph)");
    println!("  Listen for the pitch drop as it passes!\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // Create a sustained engine sound
    comp.instrument("engine", &Instrument::synth_bass())
        .filter(Filter::low_pass(400.0, 0.7))
        .note(&[110.0], 10.0); // A2 for 10 seconds

    let mixer = comp.into_mixer();
    let car_id = engine.play_mixer_realtime(&mixer)?;

    // Simulate car passing: starts at left (-50m), moves to right (50m)
    let duration = 5.0; // seconds
    let steps = 50;
    let velocity = 30.0; // m/s (about 67 mph)

    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = -50.0 + (100.0 * t); // -50m to +50m
        let z = 5.0; // 5m in front of listener

        engine.set_sound_position(car_id, x, 0.0, z)?;
        engine.set_sound_velocity(car_id, velocity, 0.0, 0.0)?; // Moving right

        sleep(Duration::from_millis((duration * 1000.0 / steps as f32) as u64));
    }

    engine.stop(car_id)?;
    println!("  ✓ Car passed by - did you hear the pitch drop?\n");

    sleep(Duration::from_secs(1));

    // Demo 2: Helicopter Flyby (Front to Back)
    println!("Demo 2: Helicopter Flyby");
    println!("------------------------");
    println!("  A helicopter flies over you from front to back");
    println!("  Notice the pitch change as it approaches and recedes\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // Create a helicopter-like sound
    comp.instrument("heli", &Instrument::warm_pad())
        .filter(Filter::low_pass(300.0, 0.5))
        .notes(&[220.0, 225.0], 5.0); // Slight beating for realism

    let mixer = comp.into_mixer();
    let heli_id = engine.play_mixer_realtime(&mixer)?;

    // Helicopter approaches from front and flies overhead
    let duration = 5.0;
    let steps = 50;
    let velocity = 25.0; // m/s

    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let z = 50.0 - (100.0 * t); // 50m front to -50m back
        let y = 20.0 - (15.0 * (2.0 * t - 1.0).abs()); // Peak at 20m overhead

        engine.set_sound_position(heli_id, 0.0, y, z)?;
        engine.set_sound_velocity(heli_id, 0.0, 0.0, -velocity)?; // Moving backwards

        sleep(Duration::from_millis((duration * 1000.0 / steps as f32) as u64));
    }

    engine.stop(heli_id)?;
    println!("  ✓ Helicopter flew by\n");

    sleep(Duration::from_secs(1));

    // Demo 3: Racing Car (Circular Track)
    println!("Demo 3: Racing Car on Circular Track");
    println!("-------------------------------------");
    println!("  A race car circles around you");
    println!("  Doppler effect continuously changes as it moves\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // Create race car engine sound
    comp.instrument("race", &Instrument::synth_lead())
        .filter(Filter::low_pass(500.0, 0.8))
        .note(&[165.0], 8.0); // E3 for 8 seconds

    let mixer = comp.into_mixer();
    let race_id = engine.play_mixer_realtime(&mixer)?;

    // Car circles at 20m radius
    let duration = 6.0;
    let steps = 60;
    let radius = 20.0;
    let angular_velocity = std::f32::consts::PI; // radians per second

    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let angle = t * angular_velocity * duration;

        let x = radius * angle.cos();
        let z = radius * angle.sin();

        // Calculate velocity (tangent to circle)
        let vx = -radius * angular_velocity * angle.sin();
        let vz = radius * angular_velocity * angle.cos();

        engine.set_sound_position(race_id, x, 0.0, z)?;
        engine.set_sound_velocity(race_id, vx, 0.0, vz)?;

        sleep(Duration::from_millis((duration * 1000.0 / steps as f32) as u64));
    }

    engine.stop(race_id)?;
    println!("  ✓ Race car completed lap\n");

    sleep(Duration::from_secs(1));

    // Demo 4: Adjusting Doppler Parameters
    println!("Demo 4: Doppler Factor Comparison");
    println!("----------------------------------");
    println!("  Same car passing, but with different doppler factors");
    println!("  1.0 = realistic, 2.0 = exaggerated, 0.0 = disabled\n");

    for &(factor, label) in &[(0.0, "disabled"), (0.5, "subtle"), (1.0, "realistic"), (2.0, "exaggerated")] {
        println!("  Playing with doppler factor: {:.1} ({})", factor, label);

        // Set doppler parameters (keep linear attenuation for audibility)
        let mut params = SpatialParams::default();
        params.attenuation_model = tunes::synthesis::spatial::AttenuationModel::Linear;
        params.ref_distance = 10.0;
        params.max_distance = 100.0;
        params.doppler_factor = factor;
        params.doppler_enabled = factor > 0.0;
        engine.set_spatial_params(params)?;

        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("car", &Instrument::synth_bass())
            .filter(Filter::low_pass(400.0, 0.7))
            .note(&[110.0], 4.0);

        let mixer = comp.into_mixer();
        let car_id = engine.play_mixer_realtime(&mixer)?;

        // Quick pass
        let duration = 3.0;
        let steps = 30;
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = -30.0 + (60.0 * t);
            engine.set_sound_position(car_id, x, 0.0, 5.0)?;
            engine.set_sound_velocity(car_id, 30.0, 0.0, 0.0)?;
            sleep(Duration::from_millis((duration * 1000.0 / steps as f32) as u64));
        }

        engine.stop(car_id)?;
        sleep(Duration::from_millis(500));
    }

    // Reset to demo defaults (not library defaults)
    let mut params = SpatialParams::default();
    params.attenuation_model = tunes::synthesis::spatial::AttenuationModel::Linear;
    params.ref_distance = 10.0;
    params.max_distance = 100.0;
    engine.set_spatial_params(params)?;
    println!("\n  ✓ Doppler factor comparison complete\n");

    println!("=== Demo Complete ===");
    println!("\nDoppler Effect Features:");
    println!("  ✓ Realistic pitch shift for approaching/receding sounds");
    println!("  ✓ Based on actual physics (speed of sound = 343 m/s)");
    println!("  ✓ Configurable doppler factor (0.0 to 2.0)");
    println!("  ✓ Automatic velocity tracking for sources and listener");
    println!("  ✓ Pitch shift clamped to 0.5x - 2.0x (one octave range)");
    println!("\nPhysics:");
    println!("  - Higher pitch when approaching (sound waves compressed)");
    println!("  - Lower pitch when receding (sound waves stretched)");
    println!("  - Formula: pitch = (v_sound - v_relative) / v_sound");
    println!("\nUse Cases:");
    println!("  - Racing games (cars, motorcycles)");
    println!("  - Flight simulators (aircraft, helicopters)");
    println!("  - Action games (projectiles, vehicles)");
    println!("  - Any game with fast-moving sound sources");

    Ok(())
}
