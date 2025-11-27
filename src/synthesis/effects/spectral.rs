//! Spectral processing effects using STFT
//!
//! These effects operate in the frequency domain using Short-Time Fourier Transform (STFT).
//! They provide high-quality time-stretching, pitch-shifting, and spectral freezing.

use crate::synthesis::spectral::{
    FormantShifter as CoreFormantShifter, PhaseVocoder as CorePhaseVocoder,
    SpectralBlur as CoreSpectralBlur, SpectralCompressor as CoreSpectralCompressor,
    SpectralDynamics as CoreSpectralDynamics, SpectralFilter as CoreSpectralFilter,
    SpectralFreeze as CoreSpectralFreeze, SpectralGate as CoreSpectralGate,
    SpectralHarmonizer as CoreSpectralHarmonizer, SpectralPanner as CoreSpectralPanner,
    SpectralResonator as CoreSpectralResonator, SpectralRobotize as CoreSpectralRobotize,
    SpectralScramble as CoreSpectralScramble, WindowType,
};

// Re-export FilterType, HarmonyVoice, PanPoint, and Resonance for public use
pub use crate::synthesis::spectral::FilterType;
pub use crate::synthesis::spectral::HarmonyVoice;
pub use crate::synthesis::spectral::PanPoint;
pub use crate::synthesis::spectral::Resonance;

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

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
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
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        time_stretch: f32,
        pitch_shift: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CorePhaseVocoder::new(fft_size, hop_size, sample_rate, WindowType::Hann);
        core.set_time_stretch(time_stretch);
        core.set_pitch_shift(pitch_shift);

        Self {
            core,
            priority: 50, // Process before reverb/delay
            sample_rate,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Pitch up a perfect fifth (7 semitones)
    pub fn pitch_up() -> Self {
        Self::new(1.0, 7.0, 44100.0)
    }

    /// Pitch down a perfect fifth (-7 semitones)
    pub fn pitch_down() -> Self {
        Self::new(1.0, -7.0, 44100.0)
    }

    /// Chipmunk effect - high pitched voice
    pub fn chipmunk() -> Self {
        Self::new(1.0, 12.0, 44100.0)
    }

    /// Slow motion - half speed without pitch change
    pub fn slow_mo() -> Self {
        Self::new(2.0, 0.0, 44100.0)
    }

    /// Fast forward - double speed without pitch change
    pub fn fast() -> Self {
        Self::new(0.5, 0.0, 44100.0)
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

        // Process using core phase vocoder - reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
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

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
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
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        freeze: bool,
        mix: f32,
        _sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralFreeze::new(fft_size, hop_size, WindowType::Hann);
        if freeze {
            core.freeze();
        }
        core.set_mix(mix);

        Self {
            core,
            priority: 50, // Process before reverb/delay, same as phase vocoder
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Frozen - full freeze effect
    pub fn frozen() -> Self {
        Self::new(true, 1.0, 44100.0)
    }

    /// Shimmer - partial freeze for ethereal effect
    pub fn shimmer() -> Self {
        Self::new(true, 0.5, 44100.0)
    }

    /// Glitch - subtle freeze for glitchy textures
    pub fn glitch() -> Self {
        Self::new(true, 0.3, 44100.0)
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

        // Process using core spectral freeze - reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
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

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
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
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        threshold: f32,
        attack: f32,
        release: f32,
        ratio: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralGate::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_threshold(threshold);
        core.set_attack(attack);
        core.set_release(release);
        core.set_ratio(ratio);

        Self {
            core,
            priority: 50, // Process before reverb/delay, same as other spectral effects
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Gentle noise reduction
    pub fn gentle() -> Self {
        Self::new(-50.0, 10.0, 100.0, 0.2, 44100.0)
    }

    /// Aggressive gating - tight cleanup
    pub fn aggressive() -> Self {
        Self::new(-35.0, 1.0, 50.0, 0.0, 44100.0)
    }

    /// Denoise - subtle noise reduction
    pub fn denoise() -> Self {
        Self::new(-60.0, 20.0, 150.0, 0.1, 44100.0)
    }

    /// Tighten - clean up loose signals
    pub fn tighten() -> Self {
        Self::new(-40.0, 2.0, 80.0, 0.0, 44100.0)
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

        // Process using core spectral gate - reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
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
    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
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
    pub fn new(
        threshold: f32,
        ratio: f32,
        attack: f32,
        release: f32,
        knee: f32,
        sample_rate: f32,
    ) -> Self {
        Self::with_params(
            2048,
            512,
            threshold,
            ratio,
            attack,
            release,
            knee,
            sample_rate,
        )
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
    #[allow(clippy::too_many_arguments)]
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        threshold: f32,
        ratio: f32,
        attack: f32,
        release: f32,
        knee: f32,
        sample_rate: f32,
    ) -> Self {
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
            input_buffer: Vec::new(),
        }
    }

    /// Gentle compression - subtle glue
    pub fn gentle() -> Self {
        Self::new(-25.0, 2.0, 10.0, 80.0, 3.0, 44100.0)
    }

    /// Aggressive compression - heavy control
    pub fn aggressive() -> Self {
        Self::new(-15.0, 6.0, 2.0, 40.0, 2.0, 44100.0)
    }

    /// Glue - subtle spectral glue compression
    pub fn glue() -> Self {
        Self::new(-30.0, 3.0, 15.0, 100.0, 4.0, 44100.0)
    }

    /// Tame peaks - reduce spectral peaks
    pub fn tame_peaks() -> Self {
        Self::new(-10.0, 8.0, 1.0, 30.0, 1.0, 44100.0)
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

        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
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
    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
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
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        target_phase: f32,
        mix: f32,
        _sample_rate: f32,
    ) -> Self {
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
            input_buffer: Vec::new(),
        }
    }

    /// Robot - classic robot voice effect
    pub fn robot() -> Self {
        Self::new(0.0, 1.0, 44100.0)
    }

    /// Vocoder - vocoder-style effect
    pub fn vocoder() -> Self {
        Self::new(0.0, 0.8, 44100.0)
    }

    /// Metallic - metallic timbre
    pub fn metallic() -> Self {
        Self::new(1.57, 0.9, 44100.0) // π/2 phase
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

        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
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
    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
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
    pub fn new(
        delay_time: f32,
        feedback: f32,
        frequency_scale: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        Self::with_params(
            2048,
            512,
            delay_time,
            feedback,
            frequency_scale,
            mix,
            sample_rate,
        )
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
            input_buffer: Vec::new(),
        }
    }

    /// Shimmer - upward pitch-shifting delay
    pub fn shimmer() -> Self {
        Self::new(200.0, 0.4, 1.5, 0.6, 44100.0)
    }

    /// Cascade - downward cascading delay
    pub fn cascade() -> Self {
        Self::new(150.0, 0.5, 0.5, 0.7, 44100.0)
    }

    /// Echo - simple spectral echo
    pub fn echo() -> Self {
        Self::new(250.0, 0.3, 1.0, 0.5, 44100.0)
    }

    /// Harmonic - harmonic delay effect
    pub fn harmonic() -> Self {
        Self::new(100.0, 0.6, 2.0, 0.5, 44100.0)
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
        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
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
            input_buffer: Vec::new(),
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

/// Spectral filter effect for frequency-domain filtering
///
/// Applies filtering by manipulating frequency bin magnitudes directly in the spectral domain.
/// This provides precise control over frequency response and can create effects not possible
/// with traditional time-domain filters.
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::{SpectralFilter, FilterType};
/// // Create low-pass filter at 1kHz with moderate resonance
/// let filter = SpectralFilter::new(FilterType::LowPass, 1000.0, 2.0, 1.0, 44100.0);
/// ```
#[derive(Clone)]
pub struct SpectralFilter {
    /// Core spectral filter engine
    core: CoreSpectralFilter,

    /// Effect priority (higher = later in chain)
    pub priority: u8,

    /// Whether this effect is enabled
    pub enabled: bool,

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralFilter {
    /// Create a new spectral filter with specified parameters
    ///
    /// # Arguments
    /// * `filter_type` - Type of filter (LowPass, HighPass, BandPass, BandStop, Notch)
    /// * `cutoff` - Cutoff frequency in Hz
    /// * `resonance` - Q factor (0.1 - 20.0)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// Uses default FFT size of 2048 and hop size of 512 (75% overlap).
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralFilter, FilterType};
    /// let filter = SpectralFilter::new(FilterType::LowPass, 1000.0, 2.0, 1.0, 44100.0);
    /// ```
    pub fn new(
        filter_type: FilterType,
        cutoff: f32,
        resonance: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        Self::with_params(2048, 512, filter_type, cutoff, resonance, mix, sample_rate)
    }

    /// Create a spectral filter with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2)
    /// * `hop_size` - Hop size in samples
    /// * `filter_type` - Type of filter
    /// * `cutoff` - Cutoff frequency in Hz
    /// * `resonance` - Q factor
    /// * `mix` - Wet/dry mix
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralFilter, FilterType};
    /// let filter = SpectralFilter::with_params(4096, 1024, FilterType::HighPass, 200.0, 1.0, 0.8, 44100.0);
    /// ```
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        filter_type: FilterType,
        cutoff: f32,
        resonance: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralFilter::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_filter_type(filter_type);
        core.set_cutoff(cutoff);
        core.set_resonance(resonance);
        core.set_mix(mix);

        Self {
            core,
            priority: 50,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Low-pass filter - remove highs (1kHz cutoff)
    pub fn low_pass() -> Self {
        Self::new(FilterType::LowPass, 1000.0, 1.0, 1.0, 44100.0)
    }

    /// High-pass filter - remove lows (200Hz cutoff)
    pub fn high_pass() -> Self {
        Self::new(FilterType::HighPass, 200.0, 1.0, 1.0, 44100.0)
    }

    /// Band-pass filter - midrange only
    pub fn band_pass() -> Self {
        Self::new(FilterType::BandPass, 1000.0, 2.0, 1.0, 44100.0)
    }

    /// Telephone - narrow band-pass for lo-fi effect
    pub fn telephone() -> Self {
        Self::new(FilterType::BandPass, 1200.0, 8.0, 1.0, 44100.0)
    }

    /// Radio - band-pass for vintage radio effect
    pub fn radio() -> Self {
        Self::new(FilterType::BandPass, 2000.0, 5.0, 1.0, 44100.0)
    }

    /// Rumble cut - remove low-end rumble
    pub fn rumble_cut() -> Self {
        Self::new(FilterType::HighPass, 80.0, 1.5, 1.0, 44100.0)
    }

    /// Air cut - remove excessive brightness
    pub fn air_cut() -> Self {
        Self::new(FilterType::LowPass, 8000.0, 1.0, 1.0, 44100.0)
    }

    /// Set the filter type
    pub fn set_filter_type(&mut self, filter_type: FilterType) {
        self.core.set_filter_type(filter_type);
    }

    /// Set the cutoff frequency in Hz
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.core.set_cutoff(cutoff);
    }

    /// Set the resonance/Q factor
    pub fn set_resonance(&mut self, resonance: f32) {
        self.core.set_resonance(resonance);
    }

    /// Set the wet/dry mix
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Process a block of audio
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to process (modified in-place)
    /// * `_sample_rate` - Sample rate (unused, kept for API consistency)
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
        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset the filter state
    pub fn reset(&mut self) {
        self.core.reset();
    }

    /// Clone into a boxed effect
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    /// Get effect priority
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// Set effect priority
    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    /// Check if effect is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable the effect
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Create a copy with the same settings
    pub fn duplicate(&self) -> Self {
        Self {
            core: self.core.clone(),
            priority: self.priority,
            enabled: self.enabled,
            input_buffer: Vec::new(),
        }
    }
}

impl std::fmt::Debug for SpectralFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralFilter")
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Spectral blur effect - temporal smoothing in frequency domain
///
/// Creates a smooth, ambient quality by blending current spectrum with previous frames.
/// Perfect for pads, ambient textures, and creating ethereal soundscapes.
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::SpectralBlur;
/// // Create gentle blur with moderate feedback
/// let blur = SpectralBlur::new(0.5, 0.3, 1.0, 44100.0);
/// ```
#[derive(Clone)]
pub struct SpectralBlur {
    /// Core spectral blur engine
    core: CoreSpectralBlur,

    /// Effect priority (higher = later in chain)
    pub priority: u8,

    /// Whether this effect is enabled
    pub enabled: bool,

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralBlur {
    /// Create a new spectral blur with specified parameters
    ///
    /// # Arguments
    /// * `blur_amount` - Amount of blurring (0.0 = none, 1.0 = maximum)
    /// * `feedback` - Temporal persistence (0.0-0.99, higher = longer trails)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// Uses default FFT size of 2048 and hop size of 512 (75% overlap).
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralBlur;
    /// let blur = SpectralBlur::new(0.5, 0.3, 1.0, 44100.0);
    /// ```
    pub fn new(blur_amount: f32, feedback: f32, mix: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, blur_amount, feedback, mix, sample_rate)
    }

    /// Create a spectral blur with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2)
    /// * `hop_size` - Hop size in samples
    /// * `blur_amount` - Amount of blurring (0.0-1.0)
    /// * `feedback` - Temporal persistence (0.0-0.99)
    /// * `mix` - Wet/dry mix (0.0-1.0)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralBlur;
    /// let blur = SpectralBlur::with_params(4096, 1024, 0.7, 0.5, 1.0, 44100.0);
    /// ```
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        blur_amount: f32,
        feedback: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralBlur::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_blur_amount(blur_amount);
        core.set_feedback(feedback);
        core.set_mix(mix);

        Self {
            core,
            priority: 50,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Subtle blur - gentle smoothing
    pub fn subtle() -> Self {
        Self::new(0.3, 0.2, 0.5, 44100.0)
    }

    /// Gentle blur - moderate smoothing for pads
    pub fn gentle() -> Self {
        Self::new(0.5, 0.3, 0.7, 44100.0)
    }

    /// Dreamy - ethereal ambient texture
    pub fn dreamy() -> Self {
        Self::new(0.7, 0.5, 0.8, 44100.0)
    }

    /// Heavy blur - maximum smoothing
    pub fn heavy() -> Self {
        Self::new(0.9, 0.7, 1.0, 44100.0)
    }

    /// Shimmer - long trails for reverb-like effect
    pub fn shimmer() -> Self {
        Self::new(0.6, 0.8, 0.9, 44100.0)
    }

    /// Set blur amount
    ///
    /// # Arguments
    /// * `amount` - Blur amount (0.0 = none, 1.0 = maximum)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralBlur;
    /// let mut blur = SpectralBlur::new(0.5, 0.3, 1.0, 44100.0);
    /// blur.set_blur_amount(0.8);
    /// ```
    pub fn set_blur_amount(&mut self, amount: f32) {
        self.core.set_blur_amount(amount);
    }

    /// Get current blur amount
    pub fn blur_amount(&self) -> f32 {
        self.core.blur_amount()
    }

    /// Set feedback amount
    ///
    /// # Arguments
    /// * `feedback` - Feedback amount (0.0-0.99, higher = longer trails)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralBlur;
    /// let mut blur = SpectralBlur::new(0.5, 0.3, 1.0, 44100.0);
    /// blur.set_feedback(0.6);
    /// ```
    pub fn set_feedback(&mut self, feedback: f32) {
        self.core.set_feedback(feedback);
    }

    /// Get current feedback amount
    pub fn feedback(&self) -> f32 {
        self.core.feedback()
    }

    /// Set wet/dry mix
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = dry, 1.0 = wet)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralBlur;
    /// let mut blur = SpectralBlur::new(0.5, 0.3, 1.0, 44100.0);
    /// blur.set_mix(0.5);
    /// ```
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Process a block of audio
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to process (modified in-place)
    /// * `_sample_rate` - Sample rate (unused, kept for API consistency)
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
        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset the blur state (clears previous spectrum)
    pub fn reset(&mut self) {
        self.core.reset();
    }

    /// Clone into a boxed effect
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    /// Get effect priority
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// Set effect priority
    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    /// Check if effect is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable the effect
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Create a copy with the same settings
    pub fn duplicate(&self) -> Self {
        Self {
            core: self.core.clone(),
            priority: self.priority,
            enabled: self.enabled,
            input_buffer: Vec::new(),
        }
    }
}

impl std::fmt::Debug for SpectralBlur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralBlur")
            .field("blur_amount", &self.core.blur_amount())
            .field("feedback", &self.core.feedback())
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

use crate::synthesis::spectral::SpectralShift as CoreSpectralShift;

/// Spectral shift effect - shift all frequencies by a fixed Hz amount
///
/// Unlike pitch shifting (which multiplies frequencies), spectral shift adds
/// a constant Hz offset to all frequencies. This breaks harmonic relationships,
/// creating metallic, bell-like, or alien timbres.
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::SpectralShift;
/// // Create spectral shift with 100 Hz upward shift
/// let shift = SpectralShift::new(100.0, 0.8, 44100.0);
/// ```
#[derive(Clone)]
pub struct SpectralShift {
    /// Core spectral shift engine
    core: CoreSpectralShift,

    /// Effect priority (higher = later in chain)
    pub priority: u8,

    /// Whether this effect is enabled
    pub enabled: bool,

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralShift {
    /// Create a new spectral shift with specified parameters
    ///
    /// # Arguments
    /// * `shift_hz` - Frequency shift in Hz (positive = up, negative = down)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// Uses default FFT size of 2048 and hop size of 512 (75% overlap).
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralShift;
    /// let shift = SpectralShift::new(100.0, 0.8, 44100.0);
    /// ```
    pub fn new(shift_hz: f32, mix: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, shift_hz, mix, sample_rate)
    }

    /// Create a spectral shift with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2)
    /// * `hop_size` - Hop size in samples
    /// * `shift_hz` - Frequency shift in Hz
    /// * `mix` - Wet/dry mix (0.0-1.0)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralShift;
    /// let shift = SpectralShift::with_params(4096, 1024, 150.0, 0.9, 44100.0);
    /// ```
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        shift_hz: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralShift::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_shift_hz(shift_hz);
        core.set_mix(mix);

        Self {
            core,
            priority: 145,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Subtle detuning for thickness
    pub fn subtle() -> Self {
        Self::new(7.0, 0.5, 44100.0)
    }

    /// Metallic timbre
    pub fn metallic() -> Self {
        Self::new(75.0, 0.8, 44100.0)
    }

    /// Bell-like sound
    pub fn bell() -> Self {
        Self::new(150.0, 0.9, 44100.0)
    }

    /// Alien/sci-fi effect
    pub fn alien() -> Self {
        Self::new(300.0, 1.0, 44100.0)
    }

    /// Downshift (darker, lower)
    pub fn down() -> Self {
        Self::new(-100.0, 0.8, 44100.0)
    }

    /// Set frequency shift in Hz
    ///
    /// # Arguments
    /// * `shift_hz` - Frequency shift in Hz (positive = up, negative = down)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralShift;
    /// let mut shift = SpectralShift::new(100.0, 0.8, 44100.0);
    /// shift.set_shift_hz(150.0);
    /// ```
    pub fn set_shift_hz(&mut self, shift_hz: f32) {
        self.core.set_shift_hz(shift_hz);
    }

    /// Get current frequency shift
    pub fn shift_hz(&self) -> f32 {
        self.core.shift_hz()
    }

    /// Set wet/dry mix
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = dry, 1.0 = wet)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralShift;
    /// let mut shift = SpectralShift::new(100.0, 0.8, 44100.0);
    /// shift.set_mix(0.5);
    /// ```
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Process a block of audio
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to process (modified in-place)
    /// * `_sample_rate` - Sample rate (unused, kept for API consistency)
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
        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset the shift state
    pub fn reset(&mut self) {
        self.core.reset();
    }

    /// Clone into a boxed effect
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    /// Get effect priority
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// Set effect priority
    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    /// Check if effect is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable the effect
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Create a copy with the same settings
    pub fn duplicate(&self) -> Self {
        Self {
            core: self.core.clone(),
            priority: self.priority,
            enabled: self.enabled,
            input_buffer: Vec::new(),
        }
    }
}

impl std::fmt::Debug for SpectralShift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralShift")
            .field("shift_hz", &self.core.shift_hz())
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

use crate::synthesis::spectral::SpectralExciter as CoreSpectralExciter;

/// Spectral exciter effect - adds harmonics and brightness to high frequencies
///
/// Unlike traditional exciters that add harmonics in the time domain,
/// spectral exciter works in the frequency domain for more precise control.
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::SpectralExciter;
/// // Create spectral exciter with 3kHz crossover and moderate drive
/// let exciter = SpectralExciter::new(3000.0, 2.0, 0.5, 0.5, 44100.0);
/// ```
#[derive(Clone)]
pub struct SpectralExciter {
    /// Core spectral exciter engine
    core: CoreSpectralExciter,

    /// Effect priority (higher = later in chain)
    pub priority: u8,

    /// Whether this effect is enabled
    pub enabled: bool,

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralExciter {
    /// Create a new spectral exciter with specified parameters
    ///
    /// # Arguments
    /// * `frequency` - Crossover frequency in Hz (excite above this)
    /// * `drive` - Drive amount (1.0-10.0)
    /// * `harmonics` - Harmonic blend (0.0-1.0)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// Uses default FFT size of 2048 and hop size of 512 (75% overlap).
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralExciter;
    /// let exciter = SpectralExciter::new(3000.0, 2.0, 0.5, 0.5, 44100.0);
    /// ```
    pub fn new(frequency: f32, drive: f32, harmonics: f32, mix: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, frequency, drive, harmonics, mix, sample_rate)
    }

    /// Create a spectral exciter with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2)
    /// * `hop_size` - Hop size in samples
    /// * `frequency` - Crossover frequency in Hz
    /// * `drive` - Drive amount (1.0-10.0)
    /// * `harmonics` - Harmonic blend (0.0-1.0)
    /// * `mix` - Wet/dry mix (0.0-1.0)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralExciter;
    /// let exciter = SpectralExciter::with_params(4096, 1024, 4000.0, 3.0, 0.7, 0.6, 44100.0);
    /// ```
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        frequency: f32,
        drive: f32,
        harmonics: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralExciter::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_frequency(frequency);
        core.set_drive(drive);
        core.set_harmonics(harmonics);
        core.set_mix(mix);

        Self {
            core,
            priority: 150,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Gentle excitement for subtle brightness
    pub fn gentle() -> Self {
        Self::new(5000.0, 1.5, 0.3, 0.25, 44100.0)
    }

    /// Moderate excitement for presence and clarity
    pub fn moderate() -> Self {
        Self::new(3500.0, 2.5, 0.5, 0.4, 44100.0)
    }

    /// Aggressive excitement for maximum brightness
    pub fn aggressive() -> Self {
        Self::new(2500.0, 4.0, 0.7, 0.6, 44100.0)
    }

    /// Air preset - adds top-end air and sparkle
    pub fn air() -> Self {
        Self::new(8000.0, 2.0, 0.8, 0.3, 44100.0)
    }

    /// Presence preset - adds midrange presence
    pub fn presence() -> Self {
        Self::new(2000.0, 3.0, 0.4, 0.5, 44100.0)
    }

    /// Set crossover frequency
    ///
    /// # Arguments
    /// * `frequency` - Crossover frequency in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralExciter;
    /// let mut exciter = SpectralExciter::new(3000.0, 2.0, 0.5, 0.5, 44100.0);
    /// exciter.set_frequency(4000.0);
    /// ```
    pub fn set_frequency(&mut self, frequency: f32) {
        self.core.set_frequency(frequency);
    }

    /// Get current crossover frequency
    pub fn frequency(&self) -> f32 {
        self.core.frequency()
    }

    /// Set drive amount
    ///
    /// # Arguments
    /// * `drive` - Drive amount (1.0-10.0)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralExciter;
    /// let mut exciter = SpectralExciter::new(3000.0, 2.0, 0.5, 0.5, 44100.0);
    /// exciter.set_drive(3.0);
    /// ```
    pub fn set_drive(&mut self, drive: f32) {
        self.core.set_drive(drive);
    }

    /// Get current drive amount
    pub fn drive(&self) -> f32 {
        self.core.drive()
    }

    /// Set harmonic blend
    ///
    /// # Arguments
    /// * `harmonics` - Harmonic blend (0.0-1.0)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralExciter;
    /// let mut exciter = SpectralExciter::new(3000.0, 2.0, 0.5, 0.5, 44100.0);
    /// exciter.set_harmonics(0.7);
    /// ```
    pub fn set_harmonics(&mut self, harmonics: f32) {
        self.core.set_harmonics(harmonics);
    }

    /// Get current harmonic blend
    pub fn harmonics(&self) -> f32 {
        self.core.harmonics()
    }

    /// Set wet/dry mix
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = dry, 1.0 = wet)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralExciter;
    /// let mut exciter = SpectralExciter::new(3000.0, 2.0, 0.5, 0.5, 44100.0);
    /// exciter.set_mix(0.3);
    /// ```
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Process a block of audio
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to process (modified in-place)
    /// * `_sample_rate` - Sample rate (unused, kept for API consistency)
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
        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset the exciter state
    pub fn reset(&mut self) {
        self.core.reset();
    }

    /// Clone into a boxed effect
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    /// Get effect priority
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// Set effect priority
    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    /// Check if effect is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable the effect
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Create a copy with the same settings
    pub fn duplicate(&self) -> Self {
        Self {
            core: self.core.clone(),
            priority: self.priority,
            enabled: self.enabled,
            input_buffer: Vec::new(),
        }
    }
}

impl std::fmt::Debug for SpectralExciter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralExciter")
            .field("frequency", &self.core.frequency())
            .field("drive", &self.core.drive())
            .field("harmonics", &self.core.harmonics())
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

use crate::synthesis::spectral::SpectralInvert as CoreSpectralInvert;

/// Spectral invert effect - flip the frequency spectrum upside down
///
/// Inverts the spectrum by swapping low and high frequencies, creating
/// alien, backwards-sounding, or otherworldly timbres.
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::SpectralInvert;
/// // Create spectral invert with 80% mix
/// let invert = SpectralInvert::new(0.8, 44100.0);
/// ```
#[derive(Clone)]
pub struct SpectralInvert {
    /// Core spectral invert engine
    core: CoreSpectralInvert,

    /// Effect priority (higher = later in chain)
    pub priority: u8,

    /// Whether this effect is enabled
    pub enabled: bool,

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralInvert {
    /// Create a new spectral invert with specified parameters
    ///
    /// # Arguments
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = fully inverted)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// Uses default FFT size of 2048 and hop size of 512 (75% overlap).
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralInvert;
    /// let invert = SpectralInvert::new(0.8, 44100.0);
    /// ```
    pub fn new(mix: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, mix, sample_rate)
    }

    /// Create a spectral invert with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2)
    /// * `hop_size` - Hop size in samples
    /// * `mix` - Wet/dry mix (0.0-1.0)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralInvert;
    /// let invert = SpectralInvert::with_params(4096, 1024, 1.0, 44100.0);
    /// ```
    pub fn with_params(fft_size: usize, hop_size: usize, mix: f32, sample_rate: f32) -> Self {
        let mut core = CoreSpectralInvert::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_mix(mix);

        Self {
            core,
            priority: 160,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Subtle inversion for slight weirdness
    pub fn subtle() -> Self {
        Self::new(0.3, 44100.0)
    }

    /// Full inversion for alien timbres
    pub fn full() -> Self {
        Self::new(1.0, 44100.0)
    }

    /// Moderate blend for interesting hybrids
    pub fn moderate() -> Self {
        Self::new(0.6, 44100.0)
    }

    /// Set wet/dry mix
    ///
    /// # Arguments
    /// * `mix` - Mix amount (0.0 = dry, 1.0 = fully inverted)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralInvert;
    /// let mut invert = SpectralInvert::new(0.8, 44100.0);
    /// invert.set_mix(0.5);
    /// ```
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Process a block of audio
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to process (modified in-place)
    /// * `_sample_rate` - Sample rate (unused, kept for API consistency)
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
        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset the invert state
    pub fn reset(&mut self) {
        self.core.reset();
    }

    /// Clone into a boxed effect
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    /// Get effect priority
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// Set effect priority
    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    /// Check if effect is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable the effect
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Create a copy with the same settings
    pub fn duplicate(&self) -> Self {
        Self {
            core: self.core.clone(),
            priority: self.priority,
            enabled: self.enabled,
            input_buffer: Vec::new(),
        }
    }
}

impl std::fmt::Debug for SpectralInvert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralInvert")
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

use crate::synthesis::spectral::SpectralWiden as CoreSpectralWiden;

/// Spectral widen effect - stereo widening in the frequency domain
///
/// Creates stereo width by manipulating phase relationships in the frequency domain.
/// More precise and artifact-free than time-domain stereo widening.
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::SpectralWiden;
/// // Create spectral widen with 150% width
/// let widen = SpectralWiden::new(1.5, 200.0, 16000.0, 44100.0);
/// ```
#[derive(Clone)]
pub struct SpectralWiden {
    /// Core spectral widen engine
    core: CoreSpectralWiden,

    /// Effect priority (higher = later in chain)
    pub priority: u8,

    /// Whether this effect is enabled
    pub enabled: bool,

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralWiden {
    /// Create a new spectral widen with specified parameters
    ///
    /// # Arguments
    /// * `width` - Width multiplier (0.0 = mono, 1.0 = normal, 2.0 = very wide)
    /// * `low_cutoff` - Low frequency cutoff in Hz (don't widen below this)
    /// * `high_cutoff` - High frequency cutoff in Hz (don't widen above this)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// Uses default FFT size of 2048 and hop size of 512 (75% overlap).
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralWiden;
    /// let widen = SpectralWiden::new(1.5, 200.0, 16000.0, 44100.0);
    /// ```
    pub fn new(width: f32, low_cutoff: f32, high_cutoff: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, width, low_cutoff, high_cutoff, sample_rate)
    }

    /// Create a spectral widen with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2)
    /// * `hop_size` - Hop size in samples
    /// * `width` - Width multiplier (0.0-3.0)
    /// * `low_cutoff` - Low frequency cutoff in Hz
    /// * `high_cutoff` - High frequency cutoff in Hz
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralWiden;
    /// let widen = SpectralWiden::with_params(4096, 1024, 2.0, 150.0, 16000.0, 44100.0);
    /// ```
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        width: f32,
        low_cutoff: f32,
        high_cutoff: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralWiden::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_width(width);
        core.set_low_cutoff(low_cutoff);
        core.set_high_cutoff(high_cutoff);

        Self {
            core,
            priority: 165,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Subtle widening for mix enhancement
    pub fn subtle() -> Self {
        Self::new(1.2, 250.0, 16000.0, 44100.0)
    }

    /// Moderate widening for stereo enhancement
    pub fn moderate() -> Self {
        Self::new(1.5, 200.0, 16000.0, 44100.0)
    }

    /// Wide stereo for dramatic effect
    pub fn wide() -> Self {
        Self::new(2.0, 150.0, 16000.0, 44100.0)
    }

    /// Ultra-wide for special effects
    pub fn ultra() -> Self {
        Self::new(2.5, 200.0, 16000.0, 44100.0)
    }

    /// Set width amount
    ///
    /// # Arguments
    /// * `width` - Width multiplier (0.0 = mono, 1.0 = normal, 2.0 = very wide)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralWiden;
    /// let mut widen = SpectralWiden::new(1.5, 200.0, 16000.0, 44100.0);
    /// widen.set_width(2.0);
    /// ```
    pub fn set_width(&mut self, width: f32) {
        self.core.set_width(width);
    }

    /// Get current width
    pub fn width(&self) -> f32 {
        self.core.width()
    }

    /// Set low frequency cutoff
    ///
    /// # Arguments
    /// * `frequency` - Low cutoff in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralWiden;
    /// let mut widen = SpectralWiden::new(1.5, 200.0, 16000.0, 44100.0);
    /// widen.set_low_cutoff(250.0);
    /// ```
    pub fn set_low_cutoff(&mut self, frequency: f32) {
        self.core.set_low_cutoff(frequency);
    }

    /// Get low cutoff
    pub fn low_cutoff(&self) -> f32 {
        self.core.low_cutoff()
    }

    /// Set high frequency cutoff
    ///
    /// # Arguments
    /// * `frequency` - High cutoff in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::SpectralWiden;
    /// let mut widen = SpectralWiden::new(1.5, 200.0, 16000.0, 44100.0);
    /// widen.set_high_cutoff(12000.0);
    /// ```
    pub fn set_high_cutoff(&mut self, frequency: f32) {
        self.core.set_high_cutoff(frequency);
    }

    /// Get high cutoff
    pub fn high_cutoff(&self) -> f32 {
        self.core.high_cutoff()
    }

    /// Process a block of audio
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to process (modified in-place)
    /// * `_sample_rate` - Sample rate (unused, kept for API consistency)
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
        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset the widen state
    pub fn reset(&mut self) {
        self.core.reset();
    }

    /// Clone into a boxed effect
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    /// Get effect priority
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// Set effect priority
    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    /// Check if effect is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable the effect
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Create a copy with the same settings
    pub fn duplicate(&self) -> Self {
        Self {
            core: self.core.clone(),
            priority: self.priority,
            enabled: self.enabled,
            input_buffer: Vec::new(),
        }
    }
}

impl std::fmt::Debug for SpectralWiden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralWiden")
            .field("width", &self.core.width())
            .field("low_cutoff", &self.core.low_cutoff())
            .field("high_cutoff", &self.core.high_cutoff())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

use crate::synthesis::spectral::SpectralMorph as CoreSpectralMorph;

// Re-export MorphTarget as SpectralMorphTarget
pub use crate::synthesis::spectral::MorphTarget as SpectralMorphTarget;

/// Spectral morph effect - morph between different spectral shapes
///
/// Morphs the input spectrum toward target spectral shapes like
/// harmonic series, noise, or filtered versions. Creates vocoder-like
/// and transformative effects.
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::{SpectralMorph, SpectralMorphTarget};
/// // Create spectral morph with robot target
/// let morph = SpectralMorph::new(SpectralMorphTarget::Robot, 0.7, 220.0, 44100.0);
/// ```
#[derive(Clone)]
pub struct SpectralMorph {
    /// Core spectral morph engine
    core: CoreSpectralMorph,

    /// Effect priority (higher = later in chain)
    pub priority: u8,

    /// Whether this effect is enabled
    pub enabled: bool,

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralMorph {
    /// Create a new spectral morph with specified parameters
    ///
    /// # Arguments
    /// * `target` - Target spectrum type
    /// * `morph_amount` - Morph amount (0.0 = original, 1.0 = full target)
    /// * `fundamental` - Fundamental frequency for harmonic target (Hz)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// Uses default FFT size of 2048 and hop size of 512 (75% overlap).
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralMorph, SpectralMorphTarget};
    /// let morph = SpectralMorph::new(SpectralMorphTarget::Robot, 0.7, 220.0, 44100.0);
    /// ```
    pub fn new(
        target: SpectralMorphTarget,
        morph_amount: f32,
        fundamental: f32,
        sample_rate: f32,
    ) -> Self {
        Self::with_params(2048, 512, target, morph_amount, fundamental, sample_rate)
    }

    /// Create a spectral morph with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2)
    /// * `hop_size` - Hop size in samples
    /// * `target` - Target spectrum type
    /// * `morph_amount` - Morph amount (0.0-1.0)
    /// * `fundamental` - Fundamental frequency for harmonic target (Hz)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralMorph, SpectralMorphTarget};
    /// let morph = SpectralMorph::with_params(4096, 1024, SpectralMorphTarget::Harmonic, 0.8, 110.0, 44100.0);
    /// ```
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        target: SpectralMorphTarget,
        morph_amount: f32,
        fundamental: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralMorph::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_target(target);
        core.set_morph_amount(morph_amount);
        core.set_fundamental(fundamental);

        Self {
            core,
            priority: 170,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Morph to harmonic series
    pub fn harmonic() -> Self {
        Self::new(SpectralMorphTarget::Harmonic, 0.7, 110.0, 44100.0)
    }

    /// Morph to noise
    pub fn noise() -> Self {
        Self::new(SpectralMorphTarget::Noise, 0.6, 110.0, 44100.0)
    }

    /// Morph to whisper
    pub fn whisper() -> Self {
        Self::new(SpectralMorphTarget::Whisper, 0.8, 110.0, 44100.0)
    }

    /// Morph to robot
    pub fn robot() -> Self {
        Self::new(SpectralMorphTarget::Robot, 0.9, 220.0, 44100.0)
    }

    /// Set target spectrum type
    ///
    /// # Arguments
    /// * `target` - Target spectrum type
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralMorph, SpectralMorphTarget};
    /// let mut morph = SpectralMorph::new(SpectralMorphTarget::Robot, 0.7, 220.0, 44100.0);
    /// morph.set_target(SpectralMorphTarget::Harmonic);
    /// ```
    pub fn set_target(&mut self, target: SpectralMorphTarget) {
        self.core.set_target(target);
    }

    /// Get current target
    pub fn target(&self) -> SpectralMorphTarget {
        self.core.target()
    }

    /// Set morph amount
    ///
    /// # Arguments
    /// * `amount` - Morph amount (0.0 = original, 1.0 = full target)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralMorph, SpectralMorphTarget};
    /// let mut morph = SpectralMorph::new(SpectralMorphTarget::Robot, 0.7, 220.0, 44100.0);
    /// morph.set_morph_amount(0.5);
    /// ```
    pub fn set_morph_amount(&mut self, amount: f32) {
        self.core.set_morph_amount(amount);
    }

    /// Get morph amount
    pub fn morph_amount(&self) -> f32 {
        self.core.morph_amount()
    }

    /// Set fundamental frequency
    ///
    /// # Arguments
    /// * `frequency` - Fundamental frequency in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralMorph, SpectralMorphTarget};
    /// let mut morph = SpectralMorph::new(SpectralMorphTarget::Harmonic, 0.7, 220.0, 44100.0);
    /// morph.set_fundamental(110.0);
    /// ```
    pub fn set_fundamental(&mut self, frequency: f32) {
        self.core.set_fundamental(frequency);
    }

    /// Get fundamental frequency
    pub fn fundamental(&self) -> f32 {
        self.core.fundamental()
    }

    /// Process a block of audio
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to process (modified in-place)
    /// * `_sample_rate` - Sample rate (unused, kept for API consistency)
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
        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset the morph state
    pub fn reset(&mut self) {
        self.core.reset();
    }

    /// Clone into a boxed effect
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    /// Get effect priority
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// Set effect priority
    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    /// Check if effect is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable the effect
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Create a copy with the same settings
    pub fn duplicate(&self) -> Self {
        Self {
            core: self.core.clone(),
            priority: self.priority,
            enabled: self.enabled,
            input_buffer: Vec::new(),
        }
    }
}

impl std::fmt::Debug for SpectralMorph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralMorph")
            .field("target", &self.core.target())
            .field("morph_amount", &self.core.morph_amount())
            .field("fundamental", &self.core.fundamental())
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

/// Spectral dynamics wrapper for effects chain integration
///
/// Provides frequency-dependent dynamic range compression/expansion with per-bin control.
/// Perfect for multiband dynamics, spectral expansion, and creative sound design.
#[derive(Clone)]
pub struct SpectralDynamics {
    core: CoreSpectralDynamics,
    pub priority: u8,
    pub enabled: bool,
    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralDynamics {
    /// Create a new spectral dynamics effect with default FFT settings (2048/512)
    ///
    /// # Arguments
    /// - `threshold` - Dynamics threshold in dB (default: -20.0)
    /// - `ratio` - Dynamics ratio (>1 = compress, <1 = expand, default: 4.0)
    /// - `attack` - Attack time in milliseconds (default: 5.0)
    /// - `release` - Release time in milliseconds (default: 50.0)
    /// - `knee` - Soft knee width in dB (default: 6.0)
    /// - `sample_rate` - Sample rate in Hz
    pub fn new(
        threshold: f32,
        ratio: f32,
        attack: f32,
        release: f32,
        knee: f32,
        sample_rate: f32,
    ) -> Self {
        Self::with_params(
            2048,
            512,
            threshold,
            ratio,
            attack,
            release,
            knee,
            sample_rate,
        )
    }

    /// Create a spectral dynamics effect with custom FFT settings
    ///
    /// # Arguments
    /// - `fft_size` - FFT size (must be power of 2, typically 2048)
    /// - `hop_size` - Hop size (typically fft_size/4)
    /// - `threshold` - Dynamics threshold in dB
    /// - `ratio` - Dynamics ratio (>1 = compress, <1 = expand)
    /// - `attack` - Attack time in milliseconds
    /// - `release` - Release time in milliseconds
    /// - `knee` - Soft knee width in dB
    /// - `sample_rate` - Sample rate in Hz
    #[allow(clippy::too_many_arguments)]
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        threshold: f32,
        ratio: f32,
        attack: f32,
        release: f32,
        knee: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralDynamics::new(
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
            priority: 155, // After spectral effects, before spatial
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Gentle dynamics - subtle control
    pub fn gentle() -> Self {
        let core = CoreSpectralDynamics::gentle();
        Self {
            core,
            priority: 155,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Moderate dynamics - balanced control
    pub fn moderate() -> Self {
        let core = CoreSpectralDynamics::moderate();
        Self {
            core,
            priority: 155,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Aggressive dynamics - heavy control
    pub fn aggressive() -> Self {
        let core = CoreSpectralDynamics::aggressive();
        Self {
            core,
            priority: 155,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Expander - dynamics expansion
    pub fn expander() -> Self {
        let core = CoreSpectralDynamics::expander();
        Self {
            core,
            priority: 155,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Gate-like - aggressive expansion
    pub fn gate_like() -> Self {
        let core = CoreSpectralDynamics::gate_like();
        Self {
            core,
            priority: 155,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Set dynamics threshold in dB
    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.core.set_threshold(threshold_db);
    }

    /// Set dynamics ratio
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

    /// Set mix between original and processed signal (0.0-1.0)
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
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

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Process a block of mono audio
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

        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset internal state
    pub fn reset(&mut self) {
        self.core.reset();
    }
}

impl std::fmt::Debug for SpectralDynamics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralDynamics")
            .field("threshold", &self.core.threshold())
            .field("ratio", &self.core.ratio())
            .field("attack", &self.core.attack())
            .field("release", &self.core.release())
            .field("knee", &self.core.knee())
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Spectral scramble wrapper for effects chain integration
///
/// Provides frequency bin randomization for glitchy, experimental effects.
/// Perfect for glitch effects, digital artifacts, and creative sound design.
#[derive(Clone)]
pub struct SpectralScramble {
    core: CoreSpectralScramble,
    pub priority: u8,
    pub enabled: bool,
    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralScramble {
    /// Create a new spectral scramble effect with default FFT settings (2048/512)
    ///
    /// # Arguments
    /// - `scramble_amount` - Scramble intensity (0.0-1.0, default: 1.0)
    /// - `low_freq` - Low frequency bound in Hz (default: 200.0)
    /// - `high_freq` - High frequency bound in Hz (default: 12000.0)
    /// - `mix` - Mix between original and scrambled (0.0-1.0, default: 1.0)
    /// - `sample_rate` - Sample rate in Hz
    pub fn new(
        scramble_amount: f32,
        low_freq: f32,
        high_freq: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        Self::with_params(
            2048,
            512,
            scramble_amount,
            low_freq,
            high_freq,
            mix,
            sample_rate,
        )
    }

    /// Create a spectral scramble effect with custom FFT settings
    ///
    /// # Arguments
    /// - `fft_size` - FFT size (must be power of 2, typically 2048)
    /// - `hop_size` - Hop size (typically fft_size/4)
    /// - `scramble_amount` - Scramble intensity (0.0-1.0)
    /// - `low_freq` - Low frequency bound in Hz
    /// - `high_freq` - High frequency bound in Hz
    /// - `mix` - Mix between original and scrambled (0.0-1.0)
    /// - `sample_rate` - Sample rate in Hz
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        scramble_amount: f32,
        low_freq: f32,
        high_freq: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralScramble::new(
            fft_size,
            hop_size,
            crate::synthesis::spectral::WindowType::Hann,
            sample_rate,
        );
        core.set_scramble_amount(scramble_amount);
        core.set_low_freq(low_freq);
        core.set_high_freq(high_freq);
        core.set_mix(mix);

        Self {
            core,
            priority: 175, // After other spectral effects
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Subtle scramble - mild glitchiness
    pub fn subtle() -> Self {
        let core = CoreSpectralScramble::subtle();
        Self {
            core,
            priority: 175,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Moderate scramble - balanced glitch
    pub fn moderate() -> Self {
        let core = CoreSpectralScramble::moderate();
        Self {
            core,
            priority: 175,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Chaos - full scramble
    pub fn chaos() -> Self {
        let core = CoreSpectralScramble::chaos();
        Self {
            core,
            priority: 175,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Glitch - mid-range scramble
    pub fn glitch() -> Self {
        let core = CoreSpectralScramble::glitch();
        Self {
            core,
            priority: 175,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Digital - high frequency scramble
    pub fn digital() -> Self {
        let core = CoreSpectralScramble::digital();
        Self {
            core,
            priority: 175,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Set scramble amount (0.0-1.0)
    pub fn set_scramble_amount(&mut self, amount: f32) {
        self.core.set_scramble_amount(amount);
    }

    /// Set low frequency bound in Hz
    pub fn set_low_freq(&mut self, freq: f32) {
        self.core.set_low_freq(freq);
    }

    /// Set high frequency bound in Hz
    pub fn set_high_freq(&mut self, freq: f32) {
        self.core.set_high_freq(freq);
    }

    /// Set mix between original and scrambled (0.0-1.0)
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Get current scramble amount
    pub fn scramble_amount(&self) -> f32 {
        self.core.scramble_amount()
    }

    /// Get current low frequency bound
    pub fn low_freq(&self) -> f32 {
        self.core.low_freq()
    }

    /// Get current high frequency bound
    pub fn high_freq(&self) -> f32 {
        self.core.high_freq()
    }

    /// Get current mix amount
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Process a block of mono audio
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

        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset internal state
    pub fn reset(&mut self) {
        self.core.reset();
    }
}

impl std::fmt::Debug for SpectralScramble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralScramble")
            .field("scramble_amount", &self.core.scramble_amount())
            .field("low_freq", &self.core.low_freq())
            .field("high_freq", &self.core.high_freq())
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Formant shifter effect - shift formants independently from pitch
///
/// Formants are the resonant frequencies that define the timbre/color of a sound.
/// This effect shifts the spectral envelope without changing pitch, enabling
/// vocal gender changes, character effects, and timbral transformations.
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::FormantShifter;
/// let shifter = FormantShifter::new(1.6, 1.0, 44100.0); // Male to female
/// ```
#[derive(Clone)]
pub struct FormantShifter {
    /// Core formant shifter engine
    core: CoreFormantShifter,

    /// Effect priority (higher = later in chain)
    pub priority: u8,

    /// Whether this effect is enabled
    pub enabled: bool,

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl FormantShifter {
    /// Create a new formant shifter with specified parameters
    ///
    /// # Arguments
    /// * `shift_ratio` - Formant shift ratio (0.5 = down octave, 2.0 = up octave)
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// Uses default FFT size of 2048 and hop size of 512 (75% overlap).
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::FormantShifter;
    /// let shifter = FormantShifter::new(0.7, 1.0, 44100.0);
    /// ```
    pub fn new(shift_ratio: f32, mix: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, shift_ratio, mix, sample_rate)
    }

    /// Create a formant shifter with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2)
    /// * `hop_size` - Hop size in samples
    /// * `shift_ratio` - Formant shift ratio
    /// * `mix` - Wet/dry mix (0.0-1.0)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::FormantShifter;
    /// let shifter = FormantShifter::with_params(4096, 1024, 1.5, 0.8, 44100.0);
    /// ```
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        shift_ratio: f32,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreFormantShifter::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_shift_ratio(shift_ratio);
        core.set_mix(mix);

        Self {
            core,
            priority: 146, // After spectral shift
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Male voice → female voice preset
    pub fn male_to_female() -> Self {
        let core = CoreFormantShifter::male_to_female();
        Self {
            core,
            priority: 146,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Female voice → male voice preset
    pub fn female_to_male() -> Self {
        let core = CoreFormantShifter::female_to_male();
        Self {
            core,
            priority: 146,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Adult voice → child voice preset
    pub fn adult_to_child() -> Self {
        let core = CoreFormantShifter::adult_to_child();
        Self {
            core,
            priority: 146,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Subtle brightening preset
    pub fn brighten() -> Self {
        let core = CoreFormantShifter::brighten();
        Self {
            core,
            priority: 146,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Subtle darkening preset
    pub fn darken() -> Self {
        let core = CoreFormantShifter::darken();
        Self {
            core,
            priority: 146,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Monster/creature voice preset
    pub fn monster() -> Self {
        let core = CoreFormantShifter::monster();
        Self {
            core,
            priority: 146,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Chipmunk/cartoon voice preset
    pub fn chipmunk() -> Self {
        let core = CoreFormantShifter::chipmunk();
        Self {
            core,
            priority: 146,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Set formant shift ratio
    pub fn set_shift_ratio(&mut self, ratio: f32) {
        self.core.set_shift_ratio(ratio);
    }

    /// Set wet/dry mix
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Get current shift ratio
    pub fn shift_ratio(&self) -> f32 {
        self.core.shift_ratio()
    }

    /// Get current mix
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Process a block of audio (called by effect chain)
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

        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset internal state
    pub fn reset(&mut self) {
        self.core.reset();
    }
}

impl std::fmt::Debug for FormantShifter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FormantShifter")
            .field("shift_ratio", &self.core.shift_ratio())
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Spectral harmonizer effect - add harmonically-related pitch-shifted voices
///
/// Creates rich harmonies by adding multiple pitch-shifted copies of the input.
/// Each voice has independent pitch shift and mix level, perfect for vocal
/// harmonies, thick synth sounds, and choir effects.
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::{SpectralHarmonizer, HarmonyVoice};
/// let mut harmonizer = SpectralHarmonizer::major_chord();
/// ```
#[derive(Clone)]
pub struct SpectralHarmonizer {
    /// Core spectral harmonizer engine
    core: CoreSpectralHarmonizer,

    /// Effect priority (higher = later in chain)
    pub priority: u8,

    /// Whether this effect is enabled
    pub enabled: bool,

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralHarmonizer {
    /// Create a new spectral harmonizer with specified parameters
    ///
    /// # Arguments
    /// * `voices` - List of harmony voices (pitch shift + mix level)
    /// * `dry_mix` - Dry signal mix level
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// Uses default FFT size of 2048 and hop size of 512 (75% overlap).
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralHarmonizer, HarmonyVoice};
    /// let harmonizer = SpectralHarmonizer::new(
    ///     vec![HarmonyVoice::new(7.0, 0.8)],  // Perfect fifth
    ///     1.0,  // Full dry signal
    ///     44100.0
    /// );
    /// ```
    pub fn new(voices: Vec<HarmonyVoice>, dry_mix: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, voices, dry_mix, sample_rate)
    }

    /// Create a spectral harmonizer with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2)
    /// * `hop_size` - Hop size in samples
    /// * `voices` - List of harmony voices
    /// * `dry_mix` - Dry signal mix level
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralHarmonizer, HarmonyVoice};
    /// let harmonizer = SpectralHarmonizer::with_params(
    ///     4096, 1024,
    ///     vec![HarmonyVoice::new(4.0, 0.7)],
    ///     0.9,
    ///     44100.0
    /// );
    /// ```
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        voices: Vec<HarmonyVoice>,
        dry_mix: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralHarmonizer::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_voices(voices);
        core.set_dry_mix(dry_mix);

        Self {
            core,
            priority: 147, // After formant shifter
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Major chord preset (major third + perfect fifth)
    pub fn major_chord() -> Self {
        let core = CoreSpectralHarmonizer::major_chord();
        Self {
            core,
            priority: 147,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Minor chord preset (minor third + perfect fifth)
    pub fn minor_chord() -> Self {
        let core = CoreSpectralHarmonizer::minor_chord();
        Self {
            core,
            priority: 147,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Perfect fifth harmony preset
    pub fn fifth() -> Self {
        let core = CoreSpectralHarmonizer::fifth();
        Self {
            core,
            priority: 147,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Octave up preset
    pub fn octave_up() -> Self {
        let core = CoreSpectralHarmonizer::octave_up();
        Self {
            core,
            priority: 147,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Octave down preset
    pub fn octave_down() -> Self {
        let core = CoreSpectralHarmonizer::octave_down();
        Self {
            core,
            priority: 147,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Octave doubling preset (both up and down)
    pub fn octave_double() -> Self {
        let core = CoreSpectralHarmonizer::octave_double();
        Self {
            core,
            priority: 147,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Choir effect preset (multiple voices)
    pub fn choir() -> Self {
        let core = CoreSpectralHarmonizer::choir();
        Self {
            core,
            priority: 147,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Barbershop quartet preset (tight 4-part harmony)
    pub fn barbershop() -> Self {
        let core = CoreSpectralHarmonizer::barbershop();
        Self {
            core,
            priority: 147,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Thick synth preset (slight detuning)
    pub fn thick() -> Self {
        let core = CoreSpectralHarmonizer::thick();
        Self {
            core,
            priority: 147,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Add a harmony voice
    pub fn add_voice(&mut self, voice: HarmonyVoice) {
        self.core.add_voice(voice);
    }

    /// Clear all voices
    pub fn clear_voices(&mut self) {
        self.core.clear_voices();
    }

    /// Set voices from a list
    pub fn set_voices(&mut self, voices: Vec<HarmonyVoice>) {
        self.core.set_voices(voices);
    }

    /// Get reference to current voices
    pub fn voices(&self) -> &[HarmonyVoice] {
        self.core.voices()
    }

    /// Set dry signal mix level
    pub fn set_dry_mix(&mut self, mix: f32) {
        self.core.set_dry_mix(mix);
    }

    /// Get current dry mix level
    pub fn dry_mix(&self) -> f32 {
        self.core.dry_mix()
    }

    /// Process a block of audio (called by effect chain)
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

        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset internal state
    pub fn reset(&mut self) {
        self.core.reset();
    }
}

impl std::fmt::Debug for SpectralHarmonizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralHarmonizer")
            .field("voices", &self.core.voices().len())
            .field("dry_mix", &self.core.dry_mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Spectral resonator effect - emphasize specific frequencies with resonant peaks
///
/// Creates narrow resonant peaks at specified frequencies, similar to a bank of
/// very narrow bandpass filters. Perfect for pitched resonance effects, formant-like
/// sounds, bell timbres, and harmonic enhancement.
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::{SpectralResonator, Resonance};
/// let mut resonator = SpectralResonator::bell();
/// ```
#[derive(Clone)]
pub struct SpectralResonator {
    /// Core spectral resonator engine
    core: CoreSpectralResonator,

    /// Effect priority (higher = later in chain)
    pub priority: u8,

    /// Whether this effect is enabled
    pub enabled: bool,

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralResonator {
    /// Create a new spectral resonator with specified parameters
    ///
    /// # Arguments
    /// * `resonances` - List of resonance peaks
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// Uses default FFT size of 2048 and hop size of 512 (75% overlap).
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralResonator, Resonance};
    /// let resonator = SpectralResonator::new(
    ///     vec![Resonance::new(440.0, 5.0, 10.0)],
    ///     1.0,
    ///     44100.0
    /// );
    /// ```
    pub fn new(resonances: Vec<crate::synthesis::spectral::Resonance>, mix: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, resonances, mix, sample_rate)
    }

    /// Create a spectral resonator with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2)
    /// * `hop_size` - Hop size in samples
    /// * `resonances` - List of resonance peaks
    /// * `mix` - Wet/dry mix (0.0-1.0)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralResonator, Resonance};
    /// let resonator = SpectralResonator::with_params(
    ///     4096, 1024,
    ///     vec![Resonance::new(440.0, 5.0, 10.0)],
    ///     0.8,
    ///     44100.0
    /// );
    /// ```
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        resonances: Vec<crate::synthesis::spectral::Resonance>,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralResonator::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_resonances(resonances);
        core.set_mix(mix);

        Self {
            core,
            priority: 148, // After spectral harmonizer
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Pitched resonance at 440Hz (A4) with harmonics
    pub fn pitched_a440() -> Self {
        let core = CoreSpectralResonator::pitched_a440();
        Self {
            core,
            priority: 148,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Bell-like timbre with strong harmonics
    pub fn bell() -> Self {
        let core = CoreSpectralResonator::bell();
        Self {
            core,
            priority: 148,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Metallic timbre with inharmonic partials
    pub fn metallic() -> Self {
        let core = CoreSpectralResonator::metallic();
        Self {
            core,
            priority: 148,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Vocal formants (simulates vowel 'a' as in "father")
    pub fn vowel_a() -> Self {
        let core = CoreSpectralResonator::vowel_a();
        Self {
            core,
            priority: 148,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Vocal formants (simulates vowel 'e' as in "beet")
    pub fn vowel_e() -> Self {
        let core = CoreSpectralResonator::vowel_e();
        Self {
            core,
            priority: 148,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Vocal formants (simulates vowel 'i' as in "beat")
    pub fn vowel_i() -> Self {
        let core = CoreSpectralResonator::vowel_i();
        Self {
            core,
            priority: 148,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Vocal formants (simulates vowel 'o' as in "boat")
    pub fn vowel_o() -> Self {
        let core = CoreSpectralResonator::vowel_o();
        Self {
            core,
            priority: 148,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Vocal formants (simulates vowel 'u' as in "boot")
    pub fn vowel_u() -> Self {
        let core = CoreSpectralResonator::vowel_u();
        Self {
            core,
            priority: 148,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Gentle harmonic enhancement
    pub fn gentle() -> Self {
        let core = CoreSpectralResonator::gentle();
        Self {
            core,
            priority: 148,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Comb filter effect
    pub fn comb() -> Self {
        let core = CoreSpectralResonator::comb();
        Self {
            core,
            priority: 148,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Add a resonance peak
    pub fn add_resonance(&mut self, resonance: crate::synthesis::spectral::Resonance) {
        self.core.add_resonance(resonance);
    }

    /// Clear all resonances
    pub fn clear_resonances(&mut self) {
        self.core.clear_resonances();
    }

    /// Set resonances from a list
    pub fn set_resonances(&mut self, resonances: Vec<crate::synthesis::spectral::Resonance>) {
        self.core.set_resonances(resonances);
    }

    /// Get reference to current resonances
    pub fn resonances(&self) -> &[crate::synthesis::spectral::Resonance] {
        self.core.resonances()
    }

    /// Set harmonic series resonances
    pub fn set_harmonic_series(&mut self, fundamental: f32, num_harmonics: usize, gain: f32, q: f32) {
        self.core.set_harmonic_series(fundamental, num_harmonics, gain, q);
    }

    /// Set wet/dry mix
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Get current mix level
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Process a block of audio (called by effect chain)
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

        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset internal state
    pub fn reset(&mut self) {
        self.core.reset();
    }
}

impl std::fmt::Debug for SpectralResonator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralResonator")
            .field("resonances", &self.core.resonances().len())
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Spectral panner effect - pan different frequencies to different stereo positions
///
/// Creates spatial effects by positioning different frequency ranges at different
/// points in the stereo field. Perfect for game audio where you want frequency-based
/// spatial positioning (e.g., low rumble centered, high sparkles wide).
///
/// **Important**: This is a block-based effect that requires buffering.
/// It processes audio in frames for STFT analysis.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::{SpectralPanner, PanPoint};
/// let mut panner = SpectralPanner::bass_center();
/// ```
#[derive(Clone)]
pub struct SpectralPanner {
    /// Core spectral panner engine
    core: CoreSpectralPanner,

    /// Effect priority (higher = later in chain)
    pub priority: u8,

    /// Whether this effect is enabled
    pub enabled: bool,

    /// Reusable input buffer to avoid allocation per frame
    input_buffer: Vec<f32>,
}

impl SpectralPanner {
    /// Create a new spectral panner with specified pan points
    ///
    /// # Arguments
    /// * `pan_points` - List of frequency-to-pan mapping points
    /// * `mix` - Wet/dry mix (0.0 = dry, 1.0 = wet)
    /// * `sample_rate` - Audio sample rate in Hz
    ///
    /// Uses default FFT size of 2048 and hop size of 512 (75% overlap).
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::effects::{SpectralPanner, PanPoint};
    /// let panner = SpectralPanner::new(
    ///     vec![PanPoint::new(100.0, 0.0), PanPoint::new(8000.0, 0.8)],
    ///     1.0,
    ///     44100.0
    /// );
    /// ```
    pub fn new(pan_points: Vec<crate::synthesis::spectral::PanPoint>, mix: f32, sample_rate: f32) -> Self {
        Self::with_params(2048, 512, pan_points, mix, sample_rate)
    }

    /// Create a spectral panner with custom FFT parameters
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2)
    /// * `hop_size` - Hop size in samples
    /// * `pan_points` - List of frequency-to-pan mapping points
    /// * `mix` - Wet/dry mix (0.0-1.0)
    /// * `sample_rate` - Audio sample rate in Hz
    pub fn with_params(
        fft_size: usize,
        hop_size: usize,
        pan_points: Vec<crate::synthesis::spectral::PanPoint>,
        mix: f32,
        sample_rate: f32,
    ) -> Self {
        let mut core = CoreSpectralPanner::new(fft_size, hop_size, WindowType::Hann, sample_rate);
        core.set_pan_points(pan_points);
        core.set_mix(mix);

        Self {
            core,
            priority: 149, // After spectral resonator
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// All frequencies centered (bypass)
    pub fn center() -> Self {
        let core = CoreSpectralPanner::center();
        Self {
            core,
            priority: 149,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Bass centered, highs progressively wider
    pub fn bass_center() -> Self {
        let core = CoreSpectralPanner::bass_center();
        Self {
            core,
            priority: 149,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Highs wide, everything else centered
    pub fn highs_wide() -> Self {
        let core = CoreSpectralPanner::highs_wide();
        Self {
            core,
            priority: 149,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Low frequencies left, high frequencies right (sweep)
    pub fn low_left_high_right() -> Self {
        let core = CoreSpectralPanner::low_left_high_right();
        Self {
            core,
            priority: 149,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Reverse sweep: low right, high left
    pub fn low_right_high_left() -> Self {
        let core = CoreSpectralPanner::low_right_high_left();
        Self {
            core,
            priority: 149,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Mids wide, bass and treble centered
    pub fn mid_wide() -> Self {
        let core = CoreSpectralPanner::mid_wide();
        Self {
            core,
            priority: 149,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Circular/spiral effect (for game audio ambience)
    pub fn circular() -> Self {
        let core = CoreSpectralPanner::circular();
        Self {
            core,
            priority: 149,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Gentle widening for subtle spatial enhancement
    pub fn gentle() -> Self {
        let core = CoreSpectralPanner::gentle();
        Self {
            core,
            priority: 149,
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Add a pan point
    pub fn add_pan_point(&mut self, point: crate::synthesis::spectral::PanPoint) {
        self.core.add_pan_point(point);
    }

    /// Clear all pan points
    pub fn clear_pan_points(&mut self) {
        self.core.clear_pan_points();
    }

    /// Set pan points from a list
    pub fn set_pan_points(&mut self, pan_points: Vec<crate::synthesis::spectral::PanPoint>) {
        self.core.set_pan_points(pan_points);
    }

    /// Get reference to current pan points
    pub fn pan_points(&self) -> &[crate::synthesis::spectral::PanPoint] {
        self.core.pan_points()
    }

    /// Set wet/dry mix
    pub fn set_mix(&mut self, mix: f32) {
        self.core.set_mix(mix);
    }

    /// Get current mix level
    pub fn mix(&self) -> f32 {
        self.core.mix()
    }

    /// Process a block of audio (called by effect chain)
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

        // Reuse buffer to avoid allocation
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(buffer);
        self.core.process(buffer, &self.input_buffer);
    }

    /// Reset internal state
    pub fn reset(&mut self) {
        self.core.reset();
    }
}

impl std::fmt::Debug for SpectralPanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectralPanner")
            .field("pan_points", &self.core.pan_points().len())
            .field("mix", &self.core.mix())
            .field("priority", &self.priority)
            .field("enabled", &self.enabled)
            .finish()
    }
}
