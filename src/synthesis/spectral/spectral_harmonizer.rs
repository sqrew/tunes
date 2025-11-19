//! Spectral harmonizer - add harmonically-related pitch-shifted voices
//!
//! Unlike a simple pitch shifter, the spectral harmonizer adds multiple
//! pitch-shifted copies (voices) of the input signal, creating rich harmonies.
//! Each voice can have independent pitch shift and mix level.
//!
//! Common use cases:
//! - Vocal harmonies (thirds, fifths, octaves)
//! - Thick synth sounds (detuned copies)
//! - Choir effects (multiple voices at different intervals)
//! - Octave doubling

use super::*;
use rustfft::num_complex::Complex;

/// A single harmony voice with pitch shift and mix level
#[derive(Clone, Debug)]
pub struct HarmonyVoice {
    /// Pitch shift in semitones (e.g., 7.0 = perfect fifth up)
    pub semitones: f32,

    /// Mix level for this voice (0.0 = silent, 1.0 = full)
    pub mix: f32,
}

impl HarmonyVoice {
    /// Create a new harmony voice
    pub fn new(semitones: f32, mix: f32) -> Self {
        Self {
            semitones,
            mix: mix.clamp(0.0, 1.0),
        }
    }
}

/// Spectral harmonizer effect
///
/// Adds multiple pitch-shifted voices to create harmonies. Each voice is
/// independently pitch-shifted and mixed, allowing for rich chords and
/// layered textures.
///
/// **Pitch Shift Guide (semitones):**
/// - `3` - Minor third (e.g., C → E♭)
/// - `4` - Major third (e.g., C → E)
/// - `5` - Perfect fourth (e.g., C → F)
/// - `7` - Perfect fifth (e.g., C → G)
/// - `12` - Octave up (e.g., C → C)
/// - `-12` - Octave down
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralHarmonizer, HarmonyVoice, WindowType};
/// let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
///
/// // Add a major third and perfect fifth
/// harmonizer.add_voice(HarmonyVoice::new(4.0, 0.7));  // Major third
/// harmonizer.add_voice(HarmonyVoice::new(7.0, 0.7));  // Perfect fifth
/// harmonizer.set_dry_mix(0.8);  // Original signal
/// ```
#[derive(Clone, Debug)]
pub struct SpectralHarmonizer {
    /// STFT processor
    stft: STFT,

    /// FFT size
    fft_size: usize,

    /// Sample rate
    sample_rate: f32,

    /// Harmony voices
    voices: Vec<HarmonyVoice>,

    /// Dry signal mix (0.0 = no dry, 1.0 = full dry)
    dry_mix: f32,

    /// Effect enabled flag
    enabled: bool,
}

