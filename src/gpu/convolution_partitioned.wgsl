// GPU Partitioned Convolution Reverb (WGSL)
//
// Uniform partitioned convolution for handling long impulse responses.
// Splits IR into fixed-size partitions and processes them in parallel.
//
// Algorithm:
// 1. IR split into N partitions of 4096 samples each
// 2. Each partition processed in parallel (one workgroup per partition)
// 3. Results combined with delay compensation
// 4. Overlap-add for seamless output

struct Complex {
    re: f32,
    im: f32,
}

fn complex_mul(a: Complex, b: Complex) -> Complex {
    return Complex(
        a.re * b.re - a.im * b.im,
        a.re * b.im + a.im * b.re
    );
}

fn complex_add(a: Complex, b: Complex) -> Complex {
    return Complex(a.re + b.re, a.im + b.im);
}

fn complex_sub(a: Complex, b: Complex) -> Complex {
    return Complex(a.re - b.re, a.im - b.im);
}

// Twiddle factor: e^(-2πi * k / N)
fn twiddle_factor(k: u32, n: u32, inverse: bool) -> Complex {
    var angle = -6.28318530718 * f32(k) / f32(n);
    if inverse {
        angle = -angle;
    }
    return Complex(cos(angle), sin(angle));
}

// Bit-reverse permutation
fn bit_reverse(x_in: u32, bits: u32) -> u32 {
    var x = x_in;
    var result = 0u;
    for (var i = 0u; i < bits; i++) {
        result = (result << 1u) | (x & 1u);
        x = x >> 1u;
    }
    return result;
}

// Parameters
struct PartitionedConvParams {
    partition_size: u32,      // Size of each partition (e.g., 4096)
    fft_size: u32,            // FFT size (partition_size * 2)
    num_partitions: u32,      // Total number of IR partitions
    block_size: u32,          // Input block size
}

@group(0) @binding(0)
var<storage, read> params: PartitionedConvParams;

// Input audio block
@group(0) @binding(1)
var<storage, read> input_buffer: array<f32>;

// All IR partition FFTs (concatenated)
// Format: [partition0_fft, partition1_fft, ...]
@group(0) @binding(2)
var<storage, read> ir_partition_ffts: array<Complex>;

// Output buffers for each partition (concatenated)
// Each partition writes fft_size samples
@group(0) @binding(3)
var<storage, read_write> partition_outputs: array<f32>;

// Shared memory for FFT (one partition at a time)
var<workgroup> shared_fft: array<Complex, 8192>;

// Main shader: Process one partition
@compute @workgroup_size(256)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(workgroup_id) workgroup_id: vec3<u32>
) {
    let partition_idx = workgroup_id.x;
    let tid = local_id.x;

    // Safety check
    if partition_idx >= params.num_partitions {
        return;
    }

    let fft_size = params.fft_size;
    let num_bits = u32(log2(f32(fft_size)));

    // Load input block into shared memory (zero-padded)
    for (var i = tid; i < fft_size; i += 256u) {
        if i < params.block_size {
            shared_fft[i] = Complex(input_buffer[i], 0.0);
        } else {
            shared_fft[i] = Complex(0.0, 0.0);
        }
    }

    workgroupBarrier();

    // === FFT: Bit-reversal permutation ===
    for (var i = tid; i < fft_size; i += 256u) {
        let j = bit_reverse(i, num_bits);
        if j > i {
            let temp = shared_fft[i];
            shared_fft[i] = shared_fft[j];
            shared_fft[j] = temp;
        }
    }

    workgroupBarrier();

    // === FFT: Cooley-Tukey stages ===
    var m = 2u;
    while m <= fft_size {
        let half_m = m / 2u;

        for (var k = tid; k < fft_size; k += 256u) {
            let group_idx = k / m;
            let idx_in_group = k % m;

            if idx_in_group < half_m {
                let i = group_idx * m + idx_in_group;
                let j = i + half_m;

                let w = twiddle_factor(idx_in_group * (fft_size / m), fft_size, false);
                let t = complex_mul(w, shared_fft[j]);

                shared_fft[j] = complex_sub(shared_fft[i], t);
                shared_fft[i] = complex_add(shared_fft[i], t);
            }
        }

        workgroupBarrier();
        m = m * 2u;
    }

    // === Multiply with this partition's IR FFT ===
    let ir_offset = partition_idx * fft_size;
    for (var i = tid; i < fft_size; i += 256u) {
        shared_fft[i] = complex_mul(shared_fft[i], ir_partition_ffts[ir_offset + i]);
    }

    workgroupBarrier();

    // === IFFT: Bit-reversal permutation ===
    for (var i = tid; i < fft_size; i += 256u) {
        let j = bit_reverse(i, num_bits);
        if j > i {
            let temp = shared_fft[i];
            shared_fft[i] = shared_fft[j];
            shared_fft[j] = temp;
        }
    }

    workgroupBarrier();

    // === IFFT: Cooley-Tukey stages (inverse) ===
    m = 2u;
    while m <= fft_size {
        let half_m = m / 2u;

        for (var k = tid; k < fft_size; k += 256u) {
            let group_idx = k / m;
            let idx_in_group = k % m;

            if idx_in_group < half_m {
                let i = group_idx * m + idx_in_group;
                let j = i + half_m;

                let w = twiddle_factor(idx_in_group * (fft_size / m), fft_size, true);
                let t = complex_mul(w, shared_fft[j]);

                shared_fft[j] = complex_sub(shared_fft[i], t);
                shared_fft[i] = complex_add(shared_fft[i], t);
            }
        }

        workgroupBarrier();
        m = m * 2u;
    }

    // === IFFT: Normalize ===
    let scale = 1.0 / f32(fft_size);
    for (var i = tid; i < fft_size; i += 256u) {
        shared_fft[i].re *= scale;
        shared_fft[i].im *= scale;
    }

    workgroupBarrier();

    // === Write this partition's output ===
    let output_offset = partition_idx * fft_size;
    for (var i = tid; i < fft_size; i += 256u) {
        partition_outputs[output_offset + i] = shared_fft[i].re;
    }
}
