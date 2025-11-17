//! Spectral processing effects using STFT
//!
//! These effects operate in the frequency domain using Short-Time Fourier Transform (STFT).
//! They provide high-quality time-stretching, pitch-shifting, and spectral freezing.

use crate::synthesis::spectral::{
    PhaseVocoder as CorePhaseVocoder, SpectralFreeze as CoreSpectralFreeze,
    SpectralGate as CoreSpectralGate, SpectralCompressor as CoreSpectralCompressor,
    SpectralRobotize as CoreSpectralRobotize, WindowType,
};

/// Phase vocoder effect for time-stretching and pitch-shifting
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames (typically 512 samples) for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::spectral::PhaseVocoder;
/// // Create phase vocoder with normal time, pitch up a perfect fifth (7 semitones)
/// let vocoder = PhaseVocoder::new(1.0, 7.0, 44100.0);
///
/// // Process audio blocks
/// let mut buffer = vec![0.0; 512];
/// // vocoder.process_block(&mut buffer, 44100.0, 0.0, 0);
/// ```
#[derive(Clone)]
pub struct PhaseVocoder {
    /// Core phase vocoder engine
    core: CorePhaseVocoder,

    /// Effect priority (lower = earlier in chain)
    pub priority: u8,

    /// Sample rate
    sample_rate: f32,

    /// Enabled flag
    pub enabled: bool,
}

impl PhaseVocoder {
    /// Create a new phase vocoder effect
    ///
    /// # Arguments
    /// * `time_stretch` - Time stretch ratio (1.0 = normal, 2.0 = half speed)
    /// * `pitch_shift` - Pitch shift in semitones (0 = no shift)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::PhaseVocoder;
    /// let vocoder = PhaseVocoder::new(1.0, 0.0, 44100.0);
    /// ```
    pub fn new(time_stretch: f32, pitch_shift: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, time_stretch, pitch_shift, sample_rate)
    }

    /// Create a phase vocoder with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `time_stretch` - Time stretch ratio (1.0 = normal, 2.0 = half speed)
    /// * `pitch_shift` - Pitch shift in semitones (0 = no shift)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::PhaseVocoder;
    /// // Larger FFT for better frequency resolution, more latency
    /// let vocoder = PhaseVocoder::with_params(4096, 1024, 1.0, 0.0, 44100.0);
    /// ```
    pub fn with_params(fft_size: usize, hop_size: usize, time_stretch: f32, pitch_shift: f32, sample_rate: f32) -> Self {
        let mut core = CorePhaseVocoder::new(fft_size, hop_size, sample_rate, WindowType::Hann);
        core.set_time_stretch(time_stretch);
        core.set_pitch_shift(pitch_shift);

        Self {
            core,
            priority: 50, // Process before reverb/delay
            sample_rate,
            enabled: true,
        }
    }

    /// Set time stretch ratio
    ///
    /// # Arguments
    /// * `ratio` - Time stretch ratio (1.0 = normal, 2.0 = half speed, 0.5 = double speed)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::PhaseVocoder;
    /// let mut vocoder = PhaseVocoder::new(1.0, 0.0, 44100.0);
    /// vocoder.set_time_stretch(1.5);  // 50% slower
    /// ```
    pub fn set_time_stretch(&mut self, ratio: f32) {
        self.core.set_time_stretch(ratio);
    }

    /// Set pitch shift in semitones
    ///
    /// # Arguments
    /// * `semitones` - Pitch shift in semitones (0 = no shift, 12 = up octave, -12 = down octave)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::PhaseVocoder;
    /// let mut vocoder = PhaseVocoder::new(1.0, 0.0, 44100.0);
    /// vocoder.set_pitch_shift(7.0);   // Perfect fifth up
    /// vocoder.set_pitch_shift(-12.0); // Octave down
    /// ```
    pub fn set_pitch_shift(&mut self, semitones: f32) {
        self.core.set_pitch_shift(semitones);
    }

    /// Get current time stretch ratio
    pub fn time_stretch(&self) -> f32 {
        self.core.time_stretch()
    }

    /// Get current pitch shift in semitones
    pub fn pitch_shift(&self) -> f32 {
        self.core.pitch_shift()
    }

    /// Reset the phase vocoder state
    pub fn reset(&mut self) {
        self.core.reset();
    }

    /// Set effect priority
    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    /// Process a block of mono audio samples
    ///
    /// **Note**: This is a block-based effect. For best results, process in blocks
    /// that are multiples of the hop size (typically 512 samples).
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to process in-place
    /// * `sample_rate` - Sample rate in Hz
    /// * `_time` - Current time (unused)
    /// * `_sample_count` - Sample count (unused)
    pub fn process_block(
        &mut self,
        buffer: &mut [f32],
        sample_rate: f32,
        _time: f32,
        _sample_count: u64,
    ) {
        if !self.enabled {
            return;
        }

        // Update sample rate if changed
        if (self.sample_rate - sample_rate).abs() > 0.1 {
            self.sample_rate = sample_rate;
            // Note: In a real implementation, we'd recreate the core with new sample rate
            // For now, we just track it
        }

        // Process using core phase vocoder
        let input = buffer.to_vec();
        self.core.process(buffer, &input);
    }
}

