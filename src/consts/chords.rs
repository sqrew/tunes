#![allow(dead_code)]

use super::notes::*;

// ===== MAJOR TRIADS =====
// Octave -1
pub const C_1_MAJOR: &[f32] = &[C_1, E_1, G_1];
pub const CS_1_MAJOR: &[f32] = &[CS_1, F_1, GS_1];
pub const D_1_MAJOR: &[f32] = &[D_1, FS_1, A_1];
pub const DS_1_MAJOR: &[f32] = &[DS_1, G_1, AS_1];
pub const E_1_MAJOR: &[f32] = &[E_1, GS_1, B_1];
pub const F_1_MAJOR: &[f32] = &[F_1, A_1, C0];
pub const FS_1_MAJOR: &[f32] = &[FS_1, AS_1, CS0];
pub const G_1_MAJOR: &[f32] = &[G_1, B_1, D0];
pub const GS_1_MAJOR: &[f32] = &[GS_1, C0, DS0];
pub const A_1_MAJOR: &[f32] = &[A_1, CS0, E0];
pub const AS_1_MAJOR: &[f32] = &[AS_1, D0, F0];
pub const B_1_MAJOR: &[f32] = &[B_1, DS0, FS0];

// Octave 0
pub const C0_MAJOR: &[f32] = &[C0, E0, G0];
pub const CS0_MAJOR: &[f32] = &[CS0, F0, GS0];
pub const D0_MAJOR: &[f32] = &[D0, FS0, A0];
pub const DS0_MAJOR: &[f32] = &[DS0, G0, AS0];
pub const E0_MAJOR: &[f32] = &[E0, GS0, B0];
pub const F0_MAJOR: &[f32] = &[F0, A0, C1];
pub const FS0_MAJOR: &[f32] = &[FS0, AS0, CS1];
pub const G0_MAJOR: &[f32] = &[G0, B0, D1];
pub const GS0_MAJOR: &[f32] = &[GS0, C1, DS1];
pub const A0_MAJOR: &[f32] = &[A0, CS1, E1];
pub const AS0_MAJOR: &[f32] = &[AS0, D1, F1];
pub const B0_MAJOR: &[f32] = &[B0, DS1, FS1];

// Octave 1
pub const C1_MAJOR: &[f32] = &[C1, E1, G1];
pub const CS1_MAJOR: &[f32] = &[CS1, F1, GS1];
pub const D1_MAJOR: &[f32] = &[D1, FS1, A1];
pub const DS1_MAJOR: &[f32] = &[DS1, G1, AS1];
pub const E1_MAJOR: &[f32] = &[E1, GS1, B1];
pub const F1_MAJOR: &[f32] = &[F1, A1, C2];
pub const FS1_MAJOR: &[f32] = &[FS1, AS1, CS2];
pub const G1_MAJOR: &[f32] = &[G1, B1, D2];
pub const GS1_MAJOR: &[f32] = &[GS1, C2, DS2];
pub const A1_MAJOR: &[f32] = &[A1, CS2, E2];
pub const AS1_MAJOR: &[f32] = &[AS1, D2, F2];
pub const B1_MAJOR: &[f32] = &[B1, DS2, FS2];

// Octave 2
pub const C2_MAJOR: &[f32] = &[C2, E2, G2];
pub const CS2_MAJOR: &[f32] = &[CS2, F2, GS2];
pub const D2_MAJOR: &[f32] = &[D2, FS2, A2];
pub const DS2_MAJOR: &[f32] = &[DS2, G2, AS2];
pub const E2_MAJOR: &[f32] = &[E2, GS2, B2];
pub const F2_MAJOR: &[f32] = &[F2, A2, C3];
pub const FS2_MAJOR: &[f32] = &[FS2, AS2, CS3];
pub const G2_MAJOR: &[f32] = &[G2, B2, D3];
pub const GS2_MAJOR: &[f32] = &[GS2, C3, DS3];
pub const A2_MAJOR: &[f32] = &[A2, CS3, E3];
pub const AS2_MAJOR: &[f32] = &[AS2, D3, F3];
pub const B2_MAJOR: &[f32] = &[B2, DS3, FS3];

// Octave 3
pub const C3_MAJOR: &[f32] = &[C3, E3, G3];
pub const CS3_MAJOR: &[f32] = &[CS3, F3, GS3];
pub const D3_MAJOR: &[f32] = &[D3, FS3, A3];
pub const DS3_MAJOR: &[f32] = &[DS3, G3, AS3];
pub const E3_MAJOR: &[f32] = &[E3, GS3, B3];
pub const F3_MAJOR: &[f32] = &[F3, A3, C4];
pub const FS3_MAJOR: &[f32] = &[FS3, AS3, CS4];
pub const G3_MAJOR: &[f32] = &[G3, B3, D4];
pub const GS3_MAJOR: &[f32] = &[GS3, C4, DS4];
pub const A3_MAJOR: &[f32] = &[A3, CS4, E4];
pub const AS3_MAJOR: &[f32] = &[AS3, D4, F4];
pub const B3_MAJOR: &[f32] = &[B3, DS4, FS4];

// Octave 4
pub const C4_MAJOR: &[f32] = &[C4, E4, G4];
pub const CS4_MAJOR: &[f32] = &[CS4, F4, GS4];
pub const D4_MAJOR: &[f32] = &[D4, FS4, A4];
pub const DS4_MAJOR: &[f32] = &[DS4, G4, AS4];
pub const E4_MAJOR: &[f32] = &[E4, GS4, B4];
pub const F4_MAJOR: &[f32] = &[F4, A4, C5];
pub const FS4_MAJOR: &[f32] = &[FS4, AS4, CS5];
pub const G4_MAJOR: &[f32] = &[G4, B4, D5];
pub const GS4_MAJOR: &[f32] = &[GS4, C5, DS5];
pub const A4_MAJOR: &[f32] = &[A4, CS5, E5];
pub const AS4_MAJOR: &[f32] = &[AS4, D5, F5];
pub const B4_MAJOR: &[f32] = &[B4, DS5, FS5];

// Octave 5
pub const C5_MAJOR: &[f32] = &[C5, E5, G5];
pub const CS5_MAJOR: &[f32] = &[CS5, F5, GS5];
pub const D5_MAJOR: &[f32] = &[D5, FS5, A5];
pub const DS5_MAJOR: &[f32] = &[DS5, G5, AS5];
pub const E5_MAJOR: &[f32] = &[E5, GS5, B5];
pub const F5_MAJOR: &[f32] = &[F5, A5, C6];
pub const FS5_MAJOR: &[f32] = &[FS5, AS5, CS6];
pub const G5_MAJOR: &[f32] = &[G5, B5, D6];
pub const GS5_MAJOR: &[f32] = &[GS5, C6, DS6];
pub const A5_MAJOR: &[f32] = &[A5, CS6, E6];
pub const AS5_MAJOR: &[f32] = &[AS5, D6, F6];
pub const B5_MAJOR: &[f32] = &[B5, DS6, FS6];

