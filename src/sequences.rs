#![allow(dead_code)]

//! Mathematical sequence generators for algorithmic music

/// Generate Fibonacci sequence up to n terms
///
/// The Fibonacci sequence is one of the most famous mathematical sequences, where each
/// number is the sum of the two preceding ones: F(n) = F(n-1) + F(n-2)
///
/// Starting with 1, 1, the sequence goes: 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144...
///
/// This sequence appears throughout nature and has been used in composition since the
/// Middle Ages. The ratio between consecutive Fibonacci numbers converges to the
/// golden ratio (φ ≈ 1.618), making it naturally pleasing to human perception.
///
/// Found in:
/// - Flower petal counts (3, 5, 8, 13, 21 petals are common)
/// - Nautilus shell spirals and pine cone patterns
/// - Musical phrase lengths in classical compositions (Bartók, Debussy)
/// - Polyrhythmic relationships
///
/// # Arguments
/// * `n` - Number of Fibonacci terms to generate
///
/// # Returns
/// Vector of the first n Fibonacci numbers: [1, 1, 2, 3, 5, 8, 13, ...]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let fib = sequences::fibonacci(8);
/// assert_eq!(fib, vec![1, 1, 2, 3, 5, 8, 13, 21]);
///
/// // Use for rhythm - note durations following Fibonacci
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let durations = sequences::normalize(&fib, 0.125, 1.0);
/// for (i, &duration) in durations.iter().enumerate() {
///     comp.track("fib_rhythm")
///         .at(i as f32)
///         .note(&[440.0], duration);
/// }
///
/// // Use for phrase lengths (in beats)
/// let phrase_lengths = sequences::fibonacci(5); // [1, 1, 2, 3, 5] beats
/// ```
///
/// # Musical Applications
/// - **Phrase structure**: Use Fibonacci numbers for phrase lengths (5-bar phrases, 8-bar sections)
/// - **Rhythmic patterns**: Note durations or rests following the sequence
/// - **Melodic intervals**: Map to semitone jumps for organic-sounding melodies
/// - **Formal structure**: Section lengths in larger compositions
/// - **Polyrhythms**: Layer rhythms based on different Fibonacci numbers (3 against 5, 5 against 8)
/// - **Dynamic curves**: Volume or filter changes following Fibonacci proportions
pub fn fibonacci(n: usize) -> Vec<u32> {
    let mut fib = vec![1, 1];
    for i in 2..n {
        let next = fib[i - 1] + fib[i - 2];
        fib.push(next);
    }
    fib.truncate(n);
    fib
}

/// Normalize a sequence to a range (map values to a new min/max)
///
/// Takes a sequence of integers and scales them proportionally to fit within a new range.
/// This is essential for converting abstract numeric sequences into musical parameters
/// like frequencies, durations, velocities, or filter cutoffs.
///
/// The normalization preserves the relative proportions of the original sequence:
/// - The smallest input value maps to `min`
/// - The largest input value maps to `max`
/// - All other values are linearly interpolated between them
///
/// # Arguments
/// * `sequence` - The sequence of values to normalize
/// * `min` - The minimum value in the output range
/// * `max` - The maximum value in the output range
///
/// # Returns
/// Vector of f32 values scaled to the range [min, max]
/// - Returns empty vector if input is empty
/// - Returns vector of `min` values if all input values are identical
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Map Fibonacci to frequency range (200-800 Hz)
/// let fib = sequences::fibonacci(8);
/// let freqs = sequences::normalize(&fib, 200.0, 800.0);
/// // Result: [200.0, 200.0, 230.0, 260.0, 320.0, 410.0, 530.0, 800.0]
///
/// // Map to MIDI velocity (0-127)
/// let primes = sequences::primes(10);
/// let velocities = sequences::normalize(&primes, 40.0, 127.0);
///
/// // Map to note durations (eighth to whole note)
/// let seq = sequences::arithmetic(1, 2, 8);
/// let durations = sequences::normalize(&seq, 0.125, 1.0);
///
/// // Use normalized sequence for melody
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let collatz = sequences::collatz(27, 20);
/// let melody = sequences::normalize(&collatz, 220.0, 880.0);
/// comp.track("collatz_melody").notes(&melody, 0.25);
/// ```
///
/// # Musical Applications
/// - **Pitch mapping**: Convert any sequence to frequency range (e.g., 200-800 Hz for melody)
/// - **Rhythm variation**: Map to note durations (0.125 = eighth note, 1.0 = whole note)
/// - **Dynamics**: Convert to velocity/volume levels (0.0-1.0 or MIDI 0-127)
/// - **Parameter automation**: Map to filter cutoff (0.0-1.0), pan (-1.0 to 1.0), etc.
/// - **Microtonal scales**: Distribute values across pitch space
/// - **Tempo changes**: Map to BPM range for tempo modulation
///
/// # Technical Notes
/// This is a linear normalization (min-max scaling). The formula is:
/// ```text
/// normalized = (value - old_min) / (old_max - old_min) * (new_max - new_min) + new_min
/// ```
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

