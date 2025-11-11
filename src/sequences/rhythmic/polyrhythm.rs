/// Generate polyrhythm patterns
///
/// Polyrhythms are the simultaneous use of two or more different rhythms that are not
/// readily perceived as deriving from one another. They create tension and interest
/// through metric complexity.
///
/// This module provides utilities for generating polyrhythmic patterns, from simple
/// 3:4 patterns to complex multi-voice polyrhythms.
///
/// # Common Polyrhythms
/// - **3:2** (hemiola): Very common in African and Latin music
/// - **3:4**: Classic jazz/classical polyrhythm
/// - **5:4**: Creates flowing, liquid feel
/// - **7:8**: Complex but groovy
/// - **5:6:7**: Triple polyrhythm (advanced)
///
/// # Arguments
/// * `ratios` - Slice of integers representing each voice's subdivision
/// * `total_length` - Total duration in steps/beats to generate
///
/// # Returns
/// Vec of Vec<usize>, where each inner Vec contains the hit indices for that voice
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Simple 3:4 polyrhythm over 12 steps
/// let patterns = sequences::generate(&[3, 4], 12);
/// // patterns[0] = [0, 4, 8]     (3 hits)
/// // patterns[1] = [0, 3, 6, 9]  (4 hits)
///
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// comp.track("poly34")
///     .drum_grid(12, 0.25)
///     .kick(&patterns[0])   // 3 hits
///     .snare(&patterns[1]); // 4 hits
///
/// // Complex triple polyrhythm: 5:6:7 over 210 steps (LCM)
/// let triple = sequences::generate(&[5, 6, 7], 210);
/// comp.track("poly567")
///     .drum_grid(210, 0.125)
///     .kick(&triple[0])
///     .snare(&triple[1])
///     .hihat(&triple[2]);
///
/// // Hemiola pattern (3:2 over 6 steps)
/// let hemiola = sequences::generate(&[3, 2], 6);
/// ```
///
/// # Musical Applications
/// - **Rhythmic complexity**: Layer independent rhythms
/// - **Metric modulation**: Transition between time feels
/// - **Polymetric composition**: Multiple simultaneous meters
/// - **Cross-rhythms**: African, Afro-Cuban, Indian classical
/// - **Phasing**: Steve Reich-style gradual phase shifting
/// - **Tension and release**: Converge on strong beats
///
/// # Tips
/// - Use LCM (least common multiple) of ratios for complete cycles
/// - Powers of 2 ratios (2, 4, 8) align on strong beats
/// - Prime numbers (3, 5, 7, 11) create maximum tension
/// - Golden ratio (5:8, 8:13) from Fibonacci create flowing patterns
/// - Start simple (3:4) before moving to complex (7:11:13)
pub fn generate(ratios: &[usize], total_length: usize) -> Vec<Vec<usize>> {
    let mut patterns = Vec::with_capacity(ratios.len());

    for &ratio in ratios {
        if ratio == 0 {
            patterns.push(Vec::new());
            continue;
        }

        let mut hits = Vec::new();
        let step_size = total_length as f32 / ratio as f32;

        for i in 0..ratio {
            let hit = (i as f32 * step_size).round() as usize;
            if hit < total_length && !hits.contains(&hit) {
                hits.push(hit);
            }
        }

        hits.sort_unstable();
        patterns.push(hits);
    }

    patterns
}

/// Calculate the least common multiple (LCM) of a slice of numbers
///
/// Useful for finding the shortest complete cycle of a polyrhythm.
/// For example, 3:4 has LCM of 12, so the pattern repeats every 12 steps.
///
/// # Arguments
/// * `numbers` - Slice of positive integers
///
/// # Returns
/// The LCM of all numbers, or 0 if any number is 0
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // 3:4 polyrhythm repeats every 12 steps
/// assert_eq!(sequences::lcm(&[3, 4]), 12);
///
/// // 5:6:7 repeats every 210 steps
/// assert_eq!(sequences::lcm(&[5, 6, 7]), 210);
///
/// // Powers of 2 are simple
/// assert_eq!(sequences::lcm(&[2, 4, 8]), 8);
/// ```
pub fn lcm(numbers: &[usize]) -> usize {
    if numbers.is_empty() || numbers.contains(&0) {
        return 0;
    }

    numbers.iter().copied().reduce(lcm_two).unwrap()
}

/// Calculate LCM of two numbers
fn lcm_two(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd_two(a, b)
}

/// Calculate greatest common divisor (GCD) using Euclidean algorithm
fn gcd_two(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Generate a complete polyrhythm cycle
///
/// Convenience function that automatically calculates the LCM and generates
/// a complete cycle of the polyrhythm.
///
/// # Arguments
/// * `ratios` - Slice of integers representing each voice's subdivision
///
/// # Returns
/// Tuple of (patterns, cycle_length) where patterns are the hit indices
/// and cycle_length is the LCM (total length of one complete cycle)
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Generate complete 3:4 cycle (12 steps)
/// let (patterns, length) = sequences::polyrhythm_cycle(&[3, 4]);
/// assert_eq!(length, 12);
///
/// // Use in composition
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(140.0));
/// let (poly, len) = sequences::polyrhythm_cycle(&[5, 7]);
/// comp.track("poly57")
///     .drum_grid(len, 0.125)
///     .kick(&poly[0])
///     .snare(&poly[1]);
/// ```
pub fn polyrhythm_cycle(ratios: &[usize]) -> (Vec<Vec<usize>>, usize) {
    let cycle_length = lcm(ratios);
    let patterns = generate(ratios, cycle_length);
    (patterns, cycle_length)
}

