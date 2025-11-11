// Slice Playback - Direct sample and slice playback in compositions
//
// This example demonstrates the new direct playback API that allows you to
// play samples and slices without the caching workflow.
//
// Run with: cargo run --example slice_playback

use tunes::prelude::*;
use tunes::synthesis::Sample;

fn main() -> anyhow::Result<()> {
    println!("🎵 Direct Sample & Slice Playback Demo\n");

    // Create a test sample (in practice: Sample::from_wav("drumloop.wav")?)
    let drumloop = create_test_drumloop();
    println!("✓ Created test drumloop: {:.2}s @ {}Hz\n",
        drumloop.duration, drumloop.sample_rate());

    // ========== METHOD 1: Traditional Caching Workflow ==========
    println!("📋 METHOD 1: Traditional Caching (still works)");
    println!("   - Load sample into composition cache");
    println!("   - Reference by name throughout composition");
    println!();

    let _comp1 = Composition::new(Tempo::new(140.0));

    // Note: In real usage, you'd load from a file:
    // comp1.load_sample("drumloop", "path/to/drumloop.wav")?;
    // comp1.track("drums")
    //     .sample("drumloop")
    //     .sample("drumloop");

    println!("   (Skipped - would load from file)");
    println!("   ✓ Traditional caching workflow available\n");

    // ========== METHOD 2: Direct Sample Playback ==========
    println!("🚀 METHOD 2: Direct Sample Playback (NEW!)");
    println!("   - No caching required");
    println!("   - Pass sample reference directly");
    println!();

    let mut comp2 = Composition::new(Tempo::new(140.0));

    // Play sample directly without caching
    comp2.track("drums")
        .play_sample(&drumloop, 1.0)
        .play_sample(&drumloop, 0.5);  // Play at half speed

    println!("   ✓ Direct sample playback (no caching)\n");

    // ========== METHOD 3: Direct Slice Playback ==========
    println!("✂️  METHOD 3: Direct Slice Playback (NEW!)");
    println!("   - Slice sample into parts");
    println!("   - Play slices directly without exporting");
    println!();

    // Slice the drumloop into 16 equal parts
    let slices = drumloop.slice_equal(16)?;
    println!("   ✓ Sliced into {} parts\n", slices.len());

    let mut comp3 = Composition::new(Tempo::new(140.0));

    // Play individual slices directly
    comp3.track("drums")
        .play_slice(&slices[0], 1.0)?   // First slice
        .play_slice(&slices[4], 1.0)?   // Fifth slice
        .play_slice(&slices[8], 1.0)?   // Ninth slice
        .play_slice(&slices[12], 1.0)?; // Thirteenth slice

    println!("   ✓ Direct slice playback\n");

    // ========== METHOD 4: Slice-Based Beat Sequencing ==========
    println!("🥁 METHOD 4: Advanced Slice Sequencing");
    println!("   - Use transient detection to find hits");
    println!("   - Rearrange and play back in new patterns");
    println!();

    // Detect transients automatically
    let transient_slices = drumloop.slice_by_transients(0.3, 50.0)?;
    println!("   ✓ Detected {} transients\n", transient_slices.len());

    let mut comp4 = Composition::new(Tempo::new(120.0));

    if transient_slices.len() >= 4 {
        // Create a new pattern from detected hits (in a different order)
        comp4.track("resequenced")
            .play_slice(&transient_slices[0], 1.0)?
            .play_slice(&transient_slices[2], 1.0)?
            .play_slice(&transient_slices[1], 1.0)?
            .play_slice(&transient_slices[3], 1.0)?;

        println!("   ✓ Resequenced from transients\n");
    }

    // ========== METHOD 5: Beat-Matched Slicing ==========
    println!("⏱️  METHOD 5: Beat-Matched Slicing");
    println!("   - Slice to exact BPM subdivisions");
    println!("   - Perfect for time-synced playback");
    println!();

    // Slice to 16th notes at 140 BPM
    let beat_slices = drumloop.slice_at_beats(140.0, 0.25)?;
    println!("   ✓ Sliced to 16th notes @ 140 BPM: {} slices\n", beat_slices.len());

    let mut comp5 = Composition::new(Tempo::new(140.0));

    if beat_slices.len() >= 8 {
        // Play every other slice (selecting specific beats)
        comp5.track("shuffled")
            .play_slice(&beat_slices[0], 1.0)?
            .play_slice(&beat_slices[2], 1.0)?
            .play_slice(&beat_slices[4], 1.0)?
            .play_slice(&beat_slices[6], 1.0)?;

        println!("   ✓ Beat-matched slice playback\n");
    }

    // ========== COMPARISON SUMMARY ==========
    println!("📊 COMPARISON:\n");

    println!("   Traditional Caching:");
    println!("   ✓ Good for: Reusing same sample many times");
    println!("   ✓ Good for: Managing samples by name");
    println!("   ✗ Requires: Pre-loading step");
    println!("   ✗ Requires: String lookup overhead\n");

    println!("   Direct Sample/Slice Playback:");
    println!("   ✓ Good for: One-off playback");
    println!("   ✓ Good for: Dynamic slice generation");
    println!("   ✓ Good for: Generative/algorithmic composition");
    println!("   ✓ No caching overhead");
    println!("   ✗ Less efficient if playing same sample repeatedly\n");

    // ========== PRACTICAL WORKFLOW ==========
    println!("💡 RECOMMENDED WORKFLOW:\n");
    println!("   1. Load audio file as Sample");
    println!("   2. Slice however you need (equal, transient, beat-based)");
    println!("   3. Play slices directly with play_slice()");
    println!("   4. No export/reimport needed!\n");

    println!("   Example:");
    println!("   ```rust");
    println!("   let sample = Sample::from_wav(\"break.wav\")?;");
    println!("   let slices = sample.slice_by_transients(0.3, 50.0)?;");
    println!("   ");
    println!("   comp.track(\"drums\")");
    println!("       .play_slice(&slices[0], 1.0)?");
    println!("       .play_slice(&slices[2], 1.0)?");
    println!("       .play_slice(&slices[1], 1.2)?;  // Speed up");
    println!("   ```\n");

    println!("🎉 All playback methods demonstrated successfully!");

    Ok(())
}

