//! Spectral blur - smooth frequency spectrum over time

use super::*;
use rustfft::num_complex::Complex;

/// Spectral blur effect
///
/// Smears the frequency spectrum over time, creating a reverb-like effect
/// by averaging current and previous spectral frames. Creates ethereal, washed-out textures.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralBlur, WindowType};
/// let mut blur = SpectralBlur::new(2048, 512, WindowType::Hann, 44100.0);
/// blur.set_blur_amount(0.7);  // Strong blur
/// blur.set_feedback(0.8);      // Long decay
/// ```
#[derive(Clone)]
pub struct SpectralBlur {
    /// STFT processor
    stft: STFT,

    /// FFT size
    fft_size: usize,

    /// Sample rate
    _sample_rate: f32,

    /// Blur amount (0.0 = no blur, 1.0 = full blur)
    blur_amount: f32,

    /// Feedback amount (0.0 = no persistence, 1.0 = infinite)
    feedback: f32,

    /// Wet/dry mix (0.0 = dry, 1.0 = wet)
    mix: f32,

    /// Previous spectrum frame for blurring
    prev_spectrum: Vec<Complex<f32>>,

    /// Effect enabled flag
    enabled: bool,
}

impl SpectralBlur {
    /// Create a new spectral blur effect
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `window_type` - Window function type
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralBlur, WindowType};
    /// let blur = SpectralBlur::new(2048, 512, WindowType::Hann, 44100.0);
    /// ```
    pub fn new(fft_size: usize, hop_size: usize, window_type: WindowType, sample_rate: f32) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(hop_size <= fft_size, "Hop size must be <= FFT size");
        assert!(sample_rate > 0.0, "Sample rate must be positive");

        let stft = STFT::new(fft_size, hop_size, window_type);

        Self {
            stft,
            fft_size,
            _sample_rate: sample_rate,
            blur_amount: 0.5,
            feedback: 0.7,
            mix: 1.0,
            prev_spectrum: vec![Complex::new(0.0, 0.0); fft_size],
            enabled: true,
        }
    }

    /// Set blur amount
    ///
    /// # Arguments
    /// * `blur_amount` - Blur amount (0.0 = no blur, 1.0 = full blur)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralBlur, WindowType};
    /// let mut blur = SpectralBlur::new(2048, 512, WindowType::Hann, 44100.0);
    /// blur.set_blur_amount(0.8);  // Heavy blur
    /// ```
    pub fn set_blur_amount(&mut self, blur_amount: f32) {
        self.blur_amount = blur_amount.clamp(0.0, 1.0);
    }

    /// Set feedback amount
    ///
    /// Controls how long the blur persists over time.
    ///
    /// # Arguments
    /// * `feedback` - Feedback amount (0.0 = no persistence, 1.0 = infinite)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralBlur, WindowType};
    /// let mut blur = SpectralBlur::new(2048, 512, WindowType::Hann, 44100.0);
    /// blur.set_feedback(0.9);  // Long persistence
    /// ```
    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback.clamp(0.0, 0.99);
    }

    /// Set wet/dry mix
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = dry, 1.0 = wet)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralBlur, WindowType};
    /// let mut blur = SpectralBlur::new(2048, 512, WindowType::Hann, 44100.0);
    /// blur.set_mix(0.7);  // 70% wet
    /// ```
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Get current blur amount
    pub fn blur_amount(&self) -> f32 {
        self.blur_amount
    }

    /// Get current feedback amount
    pub fn feedback(&self) -> f32 {
        self.feedback
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.mix
    }

    /// Process audio through the spectral blur
    ///
    /// # Arguments
    /// * `output` - Output buffer (will be filled with processed audio)
    /// * `input` - Input audio buffer
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralBlur, WindowType};
    /// let mut blur = SpectralBlur::new(2048, 512, WindowType::Hann, 44100.0);
    /// blur.set_blur_amount(0.6);
    ///
    /// let input = vec![0.0; 512];
    /// let mut output = vec![0.0; 512];
    /// blur.process(&mut output, &input);
    /// ```
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        if !self.enabled {
            return;
        }

        let blur_amount = self.blur_amount;
        let feedback = self.feedback;
        let mix = self.mix;
        let prev_spectrum = &mut self.prev_spectrum;

        self.stft.process(output, |spectrum| {
            Self::apply_blur_static(spectrum, prev_spectrum, blur_amount, feedback, mix);
        });
    }

    /// Apply blur to spectrum (static version for closure)
    #[inline]
    fn apply_blur_static(
        spectrum: &mut [Complex<f32>],
        prev_spectrum: &mut [Complex<f32>],
        blur_amount: f32,
        feedback: f32,
        mix: f32,
    ) {
        let len = spectrum.len();

        // Store original spectrum for dry/wet mixing
        let mut dry_spectrum = vec![Complex::new(0.0, 0.0); len];
        dry_spectrum.copy_from_slice(spectrum);

        // Blend current spectrum with previous spectrum
        for i in 0..len {
            // Blur: blend current with previous
            let blurred = Complex::new(
                spectrum[i].re * (1.0 - blur_amount) + prev_spectrum[i].re * blur_amount,
                spectrum[i].im * (1.0 - blur_amount) + prev_spectrum[i].im * blur_amount,
            );

            // Update spectrum with blurred version
            spectrum[i] = blurred;

            // Update previous spectrum with feedback
            prev_spectrum[i] = Complex::new(
                blurred.re * feedback,
                blurred.im * feedback,
            );
        }

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

    /// Reset the spectral blur state
    pub fn reset(&mut self) {
        self.stft.reset();
        self.prev_spectrum.fill(Complex::new(0.0, 0.0));
    }

    /// Clear the blur history
    pub fn clear(&mut self) {
        self.prev_spectrum.fill(Complex::new(0.0, 0.0));
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

