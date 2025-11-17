// Test twiddle factor calculation
use std::f32::consts::PI;

fn compute_twiddle_factors(size: usize) -> Vec<f32> {
    let mut factors = Vec::with_capacity(size * 2);

    for k in 0..size {
        let angle = -2.0 * PI * k as f32 / size as f32;
        factors.push(angle.cos());  // Real part
        factors.push(angle.sin());  // Imaginary part
    }

    factors
}

fn main() {
    let twiddles = compute_twiddle_factors(8);

    println!("Twiddle factors for size 8:");
    for k in 0..8 {
        let re = twiddles[k * 2];
        let im = twiddles[k * 2 + 1];
        let angle = -2.0 * PI * k as f32 / 8.0;
        println!("  W_8^{} = {:.3} + {:.3}i  (angle = {:.3} rad)", k, re, im, angle);
    }

    println!("\nFor stage 0, index 0: stride=1, twiddle_index=0*1=0 → W_8^0 = {:.3} + {:.3}i", twiddles[0], twiddles[1]);
    println!("For stage 1, index 0: stride=2, twiddle_index=0*2=0 → W_8^0 = {:.3} + {:.3}i", twiddles[0], twiddles[1]);
    println!("For stage 1, index 1: stride=2, twiddle_index=1*2=2 → W_8^2 = {:.3} + {:.3}i", twiddles[4], twiddles[5]);
    println!("For stage 2, index 0: stride=4, twiddle_index=0*4=0 → W_8^0 = {:.3} + {:.3}i", twiddles[0], twiddles[1]);
    println!("For stage 2, index 1: stride=4, twiddle_index=1*4=4 → W_8^4 = {:.3} + {:.3}i", twiddles[8], twiddles[9]);
}
