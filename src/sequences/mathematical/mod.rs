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

pub use fibonacci::fibonacci;
pub use primes::primes;
pub use arithmetic::arithmetic;
pub use geometric::geometric;
pub use triangular::triangular;
pub use powers_of_two::powers_of_two;
pub use collatz::collatz;
