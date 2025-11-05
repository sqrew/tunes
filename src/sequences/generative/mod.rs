//! Generative algorithm generators
//!
//! This module contains algorithms for generating complex, non-repetitive patterns
//! including random walks, chaos theory, L-systems, cellular automata, and Markov chains.

pub mod random_walk;
pub mod bounded_walk;
pub mod logistic_map;
pub mod thue_morse;
pub mod recaman;
pub mod van_der_corput;
pub mod cellular_automaton;
pub mod lsystem;
pub mod markov;
pub mod cantor_set;

pub use random_walk::random_walk;
pub use bounded_walk::bounded_walk;
pub use logistic_map::logistic_map;
pub use thue_morse::thue_morse;
pub use recaman::recaman;
pub use van_der_corput::van_der_corput;
pub use cellular_automaton::cellular_automaton;
pub use lsystem::{lsystem, lsystem_to_sequence};
pub use markov::{markov_chain, build_markov_transitions};
pub use cantor_set::cantor_set;
