// GPU FFT debug - Print actual GPU vs CPU outputs
//
// Run with: cargo run --release --features gpu --example gpu_fft_debug

use anyhow::Result;
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use std::sync::Arc;
use tunes::gpu::{GpuDevice, GpuFft};

fn main() -> Result<()> {
    println!("GPU FFT Debug");
    println!("=============\n");

    // Initialize GPU
    let device = Arc::new(GpuDevice::new()?);
    println!();

    // Small size for debugging
    let size = 8;
    let mut gpu_fft = GpuFft::new(device.clone(), size)?;

    // Simple test: [0, 1, 2, 3, 4, 5, 6, 7]
    let mut gpu_data: Vec<Complex<f32>> = (0..size)
        .map(|i| Complex::new(i as f32, 0.0))
        .collect();

    let mut cpu_data = gpu_data.clone();

    println!("Input:");
    for (i, x) in gpu_data.iter().enumerate() {
        println!("  [{}] = {:.3} + {:.3}i", i, x.re, x.im);
    }
    println!();

    // GPU FFT
    gpu_fft.forward(&mut gpu_data)?;

    // CPU FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(size);
    fft.process(&mut cpu_data);

    println!("GPU FFT output:");
    for (i, x) in gpu_data.iter().enumerate() {
        println!("  [{}] = {:.3} + {:.3}i", i, x.re, x.im);
    }
    println!();

    println!("CPU FFT output:");
    for (i, x) in cpu_data.iter().enumerate() {
        println!("  [{}] = {:.3} + {:.3}i", i, x.re, x.im);
    }
    println!();

    println!("Differences:");
    for (i, (g, c)) in gpu_data.iter().zip(cpu_data.iter()).enumerate() {
        let diff_re = (g.re - c.re).abs();
        let diff_im = (g.im - c.im).abs();
        println!("  [{}] = {:.6} + {:.6}i", i, diff_re, diff_im);
    }

    // Try bit-reversing GPU output
    println!("\nGPU output with bit-reversal:");
    fn bit_reverse(mut index: usize, log2_size: u32) -> usize {
        let mut result = 0;
        for _ in 0..log2_size {
            result = (result << 1) | (index & 1);
            index >>= 1;
        }
        result
    }

    let mut gpu_reversed = gpu_data.clone();
    for i in 0..size {
        gpu_reversed[i] = gpu_data[bit_reverse(i, 3)];
    }

    for (i, c) in gpu_reversed.iter().enumerate() {
        println!("  [{}] = {:.3} + {:.3}i", i, c.re, c.im);
    }

    println!("\nDifferences after bit-reversal:");
    for (i, (g, c)) in gpu_reversed.iter().zip(cpu_data.iter()).enumerate() {
        let diff_re = (g.re - c.re).abs();
        let diff_im = (g.im - c.im).abs();
        println!("  [{}] = {:.6} + {:.6}i", i, diff_re, diff_im);
    }

    Ok(())
}
