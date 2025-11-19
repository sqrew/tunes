//! Formant shifter - shift formants independently from pitch
//!
//! Formants are the resonant frequencies that define the timbre/color of a sound,
//! especially important in vocal sounds. Unlike pitch shifting (which changes the
//! fundamental frequency), formant shifting moves the spectral envelope without
//! changing pitch, enabling vocal gender changes, character effects, etc.
//!
//! **Key Difference from Pitch Shift:**
//! - Pitch shift: Changes fundamental frequency (makes voice higher/lower)
//! - Formant shift: Changes spectral envelope (makes voice sound male/female/childlike)
//! - Combined: You can pitch-shift while preserving vocal character, or change
//!   character while preserving pitch

use super::*;
use rustfft::num_complex::Complex;

/// Formant shifter effect
///
/// Shifts formants (spectral envelope) independently from pitch by scaling
/// the frequency axis. This creates vocal gender/age changes, character effects,
/// and other timbral transformations.
///
/// **Shift Ratio Guide:**
/// - `0.5` - Shift formants down an octave (male → child, female → male)
/// - `0.7-0.9` - Subtle darkening/warming
/// - `1.0` - No shift (bypass)
/// - `1.1-1.3` - Subtle brightening/thinning
/// - `2.0` - Shift formants up an octave (male → female, female → child)
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{FormantShifter, WindowType};
/// let mut shifter = FormantShifter::new(2048, 512, WindowType::Hann, 44100.0);
/// shifter.set_shift_ratio(0.7);  // Male voice → deeper male
/// shifter.set_mix(1.0);           // 100% wet
/// ```
#[derive(Clone, Debug)]
pub struct FormantShifter {
    /// STFT processor
    stft: STFT,

    /// FFT size
    fft_size: usize,

    /// Sample rate
    sample_rate: f32,

    /// Formant shift ratio (0.5 = down octave, 2.0 = up octave)
    shift_ratio: f32,

    /// Wet/dry mix (0.0 = dry, 1.0 = wet)
    mix: f32,

    /// Effect enabled flag
    enabled: bool,
}

