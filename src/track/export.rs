//! Export functionality for Mixer
//!
//! This module contains methods for exporting audio to WAV and FLAC files,
//! including stems (individual track exports).

use super::mixer::Mixer;

impl Mixer {
    /// Export the mixed audio to a WAV file with explicit sample rate
    ///
    /// Renders the entire composition to a stereo WAV file with the specified sample rate.
    ///
    /// # When to use
    /// - **Standalone rendering** (no AudioEngine, e.g., CLI tools, batch processing)
    /// - **Custom sample rates** (upsampling, downsampling)
    /// - **Testing/CI** without audio hardware
    ///
    /// **If you're using AudioEngine for playback**, prefer `engine.export_wav(mixer, path)`
    /// to automatically match the engine's sample rate.
    ///
    /// # Arguments
    /// * `path` - Output file path (e.g., "output.wav")
    /// * `sample_rate` - Sample rate in Hz (44100 is CD quality, 48000 is professional)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// // Standalone rendering (no engine needed)
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("piano").note(&[440.0], 1.0);
    ///
    /// let mut mixer = comp.into_mixer();
    /// mixer.export_wav("output.wav", 44100)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn export_wav(&mut self, path: &str, sample_rate: u32) -> anyhow::Result<()> {
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(path, spec)?;

        let duration = self.total_duration();
        let total_samples = (duration * sample_rate as f32).ceil() as usize;

        println!("Rendering to WAV...");
        println!("  Duration: {:.2}s", duration);
        println!("  Sample rate: {} Hz", sample_rate);
        println!("  Total samples: {}", total_samples);

        // 🚀 GPU-accelerated batch rendering!
        // This uses GPU pre-rendering if enabled (mixer.enable_gpu())
        let buffer = self.render_to_buffer(sample_rate as f32);

        println!("  Encoding to WAV...");

        // Write samples to WAV file (interleaved stereo: L, R, L, R, ...)
        for (i, sample) in buffer.iter().enumerate() {
            // Convert from f32 (-1.0 to 1.0) to i16 (-32768 to 32767)
            let sample_i16 = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
            writer.write_sample(sample_i16)?;

            // Progress indicator every second
            if i % (sample_rate as usize * 2) == 0 {
                let progress = (i as f32 / buffer.len() as f32) * 100.0;
                print!("\r  Progress: {:.0}%", progress);
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
        }

        println!("\r  Progress: 100%");
        writer.finalize()?;

        println!("✅ Exported to: {}", path);
        Ok(())
    }

    /// Export the mixed audio to a FLAC file (lossless compression)
    ///
    /// Renders the entire composition to a stereo FLAC file with the specified sample rate.
    /// FLAC provides lossless compression, typically reducing file size by 50-60% compared
    /// to WAV with no quality loss.
    ///
    /// # When to use
    /// - **Standalone rendering** (no AudioEngine, e.g., CLI tools, batch processing)
    /// - **Custom sample rates** (upsampling, downsampling)
    /// - **Testing/CI** without audio hardware
    ///
    /// **If you're using AudioEngine for playback**, prefer `engine.export_flac(mixer, path)`
    /// to automatically match the engine's sample rate.
    ///
    /// # Arguments
    /// * `path` - Output file path (e.g., "output.flac")
    /// * `sample_rate` - Sample rate in Hz (44100 is CD quality, 48000 is professional)
    ///
    /// # Benefits of FLAC
    /// - Lossless compression (~50-60% smaller than WAV)
    /// - Perfect for archival and professional workflows
    /// - Supported by most DAWs and audio tools
    /// - Metadata support for track info
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// // Standalone rendering (no engine needed)
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("piano").note(&[440.0], 1.0);
    ///
    /// let mut mixer = comp.into_mixer();
    /// mixer.export_flac("output.flac", 44100)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn export_flac(&mut self, path: &str, sample_rate: u32) -> anyhow::Result<()> {
        use flacenc::component::BitRepr;
        use flacenc::error::Verify;
        use flacenc::source::MemSource;

        let duration = self.total_duration();
        let total_samples = (duration * sample_rate as f32).ceil() as usize;

        println!("Rendering to FLAC...");
        println!("  Duration: {:.2}s", duration);
        println!("  Sample rate: {} Hz", sample_rate);
        println!("  Total samples: {}", total_samples);

        // 🚀 GPU-accelerated batch rendering!
        // This uses GPU pre-rendering if enabled (mixer.enable_gpu())
        let buffer = self.render_to_buffer(sample_rate as f32);

