//! Live coding support for tunes
//!
//! This module provides hot-reload capabilities for iterative music composition.
//! Edit your composition code and hear changes in real-time.
//!
//! # Quick Start
//!
//! Option 1 - Edit templates directly with IDE support:
//!    ```bash
//!    cargo run --release --bin tunes-live src/templates/jams.rs
//!    ```
//!    Edit `src/templates/jams.rs` and save - hear changes instantly!
//!
//! Option 2 - Create your own file:
//! 1. Copy the template:
//!    ```bash
//!    cp src/templates/live_template.rs my_live.rs
//!    ```
//!
//! 2. Update imports from `crate::` to `tunes::`
//!
//! 3. Start live coding:
//!    ```bash
//!    cargo run --release --bin tunes-live my_live.rs
//!    ```
//!
//! 4. Edit and save - hear your changes instantly!
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
