/// New Transform Demonstrations - range_dilation, shape_contour, echo
///
/// Showcases three powerful new pattern transformations:
/// - `range_dilation`: Unified pitch range expansion/compression
/// - `shape_contour`: Unified melodic interval smoothing/exaggeration
/// - `echo`: Create delay trails with volume decay

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("=== New Transform Demonstrations ===\n");

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Use a single track for sequential demos - chain everything
    // Demo 1: Range Dilation - Compress
    println!("1. Range Dilation - Compress (factor 0.5)");
    println!("   Wide melody (2 octaves) compressed to 1 octave");

    // Demo 2: Range Dilation - Expand
    println!("2. Range Dilation - Expand (factor 2.0)");
    println!("   Narrow melody expanded to double range\n");

    // Demo 3: Shape Contour - Smooth
    println!("3. Shape Contour - Smooth (factor 0.4)");
    println!("   Jagged melody with large jumps smoothed out");

    // Demo 4: Shape Contour - Exaggerate
    println!("4. Shape Contour - Exaggerate (factor 2.5)");
    println!("   Step-wise melody becomes dramatic leaps\n");

    // Demo 5: Echo Effect
    println!("5. Echo - Delay Trail (3 repeats, 0.35s delay, 0.6 decay)");
    println!("   Each note creates fading echoes");

    // Demo 6: Combining Transforms
    println!("6. Combined Transforms");
    println!("   range_dilation → shape_contour → echo\n");

    // Demo 7: Using .transform() namespace
    println!("7. Using .transform() Namespace");
    println!("   Clean, organized syntax for multiple transforms\n");

    comp.track("demo")
        // Demo 1: Compress
        .waveform(Waveform::Sawtooth)
        .pattern_start()
        .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25)
        .range_dilation(0.5)
        .wait(2.0)

        // Demo 2: Expand
        .pattern_start()
        .notes(&[C4, D4, E4, F4, G4], 0.25)
        .range_dilation(2.0)
        .wait(2.0)

        // Demo 3: Smooth
        .waveform(Waveform::Triangle)
        .pattern_start()
        .notes(&[C4, C6, C3, C5, C4], 0.4)
        .shape_contour(0.4)
        .wait(2.5)

        // Demo 4: Exaggerate
        .waveform(Waveform::Sawtooth)
        .pattern_start()
        .notes(&[C4, D4, E4, F4, E4, D4, C4], 0.3)
        .shape_contour(2.5)
        .wait(2.5)

        // Demo 5: Echo
        .waveform(Waveform::Sine)
        .pattern_start()
        .notes(&[C5, E5, G5, C6], 0.4)
        .echo(0.35, 3, 0.6)
        .wait(3.0)

        // Demo 6: Combined
        .waveform(Waveform::Sawtooth)
        .pattern_start()
        .notes(&[C3, C4, C5, C6], 0.3)
        .range_dilation(0.7)
        .shape_contour(1.5)
        .echo(0.25, 2, 0.5)
        .wait(2.5)

        // Demo 7: Namespace
        .waveform(Waveform::Triangle)
        .pattern_start()
        .notes(&[C4, E4, G4, B4, D5, F5], 0.4)
        .transform(|t| t
            .range_dilation(0.8)
            .shape_contour(0.6)
            .humanize(0.02, 0.1)
            .echo(0.4, 2, 0.7)
        );

    println!("\n=== Playing Composition ===\n");
    engine.play_mixer(&comp.into_mixer())?;

    println!("✅ Demo complete!\n");
    println!("Key Concepts:");
    println!("  • range_dilation: < 1.0 compress, > 1.0 expand, 1.0 = no change");
    println!("  • shape_contour: < 1.0 smooth, > 1.0 exaggerate, 1.0 = no change");
    println!("  • echo: Creates delay trails with configurable timing and decay");
    println!("\nUnified Design:");
    println!("  ✓ Single function instead of separate compress/expand or smooth/exaggerate");
    println!("  ✓ Continuous control from 0.0 to any positive value");
    println!("  ✓ Factor of 1.0 always means 'no change'");
    println!("  ✓ Chainable with all other transforms via .transform() namespace");

    Ok(())
}