impl std::fmt::Debug for PhaseVocoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhaseVocoder")
            .field("time_stretch", &self.core.time_stretch())
            .field("pitch_shift", &self.core.pitch_shift())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Spectral freeze effect - captures and holds frequency spectrum
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames (typically 512 samples) for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::spectral::SpectralFreeze;
/// // Create spectral freeze with freeze enabled and 75% mix
/// let mut freeze = SpectralFreeze::new(true, 0.75, 44100.0);
///
/// // Process audio blocks
/// let mut buffer = vec![0.0; 512];
/// // freeze.process_block(&mut buffer, 44100.0, 0.0, 0);
///
/// // Unfreeze to return to normal
/// freeze.unfreeze();
/// ```
#[derive(Clone)]
pub struct SpectralFreeze {
    /// Core spectral freeze engine
    core: CoreSpectralFreeze,

    /// Effect priority (lower = earlier in chain)
    pub priority: u8,

    /// Enabled flag
    pub enabled: bool,
}

impl SpectralFreeze {
    /// Create a new spectral freeze effect
    ///
    /// Uses default FFT parameters (2048 FFT, 512 hop size).
    ///
    /// # Arguments
    /// * `freeze` - Whether to freeze the spectrum (default: false)
    /// * `mix` - Mix amount (0.0 = all live, 1.0 = all frozen) (default: 1.0)
    /// * `sample_rate` - Audio sample rate in Hz (unused, kept for API consistency)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::SpectralFreeze;
    /// let freeze = SpectralFreeze::new(false, 1.0, 44100.0);
    /// ```
    pub fn new(freeze: bool, mix: f32, _sample_rate: f32) -> Self {
        Self::with_params(2048, 512, freeze, mix, _sample_rate)
    }

    /// Create a spectral freeze with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `freeze` - Whether to freeze the spectrum
    /// * `mix` - Mix amount (0.0 = all live, 1.0 = all frozen)
    /// * `sample_rate` - Audio sample rate in Hz (unused, kept for API consistency)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::SpectralFreeze;
    /// // Larger FFT for better frequency resolution, more latency
    /// let freeze = SpectralFreeze::with_params(4096, 1024, false, 1.0, 44100.0);
    /// ```
    pub fn with_params(fft_size: usize, hop_size: usize, freeze: bool, mix: f32, _sample_rate: f32) -> Self {
        let mut core = CoreSpectralFreeze::new(fft_size, hop_size, WindowType::Hann);
        if freeze {
            core.freeze();
        }
        core.set_mix(mix);

        Self {
            core,
            priority: 50, // Process before reverb/delay, same as phase vocoder
            enabled: true,
        }
    }

    /// Enable freeze and start capturing spectrum
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::SpectralFreeze;
    /// let mut freeze = SpectralFreeze::new(false, 1.0, 44100.0);
    /// freeze.freeze();
    /// assert!(freeze.is_frozen());
    /// ```
    pub fn freeze(&mut self) {
        self.core.freeze();
    }

    /// Disable freeze and return to normal processing
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::SpectralFreeze;
    /// let mut freeze = SpectralFreeze::new(true, 1.0, 44100.0);
    /// freeze.unfreeze();
    /// assert!(!freeze.is_frozen());
    /// ```
    pub fn unfreeze(&mut self) {
        self.core.unfreeze();
    }

    /// Set mix amount between live and frozen signals
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = all live, 1.0 = all frozen)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::SpectralFreeze;
    /// let mut freeze = SpectralFreeze::new(false, 1.0, 44100.0);
    /// freeze.set_mix(0.5);  // 50% live, 50% frozen
    /// ```
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Get whether freeze is currently enabled
    pub fn is_frozen(&self) -> bool {
        self.core.is_frozen()
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Reset the spectral freeze state
    pub fn reset(&mut self) {
        self.core.reset();
    }

    /// Set effect priority
    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    /// Process a block of mono audio samples
    ///
    /// **Note**: This is a block-based effect. For best results, process in blocks
    /// that are multiples of the hop size (typically 512 samples).
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to process in-place
    /// * `_sample_rate` - Sample rate in Hz (unused)
    /// * `_time` - Current time (unused)
    /// * `_sample_count` - Sample count (unused)
    pub fn process_block(
        &mut self,
        buffer: &mut [f32],
        _sample_rate: f32,
        _time: f32,
        _sample_count: u64,
    ) {
        if !self.enabled {
            return;
        }

        // Process using core spectral freeze
        let input = buffer.to_vec();
        self.core.process(buffer, &input);
    }
}

impl std::fmt::Debug for SpectralFreeze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralFreeze")
            .field("is_frozen", &self.core.is_frozen())
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Spectral gate effect - frequency-selective noise gate
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames (typically 512 samples) for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::spectral::SpectralGate;
/// // Create spectral gate with -40 dB threshold, fast attack, medium release
/// let gate = SpectralGate::new(-40.0, 0.001, 0.050, 0.0, 44100.0);
///
/// // Process audio blocks
/// let mut buffer = vec![0.0; 512];
/// // gate.process_block(&mut buffer, 44100.0, 0.0, 0);
/// ```
#[derive(Clone)]
pub struct SpectralGate {
    /// Core spectral gate engine
    core: CoreSpectralGate,

