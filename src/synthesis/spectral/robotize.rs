//! Spectral robotize effect - quantize phases for robotic sound

use super::*;
use rustfft::num_complex::Complex;

/// Spectral robotize effect
///
/// Creates a robotic/vocoded sound by quantizing the phase of each frequency bin.
/// This removes natural phase relationships while preserving the magnitude spectrum.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralRobotize, WindowType};
/// let mut robotize = SpectralRobotize::new(2048, 512, WindowType::Hann);
/// robotize.set_mix(1.0);  // 100% robotized
///
/// let input = vec![0.0; 512];
/// let mut output = vec![0.0; 512];
/// robotize.process(&mut output, &input);
/// ```
#[derive(Clone, Debug)]
pub struct SpectralRobotize {
    stft: STFT,

    /// Target phase to quantize to (typically 0.0)
    target_phase: f32,

    /// Mix between original (0.0) and robotized (1.0)
    mix: f32,

    enabled: bool,
}

impl SpectralRobotize {
    /// Create a new spectral robotize effect
    ///
    /// # Arguments
    ///
    /// * `fft_size` - FFT size (must be power of 2, typically 2048)
    /// * `hop_size` - Hop size between FFT frames (typically fft_size/4)
    /// * `window_type` - Window function (Hann recommended)
    ///
    /// # Panics
    ///
    /// Panics if fft_size is not a power of 2 or hop_size > fft_size
    pub fn new(fft_size: usize, hop_size: usize, window_type: WindowType) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(hop_size <= fft_size, "Hop size must be <= FFT size");

