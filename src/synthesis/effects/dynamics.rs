use crate::synthesis::automation::Automation;
use crate::track::{
    PRIORITY_EARLY, PRIORITY_LAST, TrackId, BusId,
};

/// Standard audio sample rate
const DEFAULT_SAMPLE_RATE: f32 = 44100.0;

/// Sidechain source for dynamic effects
///
/// Specifies which external signal should control a dynamic effect (e.g., compressor).
/// The sidechain source's envelope is monitored to trigger compression/gating on the
/// target signal.
///
/// # Example
/// ```
/// # use tunes::synthesis::effects::{Compressor, SidechainSource};
/// // Bass compressor ducks when kick hits
/// let compressor = Compressor::new(0.6, 4.0, 0.01, 0.1, 44100.0)
///     .with_sidechain_track("kick");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum SidechainSource {
    /// Sidechain from a specific track by name
    Track(String),
    /// Sidechain from an entire bus by name
    Bus(String),
}

/// Resolved sidechain source using integer IDs for performance
///
/// This is an internal type used during audio rendering. The user-facing API uses
/// `SidechainSource` with string names, which are resolved to integer IDs when
/// converting a Composition to a Mixer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResolvedSidechainSource {
    /// Sidechain from a specific track by ID
    Track(TrackId),
    /// Sidechain from an entire bus by ID
    Bus(BusId),
}

/// Frequency band for multiband compression
///
/// Defines a frequency range and its associated compressor settings.
/// Used with `Compressor::with_multiband()` or `Compressor::with_multibands()`.
#[derive(Debug, Clone)]
pub struct CompressorBand {
    pub low_freq: f32,      // Lower frequency bound in Hz
    pub high_freq: f32,     // Upper frequency bound in Hz
    pub compressor: Compressor, // Compressor settings for this band

    // Bandpass filter state (Butterworth 2nd order)
    low_z1: f32,
    low_z2: f32,
    high_z1: f32,
    high_z2: f32,
}

impl CompressorBand {
    /// Create a new frequency band with compressor settings
    ///
    /// # Arguments
    /// * `low_freq` - Lower frequency bound in Hz (e.g., 80.0 for bass)
    /// * `high_freq` - Upper frequency bound in Hz (e.g., 200.0 for bass)
    /// * `compressor` - Compressor to apply to this frequency band
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::synthesis::effects::{Compressor, CompressorBand};
    /// let bass_band = CompressorBand::new(
    ///     40.0,   // Low freq
    ///     150.0,  // High freq
    ///     Compressor::new(0.3, 4.0, 0.005, 0.05, 1.0) // Tight compression
    /// );
    /// ```
    pub fn new(low_freq: f32, high_freq: f32, compressor: Compressor) -> Self {
        Self {
            low_freq,
            high_freq,
            compressor,
            low_z1: 0.0,
            low_z2: 0.0,
            high_z1: 0.0,
            high_z2: 0.0,
        }
    }

    /// Process a sample through this band's filter and compressor
    #[inline]
    fn process_sample(&mut self, input: f32, sample_rate: f32, time: f32, sample_count: u64) -> f32 {
        // Apply bandpass filter (highpass then lowpass)
        let filtered = self.bandpass(input, sample_rate);

        // Compress the filtered signal
        self.compressor.process(filtered, sample_rate, time, sample_count, None)
    }

