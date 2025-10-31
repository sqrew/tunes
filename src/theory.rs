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
/// # use tunes::notes::*;
/// use tunes::theory::transpose;
/// use tunes::notes::C4;
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
/// # use tunes::notes::*;
/// use tunes::theory::{scale, ScalePattern};
/// use tunes::notes::C4;
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
    pub const POWER: ChordPattern = ChordPattern {
        intervals: &[0, 7],
    };

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
/// # use tunes::notes::*;
/// use tunes::theory::{chord, ChordPattern};
/// use tunes::notes::C4;
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
/// # use tunes::notes::*;
/// use tunes::theory::{progression, ScalePattern, ProgressionType};
/// use tunes::notes::C4;
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
            1 | 4 | 5 => &ChordPattern::MAJOR,     // I, IV, V
            2 | 3 | 6 => &ChordPattern::MINOR,     // ii, iii, vi
            7 => &ChordPattern::DIMINISHED,        // vii°
            _ => &ChordPattern::MAJOR,
        }
    }
    // For minor scale: i, ii°, III, iv, v, VI, VII
    else if scale_pattern.intervals == ScalePattern::MINOR.intervals {
        match degree {
            3 | 6 | 7 => &ChordPattern::MAJOR,     // III, VI, VII
            1 | 4 | 5 => &ChordPattern::MINOR,     // i, iv, v
            2 => &ChordPattern::DIMINISHED,        // ii°
            _ => &ChordPattern::MINOR,
        }
    }
    else {
        // Default to major for other scales
        &ChordPattern::MAJOR
    }
}

fn determine_seventh(scale_pattern: &ScalePattern, degree: usize) -> &'static ChordPattern {
    // For major scale: Imaj7, ii7, iii7, IVmaj7, V7, vi7, viiø7
    if scale_pattern.intervals == ScalePattern::MAJOR.intervals {
        match degree {
            1 | 4 => &ChordPattern::MAJOR7,        // Imaj7, IVmaj7
            5 => &ChordPattern::DOMINANT7,         // V7
            2 | 3 | 6 => &ChordPattern::MINOR7,    // ii7, iii7, vi7
            7 => &ChordPattern::HALF_DIMINISHED7,  // viiø7
            _ => &ChordPattern::MAJOR7,
        }
    }
    // For minor scale: i7, iiø7, IIImaj7, iv7, v7, VImaj7, VII7
    else if scale_pattern.intervals == ScalePattern::MINOR.intervals {
        match degree {
            1 | 4 | 5 => &ChordPattern::MINOR7,    // i7, iv7, v7
            3 | 6 => &ChordPattern::MAJOR7,        // IIImaj7, VImaj7
            7 => &ChordPattern::DOMINANT7,         // VII7
            2 => &ChordPattern::HALF_DIMINISHED7,  // iiø7
            _ => &ChordPattern::MINOR7,
        }
    }
    else {
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
/// # use tunes::notes::*;
/// use tunes::theory::transpose_sequence;
///
/// let melody = vec![261.63, 293.66, 329.63]; // C4, D4, E4
/// let transposed = transpose_sequence(&melody, 2); // D4, E4, F#4
/// ```
pub fn transpose_sequence(notes: &[f32], semitones: i32) -> Vec<f32> {
    notes.iter().map(|&note| transpose(note, semitones)).collect()
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
/// # use tunes::notes::*;
/// use tunes::theory::{scale_degree, ScalePattern};
/// use tunes::notes::C4;
///
/// let fifth = scale_degree(C4, &ScalePattern::MAJOR, 5); // G4
/// ```
pub fn scale_degree(root: f32, scale_pattern: &ScalePattern, degree: usize) -> f32 {
    let scale_notes = scale(root, scale_pattern);
    let index = (degree - 1) % scale_notes.len();
    scale_notes[index]
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
}
