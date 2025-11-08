//! Music theory helpers for programmatic scale and chord generation
//!
//! This module provides functions to:
//! - Generate scales from any root note
//! - Create chord progressions
//! - Transpose notes and sequences
//! - Work with intervals programmatically

/// Transposes a frequency by a given number of semitones
///
/// # Arguments
/// * `frequency` - The starting frequency in Hz
/// * `semitones` - Number of semitones to transpose (can be negative)
///
/// # Example
/// ```
/// # use tunes::consts::notes::*;
/// use tunes::theory::core::transpose;
/// use tunes::consts::notes::C4;
///
/// let d4 = transpose(C4, 2);  // C4 up 2 semitones = D4
/// let a3 = transpose(C4, -3); // C4 down 3 semitones = A3
/// ```
pub fn transpose(frequency: f32, semitones: i32) -> f32 {
    frequency * 2.0f32.powf(semitones as f32 / 12.0)
}

/// Scale intervals (in semitones from root)
pub struct ScalePattern {
    pub intervals: &'static [i32],
}

impl ScalePattern {
    /// Major scale: W-W-H-W-W-W-H (2-2-1-2-2-2-1 semitones)
    pub const MAJOR: ScalePattern = ScalePattern {
        intervals: &[0, 2, 4, 5, 7, 9, 11, 12],
    };

    /// Natural minor scale: W-H-W-W-H-W-W (2-1-2-2-1-2-2 semitones)
    pub const MINOR: ScalePattern = ScalePattern {
        intervals: &[0, 2, 3, 5, 7, 8, 10, 12],
    };

    /// Harmonic minor scale: W-H-W-W-H-WH-H (2-1-2-2-1-3-1 semitones)
    pub const HARMONIC_MINOR: ScalePattern = ScalePattern {
        intervals: &[0, 2, 3, 5, 7, 8, 11, 12],
    };

    /// Melodic minor scale (ascending): W-H-W-W-W-W-H (2-1-2-2-2-2-1 semitones)
    pub const MELODIC_MINOR: ScalePattern = ScalePattern {
        intervals: &[0, 2, 3, 5, 7, 9, 11, 12],
    };

    /// Major pentatonic: R-W-W-WH-W-WH (0-2-4-7-9-12)
    pub const MAJOR_PENTATONIC: ScalePattern = ScalePattern {
        intervals: &[0, 2, 4, 7, 9, 12],
    };

    /// Minor pentatonic: R-WH-W-W-WH-W (0-3-5-7-10-12)
    pub const MINOR_PENTATONIC: ScalePattern = ScalePattern {
        intervals: &[0, 3, 5, 7, 10, 12],
    };

    /// Blues scale: R-WH-W-H-H-WH-W (0-3-5-6-7-10-12)
    pub const BLUES: ScalePattern = ScalePattern {
        intervals: &[0, 3, 5, 6, 7, 10, 12],
    };

    /// Chromatic scale (all 12 semitones)
    pub const CHROMATIC: ScalePattern = ScalePattern {
        intervals: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    };

    /// Whole tone scale (6 whole steps)
    pub const WHOLE_TONE: ScalePattern = ScalePattern {
        intervals: &[0, 2, 4, 6, 8, 10, 12],
    };

    // === MODES ===

    /// Dorian mode: W-H-W-W-W-H-W (2-1-2-2-2-1-2)
    pub const DORIAN: ScalePattern = ScalePattern {
        intervals: &[0, 2, 3, 5, 7, 9, 10, 12],
    };

    /// Phrygian mode: H-W-W-W-H-W-W (1-2-2-2-1-2-2)
    pub const PHRYGIAN: ScalePattern = ScalePattern {
        intervals: &[0, 1, 3, 5, 7, 8, 10, 12],
    };

    /// Lydian mode: W-W-W-H-W-W-H (2-2-2-1-2-2-1)
    pub const LYDIAN: ScalePattern = ScalePattern {
        intervals: &[0, 2, 4, 6, 7, 9, 11, 12],
    };

    /// Mixolydian mode: W-W-H-W-W-H-W (2-2-1-2-2-1-2)
    pub const MIXOLYDIAN: ScalePattern = ScalePattern {
        intervals: &[0, 2, 4, 5, 7, 9, 10, 12],
    };

    /// Locrian mode: H-W-W-H-W-W-W (1-2-2-1-2-2-2)
    pub const LOCRIAN: ScalePattern = ScalePattern {
        intervals: &[0, 1, 3, 5, 6, 8, 10, 12],
    };

    // === JAZZ & BEBOP SCALES ===

    /// Bebop major scale (major with chromatic passing tone)
    pub const BEBOP_MAJOR: ScalePattern = ScalePattern {
        intervals: &[0, 2, 4, 5, 7, 8, 9, 11, 12],
    };

    /// Bebop dominant scale (mixolydian with chromatic passing tone)
    pub const BEBOP_DOMINANT: ScalePattern = ScalePattern {
        intervals: &[0, 2, 4, 5, 7, 9, 10, 11, 12],
    };

    /// Bebop minor scale (dorian with chromatic passing tone)
    pub const BEBOP_MINOR: ScalePattern = ScalePattern {
        intervals: &[0, 2, 3, 5, 7, 8, 9, 10, 12],
    };

    /// Altered scale (super locrian, diminished whole tone)
    pub const ALTERED: ScalePattern = ScalePattern {
        intervals: &[0, 1, 3, 4, 6, 8, 10, 12],
    };

    /// Half-whole diminished scale (octatonic)
    pub const DIMINISHED_HALF_WHOLE: ScalePattern = ScalePattern {
        intervals: &[0, 1, 3, 4, 6, 7, 9, 10, 12],
    };

    /// Whole-half diminished scale (octatonic)
    pub const DIMINISHED_WHOLE_HALF: ScalePattern = ScalePattern {
        intervals: &[0, 2, 3, 5, 6, 8, 9, 11, 12],
    };

    // === JAPANESE SCALES ===

    /// Hirajoshi scale (Japanese pentatonic)
    pub const HIRAJOSHI: ScalePattern = ScalePattern {
        intervals: &[0, 2, 3, 7, 8, 12],
    };

    /// In Sen scale (Japanese pentatonic)
    pub const IN_SEN: ScalePattern = ScalePattern {
        intervals: &[0, 1, 5, 7, 10, 12],
    };

    /// Iwato scale (Japanese pentatonic)
    pub const IWATO: ScalePattern = ScalePattern {
        intervals: &[0, 1, 5, 6, 10, 12],
    };

    /// Yo scale (Japanese pentatonic, similar to major pentatonic)
    pub const YO: ScalePattern = ScalePattern {
        intervals: &[0, 2, 5, 7, 9, 12],
    };

    /// Kumoi scale (Japanese pentatonic)
    pub const KUMOI: ScalePattern = ScalePattern {
        intervals: &[0, 2, 3, 7, 9, 12],
    };

