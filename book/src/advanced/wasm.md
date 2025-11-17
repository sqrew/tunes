# WebAssembly Support

Tunes supports WebAssembly, allowing you to run your audio synthesis and processing code directly in web browsers. This opens up exciting possibilities for browser-based music applications, interactive demos, educational tools, and web-based games.

## Overview

WebAssembly (WASM) support in Tunes provides:

-  **Full synthesis capabilities** - All synthesis methods work in the browser
-  **Effects processing** - All 17+ effects available
-  **Sample playback** - Load and play audio from memory
-  **Sample transformations** - Normalize, fade, pitch shift, time stretch
-  **Real-time audio** - Low-latency playback via Web Audio API
-  **SIMD acceleration** - Fast DSP operations in the browser
-  **Mixer and Track system** - Full composition capabilities

### Platform Differences

Some features are native-only due to browser limitations:

- L **File streaming** - No `stream_file()` methods (no file system in browser)
- L **Multi-threading** - Parallel processing falls back to sequential
- L **Direct file access** - Use `Sample::from_bytes()` instead

These limitations don't significantly impact browser use cases, as browsers prefer in-memory audio and handle parallelism differently.

## Quick Start

### 1. Install Prerequisites

```bash
# Install wasm-pack
cargo install wasm-pack

# Add wasm32 target (if not already installed)
rustup target add wasm32-unknown-unknown
```

### 2. Add Tunes to Your Project

```toml
[dependencies]
tunes = { version = "0.22.0", features = ["web"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["console"] }
console_error_panic_hook = "0.1"  # Better error messages

# Optional: For loading audio files dynamically (see "Loading Audio Files" section)
# wasm-bindgen-futures = "0.4"
# js-sys = "0.3"
# web-sys = { version = "0.3", features = ["console", "Request", "RequestInit", "RequestMode", "Response", "Window"] }
```

### 3. Create Your Audio Module

```rust
// src/lib.rs
use tunes::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn play_music() -> Result<(), JsValue> {
    // Better error messages in browser console
    console_error_panic_hook::set_once();

    // Create audio engine
    let engine = AudioEngine::new()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Create a composition
    let mut comp = Composition::new(Tempo::new(120.0));

    comp.instrument("piano", &Instrument::electric_piano())
        .notes(&[C4, E4, G4, C5], 0.5);

    // Play it!
    engine.play_mixer(&comp.into_mixer())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}
```

### 4. Build for Web

```bash
wasm-pack build --target web --features web
```

This creates a `pkg/` directory with your compiled WASM module and JavaScript bindings.

### 5. Use in HTML

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Tunes WASM Demo</title>
</head>
<body>
    <button id="playButton">Play Music</button>

    <script type="module">
        import init, { play_music } from './pkg/your_crate_name.js';

        async function run() {
            // Initialize WASM module
            await init();

            document.getElementById('playButton').addEventListener('click', () => {
                play_music();
            });
        }

        run();
    </script>
</body>
</html>
```

### 6. Serve and Test

```bash
# Simple Python server
python3 -m http.server 8000

# Or use a dedicated server
cargo install basic-http-server
basic-http-server .
```

Open `http://localhost:8000` in your browser and click the button!

## Loading Audio Files in the Browser

Since browsers don't have file system access, you need to load audio data differently.

### Using `Sample::from_bytes()`

**Note:** The following example requires additional dependencies. Add these to your `Cargo.toml`:

```toml
wasm-bindgen-futures = "0.4"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["console", "Request", "RequestInit", "RequestMode", "Response", "Window"] }
```

```rust
use tunes::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

#[wasm_bindgen]
pub async fn load_and_play_sample() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Fetch audio file from server
    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init("/audio/sample.wav", &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    // Get bytes
    let array_buffer = JsFuture::from(resp.array_buffer()?).await?;
    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
    let bytes = uint8_array.to_vec();

    // Create sample from bytes
    let sample = Sample::from_bytes(&bytes)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Create engine and composition
    let engine = AudioEngine::new()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("sample").sample(sample, 0.0);

    engine.play_mixer_realtime(&comp.into_mixer())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}
```

### Using Embedded Audio Data

For small samples, you can embed them directly:

```rust
const KICK_SAMPLE: &[u8] = include_bytes!("../assets/kick.wav");

#[wasm_bindgen]
pub fn play_embedded_sample() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let sample = Sample::from_bytes(KICK_SAMPLE)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let engine = AudioEngine::new()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut comp = Composition::new(Tempo::new(120.0));
    comp.track("kick").sample(sample, 0.0);

    engine.play_mixer_realtime(&comp.into_mixer())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}
```

## Performance Considerations

### SIMD Acceleration

SIMD operations work in WebAssembly! Modern browsers support SIMD instructions, giving you fast DSP performance:

-  Sample playback uses SIMD
-  Effects processing uses SIMD
-  Wavetable synthesis uses SIMD

**Enable SIMD in your build:**

```bash
RUSTFLAGS='-C target-feature=+simd128' wasm-pack build --target web --features web
```

### No Multi-threading

Unlike native code, WASM runs single-threaded in browsers (with limited exceptions). Tunes automatically falls back to sequential processing:

```rust
// Native: Uses rayon for parallel processing
samples.par_iter_mut().for_each(|s| s.normalize());

// WASM: Automatically falls back to sequential
samples.iter_mut().for_each(|s| s.normalize());
```

This is transparent - your code works the same on both platforms.

### Memory Considerations

Browsers have memory limits:

- Keep total audio data under 100-200MB for compatibility
- Use `Sample::from_bytes()` instead of streaming
- Consider lazy-loading samples as needed
- Clear unused samples to free memory