    /// Simple 2nd-order Butterworth bandpass filter
    #[inline]
    fn bandpass(&mut self, input: f32, sample_rate: f32) -> f32 {
        // Highpass at low_freq
        let omega_low = 2.0 * std::f32::consts::PI * self.low_freq / sample_rate;
        let cos_omega_low = omega_low.cos();
        let alpha_low = omega_low.sin() / (2.0 * 0.707); // Q = 0.707 for Butterworth

        let b0_low = (1.0 + cos_omega_low) / 2.0;
        let b1_low = -(1.0 + cos_omega_low);
        let b2_low = (1.0 + cos_omega_low) / 2.0;
        let a0_low = 1.0 + alpha_low;
        let a1_low = -2.0 * cos_omega_low;
        let a2_low = 1.0 - alpha_low;

        let high_passed = (b0_low * input + b1_low * self.low_z1 + b2_low * self.low_z2
            - a1_low * self.low_z1 - a2_low * self.low_z2) / a0_low;

        self.low_z2 = self.low_z1;
        self.low_z1 = high_passed;

        // Lowpass at high_freq
        let omega_high = 2.0 * std::f32::consts::PI * self.high_freq / sample_rate;
        let cos_omega_high = omega_high.cos();
        let alpha_high = omega_high.sin() / (2.0 * 0.707);

        let b0_high = (1.0 - cos_omega_high) / 2.0;
        let b1_high = 1.0 - cos_omega_high;
        let b2_high = (1.0 - cos_omega_high) / 2.0;
        let a0_high = 1.0 + alpha_high;
        let a1_high = -2.0 * cos_omega_high;
        let a2_high = 1.0 - alpha_high;

        let band_passed = (b0_high * high_passed + b1_high * self.high_z1 + b2_high * self.high_z2
            - a1_high * self.high_z1 - a2_high * self.high_z2) / a0_high;

        self.high_z2 = self.high_z1;
        self.high_z1 = band_passed;

        band_passed
    }
}

/// Compressor - dynamic range compression
#[derive(Debug, Clone)]
pub struct Compressor {
    pub threshold: f32, // Threshold in amplitude 0.0-1.0 (NOT dB! 0.3 ≈ -10dB, 0.5 ≈ -6dB)
    pub ratio: f32,     // Compression ratio (1.0 = no compression, 10.0 = heavy)
    pub attack: f32,    // Attack time in seconds
    pub release: f32,   // Release time in seconds
    pub makeup_gain: f32, // Makeup gain to compensate for volume reduction
    pub priority: u8,   // Processing priority (lower = earlier in signal chain)
    envelope: f32,

    // Sidechaining support (user-facing API)
    pub sidechain_source: Option<SidechainSource>, // External signal to trigger compression (by name)

    // Resolved sidechain source (internal, used during rendering)
    pub(crate) resolved_sidechain_source: Option<ResolvedSidechainSource>,

    // Multiband compression support
    bands: Option<Vec<CompressorBand>>,

    // Automation (optional)
    threshold_automation: Option<Automation>,
    ratio_automation: Option<Automation>,
    attack_automation: Option<Automation>,
    release_automation: Option<Automation>,
    makeup_gain_automation: Option<Automation>,
}

