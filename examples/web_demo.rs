//! Basic Web Demo for Tunes
//!
//! This example demonstrates the Tunes audio engine running in WebAssembly.
//! It plays a simple piano melody using the browser's Web Audio API.
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
pub fn run_web_demo() -> std::result::Result<(), JsValue> {
    // Set panic hook for better error messages in the browser console
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"Initializing Tunes audio engine...".into());

    // Create audio engine
    let engine = AudioEngine::new()
        .map_err(|e| JsValue::from_str(&format!("Failed to create audio engine: {}", e)))?;

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

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("This example is for WebAssembly only.");
        println!("Build it with: wasm-pack build --target web --features web");
    }

    #[cfg(target_arch = "wasm32")]
    {
        // On WASM, the function is called via run_web_demo() from JavaScript
        // No main function execution needed
    }
}
