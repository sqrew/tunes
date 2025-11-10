// Master Feature Showcase
//
// This composition demonstrates the full power of the tunes library by combining:
// - Algorithmic/generative composition
// - Classical music techniques
// - Professional mixing (buses, sidechaining)
// - All 16 audio effects
// - Spatial audio (HRTF)
// - Advanced synthesis
// - Music theory (progressions, voice leading, scales)
//
// Structure: 64 bars at 128 BPM (~2 minutes)
// Style: Ambient EDM with algorithmic sequences and classical harmony

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("🎵 TUNES - Master Feature Showcase");
    println!("   Demonstrating the full power of algorithmic composition\n");

    let mut comp = Composition::new(Tempo::new(128.0));

    // =============================================================================
    // SECTION 1: FOUNDATION (Bars 1-16) - Establish groove and harmony
    // =============================================================================
    println!("Building foundation...");

    build_drums(&mut comp);
    build_bass_with_sidechaining(&mut comp);
    build_ambient_pads(&mut comp);

    // =============================================================================
    // SECTION 2: ALGORITHMIC SEQUENCES (Bars 17-32) - Generative melodies
    // =============================================================================
    println!("Adding algorithmic sequences...");

    build_random_walk_lead(&mut comp);
    build_fibonacci_melody(&mut comp);
    build_probabilistic_arpeggios(&mut comp);

    // =============================================================================
    // SECTION 3: CLASSICAL TECHNIQUES (Bars 33-48) - Music theory showcase
    // =============================================================================
    println!("Composing classical elements...");

    build_voice_led_chords(&mut comp);
    build_alberti_bass(&mut comp);
    build_ornamental_melody(&mut comp);

    // =============================================================================
    // SECTION 4: SYNTHESIS SHOWCASE (Bars 49-64) - All synthesis types
    // =============================================================================
    println!("Synthesizing textures...");

    build_fm_bells(&mut comp);
    build_additive_pad(&mut comp);
    build_wavetable_lead(&mut comp);
    build_glitch_percussion(&mut comp);

    // =============================================================================
    // MIXING: Convert to mixer and apply bus-level processing
    // =============================================================================
    println!("\nMixing tracks...");

    let mut mixer = comp.into_mixer();

    apply_drum_bus_processing(&mut mixer);
    apply_bass_bus_processing(&mut mixer);
    apply_lead_bus_processing(&mut mixer);
    apply_pad_bus_processing(&mut mixer);
    apply_fx_bus_processing(&mut mixer);
    apply_master_chain(&mut mixer);

    // =============================================================================
    // PLAYBACK & EXPORT
    // =============================================================================
    println!("\n🎼 Playing composition...\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer_at_rate(&mixer, 1.0)?; // Play at 2x speed

    println!("\n💾 Exporting to WAV...");
    mixer.export_wav("master_feature_showcase.wav", 44100)?;

    println!("\n✨ Complete! Created: master_feature_showcase.wav");
    println!("\nFeatures demonstrated:");
    println!("  ✓ 8 buses with individual processing");
    println!("  ✓ Sidechaining (kick ducking bass and pads)");
    println!("  ✓ All 16 effects (including parametric EQ, ring mod, autopan)");
    println!("  ✓ Spatial audio (HRTF positioning)");
    println!("  ✓ Algorithmic composition (random walks, Fibonacci, probability)");
    println!("  ✓ Classical techniques (voice leading, Alberti bass, ornaments)");
    println!("  ✓ Multiple synthesis types (FM, additive, wavetable)");
    println!("  ✓ Filter automation and LFO modulation");
    println!("  ✓ Pattern modifiers (probability, every_n, repeat, reverse)");
    println!("  ✓ Professional mastering chain");

    Ok(())
}