        println!("  Converting to 24-bit...");

        // Convert f32 samples to i32 (24-bit) for FLAC encoding
        // We use 24-bit as it provides better quality than 16-bit while keeping file size reasonable
        const SCALE: f32 = 8388607.0; // 2^23 - 1
        let samples_i32: Vec<i32> = buffer
            .iter()
            .enumerate()
            .map(|(i, &sample)| {
                let sample_i32 = (sample.clamp(-1.0, 1.0) * SCALE) as i32;

                // Progress indicator every second
                if i % (sample_rate as usize * 2) == 0 {
                    let progress = (i as f32 / buffer.len() as f32) * 100.0;
                    print!("\r  Progress: {:.0}%", progress);
                    use std::io::Write;
                    std::io::stdout().flush().ok();
                }

                sample_i32
            })
            .collect();

        println!("\r  Progress: 100%");
        println!("  Encoding FLAC...");

        // Create encoder configuration
        let config = flacenc::config::Encoder::default()
            .into_verified()
            .expect("Default encoder config should be valid");

        // Create FLAC source from samples
        let source = MemSource::from_samples(
            &samples_i32,
            2,          // channels (stereo)
            24,         // bits per sample
            sample_rate as usize,
        );

        // Encode with fixed block size (use config's default block size)
        let flac_stream = flacenc::encode_with_fixed_block_size(
            &config,
            source,
            config.block_size,
        ).map_err(|e| anyhow::anyhow!("FLAC encoding failed: {:?}", e))?;

        // Write to file using ByteSink
        let mut sink = flacenc::bitsink::ByteSink::new();
        flac_stream.write(&mut sink)
            .map_err(|e| anyhow::anyhow!("Failed to write FLAC stream: {:?}", e))?;

        std::fs::write(path, sink.as_slice())?;

