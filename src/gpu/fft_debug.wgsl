// Debug version of FFT shader with output buffer

struct FftParams {
    size: u32,
    log2_size: u32,
    inverse: u32,
    _padding: u32,
}

struct ButterflyParams {
    stage: u32,
    _padding: array<u32, 3>,
}

struct DebugOutput {
    thread_id: u32,
    idx1: u32,
    idx2: u32,
    twiddle_re: f32,
    twiddle_im: f32,
    a_re: f32,
    a_im: f32,
    b_re: f32,
    b_im: f32,
    out1_re: f32,
    out1_im: f32,
    out2_re: f32,
    out2_im: f32,
}

@group(0) @binding(0)
var<storage, read> params: FftParams;

@group(0) @binding(1)
var<storage, read_write> data: array<f32>;

@group(0) @binding(2)
var<storage, read> twiddle_factors: array<f32>;

@group(1) @binding(0)
var<storage, read> butterfly_params: ButterflyParams;

@group(2) @binding(0)
var<storage, read_write> debug: array<DebugOutput>;

const PI: f32 = 3.14159265359;

fn complex_mul(a_re: f32, a_im: f32, b_re: f32, b_im: f32) -> vec2<f32> {
    let real = a_re * b_re - a_im * b_im;
    let imag = a_re * b_im + a_im * b_re;
    return vec2<f32>(real, imag);
}

fn get_twiddle(stage: u32, index: u32) -> vec2<f32> {
    let stride = 1u << (params.log2_size - stage - 1u);
    let twiddle_index = (index * stride) % params.size;

    let base_index = twiddle_index * 2u;
    let cos_val = twiddle_factors[base_index];
    let sin_val = twiddle_factors[base_index + 1u];

    if params.inverse != 0u {
        return vec2<f32>(cos_val, -sin_val);
    }

    return vec2<f32>(cos_val, sin_val);
}

@compute @workgroup_size(256)
fn fft_butterfly_debug(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let pair_index = global_id.x;
    let stage = butterfly_params.stage;

    let block_size = 1u << (stage + 1u);
    let half_block = block_size >> 1u;
    let num_pairs = params.size / block_size;

    if pair_index >= num_pairs * half_block {
        return;
    }

    let block = pair_index / half_block;
    let pos_in_half = pair_index % half_block;

    let idx1 = block * block_size + pos_in_half;
    let idx2 = idx1 + half_block;

    let base1 = idx1 * 2u;
    let base2 = idx2 * 2u;

    let a_re = data[base1];
    let a_im = data[base1 + 1u];
    let b_re = data[base2];
    let b_im = data[base2 + 1u];

    let twiddle = get_twiddle(stage, pos_in_half);
    let twiddle_b = complex_mul(twiddle.x, twiddle.y, b_re, b_im);

    let out1_re = a_re + twiddle_b.x;
    let out1_im = a_im + twiddle_b.y;
    let out2_re = a_re - twiddle_b.x;
    let out2_im = a_im - twiddle_b.y;

    // Write debug info
    debug[pair_index].thread_id = pair_index;
    debug[pair_index].idx1 = idx1;
    debug[pair_index].idx2 = idx2;
    debug[pair_index].twiddle_re = twiddle.x;
    debug[pair_index].twiddle_im = twiddle.y;
    debug[pair_index].a_re = a_re;
    debug[pair_index].a_im = a_im;
    debug[pair_index].b_re = b_re;
    debug[pair_index].b_im = b_im;
    debug[pair_index].out1_re = out1_re;
    debug[pair_index].out1_im = out1_im;
    debug[pair_index].out2_re = out2_re;
    debug[pair_index].out2_im = out2_im;

    // Store results
    data[base1] = out1_re;
    data[base1 + 1u] = out1_im;
    data[base2] = out2_re;
    data[base2 + 1u] = out2_im;
}
