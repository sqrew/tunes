//! Spectral gate - frequency-selective noise gate

use super::*;
use rustfft::num_complex::Complex;

/// Spectral gate - frequency-selective noise gate with per-bin gating
///
/// Unlike traditional gates that gate the entire signal, SpectralGate applies
/// independent gating to each frequency bin. This enables surgical noise reduction,
/// removing hum, hiss, and unwanted frequencies while preserving the wanted signal.
///
/// Uses SIMD-accelerated STFT for efficient real-time processing.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{SpectralGate, WindowType};
/// // Create spectral gate with 2048 FFT, 512 hop
/// let mut gate = SpectralGate::new(2048, 512, WindowType::Hann, 44100.0);
///
/// // Set threshold to -40 dB (bins below this get gated)
/// gate.set_threshold(-40.0);
///
/// // Fast attack, medium release
/// gate.set_attack(0.001);  // 1ms
/// gate.set_release(0.050); // 50ms
///
/// // Process audio - removes noise below -40 dB per frequency bin
/// let input = vec![0.0; 512];
/// let mut output = vec![0.0; 512];
/// gate.process(&mut output, &input);
/// ```
#[derive(Clone)]
pub struct SpectralGate {
    /// STFT processor for analysis/synthesis
    stft: STFT,

    /// FFT size
    fft_size: usize,

    /// Sample rate
    sample_rate: f32,

    /// Threshold in dB (bins below this get gated)
    threshold_db: f32,

    /// Attack time in seconds (how fast gate opens)
    attack: f32,

    /// Release time in seconds (how fast gate closes)
    release: f32,

    /// Gate ratio (0.0 = full gate/mute, 1.0 = no gating)
    ratio: f32,

    /// Attack coefficient (pre-calculated from attack time)
    attack_coeff: f32,

    /// Release coefficient (pre-calculated from release time)
    release_coeff: f32,

    /// Per-bin envelope state (for attack/release smoothing)
    envelope: Vec<f32>,

    /// Gate enabled flag
    enabled: bool,
}

