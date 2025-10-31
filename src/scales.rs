#![allow(dead_code)]

use crate::notes::*;

// ===== MAJOR SCALES =====

// Octave 1
pub const C1_MAJOR_SCALE: &[f32] = &[C1, D1, E1, F1, G1, A1, B1, C2];
pub const CS1_MAJOR_SCALE: &[f32] = &[CS1, DS1, F1, FS1, GS1, AS1, C2, CS2];
pub const D1_MAJOR_SCALE: &[f32] = &[D1, E1, FS1, G1, A1, B1, CS2, D2];
pub const DS1_MAJOR_SCALE: &[f32] = &[DS1, F1, G1, GS1, AS1, C2, D2, DS2];
pub const E1_MAJOR_SCALE: &[f32] = &[E1, FS1, GS1, A1, B1, CS2, DS2, E2];
pub const F1_MAJOR_SCALE: &[f32] = &[F1, G1, A1, AS1, C2, D2, E2, F2];
pub const FS1_MAJOR_SCALE: &[f32] = &[FS1, GS1, AS1, B1, CS2, DS2, F2, FS2];
pub const G1_MAJOR_SCALE: &[f32] = &[G1, A1, B1, C2, D2, E2, FS2, G2];
pub const GS1_MAJOR_SCALE: &[f32] = &[GS1, AS1, C2, CS2, DS2, F2, G2, GS2];
pub const A1_MAJOR_SCALE: &[f32] = &[A1, B1, CS2, D2, E2, FS2, GS2, A2];
pub const AS1_MAJOR_SCALE: &[f32] = &[AS1, C2, D2, DS2, F2, G2, A2, AS2];
pub const B1_MAJOR_SCALE: &[f32] = &[B1, CS2, DS2, E2, FS2, GS2, AS2, B2];

// Octave 2
pub const C2_MAJOR_SCALE: &[f32] = &[C2, D2, E2, F2, G2, A2, B2, C3];
pub const CS2_MAJOR_SCALE: &[f32] = &[CS2, DS2, F2, FS2, GS2, AS2, C3, CS3];
pub const D2_MAJOR_SCALE: &[f32] = &[D2, E2, FS2, G2, A2, B2, CS3, D3];
pub const DS2_MAJOR_SCALE: &[f32] = &[DS2, F2, G2, GS2, AS2, C3, D3, DS3];
pub const E2_MAJOR_SCALE: &[f32] = &[E2, FS2, GS2, A2, B2, CS3, DS3, E3];
pub const F2_MAJOR_SCALE: &[f32] = &[F2, G2, A2, AS2, C3, D3, E3, F3];
pub const FS2_MAJOR_SCALE: &[f32] = &[FS2, GS2, AS2, B2, CS3, DS3, F3, FS3];
pub const G2_MAJOR_SCALE: &[f32] = &[G2, A2, B2, C3, D3, E3, FS3, G3];
pub const GS2_MAJOR_SCALE: &[f32] = &[GS2, AS2, C3, CS3, DS3, F3, G3, GS3];
pub const A2_MAJOR_SCALE: &[f32] = &[A2, B2, CS3, D3, E3, FS3, GS3, A3];
pub const AS2_MAJOR_SCALE: &[f32] = &[AS2, C3, D3, DS3, F3, G3, A3, AS3];
pub const B2_MAJOR_SCALE: &[f32] = &[B2, CS3, DS3, E3, FS3, GS3, AS3, B3];

// Octave 3
pub const C3_MAJOR_SCALE: &[f32] = &[C3, D3, E3, F3, G3, A3, B3, C4];
pub const CS3_MAJOR_SCALE: &[f32] = &[CS3, DS3, F3, FS3, GS3, AS3, C4, CS4];
pub const D3_MAJOR_SCALE: &[f32] = &[D3, E3, FS3, G3, A3, B3, CS4, D4];
pub const DS3_MAJOR_SCALE: &[f32] = &[DS3, F3, G3, GS3, AS3, C4, D4, DS4];
pub const E3_MAJOR_SCALE: &[f32] = &[E3, FS3, GS3, A3, B3, CS4, DS4, E4];
pub const F3_MAJOR_SCALE: &[f32] = &[F3, G3, A3, AS3, C4, D4, E4, F4];
pub const FS3_MAJOR_SCALE: &[f32] = &[FS3, GS3, AS3, B3, CS4, DS4, F4, FS4];
pub const G3_MAJOR_SCALE: &[f32] = &[G3, A3, B3, C4, D4, E4, FS4, G4];
pub const GS3_MAJOR_SCALE: &[f32] = &[GS3, AS3, C4, CS4, DS4, F4, G4, GS4];
pub const A3_MAJOR_SCALE: &[f32] = &[A3, B3, CS4, D4, E4, FS4, GS4, A4];
pub const AS3_MAJOR_SCALE: &[f32] = &[AS3, C4, D4, DS4, F4, G4, A4, AS4];
pub const B3_MAJOR_SCALE: &[f32] = &[B3, CS4, DS4, E4, FS4, GS4, AS4, B4];

// Octave 4
pub const C4_MAJOR_SCALE: &[f32] = &[C4, D4, E4, F4, G4, A4, B4, C5];
pub const CS4_MAJOR_SCALE: &[f32] = &[CS4, DS4, F4, FS4, GS4, AS4, C5, CS5];
pub const D4_MAJOR_SCALE: &[f32] = &[D4, E4, FS4, G4, A4, B4, CS5, D5];
pub const DS4_MAJOR_SCALE: &[f32] = &[DS4, F4, G4, GS4, AS4, C5, D5, DS5];
pub const E4_MAJOR_SCALE: &[f32] = &[E4, FS4, GS4, A4, B4, CS5, DS5, E5];
pub const F4_MAJOR_SCALE: &[f32] = &[F4, G4, A4, AS4, C5, D5, E5, F5];
pub const FS4_MAJOR_SCALE: &[f32] = &[FS4, GS4, AS4, B4, CS5, DS5, F5, FS5];
pub const G4_MAJOR_SCALE: &[f32] = &[G4, A4, B4, C5, D5, E5, FS5, G5];
pub const GS4_MAJOR_SCALE: &[f32] = &[GS4, AS4, C5, CS5, DS5, F5, G5, GS5];
pub const A4_MAJOR_SCALE: &[f32] = &[A4, B4, CS5, D5, E5, FS5, GS5, A5];
pub const AS4_MAJOR_SCALE: &[f32] = &[AS4, C5, D5, DS5, F5, G5, A5, AS5];
pub const B4_MAJOR_SCALE: &[f32] = &[B4, CS5, DS5, E5, FS5, GS5, AS5, B5];

// Octave 5
pub const C5_MAJOR_SCALE: &[f32] = &[C5, D5, E5, F5, G5, A5, B5, C6];
pub const CS5_MAJOR_SCALE: &[f32] = &[CS5, DS5, F5, FS5, GS5, AS5, C6, CS6];
pub const D5_MAJOR_SCALE: &[f32] = &[D5, E5, FS5, G5, A5, B5, CS6, D6];
pub const DS5_MAJOR_SCALE: &[f32] = &[DS5, F5, G5, GS5, AS5, C6, D6, DS6];
pub const E5_MAJOR_SCALE: &[f32] = &[E5, FS5, GS5, A5, B5, CS6, DS6, E6];
pub const F5_MAJOR_SCALE: &[f32] = &[F5, G5, A5, AS5, C6, D6, E6, F6];
pub const FS5_MAJOR_SCALE: &[f32] = &[FS5, GS5, AS5, B5, CS6, DS6, F6, FS6];
pub const G5_MAJOR_SCALE: &[f32] = &[G5, A5, B5, C6, D6, E6, FS6, G6];
pub const GS5_MAJOR_SCALE: &[f32] = &[GS5, AS5, C6, CS6, DS6, F6, G6, GS6];
pub const A5_MAJOR_SCALE: &[f32] = &[A5, B5, CS6, D6, E6, FS6, GS6, A6];
pub const AS5_MAJOR_SCALE: &[f32] = &[AS5, C6, D6, DS6, F6, G6, A6, AS6];
pub const B5_MAJOR_SCALE: &[f32] = &[B5, CS6, DS6, E6, FS6, GS6, AS6, B6];

// Octave 6
pub const C6_MAJOR_SCALE: &[f32] = &[C6, D6, E6, F6, G6, A6, B6, C7];
pub const CS6_MAJOR_SCALE: &[f32] = &[CS6, DS6, F6, FS6, GS6, AS6, C7, CS7];
pub const D6_MAJOR_SCALE: &[f32] = &[D6, E6, FS6, G6, A6, B6, CS7, D7];
pub const DS6_MAJOR_SCALE: &[f32] = &[DS6, F6, G6, GS6, AS6, C7, D7, DS7];
pub const E6_MAJOR_SCALE: &[f32] = &[E6, FS6, GS6, A6, B6, CS7, DS7, E7];
pub const F6_MAJOR_SCALE: &[f32] = &[F6, G6, A6, AS6, C7, D7, E7, F7];
pub const FS6_MAJOR_SCALE: &[f32] = &[FS6, GS6, AS6, B6, CS7, DS7, F7, FS7];
pub const G6_MAJOR_SCALE: &[f32] = &[G6, A6, B6, C7, D7, E7, FS7, G7];
pub const GS6_MAJOR_SCALE: &[f32] = &[GS6, AS6, C7, CS7, DS7, F7, G7, GS7];
pub const A6_MAJOR_SCALE: &[f32] = &[A6, B6, CS7, D7, E7, FS7, GS7, A7];
pub const AS6_MAJOR_SCALE: &[f32] = &[AS6, C7, D7, DS7, F7, G7, A7, AS7];
pub const B6_MAJOR_SCALE: &[f32] = &[B6, CS7, DS7, E7, FS7, GS7, AS7, B7];

// ===== NATURAL MINOR SCALES =====

// Octave 1
pub const C1_MINOR_SCALE: &[f32] = &[C1, D1, DS1, F1, G1, GS1, AS1, C2];
pub const CS1_MINOR_SCALE: &[f32] = &[CS1, DS1, E1, FS1, GS1, A1, B1, CS2];
pub const D1_MINOR_SCALE: &[f32] = &[D1, E1, F1, G1, A1, AS1, C2, D2];
pub const DS1_MINOR_SCALE: &[f32] = &[DS1, F1, FS1, GS1, AS1, B1, CS2, DS2];
pub const E1_MINOR_SCALE: &[f32] = &[E1, FS1, G1, A1, B1, C2, D2, E2];
pub const F1_MINOR_SCALE: &[f32] = &[F1, G1, GS1, AS1, C2, CS2, DS2, F2];
pub const FS1_MINOR_SCALE: &[f32] = &[FS1, GS1, A1, B1, CS2, D2, E2, FS2];
pub const G1_MINOR_SCALE: &[f32] = &[G1, A1, AS1, C2, D2, DS2, F2, G2];
pub const GS1_MINOR_SCALE: &[f32] = &[GS1, AS1, B1, CS2, DS2, E2, FS2, GS2];
pub const A1_MINOR_SCALE: &[f32] = &[A1, B1, C2, D2, E2, F2, G2, A2];
pub const AS1_MINOR_SCALE: &[f32] = &[AS1, C2, CS2, DS2, F2, FS2, GS2, AS2];
pub const B1_MINOR_SCALE: &[f32] = &[B1, CS2, D2, E2, FS2, G2, A2, B2];

// Octave 2
pub const C2_MINOR_SCALE: &[f32] = &[C2, D2, DS2, F2, G2, GS2, AS2, C3];
pub const CS2_MINOR_SCALE: &[f32] = &[CS2, DS2, E2, FS2, GS2, A2, B2, CS3];
pub const D2_MINOR_SCALE: &[f32] = &[D2, E2, F2, G2, A2, AS2, C3, D3];
pub const DS2_MINOR_SCALE: &[f32] = &[DS2, F2, FS2, GS2, AS2, B2, CS3, DS3];
pub const E2_MINOR_SCALE: &[f32] = &[E2, FS2, G2, A2, B2, C3, D3, E3];
pub const F2_MINOR_SCALE: &[f32] = &[F2, G2, GS2, AS2, C3, CS3, DS3, F3];
pub const FS2_MINOR_SCALE: &[f32] = &[FS2, GS2, A2, B2, CS3, D3, E3, FS3];
pub const G2_MINOR_SCALE: &[f32] = &[G2, A2, AS2, C3, D3, DS3, F3, G3];
pub const GS2_MINOR_SCALE: &[f32] = &[GS2, AS2, B2, CS3, DS3, E3, FS3, GS3];
pub const A2_MINOR_SCALE: &[f32] = &[A2, B2, C3, D3, E3, F3, G3, A3];
pub const AS2_MINOR_SCALE: &[f32] = &[AS2, C3, CS3, DS3, F3, FS3, GS3, AS3];
pub const B2_MINOR_SCALE: &[f32] = &[B2, CS3, D3, E3, FS3, G3, A3, B3];

// Octave 3
pub const C3_MINOR_SCALE: &[f32] = &[C3, D3, DS3, F3, G3, GS3, AS3, C4];
pub const CS3_MINOR_SCALE: &[f32] = &[CS3, DS3, E3, FS3, GS3, A3, B3, CS4];
pub const D3_MINOR_SCALE: &[f32] = &[D3, E3, F3, G3, A3, AS3, C4, D4];
pub const DS3_MINOR_SCALE: &[f32] = &[DS3, F3, FS3, GS3, AS3, B3, CS4, DS4];
pub const E3_MINOR_SCALE: &[f32] = &[E3, FS3, G3, A3, B3, C4, D4, E4];
pub const F3_MINOR_SCALE: &[f32] = &[F3, G3, GS3, AS3, C4, CS4, DS4, F4];
pub const FS3_MINOR_SCALE: &[f32] = &[FS3, GS3, A3, B3, CS4, D4, E4, FS4];
pub const G3_MINOR_SCALE: &[f32] = &[G3, A3, AS3, C4, D4, DS4, F4, G4];
pub const GS3_MINOR_SCALE: &[f32] = &[GS3, AS3, B3, CS4, DS4, E4, FS4, GS4];
pub const A3_MINOR_SCALE: &[f32] = &[A3, B3, C4, D4, E4, F4, G4, A4];
pub const AS3_MINOR_SCALE: &[f32] = &[AS3, C4, CS4, DS4, F4, FS4, GS4, AS4];
pub const B3_MINOR_SCALE: &[f32] = &[B3, CS4, D4, E4, FS4, G4, A4, B4];

// Octave 4
pub const C4_MINOR_SCALE: &[f32] = &[C4, D4, DS4, F4, G4, GS4, AS4, C5];
pub const CS4_MINOR_SCALE: &[f32] = &[CS4, DS4, E4, FS4, GS4, A4, B4, CS5];
pub const D4_MINOR_SCALE: &[f32] = &[D4, E4, F4, G4, A4, AS4, C5, D5];
pub const DS4_MINOR_SCALE: &[f32] = &[DS4, F4, FS4, GS4, AS4, B4, CS5, DS5];
pub const E4_MINOR_SCALE: &[f32] = &[E4, FS4, G4, A4, B4, C5, D5, E5];
pub const F4_MINOR_SCALE: &[f32] = &[F4, G4, GS4, AS4, C5, CS5, DS5, F5];
pub const FS4_MINOR_SCALE: &[f32] = &[FS4, GS4, A4, B4, CS5, D5, E5, FS5];
pub const G4_MINOR_SCALE: &[f32] = &[G4, A4, AS4, C5, D5, DS5, F5, G5];
pub const GS4_MINOR_SCALE: &[f32] = &[GS4, AS4, B4, CS5, DS5, E5, FS5, GS5];
pub const A4_MINOR_SCALE: &[f32] = &[A4, B4, C5, D5, E5, F5, G5, A5];
pub const AS4_MINOR_SCALE: &[f32] = &[AS4, C5, CS5, DS5, F5, FS5, GS5, AS5];
pub const B4_MINOR_SCALE: &[f32] = &[B4, CS5, D5, E5, FS5, G5, A5, B5];

// Octave 5
pub const C5_MINOR_SCALE: &[f32] = &[C5, D5, DS5, F5, G5, GS5, AS5, C6];
pub const CS5_MINOR_SCALE: &[f32] = &[CS5, DS5, E5, FS5, GS5, A5, B5, CS6];
pub const D5_MINOR_SCALE: &[f32] = &[D5, E5, F5, G5, A5, AS5, C6, D6];
pub const DS5_MINOR_SCALE: &[f32] = &[DS5, F5, FS5, GS5, AS5, B5, CS6, DS6];
pub const E5_MINOR_SCALE: &[f32] = &[E5, FS5, G5, A5, B5, C6, D6, E6];
pub const F5_MINOR_SCALE: &[f32] = &[F5, G5, GS5, AS5, C6, CS6, DS6, F6];
pub const FS5_MINOR_SCALE: &[f32] = &[FS5, GS5, A5, B5, CS6, D6, E6, FS6];
pub const G5_MINOR_SCALE: &[f32] = &[G5, A5, AS5, C6, D6, DS6, F6, G6];
pub const GS5_MINOR_SCALE: &[f32] = &[GS5, AS5, B5, CS6, DS6, E6, FS6, GS6];
pub const A5_MINOR_SCALE: &[f32] = &[A5, B5, C6, D6, E6, F6, G6, A6];
pub const AS5_MINOR_SCALE: &[f32] = &[AS5, C6, CS6, DS6, F6, FS6, GS6, AS6];
pub const B5_MINOR_SCALE: &[f32] = &[B5, CS6, D6, E6, FS6, G6, A6, B6];

