use crate::wavetable::{WAVETABLE, SAWTOOTH_WAVETABLE, SQUARE_WAVETABLE, TRIANGLE_WAVETABLE};

/// Different waveform types for synthesis
///
/// All waveforms use band-limited wavetables to prevent aliasing at high frequencies.
/// This ensures clean audio quality across the entire frequency spectrum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

impl Waveform {
    /// Generate a sample for this waveform at a given phase (0.0 to 1.0)
    ///
    /// All waveforms use pre-computed band-limited wavetables for high-quality,
    /// alias-free synthesis. This is much faster than computing waveforms
    /// mathematically and produces better audio quality.
    #[inline(always)]
    pub fn sample(&self, phase: f32) -> f32 {
        match self {
            Waveform::Sine => Self::sine(phase),
            Waveform::Square => Self::square(phase),
            Waveform::Sawtooth => Self::sawtooth(phase),
            Waveform::Triangle => Self::triangle(phase),
        }
    }

    /// Sine wave: smooth, pure tone (band-limited wavetable)
    ///
    /// Sine waves are already band-limited (single harmonic), so no aliasing occurs.
    #[inline]
    fn sine(phase: f32) -> f32 {
        WAVETABLE.sample(phase)
    }

    /// Square wave: rich in odd harmonics, hollow sound (band-limited wavetable)
    ///
    /// Uses additive synthesis with band-limited harmonics to prevent aliasing.
    /// Sounds identical to a perfect square wave but without the harsh digital artifacts.
    #[inline]
    fn square(phase: f32) -> f32 {
        SQUARE_WAVETABLE.sample(phase)
    }

    /// Sawtooth wave: rich in all harmonics, buzzy/bright sound (band-limited wavetable)
    ///
    /// Uses additive synthesis with band-limited harmonics to prevent aliasing.
    /// Produces a clean, bright sound suitable for leads and basses.
    #[inline]
    fn sawtooth(phase: f32) -> f32 {
        SAWTOOTH_WAVETABLE.sample(phase)
    }

    /// Triangle wave: smooth, few harmonics (band-limited wavetable)
    ///
    /// Uses additive synthesis with band-limited odd harmonics at 1/n² amplitude.
    /// Produces a rounder, softer sound than square waves.
    #[inline]
    fn triangle(phase: f32) -> f32 {
        TRIANGLE_WAVETABLE.sample(phase)
    }

    /// Generate a sample for a frequency at a given sample clock and sample rate
    pub fn sample_at(
        &self,
        frequency: f32,
        sample_clock: f32,
        sample_rate: f32,
    ) -> f32 {
        let phase = (sample_clock * frequency / sample_rate) % 1.0;
        self.sample(phase)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waveforms() {
        // Test that waveforms produce values in expected range
        let waveforms = [
            Waveform::Sine,
            Waveform::Square,
            Waveform::Sawtooth,
            Waveform::Triangle,
        ];

        for waveform in &waveforms {
            for i in 0..100 {
                let phase = i as f32 / 100.0;
                let sample = waveform.sample(phase);
                assert!(sample >= -1.0 && sample <= 1.0,
                    "{:?} produced out of range sample: {}", waveform, sample);
            }
        }
    }
}
