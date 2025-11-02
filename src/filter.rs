use std::f32::consts::PI;

/// Types of filters available
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    Notch,    // Notch/band-reject filter (removes frequencies at cutoff)
    AllPass,  // All-pass filter (affects phase but not amplitude)
    None,     // Bypass filter
}

/// Filter slope/steepness options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterSlope {
    Pole12dB, // 12dB/octave - smoother, more musical
    Pole24dB, // 24dB/octave - steeper, more aggressive
}

/// A simple state-variable filter implementation
/// This is a 2-pole resonant filter with controllable cutoff and resonance
#[derive(Debug, Clone, Copy)]
pub struct Filter {
    pub filter_type: FilterType,
    pub cutoff: f32,    // Cutoff frequency in Hz
    pub resonance: f32, // Resonance/Q factor (0.0 to 1.0)
    pub slope: FilterSlope, // Filter steepness (12dB or 24dB per octave)

    // Internal state for filter processing
    low: f32,
    high: f32,
    band: f32,
    notch: f32,

    // Second stage for 24dB/octave mode
    low2: f32,
    high2: f32,
    band2: f32,
    notch2: f32,

    // Smoothing for parameter changes to avoid zipper noise
    smooth_cutoff: f32,
    smooth_resonance: f32,
}

impl Filter {
    /// Create a new filter with default 12dB/octave slope
    pub fn new(filter_type: FilterType, cutoff: f32, resonance: f32) -> Self {
        Self::with_slope(filter_type, cutoff, resonance, FilterSlope::Pole12dB)
    }

    /// Create a new filter with specified slope
    pub fn with_slope(filter_type: FilterType, cutoff: f32, resonance: f32, slope: FilterSlope) -> Self {
        let cutoff = cutoff.clamp(20.0, 20000.0);
        let resonance = resonance.clamp(0.0, 0.99);
        Self {
            filter_type,
            cutoff,
            resonance,
            slope,
            low: 0.0,
            high: 0.0,
            band: 0.0,
            notch: 0.0,
            low2: 0.0,
            high2: 0.0,
            band2: 0.0,
            notch2: 0.0,
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

    /// Create a notch filter (band-reject)
    pub fn notch(cutoff: f32, resonance: f32) -> Self {
        Self::new(FilterType::Notch, cutoff, resonance)
    }

    /// Create an all-pass filter (phase shift without amplitude change)
    pub fn all_pass(cutoff: f32, resonance: f32) -> Self {
        Self::new(FilterType::AllPass, cutoff, resonance)
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
        self.smooth_resonance =
            self.smooth_resonance * smoothing + self.resonance * (1.0 - smoothing);

        // Calculate filter coefficients using smoothed parameters
        let f = 2.0 * (PI * self.smooth_cutoff / sample_rate).sin();
        let q = 1.0 - self.smooth_resonance;

        // First stage: State-variable filter algorithm
        self.low += f * self.band;
        self.high = input - self.low - q * self.band;
        self.band += f * self.high;
        self.notch = self.high + self.low;

        // Stability check - prevent denormals and infinity
        if !self.low.is_finite() || self.low.abs() > 10.0 {
            self.reset();
            return input;
        }

        // Get output from first stage
        let stage1_output = match self.filter_type {
            FilterType::LowPass => self.low,
            FilterType::HighPass => self.high,
            FilterType::BandPass => self.band,
            FilterType::Notch => self.notch,
            FilterType::AllPass => self.notch - self.band,
            FilterType::None => input,
        };

        // For 24dB/octave, run a second stage in series
        let output = match self.slope {
            FilterSlope::Pole12dB => stage1_output,
            FilterSlope::Pole24dB => {
                // Second stage processing (same algorithm, different state)
                self.low2 += f * self.band2;
                self.high2 = stage1_output - self.low2 - q * self.band2;
                self.band2 += f * self.high2;
                self.notch2 = self.high2 + self.low2;

                // Stability check for second stage
                if !self.low2.is_finite() || self.low2.abs() > 10.0 {
                    self.reset();
                    return input;
                }

                // Get output from second stage
                match self.filter_type {
                    FilterType::LowPass => self.low2,
                    FilterType::HighPass => self.high2,
                    FilterType::BandPass => self.band2,
                    FilterType::Notch => self.notch2,
                    FilterType::AllPass => self.notch2 - self.band2,
                    FilterType::None => stage1_output,
                }
            }
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
        self.low2 = 0.0;
        self.high2 = 0.0;
        self.band2 = 0.0;
        self.notch2 = 0.0;
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

    #[test]
    fn test_notch_filter_creation() {
        let notch = Filter::notch(1000.0, 0.7);
        assert_eq!(notch.filter_type, FilterType::Notch);
        assert_eq!(notch.cutoff, 1000.0);
        assert_eq!(notch.resonance, 0.7);
    }

    #[test]
    fn test_allpass_filter_creation() {
        let allpass = Filter::all_pass(2000.0, 0.5);
        assert_eq!(allpass.filter_type, FilterType::AllPass);
        assert_eq!(allpass.cutoff, 2000.0);
    }

    #[test]
    fn test_filter_slopes() {
        let filter_12db = Filter::with_slope(FilterType::LowPass, 1000.0, 0.5, FilterSlope::Pole12dB);
        assert_eq!(filter_12db.slope, FilterSlope::Pole12dB);

        let filter_24db = Filter::with_slope(FilterType::LowPass, 1000.0, 0.5, FilterSlope::Pole24dB);
        assert_eq!(filter_24db.slope, FilterSlope::Pole24dB);
    }

    #[test]
    fn test_24db_filter_processes() {
        let mut filter = Filter::with_slope(FilterType::LowPass, 1000.0, 0.5, FilterSlope::Pole24dB);
        let input = 0.5;
        let output = filter.process(input, 44100.0);

        // Should process without crashing and produce valid output
        assert!(output.is_finite());
        assert!(output.abs() <= 2.0);
    }

    #[test]
    fn test_notch_filter_processes() {
        let mut filter = Filter::notch(1000.0, 0.7);
        let input = 0.5;
        let output = filter.process(input, 44100.0);

        // Should process without crashing
        assert!(output.is_finite());
    }

    #[test]
    fn test_allpass_filter_processes() {
        let mut filter = Filter::all_pass(1000.0, 0.5);
        let input = 0.5;
        let output = filter.process(input, 44100.0);

        // Should process without crashing
        assert!(output.is_finite());
    }
}
