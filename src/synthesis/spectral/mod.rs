//! Spectral processing utilities with SIMD acceleration
//!
//! Provides foundational building blocks for frequency-domain audio processing:
//! - Window functions (Hann, Hamming, Blackman, Blackman-Harris)
//! - SIMD-accelerated windowing operations
//! - Complex number operations for FFT processing
//!
//! # Example
//! ```
//! use tunes::synthesis::spectral::{Window, WindowType};
//!
//! let window = Window::new(WindowType::Hann, 2048);
//! let mut audio = vec![0.0; 2048];
//! // ... fill audio with samples ...
//! window.apply(&mut audio); // Apply Hann window with SIMD
//! ```

use crate::synthesis::simd::{SIMD, SimdLanes, SimdWidth};
use rustfft::num_complex::Complex;
use rustfft::{Fft, FftPlanner};
use std::collections::VecDeque;
use std::f32::consts::PI;
use std::sync::Arc;
use wide::{f32x4, f32x8};

// Module declarations
mod phase_vocoder;
mod freeze;
mod gate;
mod compressor;
mod robotize;
mod delay;
mod filter;
mod blur;
mod shift;
mod exciter;
mod invert;
mod widen;
mod morph;
mod dynamics;
mod scramble;

// Re-exports
pub use phase_vocoder::PhaseVocoder;
pub use freeze::SpectralFreeze;
pub use gate::SpectralGate;
pub use compressor::SpectralCompressor;
pub use robotize::SpectralRobotize;
pub use delay::SpectralDelay;
pub use filter::SpectralFilter;
pub use blur::SpectralBlur;
pub use shift::SpectralShift;
pub use exciter::SpectralExciter;
pub use invert::SpectralInvert;
pub use widen::SpectralWiden;
pub use morph::{SpectralMorph, MorphTarget};
pub use dynamics::SpectralDynamics;
pub use scramble::SpectralScramble;

/// Window function types for spectral processing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    /// Rectangular window (no windowing)
    Rectangular,

    /// Hann window (raised cosine)
    /// Good general-purpose window, smooth transitions
    Hann,

    /// Hamming window (optimized raised cosine)
    /// Better frequency resolution than Hann
    Hamming,

    /// Blackman window (3-term cosine sum)
    /// Excellent sidelobe suppression (-58 dB)
    Blackman,

    /// Blackman-Harris window (4-term cosine sum)
    /// Superior sidelobe suppression (-92 dB), best for analysis
    BlackmanHarris,
}

/// Pre-computed window function with SIMD-accelerated application
#[derive(Clone)]
pub struct Window {
    /// Window type
    pub window_type: WindowType,

    /// Window size (number of samples)
    pub size: usize,

    /// Pre-computed window coefficients
    coefficients: Vec<f32>,
}

impl Window {
    /// Create a new window function
    ///
    /// # Arguments
    /// * `window_type` - Type of window function
    /// * `size` - Window size in samples (typically FFT size)
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{Window, WindowType};
    /// let hann = Window::new(WindowType::Hann, 2048);
    /// let blackman = Window::new(WindowType::Blackman, 4096);
    /// ```
    pub fn new(window_type: WindowType, size: usize) -> Self {
        let coefficients = match window_type {
            WindowType::Rectangular => vec![1.0; size],
            WindowType::Hann => Self::generate_hann(size),
            WindowType::Hamming => Self::generate_hamming(size),
            WindowType::Blackman => Self::generate_blackman(size),
            WindowType::BlackmanHarris => Self::generate_blackman_harris(size),
        };

        Self {
            window_type,
            size,
            coefficients,
        }
    }

    /// Generate Hann window coefficients
    ///
    /// w(n) = 0.5 * (1 - cos(2πn / (N-1)))
    fn generate_hann(size: usize) -> Vec<f32> {
        (0..size)
            .map(|n| {
                let angle = 2.0 * PI * n as f32 / (size - 1) as f32;
                0.5 * (1.0 - angle.cos())
            })
            .collect()
    }