        Self {
            stft: STFT::new(fft_size, hop_size, window_type),
            target_phase: 0.0, // Default to zero phase
            mix: 1.0,          // Default to 100% robotized
            enabled: true,
        }
    }

    /// Set the target phase to quantize to
    ///
    /// # Arguments
    ///
    /// * `phase` - Target phase in radians (typically 0.0)
    pub fn set_target_phase(&mut self, phase: f32) {
        self.target_phase = phase;
    }

    /// Set the mix between original and robotized signal
    ///
    /// # Arguments
    ///
    /// * `mix` - Mix amount (0.0 = original, 1.0 = fully robotized)
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Get current target phase
    pub fn target_phase(&self) -> f32 {
        self.target_phase
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.mix
    }

    /// Process audio with spectral robotization
    ///
    /// # Arguments
    ///
    /// * `output` - Output buffer to write processed audio
    /// * `input` - Input audio buffer
    pub fn process(&mut self, output: &mut [f32], input: &[f32]) {
        if !self.enabled {
            output.copy_from_slice(input);
            return;
        }

        // Add input to STFT buffer
        self.stft.add_input(input);

        let target_phase = self.target_phase;
        let mix = self.mix;

        self.stft.process(output, |spectrum| {
            Self::robotize_spectrum(spectrum, target_phase, mix);
        });
    }

    /// Robotize a spectrum by quantizing phases
    ///
    /// This is the core robotization algorithm that runs per FFT frame.
    #[inline]
    fn robotize_spectrum(spectrum: &mut [Complex<f32>], target_phase: f32, mix: f32) {
        // Pre-calculate target complex value for efficiency
        let _target_re = target_phase.cos();
        let _target_im = target_phase.sin();

        for bin in spectrum.iter_mut() {
            // Calculate magnitude (preserve energy)
            let magnitude = (bin.re * bin.re + bin.im * bin.im).sqrt();

            // Calculate original phase
            let original_phase = bin.im.atan2(bin.re);

            // Interpolate phase between original and target
            let new_phase = if mix >= 1.0 {
                target_phase
            } else if mix <= 0.0 {
                original_phase
            } else {
                // Linear interpolation in phase space
                original_phase * (1.0 - mix) + target_phase * mix
            };

            // Reconstruct with new phase
            bin.re = magnitude * new_phase.cos();
            bin.im = magnitude * new_phase.sin();
        }
    }

    /// Reset internal state
    pub fn reset(&mut self) {
        self.stft.reset();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_robotize_creation() {
        let robotize = SpectralRobotize::new(2048, 512, WindowType::Hann);
        assert_eq!(robotize.target_phase(), 0.0);
        assert_eq!(robotize.mix(), 1.0);
        assert!(robotize.is_enabled());
    }

    #[test]
    #[should_panic(expected = "FFT size must be power of 2")]
    fn test_spectral_robotize_requires_power_of_two() {
        SpectralRobotize::new(1000, 250, WindowType::Hann);
    }

    #[test]
    #[should_panic(expected = "Hop size must be <= FFT size")]
    fn test_spectral_robotize_hop_validation() {
        SpectralRobotize::new(512, 1024, WindowType::Hann);
    }

    #[test]
    fn test_spectral_robotize_set_target_phase() {
        let mut robotize = SpectralRobotize::new(512, 128, WindowType::Hann);
        robotize.set_target_phase(std::f32::consts::PI / 2.0);
        assert_eq!(robotize.target_phase(), std::f32::consts::PI / 2.0);
    }

    #[test]
    fn test_spectral_robotize_set_mix() {
        let mut robotize = SpectralRobotize::new(512, 128, WindowType::Hann);

        robotize.set_mix(0.5);
        assert_eq!(robotize.mix(), 0.5);

        // Test clamping
        robotize.set_mix(1.5);
        assert_eq!(robotize.mix(), 1.0);

        robotize.set_mix(-0.5);
        assert_eq!(robotize.mix(), 0.0);
    }

    #[test]
    fn test_spectral_robotize_process_silent() {
        let mut robotize = SpectralRobotize::new(512, 128, WindowType::Hann);

        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];

        robotize.process(&mut output, &input);

        // Silent input should produce silent output
        for &sample in &output {
            assert!(sample.abs() < 1e-6);
        }
    }

    #[test]
    fn test_spectral_robotize_process_with_signal() {
        let mut robotize = SpectralRobotize::new(2048, 512, WindowType::Hann);

        // Create a test signal with known frequency
        let mut input = vec![0.0; 2048];
        for i in 0..2048 {
            let t = i as f32 / 44100.0;
            input[i] = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.5;
        }

        let mut output = vec![0.0; 2048];
        robotize.process(&mut output, &input);

        // Output should be non-zero (signal was robotized, not silenced)
        let output_energy: f32 = output.iter().map(|x| x * x).sum();
        assert!(output_energy > 0.0);
    }

    #[test]
    fn test_spectral_robotize_disabled() {
        let mut robotize = SpectralRobotize::new(512, 128, WindowType::Hann);
        robotize.set_enabled(false);

        let input = vec![0.5; 256];
        let mut output = vec![0.0; 256];

        robotize.process(&mut output, &input);

        // When disabled, output should equal input
        for i in 0..256 {
            assert_eq!(output[i], input[i]);
        }
    }

    #[test]
    fn test_spectral_robotize_reset() {
        let mut robotize = SpectralRobotize::new(512, 128, WindowType::Hann);

        // Process some audio
        let input = vec![0.5; 256];
        let mut output = vec![0.0; 256];
        robotize.process(&mut output, &input);

        // Reset
        robotize.reset();

        // Should not crash after reset
        robotize.process(&mut output, &input);
        assert_eq!(output.len(), 256);
    }

    #[test]
    fn test_spectral_robotize_all_window_types() {
        for window_type in [WindowType::Hann, WindowType::Hamming, WindowType::Blackman, WindowType::Rectangular] {
            let mut robotize = SpectralRobotize::new(512, 128, window_type);

            let input = vec![0.0; 256];
            let mut output = vec![0.0; 256];

            robotize.process(&mut output, &input);
            assert_eq!(output.len(), 256);
        }
    }

    #[test]
    fn test_spectral_robotize_various_fft_sizes() {
        for fft_size in [512, 1024, 2048, 4096] {
            let hop_size = fft_size / 4;
            let mut robotize = SpectralRobotize::new(fft_size, hop_size, WindowType::Hann);

            let input = vec![0.0; 512];
            let mut output = vec![0.0; 512];

            robotize.process(&mut output, &input);
            assert_eq!(output.len(), 512);
        }
    }

    #[test]
    fn test_spectral_robotize_enable_disable() {
        let mut robotize = SpectralRobotize::new(512, 128, WindowType::Hann);

        assert!(robotize.is_enabled());

        robotize.set_enabled(false);
        assert!(!robotize.is_enabled());

        robotize.set_enabled(true);
        assert!(robotize.is_enabled());
    }

    #[test]
    fn test_spectral_robotize_zero_mix() {
        let mut robotize = SpectralRobotize::new(2048, 512, WindowType::Hann);
        robotize.set_mix(0.0); // No robotization

        let mut input = vec![0.0; 2048];
        for i in 0..2048 {
            let t = i as f32 / 44100.0;
            input[i] = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.3;
        }

        let mut output = vec![0.0; 2048];
        robotize.process(&mut output, &input);

        // With 0 mix, output should be similar to input (allowing for STFT artifacts)
        let output_energy: f32 = output.iter().map(|x| x * x).sum();
        assert!(output_energy > 0.0);
    }

    #[test]
    fn test_spectral_robotize_full_mix() {
        let mut robotize = SpectralRobotize::new(2048, 512, WindowType::Hann);
        robotize.set_mix(1.0); // Full robotization

        let mut input = vec![0.0; 2048];
        for i in 0..2048 {
            let t = i as f32 / 44100.0;
            input[i] = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.3;
        }

        let mut output = vec![0.0; 2048];
        robotize.process(&mut output, &input);

        // Output should be non-zero
        let output_energy: f32 = output.iter().map(|x| x * x).sum();
        assert!(output_energy > 0.0);
    }
}

