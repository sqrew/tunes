// Tunes Live Coding Template
//
// Edit this file and save to hear your changes in real-time!
//
// To start live coding:
//   1. Copy this file: cp templates/live_template.rs my_live.rs
//   2. Run: cargo run --bin tunes-live -- my_live.rs
//   3. Edit my_live.rs and save - you'll hear changes instantly!

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    // Create your composition here
    let mut comp = Composition::new(Tempo::new(140.0));

    // Add drums
    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

    // Add bass
    comp.instrument("bass", &Instrument::sub_bass())
        .at(0.0)
        .notes(&[C2, C2, G1, G2], 0.5);

    // Add melody
    comp.instrument("lead", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5, G4, E4, C4, A4], 0.25);

    // Convert to mixer
    let mixer = comp.into_mixer();

    // Loop the playback instead of repeating the mixer
    // (Repeating creates too many events for smooth real-time synthesis)
    // 4096 samples = ~93ms latency at 44.1kHz - good balance for live coding
    let engine = AudioEngine::with_buffer_size(4096)?;

    loop {
        engine.play_mixer(&mixer)?;
    }
}