    // === MIDDLE EASTERN SCALES ===

    /// Maqam Hijaz (Middle Eastern scale)
    pub const HIJAZ: ScalePattern = ScalePattern {
        intervals: &[0, 1, 4, 5, 7, 8, 11, 12],
    };

    /// Double harmonic major (Byzantine, Arabic)
    pub const DOUBLE_HARMONIC: ScalePattern = ScalePattern {
        intervals: &[0, 1, 4, 5, 7, 8, 11, 12],
    };

    /// Phrygian dominant (Spanish Phrygian, Ahava Rabbah)
    pub const PHRYGIAN_DOMINANT: ScalePattern = ScalePattern {
        intervals: &[0, 1, 4, 5, 7, 8, 10, 12],
    };

    /// Persian scale
    pub const PERSIAN: ScalePattern = ScalePattern {
        intervals: &[0, 1, 4, 5, 6, 8, 11, 12],
    };

    // === INDIAN SCALES (Basic Ragas) ===

    /// Bhairav raga (Indian classical)
    pub const BHAIRAV: ScalePattern = ScalePattern {
        intervals: &[0, 1, 4, 5, 7, 8, 11, 12],
    };

    /// Kafi raga (similar to Dorian)
    pub const KAFI: ScalePattern = ScalePattern {
        intervals: &[0, 2, 3, 5, 7, 9, 10, 12],
    };

    /// Bhairavi raga
    pub const BHAIRAVI: ScalePattern = ScalePattern {
        intervals: &[0, 1, 3, 5, 7, 8, 10, 12],
    };

    /// Purvi raga
    pub const PURVI: ScalePattern = ScalePattern {
        intervals: &[0, 1, 4, 6, 7, 8, 11, 12],
    };

    /// Marva raga
    pub const MARVA: ScalePattern = ScalePattern {
        intervals: &[0, 1, 4, 6, 7, 9, 11, 12],
    };

    // === HUNGARIAN & GYPSY SCALES ===

    /// Hungarian minor scale (Gypsy minor)
    pub const HUNGARIAN_MINOR: ScalePattern = ScalePattern {
        intervals: &[0, 2, 3, 6, 7, 8, 11, 12],
    };

    /// Hungarian major scale
    pub const HUNGARIAN_MAJOR: ScalePattern = ScalePattern {
        intervals: &[0, 3, 4, 6, 7, 9, 10, 12],
    };

    /// Gypsy scale (Byzantine)
    pub const GYPSY: ScalePattern = ScalePattern {
        intervals: &[0, 1, 4, 5, 7, 8, 10, 12],
    };

    // === SPANISH & FLAMENCO SCALES ===

    /// Spanish scale (Phrygian dominant)
    pub const SPANISH: ScalePattern = ScalePattern {
        intervals: &[0, 1, 4, 5, 7, 8, 10, 12],
    };

    /// Flamenco scale
    pub const FLAMENCO: ScalePattern = ScalePattern {
        intervals: &[0, 1, 3, 4, 5, 7, 8, 10, 11, 12],
    };

    // === ENIGMATIC & EXOTIC SCALES ===

    /// Enigmatic scale
    pub const ENIGMATIC: ScalePattern = ScalePattern {
        intervals: &[0, 1, 4, 6, 8, 10, 11, 12],
    };

    /// Neapolitan major scale
    pub const NEAPOLITAN_MAJOR: ScalePattern = ScalePattern {
        intervals: &[0, 1, 3, 5, 7, 9, 11, 12],
    };

    /// Neapolitan minor scale
    pub const NEAPOLITAN_MINOR: ScalePattern = ScalePattern {
        intervals: &[0, 1, 3, 5, 7, 8, 11, 12],
    };

    /// Prometheus scale
    pub const PROMETHEUS: ScalePattern = ScalePattern {
        intervals: &[0, 2, 4, 6, 9, 10, 12],
    };

    /// Tritone scale
    pub const TRITONE: ScalePattern = ScalePattern {
        intervals: &[0, 1, 4, 6, 7, 10, 12],
    };

    /// Augmented scale (hexatonic)
    pub const AUGMENTED: ScalePattern = ScalePattern {
        intervals: &[0, 3, 4, 7, 8, 11, 12],
    };

    // === PENTATONIC VARIATIONS ===

    /// Egyptian pentatonic (suspended pentatonic)
    pub const EGYPTIAN: ScalePattern = ScalePattern {
        intervals: &[0, 2, 5, 7, 10, 12],
    };

    /// Chinese pentatonic
    pub const CHINESE: ScalePattern = ScalePattern {
        intervals: &[0, 4, 6, 7, 11, 12],
    };

    /// Mongolian pentatonic
    pub const MONGOLIAN: ScalePattern = ScalePattern {
        intervals: &[0, 2, 4, 7, 9, 12],
    };

    // === MODERN & EXPERIMENTAL SCALES ===

    /// Lydian augmented scale
    pub const LYDIAN_AUGMENTED: ScalePattern = ScalePattern {
        intervals: &[0, 2, 4, 6, 8, 9, 11, 12],
    };

    /// Lydian dominant scale (acoustic scale, overtone scale)
    pub const LYDIAN_DOMINANT: ScalePattern = ScalePattern {
        intervals: &[0, 2, 4, 6, 7, 9, 10, 12],
    };

    /// Super Locrian (altered scale, diminished whole tone)
    pub const SUPER_LOCRIAN: ScalePattern = ScalePattern {
        intervals: &[0, 1, 3, 4, 6, 8, 10, 12],
    };

    /// Ultra Locrian (super super locrian)
    pub const ULTRA_LOCRIAN: ScalePattern = ScalePattern {
        intervals: &[0, 1, 3, 4, 6, 8, 9, 12],
    };

    /// Half diminished (Locrian #2)
    pub const HALF_DIMINISHED: ScalePattern = ScalePattern {
        intervals: &[0, 2, 3, 5, 6, 8, 10, 12],
    };
}

/// Generates a scale from a root note using a scale pattern
///
/// # Arguments
/// * `root` - The root frequency in Hz
/// * `pattern` - The scale pattern to use
///
/// # Returns
/// A Vec of frequencies representing the scale
///
/// # Example
/// ```
/// # use tunes::consts::notes::*;
/// use tunes::theory::core::{scale, ScalePattern};
/// use tunes::consts::notes::C4;
///
/// let c_major = scale(C4, &ScalePattern::MAJOR);
/// let a_minor_pent = scale(A3, &ScalePattern::MINOR_PENTATONIC);
/// ```
pub fn scale(root: f32, pattern: &ScalePattern) -> Vec<f32> {
    pattern
        .intervals
        .iter()
        .map(|&semitones| transpose(root, semitones))
        .collect()
}

/// Chord intervals (in semitones from root)
pub struct ChordPattern {
    pub intervals: &'static [i32],
}

