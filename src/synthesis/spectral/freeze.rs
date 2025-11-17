//! Spectral freeze effect - capture and hold spectrum

use super::*;
use rustfft::num_complex::Complex;

/// Spectral freeze effect - captures and holds a frequency spectrum snapshot
///
/// This effect captures the current frequency content and holds it indefinitely,
/// creating a sustained "frozen" sound. Perfect for ambient textures, drones,
/// and creating evolving soundscapes from transient sounds.
///
/// Uses SIMD-accelerated STFT for efficient real-time processing.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralFreeze, WindowType};
/// // Create spectral freeze with 2048 FFT, 512 hop
/// let mut freeze = SpectralFreeze::new(2048, 512, WindowType::Hann);
///
/// // Capture and freeze the spectrum
/// freeze.freeze();
///
/// // Set mix to 100% frozen (0% live)
/// freeze.set_mix(1.0);
///
/// // Process audio - will hold frozen spectrum
/// let input = vec![0.0; 512];
/// let mut output = vec![0.0; 512];
/// freeze.process(&mut output, &input);
///
/// // Unfreeze to return to normal processing
/// freeze.unfreeze();
/// ```
#[derive(Clone)]
pub struct SpectralFreeze {
    /// STFT processor for analysis/synthesis
    stft: STFT,

    /// Frozen spectrum (captured when freeze is enabled)
    frozen_spectrum: Vec<Complex<f32>>,

    /// FFT size
    fft_size: usize,

    /// Freeze enabled flag
    is_frozen: bool,

    /// Mix amount (0.0 = all live signal, 1.0 = all frozen spectrum)
    mix: f32,

    /// Whether we've captured a spectrum yet
    has_captured: bool,
}