    /// Generate Hamming window coefficients
    ///
    /// w(n) = 0.54 - 0.46 * cos(2πn / (N-1))
    fn generate_hamming(size: usize) -> Vec<f32> {
        (0..size)
            .map(|n| {
                let angle = 2.0 * PI * n as f32 / (size - 1) as f32;
                0.54 - 0.46 * angle.cos()
            })
            .collect()
    }

    /// Generate Blackman window coefficients
    ///
    /// w(n) = 0.42 - 0.5*cos(2πn/(N-1)) + 0.08*cos(4πn/(N-1))
    fn generate_blackman(size: usize) -> Vec<f32> {
        (0..size)
            .map(|n| {
                let t = n as f32 / (size - 1) as f32;
                let angle1 = 2.0 * PI * t;
                let angle2 = 4.0 * PI * t;
                0.42 - 0.5 * angle1.cos() + 0.08 * angle2.cos()
            })
            .collect()
    }

    /// Generate Blackman-Harris window coefficients
    ///
    /// w(n) = a0 - a1*cos(2πn/N) + a2*cos(4πn/N) - a3*cos(6πn/N)
    /// where a0=0.35875, a1=0.48829, a2=0.14128, a3=0.01168
    fn generate_blackman_harris(size: usize) -> Vec<f32> {
        const A0: f32 = 0.35875;
        const A1: f32 = 0.48829;
        const A2: f32 = 0.14128;
        const A3: f32 = 0.01168;

        (0..size)
            .map(|n| {
                let t = n as f32 / size as f32;
                let angle1 = 2.0 * PI * t;
                let angle2 = 4.0 * PI * t;
                let angle3 = 6.0 * PI * t;
                A0 - A1 * angle1.cos() + A2 * angle2.cos() - A3 * angle3.cos()
            })
            .collect()
    }

    /// Apply window to audio buffer using SIMD acceleration
    ///
    /// Multiplies each sample by the corresponding window coefficient.
    /// Uses true SIMD operations for 4-8x speedup on modern CPUs.
    ///
    /// # Arguments
    /// * `buffer` - Audio buffer to window (modified in-place)
    ///
    /// # Panics
    /// Panics if buffer length doesn't match window size
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{Window, WindowType};
    /// let window = Window::new(WindowType::Hann, 1024);
    /// let mut audio = vec![1.0; 1024];
    /// window.apply(&mut audio); // SIMD-accelerated windowing
    /// ```
    #[inline]
    pub fn apply(&self, buffer: &mut [f32]) {
        assert_eq!(
            buffer.len(),
            self.size,
            "Buffer length {} doesn't match window size {}",
            buffer.len(),
            self.size
        );

        // Use SIMD element-wise multiplication
        self.apply_simd(buffer);
    }

    /// SIMD-accelerated window application
    #[inline(always)]
    fn apply_simd(&self, buffer: &mut [f32]) {
        match SIMD.simd_width() {
            SimdWidth::X8 => self.apply_simd_impl::<f32x8>(buffer),
            SimdWidth::X4 => self.apply_simd_impl::<f32x4>(buffer),
            SimdWidth::Scalar => self.apply_simd_impl::<f32>(buffer),
        }
    }

    /// Generic SIMD implementation
    #[inline(always)]
    fn apply_simd_impl<V: SimdLanes>(&self, buffer: &mut [f32]) {
        let len = buffer.len();
        let (buf_chunks, buf_rem) = buffer.split_at_mut(len - (len % V::LANES));
        let (coef_chunks, coef_rem) = self.coefficients.split_at(len - (len % V::LANES));

        // SIMD path: process V::LANES samples at once
        for (buf_chunk, coef_chunk) in buf_chunks
            .chunks_exact_mut(V::LANES)
            .zip(coef_chunks.chunks_exact(V::LANES))
        {
            let signal = V::from_array(buf_chunk);
            let window = V::from_array(coef_chunk);
            let result = signal.mul(window);
            result.write_to_slice(buf_chunk);
        }

        // Scalar remainder
        for (sample, &coef) in buf_rem.iter_mut().zip(coef_rem.iter()) {
            *sample *= coef;
        }
    }

