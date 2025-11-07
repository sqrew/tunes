// Complete 808 & 909 Drum Machine Demo + Transition Effects
//
// Showcasing the legendary 808 and 909 drum machines plus modern transition FX:
//
// 808 Kit (12 total):
// - Kick, Snare, 2 Hi-Hats, Clap
// - 3 Toms (Low, Mid, High) - NEW!
// - Cowbell, Clave - NEW!
//
// 909 Kit (7 total):
// - Kick, Snare
// - 2 Hi-Hats (Closed, Open) - NEW!
// - Clap, Cowbell, Rim - NEW!
//
// Transition Effects (2):
// - Reverse Snare (buildup)
// - Cymbal Swell (wash)

use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut comp = Composition::new(Tempo::new(115.0));
    let eighth = comp.tempo().eighth_note();
    let quarter = comp.tempo().quarter_note();
    let half = quarter * 2.0;

    println!("=== 808 & 909 DRUM MACHINE LIBRARY ===\n");

    // ==================
    // Section 1: 808 Kit Demo
    // ==================
    println!("1. COMPLETE 808 KIT");
    println!("   Classic Roland TR-808 sounds");
    println!("   - Known for: Deep bass, snappy snare, metallic hi-hats");
    println!("   - Era: 1980-1984");
    println!("   - Genre: Hip-hop, trap, electronic\n");

    comp.track("808_basics")
        .at(0.0)
        .drum(DrumType::Kick808)
        .wait(quarter)
        .drum(DrumType::Snare808)
        .wait(quarter)
        .drum(DrumType::HiHat808Closed)
        .wait(eighth)
        .drum(DrumType::HiHat808Open)
        .wait(eighth)
        .drum(DrumType::Clap808);

    // NEW: 808 Toms
    println!("   808 Toms (NEW):");
    comp.track("808_toms")
        .at(2.5)
        .drum(DrumType::Tom808Low)
        .wait(0.3)
        .drum(DrumType::Tom808Mid)
        .wait(0.3)
        .drum(DrumType::Tom808High);

    // NEW: 808 Cowbell & Clave
    println!("   808 Cowbell & Clave (NEW):");
    comp.track("808_perc")
        .at(3.5)
        .drum(DrumType::Cowbell808)
        .wait(0.25)
        .drum(DrumType::Cowbell808)
        .wait(0.25)
        .drum(DrumType::Clave808)
        .wait(0.25)
        .drum(DrumType::Clave808);

    // ==================
    // Section 2: 909 Kit Demo
    // ==================
    println!("\n2. COMPLETE 909 KIT");
    println!("   Classic Roland TR-909 sounds");
    println!("   - Known for: Punchy kick, crisp hi-hats, tight snare");
    println!("   - Era: 1984-1985");
    println!("   - Genre: House, techno, dance\n");

    comp.track("909_basics")
        .at(5.0)
        .drum(DrumType::Kick909)
        .wait(quarter)
        .drum(DrumType::Snare909);

    // NEW: 909 Hi-Hats
    println!("   909 Hi-Hats (NEW):");
    comp.track("909_hats")
        .at(6.0)
        .drum(DrumType::HiHat909Closed)
        .wait(eighth)
        .drum(DrumType::HiHat909Closed)
        .wait(eighth)
        .drum(DrumType::HiHat909Open);

    // NEW: 909 Clap, Cowbell, Rim
    println!("   909 Clap, Cowbell, Rim (NEW):");
    comp.track("909_perc")
        .at(7.5)
        .drum(DrumType::Clap909)
        .wait(quarter)
        .drum(DrumType::Cowbell909)
        .wait(quarter)
        .drum(DrumType::Rim909);

    // ==================
    // Section 3: Transition Effects
    // ==================
    println!("\n3. TRANSITION EFFECTS (NEW)");
    println!("   Modern production techniques");
    println!("   - Reverse Snare: Buildup effect (1.2s)");
    println!("   - Cymbal Swell: Wash/riser (2.0s)\n");

    // Reverse snare buildup
    comp.track("transitions")
        .at(9.0)
        .drum(DrumType::ReverseSnare)
        .wait(1.2)
        .drum(DrumType::Kick808); // Drop after reverse

    // Cymbal swell
    comp.track("transitions2")
        .at(11.0)
        .drum(DrumType::CymbalSwell)
        .wait(2.0)
        .drum(DrumType::Snare909); // Hit after swell

    // ==================
    // Section 4: 808 Groove
    // ==================
    println!("4. CLASSIC 808 PATTERN");
    println!("   Hip-hop style with new toms and cowbell\n");

    let groove_start = 14.0;
    comp.track("808_groove_kick")
        .at(groove_start)
        .drum(DrumType::Kick808)
        .wait(quarter)
        .wait(quarter)
        .drum(DrumType::Kick808)
        .wait(quarter)
        .wait(eighth)
        .drum(DrumType::Kick808);

    comp.track("808_groove_snare")
        .at(groove_start + quarter)
        .drum(DrumType::Snare808)
        .wait(half)
        .drum(DrumType::Snare808);

    comp.track("808_groove_hats")
        .at(groove_start)
        .drum(DrumType::HiHat808Closed)
        .wait(eighth)
        .drum(DrumType::HiHat808Closed)
        .wait(eighth)
        .drum(DrumType::HiHat808Closed)
        .wait(eighth)
        .drum(DrumType::HiHat808Open)
        .wait(eighth)
        .drum(DrumType::HiHat808Closed)
        .wait(eighth)
        .drum(DrumType::HiHat808Closed)
        .wait(eighth)
        .drum(DrumType::HiHat808Closed)
        .wait(eighth)
        .drum(DrumType::HiHat808Closed);

    // Add cowbell on offbeats
    comp.track("808_groove_cowbell")
        .at(groove_start + eighth)
        .drum(DrumType::Cowbell808)
        .wait(quarter)
        .drum(DrumType::Cowbell808)
        .wait(quarter)
        .drum(DrumType::Cowbell808)
        .wait(quarter)
        .drum(DrumType::Cowbell808);

    // Tom fill at end
    comp.track("808_groove_fill")
        .at(groove_start + half + quarter + eighth)
        .drum(DrumType::Tom808High)
        .wait(0.15)
        .drum(DrumType::Tom808Mid)
        .wait(0.15)
        .drum(DrumType::Tom808Low);

    // ==================
    // Section 5: 909 Groove
    // ==================
    println!("5. CLASSIC 909 PATTERN");
    println!("   House/techno style with crisp hi-hats\n");

    let groove2_start = 16.5;
    comp.track("909_groove_kick")
        .at(groove2_start)
        .drum(DrumType::Kick909)
        .wait(quarter)
        .drum(DrumType::Kick909)
        .wait(quarter)
        .drum(DrumType::Kick909)
        .wait(quarter)
        .drum(DrumType::Kick909);

    comp.track("909_groove_snare")
        .at(groove2_start + quarter)
        .drum(DrumType::Clap909)
        .wait(half)
        .drum(DrumType::Clap909);

    comp.track("909_groove_hats")
        .at(groove2_start)
        .drum(DrumType::HiHat909Closed)
        .wait(eighth)
        .drum(DrumType::HiHat909Closed)
        .wait(eighth)
        .drum(DrumType::HiHat909Open)
        .wait(eighth)
        .drum(DrumType::HiHat909Closed)
        .wait(eighth)
        .drum(DrumType::HiHat909Closed)
        .wait(eighth)
        .drum(DrumType::HiHat909Closed)
        .wait(eighth)
        .drum(DrumType::HiHat909Open)
        .wait(eighth)
        .drum(DrumType::HiHat909Closed);

    // Add rim clicks
    comp.track("909_groove_rim")
        .at(groove2_start + eighth)
        .drum(DrumType::Rim909)
        .wait(quarter)
        .drum(DrumType::Rim909)
        .wait(quarter)
        .drum(DrumType::Rim909)
        .wait(quarter)
        .drum(DrumType::Rim909);

    // ==================
    // Section 6: Transition Demo
    // ==================
    println!("6. FULL TRANSITION SEQUENCE");
    println!("   Using buildups and drops\n");

    let trans_start = 19.0;

    // Build tension with reverse snare
    comp.track("transition_buildup")
        .at(trans_start)
        .drum(DrumType::ReverseSnare)
        .wait(1.2);

    // Drop with 808 kick
    comp.track("transition_drop")
        .at(trans_start + 1.2)
        .drum(DrumType::Kick808)
        .wait(0.0)
        .drum(DrumType::CrashShort);

    // Cymbal swell into 909 pattern
    comp.track("transition_swell")
        .at(trans_start + 2.5)
        .drum(DrumType::CymbalSwell);

    comp.track("transition_drop2")
        .at(trans_start + 4.5)
        .drum(DrumType::Kick909)
        .wait(0.0)
        .drum(DrumType::Clap909);

    println!("=== DRUM LIBRARY TOTALS ===");
    println!("808 Kit: 12 drums (Kick, Snare, 2 HiHats, Clap, 3 Toms, Cowbell, Clave, plus variations)");
    println!("909 Kit: 7 drums (Kick, Snare, 2 HiHats, Clap, Cowbell, Rim)");
    println!("Transition FX: 2 (Reverse Snare, Cymbal Swell)");
    println!("\n**Grand Total: 91 drums!**\n");

    let engine = AudioEngine::new()?;
    engine.play_mixer(&comp.into_mixer())?;

    Ok(())
}