// Octave 6
pub const C6_MINOR_SCALE: &[f32] = &[C6, D6, DS6, F6, G6, GS6, AS6, C7];
pub const CS6_MINOR_SCALE: &[f32] = &[CS6, DS6, E6, FS6, GS6, A6, B6, CS7];
pub const D6_MINOR_SCALE: &[f32] = &[D6, E6, F6, G6, A6, AS6, C7, D7];
pub const DS6_MINOR_SCALE: &[f32] = &[DS6, F6, FS6, GS6, AS6, B6, CS7, DS7];
pub const E6_MINOR_SCALE: &[f32] = &[E6, FS6, G6, A6, B6, C7, D7, E7];
pub const F6_MINOR_SCALE: &[f32] = &[F6, G6, GS6, AS6, C7, CS7, DS7, F7];
pub const FS6_MINOR_SCALE: &[f32] = &[FS6, GS6, A6, B6, CS7, D7, E7, FS7];
pub const G6_MINOR_SCALE: &[f32] = &[G6, A6, AS6, C7, D7, DS7, F7, G7];
pub const GS6_MINOR_SCALE: &[f32] = &[GS6, AS6, B6, CS7, DS7, E7, FS7, GS7];
pub const A6_MINOR_SCALE: &[f32] = &[A6, B6, C7, D7, E7, F7, G7, A7];
pub const AS6_MINOR_SCALE: &[f32] = &[AS6, C7, CS7, DS7, F7, FS7, GS7, AS7];
pub const B6_MINOR_SCALE: &[f32] = &[B6, CS7, D7, E7, FS7, G7, A7, B7];

// ===== MAJOR PENTATONIC SCALES =====

// Octave 1
pub const C1_MAJOR_PENTATONIC_SCALE: &[f32] = &[C1, D1, E1, G1, A1, C2];
pub const CS1_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS1, DS1, F1, GS1, AS1, CS2];
pub const D1_MAJOR_PENTATONIC_SCALE: &[f32] = &[D1, E1, FS1, A1, B1, D2];
pub const DS1_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS1, F1, G1, AS1, C2, DS2];
pub const E1_MAJOR_PENTATONIC_SCALE: &[f32] = &[E1, FS1, GS1, B1, CS2, E2];
pub const F1_MAJOR_PENTATONIC_SCALE: &[f32] = &[F1, G1, A1, C2, D2, F2];
pub const FS1_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS1, GS1, AS1, CS2, DS2, FS2];
pub const G1_MAJOR_PENTATONIC_SCALE: &[f32] = &[G1, A1, B1, D2, E2, G2];
pub const GS1_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS1, AS1, C2, DS2, F2, GS2];
pub const A1_MAJOR_PENTATONIC_SCALE: &[f32] = &[A1, B1, CS2, E2, FS2, A2];
pub const AS1_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS1, C2, D2, F2, G2, AS2];
pub const B1_MAJOR_PENTATONIC_SCALE: &[f32] = &[B1, CS2, DS2, FS2, GS2, B2];

// Octave 2
pub const C2_MAJOR_PENTATONIC_SCALE: &[f32] = &[C2, D2, E2, G2, A2, C3];
pub const CS2_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS2, DS2, F2, GS2, AS2, CS3];
pub const D2_MAJOR_PENTATONIC_SCALE: &[f32] = &[D2, E2, FS2, A2, B2, D3];
pub const DS2_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS2, F2, G2, AS2, C3, DS3];
pub const E2_MAJOR_PENTATONIC_SCALE: &[f32] = &[E2, FS2, GS2, B2, CS3, E3];
pub const F2_MAJOR_PENTATONIC_SCALE: &[f32] = &[F2, G2, A2, C3, D3, F3];
pub const FS2_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS2, GS2, AS2, CS3, DS3, FS3];
pub const G2_MAJOR_PENTATONIC_SCALE: &[f32] = &[G2, A2, B2, D3, E3, G3];
pub const GS2_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS2, AS2, C3, DS3, F3, GS3];
pub const A2_MAJOR_PENTATONIC_SCALE: &[f32] = &[A2, B2, CS3, E3, FS3, A3];
pub const AS2_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS2, C3, D3, F3, G3, AS3];
pub const B2_MAJOR_PENTATONIC_SCALE: &[f32] = &[B2, CS3, DS3, FS3, GS3, B3];

// Octave 3
pub const C3_MAJOR_PENTATONIC_SCALE: &[f32] = &[C3, D3, E3, G3, A3, C4];
pub const CS3_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS3, DS3, F3, GS3, AS3, CS4];
pub const D3_MAJOR_PENTATONIC_SCALE: &[f32] = &[D3, E3, FS3, A3, B3, D4];
pub const DS3_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS3, F3, G3, AS3, C4, DS4];
pub const E3_MAJOR_PENTATONIC_SCALE: &[f32] = &[E3, FS3, GS3, B3, CS4, E4];
pub const F3_MAJOR_PENTATONIC_SCALE: &[f32] = &[F3, G3, A3, C4, D4, F4];
pub const FS3_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS3, GS3, AS3, CS4, DS4, FS4];
pub const G3_MAJOR_PENTATONIC_SCALE: &[f32] = &[G3, A3, B3, D4, E4, G4];
pub const GS3_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS3, AS3, C4, DS4, F4, GS4];
pub const A3_MAJOR_PENTATONIC_SCALE: &[f32] = &[A3, B3, CS4, E4, FS4, A4];
pub const AS3_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS3, C4, D4, F4, G4, AS4];
pub const B3_MAJOR_PENTATONIC_SCALE: &[f32] = &[B3, CS4, DS4, FS4, GS4, B4];

// Octave 4
pub const C4_MAJOR_PENTATONIC_SCALE: &[f32] = &[C4, D4, E4, G4, A4, C5];
pub const CS4_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS4, DS4, F4, GS4, AS4, CS5];
pub const D4_MAJOR_PENTATONIC_SCALE: &[f32] = &[D4, E4, FS4, A4, B4, D5];
pub const DS4_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS4, F4, G4, AS4, C5, DS5];
pub const E4_MAJOR_PENTATONIC_SCALE: &[f32] = &[E4, FS4, GS4, B4, CS5, E5];
pub const F4_MAJOR_PENTATONIC_SCALE: &[f32] = &[F4, G4, A4, C5, D5, F5];
pub const FS4_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS4, GS4, AS4, CS5, DS5, FS5];
pub const G4_MAJOR_PENTATONIC_SCALE: &[f32] = &[G4, A4, B4, D5, E5, G5];
pub const GS4_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS4, AS4, C5, DS5, F5, GS5];
pub const A4_MAJOR_PENTATONIC_SCALE: &[f32] = &[A4, B4, CS5, E5, FS5, A5];
pub const AS4_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS4, C5, D5, F5, G5, AS5];
pub const B4_MAJOR_PENTATONIC_SCALE: &[f32] = &[B4, CS5, DS5, FS5, GS5, B5];

// Octave 5
pub const C5_MAJOR_PENTATONIC_SCALE: &[f32] = &[C5, D5, E5, G5, A5, C6];
pub const CS5_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS5, DS5, F5, GS5, AS5, CS6];
pub const D5_MAJOR_PENTATONIC_SCALE: &[f32] = &[D5, E5, FS5, A5, B5, D6];
pub const DS5_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS5, F5, G5, AS5, C6, DS6];
pub const E5_MAJOR_PENTATONIC_SCALE: &[f32] = &[E5, FS5, GS5, B5, CS6, E6];
pub const F5_MAJOR_PENTATONIC_SCALE: &[f32] = &[F5, G5, A5, C6, D6, F6];
pub const FS5_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS5, GS5, AS5, CS6, DS6, FS6];
pub const G5_MAJOR_PENTATONIC_SCALE: &[f32] = &[G5, A5, B5, D6, E6, G6];
pub const GS5_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS5, AS5, C6, DS6, F6, GS6];
pub const A5_MAJOR_PENTATONIC_SCALE: &[f32] = &[A5, B5, CS6, E6, FS6, A6];
pub const AS5_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS5, C6, D6, F6, G6, AS6];
pub const B5_MAJOR_PENTATONIC_SCALE: &[f32] = &[B5, CS6, DS6, FS6, GS6, B6];

// Octave 6
pub const C6_MAJOR_PENTATONIC_SCALE: &[f32] = &[C6, D6, E6, G6, A6, C7];
pub const CS6_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS6, DS6, F6, GS6, AS6, CS7];
pub const D6_MAJOR_PENTATONIC_SCALE: &[f32] = &[D6, E6, FS6, A6, B6, D7];
pub const DS6_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS6, F6, G6, AS6, C7, DS7];
pub const E6_MAJOR_PENTATONIC_SCALE: &[f32] = &[E6, FS6, GS6, B6, CS7, E7];
pub const F6_MAJOR_PENTATONIC_SCALE: &[f32] = &[F6, G6, A6, C7, D7, F7];
pub const FS6_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS6, GS6, AS6, CS7, DS7, FS7];
pub const G6_MAJOR_PENTATONIC_SCALE: &[f32] = &[G6, A6, B6, D7, E7, G7];
pub const GS6_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS6, AS6, C7, DS7, F7, GS7];
pub const A6_MAJOR_PENTATONIC_SCALE: &[f32] = &[A6, B6, CS7, E7, FS7, A7];
pub const AS6_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS6, C7, D7, F7, G7, AS7];
pub const B6_MAJOR_PENTATONIC_SCALE: &[f32] = &[B6, CS7, DS7, FS7, GS7, B7];

// ===== MINOR PENTATONIC SCALES =====

// Octave 1
pub const C1_MINOR_PENTATONIC_SCALE: &[f32] = &[C1, DS1, F1, G1, AS1, C2];
pub const CS1_MINOR_PENTATONIC_SCALE: &[f32] = &[CS1, E1, FS1, GS1, B1, CS2];
pub const D1_MINOR_PENTATONIC_SCALE: &[f32] = &[D1, F1, G1, A1, C2, D2];
pub const DS1_MINOR_PENTATONIC_SCALE: &[f32] = &[DS1, FS1, GS1, AS1, CS2, DS2];
pub const E1_MINOR_PENTATONIC_SCALE: &[f32] = &[E1, G1, A1, B1, D2, E2];
pub const F1_MINOR_PENTATONIC_SCALE: &[f32] = &[F1, GS1, AS1, C2, DS2, F2];
pub const FS1_MINOR_PENTATONIC_SCALE: &[f32] = &[FS1, A1, B1, CS2, E2, FS2];
pub const G1_MINOR_PENTATONIC_SCALE: &[f32] = &[G1, AS1, C2, D2, F2, G2];
pub const GS1_MINOR_PENTATONIC_SCALE: &[f32] = &[GS1, B1, CS2, DS2, FS2, GS2];
pub const A1_MINOR_PENTATONIC_SCALE: &[f32] = &[A1, C2, D2, E2, G2, A2];
pub const AS1_MINOR_PENTATONIC_SCALE: &[f32] = &[AS1, CS2, DS2, F2, GS2, AS2];
pub const B1_MINOR_PENTATONIC_SCALE: &[f32] = &[B1, D2, E2, FS2, A2, B2];

// Octave 2
pub const C2_MINOR_PENTATONIC_SCALE: &[f32] = &[C2, DS2, F2, G2, AS2, C3];
pub const CS2_MINOR_PENTATONIC_SCALE: &[f32] = &[CS2, E2, FS2, GS2, B2, CS3];
pub const D2_MINOR_PENTATONIC_SCALE: &[f32] = &[D2, F2, G2, A2, C3, D3];
pub const DS2_MINOR_PENTATONIC_SCALE: &[f32] = &[DS2, FS2, GS2, AS2, CS3, DS3];
pub const E2_MINOR_PENTATONIC_SCALE: &[f32] = &[E2, G2, A2, B2, D3, E3];
pub const F2_MINOR_PENTATONIC_SCALE: &[f32] = &[F2, GS2, AS2, C3, DS3, F3];
pub const FS2_MINOR_PENTATONIC_SCALE: &[f32] = &[FS2, A2, B2, CS3, E3, FS3];
pub const G2_MINOR_PENTATONIC_SCALE: &[f32] = &[G2, AS2, C3, D3, F3, G3];
pub const GS2_MINOR_PENTATONIC_SCALE: &[f32] = &[GS2, B2, CS3, DS3, FS3, GS3];
pub const A2_MINOR_PENTATONIC_SCALE: &[f32] = &[A2, C3, D3, E3, G3, A3];
pub const AS2_MINOR_PENTATONIC_SCALE: &[f32] = &[AS2, CS3, DS3, F3, GS3, AS3];
pub const B2_MINOR_PENTATONIC_SCALE: &[f32] = &[B2, D3, E3, FS3, A3, B3];

// Octave 3
pub const C3_MINOR_PENTATONIC_SCALE: &[f32] = &[C3, DS3, F3, G3, AS3, C4];
pub const CS3_MINOR_PENTATONIC_SCALE: &[f32] = &[CS3, E3, FS3, GS3, B3, CS4];
pub const D3_MINOR_PENTATONIC_SCALE: &[f32] = &[D3, F3, G3, A3, C4, D4];
pub const DS3_MINOR_PENTATONIC_SCALE: &[f32] = &[DS3, FS3, GS3, AS3, CS4, DS4];
pub const E3_MINOR_PENTATONIC_SCALE: &[f32] = &[E3, G3, A3, B3, D4, E4];
pub const F3_MINOR_PENTATONIC_SCALE: &[f32] = &[F3, GS3, AS3, C4, DS4, F4];
pub const FS3_MINOR_PENTATONIC_SCALE: &[f32] = &[FS3, A3, B3, CS4, E4, FS4];
pub const G3_MINOR_PENTATONIC_SCALE: &[f32] = &[G3, AS3, C4, D4, F4, G4];
pub const GS3_MINOR_PENTATONIC_SCALE: &[f32] = &[GS3, B3, CS4, DS4, FS4, GS4];
pub const A3_MINOR_PENTATONIC_SCALE: &[f32] = &[A3, C4, D4, E4, G4, A4];
pub const AS3_MINOR_PENTATONIC_SCALE: &[f32] = &[AS3, CS4, DS4, F4, GS4, AS4];
pub const B3_MINOR_PENTATONIC_SCALE: &[f32] = &[B3, D4, E4, FS4, A4, B4];

// Octave 4
pub const C4_MINOR_PENTATONIC_SCALE: &[f32] = &[C4, DS4, F4, G4, AS4, C5];
pub const CS4_MINOR_PENTATONIC_SCALE: &[f32] = &[CS4, E4, FS4, GS4, B4, CS5];
pub const D4_MINOR_PENTATONIC_SCALE: &[f32] = &[D4, F4, G4, A4, C5, D5];
pub const DS4_MINOR_PENTATONIC_SCALE: &[f32] = &[DS4, FS4, GS4, AS4, CS5, DS5];
pub const E4_MINOR_PENTATONIC_SCALE: &[f32] = &[E4, G4, A4, B4, D5, E5];
pub const F4_MINOR_PENTATONIC_SCALE: &[f32] = &[F4, GS4, AS4, C5, DS5, F5];
pub const FS4_MINOR_PENTATONIC_SCALE: &[f32] = &[FS4, A4, B4, CS5, E5, FS5];
pub const G4_MINOR_PENTATONIC_SCALE: &[f32] = &[G4, AS4, C5, D5, F5, G5];
pub const GS4_MINOR_PENTATONIC_SCALE: &[f32] = &[GS4, B4, CS5, DS5, FS5, GS5];
pub const A4_MINOR_PENTATONIC_SCALE: &[f32] = &[A4, C5, D5, E5, G5, A5];
pub const AS4_MINOR_PENTATONIC_SCALE: &[f32] = &[AS4, CS5, DS5, F5, GS5, AS5];
pub const B4_MINOR_PENTATONIC_SCALE: &[f32] = &[B4, D5, E5, FS5, A5, B5];