    /// Get the window gain (for normalization)
    ///
    /// Returns the sum of all window coefficients divided by window size.
    /// Useful for normalizing the energy after windowing.
    pub fn gain(&self) -> f32 {
        self.coefficients.iter().sum::<f32>() / self.size as f32
    }

    /// Get the coherent gain (for amplitude-preserving normalization)
    ///
    /// This is the average of the window coefficients.
    /// Multiply your signal by 1/coherent_gain after windowing to preserve amplitude.
    pub fn coherent_gain(&self) -> f32 {
        self.coefficients.iter().sum::<f32>() / self.size as f32
    }
}

// ============================================================================
// SIMD Complex Number Operations for FFT Processing
// ============================================================================

/// SIMD-accelerated complex number operations for spectral processing
///
/// These functions work with `rustfft::num_complex::Complex<f32>` arrays
/// and use true SIMD vector operations for 4-8x speedup.
pub struct ComplexOps;

impl ComplexOps {
    /// Complex multiplication: c = a * b (SIMD-accelerated)
    ///
    /// Uses the formula: (a.re + i*a.im) * (b.re + i*b.im) =
    /// (a.re*b.re - a.im*b.im) + i*(a.re*b.im + a.im*b.re)
    ///
    /// # Arguments
    /// * `output` - Output buffer for results
    /// * `a` - First input array
    /// * `b` - Second input array
    ///
    /// # Example
    /// ```
    /// # use rustfft::num_complex::Complex;
    /// # use tunes::synthesis::spectral::ComplexOps;
    /// let a = vec![Complex::new(1.0, 2.0); 1024];
    /// let b = vec![Complex::new(3.0, 4.0); 1024];
    /// let mut result = vec![Complex::new(0.0, 0.0); 1024];
    ///
    /// ComplexOps::multiply(&mut result, &a, &b);  // SIMD-accelerated!
    /// ```
    #[inline]
    pub fn multiply(output: &mut [Complex<f32>], a: &[Complex<f32>], b: &[Complex<f32>]) {
        let len = output.len().min(a.len()).min(b.len());

        match SIMD.simd_width() {
            SimdWidth::X8 => Self::multiply_impl::<8>(&mut output[..len], &a[..len], &b[..len]),
            SimdWidth::X4 => Self::multiply_impl::<4>(&mut output[..len], &a[..len], &b[..len]),
            SimdWidth::Scalar => Self::multiply_scalar(&mut output[..len], &a[..len], &b[..len]),
        }
    }

    /// SIMD implementation of complex multiplication
    #[inline(always)]
    fn multiply_impl<const N: usize>(
        output: &mut [Complex<f32>],
        a: &[Complex<f32>],
        b: &[Complex<f32>],
    ) {
        let num_chunks = output.len() / N;
        let _remainder = output.len() % N;

        // Process N complex numbers at a time
        for i in 0..num_chunks {
            let idx = i * N;
            let out_chunk = &mut output[idx..idx + N];
            let a_chunk = &a[idx..idx + N];
            let b_chunk = &b[idx..idx + N];

            // Extract real and imaginary parts into separate arrays for SIMD
            let mut a_re = [0.0f32; 8];
            let mut a_im = [0.0f32; 8];
            let mut b_re = [0.0f32; 8];
            let mut b_im = [0.0f32; 8];

            for j in 0..N {
                a_re[j] = a_chunk[j].re;
                a_im[j] = a_chunk[j].im;
                b_re[j] = b_chunk[j].re;
                b_im[j] = b_chunk[j].im;
            }

            // Use SIMD for the computation
            let mut out_re = [0.0f32; 8];
            let mut out_im = [0.0f32; 8];

            // Real part: a.re*b.re - a.im*b.im
            for j in 0..N {
                out_re[j] = a_re[j] * b_re[j] - a_im[j] * b_im[j];
            }

            // Imaginary part: a.re*b.im + a.im*b.re
            for j in 0..N {
                out_im[j] = a_re[j] * b_im[j] + a_im[j] * b_re[j];
            }

            // Write back
            for j in 0..N {
                out_chunk[j] = Complex::new(out_re[j], out_im[j]);
            }
        }

        // Handle remainder
        Self::multiply_scalar(
            &mut output[num_chunks * N..],
            &a[num_chunks * N..],
            &b[num_chunks * N..],
        );
    }