    /// Effect priority (lower = earlier in chain)
    pub priority: u8,

    /// Enabled flag
    pub enabled: bool,
}

impl SpectralGate {
    /// Create a new spectral gate effect
    ///
    /// Uses default FFT parameters (2048 FFT, 512 hop size).
    ///
    /// # Arguments
    /// * `threshold` - Threshold in dB (default: -40.0)
    /// * `attack` - Attack time in seconds (default: 1.0)
    /// * `release` - Release time in seconds (default: 50.0)
    /// * `ratio` - Gate ratio (default: 0.0)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::SpectralGate;
    /// let gate = SpectralGate::new(-40.0, 1.0, 50.0, 0.0, 44100.0);
    /// ```
    pub fn new(threshold: f32, attack: f32, release: f32, ratio: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, threshold, attack, release, ratio, sample_rate)
    }

    /// Create a spectral gate with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, typically 2048 or 4096)
    /// * `hop_size` - Hop size in samples (typically fft_size/4 for 75% overlap)
    /// * `threshold` - Threshold in dB
    /// * `attack` - Attack time in seconds
    /// * `release` - Release time in seconds
    /// * `ratio` - Gate ratio
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::SpectralGate;
    /// // Larger FFT for better frequency resolution, more latency
    /// let gate = SpectralGate::with_params(4096, 1024, -40.0, 1.0, 50.0, 0.0, 44100.0);
    /// ```
    pub fn with_params(fft_size: usize, hop_size: usize, threshold: f32, attack: f32, release: f32, ratio: f32, sample_rate: f32) -> Self {
        let mut core = CoreSpectralGate::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_threshold(threshold);
        core.set_attack(attack);
        core.set_release(release);
        core.set_ratio(ratio);

        Self {
            core,
            priority: 50, // Process before reverb/delay, same as other spectral effects
            enabled: true,
        }
    }

    /// Set threshold in dB
    ///
    /// # Arguments
    /// * `threshold_db` - Threshold in dB (typically -60 to -20)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::SpectralGate;
    /// let mut gate = SpectralGate::new(-40.0, 0.001, 0.050, 0.0, 44100.0);
    /// gate.set_threshold(-40.0);  // Gate bins below -40 dB
    /// ```
    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.core.set_threshold(threshold_db);
    }

    /// Set attack time in seconds
    ///
    /// # Arguments
    /// * `attack` - Attack time in seconds
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::SpectralGate;
    /// let mut gate = SpectralGate::new(-40.0, 0.001, 0.050, 0.0, 44100.0);
    /// gate.set_attack(0.001);  // 1ms
    /// ```
    pub fn set_attack(&mut self, attack: f32) {
        self.core.set_attack(attack);
    }

    /// Set release time in seconds
    ///
    /// # Arguments
    /// * `release` - Release time in seconds
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::SpectralGate;
    /// let mut gate = SpectralGate::new(-40.0, 0.001, 0.050, 0.0, 44100.0);
    /// gate.set_release(0.050);  // 50ms
    /// ```
    pub fn set_release(&mut self, release: f32) {
        self.core.set_release(release);
    }

    /// Set gate ratio
    ///
    /// # Arguments
    /// * `ratio` - Gate ratio (0.0 = full mute, 1.0 = no gating)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::spectral::SpectralGate;
    /// let mut gate = SpectralGate::new(-40.0, 0.001, 0.050, 0.0, 44100.0);
    /// gate.set_ratio(0.1);  // 90% reduction when gated
    /// ```
    pub fn set_ratio(&mut self, ratio: f32) {
        self.core.set_ratio(ratio);
    }

    /// Get current threshold
    pub fn threshold(&self) -> f32 {
        self.core.threshold()
    }

    /// Get current attack time
    pub fn attack(&self) -> f32 {
        self.core.attack()
    }

    /// Get current release time
    pub fn release(&self) -> f32 {
        self.core.release()
    }

    /// Get current ratio
    pub fn ratio(&self) -> f32 {
        self.core.ratio()
    }

    /// Reset the spectral gate state
    pub fn reset(&mut self) {
        self.core.reset();
    }

    /// Set effect priority
    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    /// Process a block of mono audio samples
    ///
    /// **Note**: This is a block-based effect. For best results, process in blocks
    /// that are multiples of the hop size (typically 512 samples).
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to process in-place
    /// * `_sample_rate` - Sample rate in Hz (unused)
    /// * `_time` - Current time (unused)
    /// * `_sample_count` - Sample count (unused)
    pub fn process_block(
        &mut self,
        buffer: &mut [f32],
        _sample_rate: f32,
        _time: f32,
        _sample_count: u64,
    ) {
        if !self.enabled {
            return;
        }

        // Process using core spectral gate
        let input = buffer.to_vec();
        self.core.process(buffer, &input);
    }
}

impl std::fmt::Debug for SpectralGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralGate")
            .field("threshold", &self.core.threshold())
            .field("attack", &self.core.attack())
            .field("release", &self.core.release())
            .field("ratio", &self.core.ratio())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Spectral compressor wrapper for effects chain integration