// Octave 5
pub const C5_MINOR_PENTATONIC_SCALE: &[f32] = &[C5, DS5, F5, G5, AS5, C6];
pub const CS5_MINOR_PENTATONIC_SCALE: &[f32] = &[CS5, E5, FS5, GS5, B5, CS6];
pub const D5_MINOR_PENTATONIC_SCALE: &[f32] = &[D5, F5, G5, A5, C6, D6];
pub const DS5_MINOR_PENTATONIC_SCALE: &[f32] = &[DS5, FS5, GS5, AS5, CS6, DS6];
pub const E5_MINOR_PENTATONIC_SCALE: &[f32] = &[E5, G5, A5, B5, D6, E6];
pub const F5_MINOR_PENTATONIC_SCALE: &[f32] = &[F5, GS5, AS5, C6, DS6, F6];
pub const FS5_MINOR_PENTATONIC_SCALE: &[f32] = &[FS5, A5, B5, CS6, E6, FS6];
pub const G5_MINOR_PENTATONIC_SCALE: &[f32] = &[G5, AS5, C6, D6, F6, G6];
pub const GS5_MINOR_PENTATONIC_SCALE: &[f32] = &[GS5, B5, CS6, DS6, FS6, GS6];
pub const A5_MINOR_PENTATONIC_SCALE: &[f32] = &[A5, C6, D6, E6, G6, A6];
pub const AS5_MINOR_PENTATONIC_SCALE: &[f32] = &[AS5, CS6, DS6, F6, GS6, AS6];
pub const B5_MINOR_PENTATONIC_SCALE: &[f32] = &[B5, D6, E6, FS6, A6, B6];

// Octave 6
pub const C6_MINOR_PENTATONIC_SCALE: &[f32] = &[C6, DS6, F6, G6, AS6, C7];
pub const CS6_MINOR_PENTATONIC_SCALE: &[f32] = &[CS6, E6, FS6, GS6, B6, CS7];
pub const D6_MINOR_PENTATONIC_SCALE: &[f32] = &[D6, F6, G6, A6, C7, D7];
pub const DS6_MINOR_PENTATONIC_SCALE: &[f32] = &[DS6, FS6, GS6, AS6, CS7, DS7];
pub const E6_MINOR_PENTATONIC_SCALE: &[f32] = &[E6, G6, A6, B6, D7, E7];
pub const F6_MINOR_PENTATONIC_SCALE: &[f32] = &[F6, GS6, AS6, C7, DS7, F7];
pub const FS6_MINOR_PENTATONIC_SCALE: &[f32] = &[FS6, A6, B6, CS7, E7, FS7];
pub const G6_MINOR_PENTATONIC_SCALE: &[f32] = &[G6, AS6, C7, D7, F7, G7];
pub const GS6_MINOR_PENTATONIC_SCALE: &[f32] = &[GS6, B6, CS7, DS7, FS7, GS7];
pub const A6_MINOR_PENTATONIC_SCALE: &[f32] = &[A6, C7, D7, E7, G7, A7];
pub const AS6_MINOR_PENTATONIC_SCALE: &[f32] = &[AS6, CS7, DS7, F7, GS7, AS7];
pub const B6_MINOR_PENTATONIC_SCALE: &[f32] = &[B6, D7, E7, FS7, A7, B7];

// ===== BLUES SCALES =====

// Octave 1
pub const C1_BLUES_SCALE: &[f32] = &[C1, DS1, F1, FS1, G1, AS1, C2];
pub const CS1_BLUES_SCALE: &[f32] = &[CS1, E1, FS1, G1, GS1, B1, CS2];
pub const D1_BLUES_SCALE: &[f32] = &[D1, F1, G1, GS1, A1, C2, D2];
pub const DS1_BLUES_SCALE: &[f32] = &[DS1, FS1, GS1, A1, AS1, CS2, DS2];
pub const E1_BLUES_SCALE: &[f32] = &[E1, G1, A1, AS1, B1, D2, E2];
pub const F1_BLUES_SCALE: &[f32] = &[F1, GS1, AS1, B1, C2, DS2, F2];
pub const FS1_BLUES_SCALE: &[f32] = &[FS1, A1, B1, C2, CS2, E2, FS2];
pub const G1_BLUES_SCALE: &[f32] = &[G1, AS1, C2, CS2, D2, F2, G2];
pub const GS1_BLUES_SCALE: &[f32] = &[GS1, B1, CS2, D2, DS2, FS2, GS2];
pub const A1_BLUES_SCALE: &[f32] = &[A1, C2, D2, DS2, E2, G2, A2];
pub const AS1_BLUES_SCALE: &[f32] = &[AS1, CS2, DS2, E2, F2, GS2, AS2];
pub const B1_BLUES_SCALE: &[f32] = &[B1, D2, E2, F2, FS2, A2, B2];

// Octave 2
pub const C2_BLUES_SCALE: &[f32] = &[C2, DS2, F2, FS2, G2, AS2, C3];
pub const CS2_BLUES_SCALE: &[f32] = &[CS2, E2, FS2, G2, GS2, B2, CS3];
pub const D2_BLUES_SCALE: &[f32] = &[D2, F2, G2, GS2, A2, C3, D3];
pub const DS2_BLUES_SCALE: &[f32] = &[DS2, FS2, GS2, A2, AS2, CS3, DS3];
pub const E2_BLUES_SCALE: &[f32] = &[E2, G2, A2, AS2, B2, D3, E3];
pub const F2_BLUES_SCALE: &[f32] = &[F2, GS2, AS2, B2, C3, DS3, F3];
pub const FS2_BLUES_SCALE: &[f32] = &[FS2, A2, B2, C3, CS3, E3, FS3];
pub const G2_BLUES_SCALE: &[f32] = &[G2, AS2, C3, CS3, D3, F3, G3];
pub const GS2_BLUES_SCALE: &[f32] = &[GS2, B2, CS3, D3, DS3, FS3, GS3];
pub const A2_BLUES_SCALE: &[f32] = &[A2, C3, D3, DS3, E3, G3, A3];
pub const AS2_BLUES_SCALE: &[f32] = &[AS2, CS3, DS3, E3, F3, GS3, AS3];
pub const B2_BLUES_SCALE: &[f32] = &[B2, D3, E3, F3, FS3, A3, B3];

// Octave 3
pub const C3_BLUES_SCALE: &[f32] = &[C3, DS3, F3, FS3, G3, AS3, C4];
pub const CS3_BLUES_SCALE: &[f32] = &[CS3, E3, FS3, G3, GS3, B3, CS4];
pub const D3_BLUES_SCALE: &[f32] = &[D3, F3, G3, GS3, A3, C4, D4];
pub const DS3_BLUES_SCALE: &[f32] = &[DS3, FS3, GS3, A3, AS3, CS4, DS4];
pub const E3_BLUES_SCALE: &[f32] = &[E3, G3, A3, AS3, B3, D4, E4];
pub const F3_BLUES_SCALE: &[f32] = &[F3, GS3, AS3, B3, C4, DS4, F4];
pub const FS3_BLUES_SCALE: &[f32] = &[FS3, A3, B3, C4, CS4, E4, FS4];
pub const G3_BLUES_SCALE: &[f32] = &[G3, AS3, C4, CS4, D4, F4, G4];
pub const GS3_BLUES_SCALE: &[f32] = &[GS3, B3, CS4, D4, DS4, FS4, GS4];
pub const A3_BLUES_SCALE: &[f32] = &[A3, C4, D4, DS4, E4, G4, A4];
pub const AS3_BLUES_SCALE: &[f32] = &[AS3, CS4, DS4, E4, F4, GS4, AS4];
pub const B3_BLUES_SCALE: &[f32] = &[B3, D4, E4, F4, FS4, A4, B4];

// Octave 4
pub const C4_BLUES_SCALE: &[f32] = &[C4, DS4, F4, FS4, G4, AS4, C5];
pub const CS4_BLUES_SCALE: &[f32] = &[CS4, E4, FS4, G4, GS4, B4, CS5];
pub const D4_BLUES_SCALE: &[f32] = &[D4, F4, G4, GS4, A4, C5, D5];
pub const DS4_BLUES_SCALE: &[f32] = &[DS4, FS4, GS4, A4, AS4, CS5, DS5];
pub const E4_BLUES_SCALE: &[f32] = &[E4, G4, A4, AS4, B4, D5, E5];
pub const F4_BLUES_SCALE: &[f32] = &[F4, GS4, AS4, B4, C5, DS5, F5];
pub const FS4_BLUES_SCALE: &[f32] = &[FS4, A4, B4, C5, CS5, E5, FS5];
pub const G4_BLUES_SCALE: &[f32] = &[G4, AS4, C5, CS5, D5, F5, G5];
pub const GS4_BLUES_SCALE: &[f32] = &[GS4, B4, CS5, D5, DS5, FS5, GS5];
pub const A4_BLUES_SCALE: &[f32] = &[A4, C5, D5, DS5, E5, G5, A5];
pub const AS4_BLUES_SCALE: &[f32] = &[AS4, CS5, DS5, E5, F5, GS5, AS5];
pub const B4_BLUES_SCALE: &[f32] = &[B4, D5, E5, F5, FS5, A5, B5];

// Octave 5
pub const C5_BLUES_SCALE: &[f32] = &[C5, DS5, F5, FS5, G5, AS5, C6];
pub const CS5_BLUES_SCALE: &[f32] = &[CS5, E5, FS5, G5, GS5, B5, CS6];
pub const D5_BLUES_SCALE: &[f32] = &[D5, F5, G5, GS5, A5, C6, D6];
pub const DS5_BLUES_SCALE: &[f32] = &[DS5, FS5, GS5, A5, AS5, CS6, DS6];
pub const E5_BLUES_SCALE: &[f32] = &[E5, G5, A5, AS5, B5, D6, E6];
pub const F5_BLUES_SCALE: &[f32] = &[F5, GS5, AS5, B5, C6, DS6, F6];
pub const FS5_BLUES_SCALE: &[f32] = &[FS5, A5, B5, C6, CS6, E6, FS6];
pub const G5_BLUES_SCALE: &[f32] = &[G5, AS5, C6, CS6, D6, F6, G6];
pub const GS5_BLUES_SCALE: &[f32] = &[GS5, B5, CS6, D6, DS6, FS6, GS6];
pub const A5_BLUES_SCALE: &[f32] = &[A5, C6, D6, DS6, E6, G6, A6];
pub const AS5_BLUES_SCALE: &[f32] = &[AS5, CS6, DS6, E6, F6, GS6, AS6];
pub const B5_BLUES_SCALE: &[f32] = &[B5, D6, E6, F6, FS6, A6, B6];

// Octave 6
pub const C6_BLUES_SCALE: &[f32] = &[C6, DS6, F6, FS6, G6, AS6, C7];
pub const CS6_BLUES_SCALE: &[f32] = &[CS6, E6, FS6, G6, GS6, B6, CS7];
pub const D6_BLUES_SCALE: &[f32] = &[D6, F6, G6, GS6, A6, C7, D7];
pub const DS6_BLUES_SCALE: &[f32] = &[DS6, FS6, GS6, A6, AS6, CS7, DS7];
pub const E6_BLUES_SCALE: &[f32] = &[E6, G6, A6, AS6, B6, D7, E7];
pub const F6_BLUES_SCALE: &[f32] = &[F6, GS6, AS6, B6, C7, DS7, F7];
pub const FS6_BLUES_SCALE: &[f32] = &[FS6, A6, B6, C7, CS7, E7, FS7];
pub const G6_BLUES_SCALE: &[f32] = &[G6, AS6, C7, CS7, D7, F7, G7];
pub const GS6_BLUES_SCALE: &[f32] = &[GS6, B6, CS7, D7, DS7, FS7, GS7];
pub const A6_BLUES_SCALE: &[f32] = &[A6, C7, D7, DS7, E7, G7, A7];
pub const AS6_BLUES_SCALE: &[f32] = &[AS6, CS7, DS7, E7, F7, GS7, AS7];
pub const B6_BLUES_SCALE: &[f32] = &[B6, D7, E7, F7, FS7, A7, B7];

// ===== MELODIC MINOR SCALES =====
// Pattern: 1, 2, ♭3, 4, 5, 6, 7, octave

// Octave 1
pub const C1_MELODIC_MINOR_SCALE: &[f32] = &[C1, D1, DS1, F1, G1, A1, B1, C2];
pub const CS1_MELODIC_MINOR_SCALE: &[f32] = &[CS1, DS1, E1, FS1, GS1, AS1, C2, CS2];
pub const D1_MELODIC_MINOR_SCALE: &[f32] = &[D1, E1, F1, G1, A1, B1, CS2, D2];
pub const DS1_MELODIC_MINOR_SCALE: &[f32] = &[DS1, F1, FS1, GS1, AS1, C2, D2, DS2];
pub const E1_MELODIC_MINOR_SCALE: &[f32] = &[E1, FS1, G1, A1, B1, CS2, DS2, E2];
pub const F1_MELODIC_MINOR_SCALE: &[f32] = &[F1, G1, GS1, AS1, C2, D2, E2, F2];
pub const FS1_MELODIC_MINOR_SCALE: &[f32] = &[FS1, GS1, A1, B1, CS2, DS2, F2, FS2];
pub const G1_MELODIC_MINOR_SCALE: &[f32] = &[G1, A1, AS1, C2, D2, E2, FS2, G2];
pub const GS1_MELODIC_MINOR_SCALE: &[f32] = &[GS1, AS1, B1, CS2, DS2, F2, G2, GS2];
pub const A1_MELODIC_MINOR_SCALE: &[f32] = &[A1, B1, C2, D2, E2, FS2, GS2, A2];
pub const AS1_MELODIC_MINOR_SCALE: &[f32] = &[AS1, C2, CS2, DS2, F2, G2, A2, AS2];
pub const B1_MELODIC_MINOR_SCALE: &[f32] = &[B1, CS2, D2, E2, FS2, GS2, AS2, B2];

// Octave 2
pub const C2_MELODIC_MINOR_SCALE: &[f32] = &[C2, D2, DS2, F2, G2, A2, B2, C3];
pub const CS2_MELODIC_MINOR_SCALE: &[f32] = &[CS2, DS2, E2, FS2, GS2, AS2, C3, CS3];
pub const D2_MELODIC_MINOR_SCALE: &[f32] = &[D2, E2, F2, G2, A2, B2, CS3, D3];
pub const DS2_MELODIC_MINOR_SCALE: &[f32] = &[DS2, F2, FS2, GS2, AS2, C3, D3, DS3];
pub const E2_MELODIC_MINOR_SCALE: &[f32] = &[E2, FS2, G2, A2, B2, CS3, DS3, E3];
pub const F2_MELODIC_MINOR_SCALE: &[f32] = &[F2, G2, GS2, AS2, C3, D3, E3, F3];
pub const FS2_MELODIC_MINOR_SCALE: &[f32] = &[FS2, GS2, A2, B2, CS3, DS3, F3, FS3];
pub const G2_MELODIC_MINOR_SCALE: &[f32] = &[G2, A2, AS2, C3, D3, E3, FS3, G3];
pub const GS2_MELODIC_MINOR_SCALE: &[f32] = &[GS2, AS2, B2, CS3, DS3, F3, G3, GS3];
pub const A2_MELODIC_MINOR_SCALE: &[f32] = &[A2, B2, C3, D3, E3, FS3, GS3, A3];
pub const AS2_MELODIC_MINOR_SCALE: &[f32] = &[AS2, C3, CS3, DS3, F3, G3, A3, AS3];
pub const B2_MELODIC_MINOR_SCALE: &[f32] = &[B2, CS3, D3, E3, FS3, GS3, AS3, B3];

// Octave 3
pub const C3_MELODIC_MINOR_SCALE: &[f32] = &[C3, D3, DS3, F3, G3, A3, B3, C4];
pub const CS3_MELODIC_MINOR_SCALE: &[f32] = &[CS3, DS3, E3, FS3, GS3, AS3, C4, CS4];
pub const D3_MELODIC_MINOR_SCALE: &[f32] = &[D3, E3, F3, G3, A3, B3, CS4, D4];
pub const DS3_MELODIC_MINOR_SCALE: &[f32] = &[DS3, F3, FS3, GS3, AS3, C4, D4, DS4];
pub const E3_MELODIC_MINOR_SCALE: &[f32] = &[E3, FS3, G3, A3, B3, CS4, DS4, E4];
pub const F3_MELODIC_MINOR_SCALE: &[f32] = &[F3, G3, GS3, AS3, C4, D4, E4, F4];
pub const FS3_MELODIC_MINOR_SCALE: &[f32] = &[FS3, GS3, A3, B3, CS4, DS4, F4, FS4];
pub const G3_MELODIC_MINOR_SCALE: &[f32] = &[G3, A3, AS3, C4, D4, E4, FS4, G4];
pub const GS3_MELODIC_MINOR_SCALE: &[f32] = &[GS3, AS3, B3, CS4, DS4, F4, G4, GS4];
pub const A3_MELODIC_MINOR_SCALE: &[f32] = &[A3, B3, C4, D4, E4, FS4, GS4, A4];
pub const AS3_MELODIC_MINOR_SCALE: &[f32] = &[AS3, C4, CS4, DS4, F4, G4, A4, AS4];
pub const B3_MELODIC_MINOR_SCALE: &[f32] = &[B3, CS4, D4, E4, FS4, GS4, AS4, B4];