impl Compressor {
    /// Create a new compressor
    ///
    /// # Arguments
    /// * `threshold` - Level above which compression starts in amplitude (0.0 to 1.0, NOT dB!)
    ///   Typical values: 0.5 = gentle, 0.3 = moderate, 0.2 = aggressive
    /// * `ratio` - Compression ratio (2.0 = gentle, 10.0 = heavy limiting)
    /// * `attack` - Attack time in seconds (typical: 0.001 to 0.1)
    /// * `release` - Release time in seconds (typical: 0.05 to 0.5)
    /// * `makeup_gain` - Output gain multiplier (1.0 to 4.0)
    pub fn new(threshold: f32, ratio: f32, attack: f32, release: f32, makeup_gain: f32) -> Self {
        Self {
            threshold: threshold.clamp(0.0, 1.0),
            ratio: ratio.max(1.0),
            attack: attack.max(0.001),
            release: release.max(0.001),
            makeup_gain: makeup_gain.max(0.1),
            priority: PRIORITY_EARLY, // Compressor typically early in chain
            envelope: 0.0,
            sidechain_source: None,
            resolved_sidechain_source: None,
            bands: None,
            threshold_automation: None,
            ratio_automation: None,
            attack_automation: None,
            release_automation: None,
            makeup_gain_automation: None,
        }
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the threshold parameter
    pub fn with_threshold_automation(mut self, automation: Automation) -> Self {
        self.threshold_automation = Some(automation);
        self
    }

    /// Add automation for the ratio parameter
    pub fn with_ratio_automation(mut self, automation: Automation) -> Self {
        self.ratio_automation = Some(automation);
        self
    }

    /// Add automation for the attack parameter
    pub fn with_attack_automation(mut self, automation: Automation) -> Self {
        self.attack_automation = Some(automation);
        self
    }

    /// Add automation for the release parameter
    pub fn with_release_automation(mut self, automation: Automation) -> Self {
        self.release_automation = Some(automation);
        self
    }

    /// Add automation for the makeup gain parameter
    pub fn with_makeup_gain_automation(mut self, automation: Automation) -> Self {
        self.makeup_gain_automation = Some(automation);
        self
    }

    /// Configure sidechain from a specific track
    ///
    /// Duck this signal when the specified track is loud. The compressor will
    /// monitor the sidechain track's envelope instead of its own signal to
    /// decide when to compress.
    ///
    /// Common use: Duck bass when kick hits.
    ///
    /// # Arguments
    /// * `track_name` - Name of the track to use as sidechain source
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::synthesis::effects::Compressor;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("kick")
    ///     .bus("drums")
    ///     .drum(DrumType::Kick);
    ///
    /// comp.track("bass")
    ///     .bus("bass")
    ///     .notes(&[C2], 0.5);
    ///
    /// let mut mixer = comp.into_mixer();
    ///
    /// // Bass ducks when kick hits
    /// mixer.bus("bass")
    ///     .compressor(
    ///         Compressor::new(0.6, 4.0, 0.01, 0.1, 1.0)
    ///             .with_sidechain_track("kick")
    ///     );
    /// ```
    pub fn with_sidechain_track(mut self, track_name: &str) -> Self {
        self.sidechain_source = Some(SidechainSource::Track(track_name.to_string()));
        self
    }

    /// Configure sidechain from an entire bus
    ///
    /// Duck this signal when the specified bus is loud. Useful for ducking
    /// a bus based on another bus (e.g., duck synths when drums bus is active).
    ///
    /// # Arguments
    /// * `bus_name` - Name of the bus to use as sidechain source
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::synthesis::effects::Compressor;
    /// # let mut mixer = tunes::track::Mixer::new(Tempo::new(120.0));
    /// // Synths duck when entire drums bus is loud
    /// mixer.bus("synths")
    ///     .compressor(
    ///         Compressor::new(0.5, 3.0, 0.005, 0.05, 1.0)
    ///             .with_sidechain_bus("drums")
    ///     );
    /// ```
    pub fn with_sidechain_bus(mut self, bus_name: &str) -> Self {
        self.sidechain_source = Some(SidechainSource::Bus(bus_name.to_string()));
        self
    }

    /// Add a single frequency band for multiband compression
    ///
    /// Splits the signal into frequency bands and applies different compression
    /// to each band independently. Perfect for mastering and frequency-specific dynamics.
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::synthesis::effects::{Compressor, CompressorBand};
    /// let comp = Compressor::new(0.5, 3.0, 0.01, 0.1, 1.0)
    ///     .with_multiband(CompressorBand::new(
    ///         80.0,    // Low freq
    ///         200.0,   // High freq
    ///         Compressor::new(0.3, 4.0, 0.005, 0.05, 1.0) // Tight bass comp
    ///     ));
    /// ```
    pub fn with_multiband(mut self, band: CompressorBand) -> Self {
        if self.bands.is_none() {
            self.bands = Some(Vec::new());
        }
        self.bands.as_mut().unwrap().push(band);
        self
    }

    /// Add multiple frequency bands for multiband compression
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::synthesis::effects::{Compressor, CompressorBand};
    /// let bands = vec![
    ///     CompressorBand::new(0.0, 200.0,
    ///         Compressor::new(0.3, 4.0, 0.005, 0.05, 1.0)), // Bass
    ///     CompressorBand::new(200.0, 2000.0,
    ///         Compressor::new(0.5, 2.5, 0.01, 0.1, 1.0)),   // Mids
    ///     CompressorBand::new(2000.0, 20000.0,
    ///         Compressor::new(0.6, 2.0, 0.01, 0.1, 1.0)),   // Highs
    /// ];
    ///
    /// let comp = Compressor::new(0.5, 1.0, 0.01, 0.1, 1.0)
    ///     .with_multibands(bands);
    /// ```
    pub fn with_multibands(mut self, bands: Vec<CompressorBand>) -> Self {
        self.bands = Some(bands);
        self
    }

    /// Convenience: Create a 3-band multiband compressor (low/mid/high)
    ///
    /// Creates a standard 3-way split with default compression settings.
    /// Use `.with_band_low()`, `.with_band_mid()`, and `.with_band_high()`
    /// to customize each band's compression.
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # use tunes::synthesis::effects::Compressor;
    /// let comp = Compressor::multiband_3way(200.0, 2000.0)
    ///     .with_band_low(0.3, 4.0)    // Tight bass control
    ///     .with_band_mid(0.5, 2.5)    // Gentle mids
    ///     .with_band_high(0.6, 2.0);  // Transparent highs
    /// ```
    pub fn multiband_3way(low_mid_crossover: f32, mid_high_crossover: f32) -> Self {
        let bands = vec![
            CompressorBand::new(
                0.0,
                low_mid_crossover,
                Compressor::new(0.5, 3.0, 0.01, 0.1, 1.0),
            ),
            CompressorBand::new(
                low_mid_crossover,
                mid_high_crossover,
                Compressor::new(0.5, 3.0, 0.01, 0.1, 1.0),
            ),
            CompressorBand::new(
                mid_high_crossover,
                20000.0,
                Compressor::new(0.5, 3.0, 0.01, 0.1, 1.0),
            ),
        ];

        Self {
            threshold: 0.5,
            ratio: 1.0, // Disabled when using bands
            attack: 0.01,
            release: 0.1,
            makeup_gain: 1.0,
            priority: PRIORITY_EARLY,
            envelope: 0.0,
            sidechain_source: None,
            resolved_sidechain_source: None,
            bands: Some(bands),
            threshold_automation: None,
            ratio_automation: None,
            attack_automation: None,
            release_automation: None,
            makeup_gain_automation: None,
        }
    }

    /// Adjust low band compression (for use with `multiband_3way`)
    pub fn with_band_low(mut self, threshold: f32, ratio: f32) -> Self {
        if let Some(bands) = &mut self.bands {
            if let Some(band) = bands.get_mut(0) {
                band.compressor.threshold = threshold;
                band.compressor.ratio = ratio;
            }
        }
        self
    }

    /// Adjust mid band compression (for use with `multiband_3way`)
    pub fn with_band_mid(mut self, threshold: f32, ratio: f32) -> Self {
        if let Some(bands) = &mut self.bands {
            if let Some(band) = bands.get_mut(1) {
                band.compressor.threshold = threshold;
                band.compressor.ratio = ratio;
            }
        }
        self
    }

    /// Adjust high band compression (for use with `multiband_3way`)
    pub fn with_band_high(mut self, threshold: f32, ratio: f32) -> Self {
        if let Some(bands) = &mut self.bands {
            if let Some(band) = bands.get_mut(2) {
                band.compressor.threshold = threshold;
                band.compressor.ratio = ratio;
            }
        }
        self
    }

    /// Process a single sample at given sample rate
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    /// * `sidechain_envelope` - Optional external envelope for sidechaining (overrides input level detection)
    #[inline]
    pub fn process(&mut self, input: f32, sample_rate: f32, time: f32, sample_count: u64, sidechain_envelope: Option<f32>) -> f32 {
        // If multiband is enabled, process through bands instead
        if let Some(bands) = &mut self.bands {
            let mut output = 0.0;
            for band in bands.iter_mut() {
                output += band.process_sample(input, sample_rate, time, sample_count);
            }
            return output.clamp(-2.0, 2.0);
        }

        // Standard single-band compression below
        // Quantized automation lookups (every 64 samples = 1.45ms @ 44.1kHz)
        // Use bitwise AND instead of modulo for power-of-2
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.threshold_automation {
                self.threshold = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.ratio_automation {
                self.ratio = auto.value_at(time).max(1.0);
            }
            if let Some(auto) = &self.attack_automation {
                self.attack = auto.value_at(time).max(0.001);
            }
            if let Some(auto) = &self.release_automation {
                self.release = auto.value_at(time).max(0.001);
            }
            if let Some(auto) = &self.makeup_gain_automation {
                self.makeup_gain = auto.value_at(time).max(0.1);
            }
        }

        // Use sidechain envelope if provided, otherwise use input level
        let input_level = sidechain_envelope.unwrap_or_else(|| input.abs());

        // Envelope follower with pre-computed coefficients
        let attack_coeff = (-1.0 / (self.attack * sample_rate)).exp();
        let release_coeff = (-1.0 / (self.release * sample_rate)).exp();

        // Use FMA for envelope calculation
        let coeff = if input_level > self.envelope {
            attack_coeff
        } else {
            release_coeff
        };
        self.envelope = self.envelope.mul_add(coeff, input_level * (1.0 - coeff));

        // Clamp envelope to prevent runaway values
        self.envelope = self.envelope.clamp(0.0, 2.0);

        // Calculate gain reduction
        let gain = if self.envelope > self.threshold {
            let over_threshold = self.envelope / self.threshold.max(0.001); // Prevent division by zero
            let compressed = over_threshold.powf(1.0 / self.ratio);
            (compressed * self.threshold / self.envelope).clamp(0.0, 1.0)
        } else {
            1.0
        };

        // Apply compression and makeup gain using FMA, clamp output to prevent clipping
        let output = input * gain * self.makeup_gain;
        output.clamp(-2.0, 2.0)
    }

    /// Process a stereo sample with properly linked compression
    ///
    /// Uses the maximum level of both channels for gain detection, then applies
    /// the same gain reduction to both channels. This prevents stereo image shifts.
    ///
    /// # Arguments
    /// * `left` - Left channel input
    /// * `right` - Right channel input
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    /// * `sidechain_envelope` - Optional external envelope for sidechaining
    ///
    /// # Returns
    /// Tuple of (left_output, right_output)
    #[inline]
    pub fn process_stereo_linked(
        &mut self,
        left: f32,
        right: f32,
        sample_rate: f32,
        time: f32,
        sample_count: u64,
        sidechain_envelope: Option<f32>,
    ) -> (f32, f32) {
        // Quantized automation lookups (every 64 samples = 1.45ms @ 44.1kHz)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.threshold_automation {
                self.threshold = auto.value_at(time).clamp(0.0, 1.0);
            }
            if let Some(auto) = &self.ratio_automation {
                self.ratio = auto.value_at(time).max(1.0);
            }
            if let Some(auto) = &self.attack_automation {
                self.attack = auto.value_at(time).max(0.001);
            }
            if let Some(auto) = &self.release_automation {
                self.release = auto.value_at(time).max(0.001);
            }
            if let Some(auto) = &self.makeup_gain_automation {
                self.makeup_gain = auto.value_at(time).max(0.1);
            }
        }