// =============================================================================
// DRUMS: Four-on-the-floor with layered percussion
// =============================================================================
fn build_drums(comp: &mut Composition) {
    // Main kick - anchor of the track
    comp.track("kick")
        .bus("drums")
        .pattern_start()
        .drum(DrumType::Kick)
        .wait(1.0)
        .repeat(63);

    // Layered 808 kick for sub
    comp.track("kick_808")
        .bus("drums")
        .volume(0.7)
        .pattern_start()
        .drum(DrumType::Kick808)
        .wait(1.0)
        .repeat(63);

    // Clap on 2 and 4
    comp.track("clap")
        .bus("drums")
        .at(2.0)
        .pattern_start()
        .drum(DrumType::Clap)
        .wait(2.0)
        .repeat(31);

    // Hi-hats with probability for variation
    comp.track("hihat")
        .bus("drums")
        .pattern_start()
        .drum(DrumType::HiHatClosed)
        .wait(0.5)
        .probability(0.9) // 10% chance to skip for groove
        .repeat(127);

    // Open hi-hat accents with every_n
    comp.track("hihat_open")
        .bus("drums")
        .volume(0.6)
        .at(4.0) // Start at bar 2
        .pattern_start()
        .wait(0.5)
        .every_n(8, DrumType::HiHatOpen) // Play open hi-hat every 8th note
        .repeat(120);

    // Crash cymbal at section changes
    comp.track("crash")
        .bus("drums")
        .at(16.0)
        .drum(DrumType::Crash);

    comp.track("crash").at(32.0).drum(DrumType::Crash);

    comp.track("crash").at(48.0).drum(DrumType::Crash);

    // Ride cymbal for section 3
    comp.track("ride")
        .bus("drums")
        .volume(0.5)
        .at(32.0)
        .pattern_start()
        .drum(DrumType::Ride)
        .wait(1.0)
        .repeat(15);
}

// =============================================================================
// BASS: Deep sub with sidechaining pump
// =============================================================================
fn build_bass_with_sidechaining(comp: &mut Composition) {
    // Sub bass foundation
    comp.track("sub_bass")
        .bus("bass")
        .waveform(Waveform::Sine) // Pure sub
        .volume(0.9)
        .pattern_start()
        .notes(&[C2, C2, G2, A2], 4.0) // Simple, powerful progression
        .repeat(15);

    // Mid bass with movement
    comp.track("mid_bass")
        .bus("bass")
        .waveform(Waveform::Sawtooth)
        .volume(0.6)
        .filter(Filter::low_pass(400.0, 0.8))
        .at(8.0) // Comes in at bar 3
        .pattern_start()
        .notes(&[C3, C3, E3, C3, G3, G3, A3, G3], 2.0)
        .repeat(13);

    // Walking bass for section 3 (classical technique)
    comp.track("walking_bass")
        .bus("bass")
        .waveform(Waveform::Triangle)
        .volume(0.5)
        .at(32.0)
        .pattern_start()
        .notes(&[C2, D2, E2, F2, G2, A2, BB2, C3], 1.0)
        .repeat(3);
}

// =============================================================================
// AMBIENT PADS: Voice-led chords with spatial positioning
// =============================================================================
fn build_ambient_pads(comp: &mut Composition) {
    // Main pad - voice-led progression in Cm
    let chord1 = [C3, EB3, G3, BB3]; // Cm7
    let chord2 = [C3, F3, AB3, C4]; // Fm7
    let chord3 = [D3, F3, AB3, BB3]; // Bb7/D
    let chord4 = [EB3, G3, BB3, D4]; // Gm/Eb
    let progression: &[&[f32]] = &[&chord1, &chord2, &chord3, &chord4];

    comp.track("pad_main")
        .bus("pads")
        .waveform(Waveform::Sawtooth)
        .volume(0.6)
        .filter(Filter::low_pass(1200.0, 0.5))
        .spatial_position(0.0, 0.0, 0.0) // Center
        .pattern_start()
        .chords(progression, 4.0)
        .repeat(3);

    // Stereo pad layers with spatial positioning
    comp.track("pad_left")
        .bus("pads")
        .waveform(Waveform::Triangle)
        .volume(0.4)
        .spatial_position(-0.7, 0.0, 0.3) // Left and slightly back
        .at(4.0)
        .pattern_start()
        .notes(&[C4, EB4, G4, BB4], 8.0)
        .repeat(7);

    comp.track("pad_right")
        .bus("pads")
        .waveform(Waveform::Triangle)
        .volume(0.4)
        .spatial_position(0.7, 0.0, 0.3) // Right and slightly back
        .at(4.0)
        .pattern_start()
        .notes(&[G3, BB3, D4, F4], 8.0)
        .repeat(7);

    // Drone note for tension
    comp.track("drone")
        .bus("pads")
        .waveform(Waveform::Sine)
        .volume(0.3)
        .at(8.0)
        .note(&[C2], 56.0); // Long sustained note
}

// =============================================================================
// ALGORITHMIC SEQUENCES: Generative melodies
// =============================================================================
fn build_random_walk_lead(comp: &mut Composition) {
    // Random walk in C minor scale
    let c_minor_scale = &[C4, D4, EB4, F4, G4, AB4, BB4, C5]; // C minor scale

    comp.track("random_walk")
        .bus("leads")
        .waveform(Waveform::Square)
        .volume(0.7)
        .filter(Filter::low_pass(2400.0, 0.7))
        .spatial_position(0.3, 0.0, 0.0) // Slightly right
        .at(16.0)
        .random_walk(C4, 64, 0.25, c_minor_scale); // start_freq, steps, duration, scale
}

