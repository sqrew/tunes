//! Spectral morphing - morph between different spectral shapes
//!
//! Morphs the input spectrum toward target spectral shapes like
//! harmonic series, noise, or filtered versions. Creates vocoder-like
//! and transformative effects.

use super::*;
use rustfft::num_complex::Complex;

/// Target spectrum type for morphing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorphTarget {
    /// Morph toward harmonic series (pitched)
    Harmonic,
    /// Morph toward white noise (unpitched)
    Noise,
    /// Morph toward whisper (high-pass noise)
    Whisper,
    /// Morph toward robot (quantized harmonics)
    Robot,
}

/// Spectral morph effect
///
/// Morphs the input spectrum toward different target spectral shapes.
/// Creates vocoder-like transformations, robotization, or noise effects.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralMorph, MorphTarget, WindowType};
/// let mut morph = SpectralMorph::new(2048, 512, WindowType::Hann, 44100.0);
/// morph.set_target(MorphTarget::Robot);
/// morph.set_morph_amount(0.7);
/// ```
#[derive(Clone, Debug)]
pub struct SpectralMorph {
    /// STFT processor
    stft: STFT,

    /// FFT size
    fft_size: usize,

    /// Sample rate
    sample_rate: f32,

    /// Target spectrum type
    target: MorphTarget,

    /// Morph amount (0.0 = original, 1.0 = full target)
    morph_amount: f32,

    /// Fundamental frequency for harmonic target (Hz)
    fundamental: f32,

    /// Effect enabled flag
    enabled: bool,
}

impl SpectralMorph {
    /// Create a new spectral morph effect
    pub fn new(fft_size: usize, hop_size: usize, window_type: WindowType, sample_rate: f32) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(hop_size <= fft_size, "Hop size must be <= FFT size");
        assert!(sample_rate > 0.0, "Sample rate must be positive");

        let stft = STFT::new(fft_size, hop_size, window_type);

