// Sample Slicing - Comprehensive demonstration of all slicing techniques
//
// This example shows how to slice audio samples in various ways:
// - Equal divisions
// - Time-based slicing
// - Transient/onset detection
// - Beat-based slicing
//
// Run with: cargo run --example sample_slicing

use tunes::prelude::*;
use tunes::synthesis::Sample;

fn main() -> anyhow::Result<()> {
    println!("🔪 Sample Slicing Techniques Demo\n");

    // Create a test sample (in practice, you'd load from a file)
    let sample = create_test_drumloop();

    println!("Original sample: {:.2}s, {} frames @ {}Hz\n",
        sample.duration, sample.num_frames(), sample.sample_rate());

    // ========== 1. EQUAL SLICING ==========
    println!("1️⃣  EQUAL SLICING - Divide into N equal parts");
    println!("   Perfect for: chopping breaks, creating variations\n");

    let equal_slices = sample.slice_equal(16)?;
    println!("   Created {} equal slices", equal_slices.len());

    for (_i, slice) in equal_slices.iter().take(4).enumerate() {
        println!("   Slice {}: {:.3}s - {:.3}s ({:.3}s duration)",
            slice.index,
            slice.start_time(),
            slice.end_time(),
            slice.duration
        );
    }
    println!("   ...\n");

    // ========== 2. TIME-BASED SLICING ==========
    println!("2️⃣  TIME-BASED SLICING - Slice at specific time points");
    println!("   Perfect for: precise edits, known section boundaries\n");

    // Slice at 0.25s, 0.5s, 0.75s
    let time_slices = sample.slice_at_times(&[0.25, 0.5, 0.75])?;
    println!("   Sliced at [0.25, 0.5, 0.75] → {} slices", time_slices.len());

    for slice in &time_slices {
        println!("   Slice {}: {:.3}s - {:.3}s",
            slice.index, slice.start_time(), slice.end_time());
    }
    println!();

    // ========== 3. TRANSIENT DETECTION ==========
    println!("3️⃣  TRANSIENT DETECTION - Auto-detect hit points");
    println!("   Perfect for: drum loops, percussion, finding onsets\n");

    // Detect transients with moderate sensitivity
    let transient_frames = sample.detect_transients(0.3, 50.0)?;
    println!("   Detected {} transients:", transient_frames.len());

    for (i, &frame) in transient_frames.iter().take(8).enumerate() {
        let time = frame as f32 / sample.sample_rate() as f32;
        println!("   Hit {}: frame {} ({:.3}s)", i + 1, frame, time);
    }

    if transient_frames.len() > 8 {
        println!("   ...");
    }
    println!();

    // Slice by transients (convenience method)
    let transient_slices = sample.slice_by_transients(0.3, 50.0)?;
    println!("   Created {} slices from transients\n", transient_slices.len());

    // ========== 4. BEAT-BASED SLICING ==========
    println!("4️⃣  BEAT-BASED SLICING - Slice to tempo");
    println!("   Perfect for: matching BPM, rhythmic subdivisions\n");

    // Slice into 16th notes at 140 BPM
    let beat_slices = sample.slice_at_beats(140.0, 0.25)?;
    println!("   Sliced to 16th notes @ 140 BPM: {} slices", beat_slices.len());

    // Quarter notes
    let quarter_slices = sample.slice_at_beats(140.0, 1.0)?;
    println!("   Sliced to quarter notes @ 140 BPM: {} slices\n", quarter_slices.len());

    // ========== 5. WORKING WITH SLICES ==========
    println!("5️⃣  WORKING WITH SLICES");
    println!("   Slices are lightweight references - no data copying!\n");

    let slices = sample.slice_equal(8)?;

    // Access slice properties
    println!("   Slice 3 properties:");
    println!("   - Index: {}", slices[3].index);
    println!("   - Start: {:.3}s (frame {})", slices[3].start_time(), slices[3].start_frame);
    println!("   - End: {:.3}s (frame {})", slices[3].end_time(), slices[3].end_frame);
    println!("   - Duration: {:.3}s", slices[3].duration);
    println!("   - Frames: {}", slices[3].num_frames());
    println!();

    // Convert slice to independent sample (copies data)
    println!("   Converting slice to independent sample...");
    let independent_sample = slices[3].to_sample()?;
    println!("   ✓ New sample: {:.3}s, {} frames\n",
        independent_sample.duration, independent_sample.num_frames());

    // ========== 6. MUSICAL EXAMPLE ==========
    println!("6️⃣  MUSICAL EXAMPLE - Sliced beat sequencing");
    println!("   Creating composition with sliced samples...\n");

    let _comp = Composition::new(Tempo::new(140.0));

    // Slice a drum loop into 16 equal parts
    let _drum_slices = sample.slice_equal(16)?;

    // Example: Play slices in a pattern (you'd use sample playback API)
    println!("   Pattern using slices: 0, 4, 8, 12 (kick hits)");
    println!("   Pattern using slices: 2, 6, 10, 14 (snare hits)");
    println!("   Pattern using slices: 1, 3, 5, 7, 9, 11, 13, 15 (hi-hat)");
    println!();

    // In practice, you'd play these with the composition API
    // comp.track("kick").sample_slice(&drum_slices[0], 1.0);
    // comp.track("snare").at(0.5).sample_slice(&drum_slices[4], 1.0);
    // etc.

    println!("   Note: Full slice playback integration coming soon!");
    println!("   For now, use slice.to_sample() to get playable samples.\n");

    // ========== SUMMARY ==========
    println!("📋 SUMMARY");
    println!("   ✓ Equal slicing: {} slices", equal_slices.len());
    println!("   ✓ Time-based: {} slices", time_slices.len());
    println!("   ✓ Transient detection: {} hits found", transient_frames.len());
    println!("   ✓ Beat-based (16ths @ 140BPM): {} slices", beat_slices.len());
    println!("   ✓ Beat-based (quarters @ 140BPM): {} slices", quarter_slices.len());
    println!();
    println!("🎵 All slicing methods working perfectly!");

    Ok(())
}

// Create a synthetic drum loop for testing
// In real usage, you'd load from a WAV file: Sample::from_file("drumloop.wav")?
fn create_test_drumloop() -> Sample {
    let sample_rate: u32 = 44100;
    let duration = 2.0; // 2 seconds
    let num_samples = (sample_rate as f32 * duration) as usize;

    let mut samples = Vec::with_capacity(num_samples);

    // Generate a pattern with clear transients (kick-snare pattern)
    let beats_per_second = 140.0 / 60.0; // 140 BPM
    let samples_per_beat = sample_rate as f32 / beats_per_second;

    for i in 0..num_samples {
        let mut value = 0.0;
        let position = i as f32;

        // Kick on beats 0, 2 (downbeats)
        for beat in [0.0, 2.0, 4.0, 6.0] {
            let kick_pos = beat * samples_per_beat;
            let dist = (position - kick_pos).abs();

            if dist < 2000.0 {
                // Kick drum envelope
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
                // Snare drum envelope (shorter, noisier)
                let t = dist / 1500.0;
                let env = (1.0 - t) * (-t * 8.0).exp();
                // Add noise-like component
                let noise = ((position * 0.1).sin() * 1.7).sin();
                value += noise * env * 0.6;
            }
        }

        // Hi-hat on 8th notes
        for beat in [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0, 6.5, 7.0, 7.5] {
            let hihat_pos = beat * samples_per_beat;
            let dist = (position - hihat_pos).abs();

            if dist < 800.0 {
                // Hi-hat envelope (very short)
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
