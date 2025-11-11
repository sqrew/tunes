#![allow(unused, dead_code)]
// Tunes Live Coding Template
//
// Edit this file and save to hear your changes in real-time!
//
// Quick start:
//   Run: cargo run --release --bin tunes-live src/templates/jams.rs
//   Edit this file and save - you'll hear changes instantly!
//
// For a new session:
//   1. Copy this file: cp src/templates/jams.rs my_live.rs
//   2. Change imports from `crate::` to `tunes::`
//   3. Run: cargo run --release --bin tunes-live my_live.rs

// Internal imports for IDE support (work when file is in src/templates/)
use crate::composition::{Composition, Tempo};
use crate::consts::*;
use crate::engine::AudioEngine;
use crate::instruments::Instrument;

fn main() -> anyhow::Result<()> {
    // Create your composition here
    let mut comp = Composition::new(Tempo::new(140.0));

    // Convert to mixer
    let mixer = comp.into_mixer();

    // 4096 samples = ~93ms latency at 44.1kHz - good balance for live coding
    let engine = AudioEngine::with_buffer_size(4096)?;

    // Start looping playback
    let _loop_id = engine.play_looping(&mixer)?;

    // Keep the program running
    // Live reload will stop this and restart with new code
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