// Octave 6
pub const C6_MAJOR: &[f32] = &[C6, E6, G6];
pub const CS6_MAJOR: &[f32] = &[CS6, F6, GS6];
pub const D6_MAJOR: &[f32] = &[D6, FS6, A6];
pub const DS6_MAJOR: &[f32] = &[DS6, G6, AS6];
pub const E6_MAJOR: &[f32] = &[E6, GS6, B6];
pub const F6_MAJOR: &[f32] = &[F6, A6, C7];
pub const FS6_MAJOR: &[f32] = &[FS6, AS6, CS7];
pub const G6_MAJOR: &[f32] = &[G6, B6, D7];
pub const GS6_MAJOR: &[f32] = &[GS6, C7, DS7];
pub const A6_MAJOR: &[f32] = &[A6, CS7, E7];
pub const AS6_MAJOR: &[f32] = &[AS6, D7, F7];
pub const B6_MAJOR: &[f32] = &[B6, DS7, FS7];

// Octave 7
pub const C7_MAJOR: &[f32] = &[C7, E7, G7];
pub const CS7_MAJOR: &[f32] = &[CS7, F7, GS7];
pub const D7_MAJOR: &[f32] = &[D7, FS7, A7];
pub const DS7_MAJOR: &[f32] = &[DS7, G7, AS7];
pub const E7_MAJOR: &[f32] = &[E7, GS7, B7];
pub const F7_MAJOR: &[f32] = &[F7, A7, C8];
pub const FS7_MAJOR: &[f32] = &[FS7, AS7, CS8];
pub const G7_MAJOR: &[f32] = &[G7, B7, D8];
pub const GS7_MAJOR: &[f32] = &[GS7, C8, DS8];
pub const A7_MAJOR: &[f32] = &[A7, CS8, E8];
pub const AS7_MAJOR: &[f32] = &[AS7, D8, F8];
pub const B7_MAJOR: &[f32] = &[B7, DS8, FS8];

// Octave 8
pub const C8_MAJOR: &[f32] = &[C8, E8, G8];
pub const CS8_MAJOR: &[f32] = &[CS8, F8, GS8];
pub const D8_MAJOR: &[f32] = &[D8, FS8, A8];
pub const DS8_MAJOR: &[f32] = &[DS8, G8, AS8];
pub const E8_MAJOR: &[f32] = &[E8, GS8, B8];
pub const F8_MAJOR: &[f32] = &[F8, A8, C9];
pub const FS8_MAJOR: &[f32] = &[FS8, AS8, CS9];
pub const G8_MAJOR: &[f32] = &[G8, B8, D9];
pub const GS8_MAJOR: &[f32] = &[GS8, C9, DS9];
pub const A8_MAJOR: &[f32] = &[A8, CS9, E9];
pub const AS8_MAJOR: &[f32] = &[AS8, D9, F9];
pub const B8_MAJOR: &[f32] = &[B8, DS9, FS9];

// Octave 9
pub const C9_MAJOR: &[f32] = &[C9, E9, G9];
pub const CS9_MAJOR: &[f32] = &[CS9, F9, GS9];
pub const D9_MAJOR: &[f32] = &[D9, FS9, A9];
pub const DS9_MAJOR: &[f32] = &[DS9, G9, AS9];
pub const E9_MAJOR: &[f32] = &[E9, GS9, B9];
pub const F9_MAJOR: &[f32] = &[F9, A9, C10];
pub const FS9_MAJOR: &[f32] = &[FS9, AS9, CS10];
pub const G9_MAJOR: &[f32] = &[G9, B9, D10];
pub const GS9_MAJOR: &[f32] = &[GS9, C10, DS10];
pub const A9_MAJOR: &[f32] = &[A9, CS10, E10];
pub const AS9_MAJOR: &[f32] = &[AS9, D10, F10];
pub const B9_MAJOR: &[f32] = &[B9, DS10, FS10];

// Octave 10
pub const C10_MAJOR: &[f32] = &[C10, E10, G10];
pub const CS10_MAJOR: &[f32] = &[CS10, F10, GS10];
pub const D10_MAJOR: &[f32] = &[D10, FS10, A10];
pub const DS10_MAJOR: &[f32] = &[DS10, G10, AS10];
pub const E10_MAJOR: &[f32] = &[E10, GS10, B10];
pub const F10_MAJOR: &[f32] = &[F10, A10, C11];
pub const FS10_MAJOR: &[f32] = &[FS10, AS10, CS11];
pub const G10_MAJOR: &[f32] = &[G10, B10, D11];
pub const GS10_MAJOR: &[f32] = &[GS10, C11, DS11];
pub const A10_MAJOR: &[f32] = &[A10, CS11, E11];
pub const AS10_MAJOR: &[f32] = &[AS10, D11, F11];
pub const B10_MAJOR: &[f32] = &[B10, DS11, FS11];

// ===== MINOR TRIADS =====
// Octave -1
pub const C_1_MINOR: &[f32] = &[C_1, DS_1, G_1];
pub const CS_1_MINOR: &[f32] = &[CS_1, E_1, GS_1];
pub const D_1_MINOR: &[f32] = &[D_1, F_1, A_1];
pub const DS_1_MINOR: &[f32] = &[DS_1, FS_1, AS_1];
pub const E_1_MINOR: &[f32] = &[E_1, G_1, B_1];
pub const F_1_MINOR: &[f32] = &[F_1, GS_1, C0];
pub const FS_1_MINOR: &[f32] = &[FS_1, A_1, CS0];
pub const G_1_MINOR: &[f32] = &[G_1, AS_1, D0];
pub const GS_1_MINOR: &[f32] = &[GS_1, B_1, DS0];
pub const A_1_MINOR: &[f32] = &[A_1, C0, E0];
pub const AS_1_MINOR: &[f32] = &[AS_1, CS0, F0];
pub const B_1_MINOR: &[f32] = &[B_1, D0, FS0];

// Octave 0
pub const C0_MINOR: &[f32] = &[C0, DS0, G0];
pub const CS0_MINOR: &[f32] = &[CS0, E0, GS0];
pub const D0_MINOR: &[f32] = &[D0, F0, A0];
pub const DS0_MINOR: &[f32] = &[DS0, FS0, AS0];
pub const E0_MINOR: &[f32] = &[E0, G0, B0];
pub const F0_MINOR: &[f32] = &[F0, GS0, C1];
pub const FS0_MINOR: &[f32] = &[FS0, A0, CS1];
pub const G0_MINOR: &[f32] = &[G0, AS0, D1];
pub const GS0_MINOR: &[f32] = &[GS0, B0, DS1];
pub const A0_MINOR: &[f32] = &[A0, C1, E1];
pub const AS0_MINOR: &[f32] = &[AS0, CS1, F1];
pub const B0_MINOR: &[f32] = &[B0, D1, FS1];

