#![allow(dead_code)]

//! Mathematical sequence generators for algorithmic music

/// Generate Fibonacci sequence up to n terms
pub fn fibonacci(n: usize) -> Vec<u32> {
    let mut fib = vec![1, 1];
    for i in 2..n {
        let next = fib[i - 1] + fib[i - 2];
        fib.push(next);
    }
    fib.truncate(n);
    fib
}

/// Normalize a sequence to a range (e.g., 0.0 to 1.0)
///
/// Returns an empty vector if the input sequence is empty.
/// If all values are identical, returns a vector of the min value.
pub fn normalize(sequence: &[u32], min: f32, max: f32) -> Vec<f32> {
    if sequence.is_empty() {
        return vec![];
    }

    let seq_min = *sequence.iter().min().unwrap() as f32; // Safe: already checked non-empty
    let seq_max = *sequence.iter().max().unwrap() as f32; // Safe: already checked non-empty

    // Handle case where all values are the same (seq_max == seq_min)
    if (seq_max - seq_min).abs() < f32::EPSILON {
        return vec![min; sequence.len()];
    }

    sequence
        .iter()
        .map(|&x| {
            let normalized = (x as f32 - seq_min) / (seq_max - seq_min);
            min + normalized * (max - min)
        })
        .collect()
}

/// Generate powers of 2 sequence: 1, 2, 4, 8, 16...
pub fn powers_of_two(n: usize) -> Vec<u32> {
    (0..n).map(|i| 2u32.pow(i as u32)).collect()
}

/// Generate prime numbers up to n terms
pub fn primes(n: usize) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut candidate = 2u32;

    while primes.len() < n {
        if is_prime(candidate) {
            primes.push(candidate);
        }
        candidate += 1;
    }
    primes
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f32).sqrt() as u32 {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}

/// Generate arithmetic sequence: start, start+step, start+2*step, ...
pub fn arithmetic(start: u32, step: u32, n: usize) -> Vec<u32> {
    (0..n).map(|i| start + step * i as u32).collect()
}

/// Generate geometric sequence: start, start*ratio, start*ratio^2, ...
pub fn geometric(start: u32, ratio: u32, n: usize) -> Vec<u32> {
    (0..n).map(|i| start * ratio.pow(i as u32)).collect()
}