// Octave 4
pub const C4_MELODIC_MINOR_SCALE: &[f32] = &[C4, D4, DS4, F4, G4, A4, B4, C5];
pub const CS4_MELODIC_MINOR_SCALE: &[f32] = &[CS4, DS4, E4, FS4, GS4, AS4, C5, CS5];
pub const D4_MELODIC_MINOR_SCALE: &[f32] = &[D4, E4, F4, G4, A4, B4, CS5, D5];
pub const DS4_MELODIC_MINOR_SCALE: &[f32] = &[DS4, F4, FS4, GS4, AS4, C5, D5, DS5];
pub const E4_MELODIC_MINOR_SCALE: &[f32] = &[E4, FS4, G4, A4, B4, CS5, DS5, E5];
pub const F4_MELODIC_MINOR_SCALE: &[f32] = &[F4, G4, GS4, AS4, C5, D5, E5, F5];
pub const FS4_MELODIC_MINOR_SCALE: &[f32] = &[FS4, GS4, A4, B4, CS5, DS5, F5, FS5];
pub const G4_MELODIC_MINOR_SCALE: &[f32] = &[G4, A4, AS4, C5, D5, E5, FS5, G5];
pub const GS4_MELODIC_MINOR_SCALE: &[f32] = &[GS4, AS4, B4, CS5, DS5, F5, G5, GS5];
pub const A4_MELODIC_MINOR_SCALE: &[f32] = &[A4, B4, C5, D5, E5, FS5, GS5, A5];
pub const AS4_MELODIC_MINOR_SCALE: &[f32] = &[AS4, C5, CS5, DS5, F5, G5, A5, AS5];
pub const B4_MELODIC_MINOR_SCALE: &[f32] = &[B4, CS5, D5, E5, FS5, GS5, AS5, B5];

// Octave 5
pub const C5_MELODIC_MINOR_SCALE: &[f32] = &[C5, D5, DS5, F5, G5, A5, B5, C6];
pub const CS5_MELODIC_MINOR_SCALE: &[f32] = &[CS5, DS5, E5, FS5, GS5, AS5, C6, CS6];
pub const D5_MELODIC_MINOR_SCALE: &[f32] = &[D5, E5, F5, G5, A5, B5, CS6, D6];
pub const DS5_MELODIC_MINOR_SCALE: &[f32] = &[DS5, F5, FS5, GS5, AS5, C6, D6, DS6];
pub const E5_MELODIC_MINOR_SCALE: &[f32] = &[E5, FS5, G5, A5, B5, CS6, DS6, E6];
pub const F5_MELODIC_MINOR_SCALE: &[f32] = &[F5, G5, GS5, AS5, C6, D6, E6, F6];
pub const FS5_MELODIC_MINOR_SCALE: &[f32] = &[FS5, GS5, A5, B5, CS6, DS6, F6, FS6];
pub const G5_MELODIC_MINOR_SCALE: &[f32] = &[G5, A5, AS5, C6, D6, E6, FS6, G6];
pub const GS5_MELODIC_MINOR_SCALE: &[f32] = &[GS5, AS5, B5, CS6, DS6, F6, G6, GS6];
pub const A5_MELODIC_MINOR_SCALE: &[f32] = &[A5, B5, C6, D6, E6, FS6, GS6, A6];
pub const AS5_MELODIC_MINOR_SCALE: &[f32] = &[AS5, C6, CS6, DS6, F6, G6, A6, AS6];
pub const B5_MELODIC_MINOR_SCALE: &[f32] = &[B5, CS6, D6, E6, FS6, GS6, AS6, B6];

// Octave 6
pub const C6_MELODIC_MINOR_SCALE: &[f32] = &[C6, D6, DS6, F6, G6, A6, B6, C7];
pub const CS6_MELODIC_MINOR_SCALE: &[f32] = &[CS6, DS6, E6, FS6, GS6, AS6, C7, CS7];
pub const D6_MELODIC_MINOR_SCALE: &[f32] = &[D6, E6, F6, G6, A6, B6, CS7, D7];
pub const DS6_MELODIC_MINOR_SCALE: &[f32] = &[DS6, F6, FS6, GS6, AS6, C7, D7, DS7];
pub const E6_MELODIC_MINOR_SCALE: &[f32] = &[E6, FS6, G6, A6, B6, CS7, DS7, E7];
pub const F6_MELODIC_MINOR_SCALE: &[f32] = &[F6, G6, GS6, AS6, C7, D7, E7, F7];
pub const FS6_MELODIC_MINOR_SCALE: &[f32] = &[FS6, GS6, A6, B6, CS7, DS7, F7, FS7];
pub const G6_MELODIC_MINOR_SCALE: &[f32] = &[G6, A6, AS6, C7, D7, E7, FS7, G7];
pub const GS6_MELODIC_MINOR_SCALE: &[f32] = &[GS6, AS6, B6, CS7, DS7, F7, G7, GS7];
pub const A6_MELODIC_MINOR_SCALE: &[f32] = &[A6, B6, C7, D7, E7, FS7, GS7, A7];
pub const AS6_MELODIC_MINOR_SCALE: &[f32] = &[AS6, C7, CS7, DS7, F7, G7, A7, AS7];
pub const B6_MELODIC_MINOR_SCALE: &[f32] = &[B6, CS7, D7, E7, FS7, GS7, AS7, B7];

// ===== HARMONIC MINOR SCALES =====
// Pattern: 1, 2, ♭3, 4, 5, ♭6, 7, octave

// Octave 1
pub const C1_HARMONIC_MINOR_SCALE: &[f32] = &[C1, D1, DS1, F1, G1, GS1, B1, C2];
pub const CS1_HARMONIC_MINOR_SCALE: &[f32] = &[CS1, DS1, E1, FS1, GS1, A1, C2, CS2];
pub const D1_HARMONIC_MINOR_SCALE: &[f32] = &[D1, E1, F1, G1, A1, AS1, CS2, D2];
pub const DS1_HARMONIC_MINOR_SCALE: &[f32] = &[DS1, F1, FS1, GS1, AS1, B1, D2, DS2];
pub const E1_HARMONIC_MINOR_SCALE: &[f32] = &[E1, FS1, G1, A1, B1, C2, DS2, E2];
pub const F1_HARMONIC_MINOR_SCALE: &[f32] = &[F1, G1, GS1, AS1, C2, CS2, E2, F2];
pub const FS1_HARMONIC_MINOR_SCALE: &[f32] = &[FS1, GS1, A1, B1, CS2, D2, F2, FS2];
pub const G1_HARMONIC_MINOR_SCALE: &[f32] = &[G1, A1, AS1, C2, D2, DS2, FS2, G2];
pub const GS1_HARMONIC_MINOR_SCALE: &[f32] = &[GS1, AS1, B1, CS2, DS2, E2, G2, GS2];
pub const A1_HARMONIC_MINOR_SCALE: &[f32] = &[A1, B1, C2, D2, E2, F2, GS2, A2];
pub const AS1_HARMONIC_MINOR_SCALE: &[f32] = &[AS1, C2, CS2, DS2, F2, FS2, A2, AS2];
pub const B1_HARMONIC_MINOR_SCALE: &[f32] = &[B1, CS2, D2, E2, FS2, G2, AS2, B2];

// Octave 2
pub const C2_HARMONIC_MINOR_SCALE: &[f32] = &[C2, D2, DS2, F2, G2, GS2, B2, C3];
pub const CS2_HARMONIC_MINOR_SCALE: &[f32] = &[CS2, DS2, E2, FS2, GS2, A2, C3, CS3];
pub const D2_HARMONIC_MINOR_SCALE: &[f32] = &[D2, E2, F2, G2, A2, AS2, CS3, D3];
pub const DS2_HARMONIC_MINOR_SCALE: &[f32] = &[DS2, F2, FS2, GS2, AS2, B2, D3, DS3];
pub const E2_HARMONIC_MINOR_SCALE: &[f32] = &[E2, FS2, G2, A2, B2, C3, DS3, E3];
pub const F2_HARMONIC_MINOR_SCALE: &[f32] = &[F2, G2, GS2, AS2, C3, CS3, E3, F3];
pub const FS2_HARMONIC_MINOR_SCALE: &[f32] = &[FS2, GS2, A2, B2, CS3, D3, F3, FS3];
pub const G2_HARMONIC_MINOR_SCALE: &[f32] = &[G2, A2, AS2, C3, D3, DS3, FS3, G3];
pub const GS2_HARMONIC_MINOR_SCALE: &[f32] = &[GS2, AS2, B2, CS3, DS3, E3, G3, GS3];
pub const A2_HARMONIC_MINOR_SCALE: &[f32] = &[A2, B2, C3, D3, E3, F3, GS3, A3];
pub const AS2_HARMONIC_MINOR_SCALE: &[f32] = &[AS2, C3, CS3, DS3, F3, FS3, A3, AS3];
pub const B2_HARMONIC_MINOR_SCALE: &[f32] = &[B2, CS3, D3, E3, FS3, G3, AS3, B3];

// Octave 3
pub const C3_HARMONIC_MINOR_SCALE: &[f32] = &[C3, D3, DS3, F3, G3, GS3, B3, C4];
pub const CS3_HARMONIC_MINOR_SCALE: &[f32] = &[CS3, DS3, E3, FS3, GS3, A3, C4, CS4];
pub const D3_HARMONIC_MINOR_SCALE: &[f32] = &[D3, E3, F3, G3, A3, AS3, CS4, D4];
pub const DS3_HARMONIC_MINOR_SCALE: &[f32] = &[DS3, F3, FS3, GS3, AS3, B3, D4, DS4];
pub const E3_HARMONIC_MINOR_SCALE: &[f32] = &[E3, FS3, G3, A3, B3, C4, DS4, E4];
pub const F3_HARMONIC_MINOR_SCALE: &[f32] = &[F3, G3, GS3, AS3, C4, CS4, E4, F4];
pub const FS3_HARMONIC_MINOR_SCALE: &[f32] = &[FS3, GS3, A3, B3, CS4, D4, F4, FS4];
pub const G3_HARMONIC_MINOR_SCALE: &[f32] = &[G3, A3, AS3, C4, D4, DS4, FS4, G4];
pub const GS3_HARMONIC_MINOR_SCALE: &[f32] = &[GS3, AS3, B3, CS4, DS4, E4, G4, GS4];
pub const A3_HARMONIC_MINOR_SCALE: &[f32] = &[A3, B3, C4, D4, E4, F4, GS4, A4];
pub const AS3_HARMONIC_MINOR_SCALE: &[f32] = &[AS3, C4, CS4, DS4, F4, FS4, A4, AS4];
pub const B3_HARMONIC_MINOR_SCALE: &[f32] = &[B3, CS4, D4, E4, FS4, G4, AS4, B4];

// Octave 4
pub const C4_HARMONIC_MINOR_SCALE: &[f32] = &[C4, D4, DS4, F4, G4, GS4, B4, C5];
pub const CS4_HARMONIC_MINOR_SCALE: &[f32] = &[CS4, DS4, E4, FS4, GS4, A4, C5, CS5];
pub const D4_HARMONIC_MINOR_SCALE: &[f32] = &[D4, E4, F4, G4, A4, AS4, CS5, D5];
pub const DS4_HARMONIC_MINOR_SCALE: &[f32] = &[DS4, F4, FS4, GS4, AS4, B4, D5, DS5];
pub const E4_HARMONIC_MINOR_SCALE: &[f32] = &[E4, FS4, G4, A4, B4, C5, DS5, E5];
pub const F4_HARMONIC_MINOR_SCALE: &[f32] = &[F4, G4, GS4, AS4, C5, CS5, E5, F5];
pub const FS4_HARMONIC_MINOR_SCALE: &[f32] = &[FS4, GS4, A4, B4, CS5, D5, F5, FS5];
pub const G4_HARMONIC_MINOR_SCALE: &[f32] = &[G4, A4, AS4, C5, D5, DS5, FS5, G5];
pub const GS4_HARMONIC_MINOR_SCALE: &[f32] = &[GS4, AS4, B4, CS5, DS5, E5, G5, GS5];
pub const A4_HARMONIC_MINOR_SCALE: &[f32] = &[A4, B4, C5, D5, E5, F5, GS5, A5];
pub const AS4_HARMONIC_MINOR_SCALE: &[f32] = &[AS4, C5, CS5, DS5, F5, FS5, A5, AS5];
pub const B4_HARMONIC_MINOR_SCALE: &[f32] = &[B4, CS5, D5, E5, FS5, G5, AS5, B5];

// Octave 5
pub const C5_HARMONIC_MINOR_SCALE: &[f32] = &[C5, D5, DS5, F5, G5, GS5, B5, C6];
pub const CS5_HARMONIC_MINOR_SCALE: &[f32] = &[CS5, DS5, E5, FS5, GS5, A5, C6, CS6];
pub const D5_HARMONIC_MINOR_SCALE: &[f32] = &[D5, E5, F5, G5, A5, AS5, CS6, D6];
pub const DS5_HARMONIC_MINOR_SCALE: &[f32] = &[DS5, F5, FS5, GS5, AS5, B5, D6, DS6];
pub const E5_HARMONIC_MINOR_SCALE: &[f32] = &[E5, FS5, G5, A5, B5, C6, DS6, E6];
pub const F5_HARMONIC_MINOR_SCALE: &[f32] = &[F5, G5, GS5, AS5, C6, CS6, E6, F6];
pub const FS5_HARMONIC_MINOR_SCALE: &[f32] = &[FS5, GS5, A5, B5, CS6, D6, F6, FS6];
pub const G5_HARMONIC_MINOR_SCALE: &[f32] = &[G5, A5, AS5, C6, D6, DS6, FS6, G6];
pub const GS5_HARMONIC_MINOR_SCALE: &[f32] = &[GS5, AS5, B5, CS6, DS6, E6, G6, GS6];
pub const A5_HARMONIC_MINOR_SCALE: &[f32] = &[A5, B5, C6, D6, E6, F6, GS6, A6];
pub const AS5_HARMONIC_MINOR_SCALE: &[f32] = &[AS5, C6, CS6, DS6, F6, FS6, A6, AS6];
pub const B5_HARMONIC_MINOR_SCALE: &[f32] = &[B5, CS6, D6, E6, FS6, G6, AS6, B6];

// Octave 6
pub const C6_HARMONIC_MINOR_SCALE: &[f32] = &[C6, D6, DS6, F6, G6, GS6, B6, C7];
pub const CS6_HARMONIC_MINOR_SCALE: &[f32] = &[CS6, DS6, E6, FS6, GS6, A6, C7, CS7];
pub const D6_HARMONIC_MINOR_SCALE: &[f32] = &[D6, E6, F6, G6, A6, AS6, CS7, D7];
pub const DS6_HARMONIC_MINOR_SCALE: &[f32] = &[DS6, F6, FS6, GS6, AS6, B6, D7, DS7];
pub const E6_HARMONIC_MINOR_SCALE: &[f32] = &[E6, FS6, G6, A6, B6, C7, DS7, E7];
pub const F6_HARMONIC_MINOR_SCALE: &[f32] = &[F6, G6, GS6, AS6, C7, CS7, E7, F7];
pub const FS6_HARMONIC_MINOR_SCALE: &[f32] = &[FS6, GS6, A6, B6, CS7, D7, F7, FS7];
pub const G6_HARMONIC_MINOR_SCALE: &[f32] = &[G6, A6, AS6, C7, D7, DS7, FS7, G7];
pub const GS6_HARMONIC_MINOR_SCALE: &[f32] = &[GS6, AS6, B6, CS7, DS7, E7, G7, GS7];
pub const A6_HARMONIC_MINOR_SCALE: &[f32] = &[A6, B6, C7, D7, E7, F7, GS7, A7];
pub const AS6_HARMONIC_MINOR_SCALE: &[f32] = &[AS6, C7, CS7, DS7, F7, FS7, A7, AS7];
pub const B6_HARMONIC_MINOR_SCALE: &[f32] = &[B6, CS7, D7, E7, FS7, G7, AS7, B7];

// ===== CHROMATIC SCALES =====

