use crate::wavetable::WAVETABLE;

/// Different waveform types for synthesis
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

impl Waveform {
    /// Generate a sample for this waveform at a given phase (0.0 to 1.0)
    pub fn sample(&self, phase: f32) -> f32 {
        match self {
            Waveform::Sine => Self::sine(phase),
            Waveform::Square => Self::square(phase),
            Waveform::Sawtooth => Self::sawtooth(phase),
            Waveform::Triangle => Self::triangle(phase),
        }
    }

    /// Sine wave: smooth, pure tone (using fast wavetable lookup)
    fn sine(phase: f32) -> f32 {
        WAVETABLE.sine(phase)
    }

    /// Square wave: rich in odd harmonics, hollow sound
    fn square(phase: f32) -> f32 {
        if phase < 0.5 {
            1.0
        } else {
            -1.0
        }
    }

    /// Sawtooth wave: rich in harmonics, buzzy/bright sound
    fn sawtooth(phase: f32) -> f32 {
        2.0 * phase - 1.0
    }

    /// Triangle wave: smoother than square, fewer harmonics
    fn triangle(phase: f32) -> f32 {
        if phase < 0.5 {
            4.0 * phase - 1.0
        } else {
            -4.0 * phase + 3.0
        }
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
