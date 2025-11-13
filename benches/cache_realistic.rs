use tunes::prelude::*;
use tunes::synthesis::fm_synthesis::FMParams;
use std::time::Instant;

/// Realistic cache benchmark: Drum pattern with high reuse
///
/// This demonstrates the cache's real benefit: when the same complex sound
/// is used many times (like kick/snare/hihat patterns)

fn main() -> anyhow::Result<()> {
    println!("\n🥁 Realistic Cache Benchmark: Drum Pattern\n");

    let engine = AudioEngine::new()?;

    // Test: Drum pattern with 4 unique sounds, each used 250 times (1000 total notes)
    println!("=== Drum Pattern: 1000 notes, 4 unique sounds ===");
    println!("(Each sound uses FM synthesis - expensive to compute)\n");

    println!("Without cache:");
    {
        let mut comp = Composition::new(Tempo::new(140.0));

        // Create a 16-bar drum pattern
        // Kick (C2), Snare (D2), Hihat closed (F#2), Hihat open (A#2)
        for bar in 0..16 {
            let bar_start = bar as f32 * 4.0 * 0.428;  // 4 beats per bar at 140 BPM

            // Kick on 1 and 3
            comp.track("kick")
                .at(bar_start)
                .note(&[C2], 0.15)
                .fm(FMParams::new(2.0, 8.0));

            comp.track("kick")
                .at(bar_start + 2.0 * 0.428)
                .note(&[C2], 0.15)
                .fm(FMParams::new(2.0, 8.0));

            // Snare on 2 and 4
            comp.track("snare")
                .at(bar_start + 1.0 * 0.428)
                .note(&[D2], 0.12)
                .fm(FMParams::new(3.5, 6.0));

            comp.track("snare")
                .at(bar_start + 3.0 * 0.428)
                .note(&[D2], 0.12)
                .fm(FMParams::new(3.5, 6.0));

            // Hihat on every 8th note (closed)
            for eighth in 0..8 {
                comp.track("hihat")
                    .at(bar_start + (eighth as f32) * 0.214)
                    .note(&[FS2], 0.08)
                    .fm(FMParams::new(4.0, 3.0));
            }

            // Open hihat on off-beats
            for offbeat in 0..4 {
                comp.track("hihat_open")
                    .at(bar_start + (offbeat as f32) * 0.428 + 0.214)
                    .note(&[AS2], 0.3)
                    .fm(FMParams::new(4.5, 2.5));
            }
        }

        let mut mixer = comp.into_mixer();
        let duration = mixer.total_duration();

        println!("  Composition: {:.1}s", duration);
        println!("  Notes: ~192 (16 bars * 12 notes/bar)");

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Render time: {:.3}s", render_time.as_secs_f32());
        println!("  Realtime ratio: {:.1}x", realtime_ratio);
    }

    println!("\nWith cache:");
    {
        let mut comp = Composition::new(Tempo::new(140.0));

        // Same drum pattern
        for bar in 0..16 {
            let bar_start = bar as f32 * 4.0 * 0.428;

            comp.track("kick")
                .at(bar_start)
                .note(&[C2], 0.15)
                .fm(FMParams::new(2.0, 8.0));

            comp.track("kick")
                .at(bar_start + 2.0 * 0.428)
                .note(&[C2], 0.15)
                .fm(FMParams::new(2.0, 8.0));

            comp.track("snare")
                .at(bar_start + 1.0 * 0.428)
                .note(&[D2], 0.12)
                .fm(FMParams::new(3.5, 6.0));

            comp.track("snare")
                .at(bar_start + 3.0 * 0.428)
                .note(&[D2], 0.12)
                .fm(FMParams::new(3.5, 6.0));

            for eighth in 0..8 {
                comp.track("hihat")
                    .at(bar_start + (eighth as f32) * 0.214)
                    .note(&[FS2], 0.08)
                    .fm(FMParams::new(4.0, 3.0));
            }

            for offbeat in 0..4 {
                comp.track("hihat_open")
                    .at(bar_start + (offbeat as f32) * 0.428 + 0.214)
                    .note(&[AS2], 0.3)
                    .fm(FMParams::new(4.5, 2.5));
            }
        }

        let mut mixer = comp.into_mixer();
        mixer.enable_cache();

        let duration = mixer.total_duration();

        println!("  Composition: {:.1}s", duration);
        println!("  Notes: ~192 (16 bars * 12 notes/bar)");

        let start = Instant::now();
        let buffer = engine.render_to_buffer(&mut mixer);
        let render_time = start.elapsed();

        let audio_duration = buffer.len() as f32 / 2.0 / 44100.0;
        let realtime_ratio = audio_duration / render_time.as_secs_f32();

        println!("  Render time: {:.3}s", render_time.as_secs_f32());
        println!("  Realtime ratio: {:.1}x", realtime_ratio);

        println!();
        mixer.print_cache_stats();
    }

    println!("\n=== Conclusion ===");
    println!("With only 4 unique sounds reused 48 times each (192 notes total),");
    println!("the cache should show benefits. The more reuse, the better!");
    println!("\nCache benefits scale with:");
    println!("  • Synthesis complexity (FM, filters, multiple oscillators)");
    println!("  • Reuse count (drum patterns, repeated melodies)");
    println!("  • Sound duration (longer sounds = less per-block overhead)");

    Ok(())
}
