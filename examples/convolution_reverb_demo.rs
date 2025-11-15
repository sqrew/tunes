/// Convolution Reverb Demo
///
/// Demonstrates how to use convolution reverb with the composition API.
/// This example creates a simple melody and applies different convolution
/// reverb presets to demonstrate the realistic room acoustics.
///
/// Run with: cargo run --example convolution_reverb_demo

use tunes::prelude::*;
use tunes::synthesis::effects::convolution;

fn main() -> anyhow::Result<()> {
    println!("🎵 Convolution Reverb Demo\n");

    // Create composition
    let mut comp = Composition::new(Tempo::new(120.0));

    // === Dry (no reverb) ===
    println!("Track 1: Dry (no reverb)");
    comp.instrument("dry", &Instrument::acoustic_piano())
        .notes(&[C4, E4, G4, C5], 0.5);

    // === Algorithmic Reverb (for comparison) ===
    println!("Track 2: Algorithmic reverb");
    comp.instrument("algorithmic", &Instrument::acoustic_piano())
        .at(2.0)
        .reverb(Reverb::new(0.8, 0.5, 0.5))
        .notes(&[C4, E4, G4, C5], 0.5);

    // === Small Room Convolution ===
    println!("Track 3: Convolution - Small Room (0.3s RT60)");
    comp.instrument("small_room", &Instrument::acoustic_piano())
        .at(4.0)
        .convolution_reverb(convolution::presets::small_room(0.5)?)
        .notes(&[C4, E4, G4, C5], 0.5);

    // === Concert Hall Convolution ===
    println!("Track 4: Convolution - Concert Hall (2.5s RT60)");
    comp.instrument("concert_hall", &Instrument::acoustic_piano())
        .at(6.0)
        .convolution_reverb(convolution::presets::concert_hall(0.5)?)
        .notes(&[C4, E4, G4, C5], 0.5);

    // === Cathedral Convolution ===
    println!("Track 5: Convolution - Cathedral (4.5s RT60)");
    comp.instrument("cathedral", &Instrument::acoustic_piano())
        .at(8.0)
        .convolution_reverb(convolution::presets::cathedral(0.5)?)
        .notes(&[C4, E4, G4, C5], 0.5);

    // === Plate Reverb Convolution ===
    println!("Track 6: Convolution - Plate (2.0s RT60)");
    comp.instrument("plate", &Instrument::acoustic_piano())
        .at(10.0)
        .convolution_reverb(convolution::presets::plate(0.5)?)
        .notes(&[C4, E4, G4, C5], 0.5);

    // === Spring Reverb Convolution ===
    println!("Track 7: Convolution - Spring (1.0s RT60)");
    comp.instrument("spring", &Instrument::acoustic_piano())
        .at(12.0)
        .convolution_reverb(convolution::presets::spring(0.5)?)
        .notes(&[C4, E4, G4, C5], 0.5);

    println!("\n🎧 Rendering audio...");

    // Convert to mixer and render
    let mut mixer = comp.into_mixer();

    // Export to WAV
    let output_path = "convolution_reverb_demo.wav";
    mixer.export_wav(output_path, 44100)?;

    println!("✅ Exported to: {}", output_path);
    println!("\n🎼 Listen to hear the difference between:");
    println!("   1. Dry (no reverb)");
    println!("   2. Algorithmic reverb (Freeverb)");
    println!("   3. Small room (tight, short reverb)");
    println!("   4. Concert hall (spacious, balanced)");
    println!("   5. Cathedral (huge, very long decay)");
    println!("   6. Plate (bright, metallic character)");
    println!("   7. Spring (vintage, bouncy character)");

    Ok(())
}