impl ChordPattern {
    /// Major triad: R-M3-P5 (0-4-7)
    pub const MAJOR: ChordPattern = ChordPattern {
        intervals: &[0, 4, 7],
    };

    /// Minor triad: R-m3-P5 (0-3-7)
    pub const MINOR: ChordPattern = ChordPattern {
        intervals: &[0, 3, 7],
    };

    /// Diminished triad: R-m3-d5 (0-3-6)
    pub const DIMINISHED: ChordPattern = ChordPattern {
        intervals: &[0, 3, 6],
    };

    /// Augmented triad: R-M3-A5 (0-4-8)
    pub const AUGMENTED: ChordPattern = ChordPattern {
        intervals: &[0, 4, 8],
    };

    /// Major 7th: R-M3-P5-M7 (0-4-7-11)
    pub const MAJOR7: ChordPattern = ChordPattern {
        intervals: &[0, 4, 7, 11],
    };

    /// Minor 7th: R-m3-P5-m7 (0-3-7-10)
    pub const MINOR7: ChordPattern = ChordPattern {
        intervals: &[0, 3, 7, 10],
    };

    /// Dominant 7th: R-M3-P5-m7 (0-4-7-10)
    pub const DOMINANT7: ChordPattern = ChordPattern {
        intervals: &[0, 4, 7, 10],
    };

    /// Diminished 7th: R-m3-d5-d7 (0-3-6-9)
    pub const DIMINISHED7: ChordPattern = ChordPattern {
        intervals: &[0, 3, 6, 9],
    };

    /// Half-diminished 7th: R-m3-d5-m7 (0-3-6-10)
    pub const HALF_DIMINISHED7: ChordPattern = ChordPattern {
        intervals: &[0, 3, 6, 10],
    };

    /// Sus2: R-M2-P5 (0-2-7)
    pub const SUS2: ChordPattern = ChordPattern {
        intervals: &[0, 2, 7],
    };

    /// Sus4: R-P4-P5 (0-5-7)
    pub const SUS4: ChordPattern = ChordPattern {
        intervals: &[0, 5, 7],
    };

    /// Add9: R-M3-P5-M9 (0-4-7-14)
    pub const ADD9: ChordPattern = ChordPattern {
        intervals: &[0, 4, 7, 14],
    };

    /// 9th chord: R-M3-P5-m7-M9 (0-4-7-10-14)
    pub const NINTH: ChordPattern = ChordPattern {
        intervals: &[0, 4, 7, 10, 14],
    };

    /// Power chord (5th): R-P5 (0-7)
    pub const POWER: ChordPattern = ChordPattern { intervals: &[0, 7] };

    /// Power chord with octave: R-P5-R8 (0-7-12)
    pub const POWER_OCTAVE: ChordPattern = ChordPattern {
        intervals: &[0, 7, 12],
    };
}

/// Generates a chord from a root note using a chord pattern
///
/// # Arguments
/// * `root` - The root frequency in Hz
/// * `pattern` - The chord pattern to use
///
/// # Returns
/// A Vec of frequencies representing the chord
///
/// # Example
/// ```
/// # use tunes::consts::notes::*;
/// use tunes::theory::core::{chord, ChordPattern};
/// use tunes::consts::notes::C4;
///
/// let c_major = chord(C4, &ChordPattern::MAJOR);
/// let g_dominant7 = chord(G3, &ChordPattern::DOMINANT7);
/// ```
pub fn chord(root: f32, pattern: &ChordPattern) -> Vec<f32> {
    pattern
        .intervals
        .iter()
        .map(|&semitones| transpose(root, semitones))
        .collect()
}

/// Generates a diatonic chord progression from a scale
///
/// # Arguments
/// * `root` - The root frequency of the scale
/// * `scale_pattern` - The scale pattern to use
/// * `degrees` - The scale degrees to build chords on (1-based)
/// * `chord_type` - Whether to use triads or seventh chords
///
/// # Returns
/// A Vec of chord Vecs
///
/// # Example
/// ```
/// # use tunes::consts::notes::*;
/// use tunes::theory::core::{progression, ScalePattern, ProgressionType};
/// use tunes::consts::notes::C4;
///
/// // I-V-vi-IV progression in C major
/// let prog = progression(C4, &ScalePattern::MAJOR, &[1, 5, 6, 4], ProgressionType::Triads);
/// ```
pub fn progression(
    root: f32,
    scale_pattern: &ScalePattern,
    degrees: &[usize],
    progression_type: ProgressionType,
) -> Vec<Vec<f32>> {
    let scale_notes = scale(root, scale_pattern);

    degrees
        .iter()
        .map(|&degree| {
            let index = (degree - 1) % scale_notes.len();
            let chord_root = scale_notes[index];

            // Determine chord quality based on scale degree in major/minor
            let chord_pattern = match progression_type {
                ProgressionType::Triads => determine_triad(scale_pattern, degree),
                ProgressionType::Sevenths => determine_seventh(scale_pattern, degree),
            };

            chord(chord_root, chord_pattern)
        })
        .collect()
}

pub enum ProgressionType {
    Triads,
    Sevenths,
}

fn determine_triad(scale_pattern: &ScalePattern, degree: usize) -> &'static ChordPattern {
    // For major scale: I, ii, iii, IV, V, vi, vii°
    if scale_pattern.intervals == ScalePattern::MAJOR.intervals {
        match degree {
            1 | 4 | 5 => &ChordPattern::MAJOR, // I, IV, V
            2 | 3 | 6 => &ChordPattern::MINOR, // ii, iii, vi
            7 => &ChordPattern::DIMINISHED,    // vii°
            _ => &ChordPattern::MAJOR,
        }
    }
    // For minor scale: i, ii°, III, iv, v, VI, VII
    else if scale_pattern.intervals == ScalePattern::MINOR.intervals {
        match degree {
            3 | 6 | 7 => &ChordPattern::MAJOR, // III, VI, VII
            1 | 4 | 5 => &ChordPattern::MINOR, // i, iv, v
            2 => &ChordPattern::DIMINISHED,    // ii°
            _ => &ChordPattern::MINOR,
        }
    } else {
        // Default to major for other scales
        &ChordPattern::MAJOR
    }
}

fn determine_seventh(scale_pattern: &ScalePattern, degree: usize) -> &'static ChordPattern {
    // For major scale: Imaj7, ii7, iii7, IVmaj7, V7, vi7, viiø7
    if scale_pattern.intervals == ScalePattern::MAJOR.intervals {
        match degree {
            1 | 4 => &ChordPattern::MAJOR7,       // Imaj7, IVmaj7
            5 => &ChordPattern::DOMINANT7,        // V7
            2 | 3 | 6 => &ChordPattern::MINOR7,   // ii7, iii7, vi7
            7 => &ChordPattern::HALF_DIMINISHED7, // viiø7
            _ => &ChordPattern::MAJOR7,
        }
    }
    // For minor scale: i7, iiø7, IIImaj7, iv7, v7, VImaj7, VII7
    else if scale_pattern.intervals == ScalePattern::MINOR.intervals {
        match degree {
            1 | 4 | 5 => &ChordPattern::MINOR7,   // i7, iv7, v7
            3 | 6 => &ChordPattern::MAJOR7,       // IIImaj7, VImaj7
            7 => &ChordPattern::DOMINANT7,        // VII7
            2 => &ChordPattern::HALF_DIMINISHED7, // iiø7
            _ => &ChordPattern::MINOR7,
        }
    } else {
        // Default to major 7 for other scales
        &ChordPattern::MAJOR7
    }
}

