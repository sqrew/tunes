use crate::synthesis::automation::Automation;
use crate::track::PRIORITY_EARLY;

/// Parametric EQ - 3-band equalizer
#[derive(Debug, Clone)]
pub struct EQ {
    pub low_gain: f32,  // Low frequency gain (0.0 to 2.0, 1.0 = unity)
    pub mid_gain: f32,  // Mid frequency gain (0.0 to 2.0, 1.0 = unity)
    pub high_gain: f32, // High frequency gain (0.0 to 2.0, 1.0 = unity)
    pub low_freq: f32,  // Low band center frequency (Hz)
    pub high_freq: f32, // High band center frequency (Hz)
    pub priority: u8,   // Processing priority (lower = earlier in signal chain)
    // State variables for filters
    low_state: [f32; 2],
    mid_state: [f32; 2],
    high_state: [f32; 2],

    // Automation (optional)
    low_gain_automation: Option<Automation>,
    mid_gain_automation: Option<Automation>,
    high_gain_automation: Option<Automation>,
}

impl EQ {
    /// Create a new 3-band parametric EQ
    ///
    /// # Arguments
    /// * `low_gain` - Low frequency gain (0.5 = -6dB, 1.0 = 0dB, 2.0 = +6dB)
    /// * `mid_gain` - Mid frequency gain
    /// * `high_gain` - High frequency gain
    /// * `low_freq` - Low/mid crossover frequency (typical: 250 Hz)
    /// * `high_freq` - Mid/high crossover frequency (typical: 4000 Hz)
    pub fn new(
        low_gain: f32,
        mid_gain: f32,
        high_gain: f32,
        low_freq: f32,
        high_freq: f32,
    ) -> Self {
        Self {
            low_gain: low_gain.clamp(0.0, 4.0),
            mid_gain: mid_gain.clamp(0.0, 4.0),
            high_gain: high_gain.clamp(0.0, 4.0),
            low_freq: low_freq.clamp(20.0, 20000.0),
            high_freq: high_freq.clamp(20.0, 20000.0),
            priority: PRIORITY_EARLY, // EQ typically early in chain
            low_state: [0.0; 2],
            mid_state: [0.0; 2],
            high_state: [0.0; 2],
            low_gain_automation: None,
            mid_gain_automation: None,
            high_gain_automation: None,
        }
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the low gain parameter
    pub fn with_low_gain_automation(mut self, automation: Automation) -> Self {
        self.low_gain_automation = Some(automation);
        self
    }

    /// Add automation for the mid gain parameter
    pub fn with_mid_gain_automation(mut self, automation: Automation) -> Self {
        self.mid_gain_automation = Some(automation);
        self
    }

    /// Add automation for the high gain parameter
    pub fn with_high_gain_automation(mut self, automation: Automation) -> Self {
        self.high_gain_automation = Some(automation);
        self
    }

    /// Process a single sample at given sample rate
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `sample_rate` - Sample rate in Hz
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn process(&mut self, input: f32, sample_rate: f32, time: f32, sample_count: u64) -> f32 {
        // Quantized automation lookups (every 64 samples = 1.45ms @ 44.1kHz)
        // Use bitwise AND instead of modulo for power-of-2
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.low_gain_automation {
                self.low_gain = auto.value_at(time).clamp(0.0, 4.0);
            }
            if let Some(auto) = &self.mid_gain_automation {
                self.mid_gain = auto.value_at(time).clamp(0.0, 4.0);
            }
            if let Some(auto) = &self.high_gain_automation {
                self.high_gain = auto.value_at(time).clamp(0.0, 4.0);
            }
        }

        // Early exit if all gains are unity (no EQ needed)
        if (self.low_gain - 1.0).abs() < 0.01
            && (self.mid_gain - 1.0).abs() < 0.01
            && (self.high_gain - 1.0).abs() < 0.01
        {
            return input;
        }

        // Simple biquad filter approximations
        let low_coeff = (2.0 * std::f32::consts::PI * self.low_freq / sample_rate).min(0.9);
        let high_coeff = (2.0 * std::f32::consts::PI * self.high_freq / sample_rate).min(0.9);

        // Low shelf (one-pole lowpass) using FMA
        let diff_low = input - self.low_state[0];
        self.low_state[0] = self.low_state[0].mul_add(1.0, low_coeff * diff_low);
        let low = self.low_state[0] * self.low_gain;

        // High shelf (one-pole highpass) using FMA
        let diff_high = input - self.high_state[0];
        self.high_state[0] = self.high_state[0].mul_add(1.0, high_coeff * diff_high);
        let high = diff_high * self.high_gain;

        // Mid (bandpass - what's left)
        let mid = (input - self.low_state[0] - diff_high) * self.mid_gain;

        low + mid + high
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

    /// Reset the EQ state
    pub fn reset(&mut self) {
        self.low_state = [0.0; 2];
        self.mid_state = [0.0; 2];
        self.high_state = [0.0; 2];
    }

    // ========== PRESETS ==========

    /// Flat EQ - no adjustment (unity gain)
    pub fn flat() -> Self {
        Self::new(1.0, 1.0, 1.0, 300.0, 3000.0)
    }

    /// Bass boost - enhanced low end for warmth
    pub fn bass_boost() -> Self {
        Self::new(1.5, 1.0, 1.0, 100.0, 3000.0)
    }

    /// Treble boost - enhanced highs for brightness
    pub fn treble_boost() -> Self {
        Self::new(1.0, 1.0, 1.5, 300.0, 5000.0)
    }

    /// Smiley face - boosted lows and highs, scooped mids
    pub fn smiley() -> Self {
        Self::new(1.4, 0.7, 1.4, 200.0, 4000.0)
    }

    /// Presence - boost mids for vocal clarity
    pub fn presence() -> Self {
        Self::new(1.0, 1.3, 1.1, 500.0, 3000.0)
    }

    /// Warmth - gentle low boost, slight high cut
    pub fn warmth() -> Self {
        Self::new(1.3, 1.0, 0.9, 150.0, 3000.0)
    }

    /// Bright - reduce lows, boost highs
    pub fn bright() -> Self {
        Self::new(0.8, 1.0, 1.4, 300.0, 5000.0)
    }
}

/// A single parametric EQ band using a biquad peaking filter
///
/// Allows boosting or cutting a specific frequency range with adjustable bandwidth.
#[derive(Debug, Clone)]
pub struct EQBand {
    /// Center frequency in Hz
    pub frequency: f32,
    /// Gain in dB (positive = boost, negative = cut)
    pub gain_db: f32,
    /// Q factor (bandwidth) - higher = narrower, typical range 0.5 - 10
    pub q: f32,
    /// Whether this band is enabled
    pub enabled: bool,