/// Generate powers of 2 sequence: 1, 2, 4, 8, 16, 32, 64...
///
/// Powers of 2 are fundamental to digital audio, binary systems, and musical structure.
/// The sequence is: 2⁰=1, 2¹=2, 2²=4, 2³=8, 2⁴=16, 2⁵=32...
///
/// This sequence appears everywhere in music and digital systems:
/// - **Octaves**: Each octave is a frequency doubling (power of 2)
/// - **Time signatures**: Common meters use powers of 2 (2/4, 4/4, 8/8, 16/16)
/// - **Note subdivisions**: Whole → half → quarter → eighth → sixteenth
/// - **Digital audio**: Buffer sizes, FFT sizes (512, 1024, 2048, 4096)
/// - **MIDI**: Pitch bend range often uses powers of 2
///
/// # Arguments
/// * `n` - Number of powers of 2 to generate
///
/// # Returns
/// Vector containing [1, 2, 4, 8, 16, 32, ...] (2⁰, 2¹, 2², ...)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let powers = sequences::powers_of_two(8);
/// assert_eq!(powers, vec![1, 2, 4, 8, 16, 32, 64, 128]);
///
/// // Use for rhythmic subdivision (whole, half, quarter, eighth...)
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let subdivisions = sequences::powers_of_two(5); // [1, 2, 4, 8, 16]
/// let durations = sequences::normalize(&subdivisions, 0.0625, 1.0);
///
/// // Use for octave relationships (frequency doubling)
/// let octaves = sequences::powers_of_two(4);
/// let freqs: Vec<f32> = octaves.iter()
///     .map(|&mult| 110.0 * mult as f32)  // A2, A3, A4, A5
///     .collect();
/// comp.track("octaves").notes(&freqs, 0.5);
/// ```
///
/// # Musical Applications
/// - **Rhythmic subdivision**: Whole notes → half → quarter → eighth → sixteenth
/// - **Octave generation**: Multiply base frequency by 2ⁿ for octaves
/// - **Time signatures**: 2/4, 4/4, 8/8 patterns
/// - **Structural proportions**: Section lengths doubling (8 bars, 16 bars, 32 bars)
/// - **Polyrhythms**: Layer patterns with power-of-2 relationships (4 against 8, 8 against 16)
/// - **Digital timing**: Synchronize with digital audio buffer boundaries
pub fn powers_of_two(n: usize) -> Vec<u32> {
    (0..n).map(|i| 2u32.pow(i as u32)).collect()
}

/// Generate prime numbers sequence
///
/// Prime numbers are integers greater than 1 that have no divisors other than 1 and themselves.
/// The sequence starts: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47...
///
/// Primes have fascinated mathematicians for millennia and create interesting musical patterns
/// because they resist regular subdivision - they're inherently "unpredictable" while still
/// being deterministic. This makes them excellent for creating rhythms and melodies that
/// feel organic and non-mechanical.
///
/// Properties that make primes useful in music:
/// - **Irregular spacing**: No obvious pattern, but not random
/// - **Avoid common factors**: Create polyrhythms that rarely align
/// - **Mathematical beauty**: Deterministic but complex
/// - **Ancient mystery**: Used in composition since medieval times
///
/// # Arguments
/// * `n` - Number of prime numbers to generate
///
/// # Returns
/// Vector containing the first n prime numbers: [2, 3, 5, 7, 11, 13, 17, ...]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let primes = sequences::primes(10);
/// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
///
/// // Use for rhythmic patterns that avoid repetition
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(140.0));
/// let prime_rhythm = sequences::primes(8);
/// let hits = sequences::normalize(&prime_rhythm, 0.0, 16.0);
/// for &hit_time in &hits {
///     comp.track("prime_kicks")
///         .at(hit_time)
///         .note(&[110.0], 0.1);
/// }
///
/// // Use for melodic intervals (semitone jumps)
/// let intervals = sequences::primes(12);
/// let pitches = sequences::normalize(&intervals, 200.0, 800.0);
/// comp.track("prime_melody").notes(&pitches, 0.25);
/// ```
///
/// # Musical Applications
/// - **Polyrhythms**: Primes create patterns that rarely align (3 against 5, 7 against 11)
/// - **Non-repetitive rhythms**: Use primes for hit positions to avoid obvious patterns
/// - **Melodic contours**: Prime-based intervals create interesting, unpredictable melodies
/// - **Phrase lengths**: Prime-numbered bar counts (5-bar, 7-bar, 11-bar phrases)
/// - **Harmonic ratios**: Prime number ratios create inharmonic/dissonant timbres
/// - **Form and structure**: Section lengths using primes (13 bars, 17 bars, 23 bars)
///
/// # Why Primes Matter in Music
/// Composers like Olivier Messiaen used prime numbers extensively to create rhythms that
/// "never repeat" within practical performance time. The lack of common factors means
/// patterns layer in complex, non-obvious ways - perfect for generative music that needs
/// to sound structured but not mechanical.
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

/// Helper function to check if a number is prime
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

/// Generate arithmetic sequence (linear progression)
///
/// An arithmetic sequence is formed by adding a constant value (the "step" or "common difference")
/// to each term: a, a+d, a+2d, a+3d, a+4d, ...
///
/// This is the simplest type of progression - just counting up (or down) by a fixed amount.
/// Examples:
/// - 1, 2, 3, 4, 5, 6... (step = 1)
/// - 2, 4, 6, 8, 10... (step = 2, even numbers)
/// - 5, 10, 15, 20, 25... (step = 5, multiples of 5)
/// - 100, 90, 80, 70... (step = -10, counting down - use with caution for u32)
///
/// # Arguments
/// * `start` - The first value in the sequence
/// * `step` - Amount to add to each subsequent term (common difference)
/// * `n` - Number of terms to generate
///
/// # Returns
/// Vector containing [start, start+step, start+2*step, ...]
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Simple counting sequence
/// let count = sequences::arithmetic(1, 1, 10);
/// assert_eq!(count, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
///
/// // Even numbers
/// let evens = sequences::arithmetic(2, 2, 8);
/// assert_eq!(evens, vec![2, 4, 6, 8, 10, 12, 14, 16]);
///
/// // Use for ascending scale pattern
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// # use tunes::scales::C4_MAJOR_SCALE;
/// let scale_indices = sequences::arithmetic(0, 1, 8); // [0, 1, 2, 3, 4, 5, 6, 7]
/// comp.track("ascending")
///     .sequence_from(&scale_indices, &C4_MAJOR_SCALE, 0.25);
///
/// // Use for regular rhythm (every 4 beats)
/// let beat_positions = sequences::arithmetic(0, 4, 16);
/// let times = sequences::normalize(&beat_positions, 0.0, 16.0);
/// ```
///
/// # Musical Applications
/// - **Scales**: Ascending/descending through scale degrees (0, 1, 2, 3...)
/// - **Regular rhythms**: Evenly spaced beats (every 2, 3, or 4 steps)
/// - **Velocity ramps**: Linear increase/decrease in volume
/// - **Filter sweeps**: Linear cutoff frequency changes
/// - **Time markers**: Evenly spaced section boundaries
/// - **Melodic steps**: Stepwise motion up or down
///
/// # Why Use Arithmetic Sequences
/// They're predictable and regular - perfect for:
/// - Creating steady, mechanical patterns
/// - Building foundations for more complex variations
/// - Establishing a baseline before applying transformations
/// - Simulating linear motion or growth
pub fn arithmetic(start: u32, step: u32, n: usize) -> Vec<u32> {
    (0..n).map(|i| start + step * i as u32).collect()
}

