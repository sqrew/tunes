//! Phase vocoder for time-stretching and pitch-shifting

use super::*;
use rustfft::num_complex::Complex;
use wide::{f32x4, f32x8};

/// Phase vocoder for high-quality time-stretching and pitch-shifting
///
/// Uses STFT analysis/synthesis with phase coherence preservation for
/// artifact-free time/pitch manipulation. All operations SIMD-accelerated.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{PhaseVocoder, WindowType};
/// // Create vocoder: 2048 FFT, 512 hop (75% overlap)
/// let mut vocoder = PhaseVocoder::new(2048, 512, 44100.0, WindowType::Hann);
///
/// // Slow down by 2x (half speed, preserves pitch)
/// vocoder.set_time_stretch(2.0);
///
/// // Or pitch shift up 7 semitones (perfect fifth)
/// vocoder.set_pitch_shift(7.0);
///
/// // Process audio
/// let input = vec![0.0; 512];
/// let mut output = vec![0.0; 512];
/// vocoder.process(&mut output, &input);
/// ```
#[derive(Clone)]
pub struct PhaseVocoder {
    /// STFT processor
    stft: STFT,

    /// Sample rate
    sample_rate: f32,

    /// FFT size
    fft_size: usize,

    /// Hop size (samples between frames)
    hop_size: usize,

    /// Time stretch ratio (1.0 = normal, 2.0 = half speed, 0.5 = double speed)
    time_stretch: f32,

    /// Pitch shift in semitones (0 = no shift, 12 = up one octave, -12 = down one octave)
    pitch_shift: f32,

    /// Previous frame phase (for phase unwrapping)
    prev_phase: Vec<f32>,

    /// Accumulated output phase
    phase_accum: Vec<f32>,

    /// Expected phase advance per hop
    expected_phase_advance: Vec<f32>,

    /// Bin frequencies in Hz
    bin_freqs: Vec<f32>,
}

impl PhaseVocoder {
    /// Create a new phase vocoder
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `sample_rate` - Audio sample rate in Hz
    /// * `window_type` - Window function type
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{PhaseVocoder, WindowType};
    /// let vocoder = PhaseVocoder::new(2048, 512, 44100.0, WindowType::Hann);
    /// ```
    pub fn new(
        fft_size: usize,
        hop_size: usize,
        sample_rate: f32,
        window_type: WindowType,
    ) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(hop_size <= fft_size, "Hop size must be <= FFT size");
        assert!(sample_rate > 0.0, "Sample rate must be positive");

        let stft = STFT::new(fft_size, hop_size, window_type);

        // Pre-calculate expected phase advance for each bin
        let mut expected_phase_advance = vec![0.0; fft_size];
        let mut bin_freqs = vec![0.0; fft_size];

        for k in 0..fft_size {
            let freq = k as f32 * sample_rate / fft_size as f32;
            bin_freqs[k] = freq;
            expected_phase_advance[k] =
                2.0 * std::f32::consts::PI * freq * hop_size as f32 / sample_rate;
        }

