// Perlin Noise Demo - Smooth Organic Modulation
//
// Demonstrates Perlin noise for natural-sounding parameter automation.
// Perlin noise creates controllable smooth randomness that sounds organic,
// unlike mechanical sine waves or jumpy random walks.
//
// This is the "secret sauce" in modern synthesizers for LFO modulation!

use tunes::prelude::*;
use tunes::sequences;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));

    println!("=== PERLIN NOISE DEMONSTRATION ===\n");
    println!("Smooth, organic modulation - the secret of natural-sounding synthesis\n");

    // ==================
    // 1. OCTAVE COMPARISON
    // ==================
    println!("1. OCTAVE COMPARISON");
    println!("   More octaves = richer texture (Fractal Brownian Motion)\n");

    let octave_start = 0.0;

    // 1 octave - smooth and simple
    println!("   A. 1 octave - smooth drift");
    let smooth = sequences::perlin_noise::generate(42, 0.15, 1, 0.5, 32);
    for (i, &vol) in smooth.iter().enumerate() {
        let volume = 0.3 + vol * 0.5; // Map to 0.3-0.8
        comp.track("octave_1")
            .at(octave_start + i as f32 * 0.125)
            .volume(volume)
            .note(&[C5], 0.1);
    }

    // 3 octaves - natural texture
    println!("   B. 3 octaves - natural feel");
    let natural = sequences::perlin_noise::generate(42, 0.15, 3, 0.5, 32);
    for (i, &vol) in natural.iter().enumerate() {
        let volume = 0.3 + vol * 0.5;
        comp.track("octave_3")
            .at(octave_start + 5.0 + i as f32 * 0.125)
            .volume(volume)
            .note(&[E5], 0.1);
    }

    // 6 octaves - rich detail
    println!("   C. 6 octaves - detailed texture");
    let detailed = sequences::perlin_noise::generate(42, 0.15, 6, 0.5, 32);
    for (i, &vol) in detailed.iter().enumerate() {
        let volume = 0.3 + vol * 0.5;
        comp.track("octave_6")
            .at(octave_start + 10.0 + i as f32 * 0.125)
            .volume(volume)
            .note(&[G5], 0.1);
    }

    // ==================
    // 2. FREQUENCY COMPARISON
    // ==================
    let freq_start = 16.0;
    println!("\n2. FREQUENCY COMPARISON");
    println!("   Frequency controls speed of variation\n");

    // Slow drift (ambient pads)
    println!("   A. 0.05 frequency - slow ambient drift");
    let slow = sequences::perlin_noise::generate(99, 0.05, 3, 0.5, 48);
    for (i, &val) in slow.iter().enumerate() {
        let pitch = 220.0 + val * 220.0; // A3 to A4
        comp.track("freq_slow")
            .at(freq_start + i as f32 * 0.167)
            .note(&[pitch], 0.15);
    }

    // Medium evolution
    println!("   B. 0.15 frequency - natural evolution");
    let medium = sequences::perlin_noise::generate(99, 0.15, 3, 0.5, 48);
    for (i, &val) in medium.iter().enumerate() {
        let pitch = 330.0 + val * 220.0; // E4 to E5
        comp.track("freq_medium")
            .at(freq_start + 8.5 + i as f32 * 0.167)
            .note(&[pitch], 0.15);
    }

    // Fast variation
    println!("   C. 0.35 frequency - fast modulation");
    let fast = sequences::perlin_noise::generate(99, 0.35, 3, 0.5, 48);
    for (i, &val) in fast.iter().enumerate() {
        let pitch = 440.0 + val * 220.0; // A4 to A5
        comp.track("freq_fast")
            .at(freq_start + 17.0 + i as f32 * 0.167)
            .note(&[pitch], 0.15);
    }

    // ==================
    // 3. BIPOLAR MODULATION
    // ==================
    let bipolar_start = 42.0;
    println!("\n3. BIPOLAR PERLIN NOISE");
    println!("   Values in [-1, 1] for symmetric modulation\n");

    // Stereo panning automation
    println!("   A. Stereo panning (-1 = left, 1 = right)");
    let pan_curve = sequences::perlin_noise_bipolar(7, 0.1, 2, 0.5, 32);
    for (i, &pan) in pan_curve.iter().enumerate() {
        comp.track(&format!("pan_{}", i))
            .at(bipolar_start + i as f32 * 0.25)
            .pan(pan)
            .note(&[440.0], 0.2);
    }

    // Pitch detune (vibrato-like)
    println!("   B. Pitch detune (±10 Hz variation)");
    let detune_curve = sequences::perlin_noise_bipolar(13, 0.2, 3, 0.5, 32);
    for (i, &detune) in detune_curve.iter().enumerate() {
        let pitch = 440.0 + detune * 10.0; // ±10 Hz around A4
        comp.track("detune")
            .at(bipolar_start + 8.5 + i as f32 * 0.25)
            .note(&[pitch], 0.2);
    }

    // ==================
    // 4. MUSICAL APPLICATIONS
    // ==================
    let musical_start = 60.0;
    println!("\n4. MUSICAL APPLICATIONS\n");

    // A. Breathing pad (volume automation)
    println!("   A. Breathing pad - organic volume swells");
    let breath = sequences::perlin_noise::generate(777, 0.08, 4, 0.5, 64);
    for (i, &vol) in breath.iter().enumerate() {
        let volume = 0.2 + vol * 0.6; // 0.2 to 0.8
        comp.track("breathing_pad")
            .at(musical_start + i as f32 * 0.125)
            .volume(volume)
            .note(&[C4, E4, G4], 0.1); // C major triad
    }

    // B. Evolving arpeggio (pitch within scale)
    println!("   B. Evolving arpeggio - pitch drift within C major");
    let pitch_evolution = sequences::perlin_noise::generate(888, 0.12, 3, 0.5, 32);
    let c_major = vec![C4, D4, E4, F4, G4, A4, B4, C5];

    for (i, &val) in pitch_evolution.iter().enumerate() {
        // Map 0-1 to scale indices
        let index = (val * (c_major.len() - 1) as f32).round() as usize;
        comp.track("evolving_arp")
            .at(musical_start + 8.5 + i as f32 * 0.125)
            .note(&[c_major[index]], 0.1);
    }

    // C. Rhythm humanization (subtle timing variation)
    println!("   C. Rhythm humanization - natural timing drift");
    let timing = sequences::perlin_noise_bipolar(555, 0.15, 2, 0.5, 16);

    for (i, &timing_offset) in timing.iter().enumerate() {
        // ±30ms timing variation
        let time = musical_start + 17.0 + i as f32 * 0.25 + timing_offset * 0.03;
        comp.track("humanized")
            .at(time)
            .drum(DrumType::HiHatClosed);
    }

    // D. Dynamic harmony (chord voicing evolution)
    println!("   D. Dynamic harmony - shifting chord inversions");
    let harmony = sequences::perlin_noise::generate(321, 0.06, 3, 0.5, 16);

    for (i, &val) in harmony.iter().enumerate() {
        // C major chord with evolving octave distribution
        let bass = C3;
        let mid = E4 + (val * 12.0 - 6.0); // ±6 semitones
        let top = G4 + (val * 24.0 - 12.0); // ±12 semitones

        comp.track("evolving_chord")
            .at(musical_start + 21.0 + i as f32 * 0.5)
            .note(&[bass, mid, top], 0.45);
    }

    // ==================
    // 5. COMPARISON: Perlin vs Alternatives
    // ==================
    let compare_start = 90.0;
    println!("\n5. WHY PERLIN NOISE?\n");

    // Mechanical sine wave (predictable)
    println!("   A. Sine wave - mechanical, predictable");
    for i in 0..32 {
        let t = i as f32 / 32.0;
        let val = (t * std::f32::consts::PI * 4.0).sin() * 0.5 + 0.5;
        let volume = 0.3 + val * 0.5;
        comp.track("sine")
            .at(compare_start + i as f32 * 0.125)
            .volume(volume)
            .note(&[A4], 0.1);
    }

    // Perlin noise (natural)
    println!("   B. Perlin noise - smooth, organic, unpredictable");
    let perlin_comp = sequences::perlin_noise::generate(404, 0.15, 3, 0.5, 32);
    for (i, &val) in perlin_comp.iter().enumerate() {
        let volume = 0.3 + val * 0.5;
        comp.track("perlin")
            .at(compare_start + 4.5 + i as f32 * 0.125)
            .volume(volume)
            .note(&[A4], 0.1);
    }

    println!("\n   Sine: Sounds mechanical and repetitive");
    println!("   Perlin: Sounds natural and alive!");

    // ==================
    // Summary
    // ==================
    println!("\n=== PERLIN NOISE SUMMARY ===");
    println!("Frequency: Controls variation speed");
    println!("  0.01-0.05: Slow drift (pads, ambient)");
    println!("  0.05-0.15: Natural evolution (filters, vibratos)");
    println!("  0.15-0.5: Fast modulation (tremolo, rhythmic)");
    println!();
    println!("Octaves: Controls texture richness (Fractal Brownian Motion)");
    println!("  1: Smooth, simple");
    println!("  2-3: Natural, balanced");
    println!("  4-6: Rich, detailed");
    println!();
    println!("Applications:");
    println!("  - Volume automation (breathing pads)");
    println!("  - Filter sweeps (organic cutoff movement)");
    println!("  - Pitch modulation (natural vibrato depth)");
    println!("  - Panning (stereo movement)");
    println!("  - Rhythm humanization (timing variation)");
    println!("  - Timbre evolution (overtone weights)");
    println!();
    println!("The secret sauce of modern synthesis!");
    println!("Used in: Serum, Massive, Omnisphere, every major synth\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