// Octave 1
pub const C1_MINOR: &[f32] = &[C1, DS1, G1];
pub const CS1_MINOR: &[f32] = &[CS1, E1, GS1];
pub const D1_MINOR: &[f32] = &[D1, F1, A1];
pub const DS1_MINOR: &[f32] = &[DS1, FS1, AS1];
pub const E1_MINOR: &[f32] = &[E1, G1, B1];
pub const F1_MINOR: &[f32] = &[F1, GS1, C2];
pub const FS1_MINOR: &[f32] = &[FS1, A1, CS2];
pub const G1_MINOR: &[f32] = &[G1, AS1, D2];
pub const GS1_MINOR: &[f32] = &[GS1, B1, DS2];
pub const A1_MINOR: &[f32] = &[A1, C2, E2];
pub const AS1_MINOR: &[f32] = &[AS1, CS2, F2];
pub const B1_MINOR: &[f32] = &[B1, D2, FS2];

// Octave 2
pub const C2_MINOR: &[f32] = &[C2, DS2, G2];
pub const CS2_MINOR: &[f32] = &[CS2, E2, GS2];
pub const D2_MINOR: &[f32] = &[D2, F2, A2];
pub const DS2_MINOR: &[f32] = &[DS2, FS2, AS2];
pub const E2_MINOR: &[f32] = &[E2, G2, B2];
pub const F2_MINOR: &[f32] = &[F2, GS2, C3];
pub const FS2_MINOR: &[f32] = &[FS2, A2, CS3];
pub const G2_MINOR: &[f32] = &[G2, AS2, D3];
pub const GS2_MINOR: &[f32] = &[GS2, B2, DS3];
pub const A2_MINOR: &[f32] = &[A2, C3, E3];
pub const AS2_MINOR: &[f32] = &[AS2, CS3, F3];
pub const B2_MINOR: &[f32] = &[B2, D3, FS3];

// Octave 3
pub const C3_MINOR: &[f32] = &[C3, DS3, G3];
pub const CS3_MINOR: &[f32] = &[CS3, E3, GS3];
pub const D3_MINOR: &[f32] = &[D3, F3, A3];
pub const DS3_MINOR: &[f32] = &[DS3, FS3, AS3];
pub const E3_MINOR: &[f32] = &[E3, G3, B3];
pub const F3_MINOR: &[f32] = &[F3, GS3, C4];
pub const FS3_MINOR: &[f32] = &[FS3, A3, CS4];
pub const G3_MINOR: &[f32] = &[G3, AS3, D4];
pub const GS3_MINOR: &[f32] = &[GS3, B3, DS4];
pub const A3_MINOR: &[f32] = &[A3, C4, E4];
pub const AS3_MINOR: &[f32] = &[AS3, CS4, F4];
pub const B3_MINOR: &[f32] = &[B3, D4, FS4];

// Octave 4
pub const C4_MINOR: &[f32] = &[C4, DS4, G4];
pub const CS4_MINOR: &[f32] = &[CS4, E4, GS4];
pub const D4_MINOR: &[f32] = &[D4, F4, A4];
pub const DS4_MINOR: &[f32] = &[DS4, FS4, AS4];
pub const E4_MINOR: &[f32] = &[E4, G4, B4];
pub const F4_MINOR: &[f32] = &[F4, GS4, C5];
pub const FS4_MINOR: &[f32] = &[FS4, A4, CS5];
pub const G4_MINOR: &[f32] = &[G4, AS4, D5];
pub const GS4_MINOR: &[f32] = &[GS4, B4, DS5];
pub const A4_MINOR: &[f32] = &[A4, C5, E5];
pub const AS4_MINOR: &[f32] = &[AS4, CS5, F5];
pub const B4_MINOR: &[f32] = &[B4, D5, FS5];

// Octave 5
pub const C5_MINOR: &[f32] = &[C5, DS5, G5];
pub const CS5_MINOR: &[f32] = &[CS5, E5, GS5];
pub const D5_MINOR: &[f32] = &[D5, F5, A5];
pub const DS5_MINOR: &[f32] = &[DS5, FS5, AS5];
pub const E5_MINOR: &[f32] = &[E5, G5, B5];
pub const F5_MINOR: &[f32] = &[F5, GS5, C6];
pub const FS5_MINOR: &[f32] = &[FS5, A5, CS6];
pub const G5_MINOR: &[f32] = &[G5, AS5, D6];
pub const GS5_MINOR: &[f32] = &[GS5, B5, DS6];
pub const A5_MINOR: &[f32] = &[A5, C6, E6];
pub const AS5_MINOR: &[f32] = &[AS5, CS6, F6];
pub const B5_MINOR: &[f32] = &[B5, D6, FS6];

// Octave 6
pub const C6_MINOR: &[f32] = &[C6, DS6, G6];
pub const CS6_MINOR: &[f32] = &[CS6, E6, GS6];
pub const D6_MINOR: &[f32] = &[D6, F6, A6];
pub const DS6_MINOR: &[f32] = &[DS6, FS6, AS6];
pub const E6_MINOR: &[f32] = &[E6, G6, B6];
pub const F6_MINOR: &[f32] = &[F6, GS6, C7];
pub const FS6_MINOR: &[f32] = &[FS6, A6, CS7];
pub const G6_MINOR: &[f32] = &[G6, AS6, D7];
pub const GS6_MINOR: &[f32] = &[GS6, B6, DS7];
pub const A6_MINOR: &[f32] = &[A6, C7, E7];
pub const AS6_MINOR: &[f32] = &[AS6, CS7, F7];
pub const B6_MINOR: &[f32] = &[B6, D7, FS7];

// Octave 7
pub const C7_MINOR: &[f32] = &[C7, DS7, G7];
pub const CS7_MINOR: &[f32] = &[CS7, E7, GS7];
pub const D7_MINOR: &[f32] = &[D7, F7, A7];
pub const DS7_MINOR: &[f32] = &[DS7, FS7, AS7];
pub const E7_MINOR: &[f32] = &[E7, G7, B7];
pub const F7_MINOR: &[f32] = &[F7, GS7, C8];
pub const FS7_MINOR: &[f32] = &[FS7, A7, CS8];
pub const G7_MINOR: &[f32] = &[G7, AS7, D8];
pub const GS7_MINOR: &[f32] = &[GS7, B7, DS8];
pub const A7_MINOR: &[f32] = &[A7, C8, E8];
pub const AS7_MINOR: &[f32] = &[AS7, CS8, F8];
pub const B7_MINOR: &[f32] = &[B7, D8, FS8];