// Create a synthetic drum loop for testing
fn create_test_drumloop() -> Sample {
    let sample_rate: u32 = 44100;
    let duration = 2.0; // 2 seconds @ 140 BPM = 4 beats
    let num_samples = (sample_rate as f32 * duration) as usize;

    let mut samples = Vec::with_capacity(num_samples);

    // Generate a 4/4 pattern at 140 BPM
    let beats_per_second = 140.0 / 60.0;
    let samples_per_beat = sample_rate as f32 / beats_per_second;

    for i in 0..num_samples {
        let mut value = 0.0;
        let position = i as f32;

        // Kick on beats 0, 2 (downbeats)
        for beat in [0.0, 2.0, 4.0, 6.0] {
            let kick_pos = beat * samples_per_beat;
            let dist = (position - kick_pos).abs();

            if dist < 2000.0 {
                let t = dist / 2000.0;
                let env = (1.0 - t) * (-t * 5.0).exp();
                value += (position * 0.02).sin() * env * 0.8;
            }
        }

        // Snare on beats 1, 3 (backbeats)
        for beat in [1.0, 3.0, 5.0, 7.0] {
            let snare_pos = beat * samples_per_beat;
            let dist = (position - snare_pos).abs();

            if dist < 1500.0 {
                let t = dist / 1500.0;
                let env = (1.0 - t) * (-t * 8.0).exp();
                let noise = ((position * 0.1).sin() * 1.7).sin();
                value += noise * env * 0.6;
            }
        }

        // Hi-hat on 8th notes
        for beat in [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0, 6.5, 7.0, 7.5] {
            let hihat_pos = beat * samples_per_beat;
            let dist = (position - hihat_pos).abs();

            if dist < 800.0 {
                let t = dist / 800.0;
                let env = (1.0 - t) * (-t * 15.0).exp();
                let noise = ((position * 0.3).sin() * 2.3).sin();
                value += noise * env * 0.3;
            }
        }

        samples.push(value * 0.5);
    }

    Sample::from_mono(samples, sample_rate)
}