impl SpectralHarmonizer {
    /// Create a new spectral harmonizer
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `window_type` - Window function type
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralHarmonizer, WindowType};
    /// let harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
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
            voices: Vec::new(),
            dry_mix: 1.0,
            enabled: true,
        }
    }

    /// Add a harmony voice
    ///
    /// # Arguments
    /// * `voice` - Harmony voice with pitch shift and mix level
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralHarmonizer, HarmonyVoice, WindowType};
    /// let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
    /// harmonizer.add_voice(HarmonyVoice::new(7.0, 0.8));  // Perfect fifth
    /// ```
    pub fn add_voice(&mut self, voice: HarmonyVoice) {
        self.voices.push(voice);
    }

    /// Clear all harmony voices
    pub fn clear_voices(&mut self) {
        self.voices.clear();
    }

    /// Set voices from a list
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralHarmonizer, HarmonyVoice, WindowType};
    /// let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
    /// harmonizer.set_voices(vec![
    ///     HarmonyVoice::new(4.0, 0.7),  // Major third
    ///     HarmonyVoice::new(7.0, 0.7),  // Perfect fifth
    /// ]);
    /// ```
    pub fn set_voices(&mut self, voices: Vec<HarmonyVoice>) {
        self.voices = voices;
    }

    /// Get reference to current voices
    pub fn voices(&self) -> &[HarmonyVoice] {
        &self.voices
    }

    /// Set dry signal mix level
    ///
    /// # Arguments
    /// * `mix` - Dry mix amount (0.0 = no dry, 1.0 = full dry)
    pub fn set_dry_mix(&mut self, mix: f32) {
        self.dry_mix = mix.clamp(0.0, 1.0);
    }

    /// Get current dry mix level
    pub fn dry_mix(&self) -> f32 {
        self.dry_mix
    }

    /// Process audio through the spectral harmonizer
    ///
    /// # Arguments
    /// * `output` - Output buffer (will be filled with processed audio)
    /// * `input` - Input audio buffer
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralHarmonizer, HarmonyVoice, WindowType};
    /// let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
    /// harmonizer.add_voice(HarmonyVoice::new(7.0, 0.8));
    ///
    /// let input = vec![0.0; 512];
    /// let mut output = vec![0.0; 512];
    /// harmonizer.process(&mut output, &input);
    /// ```
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        if !self.enabled || self.voices.is_empty() {
            return;
        }

        let voices = self.voices.clone(); // Clone to avoid borrow issues
        let dry_mix = self.dry_mix;
        let sample_rate = self.sample_rate;

        self.stft.process(output, |spectrum| {
            Self::apply_harmonizer_static(spectrum, &voices, dry_mix, sample_rate);
        });
    }

    /// Apply harmonizer (static version for closure)
    #[inline]
    fn apply_harmonizer_static(
        spectrum: &mut [Complex<f32>],
        voices: &[HarmonyVoice],
        dry_mix: f32,
        _sample_rate: f32,
    ) {
        let len = spectrum.len();

        // Store original spectrum for dry signal
        let dry_spectrum: Vec<Complex<f32>> = spectrum.to_vec();

        // Clear output spectrum
        for bin in spectrum.iter_mut() {
            *bin = Complex::new(0.0, 0.0);
        }

        // Add dry signal
        if dry_mix > 0.0 {
            for i in 0..len {
                spectrum[i] = dry_spectrum[i] * dry_mix;
            }
        }

        // Add each harmony voice
        for voice in voices {
            if voice.mix <= 0.0 {
                continue;
            }

            // Calculate pitch shift ratio from semitones
            let pitch_ratio = 2.0f32.powf(voice.semitones / 12.0);

            // Create pitch-shifted spectrum
            let mut shifted = vec![Complex::new(0.0, 0.0); len];

            // Pitch shift by resampling frequency bins
            for (i, shifted_bin) in shifted.iter_mut().enumerate().take(len) {
                let src_bin = i as f32 / pitch_ratio;

                if src_bin >= 0.0 && src_bin < (len - 1) as f32 {
                    // Linear interpolation between adjacent bins
                    let bin_floor = src_bin.floor() as usize;
                    let bin_ceil = (bin_floor + 1).min(len - 1);
                    let frac = src_bin - bin_floor as f32;

                    // Interpolate magnitude
                    let mag_floor = dry_spectrum[bin_floor].norm();
                    let mag_ceil = dry_spectrum[bin_ceil].norm();
                    let mag = mag_floor * (1.0 - frac) + mag_ceil * frac;

                    // Interpolate phase with wrapping
                    let phase_floor = dry_spectrum[bin_floor].arg();
                    let phase_ceil = dry_spectrum[bin_ceil].arg();
                    let mut phase_diff = phase_ceil - phase_floor;

                    // Handle phase wrapping
                    if phase_diff > std::f32::consts::PI {
                        phase_diff -= 2.0 * std::f32::consts::PI;
                    } else if phase_diff < -std::f32::consts::PI {
                        phase_diff += 2.0 * std::f32::consts::PI;
                    }

                    let phase = phase_floor + phase_diff * frac;

                    *shifted_bin = Complex::from_polar(mag, phase);
                }
            }

            // Mix this voice into output
            for i in 0..len {
                spectrum[i] += shifted[i] * voice.mix;
            }
        }
    }

    /// Reset the harmonizer state
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
impl SpectralHarmonizer {
    /// Major chord (major third + perfect fifth)
    pub fn major_chord() -> Self {
        let mut harmonizer = Self::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.set_voices(vec![
            HarmonyVoice::new(4.0, 0.7),  // Major third
            HarmonyVoice::new(7.0, 0.7),  // Perfect fifth
        ]);
        harmonizer.set_dry_mix(0.8);
        harmonizer
    }

    /// Minor chord (minor third + perfect fifth)
    pub fn minor_chord() -> Self {
        let mut harmonizer = Self::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.set_voices(vec![
            HarmonyVoice::new(3.0, 0.7),  // Minor third
            HarmonyVoice::new(7.0, 0.7),  // Perfect fifth
        ]);
        harmonizer.set_dry_mix(0.8);
        harmonizer
    }

    /// Perfect fifth harmony
    pub fn fifth() -> Self {
        let mut harmonizer = Self::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.add_voice(HarmonyVoice::new(7.0, 0.8));
        harmonizer.set_dry_mix(0.9);
        harmonizer
    }

    /// Octave up
    pub fn octave_up() -> Self {
        let mut harmonizer = Self::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.add_voice(HarmonyVoice::new(12.0, 0.7));
        harmonizer.set_dry_mix(1.0);
        harmonizer
    }

