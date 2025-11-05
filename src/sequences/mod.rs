#![allow(dead_code)]

//! Mathematical sequence generators for algorithmic music
//!
//! This module provides a comprehensive collection of sequence generators organized into four categories:
//!
//! # Mathematical Sequences
//!
//! Classic mathematical sequences that create interesting melodic and rhythmic patterns:
//! - **Fibonacci**: Natural growth patterns converging to golden ratio
//! - **Primes**: Irregular but deterministic patterns for non-repetitive rhythms
//! - **Arithmetic**: Linear progressions for scales and regular patterns
//! - **Geometric**: Exponential growth for dramatic changes
//! - **Triangular**: Sum sequences for smooth contours
//! - **Powers of Two**: Binary patterns fundamental to digital audio
//! - **Collatz**: The 3n+1 problem creating unpredictable wandering melodies
//!
//! # Rhythmic Patterns
//!
//! Specialized algorithms for generating rhythmic patterns:
//! - **Euclidean**: Mathematically optimal beat distribution used worldwide
//! - **Golden Ratio Rhythm**: Non-periodic but balanced rhythmic patterns
//! - **Shepard Tone**: Illusion of infinitely rising/falling pitch
//!
//! # Generative Algorithms
//!
//! Complex algorithms for creating evolving, non-repetitive patterns:
//! - **Random Walk**: Smooth stochastic variation (Brownian motion)
//! - **Bounded Walk**: Random variation constrained to a range
//! - **Logistic Map**: Chaos theory for controllable complexity
//! - **Thue-Morse**: Fair binary sequences avoiding repetition
//! - **Recamán**: Backward-looking sequence creating spiraling patterns
//! - **Van der Corput**: Quasi-random low-discrepancy sequences
//! - **Cellular Automaton**: Rule-based pattern evolution
//! - **L-System**: Fractal growth patterns from rewriting rules
//! - **Markov Chain**: Probabilistic sequences learned from data
//! - **Cantor Set**: Fractal rhythm patterns from recursive subdivision
//!
//! # Musical Transformations
//!
//! Functions for mapping sequences into musical parameters:
//! - **Harmonic Series**: Generate overtone frequencies
//! - **Golden Ratio**: Powers of φ for natural proportions
//! - **Golden Sections**: Divide values by golden ratio
//! - **Normalize**: Map sequences to specified ranges
//! - **Map to Scale**: Quantize values to musical scales
//!
//! # Examples
//!
//! ```
//! use tunes::sequences;
//! use tunes::prelude::*;
//!
//! // Create a Fibonacci melody
//! let fib = sequences::fibonacci(8);
//! let melody = sequences::normalize(&fib, 220.0, 880.0);
//!
//! // Create Euclidean rhythm
//! let kick_pattern = sequences::euclidean(4, 16);
//! let snare_pattern = sequences::euclidean(3, 16);
//!
//! // Generate chaotic variation
//! let chaos = sequences::logistic_map(3.9, 0.5, 32);
//!
//! // Map to C major scale
//! let scale_notes = sequences::map_to_scale(
//!     &fib,
//!     &sequences::Scale::major(),
//!     60,  // Middle C
//!     2    // Two octaves
//! );
//! ```

pub mod mathematical;
pub mod rhythmic;
pub mod generative;
pub mod musical;

// Re-export all functions for backward compatibility
pub use mathematical::*;
pub use rhythmic::*;
pub use generative::*;
pub use musical::*;