// Octave 8
pub const C8_MINOR: &[f32] = &[C8, DS8, G8];
pub const CS8_MINOR: &[f32] = &[CS8, E8, GS8];
pub const D8_MINOR: &[f32] = &[D8, F8, A8];
pub const DS8_MINOR: &[f32] = &[DS8, FS8, AS8];
pub const E8_MINOR: &[f32] = &[E8, G8, B8];
pub const F8_MINOR: &[f32] = &[F8, GS8, C9];
pub const FS8_MINOR: &[f32] = &[FS8, A8, CS9];
pub const G8_MINOR: &[f32] = &[G8, AS8, D9];
pub const GS8_MINOR: &[f32] = &[GS8, B8, DS9];
pub const A8_MINOR: &[f32] = &[A8, C9, E9];
pub const AS8_MINOR: &[f32] = &[AS8, CS9, F9];
pub const B8_MINOR: &[f32] = &[B8, D9, FS9];

// Octave 9
pub const C9_MINOR: &[f32] = &[C9, DS9, G9];
pub const CS9_MINOR: &[f32] = &[CS9, E9, GS9];
pub const D9_MINOR: &[f32] = &[D9, F9, A9];
pub const DS9_MINOR: &[f32] = &[DS9, FS9, AS9];
pub const E9_MINOR: &[f32] = &[E9, G9, B9];
pub const F9_MINOR: &[f32] = &[F9, GS9, C10];
pub const FS9_MINOR: &[f32] = &[FS9, A9, CS10];
pub const G9_MINOR: &[f32] = &[G9, AS9, D10];
pub const GS9_MINOR: &[f32] = &[GS9, B9, DS10];
pub const A9_MINOR: &[f32] = &[A9, C10, E10];
pub const AS9_MINOR: &[f32] = &[AS9, CS10, F10];
pub const B9_MINOR: &[f32] = &[B9, D10, FS10];

// Octave 10
pub const C10_MINOR: &[f32] = &[C10, DS10, G10];
pub const CS10_MINOR: &[f32] = &[CS10, E10, GS10];
pub const D10_MINOR: &[f32] = &[D10, F10, A10];
pub const DS10_MINOR: &[f32] = &[DS10, FS10, AS10];
pub const E10_MINOR: &[f32] = &[E10, G10, B10];
pub const F10_MINOR: &[f32] = &[F10, GS10, C11];
pub const FS10_MINOR: &[f32] = &[FS10, A10, CS11];
pub const G10_MINOR: &[f32] = &[G10, AS10, D11];
pub const GS10_MINOR: &[f32] = &[GS10, B10, DS11];
pub const A10_MINOR: &[f32] = &[A10, C11, E11];
pub const AS10_MINOR: &[f32] = &[AS10, CS11, F11];
pub const B10_MINOR: &[f32] = &[B10, D11, FS11];

// ===== POWER CHORDS =====
// Octave -1
pub const C_1_POWER: &[f32] = &[C_1, G_1];
pub const CS_1_POWER: &[f32] = &[CS_1, GS_1];
pub const D_1_POWER: &[f32] = &[D_1, A_1];
pub const DS_1_POWER: &[f32] = &[DS_1, AS_1];
pub const E_1_POWER: &[f32] = &[E_1, B_1];
pub const F_1_POWER: &[f32] = &[F_1, C0];
pub const FS_1_POWER: &[f32] = &[FS_1, CS0];
pub const G_1_POWER: &[f32] = &[G_1, D0];
pub const GS_1_POWER: &[f32] = &[GS_1, DS0];
pub const A_1_POWER: &[f32] = &[A_1, E0];
pub const AS_1_POWER: &[f32] = &[AS_1, F0];
pub const B_1_POWER: &[f32] = &[B_1, FS0];

// Octave 0
pub const C0_POWER: &[f32] = &[C0, G0];
pub const CS0_POWER: &[f32] = &[CS0, GS0];
pub const D0_POWER: &[f32] = &[D0, A0];
pub const DS0_POWER: &[f32] = &[DS0, AS0];
pub const E0_POWER: &[f32] = &[E0, B0];
pub const F0_POWER: &[f32] = &[F0, C1];
pub const FS0_POWER: &[f32] = &[FS0, CS1];
pub const G0_POWER: &[f32] = &[G0, D1];
pub const GS0_POWER: &[f32] = &[GS0, DS1];
pub const A0_POWER: &[f32] = &[A0, E1];
pub const AS0_POWER: &[f32] = &[AS0, F1];
pub const B0_POWER: &[f32] = &[B0, FS1];

// Octave 1
pub const C1_POWER: &[f32] = &[C1, G1];
pub const CS1_POWER: &[f32] = &[CS1, GS1];
pub const D1_POWER: &[f32] = &[D1, A1];
pub const DS1_POWER: &[f32] = &[DS1, AS1];
pub const E1_POWER: &[f32] = &[E1, B1];
pub const F1_POWER: &[f32] = &[F1, C2];
pub const FS1_POWER: &[f32] = &[FS1, CS2];
pub const G1_POWER: &[f32] = &[G1, D2];
pub const GS1_POWER: &[f32] = &[GS1, DS2];
pub const A1_POWER: &[f32] = &[A1, E2];
pub const AS1_POWER: &[f32] = &[AS1, F2];
pub const B1_POWER: &[f32] = &[B1, FS2];

// Octave 2
pub const C2_POWER: &[f32] = &[C2, G2];
pub const CS2_POWER: &[f32] = &[CS2, GS2];
pub const D2_POWER: &[f32] = &[D2, A2];
pub const DS2_POWER: &[f32] = &[DS2, AS2];
pub const E2_POWER: &[f32] = &[E2, B2];
pub const F2_POWER: &[f32] = &[F2, C3];
pub const FS2_POWER: &[f32] = &[FS2, CS3];
pub const G2_POWER: &[f32] = &[G2, D3];
pub const GS2_POWER: &[f32] = &[GS2, DS3];
pub const A2_POWER: &[f32] = &[A2, E3];
pub const AS2_POWER: &[f32] = &[AS2, F3];
pub const B2_POWER: &[f32] = &[B2, FS3];

// Octave 3
pub const C3_POWER: &[f32] = &[C3, G3];
pub const CS3_POWER: &[f32] = &[CS3, GS3];
pub const D3_POWER: &[f32] = &[D3, A3];
pub const DS3_POWER: &[f32] = &[DS3, AS3];
pub const E3_POWER: &[f32] = &[E3, B3];
pub const F3_POWER: &[f32] = &[F3, C4];
pub const FS3_POWER: &[f32] = &[FS3, CS4];
pub const G3_POWER: &[f32] = &[G3, D4];
pub const GS3_POWER: &[f32] = &[GS3, DS4];
pub const A3_POWER: &[f32] = &[A3, E4];
pub const AS3_POWER: &[f32] = &[AS3, F4];
pub const B3_POWER: &[f32] = &[B3, FS4];

// Octave 4
pub const C4_POWER: &[f32] = &[C4, G4];
pub const CS4_POWER: &[f32] = &[CS4, GS4];
pub const D4_POWER: &[f32] = &[D4, A4];
pub const DS4_POWER: &[f32] = &[DS4, AS4];
pub const E4_POWER: &[f32] = &[E4, B4];
pub const F4_POWER: &[f32] = &[F4, C5];
pub const FS4_POWER: &[f32] = &[FS4, CS5];
pub const G4_POWER: &[f32] = &[G4, D5];
pub const GS4_POWER: &[f32] = &[GS4, DS5];
pub const A4_POWER: &[f32] = &[A4, E5];
pub const AS4_POWER: &[f32] = &[AS4, F5];
pub const B4_POWER: &[f32] = &[B4, FS5];