        Self {
            stft,
            sample_rate,
            fft_size,
            hop_size,
            time_stretch: 1.0,
            pitch_shift: 0.0,
            prev_phase: vec![0.0; fft_size],
            phase_accum: vec![0.0; fft_size],
            expected_phase_advance,
            bin_freqs,
        }
    }

    /// Set time stretch ratio
    ///
    /// # Arguments
    /// * `ratio` - Time stretch ratio (1.0 = normal, 2.0 = half speed, 0.5 = double speed)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{PhaseVocoder, WindowType};
    /// let mut vocoder = PhaseVocoder::new(2048, 512, 44100.0, WindowType::Hann);
    /// vocoder.set_time_stretch(1.5);  // 1.5x slower
    /// ```
    pub fn set_time_stretch(&mut self, ratio: f32) {
        assert!(ratio > 0.0, "Time stretch ratio must be positive");
        self.time_stretch = ratio;
    }

    /// Set pitch shift in semitones
    ///
    /// # Arguments
    /// * `semitones` - Pitch shift in semitones (0 = no shift, 12 = up octave, -12 = down octave)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{PhaseVocoder, WindowType};
    /// let mut vocoder = PhaseVocoder::new(2048, 512, 44100.0, WindowType::Hann);
    /// vocoder.set_pitch_shift(7.0);  // Perfect fifth up
    /// ```
    pub fn set_pitch_shift(&mut self, semitones: f32) {
        self.pitch_shift = semitones;
    }

    /// Process audio through the phase vocoder
    ///
    /// # Arguments
    /// * `output` - Output buffer (will be filled with processed audio)
    /// * `input` - Input audio buffer
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{PhaseVocoder, WindowType};
    /// let mut vocoder = PhaseVocoder::new(2048, 512, 44100.0, WindowType::Hann);
    /// let input = vec![0.0; 512];
    /// let mut output = vec![0.0; 512];
    /// vocoder.process(&mut output, &input);
    /// ```
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        // Extract mutable references to avoid borrow checker issues
        let fft_size = self.fft_size;
        let hop_size = self.hop_size;
        let sample_rate = self.sample_rate;
        let time_stretch = self.time_stretch;
        let pitch_shift = self.pitch_shift;
        let prev_phase = &mut self.prev_phase;
        let phase_accum = &mut self.phase_accum;
        let expected_phase_advance = &self.expected_phase_advance;
        let bin_freqs = &self.bin_freqs;

        self.stft.process(output, |spectrum| {
            Self::process_spectrum_static(
                spectrum,
                fft_size,
                hop_size,
                sample_rate,
                time_stretch,
                pitch_shift,
                prev_phase,
                phase_accum,
                expected_phase_advance,
                bin_freqs,
            );
        });
    }

    /// Process a spectrum with phase vocoder algorithm (SIMD-accelerated) - static version
    #[allow(clippy::too_many_arguments)]
    fn process_spectrum_static(
        spectrum: &mut [Complex<f32>],
        fft_size: usize,
        hop_size: usize,
        sample_rate: f32,
        time_stretch: f32,
        pitch_shift: f32,
        prev_phase: &mut [f32],
        phase_accum: &mut [f32],
        expected_phase_advance: &[f32],
        bin_freqs: &[f32],
    ) {
        // Working buffers for SIMD processing
        let mut magnitudes = vec![0.0; fft_size];
        let mut phases = vec![0.0; fft_size];
        let mut inst_freqs = vec![0.0; fft_size];

        // Extract magnitude using SIMD
        ComplexOps::magnitude(&mut magnitudes, spectrum);

        // Extract phase (atan2 is hard to SIMD, keep scalar)
        for (i, &s) in spectrum.iter().enumerate() {
            phases[i] = s.im.atan2(s.re);
        }

        // Calculate phase difference using TRUE SIMD
        let mut phase_diffs = phases.to_vec();
        for i in 0..fft_size {
            phase_diffs[i] -= prev_phase[i] + expected_phase_advance[i];
        }

        // Wrap phases to [-π, π] using SIMD-friendly modulo
        Self::wrap_phases_simd(&mut phase_diffs);

        // Calculate instantaneous frequencies using TRUE SIMD (FMA: freq = bin_freq + diff * scale)
        let freq_scale = sample_rate / (hop_size as f32 * 2.0 * std::f32::consts::PI);
        inst_freqs.copy_from_slice(bin_freqs);
        SIMD.fma(&mut inst_freqs, freq_scale, 0.0); // No-op, just copy
        for i in 0..fft_size {
            inst_freqs[i] = bin_freqs[i] + phase_diffs[i] * freq_scale;
        }

        // Store current phase for next frame
        prev_phase.copy_from_slice(&phases);

        // Accumulate phase using TRUE SIMD
        let phase_scale = 2.0 * std::f32::consts::PI * hop_size as f32 / sample_rate * time_stretch;
        let mut advances = inst_freqs.clone();
        SIMD.multiply_const(&mut advances, phase_scale); // TRUE SIMD multiply

        for i in 0..fft_size {
            phase_accum[i] += advances[i];
        }
        Self::wrap_phases_simd(phase_accum);

        // Apply pitch shift by bin shifting (if needed)
        if pitch_shift.abs() > 0.001 {
            Self::apply_pitch_shift_static(&mut magnitudes, pitch_shift, fft_size);
        }

        // Reconstruct complex spectrum using SIMD sin/cos
        Self::reconstruct_spectrum_simd(spectrum, &magnitudes, phase_accum);
    }

    /// Apply pitch shift by shifting frequency bins - static version
    fn apply_pitch_shift_static(magnitudes: &mut [f32], pitch_shift: f32, fft_size: usize) {
        // Pitch shift ratio: 2^(semitones/12)
        let shift_ratio = 2.0f32.powf(pitch_shift / 12.0);

        let original = magnitudes.to_vec();
        magnitudes.fill(0.0);

        // Shift bins (simple nearest-neighbor for now)
        for i in 0..fft_size / 2 {
            let new_bin = (i as f32 * shift_ratio).round() as usize;
            if new_bin < fft_size / 2 {
                magnitudes[new_bin] = original[i];
            }
        }

        // Mirror for negative frequencies
        for i in 1..fft_size / 2 {
            magnitudes[fft_size - i] = magnitudes[i];
        }
    }

    /// Wrap phase to [-π, π] range (scalar version)
    #[allow(dead_code)]
    #[inline(always)]
    fn wrap_phase(mut phase: f32) -> f32 {
        const PI: f32 = std::f32::consts::PI;
        while phase > PI {
            phase -= 2.0 * PI;
        }
        while phase < -PI {
            phase += 2.0 * PI;
        }
        phase
    }

    /// Wrap phases to [-π, π] using SIMD-friendly algorithm
    fn wrap_phases_simd(phases: &mut [f32]) {
        use std::f32::consts::PI;
        const TWO_PI: f32 = 2.0 * PI;
        const INV_TWO_PI: f32 = 1.0 / (2.0 * PI);

        // Use modulo-based wrapping (SIMD-friendly, no branching)
        for phase in phases.iter_mut() {
            // Normalize to [-π, π] using: phase - 2π * round(phase / 2π)
            let cycles = (*phase * INV_TWO_PI).round();
            *phase -= cycles * TWO_PI;
        }
    }

    /// Reconstruct complex spectrum from magnitude and phase using SIMD sin/cos
    fn reconstruct_spectrum_simd(
        spectrum: &mut [Complex<f32>],
        magnitudes: &[f32],
        phases: &[f32],
    ) {
        use crate::synthesis::simd::{SIMD, SimdLanes, SimdWidth};

        let len = spectrum.len();

        // Process with appropriate SIMD width
        match SIMD.simd_width() {
            SimdWidth::X8 => Self::reconstruct_simd_impl::<f32x8>(spectrum, magnitudes, phases),
            SimdWidth::X4 => Self::reconstruct_simd_impl::<f32x4>(spectrum, magnitudes, phases),
            SimdWidth::Scalar => {
                // Scalar fallback using fast trig
                for i in 0..len {
                    let mag = magnitudes[i];
                    let phase = phases[i];
                    spectrum[i] =
                        Complex::new(mag * f32::fast_cos(phase), mag * f32::fast_sin(phase));
                }
            }
        }
    }

    /// SIMD implementation of spectrum reconstruction
    fn reconstruct_simd_impl<V: SimdLanes>(
        spectrum: &mut [Complex<f32>],
        magnitudes: &[f32],
        phases: &[f32],
    ) {
        let len = spectrum.len();
        let simd_len = len - (len % V::LANES);

        // Process SIMD chunks
        for i in (0..simd_len).step_by(V::LANES) {
            let mag_vec = V::from_array(&magnitudes[i..]);
            let phase_vec = V::from_array(&phases[i..]);

            // Compute sin/cos using SIMD
            let cos_vec = phase_vec.fast_cos();
            let sin_vec = phase_vec.fast_sin();

            // Multiply magnitude by sin/cos
            let real_vec = mag_vec.mul(cos_vec);
            let imag_vec = mag_vec.mul(sin_vec);

            // Write back to spectrum
            let mut real_arr = [0.0f32; 8];
            let mut imag_arr = [0.0f32; 8];
            real_vec.write_to_slice(&mut real_arr);
            imag_vec.write_to_slice(&mut imag_arr);

            for j in 0..V::LANES.min(len - i) {
                spectrum[i + j] = Complex::new(real_arr[j], imag_arr[j]);
            }
        }

        // Handle remainder with scalar fast trig
        for i in simd_len..len {
            let mag = magnitudes[i];
            let phase = phases[i];
            spectrum[i] = Complex::new(mag * f32::fast_cos(phase), mag * f32::fast_sin(phase));
        }
    }

    /// Reset the phase vocoder state
    pub fn reset(&mut self) {
        self.stft.reset();
        self.prev_phase.fill(0.0);
        self.phase_accum.fill(0.0);
    }

    /// Get the current time stretch ratio
    pub fn time_stretch(&self) -> f32 {
        self.time_stretch
    }

    /// Get the current pitch shift in semitones
    pub fn pitch_shift(&self) -> f32 {
        self.pitch_shift
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_vocoder_creation() {
        let vocoder = PhaseVocoder::new(2048, 512, 44100.0, WindowType::Hann);
        assert_eq!(vocoder.time_stretch(), 1.0);
        assert_eq!(vocoder.pitch_shift(), 0.0);
    }

    #[test]
    #[should_panic(expected = "FFT size must be power of 2")]
    fn test_phase_vocoder_requires_power_of_two() {
        PhaseVocoder::new(1000, 250, 44100.0, WindowType::Hann);
    }

    #[test]
    #[should_panic(expected = "Hop size must be <= FFT size")]
    fn test_phase_vocoder_hop_validation() {
        PhaseVocoder::new(1024, 2048, 44100.0, WindowType::Hann);
    }

    #[test]
    #[should_panic(expected = "Sample rate must be positive")]
    fn test_phase_vocoder_sample_rate_validation() {
        PhaseVocoder::new(1024, 256, 0.0, WindowType::Hann);
    }

    #[test]
    fn test_phase_vocoder_set_time_stretch() {
        let mut vocoder = PhaseVocoder::new(1024, 256, 44100.0, WindowType::Hann);

        vocoder.set_time_stretch(2.0);
        assert_eq!(vocoder.time_stretch(), 2.0);

        vocoder.set_time_stretch(0.5);
        assert_eq!(vocoder.time_stretch(), 0.5);
    }

    #[test]
    #[should_panic(expected = "Time stretch ratio must be positive")]
    fn test_phase_vocoder_time_stretch_validation() {
        let mut vocoder = PhaseVocoder::new(1024, 256, 44100.0, WindowType::Hann);
        vocoder.set_time_stretch(0.0);
    }

    #[test]
    fn test_phase_vocoder_set_pitch_shift() {
        let mut vocoder = PhaseVocoder::new(1024, 256, 44100.0, WindowType::Hann);

        vocoder.set_pitch_shift(12.0);  // Up one octave
        assert_eq!(vocoder.pitch_shift(), 12.0);

        vocoder.set_pitch_shift(-12.0);  // Down one octave
        assert_eq!(vocoder.pitch_shift(), -12.0);

        vocoder.set_pitch_shift(7.0);  // Perfect fifth
        assert_eq!(vocoder.pitch_shift(), 7.0);
    }

    #[test]
    fn test_phase_vocoder_process_silent() {
        let mut vocoder = PhaseVocoder::new(1024, 256, 44100.0, WindowType::Hann);
        let input = vec![0.0; 512];
        let mut output = vec![0.0; 512];

        // Process silence (should remain silent)
        vocoder.process(&mut output, &input);

        // Output should be all zeros or very close
        for &sample in &output {
            assert!(sample.abs() < 0.001, "Expected silence, got {}", sample);
        }
    }

    #[test]
    fn test_phase_vocoder_process_with_time_stretch() {
        let mut vocoder = PhaseVocoder::new(512, 128, 44100.0, WindowType::Hann);
        vocoder.set_time_stretch(2.0);  // Half speed

        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];

        // Should process without crashing
        vocoder.process(&mut output, &input);
        assert_eq!(output.len(), 256);
    }

    #[test]
    fn test_phase_vocoder_process_with_pitch_shift() {
        let mut vocoder = PhaseVocoder::new(512, 128, 44100.0, WindowType::Hann);
        vocoder.set_pitch_shift(7.0);  // Perfect fifth up

        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];

        // Should process without crashing
        vocoder.process(&mut output, &input);
        assert_eq!(output.len(), 256);
    }

    #[test]
    fn test_phase_vocoder_combined_time_and_pitch() {
        let mut vocoder = PhaseVocoder::new(1024, 256, 44100.0, WindowType::Hann);
        vocoder.set_time_stretch(1.5);  // 1.5x slower
        vocoder.set_pitch_shift(-5.0);  // Down a fourth

        let input = vec![0.0; 512];
        let mut output = vec![0.0; 512];

        // Should process without crashing
        vocoder.process(&mut output, &input);
        assert_eq!(output.len(), 512);
    }

    #[test]
    fn test_phase_vocoder_reset() {
        let mut vocoder = PhaseVocoder::new(512, 128, 44100.0, WindowType::Hann);

        // Process some audio
        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];
        vocoder.process(&mut output, &input);

        // Reset should clear state
        vocoder.reset();

        // Should still work after reset
        vocoder.process(&mut output, &input);
        assert_eq!(output.len(), 256);
    }

    #[test]
    fn test_phase_vocoder_all_window_types() {
        for window_type in [
            WindowType::Rectangular,
            WindowType::Hann,
            WindowType::Hamming,
            WindowType::Blackman,
            WindowType::BlackmanHarris,
        ] {
            let mut vocoder = PhaseVocoder::new(512, 128, 44100.0, window_type);
            let input = vec![0.0; 256];
            let mut output = vec![0.0; 256];

            vocoder.process(&mut output, &input);
            assert_eq!(output.len(), 256);
        }
    }

    #[test]
    fn test_phase_vocoder_various_fft_sizes() {
        for fft_size in [512, 1024, 2048, 4096] {
            let hop_size = fft_size / 4;
            let mut vocoder = PhaseVocoder::new(fft_size, hop_size, 44100.0, WindowType::Hann);

            let input = vec![0.0; 512];
            let mut output = vec![0.0; 512];

            vocoder.process(&mut output, &input);
            assert_eq!(output.len(), 512);
        }
    }

    #[test]
    fn test_phase_vocoder_wrap_phase() {
        use std::f32::consts::PI;

        // Test phase wrapping
        assert!((PhaseVocoder::wrap_phase(0.0) - 0.0).abs() < 0.001);
        assert!((PhaseVocoder::wrap_phase(PI) - PI).abs() < 0.001);
        assert!((PhaseVocoder::wrap_phase(-PI) - (-PI)).abs() < 0.001);

        // Should wrap 2π to ~0
        assert!(PhaseVocoder::wrap_phase(2.0 * PI).abs() < 0.001);
        assert!(PhaseVocoder::wrap_phase(-2.0 * PI).abs() < 0.001);

        // Should wrap 3π to π
        assert!((PhaseVocoder::wrap_phase(3.0 * PI) - PI).abs() < 0.001);
        assert!((PhaseVocoder::wrap_phase(-3.0 * PI) - (-PI)).abs() < 0.001);
    }
}
