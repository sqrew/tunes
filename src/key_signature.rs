//! Key signature support for musical notation and MIDI export
//!
//! Provides type-safe key signature representation with support for
//! major, minor, and extensible modal systems.

/// Root note of a key signature
///
/// Includes both sharp and flat enharmonic equivalents for proper notation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyRoot {
    C,
    Cs,  // C#
    D,
    Ds,  // D# / Eb
    E,
    F,
    Fs,  // F#
    G,
    Gs,  // G# / Ab
    A,
    As,  // A# / Bb
    B,
    // Enharmonic equivalents (for proper notation context)
    Df,  // Db (same as C#)
    Ef,  // Eb (same as D#)
    Gf,  // Gb (same as F#)
    Af,  // Ab (same as G#)
    Bf,  // Bb (same as A#)
}

/// Mode of a key signature
///
/// Supports major, minor, and all seven Greek modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyMode {
    Major,      // Ionian - standard major scale
    Minor,      // Natural minor (same as Aeolian)
    Dorian,     // Minor with raised 6th
    Phrygian,   // Minor with lowered 2nd
    Lydian,     // Major with raised 4th
    Mixolydian, // Major with lowered 7th
    Aeolian,    // Natural minor (same as Minor)
    Locrian,    // Diminished mode (lowered 2nd and 5th)
}

/// Complete key signature specification
///
/// Combines a root note and mode to define a key signature.
/// Used for MIDI export and musical notation context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeySignature {
    pub root: KeyRoot,
    pub mode: KeyMode,
}

impl KeySignature {
    /// Create a new key signature
    pub fn new(root: KeyRoot, mode: KeyMode) -> Self {
        Self { root, mode }
    }

    /// Get the parent major scale root for modal key signatures
    /// (helper for converting modes to their parent major scale)
    fn modal_parent(&self, semitones: i8) -> KeyRoot {
        use KeyRoot::*;

        // Map each root to semitone value (0-11)
        let root_value = match self.root {
            C => 0,
            Cs | Df => 1,
            D => 2,
            Ds | Ef => 3,
            E => 4,
            F => 5,
            Fs | Gf => 6,
            G => 7,
            Gs | Af => 8,
            A => 9,
            As | Bf => 10,
            B => 11,
        };

        // Add semitones and wrap around (modulo 12)
        let new_value = (root_value + semitones).rem_euclid(12);

        // Map back to KeyRoot (prefer sharps for now)
        match new_value {
            0 => C,
            1 => Cs,
            2 => D,
            3 => Ef,  // Use flat for Eb
            4 => E,
            5 => F,
            6 => Fs,
            7 => G,
            8 => Af,  // Use flat for Ab
            9 => A,
            10 => Bf, // Use flat for Bb
            11 => B,
            _ => C,   // Should never happen
        }
    }

    /// Convert key signature to MIDI format (sharps/flats count)
    ///
    /// Returns:
    /// - Positive values = number of sharps
    /// - Negative values = number of flats
    /// - Value range: -7 to +7
    ///
    /// For modes, the parent major scale's key signature is used.
    /// For example, D Dorian uses C Major's key signature (0 sharps/flats).
    pub fn to_midi_sharps_flats(&self) -> i8 {
        use KeyRoot::*;
        use KeyMode::*;

        // For modes, find the parent major scale and use its key signature
        // Dorian = 2nd mode (parent is 2 semitones below)
        // Phrygian = 3rd mode (parent is 4 semitones below)
        // Lydian = 4th mode (parent is 5 semitones below)
        // Mixolydian = 5th mode (parent is 7 semitones below)
        // Aeolian = 6th mode (parent is 9 semitones below = 3 above)
        // Locrian = 7th mode (parent is 11 semitones below = 1 above)
        let (effective_root, effective_mode) = match self.mode {
            Dorian => (self.modal_parent(-2), Major),
            Phrygian => (self.modal_parent(-4), Major),
            Lydian => (self.modal_parent(-5), Major),
            Mixolydian => (self.modal_parent(-7), Major),
            Aeolian => (self.modal_parent(3), Major),
            Locrian => (self.modal_parent(1), Major),
            _ => (self.root, self.mode),
        };

        match (effective_root, effective_mode) {
            // Major keys
            (C, Major) => 0,   // No sharps or flats
            (G, Major) => 1,   // 1 sharp
            (D, Major) => 2,   // 2 sharps
            (A, Major) => 3,   // 3 sharps
            (E, Major) => 4,   // 4 sharps
            (B, Major) => 5,   // 5 sharps
            (Fs, Major) | (Gf, Major) => 6,  // 6 sharps (or 6 flats enharmonic)
            (Cs, Major) | (Df, Major) => 7,  // 7 sharps (or 5 flats enharmonic)

            (F, Major) => -1,  // 1 flat
            (Bf, Major) => -2, // 2 flats
            (Ef, Major) => -3, // 3 flats
            (Af, Major) => -4, // 4 flats
            // Db Major = 5 flats (handled above as Df)
            // Gb Major = 6 flats (handled above as Gf)
            // Cb Major would be 7 flats (not commonly used)

            // Minor keys (relative minor relationship)
            (A, Minor) => 0,   // No sharps or flats (relative to C major)
            (E, Minor) => 1,   // 1 sharp (relative to G major)
            (B, Minor) => 2,   // 2 sharps (relative to D major)
            (Fs, Minor) => 3,  // 3 sharps (relative to A major)
            (Cs, Minor) => 4,  // 4 sharps (relative to E major)
            (Gs, Minor) => 5,  // 5 sharps (relative to B major)
            (Ds, Minor) | (Ef, Minor) => 6,  // 6 sharps (or enharmonic)
            (As, Minor) | (Bf, Minor) => 7,  // 7 sharps (or enharmonic)

            (D, Minor) => -1,  // 1 flat (relative to F major)
            (G, Minor) => -2,  // 2 flats (relative to Bb major)
            (C, Minor) => -3,  // 3 flats (relative to Eb major)
            (F, Minor) => -4,  // 4 flats (relative to Ab major)
            // Bb minor = 5 flats
            // Eb minor = 6 flats
            // Ab minor = 7 flats

            // Uncommon combinations - default to 0
            _ => 0,
        }
    }

