//! Tunes Live Coding Mode
//!
//! Watch a Rust file and automatically recompile + restart when it changes.
//!
//! Usage:
//!   cargo run --bin tunes-live -- my_composition.rs

use std::path::PathBuf;
use std::time::Duration;
use tunes::live_coding::{runner::LiveRunner, watcher::FileWatcher};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path-to-rust-file>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} live_session.rs", args[0]);
        std::process::exit(1);
    }

    let source_file = PathBuf::from(&args[1]);

    if !source_file.exists() {
        eprintln!("Error: File '{}' not found", source_file.display());
        std::process::exit(1);
    }

    println!("🎵 Tunes Live Coding Mode");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📁 Watching: {}", source_file.display());
    println!("💡 Edit and save to hear your changes!");
    println!("   Press Ctrl+C to exit");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Create file watcher
    let watcher = FileWatcher::new(&source_file)?;

    // Create runner
    let mut runner = LiveRunner::new(source_file.clone())?;

    // Initial compile and run
    if let Err(e) = runner.compile_and_run() {
        eprintln!("Initial compilation failed: {}", e);
        println!("\n💡 Fix the errors and save to try again...\n");
    }

    // Watch loop
    loop {
        // Check for file changes (poll every 500ms to reduce CPU usage)
        if watcher.wait_for_change(Duration::from_millis(500)) {
            println!("\n📝 File changed detected!");

            // Give extra time to ensure file is fully written
            std::thread::sleep(Duration::from_millis(500));

            // Recompile and restart
            if let Err(e) = runner.compile_and_run() {
                eprintln!("Compilation failed: {}", e);
                println!("\n💡 Fix the errors and save to try again...\n");
            }
        }

        // Check if process died unexpectedly (less frequently)
        if !runner.is_running() {
            eprintln!("\n⚠️  Process stopped unexpectedly");
            println!("💡 Waiting for file changes...\n");

            // Wait for next change
            while !watcher.wait_for_change(Duration::from_secs(1)) {
                // Keep waiting
            }

            println!("\n📝 File changed - recompiling...");
            if let Err(e) = runner.compile_and_run() {
                eprintln!("Compilation failed: {}", e);
            }
        }
    }
}