pub const C1_CHROMATIC_SCALE: &[f32] = &[C1, CS1, D1, DS1, E1, F1, FS1, G1, GS1, A1, AS1, B1, C2];
pub const C2_CHROMATIC_SCALE: &[f32] = &[C2, CS2, D2, DS2, E2, F2, FS2, G2, GS2, A2, AS2, B2, C3];
pub const C3_CHROMATIC_SCALE: &[f32] = &[C3, CS3, D3, DS3, E3, F3, FS3, G3, GS3, A3, AS3, B3, C4];
pub const C4_CHROMATIC_SCALE: &[f32] = &[C4, CS4, D4, DS4, E4, F4, FS4, G4, GS4, A4, AS4, B4, C5];
pub const C5_CHROMATIC_SCALE: &[f32] = &[C5, CS5, D5, DS5, E5, F5, FS5, G5, GS5, A5, AS5, B5, C6];
pub const C6_CHROMATIC_SCALE: &[f32] = &[C6, CS6, D6, DS6, E6, F6, FS6, G6, GS6, A6, AS6, B6, C7];
// ===== MAJOR_SCALE =====
// Octave -1
pub const C_1_MAJOR_SCALE: &[f32] = &[C_1, D_1, E_1, F_1, G_1, A_1, B_1, C0];
pub const CS_1_MAJOR_SCALE: &[f32] = &[CS_1, DS_1, F_1, FS_1, GS_1, AS_1, C0, CS0];
pub const D_1_MAJOR_SCALE: &[f32] = &[D_1, E_1, FS_1, G_1, A_1, B_1, CS0, D0];
pub const DS_1_MAJOR_SCALE: &[f32] = &[DS_1, F_1, G_1, GS_1, AS_1, C0, D0, DS0];
pub const E_1_MAJOR_SCALE: &[f32] = &[E_1, FS_1, GS_1, A_1, B_1, CS0, DS0, E0];
pub const F_1_MAJOR_SCALE: &[f32] = &[F_1, G_1, A_1, AS_1, C0, D0, E0, F0];
pub const FS_1_MAJOR_SCALE: &[f32] = &[FS_1, GS_1, AS_1, B_1, CS0, DS0, F0, FS0];
pub const G_1_MAJOR_SCALE: &[f32] = &[G_1, A_1, B_1, C0, D0, E0, FS0, G0];
pub const GS_1_MAJOR_SCALE: &[f32] = &[GS_1, AS_1, C0, CS0, DS0, F0, G0, GS0];
pub const A_1_MAJOR_SCALE: &[f32] = &[A_1, B_1, CS0, D0, E0, FS0, GS0, A0];
pub const AS_1_MAJOR_SCALE: &[f32] = &[AS_1, C0, D0, DS0, F0, G0, A0, AS0];
pub const B_1_MAJOR_SCALE: &[f32] = &[B_1, CS0, DS0, E0, FS0, GS0, AS0, B0];

// Octave 0
pub const C0_MAJOR_SCALE: &[f32] = &[C0, D0, E0, F0, G0, A0, B0, C1];
pub const CS0_MAJOR_SCALE: &[f32] = &[CS0, DS0, F0, FS0, GS0, AS0, C1, CS1];
pub const D0_MAJOR_SCALE: &[f32] = &[D0, E0, FS0, G0, A0, B0, CS1, D1];
pub const DS0_MAJOR_SCALE: &[f32] = &[DS0, F0, G0, GS0, AS0, C1, D1, DS1];
pub const E0_MAJOR_SCALE: &[f32] = &[E0, FS0, GS0, A0, B0, CS1, DS1, E1];
pub const F0_MAJOR_SCALE: &[f32] = &[F0, G0, A0, AS0, C1, D1, E1, F1];
pub const FS0_MAJOR_SCALE: &[f32] = &[FS0, GS0, AS0, B0, CS1, DS1, F1, FS1];
pub const G0_MAJOR_SCALE: &[f32] = &[G0, A0, B0, C1, D1, E1, FS1, G1];
pub const GS0_MAJOR_SCALE: &[f32] = &[GS0, AS0, C1, CS1, DS1, F1, G1, GS1];
pub const A0_MAJOR_SCALE: &[f32] = &[A0, B0, CS1, D1, E1, FS1, GS1, A1];
pub const AS0_MAJOR_SCALE: &[f32] = &[AS0, C1, D1, DS1, F1, G1, A1, AS1];
pub const B0_MAJOR_SCALE: &[f32] = &[B0, CS1, DS1, E1, FS1, GS1, AS1, B1];

// Octave 7
pub const C7_MAJOR_SCALE: &[f32] = &[C7, D7, E7, F7, G7, A7, B7, C8];
pub const CS7_MAJOR_SCALE: &[f32] = &[CS7, DS7, F7, FS7, GS7, AS7, C8, CS8];
pub const D7_MAJOR_SCALE: &[f32] = &[D7, E7, FS7, G7, A7, B7, CS8, D8];
pub const DS7_MAJOR_SCALE: &[f32] = &[DS7, F7, G7, GS7, AS7, C8, D8, DS8];
pub const E7_MAJOR_SCALE: &[f32] = &[E7, FS7, GS7, A7, B7, CS8, DS8, E8];
pub const F7_MAJOR_SCALE: &[f32] = &[F7, G7, A7, AS7, C8, D8, E8, F8];
pub const FS7_MAJOR_SCALE: &[f32] = &[FS7, GS7, AS7, B7, CS8, DS8, F8, FS8];
pub const G7_MAJOR_SCALE: &[f32] = &[G7, A7, B7, C8, D8, E8, FS8, G8];
pub const GS7_MAJOR_SCALE: &[f32] = &[GS7, AS7, C8, CS8, DS8, F8, G8, GS8];
pub const A7_MAJOR_SCALE: &[f32] = &[A7, B7, CS8, D8, E8, FS8, GS8, A8];
pub const AS7_MAJOR_SCALE: &[f32] = &[AS7, C8, D8, DS8, F8, G8, A8, AS8];
pub const B7_MAJOR_SCALE: &[f32] = &[B7, CS8, DS8, E8, FS8, GS8, AS8, B8];

// Octave 8
pub const C8_MAJOR_SCALE: &[f32] = &[C8, D8, E8, F8, G8, A8, B8, C9];
pub const CS8_MAJOR_SCALE: &[f32] = &[CS8, DS8, F8, FS8, GS8, AS8, C9, CS9];
pub const D8_MAJOR_SCALE: &[f32] = &[D8, E8, FS8, G8, A8, B8, CS9, D9];
pub const DS8_MAJOR_SCALE: &[f32] = &[DS8, F8, G8, GS8, AS8, C9, D9, DS9];
pub const E8_MAJOR_SCALE: &[f32] = &[E8, FS8, GS8, A8, B8, CS9, DS9, E9];
pub const F8_MAJOR_SCALE: &[f32] = &[F8, G8, A8, AS8, C9, D9, E9, F9];
pub const FS8_MAJOR_SCALE: &[f32] = &[FS8, GS8, AS8, B8, CS9, DS9, F9, FS9];
pub const G8_MAJOR_SCALE: &[f32] = &[G8, A8, B8, C9, D9, E9, FS9, G9];
pub const GS8_MAJOR_SCALE: &[f32] = &[GS8, AS8, C9, CS9, DS9, F9, G9, GS9];
pub const A8_MAJOR_SCALE: &[f32] = &[A8, B8, CS9, D9, E9, FS9, GS9, A9];
pub const AS8_MAJOR_SCALE: &[f32] = &[AS8, C9, D9, DS9, F9, G9, A9, AS9];
pub const B8_MAJOR_SCALE: &[f32] = &[B8, CS9, DS9, E9, FS9, GS9, AS9, B9];

// Octave 9
pub const C9_MAJOR_SCALE: &[f32] = &[C9, D9, E9, F9, G9, A9, B9, C10];
pub const CS9_MAJOR_SCALE: &[f32] = &[CS9, DS9, F9, FS9, GS9, AS9, C10, CS10];
pub const D9_MAJOR_SCALE: &[f32] = &[D9, E9, FS9, G9, A9, B9, CS10, D10];
pub const DS9_MAJOR_SCALE: &[f32] = &[DS9, F9, G9, GS9, AS9, C10, D10, DS10];
pub const E9_MAJOR_SCALE: &[f32] = &[E9, FS9, GS9, A9, B9, CS10, DS10, E10];
pub const F9_MAJOR_SCALE: &[f32] = &[F9, G9, A9, AS9, C10, D10, E10, F10];
pub const FS9_MAJOR_SCALE: &[f32] = &[FS9, GS9, AS9, B9, CS10, DS10, F10, FS10];
pub const G9_MAJOR_SCALE: &[f32] = &[G9, A9, B9, C10, D10, E10, FS10, G10];
pub const GS9_MAJOR_SCALE: &[f32] = &[GS9, AS9, C10, CS10, DS10, F10, G10, GS10];
pub const A9_MAJOR_SCALE: &[f32] = &[A9, B9, CS10, D10, E10, FS10, GS10, A10];
pub const AS9_MAJOR_SCALE: &[f32] = &[AS9, C10, D10, DS10, F10, G10, A10, AS10];
pub const B9_MAJOR_SCALE: &[f32] = &[B9, CS10, DS10, E10, FS10, GS10, AS10, B10];

// Octave 10
pub const C10_MAJOR_SCALE: &[f32] = &[C10, D10, E10, F10, G10, A10, B10, C11];
pub const CS10_MAJOR_SCALE: &[f32] = &[CS10, DS10, F10, FS10, GS10, AS10, C11, CS11];
pub const D10_MAJOR_SCALE: &[f32] = &[D10, E10, FS10, G10, A10, B10, CS11, D11];
pub const DS10_MAJOR_SCALE: &[f32] = &[DS10, F10, G10, GS10, AS10, C11, D11, DS11];
pub const E10_MAJOR_SCALE: &[f32] = &[E10, FS10, GS10, A10, B10, CS11, DS11, E11];
pub const F10_MAJOR_SCALE: &[f32] = &[F10, G10, A10, AS10, C11, D11, E11, F11];
pub const FS10_MAJOR_SCALE: &[f32] = &[FS10, GS10, AS10, B10, CS11, DS11, F11, FS11];
pub const G10_MAJOR_SCALE: &[f32] = &[G10, A10, B10, C11, D11, E11, FS11, G11];
pub const GS10_MAJOR_SCALE: &[f32] = &[GS10, AS10, C11, CS11, DS11, F11, G11, GS11];
pub const A10_MAJOR_SCALE: &[f32] = &[A10, B10, CS11, D11, E11, FS11, GS11, A11];
pub const AS10_MAJOR_SCALE: &[f32] = &[AS10, C11, D11, DS11, F11, G11, A11, AS11];
pub const B10_MAJOR_SCALE: &[f32] = &[B10, CS11, DS11, E11, FS11, GS11, AS11, B11];

// ===== MINOR_SCALE =====
// Octave -1
pub const C_1_MINOR_SCALE: &[f32] = &[C_1, D_1, DS_1, F_1, G_1, GS_1, AS_1, C0];
pub const CS_1_MINOR_SCALE: &[f32] = &[CS_1, DS_1, E_1, FS_1, GS_1, A_1, B_1, CS0];
pub const D_1_MINOR_SCALE: &[f32] = &[D_1, E_1, F_1, G_1, A_1, AS_1, C0, D0];
pub const DS_1_MINOR_SCALE: &[f32] = &[DS_1, F_1, FS_1, GS_1, AS_1, B_1, CS0, DS0];
pub const E_1_MINOR_SCALE: &[f32] = &[E_1, FS_1, G_1, A_1, B_1, C0, D0, E0];
pub const F_1_MINOR_SCALE: &[f32] = &[F_1, G_1, GS_1, AS_1, C0, CS0, DS0, F0];
pub const FS_1_MINOR_SCALE: &[f32] = &[FS_1, GS_1, A_1, B_1, CS0, D0, E0, FS0];
pub const G_1_MINOR_SCALE: &[f32] = &[G_1, A_1, AS_1, C0, D0, DS0, F0, G0];
pub const GS_1_MINOR_SCALE: &[f32] = &[GS_1, AS_1, B_1, CS0, DS0, E0, FS0, GS0];
pub const A_1_MINOR_SCALE: &[f32] = &[A_1, B_1, C0, D0, E0, F0, G0, A0];
pub const AS_1_MINOR_SCALE: &[f32] = &[AS_1, C0, CS0, DS0, F0, FS0, GS0, AS0];
pub const B_1_MINOR_SCALE: &[f32] = &[B_1, CS0, D0, E0, FS0, G0, A0, B0];

// Octave 0
pub const C0_MINOR_SCALE: &[f32] = &[C0, D0, DS0, F0, G0, GS0, AS0, C1];
pub const CS0_MINOR_SCALE: &[f32] = &[CS0, DS0, E0, FS0, GS0, A0, B0, CS1];
pub const D0_MINOR_SCALE: &[f32] = &[D0, E0, F0, G0, A0, AS0, C1, D1];
pub const DS0_MINOR_SCALE: &[f32] = &[DS0, F0, FS0, GS0, AS0, B0, CS1, DS1];
pub const E0_MINOR_SCALE: &[f32] = &[E0, FS0, G0, A0, B0, C1, D1, E1];
pub const F0_MINOR_SCALE: &[f32] = &[F0, G0, GS0, AS0, C1, CS1, DS1, F1];
pub const FS0_MINOR_SCALE: &[f32] = &[FS0, GS0, A0, B0, CS1, D1, E1, FS1];
pub const G0_MINOR_SCALE: &[f32] = &[G0, A0, AS0, C1, D1, DS1, F1, G1];
pub const GS0_MINOR_SCALE: &[f32] = &[GS0, AS0, B0, CS1, DS1, E1, FS1, GS1];
pub const A0_MINOR_SCALE: &[f32] = &[A0, B0, C1, D1, E1, F1, G1, A1];
pub const AS0_MINOR_SCALE: &[f32] = &[AS0, C1, CS1, DS1, F1, FS1, GS1, AS1];
pub const B0_MINOR_SCALE: &[f32] = &[B0, CS1, D1, E1, FS1, G1, A1, B1];

// Octave 7
pub const C7_MINOR_SCALE: &[f32] = &[C7, D7, DS7, F7, G7, GS7, AS7, C8];
pub const CS7_MINOR_SCALE: &[f32] = &[CS7, DS7, E7, FS7, GS7, A7, B7, CS8];
pub const D7_MINOR_SCALE: &[f32] = &[D7, E7, F7, G7, A7, AS7, C8, D8];
pub const DS7_MINOR_SCALE: &[f32] = &[DS7, F7, FS7, GS7, AS7, B7, CS8, DS8];
pub const E7_MINOR_SCALE: &[f32] = &[E7, FS7, G7, A7, B7, C8, D8, E8];
pub const F7_MINOR_SCALE: &[f32] = &[F7, G7, GS7, AS7, C8, CS8, DS8, F8];
pub const FS7_MINOR_SCALE: &[f32] = &[FS7, GS7, A7, B7, CS8, D8, E8, FS8];
pub const G7_MINOR_SCALE: &[f32] = &[G7, A7, AS7, C8, D8, DS8, F8, G8];
pub const GS7_MINOR_SCALE: &[f32] = &[GS7, AS7, B7, CS8, DS8, E8, FS8, GS8];
pub const A7_MINOR_SCALE: &[f32] = &[A7, B7, C8, D8, E8, F8, G8, A8];
pub const AS7_MINOR_SCALE: &[f32] = &[AS7, C8, CS8, DS8, F8, FS8, GS8, AS8];
pub const B7_MINOR_SCALE: &[f32] = &[B7, CS8, D8, E8, FS8, G8, A8, B8];

// Octave 8
pub const C8_MINOR_SCALE: &[f32] = &[C8, D8, DS8, F8, G8, GS8, AS8, C9];
pub const CS8_MINOR_SCALE: &[f32] = &[CS8, DS8, E8, FS8, GS8, A8, B8, CS9];
pub const D8_MINOR_SCALE: &[f32] = &[D8, E8, F8, G8, A8, AS8, C9, D9];
pub const DS8_MINOR_SCALE: &[f32] = &[DS8, F8, FS8, GS8, AS8, B8, CS9, DS9];
pub const E8_MINOR_SCALE: &[f32] = &[E8, FS8, G8, A8, B8, C9, D9, E9];
pub const F8_MINOR_SCALE: &[f32] = &[F8, G8, GS8, AS8, C9, CS9, DS9, F9];
pub const FS8_MINOR_SCALE: &[f32] = &[FS8, GS8, A8, B8, CS9, D9, E9, FS9];
pub const G8_MINOR_SCALE: &[f32] = &[G8, A8, AS8, C9, D9, DS9, F9, G9];
pub const GS8_MINOR_SCALE: &[f32] = &[GS8, AS8, B8, CS9, DS9, E9, FS9, GS9];
pub const A8_MINOR_SCALE: &[f32] = &[A8, B8, C9, D9, E9, F9, G9, A9];
pub const AS8_MINOR_SCALE: &[f32] = &[AS8, C9, CS9, DS9, F9, FS9, GS9, AS9];
pub const B8_MINOR_SCALE: &[f32] = &[B8, CS9, D9, E9, FS9, G9, A9, B9];

// Octave 9
pub const C9_MINOR_SCALE: &[f32] = &[C9, D9, DS9, F9, G9, GS9, AS9, C10];
pub const CS9_MINOR_SCALE: &[f32] = &[CS9, DS9, E9, FS9, GS9, A9, B9, CS10];
pub const D9_MINOR_SCALE: &[f32] = &[D9, E9, F9, G9, A9, AS9, C10, D10];
pub const DS9_MINOR_SCALE: &[f32] = &[DS9, F9, FS9, GS9, AS9, B9, CS10, DS10];
pub const E9_MINOR_SCALE: &[f32] = &[E9, FS9, G9, A9, B9, C10, D10, E10];
pub const F9_MINOR_SCALE: &[f32] = &[F9, G9, GS9, AS9, C10, CS10, DS10, F10];
pub const FS9_MINOR_SCALE: &[f32] = &[FS9, GS9, A9, B9, CS10, D10, E10, FS10];
pub const G9_MINOR_SCALE: &[f32] = &[G9, A9, AS9, C10, D10, DS10, F10, G10];
pub const GS9_MINOR_SCALE: &[f32] = &[GS9, AS9, B9, CS10, DS10, E10, FS10, GS10];
pub const A9_MINOR_SCALE: &[f32] = &[A9, B9, C10, D10, E10, F10, G10, A10];
pub const AS9_MINOR_SCALE: &[f32] = &[AS9, C10, CS10, DS10, F10, FS10, GS10, AS10];
pub const B9_MINOR_SCALE: &[f32] = &[B9, CS10, D10, E10, FS10, G10, A10, B10];