    /// Check if this key signature is in a minor-type mode
    /// (includes Minor, Dorian, Phrygian, Aeolian, Locrian)
    pub fn is_minor(&self) -> bool {
        matches!(
            self.mode,
            KeyMode::Minor | KeyMode::Dorian | KeyMode::Phrygian | KeyMode::Aeolian | KeyMode::Locrian
        )
    }

    /// Get a human-readable name for this key signature
    pub fn name(&self) -> String {
        let root_name = match self.root {
            KeyRoot::C => "C",
            KeyRoot::Cs => "C#",
            KeyRoot::D => "D",
            KeyRoot::Ds => "D#",
            KeyRoot::E => "E",
            KeyRoot::F => "F",
            KeyRoot::Fs => "F#",
            KeyRoot::G => "G",
            KeyRoot::Gs => "G#",
            KeyRoot::A => "A",
            KeyRoot::As => "A#",
            KeyRoot::B => "B",
            KeyRoot::Df => "Db",
            KeyRoot::Ef => "Eb",
            KeyRoot::Gf => "Gb",
            KeyRoot::Af => "Ab",
            KeyRoot::Bf => "Bb",
        };

        let mode_name = match self.mode {
            KeyMode::Major => "Major",
            KeyMode::Minor => "Minor",
            KeyMode::Dorian => "Dorian",
            KeyMode::Phrygian => "Phrygian",
            KeyMode::Lydian => "Lydian",
            KeyMode::Mixolydian => "Mixolydian",
            KeyMode::Aeolian => "Aeolian",
            KeyMode::Locrian => "Locrian",
        };

        format!("{} {}", root_name, mode_name)
    }
}

// Common key signature constants for convenience
impl KeySignature {
    pub const C_MAJOR: Self = Self { root: KeyRoot::C, mode: KeyMode::Major };
    pub const G_MAJOR: Self = Self { root: KeyRoot::G, mode: KeyMode::Major };
    pub const D_MAJOR: Self = Self { root: KeyRoot::D, mode: KeyMode::Major };
    pub const A_MAJOR: Self = Self { root: KeyRoot::A, mode: KeyMode::Major };
    pub const E_MAJOR: Self = Self { root: KeyRoot::E, mode: KeyMode::Major };
    pub const B_MAJOR: Self = Self { root: KeyRoot::B, mode: KeyMode::Major };
    pub const F_SHARP_MAJOR: Self = Self { root: KeyRoot::Fs, mode: KeyMode::Major };
    pub const C_SHARP_MAJOR: Self = Self { root: KeyRoot::Cs, mode: KeyMode::Major };