/// Transpose an entire sequence of notes
///
/// # Arguments
/// * `notes` - The sequence of frequencies to transpose
/// * `semitones` - Number of semitones to transpose (can be negative)
///
/// # Example
/// ```
/// # use tunes::consts::notes::*;
/// use tunes::theory::core::transpose_sequence;
///
/// let melody = vec![261.63, 293.66, 329.63]; // C4, D4, E4
/// let transposed = transpose_sequence(&melody, 2); // D4, E4, F#4
/// ```
pub fn transpose_sequence(notes: &[f32], semitones: i32) -> Vec<f32> {
    notes
        .iter()
        .map(|&note| transpose(note, semitones))
        .collect()
}

/// Get a specific scale degree from a scale
///
/// # Arguments
/// * `root` - The root frequency
/// * `scale_pattern` - The scale pattern
/// * `degree` - The scale degree (1-based)
///
/// # Example
/// ```
/// # use tunes::consts::notes::*;
/// use tunes::theory::core::{scale_degree, ScalePattern};
/// use tunes::consts::notes::C4;
///
/// let fifth = scale_degree(C4, &ScalePattern::MAJOR, 5); // G4
/// ```
pub fn scale_degree(root: f32, scale_pattern: &ScalePattern, degree: usize) -> f32 {
    let scale_notes = scale(root, scale_pattern);
    let index = (degree - 1) % scale_notes.len();
    scale_notes[index]
}

//
// ============================================================================
// CHORD VOICING & VOICE LEADING
// ============================================================================
//

/// Inverts a chord to a different position
///
/// # Arguments
/// * `chord` - The chord frequencies to invert
/// * `inversion` - Which inversion (0 = root position, 1 = first inversion, 2 = second, etc.)
///
/// # Example
/// ```
/// # use tunes::consts::chords::*;
/// use tunes::theory::core::chord_inversion;
///
/// // C major triad: C-E-G (root position)
/// let root_position = vec![261.63, 329.63, 392.00];
///
/// // First inversion: E-G-C (E in bass)
/// let first = chord_inversion(&root_position, 1);
/// assert!((first[0] - 329.63).abs() < 0.1); // E is now lowest
///
/// // Second inversion: G-C-E (G in bass)
/// let second = chord_inversion(&root_position, 2);
/// assert!((second[0] - 392.00).abs() < 0.1); // G is now lowest
/// ```
pub fn chord_inversion(chord: &[f32], inversion: usize) -> Vec<f32> {
    if chord.is_empty() {
        return vec![];
    }

    let mut inverted = chord.to_vec();
    let inv = inversion % chord.len(); // Handle inversions > chord size

    // Move `inv` notes from bottom to top octave
    for _ in 0..inv {
        let note = inverted.remove(0);
        inverted.push(note * 2.0); // Octave up
    }

    inverted
}

/// Creates a slash chord (chord with specific bass note)
///
/// # Arguments
/// * `chord` - The chord frequencies
/// * `bass` - The bass note frequency to use
///
/// # Example
/// ```
/// # use tunes::consts::chords::*;
/// # use tunes::consts::notes::*;
/// use tunes::theory::core::chord_over_bass;
///
/// // C/E - C major chord over E bass
/// let c_major = vec![C4, E4, G4];
/// let slash = chord_over_bass(&c_major, E3);
/// assert_eq!(slash[0], E3); // E3 is the bass
/// ```
pub fn chord_over_bass(chord: &[f32], bass: f32) -> Vec<f32> {
    let mut result = vec![bass];

    // Add chord notes, filtering out any that are the bass note (same pitch class)
    for &note in chord {
        // Check if it's not the same pitch class as bass (within an octave)
        let ratio = note / bass;
        let octaves = ratio.log2();
        let remainder = octaves - octaves.floor();

        // If not the same pitch class, include it
        if remainder.abs() > 0.01 && (1.0 - remainder).abs() > 0.01 {
            result.push(note);
        }
    }

    result
}

/// Finds the smoothest voice leading from one chord to another
///
/// Uses the nearest voice leading algorithm: each voice moves to the
/// closest available note in the target chord.
///
/// # Arguments
/// * `from_chord` - Starting chord frequencies
/// * `to_chord` - Target chord frequencies
///
/// # Example
/// ```
/// # use tunes::consts::notes::*;
/// use tunes::theory::core::voice_lead;
///
/// // C major to F major with smooth voice leading
/// let c_maj = vec![C4, E4, G4];
/// let f_maj = vec![F4, A4, C5];
///
/// let smooth = voice_lead(&c_maj, &f_maj);
/// // Result: [C4, F4, A4] - minimal movement
/// // C4 stays, E4→F4 (up 1), G4→A4 (up 2)
/// ```
pub fn voice_lead(from_chord: &[f32], to_chord: &[f32]) -> Vec<f32> {
    if to_chord.is_empty() {
        return vec![];
    }
    if from_chord.is_empty() {
        return to_chord.to_vec();
    }

    let mut result = Vec::with_capacity(from_chord.len());
    let mut used = vec![false; to_chord.len()];

    // For each voice in the source chord
    for &from_note in from_chord {
        let mut best_idx = 0;
        let mut best_distance = f32::MAX;

        // Find the closest unused note in the target chord
        // considering octave equivalents
        for (i, &to_note) in to_chord.iter().enumerate() {
            if used[i] {
                continue;
            }

            // Check distance to this note and its octave transpositions
            for octave_shift in -2..=2 {
                let transposed = to_note * 2.0f32.powi(octave_shift);
                let distance = (from_note - transposed).abs();

                if distance < best_distance {
                    best_distance = distance;
                    best_idx = i;
                }
            }
        }

        // Find the best octave for the chosen note
        let chosen_note = to_chord[best_idx];
        let mut best_octave_note = chosen_note;
        let mut best_octave_distance = (from_note - chosen_note).abs();

        for octave_shift in -2..=2 {
            let transposed = chosen_note * 2.0f32.powi(octave_shift);
            let distance = (from_note - transposed).abs();

            if distance < best_octave_distance {
                best_octave_distance = distance;
                best_octave_note = transposed;
            }
        }

        result.push(best_octave_note);
        used[best_idx] = true;
    }

    result
}

