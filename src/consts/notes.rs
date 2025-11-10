#![allow(dead_code)]

// Note frequencies in Hz (A4 = 440Hz standard)
// Frequency doubles with each octave (C1 = C0 * 2, C2 = C1 * 2, etc.)

// Octave -1 (Ultra sub-bass, mostly felt rather than heard)
pub const C_1: f32 = 8.18;
pub const CS_1: f32 = 8.66;
pub const D_1: f32 = 9.18;
pub const DS_1: f32 = 9.72;
pub const E_1: f32 = 10.30;
pub const F_1: f32 = 10.91;
pub const FS_1: f32 = 11.56;
pub const G_1: f32 = 12.25;
pub const GS_1: f32 = 12.98;
pub const A_1: f32 = 13.75;
pub const AS_1: f32 = 14.57;
pub const B_1: f32 = 15.43;

// Octave 0 (Sub-bass)
pub const C0: f32 = 16.35;
pub const CS0: f32 = 17.32;
pub const D0: f32 = 18.35;
pub const DS0: f32 = 19.45;
pub const E0: f32 = 20.60;
pub const F0: f32 = 21.83;
pub const FS0: f32 = 23.12;
pub const G0: f32 = 24.50;
pub const GS0: f32 = 25.96;
pub const A0: f32 = 27.50;
pub const AS0: f32 = 29.14;
pub const B0: f32 = 30.87;

// Octave 1
pub const C1: f32 = 32.70;
pub const CS1: f32 = 34.65;
pub const D1: f32 = 36.71;
pub const DS1: f32 = 38.89;
pub const E1: f32 = 41.20;
pub const F1: f32 = 43.65;
pub const FS1: f32 = 46.25;
pub const G1: f32 = 49.00;
pub const GS1: f32 = 51.91;
pub const A1: f32 = 55.00;
pub const AS1: f32 = 58.27;
pub const B1: f32 = 61.74;

// Octave 2
pub const C2: f32 = 65.41;
pub const CS2: f32 = 69.30;
pub const D2: f32 = 73.42;
pub const DS2: f32 = 77.78;
pub const E2: f32 = 82.41;
pub const F2: f32 = 87.31;
pub const FS2: f32 = 92.50;
pub const G2: f32 = 98.00;
pub const GS2: f32 = 103.83;
pub const A2: f32 = 110.00;
pub const AS2: f32 = 116.54;
pub const B2: f32 = 123.47;

// Octave 3
pub const C3: f32 = 130.81;
pub const CS3: f32 = 138.59;
pub const D3: f32 = 146.83;
pub const DS3: f32 = 155.56;
pub const E3: f32 = 164.81;
pub const F3: f32 = 174.61;
pub const FS3: f32 = 185.00;
pub const G3: f32 = 196.00;
pub const GS3: f32 = 207.65;
pub const A3: f32 = 220.00;
pub const AS3: f32 = 233.08;
pub const B3: f32 = 246.94;

// Octave 4 (Middle C to B)
pub const C4: f32 = 261.63;
pub const CS4: f32 = 277.18;
pub const D4: f32 = 293.66;
pub const DS4: f32 = 311.13;
pub const E4: f32 = 329.63;
pub const F4: f32 = 349.23;
pub const FS4: f32 = 369.99;
pub const G4: f32 = 392.00;
pub const GS4: f32 = 415.30;
pub const A4: f32 = 440.00; // Concert pitch reference
pub const AS4: f32 = 466.16;
pub const B4: f32 = 493.88;

// Octave 5
pub const C5: f32 = 523.25;
pub const CS5: f32 = 554.37;
pub const D5: f32 = 587.33;
pub const DS5: f32 = 622.25;
pub const E5: f32 = 659.25;
pub const F5: f32 = 698.46;
pub const FS5: f32 = 739.99;
pub const G5: f32 = 783.99;
pub const GS5: f32 = 830.61;
pub const A5: f32 = 880.00;
pub const AS5: f32 = 932.33;
pub const B5: f32 = 987.77;

// Octave 6
pub const C6: f32 = 1046.50;
pub const CS6: f32 = 1108.73;
pub const D6: f32 = 1174.66;
pub const DS6: f32 = 1244.51;
pub const E6: f32 = 1318.51;
pub const F6: f32 = 1396.91;
pub const FS6: f32 = 1479.98;
pub const G6: f32 = 1567.98;
pub const GS6: f32 = 1661.22;
pub const A6: f32 = 1760.00;
pub const AS6: f32 = 1864.66;
pub const B6: f32 = 1975.53;

// Octave 7
pub const C7: f32 = 2093.00;
pub const CS7: f32 = 2217.46;
pub const D7: f32 = 2349.32;
pub const DS7: f32 = 2489.02;
pub const E7: f32 = 2637.02;
pub const F7: f32 = 2793.83;
pub const FS7: f32 = 2959.96;
pub const G7: f32 = 3135.96;
pub const GS7: f32 = 3322.44;
pub const A7: f32 = 3520.00;
pub const AS7: f32 = 3729.31;
pub const B7: f32 = 3951.07;

