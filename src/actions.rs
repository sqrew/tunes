//! Legacy audio playback functions
//!
//! This module contains low-level audio playback utilities that were used
//! in earlier versions of the library. Most functionality has been superseded
//! by the `AudioEngine` and `Composition` APIs.
//!
//! **Note:** This module is internal and not exposed in the public API.
//! Users should use `AudioEngine::play_mixer()` instead.

use crate::{
    error::{Result, TunesError},
    rhythm::{NoteDuration, Tempo},
    run,
};

/// Play notes directly to an audio device
///
/// Low-level function that plays a chord (multiple frequencies) for a duration.
/// Handles different sample formats (F32, I16, U16).
///
/// # Arguments
/// * `device` - The cpal audio output device
/// * `config` - Audio stream configuration
/// * `frequencies` - Array of note frequencies in Hz to play simultaneously
/// * `duration_secs` - How long to play the notes, in seconds
///
/// # Errors
/// Returns an error if the audio format is unsupported or playback fails.
pub fn play_notes(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    frequencies: &[f32],
    duration_secs: f32,
) -> Result<()> {
    match config.sample_format() {
        cpal::SampleFormat::F32 => {
            run::<f32>(device, &config.clone().into(), frequencies, duration_secs)?
        }
        cpal::SampleFormat::I16 => {
            run::<i16>(device, &config.clone().into(), frequencies, duration_secs)?
        }
        cpal::SampleFormat::U16 => {
            run::<u16>(device, &config.clone().into(), frequencies, duration_secs)?
        }
        _ => return Err(TunesError::InvalidAudioFormat("Unsupported audio format".to_string())),
    }
    Ok(())
}

/// Play notes with tempo-based duration
///
/// Similar to `play_notes()` but uses musical duration (quarter note, eighth note, etc.)
/// and tempo to calculate the playback time in seconds.
///
/// # Arguments
/// * `device` - The cpal audio output device
/// * `config` - Audio stream configuration
/// * `frequencies` - Array of note frequencies in Hz to play simultaneously
/// * `duration` - Musical duration (e.g., NoteDuration::Quarter)
/// * `tempo` - Tempo to use for duration calculation
///
/// # Errors
/// Returns an error if playback fails.
pub fn play_notes_tempo(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    frequencies: &[f32],
    duration: NoteDuration,
    tempo: &Tempo,
) -> Result<()> {
    let duration_secs = tempo.duration_to_seconds(duration);
    play_notes(device, config, frequencies, duration_secs)
}

/// Arpeggio playback styles
///
/// Defines different patterns for playing a sequence of notes.
#[derive(Debug, Clone, Copy)]
pub enum ArpeggioStyle {
    /// Play notes from low to high
    Up,
    /// Play notes from high to low
    Down,
    /// Play notes up then back down (triangle pattern)
    UpDown,
    /// Play notes in random order
    Random,
}

/// Play an arpeggio pattern with tempo-based timing
///
/// Plays a scale or chord as an arpeggio in various patterns.
/// Each note is played for the specified duration.
///
/// # Arguments
/// * `device` - The cpal audio output device
/// * `config` - Audio stream configuration
/// * `scale` - Array of note frequencies to arpeggiate
/// * `style` - Arpeggio pattern (Up, Down, UpDown, Random)
/// * `note_duration` - Musical duration for each note
/// * `tempo` - Tempo to use for duration calculation
///
/// # Errors
/// Returns an error if playback fails.
pub fn play_arpeggio_tempo(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    scale: &[f32],
    style: ArpeggioStyle,
    note_duration: NoteDuration,
    tempo: &Tempo,
) -> Result<()> {
    let duration_secs = tempo.duration_to_seconds(note_duration);
    match style {
        ArpeggioStyle::Up => {
            for &freq in scale {
                play_notes(device, config, &[freq], duration_secs)?;
            }
        }
        ArpeggioStyle::Down => {
            for &freq in scale.iter().rev() {
                play_notes(device, config, &[freq], duration_secs)?;
            }
        }
        ArpeggioStyle::UpDown => {
            // Go up
            for &freq in scale {
                play_notes(device, config, &[freq], duration_secs)?;
            }
            // Go down (skip first note to avoid repetition)
            for &freq in scale.iter().rev().skip(1) {
                play_notes(device, config, &[freq], duration_secs)?;
            }
        }
        ArpeggioStyle::Random => {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            let mut shuffled = scale.to_vec();
            shuffled.shuffle(&mut rng);
            for freq in shuffled {
                play_notes(device, config, &[freq], duration_secs)?;
            }
        }
    }
    Ok(())
}

/// Play a pitch glide by interpolating between two frequencies
///
/// Creates a stepped pitch glide effect by playing a sequence of notes
/// at intermediate frequencies between start and end. The more segments,
/// the smoother the glide.
///
/// # Arguments
/// * `device` - The cpal audio output device
/// * `config` - Audio stream configuration
/// * `start_freq` - Starting frequency in Hz
/// * `end_freq` - Ending frequency in Hz
/// * `segments` - Number of steps in the interpolation (higher = smoother)
/// * `note_duration` - Duration of each step in seconds
///
/// # Example
/// ```ignore
/// // Glide from C4 (261.63 Hz) to C5 (523.25 Hz) in 10 steps
/// play_interpolated(&device, &config, 261.63, 523.25, 10, 0.1)?;
/// ```
///
/// # Errors
/// Returns an error if playback fails.
pub fn play_interpolated(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    start_freq: f32,
    end_freq: f32,
    segments: usize,
    note_duration: f32,
) -> Result<()> {
    for i in 0..segments {
        let t = i as f32 / (segments - 1) as f32;
        let freq = start_freq + (end_freq - start_freq) * t;
        play_notes(device, config, &[freq], note_duration)?;
    }
    Ok(())
}