// Octave 10
pub const C10_MINOR_SCALE: &[f32] = &[C10, D10, DS10, F10, G10, GS10, AS10, C11];
pub const CS10_MINOR_SCALE: &[f32] = &[CS10, DS10, E10, FS10, GS10, A10, B10, CS11];
pub const D10_MINOR_SCALE: &[f32] = &[D10, E10, F10, G10, A10, AS10, C11, D11];
pub const DS10_MINOR_SCALE: &[f32] = &[DS10, F10, FS10, GS10, AS10, B10, CS11, DS11];
pub const E10_MINOR_SCALE: &[f32] = &[E10, FS10, G10, A10, B10, C11, D11, E11];
pub const F10_MINOR_SCALE: &[f32] = &[F10, G10, GS10, AS10, C11, CS11, DS11, F11];
pub const FS10_MINOR_SCALE: &[f32] = &[FS10, GS10, A10, B10, CS11, D11, E11, FS11];
pub const G10_MINOR_SCALE: &[f32] = &[G10, A10, AS10, C11, D11, DS11, F11, G11];
pub const GS10_MINOR_SCALE: &[f32] = &[GS10, AS10, B10, CS11, DS11, E11, FS11, GS11];
pub const A10_MINOR_SCALE: &[f32] = &[A10, B10, C11, D11, E11, F11, G11, A11];
pub const AS10_MINOR_SCALE: &[f32] = &[AS10, C11, CS11, DS11, F11, FS11, GS11, AS11];
pub const B10_MINOR_SCALE: &[f32] = &[B10, CS11, D11, E11, FS11, G11, A11, B11];

// ===== MAJOR_PENTATONIC_SCALE =====
// Octave -1
pub const C_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[C_1, D_1, E_1, G_1, A_1, C0];
pub const CS_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS_1, DS_1, F_1, GS_1, AS_1, CS0];
pub const D_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[D_1, E_1, FS_1, A_1, B_1, D0];
pub const DS_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS_1, F_1, G_1, AS_1, C0, DS0];
pub const E_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[E_1, FS_1, GS_1, B_1, CS0, E0];
pub const F_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[F_1, G_1, A_1, C0, D0, F0];
pub const FS_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS_1, GS_1, AS_1, CS0, DS0, FS0];
pub const G_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[G_1, A_1, B_1, D0, E0, G0];
pub const GS_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS_1, AS_1, C0, DS0, F0, GS0];
pub const A_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[A_1, B_1, CS0, E0, FS0, A0];
pub const AS_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS_1, C0, D0, F0, G0, AS0];
pub const B_1_MAJOR_PENTATONIC_SCALE: &[f32] = &[B_1, CS0, DS0, FS0, GS0, B0];

// Octave 0
pub const C0_MAJOR_PENTATONIC_SCALE: &[f32] = &[C0, D0, E0, G0, A0, C1];
pub const CS0_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS0, DS0, F0, GS0, AS0, CS1];
pub const D0_MAJOR_PENTATONIC_SCALE: &[f32] = &[D0, E0, FS0, A0, B0, D1];
pub const DS0_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS0, F0, G0, AS0, C1, DS1];
pub const E0_MAJOR_PENTATONIC_SCALE: &[f32] = &[E0, FS0, GS0, B0, CS1, E1];
pub const F0_MAJOR_PENTATONIC_SCALE: &[f32] = &[F0, G0, A0, C1, D1, F1];
pub const FS0_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS0, GS0, AS0, CS1, DS1, FS1];
pub const G0_MAJOR_PENTATONIC_SCALE: &[f32] = &[G0, A0, B0, D1, E1, G1];
pub const GS0_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS0, AS0, C1, DS1, F1, GS1];
pub const A0_MAJOR_PENTATONIC_SCALE: &[f32] = &[A0, B0, CS1, E1, FS1, A1];
pub const AS0_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS0, C1, D1, F1, G1, AS1];
pub const B0_MAJOR_PENTATONIC_SCALE: &[f32] = &[B0, CS1, DS1, FS1, GS1, B1];

// Octave 7
pub const C7_MAJOR_PENTATONIC_SCALE: &[f32] = &[C7, D7, E7, G7, A7, C8];
pub const CS7_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS7, DS7, F7, GS7, AS7, CS8];
pub const D7_MAJOR_PENTATONIC_SCALE: &[f32] = &[D7, E7, FS7, A7, B7, D8];
pub const DS7_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS7, F7, G7, AS7, C8, DS8];
pub const E7_MAJOR_PENTATONIC_SCALE: &[f32] = &[E7, FS7, GS7, B7, CS8, E8];
pub const F7_MAJOR_PENTATONIC_SCALE: &[f32] = &[F7, G7, A7, C8, D8, F8];
pub const FS7_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS7, GS7, AS7, CS8, DS8, FS8];
pub const G7_MAJOR_PENTATONIC_SCALE: &[f32] = &[G7, A7, B7, D8, E8, G8];
pub const GS7_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS7, AS7, C8, DS8, F8, GS8];
pub const A7_MAJOR_PENTATONIC_SCALE: &[f32] = &[A7, B7, CS8, E8, FS8, A8];
pub const AS7_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS7, C8, D8, F8, G8, AS8];
pub const B7_MAJOR_PENTATONIC_SCALE: &[f32] = &[B7, CS8, DS8, FS8, GS8, B8];

// Octave 8
pub const C8_MAJOR_PENTATONIC_SCALE: &[f32] = &[C8, D8, E8, G8, A8, C9];
pub const CS8_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS8, DS8, F8, GS8, AS8, CS9];
pub const D8_MAJOR_PENTATONIC_SCALE: &[f32] = &[D8, E8, FS8, A8, B8, D9];
pub const DS8_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS8, F8, G8, AS8, C9, DS9];
pub const E8_MAJOR_PENTATONIC_SCALE: &[f32] = &[E8, FS8, GS8, B8, CS9, E9];
pub const F8_MAJOR_PENTATONIC_SCALE: &[f32] = &[F8, G8, A8, C9, D9, F9];
pub const FS8_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS8, GS8, AS8, CS9, DS9, FS9];
pub const G8_MAJOR_PENTATONIC_SCALE: &[f32] = &[G8, A8, B8, D9, E9, G9];
pub const GS8_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS8, AS8, C9, DS9, F9, GS9];
pub const A8_MAJOR_PENTATONIC_SCALE: &[f32] = &[A8, B8, CS9, E9, FS9, A9];
pub const AS8_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS8, C9, D9, F9, G9, AS9];
pub const B8_MAJOR_PENTATONIC_SCALE: &[f32] = &[B8, CS9, DS9, FS9, GS9, B9];

// Octave 9
pub const C9_MAJOR_PENTATONIC_SCALE: &[f32] = &[C9, D9, E9, G9, A9, C10];
pub const CS9_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS9, DS9, F9, GS9, AS9, CS10];
pub const D9_MAJOR_PENTATONIC_SCALE: &[f32] = &[D9, E9, FS9, A9, B9, D10];
pub const DS9_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS9, F9, G9, AS9, C10, DS10];
pub const E9_MAJOR_PENTATONIC_SCALE: &[f32] = &[E9, FS9, GS9, B9, CS10, E10];
pub const F9_MAJOR_PENTATONIC_SCALE: &[f32] = &[F9, G9, A9, C10, D10, F10];
pub const FS9_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS9, GS9, AS9, CS10, DS10, FS10];
pub const G9_MAJOR_PENTATONIC_SCALE: &[f32] = &[G9, A9, B9, D10, E10, G10];
pub const GS9_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS9, AS9, C10, DS10, F10, GS10];
pub const A9_MAJOR_PENTATONIC_SCALE: &[f32] = &[A9, B9, CS10, E10, FS10, A10];
pub const AS9_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS9, C10, D10, F10, G10, AS10];
pub const B9_MAJOR_PENTATONIC_SCALE: &[f32] = &[B9, CS10, DS10, FS10, GS10, B10];

// Octave 10
pub const C10_MAJOR_PENTATONIC_SCALE: &[f32] = &[C10, D10, E10, G10, A10, C11];
pub const CS10_MAJOR_PENTATONIC_SCALE: &[f32] = &[CS10, DS10, F10, GS10, AS10, CS11];
pub const D10_MAJOR_PENTATONIC_SCALE: &[f32] = &[D10, E10, FS10, A10, B10, D11];
pub const DS10_MAJOR_PENTATONIC_SCALE: &[f32] = &[DS10, F10, G10, AS10, C11, DS11];
pub const E10_MAJOR_PENTATONIC_SCALE: &[f32] = &[E10, FS10, GS10, B10, CS11, E11];
pub const F10_MAJOR_PENTATONIC_SCALE: &[f32] = &[F10, G10, A10, C11, D11, F11];
pub const FS10_MAJOR_PENTATONIC_SCALE: &[f32] = &[FS10, GS10, AS10, CS11, DS11, FS11];
pub const G10_MAJOR_PENTATONIC_SCALE: &[f32] = &[G10, A10, B10, D11, E11, G11];
pub const GS10_MAJOR_PENTATONIC_SCALE: &[f32] = &[GS10, AS10, C11, DS11, F11, GS11];
pub const A10_MAJOR_PENTATONIC_SCALE: &[f32] = &[A10, B10, CS11, E11, FS11, A11];
pub const AS10_MAJOR_PENTATONIC_SCALE: &[f32] = &[AS10, C11, D11, F11, G11, AS11];
pub const B10_MAJOR_PENTATONIC_SCALE: &[f32] = &[B10, CS11, DS11, FS11, GS11, B11];

// ===== MINOR_PENTATONIC_SCALE =====
// Octave -1
pub const C_1_MINOR_PENTATONIC_SCALE: &[f32] = &[C_1, DS_1, F_1, G_1, AS_1, C0];
pub const CS_1_MINOR_PENTATONIC_SCALE: &[f32] = &[CS_1, E_1, FS_1, GS_1, B_1, CS0];
pub const D_1_MINOR_PENTATONIC_SCALE: &[f32] = &[D_1, F_1, G_1, A_1, C0, D0];
pub const DS_1_MINOR_PENTATONIC_SCALE: &[f32] = &[DS_1, FS_1, GS_1, AS_1, CS0, DS0];
pub const E_1_MINOR_PENTATONIC_SCALE: &[f32] = &[E_1, G_1, A_1, B_1, D0, E0];
pub const F_1_MINOR_PENTATONIC_SCALE: &[f32] = &[F_1, GS_1, AS_1, C0, DS0, F0];
pub const FS_1_MINOR_PENTATONIC_SCALE: &[f32] = &[FS_1, A_1, B_1, CS0, E0, FS0];
pub const G_1_MINOR_PENTATONIC_SCALE: &[f32] = &[G_1, AS_1, C0, D0, F0, G0];
pub const GS_1_MINOR_PENTATONIC_SCALE: &[f32] = &[GS_1, B_1, CS0, DS0, FS0, GS0];
pub const A_1_MINOR_PENTATONIC_SCALE: &[f32] = &[A_1, C0, D0, E0, G0, A0];
pub const AS_1_MINOR_PENTATONIC_SCALE: &[f32] = &[AS_1, CS0, DS0, F0, GS0, AS0];
pub const B_1_MINOR_PENTATONIC_SCALE: &[f32] = &[B_1, D0, E0, FS0, A0, B0];

// Octave 0
pub const C0_MINOR_PENTATONIC_SCALE: &[f32] = &[C0, DS0, F0, G0, AS0, C1];
pub const CS0_MINOR_PENTATONIC_SCALE: &[f32] = &[CS0, E0, FS0, GS0, B0, CS1];
pub const D0_MINOR_PENTATONIC_SCALE: &[f32] = &[D0, F0, G0, A0, C1, D1];
pub const DS0_MINOR_PENTATONIC_SCALE: &[f32] = &[DS0, FS0, GS0, AS0, CS1, DS1];
pub const E0_MINOR_PENTATONIC_SCALE: &[f32] = &[E0, G0, A0, B0, D1, E1];
pub const F0_MINOR_PENTATONIC_SCALE: &[f32] = &[F0, GS0, AS0, C1, DS1, F1];
pub const FS0_MINOR_PENTATONIC_SCALE: &[f32] = &[FS0, A0, B0, CS1, E1, FS1];
pub const G0_MINOR_PENTATONIC_SCALE: &[f32] = &[G0, AS0, C1, D1, F1, G1];
pub const GS0_MINOR_PENTATONIC_SCALE: &[f32] = &[GS0, B0, CS1, DS1, FS1, GS1];
pub const A0_MINOR_PENTATONIC_SCALE: &[f32] = &[A0, C1, D1, E1, G1, A1];
pub const AS0_MINOR_PENTATONIC_SCALE: &[f32] = &[AS0, CS1, DS1, F1, GS1, AS1];
pub const B0_MINOR_PENTATONIC_SCALE: &[f32] = &[B0, D1, E1, FS1, A1, B1];

// Octave 7
pub const C7_MINOR_PENTATONIC_SCALE: &[f32] = &[C7, DS7, F7, G7, AS7, C8];
pub const CS7_MINOR_PENTATONIC_SCALE: &[f32] = &[CS7, E7, FS7, GS7, B7, CS8];
pub const D7_MINOR_PENTATONIC_SCALE: &[f32] = &[D7, F7, G7, A7, C8, D8];
pub const DS7_MINOR_PENTATONIC_SCALE: &[f32] = &[DS7, FS7, GS7, AS7, CS8, DS8];
pub const E7_MINOR_PENTATONIC_SCALE: &[f32] = &[E7, G7, A7, B7, D8, E8];
pub const F7_MINOR_PENTATONIC_SCALE: &[f32] = &[F7, GS7, AS7, C8, DS8, F8];
pub const FS7_MINOR_PENTATONIC_SCALE: &[f32] = &[FS7, A7, B7, CS8, E8, FS8];
pub const G7_MINOR_PENTATONIC_SCALE: &[f32] = &[G7, AS7, C8, D8, F8, G8];
pub const GS7_MINOR_PENTATONIC_SCALE: &[f32] = &[GS7, B7, CS8, DS8, FS8, GS8];
pub const A7_MINOR_PENTATONIC_SCALE: &[f32] = &[A7, C8, D8, E8, G8, A8];
pub const AS7_MINOR_PENTATONIC_SCALE: &[f32] = &[AS7, CS8, DS8, F8, GS8, AS8];
pub const B7_MINOR_PENTATONIC_SCALE: &[f32] = &[B7, D8, E8, FS8, A8, B8];

// Octave 8
pub const C8_MINOR_PENTATONIC_SCALE: &[f32] = &[C8, DS8, F8, G8, AS8, C9];
pub const CS8_MINOR_PENTATONIC_SCALE: &[f32] = &[CS8, E8, FS8, GS8, B8, CS9];
pub const D8_MINOR_PENTATONIC_SCALE: &[f32] = &[D8, F8, G8, A8, C9, D9];
pub const DS8_MINOR_PENTATONIC_SCALE: &[f32] = &[DS8, FS8, GS8, AS8, CS9, DS9];
pub const E8_MINOR_PENTATONIC_SCALE: &[f32] = &[E8, G8, A8, B8, D9, E9];
pub const F8_MINOR_PENTATONIC_SCALE: &[f32] = &[F8, GS8, AS8, C9, DS9, F9];
pub const FS8_MINOR_PENTATONIC_SCALE: &[f32] = &[FS8, A8, B8, CS9, E9, FS9];
pub const G8_MINOR_PENTATONIC_SCALE: &[f32] = &[G8, AS8, C9, D9, F9, G9];
pub const GS8_MINOR_PENTATONIC_SCALE: &[f32] = &[GS8, B8, CS9, DS9, FS9, GS9];
pub const A8_MINOR_PENTATONIC_SCALE: &[f32] = &[A8, C9, D9, E9, G9, A9];
pub const AS8_MINOR_PENTATONIC_SCALE: &[f32] = &[AS8, CS9, DS9, F9, GS9, AS9];
pub const B8_MINOR_PENTATONIC_SCALE: &[f32] = &[B8, D9, E9, FS9, A9, B9];

