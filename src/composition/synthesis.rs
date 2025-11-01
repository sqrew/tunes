use crate::composition::TrackBuilder;
use crate::prelude::{FMParams, FilterEnvelope};

/// Synthesis methods for TrackBuilder
///
/// These methods provide convenient access to advanced synthesis features:
/// - Filter envelopes for subtractive synthesis
/// - FM synthesis for complex harmonic timbres
impl<'a> TrackBuilder<'a> {
    /// Set the filter envelope for subsequent notes
    ///
    /// The filter envelope controls how the filter cutoff frequency changes over time,
    /// creating classic subtractive synthesis sweeps.
    ///
    /// # Arguments
    /// * `filter_env` - FilterEnvelope to use
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("synth")
    ///     .filter_envelope(FilterEnvelope::classic())
    ///     .note(&[440.0], 1.0);
    /// ```
    pub fn filter_envelope(mut self, filter_env: FilterEnvelope) -> Self {
        // Store the filter envelope for subsequent notes
        // We'll need to track this in the builder state
        self.filter_envelope = filter_env;
        self
    }

    /// Set FM synthesis parameters for subsequent notes
    ///
    /// FM (Frequency Modulation) synthesis creates complex, harmonically rich timbres
    /// by modulating the frequency of one oscillator with another.
    ///
    /// # Arguments
    /// * `fm_params` - FM synthesis parameters
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("fm_piano")
    ///     .fm(FMParams::electric_piano())
    ///     .note(&[C4], 0.5)
    ///     .note(&[E4], 0.5);
    /// ```
    pub fn fm(mut self, fm_params: FMParams) -> Self {
        self.fm_params = fm_params;
        self
    }

    /// Create a custom FM sound with specific parameters
    ///
    /// # Arguments
    /// * `mod_ratio` - Modulator to carrier frequency ratio
    /// * `mod_index` - Modulation index (brightness, 0.0 to 10.0+)
    pub fn fm_custom(self, mod_ratio: f32, mod_index: f32) -> Self {
        self.fm(FMParams::new(mod_ratio, mod_index))
    }

    /// Combine filter envelope and FM for rich, evolving timbres
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("fm_synth")
    ///     .fm(FMParams::bell())
    ///     .filter_envelope(FilterEnvelope::classic())
    ///     .note(&[C4], 1.0);
    /// ```
    pub fn fm_with_filter(self, fm_params: FMParams, filter_env: FilterEnvelope) -> Self {
        self.fm(fm_params).filter_envelope(filter_env)
    }
}
