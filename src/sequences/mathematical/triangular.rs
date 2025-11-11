/// Generate triangular numbers: 1, 3, 6, 10, 15, 21...
///
/// The nth triangular number is the sum of the first n positive integers: T(n) = n*(n+1)/2
/// Creates natural ascending melodic contours.
///
/// # Examples
/// ```
/// use tunes::sequences;
/// let tri = sequences::triangular::generate(6);
/// assert_eq!(tri, vec![1, 3, 6, 10, 15, 21]);
/// ```
pub fn generate(n: usize) -> Vec<u32> {
    (1..=n)
        .map(|i| {
            let i = i as u32;
            (i * (i + 1)) / 2
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangular() {
        let tri = generate(6);
        assert_eq!(tri, vec![1, 3, 6, 10, 15, 21]);
    }
}

// ========== PRESETS ==========

/// Short triangular sequence - [1, 3, 6, 10, 15, 21]
pub fn short() -> Vec<u32> {
    generate(6)
}

/// Classic triangular sequence - [1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78]
pub fn classic() -> Vec<u32> {
    generate(12)
}

/// Extended triangular sequence - 16 terms
pub fn extended() -> Vec<u32> {
    generate(16)
}