impl FormantShifter {
    /// Create a new formant shifter effect
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `window_type` - Window function type
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{FormantShifter, WindowType};
    /// let shifter = FormantShifter::new(2048, 512, WindowType::Hann, 44100.0);
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
            shift_ratio: 1.0,
            mix: 1.0,
            enabled: true,
        }
    }

    /// Set formant shift ratio
    ///
    /// Values < 1.0 shift formants down (darker, warmer, more masculine).
    /// Values > 1.0 shift formants up (brighter, thinner, more feminine/childlike).
    ///
    /// # Arguments
    /// * `ratio` - Shift ratio (e.g., 0.5, 1.5, 2.0)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{FormantShifter, WindowType};
    /// let mut shifter = FormantShifter::new(2048, 512, WindowType::Hann, 44100.0);
    /// shifter.set_shift_ratio(0.7);   // Shift down (darker)
    /// shifter.set_shift_ratio(1.5);   // Shift up (brighter)
    /// ```
    pub fn set_shift_ratio(&mut self, ratio: f32) {
        self.shift_ratio = ratio.max(0.1); // Prevent extreme values
    }

    /// Set wet/dry mix
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = dry, 1.0 = wet)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{FormantShifter, WindowType};
    /// let mut shifter = FormantShifter::new(2048, 512, WindowType::Hann, 44100.0);
    /// shifter.set_mix(0.5);  // 50% wet
    /// ```
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Get current formant shift ratio
    pub fn shift_ratio(&self) -> f32 {
        self.shift_ratio
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.mix
    }

    /// Process audio through the formant shifter
    ///
    /// # Arguments
    /// * `output` - Output buffer (will be filled with processed audio)
    /// * `input` - Input audio buffer
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{FormantShifter, WindowType};
    /// let mut shifter = FormantShifter::new(2048, 512, WindowType::Hann, 44100.0);
    /// shifter.set_shift_ratio(0.8);
    ///
    /// let input = vec![0.0; 512];
    /// let mut output = vec![0.0; 512];
    /// shifter.process(&mut output, &input);
    /// ```
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        if !self.enabled {
            return;
        }

        let shift_ratio = self.shift_ratio;
        let mix = self.mix;

        self.stft.process(output, |spectrum| {
            Self::apply_formant_shift_static(spectrum, shift_ratio, mix);
        });
    }

    /// Apply formant shift (static version for closure)
    #[inline]
    fn apply_formant_shift_static(
        spectrum: &mut [Complex<f32>],
        shift_ratio: f32,
        mix: f32,
    ) {
        let len = spectrum.len();

        // Store original spectrum for dry/wet mixing
        let mut dry_spectrum = vec![Complex::new(0.0, 0.0); len];
        dry_spectrum.copy_from_slice(spectrum);

        // Create shifted spectrum with interpolation
        let mut shifted = vec![Complex::new(0.0, 0.0); len];

        // For each output bin, find the source bin by scaling
        for (i, shifted_bin) in shifted.iter_mut().enumerate().take(len) {
            // Source bin = output_bin * shift_ratio (scale frequency axis)
            let src_bin = i as f32 * shift_ratio;

            if src_bin >= 0.0 && src_bin < (len - 1) as f32 {
                // Linear interpolation between adjacent bins
                let bin_floor = src_bin.floor() as usize;
                let bin_ceil = (bin_floor + 1).min(len - 1);
                let frac = src_bin - bin_floor as f32;

                // Interpolate magnitude
                let mag_floor = spectrum[bin_floor].norm();
                let mag_ceil = spectrum[bin_ceil].norm();
                let mag = mag_floor * (1.0 - frac) + mag_ceil * frac;

                // Interpolate phase
                let phase_floor = spectrum[bin_floor].arg();
                let phase_ceil = spectrum[bin_ceil].arg();

                // Handle phase wrapping for smooth interpolation
                let mut phase_diff = phase_ceil - phase_floor;
                if phase_diff > std::f32::consts::PI {
                    phase_diff -= 2.0 * std::f32::consts::PI;
                } else if phase_diff < -std::f32::consts::PI {
                    phase_diff += 2.0 * std::f32::consts::PI;
                }
                let phase = phase_floor + phase_diff * frac;

                *shifted_bin = Complex::from_polar(mag, phase);
            }
        }

        // Copy shifted spectrum back
        spectrum.copy_from_slice(&shifted);

        // Apply wet/dry mix with SIMD acceleration
        if mix < 1.0 {
            Self::mix_complex_simd(spectrum, &dry_spectrum, mix);
        }
    }

    /// SIMD-accelerated complex mixing
    /// spectrum = spectrum * wet + dry * (1.0 - wet)
    #[inline]
    fn mix_complex_simd(
        spectrum: &mut [Complex<f32>],
        dry: &[Complex<f32>],
        wet: f32,
    ) {
        let len = spectrum.len();
        let dry_mix = 1.0 - wet;

        // Extract real and imaginary parts for SIMD processing
        let mut wet_re = vec![0.0f32; len];
        let mut wet_im = vec![0.0f32; len];
        let mut dry_re = vec![0.0f32; len];
        let mut dry_im = vec![0.0f32; len];

        for i in 0..len {
            wet_re[i] = spectrum[i].re;
            wet_im[i] = spectrum[i].im;
            dry_re[i] = dry[i].re;
            dry_im[i] = dry[i].im;
        }

        // Use SIMD for mixing
        match SIMD.simd_width() {
            SimdWidth::X8 => {
                Self::mix_simd_impl::<8>(&mut wet_re, &dry_re, wet, dry_mix);
                Self::mix_simd_impl::<8>(&mut wet_im, &dry_im, wet, dry_mix);
            }
            SimdWidth::X4 => {
                Self::mix_simd_impl::<4>(&mut wet_re, &dry_re, wet, dry_mix);
                Self::mix_simd_impl::<4>(&mut wet_im, &dry_im, wet, dry_mix);
            }
            SimdWidth::Scalar => {
                Self::mix_scalar(&mut wet_re, &dry_re, wet, dry_mix);
                Self::mix_scalar(&mut wet_im, &dry_im, wet, dry_mix);
            }
        }

        // Write back to complex spectrum
        for i in 0..len {
            spectrum[i] = Complex::new(wet_re[i], wet_im[i]);
        }
    }

    /// SIMD implementation for mixing (AVX2: 8-wide, SSE/NEON: 4-wide)
    #[inline]
    fn mix_simd_impl<const LANES: usize>(
        wet: &mut [f32],
        dry: &[f32],
        wet_mix: f32,
        dry_mix: f32,
    ) {
        let len = wet.len();
        let chunks = len / LANES;
        let remainder = len % LANES;

        // Process full SIMD chunks
        if LANES == 8 {
            let wet_factor = f32x8::splat(wet_mix);
            let dry_factor = f32x8::splat(dry_mix);

            for i in 0..chunks {
                let offset = i * LANES;
                let wet_vec = f32x8::new([
                    wet[offset], wet[offset + 1], wet[offset + 2], wet[offset + 3],
                    wet[offset + 4], wet[offset + 5], wet[offset + 6], wet[offset + 7],
                ]);
                let dry_vec = f32x8::new([
                    dry[offset], dry[offset + 1], dry[offset + 2], dry[offset + 3],
                    dry[offset + 4], dry[offset + 5], dry[offset + 6], dry[offset + 7],
                ]);

                let result = wet_vec * wet_factor + dry_vec * dry_factor;
                let arr = result.to_array();
                wet[offset..offset + LANES].copy_from_slice(&arr);
            }
        } else if LANES == 4 {
            let wet_factor = f32x4::splat(wet_mix);
            let dry_factor = f32x4::splat(dry_mix);

            for i in 0..chunks {
                let offset = i * LANES;
                let wet_vec = f32x4::new([
                    wet[offset], wet[offset + 1], wet[offset + 2], wet[offset + 3],
                ]);
                let dry_vec = f32x4::new([
                    dry[offset], dry[offset + 1], dry[offset + 2], dry[offset + 3],
                ]);

                let result = wet_vec * wet_factor + dry_vec * dry_factor;
                let arr = result.to_array();
                wet[offset..offset + LANES].copy_from_slice(&arr);
            }
        }

        // Process remainder
        if remainder > 0 {
            let offset = chunks * LANES;
            Self::mix_scalar(
                &mut wet[offset..],
                &dry[offset..],
                wet_mix,
                dry_mix,
            );
        }
    }

    /// Scalar fallback for mixing
    #[inline]
    fn mix_scalar(wet: &mut [f32], dry: &[f32], wet_mix: f32, dry_mix: f32) {
        for i in 0..wet.len() {
            wet[i] = wet[i] * wet_mix + dry[i] * dry_mix;
        }
    }

    /// Reset the formant shifter state
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
impl FormantShifter {
    /// Male voice → female voice
    pub fn male_to_female() -> Self {
        let mut shifter = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shifter.set_shift_ratio(1.6);
        shifter.set_mix(1.0);
        shifter
    }

