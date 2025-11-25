//! # tunes
//!
//! A comprehensive music composition, synthesis, and audio generation library.
//!
//! ## Quick Start
//!
//! ```no_run
//! use tunes::prelude::*;
//!
//! fn main() -> anyhow::Result<()> {
//!     let engine = AudioEngine::new()?;
//!     let mut comp = Composition::new(Tempo::new(120.0));
//!
//!     comp.track("piano")
//!         .note(&[C4], 0.5)
//!         .note(&[E4], 0.5)
//!         .note(&[G4], 0.5)
//!         .note(&[C5], 0.5);
//!
//!     engine.play_mixer(&comp.into_mixer())?;
//!     Ok(())
//! }
//! ```

pub mod audio;
pub mod cache;
pub mod composition;
pub mod consts;
pub mod engine;
pub mod error;
#[cfg(feature = "gpu")]
pub mod gpu;
pub mod instruments;
pub mod live_coding;
pub mod midi;
pub mod sequences;
pub mod synthesis;
pub mod templates;
pub mod theory;
pub mod track;

// Re-export inventory for macro use
#[doc(hidden)]
pub use inventory;

/// Registered sample for startup validation
#[derive(Debug)]
pub struct RegisteredSample {
    pub path: &'static str,
}

inventory::collect!(RegisteredSample);

/// Validate all registered samples exist at startup
///
/// This function checks that all samples registered via `play_sample!()` macro
/// exist on disk. It collects all missing samples and reports them together,
/// making it easy to catch typos and missing files during development.
///
/// # Examples
///
/// ```no_run
/// use tunes::prelude::*;
///
/// fn main() -> anyhow::Result<()> {
///     // Validate all samples at startup
///     validate_all_samples()?;
///
///     let engine = AudioEngine::new()?;
///     play_sample!(engine, "assets/explosion.wav");
///     Ok(())
/// }
/// ```
pub fn validate_all_samples() -> error::Result<()> {
    let mut missing = Vec::new();

    for sample in inventory::iter::<RegisteredSample> {
        if !std::path::Path::new(sample.path).exists() {
            missing.push(sample.path);
        }
    }

    if !missing.is_empty() {
        eprintln!("ERROR: Missing {} sample(s) at startup:", missing.len());
        for path in &missing {
            eprintln!("  - {}", path);
        }
        return Err(error::TunesError::SampleNotFound(format!(
            "{} sample file(s) not found",
            missing.len()
        )));
    }

    let total = inventory::iter::<RegisteredSample>().count();
    if total > 0 {
        eprintln!("✓ All {} sample(s) validated successfully", total);
    }
    Ok(())
}

/// Play a sample with consistent path resolution
///
/// This macro provides an ergonomic way to play audio samples with automatic
/// path resolution relative to the project root. It also registers the sample
/// for startup validation via `validate_all_samples()`.
///
/// # Examples
///
/// ```no_run
/// use tunes::prelude::*;
///
/// let engine = AudioEngine::new()?;
/// play_sample!(engine, "assets/explosion.wav");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[macro_export]
macro_rules! play_sample {
    ($engine:expr, $path:literal) => {{
        const SAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

        // Auto-register for startup validation
        $crate::inventory::submit! {
            $crate::RegisteredSample { path: SAMPLE_PATH }
        }

        $engine.play_sample(SAMPLE_PATH)
    }};
}

/// Prelude module for convenient imports
pub mod prelude {
    // Macros and validation
    pub use crate::play_sample;
    pub use crate::validate_all_samples;

    // Core composition
    pub use crate::composition::{Composition, DrumGrid, DrumType, Tempo};
    pub use crate::engine::{AudioEngine, SamplePlaybackBuilder, SoundId};
    pub use crate::track::Mixer;

    // Error handling
    pub use crate::error::{Result, TunesError};

    // Notes, Scales, and Chords
    pub use crate::consts::*;

