/// Generate Euclidean rhythm pattern - returns step indices where hits occur
///
/// Distributes `pulses` as evenly as possible across `steps` using Bjorklund's algorithm.
/// This is used extensively in music traditions worldwide and creates mathematically optimal rhythms.
///
/// # Arguments
/// * `pulses` - Number of hits/beats to distribute (k)
/// * `steps` - Total number of steps in the pattern (n)
///
/// # Returns
/// Vector of step indices (0-indexed) where hits occur
///
/// # Common Patterns
///
/// **Traditional World Music:**
/// - **(3, 8)**: Cuban tresillo - [0, 3, 6] or [0, 2, 5] depending on rotation
/// - **(5, 8)**: Cuban cinquillo - [0, 2, 3, 5, 7]
/// - **(5, 12)**: York-Samai pattern (Middle East)
/// - **(7, 16)**: Bossa nova clave variation
///
/// **Electronic/Dance:**
/// - **(4, 16)**: Four-on-floor kick - [0, 4, 8, 12]
/// - **(8, 16)**: Eighth note hi-hats - [0, 2, 4, 6, 8, 10, 12, 14]
/// - **(3, 16)**: Syncopated snare - [0, 5, 11]
/// - **(7, 16)**: Complex hi-hat - creates shifting pattern
///
/// **Try these combinations:**
/// - Kick (4,16) + Snare (3,16) + Hihat (7,16) = Interesting groove
/// - Kick (5,16) + Snare (3,16) + Hihat (13,16) = Complex polyrhythm
///
/// # Recipe: Complete Euclidean Drum Pattern
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences;
///
/// let mut comp = Composition::new(Tempo::new(120.0));
///
/// comp.track("euclidean_drums")
///     .drum_grid(16, 0.125)
///     .kick(&sequences::euclidean::generate(4, 16))     // Four-on-floor
///     .snare(&sequences::euclidean::generate(3, 16))    // Syncopated
///     .hihat(&sequences::euclidean::generate(7, 16))    // Complex
///     .clap(&sequences::euclidean::generate(2, 16));    // Backbeat feel
/// ```
///
/// # Examples
/// ```
/// # use tunes::composition::Composition;
/// # use tunes::composition::rhythm::Tempo;
/// use tunes::sequences;
///
/// // Classic Cuban tresillo pattern
/// let pattern = sequences::euclidean::generate(3, 8);
/// assert_eq!(pattern, vec![0, 2, 5]);
///
/// // Cuban cinquillo
/// let pattern = sequences::euclidean::generate(5, 8);
/// assert_eq!(pattern, vec![0, 2, 3, 5, 7]);
///
/// // Perfect for drum patterns:
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// comp.track("drums").drum_grid(16, 0.125)
///     .kick(&sequences::euclidean::generate(4, 16))    // Four-on-floor
///     .snare(&sequences::euclidean::generate(3, 16))   // Syncopated snare
///     .hihat(&sequences::euclidean::generate(7, 16));  // Complex hi-hat
/// ```
///
/// # Usage
/// ```
/// use tunes::sequences::euclidean;
///
/// // Custom parameters
/// let pattern = euclidean::generate(5, 16);
///
/// // Or use presets
/// let kick = euclidean::kick_four_floor();
/// ```

/// Generate Euclidean rhythm pattern - returns step indices where hits occur
///
/// See module-level documentation for details on Euclidean rhythms,
/// musical applications, and common patterns.
pub fn generate(pulses: usize, steps: usize) -> Vec<usize> {
    if pulses == 0 || steps == 0 || pulses > steps {
        return vec![];
    }

    // Generate the full pattern using Bjorklund's algorithm
    let pattern = bjorklund(pulses, steps);

    // Extract indices where pulses occur
    pattern
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == 1)
        .map(|(i, _)| i)
        .collect()
}

/// Generate Euclidean rhythm as a full binary pattern
///
/// Returns a vector where 1 represents a hit and 0 represents a rest.
/// Useful for visualization or when you need the complete pattern.
///
/// # Examples
/// ```
/// use tunes::sequences::euclidean;
/// let pattern = euclidean::pattern(5, 8);
/// // Returns: [1, 0, 1, 1, 0, 1, 1, 0]
/// ```
pub fn pattern(pulses: usize, steps: usize) -> Vec<u32> {
    bjorklund(pulses, steps)
}