/// Closes the spacing of a chord (moves all notes within one octave)
///
/// # Example
/// ```
/// use tunes::theory::core::close_voicing;
///
/// // Wide spacing
/// let wide = vec![130.0, 330.0, 520.0]; // C2, E3, C4
///
/// // Close spacing (all within one octave of lowest note)
/// let close = close_voicing(&wide);
/// // Result: [130.0, 164.0, 195.0] - C2, E2, G2
/// ```
pub fn close_voicing(chord: &[f32]) -> Vec<f32> {
    if chord.is_empty() {
        return vec![];
    }

    let mut sorted = chord.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let root = sorted[0];
    let mut result = vec![root];

    // Move all notes into the octave above root
    for &note in &sorted[1..] {
        let mut adjusted = note;

        // Bring down octaves until within one octave of root
        // Use small tolerance to handle floating point precision
        while adjusted > root * 2.0 + 0.1 {
            adjusted /= 2.0;
        }

        // If still below root, bring up
        while adjusted < root - 0.1 {
            adjusted *= 2.0;
        }

        // Clamp to valid range to handle floating point precision
        adjusted = adjusted.max(root).min(root * 2.0);

        result.push(adjusted);
    }

    // Sort and remove duplicates
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());
    result.dedup_by(|a, b| (*a - *b).abs() < 0.1);

    result
}

/// Opens the spacing of a chord (spreads notes across multiple octaves)
///
/// # Example
/// ```
/// use tunes::theory::core::open_voicing;
///
/// // Close spacing
/// let close = vec![261.63, 329.63, 392.00]; // C4-E4-G4
///
/// // Open spacing (drop middle voices down an octave)
/// let open = open_voicing(&close);
/// // Result might be: [261.63, 164.81, 392.00] - C4-E3-G4
/// ```
pub fn open_voicing(chord: &[f32]) -> Vec<f32> {
    if chord.len() < 3 {
        return chord.to_vec();
    }

    let mut result = chord.to_vec();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Drop-2 voicing: drop the second-highest note down an octave
    let len = result.len();
    if len >= 2 {
        result[len - 2] /= 2.0;
    }

    // Re-sort after dropping
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());

    result
}

/// Calculates the total voice leading distance between two chords
///
/// Returns the sum of all voice movements in Hz. Lower is smoother.
///
/// # Example
/// ```
/// use tunes::theory::core::voice_leading_distance;
///
/// let c_maj = vec![261.63, 329.63, 392.00];
/// let f_maj = vec![349.23, 440.00, 523.25];
///
/// let distance = voice_leading_distance(&c_maj, &f_maj);
/// // Returns total Hz movement across all voices
/// ```
pub fn voice_leading_distance(from_chord: &[f32], to_chord: &[f32]) -> f32 {
    if from_chord.len() != to_chord.len() {
        return f32::MAX; // Incompatible chord sizes
    }

    let voiced = voice_lead(from_chord, to_chord);

    from_chord
        .iter()
        .zip(voiced.iter())
        .map(|(from, to)| (from - to).abs())
        .sum()
}

// ============================================================================
// Interval Semantics
// ============================================================================

/// Musical interval constants for expressive transposition and harmony.
///
/// Instead of thinking in semitones, use named intervals that match music theory terminology.
///
/// # Example
/// ```
/// # use tunes::theory::core::{transpose, Interval};
/// # use tunes::consts::notes::C4;
/// // Clear and expressive
/// let perfect_fifth = transpose(C4, Interval::PERFECT_FIFTH);
/// let major_third = transpose(C4, Interval::MAJOR_THIRD);
///
/// // Instead of cryptic semitone counts
/// // let perfect_fifth = transpose(C4, 7);  // What interval is 7?
/// ```
pub struct Interval;

impl Interval {
    // Perfect intervals (1, 4, 5, 8)
    /// Perfect unison (0 semitones)
    pub const UNISON: i32 = 0;
    /// Perfect unison (0 semitones) - alias for UNISON
    pub const PERFECT_UNISON: i32 = 0;
    /// Perfect fourth (5 semitones)
    pub const PERFECT_FOURTH: i32 = 5;
    /// Perfect fifth (7 semitones)
    pub const PERFECT_FIFTH: i32 = 7;
    /// Perfect octave (12 semitones)
    pub const OCTAVE: i32 = 12;
    /// Perfect octave (12 semitones) - alias for OCTAVE
    pub const PERFECT_OCTAVE: i32 = 12;

    // Major intervals (2, 3, 6, 7)
    /// Major second (2 semitones) - whole step
    pub const MAJOR_SECOND: i32 = 2;
    /// Major third (4 semitones)
    pub const MAJOR_THIRD: i32 = 4;
    /// Major sixth (9 semitones)
    pub const MAJOR_SIXTH: i32 = 9;
    /// Major seventh (11 semitones)
    pub const MAJOR_SEVENTH: i32 = 11;

    // Minor intervals (2, 3, 6, 7)
    /// Minor second (1 semitone) - half step
    pub const MINOR_SECOND: i32 = 1;
    /// Minor third (3 semitones)
    pub const MINOR_THIRD: i32 = 3;
    /// Minor sixth (8 semitones)
    pub const MINOR_SIXTH: i32 = 8;
    /// Minor seventh (10 semitones)
    pub const MINOR_SEVENTH: i32 = 10;

    // Augmented intervals
    /// Augmented unison (1 semitone) - same as MINOR_SECOND
    pub const AUGMENTED_UNISON: i32 = 1;
    /// Augmented second (3 semitones) - same as MINOR_THIRD
    pub const AUGMENTED_SECOND: i32 = 3;
    /// Augmented fourth (6 semitones) - tritone
    pub const AUGMENTED_FOURTH: i32 = 6;
    /// Augmented fifth (8 semitones) - same as MINOR_SIXTH
    pub const AUGMENTED_FIFTH: i32 = 8;

    // Diminished intervals
    /// Diminished third (2 semitones) - same as MAJOR_SECOND
    pub const DIMINISHED_THIRD: i32 = 2;
    /// Diminished fourth (4 semitones) - same as MAJOR_THIRD
    pub const DIMINISHED_FOURTH: i32 = 4;
    /// Diminished fifth (6 semitones) - tritone
    pub const DIMINISHED_FIFTH: i32 = 6;
    /// Diminished seventh (9 semitones) - same as MAJOR_SIXTH
    pub const DIMINISHED_SEVENTH: i32 = 9;
    /// Diminished octave (11 semitones) - same as MAJOR_SEVENTH
    pub const DIMINISHED_OCTAVE: i32 = 11;

    // Common aliases
    /// Tritone (6 semitones) - the interval that divides the octave in half
    pub const TRITONE: i32 = 6;
    /// Half step (1 semitone) - alias for MINOR_SECOND
    pub const HALF_STEP: i32 = 1;
    /// Whole step (2 semitones) - alias for MAJOR_SECOND
    pub const WHOLE_STEP: i32 = 2;
}

