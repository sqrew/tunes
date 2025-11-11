/// L-System (Lindenmayer System) - Fractal growth patterns
///
/// L-Systems are parallel rewriting systems that produce complex patterns from simple rules.
/// Originally developed by biologist Aristid Lindenmayer to model plant growth, they're now
/// used extensively in computer graphics, music, and generative art.
///
/// An L-System consists of:
/// - **Axiom**: Starting string/pattern
/// - **Rules**: How each symbol transforms in parallel
/// - **Iterations**: How many times to apply the rules
///
/// Example: Algae growth
/// - Axiom: "A"
/// - Rules: A → AB, B → A
/// - Evolution: A → AB → ABA → ABAAB → ABAABABA...
///
/// This creates the Fibonacci sequence in string length!
///
/// # Arguments
/// * `axiom` - Starting pattern (string of characters)
/// * `rules` - HashMap of transformation rules (char → String)
/// * `iterations` - Number of generations to evolve
///
/// # Returns
/// String representing the evolved pattern after n iterations
///
/// # Examples
/// ```
/// use tunes::sequences;
/// use std::collections::HashMap;
///
/// // Fibonacci pattern (algae growth)
/// let mut rules = HashMap::new();
/// rules.insert('A', "AB".to_string());
/// rules.insert('B', "A".to_string());
/// let pattern = sequences::generate("A", &rules, 4);
/// // "A" → "AB" → "ABA" → "ABAAB" → "ABAABABA"
/// assert_eq!(pattern, "ABAABABA");
///
/// // Convert to numeric sequence for music
/// let values: Vec<u32> = pattern.chars()
///     .map(|c| if c == 'A' { 1 } else { 2 })
///     .collect();
/// // Use for melody, rhythm, or structure!
/// ```
///
/// # Musical Applications
/// - **Melodic contours**: Map symbols to pitches (A=C, B=D, C=E, etc.)
/// - **Rhythmic patterns**: Map symbols to note durations
/// - **Formal structure**: Use pattern length to determine section lengths
/// - **Fractal melodies**: Self-similar patterns at different scales
/// - **Branching harmonies**: Create chord progressions that branch and grow
/// - **Texture evolution**: Map symbols to instrument layers appearing/disappearing
///
/// # Famous L-Systems
///
/// **Fibonacci (Algae):**
/// - Rules: A→AB, B→A
/// - Creates Fibonacci sequence lengths: 1,2,3,5,8,13,21...
///
/// **Cantor Set (Fractal):**
/// - Rules: A→ABA, B→BBB
/// - Creates Cantor set (removing middle thirds)
///
/// **Dragon Curve:**
/// - Rules: X→X+YF+, Y→-FX-Y
/// - Creates famous dragon fractal
///
/// **Thue-Morse:**
/// - Rules: A→AB, B→BA
/// - Same as Thue-Morse sequence!
///
/// **Binary Tree:**
/// - Rules: 0→1[0]0, 1→11
/// - Creates branching tree structure
///
/// # Example: Musical Phrase Generator
/// ```
/// # use tunes::sequences;
/// # use std::collections::HashMap;
/// // Create a melodic pattern that grows organically
/// let mut rules = HashMap::new();
/// rules.insert('C', "CD".to_string());   // Root expands up
/// rules.insert('D', "CE".to_string());   // Second up to third
/// rules.insert('E', "CG".to_string());   // Third jumps to fifth
/// rules.insert('G', "C".to_string());    // Fifth returns home
///
/// let melody = sequences::generate("C", &rules, 4);
/// // Evolution: C → CD → CDCE → CDCECG → CDCECGCE...
///
/// // Map to frequencies
/// let pitch_map: HashMap<char, f32> = [
///     ('C', 261.63),
///     ('D', 293.66),
///     ('E', 329.63),
///     ('G', 392.00),
/// ].iter().cloned().collect();
///
/// let frequencies: Vec<f32> = melody.chars()
///     .filter_map(|c| pitch_map.get(&c))
///     .copied()
///     .collect();
/// ```
pub fn generate(axiom: &str, rules: &std::collections::HashMap<char, String>, iterations: usize) -> String {
    let mut current = axiom.to_string();

    for _ in 0..iterations {
        let mut next = String::new();

        for ch in current.chars() {
            if let Some(replacement) = rules.get(&ch) {
                next.push_str(replacement);
            } else {
                // If no rule exists, keep the character unchanged
                next.push(ch);
            }
        }

        current = next;
    }

    current
}

/// Convert L-System string to numeric sequence
///
/// Maps each unique character to a number (A=0, B=1, C=2, etc.)
/// Useful for converting L-System patterns into musical parameters.
///
/// # Arguments
/// * `pattern` - L-System generated string
///
/// # Returns
/// Vector of u32 values representing the pattern
///
/// # Examples
/// ```
/// use tunes::sequences;
/// use std::collections::HashMap;
///
/// let mut rules = HashMap::new();
/// rules.insert('A', "AB".to_string());
/// rules.insert('B', "A".to_string());
/// let pattern = sequences::generate("A", &rules, 4);
/// let values = sequences::lsystem_to_sequence(&pattern);
/// // Maps: A=0, B=1 → [0,1,0,0,1]
/// ```
pub fn lsystem_to_sequence(pattern: &str) -> Vec<u32> {
    use std::collections::HashMap;

    let mut char_map: HashMap<char, u32> = HashMap::new();
    let mut next_id = 0u32;

    pattern
        .chars()
        .map(|ch| {
            *char_map.entry(ch).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_lsystem_fibonacci() {
        let mut rules = HashMap::new();
        rules.insert('A', "AB".to_string());
        rules.insert('B', "A".to_string());

        let gen0 = generate("A", &rules, 0);
        assert_eq!(gen0, "A");
        
        let gen1 = generate("A", &rules, 1);
        assert_eq!(gen1, "AB");
        
        let gen2 = generate("A", &rules, 2);
        assert_eq!(gen2, "ABA");
        
        let gen3 = generate("A", &rules, 3);
        assert_eq!(gen3, "ABAAB");
        
        let gen4 = generate("A", &rules, 4);
        assert_eq!(gen4, "ABAABABA");
    }

    #[test]
    fn test_lsystem_to_sequence_basic() {
        let pattern = "ABAAB";
        let seq = lsystem_to_sequence(pattern);
        assert_eq!(seq, vec![0, 1, 0, 0, 1]);
    }
}