/// Generate geometric sequence (exponential progression)
///
/// A geometric sequence is formed by multiplying each term by a constant value (the "ratio"
/// or "common ratio"): a, a×r, a×r², a×r³, a×r⁴, ...
///
/// Unlike arithmetic sequences (which grow linearly), geometric sequences grow exponentially.
/// This creates dramatic acceleration - values get much larger very quickly.
///
/// Examples:
/// - 1, 2, 4, 8, 16, 32... (ratio = 2, doubling sequence)
/// - 3, 9, 27, 81, 243... (ratio = 3, tripling sequence)
/// - 5, 25, 125, 625... (ratio = 5)
///
/// # Arguments
/// * `start` - The first value in the sequence
/// * `ratio` - The multiplier for each subsequent term (common ratio)
/// * `n` - Number of terms to generate
///
/// # Returns
/// Vector containing [start, start×ratio, start×ratio², ...]
///
/// # Warning
/// Geometric sequences with ratio > 2 grow VERY rapidly. For example, with start=2 and ratio=3:
/// - Term 5: 162
/// - Term 10: 39,366
/// - Term 15: 9,565,938
/// Use normalize() to map to usable ranges.
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Doubling sequence (same as powers of 2 but with custom start)
/// let doubling = sequences::geometric(1, 2, 8);
/// assert_eq!(doubling, vec![1, 2, 4, 8, 16, 32, 64, 128]);
///
/// // Tripling sequence
/// let tripling = sequences::geometric(1, 3, 5);
/// assert_eq!(tripling, vec![1, 3, 9, 27, 81]);
///
/// // Use for accelerating rhythms
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let accel = sequences::geometric(1, 2, 6);
/// let durations = sequences::normalize(&accel, 0.125, 1.0);
/// // Creates accelerating pattern: long → medium → short → very short
///
/// // Use for exponential volume increase (careful!)
/// let growth = sequences::geometric(1, 2, 8);
/// let volumes = sequences::normalize(&growth, 0.1, 1.0);
/// ```
///
/// # Musical Applications
/// - **Accelerando**: Exponentially decreasing note durations (tempo acceleration)
/// - **Crescendo curves**: Exponential volume increase (more dramatic than linear)
/// - **Octave stacking**: Multiply base frequency by 2ⁿ
/// - **Rhythmic density**: Exponentially increasing subdivisions
/// - **Filter sweeps**: Exponential cutoff changes (more natural than linear)
/// - **Spatial effects**: Exponential pan or reverb changes
///
/// # Musical Context
/// Geometric sequences feel more "natural" than arithmetic for many parameters because:
/// - Human hearing is logarithmic (each octave is a doubling)
/// - Perceived loudness scales logarithmically
/// - Musical intervals are multiplicative ratios, not additive
/// - Natural phenomena (sound decay, reverberation) are exponential
///
/// However, they grow very fast - almost always use normalize() to constrain the output
/// to musical ranges.
pub fn geometric(start: u32, ratio: u32, n: usize) -> Vec<u32> {
    (0..n).map(|i| start * ratio.pow(i as u32)).collect()
}

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
/// let seq = sequences::collatz(10, 100);
/// assert_eq!(seq, vec![10, 5, 16, 8, 4, 2, 1]);
///
/// let seq27 = sequences::collatz(27, 150);
/// // Takes 111 steps to reach 1, with dramatic ups and downs!
/// assert_eq!(seq27.len(), 112); // 111 steps + starting value
///
/// // Use for unpredictable melodic contours
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let melody_seq = sequences::collatz(19, 50);
/// let melody = sequences::normalize(&melody_seq, 220.0, 880.0);
/// comp.track("collatz_melody").notes(&melody, 0.2);
///
/// // Use for rhythmic variation
/// let rhythm_seq = sequences::collatz(15, 30);
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
/// # Famous Starting Values
/// - **27**: Reaches 9,232 before descending (takes 111 steps)
/// - **31**: Also goes very high (9,232) before reaching 1
/// - **97**: Short but spiky sequence
/// - **127**: Long journey with interesting patterns
///
/// Try different starting values to find interesting contours for your music!
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

