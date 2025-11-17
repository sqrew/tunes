// GPU FFT Performance Benchmark
// Compares GPU FFT vs CPU FFT performance across different sizes

use anyhow::Result;
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use std::sync::Arc;
use std::time::Instant;
use tunes::gpu::{GpuDevice, GpuFft};

fn benchmark_cpu_fft(size: usize, iterations: usize) -> Result<f64> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(size);

    let mut data: Vec<Complex<f32>> = (0..size)
        .map(|i| Complex::new((i as f32).sin(), 0.0))
        .collect();

    // Warmup
    for _ in 0..5 {
        fft.process(&mut data);
    }

    let start = Instant::now();
    for _ in 0..iterations {
        fft.process(&mut data);
    }
    let elapsed = start.elapsed();

    Ok(elapsed.as_secs_f64() / iterations as f64 * 1_000_000.0) // Return microseconds
}

fn benchmark_gpu_fft(device: Arc<GpuDevice>, size: usize, iterations: usize) -> Result<f64> {
    let mut gpu_fft = GpuFft::new(device, size)?;

    let mut data: Vec<Complex<f32>> = (0..size)
        .map(|i| Complex::new((i as f32).sin(), 0.0))
        .collect();

    // Warmup
    for _ in 0..5 {
        gpu_fft.forward(&mut data)?;
    }

    let start = Instant::now();
    for _ in 0..iterations {
        gpu_fft.forward(&mut data)?;
    }
    let elapsed = start.elapsed();

    Ok(elapsed.as_secs_f64() / iterations as f64 * 1_000_000.0) // Return microseconds
}

fn benchmark_roundtrip_cpu(size: usize, iterations: usize) -> Result<f64> {
    let mut planner = FftPlanner::new();
    let fft_forward = planner.plan_fft_forward(size);
    let fft_inverse = planner.plan_fft_inverse(size);

    let mut data: Vec<Complex<f32>> = (0..size)
        .map(|i| Complex::new((i as f32).sin(), 0.0))
        .collect();

    // Warmup
    for _ in 0..5 {
        fft_forward.process(&mut data);
        fft_inverse.process(&mut data);
        for x in data.iter_mut() {
            *x /= size as f32;
        }
    }

    let start = Instant::now();
    for _ in 0..iterations {
        fft_forward.process(&mut data);
        fft_inverse.process(&mut data);
        for x in data.iter_mut() {
            *x /= size as f32;
        }
    }
    let elapsed = start.elapsed();

    Ok(elapsed.as_secs_f64() / iterations as f64 * 1_000_000.0)
}

fn benchmark_roundtrip_gpu(device: Arc<GpuDevice>, size: usize, iterations: usize) -> Result<f64> {
    let mut gpu_fft = GpuFft::new(device, size)?;

    let mut data: Vec<Complex<f32>> = (0..size)
        .map(|i| Complex::new((i as f32).sin(), 0.0))
        .collect();

    // Warmup
    for _ in 0..5 {
        gpu_fft.forward(&mut data)?;
        gpu_fft.inverse(&mut data)?;
    }

    let start = Instant::now();
    for _ in 0..iterations {
        gpu_fft.forward(&mut data)?;
        gpu_fft.inverse(&mut data)?;
    }
    let elapsed = start.elapsed();

    Ok(elapsed.as_secs_f64() / iterations as f64 * 1_000_000.0)
}

fn main() -> Result<()> {
    println!("\n🎵 GPU FFT Benchmark\n");
    println!("Comparing CPU vs GPU FFT performance\n");

    // Initialize GPU device
    let device = match GpuDevice::new() {
        Ok(d) => {
            println!();
            Arc::new(d)
        }
        Err(e) => {
            eprintln!("⚠️  GPU not available: {}. Cannot run GPU benchmarks.", e);
            return Ok(());
        }
    };

    let sizes = vec![256, 512, 1024, 2048, 4096];
    let iterations = 1000;

    println!("Forward FFT (single direction)");
    println!("=====================================");
    println!("{:<10} {:<15} {:<15} {:<15}", "Size", "CPU (μs)", "GPU (μs)", "Speedup");
    println!("-------------------------------------");

    for size in &sizes {
        let cpu_time = benchmark_cpu_fft(*size, iterations)?;
        let gpu_time = benchmark_gpu_fft(device.clone(), *size, iterations)?;
        let speedup = cpu_time / gpu_time;

        println!(
            "{:<10} {:<15.2} {:<15.2} {:<15.2}x",
            size, cpu_time, gpu_time, speedup
        );
    }

    println!("\nRound-trip FFT (forward + inverse)");
    println!("=====================================");
    println!("{:<10} {:<15} {:<15} {:<15}", "Size", "CPU (μs)", "GPU (μs)", "Speedup");
    println!("-------------------------------------");

    for size in &sizes {
        let cpu_time = benchmark_roundtrip_cpu(*size, iterations / 2)?;
        let gpu_time = benchmark_roundtrip_gpu(device.clone(), *size, iterations / 2)?;
        let speedup = cpu_time / gpu_time;

        println!(
            "{:<10} {:<15.2} {:<15.2} {:<15.2}x",
            size, cpu_time, gpu_time, speedup
        );
    }

    println!("\n✅ Benchmark complete!");

    Ok(())
}
