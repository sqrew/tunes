//! Advanced spatial audio demonstration
//!
//! This example demonstrates the new spatial audio features:
//! 1. Elevation panning (sounds above/below the listener)
//! 2. Directional sound cones (loudspeakers, NPCs, etc.)
//! 3. Occlusion (sounds blocked by obstacles)
//!
//! Run with: cargo run --example advanced_spatial_demo

use std::thread;
use std::time::Duration;
use tunes::prelude::*;
use tunes::synthesis::spatial::SoundCone;

fn main() -> anyhow::Result<()> {
    println!("Advanced Spatial Audio Demo");
    println!("===========================\n");

    let engine = AudioEngine::new()?;

    // Configure spatial audio for better audibility in demos
    // Use Linear attenuation instead of InverseSquare for gentler falloff
    use tunes::synthesis::spatial::{AttenuationModel, SpatialParams};
    let mut params = SpatialParams::default();
    params.attenuation_model = AttenuationModel::Linear;
    params.ref_distance = 1.0;
    params.max_distance = 20.0;  // Shorter max distance for demo
    engine.set_spatial_params(params)?;

    // Create a simple sound for testing
    let mut comp = Composition::new(Tempo::new(120.0));
    comp.instrument("tone", &Instrument::synth_lead())
        .notes(&[C4], 0.5)
        .repeat(100); // Long duration for testing

    let mixer = comp.into_mixer();

    // Demo 1: Elevation Panning
    println!("Demo 1: Elevation Panning");
    println!("--------------------------");
    println!("Sound moving from below to above the listener...\n");

    let sound_id = engine.play_mixer_realtime(&mixer)?;

    // Set listener at head height
    engine.set_listener_position(0.0, 1.7, 0.0)?;

    // Move sound from below to above
    for i in 0..=20 {
        let y = -3.0 + (i as f32 * 6.0 / 20.0); // -3.0 to +3.0
        engine.set_sound_position(sound_id, 0.0, y, 5.0)?;

        if i == 0 {
            println!("  Below listener (y = -3.0)");
        } else if i == 10 {
            println!("  At ear level (y = 0.0)");
        } else if i == 20 {
            println!("  Above listener (y = +3.0)");
        }

        thread::sleep(Duration::from_millis(200));
    }

    engine.stop(sound_id)?;
    thread::sleep(Duration::from_millis(500));

    // Demo 2: Directional Sound Cone
    println!("\nDemo 2: Directional Sound Cone");
    println!("--------------------------------");
    println!("Walking smoothly around a directional speaker...\n");

    let sound_id = engine.play_mixer_realtime(&mixer)?;

    // Create a narrow directional cone pointing forward (like a speaker)
    let cone = SoundCone::narrow().with_direction(0.0, 0.0, 1.0);

    // Position speaker at origin, pointing forward (+Z)
    engine.set_sound_position(sound_id, 0.0, 1.7, 0.0)?;
    engine.set_sound_cone(sound_id, Some(cone))?;

    // Walk in a circle around the speaker (smooth interpolation)
    for i in 0..=72 {
        let angle = (i as f32 * 5.0).to_radians(); // 0 to 360 degrees in 5° steps
        let x = angle.sin() * 5.0;
        let z = angle.cos() * 5.0;

        engine.set_listener_position(x, 1.7, z)?;

        // Look at the speaker
        engine.set_listener_forward(-x, 0.0, -z)?;

        // Print position updates every 90 degrees
        let degrees = i * 5;
        if degrees % 90 == 0 {
            let position = match degrees {
                0 => "Front (loud)",
                90 => "90° Right (quieter)",
                180 => "Behind (very quiet)",
                270 => "270° Left (quieter)",
                _ => "Back to front",
            };
            println!("  {} - {}°", position, degrees);
        }

        thread::sleep(Duration::from_millis(50));
    }

    engine.stop(sound_id)?;
    thread::sleep(Duration::from_millis(500));

    // Reset listener
    engine.set_listener_position(0.0, 1.7, 0.0)?;
    engine.set_listener_forward(0.0, 0.0, 1.0)?;

    // Demo 3: Occlusion
    println!("\nDemo 3: Occlusion");
    println!("-----------------");
    println!("Sound becoming occluded/unoccluded (volume reduction)...\n");

    let sound_id = engine.play_mixer_realtime(&mixer)?;

    // Remove cone directionality
    engine.set_sound_cone(sound_id, None)?;

    // Position sound to the right
    engine.set_sound_position(sound_id, 5.0, 1.7, 0.0)?;

    println!("  0% occluded - Clear path (100% volume)");
    engine.set_sound_occlusion(sound_id, 0.0)?;
    thread::sleep(Duration::from_millis(1500));

    println!("  30% occluded - Thin wall (70% volume)");
    engine.set_sound_occlusion(sound_id, 0.3)?;
    thread::sleep(Duration::from_millis(1500));

    println!("  60% occluded - Thick wall (40% volume)");
    engine.set_sound_occlusion(sound_id, 0.6)?;
    thread::sleep(Duration::from_millis(1500));

    println!("  90% occluded - Multiple walls (10% volume)");
    engine.set_sound_occlusion(sound_id, 0.9)?;
    thread::sleep(Duration::from_millis(1500));

    println!("  100% occluded - Completely blocked (0% volume / SILENT)");
    engine.set_sound_occlusion(sound_id, 1.0)?;
    thread::sleep(Duration::from_millis(1500));

    println!("  Back to clear (100% volume)");
    engine.set_sound_occlusion(sound_id, 0.0)?;
    thread::sleep(Duration::from_millis(1000));

    engine.stop(sound_id)?;
    thread::sleep(Duration::from_millis(500));

    // Demo 4: Combined Features
    println!("\nDemo 4: Combined Features");
    println!("-------------------------");
    println!("Helicopter flying overhead with directional rotor sound...\n");

    let sound_id = engine.play_mixer_realtime(&mixer)?;

    // Create a wide downward-facing cone (rotor noise is directional downward)
    let rotor_cone = SoundCone::wide().with_direction(0.0, -1.0, 0.0);
    engine.set_sound_cone(sound_id, Some(rotor_cone))?;

    // Fly overhead
    for i in 0..=30 {
        let t = i as f32 / 30.0;

        // X: -10 to +10 (flying left to right)
        let x = -10.0 + (t * 20.0);
        // Y: 10 to 15 and back down (arc)
        let y = 10.0 + ((t * std::f32::consts::PI).sin() * 5.0);
        // Z: constant
        let z = 0.0;

        engine.set_sound_position(sound_id, x, y, z)?;

        // Velocity for Doppler effect (moving at ~10 units/sec)
        engine.set_sound_velocity(sound_id, 20.0 / 3.0, 0.0, 0.0)?;

        // Simulate occlusion when behind buildings
        let occlusion = if t > 0.4 && t < 0.6 {
            0.7 // Behind buildings
        } else {
            0.0 // Clear sky
        };
        engine.set_sound_occlusion(sound_id, occlusion)?;

        if i == 0 {
            println!("  Approaching from the left...");
        } else if i == 15 {
            println!("  Directly overhead!");
        } else if (12..=18).contains(&i) {
            if i == 12 {
                println!("  Behind buildings (occluded)");
            }
        } else if i == 30 {
            println!("  Departing to the right...");
        }

        thread::sleep(Duration::from_millis(100));
    }

    engine.stop(sound_id)?;

    println!("\n✓ Demo complete!");
    println!("\nNew Features Summary:");
    println!("---------------------");
    println!("1. Elevation: Sounds above/below are quieter and centered");
    println!("2. Sound Cones: Directional sources (speakers, NPCs, vehicles)");
    println!("3. Occlusion: Sounds can be blocked by obstacles");
    println!("\nThese features make spatial audio more realistic for games!");
    println!("\nNote: This demo uses Linear attenuation for better audibility.");
    println!("      Default is InverseSquare (more realistic but quieter at distance).");
    println!("      Adjust with SpatialParams::attenuation_model.");

    Ok(())
}
