//! Live coding support for tunes
//!
//! This module provides hot-reload capabilities for iterative music composition.
//! Edit your composition code and hear changes in real-time.
//!
//! # Quick Start
//!
//! 1. Copy the template:
//!    ```bash
//!    cp templates/live_template.rs my_live.rs
//!    ```
//!
//! 2. Start live coding:
//!    ```bash
//!    cargo run --bin tunes-live -- my_live.rs
//!    ```
//!
//! 3. Edit `my_live.rs` and save - hear your changes instantly!
//!
//! # How It Works
//!
//! The live coding system watches your Rust file for changes. When you save:
//! 1. The file is automatically recompiled
//! 2. The old audio process is stopped
//! 3. The new version starts playing
//!
//! This gives you instant feedback while composing!

pub mod watcher;
pub mod runner;
