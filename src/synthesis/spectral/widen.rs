//! Spectral widening - stereo widening in the frequency domain
//!
//! Creates stereo width by manipulating phase relationships between
//! left and right channels in the frequency domain. More precise and
//! artifact-free than time-domain stereo widening.

use super::*;
use rustfft::num_complex::Complex;

/// Spectral widening effect for stereo enhancement
///
/// Widens the stereo image by manipulating phase in the frequency domain.
/// Different frequency bands can be widened by different amounts.
///
/// **Note**: This effect processes stereo, but the current STFT is mono.
/// For now, it applies decorrelation to create stereo from mono.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralWiden, WindowType};
/// let mut widen = SpectralWiden::new(2048, 512, WindowType::Hann, 44100.0);
/// widen.set_width(1.5);  // 150% width
/// ```
#[derive(Clone, Debug)]
pub struct SpectralWiden {
    /// STFT processor
    stft: STFT,

    /// FFT size
    fft_size: usize,

    /// Sample rate
    sample_rate: f32,

    /// Width amount (0.0 = mono, 1.0 = normal, >1.0 = widened)
    width: f32,

    /// Low frequency cutoff (don't widen below this)
    low_cutoff: f32,

    /// High frequency cutoff (don't widen above this)
    high_cutoff: f32,

    /// Effect enabled flag
    enabled: bool,
}

impl SpectralWiden {
    /// Create a new spectral widen effect
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `window_type` - Window function type
    /// * `sample_rate` - Audio sample rate in Hz
    pub fn new(fft_size: usize, hop_size: usize, window_type: WindowType, sample_rate: f32) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(hop_size <= fft_size, "Hop size must be <= FFT size");
        assert!(sample_rate > 0.0, "Sample rate must be positive");

        let stft = STFT::new(fft_size, hop_size, window_type);

        Self {
            stft,
            fft_size,
            sample_rate,
            width: 1.5,
            low_cutoff: 200.0,
            high_cutoff: 16000.0,
            enabled: true,
        }
    }

    /// Set width amount
    ///
    /// # Arguments
    /// * `width` - Width multiplier (0.0 = mono, 1.0 = normal, 2.0 = very wide)
    pub fn set_width(&mut self, width: f32) {
        self.width = width.clamp(0.0, 3.0);
    }

    /// Set low frequency cutoff
    ///
    /// Frequencies below this won't be widened (keeps bass centered)
    pub fn set_low_cutoff(&mut self, frequency: f32) {
        self.low_cutoff = frequency.max(20.0);
    }

    /// Set high frequency cutoff
    ///
    /// Frequencies above this won't be widened
    pub fn set_high_cutoff(&mut self, frequency: f32) {
        self.high_cutoff = frequency.min(self.sample_rate * 0.45);
    }

    /// Get current width
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Get low cutoff
    pub fn low_cutoff(&self) -> f32 {
        self.low_cutoff
    }

    /// Get high cutoff
    pub fn high_cutoff(&self) -> f32 {
        self.high_cutoff
    }

    /// Process audio through spectral widening
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        if !self.enabled {
            return;
        }

        let width = self.width;
        let low_cutoff = self.low_cutoff;
        let high_cutoff = self.high_cutoff;
        let sample_rate = self.sample_rate;
        let fft_size = self.fft_size;

        self.stft.process(output, |spectrum| {
            Self::apply_widen_static(spectrum, width, low_cutoff, high_cutoff, sample_rate, fft_size);
        });
    }

    /// Apply spectral widening (static version for closure)
    #[inline]
    fn apply_widen_static(
        spectrum: &mut [Complex<f32>],
        width: f32,
        low_cutoff: f32,
        high_cutoff: f32,
        sample_rate: f32,
        fft_size: usize,
    ) {
        let len = spectrum.len();
        let hz_per_bin = sample_rate / fft_size as f32;

        // Apply phase modulation for stereo decorrelation
        // This creates width by shifting phase differently per frequency
        for (i, bin) in spectrum.iter_mut().enumerate().take(len) {
            let freq = i as f32 * hz_per_bin;

            // Only widen within cutoff range
            if freq >= low_cutoff && freq <= high_cutoff {
                let mag = bin.norm();
                let phase = bin.arg();

                // Frequency-dependent phase shift for decorrelation
                // Higher frequencies get more shift for natural stereo
                let freq_ratio = ((freq - low_cutoff) / (high_cutoff - low_cutoff)).clamp(0.0, 1.0);

                // Phase modulation amount based on width and frequency
                let phase_shift = (width - 1.0) * freq_ratio * 0.5;

                // Alternate phase shift direction by bin for decorrelation
                let direction = if i % 2 == 0 { 1.0 } else { -1.0 };
                let new_phase = phase + phase_shift * direction;

                *bin = Complex::from_polar(mag, new_phase);
            }
        }
    }

    /// Reset the spectral widen state
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
impl SpectralWiden {
    /// Subtle widening for mix enhancement
    pub fn subtle() -> Self {
        let mut widen = Self::new(2048, 512, WindowType::Hann, 44100.0);
        widen.set_width(1.2);
        widen.set_low_cutoff(250.0);
        widen
    }

    /// Moderate widening for stereo enhancement
    pub fn moderate() -> Self {
        let mut widen = Self::new(2048, 512, WindowType::Hann, 44100.0);
        widen.set_width(1.5);
        widen.set_low_cutoff(200.0);
        widen
    }

    /// Wide stereo for dramatic effect
    pub fn wide() -> Self {
        let mut widen = Self::new(2048, 512, WindowType::Hann, 44100.0);
        widen.set_width(2.0);
        widen.set_low_cutoff(150.0);
        widen
    }

    /// Ultra-wide for special effects
    pub fn ultra() -> Self {
        let mut widen = Self::new(2048, 512, WindowType::Hann, 44100.0);
        widen.set_width(2.5);
        widen.set_low_cutoff(200.0);
        widen
    }
}
