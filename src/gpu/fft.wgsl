// GPU FFT Compute Shader (WGSL)
//
// Implements Cooley-Tukey radix-2 FFT algorithm for spectral processing.
// Supports both forward and inverse FFT for sizes up to 4096 samples.
//
// Algorithm: Iterative radix-2 decimation-in-time (DIT) FFT
// Complexity: O(N log N) where N is FFT size
// Parallelization: Each stage processes N/2 butterfly operations in parallel

// FFT parameters
struct FftParams {
    size: u32,              // FFT size (must be power of 2: 256, 512, 1024, 2048, 4096)
    log2_size: u32,         // log2(size) for stage iteration
    inverse: u32,           // 0=forward FFT, 1=inverse FFT
    _padding: u32,          // Alignment
}

// Complex number representation
// Layout: [re0, im0, re1, im1, re2, im2, ...]
// We store real and imaginary parts interleaved for better memory access

@group(0) @binding(0)
var<storage, read> params: FftParams;

@group(0) @binding(1)
var<storage, read_write> data: array<f32>;  // Input/output buffer (interleaved complex)

@group(0) @binding(2)
var<storage, read> twiddle_factors: array<f32>;  // Pre-computed twiddle factors

// Constants
const PI: f32 = 3.14159265359;
const TWO_PI: f32 = 6.28318530718;

// Bit reversal permutation for DIT FFT
// Maps index i to bit-reversed index for given log2_size
fn bit_reverse(index: u32, log2_size: u32) -> u32 {
    var result: u32 = 0u;
    var temp = index;

    for (var i: u32 = 0u; i < log2_size; i = i + 1u) {
        result = (result << 1u) | (temp & 1u);
        temp = temp >> 1u;
    }

    return result;
}

// Complex multiplication: (a_re + i*a_im) * (b_re + i*b_im)
// Returns: (a_re*b_re - a_im*b_im, a_re*b_im + a_im*b_re)
fn complex_mul(a_re: f32, a_im: f32, b_re: f32, b_im: f32) -> vec2<f32> {
    let real = a_re * b_re - a_im * b_im;
    let imag = a_re * b_im + a_im * b_re;
    return vec2<f32>(real, imag);
}

// Get twiddle factor for stage and index
// Twiddle factor: W_N^(k * stride) where stride = 2^(log2_size - s - 1)
// W_N^m = e^(-2πim/N) = cos(2πm/N) - i*sin(2πm/N) for forward FFT
fn get_twiddle(stage: u32, index: u32) -> vec2<f32> {
    // For stage s, position k within half-block, twiddle is W_N^(k * 2^(log2_size - s - 1))
    // This gives us the index m = k * stride into our pre-computed twiddle factors

    let stride = 1u << (params.log2_size - stage - 1u);  // 2^(log2_size - s - 1)
    let twiddle_index = (index * stride) % params.size;

    let base_index = twiddle_index * 2u;
    let cos_val = twiddle_factors[base_index];
    let sin_val = twiddle_factors[base_index + 1u];

    // For inverse FFT, conjugate the twiddle factor (negate imaginary part)
    if params.inverse != 0u {
        return vec2<f32>(cos_val, -sin_val);
    }

    return vec2<f32>(cos_val, sin_val);
}

// Stage 1: Bit reversal permutation
// Each workgroup handles one swap
@compute @workgroup_size(256)
fn bit_reversal(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    if index >= params.size {
        return;
    }

    let reversed = bit_reverse(index, params.log2_size);

    // Only swap if index < reversed to avoid double-swapping
    if index < reversed {
        let idx1 = index * 2u;
        let idx2 = reversed * 2u;

        // Swap real parts
        let temp_re = data[idx1];
        data[idx1] = data[idx2];
        data[idx2] = temp_re;

        // Swap imaginary parts
        let temp_im = data[idx1 + 1u];
        data[idx1 + 1u] = data[idx2 + 1u];
        data[idx2 + 1u] = temp_im;
    }
}

// Stage 2: FFT butterfly operations
// Each stage processes pairs of elements (butterfly computation)
// This kernel is called log2(N) times with different stage parameters

struct ButterflyParams {
    stage: u32,             // Current FFT stage (0 to log2_size-1)
    _padding: array<u32, 3>,
}

@group(1) @binding(0)
var<storage, read> butterfly_params: ButterflyParams;

@compute @workgroup_size(256)
fn fft_butterfly(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let pair_index = global_id.x;
    let stage = butterfly_params.stage;

    // Size of FFT blocks at this stage
    let block_size = 1u << (stage + 1u);
    let half_block = block_size >> 1u;

    // Total number of pairs to process at this stage
    let num_pairs = params.size / block_size;

    if pair_index >= num_pairs * half_block {
        return;
    }

    // Determine which block and position within block
    let block = pair_index / half_block;
    let pos_in_half = pair_index % half_block;

    // Indices of the two elements in the butterfly
    let idx1 = block * block_size + pos_in_half;
    let idx2 = idx1 + half_block;

    // Get complex values
    let base1 = idx1 * 2u;
    let base2 = idx2 * 2u;

    let a_re = data[base1];
    let a_im = data[base1 + 1u];
    let b_re = data[base2];
    let b_im = data[base2 + 1u];

    // Get twiddle factor
    let twiddle = get_twiddle(stage, pos_in_half);

    // Butterfly computation:
    // output[idx1] = a + twiddle * b
    // output[idx2] = a - twiddle * b

    let twiddle_b = complex_mul(twiddle.x, twiddle.y, b_re, b_im);

    // Store results
    data[base1] = a_re + twiddle_b.x;
    data[base1 + 1u] = a_im + twiddle_b.y;
    data[base2] = a_re - twiddle_b.x;
    data[base2 + 1u] = a_im - twiddle_b.y;
}

// Stage 3: Normalization (for inverse FFT)
// Divide all values by N
@compute @workgroup_size(256)
fn normalize(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    if index >= params.size {
        return;
    }

    if params.inverse != 0u {
        let scale = 1.0 / f32(params.size);
        let base = index * 2u;
        data[base] *= scale;
        data[base + 1u] *= scale;
    }
}