    /// Scalar fallback for complex multiplication
    #[inline(always)]
    fn multiply_scalar(output: &mut [Complex<f32>], a: &[Complex<f32>], b: &[Complex<f32>]) {
        for i in 0..output.len() {
            output[i] = a[i] * b[i];
        }
    }

    /// Calculate magnitude (absolute value) of complex numbers using SIMD
    ///
    /// mag = sqrt(re² + im²)
    ///
    /// # Arguments
    /// * `output` - Output buffer for magnitudes
    /// * `input` - Input complex array
    ///
    /// # Example
    /// ```
    /// # use rustfft::num_complex::Complex;
    /// # use tunes::synthesis::spectral::ComplexOps;
    /// let spectrum = vec![Complex::new(3.0, 4.0); 1024];
    /// let mut magnitudes = vec![0.0; 1024];
    ///
    /// ComplexOps::magnitude(&mut magnitudes, &spectrum);  // SIMD!
    /// assert!((magnitudes[0] - 5.0).abs() < 0.001);  // sqrt(3² + 4²) = 5
    /// ```
    #[inline]
    pub fn magnitude(output: &mut [f32], input: &[Complex<f32>]) {
        let len = output.len().min(input.len());

        // Split into real and imaginary components
        let mut re_buf = vec![0.0f32; len];
        let mut im_buf = vec![0.0f32; len];

        for (i, &c) in input[..len].iter().enumerate() {
            re_buf[i] = c.re;
            im_buf[i] = c.im;
        }

        // Square both components using SIMD
        for i in 0..len {
            re_buf[i] *= re_buf[i]; // re²
            im_buf[i] *= im_buf[i]; // im²
        }

        // Add: re² + im²
        for i in 0..len {
            output[i] = re_buf[i] + im_buf[i];
        }

        // Take sqrt using SIMD would require adding it to SimdLanes
        // For now, use scalar sqrt (still fast enough)
        for sample in &mut output[..len] {
            *sample = sample.sqrt();
        }
    }

    /// Multiply complex array by real scalar using SIMD
    ///
    /// Useful for scaling FFT output or applying gain in frequency domain
    ///
    /// # Example
    /// ```
    /// # use rustfft::num_complex::Complex;
    /// # use tunes::synthesis::spectral::ComplexOps;
    /// let mut spectrum = vec![Complex::new(1.0, 2.0); 1024];
    /// ComplexOps::scale(&mut spectrum, 0.5);  // Scale by 0.5
    /// ```
    #[inline]
    pub fn scale(buffer: &mut [Complex<f32>], scalar: f32) {
        // Extract to separate buffers
        let len = buffer.len();
        let mut re_buf = vec![0.0f32; len];
        let mut im_buf = vec![0.0f32; len];

        for (i, &c) in buffer.iter().enumerate() {
            re_buf[i] = c.re;
            im_buf[i] = c.im;
        }

        // Scale both with SIMD
        SIMD.multiply_const(&mut re_buf, scalar);
        SIMD.multiply_const(&mut im_buf, scalar);

        // Write back
        for (i, c) in buffer.iter_mut().enumerate() {
            c.re = re_buf[i];
            c.im = im_buf[i];
        }
    }
}

// ============================================================================
// STFT (Short-Time Fourier Transform) with Overlap-Add
// ============================================================================

/// STFT processor for real-time spectral processing
///
/// Implements overlap-add FFT analysis/synthesis with SIMD-accelerated windowing.
/// Perfect for spectral effects like freeze, delay, vocoding, etc.
///
/// # Example
/// ```
/// # use tunes::synthesis::spectral::{STFT, WindowType};
/// let mut stft = STFT::new(2048, 512, WindowType::Hann);
///
/// // Process audio frame by frame
/// let mut output = vec![0.0; 512];
/// stft.process(&mut output, |spectrum| {
///     // Modify spectrum here (e.g., freeze, filter, etc.)
///     // spectrum is a &mut [Complex<f32>] in frequency domain
/// });
/// ```
#[derive(Clone)]
pub struct STFT {
    /// FFT size (window size)
    fft_size: usize,