// Octave 9
pub const C9_MINOR_PENTATONIC_SCALE: &[f32] = &[C9, DS9, F9, G9, AS9, C10];
pub const CS9_MINOR_PENTATONIC_SCALE: &[f32] = &[CS9, E9, FS9, GS9, B9, CS10];
pub const D9_MINOR_PENTATONIC_SCALE: &[f32] = &[D9, F9, G9, A9, C10, D10];
pub const DS9_MINOR_PENTATONIC_SCALE: &[f32] = &[DS9, FS9, GS9, AS9, CS10, DS10];
pub const E9_MINOR_PENTATONIC_SCALE: &[f32] = &[E9, G9, A9, B9, D10, E10];
pub const F9_MINOR_PENTATONIC_SCALE: &[f32] = &[F9, GS9, AS9, C10, DS10, F10];
pub const FS9_MINOR_PENTATONIC_SCALE: &[f32] = &[FS9, A9, B9, CS10, E10, FS10];
pub const G9_MINOR_PENTATONIC_SCALE: &[f32] = &[G9, AS9, C10, D10, F10, G10];
pub const GS9_MINOR_PENTATONIC_SCALE: &[f32] = &[GS9, B9, CS10, DS10, FS10, GS10];
pub const A9_MINOR_PENTATONIC_SCALE: &[f32] = &[A9, C10, D10, E10, G10, A10];
pub const AS9_MINOR_PENTATONIC_SCALE: &[f32] = &[AS9, CS10, DS10, F10, GS10, AS10];
pub const B9_MINOR_PENTATONIC_SCALE: &[f32] = &[B9, D10, E10, FS10, A10, B10];

// Octave 10
pub const C10_MINOR_PENTATONIC_SCALE: &[f32] = &[C10, DS10, F10, G10, AS10, C11];
pub const CS10_MINOR_PENTATONIC_SCALE: &[f32] = &[CS10, E10, FS10, GS10, B10, CS11];
pub const D10_MINOR_PENTATONIC_SCALE: &[f32] = &[D10, F10, G10, A10, C11, D11];
pub const DS10_MINOR_PENTATONIC_SCALE: &[f32] = &[DS10, FS10, GS10, AS10, CS11, DS11];
pub const E10_MINOR_PENTATONIC_SCALE: &[f32] = &[E10, G10, A10, B10, D11, E11];
pub const F10_MINOR_PENTATONIC_SCALE: &[f32] = &[F10, GS10, AS10, C11, DS11, F11];
pub const FS10_MINOR_PENTATONIC_SCALE: &[f32] = &[FS10, A10, B10, CS11, E11, FS11];
pub const G10_MINOR_PENTATONIC_SCALE: &[f32] = &[G10, AS10, C11, D11, F11, G11];
pub const GS10_MINOR_PENTATONIC_SCALE: &[f32] = &[GS10, B10, CS11, DS11, FS11, GS11];
pub const A10_MINOR_PENTATONIC_SCALE: &[f32] = &[A10, C11, D11, E11, G11, A11];
pub const AS10_MINOR_PENTATONIC_SCALE: &[f32] = &[AS10, CS11, DS11, F11, GS11, AS11];
pub const B10_MINOR_PENTATONIC_SCALE: &[f32] = &[B10, D11, E11, FS11, A11, B11];

// ===== BLUES_SCALE =====
// Octave -1
pub const C_1_BLUES_SCALE: &[f32] = &[C_1, DS_1, F_1, FS_1, G_1, AS_1, C0];
pub const CS_1_BLUES_SCALE: &[f32] = &[CS_1, E_1, FS_1, G_1, GS_1, B_1, CS0];
pub const D_1_BLUES_SCALE: &[f32] = &[D_1, F_1, G_1, GS_1, A_1, C0, D0];
pub const DS_1_BLUES_SCALE: &[f32] = &[DS_1, FS_1, GS_1, A_1, AS_1, CS0, DS0];
pub const E_1_BLUES_SCALE: &[f32] = &[E_1, G_1, A_1, AS_1, B_1, D0, E0];
pub const F_1_BLUES_SCALE: &[f32] = &[F_1, GS_1, AS_1, B_1, C0, DS0, F0];
pub const FS_1_BLUES_SCALE: &[f32] = &[FS_1, A_1, B_1, C0, CS0, E0, FS0];
pub const G_1_BLUES_SCALE: &[f32] = &[G_1, AS_1, C0, CS0, D0, F0, G0];
pub const GS_1_BLUES_SCALE: &[f32] = &[GS_1, B_1, CS0, D0, DS0, FS0, GS0];
pub const A_1_BLUES_SCALE: &[f32] = &[A_1, C0, D0, DS0, E0, G0, A0];
pub const AS_1_BLUES_SCALE: &[f32] = &[AS_1, CS0, DS0, E0, F0, GS0, AS0];
pub const B_1_BLUES_SCALE: &[f32] = &[B_1, D0, E0, F0, FS0, A0, B0];

// Octave 0
pub const C0_BLUES_SCALE: &[f32] = &[C0, DS0, F0, FS0, G0, AS0, C1];
pub const CS0_BLUES_SCALE: &[f32] = &[CS0, E0, FS0, G0, GS0, B0, CS1];
pub const D0_BLUES_SCALE: &[f32] = &[D0, F0, G0, GS0, A0, C1, D1];
pub const DS0_BLUES_SCALE: &[f32] = &[DS0, FS0, GS0, A0, AS0, CS1, DS1];
pub const E0_BLUES_SCALE: &[f32] = &[E0, G0, A0, AS0, B0, D1, E1];
pub const F0_BLUES_SCALE: &[f32] = &[F0, GS0, AS0, B0, C1, DS1, F1];
pub const FS0_BLUES_SCALE: &[f32] = &[FS0, A0, B0, C1, CS1, E1, FS1];
pub const G0_BLUES_SCALE: &[f32] = &[G0, AS0, C1, CS1, D1, F1, G1];
pub const GS0_BLUES_SCALE: &[f32] = &[GS0, B0, CS1, D1, DS1, FS1, GS1];
pub const A0_BLUES_SCALE: &[f32] = &[A0, C1, D1, DS1, E1, G1, A1];
pub const AS0_BLUES_SCALE: &[f32] = &[AS0, CS1, DS1, E1, F1, GS1, AS1];
pub const B0_BLUES_SCALE: &[f32] = &[B0, D1, E1, F1, FS1, A1, B1];

// Octave 7
pub const C7_BLUES_SCALE: &[f32] = &[C7, DS7, F7, FS7, G7, AS7, C8];
pub const CS7_BLUES_SCALE: &[f32] = &[CS7, E7, FS7, G7, GS7, B7, CS8];
pub const D7_BLUES_SCALE: &[f32] = &[D7, F7, G7, GS7, A7, C8, D8];
pub const DS7_BLUES_SCALE: &[f32] = &[DS7, FS7, GS7, A7, AS7, CS8, DS8];
pub const E7_BLUES_SCALE: &[f32] = &[E7, G7, A7, AS7, B7, D8, E8];
pub const F7_BLUES_SCALE: &[f32] = &[F7, GS7, AS7, B7, C8, DS8, F8];
pub const FS7_BLUES_SCALE: &[f32] = &[FS7, A7, B7, C8, CS8, E8, FS8];
pub const G7_BLUES_SCALE: &[f32] = &[G7, AS7, C8, CS8, D8, F8, G8];
pub const GS7_BLUES_SCALE: &[f32] = &[GS7, B7, CS8, D8, DS8, FS8, GS8];
pub const A7_BLUES_SCALE: &[f32] = &[A7, C8, D8, DS8, E8, G8, A8];
pub const AS7_BLUES_SCALE: &[f32] = &[AS7, CS8, DS8, E8, F8, GS8, AS8];
pub const B7_BLUES_SCALE: &[f32] = &[B7, D8, E8, F8, FS8, A8, B8];

// Octave 8
pub const C8_BLUES_SCALE: &[f32] = &[C8, DS8, F8, FS8, G8, AS8, C9];
pub const CS8_BLUES_SCALE: &[f32] = &[CS8, E8, FS8, G8, GS8, B8, CS9];
pub const D8_BLUES_SCALE: &[f32] = &[D8, F8, G8, GS8, A8, C9, D9];
pub const DS8_BLUES_SCALE: &[f32] = &[DS8, FS8, GS8, A8, AS8, CS9, DS9];
pub const E8_BLUES_SCALE: &[f32] = &[E8, G8, A8, AS8, B8, D9, E9];
pub const F8_BLUES_SCALE: &[f32] = &[F8, GS8, AS8, B8, C9, DS9, F9];
pub const FS8_BLUES_SCALE: &[f32] = &[FS8, A8, B8, C9, CS9, E9, FS9];
pub const G8_BLUES_SCALE: &[f32] = &[G8, AS8, C9, CS9, D9, F9, G9];
pub const GS8_BLUES_SCALE: &[f32] = &[GS8, B8, CS9, D9, DS9, FS9, GS9];
pub const A8_BLUES_SCALE: &[f32] = &[A8, C9, D9, DS9, E9, G9, A9];
pub const AS8_BLUES_SCALE: &[f32] = &[AS8, CS9, DS9, E9, F9, GS9, AS9];
pub const B8_BLUES_SCALE: &[f32] = &[B8, D9, E9, F9, FS9, A9, B9];

// Octave 9
pub const C9_BLUES_SCALE: &[f32] = &[C9, DS9, F9, FS9, G9, AS9, C10];
pub const CS9_BLUES_SCALE: &[f32] = &[CS9, E9, FS9, G9, GS9, B9, CS10];
pub const D9_BLUES_SCALE: &[f32] = &[D9, F9, G9, GS9, A9, C10, D10];
pub const DS9_BLUES_SCALE: &[f32] = &[DS9, FS9, GS9, A9, AS9, CS10, DS10];
pub const E9_BLUES_SCALE: &[f32] = &[E9, G9, A9, AS9, B9, D10, E10];
pub const F9_BLUES_SCALE: &[f32] = &[F9, GS9, AS9, B9, C10, DS10, F10];
pub const FS9_BLUES_SCALE: &[f32] = &[FS9, A9, B9, C10, CS10, E10, FS10];
pub const G9_BLUES_SCALE: &[f32] = &[G9, AS9, C10, CS10, D10, F10, G10];
pub const GS9_BLUES_SCALE: &[f32] = &[GS9, B9, CS10, D10, DS10, FS10, GS10];
pub const A9_BLUES_SCALE: &[f32] = &[A9, C10, D10, DS10, E10, G10, A10];
pub const AS9_BLUES_SCALE: &[f32] = &[AS9, CS10, DS10, E10, F10, GS10, AS10];
pub const B9_BLUES_SCALE: &[f32] = &[B9, D10, E10, F10, FS10, A10, B10];

// Octave 10
pub const C10_BLUES_SCALE: &[f32] = &[C10, DS10, F10, FS10, G10, AS10, C11];
pub const CS10_BLUES_SCALE: &[f32] = &[CS10, E10, FS10, G10, GS10, B10, CS11];
pub const D10_BLUES_SCALE: &[f32] = &[D10, F10, G10, GS10, A10, C11, D11];
pub const DS10_BLUES_SCALE: &[f32] = &[DS10, FS10, GS10, A10, AS10, CS11, DS11];
pub const E10_BLUES_SCALE: &[f32] = &[E10, G10, A10, AS10, B10, D11, E11];
pub const F10_BLUES_SCALE: &[f32] = &[F10, GS10, AS10, B10, C11, DS11, F11];
pub const FS10_BLUES_SCALE: &[f32] = &[FS10, A10, B10, C11, CS11, E11, FS11];
pub const G10_BLUES_SCALE: &[f32] = &[G10, AS10, C11, CS11, D11, F11, G11];
pub const GS10_BLUES_SCALE: &[f32] = &[GS10, B10, CS11, D11, DS11, FS11, GS11];
pub const A10_BLUES_SCALE: &[f32] = &[A10, C11, D11, DS11, E11, G11, A11];
pub const AS10_BLUES_SCALE: &[f32] = &[AS10, CS11, DS11, E11, F11, GS11, AS11];
pub const B10_BLUES_SCALE: &[f32] = &[B10, D11, E11, F11, FS11, A11, B11];

// ===== MELODIC_MINOR_SCALE =====
// Octave -1
pub const C_1_MELODIC_MINOR_SCALE: &[f32] = &[C_1, D_1, DS_1, F_1, G_1, A_1, B_1, C0];
pub const CS_1_MELODIC_MINOR_SCALE: &[f32] = &[CS_1, DS_1, E_1, FS_1, GS_1, AS_1, C0, CS0];
pub const D_1_MELODIC_MINOR_SCALE: &[f32] = &[D_1, E_1, F_1, G_1, A_1, B_1, CS0, D0];
pub const DS_1_MELODIC_MINOR_SCALE: &[f32] = &[DS_1, F_1, FS_1, GS_1, AS_1, C0, D0, DS0];
pub const E_1_MELODIC_MINOR_SCALE: &[f32] = &[E_1, FS_1, G_1, A_1, B_1, CS0, DS0, E0];
pub const F_1_MELODIC_MINOR_SCALE: &[f32] = &[F_1, G_1, GS_1, AS_1, C0, D0, E0, F0];
pub const FS_1_MELODIC_MINOR_SCALE: &[f32] = &[FS_1, GS_1, A_1, B_1, CS0, DS0, F0, FS0];
pub const G_1_MELODIC_MINOR_SCALE: &[f32] = &[G_1, A_1, AS_1, C0, D0, E0, FS0, G0];
pub const GS_1_MELODIC_MINOR_SCALE: &[f32] = &[GS_1, AS_1, B_1, CS0, DS0, F0, G0, GS0];
pub const A_1_MELODIC_MINOR_SCALE: &[f32] = &[A_1, B_1, C0, D0, E0, FS0, GS0, A0];
pub const AS_1_MELODIC_MINOR_SCALE: &[f32] = &[AS_1, C0, CS0, DS0, F0, G0, A0, AS0];
pub const B_1_MELODIC_MINOR_SCALE: &[f32] = &[B_1, CS0, D0, E0, FS0, GS0, AS0, B0];

// Octave 0
pub const C0_MELODIC_MINOR_SCALE: &[f32] = &[C0, D0, DS0, F0, G0, A0, B0, C1];
pub const CS0_MELODIC_MINOR_SCALE: &[f32] = &[CS0, DS0, E0, FS0, GS0, AS0, C1, CS1];
pub const D0_MELODIC_MINOR_SCALE: &[f32] = &[D0, E0, F0, G0, A0, B0, CS1, D1];
pub const DS0_MELODIC_MINOR_SCALE: &[f32] = &[DS0, F0, FS0, GS0, AS0, C1, D1, DS1];
pub const E0_MELODIC_MINOR_SCALE: &[f32] = &[E0, FS0, G0, A0, B0, CS1, DS1, E1];
pub const F0_MELODIC_MINOR_SCALE: &[f32] = &[F0, G0, GS0, AS0, C1, D1, E1, F1];
pub const FS0_MELODIC_MINOR_SCALE: &[f32] = &[FS0, GS0, A0, B0, CS1, DS1, F1, FS1];
pub const G0_MELODIC_MINOR_SCALE: &[f32] = &[G0, A0, AS0, C1, D1, E1, FS1, G1];
pub const GS0_MELODIC_MINOR_SCALE: &[f32] = &[GS0, AS0, B0, CS1, DS1, F1, G1, GS1];
pub const A0_MELODIC_MINOR_SCALE: &[f32] = &[A0, B0, C1, D1, E1, FS1, GS1, A1];
pub const AS0_MELODIC_MINOR_SCALE: &[f32] = &[AS0, C1, CS1, DS1, F1, G1, A1, AS1];
pub const B0_MELODIC_MINOR_SCALE: &[f32] = &[B0, CS1, D1, E1, FS1, GS1, AS1, B1];

// Octave 7
pub const C7_MELODIC_MINOR_SCALE: &[f32] = &[C7, D7, DS7, F7, G7, A7, B7, C8];
pub const CS7_MELODIC_MINOR_SCALE: &[f32] = &[CS7, DS7, E7, FS7, GS7, AS7, C8, CS8];
pub const D7_MELODIC_MINOR_SCALE: &[f32] = &[D7, E7, F7, G7, A7, B7, CS8, D8];
pub const DS7_MELODIC_MINOR_SCALE: &[f32] = &[DS7, F7, FS7, GS7, AS7, C8, D8, DS8];
pub const E7_MELODIC_MINOR_SCALE: &[f32] = &[E7, FS7, G7, A7, B7, CS8, DS8, E8];
pub const F7_MELODIC_MINOR_SCALE: &[f32] = &[F7, G7, GS7, AS7, C8, D8, E8, F8];
pub const FS7_MELODIC_MINOR_SCALE: &[f32] = &[FS7, GS7, A7, B7, CS8, DS8, F8, FS8];
pub const G7_MELODIC_MINOR_SCALE: &[f32] = &[G7, A7, AS7, C8, D8, E8, FS8, G8];
pub const GS7_MELODIC_MINOR_SCALE: &[f32] = &[GS7, AS7, B7, CS8, DS8, F8, G8, GS8];
pub const A7_MELODIC_MINOR_SCALE: &[f32] = &[A7, B7, C8, D8, E8, FS8, GS8, A8];
pub const AS7_MELODIC_MINOR_SCALE: &[f32] = &[AS7, C8, CS8, DS8, F8, G8, A8, AS8];
pub const B7_MELODIC_MINOR_SCALE: &[f32] = &[B7, CS8, D8, E8, FS8, GS8, AS8, B8];

