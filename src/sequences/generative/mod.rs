//! Generative algorithm generators
//!
//! This module contains algorithms for generating complex, non-repetitive patterns
//! including random walks, chaos theory, L-systems, cellular automata, and Markov chains.

pub mod random_walk;
pub mod bounded_walk;
pub mod logistic_map;
pub mod tent_map;
pub mod sine_map;
pub mod henon_map;
pub mod bakers_map;
pub mod thue_morse;
pub mod recaman;
pub mod van_der_corput;
pub mod cellular_automaton;
pub mod lsystem;
pub mod markov;
pub mod cantor_set;
pub mod lorenz_attractor;
pub mod perlin_noise;
pub mod rossler_attractor;
pub mod clifford_attractor;
pub mod ikeda_map;

// All generative sequences are now modules with generate() functions
// Use as: random_walk::generate(), logistic_map::generate(), etc.
//
// Helper functions are still available:
pub use henon_map::{henon_x, henon_y};
pub use bakers_map::{bakers_x, bakers_y};
pub use lsystem::lsystem_to_sequence;
pub use markov::build_markov_transitions;
pub use lorenz_attractor::lorenz_butterfly;
pub use perlin_noise::perlin_noise_bipolar;
pub use rossler_attractor::rossler_spiral;
pub use clifford_attractor::{clifford_x, clifford_y, clifford_flow};
pub use ikeda_map::{ikeda_x, ikeda_y, ikeda_spiral};