        // Use sidechain envelope if provided, otherwise use max of both channels for detection
        let input_level = sidechain_envelope.unwrap_or_else(|| left.abs().max(right.abs()));

        // Envelope follower with pre-computed coefficients
        let attack_coeff = (-1.0 / (self.attack * sample_rate)).exp();
        let release_coeff = (-1.0 / (self.release * sample_rate)).exp();

        // Use FMA for envelope calculation
        let coeff = if input_level > self.envelope {
            attack_coeff
        } else {
            release_coeff
        };
        self.envelope = self.envelope.mul_add(coeff, input_level * (1.0 - coeff));

        // Clamp envelope to prevent runaway values
        self.envelope = self.envelope.clamp(0.0, 2.0);

        // Calculate gain reduction (same for both channels)
        let gain = if self.envelope > self.threshold {
            let over_threshold = self.envelope / self.threshold.max(0.001);
            let compressed = over_threshold.powf(1.0 / self.ratio);
            (compressed * self.threshold / self.envelope).clamp(0.0, 1.0)
        } else {
            1.0
        };

        // Apply same gain to both channels with makeup gain
        let left_out = (left * gain * self.makeup_gain).clamp(-2.0, 2.0);
        let right_out = (right * gain * self.makeup_gain).clamp(-2.0, 2.0);