/// Generate Euclidean rhythm using Bresenham-style algorithm
/// Returns a binary pattern where 1 = pulse, 0 = rest
/// This produces the canonical rotation of the Euclidean rhythm (starting with 1)
fn bjorklund(pulses: usize, steps: usize) -> Vec<u32> {
    if pulses == 0 {
        return vec![0; steps];
    }
    if pulses >= steps {
        return vec![1; steps];
    }

    let mut pattern = vec![0; steps];

    // Use Bresenham's line algorithm for even distribution
    // Start at steps/2 to get the best rounding behavior
    let mut error = steps / 2;

    for pattern_val in &mut pattern {
        error += pulses;
        if error >= steps {
            error -= steps;
            *pattern_val = 1;
        }
    }

    // Rotate so the pattern starts with 1 (canonical form)
    if let Some(first_one) = pattern.iter().position(|&x| x == 1) {
        pattern.rotate_left(first_one);
    }

    pattern
}

// ========== PRESETS ==========

// ========== KICK PATTERNS ==========

/// Four-on-floor kick pattern (4/16) - [0, 4, 8, 12]
pub fn kick_four_floor() -> Vec<usize> {
    generate(4, 16)
}

/// Syncopated kick pattern (5/16)
pub fn kick_syncopated() -> Vec<usize> {
    generate(5, 16)
}

/// Sparse kick pattern (3/16)
pub fn kick_sparse() -> Vec<usize> {
    generate(3, 16)
}

// ========== SNARE PATTERNS ==========

/// Standard snare pattern (2/16) - backbeat on 2 and 4
pub fn snare_standard() -> Vec<usize> {
    generate(2, 16)
}

/// Syncopated snare pattern (3/16)
pub fn snare_syncopated() -> Vec<usize> {
    generate(3, 16)
}

/// Complex snare pattern (5/16)
pub fn snare_complex() -> Vec<usize> {
    generate(5, 16)
}

// ========== HI-HAT PATTERNS ==========

/// Eighth note hi-hats (8/16) - steady rhythm
pub fn hihat_eighths() -> Vec<usize> {
    generate(8, 16)
}

/// Complex hi-hat pattern (11/16) - interesting groove
pub fn hihat_complex() -> Vec<usize> {
    generate(11, 16)
}

/// Syncopated hi-hat pattern (7/16)
pub fn hihat_syncopated() -> Vec<usize> {
    generate(7, 16)
}

/// Sparse hi-hat pattern (5/16)
pub fn hihat_sparse() -> Vec<usize> {
    generate(5, 16)
}

// ========== TRADITIONAL PATTERNS ==========

/// Cuban tresillo (3/8) - classic Latin rhythm
pub fn tresillo() -> Vec<usize> {
    generate(3, 8)
}

/// Cuban cinquillo (5/8) - complex Latin pattern
pub fn cinquillo() -> Vec<usize> {
    generate(5, 8)
}

/// York-Samai pattern (5/12) - Middle Eastern rhythm
pub fn york_samai() -> Vec<usize> {
    generate(5, 12)
}

/// Bossa nova variation (7/16)
pub fn bossa_nova() -> Vec<usize> {
    generate(7, 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_basic() {
        // Test basic Euclidean rhythm generation
        let e38 = generate(3, 8);
        assert_eq!(e38.len(), 3); // Should have 3 pulses
        assert_eq!(e38[0], 0); // Should start at position 0

        let e58 = generate(5, 8);
        assert_eq!(e58.len(), 5); // Should have 5 pulses
        assert_eq!(e58[0], 0); // Should start at position 0

        let e416 = generate(4, 16);
        assert_eq!(e416, vec![0, 4, 8, 12]); // Four-on-floor - perfectly even
    }

    #[test]
    fn test_euclidean_edge_cases() {
        assert_eq!(generate(0, 8), vec![]); // No pulses
        assert_eq!(generate(8, 0), vec![]); // No steps
        assert_eq!(generate(10, 8), vec![]); // More pulses than steps
        assert_eq!(generate(8, 8), vec![0, 1, 2, 3, 4, 5, 6, 7]); // All steps
    }

    #[test]
    fn test_euclidean_pattern() {
        let pattern_result = pattern(5, 8);
        assert_eq!(pattern_result.len(), 8); // Should have 8 steps
        assert_eq!(pattern_result.iter().filter(|&&x| x == 1).count(), 5); // Should have 5 pulses
        assert_eq!(pattern_result[0], 1); // Should start with a pulse

        let pattern_result = pattern(3, 8);
        assert_eq!(pattern_result.len(), 8); // Should have 8 steps
        assert_eq!(pattern_result.iter().filter(|&&x| x == 1).count(), 3); // Should have 3 pulses
        assert_eq!(pattern_result[0], 1); // Should start with a pulse
    }
}
