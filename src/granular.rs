//! Granular synthesis for texture creation and time/pitch manipulation
//!
//! Granular synthesis breaks audio into tiny "grains" (5-100ms) and rearranges/overlaps them
//! to create rich textures, time-stretch effects, or pitch shifting without artifacts.
//!
//! # Examples
//!
//! ```no_run
//! use tunes::prelude::*;
//! use tunes::granular::GranularParams;
//!
//! let mut comp = Composition::new(Tempo::new(120.0));
//!
//! // Create lush texture from a vocal sample
//! comp.track("texture")
//!     .granular("voice.wav", GranularParams::texture(), 5.0);
//!
//! // Time-stretch with minimal pitch change
//! comp.track("stretched")
//!     .granular("drums.wav", GranularParams::time_stretch(), 8.0);
//! ```

use crate::sample::Sample;
use crate::track::SampleEvent;
use rand::Rng;

/// Parameters for granular synthesis
///
/// Controls how audio is broken into grains and reassembled.
#[derive(Debug, Clone)]
pub struct GranularParams {
    /// Size of each grain in milliseconds (typically 5-100ms)
    ///
    /// - Smaller grains (5-20ms): Crisp, glitchy textures
    /// - Medium grains (30-60ms): Smooth textures, time-stretching
    /// - Large grains (70-100ms): More recognizable source material
    pub grain_size_ms: f32,

    /// Grain overlap density (0.0 to 1.0)
    ///
    /// - 0.0: No overlap (grains play back-to-back)
    /// - 0.5: 50% overlap (grains overlap halfway)
    /// - 0.8+: Heavy overlap (lush, thick textures)
    pub density: f32,

    /// Center position to read from source sample (0.0 to 1.0)
    ///
    /// - 0.0: Read from beginning
    /// - 0.5: Read from middle
    /// - 1.0: Read from end
    pub position: f32,

    /// Random variation around center position (0.0 to 1.0)
    ///
    /// - 0.0: All grains read from same position (frozen)
    /// - 0.1: Slight variation (subtle movement)
    /// - 0.5+: Wide variation (chaotic, textural)
    pub position_spread: f32,

    /// Random pitch variation per grain (0.0 to 0.5)
    ///
    /// - 0.0: No pitch variation
    /// - 0.1: Subtle detuning (chorus effect)
    /// - 0.3+: Extreme pitch variation (noisy, abstract)
    pub pitch_variation: f32,
}

impl GranularParams {
    /// Create granular parameters with specified settings
    ///
    /// # Arguments
    /// * `grain_size_ms` - Grain size in milliseconds (5-100ms)
    /// * `density` - Overlap density (0.0-1.0)
    /// * `position` - Center read position (0.0-1.0)
    /// * `position_spread` - Random position variation (0.0-1.0)
    /// * `pitch_variation` - Random pitch variation (0.0-0.5)
    pub fn new(
        grain_size_ms: f32,
        density: f32,
        position: f32,
        position_spread: f32,
        pitch_variation: f32,
    ) -> Self {
        Self {
            grain_size_ms: grain_size_ms.max(5.0).min(500.0),
            density: density.clamp(0.0, 1.0),
            position: position.clamp(0.0, 1.0),
            position_spread: position_spread.clamp(0.0, 1.0),
            pitch_variation: pitch_variation.clamp(0.0, 1.0),
        }
    }

    /// Default granular parameters (balanced)
    ///
    /// Good starting point for experimentation.
    pub fn default() -> Self {
        Self {
            grain_size_ms: 50.0,
            density: 0.5,
            position: 0.5,
            position_spread: 0.1,
            pitch_variation: 0.0,
        }
    }

    /// Rich, evolving texture preset
    ///
    /// Creates lush, pad-like textures from any source material.
    pub fn texture() -> Self {
        Self {
            grain_size_ms: 80.0,
            density: 0.8,
            position: 0.5,
            position_spread: 0.2,
            pitch_variation: 0.15,
        }
    }

    /// Time-stretch preset (minimal pitch change)
    ///
    /// Stretches time while keeping pitch relatively stable.
    pub fn time_stretch() -> Self {
        Self {
            grain_size_ms: 40.0,
            density: 0.7,
            position: 0.5,
            position_spread: 0.05,
            pitch_variation: 0.02,
        }
    }

    /// Frozen moment preset
    ///
    /// Freezes a moment in time, creating a sustained pad from a single position.
    pub fn freeze() -> Self {
        Self {
            grain_size_ms: 60.0,
            density: 0.85,
            position: 0.5,
            position_spread: 0.02,
            pitch_variation: 0.05,
        }
    }

    /// Glitch/stutter preset
    ///
    /// Creates rhythmic, stuttering glitch effects.
    pub fn glitch() -> Self {
        Self {
            grain_size_ms: 15.0,
            density: 0.3,
            position: 0.5,
            position_spread: 0.4,
            pitch_variation: 0.25,
        }
    }

