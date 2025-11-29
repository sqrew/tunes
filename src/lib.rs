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

/// Repeat a pattern string N times for use in drum grids.
///
/// This is a convenience macro to avoid typing out long repetitive patterns.
///
/// # Example
/// ```
/// use tunes::pat;
///
/// // Instead of: "x-x-x-x-x-x-x-x-x-x-x-x-x-x-x-x-"
/// let pattern = pat!("x-", 16);
/// assert_eq!(pattern, "x-x-x-x-x-x-x-x-x-x-x-x-x-x-x-x-");
///
/// // Works great in drum_grid:
/// // .sound(DrumType::HiHatClosed, pat!("x-", 16))
/// ```
#[macro_export]
macro_rules! pat {
    ($pattern:expr, $times:expr) => {
        $pattern.repeat($times)
    };
}

/// Prelude module for convenient imports
pub mod prelude {
    // Macros and validation
    pub use crate::pat;
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

/// Interactive web piano synthesizer
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebPiano {
    engine: std::cell::RefCell<Option<&'static engine::AudioEngine>>,
    current_octave: i32,
    current_instrument: u8,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebPiano {
    /// Create a new WebPiano instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> std::result::Result<WebPiano, JsValue> {
        console_error_panic_hook::set_once();
        web_sys::console::log_1(&"WebPiano ready! Click a key to start.".into());

        Ok(WebPiano {
            engine: std::cell::RefCell::new(None),
            current_octave: 4,
            current_instrument: 0,
        })
    }

    /// Initialize the audio engine (called on first user interaction)
    fn get_or_init_engine(&self) -> std::result::Result<&'static engine::AudioEngine, JsValue> {
        let mut engine_ref = self.engine.borrow_mut();
        if engine_ref.is_none() {
            web_sys::console::log_1(&"Initializing audio engine...".into());
            let engine = Box::leak(Box::new(
                engine::AudioEngine::new()
                    .map_err(|e| JsValue::from_str(&format!("Failed to create audio engine: {}", e)))?,
            ));
            web_sys::console::log_1(&"Audio engine ready!".into());
            *engine_ref = Some(engine);
        }
        Ok(engine_ref.unwrap())
    }

    /// Play a note by semitone (0-11) in current octave
    pub fn play_note(&self, semitone: u8) -> std::result::Result<(), JsValue> {
        self.play_note_in_octave(semitone, self.current_octave)
    }

    /// Play a note by semitone (0-11) in a specific octave
    pub fn play_note_in_octave(&self, semitone: u8, octave: i32) -> std::result::Result<(), JsValue> {
        use crate::consts::*;

        let engine = self.get_or_init_engine()?;

        // Map semitone + octave to note constant
        let notes_oct2: [f32; 12] = [C2, CS2, D2, DS2, E2, F2, FS2, G2, GS2, A2, AS2, B2];
        let notes_oct3: [f32; 12] = [C3, CS3, D3, DS3, E3, F3, FS3, G3, GS3, A3, AS3, B3];
        let notes_oct4: [f32; 12] = [C4, CS4, D4, DS4, E4, F4, FS4, G4, GS4, A4, AS4, B4];
        let notes_oct5: [f32; 12] = [C5, CS5, D5, DS5, E5, F5, FS5, G5, GS5, A5, AS5, B5];
        let notes_oct6: [f32; 12] = [C6, CS6, D6, DS6, E6, F6, FS6, G6, GS6, A6, AS6, B6];

        let frequency = match octave {
            2 => notes_oct2[semitone as usize % 12],
            3 => notes_oct3[semitone as usize % 12],
            4 => notes_oct4[semitone as usize % 12],
            5 => notes_oct5[semitone as usize % 12],
            6 => notes_oct6[semitone as usize % 12],
            _ => notes_oct4[semitone as usize % 12],
        };

        let instrument = self.get_instrument();

        let mut comp = composition::Composition::new(composition::Tempo::new(120.0));
        comp.instrument("piano", &instrument)
            .notes(&[frequency], 0.5);

        let mixer = comp.into_mixer();

        engine
            .play_mixer_realtime(&mixer)
            .map_err(|e| JsValue::from_str(&format!("Failed to play note: {}", e)))?;

        Ok(())
    }

    /// Set the current octave (2-6)
    pub fn set_octave(&mut self, octave: i32) {
        self.current_octave = octave.clamp(2, 6);
    }

    /// Get the current octave
    pub fn get_octave(&self) -> i32 {
        self.current_octave
    }

    /// Set the current instrument (0-7)
    pub fn set_instrument(&mut self, index: u8) {
        self.current_instrument = index.min(7);
    }

    /// Get instrument name by index
    pub fn get_instrument_name(&self, index: u8) -> String {
        match index {
            0 => "Acoustic Piano".to_string(),
            1 => "Electric Piano".to_string(),
            2 => "Stage 73 Rhodes".to_string(),
            3 => "Wurlitzer".to_string(),
            4 => "Hammond Organ".to_string(),
            5 => "Church Organ".to_string(),
            6 => "Clavinet".to_string(),
            7 => "Harpsichord".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    fn get_instrument(&self) -> instruments::Instrument {
        match self.current_instrument {
            0 => instruments::Instrument::acoustic_piano(),
            1 => instruments::Instrument::electric_piano(),
            2 => instruments::Instrument::stage_73(),
            3 => instruments::Instrument::wurlitzer(),
            4 => instruments::Instrument::hammond_organ(),
            5 => instruments::Instrument::church_organ(),
            6 => instruments::Instrument::clavinet(),
            7 => instruments::Instrument::harpsichord(),
            _ => instruments::Instrument::acoustic_piano(),
        }
    }
}