    // Theory
    pub use crate::theory::{
        ChordPattern, KeyMode, KeyRoot, KeySignature, ProgressionType, ScalePattern, chord,
        progression, scale, transpose, transpose_sequence,
    };

    // Instruments
    pub use crate::instruments::Instrument;

    // Effects and filters
    pub use crate::synthesis::effects::*;
    pub use crate::synthesis::{Filter, FilterType};

    // Advanced synthesis
    pub use crate::synthesis::{
        AdditiveSynth, Envelope, FMParams, FilterEnvelope, GranularParams, KarplusStrong,
        NoiseType, Partial, Sample, SampleSlice, Waveform, Wavetable,
    };

    // Noise generators
    pub use crate::synthesis::{
        BlueNoise, BrownNoise, GreenNoise, NoiseGenerator, PerlinNoise, PinkNoise, WhiteNoise,
    };

    // Effects (Parametric EQ)
    pub use crate::synthesis::{EQBand, EQPreset, ParametricEQ};

    // Spatial Audio
    pub use crate::synthesis::{
        AttenuationModel, ListenerConfig, SpatialParams, SpatialPosition, SpatialResult, Vec3,
    };

    // LFO
    pub use crate::synthesis::{LFO, ModRoute, ModTarget};

    // Sequences
    pub use crate::sequences::{
        golden_ratio, golden_ratio_rhythm, golden_sections, harmonic_series,
    };
    // Note: euclidean, fibonacci, and collatz are now modules - use:
    //   sequences::euclidean::generate(pulses, steps) or sequences::euclidean::kick_four_floor()
    //   sequences::fibonacci::generate(n) or sequences::fibonacci::classic()
    //   sequences::collatz::generate(start, max) or sequences::collatz::dramatic()

    // Automation
    pub use crate::synthesis::{Automation, Interpolation};

    // Microtonal
    pub use crate::theory::{
        EDO12, EDO19, EDO24, EDO31, EDO53, Edo, cents_to_ratio, freq_from_cents, half_flat,
        half_sharp, just_major_scale, just_minor_scale, just_ratio, just_scale, pythagorean_scale,
        quarter_flat, quarter_sharp, ratio_to_cents,
    };

    // MIDI utilities
    pub use crate::midi::{
        drum_type_to_midi_note, frequency_to_midi_note, midi_note_to_drum_type,
        midi_note_to_frequency,
    };

    // Live audio input
    pub use crate::audio::LiveInput;
}

// WebAssembly demo function
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_web_demo() -> std::result::Result<(), JsValue> {
    use crate::prelude::*;
    use std::sync::Mutex;

    // Set panic hook for better error messages in the browser console
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"Initializing Tunes audio engine...".into());

    // Create audio engine and keep it alive by leaking it
    // This is intentional - we want the audio engine to persist for the lifetime of the page
    let engine = Box::leak(Box::new(AudioEngine::new()
        .map_err(|e| JsValue::from_str(&format!("Failed to create audio engine: {}", e)))?));

    web_sys::console::log_1(&"Audio engine created successfully!".into());

    // Create a composition with a simple melody
    let mut comp = Composition::new(Tempo::new(120.0));

    web_sys::console::log_1(&"Creating melody...".into());

    // Play a simple melody using synthesis
    comp.instrument("piano", &Instrument::electric_piano())
        .notes(&[C4, E4, G4, C5], 0.5)
        .notes(&[C5, G4, E4, C4], 0.5);

    web_sys::console::log_1(&"Created composition with piano melody".into());

    // Convert to mixer and play (non-blocking in web environment)
    let mixer = comp.into_mixer();
    let id = engine.play_mixer_realtime(&mixer)
        .map_err(|e| JsValue::from_str(&format!("Failed to play mixer: {}", e)))?;

    web_sys::console::log_1(&format!("Playing mixer with ID: {}", id).into());
    web_sys::console::log_1(&"Tunes web demo complete! You should hear a C major arpeggio.".into());

    Ok(())
}
