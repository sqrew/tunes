//! Spectral scramble - randomize frequency bin order
//!
//! Shuffles or randomizes frequency bins for glitchy, experimental effects.

use crate::synthesis::spectral::{WindowType, STFT};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Spectral scramble processor
#[derive(Clone)]
pub struct SpectralScramble {
    stft: STFT,
    fft_size: usize,
    sample_rate: f32,
    scramble_amount: f32,  // 0-1
    low_freq: f32,         // Hz
    high_freq: f32,        // Hz
    mix: f32,              // 0-1
    enabled: bool,
    rng: StdRng,
    permutation: Vec<usize>,
}

impl SpectralScramble {
    pub fn new(fft_size: usize, hop_size: usize, window: WindowType, sample_rate: f32) -> Self {
        let num_bins = fft_size / 2 + 1;
        let rng = StdRng::seed_from_u64(42); // Deterministic seed
        Self {
            stft: STFT::new(fft_size, hop_size, window),
            fft_size,
            sample_rate,
            scramble_amount: 1.0,
            low_freq: 200.0,
            high_freq: 12000.0,
            mix: 1.0,
            enabled: true,
            rng,
            permutation: (0..num_bins).collect(),
        }
    }

    pub fn scramble_amount(&self) -> f32 { self.scramble_amount }
    pub fn set_scramble_amount(&mut self, amount: f32) {
        self.scramble_amount = amount.clamp(0.0, 1.0);
        self.update_permutation();
    }

    pub fn low_freq(&self) -> f32 { self.low_freq }
    pub fn set_low_freq(&mut self, freq: f32) {
        self.low_freq = freq.max(0.0);
        self.update_permutation();
    }

    pub fn high_freq(&self) -> f32 { self.high_freq }
    pub fn set_high_freq(&mut self, freq: f32) {
        self.high_freq = freq.min(self.sample_rate / 2.0);
        self.update_permutation();
    }

    pub fn mix(&self) -> f32 { self.mix }
    pub fn set_mix(&mut self, mix: f32) { self.mix = mix.clamp(0.0, 1.0); }

    fn update_permutation(&mut self) {
        let num_bins = self.permutation.len();
        let bin_width = self.sample_rate / self.fft_size as f32;
        let low_bin = (self.low_freq / bin_width) as usize;
        let high_bin = ((self.high_freq / bin_width) as usize).min(num_bins - 1);

        // Reset to identity
        for i in 0..num_bins {
            self.permutation[i] = i;
        }

        // Shuffle bins in range based on scramble_amount
        if self.scramble_amount > 0.0 {
            let shuffle_count = ((high_bin - low_bin) as f32 * self.scramble_amount) as usize;
            for _ in 0..shuffle_count {
                let i = self.rng.random_range(low_bin..=high_bin);
                let j = self.rng.random_range(low_bin..=high_bin);
                self.permutation.swap(i, j);
            }
        }
    }

    pub fn subtle() -> Self {
        let mut s = Self::new(2048, 512, WindowType::Hann, 44100.0);
        s.scramble_amount = 0.3; s.low_freq = 1000.0; s.high_freq = 8000.0; s.mix = 0.5;
        s.update_permutation();
        s
    }

    pub fn moderate() -> Self {
        let mut s = Self::new(2048, 512, WindowType::Hann, 44100.0);
        s.scramble_amount = 0.6; s.low_freq = 500.0; s.high_freq = 12000.0; s.mix = 0.7;
        s.update_permutation();
        s
    }

    pub fn chaos() -> Self {
        let mut s = Self::new(2048, 512, WindowType::Hann, 44100.0);
        s.scramble_amount = 1.0; s.low_freq = 200.0; s.high_freq = 16000.0; s.mix = 1.0;
        s.update_permutation();
        s
    }

    pub fn glitch() -> Self {
        let mut s = Self::new(2048, 512, WindowType::Hann, 44100.0);
        s.scramble_amount = 0.8; s.low_freq = 2000.0; s.high_freq = 8000.0; s.mix = 0.9;
        s.update_permutation();
        s
    }

    pub fn digital() -> Self {
        let mut s = Self::new(2048, 512, WindowType::Hann, 44100.0);
        s.scramble_amount = 0.5; s.low_freq = 4000.0; s.high_freq = 12000.0; s.mix = 0.8;
        s.update_permutation();
        s
    }

    pub fn process(&mut self, output: &mut [f32], input: &[f32]) {
        if !self.enabled {
            output.copy_from_slice(input);
            return;
        }

        self.stft.add_input(input);
        let permutation = self.permutation.clone();

        self.stft.process(output, |spectrum| {
            let mut scrambled = spectrum.to_vec();
            for i in 0..spectrum.len() {
                scrambled[i] = spectrum[permutation[i]];
            }

            // Mix with original
            for i in 0..spectrum.len() {
                spectrum[i].re = spectrum[i].re * (1.0 - self.mix) + scrambled[i].re * self.mix;
                spectrum[i].im = spectrum[i].im * (1.0 - self.mix) + scrambled[i].im * self.mix;
            }
        });
    }

    pub fn reset(&mut self) {
        self.stft.reset();
    }
}
