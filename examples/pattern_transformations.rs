// Pattern Transformation Methods Showcase
//
// Demonstrates the fifteen pattern manipulation methods for live coding and generative music:
// - shift() - Transpose patterns
// - humanize() - Add organic feel
// - rotate() - Cycle pitches
// - retrograde() - Reverse pitch order
// - shuffle() - Random reordering
// - thin() - Reduce note density
// - stack() - Layer harmonic voices
// - mutate() - Evolutionary pitch variation
// - stretch() - Time manipulation (speed up/slow down)
// - compress() - Ergonomic time compression to target duration
// - quantize() - Snap to rhythmic grid
// - palindrome() - Mirror pattern forward then backward
// - stutter() - Random glitchy stuttering
// - stutter_every() - Deterministic stuttering (every Nth note)
// - granularize() - Break notes into micro-fragments

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

    // 7. STACK - Layer harmonic voices on each note
    println!("7️⃣  STACK - Layer harmonic voices");
    comp.instrument("stack_demo", &Instrument::synth_lead())
        .wait(48.0)
        // Single note (no stacking)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .wait(0.2)
        // Stack octave above (classic doubling)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .stack(12, 1)
        .wait(0.2)
        // Stack two octaves (thick unison)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .stack(12, 2)
        .wait(0.2)
        // Stack perfect fifth (rich harmonics)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .stack(7, 1);

    // 8. MUTATE - Evolutionary pitch variation
    println!("8️⃣  MUTATE - Evolutionary pitch variation");
    comp.instrument("mutate_demo", &Instrument::synth_lead())
        .wait(56.0)
        // Original pattern
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .wait(0.2)
        // Subtle mutation (±1 semitone)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .mutate(1)
        .wait(0.2)
        // Moderate mutation (±3 semitones)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .mutate(3)
        .wait(0.2)
        // Wild mutation (±7 semitones)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .mutate(7);

    // 9. STRETCH - Time manipulation (speed up/slow down)
    println!("9️⃣  STRETCH - Time manipulation");
    comp.instrument("stretch_demo", &Instrument::synth_lead())
        .wait(64.0)
        // Original pattern at normal speed
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .wait(0.2)
        // Half speed (twice as long)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .stretch(2.0)
        .wait(0.2)
        // Double speed (half duration)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .stretch(0.5)
        .wait(0.2)
        // 1.5x slower
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .stretch(1.5);

    // 10. COMPRESS - Ergonomic time compression (no ratio math!)
    println!("🔟 COMPRESS - Target duration compression");
    comp.instrument("compress_demo", &Instrument::synth_lead())
        .wait(72.0)
        // Original pattern (2 seconds long)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .wait(0.2)
        // Compress to 1 second (half speed)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .compress(1.0)
        .wait(0.2)
        // Compress to 0.5 seconds (double speed)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .compress(0.5)
        .wait(0.2)
        // Expand to 3 seconds (1.5x slower)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .compress(3.0);

    // 11. QUANTIZE - Snap to rhythmic grid
    println!("1️⃣1️⃣  QUANTIZE - Snap to rhythmic grid");
    comp.instrument("quantize_demo", &Instrument::synth_lead())
        .wait(80.0)
        // Humanized pattern (sloppy timing)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .humanize(0.05, 0.0)  // Add timing jitter only
        .wait(0.2)
        // Same pattern quantized to 16th note grid
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .humanize(0.05, 0.0)
        .quantize(0.25)  // Clean up timing
        .wait(0.2)
        // Quantize to 8th note grid (less strict)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.25)
        .humanize(0.05, 0.0)
        .quantize(0.5);

    // 12. PALINDROME - Mirror pattern (forward then backward)
    println!("1️⃣2️⃣  PALINDROME - Mirror pattern");
    comp.instrument("palindrome_demo", &Instrument::synth_lead())
        .wait(88.0)
        // Original ascending pattern
        .pattern_start()
        .notes(&[C4, D4, E4, F4], 0.25)
        .wait(0.2)
        // Palindrome: C4, D4, E4, F4, F4, E4, D4, C4
        .pattern_start()
        .notes(&[C4, D4, E4, F4], 0.25)
        .palindrome();

    // 13. STUTTER - Random glitchy stuttering
    println!("1️⃣3️⃣  STUTTER - Random glitchy stuttering");
    comp.instrument("stutter_demo", &Instrument::synth_lead())
        .wait(96.0)
        // Original pattern
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .wait(0.2)
        // 30% chance each note stutters 4x
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .stutter(0.3, 4)
        .wait(0.2)
        // 50% chance, 8x stutter (heavy glitch)
        .pattern_start()
        .notes(&[C4, E4, G4, C5], 0.5)
        .stutter(0.5, 8);

    // 14. STUTTER_EVERY - Deterministic stuttering
    println!("1️⃣4️⃣  STUTTER_EVERY - Trap-style rolls");
    comp.instrument("stutter_every_demo", &Instrument::synth_lead())
        .wait(104.0)
        // Original pattern
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.25)
        .wait(0.2)
        // Every 4th note rolls 8x (trap hi-hat style)
        .pattern_start()
        .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.25)
        .stutter_every(4, 8);

    // 15. GRANULARIZE - Break into micro-fragments
    println!("1️⃣5️⃣  GRANULARIZE - Micro-fragments");
    comp.instrument("granularize_demo", &Instrument::synth_lead())
        .wait(112.0)
        // Original sustained note
        .pattern_start()
        .note(&[C4], 1.0)
        .wait(0.2)
        // Granularize into 10 grains
        .pattern_start()
        .note(&[C4], 1.0)
        .granularize(10)
        .wait(0.2)
        // Granular shimmer with mutation
        .pattern_start()
        .note(&[C4], 1.0)
        .granularize(20)
        .mutate(3);

    // COMBINED EXAMPLE - Chain multiple transformations
    println!("🔗 COMBINED - Chaining transformations\n");
    comp.instrument("combined_demo", &Instrument::synth_lead())
        .wait(120.0)
        .pattern_start()
        .note(&[C4], 1.0)
        .granularize(16)    // Break into 16 micro-fragments
        .mutate(4)          // Random pitch shifts
        .thin(0.7)          // Remove some grains
        .shuffle()          // Randomize order
        .humanize(0.01, 0.08);  // Add organic feel

    // Play composition (blocking - waits for playback to complete)
    println!("▶️  Playing pattern transformations demo...");
    println!("   Listen for the progression of each transformation\n");

    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    println!("✅ Demo complete!\n");
    println!("💡 Tips:");
    println!("   - Use .shift() for quick transposition in live coding");
    println!("   - Use .humanize() to make programmed parts feel natural");
    println!("   - Use .rotate() for melodic variations without retyping");
    println!("   - Use .retrograde() for classical compositional techniques");
    println!("   - Use .shuffle() for generative/random variations");
    println!("   - Use .thin() to create space or hi-hat variations");
    println!("   - Use .stack() to make sounds bigger with harmonic layers");
    println!("   - Use .mutate() for evolutionary variations and generative music");
    println!("   - Use .stretch() to speed up or slow down patterns");
    println!("   - Use .compress() to fit patterns to exact durations (no ratio math!)");
    println!("   - Use .quantize() to clean up timing after humanization");
    println!("   - Use .palindrome() for symmetrical/balanced phrases");
    println!("   - Use .stutter() for random glitch effects");
    println!("   - Use .stutter_every() for trap-style rolls and rhythmic glitches");
    println!("   - Use .granularize() for shimmering textures (amazing with .mutate())");
    println!("   - Chain them together for complex transformations!");

    Ok(())
}