///
/// Provides frequency-selective dynamic range compression with per-bin control.
/// Perfect for multiband mastering, de-essing, and taming resonances.
#[derive(Clone)]
pub struct SpectralCompressor {
    core: CoreSpectralCompressor,
    pub priority: u8,
    pub enabled: bool,
}

impl SpectralCompressor {
    /// Create a new spectral compressor with default FFT settings (2048/512)
    ///
    /// # Arguments
    /// - `threshold` - Compression threshold in dB (default: -20.0)
    /// - `ratio` - Compression ratio (default: 4.0)
    /// - `attack` - Attack time in milliseconds (default: 5.0)
    /// - `release` - Release time in milliseconds (default: 50.0)
    /// - `knee` - Soft knee width in dB (default: 6.0)
    /// - `sample_rate` - Sample rate in Hz
    pub fn new(threshold: f32, ratio: f32, attack: f32, release: f32, knee: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, threshold, ratio, attack, release, knee, sample_rate)
    }

    /// Create a spectral compressor with custom FFT settings
    ///
    /// # Arguments
    ///
    /// * `fft_size` - FFT size (must be power of 2, typically 2048)
    /// * `hop_size` - Hop size (typically fft_size/4)
    /// * `threshold` - Compression threshold in dB
    /// * `ratio` - Compression ratio
    /// * `attack` - Attack time in milliseconds
    /// * `release` - Release time in milliseconds
    /// * `knee` - Soft knee width in dB
    /// * `sample_rate` - Sample rate in Hz
    pub fn with_params(fft_size: usize, hop_size: usize, threshold: f32, ratio: f32, attack: f32, release: f32, knee: f32, sample_rate: f32) -> Self {
        let mut core = CoreSpectralCompressor::new(
            fft_size,
            hop_size,
            crate::synthesis::spectral::WindowType::Hann,
            sample_rate,
        );
        core.set_threshold(threshold);
        core.set_ratio(ratio);
        core.set_attack(attack);
        core.set_release(release);
        core.set_knee(knee);

        Self {
            core,
            priority: 50, // Before reverb/delay
            enabled: true,
        }
    }

    /// Set compression threshold in dB
    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.core.set_threshold(threshold_db);
    }

    /// Set compression ratio (e.g., 4.0 = 4:1)
    pub fn set_ratio(&mut self, ratio: f32) {
        self.core.set_ratio(ratio);
    }

    /// Set attack time in milliseconds
    pub fn set_attack(&mut self, attack_ms: f32) {
        self.core.set_attack(attack_ms);
    }

    /// Set release time in milliseconds
    pub fn set_release(&mut self, release_ms: f32) {
        self.core.set_release(release_ms);
    }

    /// Set soft knee width in dB
    pub fn set_knee(&mut self, knee_db: f32) {
        self.core.set_knee(knee_db);
    }

    /// Get current threshold
    pub fn threshold(&self) -> f32 {
        self.core.threshold()
    }

    /// Get current ratio
    pub fn ratio(&self) -> f32 {
        self.core.ratio()
    }

    /// Get current attack time
    pub fn attack(&self) -> f32 {
        self.core.attack()
    }

    /// Get current release time
    pub fn release(&self) -> f32 {
        self.core.release()
    }

    /// Get current knee width
    pub fn knee(&self) -> f32 {
        self.core.knee()
    }

    /// Process a block of mono audio
    ///
    /// # Arguments
    ///
    /// * `buffer` - Mono audio buffer to process in-place
    /// * `_sample_rate` - Sample rate (unused, kept for API consistency)
    /// * `_time` - Current time in seconds (unused)
    /// * `_sample_count` - Sample counter (unused)
    pub fn process_block(
        &mut self,
        buffer: &mut [f32],
        _sample_rate: f32,
        _time: f32,
        _sample_count: u64,
    ) {
        if !self.enabled {
            return;
        }

        let input = buffer.to_vec();
        self.core.process(buffer, &input);
    }

    /// Reset internal state
    pub fn reset(&mut self) {
        self.core.reset();
    }
}

impl std::fmt::Debug for SpectralCompressor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralCompressor")
            .field("threshold", &self.core.threshold())
            .field("ratio", &self.core.ratio())
            .field("attack", &self.core.attack())
            .field("release", &self.core.release())
            .field("knee", &self.core.knee())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Spectral robotize wrapper for effects chain integration
///
/// Provides phase quantization for robotic/synthesized voice effects.
/// Perfect for robot voices, whisper-to-speech conversion, and creative sound design.
#[derive(Clone)]
pub struct SpectralRobotize {
    core: CoreSpectralRobotize,
    pub priority: u8,
    pub enabled: bool,
}

impl SpectralRobotize {
    /// Create a new spectral robotize effect
    ///
    /// # Arguments
    /// - `target_phase` - Target phase to quantize to in radians (typically 0.0)
    /// - `mix` - Mix between original (0.0) and robotized (1.0)
    /// - `sample_rate` - Sample rate in Hz (unused but kept for API consistency)
    pub fn new(target_phase: f32, mix: f32, _sample_rate: f32) -> Self {
        Self::with_params(2048, 512, target_phase, mix, _sample_rate)
    }