    pub const F_MAJOR: Self = Self { root: KeyRoot::F, mode: KeyMode::Major };
    pub const B_FLAT_MAJOR: Self = Self { root: KeyRoot::Bf, mode: KeyMode::Major };
    pub const E_FLAT_MAJOR: Self = Self { root: KeyRoot::Ef, mode: KeyMode::Major };
    pub const A_FLAT_MAJOR: Self = Self { root: KeyRoot::Af, mode: KeyMode::Major };
    pub const D_FLAT_MAJOR: Self = Self { root: KeyRoot::Df, mode: KeyMode::Major };
    pub const G_FLAT_MAJOR: Self = Self { root: KeyRoot::Gf, mode: KeyMode::Major };

    pub const A_MINOR: Self = Self { root: KeyRoot::A, mode: KeyMode::Minor };
    pub const E_MINOR: Self = Self { root: KeyRoot::E, mode: KeyMode::Minor };
    pub const B_MINOR: Self = Self { root: KeyRoot::B, mode: KeyMode::Minor };
    pub const F_SHARP_MINOR: Self = Self { root: KeyRoot::Fs, mode: KeyMode::Minor };
    pub const C_SHARP_MINOR: Self = Self { root: KeyRoot::Cs, mode: KeyMode::Minor };
    pub const G_SHARP_MINOR: Self = Self { root: KeyRoot::Gs, mode: KeyMode::Minor };
    pub const D_SHARP_MINOR: Self = Self { root: KeyRoot::Ds, mode: KeyMode::Minor };

    pub const D_MINOR: Self = Self { root: KeyRoot::D, mode: KeyMode::Minor };
    pub const G_MINOR: Self = Self { root: KeyRoot::G, mode: KeyMode::Minor };
    pub const C_MINOR: Self = Self { root: KeyRoot::C, mode: KeyMode::Minor };
    pub const F_MINOR: Self = Self { root: KeyRoot::F, mode: KeyMode::Minor };
    pub const B_FLAT_MINOR: Self = Self { root: KeyRoot::Bf, mode: KeyMode::Minor };
    pub const E_FLAT_MINOR: Self = Self { root: KeyRoot::Ef, mode: KeyMode::Minor };

    // Common Dorian mode keys (jazz, funk, rock)
    pub const D_DORIAN: Self = Self { root: KeyRoot::D, mode: KeyMode::Dorian };
    pub const E_DORIAN: Self = Self { root: KeyRoot::E, mode: KeyMode::Dorian };
    pub const A_DORIAN: Self = Self { root: KeyRoot::A, mode: KeyMode::Dorian };
    pub const G_DORIAN: Self = Self { root: KeyRoot::G, mode: KeyMode::Dorian };

    // Common Phrygian mode keys (Spanish, metal, Middle Eastern)
    pub const E_PHRYGIAN: Self = Self { root: KeyRoot::E, mode: KeyMode::Phrygian };
    pub const A_PHRYGIAN: Self = Self { root: KeyRoot::A, mode: KeyMode::Phrygian };
    pub const B_PHRYGIAN: Self = Self { root: KeyRoot::B, mode: KeyMode::Phrygian };

    // Common Lydian mode keys (dreamy, bright)
    pub const F_LYDIAN: Self = Self { root: KeyRoot::F, mode: KeyMode::Lydian };
    pub const C_LYDIAN: Self = Self { root: KeyRoot::C, mode: KeyMode::Lydian };
    pub const G_LYDIAN: Self = Self { root: KeyRoot::G, mode: KeyMode::Lydian };

    // Common Mixolydian mode keys (rock, blues, folk)
    pub const G_MIXOLYDIAN: Self = Self { root: KeyRoot::G, mode: KeyMode::Mixolydian };
    pub const D_MIXOLYDIAN: Self = Self { root: KeyRoot::D, mode: KeyMode::Mixolydian };
    pub const A_MIXOLYDIAN: Self = Self { root: KeyRoot::A, mode: KeyMode::Mixolydian };
    pub const C_MIXOLYDIAN: Self = Self { root: KeyRoot::C, mode: KeyMode::Mixolydian };

    // Aeolian (natural minor - same as Minor mode)
    pub const A_AEOLIAN: Self = Self { root: KeyRoot::A, mode: KeyMode::Aeolian };
    pub const E_AEOLIAN: Self = Self { root: KeyRoot::E, mode: KeyMode::Aeolian };
    pub const D_AEOLIAN: Self = Self { root: KeyRoot::D, mode: KeyMode::Aeolian };

