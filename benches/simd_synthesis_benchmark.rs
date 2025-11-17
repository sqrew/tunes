use std::time::Instant;
use tunes::synthesis::additive::{AdditiveSynth, Partial};
use tunes::synthesis::karplus_strong::KarplusStrong;

const SAMPLE_RATE: f32 = 44100.0;
const BENCH_DURATION: f32 = 1.0; // 1 second
const ITERATIONS: usize = 100;

fn benchmark_additive_synthesis() {
    println!("\n=== Additive Synthesis Benchmark ===");

    // Test with different partial counts
    for num_partials in [4, 8, 16, 32] {
        let mut synth = AdditiveSynth::new(440.0, SAMPLE_RATE);

        // Add partials (harmonic series with 1/n amplitude)
        for i in 1..=num_partials {
            synth = synth.add_partial(Partial::harmonic(i, 1.0 / i as f32));
        }

        let samples = (BENCH_DURATION * SAMPLE_RATE) as usize;

        // Warmup
        synth.generate(samples);

        // Benchmark
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _ = synth.generate(samples);
        }
        let elapsed = start.elapsed();

        let avg_time_us = elapsed.as_micros() / ITERATIONS as u128;
        let samples_per_sec = (samples * ITERATIONS) as f64 / elapsed.as_secs_f64();
        let realtime_factor = samples_per_sec / SAMPLE_RATE as f64;

        println!("  {} partials:", num_partials);
        println!("    Average time: {} μs", avg_time_us);
        println!("    Throughput: {:.2} Msamples/sec", samples_per_sec / 1_000_000.0);
        println!("    Realtime factor: {:.1}x", realtime_factor);
    }
}

fn benchmark_karplus_strong() {
    println!("\n=== Karplus-Strong Synthesis Benchmark ===");

    // Test different frequencies (different buffer sizes)
    for freq in [110.0, 220.0, 440.0, 880.0] {
        let mut synth = KarplusStrong::new(freq, SAMPLE_RATE);
        synth.pluck();

        let samples = (BENCH_DURATION * SAMPLE_RATE) as usize;

        // Warmup
        let _ = synth.generate(samples);

        // Reset for benchmark
        synth = KarplusStrong::new(freq, SAMPLE_RATE);
        synth.pluck();

        // Benchmark
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _ = synth.generate(samples);
            // Reset for next iteration
            synth = KarplusStrong::new(freq, SAMPLE_RATE);
            synth.pluck();
        }
        let elapsed = start.elapsed();

        let avg_time_us = elapsed.as_micros() / ITERATIONS as u128;
        let samples_per_sec = (samples * ITERATIONS) as f64 / elapsed.as_secs_f64();
        let realtime_factor = samples_per_sec / SAMPLE_RATE as f64;

        println!("  {} Hz:", freq);
        println!("    Average time: {} μs", avg_time_us);
        println!("    Throughput: {:.2} Msamples/sec", samples_per_sec / 1_000_000.0);
        println!("    Realtime factor: {:.1}x", realtime_factor);
    }
}

fn main() {
    println!("🚀 SIMD Synthesis Performance Benchmark");
    println!("========================================");
    println!("Sample Rate: {} Hz", SAMPLE_RATE);
    println!("Duration: {} seconds", BENCH_DURATION);
    println!("Iterations: {}", ITERATIONS);

    benchmark_additive_synthesis();
    benchmark_karplus_strong();

    println!("\n✅ Benchmark complete!");
}