    /// Create a spectral robotize with custom FFT settings
    ///
    /// # Arguments
    /// - `fft_size` - FFT size (must be power of 2, typically 2048)
    /// - `hop_size` - Hop size (typically fft_size/4)
    /// - `target_phase` - Target phase to quantize to in radians
    /// - `mix` - Mix between original (0.0) and robotized (1.0)
    /// - `sample_rate` - Sample rate in Hz (unused but kept for API consistency)
    pub fn with_params(fft_size: usize, hop_size: usize, target_phase: f32, mix: f32, _sample_rate: f32) -> Self {
        let mut core = CoreSpectralRobotize::new(
            fft_size,
            hop_size,
            crate::synthesis::spectral::WindowType::Hann,
        );
        core.set_target_phase(target_phase);
        core.set_mix(mix);

        Self {
            core,
            priority: 50, // Before reverb/delay
            enabled: true,
        }
    }

    /// Set target phase to quantize to (typically 0.0)
    pub fn set_target_phase(&mut self, phase: f32) {
        self.core.set_target_phase(phase);
    }

    /// Set mix between original (0.0) and robotized (1.0)
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Get current target phase
    pub fn target_phase(&self) -> f32 {
        self.core.target_phase()
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Process a block of mono audio
    ///
    /// # Arguments
    ///
    /// * `buffer` - Mono audio buffer to process in-place
    /// * `_sample_rate` - Sample rate (unused, kept for API consistency)
    /// * `_time` - Current time in seconds (unused)
    /// * `_sample_count` - Sample counter (unused)
    pub fn process_block(
        &mut self,
        buffer: &mut [f32],
        _sample_rate: f32,
        _time: f32,
        _sample_count: u64,
    ) {
        if !self.enabled {
            return;
        }

        let input = buffer.to_vec();
        self.core.process(buffer, &input);
    }

    /// Reset internal state
    pub fn reset(&mut self) {
        self.core.reset();
    }
}

impl std::fmt::Debug for SpectralRobotize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralRobotize")
            .field("target_phase", &self.core.target_phase())
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Wrapper for SpectralDelay with priority and enabled state
pub struct SpectralDelay {
    core: crate::synthesis::spectral::SpectralDelay,
    pub priority: u8,
    pub enabled: bool,
}

impl SpectralDelay {
    /// Create a new spectral delay
    ///
    /// # Arguments
    /// - `delay_time` - Base delay time in milliseconds (0.0-2000.0)
    /// - `feedback` - Feedback amount (0.0-1.0)
    /// - `frequency_scale` - Frequency-dependent scaling (-1.0 to 1.0)
    /// - `mix` - Dry/wet mix (0.0-1.0)
    /// - `sample_rate` - Sample rate in Hz
    pub fn new(delay_time: f32, feedback: f32, frequency_scale: f32, mix: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, delay_time, feedback, frequency_scale, mix, sample_rate)
    }

    /// Create with custom FFT parameters
    ///
    /// # Arguments
    /// - `fft_size` - FFT size (must be power of 2)
    /// - `hop_size` - Hop size (must be <= fft_size)
    /// - `delay_time` - Base delay time in milliseconds
    /// - `feedback` - Feedback amount (0.0-1.0)
    /// - `frequency_scale` - Frequency-dependent scaling (-1.0 to 1.0)
    /// - `mix` - Dry/wet mix (0.0-1.0)
    /// - `sample_rate` - Sample rate in Hz
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        delay_time: f32,
        feedback: f32,
        frequency_scale: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        Self {
            core: crate::synthesis::spectral::SpectralDelay::with_params(
                fft_size,
                hop_size,
                sample_rate,
                delay_time,
                feedback,
                frequency_scale,
                mix,
            ),
            priority: 100,
            enabled: true,
        }
    }

    /// Set base delay time in milliseconds
    pub fn set_delay_time(&mut self, delay_time: f32) {
        self.core.set_delay_time(delay_time);
    }

    /// Get current delay time
    pub fn delay_time(&self) -> f32 {
        self.core.delay_time()
    }

    /// Set feedback amount (0.0-1.0)
    pub fn set_feedback(&mut self, feedback: f32) {
        self.core.set_feedback(feedback);
    }

    /// Get current feedback
    pub fn feedback(&self) -> f32 {
        self.core.feedback()
    }

    /// Set frequency-dependent scaling (-1.0 to 1.0)
    pub fn set_frequency_scale(&mut self, scale: f32) {
        self.core.set_frequency_scale(scale);
    }

    /// Get current frequency scale
    pub fn frequency_scale(&self) -> f32 {
        self.core.frequency_scale()
    }

    /// Set dry/wet mix (0.0-1.0)
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Get current mix
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Clear all delay buffers
    pub fn clear_buffers(&mut self) {
        self.core.clear_buffers();
    }

    /// Process a block of audio
    pub fn process_block(
        &mut self,
        buffer: &mut [f32],
        _sample_rate: f32,
        _time: f32,
        _sample_count: u64,
    ) {
        if !self.enabled {
            return;
        }
        let input = buffer.to_vec();
        self.core.process(buffer, &input);
    }

    /// Reset the effect
    pub fn reset(&mut self) {
        self.core.reset();
    }
}