/// Calculate the interval (in semitones) between two frequencies.
///
/// Returns the number of semitones from the first note to the second.
/// Positive values mean the second note is higher, negative means lower.
///
/// # Example
/// ```
/// # use tunes::theory::core::{interval_between, Interval};
/// # use tunes::consts::notes::{C4, G4, E4};
/// let fifth = interval_between(C4, G4);
/// assert_eq!(fifth, 7);  // Perfect fifth
///
/// let third = interval_between(C4, E4);
/// assert_eq!(third, 4);  // Major third
///
/// // Works in reverse too
/// let fourth_down = interval_between(G4, C4);
/// assert_eq!(fourth_down, -7);  // Perfect fifth down
/// ```
pub fn interval_between(from: f32, to: f32) -> i32 {
    // Calculate semitones using the formula: semitones = 12 * log2(freq_ratio)
    let ratio = to / from;
    (12.0 * ratio.log2()).round() as i32
}

/// Get a human-readable name for an interval (in semitones).
///
/// Returns the most common name for the interval. For enharmonic equivalents
/// (intervals with the same semitone count but different names), this returns
/// the most frequently used name.
///
/// # Example
/// ```
/// # use tunes::theory::core::interval_name;
/// assert_eq!(interval_name(0), "Perfect Unison");
/// assert_eq!(interval_name(7), "Perfect Fifth");
/// assert_eq!(interval_name(4), "Major Third");
/// assert_eq!(interval_name(6), "Tritone");
/// assert_eq!(interval_name(-5), "Perfect Fourth (down)");
/// ```
pub fn interval_name(semitones: i32) -> &'static str {
    match semitones {
        0 => "Perfect Unison",
        1 => "Minor Second",
        2 => "Major Second",
        3 => "Minor Third",
        4 => "Major Third",
        5 => "Perfect Fourth",
        6 => "Tritone",
        7 => "Perfect Fifth",
        8 => "Minor Sixth",
        9 => "Major Sixth",
        10 => "Minor Seventh",
        11 => "Major Seventh",
        12 => "Perfect Octave",
        -1 => "Minor Second (down)",
        -2 => "Major Second (down)",
        -3 => "Minor Third (down)",
        -4 => "Major Third (down)",
        -5 => "Perfect Fourth (down)",
        -6 => "Tritone (down)",
        -7 => "Perfect Fifth (down)",
        -8 => "Minor Sixth (down)",
        -9 => "Major Sixth (down)",
        -10 => "Minor Seventh (down)",
        -11 => "Major Seventh (down)",
        -12 => "Perfect Octave (down)",
        _ if semitones > 12 => {
            let remainder = semitones % 12;
            if remainder == 0 {
                return "Multiple Octaves";
            }
            "Compound Interval"
        }
        _ if semitones < -12 => {
            let remainder = semitones.abs() % 12;
            if remainder == 0 {
                return "Multiple Octaves (down)";
            }
            "Compound Interval (down)"
        }
        _ => "Unknown Interval",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let c4 = 261.63;
        let d4 = transpose(c4, 2);
        assert!((d4 - 293.66).abs() < 0.1);
    }

    #[test]
    fn test_scale_generation() {
        let c4 = 261.63;
        let c_major = scale(c4, &ScalePattern::MAJOR);
        assert_eq!(c_major.len(), 8); // 8 notes including octave
    }

    #[test]
    fn test_chord_generation() {
        let c4 = 261.63;
        let c_major_chord = chord(c4, &ChordPattern::MAJOR);
        assert_eq!(c_major_chord.len(), 3); // Root, 3rd, 5th
    }

    #[test]
    fn test_all_scales_generate() {
        let c4 = 261.63;

        // Test that all scale patterns generate valid scales
        let scales = vec![
            // Western
            (&ScalePattern::MAJOR, 8),
            (&ScalePattern::MINOR, 8),
            (&ScalePattern::HARMONIC_MINOR, 8),
            (&ScalePattern::MELODIC_MINOR, 8),
            (&ScalePattern::MAJOR_PENTATONIC, 6),
            (&ScalePattern::MINOR_PENTATONIC, 6),
            (&ScalePattern::BLUES, 7),
            (&ScalePattern::CHROMATIC, 13),
            (&ScalePattern::WHOLE_TONE, 7),
            // Modes
            (&ScalePattern::DORIAN, 8),
            (&ScalePattern::PHRYGIAN, 8),
            (&ScalePattern::LYDIAN, 8),
            (&ScalePattern::MIXOLYDIAN, 8),
            (&ScalePattern::LOCRIAN, 8),
            // Jazz & Bebop
            (&ScalePattern::BEBOP_MAJOR, 9),
            (&ScalePattern::BEBOP_DOMINANT, 9),
            (&ScalePattern::BEBOP_MINOR, 9),
            (&ScalePattern::ALTERED, 8),
            (&ScalePattern::DIMINISHED_HALF_WHOLE, 9),
            (&ScalePattern::DIMINISHED_WHOLE_HALF, 9),
            // Japanese
            (&ScalePattern::HIRAJOSHI, 6),
            (&ScalePattern::IN_SEN, 6),
            (&ScalePattern::IWATO, 6),
            (&ScalePattern::YO, 6),
            (&ScalePattern::KUMOI, 6),
            // Middle Eastern
            (&ScalePattern::HIJAZ, 8),
            (&ScalePattern::DOUBLE_HARMONIC, 8),
            (&ScalePattern::PHRYGIAN_DOMINANT, 8),
            (&ScalePattern::PERSIAN, 8),
            // Indian
            (&ScalePattern::BHAIRAV, 8),
            (&ScalePattern::KAFI, 8),
            (&ScalePattern::BHAIRAVI, 8),
            (&ScalePattern::PURVI, 8),
            (&ScalePattern::MARVA, 8),
            // Hungarian & Gypsy
            (&ScalePattern::HUNGARIAN_MINOR, 8),
            (&ScalePattern::HUNGARIAN_MAJOR, 8),
            (&ScalePattern::GYPSY, 8),
            // Spanish & Flamenco
            (&ScalePattern::SPANISH, 8),
            (&ScalePattern::FLAMENCO, 10),
            // Enigmatic & Exotic
            (&ScalePattern::ENIGMATIC, 8),
            (&ScalePattern::NEAPOLITAN_MAJOR, 8),
            (&ScalePattern::NEAPOLITAN_MINOR, 8),
            (&ScalePattern::PROMETHEUS, 7),
            (&ScalePattern::TRITONE, 7),
            (&ScalePattern::AUGMENTED, 7),
            // Pentatonic variations
            (&ScalePattern::EGYPTIAN, 6),
            (&ScalePattern::CHINESE, 6),
            (&ScalePattern::MONGOLIAN, 6),
            // Modern & Experimental
            (&ScalePattern::LYDIAN_AUGMENTED, 8),
            (&ScalePattern::LYDIAN_DOMINANT, 8),
            (&ScalePattern::SUPER_LOCRIAN, 8),
            (&ScalePattern::ULTRA_LOCRIAN, 8),
            (&ScalePattern::HALF_DIMINISHED, 8),
        ];

        for (pattern, expected_len) in scales {
            let generated = scale(c4, pattern);
            assert_eq!(generated.len(), expected_len);
            // First note should be root
            assert!((generated[0] - c4).abs() < 0.01);
            // Last note should be octave (2x frequency)
            assert!((generated[expected_len - 1] - c4 * 2.0).abs() < 1.0);
        }
    }

    #[test]
    fn test_japanese_scales_unique() {
        let c4 = 261.63;
        let hirajoshi = scale(c4, &ScalePattern::HIRAJOSHI);
        let in_sen = scale(c4, &ScalePattern::IN_SEN);
        let iwato = scale(c4, &ScalePattern::IWATO);

        // Each Japanese scale should be different
        assert_ne!(hirajoshi, in_sen);
        assert_ne!(in_sen, iwato);
        assert_ne!(hirajoshi, iwato);
    }

    #[test]
    fn test_middle_eastern_scales() {
        let a4 = 440.0;
        let hijaz = scale(a4, &ScalePattern::HIJAZ);

        // Hijaz should have characteristic augmented second (3 semitones)
        // between 2nd and 3rd degrees
        assert_eq!(hijaz.len(), 8);
        let interval = hijaz[2] / hijaz[1];
        assert!((interval - 2.0f32.powf(3.0 / 12.0)).abs() < 0.01);
    }

    // ========================================================================
    // VOICING & VOICE LEADING TESTS
    // ========================================================================

    #[test]
    fn test_chord_inversion_basic() {
        let c_major = vec![261.63, 329.63, 392.00]; // C-E-G

        // Root position (inversion 0)
        let root = chord_inversion(&c_major, 0);
        assert_eq!(root.len(), 3);
        assert!((root[0] - 261.63).abs() < 0.1); // C is lowest

        // First inversion (E-G-C)
        let first = chord_inversion(&c_major, 1);
        assert_eq!(first.len(), 3);
        assert!((first[0] - 329.63).abs() < 0.1); // E is lowest
        assert!((first[2] - 523.26).abs() < 1.0); // C is highest (octave up)

        // Second inversion (G-C-E)
        let second = chord_inversion(&c_major, 2);
        assert_eq!(second.len(), 3);
        assert!((second[0] - 392.00).abs() < 0.1); // G is lowest
    }

    #[test]
    fn test_chord_inversion_wrapping() {
        let triad = vec![100.0, 125.0, 150.0];

        // Inversion 3 should wrap to inversion 0 for a triad
        let inv3 = chord_inversion(&triad, 3);
        let inv0 = chord_inversion(&triad, 0);
        assert_eq!(inv3, inv0);

        // Inversion 4 should wrap to inversion 1
        let inv4 = chord_inversion(&triad, 4);
        let inv1 = chord_inversion(&triad, 1);
        assert_eq!(inv4, inv1);
    }

    #[test]
    fn test_chord_over_bass() {
        let c_major = vec![261.63, 329.63, 392.00]; // C4-E4-G4

        // C/E - C major over E bass
        let slash = chord_over_bass(&c_major, 164.81); // E3
        assert_eq!(slash[0], 164.81); // E3 is bass
        assert!(slash.contains(&261.63)); // C4 present
        assert!(slash.contains(&392.00)); // G4 present
        // E4 should be filtered out (same pitch class as bass)
    }

    #[test]
    fn test_voice_lead_smooth() {
        // C major to F major - should have smooth voice leading
        let c_maj = vec![261.63, 329.63, 392.00]; // C4-E4-G4
        let f_maj = vec![349.23, 440.00, 523.25]; // F4-A4-C5

        let smooth = voice_lead(&c_maj, &f_maj);

        // Should be 3 notes
        assert_eq!(smooth.len(), 3);

        // Check that we got reasonable voice leading
        // C4 should probably stay close (maybe go to C4 or C5)
        // E4 should go to F4 (up 1 semitone)
        // G4 should go to A4 (up 2 semitones)
        assert!(smooth.iter().any(|&n| (n - 261.63).abs() < 10.0 || (n - 523.25).abs() < 10.0)); // C somewhere
        assert!(smooth.iter().any(|&n| (n - 349.23).abs() < 10.0)); // F4
        assert!(smooth.iter().any(|&n| (n - 440.00).abs() < 10.0)); // A4
    }

    #[test]
    fn test_voice_lead_empty_chords() {
        let empty: Vec<f32> = vec![];
        let chord = vec![261.63, 329.63, 392.00];

        // From empty should return target chord
        let result1 = voice_lead(&empty, &chord);
        assert_eq!(result1, chord);

        // To empty should return empty
        let result2 = voice_lead(&chord, &empty);
        assert_eq!(result2.len(), 0);
    }

    #[test]
    fn test_close_voicing() {
        // Wide spacing across multiple octaves
        let wide = vec![130.81, 329.63, 523.25]; // C2-E3-C4

        let close = close_voicing(&wide);

        // All notes should be within one octave of the lowest
        let lowest = close[0];
        for &note in &close {
            assert!(note >= lowest);
            assert!(note <= lowest * 2.0);
        }

        // Should still have 3 notes
        assert_eq!(close.len(), 3);
    }

    #[test]
    fn test_close_voicing_already_close() {
        let already_close = vec![261.63, 329.63, 392.00]; // C4-E4-G4

        let close = close_voicing(&already_close);

        // Should remain essentially the same
        assert_eq!(close.len(), 3);
        assert!((close[0] - 261.63).abs() < 0.1);
    }

    #[test]
    fn test_open_voicing() {
        let close = vec![261.63, 329.63, 392.00]; // C4-E4-G4

        let open = open_voicing(&close);

        // Should have 3 notes
        assert_eq!(open.len(), 3);

        // Lowest note should still be C4 (approximately)
        assert!((open[0] - 164.81).abs() < 10.0 || (open[0] - 261.63).abs() < 10.0);

        // Should span more than one octave
        let span = open[2] / open[0];
        assert!(span > 2.0); // More than one octave
    }

    #[test]
    fn test_voice_leading_distance() {
        let c_maj = vec![261.63, 329.63, 392.00];
        let c_maj_nearby = vec![261.63, 330.00, 393.00]; // Very close

        let distance_close = voice_leading_distance(&c_maj, &c_maj_nearby);

        // Should be small (notes barely moved)
        assert!(distance_close < 10.0);

        // Distant chord should have larger distance
        let f_maj = vec![349.23, 440.00, 523.25];
        let distance_far = voice_leading_distance(&c_maj, &f_maj);

        assert!(distance_far > distance_close);
    }

    #[test]
    fn test_voice_leading_distance_incompatible() {
        let triad = vec![261.63, 329.63, 392.00];
        let seventh = vec![261.63, 329.63, 392.00, 466.16];

        // Different sized chords should return MAX
        let distance = voice_leading_distance(&triad, &seventh);
        assert_eq!(distance, f32::MAX);
    }

    // ========================================================================
    // Interval Tests
    // ========================================================================

    #[test]
    fn test_interval_constants() {
        // Test perfect intervals
        assert_eq!(Interval::UNISON, 0);
        assert_eq!(Interval::PERFECT_FOURTH, 5);
        assert_eq!(Interval::PERFECT_FIFTH, 7);
        assert_eq!(Interval::OCTAVE, 12);

        // Test major intervals
        assert_eq!(Interval::MAJOR_SECOND, 2);
        assert_eq!(Interval::MAJOR_THIRD, 4);
        assert_eq!(Interval::MAJOR_SIXTH, 9);
        assert_eq!(Interval::MAJOR_SEVENTH, 11);

        // Test minor intervals
        assert_eq!(Interval::MINOR_SECOND, 1);
        assert_eq!(Interval::MINOR_THIRD, 3);
        assert_eq!(Interval::MINOR_SIXTH, 8);
        assert_eq!(Interval::MINOR_SEVENTH, 10);

        // Test tritone
        assert_eq!(Interval::TRITONE, 6);
        assert_eq!(Interval::AUGMENTED_FOURTH, 6);
        assert_eq!(Interval::DIMINISHED_FIFTH, 6);

        // Test aliases
        assert_eq!(Interval::HALF_STEP, 1);
        assert_eq!(Interval::WHOLE_STEP, 2);
    }

    #[test]
    fn test_interval_between() {
        let c4 = 261.63;
        let d4 = transpose(c4, 2);
        let e4 = transpose(c4, 4);
        let g4 = transpose(c4, 7);
        let c5 = transpose(c4, 12);

        // Test ascending intervals
        assert_eq!(interval_between(c4, c4), 0); // Unison
        assert_eq!(interval_between(c4, d4), 2); // Major second
        assert_eq!(interval_between(c4, e4), 4); // Major third
        assert_eq!(interval_between(c4, g4), 7); // Perfect fifth
        assert_eq!(interval_between(c4, c5), 12); // Octave

        // Test descending intervals
        assert_eq!(interval_between(g4, c4), -7); // Perfect fifth down
        assert_eq!(interval_between(c5, c4), -12); // Octave down
    }

    #[test]
    fn test_interval_between_with_constants() {
        let c4 = 261.63;

        // Create notes using Interval constants
        let e4 = transpose(c4, Interval::MAJOR_THIRD);
        let g4 = transpose(c4, Interval::PERFECT_FIFTH);

        // Verify the intervals
        assert_eq!(interval_between(c4, e4), Interval::MAJOR_THIRD);
        assert_eq!(interval_between(c4, g4), Interval::PERFECT_FIFTH);
        assert_eq!(interval_between(e4, g4), Interval::MINOR_THIRD);
    }

    #[test]
    fn test_interval_name() {
        // Test perfect intervals
        assert_eq!(interval_name(0), "Perfect Unison");
        assert_eq!(interval_name(5), "Perfect Fourth");
        assert_eq!(interval_name(7), "Perfect Fifth");
        assert_eq!(interval_name(12), "Perfect Octave");

        // Test major intervals
        assert_eq!(interval_name(2), "Major Second");
        assert_eq!(interval_name(4), "Major Third");
        assert_eq!(interval_name(9), "Major Sixth");
        assert_eq!(interval_name(11), "Major Seventh");

        // Test minor intervals
        assert_eq!(interval_name(1), "Minor Second");
        assert_eq!(interval_name(3), "Minor Third");
        assert_eq!(interval_name(8), "Minor Sixth");
        assert_eq!(interval_name(10), "Minor Seventh");

        // Test tritone
        assert_eq!(interval_name(6), "Tritone");

        // Test descending intervals
        assert_eq!(interval_name(-5), "Perfect Fourth (down)");
        assert_eq!(interval_name(-7), "Perfect Fifth (down)");

        // Test compound intervals
        assert_eq!(interval_name(24), "Multiple Octaves");
        assert_eq!(interval_name(19), "Compound Interval"); // Octave + 7
        assert_eq!(interval_name(-24), "Multiple Octaves (down)");
    }

    #[test]
    fn test_transpose_with_interval_constants() {
        let c4 = 261.63;

        // Use Interval constants for clarity
        let e4 = transpose(c4, Interval::MAJOR_THIRD);
        let g4 = transpose(c4, Interval::PERFECT_FIFTH);
        let c5 = transpose(c4, Interval::OCTAVE);

        // Verify they produce expected frequencies
        assert!((e4 - transpose(c4, 4)).abs() < 0.01);
        assert!((g4 - transpose(c4, 7)).abs() < 0.01);
        assert!((c5 - transpose(c4, 12)).abs() < 0.01);

        // Test negative intervals (descending)
        let g3 = transpose(c4, -Interval::PERFECT_FOURTH);
        assert!((g3 - transpose(c4, -5)).abs() < 0.01);
    }

    #[test]
    fn test_interval_semantic_chord_building() {
        // Build a major chord using Interval constants
        let root = 261.63;
        let third = transpose(root, Interval::MAJOR_THIRD);
        let fifth = transpose(root, Interval::PERFECT_FIFTH);

        let manual_chord = vec![root, third, fifth];
        let library_chord = chord(root, &ChordPattern::MAJOR);

        // Should produce the same chord
        assert_eq!(manual_chord.len(), library_chord.len());
        for (manual, library) in manual_chord.iter().zip(library_chord.iter()) {
            assert!((manual - library).abs() < 0.01);
        }
    }

    #[test]
    fn test_interval_semantic_harmony() {
        // Create parallel thirds (common in classical music)
        let melody = vec![261.63, 293.66, 329.63]; // C4, D4, E4

        let harmony: Vec<f32> = melody
            .iter()
            .map(|&note| transpose(note, Interval::MAJOR_THIRD))
            .collect();

        // Each harmony note should be a major third above
        for (melody_note, harmony_note) in melody.iter().zip(harmony.iter()) {
            assert_eq!(interval_between(*melody_note, *harmony_note), Interval::MAJOR_THIRD);
        }
    }

    #[test]
    fn test_all_interval_constants_are_valid() {
        let c4 = 261.63;

        // Test that all interval constants can be used with transpose
        let intervals = vec![
            Interval::UNISON,
            Interval::MINOR_SECOND,
            Interval::MAJOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::TRITONE,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SIXTH,
            Interval::MAJOR_SIXTH,
            Interval::MINOR_SEVENTH,
            Interval::MAJOR_SEVENTH,
            Interval::OCTAVE,
        ];

        for interval in intervals {
            let result = transpose(c4, interval);
            assert!(result > 0.0);
            assert!(result < 10000.0); // Reasonable frequency range
        }
    }
}
