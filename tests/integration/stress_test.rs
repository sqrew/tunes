use tunes::prelude::*;

/// Aggressive stress test - if this works, you're golden
fn main() -> anyhow::Result<()> {
    println!("\n🔥 STRESS TEST: Maximum Complexity\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(140.0));

    // Way more aggressive
    let num_tracks = 50;
    let events_per_track = 80;
    println!("Creating composition with:");
    println!("  - {} tracks", num_tracks);
    println!("  - {} events per track", events_per_track);
    println!("  - Total events: {}", num_tracks * events_per_track);
    println!("  - Heavy effects: reverb, delay, chorus\n");

    // Create many overlapping tracks
    for track_idx in 0..num_tracks {
        let track_name = format!("track_{}", track_idx);

        let instrument = match track_idx % 4 {
            0 => Instrument::pluck(),
            1 => Instrument::synth_lead(),
            2 => Instrument::warm_pad(),
            _ => Instrument::sub_bass(),
        };

        let base_freq = 150.0 + (track_idx as f32 * 30.0);
        let freqs: Vec<f32> = (0..events_per_track)
            .map(|i| base_freq * (1.0 + (i as f32 * 0.03)))
            .collect();

        // Add heavy effects
        comp.instrument(&track_name, &instrument)
            .at(0.0)
            .filter(Filter::low_pass(1200.0, 0.7))
            .reverb(Reverb::new(0.8, 0.5, 0.3)) // Large hall: high room size, medium damping
            .delay(Delay::new(0.2, 0.4, 0.3))
            .chorus(Chorus::new(0.7, 0.5, 0.3))
            .bitcrusher(BitCrusher::new(4.0, 8.0, 1.0))
            .distortion(Distortion::new(1.0, 1.0))
            .limiter(Limiter::new(0.0, 0.0))
            .ring_mod(RingModulator::new(440.0, 1.0))
            .phaser(Phaser::new(1.0, 1.0, 0.5, 4, 1.0))
            .flanger(Flanger::new(1.0, 3.0, 0.5, 1.0))
            .notes(&freqs, 0.1);
    }

    let mixer = comp.into_mixer();
    let duration = mixer.total_duration();

    println!("Composition ready:");
    println!("  Duration: {:.2}s", duration);
    println!(
        "  Polyphony: {} simultaneous events (worst case)",
        num_tracks
    );
    println!();

    println!("🎵 Playing... watch for ALSA underruns!\n");

    engine.play_mixer(&mixer)?;

    println!("\n✅ If you made it here without underruns, you're good!");
    println!("   No further optimization needed for real-world use.\n");

    Ok(())
}
