//! Spectral resonator - emphasize specific frequencies or harmonic series
//!
//! Creates resonant peaks at specified frequencies, similar to a bank of
//! very narrow bandpass filters. Perfect for pitched resonance effects,
//! formant-like sounds, bell timbres, and harmonic enhancement.
//!
//! Unlike traditional resonators, this operates in the frequency domain
//! for precise control over individual frequency bins.

use super::*;
use rustfft::num_complex::Complex;

/// A single resonance peak
#[derive(Clone, Debug)]
pub struct Resonance {
    /// Center frequency in Hz
    pub frequency: f32,

    /// Gain multiplier for this resonance (1.0 = unity, >1.0 = boost)
    pub gain: f32,

    /// Q factor (bandwidth) - higher = narrower peak (typical: 1.0-50.0)
    pub q: f32,
}

impl Resonance {
    /// Create a new resonance peak
    ///
    /// # Arguments
    /// * `frequency` - Center frequency in Hz
    /// * `gain` - Gain multiplier (e.g., 2.0 = +6dB, 10.0 = +20dB)
    /// * `q` - Q factor / sharpness (1.0 = wide, 50.0 = very narrow)
    pub fn new(frequency: f32, gain: f32, q: f32) -> Self {
        Self {
            frequency,
            gain: gain.max(0.0),
            q: q.max(0.1),
        }
    }
}

/// Spectral resonator effect
///
/// Boosts specific frequencies with narrow peaks, creating resonant,
/// ringing timbres. Can operate in harmonic series mode (fundamental +
/// overtones) or with custom arbitrary frequencies.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralResonator, Resonance, WindowType};
/// let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
///
/// // Add resonances at 440Hz and 880Hz
/// resonator.add_resonance(Resonance::new(440.0, 3.0, 10.0));
/// resonator.add_resonance(Resonance::new(880.0, 2.0, 10.0));
/// ```
#[derive(Clone, Debug)]
pub struct SpectralResonator {
    /// STFT processor
    stft: STFT,

    /// FFT size
    fft_size: usize,

    /// Sample rate
    sample_rate: f32,

    /// List of resonance peaks
    resonances: Vec<Resonance>,

    /// Wet/dry mix (0.0 = dry, 1.0 = wet)
    mix: f32,

    /// Effect enabled flag
    enabled: bool,
}

