// Pattern Transformation Methods Showcase
//
// Demonstrates the six new pattern manipulation methods for live coding and generative music:
// - shift() - Transpose patterns
// - humanize() - Add organic feel
// - rotate() - Cycle pitches
// - retrograde() - Reverse pitch order
// - shuffle() - Random reordering
// - thin() - Reduce note density

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(120.0));
    let engine = AudioEngine::new()?;

    // Original melody pattern for comparison
    println!("🎵 Pattern Transformation Methods Showcase\n");

    // 1. SHIFT - Transpose patterns up or down
    println!("1️⃣  SHIFT - Transpose patterns");
    comp.instrument("shift_demo", &Instrument::electric_piano())
        // Original phrase
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .wait(0.2)
        // Shift up a perfect fifth (7 semitones)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .shift(7)
        .wait(0.2)
        // Shift down an octave (12 semitones)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .shift(-12);

    // 2. HUMANIZE - Add organic timing and velocity variations
    println!("2️⃣  HUMANIZE - Add organic feel");
    comp.instrument("humanize_demo", &Instrument::electric_piano())
        .wait(8.0)
        // Mechanical (no humanization)
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.25)
        .wait(0.2)
        // Subtle humanization (realistic)
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.25)
        .humanize(0.01, 0.05)  // ±10ms timing, ±5% velocity
        .wait(0.2)
        // Heavy humanization (drunk piano)
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.25)
        .humanize(0.05, 0.2);  // ±50ms timing, ±20% velocity

    // 3. ROTATE - Cycle pitch positions
    println!("3️⃣  ROTATE - Cycle pitches");
    comp.instrument("rotate_demo", &Instrument::synth_lead())
        .wait(16.0)
        // Original: C4, E4, G4, C5
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .wait(0.2)
        // Rotate forward: E4, G4, C5, C4
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .rotate(1)
        .wait(0.2)
        // Rotate backward: C5, C4, E4, G4
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .rotate(-1);

    // 4. RETROGRADE - Reverse pitch sequence (classical technique)
    println!("4️⃣  RETROGRADE - Reverse pitch order");
    comp.instrument("retrograde_demo", &Instrument::synth_lead())
        .wait(24.0)
        // Original ascending line
        .pattern_start()
        .notes(&[C4, D4, E4, F4, G4, A4, B4, C5], 0.25)
        .wait(0.2)
        // Retrograde: descending from C5 to C4
        .pattern_start()
        .notes(&[C4, D4, E4, F4, G4, A4, B4, C5], 0.25)
        .retrograde();

    // 5. SHUFFLE - Random pitch reordering
    println!("5️⃣  SHUFFLE - Random reordering");
    comp.instrument("shuffle_demo", &Instrument::synth_lead())
        .wait(32.0)
        // Original arpeggio
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .wait(0.1)
        // First shuffle (random)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .shuffle()
        .wait(0.1)
        // Second shuffle (different random order)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .shuffle()
        .wait(0.1)
        // Third shuffle (yet another order)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .shuffle();

    // 6. THIN - Probabilistically remove notes
    println!("6️⃣  THIN - Reduce note density");
    comp.instrument("thin_demo", &Instrument::synth_lead())
        .wait(40.0)
        // Dense pattern (all notes)
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.125)
        .wait(0.2)
        // 70% density
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.125)
        .thin(0.7)
        .wait(0.2)
        // 50% density (half the notes)
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.125)
        .thin(0.5)
        .wait(0.2)
        // 30% density (sparse)
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.125)
        .thin(0.3);

    // COMBINED EXAMPLE - Chain multiple transformations
    println!("🔗 COMBINED - Chaining transformations\n");
    comp.instrument("combined_demo", &Instrument::synth_lead())
        .wait(48.0)
        .pattern_start()
        .notes(&[C4, D4, E4, F4, G4, A4, B4, C5], 0.25)
        .shuffle()          // Randomize order
        .shift(7)           // Transpose up a fifth
        .thin(0.75)         // Keep 75% of notes
        .humanize(0.01, 0.08);  // Add organic feel

    // Play composition
    println!("▶️  Playing pattern transformations demo...");
    println!("   Listen for the progression of each transformation\n");

    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    // Wait for playback to complete
    std::thread::sleep(std::time::Duration::from_secs(55));

    println!("✅ Demo complete!\n");
    println!("💡 Tips:");
    println!("   - Use .shift() for quick transposition in live coding");
    println!("   - Use .humanize() to make programmed parts feel natural");
    println!("   - Use .rotate() for melodic variations without retyping");
    println!("   - Use .retrograde() for classical compositional techniques");
    println!("   - Use .shuffle() for generative/random variations");
    println!("   - Use .thin() to create space or hi-hat variations");
    println!("   - Chain them together for complex transformations!");

    Ok(())
}
