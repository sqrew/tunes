use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("Generating time signature examples...");

    // Example 1: Simple time signature changes
    let mut comp1 = Composition::new(Tempo::new(120.0));
    comp1
        .track("melody")
        .time_signature(4, 4)
        .notes(&[C4, E4, G4, C5], 0.5) // 4 beats in 4/4
        .time_signature(3, 4)
        .notes(&[C5, G4, E4], 0.5) // 3 beats in 3/4
        .time_signature(4, 4)
        .notes(&[C4, E4, G4, C5], 0.5); // Back to 4/4

    let mixer1 = comp1.into_mixer();
    mixer1.export_midi("time_sig_simple.mid")?;
    println!("✓ Created time_sig_simple.mid - demonstrates 4/4 → 3/4 → 4/4");

    // Example 2: Waltz (3/4 throughout)
    let mut comp2 = Composition::new(Tempo::new(180.0));
    comp2
        .track("melody")
        .time_signature(3, 4)
        .notes(&[C5, G4, E4], 0.5)
        .notes(&[D5, A4, F4], 0.5)
        .notes(&[E5, G4, C4], 0.5)
        .notes(&[C5, E4, C4], 0.5);

    comp2
        .track("bass")
        .time_signature(3, 4)
        .at(0.0)
        .notes(&[C3, C3, C3], 0.5)
        .notes(&[D3, D3, D3], 0.5)
        .notes(&[E3, E3, E3], 0.5)
        .notes(&[C3, C3, C3], 0.5);

    let mixer2 = comp2.into_mixer();
    mixer2.export_midi("time_sig_waltz.mid")?;
    println!("✓ Created time_sig_waltz.mid - waltz in 3/4 time");

    // Example 3: Complex time signatures (5/4, 7/8)
    let mut comp3 = Composition::new(Tempo::new(140.0));
    comp3
        .track("melody")
        .time_signature(5, 4)
        .notes(&[E4, F4, G4, A4, G4], 0.5) // Famous 5/4 pattern
        .notes(&[E4, F4, G4, A4, G4], 0.5)
        .time_signature(7, 8)
        .notes(&[C5, D5, E5, F5, E5, D5, C5], 0.25) // 7/8 time
        .time_signature(4, 4)
        .notes(&[C4, E4, G4, C5], 0.5);

    let mixer3 = comp3.into_mixer();
    mixer3.export_midi("time_sig_complex.mid")?;
    println!("✓ Created time_sig_complex.mid - demonstrates 5/4, 7/8, and 4/4");

    // Example 4: Progressive time signature changes
    let mut comp4 = Composition::new(Tempo::new(120.0));
    comp4
        .track("melody")
        .time_signature(4, 4)
        .notes(&[C4, D4, E4, F4], 0.5)
        .time_signature(3, 4)
        .notes(&[G4, A4, B4], 0.5)
        .time_signature(5, 4)
        .notes(&[C5, B4, A4, G4, F4], 0.5)
        .time_signature(6, 8)
        .notes(&[E4, F4, G4, A4, B4, C5], 0.25)
        .time_signature(4, 4)
        .notes(&[C5, E5, G5, C6], 0.5);

    let mixer4 = comp4.into_mixer();
    mixer4.export_midi("time_sig_progressive.mid")?;
    println!("✓ Created time_sig_progressive.mid - 4/4 → 3/4 → 5/4 → 6/8 → 4/4");

    // Example 5: Combining tempo and time signature changes
    let mut comp5 = Composition::new(Tempo::new(120.0));
    comp5
        .track("melody")
        .time_signature(4, 4)
        .tempo(120.0)
        .notes(&[C4, E4, G4, C5], 0.5)
        .time_signature(3, 4)
        .tempo(90.0) // Slow waltz
        .notes(&[C5, G4, E4], 0.66)
        .notes(&[C5, G4, E4], 0.66)
        .time_signature(4, 4)
        .tempo(160.0) // Fast 4/4
        .notes(&[C4, E4, G4, C5], 0.25);

    let mixer5 = comp5.into_mixer();
    mixer5.export_midi("time_sig_with_tempo.mid")?;
    println!("✓ Created time_sig_with_tempo.mid - combines tempo and time signature changes");

    // Example 6: Multi-track with different time signature changes
    let mut comp6 = Composition::new(Tempo::new(120.0));

    comp6
        .track("melody")
        .time_signature(4, 4)
        .notes(&[C5, E5, G5, C6], 0.5)
        .time_signature(3, 4)
        .notes(&[C6, G5, E5], 0.5)
        .time_signature(4, 4)
        .notes(&[C5, E5, G5, C6], 0.5);

    comp6
        .track("bass")
        .at(0.0)
        .time_signature(4, 4)
        .notes(&[C3, C3, C3, C3], 0.5)
        .time_signature(3, 4)
        .notes(&[C3, C3, C3], 0.5)
        .time_signature(4, 4)
        .notes(&[C3, C3, C3, C3], 0.5);

    comp6
        .track("drums")
        .at(0.0)
        .time_signature(4, 4)
        .drum(DrumType::Kick)
        .drum(DrumType::Snare)
        .drum(DrumType::Kick)
        .drum(DrumType::Snare)
        .time_signature(3, 4)
        .drum(DrumType::Kick)
        .drum(DrumType::Snare)
        .drum(DrumType::Kick)
        .time_signature(4, 4)
        .drum(DrumType::Kick)
        .drum(DrumType::Snare)
        .drum(DrumType::Kick)
        .drum(DrumType::Snare);

    let mixer6 = comp6.into_mixer();
    mixer6.export_midi("time_sig_multitrack.mid")?;
    println!(
        "✓ Created time_sig_multitrack.mid - multiple instruments with time signature changes"
    );

    // Example 7: 6/8 time (compound meter)
    let mut comp7 = Composition::new(Tempo::new(80.0));
    comp7
        .track("melody")
        .time_signature(6, 8)
        // Typical 6/8 pattern: emphasis on beats 1 and 4
        .notes(&[C5, E5, G5, C5, E5, G5], 0.33)
        .notes(&[D5, F5, A5, D5, F5, A5], 0.33)
        .notes(&[E5, G5, B5, E5, G5, B5], 0.33)
        .notes(&[C5, E5, G5, C5, E5, G5], 0.33);

    let mixer7 = comp7.into_mixer();
    mixer7.export_midi("time_sig_6_8.mid")?;
    println!("✓ Created time_sig_6_8.mid - compound meter (6/8 time)");

    // Example 8: Odd time signatures (7/4, 11/8)
    let mut comp8 = Composition::new(Tempo::new(100.0));
    comp8
        .track("melody")
        .time_signature(7, 4)
        .notes(&[C4, D4, E4, F4, G4, A4, B4], 0.5)
        .notes(&[B4, A4, G4, F4, E4, D4, C4], 0.5)
        .time_signature(11, 8)
        .notes(&[C4, D4, E4, F4, G4, A4, B4, C5, B4, A4, G4], 0.25);

    let mixer8 = comp8.into_mixer();
    mixer8.export_midi("time_sig_odd.mid")?;
    println!("✓ Created time_sig_odd.mid - odd time signatures (7/4 and 11/8)");

    println!("\n✓ All time signature examples generated successfully!");
    println!("  Import these MIDI files into your DAW or notation software");
    println!("  to see the time signature changes properly displayed.");

    Ok(())
}