    // Biquad filter coefficients
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,

    // State variables (for filtering)
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl EQBand {
    /// Create a new EQ band
    ///
    /// # Arguments
    /// * `frequency` - Center frequency in Hz
    /// * `gain_db` - Gain in dB (+ = boost, - = cut)
    /// * `q` - Bandwidth (0.5 = wide, 2.0 = medium, 10.0 = narrow)
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::effects::EQBand;
    ///
    /// // Boost 3kHz by 6dB with medium bandwidth
    /// let band = EQBand::new(3000.0, 6.0, 2.0);
    /// ```
    pub fn new(frequency: f32, gain_db: f32, q: f32) -> Self {
        let mut band = Self {
            frequency,
            gain_db,
            q,
            enabled: true,
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        };
        band.update_coefficients(44100.0);
        band
    }

    /// Update biquad coefficients for peaking EQ
    fn update_coefficients(&mut self, sample_rate: f32) {
        use std::f32::consts::PI;

        let w0 = 2.0 * PI * self.frequency / sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * self.q);
        let a = 10.0_f32.powf(self.gain_db / 40.0); // Amplitude multiplier

        // Peaking EQ coefficients
        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_w0;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha / a;

        // Normalize by a0
        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }

    /// Process a sample through this EQ band
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        if !self.enabled {
            return input;
        }

        // Biquad filter
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
                   - self.a1 * self.y1 - self.a2 * self.y2;

        // Update state
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    /// Reset filter state
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

/// Parametric Equalizer - multi-band frequency shaping
///
/// A parametric EQ allows surgical control over the frequency spectrum by
/// boosting or cutting specific frequency bands. Essential for mixing and
/// sound design.
///
/// # Common Uses
///
/// - **Mixing**: Balance frequency content between instruments
/// - **Vocal clarity**: Boost presence (2-5kHz), cut muddiness (200-400Hz)
/// - **Bass control**: Tighten low end, remove rumble
/// - **Fixing problems**: Remove resonances, feedback frequencies
///
/// # Example
///
/// ```
/// use tunes::synthesis::effects::ParametricEQ;
///
/// // Professional vocal EQ
/// let mut eq = ParametricEQ::new()
///     .band(100.0, -6.0, 1.0)    // Cut low rumble
///     .band(250.0, -3.0, 1.5)    // Reduce muddiness
///     .band(3000.0, 4.0, 2.0)    // Boost presence
///     .band(8000.0, -2.0, 1.5);  // Tame harshness
/// ```
#[derive(Debug, Clone)]
pub struct ParametricEQ {
    /// EQ bands
    pub bands: Vec<EQBand>,
    /// Processing priority
    pub priority: u8,
}

impl ParametricEQ {
    /// Create a new parametric EQ with no bands
    pub fn new() -> Self {
        Self {
            bands: Vec::new(),
            priority: 50,
        }
    }

