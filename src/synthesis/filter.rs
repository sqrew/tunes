use std::f32::consts::PI;

/// Types of filters available
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    Notch,    // Notch/band-reject filter (removes frequencies at cutoff)
    AllPass,  // All-pass filter (affects phase but not amplitude)
    Moog,     // Moog ladder filter (classic analog sound with saturation)
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

    // Moog ladder filter state (4 stages)
    moog_stage: [f32; 4],
    moog_stage_tanh: [f32; 4],
    moog_delay: [f32; 4],
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
            moog_stage: [0.0; 4],
            moog_stage_tanh: [0.0; 4],
            moog_delay: [0.0; 4],
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

    /// Create a Moog ladder filter (classic analog sound)
    pub fn moog(cutoff: f32, resonance: f32) -> Self {
        Self::new(FilterType::Moog, cutoff, resonance)
    }

    /// Create a bypass filter (no filtering)
    pub fn none() -> Self {
        Self::new(FilterType::None, 20000.0, 0.0)
    }

    /// Process a single sample through the filter
    #[inline]
    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        if self.filter_type == FilterType::None {
            return input;
        }

        // Use Moog ladder filter algorithm for Moog type
        if self.filter_type == FilterType::Moog {
            return self.process_moog(input, sample_rate);
        }

        // Smooth parameter changes to avoid zipper noise (simple one-pole smoothing)
        // Rewritten to use FMA operations
        // Reduced smoothing for better modulation response
        const SMOOTHING: f32 = 0.95;
        const INV_SMOOTHING: f32 = 1.0 - SMOOTHING;
        self.smooth_cutoff = self.smooth_cutoff.mul_add(SMOOTHING, self.cutoff * INV_SMOOTHING);
        self.smooth_resonance = self.smooth_resonance.mul_add(SMOOTHING, self.resonance * INV_SMOOTHING);

        // Calculate filter coefficients using smoothed parameters
        let f = 2.0 * (PI * self.smooth_cutoff / sample_rate).sin();
        let q = 1.0 - self.smooth_resonance;

        // First stage: State-variable filter algorithm
        self.low = self.low.mul_add(1.0, f * self.band);
        self.high = input - self.low - q * self.band;
        self.band = self.band.mul_add(1.0, f * self.high);
        self.notch = self.high + self.low;

        // Flush denormals to zero (faster than checking is_finite)
        // This adds a tiny DC offset but prevents denormal slowdown
        self.low = if self.low.abs() < 1e-15 { 0.0 } else { self.low };
        self.band = if self.band.abs() < 1e-15 { 0.0 } else { self.band };

        // Stability check - prevent infinity by clamping instead of resetting
        // Resetting causes audible glitches, so we clamp the state instead
        if self.low.abs() > 100.0 {
            self.low = self.low.clamp(-100.0, 100.0);
            self.band = self.band.clamp(-100.0, 100.0);
            self.high = self.high.clamp(-100.0, 100.0);
        }

        // Get output from first stage
        let stage1_output = match self.filter_type {
            FilterType::LowPass => self.low,
            FilterType::HighPass => self.high,
            FilterType::BandPass => self.band,
            FilterType::Notch => self.notch,
            FilterType::AllPass => self.notch - self.band,
            FilterType::Moog => unreachable!(), // Handled separately above
            FilterType::None => input,
        };

        // For 24dB/octave, run a second stage in series
        let output = match self.slope {
            FilterSlope::Pole12dB => stage1_output,
            FilterSlope::Pole24dB => {
                // Second stage processing (same algorithm, different state)
                self.low2 = self.low2.mul_add(1.0, f * self.band2);
                self.high2 = stage1_output - self.low2 - q * self.band2;
                self.band2 = self.band2.mul_add(1.0, f * self.high2);
                self.notch2 = self.high2 + self.low2;

                // Flush denormals to zero
                self.low2 = if self.low2.abs() < 1e-15 { 0.0 } else { self.low2 };
                self.band2 = if self.band2.abs() < 1e-15 { 0.0 } else { self.band2 };

                // Stability check for second stage - clamp instead of resetting
                if self.low2.abs() > 100.0 {
                    self.low2 = self.low2.clamp(-100.0, 100.0);
                    self.band2 = self.band2.clamp(-100.0, 100.0);
                    self.high2 = self.high2.clamp(-100.0, 100.0);
                }

                // Get output from second stage
                match self.filter_type {
                    FilterType::LowPass => self.low2,
                    FilterType::HighPass => self.high2,
                    FilterType::BandPass => self.band2,
                    FilterType::Notch => self.notch2,
                    FilterType::AllPass => self.notch2 - self.band2,
                    FilterType::Moog => unreachable!(), // Handled separately above
                    FilterType::None => stage1_output,
                }
            }
        };

        // Clamp output to prevent explosion
        output.clamp(-2.0, 2.0)
    }

    /// Moog ladder filter processing (4-pole lowpass with resonance and saturation)
    ///
    /// This is a digital implementation of the legendary Moog ladder filter.
    /// It features:
    /// - 24dB/octave slope (4-pole)
    /// - Self-oscillation at high resonance
    /// - Soft saturation for analog warmth
    /// - Classic "fat" Moog sound
    #[inline]
    fn process_moog(&mut self, input: f32, sample_rate: f32) -> f32 {
        // Smooth parameter changes - reduced for better modulation response
        const SMOOTHING: f32 = 0.95;
        const INV_SMOOTHING: f32 = 1.0 - SMOOTHING;
        self.smooth_cutoff = self.smooth_cutoff.mul_add(SMOOTHING, self.cutoff * INV_SMOOTHING);
        self.smooth_resonance = self.smooth_resonance.mul_add(SMOOTHING, self.resonance * INV_SMOOTHING);

        // Calculate filter coefficient
        // Moog uses a different tuning than SVF
        let fc = self.smooth_cutoff / sample_rate;
        let f = fc.mul_add(1.16, 0.0); // Frequency warping for better tracking

        // Nonlinear feedback coefficient for resonance
        // At resonance = 1.0, filter self-oscillates
        let k = self.smooth_resonance.mul_add(3.96, 0.0);

        // Input with resonance feedback
        // The tanh provides soft saturation (analog modeling)
        let input_compensated = input - k * self.moog_delay[3];
        let input_tanh = input_compensated.tanh();

        // Process through 4 cascaded one-pole filters (the "ladder")
        // Each stage adds saturation for analog warmth
        for i in 0..4 {
            let stage_input = if i == 0 {
                input_tanh
            } else {
                self.moog_stage_tanh[i - 1]
            };

            // One-pole lowpass: y[n] = y[n-1] + f * (x[n] - y[n-1])
            self.moog_stage[i] = self.moog_delay[i].mul_add(1.0 - f, stage_input * f);

            // Soft saturation using tanh
            self.moog_stage_tanh[i] = self.moog_stage[i].tanh();

            // Store for next sample
            self.moog_delay[i] = self.moog_stage[i];
        }

        // Output is from the 4th stage
        // Apply compensation for resonance boost
        let output = self.moog_stage_tanh[3];

        // Clamp output
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
        self.moog_stage = [0.0; 4];
        self.moog_stage_tanh = [0.0; 4];
        self.moog_delay = [0.0; 4];
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

    #[test]
    fn test_moog_filter_creation() {
        let moog = Filter::moog(1000.0, 0.7);
        assert_eq!(moog.filter_type, FilterType::Moog);
        assert_eq!(moog.cutoff, 1000.0);
        assert_eq!(moog.resonance, 0.7);
    }

    #[test]
    fn test_moog_filter_processes() {
        let mut filter = Filter::moog(1000.0, 0.5);
        let input = 0.5;
        let output = filter.process(input, 44100.0);

        // Should process without crashing and produce valid output
        assert!(output.is_finite());
        assert!(output.abs() <= 2.0);
    }

    #[test]
    fn test_moog_high_resonance() {
        // Test Moog filter with high resonance (should self-oscillate)
        let mut filter = Filter::moog(440.0, 0.95);

        // Process multiple samples to let oscillation build up
        for _ in 0..100 {
            let output = filter.process(0.0, 44100.0);
            assert!(output.is_finite());
            assert!(output.abs() <= 2.0);
        }
    }

    #[test]
    fn test_moog_sweep() {
        // Test Moog filter with cutoff sweep
        let mut filter = Filter::moog(100.0, 0.7);

        for i in 0..100 {
            filter.cutoff = 100.0 + i as f32 * 100.0;
            let output = filter.process(0.5, 44100.0);
            assert!(output.is_finite());
        }
    }
}