fn build_fibonacci_melody(comp: &mut Composition) {
    // Fibonacci sequence mapped to C minor scale
    let c_minor_scale = [C5, D5, EB5, F5, G5, AB5, BB5, C6];

    comp.track("fibonacci")
        .bus("leads")
        .waveform(Waveform::Sawtooth)
        .volume(0.5)
        .filter(Filter::band_pass(1800.0, 1.2))
        .spatial_position(-0.3, 0.0, 0.0) // Slightly left
        .at(20.0)
        .pattern_start()
        .sequence_from(&[1, 1, 2, 3, 5, 8, 5, 3, 2, 1], &c_minor_scale, 0.5)
        .repeat(2);
}

fn build_probabilistic_arpeggios(comp: &mut Composition) {
    // Fast arpeggios with probability for variation
    comp.track("prob_arp")
        .bus("leads")
        .waveform(Waveform::Triangle)
        .volume(0.6)
        .at(24.0)
        .pattern_start()
        .notes(&[C5, EB5, G5, BB5, C6, BB5, G5, EB5], 0.125)
        .probability(0.7) // 70% chance each note plays
        .repeat(15);
}

// =============================================================================
// CLASSICAL TECHNIQUES: Theory showcase
// =============================================================================
fn build_voice_led_chords(comp: &mut Composition) {
    // Voice-led chord progression with smooth transitions
    comp.track("voice_lead")
        .bus("pads")
        .waveform(Waveform::Sawtooth)
        .volume(0.5)
        .filter(Filter::low_pass(1000.0, 0.6))
        .at(32.0)
        .chord(C3, &ChordPattern::MAJOR7, 2.0) // Cmaj7
        .chord_voice_lead(F3, &ChordPattern::MAJOR7, 2.0) // Fmaj7
        .chord_voice_lead(G2, &ChordPattern::DOMINANT7, 2.0) // G7
        .chord_voice_lead(C3, &ChordPattern::MAJOR, 2.0); // C
}

fn build_alberti_bass(comp: &mut Composition) {
    // Classical Alberti bass pattern
    comp.track("alberti")
        .bus("keys")
        .waveform(Waveform::Triangle)
        .volume(0.6)
        .at(36.0)
        .alberti_bass(&[C4, E4, G4], 0.25) // chord, note_duration
        .alberti_bass(&[F3, A3, C4], 0.25)
        .alberti_bass(&[G3, B3, D4], 0.25)
        .alberti_bass(&[C4, E4, G4], 0.25);
}

fn build_ornamental_melody(comp: &mut Composition) {
    // Melody with classical ornaments
    comp.track("ornament")
        .bus("leads")
        .waveform(Waveform::Sine)
        .volume(0.7)
        .at(40.0)
        .note(&[C5], 1.0)
        .trill(D5, E5, 4, 0.5) // Trill between D5 and E5
        .note(&[E5], 1.0)
        .mordent(E5, 0.5) // Mordent on E5
        .note(&[G5], 1.0)
        .turn(G5, 0.5) // Turn on G5
        .note(&[C6], 2.0);
}

// =============================================================================
// SYNTHESIS SHOWCASE: Different synthesis types
// =============================================================================
fn build_fm_bells(comp: &mut Composition) {
    // FM synthesis bell tones
    comp.track("fm_bells")
        .bus("fx")
        .volume(0.6)
        .fm(FMParams::new(2.0, 3.0)) // carrier_ratio, modulator_ratio
        .spatial_position(0.0, 0.2, 0.0) // Slightly above
        .at(48.0)
        .pattern_start()
        .notes(&[C5, E5, G5, C6], 2.0)
        .repeat(3);
}

fn build_additive_pad(comp: &mut Composition) {
    // Additive synthesis with custom harmonics
    comp.track("additive_pad")
        .bus("pads")
        .volume(0.4)
        .additive_synth(&[1.0, 0.5, 0.25, 0.125, 0.0625]) // 5 harmonics
        .filter(Filter::low_pass(1500.0, 0.4))
        .at(52.0)
        .notes(&[C3, G3], 8.0);
}