impl SpectralResonator {
    /// Create a new spectral resonator
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `window_type` - Window function type
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralResonator, WindowType};
    /// let resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
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
            resonances: Vec::new(),
            mix: 1.0,
            enabled: true,
        }
    }

    /// Add a resonance peak
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralResonator, Resonance, WindowType};
    /// let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
    /// resonator.add_resonance(Resonance::new(440.0, 5.0, 20.0));
    /// ```
    pub fn add_resonance(&mut self, resonance: Resonance) {
        self.resonances.push(resonance);
    }

    /// Clear all resonances
    pub fn clear_resonances(&mut self) {
        self.resonances.clear();
    }

    /// Set resonances from a list
    pub fn set_resonances(&mut self, resonances: Vec<Resonance>) {
        self.resonances = resonances;
    }

    /// Get reference to current resonances
    pub fn resonances(&self) -> &[Resonance] {
        &self.resonances
    }

    /// Set harmonic series resonances
    ///
    /// Creates resonances at fundamental frequency and its harmonics.
    ///
    /// # Arguments
    /// * `fundamental` - Fundamental frequency in Hz
    /// * `num_harmonics` - Number of harmonics to create (including fundamental)
    /// * `gain` - Gain for each resonance
    /// * `q` - Q factor for resonance sharpness
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralResonator, WindowType};
    /// let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
    /// resonator.set_harmonic_series(440.0, 8, 3.0, 15.0);  // A4 + 7 overtones
    /// ```
    pub fn set_harmonic_series(&mut self, fundamental: f32, num_harmonics: usize, gain: f32, q: f32) {
        self.resonances.clear();
        for n in 1..=num_harmonics {
            let freq = fundamental * n as f32;
            // Reduce gain for higher harmonics (natural behavior)
            let harmonic_gain = gain / (n as f32).sqrt();
            self.resonances.push(Resonance::new(freq, harmonic_gain, q));
        }
    }

    /// Set wet/dry mix
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = dry, 1.0 = wet)
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Get current mix level
    pub fn mix(&self) -> f32 {
        self.mix
    }

    /// Process audio through the spectral resonator
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralResonator, Resonance, WindowType};
    /// let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
    /// resonator.add_resonance(Resonance::new(440.0, 5.0, 20.0));
    ///
    /// let input = vec![0.0; 512];
    /// let mut output = vec![0.0; 512];
    /// resonator.process(&mut output, &input);
    /// ```
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        if !self.enabled || self.resonances.is_empty() {
            return;
        }

        let resonances = self.resonances.clone();
        let mix = self.mix;
        let sample_rate = self.sample_rate;
        let fft_size = self.fft_size;

        self.stft.process(output, |spectrum| {
            Self::apply_resonator_static(spectrum, &resonances, mix, sample_rate, fft_size);
        });
    }

    /// Apply resonator (static version for closure)
    #[inline]
    fn apply_resonator_static(
        spectrum: &mut [Complex<f32>],
        resonances: &[Resonance],
        mix: f32,
        sample_rate: f32,
        fft_size: usize,
    ) {
        let len = spectrum.len();
        let hz_per_bin = sample_rate / fft_size as f32;

        // Store original spectrum for dry/wet mixing
        let dry_spectrum: Vec<Complex<f32>> = spectrum.to_vec();

        // Calculate gain multiplier for each bin
        let mut gain_curve = vec![1.0f32; len];

        for resonance in resonances {
            let center_bin = resonance.frequency / hz_per_bin;

            // Apply bell-shaped gain curve around resonance frequency
            for (i, gain) in gain_curve.iter_mut().enumerate() {
                let bin_distance = (i as f32 - center_bin).abs();

                // Gaussian-like resonance curve based on Q factor
                // Higher Q = narrower peak
                let bandwidth = 1.0 / resonance.q;
                let curve = (-((bin_distance * bandwidth).powi(2))).exp();

                // Add to gain curve (allowing multiple resonances to stack)
                *gain += curve * (resonance.gain - 1.0);
            }
        }

        // Apply gain curve to spectrum
        for (spec, &gain) in spectrum.iter_mut().zip(gain_curve.iter()) {
            *spec *= gain;
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

    /// Reset the resonator state
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
impl SpectralResonator {
    /// Pitched resonance at 440Hz (A4) with harmonics
    pub fn pitched_a440() -> Self {
        let mut resonator = Self::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_harmonic_series(440.0, 8, 4.0, 15.0);
        resonator.set_mix(0.8);
        resonator
    }

    /// Bell-like timbre with strong harmonics
    pub fn bell() -> Self {
        let mut resonator = Self::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_harmonic_series(523.25, 12, 6.0, 25.0);  // C5
        resonator.set_mix(0.9);
        resonator
    }

    /// Metallic timbre with inharmonic partials
    pub fn metallic() -> Self {
        let mut resonator = Self::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_resonances(vec![
            Resonance::new(440.0, 5.0, 20.0),
            Resonance::new(667.0, 4.0, 18.0),   // Not a harmonic!
            Resonance::new(1021.0, 3.5, 15.0),  // Inharmonic
            Resonance::new(1543.0, 3.0, 12.0),
        ]);
        resonator.set_mix(0.85);
        resonator
    }

    /// Vocal formants (simulates vowel 'a' as in "father")
    pub fn vowel_a() -> Self {
        let mut resonator = Self::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_resonances(vec![
            Resonance::new(730.0, 8.0, 5.0),   // F1
            Resonance::new(1090.0, 6.0, 8.0),  // F2
            Resonance::new(2440.0, 4.0, 10.0), // F3
        ]);
        resonator.set_mix(0.7);
        resonator
    }

    /// Vocal formants (simulates vowel 'e' as in "beet")
    pub fn vowel_e() -> Self {
        let mut resonator = Self::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_resonances(vec![
            Resonance::new(270.0, 8.0, 5.0),   // F1
            Resonance::new(2290.0, 6.0, 8.0),  // F2
            Resonance::new(3010.0, 4.0, 10.0), // F3
        ]);
        resonator.set_mix(0.7);
        resonator
    }

    /// Vocal formants (simulates vowel 'i' as in "beat")
    pub fn vowel_i() -> Self {
        let mut resonator = Self::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_resonances(vec![
            Resonance::new(300.0, 8.0, 5.0),   // F1
            Resonance::new(2310.0, 6.0, 8.0),  // F2
            Resonance::new(3200.0, 4.0, 10.0), // F3
        ]);
        resonator.set_mix(0.7);
        resonator
    }

    /// Vocal formants (simulates vowel 'o' as in "boat")
    pub fn vowel_o() -> Self {
        let mut resonator = Self::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_resonances(vec![
            Resonance::new(570.0, 8.0, 5.0),   // F1
            Resonance::new(840.0, 6.0, 8.0),   // F2
            Resonance::new(2410.0, 4.0, 10.0), // F3
        ]);
        resonator.set_mix(0.7);
        resonator
    }

    /// Vocal formants (simulates vowel 'u' as in "boot")
    pub fn vowel_u() -> Self {
        let mut resonator = Self::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_resonances(vec![
            Resonance::new(300.0, 8.0, 5.0),   // F1
            Resonance::new(870.0, 6.0, 8.0),   // F2
            Resonance::new(2240.0, 4.0, 10.0), // F3
        ]);
        resonator.set_mix(0.7);
        resonator
    }

    /// Gentle harmonic enhancement
    pub fn gentle() -> Self {
        let mut resonator = Self::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_harmonic_series(220.0, 6, 2.0, 8.0);
        resonator.set_mix(0.4);
        resonator
    }

    /// Comb filter effect
    pub fn comb() -> Self {
        let mut resonator = Self::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_harmonic_series(100.0, 20, 3.0, 30.0);
        resonator.set_mix(1.0);
        resonator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_resonator_creation() {
        let resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
        assert_eq!(resonator.fft_size(), 2048);
        assert_eq!(resonator.hop_size(), 512);
        assert_eq!(resonator.mix(), 1.0);
        assert_eq!(resonator.resonances().len(), 0);
        assert!(resonator.is_enabled());
    }

    #[test]
    #[should_panic(expected = "FFT size must be power of 2")]
    fn test_spectral_resonator_requires_power_of_two() {
        SpectralResonator::new(1000, 250, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Hop size must be <= FFT size")]
    fn test_spectral_resonator_hop_validation() {
        SpectralResonator::new(1024, 2048, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Sample rate must be positive")]
    fn test_spectral_resonator_sample_rate_validation() {
        SpectralResonator::new(1024, 256, WindowType::Hann, 0.0);
    }

    #[test]
    fn test_resonance_creation() {
        let res = Resonance::new(440.0, 5.0, 10.0);
        assert_eq!(res.frequency, 440.0);
        assert_eq!(res.gain, 5.0);
        assert_eq!(res.q, 10.0);
    }

    #[test]
    fn test_resonance_gain_clamps_negative() {
        let res = Resonance::new(440.0, -5.0, 10.0);
        assert_eq!(res.gain, 0.0);
    }

    #[test]
    fn test_resonance_q_clamps_minimum() {
        let res = Resonance::new(440.0, 5.0, 0.05);
        assert_eq!(res.q, 0.1);
    }

    #[test]
    fn test_add_resonance() {
        let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
        assert_eq!(resonator.resonances().len(), 0);

        resonator.add_resonance(Resonance::new(440.0, 5.0, 10.0));
        assert_eq!(resonator.resonances().len(), 1);
        assert_eq!(resonator.resonances()[0].frequency, 440.0);
    }

    #[test]
    fn test_clear_resonances() {
        let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.add_resonance(Resonance::new(440.0, 5.0, 10.0));
        resonator.add_resonance(Resonance::new(880.0, 4.0, 8.0));
        assert_eq!(resonator.resonances().len(), 2);

        resonator.clear_resonances();
        assert_eq!(resonator.resonances().len(), 0);
    }

    #[test]
    fn test_set_resonances() {
        let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_resonances(vec![
            Resonance::new(440.0, 5.0, 10.0),
            Resonance::new(880.0, 4.0, 8.0),
        ]);
        assert_eq!(resonator.resonances().len(), 2);
    }

    #[test]
    fn test_set_harmonic_series() {
        let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.set_harmonic_series(440.0, 4, 5.0, 10.0);

        assert_eq!(resonator.resonances().len(), 4);
        assert_eq!(resonator.resonances()[0].frequency, 440.0);   // 1st harmonic
        assert_eq!(resonator.resonances()[1].frequency, 880.0);   // 2nd harmonic
        assert_eq!(resonator.resonances()[2].frequency, 1320.0);  // 3rd harmonic
        assert_eq!(resonator.resonances()[3].frequency, 1760.0);  // 4th harmonic

        // Gain should decrease for higher harmonics
        assert!(resonator.resonances()[0].gain > resonator.resonances()[1].gain);
    }

    #[test]
    fn test_set_mix() {
        let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);

        resonator.set_mix(0.5);
        assert_eq!(resonator.mix(), 0.5);

        resonator.set_mix(1.5);
        assert_eq!(resonator.mix(), 1.0);

        resonator.set_mix(-0.5);
        assert_eq!(resonator.mix(), 0.0);
    }

    #[test]
    fn test_enable_disable() {
        let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
        assert!(resonator.is_enabled());

        resonator.set_enabled(false);
        assert!(!resonator.is_enabled());

        resonator.set_enabled(true);
        assert!(resonator.is_enabled());
    }

    #[test]
    fn test_process_doesnt_crash() {
        let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.add_resonance(Resonance::new(440.0, 5.0, 10.0));

        let mut output = vec![0.0; 512];
        let input = vec![0.5; 512];

        resonator.process(&mut output, &input);
    }

    #[test]
    fn test_process_disabled() {
        let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.add_resonance(Resonance::new(440.0, 5.0, 10.0));
        resonator.set_enabled(false);

        let mut output = vec![1.0; 512];
        let input = vec![0.5; 512];

        resonator.process(&mut output, &input);

        // When disabled, output should remain unchanged
        for sample in output.iter() {
            assert_eq!(*sample, 1.0);
        }
    }

    #[test]
    fn test_process_no_resonances() {
        let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);

        let mut output = vec![1.0; 512];
        let input = vec![0.5; 512];

        resonator.process(&mut output, &input);

        // With no resonances, output should remain unchanged
        for sample in output.iter() {
            assert_eq!(*sample, 1.0);
        }
    }

    #[test]
    fn test_reset() {
        let mut resonator = SpectralResonator::new(2048, 512, WindowType::Hann, 44100.0);
        resonator.add_resonance(Resonance::new(440.0, 5.0, 10.0));

        let mut output = vec![0.0; 512];
        let input = vec![0.5; 512];

        resonator.process(&mut output, &input);
        resonator.reset();
        resonator.process(&mut output, &input);
    }

    // Preset tests
    #[test]
    fn test_pitched_a440_preset() {
        let resonator = SpectralResonator::pitched_a440();
        assert_eq!(resonator.resonances().len(), 8);
        assert_eq!(resonator.resonances()[0].frequency, 440.0);
    }

    #[test]
    fn test_bell_preset() {
        let resonator = SpectralResonator::bell();
        assert_eq!(resonator.resonances().len(), 12);
    }

    #[test]
    fn test_metallic_preset() {
        let resonator = SpectralResonator::metallic();
        assert_eq!(resonator.resonances().len(), 4);
    }

    #[test]
    fn test_vowel_presets() {
        let vowel_a = SpectralResonator::vowel_a();
        assert_eq!(vowel_a.resonances().len(), 3);

        let vowel_e = SpectralResonator::vowel_e();
        assert_eq!(vowel_e.resonances().len(), 3);

        let vowel_i = SpectralResonator::vowel_i();
        assert_eq!(vowel_i.resonances().len(), 3);

        let vowel_o = SpectralResonator::vowel_o();
        assert_eq!(vowel_o.resonances().len(), 3);

        let vowel_u = SpectralResonator::vowel_u();
        assert_eq!(vowel_u.resonances().len(), 3);
    }

    #[test]
    fn test_gentle_preset() {
        let resonator = SpectralResonator::gentle();
        assert_eq!(resonator.resonances().len(), 6);
        assert!(resonator.mix() < 0.5);  // Gentle should have low mix
    }

    #[test]
    fn test_comb_preset() {
        let resonator = SpectralResonator::comb();
        assert_eq!(resonator.resonances().len(), 20);
        assert_eq!(resonator.mix(), 1.0);
    }
}