        (left_out, right_out)
    }

    /// Process a block of samples
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    /// * `sidechain_envelope` - Optional external envelope for sidechaining
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], sample_rate: f32, time: f32, sample_count: u64, sidechain_envelope: Option<f32>) {
        let time_delta = 1.0 / sample_rate;
        for (i, sample) in buffer.iter_mut().enumerate() {
            let current_time = time + (i as f32 * time_delta);
            let current_sample_count = sample_count + i as u64;
            *sample = self.process(*sample, sample_rate, current_time, current_sample_count, sidechain_envelope);
        }
    }

    /// Reset the compressor state
    pub fn reset(&mut self) {
        self.envelope = 0.0;
    }

    // ========== PRESETS ==========

    /// Gentle compression - transparent, barely noticeable (2:1 ratio)
    pub fn gentle() -> Self {
        Self::new(0.5, 2.0, 0.01, 0.1, DEFAULT_SAMPLE_RATE)
    }

    /// Vocal compression - fast attack for controlling vocals (4:1 ratio)
    pub fn vocal() -> Self {
        Self::new(0.4, 4.0, 0.005, 0.05, DEFAULT_SAMPLE_RATE)
    }

    /// Drum bus compression - punchy, adds glue to drums (4:1 ratio)
    pub fn drum_bus() -> Self {
        Self::new(0.6, 4.0, 0.01, 0.15, DEFAULT_SAMPLE_RATE)
    }

    /// Bass compression - evens out bass notes (6:1 ratio)
    pub fn bass() -> Self {
        Self::new(0.5, 6.0, 0.02, 0.2, DEFAULT_SAMPLE_RATE)
    }

    /// Master compression - gentle glue for final mix (2.5:1 ratio)
    pub fn master() -> Self {
        Self::new(0.6, 2.5, 0.01, 0.1, DEFAULT_SAMPLE_RATE)
    }

    /// Limiter - brick wall limiting (20:1 ratio)
    pub fn limiter() -> Self {
        Self::new(0.8, 20.0, 0.001, 0.05, DEFAULT_SAMPLE_RATE)
    }

    /// Aggressive compression - heavy squashing (8:1 ratio)
    pub fn aggressive() -> Self {
        Self::new(0.3, 8.0, 0.005, 0.08, DEFAULT_SAMPLE_RATE)
    }
}

