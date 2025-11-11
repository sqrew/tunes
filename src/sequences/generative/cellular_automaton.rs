/// Generate cellular automaton pattern (1D)
///
/// Cellular automata are simple systems that produce complex, often chaotic patterns
/// from basic rules. Each cell looks at its neighbors and updates according to a rule.
///
/// Famous rules:
/// - **Rule 30**: Chaotic, used in Mathematica's random number generator
/// - **Rule 110**: Turing complete (can compute anything!)
/// - **Rule 90**: Sierpinski triangle pattern
/// - **Rule 184**: Traffic flow simulation
///
/// The rule number (0-255) encodes what happens for each neighborhood:
/// ```text
/// Neighborhood: 111 110 101 100 011 010 001 000
/// Rule 30:        0   0   0   1   1   1   1   0  = 30 in binary
/// ```
///
/// # Arguments
/// * `rule` - Rule number (0-255) defining the cellular automaton behavior
/// * `steps` - Number of generations to evolve
/// * `width` - Width of the cell array
/// * `initial_state` - Optional starting pattern (if None, starts with center cell on)
///
/// # Returns
/// 2D vector where each row is a generation, each value is 0 or 1
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Rule 30 - classic chaotic pattern
/// let rule30 = sequences::cellular_automaton::generate(30, 16, 32, None);
/// // Each row is a generation, creates chaotic rhythm patterns
///
/// // Use first row as rhythm
/// let rhythm: Vec<usize> = rule30[10].iter()
///     .enumerate()
///     .filter(|(_, &v)| v == 1)
///     .map(|(i, _)| i)
///     .collect();
///
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(140.0));
/// comp.track("ca_drums")
///     .drum_grid(32, 0.125)
///     .kick(&rhythm);
///
/// // Rule 90 - Sierpinski fractal
/// let rule90 = sequences::cellular_automaton::generate(90, 16, 32, None);
/// // Creates self-similar fractal patterns
/// ```
///
/// # Musical Applications
/// - **Rhythm generation**: Each generation = different rhythm pattern
/// - **Evolving textures**: Watch patterns evolve over time
/// - **Polyrhythms**: Multiple rows simultaneously
/// - **Structural organization**: Use as formal blueprint
/// - **Timbral evolution**: Map cells to overtone presence
/// - **Generative scores**: Visual representation becomes music
///
/// # Popular Rules
/// - **Rule 30**: Chaos, randomness, unpredictable evolution
/// - **Rule 110**: Complex but structured, Turing complete
/// - **Rule 90**: Sierpinski triangle, fractal self-similarity
/// - **Rule 184**: Traffic flow, creates wave patterns
///
/// # Why It Matters
/// Cellular automata are used by composers like Iannis Xenakis and in generative
/// art worldwide. They create patterns that are deterministic but unpredictable,
/// perfect for algorithmic composition that needs structure without repetition.
pub fn generate(
    rule: u8,
    steps: usize,
    width: usize,
    initial_state: Option<Vec<u32>>,
) -> Vec<Vec<u32>> {
    if width == 0 {
        return vec![];
    }

    // Initialize first generation
    let mut current = if let Some(state) = initial_state {
        state.iter().take(width).copied().collect()
    } else {
        let mut state = vec![0; width];
        state[width / 2] = 1; // Start with center cell on
        state
    };

    let mut history = vec![current.clone()];

    // Evolve for specified steps
    for _ in 0..steps.saturating_sub(1) {
        let mut next = vec![0; width];

        for i in 0..width {
            let left = if i > 0 { current[i - 1] } else { 0 };
            let center = current[i];
            let right = if i < width - 1 { current[i + 1] } else { 0 };

            // Convert neighborhood to rule index (0-7)
            let neighborhood = (left << 2) | (center << 1) | right;

            // Check if rule bit is set for this neighborhood
            next[i] = if (rule >> neighborhood) & 1 == 1 {
                1
            } else {
                0
            };
        }

        current = next;
        history.push(current.clone());
    }

    history
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cellular_automaton_rule30() {
        let ca = generate(30, 5, 7, None);
        assert_eq!(ca.len(), 5);
        assert_eq!(ca[0].len(), 7);
        assert_eq!(ca[0], vec![0, 0, 0, 1, 0, 0, 0]);
        
        for gen in &ca {
            for &cell in gen {
                assert!(cell == 0 || cell == 1);
            }
        }
    }

    #[test]
    fn test_cellular_automaton_edge_cases() {
        let empty = generate(30, 5, 0, None);
        assert_eq!(empty, Vec::<Vec<u32>>::new());

        let single = generate(30, 1, 7, None);
        assert_eq!(single.len(), 1);
        assert_eq!(single[0].len(), 7);
    }

    #[test]
    fn test_cellular_automaton_all_rules_binary() {
        // Test various rules all produce binary output
        for rule in [0, 30, 90, 110, 184, 255].iter() {
            let ca = generate(*rule, 5, 10, None);

            for gen in &ca {
                for &cell in gen {
                    assert!(
                        cell == 0 || cell == 1,
                        "Rule {} produced non-binary value: {}",
                        rule,
                        cell
                    );
                }
            }
        }
    }

    #[test]
    fn test_cellular_automaton_as_rhythm() {
        // Use CA generation as rhythm pattern
        let ca = generate(30, 8, 16, None);

        // Convert 5th generation to rhythm
        let rhythm: Vec<usize> = ca[4]
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| i)
            .collect();

        // Should have some hits but not all
        assert!(!rhythm.is_empty());
        assert!(rhythm.len() < 16);
    }

    #[test]
    fn test_cellular_automaton_custom_initial() {
        // Test with custom initial state
        let initial = vec![1, 0, 1, 0, 1];
        let ca = generate(30, 3, 5, Some(initial.clone()));

        assert_eq!(ca.len(), 3);
        assert_eq!(ca[0], initial); // First generation matches initial state
    }

    #[test]
    fn test_cellular_automaton_rule110() {
        // Rule 110 - Turing complete!
        let ca = generate(110, 10, 20, None);

        assert_eq!(ca.len(), 10);

        // Rule 110 should create complex but structured patterns
        // Not all zeros, not all ones
        let total_ones: usize = ca
            .iter()
            .map(|gen| gen.iter().filter(|&&x| x == 1).count())
            .sum();

        let total_cells = ca.len() * ca[0].len();
        assert!(
            total_ones > 0 && total_ones < total_cells,
            "Rule 110 should create mixed patterns"
        );
    }

    #[test]
    fn test_cellular_automaton_rule90() {
        // Rule 90 - Sierpinski triangle
        let ca = generate(90, 8, 15, None);

        assert_eq!(ca.len(), 8);
        assert_eq!(ca[0].len(), 15);

        // First generation: center cell on
        assert_eq!(ca[0][7], 1);

        // Rule 90 creates symmetric patterns
        for gen in &ca {
            let mid = gen.len() / 2;
            // Check some symmetry (not perfect at edges)
            for i in 1..mid {
                if i < gen.len() - i - 1 {
                    assert_eq!(gen[mid - i], gen[mid + i], "Rule 90 should be symmetric");
                }
            }
        }
    }
}

// ========== PRESETS ==========

/// Rule 30 - Chaotic, unpredictable evolution (16 steps, 32 width)
pub fn rule30() -> Vec<Vec<u32>> {
    generate(30, 16, 32, None)
}

/// Rule 110 - Turing complete, complex patterns (20 steps, 40 width)
pub fn rule110() -> Vec<Vec<u32>> {
    generate(110, 20, 40, None)
}

/// Rule 90 - Sierpinski triangle fractal (16 steps, 32 width)
pub fn rule90() -> Vec<Vec<u32>> {
    generate(90, 16, 32, None)
}

/// Rule 184 - Traffic flow patterns (16 steps, 32 width)
pub fn rule184() -> Vec<Vec<u32>> {
    generate(184, 16, 32, None)
}

/// Classic - Rule 30, medium size (24 steps, 48 width)
pub fn classic() -> Vec<Vec<u32>> {
    generate(30, 24, 48, None)
}
