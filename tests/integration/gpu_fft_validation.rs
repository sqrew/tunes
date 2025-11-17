// GPU FFT validation - Compare GPU FFT against CPU rustfft
//
// This example validates that our GPU FFT implementation produces
// results identical (or very close) to CPU FFT.
//
// Run with: cargo run --release --features gpu --example gpu_fft_validation

use anyhow::Result;
use rustfft::num_complex::Complex;
use rustfft::{Fft, FftPlanner};
use std::f32::consts::PI;
use std::sync::Arc;
use tunes::gpu::{GpuDevice, GpuFft};

fn main() -> Result<()> {
    println!("GPU FFT Validation");
    println!("==================\n");

    // Initialize GPU
    let device = Arc::new(GpuDevice::new()?);
    println!();

    // Test different FFT sizes
    let sizes = vec![256, 512, 1024, 2048];

    for size in sizes {
        println!("Testing FFT size: {}", size);
        println!("{}", "-".repeat(40));

        test_fft_size(&device, size)?;
        println!();
    }

    println!("✓ All FFT validations passed!");
    println!("\nGPU FFT is producing correct results! 🎉");

    Ok(())
}

fn test_fft_size(device: &Arc<GpuDevice>, size: usize) -> Result<()> {
    // Create GPU FFT
    let mut gpu_fft = GpuFft::new(device.clone(), size)?;

    // Create CPU FFT for comparison
    let mut planner = FftPlanner::new();
    let cpu_fft_forward = planner.plan_fft_forward(size);
    let cpu_fft_inverse = planner.plan_fft_inverse(size);

    // Test 1: Impulse (delta function)
    println!("  Test 1: Impulse response");
    test_impulse(&mut gpu_fft, &cpu_fft_forward, size)?;

    // Test 2: Sine wave
    println!("  Test 2: Sine wave (10 Hz)");
    test_sine_wave(&mut gpu_fft, &cpu_fft_forward, size, 10.0)?;

    // Test 3: Complex signal
    println!("  Test 3: Complex signal (multiple frequencies)");
    test_complex_signal(&mut gpu_fft, &cpu_fft_forward, size)?;

    // Test 4: Round-trip (forward then inverse)
    println!("  Test 4: Round-trip (FFT → IFFT)");
    test_round_trip(&mut gpu_fft, &cpu_fft_inverse, size)?;

    // Test 5: Random noise
    println!("  Test 5: Random noise");
    test_random_noise(&mut gpu_fft, &cpu_fft_forward, size)?;

    Ok(())
}

fn test_impulse(gpu_fft: &mut GpuFft, cpu_fft: &Arc<dyn Fft<f32>>, size: usize) -> Result<()> {
    // Create impulse signal
    let mut gpu_data = vec![Complex::new(0.0, 0.0); size];
    gpu_data[0] = Complex::new(1.0, 0.0);  // Impulse at t=0

    let mut cpu_data = gpu_data.clone();

    // Compute FFTs
    gpu_fft.forward(&mut gpu_data)?;
    cpu_fft.process(&mut cpu_data);

    // Compare results
    let max_error = compare_results(&gpu_data, &cpu_data);
    println!("    Max error: {:.6e}", max_error);

    if max_error > 0.001 {
        anyhow::bail!("FFT error too large: {}", max_error);
    }

    Ok(())
}

fn test_sine_wave(
    gpu_fft: &mut GpuFft,
    cpu_fft: &Arc<dyn Fft<f32>>,
    size: usize,
    freq: f32,
) -> Result<()> {
    // Create sine wave
    let mut gpu_data = vec![Complex::new(0.0, 0.0); size];
    for i in 0..size {
        let t = i as f32 / size as f32;
        gpu_data[i] = Complex::new((2.0 * PI * freq * t).sin(), 0.0);
    }

    let mut cpu_data = gpu_data.clone();

    // Compute FFTs
    gpu_fft.forward(&mut gpu_data)?;
    cpu_fft.process(&mut cpu_data);

    // Compare results
    let max_error = compare_results(&gpu_data, &cpu_data);
    println!("    Max error: {:.6e}", max_error);

    if max_error > 0.001 {
        anyhow::bail!("FFT error too large: {}", max_error);
    }

    Ok(())
}

fn test_complex_signal(
    gpu_fft: &mut GpuFft,
    cpu_fft: &Arc<dyn Fft<f32>>,
    size: usize,
) -> Result<()> {
    // Create complex signal (sum of multiple sine waves)
    let mut gpu_data = vec![Complex::new(0.0, 0.0); size];
    for i in 0..size {
        let t = i as f32 / size as f32;
        let signal = (2.0 * PI * 5.0 * t).sin() * 0.5
            + (2.0 * PI * 12.0 * t).sin() * 0.3
            + (2.0 * PI * 25.0 * t).sin() * 0.2;
        gpu_data[i] = Complex::new(signal, 0.0);
    }

    let mut cpu_data = gpu_data.clone();

    // Compute FFTs
    gpu_fft.forward(&mut gpu_data)?;
    cpu_fft.process(&mut cpu_data);

    // Compare results
    let max_error = compare_results(&gpu_data, &cpu_data);
    println!("    Max error: {:.6e}", max_error);

    if max_error > 0.001 {
        anyhow::bail!("FFT error too large: {}", max_error);
    }

    Ok(())
}

fn test_round_trip(
    gpu_fft: &mut GpuFft,
    _cpu_ifft: &Arc<dyn Fft<f32>>,
    size: usize,
) -> Result<()> {
    // Create test signal
    let original = (0..size)
        .map(|i| {
            let t = i as f32 / size as f32;
            Complex::new((2.0 * PI * 7.0 * t).sin(), 0.0)
        })
        .collect::<Vec<_>>();

    let mut gpu_data = original.clone();

    // Forward then inverse FFT
    gpu_fft.forward(&mut gpu_data)?;
    gpu_fft.inverse(&mut gpu_data)?;

    // Compare with original
    let max_error = compare_results(&gpu_data, &original);
    println!("    Max error: {:.6e}", max_error);

    if max_error > 0.01 {
        // Slightly higher tolerance for round-trip
        anyhow::bail!("Round-trip error too large: {}", max_error);
    }

    Ok(())
}

fn test_random_noise(
    gpu_fft: &mut GpuFft,
    cpu_fft: &Arc<dyn Fft<f32>>,
    size: usize,
) -> Result<()> {
    use rand::Rng;
    let mut rng = rand::rng();

    // Create random noise
    let mut gpu_data = (0..size)
        .map(|_| {
            Complex::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            )
        })
        .collect::<Vec<_>>();

    let mut cpu_data = gpu_data.clone();

    // Compute FFTs
    gpu_fft.forward(&mut gpu_data)?;
    cpu_fft.process(&mut cpu_data);

    // Compare results
    let max_error = compare_results(&gpu_data, &cpu_data);
    println!("    Max error: {:.6e}", max_error);

    if max_error > 0.001 {
        anyhow::bail!("FFT error too large: {}", max_error);
    }

    Ok(())
}

/// Compare two complex arrays and return maximum absolute error
fn compare_results(a: &[Complex<f32>], b: &[Complex<f32>]) -> f32 {
    let mut max_error = 0.0f32;

    for (x, y) in a.iter().zip(b.iter()) {
        let error_re = (x.re - y.re).abs();
        let error_im = (x.im - y.im).abs();
        max_error = max_error.max(error_re).max(error_im);
    }

    max_error
}