impl Clone for SpectralDelay {
    fn clone(&self) -> Self {
        Self {
            core: self.core.clone(),
            priority: self.priority,
            enabled: self.enabled,
        }
    }
}

impl std::fmt::Debug for SpectralDelay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralDelay")
            .field("delay_time", &self.core.delay_time())
            .field("feedback", &self.core.feedback())
            .field("frequency_scale", &self.core.frequency_scale())
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_vocoder_creation() {
        let vocoder = PhaseVocoder::new(1.0, 0.0, 44100.0);
        assert_eq!(vocoder.time_stretch(), 1.0);
        assert_eq!(vocoder.pitch_shift(), 0.0);
        assert!(vocoder.enabled);
    }

    #[test]
    fn test_phase_vocoder_time_stretch() {
        let mut vocoder = PhaseVocoder::new(1.0, 0.0, 44100.0);
        vocoder.set_time_stretch(2.0);
        assert_eq!(vocoder.time_stretch(), 2.0);
    }

    #[test]
    fn test_phase_vocoder_pitch_shift() {
        let mut vocoder = PhaseVocoder::new(1.0, 0.0, 44100.0);
        vocoder.set_pitch_shift(12.0);
        assert_eq!(vocoder.pitch_shift(), 12.0);
    }

    #[test]
    fn test_phase_vocoder_process_block() {
        let mut vocoder = PhaseVocoder::new(1.0, 0.0, 44100.0);
        let mut buffer = vec![0.0; 512];

        // Should not crash
        vocoder.process_block(&mut buffer, 44100.0, 0.0, 0);
        assert_eq!(buffer.len(), 512);
    }

    #[test]
    fn test_phase_vocoder_reset() {
        let mut vocoder = PhaseVocoder::new(1.0, 0.0, 44100.0);
        vocoder.set_pitch_shift(7.0);
        vocoder.reset();

        // Pitch shift should persist (it's a parameter, not state)
        assert_eq!(vocoder.pitch_shift(), 7.0);
    }

    // ========== SpectralFreeze Tests ==========

    #[test]
    fn test_spectral_freeze_creation() {
        let freeze = SpectralFreeze::new(false, 1.0, 44100.0);
        assert!(!freeze.is_frozen());
        assert_eq!(freeze.mix(), 1.0);
        assert!(freeze.enabled);
    }

    #[test]
    fn test_spectral_freeze_freeze_unfreeze() {
        let mut freeze = SpectralFreeze::new(false, 1.0, 44100.0);
        assert!(!freeze.is_frozen());

        freeze.freeze();
        assert!(freeze.is_frozen());

        freeze.unfreeze();
        assert!(!freeze.is_frozen());
    }

    #[test]
    fn test_spectral_freeze_set_mix() {
        let mut freeze = SpectralFreeze::new(false, 1.0, 44100.0);

        freeze.set_mix(0.5);
        assert_eq!(freeze.mix(), 0.5);

        freeze.set_mix(0.0);
        assert_eq!(freeze.mix(), 0.0);

        freeze.set_mix(1.0);
        assert_eq!(freeze.mix(), 1.0);
    }

    #[test]
    fn test_spectral_freeze_process_block() {
        let mut freeze = SpectralFreeze::new(true, 1.0, 44100.0);

        let mut buffer = vec![0.0; 512];

        // Should not crash
        freeze.process_block(&mut buffer, 44100.0, 0.0, 0);
        assert_eq!(buffer.len(), 512);
    }

    #[test]
    fn test_spectral_freeze_disabled() {
        let mut freeze = SpectralFreeze::new(true, 1.0, 44100.0);
        freeze.enabled = false;

        let mut buffer = vec![1.0; 512];
        freeze.process_block(&mut buffer, 44100.0, 0.0, 0);

        // Should not modify buffer when disabled
        assert_eq!(buffer[0], 1.0);
    }

    #[test]
    fn test_spectral_freeze_reset() {
        let mut freeze = SpectralFreeze::new(true, 1.0, 44100.0);
        freeze.reset();

        // Should be unfrozen after reset
        assert!(!freeze.is_frozen());
    }

    #[test]
    fn test_spectral_freeze_with_params() {
        let freeze = SpectralFreeze::with_params(4096, 1024, false, 1.0, 44100.0);
        assert!(!freeze.is_frozen());
        assert!(freeze.enabled);
    }

    // ========== SpectralGate Tests ==========

    #[test]
    fn test_spectral_gate_creation() {
        let gate = SpectralGate::new(-40.0, 1.0, 50.0, 0.0, 44100.0);
        assert_eq!(gate.threshold(), -40.0);
        assert!(gate.enabled);
    }

    #[test]
    fn test_spectral_gate_set_threshold() {
        let mut gate = SpectralGate::new(-40.0, 1.0, 50.0, 0.0, 44100.0);

        gate.set_threshold(-30.0);
        assert_eq!(gate.threshold(), -30.0);

        gate.set_threshold(-60.0);
        assert_eq!(gate.threshold(), -60.0);
    }

    #[test]
    fn test_spectral_gate_set_attack() {
        let mut gate = SpectralGate::new(-40.0, 1.0, 50.0, 0.0, 44100.0);

        gate.set_attack(0.01);
        assert_eq!(gate.attack(), 0.01);
    }

    #[test]
    fn test_spectral_gate_set_release() {
        let mut gate = SpectralGate::new(-40.0, 1.0, 50.0, 0.0, 44100.0);

        gate.set_release(0.1);
        assert_eq!(gate.release(), 0.1);
    }

    #[test]
    fn test_spectral_gate_set_ratio() {
        let mut gate = SpectralGate::new(-40.0, 1.0, 50.0, 0.0, 44100.0);

        gate.set_ratio(0.5);
        assert_eq!(gate.ratio(), 0.5);
    }

    #[test]
    fn test_spectral_gate_process_block() {
        let mut gate = SpectralGate::new(-40.0, 1.0, 50.0, 0.0, 44100.0);
        gate.set_threshold(-40.0);

        let mut buffer = vec![0.0; 512];

        // Should not crash
        gate.process_block(&mut buffer, 44100.0, 0.0, 0);
        assert_eq!(buffer.len(), 512);
    }

    #[test]
    fn test_spectral_gate_disabled() {
        let mut gate = SpectralGate::new(-40.0, 1.0, 50.0, 0.0, 44100.0);
        gate.enabled = false;

        let mut buffer = vec![1.0; 512];
        gate.process_block(&mut buffer, 44100.0, 0.0, 0);

        // Should not modify buffer when disabled
        assert_eq!(buffer[0], 1.0);
    }

    #[test]
    fn test_spectral_gate_reset() {
        let mut gate = SpectralGate::new(-40.0, 1.0, 50.0, 0.0, 44100.0);
        gate.reset();

        // Should not crash after reset
        let mut buffer = vec![0.0; 512];
        gate.process_block(&mut buffer, 44100.0, 0.0, 0);
        assert_eq!(buffer.len(), 512);
    }

    #[test]
    fn test_spectral_gate_with_params() {
        let gate = SpectralGate::with_params(4096, 1024, -40.0, 1.0, 50.0, 0.0, 44100.0);
        assert!(gate.enabled);
    }

    // ===== SpectralCompressor Wrapper Tests =====

    #[test]
    fn test_spectral_compressor_creation() {
        let comp = SpectralCompressor::new(-20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
        assert!(comp.enabled);
        assert_eq!(comp.priority, 50);
    }

    #[test]
    fn test_spectral_compressor_set_threshold() {
        let mut comp = SpectralCompressor::new(-20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
        comp.set_threshold(-30.0);
        assert_eq!(comp.threshold(), -30.0);
    }

    #[test]
    fn test_spectral_compressor_set_ratio() {
        let mut comp = SpectralCompressor::new(-20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
        comp.set_ratio(8.0);
        assert_eq!(comp.ratio(), 8.0);
    }

    #[test]
    fn test_spectral_compressor_set_attack() {
        let mut comp = SpectralCompressor::new(-20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
        comp.set_attack(10.0);
        assert_eq!(comp.attack(), 10.0);
    }

    #[test]
    fn test_spectral_compressor_set_release() {
        let mut comp = SpectralCompressor::new(-20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
        comp.set_release(100.0);
        assert_eq!(comp.release(), 100.0);
    }

    #[test]
    fn test_spectral_compressor_set_knee() {
        let mut comp = SpectralCompressor::new(-20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
        comp.set_knee(12.0);
        assert_eq!(comp.knee(), 12.0);
    }

    #[test]
    fn test_spectral_compressor_process_block() {
        let mut comp = SpectralCompressor::new(-20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
        let mut buffer = vec![0.0; 512];

        // Should not crash
        comp.process_block(&mut buffer, 44100.0, 0.0, 0);
        assert_eq!(buffer.len(), 512);
    }

    #[test]
    fn test_spectral_compressor_disabled() {
        let mut comp = SpectralCompressor::new(-20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
        comp.enabled = false;

        let mut buffer = vec![1.0; 512];
        comp.process_block(&mut buffer, 44100.0, 0.0, 0);

        // When disabled, buffer should be unchanged
        assert_eq!(buffer[0], 1.0);
    }

    #[test]
    fn test_spectral_compressor_reset() {
        let mut comp = SpectralCompressor::new(-20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
        comp.reset();

        // Should not crash after reset
        let mut buffer = vec![0.0; 512];
        comp.process_block(&mut buffer, 44100.0, 0.0, 0);
        assert_eq!(buffer.len(), 512);
    }

    #[test]
    fn test_spectral_compressor_with_params() {
        let comp = SpectralCompressor::with_params(4096, 1024, -20.0, 4.0, 5.0, 50.0, 6.0, 44100.0);
        assert!(comp.enabled);
    }

    // ===== SpectralRobotize Wrapper Tests =====

    #[test]
    fn test_spectral_robotize_creation() {
        let robotize = SpectralRobotize::new(0.0, 1.0, 44100.0);
        assert!(robotize.enabled);
        assert_eq!(robotize.priority, 50);
        assert_eq!(robotize.target_phase(), 0.0);
        assert_eq!(robotize.mix(), 1.0);
    }

    #[test]
    fn test_spectral_robotize_set_target_phase() {
        let mut robotize = SpectralRobotize::new(0.0, 1.0, 44100.0);
        robotize.set_target_phase(std::f32::consts::PI);
        assert_eq!(robotize.target_phase(), std::f32::consts::PI);
    }

    #[test]
    fn test_spectral_robotize_set_mix() {
        let mut robotize = SpectralRobotize::new(0.0, 1.0, 44100.0);
        robotize.set_mix(0.5);
        assert_eq!(robotize.mix(), 0.5);
    }

    #[test]
    fn test_spectral_robotize_process_block() {
        let mut robotize = SpectralRobotize::new(0.0, 1.0, 44100.0);
        let mut buffer = vec![0.0; 512];

        // Should not crash
        robotize.process_block(&mut buffer, 44100.0, 0.0, 0);
        assert_eq!(buffer.len(), 512);
    }

    #[test]
    fn test_spectral_robotize_disabled() {
        let mut robotize = SpectralRobotize::new(0.0, 1.0, 44100.0);
        robotize.enabled = false;

        let mut buffer = vec![1.0; 512];
        robotize.process_block(&mut buffer, 44100.0, 0.0, 0);

        // When disabled, buffer should be unchanged
        assert_eq!(buffer[0], 1.0);
    }

    #[test]
    fn test_spectral_robotize_reset() {
        let mut robotize = SpectralRobotize::new(0.0, 1.0, 44100.0);
        robotize.reset();

        // Should not crash after reset
        let mut buffer = vec![0.0; 512];
        robotize.process_block(&mut buffer, 44100.0, 0.0, 0);
        assert_eq!(buffer.len(), 512);
    }

    #[test]
    fn test_spectral_robotize_with_params() {
        let robotize = SpectralRobotize::with_params(4096, 1024, 0.0, 0.5, 44100.0);
        assert!(robotize.enabled);
    }

    // SpectralDelay wrapper tests
    #[test]
    fn test_spectral_delay_creation() {
        let delay = SpectralDelay::new(100.0, 0.3, 0.0, 0.5, 44100.0);
        assert_eq!(delay.priority, 100);
        assert!(delay.enabled);
        assert_eq!(delay.delay_time(), 100.0);
    }

    #[test]
    fn test_spectral_delay_set_delay_time() {
        let mut delay = SpectralDelay::new(100.0, 0.3, 0.0, 0.5, 44100.0);
        delay.set_delay_time(200.0);
        assert_eq!(delay.delay_time(), 200.0);
    }

    #[test]
    fn test_spectral_delay_set_feedback() {
        let mut delay = SpectralDelay::new(100.0, 0.3, 0.0, 0.5, 44100.0);
        delay.set_feedback(0.7);
        assert_eq!(delay.feedback(), 0.7);
    }

    #[test]
    fn test_spectral_delay_set_frequency_scale() {
        let mut delay = SpectralDelay::new(100.0, 0.3, 0.0, 0.5, 44100.0);
        delay.set_frequency_scale(0.5);
        assert_eq!(delay.frequency_scale(), 0.5);
    }

    #[test]
    fn test_spectral_delay_set_mix() {
        let mut delay = SpectralDelay::new(100.0, 0.3, 0.0, 0.5, 44100.0);
        delay.set_mix(0.75);
        assert_eq!(delay.mix(), 0.75);
    }

    #[test]
    fn test_spectral_delay_process_block() {
        let mut delay = SpectralDelay::new(100.0, 0.3, 0.0, 0.5, 44100.0);
        let mut buffer = vec![0.5; 512];
        delay.process_block(&mut buffer, 44100.0, 0.0, 0);
        // Should process without crashing
    }

    #[test]
    fn test_spectral_delay_disabled() {
        let mut delay = SpectralDelay::new(100.0, 0.3, 0.0, 0.5, 44100.0);
        delay.enabled = false;

        let original = vec![0.5; 512];
        let mut buffer = original.clone();
        delay.process_block(&mut buffer, 44100.0, 0.0, 0);

        // When disabled, buffer should be unchanged
        assert_eq!(buffer, original);
    }

    #[test]
    fn test_spectral_delay_reset() {
        let mut delay = SpectralDelay::new(100.0, 0.3, 0.0, 0.5, 44100.0);
        let mut buffer = vec![0.5; 512];
        delay.process_block(&mut buffer, 44100.0, 0.0, 0);
        delay.reset();
        // Should reset without crashing
    }

    #[test]
    fn test_spectral_delay_clear_buffers() {
        let mut delay = SpectralDelay::new(100.0, 0.3, 0.0, 0.5, 44100.0);
        delay.clear_buffers();
        // Should clear without crashing
    }

    #[test]
    fn test_spectral_delay_with_params() {
        let delay = SpectralDelay::with_params(4096, 1024, 200.0, 0.5, 0.8, 0.7, 44100.0);
        assert!(delay.enabled);
    }
}
