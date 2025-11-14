//! Generative and algorithmic music composition tools
//!
//! This module provides tools for creating music algorithmically, including
//! random walks, sequence transformations, and pattern manipulations.

// Submodules
mod random_walk;
mod transforms;
mod generators;
mod builders;

// Re-export public items
pub use random_walk::{random_walk_sequence, biased_random_walk_sequence};
pub use builders::{TransformBuilder, GeneratorBuilder};

// Note: Transform and generator method implementations are in their respective modules
// and are automatically available on TrackBuilder through the impl blocks
#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::Composition;
    use crate::composition::timing::Tempo;
    use crate::consts::notes::*;
    use crate::consts::scales::C4_MAJOR_SCALE;
    use crate::track::AudioEvent;

    // Sequence generation tests

    #[test]
    fn test_random_walk_sequence_generates_correct_length() {
        let seq = random_walk_sequence(3, 16, 0, 7);
        assert_eq!(seq.len(), 16);
    }

    #[test]
    fn test_random_walk_sequence_stays_in_bounds() {
        let seq = random_walk_sequence(5, 100, 0, 12);
        for &val in &seq {
            assert!(val < 12);
        }
    }

    #[test]
    fn test_random_walk_sequence_empty() {
        let seq = random_walk_sequence(0, 0, 0, 10);
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_biased_random_walk_tends_upward() {
        let seq = biased_random_walk_sequence(0, 50, 0, 20, 0.8);
        // Starting at 0 with 80% upward bias, should generally increase
        let avg = seq.iter().sum::<u32>() as f32 / seq.len() as f32;
        assert!(
            avg > 5.0,
            "Average {} should be > 5.0 with upward bias",
            avg
        );
    }

    #[test]
    fn test_biased_random_walk_tends_downward() {
        let seq = biased_random_walk_sequence(19, 50, 0, 20, 0.2);
        // Starting at 19 with 20% upward bias (80% down), should generally decrease
        let avg = seq.iter().sum::<u32>() as f32 / seq.len() as f32;
        assert!(
            avg < 15.0,
            "Average {} should be < 15.0 with downward bias",
            avg
        );
    }

    #[test]
    fn test_random_walk_sequence_with_sequence_from() {
        let mut comp = Composition::new(Tempo::new(120.0));
        let walk = random_walk_sequence(3, 16, 0, 7);
        comp.track("walk")
            .sequence_from(&walk, &C4_MAJOR_SCALE, 0.25);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks()[0].events.len(), 16);
    }

    // TrackBuilder method tests

    #[test]
    fn test_random_walk_generates_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("walk")
            .random_walk(C4, 16, 0.25, &C4_MAJOR_SCALE);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks()[0].events.len(), 16);
    }

    #[test]
    fn test_random_walk_stays_in_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("walk")
            .random_walk(C4, 32, 0.25, &C4_MAJOR_SCALE);

        let mixer = comp.into_mixer();

        // Check that all generated notes are in the scale
        for event in &mixer.tracks()[0].events {
            if let AudioEvent::Note(note) = event {
                let freq = note.frequencies[0];
                let in_scale = C4_MAJOR_SCALE
                    .iter()
                    .any(|&scale_note| (freq - scale_note).abs() < 0.1);
                assert!(in_scale, "Generated note {} not in scale", freq);
            }
        }
    }

    #[test]
    fn test_random_walk_empty_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("walk").random_walk(C4, 16, 0.25, &[]);

        let mixer = comp.into_mixer();
        // Should create no track with empty scale
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_shift_transposes_up() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .shift(12); // Transpose up one octave

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Check frequencies are transposed up an octave
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C5).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E5).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - G5).abs() < 0.1);
        }
    }

    #[test]
    fn test_shift_transposes_down() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .shift(-12); // Transpose down one octave

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Check frequencies are transposed down an octave
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C3).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E3).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - G3).abs() < 0.1);
        }
    }

    #[test]
    fn test_shift_by_zero_no_change() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .shift(0); // No transposition

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should still have original frequencies
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_shift_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().shift(12); // Shift with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_humanize_adds_variance() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4, C4], 0.25)
            .humanize(0.05, 0.2);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // Check that at least one note has non-exact timing or velocity
        // (with high probability due to randomness)
        let mut has_variance = false;
        for event in events {
            if let AudioEvent::Note(note) = event {
                // Check if timing is offset from exact multiples of 0.25
                let expected_times = [0.0, 0.25, 0.5, 0.75];
                let time_exact = expected_times.iter().any(|&t| (note.start_time - t).abs() < 0.001);
                if !time_exact || (note.velocity - 0.7).abs() > 0.01 {
                    has_variance = true;
                    break;
                }
            }
        }
        // With timing_variance=0.05 and velocity_variance=0.2, very likely to have variance
        assert!(has_variance, "Humanize should add some variance");
    }

    #[test]
    fn test_rotate_cycles_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .rotate(1); // Rotate forward by 1

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // After rotate(1): should be E4, G4, C5, C4
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
            assert_eq!(note.start_time, 0.0); // Timing unchanged
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
            assert_eq!(note.start_time, 0.25);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - C5).abs() < 0.1);
            assert_eq!(note.start_time, 0.5);
        }
        if let AudioEvent::Note(note) = &events[3] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.75);
        }
    }

    #[test]
    fn test_rotate_negative() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .rotate(-1); // Rotate backward by 1

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // After rotate(-1): should be G4, C4, E4
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
        }
    }

    #[test]
    fn test_rotate_by_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .rotate(0); // No rotation

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should remain unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_retrograde_reverses_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .retrograde(); // Reverse pitch sequence

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // After retrograde: should be C5, G4, E4, C4 (reversed pitches)
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C5).abs() < 0.1);
            assert_eq!(note.start_time, 0.0); // Timing unchanged
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
            assert_eq!(note.start_time, 0.25);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
            assert_eq!(note.start_time, 0.5);
        }
        if let AudioEvent::Note(note) = &events[3] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.75);
        }
    }

    #[test]
    fn test_retrograde_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().retrograde(); // Retrograde with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_mutate_changes_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .mutate(2); // Each note can shift by -2, -1, 0, +1, or +2 semitones

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // Check that timing is preserved
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert_eq!(note.start_time, 0.25);
        }

        // At least one note should be different (with very high probability)
        let original_freqs = vec![C4, E4, G4, C5];
        let mut has_mutation = false;
        for (i, event) in events.iter().enumerate() {
            if let AudioEvent::Note(note) = event {
                // Allow for ±2 semitones of variation
                let diff = (note.frequencies[0] - original_freqs[i]).abs();
                if diff > 0.1 {
                    has_mutation = true;
                    break;
                }
            }
        }
        // With 4 notes and mutate(2), very likely at least one changes
        assert!(has_mutation, "Mutate should change at least one note");
    }

    #[test]
    fn test_mutate_by_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .mutate(0); // No mutation

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should remain unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
        }
    }

    #[test]
    fn test_mutate_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().mutate(2); // Mutate with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_stack_octave_above() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4], 0.5)
            .stack(12, 1); // Add one octave above

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 1);

        // Should have 2 frequencies: C4 and C5
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 2);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.frequencies[1] - C5).abs() < 0.1);
        }
    }

    #[test]
    fn test_stack_two_octaves() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4], 0.5)
            .stack(12, 2); // Add two octaves above

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 1);

        // Should have 3 frequencies: C4, C5, C6
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 3);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.frequencies[1] - C5).abs() < 0.1);
            assert!((note.frequencies[2] - C6).abs() < 0.1);
        }
    }

    #[test]
    fn test_stack_octave_below() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4], 0.5)
            .stack(-12, 1); // Add one octave below

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 1);

        // Should have 2 frequencies: C4 and C3
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 2);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.frequencies[1] - C3).abs() < 0.1);
        }
    }

    #[test]
    fn test_stack_perfect_fifth() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4], 0.5)
            .stack(7, 2); // Add perfect fifth and major ninth

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 1);

        // Should have 3 frequencies: C4, G4 (+7), D5 (+14)
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 3);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.frequencies[1] - G4).abs() < 0.1);
            // D5 is 14 semitones above C4
            let d5 = C4 * 2.0_f32.powf(14.0 / 12.0);
            assert!((note.frequencies[2] - d5).abs() < 1.0);
        }
    }

    #[test]
    fn test_stack_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25) // C major chord
            .stack(12, 1); // Add octave above each note

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Each note should be doubled
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 2);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.frequencies[1] - C5).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert_eq!(note.num_freqs, 2);
            assert!((note.frequencies[0] - E4).abs() < 0.1);
            assert!((note.frequencies[1] - E5).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert_eq!(note.num_freqs, 2);
            assert!((note.frequencies[0] - G4).abs() < 0.1);
            assert!((note.frequencies[1] - G5).abs() < 0.1);
        }
    }

    #[test]
    fn test_stack_count_zero() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4], 0.5)
            .stack(12, 0); // No stacking

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should remain unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 1);
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_stack_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().stack(12, 1); // Stack with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_stretch_double_speed() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .stretch(2.0); // Half speed (twice as long)

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Check timing is stretched (doubled)
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.start_time - 0.0).abs() < 0.01);
            assert!((note.duration - 1.0).abs() < 0.01); // 0.5 * 2.0 = 1.0
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 1.0).abs() < 0.01); // 0.5 * 2.0 = 1.0
            assert!((note.duration - 1.0).abs() < 0.01);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.start_time - 2.0).abs() < 0.01); // 1.0 * 2.0 = 2.0
            assert!((note.duration - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_stretch_half_speed() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 1.0)
            .stretch(0.5); // Double speed (half duration)

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 2);

        // Check timing is compressed (halved)
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.start_time - 0.0).abs() < 0.01);
            assert!((note.duration - 0.5).abs() < 0.01); // 1.0 * 0.5 = 0.5
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 0.5).abs() < 0.01); // 1.0 * 0.5 = 0.5
            assert!((note.duration - 0.5).abs() < 0.01);
        }
    }

    #[test]
    fn test_stretch_by_one() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 0.25)
            .stretch(1.0); // No change

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should remain unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.start_time - 0.0).abs() < 0.01);
            assert!((note.duration - 0.25).abs() < 0.01);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 0.25).abs() < 0.01);
            assert!((note.duration - 0.25).abs() < 0.01);
        }
    }

    #[test]
    fn test_stretch_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().stretch(2.0); // Stretch with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_compress_to_target_duration() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)  // Naturally 0.75 beats
            .compress(0.5);  // Compress to 0.5 beats

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Should be compressed by factor of 0.5/0.75 = 0.666...
        // First note at 0.0 with duration ~0.167 (0.25 * 0.666)
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.start_time, 0.0);
            assert!((note.duration - 0.167).abs() < 0.01);
        }
        // Second note at ~0.167
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 0.167).abs() < 0.01);
        }
    }

    #[test]
    fn test_compress_expand_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 0.5)  // Naturally 1.0 beat
            .compress(2.0);  // Expand to 2.0 beats

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should be expanded by factor of 2.0
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.start_time, 0.0);
            assert!((note.duration - 1.0).abs() < 0.01);  // 0.5 * 2.0
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 1.0).abs() < 0.01);  // 0.5 * 2.0
        }
    }

    #[test]
    fn test_compress_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().compress(1.0);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_quantize_to_grid() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .wait(0.12)
            .note(&[C4], 0.25)
            .wait(0.11)
            .note(&[E4], 0.25)
            .wait(0.04)
            .note(&[G4], 0.25)
            .quantize(0.25);  // Snap to 16th note grid

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Check quantized to nearest 0.25 grid
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.start_time - 0.0).abs() < 0.01);  // 0.12 → 0.0
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 0.5).abs() < 0.01);  // 0.48 → 0.5
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.start_time - 0.75).abs() < 0.01); // 0.77 → 0.75
        }
    }

    #[test]
    fn test_quantize_eighth_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .wait(0.13)
            .note(&[C4], 0.25)
            .wait(0.24)
            .note(&[E4], 0.25)
            .quantize(0.5);  // Snap to 8th note grid

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Check quantized to nearest 0.5 grid
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.start_time - 0.0).abs() < 0.01);  // 0.13 → 0.0
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 0.5).abs() < 0.01);  // 0.62 → 0.5
        }
    }

    #[test]
    fn test_quantize_preserves_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .wait(0.12)
            .note(&[C4], 0.25)
            .wait(0.01)
            .note(&[E4], 0.25)
            .quantize(0.25);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Pitches should remain unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
        }
    }

    #[test]
    fn test_quantize_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().quantize(0.25); // Quantize with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_palindrome_mirrors_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .palindrome();  // Should become: C4, E4, G4, G4, E4, C4

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 6);  // Original 3 + mirrored 3

        // Check forward sequence
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
            assert_eq!(note.start_time, 0.25);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
            assert_eq!(note.start_time, 0.5);
        }

        // Check reversed sequence (should be G4, E4, C4)
        if let AudioEvent::Note(note) = &events[3] {
            assert!((note.frequencies[0] - G4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[4] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[5] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_palindrome_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().palindrome();

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_stutter_adds_repeats() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 0.25)
            .stutter(1.0, 3);  // 100% probability, 3 repeats

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have original 2 notes + 3 stutters for each = 2 + 6 = 8 total
        assert_eq!(events.len(), 8);

        // Check first note and its stutters
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.0);
        }
    }

    #[test]
    fn test_stutter_with_zero_probability() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .stutter(0.0, 4);  // 0% probability

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should remain unchanged (3 notes)
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_stutter_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().stutter(1.0, 4);

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_stutter_every_nth_note() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .stutter_every(2, 3);  // Every 2nd note stutters 3 times

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 4 original + (2 notes * 3 stutters) = 10 total
        assert_eq!(events.len(), 10);

        // Check that 2nd and 4th notes got stuttered
        // Original: C4, E4, G4, C5
        // E4 (2nd) and C5 (4th) should have 3 additional copies each
    }

    #[test]
    fn test_stutter_every_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().stutter_every(2, 4);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_granularize_splits_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("texture")
            .pattern_start()
            .note(&[C4], 1.0)
            .granularize(10);  // Split into 10 grains

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 10);  // 1 note → 10 grains

        // Check first grain
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert_eq!(note.start_time, 0.0);
            assert!((note.duration - 0.09).abs() < 0.01);  // 1.0/10 * 0.9 = 0.09
        }

        // Check second grain
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
            assert!((note.start_time - 0.1).abs() < 0.01);  // 1.0/10 = 0.1
        }
    }

    #[test]
    fn test_granularize_multiple_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("shimmer")
            .pattern_start()
            .notes(&[C4, E4], 0.5)
            .granularize(5);  // Split each into 5 grains

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 10);  // 2 notes * 5 grains = 10 total
    }

    #[test]
    fn test_granularize_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("texture").pattern_start().granularize(10);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_shuffle_reorders_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .shuffle(); // Random reorder

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // Check that timing is preserved
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.start_time, 0.0);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert_eq!(note.start_time, 0.25);
        }

        // Check that all original frequencies are still present (just reordered)
        let original_freqs = vec![C4, E4, G4, C5];
        let mut result_freqs = Vec::new();
        for event in events {
            if let AudioEvent::Note(note) = event {
                result_freqs.push(note.frequencies[0]);
            }
        }
        result_freqs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut sorted_original = original_freqs.clone();
        sorted_original.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for (a, b) in result_freqs.iter().zip(sorted_original.iter()) {
            assert!((a - b).abs() < 0.1);
        }
    }

    #[test]
    fn test_shuffle_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().shuffle(); // Shuffle with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_thin_removes_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4, C4, C4, C4, C4, C4], 0.125)
            .thin(0.5); // Keep ~50%

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have removed some notes (probabilistic, but with 8 notes and 50% probability,
        // very unlikely to keep all or remove all)
        assert!(events.len() < 8, "Should remove some notes");
        assert!(events.len() > 0, "Should keep some notes");
    }

    #[test]
    fn test_thin_keep_all() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .thin(1.0); // Keep all

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4); // All notes kept
    }

    #[test]
    fn test_thin_remove_all() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .thin(0.0); // Remove all

        let mixer = comp.into_mixer();
        // Track exists but all notes removed
        assert_eq!(mixer.tracks().len(), 1);
        assert_eq!(mixer.tracks()[0].events.len(), 0);
    }

    #[test]
    fn test_thin_with_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("melody").pattern_start().thin(0.5); // Thin with no notes

        let mixer = comp.into_mixer();
        // Should create no track since no notes were added
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_invert_mirrors_pitches() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .invert(C4);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];

        assert_eq!(track.events.len(), 3);

        // C4 inverted around C4 should stay C4
        if let AudioEvent::Note(note) = &track.events[0] {
            assert!((note.frequencies[0] - C4).abs() < 1.0);
        }

        // E4 is 4 semitones above C4, so inverted should be 4 semitones below
        // E4 = C4 * 2^(4/12), inverted = C4 * 2^(-4/12)
        if let AudioEvent::Note(note) = &track.events[1] {
            let expected = C4 * 2.0_f32.powf(-4.0 / 12.0); // G#3
            assert!((note.frequencies[0] - expected).abs() < 1.0);
        }
    }

    #[test]
    fn test_invert_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody").pattern_start().invert(C4); // Invert with no notes

        let mixer = comp.into_mixer();
        // Should create no track
        assert_eq!(mixer.tracks().len(), 0);
    }

    #[test]
    fn test_invert_constrained_keeps_in_range() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.5)
            .invert_constrained(C4, C3, C5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];

        // All notes should be between C3 and C5
        for event in &track.events {
            if let AudioEvent::Note(note) = event {
                assert!(note.frequencies[0] >= C3 - 1.0);
                assert!(note.frequencies[0] <= C5 + 1.0);
            }
        }
    }

    #[test]
    fn test_magnetize_snaps_to_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Chromatic notes that should snap to C major pentatonic (C, D, E, G, A)
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, CS4, D4, DS4, E4], 0.25)
            .magnetize(&[C4, D4, E4, G4, A4]);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 5);

        // CS4 should snap to C4 or D4 (equidistant, so could be either)
        if let AudioEvent::Note(note) = &events[1] {
            // CS4 is equidistant from C4 and D4 (1 semitone each way)
            let snapped_to_c = (note.frequencies[0] - C4).abs() < 1.0;
            let snapped_to_d = (note.frequencies[0] - D4).abs() < 1.0;
            assert!(snapped_to_c || snapped_to_d);
        }

        // DS4 should snap to D4 or E4 (equidistant)
        if let AudioEvent::Note(note) = &events[3] {
            // DS4 is equidistant from D4 and E4 (1 semitone each way)
            let snapped_to_d = (note.frequencies[0] - D4).abs() < 1.0;
            let snapped_to_e = (note.frequencies[0] - E4).abs() < 1.0;
            assert!(snapped_to_d || snapped_to_e);
        }
    }

    #[test]
    fn test_magnetize_empty_scale() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .magnetize(&[]); // Empty scale should do nothing

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Notes should be unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_magnetize_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .notes(&[C4, E4, G4], 0.25)
            .magnetize(&[C4, D4, E4]); // No pattern_start()

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_gravity_pulls_toward_center() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C3, C5], 0.5)
            .gravity(C4, 0.5); // 50% pull toward C4

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 2);

        // C3 should move up toward C4
        if let AudioEvent::Note(note) = &events[0] {
            assert!(note.frequencies[0] > C3);
            assert!(note.frequencies[0] < C4);
        }

        // C5 should move down toward C4
        if let AudioEvent::Note(note) = &events[1] {
            assert!(note.frequencies[0] < C5);
            assert!(note.frequencies[0] > C4);
        }
    }

    #[test]
    fn test_gravity_repels_from_center() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 0.5)
            .gravity(D4, -0.3); // Negative strength = repulsion

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 2);

        // C4 should move away (down) from D4
        if let AudioEvent::Note(note) = &events[0] {
            assert!(note.frequencies[0] < C4);
        }

        // E4 should move away (up) from D4
        if let AudioEvent::Note(note) = &events[1] {
            assert!(note.frequencies[0] > E4);
        }
    }

    #[test]
    fn test_gravity_zero_strength() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .gravity(D4, 0.0); // Zero strength = no effect

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Notes should be unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_gravity_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .notes(&[C4, E4, G4], 0.5)
            .gravity(D4, 0.5); // No pattern_start()

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_ripple_affects_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4, C4], 0.25)
            .ripple(0.02);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 4);

        // Later notes should be shifted more in time
        if let (AudioEvent::Note(note1), AudioEvent::Note(note2)) = (&events[0], &events[1]) {
            let expected_interval = 0.25;
            let actual_interval = note2.start_time - note1.start_time;
            // Second note should be pushed forward
            assert!(actual_interval > expected_interval);
        }

        if let (AudioEvent::Note(note2), AudioEvent::Note(note3)) = (&events[1], &events[2]) {
            let interval = note3.start_time - note2.start_time;
            // Third note interval should be even larger due to accumulation
            assert!(interval > 0.25);
        }
    }

    #[test]
    fn test_ripple_affects_pitch() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4], 0.25)
            .ripple(0.05);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // First note should be unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }

        // Later notes should be shifted up in pitch
        if let AudioEvent::Note(note) = &events[1] {
            assert!(note.frequencies[0] > C4);
        }

        if let AudioEvent::Note(note) = &events[2] {
            assert!(note.frequencies[0] > C4);
        }
    }

    #[test]
    fn test_ripple_zero_intensity() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C4, C4], 0.25)
            .ripple(0.0); // Zero intensity = no effect

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // All notes should be unchanged
        for event in events {
            if let AudioEvent::Note(note) = event {
                assert!((note.frequencies[0] - C4).abs() < 0.1);
            }
        }
    }

    #[test]
    fn test_ripple_no_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .notes(&[C4, C4, C4], 0.25)
            .ripple(0.05); // No pattern_start()

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_transform_closure_syntax() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Test the new closure-based .transform() API
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .transform(|t| t
                .shift(7)           // Transpose up a fifth
                .humanize(0.01, 0.05)
                .rotate(1)          // Rotate pitches
            )
            .wait(1.0);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // First note should be shifted and rotated (originally C4, rotated to E4, then shifted +7)
        if let AudioEvent::Note(note) = &events[0] {
            // E4 + 7 semitones = B4
            let expected = E4 * 2.0_f32.powf(7.0 / 12.0);
            assert!((note.frequencies[0] - expected).abs() < 1.0);
        }
    }

    #[test]
    fn test_transform_chaining_multiple_calls() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Test chaining multiple .transform() calls
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, D4, E4], 0.25)
            .transform(|t| t.shift(12))   // First transform block: up an octave
            .transform(|t| t.rotate(1))   // Second transform block: rotate
            .wait(1.0);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // All notes should be shifted up an octave
        // Original: C4, D4, E4 -> Shifted: C5, D5, E5 -> Rotated: D5, E5, C5
        for event in events {
            if let AudioEvent::Note(note) = event {
                // Should be in the 5th octave (C5 and above)
                assert!(note.frequencies[0] >= C5 - 1.0);
                assert!(note.frequencies[0] <= E5 + 1.0);
            }
        }
    }

    #[test]
    fn test_sieve_inclusive_keeps_only_range() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // C3 (~130Hz), E3 (~165Hz), G3 (~196Hz), C4 (~261Hz), E4 (~330Hz), G4 (~392Hz)
        comp.track("melody")
            .pattern_start()
            .notes(&[C3, E3, G3, C4, E4, G4], 0.25)
            .sieve_inclusive(150.0, 300.0);  // Keep only E3, G3, C4

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 3 notes remaining (E3, G3, C4)
        assert_eq!(events.len(), 3);

        // Verify all remaining frequencies are in range
        for event in events {
            if let AudioEvent::Note(note) = event {
                assert!(note.frequencies[0] >= 150.0);
                assert!(note.frequencies[0] <= 300.0);
            }
        }
    }

    #[test]
    fn test_sieve_inclusive_with_namespace() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .transform(|t| t.sieve_inclusive(250.0, 400.0));

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // C4 (~261Hz), E4 (~330Hz), and G4 (~392Hz) are in range
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_sieve_exclusive_removes_range() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Remove midrange frequencies
        comp.track("melody")
            .pattern_start()
            .notes(&[C3, E3, G3, C4, E4, G4], 0.25)
            .sieve_exclusive(150.0, 300.0);  // Remove E3, G3, C4

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 3 notes remaining (C3, E4, G4)
        assert_eq!(events.len(), 3);

        // Verify all remaining frequencies are outside range
        for event in events {
            if let AudioEvent::Note(note) = event {
                assert!(note.frequencies[0] < 150.0 || note.frequencies[0] > 300.0);
            }
        }
    }

    #[test]
    fn test_sieve_exclusive_with_namespace() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C3, E3, G3, C4, E4, G4], 0.25)
            .transform(|t| t.sieve_exclusive(150.0, 300.0));

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_sieve_inclusive_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Pattern start but no notes - should not crash
        let builder = comp.track("empty")
            .pattern_start()
            .sieve_inclusive(100.0, 500.0);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_sieve_exclusive_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder = comp.track("empty")
            .pattern_start()
            .sieve_exclusive(100.0, 500.0);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_sieve_chaining() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Chain both sieves
        comp.track("melody")
            .pattern_start()
            .notes(&[C3, E3, G3, C4, E4, G4, C5], 0.25)
            .transform(|t| t
                .sieve_exclusive(100.0, 200.0)  // Remove low frequencies (C3, E3, G3)
                .sieve_exclusive(380.0, 600.0)  // Remove high frequencies (G4, C5)
            );

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should only have C4 and E4 remaining (G3 removed by first, G4+C5 removed by second)
        assert_eq!(events.len(), 2);

        for event in events {
            if let AudioEvent::Note(note) = event {
                let freq = note.frequencies[0];
                // Should be between 200-380 Hz (C4 and E4)
                assert!(freq > 200.0 && freq < 380.0);
            }
        }
    }

    #[test]
    fn test_group_collapses_to_chord() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Sequential notes -> chord
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4, C5], 0.25)
            .group(2.0);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 1 chord instead of 4 notes
        assert_eq!(events.len(), 1);

        // Check it's a chord with 4 frequencies
        if let AudioEvent::Note(note) = &events[0] {
            assert_eq!(note.num_freqs, 4);
            assert_eq!(note.duration, 2.0);
        }
    }

    #[test]
    fn test_group_with_namespace() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.5)
            .transform(|t| t.group(1.5));

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_group_updates_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder = comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .group(3.0);

        // Cursor should be at pattern_start + duration
        assert_eq!(builder.cursor, 3.0);
    }

    #[test]
    fn test_group_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder = comp.track("empty")
            .pattern_start()
            .group(2.0);

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_duplicate_doubles_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .duplicate();

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 6 notes (3 original + 3 duplicated)
        assert_eq!(events.len(), 6);
    }

    #[test]
    fn test_duplicate_with_transform() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("harmony")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .duplicate()
            .transform(|t| t.shift(12));  // Shift duplicated notes

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 6 notes total
        assert_eq!(events.len(), 6);

        // Last 3 notes should be an octave higher
        if let (AudioEvent::Note(original), AudioEvent::Note(shifted)) = (&events[0], &events[3]) {
            let expected = original.frequencies[0] * 2.0; // One octave up
            assert!((shifted.frequencies[0] - expected).abs() < 1.0);
        }
    }

    #[test]
    fn test_duplicate_preserves_timing() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 0.5)
            .duplicate();

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Check timing is preserved but shifted
        if let (AudioEvent::Note(note1), AudioEvent::Note(note3)) = (&events[0], &events[2]) {
            assert_eq!(note1.start_time, 0.0);
            assert_eq!(note3.start_time, 1.0); // After 2 notes of 0.5 duration each
        }
    }

    #[test]
    fn test_duplicate_with_namespace() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .transform(|t| t.duplicate());

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        assert_eq!(events.len(), 6);
    }

    #[test]
    fn test_duplicate_updates_cursor() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder = comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .duplicate();

        // Cursor should be doubled (3 notes * 0.25 * 2 = 1.5)
        assert_eq!(builder.cursor, 1.5);
    }

    #[test]
    fn test_duplicate_empty_pattern() {
        let mut comp = Composition::new(Tempo::new(120.0));

        let builder = comp.track("empty")
            .pattern_start()
            .duplicate();

        assert_eq!(builder.cursor, 0.0);
    }

    #[test]
    fn test_group_then_duplicate() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Group into chord, then duplicate
        comp.track("chords")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .group(1.0)
            .duplicate();

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 2 chords
        assert_eq!(events.len(), 2);

        // Both should be chords with 3 frequencies
        for event in events {
            if let AudioEvent::Note(note) = event {
                assert_eq!(note.num_freqs, 3);
            }
        }
    }

    #[test]
    fn test_range_dilation_compress() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Pattern with wide range: C3, C5 (2 octaves = 24 semitones)
        comp.track("melody")
            .pattern_start()
            .notes(&[C3, C5], 0.5)
            .range_dilation(0.5); // Compress to half range (1 octave)

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 2);

        // Center should be geometric mean: sqrt(C3 * C5) = C4
        // C3 is 12 semitones below C4, compressed to 6 = G3
        // C5 is 12 semitones above C4, compressed to 6 = F#4
        if let (AudioEvent::Note(note1), AudioEvent::Note(note2)) =
            (&events[0], &events[1])
        {
            let center = C4;
            let expected_low = center * 2.0_f32.powf(-6.0 / 12.0); // G3
            let expected_high = center * 2.0_f32.powf(6.0 / 12.0); // F#4

            assert!((note1.frequencies[0] - expected_low).abs() < 1.0);
            assert!((note2.frequencies[0] - expected_high).abs() < 1.0);
        }
    }

    #[test]
    fn test_range_dilation_expand() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Pattern with narrow range: B3, C4 (1 semitone)
        comp.track("melody")
            .pattern_start()
            .notes(&[B3, CS4], 0.5)
            .range_dilation(2.0); // Expand to double range (2 semitones)

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 2);

        // Center is geometric mean of B3 and CS4
        // Intervals should be doubled
        if let (AudioEvent::Note(note1), AudioEvent::Note(note2)) =
            (&events[0], &events[1])
        {
            // Original interval was 2 semitones, doubled = 4 semitones
            let interval_semitones = 12.0 * (note2.frequencies[0] / note1.frequencies[0]).log2();
            assert!((interval_semitones - 4.0).abs() < 0.1);
        }
    }

    #[test]
    fn test_range_dilation_no_change() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4, G4], 0.25)
            .range_dilation(1.0); // Factor 1.0 = no change

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should be unchanged
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.frequencies[0] - E4).abs() < 0.1);
        }
    }

    #[test]
    fn test_shape_contour_smooth() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Wide intervals: C4, C6 (2 octaves = 24 semitones)
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, C6], 0.5)
            .shape_contour(0.5); // Smooth to 50% of interval = 12 semitones = 1 octave

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 2);

        if let (AudioEvent::Note(note1), AudioEvent::Note(note2)) =
            (&events[0], &events[1])
        {
            // First note (anchor) should be unchanged
            assert!((note1.frequencies[0] - C4).abs() < 0.1);

            // Second note should be 12 semitones above (C5)
            let expected = C4 * 2.0_f32.powf(12.0 / 12.0); // C5
            assert!((note2.frequencies[0] - expected).abs() < 1.0);
        }
    }

    #[test]
    fn test_shape_contour_exaggerate() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Small intervals: C4, D4 (2 semitones)
        comp.track("melody")
            .pattern_start()
            .notes(&[C4, D4], 0.5)
            .shape_contour(2.0); // Exaggerate to 200% = 4 semitones

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 2);

        if let (AudioEvent::Note(note1), AudioEvent::Note(note2)) =
            (&events[0], &events[1])
        {
            // First note (anchor) should be unchanged
            assert!((note1.frequencies[0] - C4).abs() < 0.1);

            // Second note should be 4 semitones above (E4)
            let expected = C4 * 2.0_f32.powf(4.0 / 12.0); // E4
            assert!((note2.frequencies[0] - expected).abs() < 0.5);
        }
    }

    #[test]
    fn test_shape_contour_single_note() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.5)
            .shape_contour(2.0); // Should do nothing with single note

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 1);

        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.frequencies[0] - C4).abs() < 0.1);
        }
    }

    #[test]
    fn test_echo_creates_repetitions() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.25)
            .echo(0.5, 3, 0.7); // 3 echoes, 0.5s apart, 70% decay

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 1 original + 3 echoes = 4 notes
        assert_eq!(events.len(), 4);

        // Check timing
        if let AudioEvent::Note(note) = &events[0] {
            assert!((note.start_time - 0.0).abs() < 0.01);
        }
        if let AudioEvent::Note(note) = &events[1] {
            assert!((note.start_time - 0.5).abs() < 0.01);
        }
        if let AudioEvent::Note(note) = &events[2] {
            assert!((note.start_time - 1.0).abs() < 0.01);
        }
        if let AudioEvent::Note(note) = &events[3] {
            assert!((note.start_time - 1.5).abs() < 0.01);
        }
    }

    #[test]
    fn test_echo_volume_decay() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.25)
            .echo(0.3, 2, 0.5); // 2 echoes, 50% decay

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;
        assert_eq!(events.len(), 3);

        // Check volume decay
        if let (
            AudioEvent::Note(note1),
            AudioEvent::Note(note2),
            AudioEvent::Note(note3),
        ) = (&events[0], &events[1], &events[2])
        {
            // First echo should be 50% of original
            assert!((note2.velocity - note1.velocity * 0.5).abs() < 0.01);

            // Second echo should be 25% of original (0.5^2)
            assert!((note3.velocity - note1.velocity * 0.25).abs() < 0.01);
        }
    }

    #[test]
    fn test_echo_with_multiple_notes() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .notes(&[C4, E4], 0.25)
            .echo(0.5, 2, 0.6);

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should have 2 original + 4 echoes (2 notes × 2 echoes) = 6 notes
        assert_eq!(events.len(), 6);
    }

    #[test]
    fn test_echo_zero_repeats() {
        let mut comp = Composition::new(Tempo::new(120.0));

        comp.track("melody")
            .pattern_start()
            .note(&[C4], 0.25)
            .echo(0.5, 0, 0.7); // 0 repeats = no echoes

        let mixer = comp.into_mixer();
        let events = &mixer.tracks()[0].events;

        // Should only have original note
        assert_eq!(events.len(), 1);
    }
}
