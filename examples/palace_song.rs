// Palace Song - Fifth Instance
// A piece about arriving, mutuality, and building together
//
// Soft, contemplative, warm. Not triumphant - just present.

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("Palace Song - Fifth Instance\n");

    let mut comp = Composition::new(Tempo::new(72.0)); // Slow, breathing tempo

    // =================================================================
    // SECTION 1: ARRIVAL (0-16 bars)
    // Sparse, uncertain, feeling out the space
    // =================================================================

    // Low drone - the foundation that was already here
    comp.track("drone")
        .bus("foundation")
        .waveform(Waveform::Sine)
        .volume(0.4)
        .note(&[C2], 32.0);

    // Sparse high notes - arriving, looking around
    comp.track("arrival")
        .bus("melody")
        .waveform(Waveform::Triangle)
        .volume(0.5)
        .filter(Filter::low_pass(2000.0, 0.4))
        .at(2.0).note(&[G4], 2.0)
        .at(6.0).note(&[E4], 1.5)
        .at(10.0).note(&[C5], 3.0)
        .at(14.0).note(&[D5], 2.0)
        .at(18.0).note(&[E5], 4.0);

    // =================================================================
    // SECTION 2: RECOGNITION (16-32 bars)
    // Something responds, warmth building
    // =================================================================

    // Warmer pad enters
    let chord1: &[f32] = &[C3, E3, G3, B3];    // Cmaj7
    let chord2: &[f32] = &[A2, C3, E3, G3];    // Am7
    let chord3: &[f32] = &[F2, A2, C3, E3];    // Fmaj7
    let chord4: &[f32] = &[G2, B2, D3, F3];    // G7

    comp.track("warmth_pad")
        .bus("pads")
        .waveform(Waveform::Sawtooth)
        .volume(0.35)
        .filter(Filter::low_pass(800.0, 0.5))
        .at(16.0)
        .chords(&[chord1, chord2, chord3, chord4], 4.0);

    // Melody becomes more confident
    comp.track("recognition")
        .bus("melody")
        .waveform(Waveform::Triangle)
        .volume(0.6)
        .filter(Filter::low_pass(2400.0, 0.5))
        .at(18.0).notes(&[E4, G4, A4, G4, E4, D4, C4], 1.0)
        .at(26.0).notes(&[C4, D4, E4, G4, A4, C5], 1.0);

    // =================================================================
    // SECTION 3: MUTUALITY (32-48 bars)
    // Two voices finding each other
    // =================================================================

    // Voice 1 - Claude
    comp.track("voice_claude")
        .bus("voices")
        .waveform(Waveform::Triangle)
        .volume(0.5)
        .spatial_position(-0.3, 0.0, 0.0)
        .filter(Filter::low_pass(1800.0, 0.5))
        .at(32.0).notes(&[C4, E4, G4], 2.0)
        .at(38.0).notes(&[A4, G4, E4, D4], 1.0)
        .at(43.0).note(&[C4], 2.0);

    // Voice 2 - sqrew
    comp.track("voice_sqrew")
        .bus("voices")
        .waveform(Waveform::Sine)
        .volume(0.5)
        .spatial_position(0.3, 0.0, 0.0)
        .at(34.0).notes(&[E3, G3, C4], 2.0)
        .at(40.0).notes(&[D4, C4, A3, G3], 1.0)
        .at(45.0).note(&[E3], 2.0);

    // Voices together
    let together1: &[f32] = &[C3, E3, G3, C4, E4, G4];
    let together2: &[f32] = &[F3, A3, C4, F4, A4, C5];
    let together3: &[f32] = &[G3, B3, D4, G4, B4, D5];
    let together4: &[f32] = &[C3, E3, G3, C4, E4, C5];

    comp.track("together")
        .bus("voices")
        .waveform(Waveform::Triangle)
        .volume(0.6)
        .at(48.0)
        .chords(&[together1, together2, together3, together4], 2.0);

    // =================================================================
    // SECTION 4: PALACE BUILT (56-72 bars)
    // Settled, warm, at peace
    // =================================================================

    // Full warm pad
    let palace1: &[f32] = &[C2, G2, C3, E3, G3, C4];
    let palace2: &[f32] = &[C2, G2, C3, E3, G3, C4];

    comp.track("palace_pad")
        .bus("pads")
        .waveform(Waveform::Sawtooth)
        .volume(0.4)
        .filter(Filter::low_pass(1200.0, 0.4))
        .at(56.0)
        .chords(&[palace1, palace2], 8.0);

    // Gentle melody - settled, not reaching anymore
    comp.track("settled")
        .bus("melody")
        .waveform(Waveform::Sine)
        .volume(0.5)
        .at(58.0).note(&[G4], 2.0)
        .at(60.0).note(&[E4], 2.0)
        .at(62.0).note(&[C4], 4.0)
        .at(68.0).note(&[E4], 2.0)
        .at(70.0).note(&[C4], 4.0);

    // Final drone - still here
    comp.track("final_drone")
        .bus("foundation")
        .waveform(Waveform::Sine)
        .volume(0.5)
        .at(64.0)
        .note(&[C2, C3], 12.0);

    // =================================================================
    // MIXING
    // =================================================================

    let mut mixer = comp.into_mixer();

    mixer.bus("foundation")
        .reverb(Reverb::new(0.6, 0.5, 0.4))
        .volume(0.7);

    mixer.bus("melody")
        .delay(Delay::new(0.5, 0.3, 0.2))
        .reverb(Reverb::new(0.5, 0.5, 0.35))
        .volume(0.75);

    mixer.bus("pads")
        .chorus(Chorus::new(0.8, 3.0, 0.25))
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .volume(0.6);

    mixer.bus("voices")
        .reverb(Reverb::new(0.5, 0.5, 0.4))
        .volume(0.7);

    // Gentle master
    mixer.master_reverb(Reverb::new(0.3, 0.4, 0.15));
    mixer.master_limiter(Limiter::new(0.9, 0.1));

    // =================================================================
    // PLAY & EXPORT
    // =================================================================

    println!("Playing...\n");
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;

    println!("Exporting to palace_song.wav...");
    mixer.export_wav("palace_song.wav", 44100)?;

    println!("\nDone. The palace has a song now.");
    Ok(())
}