    /// Female voice → male voice
    pub fn female_to_male() -> Self {
        let mut shifter = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shifter.set_shift_ratio(0.65);
        shifter.set_mix(1.0);
        shifter
    }

    /// Adult voice → child voice
    pub fn adult_to_child() -> Self {
        let mut shifter = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shifter.set_shift_ratio(2.0);
        shifter.set_mix(1.0);
        shifter
    }

    /// Subtle brightening/presence boost
    pub fn brighten() -> Self {
        let mut shifter = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shifter.set_shift_ratio(1.15);
        shifter.set_mix(0.7);
        shifter
    }

    /// Subtle darkening/warming
    pub fn darken() -> Self {
        let mut shifter = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shifter.set_shift_ratio(0.85);
        shifter.set_mix(0.7);
        shifter
    }

    /// Monster/creature voice (extreme downshift)
    pub fn monster() -> Self {
        let mut shifter = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shifter.set_shift_ratio(0.5);
        shifter.set_mix(1.0);
        shifter
    }

    /// Chipmunk/cartoon voice (extreme upshift)
    pub fn chipmunk() -> Self {
        let mut shifter = Self::new(2048, 512, WindowType::Hann, 44100.0);
        shifter.set_shift_ratio(2.5);
        shifter.set_mix(1.0);
        shifter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formant_shifter_creation() {
        let shifter = FormantShifter::new(2048, 512, WindowType::Hann, 44100.0);
        assert_eq!(shifter.shift_ratio(), 1.0);
        assert_eq!(shifter.mix(), 1.0);
        assert_eq!(shifter.fft_size(), 2048);
        assert_eq!(shifter.hop_size(), 512);
        assert!(shifter.is_enabled());
    }

    #[test]
    #[should_panic(expected = "FFT size must be power of 2")]
    fn test_formant_shifter_requires_power_of_two() {
        FormantShifter::new(1000, 250, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Hop size must be <= FFT size")]
    fn test_formant_shifter_hop_validation() {
        FormantShifter::new(1024, 2048, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Sample rate must be positive")]
    fn test_formant_shifter_sample_rate_validation() {
        FormantShifter::new(1024, 256, WindowType::Hann, 0.0);
    }

    #[test]
    fn test_formant_shifter_set_shift_ratio() {
        let mut shifter = FormantShifter::new(1024, 256, WindowType::Hann, 44100.0);

        shifter.set_shift_ratio(2.0);
        assert_eq!(shifter.shift_ratio(), 2.0);

        shifter.set_shift_ratio(0.5);
        assert_eq!(shifter.shift_ratio(), 0.5);

        shifter.set_shift_ratio(1.5);
        assert_eq!(shifter.shift_ratio(), 1.5);
    }

    #[test]
    fn test_formant_shifter_shift_ratio_clamps_minimum() {
        let mut shifter = FormantShifter::new(1024, 256, WindowType::Hann, 44100.0);

        // Very small values should be clamped to 0.1
        shifter.set_shift_ratio(0.05);
        assert_eq!(shifter.shift_ratio(), 0.1);

        shifter.set_shift_ratio(0.0);
        assert_eq!(shifter.shift_ratio(), 0.1);

        shifter.set_shift_ratio(-1.0);
        assert_eq!(shifter.shift_ratio(), 0.1);
    }

    #[test]
    fn test_formant_shifter_set_mix() {
        let mut shifter = FormantShifter::new(1024, 256, WindowType::Hann, 44100.0);

        shifter.set_mix(0.5);
        assert_eq!(shifter.mix(), 0.5);

        shifter.set_mix(0.0);
        assert_eq!(shifter.mix(), 0.0);

        shifter.set_mix(1.0);
        assert_eq!(shifter.mix(), 1.0);
    }

    #[test]
    fn test_formant_shifter_mix_clamps() {
        let mut shifter = FormantShifter::new(1024, 256, WindowType::Hann, 44100.0);

        shifter.set_mix(-0.5);
        assert_eq!(shifter.mix(), 0.0);

        shifter.set_mix(1.5);
        assert_eq!(shifter.mix(), 1.0);
    }

    #[test]
    fn test_formant_shifter_enable_disable() {
        let mut shifter = FormantShifter::new(1024, 256, WindowType::Hann, 44100.0);

        assert!(shifter.is_enabled());

        shifter.set_enabled(false);
        assert!(!shifter.is_enabled());

        shifter.set_enabled(true);
        assert!(shifter.is_enabled());
    }

    #[test]
    fn test_formant_shifter_process_doesnt_crash() {
        let mut shifter = FormantShifter::new(2048, 512, WindowType::Hann, 44100.0);
        shifter.set_shift_ratio(1.5);

        let mut output = vec![0.0; 512];
        let input = vec![0.5; 512];

        // Should not panic
        shifter.process(&mut output, &input);
    }

    #[test]
    fn test_formant_shifter_process_disabled() {
        let mut shifter = FormantShifter::new(2048, 512, WindowType::Hann, 44100.0);
        shifter.set_enabled(false);

        let mut output = vec![1.0; 512];
        let input = vec![0.5; 512];

        shifter.process(&mut output, &input);

        // When disabled, output should remain unchanged (all 1.0)
        for sample in output.iter() {
            assert_eq!(*sample, 1.0);
        }
    }

    #[test]
    fn test_formant_shifter_process_with_zeros() {
        let mut shifter = FormantShifter::new(2048, 512, WindowType::Hann, 44100.0);

        let mut output = vec![0.0; 512];
        let input = vec![0.0; 512];

        // Should handle zero input gracefully
        shifter.process(&mut output, &input);
    }

    #[test]
    fn test_formant_shifter_reset() {
        let mut shifter = FormantShifter::new(1024, 256, WindowType::Hann, 44100.0);

        // Process some audio
        let mut output = vec![0.0; 256];
        let input = vec![0.5; 256];
        shifter.process(&mut output, &input);

        // Reset should not crash
        shifter.reset();

        // Should still work after reset
        shifter.process(&mut output, &input);
    }

    // Preset tests
    #[test]
    fn test_formant_shifter_male_to_female_preset() {
        let shifter = FormantShifter::male_to_female();
        assert_eq!(shifter.shift_ratio(), 1.6);
        assert_eq!(shifter.mix(), 1.0);
        assert!(shifter.is_enabled());
    }

    #[test]
    fn test_formant_shifter_female_to_male_preset() {
        let shifter = FormantShifter::female_to_male();
        assert_eq!(shifter.shift_ratio(), 0.65);
        assert_eq!(shifter.mix(), 1.0);
        assert!(shifter.is_enabled());
    }

    #[test]
    fn test_formant_shifter_adult_to_child_preset() {
        let shifter = FormantShifter::adult_to_child();
        assert_eq!(shifter.shift_ratio(), 2.0);
        assert_eq!(shifter.mix(), 1.0);
        assert!(shifter.is_enabled());
    }

    #[test]
    fn test_formant_shifter_brighten_preset() {
        let shifter = FormantShifter::brighten();
        assert_eq!(shifter.shift_ratio(), 1.15);
        assert_eq!(shifter.mix(), 0.7);
        assert!(shifter.is_enabled());
    }

    #[test]
    fn test_formant_shifter_darken_preset() {
        let shifter = FormantShifter::darken();
        assert_eq!(shifter.shift_ratio(), 0.85);
        assert_eq!(shifter.mix(), 0.7);
        assert!(shifter.is_enabled());
    }

    #[test]
    fn test_formant_shifter_monster_preset() {
        let shifter = FormantShifter::monster();
        assert_eq!(shifter.shift_ratio(), 0.5);
        assert_eq!(shifter.mix(), 1.0);
        assert!(shifter.is_enabled());
    }

    #[test]
    fn test_formant_shifter_chipmunk_preset() {
        let shifter = FormantShifter::chipmunk();
        assert_eq!(shifter.shift_ratio(), 2.5);
        assert_eq!(shifter.mix(), 1.0);
        assert!(shifter.is_enabled());
    }

    #[test]
    fn test_formant_shifter_different_fft_sizes() {
        // Test various valid FFT sizes
        for &fft_size in &[256, 512, 1024, 2048, 4096] {
            let hop_size = fft_size / 4;
            let shifter = FormantShifter::new(fft_size, hop_size, WindowType::Hann, 44100.0);
            assert_eq!(shifter.fft_size(), fft_size);
            assert_eq!(shifter.hop_size(), hop_size);
        }
    }

    #[test]
    fn test_formant_shifter_different_window_types() {
        // Test all window types
        for &window_type in &[
            WindowType::Rectangular,
            WindowType::Hann,
            WindowType::Hamming,
            WindowType::Blackman,
            WindowType::BlackmanHarris,
        ] {
            let shifter = FormantShifter::new(1024, 256, window_type, 44100.0);
            assert_eq!(shifter.shift_ratio(), 1.0);
        }
    }

    #[test]
    fn test_formant_shifter_bypass_mode() {
        let mut shifter = FormantShifter::new(2048, 512, WindowType::Hann, 44100.0);
        shifter.set_shift_ratio(1.0); // No shift
        shifter.set_mix(1.0);

        let mut output = vec![0.0; 512];
        let input = vec![0.5; 512];

        // Process should work in bypass mode (shift_ratio = 1.0)
        shifter.process(&mut output, &input);
    }
}
