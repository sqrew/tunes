use tunes::composition::Composition;
use tunes::engine::AudioEngine;
use tunes::envelope::Envelope;
use tunes::notes::*;
use tunes::rhythm::Tempo;

/// Demonstrate ADSR envelopes
fn main() -> Result<(), anyhow::Error> {
    println!("\n📈 Example: ADSR Envelopes\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Pluck: Fast attack and decay, no sustain
    comp.track("pluck")
        .envelope(Envelope::pluck())  // Built-in preset
        .at(0.0)
        .note(&[C4], 0.5);

    // Piano: Medium attack, quick decay to sustain
    comp.track("piano")
        .envelope(Envelope::piano())
        .at(0.6)
        .note(&[E4], 0.5);

    // Pad: Slow attack, long release
    comp.track("pad")
        .envelope(Envelope::pad())
        .at(1.2)
        .note(&[G4], 0.8);

    // Organ: Instant attack, full sustain, instant release
    comp.track("organ")
        .envelope(Envelope::organ())
        .at(2.2)
        .note(&[C5], 0.5);

    // Custom envelope
    comp.track("custom")
        .envelope(Envelope::new(
            0.1,   // Attack: 0.1s
            0.2,   // Decay: 0.2s
            0.7,   // Sustain: 70% level
            0.5    // Release: 0.5s
        ))
        .at(3.0)
        .note(&[E5], 0.8);

    // Show envelope shapes with a sequence
    comp.track("sequence_pluck")
        .envelope(Envelope::pluck())
        .at(4.5)
        .notes(&[C4, D4, E4, F4, G4], 0.2);

    comp.track("sequence_pad")
        .envelope(Envelope::pad())
        .at(6.0)
        .notes(&[C4, D4, E4, F4, G4], 0.4);

    println!("✓ ADSR Envelope parameters:");
    println!("  - Attack: Time to reach full volume");
    println!("  - Decay: Time to drop to sustain level");
    println!("  - Sustain: Level held while note plays");
    println!("  - Release: Fade out time after note ends");
    println!("\n✓ Built-in presets:");
    println!("  - Envelope::pluck() - Fast percussive");
    println!("  - Envelope::piano() - Natural piano-like");
    println!("  - Envelope::pad() - Slow atmospheric");
    println!("  - Envelope::organ() - Instant on/off");
    println!("\n✓ Custom: Envelope::new(attack, decay, sustain, release)\n");

    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}