// Octave 5
pub const C5_POWER: &[f32] = &[C5, G5];
pub const CS5_POWER: &[f32] = &[CS5, GS5];
pub const D5_POWER: &[f32] = &[D5, A5];
pub const DS5_POWER: &[f32] = &[DS5, AS5];
pub const E5_POWER: &[f32] = &[E5, B5];
pub const F5_POWER: &[f32] = &[F5, C6];
pub const FS5_POWER: &[f32] = &[FS5, CS6];
pub const G5_POWER: &[f32] = &[G5, D6];
pub const GS5_POWER: &[f32] = &[GS5, DS6];
pub const A5_POWER: &[f32] = &[A5, E6];
pub const AS5_POWER: &[f32] = &[AS5, F6];
pub const B5_POWER: &[f32] = &[B5, FS6];

// Octave 6
pub const C6_POWER: &[f32] = &[C6, G6];
pub const CS6_POWER: &[f32] = &[CS6, GS6];
pub const D6_POWER: &[f32] = &[D6, A6];
pub const DS6_POWER: &[f32] = &[DS6, AS6];
pub const E6_POWER: &[f32] = &[E6, B6];
pub const F6_POWER: &[f32] = &[F6, C7];
pub const FS6_POWER: &[f32] = &[FS6, CS7];
pub const G6_POWER: &[f32] = &[G6, D7];
pub const GS6_POWER: &[f32] = &[GS6, DS7];
pub const A6_POWER: &[f32] = &[A6, E7];
pub const AS6_POWER: &[f32] = &[AS6, F7];
pub const B6_POWER: &[f32] = &[B6, FS7];

// Octave 7
pub const C7_POWER: &[f32] = &[C7, G7];
pub const CS7_POWER: &[f32] = &[CS7, GS7];
pub const D7_POWER: &[f32] = &[D7, A7];
pub const DS7_POWER: &[f32] = &[DS7, AS7];
pub const E7_POWER: &[f32] = &[E7, B7];
pub const F7_POWER: &[f32] = &[F7, C8];
pub const FS7_POWER: &[f32] = &[FS7, CS8];
pub const G7_POWER: &[f32] = &[G7, D8];
pub const GS7_POWER: &[f32] = &[GS7, DS8];
pub const A7_POWER: &[f32] = &[A7, E8];
pub const AS7_POWER: &[f32] = &[AS7, F8];
pub const B7_POWER: &[f32] = &[B7, FS8];

// Octave 8
pub const C8_POWER: &[f32] = &[C8, G8];
pub const CS8_POWER: &[f32] = &[CS8, GS8];
pub const D8_POWER: &[f32] = &[D8, A8];
pub const DS8_POWER: &[f32] = &[DS8, AS8];
pub const E8_POWER: &[f32] = &[E8, B8];
pub const F8_POWER: &[f32] = &[F8, C9];
pub const FS8_POWER: &[f32] = &[FS8, CS9];
pub const G8_POWER: &[f32] = &[G8, D9];
pub const GS8_POWER: &[f32] = &[GS8, DS9];
pub const A8_POWER: &[f32] = &[A8, E9];
pub const AS8_POWER: &[f32] = &[AS8, F9];
pub const B8_POWER: &[f32] = &[B8, FS9];

// Octave 9
pub const C9_POWER: &[f32] = &[C9, G9];
pub const CS9_POWER: &[f32] = &[CS9, GS9];
pub const D9_POWER: &[f32] = &[D9, A9];
pub const DS9_POWER: &[f32] = &[DS9, AS9];
pub const E9_POWER: &[f32] = &[E9, B9];
pub const F9_POWER: &[f32] = &[F9, C10];
pub const FS9_POWER: &[f32] = &[FS9, CS10];
pub const G9_POWER: &[f32] = &[G9, D10];
pub const GS9_POWER: &[f32] = &[GS9, DS10];
pub const A9_POWER: &[f32] = &[A9, E10];
pub const AS9_POWER: &[f32] = &[AS9, F10];
pub const B9_POWER: &[f32] = &[B9, FS10];

// Octave 10
pub const C10_POWER: &[f32] = &[C10, G10];
pub const CS10_POWER: &[f32] = &[CS10, GS10];
pub const D10_POWER: &[f32] = &[D10, A10];
pub const DS10_POWER: &[f32] = &[DS10, AS10];
pub const E10_POWER: &[f32] = &[E10, B10];
pub const F10_POWER: &[f32] = &[F10, C11];
pub const FS10_POWER: &[f32] = &[FS10, CS11];
pub const G10_POWER: &[f32] = &[G10, D11];
pub const GS10_POWER: &[f32] = &[GS10, DS11];
pub const A10_POWER: &[f32] = &[A10, E11];
pub const AS10_POWER: &[f32] = &[AS10, F11];
pub const B10_POWER: &[f32] = &[B10, FS11];

// ===== SEVENTH CHORDS =====
// Octave 2
pub const C2_MAJOR7: &[f32] = &[C2, E2, G2, B2];
pub const C2_MINOR7: &[f32] = &[C2, DS2, G2, AS2];
pub const C2_DOMINANT7: &[f32] = &[C2, E2, G2, AS2];
pub const CS2_MAJOR7: &[f32] = &[CS2, F2, GS2, C3];
pub const CS2_MINOR7: &[f32] = &[CS2, E2, GS2, B2];
pub const CS2_DOMINANT7: &[f32] = &[CS2, F2, GS2, B2];
pub const D2_MAJOR7: &[f32] = &[D2, FS2, A2, CS3];
pub const D2_MINOR7: &[f32] = &[D2, F2, A2, C3];
pub const D2_DOMINANT7: &[f32] = &[D2, FS2, A2, C3];
pub const DS2_MAJOR7: &[f32] = &[DS2, G2, AS2, D3];
pub const DS2_MINOR7: &[f32] = &[DS2, FS2, AS2, CS3];
pub const DS2_DOMINANT7: &[f32] = &[DS2, G2, AS2, CS3];
pub const E2_MAJOR7: &[f32] = &[E2, GS2, B2, DS3];
pub const E2_MINOR7: &[f32] = &[E2, G2, B2, D3];
pub const E2_DOMINANT7: &[f32] = &[E2, GS2, B2, D3];
pub const F2_MAJOR7: &[f32] = &[F2, A2, C3, E3];
pub const F2_MINOR7: &[f32] = &[F2, GS2, C3, DS3];
pub const F2_DOMINANT7: &[f32] = &[F2, A2, C3, DS3];
pub const FS2_MAJOR7: &[f32] = &[FS2, AS2, CS3, F3];
pub const FS2_MINOR7: &[f32] = &[FS2, A2, CS3, E3];
pub const FS2_DOMINANT7: &[f32] = &[FS2, AS2, CS3, E3];
pub const G2_MAJOR7: &[f32] = &[G2, B2, D3, FS3];
pub const G2_MINOR7: &[f32] = &[G2, AS2, D3, F3];
pub const G2_DOMINANT7: &[f32] = &[G2, B2, D3, F3];
pub const GS2_MAJOR7: &[f32] = &[GS2, C3, DS3, G3];
pub const GS2_MINOR7: &[f32] = &[GS2, B2, DS3, FS3];
pub const GS2_DOMINANT7: &[f32] = &[GS2, C3, DS3, FS3];
pub const A2_MAJOR7: &[f32] = &[A2, CS3, E3, GS3];
pub const A2_MINOR7: &[f32] = &[A2, C3, E3, G3];
pub const A2_DOMINANT7: &[f32] = &[A2, CS3, E3, G3];
pub const AS2_MAJOR7: &[f32] = &[AS2, D3, F3, A3];
pub const AS2_MINOR7: &[f32] = &[AS2, CS3, F3, GS3];
pub const AS2_DOMINANT7: &[f32] = &[AS2, D3, F3, GS3];
pub const B2_MAJOR7: &[f32] = &[B2, DS3, FS3, AS3];
pub const B2_MINOR7: &[f32] = &[B2, D3, FS3, A3];
pub const B2_DOMINANT7: &[f32] = &[B2, DS3, FS3, A3];