/// Gate - noise gate / expander
///
/// Reduces the level of signals below a threshold, useful for removing
/// background noise or creating rhythmic gating effects.
#[derive(Debug, Clone)]
pub struct Gate {
    pub threshold: f32, // Threshold in dB (e.g., -40.0)
    pub ratio: f32,     // Expansion ratio (typically 10:1 to ∞:1, where ∞ = hard gate)
    pub attack: f32,    // Attack time in seconds
    pub release: f32,   // Release time in seconds
    pub priority: u8,   // Processing priority
    envelope: f32,      // Current envelope value (0.0 to 1.0)
    _sample_rate: f32,

    // Automation (optional)
    threshold_automation: Option<Automation>,
    ratio_automation: Option<Automation>,
}

impl Gate {
    /// Create a new gate effect
    ///
    /// # Arguments
    /// * `threshold` - Threshold in dB (signals below this are reduced)
    /// * `ratio` - Expansion ratio (10.0 = 10:1, f32::INFINITY = hard gate)
    /// * `attack` - Attack time in seconds (how quickly gate opens)
    /// * `release` - Release time in seconds (how quickly gate closes)
    /// * `sample_rate` - Audio sample rate in Hz
    pub fn with_sample_rate(
        threshold: f32,
        ratio: f32,
        attack: f32,
        release: f32,
        sample_rate: f32,
    ) -> Self {
        Self {
            threshold,
            ratio: ratio.max(1.0),
            attack: attack.max(0.0001),
            release: release.max(0.001),
            priority: PRIORITY_EARLY, // Gates typically go early in the chain
            envelope: 0.0,
            _sample_rate: sample_rate,
            threshold_automation: None,
            ratio_automation: None,
        }
    }

