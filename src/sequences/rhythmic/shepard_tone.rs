/// Shepard Tone Sequence - Creates illusion of infinitely rising/falling pitch
///
/// A Shepard tone is an auditory illusion of a tone that seems to continually
/// rise or fall in pitch, yet never actually gets higher or lower. This function
/// generates the note pattern for creating this effect.
///
/// The sequence cycles through pitch classes while maintaining the illusion
/// of continuous movement.
///
/// # Musical Applications
/// - **Tension building**: Create sense of rising tension that never resolves
/// - **Ambient textures**: Hypnotic, meditation-inducing soundscapes
/// - **Film scores**: Famous in films like Dunkirk for building suspense
///
/// # Arguments
/// * `length` - Number of steps in the sequence
/// * `steps_per_octave` - How many steps before pattern repeats (typically 12 for semitones)
/// * `direction` - true for ascending, false for descending
///
/// # Returns
/// A vector of pitch class indices (0 to steps_per_octave-1)
///
/// # Example
/// ```
/// use tunes::sequences::shepard_tone;
///
/// // Create ascending Shepard tone sequence
/// let rising = generate(24, 12, true);
/// // Result: [0,1,2,3,4,5,6,7,8,9,10,11,0,1,2,3,4,5,6,7,8,9,10,11]
///
/// // Create descending sequence
/// let falling = generate(16, 12, false);
/// // Result: [0,11,10,9,8,7,6,5,4,3,2,1,0,11,10,9]
/// ```
pub fn generate(length: usize, steps_per_octave: u32, direction: bool) -> Vec<u32> {
    if length == 0 || steps_per_octave == 0 {
        return vec![];
    }

    (0..length)
        .map(|i| {
            if direction {
                // Ascending: 0, 1, 2, 3, ... steps_per_octave-1, 0, 1, ...
                (i as u32) % steps_per_octave
            } else {
                // Descending: 0, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 11, ...
                // Formula: (steps_per_octave - (i % steps_per_octave)) % steps_per_octave
                let offset = (i as u32) % steps_per_octave;
                if offset == 0 {
                    0
                } else {
                    steps_per_octave - offset
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shepard_tone_ascending() {
        let tone = generate(24, 12, true);

        assert_eq!(tone.len(), 24);

        // Should cycle through 0-11 twice
        assert_eq!(tone[0], 0);
        assert_eq!(tone[1], 1);
        assert_eq!(tone[11], 11);
        assert_eq!(tone[12], 0); // Wraps back
        assert_eq!(tone[23], 11);
    }

    #[test]
    fn test_shepard_tone_descending() {
        let tone = generate(13, 12, false);

        assert_eq!(tone.len(), 13);

        // Should go: 0, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
        assert_eq!(tone[0], 0);
        assert_eq!(tone[1], 11);
        assert_eq!(tone[2], 10);
        assert_eq!(tone[11], 1);
        assert_eq!(tone[12], 0); // Wraps back
    }

    #[test]
    fn test_shepard_tone_properties() {
        let ascending = generate(100, 12, true);
        let descending = generate(100, 12, false);

        // All values should be within range
        for &val in &ascending {
            assert!(val < 12);
        }
        for &val in &descending {
            assert!(val < 12);
        }

        // Should contain all pitch classes if length >= steps_per_octave
        let mut seen = vec![false; 12];
        for &val in ascending.iter().take(12) {
            seen[val as usize] = true;
        }
        assert!(seen.iter().all(|&x| x), "Should contain all 12 pitch classes");
    }

    #[test]
    fn test_shepard_tone_edge_cases() {
        // Empty sequence
        let empty = generate(0, 12, true);
        assert_eq!(empty, Vec::<u32>::new());

        // Zero steps per octave
        let zero_steps = generate(10, 0, true);
        assert_eq!(zero_steps, Vec::<u32>::new());

        // Single step
        let single = generate(1, 12, true);
        assert_eq!(single, vec![0]);

        // Different divisions (quarter tones)
        let quarter = generate(24, 24, true);
        assert_eq!(quarter.len(), 24);
        assert_eq!(quarter[23], 23);
    }

    #[test]
    fn test_shepard_tone_continuous_ascending() {
        // Verify ascending pattern is monotonic within each cycle
        let tone = generate(12, 12, true);

        for i in 0..11 {
            assert_eq!(tone[i] + 1, tone[i + 1], "Ascending should increment by 1");
        }
    }

    #[test]
    fn test_shepard_tone_continuous_descending() {
        // Verify descending pattern decrements within each cycle
        let tone = generate(13, 12, false);

        // Skip first element (0), check that each decrements
        for i in 1..12 {
            assert_eq!(
                tone[i],
                12 - i as u32,
                "Descending should decrement: tone[{}] = {}",
                i,
                tone[i]
            );
        }
    }
}

// ========== PRESETS ==========

/// Ascending Shepard tone - 24 steps, 12 steps per octave
pub fn ascending() -> Vec<u32> {
    generate(24, 12, true)
}

/// Descending Shepard tone - 24 steps, 12 steps per octave
pub fn descending() -> Vec<u32> {
    generate(24, 12, false)
}

/// Long ascending - 48 steps for extended illusion
pub fn long_ascending() -> Vec<u32> {
    generate(48, 12, true)
}

/// Microtonal ascending - 24 steps per octave for smoother transitions
pub fn microtonal() -> Vec<u32> {
    generate(48, 24, true)
}
