//! Mathematical sequence generators
//!
//! This module contains classic mathematical sequences that are useful for
//! generating melodies, rhythms, and structural patterns in algorithmic music.

pub mod fibonacci;
pub mod primes;
pub mod arithmetic;
pub mod geometric;
pub mod triangular;
pub mod powers_of_two;
pub mod collatz;
pub mod lucas;
pub mod catalan;
pub mod padovan;
pub mod pell;
pub mod pentagonal;

// All mathematical sequences are now modules with generate() functions
// Use as: fibonacci::generate(), collatz::generate(), primes::generate(), etc.