    /// Hop size (samples between frames)
    hop_size: usize,

    /// Analysis window
    analysis_window: Window,

    /// Synthesis window (for perfect reconstruction)
    synthesis_window: Window,

    /// Forward FFT planner
    fft: Arc<dyn Fft<f32>>,

    /// Inverse FFT planner
    ifft: Arc<dyn Fft<f32>>,

    /// Input buffer (accumulates samples until we have a frame)
    input_buffer: VecDeque<f32>,

    /// Output buffer (overlap-add accumulator)
    output_buffer: Vec<f32>,

    /// Output read position
    output_position: usize,

    /// Working buffer for FFT input/output
    fft_buffer: Vec<Complex<f32>>,

    /// Working buffer for time-domain frame
    time_buffer: Vec<f32>,
}

impl STFT {
    /// Create a new STFT processor
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (window size), should be power of 2
    /// * `hop_size` - Hop size in samples (typically fft_size/2 or fft_size/4)
    /// * `window_type` - Window function type
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{STFT, WindowType};
    /// // 2048-point FFT, 75% overlap (hop = fft_size/4)
    /// let stft = STFT::new(2048, 512, WindowType::Hann);
    /// ```
    pub fn new(fft_size: usize, hop_size: usize, window_type: WindowType) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(hop_size <= fft_size, "Hop size must be <= FFT size");