impl SpectralGate {
    /// Create a new spectral gate
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `window_type` - Window function type
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralGate, WindowType};
    /// let gate = SpectralGate::new(2048, 512, WindowType::Hann, 44100.0);
    /// ```
    pub fn new(
        fft_size: usize,
        hop_size: usize,
        window_type: WindowType,
        sample_rate: f32,
    ) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(hop_size <= fft_size, "Hop size must be <= FFT size");
        assert!(sample_rate > 0.0, "Sample rate must be positive");

        let stft = STFT::new(fft_size, hop_size, window_type);

        // Default attack/release times
        let attack = 0.001; // 1ms
        let release = 0.050; // 50ms

        let mut gate = Self {
            stft,
            fft_size,
            sample_rate,
            threshold_db: -40.0, // Default threshold
            attack,
            release,
            ratio: 0.0, // Full gate by default
            attack_coeff: 0.0,
            release_coeff: 0.0,
            envelope: vec![0.0; fft_size],
            enabled: true,
        };

        // Calculate attack/release coefficients
        gate.update_coefficients();
        gate
    }

    /// Set threshold in dB
    ///
    /// Frequency bins below this threshold will be gated.
    ///
    /// # Arguments
    /// * `threshold_db` - Threshold in dB (typically -60 to -20)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralGate, WindowType};
    /// let mut gate = SpectralGate::new(2048, 512, WindowType::Hann, 44100.0);
    /// gate.set_threshold(-40.0);  // Gate bins below -40 dB
    /// ```
    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.threshold_db = threshold_db;
    }

    /// Set attack time in seconds
    ///
    /// Controls how quickly the gate opens when signal exceeds threshold.
    ///
    /// # Arguments
    /// * `attack` - Attack time in seconds (typically 0.001 to 0.1)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralGate, WindowType};
    /// let mut gate = SpectralGate::new(2048, 512, WindowType::Hann, 44100.0);
    /// gate.set_attack(0.001);  // 1ms - very fast
    /// ```
    pub fn set_attack(&mut self, attack: f32) {
        self.attack = attack.max(0.0001); // Minimum 0.1ms
        self.update_coefficients();
    }

    /// Set release time in seconds
    ///
    /// Controls how quickly the gate closes when signal drops below threshold.
    ///
    /// # Arguments
    /// * `release` - Release time in seconds (typically 0.01 to 0.5)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralGate, WindowType};
    /// let mut gate = SpectralGate::new(2048, 512, WindowType::Hann, 44100.0);
    /// gate.set_release(0.050);  // 50ms
    /// ```
    pub fn set_release(&mut self, release: f32) {
        self.release = release.max(0.001); // Minimum 1ms
        self.update_coefficients();
    }

    /// Set gate ratio
    ///
    /// Controls the depth of gating. 0.0 = full mute, 1.0 = no gating.
    ///
    /// # Arguments
    /// * `ratio` - Gate ratio (0.0 to 1.0)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralGate, WindowType};
    /// let mut gate = SpectralGate::new(2048, 512, WindowType::Hann, 44100.0);
    /// gate.set_ratio(0.1);  // Reduce to 10% when gated (90% reduction)
    /// ```
    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio.clamp(0.0, 1.0);
    }

    /// Get current threshold
    pub fn threshold(&self) -> f32 {
        self.threshold_db
    }

    /// Get current attack time
    pub fn attack(&self) -> f32 {
        self.attack
    }

    /// Get current release time
    pub fn release(&self) -> f32 {
        self.release
    }

    /// Get current ratio
    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    /// Update attack/release coefficients based on sample rate and hop size
    fn update_coefficients(&mut self) {
        let hop_time = self.stft.hop_size() as f32 / self.sample_rate;

        // Exponential envelope coefficients
        self.attack_coeff = (-hop_time / self.attack).exp();
        self.release_coeff = (-hop_time / self.release).exp();
    }

    /// Process audio through the spectral gate
    ///
    /// # Arguments
    /// * `output` - Output buffer (will be filled with processed audio)
    /// * `input` - Input audio buffer
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{SpectralGate, WindowType};
    /// let mut gate = SpectralGate::new(2048, 512, WindowType::Hann, 44100.0);
    /// gate.set_threshold(-40.0);
    ///
    /// let input = vec![0.0; 512];
    /// let mut output = vec![0.0; 512];
    /// gate.process(&mut output, &input);
    /// ```
    pub fn process(&mut self, output: &mut [f32], _input: &[f32]) {
        if !self.enabled {
            return;
        }

        let threshold_db = self.threshold_db;
        let ratio = self.ratio;
        let attack_coeff = self.attack_coeff;
        let release_coeff = self.release_coeff;
        let envelope = &mut self.envelope;

        self.stft.process(output, |spectrum| {
            Self::apply_gate_static(
                spectrum,
                envelope,
                threshold_db,
                ratio,
                attack_coeff,
                release_coeff,
            );
        });
    }

    /// Apply gating to spectrum (static version for closure)
    #[inline]
    fn apply_gate_static(
        spectrum: &mut [Complex<f32>],
        envelope: &mut [f32],
        threshold_db: f32,
        ratio: f32,
        attack_coeff: f32,
        release_coeff: f32,
    ) {
        let len = spectrum.len();

        // Calculate magnitudes using SIMD
        let mut magnitudes = vec![0.0; len];
        ComplexOps::magnitude(&mut magnitudes, spectrum);

        // Convert to dB and apply gating per bin
        for i in 0..len {
            // Convert magnitude to dB (with floor to avoid log(0))
            let mag_db = if magnitudes[i] > 1e-10 {
                20.0 * magnitudes[i].log10()
            } else {
                -100.0 // Floor at -100 dB
            };

            // Determine target gain (0.0 = gate closed, 1.0 = gate open)
            let target_gain = if mag_db >= threshold_db {
                1.0 // Above threshold: gate open
            } else {
                ratio // Below threshold: apply ratio (0.0 = full gate)
            };

            // Apply attack/release envelope smoothing
            let current_env = envelope[i];
            let coeff = if target_gain > current_env {
                attack_coeff // Opening gate (attack)
            } else {
                release_coeff // Closing gate (release)
            };

            // Exponential smoothing
            envelope[i] = target_gain + coeff * (current_env - target_gain);

            // Apply gain to spectrum
            let gain = envelope[i];
            spectrum[i].re *= gain;
            spectrum[i].im *= gain;
        }
    }

    /// Reset the spectral gate state
    pub fn reset(&mut self) {
        self.stft.reset();
        self.envelope.fill(0.0);
    }

    /// Get the FFT size
    pub fn fft_size(&self) -> usize {
        self.fft_size
    }

    /// Get the hop size
    pub fn hop_size(&self) -> usize {
        self.stft.hop_size()
    }

    /// Enable or disable the gate
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if gate is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_gate_creation() {
        let gate = SpectralGate::new(2048, 512, WindowType::Hann, 44100.0);
        assert_eq!(gate.fft_size(), 2048);
        assert_eq!(gate.hop_size(), 512);
        assert_eq!(gate.threshold(), -40.0);
        assert!(gate.is_enabled());
    }

    #[test]
    #[should_panic(expected = "FFT size must be power of 2")]
    fn test_spectral_gate_requires_power_of_two() {
        SpectralGate::new(1000, 250, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Hop size must be <= FFT size")]
    fn test_spectral_gate_hop_validation() {
        SpectralGate::new(1024, 2048, WindowType::Hann, 44100.0);
    }

    #[test]
    #[should_panic(expected = "Sample rate must be positive")]
    fn test_spectral_gate_sample_rate_validation() {
        SpectralGate::new(1024, 256, WindowType::Hann, 0.0);
    }

    #[test]
    fn test_spectral_gate_set_threshold() {
        let mut gate = SpectralGate::new(1024, 256, WindowType::Hann, 44100.0);

        gate.set_threshold(-30.0);
        assert_eq!(gate.threshold(), -30.0);

        gate.set_threshold(-60.0);
        assert_eq!(gate.threshold(), -60.0);
    }

    #[test]
    fn test_spectral_gate_set_attack() {
        let mut gate = SpectralGate::new(1024, 256, WindowType::Hann, 44100.0);

        gate.set_attack(0.01);
        assert_eq!(gate.attack(), 0.01);

        // Should clamp to minimum
        gate.set_attack(0.00001);
        assert_eq!(gate.attack(), 0.0001);
    }

    #[test]
    fn test_spectral_gate_set_release() {
        let mut gate = SpectralGate::new(1024, 256, WindowType::Hann, 44100.0);

        gate.set_release(0.1);
        assert_eq!(gate.release(), 0.1);

        // Should clamp to minimum
        gate.set_release(0.0001);
        assert_eq!(gate.release(), 0.001);
    }

    #[test]
    fn test_spectral_gate_set_ratio() {
        let mut gate = SpectralGate::new(1024, 256, WindowType::Hann, 44100.0);

        gate.set_ratio(0.5);
        assert_eq!(gate.ratio(), 0.5);

        // Should clamp to [0.0, 1.0]
        gate.set_ratio(1.5);
        assert_eq!(gate.ratio(), 1.0);

        gate.set_ratio(-0.5);
        assert_eq!(gate.ratio(), 0.0);
    }

    #[test]
    fn test_spectral_gate_process_silent() {
        let mut gate = SpectralGate::new(1024, 256, WindowType::Hann, 44100.0);
        let input = vec![0.0; 512];
        let mut output = vec![0.0; 512];

        // Process silence (should remain silent)
        gate.process(&mut output, &input);

        for &sample in &output {
            assert!(sample.abs() < 0.001, "Expected silence, got {}", sample);
        }
    }

    #[test]
    fn test_spectral_gate_process_with_threshold() {
        let mut gate = SpectralGate::new(512, 128, WindowType::Hann, 44100.0);
        gate.set_threshold(-20.0);
        gate.set_ratio(0.0); // Full gate

        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];

        // Should process without crashing
        gate.process(&mut output, &input);
        assert_eq!(output.len(), 256);
    }

    #[test]
    fn test_spectral_gate_disabled() {
        let mut gate = SpectralGate::new(512, 128, WindowType::Hann, 44100.0);
        gate.set_enabled(false);

        let input = vec![1.0; 256];
        let mut output = vec![1.0; 256];
        gate.process(&mut output, &input);

        // Should not modify output when disabled
        assert_eq!(output[0], 1.0);
    }

    #[test]
    fn test_spectral_gate_reset() {
        let mut gate = SpectralGate::new(512, 128, WindowType::Hann, 44100.0);

        // Process some audio to build up envelope state
        let input = vec![0.0; 256];
        let mut output = vec![0.0; 256];
        gate.process(&mut output, &input);

        // Reset should clear state
        gate.reset();

        // Should still work after reset
        gate.process(&mut output, &input);
        assert_eq!(output.len(), 256);
    }

    #[test]
    fn test_spectral_gate_all_window_types() {
        for window_type in [
            WindowType::Rectangular,
            WindowType::Hann,
            WindowType::Hamming,
            WindowType::Blackman,
            WindowType::BlackmanHarris,
        ] {
            let mut gate = SpectralGate::new(512, 128, window_type, 44100.0);

            let input = vec![0.0; 256];
            let mut output = vec![0.0; 256];

            gate.process(&mut output, &input);
            assert_eq!(output.len(), 256);
        }
    }

    #[test]
    fn test_spectral_gate_various_fft_sizes() {
        for fft_size in [512, 1024, 2048, 4096] {
            let hop_size = fft_size / 4;
            let mut gate = SpectralGate::new(fft_size, hop_size, WindowType::Hann, 44100.0);

            let input = vec![0.0; 512];
            let mut output = vec![0.0; 512];

            gate.process(&mut output, &input);
            assert_eq!(output.len(), 512);
        }
    }

    #[test]
    fn test_spectral_gate_enable_disable() {
        let mut gate = SpectralGate::new(512, 128, WindowType::Hann, 44100.0);

        assert!(gate.is_enabled());

        gate.set_enabled(false);
        assert!(!gate.is_enabled());

        gate.set_enabled(true);
        assert!(gate.is_enabled());
    }
}
