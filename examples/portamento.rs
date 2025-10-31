use tunes::composition::Composition;
use tunes::engine::AudioEngine;
use tunes::instruments::Instrument;
use tunes::notes::*;
use tunes::rhythm::Tempo;
use tunes::scales::*;

/// Demonstrate scale-aware portamento (glissando within a scale)
fn main() -> Result<(), anyhow::Error> {
    println!("\n🎹 Example: Portamento (Scale-Aware Glides)\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // C major scale portamento
    comp.instrument("major_scale_glide", &Instrument::synth_lead())
        .at(0.0)
        .portamento(C3, C4, &C3_MAJOR_SCALE, 0.08);

    // D minor scale portamento
    comp.instrument("minor_scale_glide", &Instrument::synth_lead())
        .at(1.2)
        .portamento(D3, D4, &D3_MINOR_SCALE, 0.08);

    // Pentatonic glide (fewer notes, more space)
    comp.instrument("pentatonic_glide", &Instrument::synth_lead())
        .at(2.4)
        .portamento(A3, A4, &A3_MINOR_PENTATONIC_SCALE, 0.1);

    // Blues scale glide
    comp.instrument("blues_glide", &Instrument::synth_lead())
        .at(3.6)
        .portamento(E3, E4, &E3_BLUES_SCALE, 0.09);

    // Fast portamento
    comp.instrument("fast_portamento", &Instrument::arp_lead())
        .at(4.8)
        .portamento(G3, G4, &G3_MAJOR_SCALE, 0.04);

    // Slow portamento
    comp.instrument("slow_portamento", &Instrument::warm_pad())
        .at(5.6)
        .portamento(F3, F4, &F3_MAJOR_SCALE, 0.15);

    // Descending portamento
    comp.instrument("descending", &Instrument::synth_lead())
        .at(7.8)
        .portamento(C5, C4, &C4_MAJOR_SCALE, 0.08);

    // Melodic minor glide
    comp.instrument("melodic_minor", &Instrument::bright_lead())
        .at(9.0)
        .portamento(A3, A4, &A3_MELODIC_MINOR_SCALE, 0.09);

    // Harmonic minor glide
    comp.instrument("harmonic_minor", &Instrument::square_lead())
        .at(10.2)
        .portamento(E3, E4, &E3_HARMONIC_MINOR_SCALE, 0.09);

    // Chromatic glide (all semitones)
    comp.instrument("chromatic", &Instrument::synth_lead())
        .at(11.4)
        .portamento(C4, C5, &C4_CHROMATIC_SCALE, 0.05);

    // Short portamento phrases
    comp.instrument("phrase_1", &Instrument::synth_lead())
        .at(12.6)
        .portamento(C4, E4, &C4_MAJOR_SCALE, 0.08)
        .portamento(E4, G4, &C4_MAJOR_SCALE, 0.08)
        .note(&[C5], 0.5);

    // Musical phrase with portamento transitions
    comp.instrument("musical_phrase", &Instrument::saw_lead())
        .at(14.0)
        .note(&[G4], 0.4)
        .portamento(G4, C5, &G4_MAJOR_SCALE, 0.06)
        .note(&[C5], 0.4)
        .portamento(C5, G4, &G4_MAJOR_SCALE, 0.06)
        .note(&[D4], 0.6);

    // Jazz-style portamento
    comp.instrument("jazz_port", &Instrument::bright_lead())
        .at(16.0)
        .note(&[E4], 0.3)
        .portamento(E4, A4, &E4_BLUES_SCALE, 0.05)
        .note(&[A4], 0.3)
        .portamento(A4, E4, &E4_BLUES_SCALE, 0.05);

    println!("✓ .portamento(start, end, scale, duration_per_note):");
    println!("  - Scale-aware glissando");
    println!("  - Only plays notes within the specified scale");
    println!("  - More musical than raw .interpolated()");
    println!("\n✓ Difference from .interpolated():");
    println!("  - interpolated: Smooth frequency sweep (all pitches)");
    println!("  - portamento: Steps through scale degrees");
    println!("\n✓ Works with any scale:");
    println!("  - Major/minor scales");
    println!("  - Pentatonic scales");
    println!("  - Blues scales");
    println!("  - Modes (Dorian, Phrygian, etc)");
    println!("  - Chromatic (all semitones)");
    println!("\n✓ Use for melodic, scale-based transitions\\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