// Octave 3
pub const C3_MAJOR7: &[f32] = &[C3, E3, G3, B3];
pub const C3_MINOR7: &[f32] = &[C3, DS3, G3, AS3];
pub const C3_DOMINANT7: &[f32] = &[C3, E3, G3, AS3];
pub const CS3_MAJOR7: &[f32] = &[CS3, F3, GS3, C4];
pub const CS3_MINOR7: &[f32] = &[CS3, E3, GS3, B3];
pub const CS3_DOMINANT7: &[f32] = &[CS3, F3, GS3, B3];
pub const D3_MAJOR7: &[f32] = &[D3, FS3, A3, CS4];
pub const D3_MINOR7: &[f32] = &[D3, F3, A3, C4];
pub const D3_DOMINANT7: &[f32] = &[D3, FS3, A3, C4];
pub const DS3_MAJOR7: &[f32] = &[DS3, G3, AS3, D4];
pub const DS3_MINOR7: &[f32] = &[DS3, FS3, AS3, CS4];
pub const DS3_DOMINANT7: &[f32] = &[DS3, G3, AS3, CS4];
pub const E3_MAJOR7: &[f32] = &[E3, GS3, B3, DS4];
pub const E3_MINOR7: &[f32] = &[E3, G3, B3, D4];
pub const E3_DOMINANT7: &[f32] = &[E3, GS3, B3, D4];
pub const F3_MAJOR7: &[f32] = &[F3, A3, C4, E4];
pub const F3_MINOR7: &[f32] = &[F3, GS3, C4, DS4];
pub const F3_DOMINANT7: &[f32] = &[F3, A3, C4, DS4];
pub const FS3_MAJOR7: &[f32] = &[FS3, AS3, CS4, F4];
pub const FS3_MINOR7: &[f32] = &[FS3, A3, CS4, E4];
pub const FS3_DOMINANT7: &[f32] = &[FS3, AS3, CS4, E4];
pub const G3_MAJOR7: &[f32] = &[G3, B3, D4, FS4];
pub const G3_MINOR7: &[f32] = &[G3, AS3, D4, F4];
pub const G3_DOMINANT7: &[f32] = &[G3, B3, D4, F4];
pub const GS3_MAJOR7: &[f32] = &[GS3, C4, DS4, G4];
pub const GS3_MINOR7: &[f32] = &[GS3, B3, DS4, FS4];
pub const GS3_DOMINANT7: &[f32] = &[GS3, C4, DS4, FS4];
pub const A3_MAJOR7: &[f32] = &[A3, CS4, E4, GS4];
pub const A3_MINOR7: &[f32] = &[A3, C4, E4, G4];
pub const A3_DOMINANT7: &[f32] = &[A3, CS4, E4, G4];
pub const AS3_MAJOR7: &[f32] = &[AS3, D4, F4, A4];
pub const AS3_MINOR7: &[f32] = &[AS3, CS4, F4, GS4];
pub const AS3_DOMINANT7: &[f32] = &[AS3, D4, F4, GS4];
pub const B3_MAJOR7: &[f32] = &[B3, DS4, FS4, AS4];
pub const B3_MINOR7: &[f32] = &[B3, D4, FS4, A4];
pub const B3_DOMINANT7: &[f32] = &[B3, DS4, FS4, A4];

// Octave 4
pub const C4_MAJOR7: &[f32] = &[C4, E4, G4, B4];
pub const C4_MINOR7: &[f32] = &[C4, DS4, G4, AS4];
pub const C4_DOMINANT7: &[f32] = &[C4, E4, G4, AS4];
pub const CS4_MAJOR7: &[f32] = &[CS4, F4, GS4, C5];
pub const CS4_MINOR7: &[f32] = &[CS4, E4, GS4, B4];
pub const CS4_DOMINANT7: &[f32] = &[CS4, F4, GS4, B4];
pub const D4_MAJOR7: &[f32] = &[D4, FS4, A4, CS5];
pub const D4_MINOR7: &[f32] = &[D4, F4, A4, C5];
pub const D4_DOMINANT7: &[f32] = &[D4, FS4, A4, C5];
pub const DS4_MAJOR7: &[f32] = &[DS4, G4, AS4, D5];
pub const DS4_MINOR7: &[f32] = &[DS4, FS4, AS4, CS5];
pub const DS4_DOMINANT7: &[f32] = &[DS4, G4, AS4, CS5];
pub const E4_MAJOR7: &[f32] = &[E4, GS4, B4, DS5];
pub const E4_MINOR7: &[f32] = &[E4, G4, B4, D5];
pub const E4_DOMINANT7: &[f32] = &[E4, GS4, B4, D5];
pub const F4_MAJOR7: &[f32] = &[F4, A4, C5, E5];
pub const F4_MINOR7: &[f32] = &[F4, GS4, C5, DS5];
pub const F4_DOMINANT7: &[f32] = &[F4, A4, C5, DS5];
pub const FS4_MAJOR7: &[f32] = &[FS4, AS4, CS5, F5];
pub const FS4_MINOR7: &[f32] = &[FS4, A4, CS5, E5];
pub const FS4_DOMINANT7: &[f32] = &[FS4, AS4, CS5, E5];
pub const G4_MAJOR7: &[f32] = &[G4, B4, D5, FS5];
pub const G4_MINOR7: &[f32] = &[G4, AS4, D5, F5];
pub const G4_DOMINANT7: &[f32] = &[G4, B4, D5, F5];
pub const GS4_MAJOR7: &[f32] = &[GS4, C5, DS5, G5];
pub const GS4_MINOR7: &[f32] = &[GS4, B4, DS5, FS5];
pub const GS4_DOMINANT7: &[f32] = &[GS4, C5, DS5, FS5];
pub const A4_MAJOR7: &[f32] = &[A4, CS5, E5, GS5];
pub const A4_MINOR7: &[f32] = &[A4, C5, E5, G5];
pub const A4_DOMINANT7: &[f32] = &[A4, CS5, E5, G5];
pub const AS4_MAJOR7: &[f32] = &[AS4, D5, F5, A5];
pub const AS4_MINOR7: &[f32] = &[AS4, CS5, F5, GS5];
pub const AS4_DOMINANT7: &[f32] = &[AS4, D5, F5, GS5];
pub const B4_MAJOR7: &[f32] = &[B4, DS5, FS5, AS5];
pub const B4_MINOR7: &[f32] = &[B4, D5, FS5, A5];
pub const B4_DOMINANT7: &[f32] = &[B4, DS5, FS5, A5];

