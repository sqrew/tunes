use tunes::prelude::*;

/// Demonstrate MIDI file export
///
/// This example creates a simple composition and exports it to a MIDI file.
/// MIDI files can be opened in DAWs, music notation software, or MIDI players.
fn main() -> anyhow::Result<()> {
    println!("\n🎹 MIDI Export Demo\n");

    let mut comp = Composition::new(Tempo::new(120.0));

    // Example 1: Simple melody
    println!("Creating melody track...");
    comp.instrument("melody", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5, G4, E4, C4], 0.5);

    // Example 2: Bass line
    println!("Creating bass track...");
    comp.instrument("bass", &Instrument::sub_bass())
        .at(0.0)
        .notes(&[C2, C2, G2, G2, C2, C2, G2, G2], 0.5);

    // Example 3: Drum pattern
    println!("Creating drum track...");
    comp.track("drums")
        .at(0.0)
        .drum_grid(16, 0.125)
        .sound(DrumType::Kick, &[0, 4, 8, 12])
        .sound(DrumType::Snare, &[4, 12])
        .sound(DrumType::HiHatClosed, &[0, 2, 4, 6, 8, 10, 12, 14]);

    // Example 4: Chords
    println!("Creating chord track...");
    comp.track("chords")
        .at(0.0)
        .note(&[C3, E3, G3], 1.0)
        .note(&[G3, B3, D4], 1.0)
        .note(&[A3, C4, E4], 1.0)
        .note(&[F3, A3, C4], 1.0);

    println!("\n=== Exporting to MIDI ===\n");

    let mixer = comp.into_mixer();

    // Export to MIDI file
    println!("Exporting to 'output.mid'...");
    mixer.export_midi("output.mid")?;

    println!("✅ MIDI file created successfully!\n");

    println!("MIDI Export Details:");
    println!("  • File: output.mid");
    println!("  • Tempo: 120 BPM");
    println!("  • Format: Standard MIDI File (Type 1)");
    println!(
        "  • Tracks: {} (including tempo track)",
        mixer.all_tracks().len() + 1
    );
    println!("  • Resolution: 480 PPQ (Pulses Per Quarter Note)\n");

    println!("What's Exported:");
    println!("  ✅ Note pitches (converted from Hz to MIDI note numbers)");
    println!("  ✅ Note durations and timing");
    println!("  ✅ Drum hits (mapped to General MIDI percussion)");
    println!("  ✅ Tempo information");
    println!("  ✅ Track separation\n");

    println!("What's NOT Exported (MIDI Limitations):");
    println!("  ❌ Sample playback (WAV files)");
    println!("  ❌ Effects (reverb, delay, filters)");
    println!("  ❌ Synthesis parameters (waveforms, envelopes, FM)");
    println!("  ❌ Custom wavetables");
    println!("  ℹ️  MIDI only stores note events, not audio or synthesis details\n");

    println!("You can now open 'output.mid' in:");
    println!("  • DAWs: Ableton, FL Studio, Logic Pro, Reaper, etc.");
    println!("  • Notation software: MuseScore, Finale, Sibelius");
    println!("  • MIDI players: VLC, Windows Media Player, QuickTime");
    println!("  • Online: https://onlinesequencer.net (paste MIDI file)\n");

    Ok(())
}