    /// Octave down
    pub fn octave_down() -> Self {
        let mut harmonizer = Self::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.add_voice(HarmonyVoice::new(-12.0, 0.7));
        harmonizer.set_dry_mix(1.0);
        harmonizer
    }

    /// Octave doubling (both up and down)
    pub fn octave_double() -> Self {
        let mut harmonizer = Self::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.set_voices(vec![
            HarmonyVoice::new(-12.0, 0.5),  // Octave down
            HarmonyVoice::new(12.0, 0.5),   // Octave up
        ]);
        harmonizer.set_dry_mix(1.0);
        harmonizer
    }

    /// Choir effect (multiple voices)
    pub fn choir() -> Self {
        let mut harmonizer = Self::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.set_voices(vec![
            HarmonyVoice::new(-12.0, 0.4),  // Bass
            HarmonyVoice::new(3.0, 0.5),    // Alto (minor third)
            HarmonyVoice::new(7.0, 0.5),    // Tenor (fifth)
            HarmonyVoice::new(12.0, 0.4),   // Soprano (octave)
        ]);
        harmonizer.set_dry_mix(0.7);
        harmonizer
    }

    /// Barbershop quartet (tight 4-part harmony)
    pub fn barbershop() -> Self {
        let mut harmonizer = Self::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.set_voices(vec![
            HarmonyVoice::new(-12.0, 0.6),  // Bass
            HarmonyVoice::new(-5.0, 0.6),   // Baritone (fourth down)
            HarmonyVoice::new(4.0, 0.6),    // Tenor (major third)
            HarmonyVoice::new(12.0, 0.5),   // Lead (octave)
        ]);
        harmonizer.set_dry_mix(0.8);
        harmonizer
    }