// Octave 8
pub const C8_MELODIC_MINOR_SCALE: &[f32] = &[C8, D8, DS8, F8, G8, A8, B8, C9];
pub const CS8_MELODIC_MINOR_SCALE: &[f32] = &[CS8, DS8, E8, FS8, GS8, AS8, C9, CS9];
pub const D8_MELODIC_MINOR_SCALE: &[f32] = &[D8, E8, F8, G8, A8, B8, CS9, D9];
pub const DS8_MELODIC_MINOR_SCALE: &[f32] = &[DS8, F8, FS8, GS8, AS8, C9, D9, DS9];
pub const E8_MELODIC_MINOR_SCALE: &[f32] = &[E8, FS8, G8, A8, B8, CS9, DS9, E9];
pub const F8_MELODIC_MINOR_SCALE: &[f32] = &[F8, G8, GS8, AS8, C9, D9, E9, F9];
pub const FS8_MELODIC_MINOR_SCALE: &[f32] = &[FS8, GS8, A8, B8, CS9, DS9, F9, FS9];
pub const G8_MELODIC_MINOR_SCALE: &[f32] = &[G8, A8, AS8, C9, D9, E9, FS9, G9];
pub const GS8_MELODIC_MINOR_SCALE: &[f32] = &[GS8, AS8, B8, CS9, DS9, F9, G9, GS9];
pub const A8_MELODIC_MINOR_SCALE: &[f32] = &[A8, B8, C9, D9, E9, FS9, GS9, A9];
pub const AS8_MELODIC_MINOR_SCALE: &[f32] = &[AS8, C9, CS9, DS9, F9, G9, A9, AS9];
pub const B8_MELODIC_MINOR_SCALE: &[f32] = &[B8, CS9, D9, E9, FS9, GS9, AS9, B9];

// Octave 9
pub const C9_MELODIC_MINOR_SCALE: &[f32] = &[C9, D9, DS9, F9, G9, A9, B9, C10];
pub const CS9_MELODIC_MINOR_SCALE: &[f32] = &[CS9, DS9, E9, FS9, GS9, AS9, C10, CS10];
pub const D9_MELODIC_MINOR_SCALE: &[f32] = &[D9, E9, F9, G9, A9, B9, CS10, D10];
pub const DS9_MELODIC_MINOR_SCALE: &[f32] = &[DS9, F9, FS9, GS9, AS9, C10, D10, DS10];
pub const E9_MELODIC_MINOR_SCALE: &[f32] = &[E9, FS9, G9, A9, B9, CS10, DS10, E10];
pub const F9_MELODIC_MINOR_SCALE: &[f32] = &[F9, G9, GS9, AS9, C10, D10, E10, F10];
pub const FS9_MELODIC_MINOR_SCALE: &[f32] = &[FS9, GS9, A9, B9, CS10, DS10, F10, FS10];
pub const G9_MELODIC_MINOR_SCALE: &[f32] = &[G9, A9, AS9, C10, D10, E10, FS10, G10];
pub const GS9_MELODIC_MINOR_SCALE: &[f32] = &[GS9, AS9, B9, CS10, DS10, F10, G10, GS10];
pub const A9_MELODIC_MINOR_SCALE: &[f32] = &[A9, B9, C10, D10, E10, FS10, GS10, A10];
pub const AS9_MELODIC_MINOR_SCALE: &[f32] = &[AS9, C10, CS10, DS10, F10, G10, A10, AS10];
pub const B9_MELODIC_MINOR_SCALE: &[f32] = &[B9, CS10, D10, E10, FS10, GS10, AS10, B10];

// Octave 10
pub const C10_MELODIC_MINOR_SCALE: &[f32] = &[C10, D10, DS10, F10, G10, A10, B10, C11];
pub const CS10_MELODIC_MINOR_SCALE: &[f32] = &[CS10, DS10, E10, FS10, GS10, AS10, C11, CS11];
pub const D10_MELODIC_MINOR_SCALE: &[f32] = &[D10, E10, F10, G10, A10, B10, CS11, D11];
pub const DS10_MELODIC_MINOR_SCALE: &[f32] = &[DS10, F10, FS10, GS10, AS10, C11, D11, DS11];
pub const E10_MELODIC_MINOR_SCALE: &[f32] = &[E10, FS10, G10, A10, B10, CS11, DS11, E11];
pub const F10_MELODIC_MINOR_SCALE: &[f32] = &[F10, G10, GS10, AS10, C11, D11, E11, F11];
pub const FS10_MELODIC_MINOR_SCALE: &[f32] = &[FS10, GS10, A10, B10, CS11, DS11, F11, FS11];
pub const G10_MELODIC_MINOR_SCALE: &[f32] = &[G10, A10, AS10, C11, D11, E11, FS11, G11];
pub const GS10_MELODIC_MINOR_SCALE: &[f32] = &[GS10, AS10, B10, CS11, DS11, F11, G11, GS11];
pub const A10_MELODIC_MINOR_SCALE: &[f32] = &[A10, B10, C11, D11, E11, FS11, GS11, A11];
pub const AS10_MELODIC_MINOR_SCALE: &[f32] = &[AS10, C11, CS11, DS11, F11, G11, A11, AS11];
pub const B10_MELODIC_MINOR_SCALE: &[f32] = &[B10, CS11, D11, E11, FS11, GS11, AS11, B11];

// ===== HARMONIC_MINOR_SCALE =====
// Octave -1
pub const C_1_HARMONIC_MINOR_SCALE: &[f32] = &[C_1, D_1, DS_1, F_1, G_1, GS_1, B_1, C0];
pub const CS_1_HARMONIC_MINOR_SCALE: &[f32] = &[CS_1, DS_1, E_1, FS_1, GS_1, A_1, C0, CS0];
pub const D_1_HARMONIC_MINOR_SCALE: &[f32] = &[D_1, E_1, F_1, G_1, A_1, AS_1, CS0, D0];
pub const DS_1_HARMONIC_MINOR_SCALE: &[f32] = &[DS_1, F_1, FS_1, GS_1, AS_1, B_1, D0, DS0];
pub const E_1_HARMONIC_MINOR_SCALE: &[f32] = &[E_1, FS_1, G_1, A_1, B_1, C0, DS0, E0];
pub const F_1_HARMONIC_MINOR_SCALE: &[f32] = &[F_1, G_1, GS_1, AS_1, C0, CS0, E0, F0];
pub const FS_1_HARMONIC_MINOR_SCALE: &[f32] = &[FS_1, GS_1, A_1, B_1, CS0, D0, F0, FS0];
pub const G_1_HARMONIC_MINOR_SCALE: &[f32] = &[G_1, A_1, AS_1, C0, D0, DS0, FS0, G0];
pub const GS_1_HARMONIC_MINOR_SCALE: &[f32] = &[GS_1, AS_1, B_1, CS0, DS0, E0, G0, GS0];
pub const A_1_HARMONIC_MINOR_SCALE: &[f32] = &[A_1, B_1, C0, D0, E0, F0, GS0, A0];
pub const AS_1_HARMONIC_MINOR_SCALE: &[f32] = &[AS_1, C0, CS0, DS0, F0, FS0, A0, AS0];
pub const B_1_HARMONIC_MINOR_SCALE: &[f32] = &[B_1, CS0, D0, E0, FS0, G0, AS0, B0];

// Octave 0
pub const C0_HARMONIC_MINOR_SCALE: &[f32] = &[C0, D0, DS0, F0, G0, GS0, B0, C1];
pub const CS0_HARMONIC_MINOR_SCALE: &[f32] = &[CS0, DS0, E0, FS0, GS0, A0, C1, CS1];
pub const D0_HARMONIC_MINOR_SCALE: &[f32] = &[D0, E0, F0, G0, A0, AS0, CS1, D1];
pub const DS0_HARMONIC_MINOR_SCALE: &[f32] = &[DS0, F0, FS0, GS0, AS0, B0, D1, DS1];
pub const E0_HARMONIC_MINOR_SCALE: &[f32] = &[E0, FS0, G0, A0, B0, C1, DS1, E1];
pub const F0_HARMONIC_MINOR_SCALE: &[f32] = &[F0, G0, GS0, AS0, C1, CS1, E1, F1];
pub const FS0_HARMONIC_MINOR_SCALE: &[f32] = &[FS0, GS0, A0, B0, CS1, D1, F1, FS1];
pub const G0_HARMONIC_MINOR_SCALE: &[f32] = &[G0, A0, AS0, C1, D1, DS1, FS1, G1];
pub const GS0_HARMONIC_MINOR_SCALE: &[f32] = &[GS0, AS0, B0, CS1, DS1, E1, G1, GS1];
pub const A0_HARMONIC_MINOR_SCALE: &[f32] = &[A0, B0, C1, D1, E1, F1, GS1, A1];
pub const AS0_HARMONIC_MINOR_SCALE: &[f32] = &[AS0, C1, CS1, DS1, F1, FS1, A1, AS1];
pub const B0_HARMONIC_MINOR_SCALE: &[f32] = &[B0, CS1, D1, E1, FS1, G1, AS1, B1];

// Octave 7
pub const C7_HARMONIC_MINOR_SCALE: &[f32] = &[C7, D7, DS7, F7, G7, GS7, B7, C8];
pub const CS7_HARMONIC_MINOR_SCALE: &[f32] = &[CS7, DS7, E7, FS7, GS7, A7, C8, CS8];
pub const D7_HARMONIC_MINOR_SCALE: &[f32] = &[D7, E7, F7, G7, A7, AS7, CS8, D8];
pub const DS7_HARMONIC_MINOR_SCALE: &[f32] = &[DS7, F7, FS7, GS7, AS7, B7, D8, DS8];
pub const E7_HARMONIC_MINOR_SCALE: &[f32] = &[E7, FS7, G7, A7, B7, C8, DS8, E8];
pub const F7_HARMONIC_MINOR_SCALE: &[f32] = &[F7, G7, GS7, AS7, C8, CS8, E8, F8];
pub const FS7_HARMONIC_MINOR_SCALE: &[f32] = &[FS7, GS7, A7, B7, CS8, D8, F8, FS8];
pub const G7_HARMONIC_MINOR_SCALE: &[f32] = &[G7, A7, AS7, C8, D8, DS8, FS8, G8];
pub const GS7_HARMONIC_MINOR_SCALE: &[f32] = &[GS7, AS7, B7, CS8, DS8, E8, G8, GS8];
pub const A7_HARMONIC_MINOR_SCALE: &[f32] = &[A7, B7, C8, D8, E8, F8, GS8, A8];
pub const AS7_HARMONIC_MINOR_SCALE: &[f32] = &[AS7, C8, CS8, DS8, F8, FS8, A8, AS8];
pub const B7_HARMONIC_MINOR_SCALE: &[f32] = &[B7, CS8, D8, E8, FS8, G8, AS8, B8];

// Octave 8
pub const C8_HARMONIC_MINOR_SCALE: &[f32] = &[C8, D8, DS8, F8, G8, GS8, B8, C9];
pub const CS8_HARMONIC_MINOR_SCALE: &[f32] = &[CS8, DS8, E8, FS8, GS8, A8, C9, CS9];
pub const D8_HARMONIC_MINOR_SCALE: &[f32] = &[D8, E8, F8, G8, A8, AS8, CS9, D9];
pub const DS8_HARMONIC_MINOR_SCALE: &[f32] = &[DS8, F8, FS8, GS8, AS8, B8, D9, DS9];
pub const E8_HARMONIC_MINOR_SCALE: &[f32] = &[E8, FS8, G8, A8, B8, C9, DS9, E9];
pub const F8_HARMONIC_MINOR_SCALE: &[f32] = &[F8, G8, GS8, AS8, C9, CS9, E9, F9];
pub const FS8_HARMONIC_MINOR_SCALE: &[f32] = &[FS8, GS8, A8, B8, CS9, D9, F9, FS9];
pub const G8_HARMONIC_MINOR_SCALE: &[f32] = &[G8, A8, AS8, C9, D9, DS9, FS9, G9];
pub const GS8_HARMONIC_MINOR_SCALE: &[f32] = &[GS8, AS8, B8, CS9, DS9, E9, G9, GS9];
pub const A8_HARMONIC_MINOR_SCALE: &[f32] = &[A8, B8, C9, D9, E9, F9, GS9, A9];
pub const AS8_HARMONIC_MINOR_SCALE: &[f32] = &[AS8, C9, CS9, DS9, F9, FS9, A9, AS9];
pub const B8_HARMONIC_MINOR_SCALE: &[f32] = &[B8, CS9, D9, E9, FS9, G9, AS9, B9];

// Octave 9
pub const C9_HARMONIC_MINOR_SCALE: &[f32] = &[C9, D9, DS9, F9, G9, GS9, B9, C10];
pub const CS9_HARMONIC_MINOR_SCALE: &[f32] = &[CS9, DS9, E9, FS9, GS9, A9, C10, CS10];
pub const D9_HARMONIC_MINOR_SCALE: &[f32] = &[D9, E9, F9, G9, A9, AS9, CS10, D10];
pub const DS9_HARMONIC_MINOR_SCALE: &[f32] = &[DS9, F9, FS9, GS9, AS9, B9, D10, DS10];
pub const E9_HARMONIC_MINOR_SCALE: &[f32] = &[E9, FS9, G9, A9, B9, C10, DS10, E10];
pub const F9_HARMONIC_MINOR_SCALE: &[f32] = &[F9, G9, GS9, AS9, C10, CS10, E10, F10];
pub const FS9_HARMONIC_MINOR_SCALE: &[f32] = &[FS9, GS9, A9, B9, CS10, D10, F10, FS10];
pub const G9_HARMONIC_MINOR_SCALE: &[f32] = &[G9, A9, AS9, C10, D10, DS10, FS10, G10];
pub const GS9_HARMONIC_MINOR_SCALE: &[f32] = &[GS9, AS9, B9, CS10, DS10, E10, G10, GS10];
pub const A9_HARMONIC_MINOR_SCALE: &[f32] = &[A9, B9, C10, D10, E10, F10, GS10, A10];
pub const AS9_HARMONIC_MINOR_SCALE: &[f32] = &[AS9, C10, CS10, DS10, F10, FS10, A10, AS10];
pub const B9_HARMONIC_MINOR_SCALE: &[f32] = &[B9, CS10, D10, E10, FS10, G10, AS10, B10];

// Octave 10
pub const C10_HARMONIC_MINOR_SCALE: &[f32] = &[C10, D10, DS10, F10, G10, GS10, B10, C11];
pub const CS10_HARMONIC_MINOR_SCALE: &[f32] = &[CS10, DS10, E10, FS10, GS10, A10, C11, CS11];
pub const D10_HARMONIC_MINOR_SCALE: &[f32] = &[D10, E10, F10, G10, A10, AS10, CS11, D11];
pub const DS10_HARMONIC_MINOR_SCALE: &[f32] = &[DS10, F10, FS10, GS10, AS10, B10, D11, DS11];
pub const E10_HARMONIC_MINOR_SCALE: &[f32] = &[E10, FS10, G10, A10, B10, C11, DS11, E11];
pub const F10_HARMONIC_MINOR_SCALE: &[f32] = &[F10, G10, GS10, AS10, C11, CS11, E11, F11];
pub const FS10_HARMONIC_MINOR_SCALE: &[f32] = &[FS10, GS10, A10, B10, CS11, D11, F11, FS11];
pub const G10_HARMONIC_MINOR_SCALE: &[f32] = &[G10, A10, AS10, C11, D11, DS11, FS11, G11];
pub const GS10_HARMONIC_MINOR_SCALE: &[f32] = &[GS10, AS10, B10, CS11, DS11, E11, G11, GS11];
pub const A10_HARMONIC_MINOR_SCALE: &[f32] = &[A10, B10, C11, D11, E11, F11, GS11, A11];
pub const AS10_HARMONIC_MINOR_SCALE: &[f32] = &[AS10, C11, CS11, DS11, F11, FS11, A11, AS11];
pub const B10_HARMONIC_MINOR_SCALE: &[f32] = &[B10, CS11, D11, E11, FS11, G11, AS11, B11];

// ===== CHROMATIC SCALES =====
pub const C_1_CHROMATIC_SCALE: &[f32] = &[C_1, CS_1, D_1, DS_1, E_1, F_1, FS_1, G_1, GS_1, A_1, AS_1, B_1, C0];
pub const C0_CHROMATIC_SCALE: &[f32] = &[C0, CS0, D0, DS0, E0, F0, FS0, G0, GS0, A0, AS0, B0, C1];
pub const C7_CHROMATIC_SCALE: &[f32] = &[C7, CS7, D7, DS7, E7, F7, FS7, G7, GS7, A7, AS7, B7, C8];
pub const C8_CHROMATIC_SCALE: &[f32] = &[C8, CS8, D8, DS8, E8, F8, FS8, G8, GS8, A8, AS8, B8, C9];
pub const C9_CHROMATIC_SCALE: &[f32] = &[C9, CS9, D9, DS9, E9, F9, FS9, G9, GS9, A9, AS9, B9, C10];
pub const C10_CHROMATIC_SCALE: &[f32] = &[C10, CS10, D10, DS10, E10, F10, FS10, G10, GS10, A10, AS10, B10, C11];
