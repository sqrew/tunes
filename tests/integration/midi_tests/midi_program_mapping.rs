/// Example demonstrating MIDI program mapping to instrument presets
///
/// This example:
/// 1. Creates a simple MIDI file with multiple tracks and program changes
/// 2. Imports the MIDI file
/// 3. Verifies instruments are automatically selected based on GM program numbers
/// 4. Exports to audio to demonstrate the different sounds
///
/// General MIDI Program Numbers:
/// - 0: Acoustic Grand Piano
/// - 33: Electric Bass (finger)
/// - 40: Violin
/// - 56: Trumpet
/// - 80: Square Lead (synth)

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("=== MIDI Program Mapping Test ===");
    println!();

    // Step 1: Create a composition with different instruments
    let mut comp = Composition::new(Tempo::new(120.0));

    // Piano part (will be exported with program 0)
    comp.track("Piano")
        .notes(&[C4, E4, G4, C5], 0.5);

    // Bass part (will be exported with program 33)
    comp.track("Bass")
        .note(&[C3], 2.0);

    // Strings part (will be exported with program 40)
    comp.track("Strings")
        .note(&[E4, G4], 2.0);

    // Brass part (will be exported with program 56)
    comp.track("Trumpet")
        .wait(1.0)
        .notes(&[C5, D5, E5], 0.33);

    // Synth lead part (will be exported with program 80)
    comp.track("Lead")
        .wait(1.0)
        .notes(&[G5, A5, B5, C6], 0.25);

    let mixer = comp.into_mixer();

    println!("📤 Exporting to program_mapping_test.mid with default programs...");
    mixer.export_midi("program_mapping_test.mid")?;
    println!("   ✓ Exported");
    println!();

    // Step 2: Create a MIDI file with explicit program changes
    // We'll manually create a MIDI file with specific program numbers
    println!("📝 Creating MIDI file with specific GM programs...");
    create_gm_midi_file()?;
    println!("   ✓ Created gm_programs_test.mid");
    println!();

    // Step 3: Import the MIDI file with program changes
    println!("📥 Importing MIDI file with program changes...");
    let mut imported_mixer = Mixer::import_midi("gm_programs_test.mid")?;
    println!("   ✓ Imported {} tracks", imported_mixer.all_tracks().len());
    println!();

    // Step 4: Display the mapped instruments
    println!("🎹 Mapped Instruments:");
    println!();
    for (i, track) in imported_mixer.all_tracks().iter().enumerate() {
        let program_info = track
            .midi_program
            .map(|p| format!("GM Program {}", p))
            .unwrap_or_else(|| "No program".to_string());

        println!(
            "  Track {}: {} ({})",
            i + 1,
            track.name.as_deref().unwrap_or("Unnamed"),
            program_info
        );
    }
    println!();

    // Step 5: Export to audio to demonstrate the different sounds
    println!("🔊 Exporting to audio...");
    imported_mixer.export_wav("gm_programs_test.wav", 44100)?;
    println!("   ✓ Exported gm_programs_test.wav");
    println!();

    println!("✅ Success!");
    println!();
    println!("What happened:");
    println!("  1. Created MIDI file with different GM program numbers");
    println!("  2. Imported MIDI and automatically mapped programs to instruments:");
    println!("     - Program 0 (Piano) → Acoustic Piano preset");
    println!("     - Program 33 (Bass) → Fingerstyle Bass preset");
    println!("     - Program 40 (Violin) → Violin preset");
    println!("     - Program 56 (Trumpet) → Solo Trumpet preset");
    println!("     - Program 80 (Synth Lead) → Square Lead preset");
    println!("  3. Each track now uses the appropriate waveform and envelope");
    println!("  4. Play gm_programs_test.wav to hear the different instruments");
    println!();
    println!("Note:");
    println!("  - 128 GM program numbers map to 160+ instrument presets");
    println!("  - Without program changes, notes use default Sine wave");
    println!("  - With program changes, notes use mapped instrument presets");

    Ok(())
}