/// Generate interleaved polyrhythm timing
///
/// Instead of returning hit indices, returns actual time values (in beats)
/// for when each voice should trigger. Useful for precise timing control.
///
/// # Arguments
/// * `ratios` - Slice of integers representing each voice's subdivision
/// * `cycle_duration` - Duration of one complete cycle in beats
///
/// # Returns
/// Vec of Vec<f32>, where each inner Vec contains timing in beats
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // 3:4 polyrhythm over 4 beats
/// let timings = sequences::polyrhythm_timings(&[3, 4], 4.0);
/// // timings[0] ≈ [0.0, 1.333, 2.666, 4.0]  (3 hits)
/// // timings[1] = [0.0, 1.0, 2.0, 3.0, 4.0] (4 hits)
///
/// # use tunes::prelude::*;
/// # let mut comp = Composition::new(Tempo::new(120.0));
/// // Use with explicit timing
/// for &t in &timings[0] {
///     comp.track("voice1").at(t).drum(DrumType::Kick);
/// }
/// ```
pub fn polyrhythm_timings(ratios: &[usize], cycle_duration: f32) -> Vec<Vec<f32>> {
    let mut timings = Vec::with_capacity(ratios.len());

    for &ratio in ratios {
        if ratio == 0 {
            timings.push(Vec::new());
            continue;
        }

        let mut times = Vec::new();
        let interval = cycle_duration / ratio as f32;

        for i in 0..=ratio {
            // Include final hit at cycle end
            let time = i as f32 * interval;
            if time <= cycle_duration {
                times.push(time);
            }
        }

        timings.push(times);
    }

    timings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polyrhythm_34() {
        // 3:4 over 12 steps
        let patterns = generate(&[3, 4], 12);
        assert_eq!(patterns.len(), 2);

        // First voice: 3 hits evenly spaced
        assert_eq!(patterns[0].len(), 3);
        assert_eq!(patterns[0], vec![0, 4, 8]);

        // Second voice: 4 hits evenly spaced
        assert_eq!(patterns[1].len(), 4);
        assert_eq!(patterns[1], vec![0, 3, 6, 9]);
    }

    #[test]
    fn test_lcm_calculation() {
        assert_eq!(lcm(&[3, 4]), 12);
        assert_eq!(lcm(&[2, 3, 4]), 12);
        assert_eq!(lcm(&[5, 7]), 35);
        assert_eq!(lcm(&[6, 8]), 24);
        assert_eq!(lcm(&[5, 6, 7]), 210);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd_two(12, 8), 4);
        assert_eq!(gcd_two(7, 5), 1);
        assert_eq!(gcd_two(100, 50), 50);
    }

    #[test]
    fn test_polyrhythm_cycle() {
        let (patterns, length) = polyrhythm_cycle(&[3, 4]);
        assert_eq!(length, 12);
        assert_eq!(patterns[0].len(), 3);
        assert_eq!(patterns[1].len(), 4);
    }

    #[test]
    fn test_polyrhythm_timings() {
        let timings = polyrhythm_timings(&[2, 3], 6.0);

        // First voice: 2 subdivisions over 6 beats = every 3 beats
        assert_eq!(timings[0].len(), 3); // 0, 3, 6
        assert!((timings[0][0] - 0.0).abs() < 0.001);
        assert!((timings[0][1] - 3.0).abs() < 0.001);
        assert!((timings[0][2] - 6.0).abs() < 0.001);

        // Second voice: 3 subdivisions over 6 beats = every 2 beats
        assert_eq!(timings[1].len(), 4); // 0, 2, 4, 6
        assert!((timings[1][0] - 0.0).abs() < 0.001);
        assert!((timings[1][1] - 2.0).abs() < 0.001);
        assert!((timings[1][2] - 4.0).abs() < 0.001);
        assert!((timings[1][3] - 6.0).abs() < 0.001);
    }

    #[test]
    fn test_triple_polyrhythm() {
        let patterns = generate(&[3, 5, 7], 105); // LCM of 3,5,7 is 105
        assert_eq!(patterns.len(), 3);
        assert_eq!(patterns[0].len(), 3); // 3 hits
        assert_eq!(patterns[1].len(), 5); // 5 hits
        assert_eq!(patterns[2].len(), 7); // 7 hits
    }

    #[test]
    fn test_empty_and_zero() {
        // Empty slice
        let patterns = generate(&[], 16);
        assert_eq!(patterns.len(), 0);

        // Zero ratio
        let patterns = generate(&[3, 0, 4], 12);
        assert_eq!(patterns.len(), 3);
        assert!(patterns[1].is_empty()); // Zero ratio produces no hits
    }
}
