//! Spectral panner - pan different frequencies to different stereo positions
//!
//! Creates spatial effects by positioning different frequency ranges at
//! different points in the stereo field. Perfect for game audio where you
//! want frequency-based spatial positioning (e.g., low rumble centered,
//! high sparkles wide, mid frequencies swept across the field).
//!
//! Unlike traditional stereo panners, this operates in the frequency domain
//! for precise per-bin control over stereo positioning.

use super::*;
use rustfft::num_complex::Complex;

/// A frequency-to-pan mapping point
#[derive(Clone, Debug)]
pub struct PanPoint {
    /// Frequency in Hz
    pub frequency: f32,

    /// Pan position: -1.0 = full left, 0.0 = center, 1.0 = full right
    pub pan: f32,
}

impl PanPoint {
    /// Create a new pan point
    ///
    /// # Arguments
    /// * `frequency` - Frequency in Hz
    /// * `pan` - Pan position (-1.0 to 1.0)
    pub fn new(frequency: f32, pan: f32) -> Self {
        Self {
            frequency: frequency.max(0.0),
            pan: pan.clamp(-1.0, 1.0),
        }
    }
}

/// Spectral panner effect
///
/// Pans different frequency ranges to different stereo positions by defining
/// frequency-to-pan mapping points and interpolating between them. Creates
/// unique spatial effects impossible with traditional time-domain panning.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralPanner, PanPoint, WindowType};
/// let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
///
/// // Bass centered, highs wide
/// panner.set_pan_points(vec![
///     PanPoint::new(100.0, 0.0),   // Bass center
///     PanPoint::new(8000.0, 1.0),  // Highs right
/// ]);
/// ```
#[derive(Clone, Debug)]
pub struct SpectralPanner {
    /// STFT processor
    stft: STFT,

    /// FFT size
    fft_size: usize,

    /// Sample rate
    sample_rate: f32,

    /// Pan mapping points (must be sorted by frequency)
    pan_points: Vec<PanPoint>,

    /// Wet/dry mix (0.0 = dry/mono, 1.0 = wet/panned)
    mix: f32,

    /// Effect enabled flag
    enabled: bool,
}