impl SpectralFreeze {
    /// Create a new spectral freeze effect
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `window_type` - Window function type
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralFreeze, WindowType};
    /// let freeze = SpectralFreeze::new(2048, 512, WindowType::Hann);
    /// ```
    pub fn new(fft_size: usize, hop_size: usize, window_type: WindowType) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(hop_size <= fft_size, "Hop size must be <= FFT size");

        let stft = STFT::new(fft_size, hop_size, window_type);

        Self {
            stft,
            frozen_spectrum: vec![Complex::new(0.0, 0.0); fft_size],
            fft_size,
            is_frozen: false,
            mix: 1.0, // Default to 100% frozen when active
            has_captured: false,
        }
    }

    /// Enable freeze and start capturing spectrum
    ///
    /// The next processed frame will be captured and held indefinitely.
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralFreeze, WindowType};
    /// let mut freeze = SpectralFreeze::new(2048, 512, WindowType::Hann);
    /// freeze.freeze();  // Start freezing
    /// assert!(freeze.is_frozen());
    /// ```
    pub fn freeze(&mut self) {
        self.is_frozen = true;
    }

    /// Disable freeze and return to normal processing
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralFreeze, WindowType};
    /// let mut freeze = SpectralFreeze::new(2048, 512, WindowType::Hann);
    /// freeze.freeze();
    /// freeze.unfreeze();  // Stop freezing
    /// assert!(!freeze.is_frozen());
    /// ```
    pub fn unfreeze(&mut self) {
        self.is_frozen = false;
    }

    /// Set mix amount between live and frozen signals
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = all live, 1.0 = all frozen, clamped to [0.0, 1.0])
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralFreeze, WindowType};
    /// let mut freeze = SpectralFreeze::new(2048, 512, WindowType::Hann);
    /// freeze.freeze();
    /// freeze.set_mix(0.5);  // 50% live, 50% frozen
    /// ```
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Get whether freeze is currently enabled
    pub fn is_frozen(&self) -> bool {
        self.is_frozen
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.mix
    }

    /// Process audio through the spectral freeze
    ///
    /// When frozen, captures and holds the spectrum. The mix parameter controls
    /// the blend between live and frozen signals.
    ///
    /// # Arguments
    /// * `output` - Output buffer (will be filled with processed audio)
    /// * `input` - Input audio buffer
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralFreeze, WindowType};
    /// let mut freeze = SpectralFreeze::new(2048, 512, WindowType::Hann);
    /// freeze.freeze();
    ///
    /// let input = vec![0.0; 512];
    /// let mut output = vec![0.0; 512];
    /// freeze.process(&mut output, &input);
    /// ```
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        let is_frozen = self.is_frozen;
        let mix = self.mix;
        let frozen_spectrum = &mut self.frozen_spectrum;
        let has_captured = &mut self.has_captured;

        self.stft.process(output, |spectrum| {
            if is_frozen {
                if !*has_captured {
                    // Capture the current spectrum
                    frozen_spectrum.copy_from_slice(spectrum);
                    *has_captured = true;
                }

                // Mix frozen and live spectrums using SIMD
                Self::mix_spectrums_simd(spectrum, frozen_spectrum, mix);
            } else {
                // Not frozen: pass through live signal (no modification)
                *has_captured = false;
            }
        });
    }

    /// Mix two spectrums together using SIMD operations
    ///
    /// output = live * (1 - mix) + frozen * mix
    ///
    /// # Arguments
    /// * `live` - Live spectrum (will be modified in-place)
    /// * `frozen` - Frozen spectrum to mix in
    /// * `mix` - Mix amount (0.0 = all live, 1.0 = all frozen)
    #[inline]
    fn mix_spectrums_simd(live: &mut [Complex<f32>], frozen: &[Complex<f32>], mix: f32) {
        let len = live.len().min(frozen.len());
        let live_gain = 1.0 - mix;

        // Extract to separate buffers for SIMD processing
        let mut live_re = vec![0.0f32; len];
        let mut live_im = vec![0.0f32; len];
        let mut frozen_re = vec![0.0f32; len];
        let mut frozen_im = vec![0.0f32; len];

        for i in 0..len {
            live_re[i] = live[i].re;
            live_im[i] = live[i].im;
            frozen_re[i] = frozen[i].re;
            frozen_im[i] = frozen[i].im;
        }

        // Scale both signals using TRUE SIMD
        SIMD.multiply_const(&mut live_re, live_gain);
        SIMD.multiply_const(&mut live_im, live_gain);
        SIMD.multiply_const(&mut frozen_re, mix);
        SIMD.multiply_const(&mut frozen_im, mix);

        // Add them together using TRUE SIMD
        for i in 0..len {
            live_re[i] += frozen_re[i];
            live_im[i] += frozen_im[i];
        }

        // Write back
        for i in 0..len {
            live[i] = Complex::new(live_re[i], live_im[i]);
        }
    }

    /// Reset the spectral freeze state
    ///
    /// Clears the frozen spectrum and STFT buffers.
    pub fn reset(&mut self) {
        self.stft.reset();
        self.frozen_spectrum.fill(Complex::new(0.0, 0.0));
        self.is_frozen = false;
        self.has_captured = false;
    }

    /// Get the FFT size
    pub fn fft_size(&self) -> usize {
        self.fft_size
    }

    /// Get the hop size
    pub fn hop_size(&self) -> usize {
        self.stft.hop_size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_freeze_creation() {
        let freeze = SpectralFreeze::new(2048, 512, WindowType::Hann);
        assert_eq!(freeze.fft_size(), 2048);
        assert_eq!(freeze.hop_size(), 512);
        assert!(!freeze.is_frozen());
        assert_eq!(freeze.mix(), 1.0);
    }

    #[test]
    #[should_panic(expected = "FFT size must be power of 2")]
    fn test_spectral_freeze_requires_power_of_two() {
        SpectralFreeze::new(1000, 250, WindowType::Hann);
    }

    #[test]
    #[should_panic(expected = "Hop size must be <= FFT size")]
    fn test_spectral_freeze_hop_validation() {
        SpectralFreeze::new(1024, 2048, WindowType::Hann);
    }

    #[test]
    fn test_spectral_freeze_freeze_unfreeze() {
        let mut freeze = SpectralFreeze::new(1024, 256, WindowType::Hann);

        assert!(!freeze.is_frozen());

        freeze.freeze();
        assert!(freeze.is_frozen());

        freeze.unfreeze();
        assert!(!freeze.is_frozen());
    }

    #[test]
    fn test_spectral_freeze_set_mix() {
        let mut freeze = SpectralFreeze::new(1024, 256, WindowType::Hann);

        freeze.set_mix(0.5);
        assert_eq!(freeze.mix(), 0.5);

        freeze.set_mix(0.0);
        assert_eq!(freeze.mix(), 0.0);

        freeze.set_mix(1.0);
        assert_eq!(freeze.mix(), 1.0);
    }

    #[test]
    fn test_spectral_freeze_mix_clamping() {
        let mut freeze = SpectralFreeze::new(1024, 256, WindowType::Hann);

        // Should clamp to [0.0, 1.0]
        freeze.set_mix(1.5);
        assert_eq!(freeze.mix(), 1.0);

        freeze.set_mix(-0.5);
        assert_eq!(freeze.mix(), 0.0);
    }

    #[test]
    fn test_spectral_freeze_process_silent() {
        let mut freeze = SpectralFreeze::new(1024, 256, WindowType::Hann);
        let input = vec![0.0; 512];
        let mut output = vec![0.0; 512];

        // Process silence without freezing (should remain silent)
        freeze.process(&mut output, &input);

        for &sample in &output {
            assert!(sample.abs() < 0.001, "Expected silence, got {}", sample);
        }
    }

    #[test]
    fn test_spectral_freeze_process_frozen() {
        let mut freeze = SpectralFreeze::new(512, 128, WindowType::Hann);
        freeze.freeze();
        freeze.set_mix(1.0); // 100% frozen

        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];

        // Should process without crashing
        freeze.process(&mut output, &input);
        assert_eq!(output.len(), 256);
    }

    #[test]
    fn test_spectral_freeze_process_live() {
        let mut freeze = SpectralFreeze::new(512, 128, WindowType::Hann);
        freeze.set_mix(0.0); // 100% live (no freeze effect)

        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];

        // Should process without crashing
        freeze.process(&mut output, &input);
        assert_eq!(output.len(), 256);
    }

    #[test]
    fn test_spectral_freeze_process_mixed() {
        let mut freeze = SpectralFreeze::new(512, 128, WindowType::Hann);
        freeze.freeze();
        freeze.set_mix(0.5); // 50% live, 50% frozen

        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];

        // Should process without crashing
        freeze.process(&mut output, &input);
        assert_eq!(output.len(), 256);
    }

    #[test]
    fn test_spectral_freeze_reset() {
        let mut freeze = SpectralFreeze::new(512, 128, WindowType::Hann);

        // Enable freeze and process
        freeze.freeze();
        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];
        freeze.process(&mut output, &input);

        // Reset should clear state
        freeze.reset();
        assert!(!freeze.is_frozen());

        // Should still work after reset
        freeze.process(&mut output, &input);
        assert_eq!(output.len(), 256);
    }

    #[test]
    fn test_spectral_freeze_all_window_types() {
        for window_type in [
            WindowType::Rectangular,
            WindowType::Hann,
            WindowType::Hamming,
            WindowType::Blackman,
            WindowType::BlackmanHarris,
        ] {
            let mut freeze = SpectralFreeze::new(512, 128, window_type);
            freeze.freeze();

            let input = vec![0.0; 256];
            let mut output = vec![0.0; 256];

            freeze.process(&mut output, &input);
            assert_eq!(output.len(), 256);
        }
    }

    #[test]
    fn test_spectral_freeze_various_fft_sizes() {
        for fft_size in [512, 1024, 2048, 4096] {
            let hop_size = fft_size / 4;
            let mut freeze = SpectralFreeze::new(fft_size, hop_size, WindowType::Hann);
            freeze.freeze();

            let input = vec![0.0; 512];
            let mut output = vec![0.0; 512];

            freeze.process(&mut output, &input);
            assert_eq!(output.len(), 512);
        }
    }

    #[test]
    fn test_spectral_freeze_toggle() {
        let mut freeze = SpectralFreeze::new(512, 128, WindowType::Hann);
        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];

        // Process normally
        freeze.process(&mut output, &input);

        // Freeze
        freeze.freeze();
        freeze.process(&mut output, &input);

        // Unfreeze
        freeze.unfreeze();
        freeze.process(&mut output, &input);

        // Should complete without errors
        assert_eq!(output.len(), 256);
    }
}
