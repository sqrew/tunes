use crate::{
    rhythm::{NoteDuration, Tempo},
    run,
};

pub fn play_notes(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    frequencies: &[f32],
    duration_secs: f32,
) {
    match config.sample_format() {
        cpal::SampleFormat::F32 => {
            run::<f32>(device, &config.clone().into(), frequencies, duration_secs).unwrap()
        }
        cpal::SampleFormat::I16 => {
            run::<i16>(device, &config.clone().into(), frequencies, duration_secs).unwrap()
        }
        cpal::SampleFormat::U16 => {
            run::<u16>(device, &config.clone().into(), frequencies, duration_secs).unwrap()
        }
        _ => panic!("Unsupported format"),
    }
}
pub fn play_notes_tempo(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    frequencies: &[f32],
    duration: NoteDuration,
    tempo: &Tempo,
) {
    let duration_secs = tempo.duration_to_seconds(duration);
    play_notes(device, config, frequencies, duration_secs);
}

#[derive(Debug, Clone, Copy)]
pub enum ArpeggioStyle {
    Up,
    Down,
    UpDown,
    Random,
}

pub fn play_arpeggio_tempo(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    scale: &[f32],
    style: ArpeggioStyle,
    note_duration: NoteDuration,
    tempo: &Tempo,
) {
    let duration_secs = tempo.duration_to_seconds(note_duration);
    match style {
        ArpeggioStyle::Up => {
            for &freq in scale {
                play_notes(device, config, &[freq], duration_secs);
            }
        }
        ArpeggioStyle::Down => {
            for &freq in scale.iter().rev() {
                play_notes(device, config, &[freq], duration_secs);
            }
        }
        ArpeggioStyle::UpDown => {
            // Go up
            for &freq in scale {
                play_notes(device, config, &[freq], duration_secs);
            }
            // Go down (skip first note to avoid repetition)
            for &freq in scale.iter().rev().skip(1) {
                play_notes(device, config, &[freq], duration_secs);
            }
        }
        ArpeggioStyle::Random => {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            let mut shuffled = scale.to_vec();
            shuffled.shuffle(&mut rng);
            for freq in shuffled {
                play_notes(device, config, &[freq], duration_secs);
            }
        }
    }
}

pub fn play_interpolated(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    start_freq: f32,
    end_freq: f32,
    segments: usize,
    note_duration: f32,
) {
    for i in 0..segments {
        let t = i as f32 / (segments - 1) as f32;
        let freq = start_freq + (end_freq - start_freq) * t;
        play_notes(device, config, &[freq], note_duration);
    }
}