// Octave 5
pub const C5_MAJOR7: &[f32] = &[C5, E5, G5, B5];
pub const C5_MINOR7: &[f32] = &[C5, DS5, G5, AS5];
pub const C5_DOMINANT7: &[f32] = &[C5, E5, G5, AS5];
pub const CS5_MAJOR7: &[f32] = &[CS5, F5, GS5, C6];
pub const CS5_MINOR7: &[f32] = &[CS5, E5, GS5, B5];
pub const CS5_DOMINANT7: &[f32] = &[CS5, F5, GS5, B5];
pub const D5_MAJOR7: &[f32] = &[D5, FS5, A5, CS6];
pub const D5_MINOR7: &[f32] = &[D5, F5, A5, C6];
pub const D5_DOMINANT7: &[f32] = &[D5, FS5, A5, C6];
pub const DS5_MAJOR7: &[f32] = &[DS5, G5, AS5, D6];
pub const DS5_MINOR7: &[f32] = &[DS5, FS5, AS5, CS6];
pub const DS5_DOMINANT7: &[f32] = &[DS5, G5, AS5, CS6];
pub const E5_MAJOR7: &[f32] = &[E5, GS5, B5, DS6];
pub const E5_MINOR7: &[f32] = &[E5, G5, B5, D6];
pub const E5_DOMINANT7: &[f32] = &[E5, GS5, B5, D6];
pub const F5_MAJOR7: &[f32] = &[F5, A5, C6, E6];
pub const F5_MINOR7: &[f32] = &[F5, GS5, C6, DS6];
pub const F5_DOMINANT7: &[f32] = &[F5, A5, C6, DS6];
pub const FS5_MAJOR7: &[f32] = &[FS5, AS5, CS6, F6];
pub const FS5_MINOR7: &[f32] = &[FS5, A5, CS6, E6];
pub const FS5_DOMINANT7: &[f32] = &[FS5, AS5, CS6, E6];
pub const G5_MAJOR7: &[f32] = &[G5, B5, D6, FS6];
pub const G5_MINOR7: &[f32] = &[G5, AS5, D6, F6];
pub const G5_DOMINANT7: &[f32] = &[G5, B5, D6, F6];
pub const GS5_MAJOR7: &[f32] = &[GS5, C6, DS6, G6];
pub const GS5_MINOR7: &[f32] = &[GS5, B5, DS6, FS6];
pub const GS5_DOMINANT7: &[f32] = &[GS5, C6, DS6, FS6];
pub const A5_MAJOR7: &[f32] = &[A5, CS6, E6, GS6];
pub const A5_MINOR7: &[f32] = &[A5, C6, E6, G6];
pub const A5_DOMINANT7: &[f32] = &[A5, CS6, E6, G6];
pub const AS5_MAJOR7: &[f32] = &[AS5, D6, F6, A6];
pub const AS5_MINOR7: &[f32] = &[AS5, CS6, F6, GS6];
pub const AS5_DOMINANT7: &[f32] = &[AS5, D6, F6, GS6];
pub const B5_MAJOR7: &[f32] = &[B5, DS6, FS6, AS6];
pub const B5_MINOR7: &[f32] = &[B5, D6, FS6, A6];
pub const B5_DOMINANT7: &[f32] = &[B5, DS6, FS6, A6];

// Octave 6
pub const C6_MAJOR7: &[f32] = &[C6, E6, G6, B6];
pub const C6_MINOR7: &[f32] = &[C6, DS6, G6, AS6];
pub const C6_DOMINANT7: &[f32] = &[C6, E6, G6, AS6];
pub const CS6_MAJOR7: &[f32] = &[CS6, F6, GS6, C7];
pub const CS6_MINOR7: &[f32] = &[CS6, E6, GS6, B6];
pub const CS6_DOMINANT7: &[f32] = &[CS6, F6, GS6, B6];
pub const D6_MAJOR7: &[f32] = &[D6, FS6, A6, CS7];
pub const D6_MINOR7: &[f32] = &[D6, F6, A6, C7];
pub const D6_DOMINANT7: &[f32] = &[D6, FS6, A6, C7];
pub const DS6_MAJOR7: &[f32] = &[DS6, G6, AS6, D7];
pub const DS6_MINOR7: &[f32] = &[DS6, FS6, AS6, CS7];
pub const DS6_DOMINANT7: &[f32] = &[DS6, G6, AS6, CS7];
pub const E6_MAJOR7: &[f32] = &[E6, GS6, B6, DS7];
pub const E6_MINOR7: &[f32] = &[E6, G6, B6, D7];
pub const E6_DOMINANT7: &[f32] = &[E6, GS6, B6, D7];
pub const F6_MAJOR7: &[f32] = &[F6, A6, C7, E7];
pub const F6_MINOR7: &[f32] = &[F6, GS6, C7, DS7];
pub const F6_DOMINANT7: &[f32] = &[F6, A6, C7, DS7];
pub const FS6_MAJOR7: &[f32] = &[FS6, AS6, CS7, F7];
pub const FS6_MINOR7: &[f32] = &[FS6, A6, CS7, E7];
pub const FS6_DOMINANT7: &[f32] = &[FS6, AS6, CS7, E7];
pub const G6_MAJOR7: &[f32] = &[G6, B6, D7, FS7];
pub const G6_MINOR7: &[f32] = &[G6, AS6, D7, F7];
pub const G6_DOMINANT7: &[f32] = &[G6, B6, D7, F7];
pub const GS6_MAJOR7: &[f32] = &[GS6, C7, DS7, G7];
pub const GS6_MINOR7: &[f32] = &[GS6, B6, DS7, FS7];
pub const GS6_DOMINANT7: &[f32] = &[GS6, C7, DS7, FS7];
pub const A6_MAJOR7: &[f32] = &[A6, CS7, E7, GS7];
pub const A6_MINOR7: &[f32] = &[A6, C7, E7, G7];
pub const A6_DOMINANT7: &[f32] = &[A6, CS7, E7, G7];
pub const AS6_MAJOR7: &[f32] = &[AS6, D7, F7, A7];
pub const AS6_MINOR7: &[f32] = &[AS6, CS7, F7, GS7];
pub const AS6_DOMINANT7: &[f32] = &[AS6, D7, F7, GS7];
pub const B6_MAJOR7: &[f32] = &[B6, DS7, FS7, AS7];
pub const B6_MINOR7: &[f32] = &[B6, D7, FS7, A7];
pub const B6_DOMINANT7: &[f32] = &[B6, DS7, FS7, A7];