    /// Create a gate with default sample rate (44100 Hz)
    pub fn new(threshold: f32, ratio: f32, attack: f32, release: f32) -> Self {
        Self::with_sample_rate(threshold, ratio, attack, release, DEFAULT_SAMPLE_RATE)
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the threshold parameter
    pub fn with_threshold_automation(mut self, automation: Automation) -> Self {
        self.threshold_automation = Some(automation);
        self
    }

    /// Add automation for the ratio parameter
    pub fn with_ratio_automation(mut self, automation: Automation) -> Self {
        self.ratio_automation = Some(automation);
        self
    }

    /// Process a single sample
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, sample_rate: f32, time: f32, sample_count: u64) -> f32 {
        // Quantized automation lookups (every 64 samples)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.threshold_automation {
                self.threshold = auto.value_at(time);
            }
            if let Some(auto) = &self.ratio_automation {
                self.ratio = auto.value_at(time).max(1.0);
            }
        }

        // Convert input to dB
        let input_db = if input.abs() > 0.0001 {
            20.0 * input.abs().log10()
        } else {
            -100.0 // Very quiet = -100 dB
        };

        // Determine target envelope based on threshold
        let target_envelope = if input_db > self.threshold {
            1.0 // Above threshold: gate open
        } else {
            // Below threshold: apply expansion/gating
            let db_below = self.threshold - input_db;
            let expansion = db_below * (self.ratio - 1.0) / self.ratio;
            10.0_f32.powf(-expansion / 20.0) // Convert back to linear
        };

        // Smooth envelope with attack/release
        let coeff = if target_envelope > self.envelope {
            // Attack (gate opening)
            (-1.0 / (self.attack * sample_rate)).exp()
        } else {
            // Release (gate closing)
            (-1.0 / (self.release * sample_rate)).exp()
        };

        self.envelope = target_envelope + coeff * (self.envelope - target_envelope);

        // Apply gating
        input * self.envelope
    }

    /// Process a block of samples
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], sample_rate: f32, time: f32, sample_count: u64) {
        let time_delta = 1.0 / sample_rate;
        for (i, sample) in buffer.iter_mut().enumerate() {
            let current_time = time + (i as f32 * time_delta);
            let current_sample_count = sample_count + i as u64;
            *sample = self.process(*sample, sample_rate, current_time, current_sample_count);
        }
    }

    /// Reset the gate state
    pub fn reset(&mut self) {
        self.envelope = 0.0;
    }

    // ========== PRESETS ==========

    /// Gentle gate - subtle noise reduction (-35 dB threshold)
    pub fn gentle() -> Self {
        Self::with_sample_rate(-35.0, 4.0, 0.001, 0.05, DEFAULT_SAMPLE_RATE)
    }

    /// Standard gate - balanced noise control (-40 dB threshold)
    pub fn standard() -> Self {
        Self::with_sample_rate(-40.0, 10.0, 0.001, 0.05, DEFAULT_SAMPLE_RATE)
    }

    /// Aggressive gate - hard gating for dramatic effect (-30 dB, high ratio)
    pub fn aggressive() -> Self {
        Self::with_sample_rate(-30.0, f32::INFINITY, 0.0001, 0.02, DEFAULT_SAMPLE_RATE)
    }

    /// Drum gate - fast attack/release for drums (-45 dB)
    pub fn drum() -> Self {
        Self::with_sample_rate(-45.0, 20.0, 0.0001, 0.03, DEFAULT_SAMPLE_RATE)
    }

    /// Vocal gate - moderate gating for vocals (-38 dB)
    pub fn vocal() -> Self {
        Self::with_sample_rate(-38.0, 8.0, 0.002, 0.08, DEFAULT_SAMPLE_RATE)
    }
}

/// Limiter - brick-wall peak limiter
///
/// Prevents signal from exceeding a threshold, acting as a safety net
/// against clipping. Typically used as the final stage in the signal chain.
#[derive(Debug, Clone)]
pub struct Limiter {
    pub threshold: f32,  // Threshold in dB (e.g., -0.1 dB)
    pub release: f32,    // Release time in seconds
    pub priority: u8,    // Processing priority
    gain_reduction: f32, // Current gain reduction (0.0 to 1.0)
    _sample_rate: f32,

    // Automation (optional)
    threshold_automation: Option<Automation>,
}

impl Limiter {
    /// Create a new limiter effect
    ///
    /// # Arguments
    /// * `threshold` - Threshold in dB (signals above this are limited)
    /// * `release` - Release time in seconds (how quickly limiter recovers)
    /// * `sample_rate` - Audio sample rate in Hz
    pub fn with_sample_rate(threshold: f32, release: f32, sample_rate: f32) -> Self {
        Self {
            threshold,
            release: release.max(0.001),
            priority: PRIORITY_LAST, // Limiters go last to catch peaks
            gain_reduction: 1.0,
            _sample_rate: sample_rate,
            threshold_automation: None,
        }
    }