## Common Patterns

### Interactive Synth

```rust
use tunes::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WebSynth {
    engine: AudioEngine,
}

#[wasm_bindgen]
impl WebSynth {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WebSynth, JsValue> {
        console_error_panic_hook::set_once();

        let engine = AudioEngine::new()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(WebSynth { engine })
    }

    pub fn play_note(&self, frequency: f32, duration: f32) -> Result<(), JsValue> {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.track("synth").sine(frequency, duration);

        self.engine.play_mixer_realtime(&comp.into_mixer())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(())
    }

    pub fn play_chord(&self, frequencies: Vec<f32>) -> Result<(), JsValue> {
        let mut comp = Composition::new(Tempo::new(120.0));

        for freq in frequencies {
            comp.track("synth").sine(freq, 1.0);
        }

        self.engine.play_mixer_realtime(&comp.into_mixer())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(())
    }
}
```

```javascript
import init, { WebSynth } from './pkg/my_synth.js';

await init();
const synth = new WebSynth();

// Play A4 (440 Hz)
synth.play_note(440.0, 1.0);

// Play C major chord
synth.play_chord([261.63, 329.63, 392.00]);
```

### Sample-Based Drum Machine

```rust
use tunes::prelude::*;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct DrumMachine {
    engine: AudioEngine,
    samples: HashMap<String, Sample>,
}

#[wasm_bindgen]
impl DrumMachine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<DrumMachine, JsValue> {
        console_error_panic_hook::set_once();

        let engine = AudioEngine::new()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(DrumMachine {
            engine,
            samples: HashMap::new(),
        })
    }

    pub fn load_sample(&mut self, name: String, bytes: Vec<u8>) -> Result<(), JsValue> {
        let sample = Sample::from_bytes(&bytes)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.samples.insert(name, sample);
        Ok(())
    }

    pub fn play_pattern(&self, pattern: Vec<String>) -> Result<(), JsValue> {
        let mut comp = Composition::new(Tempo::new(120.0));

        for (i, sample_name) in pattern.iter().enumerate() {
            if let Some(sample) = self.samples.get(sample_name) {
                let time = i as f32 * 0.25; // 16th notes
                comp.track(&sample_name).sample(sample.clone(), time);
            }
        }

        self.engine.play_mixer_realtime(&comp.into_mixer())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(())
    }
}
```

## Troubleshooting

### CORS Errors

If you see CORS errors when loading audio files:

```
Access to fetch at 'file:///path/to/audio.wav' from origin 'null' has been blocked by CORS policy
```

**Solution:** Always use a web server, never `file://` protocol:

```bash
python3 -m http.server 8000
# or
basic-http-server .
```

### Audio Not Playing

**Issue:** No sound in the browser.

**Solutions:**

1. **Check browser console** for errors
2. **User interaction required:** Most browsers require user interaction before playing audio:

```javascript
// Wait for user click
document.getElementById('playButton').addEventListener('click', async () => {
    await init();
    play_music();  // Now it will work
});
```

3. **Check Web Audio API support:** All modern browsers support it, but check:

```javascript
if (!window.AudioContext && !window.webkitAudioContext) {
    alert('Web Audio API not supported');
}
```

### Build Errors

**Issue:** `error: linking with 'rust-lld' failed`

**Solution:** Ensure wasm32 target is installed:

```bash
rustup target add wasm32-unknown-unknown
```

**Issue:** `cannot find type 'JsValue'`

**Solution:** Add wasm-bindgen dependency:

```toml
wasm-bindgen = "0.2"
```

### Memory Errors

**Issue:** `RuntimeError: memory access out of bounds`

**Solution:** Reduce audio data size or increase WASM memory limit in your JavaScript:

```javascript
import init from './pkg/my_crate.js';

await init({
    module_or_path: './pkg/my_crate_bg.wasm',
    memory: new WebAssembly.Memory({ initial: 256, maximum: 512 })
});
```

## Examples

The repository includes a complete working example:

- `examples/web_demo.rs` - Basic synthesis and playback
- `examples/web_demo.html` - HTML interface
- `WEB_DEMO.md` - Complete setup guide

To run it:

```bash
wasm-pack build --target web --features web
python3 -m http.server 8000
# Open http://localhost:8000/examples/web_demo.html
```

## API Compatibility

### Available on Web

All core APIs work on WebAssembly:

-  `AudioEngine::new()`
-  `Composition`, `Mixer`, `Track`
-  `Instrument::*` (all 150+ instruments)
-  `Sample::from_bytes()`
-  All synthesis methods
-  All effects
-  `play_mixer()`, `play_mixer_realtime()`
-  Sample transformations (normalize, fade, pitch_shift, etc.)

### Not Available on Web

These methods are conditionally compiled out on WASM:

- L `stream_file()` - No file system
- L `stream_file_looping()` - No file system
- L `stop_stream()` - Streaming not supported
- L `pause_stream()` - Streaming not supported
- L `resume_stream()` - Streaming not supported
- L `set_stream_volume()` - Streaming not supported
- L `set_stream_pan()` - Streaming not supported

Use `Sample::from_bytes()` for all audio loading on web.

## Next Steps

- **Interactive demos:** Create educational music theory tools
- **Browser-based DAW:** Build a simple sequencer or synthesizer
- **Game audio:** Use Tunes for web-based game sound
- **Live coding:** Port the live coding experience to the browser
- **Generative music:** Create algorithmic music players

WebAssembly support makes Tunes accessible to a whole new audience. Share your creations online without requiring users to install anything!