/// Generate logistic map sequence (chaos theory)
///
/// The logistic map is a simple equation that exhibits complex chaotic behavior:
/// `x(n+1) = r * x(n) * (1 - x(n))`
///
/// Originally used to model population growth, it demonstrates how simple deterministic
/// systems can produce seemingly random, unpredictable behavior - the foundation of chaos theory.
///
/// The behavior dramatically changes based on the parameter `r`:
/// - **r < 1.0**: Population dies out (converges to 0)
/// - **r < 3.0**: Converges to a stable fixed point
/// - **3.0 < r < 3.57**: Oscillates between multiple values (period doubling)
/// - **r > 3.57**: Chaotic behavior (appears random but is deterministic)
/// - **r ≈ 3.828**: Extreme chaos
///
/// # Arguments
/// * `r` - Growth rate parameter (typically 0.0 to 4.0)
/// * `initial` - Starting value (typically 0.0 to 1.0, often 0.5)
/// * `n` - Number of iterations to generate
///
/// # Returns
/// Vector of values in range 0.0 to 1.0 (except edge cases where it may converge to 0)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Stable behavior (r=2.5)
/// let stable = sequences::logistic_map(2.5, 0.5, 20);
/// // Converges to a fixed point around 0.6
///
/// // Oscillating behavior (r=3.2)
/// let oscillating = sequences::logistic_map(3.2, 0.5, 50);
/// // Alternates between two values
///
/// // Chaotic behavior (r=3.9) - great for generative music!
/// let chaotic = sequences::logistic_map(3.9, 0.5, 100);
/// let freqs = sequences::normalize(
///     &chaotic.iter().map(|&x| (x * 100.0) as u32).collect::<Vec<_>>(),
///     200.0, 800.0
/// );
///
/// // Use for game intensity scaling
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(160.0));
/// let enemy_count = 50;
/// let chaos_param = 2.5 + (enemy_count as f32 / 100.0) * 1.5; // 2.5 to 4.0
/// let intensity = sequences::logistic_map(chaos_param, 0.5, 16);
/// ```
///
/// # Musical Applications
/// - **Dynamic complexity**: Map game state to r parameter for intensity control
/// - **Melodic variation**: Chaotic but deterministic pitch sequences
/// - **Rhythm patterns**: Use values to determine hit probabilities
/// - **Texture density**: More enemies → higher r → more chaotic/dense music
/// - **Filter sweeps**: Smooth but unpredictable parameter changes
///
/// # Chaos Control
/// The logistic map is perfect for game audio because you can smoothly transition
/// from stable (calm) to chaotic (intense) music by adjusting the `r` parameter
/// based on gameplay state (enemy count, health, proximity to danger, etc.).
pub fn logistic_map(r: f32, initial: f32, n: usize) -> Vec<f32> {
    let mut seq = vec![initial.clamp(0.0, 1.0)];
    let mut x = initial.clamp(0.0, 1.0);

    for _ in 1..n {
        x = (r * x * (1.0 - x)).clamp(0.0, 1.0);
        seq.push(x);
    }
    seq
}

/// Generate random walk sequence (Brownian motion)
///
/// A random walk is a path consisting of successive random steps. Each value is
/// the previous value plus a random delta. This creates smooth but unpredictable
/// variation - like a drunk person walking or a particle in fluid (Brownian motion).
///
/// Random walks appear throughout nature and music:
/// - Stock market prices
/// - Particle diffusion
/// - Melodic contours in jazz improvisation
/// - Bass line variation
///
/// # Arguments
/// * `start` - Initial value
/// * `step_size` - Maximum step size (positive or negative)
/// * `steps` - Number of steps to generate
///
/// # Returns
/// Vector of values forming a random walk (unbounded - can go anywhere)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Simple random walk
/// let walk = sequences::random_walk(440.0, 20.0, 20);
/// // Starts at 440, each step changes by up to ±20
///
/// // Use for bass line variation
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let bass_line = sequences::random_walk(110.0, 5.0, 16);
/// comp.instrument("bass", &Instrument::sub_bass())
///     .notes(&bass_line, 0.25);
/// ```
///
/// # Musical Applications
/// - **Melodic variation**: Organic-sounding pitch movement
/// - **Bass lines**: Smooth but unpredictable bass patterns
/// - **Parameter automation**: Filter cutoff, pan, volume variation
/// - **Generative composition**: Non-repetitive sequences
///
/// # Note
/// This is an unbounded walk - values can grow arbitrarily large or small.
/// Use `bounded_walk()` if you need to constrain the range.
pub fn random_walk(start: f32, step_size: f32, steps: usize) -> Vec<f32> {
    use rand::Rng;
    let mut rng = rand::rng();
    let mut seq = vec![start];

    for _ in 1..steps {
        let delta = rng.random_range(-step_size..=step_size);
        let next = seq.last().unwrap() + delta;
        seq.push(next);
    }
    seq
}

/// Generate bounded random walk sequence
///
/// Like `random_walk()`, but constrained to stay within a specified range.
/// When the walk would exceed the bounds, it's clamped to min/max.
///
/// This is useful when you want random variation but need to guarantee
/// the values stay within musical constraints (e.g., a specific frequency range,
/// or normalized 0.0-1.0 values for parameters).
///
/// # Arguments
/// * `start` - Initial value (should be within min..max)
/// * `step` - Maximum step size (positive or negative)
/// * `min` - Minimum allowed value
/// * `max` - Maximum allowed value
/// * `steps` - Number of steps to generate
///
/// # Returns
/// Vector of values forming a bounded random walk (all values in min..=max)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Random walk constrained to one octave
/// let melody = sequences::bounded_walk(440.0, 30.0, 220.0, 880.0, 32);
/// // Wanders around 440Hz but stays in 220-880 range
///
/// // Random filter cutoff (normalized 0-1)
/// let cutoff_walk = sequences::bounded_walk(0.5, 0.1, 0.0, 1.0, 64);
/// // Smooth filter movement staying in 0-1 range
///
/// // Use for bass line that stays in key
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(140.0));
/// let bass_notes = sequences::bounded_walk(110.0, 10.0, 80.0, 150.0, 16);
/// comp.instrument("bass", &Instrument::sub_bass())
///     .notes(&bass_notes, 0.25);
/// ```
///
/// # Musical Applications
/// - **Constrained melodies**: Wandering pitch that stays in range
/// - **Bass lines**: Random variation within an octave
/// - **Parameter automation**: Filter, pan, volume staying in valid ranges
/// - **Scale-based melodies**: Step between scale degrees randomly
/// - **Dynamic contrast**: Volume variation within acceptable range
pub fn bounded_walk(start: f32, step: f32, min: f32, max: f32, steps: usize) -> Vec<f32> {
    use rand::Rng;
    let mut rng = rand::rng();
    let start_clamped = start.clamp(min, max);
    let mut seq = vec![start_clamped];

    for _ in 1..steps {
        let delta = rng.random_range(-step..=step);
        let next = (seq.last().unwrap() + delta).clamp(min, max);
        seq.push(next);
    }
    seq
}

