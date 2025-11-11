/// Generate Collatz sequence (3n+1 problem)
///
/// The Collatz conjecture, also known as the 3n+1 problem, is one of mathematics' most famous
/// unsolved problems. Despite its simple rules, it produces complex, unpredictable behavior.
///
/// The rules are:
/// - If n is even: divide by 2
/// - If n is odd: multiply by 3 and add 1
/// - Repeat until reaching 1
///
/// The **Collatz conjecture** states that no matter what positive integer you start with,
/// you'll always eventually reach 1. This has been verified for enormous numbers but never proven!
///
/// Examples:
/// - Start with 10: 10 → 5 → 16 → 8 → 4 → 2 → 1
/// - Start with 27: 27 → 82 → 41 → 124 → 62 → 31 → 94 → 47 → 142... (111 steps to 1!)
/// - Start with 7: 7 → 22 → 11 → 34 → 17 → 52 → 26 → 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
///
/// # Arguments
/// * `start` - The starting positive integer
/// * `max_terms` - Maximum number of terms to generate (safety limit to prevent infinite loops)
///
/// # Returns
/// Vector containing the Collatz sequence from start until reaching 1 (or max_terms)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let seq = sequences::collatz::generate(10, 100);
/// assert_eq!(seq, vec![10, 5, 16, 8, 4, 2, 1]);
///
/// let seq27 = sequences::collatz::generate(27, 150);
/// // Takes 111 steps to reach 1, with dramatic ups and downs!
/// assert_eq!(seq27.len(), 112); // 111 steps + starting value
///
/// // Use for unpredictable melodic contours
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let melody_seq = sequences::collatz::generate(19, 50);
/// let melody = sequences::normalize(&melody_seq, 220.0, 880.0);
/// comp.track("collatz_melody").notes(&melody, 0.2);
///
/// // Use for rhythmic variation
/// let rhythm_seq = sequences::collatz::generate(15, 30);
/// let durations = sequences::normalize(&rhythm_seq, 0.1, 0.5);
/// ```
///
/// # Musical Applications
/// - **Unpredictable melodies**: Creates wandering pitch contours with dramatic jumps
/// - **Organic rhythms**: Maps to note durations for non-mechanical timing
/// - **Dynamic contrast**: Use for volume, filter, or pan automation
/// - **Structural form**: Different starting values create unique "narratives"
/// - **Polyrhythmic cycles**: Different start values create different length sequences
/// - **Generative variation**: Each start number produces a unique musical gesture
///
/// # Why Collatz for Music
/// The Collatz sequence has special properties that make it musically interesting:
/// - **Deterministic but unpredictable**: You can't predict the path without computing it
/// - **Mix of ascent and descent**: Creates natural rise and fall (unlike monotonic sequences)
/// - **Unique character**: Each starting number produces a distinct "personality"
/// - **Natural climax**: Often spikes high before descending to 1
/// - **Mathematical mystery**: Adds conceptual depth to algorithmic compositions
///
/// # Typical Parameters
///
/// **start (interesting starting values):**
/// - **27**: Dramatic (111 steps, reaches 9232!) - great for intense sections
/// - **19**: Medium (20 steps) - good all-purpose melodic contour
/// - **15**: Short (17 steps) - quick musical gestures
/// - **31**: Very dramatic (106 steps) - epic journeys
/// - **7**: Classic (16 steps) - nice ups and downs
///
/// **max_terms:**
/// - **30-50**: Short melodic phrases
/// - **50-100**: Complete melodies
/// - **150+**: For dramatic starts like 27 or 31
///
/// # Recipe: Dramatic Collatz Melody
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences;
///
/// let mut comp = Composition::new(Tempo::new(110.0));
///
/// // Use 27 for dramatic journey
/// let collatz = sequences::collatz::generate(27, 120);
///
/// // Map to A minor pentatonic
/// let melody = sequences::map_to_scale(
///     &collatz,
///     &sequences::Scale::minor_pentatonic(),
///     A4,
///     2
/// );
///
/// comp.instrument("collatz_lead", &Instrument::synth_lead())
///     .reverb(Reverb::new(0.5, 0.5, 0.4))
///     .notes(&melody, 0.2);
/// ```
///
/// # Famous Starting Values
/// - **27**: Reaches 9,232 before descending (takes 111 steps)
/// - **31**: Also goes very high (9,232) before reaching 1
/// - **97**: Short but spiky sequence
/// - **127**: Long journey with interesting patterns
///
/// Try different starting values to find interesting contours for your music!
///
/// # Usage
/// ```
/// use tunes::sequences::collatz;
///
/// // Custom parameters
/// let seq = collatz::generate(27, 32);
///
/// // Or use presets
/// let seq = collatz::dramatic();
/// ```

/// Generate Collatz sequence with custom parameters
///
/// See module-level documentation for details on the Collatz conjecture,
/// musical applications, and typical parameter values.
pub fn generate(start: u32, max_terms: usize) -> Vec<u32> {
    let mut seq = vec![start];
    let mut current = start;

    while current != 1 && seq.len() < max_terms {
        current = if current.is_multiple_of(2) {
            current / 2
        } else {
            3 * current + 1
        };
        seq.push(current);
    }
    seq
}

// ========== PRESETS ==========

/// Short Collatz sequence (15 start, 17 steps) - quick musical gesture
pub fn short() -> Vec<u32> {
    generate(15, 20)
}

/// Medium Collatz sequence (19 start, 20 steps) - balanced melodic contour
pub fn medium() -> Vec<u32> {
    generate(19, 25)
}

/// Classic Collatz sequence (7 start, 16 steps) - nice ups and downs
pub fn classic() -> Vec<u32> {
    generate(7, 20)
}

/// Dramatic Collatz sequence (27 start, 111 steps!) - reaches 9,232 before descending
pub fn dramatic() -> Vec<u32> {
    generate(27, 120)
}

/// Epic Collatz sequence (31 start, 106 steps) - long journey with high peaks
pub fn epic() -> Vec<u32> {
    generate(31, 115)
}

/// Sparse Collatz sequence (27 start, truncated) - dramatic but shorter
pub fn sparse() -> Vec<u32> {
    generate(27, 32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz() {
        let seq = generate(10, 100);
        assert_eq!(seq[0], 10);
        assert_eq!(*seq.last().unwrap(), 1);
    }
}
