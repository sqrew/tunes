use std::f32::consts::PI;

/// Types of filters available
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    None, // Bypass filter
}

/// A simple state-variable filter implementation
/// This is a 2-pole resonant filter with controllable cutoff and resonance
#[derive(Debug, Clone, Copy)]
pub struct Filter {
    pub filter_type: FilterType,
    pub cutoff: f32,      // Cutoff frequency in Hz
    pub resonance: f32,   // Resonance/Q factor (0.0 to 1.0)

    // Internal state for filter processing
    low: f32,
    high: f32,
    band: f32,
    notch: f32,

    // Smoothing for parameter changes to avoid zipper noise
    smooth_cutoff: f32,
    smooth_resonance: f32,
}

impl Filter {
    /// Create a new filter
    pub fn new(filter_type: FilterType, cutoff: f32, resonance: f32) -> Self {
        let cutoff = cutoff.clamp(20.0, 20000.0);
        let resonance = resonance.clamp(0.0, 0.99);
        Self {
            filter_type,
            cutoff,
            resonance,
            low: 0.0,
            high: 0.0,
            band: 0.0,
            notch: 0.0,
            smooth_cutoff: cutoff,
            smooth_resonance: resonance,
        }
    }

    /// Create a low-pass filter
    pub fn low_pass(cutoff: f32, resonance: f32) -> Self {
        Self::new(FilterType::LowPass, cutoff, resonance)
    }

    /// Create a high-pass filter
    pub fn high_pass(cutoff: f32, resonance: f32) -> Self {
        Self::new(FilterType::HighPass, cutoff, resonance)
    }

    /// Create a band-pass filter
    pub fn band_pass(cutoff: f32, resonance: f32) -> Self {
        Self::new(FilterType::BandPass, cutoff, resonance)
    }

    /// Create a bypass filter (no filtering)
    pub fn none() -> Self {
        Self::new(FilterType::None, 20000.0, 0.0)
    }

    /// Process a single sample through the filter
    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        if self.filter_type == FilterType::None {
            return input;
        }

        // Smooth parameter changes to avoid zipper noise (simple one-pole smoothing)
        let smoothing = 0.999; // Higher = smoother but slower response
        self.smooth_cutoff = self.smooth_cutoff * smoothing + self.cutoff * (1.0 - smoothing);
        self.smooth_resonance = self.smooth_resonance * smoothing + self.resonance * (1.0 - smoothing);

        // Calculate filter coefficients using smoothed parameters
        let f = 2.0 * (PI * self.smooth_cutoff / sample_rate).sin();
        let q = 1.0 - self.smooth_resonance;

        // State-variable filter algorithm
        self.low += f * self.band;
        self.high = input - self.low - q * self.band;
        self.band += f * self.high;
        self.notch = self.high + self.low;

        // Stability check - prevent denormals and infinity
        if !self.low.is_finite() || self.low.abs() > 10.0 {
            self.reset();
            return input;
        }

        // Return the appropriate filter output
        let output = match self.filter_type {
            FilterType::LowPass => self.low,
            FilterType::HighPass => self.high,
            FilterType::BandPass => self.band,
            FilterType::None => input,
        };

        // Clamp output to prevent explosion
        output.clamp(-2.0, 2.0)
    }

    /// Reset the filter state (call when starting new notes or to avoid clicks)
    pub fn reset(&mut self) {
        self.low = 0.0;
        self.high = 0.0;
        self.band = 0.0;
        self.notch = 0.0;
        self.smooth_cutoff = self.cutoff;
        self.smooth_resonance = self.resonance;
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_creation() {
        let lpf = Filter::low_pass(1000.0, 0.5);
        assert_eq!(lpf.filter_type, FilterType::LowPass);
        assert_eq!(lpf.cutoff, 1000.0);

        let hpf = Filter::high_pass(500.0, 0.3);
        assert_eq!(hpf.filter_type, FilterType::HighPass);

        let bpf = Filter::band_pass(2000.0, 0.7);
        assert_eq!(bpf.filter_type, FilterType::BandPass);
    }

    #[test]
    fn test_filter_bypass() {
        let mut filter = Filter::none();
        let input = 0.5;
        let output = filter.process(input, 44100.0);
        assert_eq!(input, output);
    }
}