/// Create a MIDI file with explicit program changes for testing
fn create_gm_midi_file() -> anyhow::Result<()> {
    use midly::{
        Format, Header, MetaMessage, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind,
    };
    use midly::num::{u15, u28, u4, u7};

    let ppq = 480;

    // Create SMF structure
    let mut smf = Smf::new(Header {
        format: Format::Parallel,
        timing: Timing::Metrical(u15::new(ppq)),
    });

    // Tempo track
    let mut tempo_track = Track::new();
    tempo_track.push(TrackEvent {
        delta: u28::new(0),
        kind: TrackEventKind::Meta(MetaMessage::Tempo(500_000.into())), // 120 BPM
    });
    tempo_track.push(TrackEvent {
        delta: u28::new(0),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });
    smf.tracks.push(tempo_track);

    // Track 1: Piano (Program 0)
    {
        let mut track = Track::new();
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::TrackName(b"Piano")),
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: MidiMessage::ProgramChange {
                    program: u7::new(0),
                },
            },
        });
        // C4
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOn { key: u7::new(60), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(240),
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOff { key: u7::new(60), vel: u7::new(0) },
            },
        });
        // E4
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOn { key: u7::new(64), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(240),
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOff { key: u7::new(64), vel: u7::new(0) },
            },
        });
        // G4
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOn { key: u7::new(67), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(240),
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOff { key: u7::new(67), vel: u7::new(0) },
            },
        });
        // C5
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOn { key: u7::new(72), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(240),
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOff { key: u7::new(72), vel: u7::new(0) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });
        smf.tracks.push(track);
    }

    // Track 2: Bass (Program 33 - Electric Bass finger)
    {
        let mut track = Track::new();
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::TrackName(b"Bass")),
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(1),
                message: MidiMessage::ProgramChange {
                    program: u7::new(33),
                },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(1),
                message: MidiMessage::NoteOn { key: u7::new(48), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(960),
            kind: TrackEventKind::Midi {
                channel: u4::new(1),
                message: MidiMessage::NoteOff { key: u7::new(48), vel: u7::new(0) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });
        smf.tracks.push(track);
    }

    // Track 3: Violin (Program 40)
    {
        let mut track = Track::new();
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::TrackName(b"Violin")),
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(2),
                message: MidiMessage::ProgramChange {
                    program: u7::new(40),
                },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(2),
                message: MidiMessage::NoteOn { key: u7::new(64), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(960),
            kind: TrackEventKind::Midi {
                channel: u4::new(2),
                message: MidiMessage::NoteOff { key: u7::new(64), vel: u7::new(0) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });
        smf.tracks.push(track);
    }

    // Track 4: Trumpet (Program 56)
    {
        let mut track = Track::new();
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::TrackName(b"Trumpet")),
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(3),
                message: MidiMessage::ProgramChange {
                    program: u7::new(56),
                },
            },
        });
        // C5
        track.push(TrackEvent {
            delta: u28::new(480),
            kind: TrackEventKind::Midi {
                channel: u4::new(3),
                message: MidiMessage::NoteOn { key: u7::new(72), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(160),
            kind: TrackEventKind::Midi {
                channel: u4::new(3),
                message: MidiMessage::NoteOff { key: u7::new(72), vel: u7::new(0) },
            },
        });
        // D5
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(3),
                message: MidiMessage::NoteOn { key: u7::new(74), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(160),
            kind: TrackEventKind::Midi {
                channel: u4::new(3),
                message: MidiMessage::NoteOff { key: u7::new(74), vel: u7::new(0) },
            },
        });
        // E5
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(3),
                message: MidiMessage::NoteOn { key: u7::new(76), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(160),
            kind: TrackEventKind::Midi {
                channel: u4::new(3),
                message: MidiMessage::NoteOff { key: u7::new(76), vel: u7::new(0) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });
        smf.tracks.push(track);
    }

    // Track 5: Square Lead (Program 80)
    {
        let mut track = Track::new();
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::TrackName(b"Lead")),
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(4),
                message: MidiMessage::ProgramChange {
                    program: u7::new(80),
                },
            },
        });
        // G5
        track.push(TrackEvent {
            delta: u28::new(480),
            kind: TrackEventKind::Midi {
                channel: u4::new(4),
                message: MidiMessage::NoteOn { key: u7::new(79), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(120),
            kind: TrackEventKind::Midi {
                channel: u4::new(4),
                message: MidiMessage::NoteOff { key: u7::new(79), vel: u7::new(0) },
            },
        });
        // A5
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(4),
                message: MidiMessage::NoteOn { key: u7::new(81), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(120),
            kind: TrackEventKind::Midi {
                channel: u4::new(4),
                message: MidiMessage::NoteOff { key: u7::new(81), vel: u7::new(0) },
            },
        });
        // B5
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(4),
                message: MidiMessage::NoteOn { key: u7::new(83), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(120),
            kind: TrackEventKind::Midi {
                channel: u4::new(4),
                message: MidiMessage::NoteOff { key: u7::new(83), vel: u7::new(0) },
            },
        });
        // C6
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Midi {
                channel: u4::new(4),
                message: MidiMessage::NoteOn { key: u7::new(84), vel: u7::new(100) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(120),
            kind: TrackEventKind::Midi {
                channel: u4::new(4),
                message: MidiMessage::NoteOff { key: u7::new(84), vel: u7::new(0) },
            },
        });
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });
        smf.tracks.push(track);
    }

    // Write to file
    smf.save("gm_programs_test.mid")?;

    Ok(())
}
