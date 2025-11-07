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

pub use fibonacci::fibonacci;
pub use primes::primes;
pub use arithmetic::arithmetic;
pub use geometric::geometric;
pub use triangular::triangular;
pub use powers_of_two::powers_of_two;
pub use collatz::collatz;
pub use lucas::lucas;
pub use catalan::catalan;
pub use padovan::padovan;
pub use pell::pell;
pub use pentagonal::pentagonal;