        Self {
            stft,
            fft_size,
            sample_rate,
            target: MorphTarget::Harmonic,
            morph_amount: 0.5,
            fundamental: 110.0, // A2
            enabled: true,
        }
    }

    /// Set target spectrum type
    pub fn set_target(&mut self, target: MorphTarget) {
        self.target = target;
    }

    /// Set morph amount
    pub fn set_morph_amount(&mut self, amount: f32) {
        self.morph_amount = amount.clamp(0.0, 1.0);
    }

    /// Set fundamental frequency for harmonic target
    pub fn set_fundamental(&mut self, frequency: f32) {
        self.fundamental = frequency.clamp(20.0, 2000.0);
    }

    /// Get current target
    pub fn target(&self) -> MorphTarget {
        self.target
    }

    /// Get morph amount
    pub fn morph_amount(&self) -> f32 {
        self.morph_amount
    }

    /// Get fundamental frequency
    pub fn fundamental(&self) -> f32 {
        self.fundamental
    }

    /// Process audio through spectral morphing
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        if !self.enabled {
            return;
        }

        let target = self.target;
        let morph_amount = self.morph_amount;
        let fundamental = self.fundamental;
        let sample_rate = self.sample_rate;
        let fft_size = self.fft_size;

        self.stft.process(output, |spectrum| {
            Self::apply_morph_static(spectrum, target, morph_amount, fundamental, sample_rate, fft_size);
        });
    }

    /// Apply spectral morphing (static version for closure)
    #[inline]
    fn apply_morph_static(
        spectrum: &mut [Complex<f32>],
        target: MorphTarget,
        morph_amount: f32,
        fundamental: f32,
        sample_rate: f32,
        fft_size: usize,
    ) {
        let len = spectrum.len();
        let hz_per_bin = sample_rate / fft_size as f32;

        // Extract magnitude and phase
        let mut magnitudes: Vec<f32> = spectrum.iter().map(|c| c.norm()).collect();
        let phases: Vec<f32> = spectrum.iter().map(|c| c.arg()).collect();

        // Generate target magnitudes based on target type
        let target_mags = match target {
            MorphTarget::Harmonic => {
                Self::generate_harmonic_spectrum(len, fundamental, hz_per_bin)
            }
            MorphTarget::Noise => {
                Self::generate_noise_spectrum(len)
            }
            MorphTarget::Whisper => {
                Self::generate_whisper_spectrum(len, hz_per_bin)
            }
            MorphTarget::Robot => {
                Self::generate_robot_spectrum(len, fundamental, hz_per_bin)
            }
        };

        // Morph magnitudes toward target
        for i in 0..len {
            magnitudes[i] = magnitudes[i] * (1.0 - morph_amount) + target_mags[i] * morph_amount;
        }

        // Reconstruct spectrum with morphed magnitudes
        for i in 0..len {
            spectrum[i] = Complex::from_polar(magnitudes[i], phases[i]);
        }
    }

    /// Generate harmonic series spectrum
    fn generate_harmonic_spectrum(len: usize, fundamental: f32, hz_per_bin: f32) -> Vec<f32> {
        let mut mags = vec![0.0; len];

        // Generate harmonic series with 1/n falloff
        for harmonic in 1..50 {
            let freq = fundamental * harmonic as f32;
            let bin = (freq / hz_per_bin) as usize;

            if bin < len {
                mags[bin] = 1.0 / harmonic as f32;
            }
        }

        mags
    }

    /// Generate white noise spectrum
    fn generate_noise_spectrum(len: usize) -> Vec<f32> {
        vec![1.0; len]
    }

    /// Generate whisper spectrum (high-pass noise)
    fn generate_whisper_spectrum(len: usize, hz_per_bin: f32) -> Vec<f32> {
        let mut mags = vec![0.0; len];
        let cutoff_freq = 1000.0;

        for i in 0..len {
            let freq = i as f32 * hz_per_bin;
            if freq > cutoff_freq {
                // High-pass with gentle slope
                let ratio = (freq - cutoff_freq) / cutoff_freq;
                mags[i] = ratio.min(1.0);
            }
        }

        mags
    }

    /// Generate robot spectrum (quantized harmonics)
    fn generate_robot_spectrum(len: usize, fundamental: f32, hz_per_bin: f32) -> Vec<f32> {
        let mut mags = vec![0.0; len];

        // Strong fundamental and octaves
        for octave in 0..6 {
            let freq = fundamental * 2.0_f32.powi(octave);
            let bin = (freq / hz_per_bin) as usize;

            if bin < len {
                mags[bin] = 1.0 / (octave + 1) as f32;
            }
        }

        mags
    }

    /// Reset the spectral morph state
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
impl SpectralMorph {
    /// Morph to harmonic series
    pub fn harmonic() -> Self {
        let mut morph = Self::new(2048, 512, WindowType::Hann, 44100.0);
        morph.set_target(MorphTarget::Harmonic);
        morph.set_morph_amount(0.7);
        morph.set_fundamental(110.0);
        morph
    }

    /// Morph to noise
    pub fn noise() -> Self {
        let mut morph = Self::new(2048, 512, WindowType::Hann, 44100.0);
        morph.set_target(MorphTarget::Noise);
        morph.set_morph_amount(0.6);
        morph
    }

    /// Morph to whisper
    pub fn whisper() -> Self {
        let mut morph = Self::new(2048, 512, WindowType::Hann, 44100.0);
        morph.set_target(MorphTarget::Whisper);
        morph.set_morph_amount(0.8);
        morph
    }

    /// Morph to robot
    pub fn robot() -> Self {
        let mut morph = Self::new(2048, 512, WindowType::Hann, 44100.0);
        morph.set_target(MorphTarget::Robot);
        morph.set_morph_amount(0.9);
        morph.set_fundamental(220.0);
        morph
    }
}