/// Collatz sequence (3n+1 problem)
pub fn collatz(start: u32, max_terms: usize) -> Vec<u32> {
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
/// # Examples
/// ```
/// # use tunes::composition::Composition;
/// # use tunes::rhythm::Tempo;
/// use tunes::sequences;
///
/// // Classic Cuban tresillo pattern
/// let pattern = sequences::euclidean(3, 8);
/// assert_eq!(pattern, vec![0, 2, 5]);
///
/// // Cuban cinquillo
/// let pattern = sequences::euclidean(5, 8);
/// assert_eq!(pattern, vec![0, 2, 3, 5, 7]);
///
/// // Perfect for drum patterns:
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// comp.track("drums").drum_grid(16, 0.125)
///     .kick(&sequences::euclidean(4, 16))    // Four-on-floor
///     .snare(&sequences::euclidean(3, 16))   // Syncopated snare
///     .hihat(&sequences::euclidean(7, 16));  // Complex hi-hat
/// ```
pub fn euclidean(pulses: usize, steps: usize) -> Vec<usize> {
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

/// Generate Euclidean rhythm as a full binary pattern
///
/// Returns a vector where 1 represents a hit and 0 represents a rest.
/// Useful for visualization or when you need the complete pattern.
///
/// # Examples
/// ```
/// use tunes::sequences;
/// let pattern = sequences::euclidean_pattern(5, 8);
/// // Returns: [1, 0, 1, 1, 0, 1, 1, 0]
/// ```
pub fn euclidean_pattern(pulses: usize, steps: usize) -> Vec<u32> {
    bjorklund(pulses, steps)
}

/// Generate triangular numbers: 1, 3, 6, 10, 15, 21...
///
/// The nth triangular number is the sum of the first n positive integers: T(n) = n*(n+1)/2
/// Creates natural ascending melodic contours.
///
/// # Examples
/// ```
/// use tunes::sequences;
/// let tri = sequences::triangular(6);
/// assert_eq!(tri, vec![1, 3, 6, 10, 15, 21]);
/// ```
pub fn triangular(n: usize) -> Vec<u32> {
    (1..=n)
        .map(|i| {
            let i = i as u32;
            (i * (i + 1)) / 2
        })
        .collect()
}

/// Generate harmonic (overtone) series from a fundamental frequency
///
/// The harmonic series is the foundation of musical timbre and consists of integer multiples
/// of a fundamental frequency. This is what makes instruments sound different from each other -
/// they emphasize different harmonics in their overtone spectrum.
///
/// The series follows: f, 2f, 3f, 4f, 5f, 6f, 7f, 8f...
///
/// Musically, these correspond to:
/// - 1st harmonic: fundamental (unison)
/// - 2nd harmonic: octave above
/// - 3rd harmonic: perfect fifth above octave
/// - 4th harmonic: second octave
/// - 5th harmonic: major third (slightly sharp)
/// - 6th harmonic: perfect fifth above second octave
/// - 7th harmonic: minor seventh (very flat - "blue note")
/// - 8th harmonic: third octave
///
/// # Arguments
/// * `fundamental` - The base frequency in Hz (e.g., 110.0 for A2)
/// * `n` - Number of harmonics to generate (including the fundamental)
///
/// # Returns
/// Vector of frequencies representing the first n harmonics
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Generate 8 harmonics of A2 (110 Hz)
/// let harmonics = sequences::harmonic_series(110.0, 8);
/// assert_eq!(harmonics, vec![110.0, 220.0, 330.0, 440.0, 550.0, 660.0, 770.0, 880.0]);
///
/// // Use for additive synthesis - build a sawtooth from harmonics:
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let harmonics = sequences::harmonic_series(220.0, 16);
/// comp.track("sawtooth").note(&harmonics, 1.0);
///
/// // Spectral music - play harmonics as a chord cluster:
/// let partials = sequences::harmonic_series(55.0, 12);  // A1 and its first 12 partials
/// comp.track("spectral").note(&partials, 4.0);
/// ```
///
/// # Musical Applications
/// - **Additive synthesis**: Build complex timbres from sine waves at harmonic frequencies
/// - **Spectral music**: Compose using the natural harmonic spectrum as source material
/// - **Chord voicings**: Harmonics 4-5-6 create major triad, 4-5-6-7 creates dom7 chord
/// - **Just intonation**: Natural harmonics represent pure integer ratios
/// - **Timbre analysis**: Understanding instrument tone color through harmonic content
pub fn harmonic_series(fundamental: f32, n: usize) -> Vec<f32> {
    (1..=n).map(|i| fundamental * i as f32).collect()
}

/// Generate golden ratio (phi) sequence: 1, φ, φ², φ³, φ⁴...
///
/// The golden ratio (φ ≈ 1.618033988749) is found throughout nature and has been used
/// in music composition for centuries. This sequence generates successive powers of phi,
/// creating naturally pleasing proportional relationships.
///
/// The golden ratio appears in:
/// - Nautilus shells, flower petals, pine cones
/// - Classical architecture (Parthenon, pyramids)
/// - Musical form (sonata proportions, phrase lengths)
/// - The ratio that Fibonacci numbers converge to
///
/// # Arguments
/// * `n` - Number of terms to generate
///
/// # Returns
/// Vector of successive powers of phi: [φ⁰, φ¹, φ², φ³, ...]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let phi_seq = sequences::golden_ratio(6);
/// // Returns approximately: [1.0, 1.618, 2.618, 4.236, 6.854, 11.090]
///
/// // Use with normalize() to map to frequencies:
/// let values = sequences::golden_ratio(8);
/// let freqs = sequences::normalize(&values.iter().map(|&x| x as u32).collect::<Vec<_>>(), 200.0, 800.0);
/// ```
///
/// # Musical Applications
/// - **Form and structure**: Divide piece duration by phi for natural section lengths
/// - **Melodic intervals**: Map phi powers to pitch space for organic contours
/// - **Rhythm**: Use phi ratios for timing relationships (not strictly metric)
/// - **Texture density**: Scale number of voices/layers by phi
pub fn golden_ratio(n: usize) -> Vec<f32> {
    const PHI: f32 = 1.618033988749;
    (0..n).map(|i| PHI.powi(i as i32)).collect()
}

/// Generate golden ratio rhythm pattern (Wythoff's sequence / Beatty sequence)
///
/// Uses the golden ratio to distribute beats evenly but non-periodically across steps.
/// This creates a rhythm that is neither regular nor random - it has structure but
/// never exactly repeats (until the full cycle).
///
/// The pattern is generated using the Beatty sequence: floor((n+1) * φ) for each step,
/// producing a binary sequence of 0s and 1s (rests and hits) with golden ratio spacing.
///
/// This is related to the Sturmian sequence and appears in:
/// - Phyllotaxis (leaf arrangement on stems)
/// - Musical canons and rounds
/// - Minimalist composition (Steve Reich, Philip Glass)
///
/// # Arguments
/// * `steps` - Number of steps in the rhythm pattern
///
/// # Returns
/// Vector of step indices where hits occur (like Euclidean rhythms)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let pattern = sequences::golden_ratio_rhythm(16);
/// // Creates a non-repeating, naturally flowing rhythm over 16 steps
///
/// // Use with drum_grid:
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let phi_rhythm = sequences::golden_ratio_rhythm(32);
/// comp.track("phi_drums")
///     .drum_grid(32, 0.125)
///     .kick(&phi_rhythm);
/// ```
///
/// # Properties
/// - **Non-periodic**: Pattern doesn't repeat (maximally even distribution)
/// - **Self-similar**: Zooming in/out reveals similar structure
/// - **Balanced**: Neither too sparse nor too dense
/// - **Organic**: Sounds natural, not mechanical
pub fn golden_ratio_rhythm(steps: usize) -> Vec<usize> {
    const PHI: f32 = 1.618033988749;

    (0..steps)
        .filter(|&i| {
            // Check if this position gets a beat using the lower Wythoff sequence
            let floor_current = ((i + 1) as f32 / PHI).floor() as usize;
            let floor_previous = (i as f32 / PHI).floor() as usize;
            floor_current != floor_previous
        })
        .collect()
}

/// Generate golden section divisions of a value
///
/// Divides a number into two parts according to the golden ratio (a/b = φ),
/// then recursively subdivides to create multiple golden sections.
///
/// This is useful for:
/// - Musical form (dividing piece into sections)
/// - Time-based structures (section durations)
/// - Amplitude scaling (dynamic levels)
///
/// # Arguments
/// * `value` - The value to divide
/// * `divisions` - Number of golden section points to generate
///
/// # Returns
/// Vector of values representing golden section divisions
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Divide 60 seconds into golden sections
/// let sections = sequences::golden_sections(60.0, 5);
/// // Use these as time markers for form: [60.0, 37.08, 22.92, ...]
///
/// // Use for dynamics (0.0 to 1.0)
/// let dynamics = sequences::golden_sections(1.0, 8);
/// // Creates naturally decreasing dynamic levels
/// ```
///
/// # Musical Applications
/// - **Sonata form**: Place development/recapitulation at golden ratio point
/// - **Climax placement**: Put emotional peak at φ proportion (≈61.8% through)
/// - **Phrase lengths**: Natural-feeling asymmetric phrase structures
/// - **Tempo changes**: Scale tempo by golden ratio for smooth transitions
pub fn golden_sections(value: f32, divisions: usize) -> Vec<f32> {
    const PHI: f32 = 1.618033988749;
    let mut sections = vec![value];

    for _ in 0..divisions {
        let last = *sections.last().unwrap();
        sections.push(last / PHI);
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let fib = fibonacci(8);
        assert_eq!(fib, vec![1, 1, 2, 3, 5, 8, 13, 21]);
    }

    #[test]
    fn test_primes() {
        let p = primes(5);
        assert_eq!(p, vec![2, 3, 5, 7, 11]);
    }

    #[test]
    fn test_collatz() {
        let seq = collatz(10, 100);
        assert_eq!(seq[0], 10);
        assert_eq!(*seq.last().unwrap(), 1);
    }

    #[test]
    fn test_euclidean_basic() {
        // Test basic Euclidean rhythm generation
        let e38 = euclidean(3, 8);
        assert_eq!(e38.len(), 3); // Should have 3 pulses
        assert_eq!(e38[0], 0); // Should start at position 0

        let e58 = euclidean(5, 8);
        assert_eq!(e58.len(), 5); // Should have 5 pulses
        assert_eq!(e58[0], 0); // Should start at position 0

        let e416 = euclidean(4, 16);
        assert_eq!(e416, vec![0, 4, 8, 12]); // Four-on-floor - perfectly even
    }

    #[test]
    fn test_euclidean_edge_cases() {
        assert_eq!(euclidean(0, 8), vec![]); // No pulses
        assert_eq!(euclidean(8, 0), vec![]); // No steps
        assert_eq!(euclidean(10, 8), vec![]); // More pulses than steps
        assert_eq!(euclidean(8, 8), vec![0, 1, 2, 3, 4, 5, 6, 7]); // All steps
    }

    #[test]
    fn test_euclidean_pattern() {
        let pattern = euclidean_pattern(5, 8);
        assert_eq!(pattern.len(), 8); // Should have 8 steps
        assert_eq!(pattern.iter().filter(|&&x| x == 1).count(), 5); // Should have 5 pulses
        assert_eq!(pattern[0], 1); // Should start with a pulse

        let pattern = euclidean_pattern(3, 8);
        assert_eq!(pattern.len(), 8); // Should have 8 steps
        assert_eq!(pattern.iter().filter(|&&x| x == 1).count(), 3); // Should have 3 pulses
        assert_eq!(pattern[0], 1); // Should start with a pulse
    }

    #[test]
    fn test_triangular() {
        let tri = triangular(6);
        assert_eq!(tri, vec![1, 3, 6, 10, 15, 21]);
    }

    #[test]
    fn test_harmonic_series() {
        // Test basic harmonic series of A2 (110 Hz)
        let harmonics = harmonic_series(110.0, 8);
        assert_eq!(harmonics.len(), 8);
        assert_eq!(harmonics, vec![110.0, 220.0, 330.0, 440.0, 550.0, 660.0, 770.0, 880.0]);

        // Test that harmonics are integer multiples
        let h = harmonic_series(100.0, 5);
        assert_eq!(h[0], 100.0);   // 1st harmonic (fundamental)
        assert_eq!(h[1], 200.0);   // 2nd harmonic (octave)
        assert_eq!(h[2], 300.0);   // 3rd harmonic (fifth above octave)
        assert_eq!(h[3], 400.0);   // 4th harmonic (two octaves)
        assert_eq!(h[4], 500.0);   // 5th harmonic (major third)
    }

    #[test]
    fn test_harmonic_series_single() {
        // Single harmonic should just return the fundamental
        let h = harmonic_series(440.0, 1);
        assert_eq!(h, vec![440.0]);
    }

    #[test]
    fn test_harmonic_series_ratios() {
        // Verify octave relationships
        let h = harmonic_series(55.0, 16);

        // Harmonics 1, 2, 4, 8, 16 are octaves
        assert_eq!(h[1], h[0] * 2.0);   // Octave relationship
        assert_eq!(h[3], h[0] * 4.0);   // Two octaves
        assert_eq!(h[7], h[0] * 8.0);   // Three octaves
        assert_eq!(h[15], h[0] * 16.0); // Four octaves

        // Harmonics 4-5-6 form a major triad
        // h[3] (4th harmonic) = root
        // h[4] (5th harmonic) = major third above
        // h[5] (6th harmonic) = perfect fifth above
        let ratio_maj3 = h[4] / h[3];  // 5/4 = 1.25 (major third)
        let ratio_p5 = h[5] / h[3];    // 6/4 = 1.5 (perfect fifth)
        assert!((ratio_maj3 - 1.25).abs() < 0.001);
        assert!((ratio_p5 - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_golden_ratio() {
        const PHI: f32 = 1.618033988749;
        let seq = golden_ratio(5);

        assert_eq!(seq.len(), 5);
        assert!((seq[0] - 1.0).abs() < 0.001);           // φ^0 = 1
        assert!((seq[1] - PHI).abs() < 0.001);           // φ^1 = φ
        assert!((seq[2] - PHI * PHI).abs() < 0.001);     // φ^2
        assert!((seq[3] - PHI.powi(3)).abs() < 0.001);   // φ^3
        assert!((seq[4] - PHI.powi(4)).abs() < 0.001);   // φ^4

        // Verify golden ratio property: φ^2 = φ + 1
        assert!((seq[2] - (seq[1] + 1.0)).abs() < 0.001);
    }

    #[test]
    fn test_golden_ratio_rhythm() {
        let pattern = golden_ratio_rhythm(16);

        // Should have hits but not all steps
        assert!(!pattern.is_empty());
        assert!(pattern.len() < 16);

        // Hits should be in ascending order
        for i in 1..pattern.len() {
            assert!(pattern[i] > pattern[i - 1]);
        }

        // All indices should be valid (< 16)
        for &idx in &pattern {
            assert!(idx < 16);
        }

        // Verify some expected properties
        // Golden ratio rhythm should have around 10 hits in 16 steps (16/φ ≈ 9.9)
        assert!(pattern.len() >= 8 && pattern.len() <= 11);
    }

    #[test]
    fn test_golden_ratio_rhythm_properties() {
        // Golden ratio rhythm has interesting mathematical properties
        let pattern = golden_ratio_rhythm(100);

        // The ratio of hits to total steps should approach 1/φ ≈ 0.618
        let ratio = pattern.len() as f32 / 100.0;
        let expected = 1.0 / 1.618033988749;
        assert!((ratio - expected).abs() < 0.05); // Within 5%
    }

    #[test]
    fn test_golden_sections() {
        const PHI: f32 = 1.618033988749;
        let sections = golden_sections(100.0, 4);

        assert_eq!(sections.len(), 5); // Original + 4 divisions
        assert_eq!(sections[0], 100.0);

        // Each section should be previous / φ
        for i in 1..sections.len() {
            let expected = sections[i - 1] / PHI;
            assert!((sections[i] - expected).abs() < 0.01);
        }

        // Verify decreasing sequence
        for i in 1..sections.len() {
            assert!(sections[i] < sections[i - 1]);
        }
    }

    #[test]
    fn test_golden_sections_single() {
        let sections = golden_sections(60.0, 1);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0], 60.0);

        // Second value should be 60 / φ ≈ 37.08
        let expected = 60.0 / 1.618033988749;
        assert!((sections[1] - expected).abs() < 0.01);
    }

    #[test]
    fn test_golden_sections_zero_divisions() {
        let sections = golden_sections(42.0, 0);
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0], 42.0);
    }
}