    /// Thick synth (slight detuning)
    pub fn thick() -> Self {
        let mut harmonizer = Self::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.set_voices(vec![
            HarmonyVoice::new(-0.1, 0.6),   // Slightly flat
            HarmonyVoice::new(0.1, 0.6),    // Slightly sharp
        ]);
        harmonizer.set_dry_mix(1.0);
        harmonizer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_harmonizer_creation() {
        let harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
        assert_eq!(harmonizer.fft_size(), 2048);
        assert_eq!(harmonizer.hop_size(), 512);
        assert_eq!(harmonizer.dry_mix(), 1.0);
        assert_eq!(harmonizer.voices().len(), 0);
        assert!(harmonizer.is_enabled());
    }

    #[test]
    #[should_panic(expected = "FFT size must be power of 2")]
    fn test_spectral_harmonizer_requires_power_of_two() {
        SpectralHarmonizer::new(1000, 250, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Hop size must be <= FFT size")]
    fn test_spectral_harmonizer_hop_validation() {
        SpectralHarmonizer::new(1024, 2048, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Sample rate must be positive")]
    fn test_spectral_harmonizer_sample_rate_validation() {
        SpectralHarmonizer::new(1024, 256, WindowType::Hann, 0.0);
    }

    #[test]
    fn test_harmony_voice_creation() {
        let voice = HarmonyVoice::new(7.0, 0.8);
        assert_eq!(voice.semitones, 7.0);
        assert_eq!(voice.mix, 0.8);
    }

    #[test]
    fn test_harmony_voice_mix_clamps() {
        let voice = HarmonyVoice::new(7.0, 1.5);
        assert_eq!(voice.mix, 1.0);

        let voice = HarmonyVoice::new(7.0, -0.5);
        assert_eq!(voice.mix, 0.0);
    }

    #[test]
    fn test_add_voice() {
        let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
        assert_eq!(harmonizer.voices().len(), 0);

        harmonizer.add_voice(HarmonyVoice::new(7.0, 0.8));
        assert_eq!(harmonizer.voices().len(), 1);
        assert_eq!(harmonizer.voices()[0].semitones, 7.0);
    }

    #[test]
    fn test_clear_voices() {
        let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.add_voice(HarmonyVoice::new(7.0, 0.8));
        harmonizer.add_voice(HarmonyVoice::new(4.0, 0.7));
        assert_eq!(harmonizer.voices().len(), 2);

        harmonizer.clear_voices();
        assert_eq!(harmonizer.voices().len(), 0);
    }

    #[test]
    fn test_set_voices() {
        let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.set_voices(vec![
            HarmonyVoice::new(4.0, 0.7),
            HarmonyVoice::new(7.0, 0.8),
        ]);
        assert_eq!(harmonizer.voices().len(), 2);
        assert_eq!(harmonizer.voices()[0].semitones, 4.0);
        assert_eq!(harmonizer.voices()[1].semitones, 7.0);
    }

    #[test]
    fn test_set_dry_mix() {
        let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);

        harmonizer.set_dry_mix(0.5);
        assert_eq!(harmonizer.dry_mix(), 0.5);

        harmonizer.set_dry_mix(1.5);
        assert_eq!(harmonizer.dry_mix(), 1.0);

        harmonizer.set_dry_mix(-0.5);
        assert_eq!(harmonizer.dry_mix(), 0.0);
    }

    #[test]
    fn test_enable_disable() {
        let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
        assert!(harmonizer.is_enabled());

        harmonizer.set_enabled(false);
        assert!(!harmonizer.is_enabled());

        harmonizer.set_enabled(true);
        assert!(harmonizer.is_enabled());
    }

    #[test]
    fn test_process_doesnt_crash() {
        let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.add_voice(HarmonyVoice::new(7.0, 0.8));

        let mut output = vec![0.0; 512];
        let input = vec![0.5; 512];

        harmonizer.process(&mut output, &input);
    }

    #[test]
    fn test_process_disabled() {
        let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.add_voice(HarmonyVoice::new(7.0, 0.8));
        harmonizer.set_enabled(false);

        let mut output = vec![1.0; 512];
        let input = vec![0.5; 512];

        harmonizer.process(&mut output, &input);

        // When disabled, output should remain unchanged
        for sample in output.iter() {
            assert_eq!(*sample, 1.0);
        }
    }

    #[test]
    fn test_process_no_voices() {
        let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);

        let mut output = vec![1.0; 512];
        let input = vec![0.5; 512];

        harmonizer.process(&mut output, &input);

        // With no voices, output should remain unchanged
        for sample in output.iter() {
            assert_eq!(*sample, 1.0);
        }
    }

    #[test]
    fn test_reset() {
        let mut harmonizer = SpectralHarmonizer::new(2048, 512, WindowType::Hann, 44100.0);
        harmonizer.add_voice(HarmonyVoice::new(7.0, 0.8));

        let mut output = vec![0.0; 512];
        let input = vec![0.5; 512];

        harmonizer.process(&mut output, &input);
        harmonizer.reset();
        harmonizer.process(&mut output, &input);
    }

    // Preset tests
    #[test]
    fn test_major_chord_preset() {
        let harmonizer = SpectralHarmonizer::major_chord();
        assert_eq!(harmonizer.voices().len(), 2);
        assert_eq!(harmonizer.voices()[0].semitones, 4.0);
        assert_eq!(harmonizer.voices()[1].semitones, 7.0);
        assert_eq!(harmonizer.dry_mix(), 0.8);
    }

    #[test]
    fn test_minor_chord_preset() {
        let harmonizer = SpectralHarmonizer::minor_chord();
        assert_eq!(harmonizer.voices().len(), 2);
        assert_eq!(harmonizer.voices()[0].semitones, 3.0);
        assert_eq!(harmonizer.voices()[1].semitones, 7.0);
    }

    #[test]
    fn test_fifth_preset() {
        let harmonizer = SpectralHarmonizer::fifth();
        assert_eq!(harmonizer.voices().len(), 1);
        assert_eq!(harmonizer.voices()[0].semitones, 7.0);
    }

    #[test]
    fn test_octave_up_preset() {
        let harmonizer = SpectralHarmonizer::octave_up();
        assert_eq!(harmonizer.voices().len(), 1);
        assert_eq!(harmonizer.voices()[0].semitones, 12.0);
    }

    #[test]
    fn test_octave_down_preset() {
        let harmonizer = SpectralHarmonizer::octave_down();
        assert_eq!(harmonizer.voices().len(), 1);
        assert_eq!(harmonizer.voices()[0].semitones, -12.0);
    }

    #[test]
    fn test_octave_double_preset() {
        let harmonizer = SpectralHarmonizer::octave_double();
        assert_eq!(harmonizer.voices().len(), 2);
        assert_eq!(harmonizer.voices()[0].semitones, -12.0);
        assert_eq!(harmonizer.voices()[1].semitones, 12.0);
    }

    #[test]
    fn test_choir_preset() {
        let harmonizer = SpectralHarmonizer::choir();
        assert_eq!(harmonizer.voices().len(), 4);
    }

    #[test]
    fn test_barbershop_preset() {
        let harmonizer = SpectralHarmonizer::barbershop();
        assert_eq!(harmonizer.voices().len(), 4);
    }

    #[test]
    fn test_thick_preset() {
        let harmonizer = SpectralHarmonizer::thick();
        assert_eq!(harmonizer.voices().len(), 2);
        assert!(harmonizer.voices()[0].semitones.abs() < 1.0);
        assert!(harmonizer.voices()[1].semitones.abs() < 1.0);
    }
}