    /// Add an EQ band (builder pattern)
    ///
    /// # Arguments
    /// * `frequency` - Center frequency in Hz
    /// * `gain_db` - Gain in dB (+ = boost, - = cut)
    /// * `q` - Bandwidth (0.5 = wide, 2.0 = medium, 10.0 = narrow)
    ///
    /// # Example
    /// ```
    /// use tunes::synthesis::effects::ParametricEQ;
    ///
    /// let eq = ParametricEQ::new()
    ///     .band(100.0, -4.0, 1.0)   // Cut 100Hz
    ///     .band(3000.0, 3.0, 2.0);  // Boost 3kHz
    /// ```
    pub fn band(mut self, frequency: f32, gain_db: f32, q: f32) -> Self {
        self.bands.push(EQBand::new(frequency, gain_db, q));
        self
    }

    /// Add a shelf EQ preset (builder pattern)
    ///
    /// Pre-configured EQ curves for common scenarios
    pub fn preset(mut self, preset: EQPreset) -> Self {
        match preset {
            EQPreset::VocalClarity => {
                self.bands.push(EQBand::new(100.0, -6.0, 1.0));   // Cut rumble
                self.bands.push(EQBand::new(250.0, -3.0, 1.5));   // Reduce mud
                self.bands.push(EQBand::new(3000.0, 4.0, 2.0));   // Presence boost
                self.bands.push(EQBand::new(8000.0, -2.0, 1.5));  // Tame sibilance
            }
            EQPreset::BassBoost => {
                self.bands.push(EQBand::new(60.0, 4.0, 1.0));     // Sub boost
                self.bands.push(EQBand::new(120.0, 3.0, 1.5));    // Bass boost
                self.bands.push(EQBand::new(300.0, -2.0, 1.0));   // Clean up mud
            }
            EQPreset::BrightAiry => {
                self.bands.push(EQBand::new(5000.0, 3.0, 1.5));   // Presence
                self.bands.push(EQBand::new(10000.0, 4.0, 1.0));  // Air
                self.bands.push(EQBand::new(15000.0, 2.0, 0.7));  // Sparkle
            }
            EQPreset::Telephone => {
                self.bands.push(EQBand::new(200.0, -12.0, 0.5));  // Cut lows
                self.bands.push(EQBand::new(1000.0, 6.0, 1.0));   // Boost mids
                self.bands.push(EQBand::new(4000.0, -12.0, 0.5)); // Cut highs
            }
            EQPreset::Warmth => {
                self.bands.push(EQBand::new(200.0, 3.0, 1.0));    // Low mids
                self.bands.push(EQBand::new(500.0, 2.0, 1.5));    // Warmth
                self.bands.push(EQBand::new(8000.0, -2.0, 1.0));  // Reduce harshness
            }
        }
        self
    }

    /// Enable or disable a specific band
    pub fn enable_band(&mut self, index: usize, enabled: bool) {
        if let Some(band) = self.bands.get_mut(index) {
            band.enabled = enabled;
        }
    }

    /// Update a band's parameters
    pub fn update_band(&mut self, index: usize, frequency: f32, gain_db: f32, q: f32, sample_rate: f32) {
        if let Some(band) = self.bands.get_mut(index) {
            band.frequency = frequency;
            band.gain_db = gain_db;
            band.q = q;
            band.update_coefficients(sample_rate);
        }
    }

    /// Reset all EQ bands
    pub fn reset(&mut self) {
        for band in &mut self.bands {
            band.reset();
        }
    }

    /// Process a sample through all EQ bands
    ///
    /// # Arguments
    /// * `input` - Input sample
    /// * `_time` - Current time (unused, for compatibility with other effects)
    /// * `_sample_index` - Sample index (unused, for compatibility with other effects)
    ///
    /// # Returns
    /// Processed sample with EQ applied
    pub fn process(&mut self, input: f32, _time: f32, _sample_index: usize) -> f32 {
        let mut output = input;

        // Process through each band in series
        for band in &mut self.bands {
            output = band.process(output);
        }

        output
    }

    /// Process a block of samples through all EQ bands
    ///
    /// # Arguments
    /// * `buffer` - Buffer of samples to process in-place
    /// * `time` - Starting time in seconds (for compatibility, currently unused)
    /// * `sample_index` - Starting sample index
    /// * `sample_rate` - Sample rate in Hz (for time advancement)
    #[inline]
    pub fn process_block(&mut self, buffer: &mut [f32], time: f32, sample_index: usize, sample_rate: f32) {
        let time_delta = 1.0 / sample_rate;
        for (i, sample) in buffer.iter_mut().enumerate() {
            let current_time = time + (i as f32 * time_delta);
            let current_sample_index = sample_index + i;
            *sample = self.process(*sample, current_time, current_sample_index);
        }
    }
}

impl Default for ParametricEQ {
    fn default() -> Self {
        Self::new()
    }
}

/// EQ presets for common scenarios
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EQPreset {
    /// Vocal clarity - cut rumble/mud, boost presence
    VocalClarity,
    /// Bass boost - enhance low end
    BassBoost,
    /// Bright and airy - high frequency enhancement
    BrightAiry,
    /// Telephone/lo-fi effect
    Telephone,
    /// Warmth - enhance low-mid richness
    Warmth,
}