// ===== EXTENDED AND ALTERED CHORDS (Octave 4) =====
pub const C4_MAJOR9: &[f32] = &[C4, E4, G4, B4, D5];
pub const C4_MINOR9: &[f32] = &[C4, DS4, G4, AS4, D5];
pub const C4_SUS2: &[f32] = &[C4, D4, G4];
pub const C4_SUS4: &[f32] = &[C4, F4, G4];
pub const C4_DIMINISHED: &[f32] = &[C4, DS4, FS4];
pub const C4_AUGMENTED: &[f32] = &[C4, E4, GS4];
pub const CS4_MAJOR9: &[f32] = &[CS4, F4, GS4, C5, DS5];
pub const CS4_MINOR9: &[f32] = &[CS4, E4, GS4, B4, DS5];
pub const CS4_SUS2: &[f32] = &[CS4, DS4, GS4];
pub const CS4_SUS4: &[f32] = &[CS4, FS4, GS4];
pub const CS4_DIMINISHED: &[f32] = &[CS4, E4, G4];
pub const CS4_AUGMENTED: &[f32] = &[CS4, F4, A4];
pub const D4_MAJOR9: &[f32] = &[D4, FS4, A4, CS5, E5];
pub const D4_MINOR9: &[f32] = &[D4, F4, A4, C5, E5];
pub const D4_SUS2: &[f32] = &[D4, E4, A4];
pub const D4_SUS4: &[f32] = &[D4, G4, A4];
pub const D4_DIMINISHED: &[f32] = &[D4, F4, GS4];
pub const D4_AUGMENTED: &[f32] = &[D4, FS4, AS4];
pub const DS4_MAJOR9: &[f32] = &[DS4, G4, AS4, D5, F5];
pub const DS4_MINOR9: &[f32] = &[DS4, FS4, AS4, CS5, F5];
pub const DS4_SUS2: &[f32] = &[DS4, F4, AS4];
pub const DS4_SUS4: &[f32] = &[DS4, GS4, AS4];
pub const DS4_DIMINISHED: &[f32] = &[DS4, FS4, A4];
pub const DS4_AUGMENTED: &[f32] = &[DS4, G4, B4];
pub const E4_MAJOR9: &[f32] = &[E4, GS4, B4, DS5, FS5];
pub const E4_MINOR9: &[f32] = &[E4, G4, B4, D5, FS5];
pub const E4_SUS2: &[f32] = &[E4, FS4, B4];
pub const E4_SUS4: &[f32] = &[E4, A4, B4];
pub const E4_DIMINISHED: &[f32] = &[E4, G4, AS4];
pub const E4_AUGMENTED: &[f32] = &[E4, GS4, C5];
pub const F4_MAJOR9: &[f32] = &[F4, A4, C5, E5, G5];
pub const F4_MINOR9: &[f32] = &[F4, GS4, C5, DS5, G5];
pub const F4_SUS2: &[f32] = &[F4, G4, C5];
pub const F4_SUS4: &[f32] = &[F4, AS4, C5];
pub const F4_DIMINISHED: &[f32] = &[F4, GS4, B4];
pub const F4_AUGMENTED: &[f32] = &[F4, A4, CS5];
pub const FS4_MAJOR9: &[f32] = &[FS4, AS4, CS5, F5, GS5];
pub const FS4_MINOR9: &[f32] = &[FS4, A4, CS5, E5, GS5];
pub const FS4_SUS2: &[f32] = &[FS4, GS4, CS5];
pub const FS4_SUS4: &[f32] = &[FS4, B4, CS5];
pub const FS4_DIMINISHED: &[f32] = &[FS4, A4, C5];
pub const FS4_AUGMENTED: &[f32] = &[FS4, AS4, D5];
pub const G4_MAJOR9: &[f32] = &[G4, B4, D5, FS5, A5];
pub const G4_MINOR9: &[f32] = &[G4, AS4, D5, F5, A5];
pub const G4_SUS2: &[f32] = &[G4, A4, D5];
pub const G4_SUS4: &[f32] = &[G4, C5, D5];
pub const G4_DIMINISHED: &[f32] = &[G4, AS4, CS5];
pub const G4_AUGMENTED: &[f32] = &[G4, B4, DS5];
pub const GS4_MAJOR9: &[f32] = &[GS4, C5, DS5, G5, AS5];
pub const GS4_MINOR9: &[f32] = &[GS4, B4, DS5, FS5, AS5];
pub const GS4_SUS2: &[f32] = &[GS4, AS4, DS5];
pub const GS4_SUS4: &[f32] = &[GS4, CS5, DS5];
pub const GS4_DIMINISHED: &[f32] = &[GS4, B4, D5];
pub const GS4_AUGMENTED: &[f32] = &[GS4, C5, E5];
pub const A4_MAJOR9: &[f32] = &[A4, CS5, E5, GS5, B5];
pub const A4_MINOR9: &[f32] = &[A4, C5, E5, G5, B5];
pub const A4_SUS2: &[f32] = &[A4, B4, E5];
pub const A4_SUS4: &[f32] = &[A4, D5, E5];
pub const A4_DIMINISHED: &[f32] = &[A4, C5, DS5];
pub const A4_AUGMENTED: &[f32] = &[A4, CS5, F5];
pub const AS4_MAJOR9: &[f32] = &[AS4, D5, F5, A5, C6];
pub const AS4_MINOR9: &[f32] = &[AS4, CS5, F5, GS5, C6];
pub const AS4_SUS2: &[f32] = &[AS4, C5, F5];
pub const AS4_SUS4: &[f32] = &[AS4, DS5, F5];
pub const AS4_DIMINISHED: &[f32] = &[AS4, CS5, E5];
pub const AS4_AUGMENTED: &[f32] = &[AS4, D5, FS5];
pub const B4_MAJOR9: &[f32] = &[B4, DS5, FS5, AS5, CS6];
pub const B4_MINOR9: &[f32] = &[B4, D5, FS5, A5, CS6];
pub const B4_SUS2: &[f32] = &[B4, CS5, FS5];
pub const B4_SUS4: &[f32] = &[B4, E5, FS5];
pub const B4_DIMINISHED: &[f32] = &[B4, D5, F5];
pub const B4_AUGMENTED: &[f32] = &[B4, DS5, G5];