    // Locrian mode keys (rare, diminished, dark)
    pub const B_LOCRIAN: Self = Self { root: KeyRoot::B, mode: KeyMode::Locrian };
    pub const E_LOCRIAN: Self = Self { root: KeyRoot::E, mode: KeyMode::Locrian };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_key_sharps() {
        assert_eq!(KeySignature::C_MAJOR.to_midi_sharps_flats(), 0);
        assert_eq!(KeySignature::G_MAJOR.to_midi_sharps_flats(), 1);
        assert_eq!(KeySignature::D_MAJOR.to_midi_sharps_flats(), 2);
        assert_eq!(KeySignature::A_MAJOR.to_midi_sharps_flats(), 3);
        assert_eq!(KeySignature::E_MAJOR.to_midi_sharps_flats(), 4);
        assert_eq!(KeySignature::B_MAJOR.to_midi_sharps_flats(), 5);
        assert_eq!(KeySignature::F_SHARP_MAJOR.to_midi_sharps_flats(), 6);
    }

    #[test]
    fn test_major_key_flats() {
        assert_eq!(KeySignature::F_MAJOR.to_midi_sharps_flats(), -1);
        assert_eq!(KeySignature::B_FLAT_MAJOR.to_midi_sharps_flats(), -2);
        assert_eq!(KeySignature::E_FLAT_MAJOR.to_midi_sharps_flats(), -3);
        assert_eq!(KeySignature::A_FLAT_MAJOR.to_midi_sharps_flats(), -4);
        assert_eq!(KeySignature::D_FLAT_MAJOR.to_midi_sharps_flats(), 7);  // Enharmonic
        assert_eq!(KeySignature::G_FLAT_MAJOR.to_midi_sharps_flats(), 6);  // Enharmonic
    }

    #[test]
    fn test_minor_keys() {
        assert_eq!(KeySignature::A_MINOR.to_midi_sharps_flats(), 0);
        assert_eq!(KeySignature::E_MINOR.to_midi_sharps_flats(), 1);
        assert_eq!(KeySignature::B_MINOR.to_midi_sharps_flats(), 2);
        assert_eq!(KeySignature::D_MINOR.to_midi_sharps_flats(), -1);
        assert_eq!(KeySignature::G_MINOR.to_midi_sharps_flats(), -2);
        assert_eq!(KeySignature::C_MINOR.to_midi_sharps_flats(), -3);
    }

    #[test]
    fn test_is_minor() {
        assert!(!KeySignature::C_MAJOR.is_minor());
        assert!(KeySignature::A_MINOR.is_minor());
        assert!(!KeySignature::G_MAJOR.is_minor());
        assert!(KeySignature::E_MINOR.is_minor());
    }

    #[test]
    fn test_key_names() {
        assert_eq!(KeySignature::C_MAJOR.name(), "C Major");
        assert_eq!(KeySignature::A_MINOR.name(), "A Minor");
        assert_eq!(KeySignature::F_SHARP_MAJOR.name(), "F# Major");
        assert_eq!(KeySignature::B_FLAT_MAJOR.name(), "Bb Major");
        assert_eq!(KeySignature::E_FLAT_MINOR.name(), "Eb Minor");
    }

    #[test]
    fn test_enharmonic_equivalents() {
        let fs_major = KeySignature::new(KeyRoot::Fs, KeyMode::Major);
        let gf_major = KeySignature::new(KeyRoot::Gf, KeyMode::Major);

        // Both should map to the same MIDI representation
        assert_eq!(fs_major.to_midi_sharps_flats(), 6);
        assert_eq!(gf_major.to_midi_sharps_flats(), 6);
    }

    #[test]
    fn test_custom_key_signature() {
        let key = KeySignature::new(KeyRoot::D, KeyMode::Major);
        assert_eq!(key.to_midi_sharps_flats(), 2);
        assert!(!key.is_minor());
        assert_eq!(key.name(), "D Major");
    }

    #[test]
    fn test_dorian_mode() {
        // D Dorian = C Major parent (0 sharps/flats)
        assert_eq!(KeySignature::D_DORIAN.to_midi_sharps_flats(), 0);
        assert!(KeySignature::D_DORIAN.is_minor());
        assert_eq!(KeySignature::D_DORIAN.name(), "D Dorian");

        // E Dorian = D Major parent (2 sharps)
        assert_eq!(KeySignature::E_DORIAN.to_midi_sharps_flats(), 2);

        // G Dorian = F Major parent (1 flat)
        assert_eq!(KeySignature::G_DORIAN.to_midi_sharps_flats(), -1);
    }

