//! Streaming audio support for playing long files from disk.
//!
//! Uses a background decoder thread and lock-free ring buffer for efficient
//! streaming without loading entire files into memory.

#![cfg(not(target_arch = "wasm32"))]

use ringbuf::traits::Producer;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::conv::IntoSample;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::sample::Sample as SymphoniaSample;

/// State for a streaming audio source (native only)
///
/// Streams audio from disk using a background decoder thread and lock-free ring buffer.
/// This allows playing long audio files (background music, ambience) without loading
/// the entire file into memory.
pub(crate) struct StreamingSound {
    /// Ring buffer consumer (audio thread reads from this)
    pub ring_consumer: ringbuf::HeapCons<f32>,
    /// Decoder thread handle (for cleanup on stop)
    pub decoder_thread: Option<JoinHandle<()>>,
    /// Signal to stop the decoder thread
    pub stop_signal: Arc<AtomicBool>,
    /// Pause signal for decoder thread
    pub pause_signal: Arc<AtomicBool>,
    /// Current volume
    pub volume: f32,
    /// Current pan (-1.0 left, 0.0 center, 1.0 right)
    pub pan: f32,
    /// Whether the stream is looping
    #[allow(dead_code)]
    pub looping: bool,
}

impl Drop for StreamingSound {
    fn drop(&mut self) {
        // Signal thread to stop and wait for it to finish
        self.stop_signal.store(true, Ordering::Relaxed);
        if let Some(handle) = self.decoder_thread.take() {
            let _ = handle.join();
        }
    }
}

/// Decoder thread function for streaming audio
///
/// Runs in a background thread, decodes audio from file, and pushes samples to ring buffer.
/// The audio callback reads from the ring buffer, creating a lock-free streaming pipeline.
pub(crate) fn decoder_thread_func(
    path: PathBuf,
    mut ring_producer: ringbuf::HeapProd<f32>,
    stop_signal: Arc<AtomicBool>,
    pause_signal: Arc<AtomicBool>,
    looping: bool,
) {
    // Helper function to convert symphonia audio buffer to f32 samples
    fn convert_audio_buffer(decoded: &AudioBufferRef, samples: &mut Vec<f32>) {
        fn convert_samples<S>(buf: &symphonia::core::audio::AudioBuffer<S>, samples: &mut Vec<f32>)
        where
            S: SymphoniaSample + IntoSample<f32>,
        {
            let num_channels = buf.spec().channels.count();
            let num_frames = buf.frames();
            samples.clear();
            samples.reserve(num_frames * num_channels);

            // Convert planar to interleaved
            for frame_idx in 0..num_frames {
                for ch in 0..num_channels {
                    let sample: f32 = buf.chan(ch)[frame_idx].into_sample();
                    samples.push(sample);
                }
            }
        }

        match decoded {
            AudioBufferRef::U8(buf) => convert_samples(buf, samples),
            AudioBufferRef::U16(buf) => convert_samples(buf, samples),
            AudioBufferRef::U24(buf) => convert_samples(buf, samples),
            AudioBufferRef::U32(buf) => convert_samples(buf, samples),
            AudioBufferRef::S8(buf) => convert_samples(buf, samples),
            AudioBufferRef::S16(buf) => convert_samples(buf, samples),
            AudioBufferRef::S24(buf) => convert_samples(buf, samples),
            AudioBufferRef::S32(buf) => convert_samples(buf, samples),
            AudioBufferRef::F32(buf) => convert_samples(buf, samples),
            AudioBufferRef::F64(buf) => convert_samples(buf, samples),
        }
    }

    loop {
        // Check stop signal
        if stop_signal.load(Ordering::Relaxed) {
            break;
        }

        // If paused, sleep briefly and continue
        if pause_signal.load(Ordering::Relaxed) {
            thread::sleep(std::time::Duration::from_millis(10));
            continue;
        }

        // Open and decode file
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Streaming: Failed to open file {:?}: {}", path, e);
                break;
            }
        };

        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();
        let decoder_opts = DecoderOptions::default();

        let probed = match symphonia::default::get_probe().format(
            &hint,
            mss,
            &format_opts,
            &metadata_opts,
        ) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Streaming: Failed to probe file {:?}: {}", path, e);
                break;
            }
        };

        let mut format = probed.format;
        let track = match format.default_track() {
            Some(t) => t,
            None => {
                eprintln!("Streaming: No default track found in {:?}", path);
                break;
            }
        };

        let mut decoder =
            match symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Streaming: Failed to create decoder for {:?}: {}", path, e);
                    break;
                }
            };

        let mut samples = Vec::new();

        // Decode loop
        loop {
            // Check stop/pause signals
            if stop_signal.load(Ordering::Relaxed) {
                return; // Exit thread entirely
            }

            if pause_signal.load(Ordering::Relaxed) {
                thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }

            // Get next packet
            let packet = match format.next_packet() {
                Ok(p) => p,
                Err(symphonia::core::errors::Error::IoError(e))
                    if e.kind() == std::io::ErrorKind::UnexpectedEof =>
                {
                    // End of file
                    if looping {
                        break; // Break inner loop, restart outer loop
                    } else {
                        return; // Exit thread
                    }
                }
                Err(e) => {
                    eprintln!("Streaming: Error reading packet: {}", e);
                    return;
                }
            };

            // Decode packet
            let decoded = match decoder.decode(&packet) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Streaming: Decode error: {}", e);
                    continue;
                }
            };

            // Convert to f32 samples
            convert_audio_buffer(&decoded, &mut samples);

            // Push samples to ring buffer (blocking if buffer is full)
            let mut offset = 0;
            while offset < samples.len() {
                // Check stop signal even while pushing
                if stop_signal.load(Ordering::Relaxed) {
                    return;
                }

                // Try to push as much as possible
                let pushed = ring_producer.push_slice(&samples[offset..]);
                offset += pushed;

                // If we couldn't push everything, the buffer is full - sleep briefly
                if pushed == 0 {
                    thread::sleep(std::time::Duration::from_millis(1));
                }
            }
        }

        // If not looping, we exit after one playthrough
        if !looping {
            break;
        }
    }
}
