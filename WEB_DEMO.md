# Tunes WebAssembly Demo

This guide explains how to build and run the Tunes audio library in WebAssembly.

## Prerequisites

1. **Install wasm-pack** (if you haven't already):
   ```bash
   cargo install wasm-pack
   ```

2. **Install a local web server** (choose one):
   ```bash
   # Python 3 (usually pre-installed on Linux/Mac)
   python3 -m http.server 8000

   # Or install a simple static server
   cargo install basic-http-server
   # or
   npm install -g http-server
   ```

## Building the Demo

Build the WebAssembly package with the `web` feature enabled:

```bash
wasm-pack build --target web --features web
```

This will:
- Compile the Rust code to WebAssembly
- Generate JavaScript bindings
- Create a `pkg/` directory with the compiled WASM and JS files

## Running the Demo

1. **Start a local web server** in the project root:

   Using Python:
   ```bash
   python3 -m http.server 8000
   ```

   Using basic-http-server:
   ```bash
   basic-http-server .
   ```

   Using http-server:
   ```bash
   http-server -p 8000
   ```

2. **Open your browser** and navigate to:
   ```
   http://localhost:8000/examples/web_demo.html
   ```

3. **Click the button** to play a 440Hz sine wave tone!

## What the Demo Does

The web demo demonstrates:

- ✅ Tunes audio engine running in WebAssembly
- ✅ Real-time audio synthesis in the browser
- ✅ Sample generation and playback
- ✅ Using the Mixer and Track API
- ✅ Integration with Web Audio API via cpal

## Features Available on Web

The following Tunes features work on WebAssembly:

- ✅ Audio synthesis (oscillators, wavetables)
- ✅ Sample playback from memory
- ✅ Effects processing (reverb, delay, filters, etc.)
- ✅ Mixer and track system
- ✅ Real-time audio playback
- ✅ Sample transformations (normalize, fade, pitch shift, time stretch)

## Features NOT Available on Web

The following features are disabled on WASM (native-only):

- ❌ File streaming (`stream_file()`, `stream_file_looping()`)
- ❌ Multi-threaded processing (falls back to sequential)
- ❌ File system access (use `Sample::from_bytes()` instead)

## Troubleshooting

### CORS Errors
If you see CORS errors in the console, make sure you're using a proper web server (not `file://` protocol).

### Audio Not Playing
- Check browser console for errors
- Ensure your browser supports Web Audio API (all modern browsers do)
- Some browsers require user interaction before playing audio

### Build Errors
Make sure you have:
- Latest Rust toolchain: `rustup update`
- wasm32 target installed: `rustup target add wasm32-unknown-unknown`
- Latest wasm-pack: `cargo install wasm-pack --force`

## Next Steps

To use Tunes in your own web application:

1. Add Tunes to your `Cargo.toml`:
   ```toml
   [dependencies]
   tunes = { version = "0.19", features = ["web"] }
   wasm-bindgen = "0.2"
   ```

2. Build with wasm-pack:
   ```bash
   wasm-pack build --target web --features web
   ```

3. Import in your JavaScript/TypeScript:
   ```javascript
   import init, { AudioEngine } from './pkg/tunes.js';

   await init();
   // Now you can use Tunes!
   ```

## Performance Notes

- SIMD operations work in WebAssembly for fast DSP
- No multi-threading on web (single-threaded audio processing)
- Sample rate typically 48kHz (browser default)
- Latency depends on browser's audio buffer size

Enjoy making music in the browser with Tunes! 🎵
