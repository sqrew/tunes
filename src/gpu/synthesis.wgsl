// GPU Synthesis Compute Shader (WGSL)
//
// This shader synthesizes audio on the GPU at 500-1000x realtime speed.
// Each workgroup processes a chunk of samples in parallel.

// Note parameters (uploaded from CPU)
struct NoteParams {
    frequency: f32,           // Hz
    duration: f32,            // seconds
    sample_rate: f32,         // samples per second (usually 44100)
    waveform: u32,            // 0=Sine, 1=Saw, 2=Square, 3=Triangle

    // Envelope (ADSR)
    attack: f32,              // seconds
    decay: f32,               // seconds
    sustain: f32,             // 0.0 to 1.0
    release: f32,             // seconds

    // FM synthesis
    fm_enabled: u32,          // 0=off, 1=on
    fm_mod_ratio: f32,        // modulator frequency ratio
    fm_mod_index: f32,        // modulation depth

    velocity: f32,            // 0.0 to 1.0
    _padding: u32,            // Alignment padding
}

// Input: Note parameters
@group(0) @binding(0)
var<storage, read> params: NoteParams;

// Output: Generated samples
@group(0) @binding(1)
var<storage, read_write> output: array<f32>;

// Constants
const PI: f32 = 3.14159265359;
const TWO_PI: f32 = 6.28318530718;

// Waveform generators
fn sine_wave(phase: f32) -> f32 {
    return sin(phase * TWO_PI);
}

fn saw_wave(phase: f32) -> f32 {
    return 2.0 * (phase - floor(phase + 0.5));
}

fn square_wave(phase: f32) -> f32 {
    let frac = phase - floor(phase);
    return select(-1.0, 1.0, frac < 0.5);
}

fn triangle_wave(phase: f32) -> f32 {
    let frac = phase - floor(phase);
    return 4.0 * abs(frac - 0.5) - 1.0;
}

// Generate waveform sample at given phase
fn generate_waveform(waveform: u32, phase: f32) -> f32 {
    switch waveform {
        case 0u: { return sine_wave(phase); }
        case 1u: { return saw_wave(phase); }
        case 2u: { return square_wave(phase); }
        case 3u: { return triangle_wave(phase); }
        default: { return sine_wave(phase); }
    }
}

// ADSR envelope
fn envelope_amplitude(time: f32, note_duration: f32, attack: f32, decay: f32, sustain: f32, release: f32) -> f32 {
    let total_duration = attack + decay + note_duration + release;

    if time < attack {
        // Attack phase
        return time / attack;
    } else if time < attack + decay {
        // Decay phase
        let decay_time = time - attack;
        return 1.0 - (1.0 - sustain) * (decay_time / decay);
    } else if time < attack + decay + note_duration {
        // Sustain phase
        return sustain;
    } else if time < total_duration {
        // Release phase
        let release_time = time - (attack + decay + note_duration);
        return sustain * (1.0 - release_time / release);
    } else {
        // After release
        return 0.0;
    }
}

// FM synthesis
fn fm_synthesis(time: f32, carrier_freq: f32, mod_ratio: f32, mod_index: f32) -> f32 {
    let modulator_freq = carrier_freq * mod_ratio;
    let modulator = sine_wave(time * modulator_freq);
    let phase_modulation = modulator * mod_index;
    return sine_wave(time * carrier_freq + phase_modulation);
}

// Main compute shader
@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let sample_idx = global_id.x;
    let total_duration = params.attack + params.decay + params.duration + params.release;
    let total_samples = u32(total_duration * params.sample_rate);

    // Bounds check
    if sample_idx >= total_samples {
        return;
    }

    // Calculate time in seconds
    let time = f32(sample_idx) / params.sample_rate;

    // Generate oscillator output
    var oscillator_output: f32;

    if params.fm_enabled != 0u {
        // FM synthesis
        oscillator_output = fm_synthesis(
            time,
            params.frequency,
            params.fm_mod_ratio,
            params.fm_mod_index
        );
    } else {
        // Basic waveform
        let phase = time * params.frequency;
        oscillator_output = generate_waveform(params.waveform, phase);
    }

    // Apply envelope
    let envelope = envelope_amplitude(
        time,
        params.duration,
        params.attack,
        params.decay,
        params.sustain,
        params.release
    );

    // Apply velocity and write output
    output[sample_idx] = oscillator_output * envelope * params.velocity;
}