    /// Cloud/swarm preset
    ///
    /// Dense cloud of micro-grains with lots of variation.
    pub fn cloud() -> Self {
        Self {
            grain_size_ms: 25.0,
            density: 0.9,
            position: 0.5,
            position_spread: 0.3,
            pitch_variation: 0.2,
        }
    }
}

/// Apply Hann window envelope to a sample to prevent clicks
///
/// The Hann window smoothly fades in/out the grain edges, eliminating
/// discontinuities that would cause audible clicks.
///
/// # Arguments
/// * `sample` - The sample to apply the window to
///
/// # Returns
/// A new sample with Hann window applied
fn apply_hann_window(sample: Sample) -> Sample {
    let mut data = (*sample.data).clone();
    let len = data.len();

    if len == 0 {
        return sample;
    }

    // Apply Hann window: w(n) = 0.5 * (1 - cos(2πn/N))
    for i in 0..len {
        let hann = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / len as f32).cos());
        data[i] *= hann;
    }

    Sample::from_mono(data, sample.sample_rate)
}

/// Generate granular synthesis events from a source sample
///
/// Creates multiple overlapping grain events that will be mixed together
/// by the audio engine.
///
/// # Arguments
/// * `source_sample` - The sample to granulate
/// * `params` - Granular synthesis parameters
/// * `output_duration` - How long the granular effect should last (seconds)
/// * `start_time` - When to start playing in the composition (seconds)
///
/// # Returns
/// Vector of SampleEvents representing all the grains
pub fn create_granular_events(
    source_sample: &Sample,
    params: &GranularParams,
    output_duration: f32,
    start_time: f32,
) -> Vec<SampleEvent> {
    let mut rng = rand::rng();
    let mut events = Vec::new();

    // Convert grain size to seconds
    let grain_size = params.grain_size_ms / 1000.0;

    // Calculate grain spacing based on density
    // density 0.0 = no overlap (spacing = grain_size)
    // density 1.0 = maximum overlap (spacing = grain_size / 4)
    let grain_spacing = grain_size * (1.0 - params.density * 0.75);

    // Avoid infinite loops with tiny spacing
    if grain_spacing <= 0.0 {
        return events;
    }

    // Calculate how many grains we need
    let num_grains = ((output_duration / grain_spacing).ceil() as usize).min(10000);

    // Create each grain
    let mut current_time = start_time;
    for _ in 0..num_grains {
        if current_time >= start_time + output_duration {
            break;
        }

        // Random position in source sample
        let position_variation = if params.position_spread > 0.0 {
            rng.random_range(-params.position_spread..params.position_spread)
        } else {
            0.0
        };
        let read_position = (params.position + position_variation).clamp(0.0, 1.0);

        // Calculate sample offset
        let sample_start = (read_position * source_sample.duration).max(0.0);
        let sample_end = (sample_start + grain_size).min(source_sample.duration);

        // Skip if grain would be too short
        if sample_end - sample_start < 0.001 {
            current_time += grain_spacing;
            continue;
        }

        // Extract grain slice
        let grain_sample = match source_sample.slice(sample_start, sample_end) {
            Ok(sample) => sample,
            Err(_) => {
                // Skip this grain if slicing fails
                current_time += grain_spacing;
                continue;
            }
        };

        // Apply Hann window (envelope to smooth edges)
        let grain_with_envelope = apply_hann_window(grain_sample);

        // Random pitch variation
        let pitch_variation = if params.pitch_variation > 0.0 {
            1.0 + rng.random_range(-params.pitch_variation..params.pitch_variation)
        } else {
            1.0
        };

        // Create the sample event
        events.push(SampleEvent {
            sample: grain_with_envelope,
            start_time: current_time,
            playback_rate: pitch_variation,
            volume: 1.0,
        });

        current_time += grain_spacing;
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_sample(duration_sec: f32) -> Sample {
        let sample_rate = 44100;
        let num_samples = (duration_sec * sample_rate as f32) as usize;

        // Generate a simple sine wave
        let mut data = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            data.push((2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.5);
        }

        Sample::from_mono(data, sample_rate)
    }

    #[test]
    fn test_granular_params_clamping() {
        let params = GranularParams::new(150.0, 1.5, 2.0, -0.5, 0.8);

        assert!(params.grain_size_ms >= 5.0 && params.grain_size_ms <= 500.0);
        assert!(params.density >= 0.0 && params.density <= 1.0);
        assert!(params.position >= 0.0 && params.position <= 1.0);
        assert!(params.position_spread >= 0.0 && params.position_spread <= 1.0);
        assert!(params.pitch_variation >= 0.0 && params.pitch_variation <= 1.0);
    }

    #[test]
    fn test_granular_params_presets() {
        let texture = GranularParams::texture();
        assert!(texture.density > 0.5, "Texture should have high density");

        let freeze = GranularParams::freeze();
        assert!(
            freeze.position_spread < 0.1,
            "Freeze should have low spread"
        );

        let glitch = GranularParams::glitch();
        assert!(
            glitch.grain_size_ms < 30.0,
            "Glitch should have small grains"
        );
    }

    #[test]
    fn test_hann_window_application() {
        let sample = create_test_sample(0.1);
        let windowed = apply_hann_window(sample.clone());

        assert_eq!(windowed.data.len(), sample.data.len());

        // Hann window should reduce amplitude at edges
        let first = windowed.data[0].abs();
        let middle = windowed.data[windowed.data.len() / 2].abs();
        let last = windowed.data[windowed.data.len() - 1].abs();

        assert!(first < middle, "Start should be quieter than middle");
        assert!(last < middle, "End should be quieter than middle");
    }

    #[test]
    fn test_hann_window_empty_sample() {
        let empty = Sample::from_mono(vec![], 44100);
        let windowed = apply_hann_window(empty.clone());
        assert_eq!(windowed.data.len(), 0);
    }

    #[test]
    fn test_create_granular_events_basic() {
        let sample = create_test_sample(1.0);
        let params = GranularParams::default();
        let events = create_granular_events(&sample, &params, 2.0, 0.0);

        assert!(!events.is_empty(), "Should generate grains");

        // All events should be within the output duration
        for event in &events {
            assert!(event.start_time >= 0.0 && event.start_time <= 2.0);
        }
    }

    #[test]
    fn test_granular_density_affects_grain_count() {
        let sample = create_test_sample(1.0);
        let output_duration = 1.0;

        let low_density = GranularParams::new(50.0, 0.2, 0.5, 0.1, 0.0);
        let high_density = GranularParams::new(50.0, 0.8, 0.5, 0.1, 0.0);

        let low_events = create_granular_events(&sample, &low_density, output_duration, 0.0);
        let high_events = create_granular_events(&sample, &high_density, output_duration, 0.0);

        assert!(
            high_events.len() > low_events.len(),
            "Higher density should create more grains: {} vs {}",
            high_events.len(),
            low_events.len()
        );
    }

    #[test]
    fn test_granular_grain_size_affects_duration() {
        let sample = create_test_sample(1.0);
        let params = GranularParams::new(50.0, 0.5, 0.5, 0.0, 0.0);
        let events = create_granular_events(&sample, &params, 1.0, 0.0);

        // Check that grains have approximately correct duration
        if !events.is_empty() {
            let grain_duration = events[0].sample.duration;
            let expected = 0.05; // 50ms
            assert!(
                (grain_duration - expected).abs() < 0.01,
                "Grain duration {} should be close to {}",
                grain_duration,
                expected
            );
        }
    }

    #[test]
    fn test_granular_position_spread() {
        let sample = create_test_sample(2.0);

        // No spread - all grains from same position
        let no_spread = GranularParams::new(50.0, 0.5, 0.5, 0.0, 0.0);
        let events_no_spread = create_granular_events(&sample, &no_spread, 0.5, 0.0);

        // High spread - grains from different positions
        let high_spread = GranularParams::new(50.0, 0.5, 0.5, 0.5, 0.0);
        let events_high_spread = create_granular_events(&sample, &high_spread, 0.5, 0.0);

        assert!(!events_no_spread.is_empty());
        assert!(!events_high_spread.is_empty());

        // Both should generate events
        // (Hard to test randomness deterministically without seeding)
    }

    #[test]
    fn test_granular_pitch_variation() {
        let sample = create_test_sample(1.0);
        let params = GranularParams::new(50.0, 0.5, 0.5, 0.1, 0.3);
        let events = create_granular_events(&sample, &params, 1.0, 0.0);

        // Should have some pitch variation in playback rates
        let playback_rates: Vec<f32> = events.iter().map(|e| e.playback_rate).collect();

        // Check that not all playback rates are identical (with high variation)
        let first_rate = playback_rates[0];
        let has_variation = playback_rates
            .iter()
            .any(|&rate| (rate - first_rate).abs() > 0.05);

        if params.pitch_variation > 0.1 {
            assert!(
                has_variation,
                "With pitch_variation={}, should have varying playback rates",
                params.pitch_variation
            );
        }
    }

    #[test]
    fn test_granular_respects_start_time() {
        let sample = create_test_sample(1.0);
        let params = GranularParams::default();
        let start_time = 5.0;
        let events = create_granular_events(&sample, &params, 1.0, start_time);

        // All events should start at or after start_time
        for event in &events {
            assert!(
                event.start_time >= start_time,
                "Event at {} should be >= {}",
                event.start_time,
                start_time
            );
        }
    }

    #[test]
    fn test_granular_zero_duration() {
        let sample = create_test_sample(1.0);
        let params = GranularParams::default();
        let events = create_granular_events(&sample, &params, 0.0, 0.0);

        assert_eq!(events.len(), 0, "Zero duration should produce no grains");
    }

    #[test]
    fn test_granular_very_short_sample() {
        let sample = create_test_sample(0.01); // 10ms sample
        let params = GranularParams::new(50.0, 0.5, 0.5, 0.1, 0.0); // 50ms grains
        let events = create_granular_events(&sample, &params, 1.0, 0.0);

        // Should still generate events, even if grains are longer than source
        // (they'll just use the full sample)
        assert!(!events.is_empty());
    }
}
