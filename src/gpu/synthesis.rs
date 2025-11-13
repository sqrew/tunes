//! GPU-accelerated synthesis using compute shaders

use super::device::GpuDevice;
use crate::track::NoteEvent;
use crate::synthesis::waveform::Waveform;
use anyhow::{Context, Result};
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// GPU synthesizer for accelerated audio generation
pub struct GpuSynthesizer {
    device: GpuDevice,
    compute_pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl std::fmt::Debug for GpuSynthesizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GpuSynthesizer")
            .field("device", &self.device)
            .finish()
    }
}

/// Note parameters for GPU (matches WGSL struct layout)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct GpuNoteParams {
    frequency: f32,
    duration: f32,
    sample_rate: f32,
    waveform: u32,

    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,

    fm_enabled: u32,
    fm_mod_ratio: f32,
    fm_mod_index: f32,

    velocity: f32,
    _padding: u32,
}

// Ensure proper alignment for GPU
unsafe impl bytemuck::Pod for GpuNoteParams {}
unsafe impl bytemuck::Zeroable for GpuNoteParams {}

impl GpuSynthesizer {
    /// Create a new GPU synthesizer
    pub fn new(device: GpuDevice) -> Result<Self> {
        // Load shader
        let shader_source = include_str!("synthesis.wgsl");
        let shader = device.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Synthesis Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create bind group layout
        let bind_group_layout = device.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Synthesis Bind Group Layout"),
            entries: &[
                // Input: Note parameters
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Output: Sample buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Create compute pipeline
        let pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Synthesis Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Synthesis Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            device,
            compute_pipeline,
            bind_group_layout,
        })
    }

    /// Synthesize a note on the GPU
    ///
    /// Returns the rendered audio samples (mono, f32)
    pub fn synthesize_note(&self, note: &NoteEvent, sample_rate: f32) -> Result<Vec<f32>> {
        // Convert NoteEvent to GPU parameters
        let gpu_params = self.note_to_gpu_params(note, sample_rate);

        // Calculate output size
        let total_duration = note.envelope.total_duration(note.duration);
        let total_samples = (total_duration * sample_rate) as usize;

        // Create GPU buffers
        let params_buffer = self.device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Note Params Buffer"),
            contents: bytemuck::cast_slice(&[gpu_params]),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let output_buffer = self.device.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: (total_samples * std::mem::size_of::<f32>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Create staging buffer for readback
        let staging_buffer = self.device.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: (total_samples * std::mem::size_of::<f32>()) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group
        let bind_group = self.device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Synthesis Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output_buffer.as_entire_binding(),
                },
            ],
        });

        // Create command encoder
        let mut encoder = self.device.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Synthesis Encoder"),
        });

        // Dispatch compute shader
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Synthesis Compute Pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);

            // Dispatch with workgroup size 256
            let workgroups = (total_samples as u32 + 255) / 256;
            compute_pass.dispatch_workgroups(workgroups, 1, 1);
        }

        // Copy output to staging buffer
        encoder.copy_buffer_to_buffer(
            &output_buffer,
            0,
            &staging_buffer,
            0,
            (total_samples * std::mem::size_of::<f32>()) as u64,
        );

        // Submit commands
        self.device.queue.submit(Some(encoder.finish()));

        // Read back results
        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();

        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            sender.send(result).ok();
        });

        // Poll device until mapping is complete
        self.device.device.poll(wgpu::Maintain::Wait);

        // Wait for mapping
        pollster::block_on(async {
            receiver.receive().await
        }).context("Failed to map buffer")?
            .context("Buffer mapping failed")?;

        // Copy data to Vec
        let data = buffer_slice.get_mapped_range();
        let samples: Vec<f32> = bytemuck::cast_slice(&data).to_vec();

        drop(data);
        staging_buffer.unmap();

        Ok(samples)
    }

    /// Convert NoteEvent to GPU-friendly parameters
    fn note_to_gpu_params(&self, note: &NoteEvent, sample_rate: f32) -> GpuNoteParams {
        let waveform_id = match note.waveform {
            Waveform::Sine => 0,
            Waveform::Sawtooth => 1,
            Waveform::Square => 2,
            Waveform::Triangle => 3,
        };

        let fm_enabled = if note.fm_params.mod_index > 0.0 { 1 } else { 0 };

        GpuNoteParams {
            frequency: note.frequencies[0], // Use first frequency
            duration: note.duration,
            sample_rate,
            waveform: waveform_id,

            attack: note.envelope.attack,
            decay: note.envelope.decay,
            sustain: note.envelope.sustain,
            release: note.envelope.release,

            fm_enabled,
            fm_mod_ratio: note.fm_params.mod_ratio,
            fm_mod_index: note.fm_params.mod_index,

            velocity: note.velocity,
            _padding: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::synthesis::envelope::Envelope;
    use crate::synthesis::fm_synthesis::FMParams;

    #[test]
    fn test_gpu_synthesis() {
        // Try to initialize GPU
        let device = match GpuDevice::new() {
            Ok(d) => d,
            Err(_) => {
                println!("GPU not available, skipping test");
                return;
            }
        };

        let synthesizer = GpuSynthesizer::new(device).expect("Failed to create synthesizer");

        // Create a simple note
        let note = NoteEvent {
            frequencies: [440.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            num_freqs: 1,
            start_time: 0.0,
            duration: 0.5,
            waveform: Waveform::Sine,
            envelope: Envelope::default(),
            filter_envelope: Default::default(),
            fm_params: FMParams::default(),
            pitch_bend_semitones: 0.0,
            custom_wavetable: None,
            velocity: 1.0,
            spatial_position: None,
        };

        // Synthesize on GPU
        let samples = synthesizer
            .synthesize_note(&note, 44100.0)
            .expect("GPU synthesis failed");

        // Verify output
        assert!(!samples.is_empty());
        assert!(samples.len() > 1000); // Should have generated samples

        println!("✅ GPU synthesized {} samples", samples.len());
    }
}
