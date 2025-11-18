//! Spectral shift - shift all frequencies by a fixed Hz amount
//!
//! Unlike pitch shifting (which multiplies frequencies), spectral shift adds
//! a constant Hz offset to all frequencies. This breaks harmonic relationships,
//! creating metallic, bell-like, or alien timbres.

use super::*;
use rustfft::num_complex::Complex;

/// Spectral shift effect
///
/// Shifts all frequencies by a fixed number of Hz, destroying harmonic
/// relationships and creating inharmonic, metallic timbres.
///
/// **Key Difference from Pitch Shift:**
/// - Pitch shift: 440 Hz → 880 Hz, 880 Hz → 1760 Hz (harmonic)
/// - Spectral shift: 440 Hz → 540 Hz, 880 Hz → 980 Hz (inharmonic!)
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralShift, WindowType};
/// let mut shift = SpectralShift::new(2048, 512, WindowType::Hann, 44100.0);
/// shift.set_shift_hz(100.0);  // Shift all frequencies up 100 Hz
/// shift.set_mix(0.8);          // 80% wet
/// ```
#[derive(Clone, Debug)]
pub struct SpectralShift {
    /// STFT processor
    stft: STFT,

    /// FFT size
    fft_size: usize,

    /// Sample rate
    sample_rate: f32,

    /// Frequency shift in Hz (can be positive or negative)
    shift_hz: f32,

    /// Wet/dry mix (0.0 = dry, 1.0 = wet)
    mix: f32,

    /// Effect enabled flag
    enabled: bool,
}

impl SpectralShift {
    /// Create a new spectral shift effect
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `window_type` - Window function type
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralShift, WindowType};
    /// let shift = SpectralShift::new(2048, 512, WindowType::Hann, 44100.0);
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
            shift_hz: 0.0,
            mix: 1.0,
            enabled: true,
        }
    }

    /// Set frequency shift in Hz
    ///
    /// Positive values shift up, negative shift down.
    ///
    /// # Arguments
    /// * `shift_hz` - Frequency shift in Hz (e.g., 100.0, -50.0)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralShift, WindowType};
    /// let mut shift = SpectralShift::new(2048, 512, WindowType::Hann, 44100.0);
    /// shift.set_shift_hz(100.0);   // Shift up 100 Hz
    /// shift.set_shift_hz(-50.0);   // Shift down 50 Hz
    /// ```
    pub fn set_shift_hz(&mut self, shift_hz: f32) {
        self.shift_hz = shift_hz;
    }

    /// Set wet/dry mix
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = dry, 1.0 = wet)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralShift, WindowType};
    /// let mut shift = SpectralShift::new(2048, 512, WindowType::Hann, 44100.0);
    /// shift.set_mix(0.5);  // 50% wet
    /// ```
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Get current frequency shift in Hz
    pub fn shift_hz(&self) -> f32 {
        self.shift_hz
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.mix
    }

    /// Process audio through the spectral shift
    ///
    /// # Arguments
    /// * `output` - Output buffer (will be filled with processed audio)
    /// * `input` - Input audio buffer
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralShift, WindowType};
    /// let mut shift = SpectralShift::new(2048, 512, WindowType::Hann, 44100.0);
    /// shift.set_shift_hz(100.0);
    ///
    /// let input = vec![0.0; 512];
    /// let mut output = vec![0.0; 512];
    /// shift.process(&mut output, &input);
    /// ```
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        if !self.enabled {
            return;
        }

        let shift_hz = self.shift_hz;
        let mix = self.mix;
        let sample_rate = self.sample_rate;
        let fft_size = self.fft_size;

        self.stft.process(output, |spectrum| {
            Self::apply_shift_static(spectrum, shift_hz, mix, sample_rate, fft_size);
        });
    }

    /// Apply spectral shift (static version for closure)
    #[inline]
    fn apply_shift_static(
        spectrum: &mut [Complex<f32>],
        shift_hz: f32,
        mix: f32,
        sample_rate: f32,
        fft_size: usize,
    ) {
        let len = spectrum.len();

        // Store original spectrum for dry/wet mixing
        let mut dry_spectrum = vec![Complex::new(0.0, 0.0); len];
        dry_spectrum.copy_from_slice(spectrum);

        // Calculate shift in bins (can be fractional)
        let hz_per_bin = sample_rate / fft_size as f32;
        let shift_bins = shift_hz / hz_per_bin;

        // Create shifted spectrum with interpolation
        let mut shifted = vec![Complex::new(0.0, 0.0); len];

        for (i, shifted_bin) in shifted.iter_mut().enumerate().take(len) {
            // Source bin (fractional)
            let src_bin = i as f32 - shift_bins;

            if src_bin >= 0.0 && src_bin < (len - 1) as f32 {
                // Linear interpolation between adjacent bins
                let bin_floor = src_bin.floor() as usize;
                let bin_ceil = bin_floor + 1;
                let frac = src_bin - bin_floor as f32;

                // Interpolate magnitude and phase
                let mag_floor = spectrum[bin_floor].norm();
                let mag_ceil = spectrum[bin_ceil].norm();
                let mag = mag_floor * (1.0 - frac) + mag_ceil * frac;

                let phase_floor = spectrum[bin_floor].arg();
                let phase_ceil = spectrum[bin_ceil].arg();
                let phase = phase_floor * (1.0 - frac) + phase_ceil * frac;

                *shifted_bin = Complex::from_polar(mag, phase);
            }
        }

        // Copy shifted spectrum back
        spectrum.copy_from_slice(&shifted);

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

    /// Reset the spectral shift state
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
impl SpectralShift {
    /// Subtle detuning for thickness (±5-10 Hz)
    pub fn subtle() -> Self {
        let mut shift = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shift.set_shift_hz(7.0);
        shift.set_mix(0.5);
        shift
    }

    /// Metallic timbre (+50-100 Hz)
    pub fn metallic() -> Self {
        let mut shift = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shift.set_shift_hz(75.0);
        shift.set_mix(0.8);
        shift
    }

    /// Bell-like sound (+100-200 Hz)
    pub fn bell() -> Self {
        let mut shift = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shift.set_shift_hz(150.0);
        shift.set_mix(0.9);
        shift
    }

    /// Alien/sci-fi effect (large shift)
    pub fn alien() -> Self {
        let mut shift = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shift.set_shift_hz(300.0);
        shift.set_mix(1.0);
        shift
    }

    /// Downshift (darker, lower)
    pub fn down() -> Self {
        let mut shift = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shift.set_shift_hz(-100.0);
        shift.set_mix(0.8);
        shift
    }
}
