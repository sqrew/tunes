//! Basic Web Demo for Tunes
//!
//! This example demonstrates the Tunes audio engine running in WebAssembly.
//! It generates a simple sine wave tone and plays it using the browser's Web Audio API.
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
//!    Then open http://localhost:8000/examples/web_demo.html
//!

use tunes::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_web_demo() -> Result<(), JsValue> {
    // Set panic hook for better error messages in the browser console
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"Initializing Tunes audio engine...".into());

    // Create audio engine
    let engine = AudioEngine::new()
        .map_err(|e| JsValue::from_str(&format!("Failed to create audio engine: {}", e)))?;

    web_sys::console::log_1(&"Audio engine created successfully!".into());

    // Create a simple sine wave tone
    let sample_rate = 48000.0;
    let duration = 2.0; // 2 seconds
    let frequency = 440.0; // A4 note

    let mut samples = Vec::new();
    for i in 0..(sample_rate * duration) as usize {
        let t = i as f32 / sample_rate;
        let sample = (2.0 * std::f32::consts::PI * frequency * t).sin() * 0.3;
        samples.push(sample);
    }

    // Create a sample from the generated audio
    let sample = Sample::new(samples, sample_rate as u32, 1)
        .map_err(|e| JsValue::from_str(&format!("Failed to create sample: {}", e)))?;

    web_sys::console::log_1(&"Generated sine wave sample".into());

    // Create a mixer and add the sample
    let mut mixer = Mixer::new();
    let mut track = Track::new();

    // Add the sample to the track at position 0
    track.add_note(0.0, sample.clone());

    // Add track to mixer
    mixer.add_track(track);

    web_sys::console::log_1(&"Created mixer with sine wave sample".into());

    // Play the mixer (non-blocking in web environment)
    let id = engine.play_mixer_realtime(&mixer)
        .map_err(|e| JsValue::from_str(&format!("Failed to play mixer: {}", e)))?;

    web_sys::console::log_1(&format!("Playing mixer with ID: {}", id).into());
    web_sys::console::log_1(&"Tunes web demo complete! You should hear a 440Hz tone.".into());

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("This example is for WebAssembly only.");
    println!("Build it with: wasm-pack build --target web --features web");
}
