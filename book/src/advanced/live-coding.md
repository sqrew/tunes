# Live Coding

Live coding is the practice of writing and modifying code in real-time to create music. Tunes provides file watching and auto-recompilation for faster iteration during composition.

## Built-In Live Coding Mode

Tunes includes a `tunes-live` binary that watches your Rust file and automatically recompiles when it changes. This is similar to `cargo-watch` but built-in, so no additional installation is required.

### Quick Start

**Option 1: Edit template in the project directory**

```bash
cargo run --release --bin tunes-live src/templates/live_template.rs
```

Edit `src/templates/live_template.rs` and save. The file will recompile and restart automatically. Since the file is in the project, you get full IDE support (autocomplete, type checking, etc.).

**Option 2: Create your own file**

```bash
# Copy the template
cp src/templates/live_template.rs my_live.rs

# Update imports from crate:: to tunes:: if the file is outside src/
# Or keep it in src/templates/ to maintain IDE support

# Start the live coding system
cargo run --release --bin tunes-live my_live.rs
```

Edit `my_live.rs` and save to trigger recompilation.

### How It Works

The `tunes-live` binary:
1. Watches your file for changes (polls every 500ms)
2. Recompiles the file when modifications are detected
3. Stops the previous audio process
4. Starts the new version

This is functionally equivalent to using `cargo-watch`, just integrated into the project.

## Live Coding Template

Basic structure from `src/templates/live_template.rs`:

```rust
use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(140.0));

    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12])
        .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);

    comp.instrument("bass", &Instrument::sub_bass())
        .notes(&[C2, C2, G2, G2], 0.5);

    comp.instrument("lead", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5], 0.25);

    let mixer = comp.into_mixer();
    let engine = AudioEngine::with_buffer_size(4096)?; // ~93ms latency at 44.1kHz

    let _loop_id = engine.play_looping(&mixer)?;

    // Keep process running - live reload will terminate and restart
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
```

## Code Organization for Live Coding

### Extract Parameters

Place frequently modified values at the top of the function:

```rust
fn main() -> anyhow::Result<()> {
    let tempo = 140.0;
    let cutoff = 1200.0;
    let resonance = 0.7;
    let melody = vec![C4, E4, G4, C5];

    let mut comp = Composition::new(Tempo::new(tempo));

    comp.instrument("lead", &Instrument::synth_lead())
        .filter(Filter::low_pass(cutoff, resonance))
        .notes(&melody, 0.5);

    // ... rest of setup
}
```

### Comment Out Unused Sections

Focus on specific parts during iteration:

```rust
comp.track("drums")
    .drum_grid(16, 0.125)
    .kick(&[0, 4, 8, 12])
    .snare(&[4, 12]);

// comp.instrument("bass", &Instrument::sub_bass())
//     .notes(&[C2, C2], 1.0);
```

### Use Algorithmic Generators

Parameterize pattern generation for quick experimentation:

```rust
use tunes::sequences::*;

let pattern = euclidean::generate(5, 16)
    .transform()
    .map_to_scale(&[C4, D4, E4, G4, A4]);

comp.instrument("melody", &Instrument::synth_lead())
    .rhythm(&pattern, 0.125);
```

### Modify Drum Patterns

Change array indices to alter rhythm:

```rust
comp.track("drums")
    .drum_grid(16, 0.125)
    .kick(&[0, 4, 8, 12])      // Modify these indices
    .snare(&[4, 12])
    .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);
```

### Add Randomization

Generate different variations on each compilation:

```rust
use rand::Rng;

let mut rng = rand::thread_rng();
let variation = rng.gen_range(-2.0..2.0);

comp.instrument("melody", &Instrument::synth_lead())
    .notes(&[C4 + variation, E4 + variation, G4 + variation], 0.5);
```

## Iterative Development Example

Building up a composition incrementally:

**Iteration 1:**
```rust
comp.instrument("bass", &Instrument::sub_bass())
    .notes(&[C2], 1.0);
```

**Iteration 2:**
```rust
comp.instrument("melody", &Instrument::synth_lead())
    .notes(&[C4, E4, G4], 0.5);
```

**Iteration 3:**
```rust
comp.track("drums")
    .drum_grid(8, 0.25)
    .kick(&[0, 4])
    .hihat(&[0, 2, 4, 6]);
```

**Iteration 4:**
```rust
comp.track("melody")
    .filter(Filter::low_pass(800.0, 0.7));
```

Each save triggers recompilation and playback.

## Audio Buffer Configuration

Adjust buffer size to balance latency and stability:

```rust
// Lower latency, may be less stable on some systems
let engine = AudioEngine::with_buffer_size(2048)?; // ~46ms at 44.1kHz

// Higher latency, more stable
let engine = AudioEngine::with_buffer_size(8192)?; // ~186ms at 44.1kHz

// Default balanced setting
let engine = AudioEngine::with_buffer_size(4096)?; // ~93ms at 44.1kHz
```

## Alternative: cargo-watch

If you prefer, use `cargo-watch` for similar functionality:

```bash
cargo install cargo-watch
cargo watch -x 'run --release --example my_song'
```

**Comparison:**

`tunes-live`:
- No external installation required
- Integrated into the project
- Stops previous audio process automatically

`cargo-watch`:
- Works with examples, tests, and other cargo commands
- Shows full cargo output
- More general-purpose tool

Both provide the same core functionality: automatic recompilation on file changes.

## Performance Considerations

1. Use `--release` mode for better audio quality (included in examples above)
2. Shorter compositions (4-8 bars) recompile faster
3. Simpler synthesis during prototyping reduces compilation time
4. Comment out complex effects chains while iterating on structure
5. Export finished compositions:
   ```rust
   mixer.export_wav("output.wav", 44100)?;
   ```

## Troubleshooting

**File path errors:**
```bash
# Verify file exists at specified path
cargo run --bin tunes-live my_live.rs
cargo run --bin tunes-live src/templates/live_template.rs
```

**Compilation failures:**
- Errors are displayed in the terminal
- Fix errors and save to retry compilation
- For external files, ensure imports use `tunes::` not `crate::`

**No audio output:**
- Verify `engine.play_looping()` is called (not `play()`)
- Check system volume settings
- Confirm composition contains audio events

**File changes not detected:**
- System polls every 500ms, so there may be a brief delay
- Some editors may not trigger file modification timestamps correctly
- Verify file modification time updates when saving

## Workflow Patterns

**Quick experimentation:**
```bash
cargo run --release --bin tunes-live src/templates/live_template.rs
```
Edit the template directly in the project.

**Dedicated composition files:**
```bash
cp src/templates/live_template.rs songs/my_song.rs
# Update imports: crate:: → tunes::
cargo run --release --bin tunes-live songs/my_song.rs
```

**Exporting final output:**
```rust
// Add before the event loop
mixer.export_wav("final.wav", 44100)?;
```

## Command Reference

```bash
# Built-in live mode
cargo run --release --bin tunes-live src/templates/live_template.rs
cargo run --release --bin tunes-live <your-file>.rs

# Using cargo-watch
cargo install cargo-watch
cargo watch -x 'run --release --example <your-example>'
cargo watch -c -x 'run --release --example <your-example>'  # Clear output
```

**File structure for live coding:**
- Place parameters at top of main function
- Build composition incrementally
- Comment out unused sections
- Use `play_looping()` for continuous playback
- Keep infinite loop at end so process doesn't terminate
