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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        let p = primes(5);
        assert_eq!(p, vec![2, 3, 5, 7, 11]);
    }
}
