//! Spectral inversion - flip the frequency spectrum upside down
//!
//! Inverts the spectrum by swapping low and high frequencies, creating
//! alien, backwards-sounding, or otherworldly timbres. Harmonic relationships
//! are destroyed in interesting ways.

use super::*;
use rustfft::num_complex::Complex;

/// Spectral inversion effect
///
/// Flips the frequency spectrum upside down - low frequencies become high,
/// high frequencies become low. Creates alien, backwards, or otherworldly timbres.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralInvert, WindowType};
/// let mut invert = SpectralInvert::new(2048, 512, WindowType::Hann, 44100.0);
/// invert.set_mix(0.8);  // 80% inverted
/// ```
#[derive(Clone, Debug)]
pub struct SpectralInvert {
    /// STFT processor
    stft: STFT,

    /// FFT size
    fft_size: usize,

    /// Sample rate
    _sample_rate: f32,

    /// Wet/dry mix (0.0 = dry, 1.0 = fully inverted)
    mix: f32,

    /// Effect enabled flag
    enabled: bool,
}

impl SpectralInvert {
    /// Create a new spectral invert effect
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
            _sample_rate: sample_rate,
            mix: 1.0,
            enabled: true,
        }
    }

    /// Set wet/dry mix
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.mix
    }

    /// Process audio through spectral inversion
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        if !self.enabled {
            return;
        }

        let mix = self.mix;

        self.stft.process(output, |spectrum| {
            Self::apply_invert_static(spectrum, mix);
        });
    }

    /// Apply spectral inversion (static version for closure)
    #[inline]
    fn apply_invert_static(spectrum: &mut [Complex<f32>], mix: f32) {
        let len = spectrum.len();

        // Store original spectrum for dry/wet mixing
        let mut dry_spectrum = vec![Complex::new(0.0, 0.0); len];
        dry_spectrum.copy_from_slice(spectrum);

        // Invert the spectrum by reversing bin order
        // Skip DC (bin 0) and Nyquist (last bin) to avoid issues
        let mut inverted = vec![Complex::new(0.0, 0.0); len];

        // Keep DC and Nyquist in place
        inverted[0] = spectrum[0];
        if len > 1 {
            inverted[len - 1] = spectrum[len - 1];
        }

        // Reverse the middle bins
        for i in 1..(len - 1) {
            inverted[i] = spectrum[len - 1 - i];
        }

        // Copy inverted spectrum back
        spectrum.copy_from_slice(&inverted);

        // Apply wet/dry mix
        if mix < 1.0 {
            for i in 0..len {
                spectrum[i] = Complex::new(
                    spectrum[i].re * mix + dry_spectrum[i].re * (1.0 - mix),
                    spectrum[i].im * mix + dry_spectrum[i].im * (1.0 - mix),
                );
            }
        }
    }

    /// Reset the spectral invert state
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
impl SpectralInvert {
    /// Subtle inversion for slight weirdness
    pub fn subtle() -> Self {
        let mut invert = Self::new(2048, 512, WindowType::Hann, 44100.0);
        invert.set_mix(0.3);
        invert
    }

    /// Full inversion for alien timbres
    pub fn full() -> Self {
        let mut invert = Self::new(2048, 512, WindowType::Hann, 44100.0);
        invert.set_mix(1.0);
        invert
    }

    /// Moderate blend for interesting hybrids
    pub fn moderate() -> Self {
        let mut invert = Self::new(2048, 512, WindowType::Hann, 44100.0);
        invert.set_mix(0.6);
        invert
    }
}