    /// Create a limiter with default sample rate (44100 Hz)
    pub fn new(threshold: f32, release: f32) -> Self {
        Self::with_sample_rate(threshold, release, DEFAULT_SAMPLE_RATE)
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the threshold parameter
    pub fn with_threshold_automation(mut self, automation: Automation) -> Self {
        self.threshold_automation = Some(automation);
        self
    }

    /// Process a single sample
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, sample_rate: f32, time: f32, sample_count: u64) -> f32 {
        // Quantized automation lookups (every 64 samples)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.threshold_automation {
                self.threshold = auto.value_at(time);
            }
        }

        // Convert threshold from dB to linear
        let threshold_linear = 10.0_f32.powf(self.threshold / 20.0);

        // Detect peak
        let input_abs = input.abs();

        // Calculate required gain reduction
        let target_gain = if input_abs > threshold_linear {
            threshold_linear / input_abs
        } else {
            1.0
        };

        // Apply gain reduction with instant attack and release envelope
        // Instant attack (0 ms) for true peak limiting
        if target_gain < self.gain_reduction {
            self.gain_reduction = target_gain;
        } else {
            // Smooth release
            let release_coeff = (-1.0 / (self.release * sample_rate)).exp();
            self.gain_reduction = target_gain + release_coeff * (self.gain_reduction - target_gain);
        }

        // Apply limiting
        input * self.gain_reduction
    }

    /// Process a stereo sample with properly linked limiting
    ///
    /// Uses the maximum level of both channels for peak detection, then applies
    /// the same gain reduction to both channels. This prevents stereo image shifts.
    ///
    /// # Arguments
    /// * `left` - Left channel input
    /// * `right` - Right channel input
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    ///
    /// # Returns
    /// Tuple of (left_output, right_output)
    #[inline]
    pub fn process_stereo_linked(
        &mut self,
        left: f32,
        right: f32,
        sample_rate: f32,
        time: f32,
        sample_count: u64,
    ) -> (f32, f32) {
        // Quantized automation lookups (every 64 samples)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.threshold_automation {
                self.threshold = auto.value_at(time);
            }
        }

        // Convert threshold from dB to linear
        let threshold_linear = 10.0_f32.powf(self.threshold / 20.0);

        // Detect peak from both channels
        let peak = left.abs().max(right.abs());

        // Calculate required gain reduction
        let target_gain = if peak > threshold_linear {
            threshold_linear / peak
        } else {
            1.0
        };

        // Apply gain reduction with instant attack and release envelope
        if target_gain < self.gain_reduction {
            self.gain_reduction = target_gain;
        } else {
            // Smooth release
            let release_coeff = (-1.0 / (self.release * sample_rate)).exp();
            self.gain_reduction = target_gain + release_coeff * (self.gain_reduction - target_gain);
        }

        // Apply same limiting gain to both channels
        (left * self.gain_reduction, right * self.gain_reduction)
    }

    /// Process a block of samples
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Starting time in seconds (for automation)
    /// * `sample_count` - Starting sample counter (for quantized automation lookups)
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], sample_rate: f32, time: f32, sample_count: u64) {
        let time_delta = 1.0 / sample_rate;
        for (i, sample) in buffer.iter_mut().enumerate() {
            let current_time = time + (i as f32 * time_delta);
            let current_sample_count = sample_count + i as u64;
            *sample = self.process(*sample, sample_rate, current_time, current_sample_count);
        }
    }

    /// Get the current gain reduction in dB
    ///
    /// Useful for metering how much limiting is occurring
    pub fn get_gain_reduction_db(&self) -> f32 {
        if self.gain_reduction > 0.0 {
            20.0 * self.gain_reduction.log10()
        } else {
            -100.0
        }
    }

    /// Reset the limiter state
    pub fn reset(&mut self) {
        self.gain_reduction = 1.0;
    }

    // ========== PRESETS ==========

    /// Transparent limiter - very light limiting (-0.5 dB)
    pub fn transparent() -> Self {
        Self::with_sample_rate(-0.5, 0.1, DEFAULT_SAMPLE_RATE)
    }

    /// Standard limiter - balanced protection (-0.3 dB)
    pub fn standard() -> Self {
        Self::with_sample_rate(-0.3, 0.05, DEFAULT_SAMPLE_RATE)
    }

    /// Brick wall - maximum protection (-0.1 dB, fast release)
    pub fn brick_wall() -> Self {
        Self::with_sample_rate(-0.1, 0.005, DEFAULT_SAMPLE_RATE)
    }

    /// Mastering limiter - professional mastering (-0.2 dB)
    pub fn mastering() -> Self {
        Self::with_sample_rate(-0.2, 0.08, DEFAULT_SAMPLE_RATE)
    }

    /// Safety limiter - emergency protection (0.0 dB)
    pub fn safety() -> Self {
        Self::with_sample_rate(0.0, 0.01, DEFAULT_SAMPLE_RATE)
    }
}
