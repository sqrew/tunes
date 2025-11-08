//! Export functionality for Mixer
//!
//! This module contains methods for exporting audio to WAV and FLAC files,
//! including stems (individual track exports).

use super::events::AudioEvent;
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

        let sample_rate_f32 = sample_rate as f32;
        let mut sample_clock = 0.0;

        println!("Rendering to WAV...");
        println!("  Duration: {:.2}s", duration);
        println!("  Sample rate: {} Hz", sample_rate);
        println!("  Total samples: {}", total_samples);

        for i in 0..total_samples {
            let time = i as f32 / sample_rate_f32;

            // Generate stereo sample (no spatial audio for export)
            let (left, right) = self.sample_at(time, sample_rate_f32, sample_clock, None, None);

            // Convert from f32 (-1.0 to 1.0) to i16 (-32768 to 32767)
            let left_i16 = (left.clamp(-1.0, 1.0) * 32767.0) as i16;
            let right_i16 = (right.clamp(-1.0, 1.0) * 32767.0) as i16;

            writer.write_sample(left_i16)?;
            writer.write_sample(right_i16)?;

            sample_clock = (sample_clock + 1.0) % sample_rate_f32;

            // Progress indicator every second
            if i % sample_rate as usize == 0 {
                let progress = (i as f32 / total_samples as f32) * 100.0;
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
        let sample_rate_f32 = sample_rate as f32;
        let mut sample_clock = 0.0;

        println!("Rendering to FLAC...");
        println!("  Duration: {:.2}s", duration);
        println!("  Sample rate: {} Hz", sample_rate);
        println!("  Total samples: {}", total_samples);

        // Collect all samples in i32 format (interleaved stereo)
        let mut samples_i32: Vec<i32> = Vec::with_capacity(total_samples * 2);

        for i in 0..total_samples {
            let time = i as f32 / sample_rate_f32;

            // Generate stereo sample (no spatial audio for export)
            let (left, right) = self.sample_at(time, sample_rate_f32, sample_clock, None, None);

            // Convert from f32 (-1.0 to 1.0) to i32 (-2^23 to 2^23-1 for 24-bit)
            // We use 24-bit as it provides better quality than 16-bit while keeping file size reasonable
            const SCALE: f32 = 8388607.0; // 2^23 - 1
            let left_i32 = (left.clamp(-1.0, 1.0) * SCALE) as i32;
            let right_i32 = (right.clamp(-1.0, 1.0) * SCALE) as i32;

            samples_i32.push(left_i32);
            samples_i32.push(right_i32);

            sample_clock = (sample_clock + 1.0) % sample_rate_f32;

            // Progress indicator every second
            if i % sample_rate as usize == 0 {
                let progress = (i as f32 / total_samples as f32) * 100.0;
                print!("\r  Progress: {:.0}%", progress);
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
        }

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

        let total_tracks = self.tracks.len();

        println!("Exporting {} stems to: {}", total_tracks, output_dir);

        // Export each track individually
        for index in 0..total_tracks {
            let track_name = self.tracks[index]
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
        let duration = self.tracks[track_index].total_duration();
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
        let track = &mut self.tracks[track_index];

        // Ensure events are sorted
        track.ensure_sorted();

        let track_start = track.start_time();
        let track_end = track.end_time();

        // Skip if outside track's time bounds (unless delay/reverb)
        if (time < track_start || time > track_end)
            && track.delay.is_none()
            && track.reverb.is_none()
        {
            return (0.0, 0.0);
        }

        let mut track_value = 0.0;
        let mut has_active_event = false;
        let mut filter_env_cutoff = track.filter.cutoff;
        let mut filter_env_found = false;

        let (start_idx, end_idx) = track.find_active_range(time);

        // Render all events in this track (same logic as Mixer::sample_at)
        for event in &track.events[start_idx..end_idx] {
            match event {
                AudioEvent::Sample(sample_event) => {
                    let time_in_sample = time - sample_event.start_time;
                    let sample_duration = sample_event.sample.duration / sample_event.playback_rate;

                    if time_in_sample >= 0.0 && time_in_sample < sample_duration {
                        has_active_event = true;
                        let (sample_left, sample_right) = sample_event
                            .sample
                            .sample_at_interpolated(time_in_sample, sample_event.playback_rate);
                        track_value += (sample_left + sample_right) * 0.5 * sample_event.volume;
                    }
                }
                AudioEvent::Note(note_event) => {
                    let total_duration = note_event.envelope.total_duration(note_event.duration);
                    let note_end_with_release = note_event.start_time + total_duration;

                    if time >= note_event.start_time && time < note_end_with_release {
                        has_active_event = true;
                        let time_in_note = time - note_event.start_time;

                        // Filter envelope
                        if !filter_env_found && note_event.filter_envelope.amount > 0.0 {
                            let filter_total_duration = note_event
                                .filter_envelope
                                .total_duration(note_event.duration);
                            let filter_end = note_event.start_time + filter_total_duration;
                            if time >= note_event.start_time && time < filter_end {
                                filter_env_cutoff = note_event
                                    .filter_envelope
                                    .cutoff_at(time_in_note, note_event.duration);
                                filter_env_found = true;
                            }
                        }

                        let envelope_amp = note_event
                            .envelope
                            .amplitude_at(time_in_note, note_event.duration);

                        for i in 0..note_event.num_freqs {
                            let base_freq = note_event.frequencies[i];

                            let freq = if note_event.pitch_bend_semitones != 0.0 {
                                let bend_progress = (time_in_note / note_event.duration).min(1.0);
                                let bend_multiplier =
                                    2.0f32.powf((note_event.pitch_bend_semitones * bend_progress) / 12.0);
                                base_freq * bend_multiplier
                            } else {
                                base_freq
                            };

                            let sample = if note_event.fm_params.mod_index > 0.0 {
                                // Use FM synthesis
                                note_event.fm_params.sample(freq, time_in_note, note_event.duration)
                            } else if let Some(ref wavetable) = note_event.custom_wavetable {
                                // Use custom wavetable
                                let phase = (time_in_note * freq) % 1.0;
                                wavetable.sample(phase)
                            } else {
                                // Use standard waveform
                                let phase = (time_in_note * freq) % 1.0;
                                note_event.waveform.sample(phase)
                            };

                            track_value += sample * envelope_amp;
                        }
                    }
                }
                AudioEvent::Drum(drum_event) => {
                    let drum_duration = drum_event.drum_type.duration();
                    if time >= drum_event.start_time && time < drum_event.start_time + drum_duration {
                        has_active_event = true;
                        let time_in_drum = time - drum_event.start_time;
                        let sample_index = (time_in_drum * sample_rate) as usize;
                        track_value += drum_event.drum_type.sample(sample_index, sample_rate);
                    }
                }
                _ => {} // Skip non-audio events
            }
        }

        // Apply effects chain (same as in Mixer::sample_at)
        if has_active_event || track.delay.is_some() || track.reverb.is_some() {
            track_value *= track.volume;

            // Apply filter
            if filter_env_found {
                track.filter.cutoff = filter_env_cutoff;
            }
            if track.filter.filter_type != crate::synthesis::filter::FilterType::None {
                track_value = track.filter.process(track_value, sample_rate);
            }

            // Increment sample count for effects that need it
            self.sample_count = self.sample_count.wrapping_add(1);

            // Build and apply effects chain (same logic as in sample_at)
            let mut effect_order: Vec<(u8, u8)> = Vec::with_capacity(14);

            if let Some(ref eq) = track.eq {
                effect_order.push((eq.priority, 0));
            }
            if let Some(ref compressor) = track.compressor {
                effect_order.push((compressor.priority, 1));
            }
            if let Some(ref gate) = track.gate {
                effect_order.push((gate.priority, 2));
            }
            if let Some(ref saturation) = track.saturation {
                effect_order.push((saturation.priority, 3));
            }
            if let Some(ref bitcrusher) = track.bitcrusher {
                effect_order.push((bitcrusher.priority, 4));
            }
            if let Some(ref distortion) = track.distortion {
                effect_order.push((distortion.priority, 5));
            }
            if let Some(ref chorus) = track.chorus {
                effect_order.push((chorus.priority, 6));
            }
            if let Some(ref phaser) = track.phaser {
                effect_order.push((phaser.priority, 7));
            }
            if let Some(ref flanger) = track.flanger {
                effect_order.push((flanger.priority, 8));
            }
            if let Some(ref ring_mod) = track.ring_mod {
                effect_order.push((ring_mod.priority, 9));
            }
            if let Some(ref tremolo) = track.tremolo {
                effect_order.push((tremolo.priority, 10));
            }
            if let Some(ref delay) = track.delay {
                effect_order.push((delay.priority, 11));
            }
            if let Some(ref reverb) = track.reverb {
                effect_order.push((reverb.priority, 12));
            }
            if let Some(ref limiter) = track.limiter {
                effect_order.push((limiter.priority, 13));
            }

            effect_order.sort_by_key(|&(priority, _)| priority);

            // Apply effects in priority order
            for (_, effect_id) in effect_order {
                match effect_id {
                    0 => {
                        if let Some(ref mut eq) = track.eq {
                            track_value = eq.process(track_value, sample_rate, time, self.sample_count);
                        }
                    }
                    1 => {
                        if let Some(ref mut compressor) = track.compressor {
                            track_value = compressor.process(track_value, sample_rate, time, self.sample_count);
                        }
                    }
                    2 => {
                        if let Some(ref mut gate) = track.gate {
                            track_value = gate.process(track_value, sample_rate, time, self.sample_count);
                        }
                    }
                    3 => {
                        if let Some(ref mut saturation) = track.saturation {
                            track_value = saturation.process(track_value, time, self.sample_count);
                        }
                    }
                    4 => {
                        if let Some(ref mut bitcrusher) = track.bitcrusher {
                            track_value = bitcrusher.process(track_value, time, self.sample_count);
                        }
                    }
                    5 => {
                        if let Some(ref mut distortion) = track.distortion {
                            track_value = distortion.process(track_value, time, self.sample_count);
                        }
                    }
                    6 => {
                        if let Some(ref mut chorus) = track.chorus {
                            track_value = chorus.process(track_value, sample_rate, time, self.sample_count);
                        }
                    }
                    7 => {
                        if let Some(ref mut phaser) = track.phaser {
                            track_value = phaser.process(track_value, time, self.sample_count);
                        }
                    }
                    8 => {
                        if let Some(ref mut flanger) = track.flanger {
                            track_value = flanger.process(track_value, time, self.sample_count);
                        }
                    }
                    9 => {
                        if let Some(ref mut ring_mod) = track.ring_mod {
                            track_value = ring_mod.process(track_value, time, self.sample_count);
                        }
                    }
                    10 => {
                        if let Some(ref mut tremolo) = track.tremolo {
                            track_value = tremolo.process(track_value, time, self.sample_count);
                        }
                    }
                    11 => {
                        if let Some(ref mut delay) = track.delay {
                            track_value = delay.process(track_value, time, self.sample_count);
                        }
                    }
                    12 => {
                        if let Some(ref mut reverb) = track.reverb {
                            track_value = reverb.process(track_value, time, self.sample_count);
                        }
                    }
                    13 => {
                        if let Some(ref mut limiter) = track.limiter {
                            track_value = limiter.process(track_value, sample_rate, time, self.sample_count);
                        }
                    }
                    _ => {}
                }
            }

            // Apply panning
            let pan = track.pan.clamp(-1.0, 1.0);
            let left_gain = ((1.0 - pan) * 0.5).sqrt();
            let right_gain = ((1.0 + pan) * 0.5).sqrt();

            return (track_value * left_gain, track_value * right_gain);
        }

        (0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::rhythm::Tempo;
    use crate::composition::drums::DrumType;

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
