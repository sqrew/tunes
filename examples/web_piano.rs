//! Interactive Web Piano Demo for Tunes
//!
//! A polished virtual piano that runs in the browser, showcasing
//! the Tunes library's synthesis capabilities via WebAssembly.
//!
//! # Building for Web
//!
//! 1. Install wasm-pack:
//!    ```bash
//!    cargo install wasm-pack
//!    ```
//!
//! 2. Build the example:
//!    ```bash
//!    wasm-pack build --target web --features web
//!    ```
//!
//! 3. Serve the HTML file with a local web server:
//!    ```bash
//!    python3 -m http.server 8000
//!    ```
//!    Then open http://localhost:8000/examples/web_piano.html
//!
//! # Features
//!
//! - 2-octave visual keyboard with mouse/touch support
//! - Computer keyboard input (A-L for white keys, W-E-T-Y-U for black keys)
//! - 8 keyboard instruments (acoustic piano, electric piano, organs, etc.)
//! - Octave shifting (C2-C6 range)

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use tunes::prelude::*;

/// Semitone offsets from C for each note in an octave
#[cfg(target_arch = "wasm32")]
const SEMITONES: [f32; 12] = [
    0.0,  // C
    1.0,  // C#
    2.0,  // D
    3.0,  // D#
    4.0,  // E
    5.0,  // F
    6.0,  // F#
    7.0,  // G
    8.0,  // G#
    9.0,  // A
    10.0, // A#
    11.0, // B
];

/// Calculate frequency from octave and semitone
/// Using A4 = 440 Hz as reference
#[cfg(target_arch = "wasm32")]
fn semitone_to_frequency(octave: i32, semitone: u8) -> f32 {
    // C4 is 261.63 Hz (MIDI note 60)
    // Formula: freq = 261.63 * 2^((octave - 4) + semitone/12)
    let octave_offset = (octave - 4) as f32;
    let semitone_offset = SEMITONES[semitone as usize % 12] / 12.0;
    261.63 * 2.0_f32.powf(octave_offset + semitone_offset)
}

/// Interactive web piano synthesizer
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebPiano {
    engine: &'static AudioEngine,
    current_octave: i32,
    current_instrument: u8,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebPiano {
    /// Create a new WebPiano instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WebPiano, JsValue> {
        // Better error messages in browser console
        console_error_panic_hook::set_once();

        web_sys::console::log_1(&"Initializing WebPiano...".into());

        // Create and leak the engine to keep it alive for the page lifetime
        let engine = Box::leak(Box::new(
            AudioEngine::new()
                .map_err(|e| JsValue::from_str(&format!("Failed to create audio engine: {}", e)))?,
        ));

        web_sys::console::log_1(&"WebPiano ready!".into());

        Ok(WebPiano {
            engine,
            current_octave: 4, // Start at middle C
            current_instrument: 0,
        })
    }

    /// Play a note by semitone offset (0-11: C, C#, D, D#, E, F, F#, G, G#, A, A#, B)
    pub fn play_note(&self, semitone: u8) -> Result<(), JsValue> {
        let frequency = semitone_to_frequency(self.current_octave, semitone);

        let instrument = self.get_current_instrument();

        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("piano", &instrument)
            .note(&[frequency], 0.4); // 0.4 second note for responsive feel

        self.engine
            .play_mixer_realtime(&comp.into_mixer())
            .map_err(|e| JsValue::from_str(&format!("Failed to play note: {}", e)))?;

        Ok(())
    }

    /// Play a note in a specific octave (for extended keyboard range)
    pub fn play_note_in_octave(&self, semitone: u8, octave: i32) -> Result<(), JsValue> {
        let frequency = semitone_to_frequency(octave, semitone);

        let instrument = self.get_current_instrument();

        let mut comp = Composition::new(Tempo::new(120.0));
        comp.instrument("piano", &instrument)
            .note(&[frequency], 0.4);

        self.engine
            .play_mixer_realtime(&comp.into_mixer())
            .map_err(|e| JsValue::from_str(&format!("Failed to play note: {}", e)))?;

        Ok(())
    }

    /// Set the current octave (valid range: 2-6)
    pub fn set_octave(&mut self, octave: i32) {
        self.current_octave = octave.clamp(2, 6);
        web_sys::console::log_1(&format!("Octave set to: {}", self.current_octave).into());
    }

    /// Get the current octave
    pub fn get_octave(&self) -> i32 {
        self.current_octave
    }

    /// Shift octave up by 1
    pub fn octave_up(&mut self) {
        if self.current_octave < 6 {
            self.current_octave += 1;
            web_sys::console::log_1(&format!("Octave: {}", self.current_octave).into());
        }
    }

    /// Shift octave down by 1
    pub fn octave_down(&mut self) {
        if self.current_octave > 2 {
            self.current_octave -= 1;
            web_sys::console::log_1(&format!("Octave: {}", self.current_octave).into());
        }
    }

    /// Set the current instrument by index (0-7)
    pub fn set_instrument(&mut self, index: u8) {
        self.current_instrument = index.min(7);
        let name = self.get_instrument_name(self.current_instrument);
        web_sys::console::log_1(&format!("Instrument: {}", name).into());
    }

    /// Get the current instrument index
    pub fn get_instrument_index(&self) -> u8 {
        self.current_instrument
    }

    /// Get the name of an instrument by index
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

    /// Get the number of available instruments
    pub fn get_instrument_count(&self) -> u8 {
        8
    }

    /// Internal helper to get the current instrument
    fn get_current_instrument(&self) -> Instrument {
        match self.current_instrument {
            0 => Instrument::acoustic_piano(),
            1 => Instrument::electric_piano(),
            2 => Instrument::stage_73(),
            3 => Instrument::wurlitzer(),
            4 => Instrument::hammond_organ(),
            5 => Instrument::church_organ(),
            6 => Instrument::clavinet(),
            7 => Instrument::harpsichord(),
            _ => Instrument::acoustic_piano(),
        }
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("This example is for WebAssembly only.");
        println!("Build it with: wasm-pack build --target web --features web");
        println!("Then open examples/web_piano.html in a browser.");
    }
}
