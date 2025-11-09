// Sidechaining / Ducking Example
//
// This example demonstrates sidechaining, a technique where one signal
// controls the compression of another signal. Most commonly used in EDM
// to make bass "duck" out of the way of kick drums, creating that
// signature "pumping" effect.

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("Creating sidechaining examples...");

    // Example 1: Classic EDM kick-ducking-bass
    classic_edm_pump()?;

    // Example 2: Subtle sidechaining for groove
    subtle_groove()?;

    // Example 3: Bus-level sidechaining (multiple tracks duck together)
    bus_level_ducking()?;

    // Example 4: Aggressive pumping
    aggressive_pump()?;

    println!("\nAll examples created successfully!");
    println!("Listen to hear the different sidechaining styles:");
    println!("  - classic_edm_pump.wav: Standard EDM bass ducking");
    println!("  - subtle_groove.wav: Gentle ducking for musical groove");
    println!("  - bus_level_ducking.wav: Entire synth bus ducks to drums");
    println!("  - aggressive_pump.wav: Extreme pumping effect");

    Ok(())
}

/// Classic EDM kick-ducking-bass pattern
///
/// This is the standard sidechaining setup you hear in house, techno, and EDM.
/// The bass ducks whenever the kick hits, creating space in the mix and adding
/// rhythmic movement.
fn classic_edm_pump() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(128.0));

    // Create a four-on-the-floor kick pattern
    comp.track("kick")
        .bus("drums")
        .pattern_start()
        .drum(DrumType::Kick)
        .wait(1.0)
        .repeat(15);

    // Create a sustained bass line
    comp.track("bass")
        .bus("bass")
        .waveform(Waveform::Sawtooth)
        .pattern_start()
        .notes(&[C2], 4.0)
        .repeat(3);

    let mut mixer = comp.into_mixer();

    // Configure bass to duck when kick hits
    // - threshold: 0.6 (compress when signal > 0.6)
    // - ratio: 8.0 (aggressive compression)
    // - attack: 0.001 (1ms - very fast, immediate ducking)
    // - release: 0.15 (150ms - quick return for pumping effect)
    // - makeup_gain: 1.2 (compensate for gain reduction)
    mixer.bus("bass").compressor(
        Compressor::new(0.6, 8.0, 0.001, 0.15, 1.2)
            .with_sidechain_track("kick"), // Duck when "kick" track plays
    );

    mixer.export_wav("classic_edm_pump.wav", 44100)?;
    println!("✓ Created classic_edm_pump.wav");
    Ok(())
}

/// Subtle sidechaining for musical groove
///
/// More gentle parameters create a subtle ducking that adds groove without
/// being obvious. Great for keeping mixes clean without the aggressive pumping.
fn subtle_groove() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    // Kick pattern
    comp.track("kick")
        .bus("drums")
        .pattern_start()
        .drum(DrumType::Kick)
        .wait(1.0)
        .repeat(15);

    // Add hi-hats for rhythm
    comp.track("hihat")
        .bus("drums")
        .pattern_start()
        .drum(DrumType::HiHatClosed)
        .wait(0.5)
        .repeat(31);

    // Bass line with some movement
    comp.track("bass")
        .bus("bass")
        .waveform(Waveform::Sawtooth)
        .pattern_start()
        .notes(&[C2, C2, G2, A2], 1.0)
        .repeat(3);

    let mut mixer = comp.into_mixer();

    // Subtle sidechaining parameters:
    // - threshold: 0.7 (only compress on louder kicks)
    // - ratio: 3.0 (gentle compression)
    // - attack: 0.01 (10ms - slightly slower for smoother ducking)
    // - release: 0.25 (250ms - longer release for musical fade-back)
    // - makeup_gain: 1.0 (no boost needed with gentle compression)
    mixer.bus("bass").compressor(
        Compressor::new(0.7, 3.0, 0.01, 0.25, 1.0).with_sidechain_track("kick"),
    );

    mixer.export_wav("subtle_groove.wav", 44100)?;
    println!("✓ Created subtle_groove.wav");
    Ok(())
}

/// Bus-level sidechaining
///
/// Multiple tracks on a bus can duck together. Here we have several synth
/// layers that all duck as a group when the drums hit.
fn bus_level_ducking() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(130.0));

    // Drum pattern with kick and snare
    comp.track("kick")
        .bus("drums")
        .pattern_start()
        .drum(DrumType::Kick)
        .wait(1.0)
        .repeat(15);

    comp.track("snare")
        .bus("drums")
        .at(2.0)
        .pattern_start()
        .drum(DrumType::Snare)
        .wait(2.0)
        .repeat(7);

    // Multiple synth tracks on the same bus
    comp.track("pad")
        .bus("synths")
        .waveform(Waveform::Sawtooth)
        .pattern_start()
        .notes(&[C3, E3, G3], 8.0)
        .repeat(1);

    comp.track("lead")
        .bus("synths")
        .waveform(Waveform::Square)
        .pattern_start()
        .notes(&[C4, D4, E4, G4], 1.0)
        .repeat(3);

    let mut mixer = comp.into_mixer();

    // The entire "synths" bus ducks when the "drums" bus plays
    // This is bus-to-bus sidechaining
    mixer.bus("synths").compressor(
        Compressor::new(0.65, 6.0, 0.005, 0.2, 1.1).with_sidechain_bus("drums"),
    );

    mixer.export_wav("bus_level_ducking.wav", 44100)?;
    println!("✓ Created bus_level_ducking.wav");
    Ok(())
}

/// Aggressive pumping effect
///
/// Extreme parameters create an obvious, rhythmic pumping effect. This is
/// the sound of modern bass music - the sidechain becomes a musical element.
fn aggressive_pump() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(128.0));

    // Fast kick pattern for intense pumping
    comp.track("kick")
        .bus("drums")
        .pattern_start()
        .drum(DrumType::Kick)
        .wait(0.5)
        .repeat(31);

    // Sustained pad that will pump heavily
    comp.track("pad")
        .bus("pads")
        .waveform(Waveform::Sawtooth)
        .filter(Filter::low_pass(800.0, 0.7))
        .reverb(Reverb::new(0.6, 0.4, 0.3))
        .pattern_start()
        .notes(&[C3, E3, G3], 16.0);

    let mut mixer = comp.into_mixer();

    // Extreme sidechaining:
    // - threshold: 0.5 (compress easily)
    // - ratio: 20.0 (extreme compression, almost like a gate)
    // - attack: 0.001 (instant ducking)
    // - release: 0.4 (longer release creates the pump)
    // - makeup_gain: 2.0 (heavy boost for dramatic effect)
    mixer.bus("pads").compressor(
        Compressor::new(0.5, 20.0, 0.001, 0.4, 2.0).with_sidechain_track("kick"),
    );

    mixer.export_wav("aggressive_pump.wav", 44100)?;
    println!("✓ Created aggressive_pump.wav");
    Ok(())
}
