use crate::synthesis::automation::Automation;
use crate::track::PRIORITY_MODULATION;

/// Standard audio sample rate
const DEFAULT_SAMPLE_RATE: f32 = 44100.0;

/// AutoPan - automatic stereo panning modulation
///
/// Creates rhythmic stereo movement by automatically panning the signal
/// left and right. This is applied at the stereo stage after mono effects.
#[derive(Debug, Clone)]
pub struct AutoPan {
    pub rate: f32,    // LFO rate in Hz (typically 0.1-10 Hz)
    pub depth: f32,   // Panning depth 0.0 to 1.0 (0.5 = full L-R sweep)
    pub priority: u8, // Processing priority (processed at stereo stage)
    phase: f32,       // LFO phase (0.0 to 1.0)

    // Automation (optional)
    rate_automation: Option<Automation>,
    depth_automation: Option<Automation>,
}

impl AutoPan {
    /// Create a new auto-pan effect
    ///
    /// # Arguments
    /// * `rate` - LFO frequency in Hz (typically 0.1-10 Hz)
    /// * `depth` - Pan modulation depth 0.0 to 1.0 (0.5 pans full left-right)
    /// * `sample_rate` - Audio sample rate in Hz (kept for API compatibility but ignored)
    ///
    /// **Note:** The actual sample rate is provided at runtime during processing.
    pub fn with_sample_rate(rate: f32, depth: f32, _sample_rate: f32) -> Self {
        Self {
            rate: rate.max(0.01),
            depth: depth.clamp(0.0, 1.0),
            priority: PRIORITY_MODULATION,
            phase: 0.0,
            rate_automation: None,
            depth_automation: None,
        }
    }

    /// Create an auto-pan with default sample rate (44100 Hz)
    pub fn new(rate: f32, depth: f32) -> Self {
        Self::with_sample_rate(rate, depth, DEFAULT_SAMPLE_RATE)
    }

    /// Set the processing priority (lower = earlier in signal chain)
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Add automation for the rate parameter
    pub fn with_rate_automation(mut self, automation: Automation) -> Self {
        self.rate_automation = Some(automation);
        self
    }

    /// Add automation for the depth parameter
    pub fn with_depth_automation(mut self, automation: Automation) -> Self {
        self.depth_automation = Some(automation);
        self
    }

    /// Get the current pan position (-1.0 to 1.0)
    ///
    /// This should be called once per sample to advance the LFO phase.
    /// The returned value is added to the track's base pan.
    ///
    /// # Arguments
    /// * `sample_rate` - Audio sample rate in Hz (provided by the engine at runtime)
    /// * `time` - Current time in seconds (for automation)
    /// * `sample_count` - Global sample counter (for quantized automation lookups)
    #[inline]
    pub fn get_pan_offset(&mut self, sample_rate: f32, time: f32, sample_count: u64) -> f32 {
        // Quantized automation lookups (every 64 samples)
        if sample_count & 63 == 0 {
            if let Some(auto) = &self.rate_automation {
                self.rate = auto.value_at(time).max(0.01);
            }
            if let Some(auto) = &self.depth_automation {
                self.depth = auto.value_at(time).clamp(0.0, 1.0);
            }
        }

        // Early out if no modulation
        if self.depth < 0.0001 {
            return 0.0;
        }

        // Generate LFO (sine wave)
        let lfo = (self.phase * 2.0 * std::f32::consts::PI).sin();

        // Advance phase
        self.phase += self.rate / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        // Return pan offset (-depth to +depth)
        lfo * self.depth
    }

    /// Reset the auto-pan state
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    // ========== PRESETS ==========

    /// Slow pan - gentle left-right sweep (0.25 Hz)
    pub fn slow() -> Self {
        Self::new(0.25, 0.75)
    }

    /// Classic autopan - steady rhythmic panning (0.5 Hz)
    pub fn classic() -> Self {
        Self::new(0.5, 0.75)
    }

    /// Fast pan - quick stereo movement (2.0 Hz)
    pub fn fast() -> Self {
        Self::new(2.0, 0.75)
    }

    /// Subtle pan - light stereo enhancement (0.3 Hz, mild depth)
    pub fn subtle() -> Self {
        Self::new(0.3, 0.4)
    }

    /// Extreme pan - hard left-right panning (1.0 Hz, full depth)
    pub fn extreme() -> Self {
        Self::new(1.0, 1.0)
    }
}