/// Generate Thue-Morse sequence (fair division sequence)
///
/// The Thue-Morse sequence is a binary sequence with remarkable fairness properties.
/// It's constructed by starting with 0, then repeatedly appending the bitwise complement:
/// - Start: `0`
/// - Append complement: `0 1`
/// - Append complement: `0 1 1 0`
/// - Append complement: `0 1 1 0 1 0 0 1`
/// - Continue...
///
/// This sequence has fascinating properties:
/// - **No three consecutive identical blocks** (avoids repetition)
/// - **Fairest possible coin flip sequence** (equal distribution)
/// - **Self-similar** (contains itself at different scales)
/// - **Appears in chemistry** (protein folding), music, and computer science
///
/// Named after mathematicians Axel Thue (1906) and Marston Morse (1921).
///
/// # Arguments
/// * `n` - Number of terms to generate
///
/// # Returns
/// Vector of 0s and 1s forming the Thue-Morse sequence
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let tm = sequences::thue_morse(16);
/// // [0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0]
///
/// // Use as rhythm pattern (0 = rest, 1 = hit)
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let pattern: Vec<usize> = sequences::thue_morse(32)
///     .iter()
///     .enumerate()
///     .filter(|(_, &v)| v == 1)
///     .map(|(i, _)| i)
///     .collect();
///
/// comp.track("thue_drums")
///     .drum_grid(32, 0.125)
///     .kick(&pattern);
///
/// // Use for parameter switching
/// let tm_seq = sequences::thue_morse(8);
/// for (i, &val) in tm_seq.iter().enumerate() {
///     let freq = if val == 0 { 440.0 } else { 554.37 };
///     comp.track("alternating").note(&[freq], 0.25);
/// }
/// ```
///
/// # Musical Applications
/// - **Non-repetitive rhythms**: Creates patterns that don't sound mechanical
/// - **Timbral alternation**: Switch between two instruments/sounds fairly
/// - **Accent patterns**: Alternate strong/weak beats without predictability
/// - **Chord voicings**: Alternate between two chord inversions
/// - **Stereo panning**: Fair left/right distribution
/// - **Minimalist composition**: Used by composers like Tom Johnson
///
/// # Why It Matters for Music
/// The Thue-Morse sequence avoids the monotony of simple alternation (0,1,0,1,...)
/// while maintaining perfect fairness. It sounds organic and interesting without
/// being truly random - ideal for generative music that needs structure but
/// wants to avoid repetitive patterns.
pub fn thue_morse(n: usize) -> Vec<u32> {
    let mut seq = vec![0];

    while seq.len() < n {
        let complement: Vec<u32> = seq.iter().map(|&x| 1 - x).collect();
        seq.extend(complement);
    }

    seq.truncate(n);
    seq
}

/// Generate Recamán's sequence
///
/// Recamán's sequence is a mathematical curiosity that creates beautiful spiraling patterns.
/// It's defined recursively with a simple rule that produces surprisingly complex behavior:
///
/// - Start with 0
/// - At step n: try to go backward by n (a(n) = a(n-1) - n)
/// - If that's negative or already visited, go forward instead (a(n) = a(n-1) + n)
///
/// The sequence: 0, 1, 3, 6, 2, 7, 13, 20, 12, 21, 11, 22, 10, 23, 9, 24, 8, 25, 43, 62...
///
/// Named after Colombian mathematician Bernardo Recamán Santos. When visualized,
/// it creates beautiful arcs that have been used in art installations and music.
///
/// # Arguments
/// * `n` - Number of terms to generate
///
/// # Returns
/// Vector of values forming Recamán's sequence
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let recaman = sequences::recaman(20);
/// // [0, 1, 3, 6, 2, 7, 13, 20, 12, 21, 11, 22, 10, 23, 9, 24, 8, 25, 43, 62]
///
/// // Use for melodic contours
/// let melody = sequences::normalize(&recaman, 220.0, 880.0);
///
/// // Use for interesting bass lines
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let bass_recaman = sequences::recaman(16);
/// let bass_freqs = sequences::normalize(&bass_recaman, 55.0, 110.0);
/// comp.instrument("bass", &Instrument::sub_bass())
///     .notes(&bass_freqs, 0.25);
/// ```
///
/// # Musical Applications
/// - **Melodic contours**: Creates interesting back-and-forth pitch movement
/// - **Bass lines**: Unpredictable but structured patterns
/// - **Phrase lengths**: Use values (mod some number) for varying phrase durations
/// - **Rhythmic displacement**: Map to beat positions for syncopation
/// - **Formal structure**: Large-scale sectional organization
/// - **Visual music**: Graph the sequence for performance visuals
///
/// # Why It's Special
/// Recamán's sequence has a unique "memory" - it remembers all previous values
/// and avoids revisiting them when possible. This creates patterns that wander
/// but never quite repeat, perfect for generative music that needs to feel
/// purposeful without being predictable.
pub fn recaman(n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    let mut seq = vec![0u32];
    let mut seen = std::collections::HashSet::new();
    seen.insert(0u32);

    for i in 1..n {
        let prev = seq[i - 1];
        let backward = prev.saturating_sub(i as u32);

        // Try backward first: must be > 0 and not previously seen
        if backward > 0 && !seen.contains(&backward) {
            seq.push(backward);
            seen.insert(backward);
        } else {
            // Go forward - standard Recamán just adds without checking duplicates
            let forward = prev + i as u32;
            seq.push(forward);
            seen.insert(forward);
        }
    }

    seq
}