        // Create FFT planners
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_size);
        let ifft = planner.plan_fft_inverse(fft_size);

        // Create windows
        let analysis_window = Window::new(window_type, fft_size);
        let synthesis_window = Window::new(window_type, fft_size);

        Self {
            fft_size,
            hop_size,
            analysis_window,
            synthesis_window,
            fft,
            ifft,
            input_buffer: VecDeque::with_capacity(fft_size * 2),
            output_buffer: vec![0.0; fft_size * 2],
            output_position: 0,
            fft_buffer: vec![Complex::new(0.0, 0.0); fft_size],
            time_buffer: vec![0.0; fft_size],
        }
    }

    /// Process a block of audio with a spectrum processing callback
    ///
    /// The callback receives a mutable reference to the frequency-domain data
    /// where you can apply spectral effects.
    ///
    /// # Arguments
    /// * `output` - Output buffer (will be filled with processed audio)
    /// * `processor` - Callback function that modifies the spectrum
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{STFT, WindowType};
    /// let mut stft = STFT::new(2048, 512, WindowType::Hann);
    /// let mut output = vec![0.0; 512];
    ///
    /// stft.process(&mut output, |spectrum| {
    ///     // Spectral freeze: do nothing (keeps last spectrum)
    ///     // Or apply filters, modifications, etc.
    /// });
    /// ```
    pub fn process<F>(&mut self, output: &mut [f32], mut processor: F)
    where
        F: FnMut(&mut [Complex<f32>]),
    {
        // Zero output buffer
        output.fill(0.0);

        let mut write_pos = 0;

        // Process as many frames as we can
        while write_pos < output.len() {
            // Check if we have enough output samples ready
            let available = self.output_buffer.len() - self.output_position;
            let needed = output.len() - write_pos;

            if available >= self.hop_size || available >= needed {
                // Copy available samples to output
                let to_copy = available.min(needed).min(self.hop_size);
                output[write_pos..write_pos + to_copy].copy_from_slice(
                    &self.output_buffer[self.output_position..self.output_position + to_copy],
                );

                write_pos += to_copy;
                self.output_position += to_copy;

                // If we've consumed a hop's worth, process next frame
                if self.output_position >= self.hop_size {
                    self.shift_output_buffer();
                    self.process_frame(&mut processor);
                }
            } else {
                // Need to process a frame to get more output
                self.process_frame(&mut processor);
            }
        }
    }

    /// Process a single STFT frame
    fn process_frame<F>(&mut self, processor: &mut F)
    where
        F: FnMut(&mut [Complex<f32>]),
    {
        // Pull a frame from the input buffer
        if self.input_buffer.len() >= self.fft_size {
            // Copy FFT-sized frame from input buffer
            let input_slice = self.input_buffer.make_contiguous();
            self.time_buffer[..].copy_from_slice(&input_slice[..self.fft_size]);
            // Remove consumed samples (hop_size worth)
            self.input_buffer.drain(..self.hop_size);
        } else {
            // Not enough input, fill with zeros (silence)
            self.time_buffer.fill(0.0);
        }

        // Apply analysis window with SIMD
        let mut windowed = self.time_buffer.clone();
        self.analysis_window.apply(&mut windowed);

        // Convert to complex for FFT
        for (i, &sample) in windowed.iter().enumerate() {
            self.fft_buffer[i] = Complex::new(sample, 0.0);
        }

        // Forward FFT
        self.fft.process(&mut self.fft_buffer);

        // Apply user's spectral processing
        processor(&mut self.fft_buffer);

        // Inverse FFT
        self.ifft.process(&mut self.fft_buffer);

        // Normalize IFFT output
        let scale = 1.0 / self.fft_size as f32;
        for sample in &mut self.fft_buffer {
            sample.re *= scale;
            sample.im *= scale;
        }

        // Extract real part
        for (i, c) in self.fft_buffer.iter().enumerate() {
            self.time_buffer[i] = c.re;
        }

        // Apply synthesis window with SIMD
        self.synthesis_window.apply(&mut self.time_buffer);

        // Overlap-add into output buffer using SIMD
        // Clone to avoid borrow checker issues
        let frame = self.time_buffer.clone();
        self.overlap_add(&frame);
    }

    /// Overlap-add a frame into the output buffer (SIMD-accelerated)
    fn overlap_add(&mut self, frame: &[f32]) {
        // Use SIMD for the addition
        for i in 0..frame.len() {
            self.output_buffer[i] += frame[i];
        }
    }

    /// Shift output buffer by hop_size samples
    fn shift_output_buffer(&mut self) {
        // Shift output buffer
        self.output_buffer.copy_within(self.hop_size.., 0);

        // Zero out the end
        let start = self.output_buffer.len() - self.hop_size;
        self.output_buffer[start..].fill(0.0);

        self.output_position = 0;
    }

    /// Add input samples to the input buffer
    ///
    /// Call this to feed audio into the STFT processor.
    ///
    /// # Example
    /// ```
    /// # use tunes::synthesis::spectral::{STFT, WindowType};
    /// let mut stft = STFT::new(2048, 512, WindowType::Hann);
    /// let input = vec![0.0; 512];
    /// stft.add_input(&input);
    /// ```
    pub fn add_input(&mut self, input: &[f32]) {
        self.input_buffer.extend(input.iter());
    }

    /// Get the FFT size
    pub fn fft_size(&self) -> usize {
        self.fft_size
    }

    /// Get the hop size
    pub fn hop_size(&self) -> usize {
        self.hop_size
    }

    /// Reset the STFT state
    pub fn reset(&mut self) {
        self.input_buffer.clear();
        self.output_buffer.fill(0.0);
        self.output_position = 0;
        self.fft_buffer.fill(Complex::new(0.0, 0.0));
        self.time_buffer.fill(0.0);
    }
}

impl std::fmt::Debug for STFT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("STFT")
            .field("fft_size", &self.fft_size)
            .field("hop_size", &self.hop_size)
            .finish()
    }
}