// Octave 8
pub const C8: f32 = 4186.01;
pub const CS8: f32 = 4434.92;
pub const D8: f32 = 4698.63;
pub const DS8: f32 = 4978.03;
pub const E8: f32 = 5274.04;
pub const F8: f32 = 5587.65;
pub const FS8: f32 = 5919.91;
pub const G8: f32 = 6271.93;
pub const GS8: f32 = 6644.88;
pub const A8: f32 = 7040.00;
pub const AS8: f32 = 7458.62;
pub const B8: f32 = 7902.13;

// Octave 9 (High frequencies, useful for harmonics and synthesis)
pub const C9: f32 = 8372.02;
pub const CS9: f32 = 8869.84;
pub const D9: f32 = 9397.27;
pub const DS9: f32 = 9956.06;
pub const E9: f32 = 10548.08;
pub const F9: f32 = 11175.3;
pub const FS9: f32 = 11839.82;
pub const G9: f32 = 12543.85;
pub const GS9: f32 = 13289.75;
pub const A9: f32 = 14080.00;
pub const AS9: f32 = 14917.24;
pub const B9: f32 = 15804.27;

// Octave 10 (Very high frequencies, mostly for synthesis effects and harmonics)
pub const C10: f32 = 16744.04;
pub const CS10: f32 = 17739.69;
pub const D10: f32 = 18794.55;
pub const DS10: f32 = 19912.13;
pub const E10: f32 = 21096.16;
pub const F10: f32 = 22350.61;
pub const FS10: f32 = 23679.64;
pub const G10: f32 = 25087.71;
pub const GS10: f32 = 26579.5;
pub const A10: f32 = 28160.00;
pub const AS10: f32 = 29834.48;
pub const B10: f32 = 31608.53;

// Octave 11 (Ultra-high frequencies, for scale completion and extreme synthesis)
pub const C11: f32 = 33488.09;
pub const CS11: f32 = 35479.38;
pub const D11: f32 = 37589.09;
pub const DS11: f32 = 39824.27;
pub const E11: f32 = 42192.31;
pub const F11: f32 = 44701.22;
pub const FS11: f32 = 47359.29;
pub const G11: f32 = 50175.42;
pub const GS11: f32 = 53159.01;
pub const A11: f32 = 56320.00;
pub const AS11: f32 = 59668.95;
pub const B11: f32 = 63217.07;

// =============================================================================
// FLAT NOTE ALIASES
// =============================================================================
// Enharmonic equivalents for flats (Db = C#, Eb = D#, etc.)
// These make chord progressions and music theory code more readable

// Octave -1
pub const DB_1: f32 = CS_1;
pub const EB_1: f32 = DS_1;
pub const GB_1: f32 = FS_1;
pub const AB_1: f32 = GS_1;
pub const BB_1: f32 = AS_1;

// Octave 0
pub const DB0: f32 = CS0;
pub const EB0: f32 = DS0;
pub const GB0: f32 = FS0;
pub const AB0: f32 = GS0;
pub const BB0: f32 = AS0;

// Octave 1
pub const DB1: f32 = CS1;
pub const EB1: f32 = DS1;
pub const GB1: f32 = FS1;
pub const AB1: f32 = GS1;
pub const BB1: f32 = AS1;

// Octave 2
pub const DB2: f32 = CS2;
pub const EB2: f32 = DS2;
pub const GB2: f32 = FS2;
pub const AB2: f32 = GS2;
pub const BB2: f32 = AS2;

// Octave 3
pub const DB3: f32 = CS3;
pub const EB3: f32 = DS3;
pub const GB3: f32 = FS3;
pub const AB3: f32 = GS3;
pub const BB3: f32 = AS3;

// Octave 4
pub const DB4: f32 = CS4;
pub const EB4: f32 = DS4;
pub const GB4: f32 = FS4;
pub const AB4: f32 = GS4;
pub const BB4: f32 = AS4;

// Octave 5
pub const DB5: f32 = CS5;
pub const EB5: f32 = DS5;
pub const GB5: f32 = FS5;
pub const AB5: f32 = GS5;
pub const BB5: f32 = AS5;

// Octave 6
pub const DB6: f32 = CS6;
pub const EB6: f32 = DS6;
pub const GB6: f32 = FS6;
pub const AB6: f32 = GS6;
pub const BB6: f32 = AS6;

// Octave 7
pub const DB7: f32 = CS7;
pub const EB7: f32 = DS7;
pub const GB7: f32 = FS7;
pub const AB7: f32 = GS7;
pub const BB7: f32 = AS7;

// Octave 8
pub const DB8: f32 = CS8;
pub const EB8: f32 = DS8;
pub const GB8: f32 = FS8;
pub const AB8: f32 = GS8;
pub const BB8: f32 = AS8;

// Octave 9
pub const DB9: f32 = CS9;
pub const EB9: f32 = DS9;
pub const GB9: f32 = FS9;
pub const AB9: f32 = GS9;
pub const BB9: f32 = AS9;

// Octave 10
pub const DB10: f32 = CS10;
pub const EB10: f32 = DS10;
pub const GB10: f32 = FS10;
pub const AB10: f32 = GS10;
pub const BB10: f32 = AS10;

// Octave 11
pub const DB11: f32 = CS11;
pub const EB11: f32 = DS11;
pub const GB11: f32 = FS11;
pub const AB11: f32 = GS11;
pub const BB11: f32 = AS11;
