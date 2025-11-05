/// Markov Chain - Probabilistic sequence generation
///
/// A Markov chain generates sequences based on learned transition probabilities.
/// It looks at the current state and chooses the next state based on observed patterns.
///
/// This is perfect for:
/// - Learning from existing music and generating similar patterns
/// - Creating variations that "sound like" a source material
/// - Building melodic or rhythmic patterns with statistical coherence
///
/// # Order
/// - **Order 1**: Next state depends only on current state (most common)
/// - **Order 2**: Next state depends on last 2 states (more context)
/// - **Order N**: Next state depends on last N states (even more memory)
///
/// Higher orders capture more complex patterns but need more training data.
///
/// # Arguments
/// * `transitions` - HashMap mapping states to possible next states with weights
/// * `start_state` - Initial state to begin generation
/// * `length` - Number of steps to generate
///
/// # Returns
/// Vector of states forming a Markov-generated sequence
///
/// # Examples
/// ```
/// use tunes::sequences;
/// use std::collections::HashMap;
///
/// // Simple melody generator (C major scale transitions)
/// let mut transitions: HashMap<u32, Vec<(u32, f32)>> = HashMap::new();
///
/// // From C (0): likely to go to D (1) or stay on C
/// transitions.insert(0, vec![(0, 0.2), (1, 0.5), (2, 0.3)]);
///
/// // From D (1): likely to go to E (2) or back to C (0)
/// transitions.insert(1, vec![(0, 0.3), (2, 0.6), (3, 0.1)]);
///
/// // From E (2): likely to go to G (4) or back to D (1)
/// transitions.insert(2, vec![(1, 0.3), (4, 0.5), (0, 0.2)]);
///
/// // From G (4): likely to resolve back down
/// transitions.insert(4, vec![(2, 0.4), (0, 0.6)]);
///
/// let melody = sequences::markov_chain(&transitions, 0, 16);
/// // Generates a 16-note melody following the transition probabilities
/// ```
///
/// # Musical Applications
/// - **Melody generation**: Learn from existing melodies, generate similar ones
/// - **Chord progressions**: Model harmonic movement (I→IV, IV→V, V→I, etc.)
/// - **Rhythm patterns**: Generate drum patterns based on observed transitions
/// - **Bass lines**: Create walking bass that follows learned patterns
/// - **Dynamics**: Model volume/intensity changes over time
/// - **Form**: Generate large-scale structural decisions (verse→chorus, etc.)
///
/// # Building Transition Tables
///
/// You can build transition tables from existing sequences:
/// ```
/// # use tunes::sequences;
/// # use std::collections::HashMap;
/// // Learn from a sequence
/// let training_data = vec![0, 1, 2, 1, 0, 1, 2, 4, 2, 0];
/// let transitions = sequences::build_markov_transitions(&training_data, 1);
///
/// // Now generate new sequences with similar patterns
/// let generated = sequences::markov_chain(&transitions, 0, 20);
/// ```
///
/// # Why Markov Chains Work for Music
/// Music has statistical structure - certain notes, chords, or rhythms are more
/// likely to follow others. Markov chains capture this without needing to understand
/// music theory. They create sequences that "feel" similar to the training data
/// while introducing variation and surprise.
pub fn markov_chain(
    transitions: &std::collections::HashMap<u32, Vec<(u32, f32)>>,
    start_state: u32,
    length: usize,
) -> Vec<u32> {
    use rand::Rng;
    let mut rng = rand::rng();
    let mut sequence = vec![start_state];
    let mut current_state = start_state;

    for _ in 1..length {
        if let Some(options) = transitions.get(&current_state) {
            if options.is_empty() {
                break; // No transitions available, stop
            }

            // Calculate total weight
            let total_weight: f32 = options.iter().map(|(_, weight)| weight).sum();

            if total_weight <= 0.0 {
                break; // Invalid weights, stop
            }

            // Choose next state based on weighted probabilities
            let mut random_value = rng.random_range(0.0..total_weight);

            for (state, weight) in options {
                random_value -= weight;
                if random_value <= 0.0 {
                    current_state = *state;
                    break;
                }
            }
        } else {
            // No transitions defined for current state, stop
            break;
        }

        sequence.push(current_state);
    }

    sequence
}

/// Build Markov chain transition table from training data
///
/// Analyzes a sequence and builds a transition probability table showing
/// how often each state follows another. This can then be used with `markov_chain()`
/// to generate new sequences with similar statistical properties.
///
/// # Arguments
/// * `data` - Training sequence to learn from
/// * `order` - Markov order (1 = first-order, looks at previous 1 state)
///
/// # Returns
/// HashMap mapping states to lists of (next_state, weight) tuples
///
/// # Examples
/// ```
/// use tunes::sequences;
///
/// // Learn from a simple melody pattern
/// let melody = vec![0, 2, 4, 2, 0, 2, 4, 5, 4, 2, 0];
/// let transitions = sequences::build_markov_transitions(&melody, 1);
///
/// // Now generate variations
/// let new_melody = sequences::markov_chain(&transitions, 0, 16);
/// // Will create melodies with similar step patterns
/// ```
///
/// # Note on Order
/// This currently implements first-order Markov chains (order=1).
/// Higher orders would require more complex state representation.
pub fn build_markov_transitions(
    data: &[u32],
    _order: usize, // Currently only order 1 is implemented
) -> std::collections::HashMap<u32, Vec<(u32, f32)>> {
    use std::collections::HashMap;

    let mut transition_counts: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

    // Count transitions
    for i in 0..data.len().saturating_sub(1) {
        let current = data[i];
        let next = data[i + 1];

        transition_counts
            .entry(current)
            .or_insert_with(HashMap::new)
            .entry(next)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    // Convert counts to probabilities (weights)
    let mut transitions: HashMap<u32, Vec<(u32, f32)>> = HashMap::new();

    for (state, next_states) in transition_counts {
        let total: u32 = next_states.values().sum();
        let total_f32 = total as f32;

        let weighted_options: Vec<(u32, f32)> = next_states
            .into_iter()
            .map(|(next_state, count)| (next_state, count as f32 / total_f32))
            .collect();

        transitions.insert(state, weighted_options);
    }

    transitions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_markov_chain_basic() {
        let mut transitions = HashMap::new();
        transitions.insert(0, vec![(1, 1.0)]);
        transitions.insert(1, vec![(2, 1.0)]);
        transitions.insert(2, vec![(0, 1.0)]);

        let seq = markov_chain(&transitions, 0, 7);
        assert_eq!(seq, vec![0, 1, 2, 0, 1, 2, 0]);
    }

    #[test]
    fn test_build_markov_transitions_simple() {
        let data = vec![0, 1, 0, 1, 0, 1];
        let transitions = build_markov_transitions(&data, 1);

        assert!(transitions.contains_key(&0));
        let from_0 = &transitions[&0];
        assert_eq!(from_0.len(), 1);
        assert_eq!(from_0[0].0, 1);
    }
}
