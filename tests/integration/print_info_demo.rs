use tunes::prelude::*;

/// Demonstrates the opt-in .print_info() method
///
/// By default, AudioEngine is silent on initialization (good library behavior).
/// Call .print_info() if you want to see device/SIMD/latency information.

fn main() -> anyhow::Result<()> {
    println!("Creating AudioEngine (silent by default)...\n");
    let engine = AudioEngine::new()?;

    println!("Now calling .print_info() to see details:\n");
    engine.print_info();

    println!("\n✓ Engine is ready to play audio!");
    println!("  This opt-in design makes tunes a better library citizen.");
    println!("  Users who want verbose output can call .print_info().");
    println!("  Users who don't want it get a clean, silent initialization.\n");

    Ok(())
}