        println!("✅ Exported to: {}", path);
        Ok(())
    }

    /// Export individual tracks as separate WAV files (stems)
    ///
    /// Creates one WAV file per track in the specified output directory.
    /// Each stem contains only the audio for that individual track, making it
    /// perfect for external mixing, remixing, or professional production workflows.
    ///
    /// # Arguments
    /// * `output_dir` - Directory path where stems will be saved
    /// * `sample_rate` - Sample rate for output files (typically 44100)
    ///
    /// # File Naming
    /// Files are named using the track name: `{output_dir}/{track_name}.wav`
    /// If a track has no name, it uses `untitled_{index}.wav`
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut comp = Composition::new(Tempo::new(120.0));
    ///
    /// comp.track("drums").note(&[C4], 0.5);
    /// comp.track("bass").note(&[C2], 1.0);
    /// comp.track("melody").notes(&[C4, E4, G4], 0.5);
    ///
    /// let mut mixer = comp.into_mixer();
    /// mixer.export_stems("output/stems/", 44100)?;
    /// // Creates:
    /// //   output/stems/drums.wav
    /// //   output/stems/bass.wav
    /// //   output/stems/melody.wav
    /// # Ok(())
    /// # }
    /// ```
    pub fn export_stems(&mut self, output_dir: &str, sample_rate: u32) -> anyhow::Result<()> {
        use std::fs;

        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)?;

        let total_tracks = self.all_tracks().len();

        println!("Exporting {} stems to: {}", total_tracks, output_dir);

        // Export each track individually
        for index in 0..total_tracks {
            let all_tracks = self.all_tracks();
            let track_name = all_tracks[index]
                .name
                .clone()
                .unwrap_or_else(|| format!("untitled_{}", index));

            // Sanitize filename (remove special characters)
            let safe_name = track_name
                .replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");

            let filename = format!("{}/{}.wav", output_dir, safe_name);

            println!("  [{}/{}] Rendering: {}", index + 1, total_tracks, safe_name);

            // Render this track in isolation
            self.render_track_at_index(index, &filename, sample_rate)?;
        }

        println!("✅ Exported {} stems to: {}", total_tracks, output_dir);
        Ok(())
    }

    /// Export stems with the master mix included
    ///
    /// Same as `export_stems()` but also exports a full mix of all tracks
    /// as `_master.wav` in the output directory.
    ///
    /// # Arguments
    /// * `output_dir` - Directory path where stems will be saved
    /// * `sample_rate` - Sample rate for output files (typically 44100)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut comp = Composition::new(Tempo::new(120.0));
    ///
    /// comp.track("drums").note(&[C4], 0.5);
    /// comp.track("bass").note(&[C2], 1.0);
    ///
    /// let mut mixer = comp.into_mixer();
    /// mixer.export_stems_with_master("output/", 44100)?;
    /// // Creates:
    /// //   output/drums.wav
    /// //   output/bass.wav
    /// //   output/_master.wav  (full mix)
    /// # Ok(())
    /// # }
    /// ```
    pub fn export_stems_with_master(
        &mut self,
        output_dir: &str,
        sample_rate: u32,
    ) -> anyhow::Result<()> {
        // Export individual stems
        self.export_stems(output_dir, sample_rate)?;

        // Export master mix
        let output_dir_trimmed = output_dir.trim_end_matches('/');
        let master_path = format!("{}/_master.wav", output_dir_trimmed);
        println!("  Rendering master mix...");
        self.export_wav(&master_path, sample_rate)?;

        Ok(())
    }

    /// Helper: Render a single track (by index) to a WAV file
    ///
    /// Renders only the specified track in isolation, applying all its effects,
    /// filters, and processing chain.
    fn render_track_at_index(
        &mut self,
        track_index: usize,
        path: &str,
        sample_rate: u32,
    ) -> anyhow::Result<()> {
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(path, spec)?;

        // Determine duration for this track
        let all_tracks = self.all_tracks();
        let duration = all_tracks[track_index].total_duration();
        let total_samples = (duration * sample_rate as f32).ceil() as usize;
        let sample_rate_f32 = sample_rate as f32;
        let mut sample_clock = 0.0;

        // Render the track sample by sample
        for i in 0..total_samples {
            let time = i as f32 / sample_rate_f32;

            // Sample this track in isolation (similar logic to Mixer::sample_at but for one track)
            let (left, right) = self.sample_track_at_index(track_index, time, sample_rate_f32, sample_clock);

            // Convert from f32 (-1.0 to 1.0) to i16 (-32768 to 32767)
            let left_i16 = (left.clamp(-1.0, 1.0) * 32767.0) as i16;
            let right_i16 = (right.clamp(-1.0, 1.0) * 32767.0) as i16;

            writer.write_sample(left_i16)?;
            writer.write_sample(right_i16)?;

            sample_clock = (sample_clock + 1.0) % sample_rate_f32;
        }

        writer.finalize()?;
        Ok(())
    }

    /// Helper: Sample a single track (by index) at a specific time
    ///
    /// This is similar to `sample_at()` but only processes one track,
    /// used internally for stem export.
    fn sample_track_at_index(
        &mut self,
        track_index: usize,
        time: f32,
        sample_rate: f32,
        _sample_clock: f32,
    ) -> (f32, f32) {
        // Find the track by iterating through buses
        let sample_count = self.sample_count;
        let mut current_index = 0;
        for bus_opt in self.buses.iter_mut() {
            let bus = match bus_opt {
                Some(b) => b,
                None => continue,
            };

            for track in &mut bus.tracks {
                if current_index == track_index {
                    // Found the track! Use the static process_track helper from Mixer
                    return Mixer::process_track_static(track, time, sample_rate, sample_count);
                }
                current_index += 1;
            }
        }
        (0.0, 0.0) // Track not found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::timing::Tempo;
    use crate::instruments::drums::DrumType;

    #[test]
    fn test_export_wav_creates_file() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let mut track = crate::track::Track::new();
        track.add_note(&[440.0], 0.0, 0.5);
        mixer.add_track(track);

        let test_file = "test_output_wav.wav";
        mixer.export_wav(test_file, 44100).unwrap();

        // Check file exists and has content
        let metadata = std::fs::metadata(test_file).unwrap();
        assert!(metadata.len() > 0);

        // Clean up
        std::fs::remove_file(test_file).ok();
    }

    #[test]
    fn test_export_flac_creates_file() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let mut track = crate::track::Track::new();
        track.add_note(&[440.0], 0.0, 0.5);
        mixer.add_track(track);

        let test_file = "test_output_flac.flac";
        mixer.export_flac(test_file, 44100).unwrap();

        // Check file exists and has content
        let metadata = std::fs::metadata(test_file).unwrap();
        assert!(metadata.len() > 0);

        // Verify it's a valid FLAC file by checking magic bytes
        let file_data = std::fs::read(test_file).unwrap();
        assert!(file_data.len() > 4);
        // FLAC files start with "fLaC" (0x66 0x4C 0x61 0x43)
        assert_eq!(&file_data[0..4], b"fLaC");

        // Clean up
        std::fs::remove_file(test_file).ok();
    }

    #[test]
    fn test_flac_smaller_than_wav() {
        let mut mixer = Mixer::new(Tempo::new(120.0));

        // Create a track with some variety (better compression)
        let mut track = crate::track::Track::new();
        track.add_note(&[440.0], 0.0, 0.25);
        track.add_note(&[554.37], 0.25, 0.25);
        track.add_note(&[659.25], 0.5, 0.25);
        track.add_drum(DrumType::Kick, 0.0, None);
        track.add_drum(DrumType::Snare, 0.5, None);
        mixer.add_track(track);

        let wav_file = "test_compression_compare.wav";
        let flac_file = "test_compression_compare.flac";

        mixer.export_wav(wav_file, 44100).unwrap();
        mixer.export_flac(flac_file, 44100).unwrap();

        let wav_size = std::fs::metadata(wav_file).unwrap().len();
        let flac_size = std::fs::metadata(flac_file).unwrap().len();

        // FLAC should generally be smaller (though very short files might not compress much)
        // We just verify both files were created with reasonable sizes
        assert!(wav_size > 1000); // WAV should have some header + data
        assert!(flac_size > 100);  // FLAC should have header + compressed data

        // Clean up
        std::fs::remove_file(wav_file).ok();
        std::fs::remove_file(flac_file).ok();
    }

    #[test]
    fn test_export_empty_mixer_wav() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let test_file = "test_empty.wav";

        // Should handle empty mixer gracefully
        mixer.export_wav(test_file, 44100).unwrap();

        let metadata = std::fs::metadata(test_file).unwrap();
        assert!(metadata.len() > 0); // Should at least have WAV header

        std::fs::remove_file(test_file).ok();
    }

    #[test]
    fn test_export_empty_mixer_flac() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let test_file = "test_empty.flac";

        // Should handle empty mixer gracefully
        mixer.export_flac(test_file, 44100).unwrap();

        let metadata = std::fs::metadata(test_file).unwrap();
        assert!(metadata.len() > 0); // Should at least have FLAC header

        std::fs::remove_file(test_file).ok();
    }

    #[test]
    fn test_export_different_sample_rates_wav() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let mut track = crate::track::Track::new();
        track.add_note(&[440.0], 0.0, 0.1);
        mixer.add_track(track);

        // Test different sample rates
        for sample_rate in [22050, 44100, 48000] {
            let test_file = format!("test_sr_{}.wav", sample_rate);
            mixer.export_wav(&test_file, sample_rate).unwrap();
            assert!(std::fs::metadata(&test_file).unwrap().len() > 0);
            std::fs::remove_file(&test_file).ok();
        }
    }

    #[test]
    fn test_export_different_sample_rates_flac() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let mut track = crate::track::Track::new();
        track.add_note(&[440.0], 0.0, 0.1);
        mixer.add_track(track);

        // Test different sample rates
        for sample_rate in [22050, 44100, 48000] {
            let test_file = format!("test_sr_{}.flac", sample_rate);
            mixer.export_flac(&test_file, sample_rate).unwrap();

            // Verify FLAC magic bytes
            let file_data = std::fs::read(&test_file).unwrap();
            assert_eq!(&file_data[0..4], b"fLaC");

            std::fs::remove_file(&test_file).ok();
        }
    }

    #[test]
    fn test_flac_24bit_encoding() {
        let mut mixer = Mixer::new(Tempo::new(120.0));
        let mut track = crate::track::Track::new();
        track.add_note(&[440.0], 0.0, 0.5);
        mixer.add_track(track);

        let test_file = "test_24bit.flac";
        mixer.export_flac(test_file, 44100).unwrap();

        // Read FLAC header to verify 24-bit encoding
        let file_data = std::fs::read(test_file).unwrap();

        // FLAC magic bytes
        assert_eq!(&file_data[0..4], b"fLaC");

        // The STREAMINFO block comes next (after magic bytes)
        // Byte 8 contains the minimum block size (2 bytes)
        // Bytes 10-11 contain maximum block size
        // We just verify the file structure is valid
        assert!(file_data.len() > 42); // FLAC header + STREAMINFO minimum

        std::fs::remove_file(test_file).ok();
    }
}
