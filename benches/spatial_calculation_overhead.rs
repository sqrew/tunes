use std::time::Instant;
use tunes::synthesis::spatial::*;

const ITERATIONS: usize = 1_000_000;

fn benchmark_spatial_basic() {
    println!("\n=== Basic Spatial Audio (Distance + Panning) ===");

    let listener = ListenerConfig::new();
    let params = SpatialParams::default();

    // Create 100 different source positions
    let sources: Vec<SpatialPosition> = (0..100)
        .map(|i| {
            let angle = (i as f32 * 2.0 * std::f32::consts::PI) / 100.0;
            let distance = 5.0 + (i as f32 % 10.0);
            SpatialPosition::new(
                distance * angle.cos(),
                (i as f32 % 5.0) - 2.0,
                distance * angle.sin(),
            )
        })
        .collect();

    let start = Instant::now();
    let mut _sum = 0.0;

    for _ in 0..ITERATIONS {
        for source in &sources {
            let result = calculate_spatial(source, &listener, &params);
            _sum += result.volume; // Prevent optimization
        }
    }

    let elapsed = start.elapsed();
    let calculations = ITERATIONS * sources.len();
    let ns_per_calc = elapsed.as_nanos() / calculations as u128;

    println!("  Total calculations: {}", calculations);
    println!("  Time per calculation: {} ns", ns_per_calc);
    println!("  Calculations per second: {:.2} M/s", calculations as f64 / elapsed.as_secs_f64() / 1_000_000.0);
}

fn benchmark_spatial_with_cones() {
    println!("\n=== Spatial Audio + Sound Cones ===");

    let listener = ListenerConfig::new();
    let params = SpatialParams::default();

    // Create 100 different source positions with cones
    let sources: Vec<(SpatialPosition, SoundCone)> = (0..100)
        .map(|i| {
            let angle = (i as f32 * 2.0 * std::f32::consts::PI) / 100.0;
            let distance = 5.0 + (i as f32 % 10.0);
            let pos = SpatialPosition::new(
                distance * angle.cos(),
                (i as f32 % 5.0) - 2.0,
                distance * angle.sin(),
            );
            let cone = SoundCone::medium().with_direction(angle.cos(), 0.0, angle.sin());
            (pos, cone)
        })
        .collect();

    let start = Instant::now();
    let mut _sum = 0.0;

    for _ in 0..ITERATIONS {
        for (source, cone) in &sources {
            let result = calculate_spatial_with_cone(source, &listener, &params, Some(cone), 0.0);
            _sum += result.volume; // Prevent optimization
        }
    }

    let elapsed = start.elapsed();
    let calculations = ITERATIONS * sources.len();
    let ns_per_calc = elapsed.as_nanos() / calculations as u128;

    println!("  Total calculations: {}", calculations);
    println!("  Time per calculation: {} ns", ns_per_calc);
    println!("  Calculations per second: {:.2} M/s", calculations as f64 / elapsed.as_secs_f64() / 1_000_000.0);
}

fn benchmark_spatial_with_cones_and_occlusion() {
    println!("\n=== Spatial Audio + Cones + Occlusion ===");

    let listener = ListenerConfig::new();
    let params = SpatialParams::default();

    // Create 100 different source positions with cones and occlusion
    let sources: Vec<(SpatialPosition, SoundCone, f32)> = (0..100)
        .map(|i| {
            let angle = (i as f32 * 2.0 * std::f32::consts::PI) / 100.0;
            let distance = 5.0 + (i as f32 % 10.0);
            let pos = SpatialPosition::new(
                distance * angle.cos(),
                (i as f32 % 5.0) - 2.0,
                distance * angle.sin(),
            );
            let cone = SoundCone::medium().with_direction(angle.cos(), 0.0, angle.sin());
            let occlusion = if i % 7 == 0 { 0.5 } else { 0.0 };
            (pos, cone, occlusion)
        })
        .collect();

    let start = Instant::now();
    let mut _sum = 0.0;

    for _ in 0..ITERATIONS {
        for (source, cone, occlusion) in &sources {
            let result = calculate_spatial_with_cone(source, &listener, &params, Some(cone), *occlusion);
            _sum += result.volume; // Prevent optimization
        }
    }

    let elapsed = start.elapsed();
    let calculations = ITERATIONS * sources.len();
    let ns_per_calc = elapsed.as_nanos() / calculations as u128;

    println!("  Total calculations: {}", calculations);
    println!("  Time per calculation: {} ns", ns_per_calc);
    println!("  Calculations per second: {:.2} M/s", calculations as f64 / elapsed.as_secs_f64() / 1_000_000.0);
}

fn estimate_realtime_capacity() {
    println!("\n=== Realtime Capacity Estimation ===");

    // Typical audio callback: 512 samples at 44.1kHz = ~86 callbacks/sec
    let callbacks_per_sec = 44100.0 / 512.0;
    println!("  Audio callbacks per second: {:.1}", callbacks_per_sec);

    // Budget per callback: 512 samples / 44100 Hz = ~11.6ms
    let budget_per_callback_ms = (512.0 / 44100.0) * 1000.0;
    println!("  Time budget per callback: {:.2} ms", budget_per_callback_ms);

    // Measure cost of 1000 spatial calculations (with cones + occlusion)
    let listener = ListenerConfig::new();
    let params = SpatialParams::default();

    let sources: Vec<(SpatialPosition, SoundCone, f32)> = (0..1000)
        .map(|i| {
            let angle = (i as f32 * 2.0 * std::f32::consts::PI) / 1000.0;
            let distance = 5.0 + (i as f32 % 10.0);
            let pos = SpatialPosition::new(
                distance * angle.cos(),
                (i as f32 % 5.0) - 2.0,
                distance * angle.sin(),
            );
            let cone = SoundCone::medium().with_direction(angle.cos(), 0.0, angle.sin());
            let occlusion = if i % 7 == 0 { 0.5 } else { 0.0 };
            (pos, cone, occlusion)
        })
        .collect();

    let start = Instant::now();
    let mut _sum = 0.0;

    for (source, cone, occlusion) in &sources {
        let result = calculate_spatial_with_cone(source, &listener, &params, Some(cone), *occlusion);
        _sum += result.volume;
    }

    let elapsed = start.elapsed();
    let time_ms = elapsed.as_secs_f64() * 1000.0;

    println!("\n  Time to calculate 1000 sounds: {:.3} ms", time_ms);
    println!("  % of audio callback budget: {:.1}%", (time_ms / budget_per_callback_ms) * 100.0);

    if time_ms < budget_per_callback_ms {
        println!("  ✅ Can handle 1000+ concurrent spatial sounds in realtime!");
    } else {
        let max_sounds = ((budget_per_callback_ms / time_ms) * 1000.0) as u32;
        println!("  ⚠️  Maximum concurrent sounds: ~{}", max_sounds);
    }
}

fn main() {
    println!("\n🎯 Spatial Audio Calculation Overhead Benchmark\n");
    println!("This benchmarks the raw cost of spatial audio calculations");
    println!("to determine if cone/occlusion features impact performance.\n");

    benchmark_spatial_basic();
    benchmark_spatial_with_cones();
    benchmark_spatial_with_cones_and_occlusion();
    estimate_realtime_capacity();

    println!("\n✅ Benchmark complete!\n");
}