    #[test]
    fn test_phrygian_mode() {
        // E Phrygian = C Major parent (0 sharps/flats)
        assert_eq!(KeySignature::E_PHRYGIAN.to_midi_sharps_flats(), 0);
        assert!(KeySignature::E_PHRYGIAN.is_minor());
        assert_eq!(KeySignature::E_PHRYGIAN.name(), "E Phrygian");

        // A Phrygian = F Major parent (1 flat)
        assert_eq!(KeySignature::A_PHRYGIAN.to_midi_sharps_flats(), -1);

        // B Phrygian = G Major parent (1 sharp)
        assert_eq!(KeySignature::B_PHRYGIAN.to_midi_sharps_flats(), 1);
    }

    #[test]
    fn test_lydian_mode() {
        // F Lydian = C Major parent (0 sharps/flats)
        assert_eq!(KeySignature::F_LYDIAN.to_midi_sharps_flats(), 0);
        assert!(!KeySignature::F_LYDIAN.is_minor());
        assert_eq!(KeySignature::F_LYDIAN.name(), "F Lydian");

        // C Lydian = G Major parent (1 sharp)
        assert_eq!(KeySignature::C_LYDIAN.to_midi_sharps_flats(), 1);

        // G Lydian = D Major parent (2 sharps)
        assert_eq!(KeySignature::G_LYDIAN.to_midi_sharps_flats(), 2);
    }

    #[test]
    fn test_mixolydian_mode() {
        // G Mixolydian = C Major parent (0 sharps/flats)
        assert_eq!(KeySignature::G_MIXOLYDIAN.to_midi_sharps_flats(), 0);
        assert!(!KeySignature::G_MIXOLYDIAN.is_minor());
        assert_eq!(KeySignature::G_MIXOLYDIAN.name(), "G Mixolydian");

        // D Mixolydian = G Major parent (1 sharp)
        assert_eq!(KeySignature::D_MIXOLYDIAN.to_midi_sharps_flats(), 1);

        // A Mixolydian = D Major parent (2 sharps)
        assert_eq!(KeySignature::A_MIXOLYDIAN.to_midi_sharps_flats(), 2);

        // C Mixolydian = F Major parent (1 flat)
        assert_eq!(KeySignature::C_MIXOLYDIAN.to_midi_sharps_flats(), -1);
    }

    #[test]
    fn test_aeolian_mode() {
        // A Aeolian = C Major parent (0 sharps/flats)
        assert_eq!(KeySignature::A_AEOLIAN.to_midi_sharps_flats(), 0);
        assert!(KeySignature::A_AEOLIAN.is_minor());
        assert_eq!(KeySignature::A_AEOLIAN.name(), "A Aeolian");

        // E Aeolian = G Major parent (1 sharp)
        assert_eq!(KeySignature::E_AEOLIAN.to_midi_sharps_flats(), 1);

        // D Aeolian = F Major parent (1 flat)
        assert_eq!(KeySignature::D_AEOLIAN.to_midi_sharps_flats(), -1);
    }

    #[test]
    fn test_locrian_mode() {
        // B Locrian = C Major parent (0 sharps/flats)
        assert_eq!(KeySignature::B_LOCRIAN.to_midi_sharps_flats(), 0);
        assert!(KeySignature::B_LOCRIAN.is_minor());
        assert_eq!(KeySignature::B_LOCRIAN.name(), "B Locrian");

        // E Locrian = F Major parent (1 flat)
        assert_eq!(KeySignature::E_LOCRIAN.to_midi_sharps_flats(), -1);
    }

    #[test]
    fn test_all_modes_of_c() {
        // All modes derived from the C major scale
        assert_eq!(KeySignature::new(KeyRoot::C, KeyMode::Major).to_midi_sharps_flats(), 0);      // C Ionian
        assert_eq!(KeySignature::new(KeyRoot::D, KeyMode::Dorian).to_midi_sharps_flats(), 0);     // D Dorian
        assert_eq!(KeySignature::new(KeyRoot::E, KeyMode::Phrygian).to_midi_sharps_flats(), 0);   // E Phrygian
        assert_eq!(KeySignature::new(KeyRoot::F, KeyMode::Lydian).to_midi_sharps_flats(), 0);     // F Lydian
        assert_eq!(KeySignature::new(KeyRoot::G, KeyMode::Mixolydian).to_midi_sharps_flats(), 0); // G Mixolydian
        assert_eq!(KeySignature::new(KeyRoot::A, KeyMode::Aeolian).to_midi_sharps_flats(), 0);    // A Aeolian
        assert_eq!(KeySignature::new(KeyRoot::B, KeyMode::Locrian).to_midi_sharps_flats(), 0);    // B Locrian
    }
}