/// Generate Van der Corput sequence (low-discrepancy/quasi-random sequence)
///
/// The Van der Corput sequence is a "quasi-random" sequence that fills space more
/// evenly than pure random numbers. It's used in ray tracing, Monte Carlo integration,
/// and anywhere you want random-looking but well-distributed values.
///
/// The sequence is generated by reversing the binary representation of integers:
/// - 1 (binary: 1) → 0.1 (binary) = 0.5
/// - 2 (binary: 10) → 0.01 (binary) = 0.25
/// - 3 (binary: 11) → 0.11 (binary) = 0.75
/// - 4 (binary: 100) → 0.001 (binary) = 0.125
///
/// This produces values in [0, 1) that are more evenly distributed than random.
///
/// # Arguments
/// * `n` - Number of terms to generate
/// * `base` - Base for the sequence (typically 2 for binary, but can use other bases)
///
/// # Returns
/// Vector of values in range [0.0, 1.0) with quasi-random distribution
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Generate quasi-random values
/// let quasi = sequences::van_der_corput(16, 2);
/// // More evenly distributed than random!
///
/// // Use for note placement that avoids clumping
/// let positions = sequences::van_der_corput(32, 2);
/// let note_times: Vec<f32> = positions.iter()
///     .map(|&x| x * 4.0)  // Spread over 4 seconds
///     .collect();
///
/// // Use for parameter sweeps
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// let cutoff_values = sequences::van_der_corput(64, 2);
/// for (i, &cutoff) in cutoff_values.iter().enumerate() {
///     let freq = 200.0 + cutoff * 600.0;  // 200-800 Hz range
///     comp.instrument("sweep", &Instrument::synth_lead())
///         .at(i as f32 * 0.125)
///         .note(&[freq], 0.1);
/// }
/// ```
///
/// # Musical Applications
/// - **Note distribution**: Place notes evenly without grid-like regularity
/// - **Rhythm generation**: Better than random for avoiding clumps
/// - **Parameter sampling**: Sweep through filter/pan/volume space efficiently
/// - **Chord voicings**: Distribute notes across register evenly
/// - **Polyrhythms**: Create non-periodic but well-distributed patterns
/// - **Microtonal scales**: Sample pitch space quasi-randomly
///
/// # Quasi-Random vs Random
/// Pure random can create clumps and gaps. Van der Corput fills space more evenly:
/// - **Random**: Unpredictable, can cluster
/// - **Quasi-random**: Looks random, mathematically even distribution
/// - **Grid**: Predictable, mechanical
///
/// Perfect middle ground for generative music that needs randomness without chaos.
pub fn van_der_corput(n: usize, base: u32) -> Vec<f32> {
    (0..n)
        .map(|i| {
            let mut result = 0.0;
            let mut f = 1.0 / base as f32;
            let mut num = (i + 1) as u32;

            while num > 0 {
                result += f * (num % base) as f32;
                num /= base;
                f /= base as f32;
            }

            result
        })
        .collect()
}

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
/// let rule30 = sequences::cellular_automaton(30, 16, 32, None);
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
/// let rule90 = sequences::cellular_automaton(90, 16, 32, None);
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
pub fn cellular_automaton(
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
        assert_eq!(
            harmonics,
            vec![110.0, 220.0, 330.0, 440.0, 550.0, 660.0, 770.0, 880.0]
        );

        // Test that harmonics are integer multiples
        let h = harmonic_series(100.0, 5);
        assert_eq!(h[0], 100.0); // 1st harmonic (fundamental)
        assert_eq!(h[1], 200.0); // 2nd harmonic (octave)
        assert_eq!(h[2], 300.0); // 3rd harmonic (fifth above octave)
        assert_eq!(h[3], 400.0); // 4th harmonic (two octaves)
        assert_eq!(h[4], 500.0); // 5th harmonic (major third)
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
        assert_eq!(h[1], h[0] * 2.0); // Octave relationship
        assert_eq!(h[3], h[0] * 4.0); // Two octaves
        assert_eq!(h[7], h[0] * 8.0); // Three octaves
        assert_eq!(h[15], h[0] * 16.0); // Four octaves

        // Harmonics 4-5-6 form a major triad
        // h[3] (4th harmonic) = root
        // h[4] (5th harmonic) = major third above
        // h[5] (6th harmonic) = perfect fifth above
        let ratio_maj3 = h[4] / h[3]; // 5/4 = 1.25 (major third)
        let ratio_p5 = h[5] / h[3]; // 6/4 = 1.5 (perfect fifth)
        assert!((ratio_maj3 - 1.25).abs() < 0.001);
        assert!((ratio_p5 - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_golden_ratio() {
        const PHI: f32 = 1.618033988749;
        let seq = golden_ratio(5);

        assert_eq!(seq.len(), 5);
        assert!((seq[0] - 1.0).abs() < 0.001); // φ^0 = 1
        assert!((seq[1] - PHI).abs() < 0.001); // φ^1 = φ
        assert!((seq[2] - PHI * PHI).abs() < 0.001); // φ^2
        assert!((seq[3] - PHI.powi(3)).abs() < 0.001); // φ^3
        assert!((seq[4] - PHI.powi(4)).abs() < 0.001); // φ^4

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

    #[test]
    fn test_logistic_map_stable() {
        // r=2.5 should converge to a stable fixed point
        let seq = logistic_map(2.5, 0.5, 50);

        assert_eq!(seq.len(), 50);
        assert_eq!(seq[0], 0.5);

        // After many iterations, should converge (last few values nearly equal)
        let last_five: Vec<f32> = seq.iter().rev().take(5).copied().collect();
        for i in 1..last_five.len() {
            assert!(
                (last_five[i] - last_five[0]).abs() < 0.01,
                "Should converge to stable value, got {:?}",
                last_five
            );
        }

        // All values should be in 0-1 range
        for &val in &seq {
            assert!(val >= 0.0 && val <= 1.0, "Value {} out of range", val);
        }
    }

    #[test]
    fn test_logistic_map_chaotic() {
        // r=3.9 should produce chaotic behavior
        let seq = logistic_map(3.9, 0.5, 100);

        assert_eq!(seq.len(), 100);

        // Chaotic sequence should have high variance (not converging to a single point)
        let mean: f32 = seq.iter().sum::<f32>() / seq.len() as f32;
        let variance: f32 = seq.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / seq.len() as f32;

        assert!(
            variance > 0.01,
            "Chaotic sequence should have significant variance, got {}",
            variance
        );

        // All values should still be in 0-1 range
        for &val in &seq {
            assert!(val >= 0.0 && val <= 1.0, "Value {} out of range", val);
        }
    }

    #[test]
    fn test_logistic_map_dies_out() {
        // r=0.5 should converge to 0 (population dies)
        let seq = logistic_map(0.5, 0.5, 20);

        // Should approach 0
        let last_val = seq.last().unwrap();
        assert!(last_val < &0.1, "Should die out, last value: {}", last_val);
    }

    #[test]
    fn test_logistic_map_edge_cases() {
        // Initial value clamping
        let seq1 = logistic_map(2.0, -0.5, 10);
        assert_eq!(seq1[0], 0.0); // Should clamp to 0

        let seq2 = logistic_map(2.0, 1.5, 10);
        assert_eq!(seq2[0], 1.0); // Should clamp to 1

        // Single value
        let seq3 = logistic_map(3.0, 0.5, 1);
        assert_eq!(seq3.len(), 1);
        assert_eq!(seq3[0], 0.5);
    }

    #[test]
    fn test_random_walk_basic() {
        let walk = random_walk(100.0, 10.0, 20);

        assert_eq!(walk.len(), 20);
        assert_eq!(walk[0], 100.0); // Starts at initial value

        // Each step should change by at most step_size
        for i in 1..walk.len() {
            let delta = (walk[i] - walk[i - 1]).abs();
            assert!(delta <= 10.0, "Step {} too large: {}", i, delta);
        }
    }

    #[test]
    fn test_random_walk_unbounded() {
        // With enough steps and large step size, should be able to go far from start
        let walk = random_walk(0.0, 50.0, 100);

        assert_eq!(walk.len(), 100);
        assert_eq!(walk[0], 0.0);

        // Very likely (but not guaranteed) to have moved significantly
        // Using a statistical check rather than exact assertion
        let max_dist = walk.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
        assert!(max_dist > 20.0, "Walk should have moved away from start");
    }

    #[test]
    fn test_bounded_walk_stays_in_range() {
        let walk = bounded_walk(50.0, 10.0, 0.0, 100.0, 50);

        assert_eq!(walk.len(), 50);
        assert_eq!(walk[0], 50.0);

        // All values should be within bounds
        for &val in &walk {
            assert!(val >= 0.0 && val <= 100.0, "Value {} out of bounds", val);
        }
    }

    #[test]
    fn test_bounded_walk_clamping() {
        // Start near boundary and use large steps - should clamp
        let walk = bounded_walk(95.0, 20.0, 0.0, 100.0, 20);

        // Should still be within bounds despite large steps
        for &val in &walk {
            assert!(val >= 0.0 && val <= 100.0, "Value {} out of bounds", val);
        }
    }

    #[test]
    fn test_bounded_walk_clamps_initial() {
        // Initial value outside bounds should be clamped
        let walk1 = bounded_walk(-10.0, 5.0, 0.0, 100.0, 10);
        assert_eq!(walk1[0], 0.0); // Clamped to min

        let walk2 = bounded_walk(150.0, 5.0, 0.0, 100.0, 10);
        assert_eq!(walk2[0], 100.0); // Clamped to max
    }

    #[test]
    fn test_bounded_walk_normalized() {
        // Useful for parameter automation (0.0 to 1.0)
        let walk = bounded_walk(0.5, 0.1, 0.0, 1.0, 32);

        for &val in &walk {
            assert!(
                val >= 0.0 && val <= 1.0,
                "Normalized value {} out of range",
                val
            );
        }
    }

    #[test]
    fn test_thue_morse_basic() {
        // Test the first few terms match the known sequence
        let tm = thue_morse(16);

        assert_eq!(tm.len(), 16);

        // Known Thue-Morse sequence
        let expected = vec![0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0];
        assert_eq!(tm, expected);
    }

    #[test]
    fn test_thue_morse_construction() {
        // Verify the construction algorithm works
        let tm4 = thue_morse(4);
        assert_eq!(tm4, vec![0, 1, 1, 0]);

        let tm8 = thue_morse(8);
        assert_eq!(tm8, vec![0, 1, 1, 0, 1, 0, 0, 1]);

        // Verify that tm8 = tm4 + complement(tm4)
        let complement: Vec<u32> = tm4.iter().map(|&x| 1 - x).collect();
        let mut expected = tm4.clone();
        expected.extend(complement);
        assert_eq!(tm8, expected);
    }

    #[test]
    fn test_thue_morse_properties() {
        let tm = thue_morse(64);

        // Should have roughly equal 0s and 1s (fairness property)
        let ones = tm.iter().filter(|&&x| x == 1).count();
        let zeros = tm.iter().filter(|&&x| x == 0).count();

        assert_eq!(ones + zeros, 64);
        assert_eq!(ones, zeros); // Exactly equal for power-of-2 lengths
    }

    #[test]
    fn test_thue_morse_no_aaa() {
        // Thue-Morse never has three consecutive identical blocks
        let tm = thue_morse(100);

        // Check no "000" pattern
        for i in 0..tm.len().saturating_sub(2) {
            if tm[i] == 0 && tm[i + 1] == 0 {
                assert_ne!(tm[i + 2], 0, "Found three consecutive 0s at position {}", i);
            }
        }

        // Check no "111" pattern
        for i in 0..tm.len().saturating_sub(2) {
            if tm[i] == 1 && tm[i + 1] == 1 {
                assert_ne!(tm[i + 2], 1, "Found three consecutive 1s at position {}", i);
            }
        }
    }

    #[test]
    fn test_thue_morse_edge_cases() {
        let tm1 = thue_morse(1);
        assert_eq!(tm1, vec![0]);

        let tm2 = thue_morse(2);
        assert_eq!(tm2, vec![0, 1]);

        let tm3 = thue_morse(3);
        assert_eq!(tm3, vec![0, 1, 1]); // Truncated from [0,1,1,0]
    }

    #[test]
    fn test_thue_morse_as_rhythm() {
        // Convert to hit indices like Euclidean rhythms
        let tm = thue_morse(16);
        let hits: Vec<usize> = tm
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| i)
            .collect();

        // Should have 8 hits (half of 16)
        assert_eq!(hits.len(), 8);

        // Hits should be at positions where tm[i] == 1
        // Sequence: [0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0]
        // Indices:   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
        let expected_hits = vec![1, 2, 4, 7, 8, 11, 13, 14];
        assert_eq!(hits, expected_hits);
    }

    #[test]
    fn test_recaman_basic() {
        let seq = recaman(20);

        // Known Recamán sequence values
        let expected = vec![
            0, 1, 3, 6, 2, 7, 13, 20, 12, 21, 11, 22, 10, 23, 9, 24, 8, 25, 43, 62,
        ];
        assert_eq!(seq, expected);
    }

    #[test]
    fn test_recaman_properties() {
        let seq = recaman(50);

        // Recamán CAN have duplicates (when forward step lands on seen value)
        // This is actually part of the sequence's interesting behavior!

        // Check that it follows the rules:
        // - First value is 0
        assert_eq!(seq[0], 0);

        // - Sequence has correct length
        assert_eq!(seq.len(), 50);

        // - All values are non-negative (by construction with u32)
        // No assertion needed, u32 guarantees this
    }

    #[test]
    fn test_recaman_backward_when_possible() {
        let seq = recaman(10);

        // At step 1: 0 - 1 would be negative, so go forward: 0 + 1 = 1
        assert_eq!(seq[1], 1);

        // At step 2: 1 + 2 = 3 (can't go backward to -1)
        assert_eq!(seq[2], 3);

        // At step 3: 3 + 3 = 6 (can't go backward to 0, already seen)
        assert_eq!(seq[3], 6);

        // At step 4: 6 - 4 = 2 (can go backward, 2 not seen yet)
        assert_eq!(seq[4], 2);
    }

    #[test]
    fn test_recaman_edge_cases() {
        let empty = recaman(0);
        assert_eq!(empty, Vec::<u32>::new());

        let single = recaman(1);
        assert_eq!(single, vec![0]);

        let two = recaman(2);
        assert_eq!(two, vec![0, 1]);
    }

    #[test]
    fn test_van_der_corput_basic() {
        // Test binary (base 2) Van der Corput
        let seq = van_der_corput(8, 2);

        assert_eq!(seq.len(), 8);

        // Known values for base 2:
        // 1 -> 0.5, 2 -> 0.25, 3 -> 0.75, 4 -> 0.125, etc.
        assert!((seq[0] - 0.5).abs() < 0.001);
        assert!((seq[1] - 0.25).abs() < 0.001);
        assert!((seq[2] - 0.75).abs() < 0.001);
        assert!((seq[3] - 0.125).abs() < 0.001);
        assert!((seq[4] - 0.625).abs() < 0.001);
        assert!((seq[5] - 0.375).abs() < 0.001);
        assert!((seq[6] - 0.875).abs() < 0.001);
        assert!((seq[7] - 0.0625).abs() < 0.001);
    }

    #[test]
    fn test_van_der_corput_range() {
        // All values should be in [0, 1)
        let seq = van_der_corput(100, 2);

        for &val in &seq {
            assert!(val >= 0.0 && val < 1.0, "Value {} out of range", val);
        }
    }

    #[test]
    fn test_van_der_corput_base_3() {
        // Test with base 3
        let seq = van_der_corput(9, 3);

        assert_eq!(seq.len(), 9);

        // Should still be in [0, 1)
        for &val in &seq {
            assert!(val >= 0.0 && val < 1.0);
        }

        // Base 3 should give different distribution than base 2
        let seq2 = van_der_corput(9, 2);
        assert_ne!(seq, seq2);
    }

    #[test]
    fn test_van_der_corput_distribution() {
        // Van der Corput should fill space more evenly than random
        let seq = van_der_corput(32, 2);

        // Divide [0,1) into 8 bins and count how many values in each
        let mut bins = vec![0; 8];
        for &val in &seq {
            let bin = (val * 8.0).floor() as usize;
            bins[bin.min(7)] += 1;
        }

        // Each bin should have some values (quasi-random distributes evenly)
        for &count in &bins {
            assert!(count > 0, "Bin empty - poor distribution");
        }
    }

    #[test]
    fn test_cellular_automaton_rule30() {
        // Rule 30 - classic chaotic rule
        let ca = cellular_automaton(30, 5, 7, None);

        assert_eq!(ca.len(), 5); // 5 generations
        assert_eq!(ca[0].len(), 7); // Width 7

        // First generation should have center cell on
        assert_eq!(ca[0], vec![0, 0, 0, 1, 0, 0, 0]);

        // Each generation should be binary
        for gen in &ca {
            for &cell in gen {
                assert!(cell == 0 || cell == 1);
            }
        }
    }

    #[test]
    fn test_cellular_automaton_rule90() {
        // Rule 90 - Sierpinski triangle
        let ca = cellular_automaton(90, 8, 15, None);

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

    #[test]
    fn test_cellular_automaton_custom_initial() {
        // Test with custom initial state
        let initial = vec![1, 0, 1, 0, 1];
        let ca = cellular_automaton(30, 3, 5, Some(initial.clone()));

        assert_eq!(ca.len(), 3);
        assert_eq!(ca[0], initial); // First generation matches initial state
    }

    #[test]
    fn test_cellular_automaton_edge_cases() {
        // Empty width
        let empty = cellular_automaton(30, 5, 0, None);
        assert_eq!(empty, Vec::<Vec<u32>>::new());

        // Single step (just returns initial state)
        let single = cellular_automaton(30, 1, 7, None);
        assert_eq!(single.len(), 1);
        assert_eq!(single[0].len(), 7);

        // Zero steps (returns single generation)
        let zero = cellular_automaton(30, 0, 5, None);
        assert_eq!(zero.len(), 1);
    }

    #[test]
    fn test_cellular_automaton_rule110() {
        // Rule 110 - Turing complete!
        let ca = cellular_automaton(110, 10, 20, None);

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
    fn test_cellular_automaton_all_rules_binary() {
        // Test various rules all produce binary output
        for rule in [0, 30, 90, 110, 184, 255].iter() {
            let ca = cellular_automaton(*rule, 5, 10, None);

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
        let ca = cellular_automaton(30, 8, 16, None);

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
}