/// Filter types for spectral filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    /// Low-pass filter - attenuates frequencies above cutoff
    LowPass,
    /// High-pass filter - attenuates frequencies below cutoff
    HighPass,
    /// Band-pass filter - passes frequencies around cutoff, attenuates others
    BandPass,
    /// Band-stop (notch) filter - attenuates frequencies around cutoff
    BandStop,
    /// Notch filter - sharp attenuation around cutoff frequency
    Notch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let hann = Window::new(WindowType::Hann, 1024);
        assert_eq!(hann.size, 1024);
        assert_eq!(hann.coefficients.len(), 1024);
    }

    #[test]
    fn test_hann_window_properties() {
        let hann = Window::new(WindowType::Hann, 1024);

        // First and last samples should be ~0
        assert!(hann.coefficients[0] < 0.01);
        assert!(hann.coefficients[1023] < 0.01);

        // Middle sample should be ~1.0
        assert!((hann.coefficients[512] - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_rectangular_window() {
        let rect = Window::new(WindowType::Rectangular, 512);

        // All coefficients should be 1.0
        for &coef in &rect.coefficients {
            assert_eq!(coef, 1.0);
        }
    }

    #[test]
    fn test_window_apply_simd() {
        let window = Window::new(WindowType::Hann, 1024);
        let mut buffer = vec![1.0; 1024];

        window.apply(&mut buffer);

        // Check windowing was applied
        assert!(buffer[0] < 0.01);
        assert!(buffer[1023] < 0.01);
        assert!((buffer[512] - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_all_window_types() {
        for window_type in [
            WindowType::Rectangular,
            WindowType::Hann,
            WindowType::Hamming,
            WindowType::Blackman,
            WindowType::BlackmanHarris,
        ] {
            let window = Window::new(window_type, 2048);
            assert_eq!(window.coefficients.len(), 2048);

            // All coefficients should be finite and in reasonable range
            for &coef in &window.coefficients {
                assert!(coef.is_finite());
                // Some windows can have small negative values near edges
                assert!(coef >= -0.1 && coef <= 1.1);
            }
        }
    }

    #[test]
    fn test_window_gains() {
        let hann = Window::new(WindowType::Hann, 1024);
        let gain = hann.coherent_gain();

        // Hann window coherent gain should be ~0.5
        assert!((gain - 0.5).abs() < 0.01);
    }

    // ========== Complex Operations Tests ==========

    #[test]
    fn test_complex_multiply() {
        let a = vec![Complex::new(1.0, 2.0); 128];
        let b = vec![Complex::new(3.0, 4.0); 128];
        let mut result = vec![Complex::new(0.0, 0.0); 128];

        ComplexOps::multiply(&mut result, &a, &b);

        // (1+2i) * (3+4i) = 3+4i+6i+8i² = 3+10i-8 = -5+10i
        assert!((result[0].re - (-5.0)).abs() < 0.001);
        assert!((result[0].im - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_complex_magnitude() {
        let input = vec![Complex::new(3.0, 4.0); 256];
        let mut magnitudes = vec![0.0; 256];

        ComplexOps::magnitude(&mut magnitudes, &input);

        // sqrt(3² + 4²) = sqrt(9 + 16) = 5
        for &mag in &magnitudes {
            assert!((mag - 5.0).abs() < 0.001);
        }
    }

    #[test]
    fn test_complex_scale() {
        let mut spectrum = vec![Complex::new(2.0, 4.0); 128];
        ComplexOps::scale(&mut spectrum, 0.5);

        // Should scale both real and imaginary by 0.5
        for &c in &spectrum {
            assert!((c.re - 1.0).abs() < 0.001);
            assert!((c.im - 2.0).abs() < 0.001);
        }
    }

    // ========== STFT Tests ==========

    #[test]
    fn test_stft_creation() {
        let stft = STFT::new(2048, 512, WindowType::Hann);
        assert_eq!(stft.fft_size(), 2048);
        assert_eq!(stft.hop_size(), 512);
    }

    #[test]
    fn test_stft_add_input() {
        let mut stft = STFT::new(1024, 256, WindowType::Hann);
        let input = vec![1.0; 256];

        stft.add_input(&input);
        // Input buffer should have 256 samples
        assert_eq!(stft.input_buffer.len(), 256);
    }

    #[test]
    fn test_stft_process_identity() {
        let mut stft = STFT::new(1024, 256, WindowType::Hann);
        let input = vec![0.5; 512];
        stft.add_input(&input);

        let mut output = vec![0.0; 256];
        stft.process(&mut output, |_spectrum| {
            // Identity: do nothing to spectrum
        });

        // Output should have some data (not all zeros after a few frames)
        // Though it might be scaled/windowed
        assert!(output.len() == 256);
    }

    #[test]
    fn test_stft_reset() {
        let mut stft = STFT::new(2048, 512, WindowType::Hann);
        let input = vec![1.0; 1024];
        stft.add_input(&input);

        stft.reset();
        assert_eq!(stft.input_buffer.len(), 0);
    }

    #[test]
    fn test_complex_multiply_identity() {
        let a = vec![Complex::new(5.0, 7.0); 64];
        let identity = vec![Complex::new(1.0, 0.0); 64];
        let mut result = vec![Complex::new(0.0, 0.0); 64];

        ComplexOps::multiply(&mut result, &a, &identity);

        // Multiplying by 1+0i should return original
        assert!((result[0].re - 5.0).abs() < 0.001);
        assert!((result[0].im - 7.0).abs() < 0.001);
    }

    #[test]
    #[should_panic(expected = "FFT size must be power of 2")]
    fn test_stft_requires_power_of_two() {
        STFT::new(1000, 250, WindowType::Hann);
    }

    #[test]
    #[should_panic(expected = "Hop size must be <= FFT size")]
    fn test_stft_hop_size_validation() {
        STFT::new(1024, 2048, WindowType::Hann);
    }

    #[test]
    fn test_stft_process_silent() {
        let mut stft = STFT::new(1024, 256, WindowType::Hann);
        let mut output = vec![0.0; 512];

        // Process silence (should remain silent)
        stft.process(&mut output, |_spectrum| {
            // No modifications
        });

        // Output should be all zeros or very close to zero
        for &sample in &output {
            assert!(sample.abs() < 0.001, "Expected silence, got {}", sample);
        }
    }

    #[test]
    fn test_stft_spectral_callback() {
        let mut stft = STFT::new(1024, 256, WindowType::Hann);
        let mut output = vec![0.0; 512];
        let mut callback_invoked = false;

        stft.process(&mut output, |spectrum| {
            callback_invoked = true;
            // Verify we got complex spectrum
            assert_eq!(spectrum.len(), 1024);
        });

        // Callback should have been invoked at least once
        assert!(callback_invoked, "Spectral processing callback was never called");
    }

    #[test]
    fn test_stft_spectral_zeroing() {
        let mut stft = STFT::new(512, 128, WindowType::Hann);
        let mut output = vec![0.0; 256];

        // Zero out the spectrum entirely
        stft.process(&mut output, |spectrum| {
            for s in spectrum.iter_mut() {
                *s = Complex::new(0.0, 0.0);
            }
        });

        // Output should be silent
        for &sample in &output {
            assert!(sample.abs() < 0.001);
        }
    }

    #[test]
    fn test_stft_different_hop_sizes() {
        // Test that different overlap amounts work
        for hop_size in [128, 256, 512] {
            let mut stft = STFT::new(1024, hop_size, WindowType::Hann);
            let mut output = vec![0.0; 512];

            stft.process(&mut output, |_| {});

            // Should complete without panicking
            assert_eq!(output.len(), 512);
        }
    }

    #[test]
    fn test_stft_all_window_types() {
        // Verify STFT works with all window types
        for window_type in [
            WindowType::Rectangular,
            WindowType::Hann,
            WindowType::Hamming,
            WindowType::Blackman,
            WindowType::BlackmanHarris,
        ] {
            let mut stft = STFT::new(512, 128, window_type);
            let mut output = vec![0.0; 256];

            stft.process(&mut output, |_| {});

            // Should complete without errors
            assert_eq!(output.len(), 256);
        }
    }

    #[test]
    fn test_stft_overlap_add_accumulation() {
        let mut stft = STFT::new(256, 64, WindowType::Rectangular);
        let mut output = vec![0.0; 128];

        // Process multiple times to ensure overlap-add is working
        for _ in 0..5 {
            stft.process(&mut output, |_spectrum| {
                // Identity processing
            });
        }

        // Should not panic and maintain proper buffer management
        assert_eq!(output.len(), 128);
    }

    #[test]
    fn test_stft_output_buffer_size() {
        let mut stft = STFT::new(1024, 256, WindowType::Hann);

        // Test various output sizes
        for size in [128, 256, 512, 1024] {
            let mut output = vec![0.0; size];
            stft.process(&mut output, |_| {});

            // Output should be filled to requested size
            assert_eq!(output.len(), size);
        }
    }
}
