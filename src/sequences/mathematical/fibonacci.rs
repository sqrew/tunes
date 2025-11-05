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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let fib = fibonacci(8);
        assert_eq!(fib, vec![1, 1, 2, 3, 5, 8, 13, 21]);
    }
}
