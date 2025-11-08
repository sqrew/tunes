use tunes::prelude::*;
use std::thread;
use std::time::Duration;

/// Demonstrate 3D spatial audio - placing sounds in 3D space
///
/// This example shows how to:
/// - Position sounds in 3D space during composition
/// - Move sounds in real-time during playback
/// - Control listener position and orientation
/// - Configure spatial audio parameters
fn main() -> anyhow::Result<()> {
    println!("\n🎧 Example: 3D Spatial Audio\n");

    let engine = AudioEngine::new()?;

    // ===========================================================================
    // Example 1: Static Spatial Composition
    // ===========================================================================
    println!("▶ Example 1: Static spatial composition");
    println!("  Creating a composition with sounds at different 3D positions\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // Guitar on the right side, 5 meters forward
    comp.instrument("guitar", &Instrument::pluck())
        .spatial_position(3.0, 0.0, 5.0)
        .notes(&[C4, E4, G4, C5], 0.5);

    // Bass at center, 2 meters forward
    comp.instrument("bass", &Instrument::synth_bass())
        .spatial_position(0.0, 0.0, 2.0)
        .notes(&[C2, C2, G2, G2], 1.0);

    // Drums at listener position (0, 0, 0)
    comp.track("drums")
        .spatial_position(0.0, 0.0, 0.0)
        .drum(DrumType::Kick)
        .wait(0.5)
        .drum(DrumType::Snare)
        .wait(0.5)
        .drum(DrumType::Kick)
        .wait(0.5)
        .drum(DrumType::Snare);

    println!("  Playing composition (guitar on right, bass center, drums at listener)...");
    engine.play_mixer(&comp.into_mixer())?;

    thread::sleep(Duration::from_millis(500));

    // ===========================================================================
    // Example 2: Moving Sound in Real-Time
    // ===========================================================================
    println!("\n▶ Example 2: Moving sound left-to-right");
    println!("  A tone moves from left to right across the stereo field\n");

    let mut moving_comp = Composition::new(Tempo::new(120.0));
    moving_comp.instrument("moving", &Instrument::synth_lead())
        .note(&[A4], 3.0);  // 3 second tone

    let sound_id = engine.play_mixer_realtime(&moving_comp.into_mixer())?;

    // Move sound from left to right over 3 seconds
    for i in 0..30 {
        let x = -5.0 + (i as f32 * 10.0 / 30.0);  // -5 to +5
        engine.set_sound_position(sound_id, x, 0.0, 5.0)?;
        thread::sleep(Duration::from_millis(100));
    }

    println!("  ✓ Sound moved from left to right");

    thread::sleep(Duration::from_millis(500));

    // ===========================================================================
    // Example 3: Distance Attenuation
    // ===========================================================================
    println!("\n▶ Example 3: Distance attenuation");
    println!("  A sound approaches from far away, getting louder\n");

    let mut distance_comp = Composition::new(Tempo::new(120.0));
    distance_comp.instrument("approaching", &Instrument::warm_pad())
        .note(&[E4, G4, B4], 4.0);  // 4 second chord

    let sound_id = engine.play_mixer_realtime(&distance_comp.into_mixer())?;

    // Move sound from far away (50m) to close (2m)
    for i in 0..40 {
        let z = 50.0 - (i as f32 * 48.0 / 40.0);  // 50 to 2
        engine.set_sound_position(sound_id, 0.0, 0.0, z)?;
        thread::sleep(Duration::from_millis(100));
    }

    println!("  ✓ Sound approached from distance");

    thread::sleep(Duration::from_millis(500));

    // ===========================================================================
    // Example 4: Listener Movement
    // ===========================================================================
    println!("\n▶ Example 4: Moving listener around sound");
    println!("  The listener rotates around a stationary sound\n");

    let mut circle_comp = Composition::new(Tempo::new(120.0));
    circle_comp.instrument("stationary", &Instrument::electric_piano())
        .note(&[C5], 5.0);  // 5 second tone

    let sound_id = engine.play_mixer_realtime(&circle_comp.into_mixer())?;

    // Place sound at (5, 0, 0) - 5 meters to the right
    engine.set_sound_position(sound_id, 5.0, 0.0, 0.0)?;

    // Rotate listener around the origin
    for i in 0..50 {
        let angle = (i as f32) * std::f32::consts::PI * 2.0 / 50.0;
        let x = angle.cos() * 2.0;
        let z = angle.sin() * 2.0;

        engine.set_listener_position(x, 0.0, z)?;
        engine.set_listener_forward(-x, 0.0, -z)?;  // Face toward origin

        thread::sleep(Duration::from_millis(100));
    }

    // Reset listener to origin
    engine.set_listener_position(0.0, 0.0, 0.0)?;
    engine.set_listener_forward(0.0, 0.0, 1.0)?;

    println!("  ✓ Listener rotated around sound");

    thread::sleep(Duration::from_millis(500));

    // ===========================================================================
    // Example 5: Custom Spatial Parameters
    // ===========================================================================
    println!("\n▶ Example 5: Custom spatial parameters");
    println!("  Using linear attenuation with custom max distance\n");

    let mut params = SpatialParams::default();
    params.attenuation_model = AttenuationModel::Linear;
    params.max_distance = 20.0;  // Silent beyond 20 meters
    params.ref_distance = 1.0;

    engine.set_spatial_params(params)?;

    let mut params_comp = Composition::new(Tempo::new(120.0));
    params_comp.instrument("test", &Instrument::synth_lead())
        .notes(&[C5, D5, E5, F5, G5, A5, B5, C6], 0.25);

    let sound_id = engine.play_mixer_realtime(&params_comp.into_mixer())?;

    // Move sound from near to beyond max distance
    for i in 0..25 {
        let z = 1.0 + (i as f32 * 25.0 / 25.0);  // 1 to 26 meters
        engine.set_sound_position(sound_id, 0.0, 0.0, z)?;
        thread::sleep(Duration::from_millis(80));
    }

    println!("  ✓ Sound faded to silence at max distance");

    // Reset to default parameters
    engine.set_spatial_params(SpatialParams::default())?;

    thread::sleep(Duration::from_millis(500));

    // ===========================================================================
    // Example 6: Multi-Source Spatial Scene
    // ===========================================================================
    println!("\n▶ Example 6: Multi-source spatial scene");
    println!("  Multiple instruments positioned in 3D space\n");

    let mut scene = Composition::new(Tempo::new(140.0));

    // Left side: Piano melody
    scene.instrument("piano-left", &Instrument::electric_piano())
        .spatial_position(-4.0, 0.0, 6.0)
        .notes(&[C4, E4, G4, E4], 0.375)
        .notes(&[D4, F4, A4, F4], 0.375);

    // Right side: Synth harmony
    scene.instrument("synth-right", &Instrument::warm_pad())
        .spatial_position(4.0, 0.0, 6.0)
        .note(&[G3, B3, D4], 3.0);

    // Center front: Lead
    scene.instrument("lead-center", &Instrument::synth_lead())
        .spatial_position(0.0, 1.0, 3.0)  // Slightly above
        .wait(1.5)
        .notes(&[E5, G5, B5, D5], 0.375);

    // Behind listener: Ambient pad
    scene.instrument("ambient-back", &Instrument::warm_pad())
        .spatial_position(0.0, 0.0, -5.0)  // Negative Z = behind
        .note(&[C3, E3, G3], 3.0);

    // At listener: Drums
    scene.track("drums-center")
        .spatial_position(0.0, 0.0, 0.0)
        .drum(DrumType::Kick)
        .wait(0.5)
        .drum(DrumType::HiHatClosed)
        .wait(0.5)
        .drum(DrumType::Snare)
        .wait(0.5)
        .drum(DrumType::HiHatClosed)
        .wait(0.5)
        .drum(DrumType::Kick)
        .wait(0.5)
        .drum(DrumType::HiHatClosed);

    println!("  Piano (left), Synth (right), Lead (center-front), Ambient (behind)");
    println!("  Playing spatial scene...");
    engine.play_mixer(&scene.into_mixer())?;

    println!("\n✓ Spatial Audio Concepts:");
    println!("  • 3D positioning: Sounds have (x, y, z) coordinates");
    println!("  • Distance attenuation: Sounds get quieter with distance");
    println!("  • Azimuth-based panning: Sounds are panned left/right based on angle");
    println!("  • Listener orientation: Sounds move relative to where you're facing");
    println!("  • Real-time updates: Positions can change during playback");
    println!("  • Multiple sources: Many sounds can exist in the same 3D space");

    println!("\n✓ API Usage:");
    println!("  ```rust");
    println!("  // Set position during composition");
    println!("  comp.track(\"guitar\")");
    println!("      .spatial_position(2.0, 0.0, 5.0)  // 2m right, 5m forward");
    println!("      .note(&[C4], 1.0);");
    println!();
    println!("  // Move sound in real-time");
    println!("  let sound_id = engine.play_mixer_realtime(&mixer)?;");
    println!("  engine.set_sound_position(sound_id, x, y, z)?;");
    println!();
    println!("  // Control listener");
    println!("  engine.set_listener_position(0.0, 1.7, 0.0)?;  // Standing height");
    println!("  engine.set_listener_forward(0.0, 0.0, 1.0)?;  // Face forward");
    println!();
    println!("  // Configure spatial parameters");
    println!("  let mut params = SpatialParams::default();");
    println!("  params.max_distance = 50.0;");
    println!("  engine.set_spatial_params(params)?;");
    println!("  ```");

    println!("\n✓ Coordinate System:");
    println!("  • X-axis: Left (-) to Right (+)");
    println!("  • Y-axis: Down (-) to Up (+)");
    println!("  • Z-axis: Behind (-) to Forward (+)");
    println!("  • Listener default: (0, 0, 0) facing +Z");

    println!("\n✓ Use Cases:");
    println!("  • Game audio: Footsteps, gunshots, ambient sounds");
    println!("  • VR/AR: Immersive 3D soundscapes");
    println!("  • Music production: Creative stereo placement");
    println!("  • Cinematic audio: Dialog, effects, ambience");
    println!("  • Interactive installations: Sound that responds to movement\n");

    Ok(())
}
