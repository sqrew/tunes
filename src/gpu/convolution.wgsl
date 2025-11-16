// GPU Convolution Reverb Compute Shader (WGSL)
//
// Fast FFT-based convolution using Cooley-Tukey radix-2 algorithm
// Performs overlap-add convolution for real-time reverb

// Complex number operations
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

// Twiddle factor for FFT: e^(-2πi * k / N)
fn twiddle_factor(k: u32, n: u32) -> Complex {
    let angle = -6.28318530718 * f32(k) / f32(n);
    return Complex(cos(angle), sin(angle));
}

// Input: Audio block (real values, zero-padded to FFT size)
@group(0) @binding(0)
var<storage, read> input_buffer: array<f32>;

// Input: Pre-computed IR FFT (frequency domain, complex)
@group(0) @binding(1)
var<storage, read> ir_fft: array<Complex>;

// Input: Overlap buffer from previous block
@group(0) @binding(2)
var<storage, read> overlap_in: array<f32>;

// Output: Processed samples (time domain, real)
@group(0) @binding(3)
var<storage, read_write> output_buffer: array<f32>;

// Output: New overlap buffer for next block
@group(0) @binding(4)
var<storage, read_write> overlap_out: array<f32>;

// Shared memory for FFT (workgroup local)
var<workgroup> shared_data: array<Complex, 8192>;

// Parameters
struct ConvolutionParams {
    fft_size: u32,
    block_size: u32,
    _padding1: u32,
    _padding2: u32,
}

@group(0) @binding(5)
var<storage, read> params: ConvolutionParams;

// Radix-2 Cooley-Tukey FFT (in-place, decimation-in-time)
fn fft_radix2(data: ptr<workgroup, array<Complex, 8192>>, n: u32, inverse: bool) {
    let local_id = u32(workgroup_id.x);

    // Bit-reversal permutation
    for (var i = local_id; i < n; i += 256u) {
        var j = 0u;
        var k = i;
        var bits = u32(log2(f32(n)));

        for (var b = 0u; b < bits; b++) {
            j = (j << 1u) | (k & 1u);
            k = k >> 1u;
        }

        if j > i {
            let temp = (*data)[i];
            (*data)[i] = (*data)[j];
            (*data)[j] = temp;
        }
    }

    workgroupBarrier();

    // FFT butterfly operations
    var m = 2u;
    while m <= n {
        let half_m = m / 2u;

        for (var k = local_id; k < n; k += 256u) {
            let group_idx = k / m;
            let idx_in_group = k % m;

            if idx_in_group < half_m {
                let i = group_idx * m + idx_in_group;
                let j = i + half_m;

                var w = twiddle_factor(idx_in_group * (n / m), n);
                if inverse {
                    w.im = -w.im;
                }

                let t = complex_mul(w, (*data)[j]);
                (*data)[j] = complex_sub((*data)[i], t);
                (*data)[i] = complex_add((*data)[i], t);
            }
        }

        workgroupBarrier();
        m = m * 2u;
    }

    // Normalize for inverse FFT
    if inverse {
        let scale = 1.0 / f32(n);
        for (var i = local_id; i < n; i += 256u) {
            (*data)[i].re *= scale;
            (*data)[i].im *= scale;
        }
    }
}

// Main convolution shader
@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>,
        @builtin(local_invocation_id) local_id: vec3<u32>,
        @builtin(workgroup_id) workgroup_id: vec3<u32>) {

    let fft_size = params.fft_size;
    let block_size = params.block_size;
    let tid = local_id.x;

    // Load input into shared memory (convert real to complex)
    for (var i = tid; i < fft_size; i += 256u) {
        if i < block_size {
            shared_data[i] = Complex(input_buffer[i], 0.0);
        } else {
            shared_data[i] = Complex(0.0, 0.0);  // Zero padding
        }
    }

    workgroupBarrier();

    // FFT of input block
    fft_radix2(&shared_data, fft_size, false);

    workgroupBarrier();

    // Complex multiplication with IR FFT (convolution in frequency domain)
    for (var i = tid; i < fft_size; i += 256u) {
        shared_data[i] = complex_mul(shared_data[i], ir_fft[i]);
    }

    workgroupBarrier();

    // IFFT back to time domain
    fft_radix2(&shared_data, fft_size, true);

    workgroupBarrier();

    // Overlap-add and write output
    for (var i = tid; i < fft_size; i += 256u) {
        if i < block_size {
            // First block_size samples: add overlap and output
            output_buffer[i] = shared_data[i].re + overlap_in[i];

            // Store overlap for next block (second half of convolution result)
            overlap_out[i] = shared_data[i + block_size].re;
        } else if i < fft_size - block_size {
            // Remaining overlap samples
            overlap_out[i] = shared_data[i + block_size].re;
        }
    }
}
