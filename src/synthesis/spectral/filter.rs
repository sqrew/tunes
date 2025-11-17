//! Spectral filter - frequency-domain filtering

use super::*;
use rustfft::num_complex::Complex;

/// Spectral filter for frequency-domain filtering
///
/// Applies filtering by manipulating frequency bin magnitudes directly in the spectral domain.
/// This allows for precise control over the frequency response and can create effects not
/// possible with traditional time-domain filters.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralFilter, FilterType, WindowType};
/// let mut filter = SpectralFilter::new(2048, 512, WindowType::Hann, 44100.0);
/// filter.set_filter_type(FilterType::LowPass);
/// filter.set_cutoff(1000.0);  // 1kHz cutoff
/// filter.set_resonance(2.0);   // Moderate resonance
/// ```
#[derive(Clone)]
pub struct SpectralFilter {
    /// STFT processor
    stft: STFT,

    /// FFT size
    fft_size: usize,

    /// Sample rate
    sample_rate: f32,

    /// Filter type
    filter_type: FilterType,

    /// Cutoff frequency in Hz
    cutoff: f32,

    /// Resonance/Q factor (0.1 - 20.0)
    resonance: f32,

    /// Wet/dry mix (0.0 = dry, 1.0 = wet)
    mix: f32,

    /// Pre-computed filter gains for each bin
    bin_gains: Vec<f32>,

    /// Effect enabled flag
    enabled: bool,
}

impl SpectralFilter {
    /// Create a new spectral filter
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `window_type` - Window function type
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralFilter, WindowType};
    /// let filter = SpectralFilter::new(2048, 512, WindowType::Hann, 44100.0);
    /// ```
    pub fn new(
        fft_size: usize,
        hop_size: usize,
        window_type: WindowType,
        sample_rate: f32,
    ) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(hop_size <= fft_size, "Hop size must be <= FFT size");
        assert!(sample_rate > 0.0, "Sample rate must be positive");

        let stft = STFT::new(fft_size, hop_size, window_type);

        let mut filter = Self {
            stft,
            fft_size,
            sample_rate,
            filter_type: FilterType::LowPass,
            cutoff: 1000.0,
            resonance: 1.0,
            mix: 1.0,
            bin_gains: vec![1.0; fft_size],
            enabled: true,
        };

        filter.update_filter_curve();
        filter
    }

    /// Set filter type
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralFilter, FilterType, WindowType};
    /// let mut filter = SpectralFilter::new(2048, 512, WindowType::Hann, 44100.0);
    /// filter.set_filter_type(FilterType::HighPass);
    /// ```
    pub fn set_filter_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
        self.update_filter_curve();
    }

    /// Set cutoff frequency in Hz
    ///
    /// # Arguments
    /// * `cutoff` - Cutoff frequency in Hz (20.0 - sample_rate/2)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralFilter, WindowType};
    /// let mut filter = SpectralFilter::new(2048, 512, WindowType::Hann, 44100.0);
    /// filter.set_cutoff(2000.0);  // 2kHz cutoff
    /// ```
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff.clamp(20.0, self.sample_rate / 2.0);
        self.update_filter_curve();
    }

    /// Set resonance/Q factor
    ///
    /// # Arguments
    /// * `resonance` - Q factor (0.1 - 20.0, typical values 0.5 - 5.0)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralFilter, WindowType};
    /// let mut filter = SpectralFilter::new(2048, 512, WindowType::Hann, 44100.0);
    /// filter.set_resonance(3.0);  // Moderate resonance
    /// ```
    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance.clamp(0.1, 20.0);
        self.update_filter_curve();
    }

    /// Set wet/dry mix
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = dry, 1.0 = wet)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralFilter, WindowType};
    /// let mut filter = SpectralFilter::new(2048, 512, WindowType::Hann, 44100.0);
    /// filter.set_mix(0.7);  // 70% wet
    /// ```
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Update the filter curve based on current parameters
    fn update_filter_curve(&mut self) {
        let nyquist = self.sample_rate / 2.0;
        let cutoff_normalized = self.cutoff / nyquist;
        let cutoff_bin = cutoff_normalized * (self.fft_size / 2) as f32;

        for i in 0..self.fft_size {
            let freq_bin = i as f32;
            let gain = self.calculate_bin_gain(freq_bin, cutoff_bin);
            self.bin_gains[i] = gain;
        }
    }

    /// Calculate gain for a specific frequency bin
    #[inline]
    fn calculate_bin_gain(&self, bin: f32, cutoff_bin: f32) -> f32 {
        let distance = (bin - cutoff_bin).abs();
        let bandwidth = cutoff_bin / self.resonance.max(0.1);

        match self.filter_type {
            FilterType::LowPass => {
                if bin <= cutoff_bin {
                    1.0
                } else {
                    // Smooth rolloff based on resonance
                    let slope = distance / bandwidth.max(1.0);
                    (-slope).exp()
                }
            }
            FilterType::HighPass => {
                if bin >= cutoff_bin {
                    1.0
                } else {
                    let slope = distance / bandwidth.max(1.0);
                    (-slope).exp()
                }
            }
            FilterType::BandPass => {
                let slope = distance / bandwidth.max(1.0);
                (-slope).exp()
            }
            FilterType::BandStop => {
                let slope = distance / bandwidth.max(1.0);
                1.0 - (-slope * 2.0).exp()
            }
            FilterType::Notch => {
                // Sharp notch
                let slope = distance / (bandwidth.max(1.0) * 0.5);
                1.0 - (-slope * slope).exp()
            }
        }
    }

    /// Process audio through the spectral filter
    ///
    /// # Arguments
    /// * `output` - Output buffer (will be filled with processed audio)
    /// * `input` - Input audio buffer
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralFilter, WindowType};
    /// let mut filter = SpectralFilter::new(2048, 512, WindowType::Hann, 44100.0);
    /// filter.set_cutoff(1000.0);
    ///
    /// let input = vec![0.0; 512];
    /// let mut output = vec![0.0; 512];
    /// filter.process(&mut output, &input);
    /// ```
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        if !self.enabled {
            return;
        }

        let bin_gains = &self.bin_gains;
        let mix = self.mix;

        self.stft.process(output, |spectrum| {
            Self::apply_filter_static(spectrum, bin_gains, mix);
        });
    }

    /// Apply filtering to spectrum (static version for closure)
    #[inline]
    fn apply_filter_static(spectrum: &mut [Complex<f32>], bin_gains: &[f32], mix: f32) {
        // Apply gain to each bin with SIMD where possible
        let len = spectrum.len();

        for i in 0..len {
            let gain = bin_gains[i];
            let wet_gain = mix * gain + (1.0 - mix);

            spectrum[i].re *= wet_gain;
            spectrum[i].im *= wet_gain;
        }
    }

    /// Reset the spectral filter state
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