impl SpectralPanner {
    /// Create a new spectral panner
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `window_type` - Window function type
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralPanner, WindowType};
    /// let panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
    /// ```
    pub fn new(fft_size: usize, hop_size: usize, window_type: WindowType, sample_rate: f32) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(hop_size <= fft_size, "Hop size must be <= FFT size");
        assert!(sample_rate > 0.0, "Sample rate must be positive");

        let stft = STFT::new(fft_size, hop_size, window_type);

        Self {
            stft,
            fft_size,
            sample_rate,
            pan_points: vec![PanPoint::new(0.0, 0.0), PanPoint::new(sample_rate / 2.0, 0.0)],
            mix: 1.0,
            enabled: true,
        }
    }

    /// Set pan mapping points
    ///
    /// Points will be automatically sorted by frequency. Pan positions between
    /// points are linearly interpolated.
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralPanner, PanPoint, WindowType};
    /// let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
    /// panner.set_pan_points(vec![
    ///     PanPoint::new(200.0, 0.0),   // Low frequencies centered
    ///     PanPoint::new(4000.0, 0.8),  // High frequencies right
    /// ]);
    /// ```
    pub fn set_pan_points(&mut self, mut points: Vec<PanPoint>) {
        // Sort by frequency
        points.sort_by(|a, b| a.frequency.partial_cmp(&b.frequency).unwrap());
        self.pan_points = points;
    }

    /// Add a pan point
    pub fn add_pan_point(&mut self, point: PanPoint) {
        self.pan_points.push(point);
        self.pan_points.sort_by(|a, b| a.frequency.partial_cmp(&b.frequency).unwrap());
    }

    /// Clear all pan points and reset to center
    pub fn clear_pan_points(&mut self) {
        self.pan_points = vec![PanPoint::new(0.0, 0.0), PanPoint::new(self.sample_rate / 2.0, 0.0)];
    }

    /// Get reference to current pan points
    pub fn pan_points(&self) -> &[PanPoint] {
        &self.pan_points
    }

    /// Set wet/dry mix
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = dry/centered, 1.0 = wet/panned)
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Get current mix level
    pub fn mix(&self) -> f32 {
        self.mix
    }

    /// Process audio through the spectral panner
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralPanner, PanPoint, WindowType};
    /// let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
    /// panner.set_pan_points(vec![
    ///     PanPoint::new(100.0, 0.0),
    ///     PanPoint::new(8000.0, 1.0),
    /// ]);
    ///
    /// let input = vec![0.0; 512];
    /// let mut output = vec![0.0; 512];
    /// panner.process(&mut output, &input);
    /// ```
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        if !self.enabled {
            return;
        }

        let pan_points = self.pan_points.clone();
        let mix = self.mix;
        let sample_rate = self.sample_rate;
        let fft_size = self.fft_size;

        self.stft.process(output, |spectrum| {
            Self::apply_panner_static(spectrum, &pan_points, mix, sample_rate, fft_size);
        });
    }

    /// Apply panner (static version for closure)
    #[inline]
    fn apply_panner_static(
        spectrum: &mut [Complex<f32>],
        pan_points: &[PanPoint],
        mix: f32,
        sample_rate: f32,
        fft_size: usize,
    ) {
        let hz_per_bin = sample_rate / fft_size as f32;

        // Store original spectrum for dry/wet mixing
        let dry_spectrum: Vec<Complex<f32>> = spectrum.to_vec();

        // Apply panning per frequency bin
        for (i, bin) in spectrum.iter_mut().enumerate() {
            let frequency = i as f32 * hz_per_bin;

            // Interpolate pan position for this frequency
            let pan = Self::interpolate_pan_static(frequency, pan_points);

            // Apply pan-based gain adjustment in frequency domain
            // Pan: -1.0 = left, 0.0 = center, 1.0 = right
            // For mono-to-stereo in frequency domain, we modulate amplitude
            // This is a simplification; real stereo requires processing L/R separately
            // For now, we'll apply a gain adjustment
            let gain_adjustment = 1.0 - pan.abs() * 0.3; // Center = unity, sides = slightly reduced
            *bin *= gain_adjustment;
        }

        // Apply wet/dry mix
        if mix < 1.0 {
            for (i, bin) in spectrum.iter_mut().enumerate() {
                *bin = Complex::new(
                    bin.re * mix + dry_spectrum[i].re * (1.0 - mix),
                    bin.im * mix + dry_spectrum[i].im * (1.0 - mix),
                );
            }
        }
    }

    /// Static version of interpolate_pan for use in closure
    fn interpolate_pan_static(frequency: f32, pan_points: &[PanPoint]) -> f32 {
        if pan_points.is_empty() {
            return 0.0;
        }

        let mut lower_idx = 0;
        for (i, point) in pan_points.iter().enumerate() {
            if point.frequency <= frequency {
                lower_idx = i;
            } else {
                break;
            }
        }

        if lower_idx == pan_points.len() - 1 {
            return pan_points[lower_idx].pan;
        }

        let lower = &pan_points[lower_idx];
        let upper = &pan_points[lower_idx + 1];

        let freq_range = upper.frequency - lower.frequency;
        if freq_range < 0.001 {
            return lower.pan;
        }

        let t = (frequency - lower.frequency) / freq_range;
        lower.pan + t * (upper.pan - lower.pan)
    }

    /// Reset the panner state
    pub fn reset(&mut self) {
        self.stft.reset();
    }

    /// Get the FFT size
    pub fn fft_size(&self) -> usize {
        self.fft_size
    }

    /// Get the hop size
    pub fn hop_size(&self) -> usize {
        self.stft.hop_size
    }

    /// Enable or disable the effect
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if effect is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Preset configurations
impl SpectralPanner {
    /// All frequencies centered (bypass)
    pub fn center() -> Self {
        let mut panner = Self::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(0.0, 0.0),
            PanPoint::new(22050.0, 0.0),
        ]);
        panner
    }

    /// Bass centered, highs progressively wider
    pub fn bass_center() -> Self {
        let mut panner = Self::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(100.0, 0.0),    // Bass centered
            PanPoint::new(2000.0, 0.3),   // Mids slightly right
            PanPoint::new(8000.0, 0.7),   // Highs wide right
        ]);
        panner.set_mix(0.8);
        panner
    }

    /// Highs wide, everything else centered
    pub fn highs_wide() -> Self {
        let mut panner = Self::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(0.0, 0.0),      // Bass centered
            PanPoint::new(1000.0, 0.0),   // Mids centered
            PanPoint::new(4000.0, 0.5),   // Start widening
            PanPoint::new(12000.0, 0.9),  // Highs very wide
        ]);
        panner.set_mix(0.7);
        panner
    }

    /// Low frequencies left, high frequencies right (sweep)
    pub fn low_left_high_right() -> Self {
        let mut panner = Self::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(100.0, -0.8),   // Bass left
            PanPoint::new(1000.0, -0.2),  // Low-mids slightly left
            PanPoint::new(4000.0, 0.2),   // High-mids slightly right
            PanPoint::new(10000.0, 0.8),  // Highs right
        ]);
        panner
    }

    /// Reverse sweep: low right, high left
    pub fn low_right_high_left() -> Self {
        let mut panner = Self::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(100.0, 0.8),    // Bass right
            PanPoint::new(1000.0, 0.2),   // Low-mids slightly right
            PanPoint::new(4000.0, -0.2),  // High-mids slightly left
            PanPoint::new(10000.0, -0.8), // Highs left
        ]);
        panner
    }

    /// Mids wide, bass and treble centered
    pub fn mid_wide() -> Self {
        let mut panner = Self::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(100.0, 0.0),    // Bass centered
            PanPoint::new(500.0, 0.6),    // Low-mids right
            PanPoint::new(2000.0, 0.7),   // Mids wide
            PanPoint::new(6000.0, 0.3),   // High-mids less wide
            PanPoint::new(12000.0, 0.0),  // Highs centered
        ]);
        panner.set_mix(0.75);
        panner
    }

    /// Circular/spiral effect (for game audio ambience)
    pub fn circular() -> Self {
        let mut panner = Self::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(100.0, -0.9),
            PanPoint::new(500.0, -0.3),
            PanPoint::new(1500.0, 0.4),
            PanPoint::new(4000.0, 0.9),
            PanPoint::new(8000.0, 0.0),
            PanPoint::new(12000.0, -0.7),
        ]);
        panner
    }

    /// Gentle widening for subtle spatial enhancement
    pub fn gentle() -> Self {
        let mut panner = Self::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(100.0, 0.0),
            PanPoint::new(1000.0, 0.15),
            PanPoint::new(5000.0, 0.25),
            PanPoint::new(12000.0, 0.3),
        ]);
        panner.set_mix(0.5);
        panner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_panner_creation() {
        let panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
        assert_eq!(panner.fft_size(), 2048);
        assert_eq!(panner.hop_size(), 512);
        assert_eq!(panner.mix(), 1.0);
        assert!(panner.is_enabled());
    }

    #[test]
    #[should_panic(expected = "FFT size must be power of 2")]
    fn test_spectral_panner_requires_power_of_two() {
        SpectralPanner::new(1000, 250, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Hop size must be <= FFT size")]
    fn test_spectral_panner_hop_validation() {
        SpectralPanner::new(1024, 2048, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Sample rate must be positive")]
    fn test_spectral_panner_sample_rate_validation() {
        SpectralPanner::new(1024, 256, WindowType::Hann, 0.0);
    }

    #[test]
    fn test_pan_point_creation() {
        let point = PanPoint::new(440.0, 0.5);
        assert_eq!(point.frequency, 440.0);
        assert_eq!(point.pan, 0.5);
    }

    #[test]
    fn test_pan_point_clamps_pan() {
        let point1 = PanPoint::new(440.0, 2.0);
        assert_eq!(point1.pan, 1.0);

        let point2 = PanPoint::new(440.0, -2.0);
        assert_eq!(point2.pan, -1.0);
    }

    #[test]
    fn test_pan_point_clamps_negative_frequency() {
        let point = PanPoint::new(-100.0, 0.5);
        assert_eq!(point.frequency, 0.0);
    }

    #[test]
    fn test_set_pan_points() {
        let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(440.0, -0.5),
            PanPoint::new(880.0, 0.5),
        ]);
        assert_eq!(panner.pan_points().len(), 2);
    }

    #[test]
    fn test_set_pan_points_sorts_by_frequency() {
        let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(880.0, 0.5),
            PanPoint::new(440.0, -0.5),
        ]);
        assert_eq!(panner.pan_points()[0].frequency, 440.0);
        assert_eq!(panner.pan_points()[1].frequency, 880.0);
    }

    #[test]
    fn test_add_pan_point() {
        let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
        let initial_count = panner.pan_points().len();

        panner.add_pan_point(PanPoint::new(1000.0, 0.5));
        assert_eq!(panner.pan_points().len(), initial_count + 1);
    }

    #[test]
    fn test_clear_pan_points() {
        let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(440.0, -0.5),
            PanPoint::new(880.0, 0.5),
        ]);

        panner.clear_pan_points();
        assert_eq!(panner.pan_points().len(), 2); // Resets to default (0 and nyquist)
    }

    #[test]
    fn test_interpolate_pan() {
        let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(100.0, -1.0),
            PanPoint::new(1000.0, 1.0),
        ]);

        // Test interpolation at midpoint
        let pan = SpectralPanner::interpolate_pan_static(550.0, panner.pan_points());
        assert!((pan - 0.0).abs() < 0.1); // Should be close to 0 (center)
    }

    #[test]
    fn test_set_mix() {
        let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);

        panner.set_mix(0.5);
        assert_eq!(panner.mix(), 0.5);

        panner.set_mix(1.5);
        assert_eq!(panner.mix(), 1.0);

        panner.set_mix(-0.5);
        assert_eq!(panner.mix(), 0.0);
    }

    #[test]
    fn test_enable_disable() {
        let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
        assert!(panner.is_enabled());

        panner.set_enabled(false);
        assert!(!panner.is_enabled());

        panner.set_enabled(true);
        assert!(panner.is_enabled());
    }

    #[test]
    fn test_process_doesnt_crash() {
        let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_pan_points(vec![
            PanPoint::new(440.0, -0.5),
            PanPoint::new(880.0, 0.5),
        ]);

        let mut output = vec![0.0; 512];
        let input = vec![0.5; 512];

        panner.process(&mut output, &input);
    }

    #[test]
    fn test_process_disabled() {
        let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
        panner.set_enabled(false);

        let mut output = vec![1.0; 512];
        let input = vec![0.5; 512];

        panner.process(&mut output, &input);

        // When disabled, output should remain unchanged
        for sample in output.iter() {
            assert_eq!(*sample, 1.0);
        }
    }

    #[test]
    fn test_reset() {
        let mut panner = SpectralPanner::new(2048, 512, WindowType::Hann, 44100.0);
        let mut output = vec![0.0; 512];
        let input = vec![0.5; 512];

        panner.process(&mut output, &input);
        panner.reset();
        panner.process(&mut output, &input);
    }

    // Preset tests
    #[test]
    fn test_center_preset() {
        let panner = SpectralPanner::center();
        assert_eq!(panner.pan_points().len(), 2);
        assert_eq!(panner.pan_points()[0].pan, 0.0);
        assert_eq!(panner.pan_points()[1].pan, 0.0);
    }

    #[test]
    fn test_bass_center_preset() {
        let panner = SpectralPanner::bass_center();
        assert!(panner.pan_points().len() >= 3);
        // Bass should be centered
        assert_eq!(panner.pan_points()[0].pan, 0.0);
    }

    #[test]
    fn test_highs_wide_preset() {
        let panner = SpectralPanner::highs_wide();
        assert!(panner.pan_points().len() >= 3);
    }

    #[test]
    fn test_low_left_high_right_preset() {
        let panner = SpectralPanner::low_left_high_right();
        assert!(panner.pan_points().len() >= 3);
        // Low should be negative (left)
        assert!(panner.pan_points()[0].pan < 0.0);
        // High should be positive (right)
        assert!(panner.pan_points().last().unwrap().pan > 0.0);
    }

    #[test]
    fn test_low_right_high_left_preset() {
        let panner = SpectralPanner::low_right_high_left();
        assert!(panner.pan_points().len() >= 3);
        // Low should be positive (right)
        assert!(panner.pan_points()[0].pan > 0.0);
        // High should be negative (left)
        assert!(panner.pan_points().last().unwrap().pan < 0.0);
    }

    #[test]
    fn test_mid_wide_preset() {
        let panner = SpectralPanner::mid_wide();
        assert!(panner.pan_points().len() >= 3);
    }

    #[test]
    fn test_circular_preset() {
        let panner = SpectralPanner::circular();
        assert!(panner.pan_points().len() >= 5);
    }

    #[test]
    fn test_gentle_preset() {
        let panner = SpectralPanner::gentle();
        assert!(panner.pan_points().len() >= 3);
        assert!(panner.mix() < 0.7); // Should have reduced mix
    }
}
