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
/// # Typical Values
/// - **n = 5-8**: Good for short phrases and melodic fragments
/// - **n = 10-12**: Medium-length sequences, complete melodies
/// - **n = 16-20**: Long evolving patterns, song structure
/// - **n > 20**: Watch out for huge numbers (F(20)=6765), normalize to usable ranges
///
/// # Recipe: Fibonacci Melody in Key
/// ```
/// use tunes::prelude::*;
/// use tunes::sequences;
///
/// let mut comp = Composition::new(Tempo::new(120.0));
///
/// // Generate Fibonacci and map to C major scale
/// let fib = sequences::fibonacci::generate(12);
/// let melody = sequences::map_to_scale(&fib, &sequences::Scale::major(), C4, 2);
///
/// comp.instrument("fib_melody", &Instrument::pluck())
///     .delay(Delay::new(0.375, 0.3, 0.5))
///     .notes(&melody, 0.25);
/// ```
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// let fib = sequences::fibonacci::generate(8);
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
/// let phrase_lengths = sequences::fibonacci::generate(5); // [1, 1, 2, 3, 5] beats
/// ```
///
/// # Musical Applications
/// - **Phrase structure**: Use Fibonacci numbers for phrase lengths (5-bar phrases, 8-bar sections)
/// - **Rhythmic patterns**: Note durations or rests following the sequence
/// - **Melodic intervals**: Map to semitone jumps for organic-sounding melodies
/// - **Formal structure**: Section lengths in larger compositions
/// - **Polyrhythms**: Layer rhythms based on different Fibonacci numbers (3 against 5, 5 against 8)
/// - **Dynamic curves**: Volume or filter changes following Fibonacci proportions
///
/// # Usage
/// ```
/// use tunes::sequences::fibonacci;
///
/// // Custom parameters
/// let seq = fibonacci::generate(12);
///
/// // Or use presets
/// let seq = fibonacci::classic();
/// ```

/// Generate Fibonacci sequence with custom length
///
/// See module-level documentation for details on the Fibonacci sequence,
/// musical applications, and typical values.
pub fn generate(n: usize) -> Vec<u32> {
    let mut fib = vec![1, 1];
    for i in 2..n {
        let next = fib[i - 1] + fib[i - 2];
        fib.push(next);
    }
    fib.truncate(n);
    fib
}

// ========== PRESETS ==========

/// Short Fibonacci sequence (6 terms) - quick phrases
pub fn short() -> Vec<u32> {
    generate(6)
}

/// Medium Fibonacci sequence (10 terms) - complete melodies
pub fn medium() -> Vec<u32> {
    generate(10)
}

/// Classic Fibonacci sequence (12 terms) - balanced length
pub fn classic() -> Vec<u32> {
    generate(12)
}

/// Long Fibonacci sequence (16 terms) - evolving patterns
pub fn long() -> Vec<u32> {
    generate(16)
}

/// Extended Fibonacci sequence (20 terms) - epic structures
pub fn extended() -> Vec<u32> {
    generate(20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let fib = generate(8);
        assert_eq!(fib, vec![1, 1, 2, 3, 5, 8, 13, 21]);
    }
}