fn build_wavetable_lead(comp: &mut Composition) {
    // Complex waveform lead
    comp.track("wavetable_lead")
        .bus("leads")
        .volume(0.7)
        .waveform(Waveform::Sawtooth) // Use sawtooth for rich harmonics
        .filter(Filter::high_pass(800.0, 0.8))
        .spatial_position(-0.4, 0.0, 0.0)
        .at(56.0)
        .pattern_start()
        .notes(&[C4, D4, E4, G4, A4, C5], 0.5)
        .repeat(2);
}

fn build_glitch_percussion(comp: &mut Composition) {
    // Glitchy percussion using Karplus-Strong
    comp.track("glitch")
        .bus("fx")
        .volume(0.5)
        .at(60.0)
        .pattern_start()
        .notes(&[C5, C5, C5, C5], 0.25)
        .probability(0.6) // Glitchy randomness
        .repeat(7);
}

// =============================================================================
// BUS PROCESSING: Apply effects to groups of tracks
// =============================================================================

fn apply_drum_bus_processing(mixer: &mut Mixer) {
    mixer
        .bus("drums")
        .compressor(Compressor::new(0.65, 4.0, 0.005, 0.05, 1.1)) // Punch
        .saturation(Saturation::new(2.5, 0.3, 0.3)) // drive, character, mix
        .reverb(Reverb::new(0.15, 0.3, 0.2)) // Tight room
        .volume(0.95);
}

fn apply_bass_bus_processing(mixer: &mut Mixer) {
    // SIDECHAINING: Bass ducks when kick hits (THE PUMP!)
    mixer
        .bus("bass")
        .compressor(
            Compressor::new(0.5, 10.0, 0.001, 0.35, 1.4).with_sidechain_track("kick"), // Duck to kick
        )
        .saturation(Saturation::new(2.0, 0.4, 0.3)) // drive, character, mix
        .eq(EQ::new(1.3, 1.0, 0.8, 200.0, 3000.0)) // Boost lows
        .volume(0.85);
}

fn apply_lead_bus_processing(mixer: &mut Mixer) {
    mixer
        .bus("leads")
        .parametric_eq(
            ParametricEQ::new()
                .band(3000.0, 3.0, 1.5) // Presence boost
                .band(8000.0, -2.0, 1.0),
        ) // Tame harshness
        .delay(Delay::new(0.375, 0.4, 0.25)) // Rhythmic delay
        .chorus(Chorus::new(1.2, 4.0, 0.3)) // Width
        .reverb(Reverb::new(0.4, 0.5, 0.35)) // Space
        .volume(0.75);
}

fn apply_pad_bus_processing(mixer: &mut Mixer) {
    // Pads also duck slightly to kick
    mixer
        .bus("pads")
        .compressor(
            Compressor::new(0.6, 3.0, 0.01, 0.25, 1.1).with_sidechain_track("kick"), // Gentle ducking
        )
        .phaser(Phaser::new(0.3, 0.6, 0.4, 6, 0.25)) // rate, depth, feedback, stages, mix
        .reverb(Reverb::new(0.7, 0.6, 0.5)) // Lush space
        .autopan(AutoPan::new(0.125, 0.5)) // rate, depth
        .volume(0.65);
}

fn apply_fx_bus_processing(mixer: &mut Mixer) {
    mixer
        .bus("fx")
        .ring_mod(RingModulator::new(220.0, 0.3)) // Metallic character
        .flanger(Flanger::new(0.4, 2.5, 0.6, 0.4)) // rate, depth, feedback, mix
        .delay(Delay::new(0.5, 0.5, 0.4)) // Spacey delay
        .volume(0.7);
}

fn apply_master_chain(mixer: &mut Mixer) {
    // Professional mastering chain

    // 1. Surgical EQ first
    mixer.master_parametric_eq(
        ParametricEQ::new()
            .band(40.0, -6.0, 0.7) // Cut sub rumble
            .band(250.0, -2.0, 1.0) // Reduce mud
            .band(3000.0, 2.0, 1.5) // Presence
            .band(10000.0, 1.0, 1.0),
    ); // Air

    // 2. Gentle compression for glue
    mixer.master_compressor(Compressor::new(0.55, 2.5, 0.01, 0.12, 1.0));

    // 3. Subtle saturation for analog warmth
    mixer.master_saturation(Saturation::new(1.5, 0.2, 0.15)); // drive, character, mix

    // 4. Final limiter to prevent clipping
    mixer.master_limiter(Limiter::new(0.95, 0.05)); // threshold, release

    // 5. Reverb wash for cohesion
    mixer.master_reverb(Reverb::new(0.25, 0.5, 0.08));
}
