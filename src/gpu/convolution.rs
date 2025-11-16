//! GPU-accelerated convolution reverb using compute shaders

use super::device::GpuDevice;
use anyhow::{Context, Result};
use rustfft::num_complex::Complex;
use wgpu::util::DeviceExt;

/// GPU-accelerated convolution processor
pub struct GpuConvolution {
    device: GpuDevice,
    compute_pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,

    // All IR partition FFTs concatenated into single buffer (GPU-resident)
    ir_partitions_buffer: wgpu::Buffer,

    // Processing parameters
    num_partitions: usize,  // Number of IR partitions
    partition_size: usize,  // Size of each IR partition
    fft_size: usize,        // FFT size for each partition
    block_size: usize,

    // Delay lines for partitioned convolution
    // Stores tail from each partition to add to next block
    partition_delays: Vec<Vec<f32>>,
}

/// Partitioned convolution parameters (matches WGSL struct layout)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct PartitionedConvParams {
    partition_size: u32,
    fft_size: u32,
    num_partitions: u32,
    block_size: u32,
}

unsafe impl bytemuck::Pod for PartitionedConvParams {}
unsafe impl bytemuck::Zeroable for PartitionedConvParams {}

/// Complex number for GPU (matches WGSL struct layout)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct GpuComplex {
    re: f32,
    im: f32,
}

unsafe impl bytemuck::Pod for GpuComplex {}
unsafe impl bytemuck::Zeroable for GpuComplex {}

impl GpuConvolution {
    /// Create a new GPU convolution processor with partitioned convolution
    ///
    /// Uses uniform partitioned convolution to handle arbitrarily long IRs.
    /// Splits IR into 4096-sample partitions, processes all partitions in parallel on GPU.
    ///
    /// # Arguments
    /// * `device` - GPU device
    /// * `ir_fft` - Pre-computed impulse response in frequency domain (will be re-partitioned)
    /// * `original_fft_size` - Original FFT size (ignored, kept for API compatibility)
    /// * `block_size` - Processing block size
    pub fn new(
        device: GpuDevice,
        ir_fft: &[Complex<f32>],
        original_fft_size: usize,
        block_size: usize,
    ) -> Result<Self> {
        use rustfft::FftPlanner;

        // Partition parameters
        const PARTITION_SIZE: usize = 4096;  // Fixed partition size for GPU
        let partition_fft_size = PARTITION_SIZE * 2;   // Need 2x for linear convolution

        // Step 1: Convert IR from frequency domain back to time domain
        let mut ir_time = ir_fft.to_vec();
        let mut planner = FftPlanner::new();
        let ifft = planner.plan_fft_inverse(original_fft_size);
        ifft.process(&mut ir_time);

        // Normalize and extract real part
        let scale = 1.0 / (original_fft_size as f32);
        let ir_samples: Vec<f32> = ir_time.iter().map(|c| c.re * scale).collect();

        // Step 2: Split IR into partitions and concatenate FFTs
        let num_partitions = (ir_samples.len() + PARTITION_SIZE - 1) / PARTITION_SIZE;
        let mut partition_delays: Vec<Vec<f32>> = Vec::new();
        let mut all_partition_ffts: Vec<GpuComplex> = Vec::new();

        println!("📦 Partitioning IR: {} samples -> {} partitions of {} samples",
                 ir_samples.len(), num_partitions, PARTITION_SIZE);

        let fft = planner.plan_fft_forward(partition_fft_size);

        for partition_idx in 0..num_partitions {
            // Extract this partition (zero-pad if needed)
            let start = partition_idx * PARTITION_SIZE;
            let end = (start + PARTITION_SIZE).min(ir_samples.len());

            let mut partition_samples = vec![Complex::new(0.0, 0.0); partition_fft_size];
            for (i, &sample) in ir_samples[start..end].iter().enumerate() {
                partition_samples[i] = Complex::new(sample, 0.0);
            }

            // FFT this partition
            fft.process(&mut partition_samples);

            // Convert to GPU format and add to concatenated buffer
            let partition_fft_gpu: Vec<GpuComplex> = partition_samples
                .iter()
                .map(|c| GpuComplex { re: c.re, im: c.im })
                .collect();

            all_partition_ffts.extend(partition_fft_gpu);

            // Initialize delay line for this partition (size = partition_fft_size for safety)
            partition_delays.push(vec![0.0; partition_fft_size]);
        }

        // Upload all partitions as single concatenated buffer
        let ir_partitions_buffer = device
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("All IR Partitions FFT"),
                contents: bytemuck::cast_slice(&all_partition_ffts),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

        // Load partitioned convolution shader
        let shader_source = include_str!("convolution_partitioned.wgsl");
        let shader = device
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Convolution Shader"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        // Create bind group layout for partitioned convolution
        let bind_group_layout =
            device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Partitioned Convolution Bind Group Layout"),
                    entries: &[
                        // @binding(0): Parameters
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
                        // @binding(1): Input buffer (audio block)
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // @binding(2): IR partition FFTs (all partitions concatenated)
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        // @binding(3): Partition outputs (all partitions)
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
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

        // Create pipeline
        let pipeline_layout =
            device
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Convolution Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let compute_pipeline =
            device
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("Convolution Pipeline"),
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
            ir_partitions_buffer,
            num_partitions,
            partition_size: PARTITION_SIZE,
            fft_size: partition_fft_size,
            block_size,
            partition_delays,
        })
    }

    /// Process an audio block through partitioned GPU convolution
    ///
    /// # Arguments
    /// * `input_block` - Input audio samples (length = block_size)
    /// * `_overlap_in` - Unused (kept for API compatibility)
    ///
    /// # Returns
    /// Tuple of (output_samples, empty_overlap_buffer)
    pub fn process_block(
        &mut self,
        input_block: &[f32],
        _overlap_in: &[f32],
    ) -> Result<(Vec<f32>, Vec<f32>)> {
        assert_eq!(
            input_block.len(),
            self.block_size,
            "Input block size mismatch"
        );

        // Create parameters for partitioned convolution
        let params = PartitionedConvParams {
            partition_size: self.partition_size as u32,
            fft_size: self.fft_size as u32,
            num_partitions: self.num_partitions as u32,
            block_size: self.block_size as u32,
        };

        let params_buffer =
            self.device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Partitioned Conv Params"),
                    contents: bytemuck::cast_slice(&[params]),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Create input buffer
        let input_buffer =
            self.device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Input Block"),
                    contents: bytemuck::cast_slice(input_block),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Create output buffer (all partitions write here)
        let total_output_size = self.num_partitions * self.fft_size;
        let partition_outputs_buffer = self.device.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Partition Outputs"),
            size: (total_output_size * std::mem::size_of::<f32>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Create bind group (using pre-concatenated IR partitions buffer)
        let bind_group = self.device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Partitioned Conv Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: input_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.ir_partitions_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: partition_outputs_buffer.as_entire_binding(),
                },
            ],
        });

        // Encode and submit GPU commands
        let mut encoder = self
            .device
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Partitioned Conv Encoder"),
            });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Partitioned Conv Pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);

            // Dispatch N workgroups (one per partition) - ALL IN PARALLEL!
            compute_pass.dispatch_workgroups(self.num_partitions as u32, 1, 1);
        }

        // Copy results to staging buffer
        let staging_outputs = self.device.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Partition Outputs"),
            size: (total_output_size * std::mem::size_of::<f32>()) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_buffer_to_buffer(
            &partition_outputs_buffer,
            0,
            &staging_outputs,
            0,
            (total_output_size * std::mem::size_of::<f32>()) as u64,
        );

        self.device.queue.submit(Some(encoder.finish()));

        // Read back all partition outputs
        let all_outputs = self.read_buffer_sync(&staging_outputs, total_output_size)?;

        // Combine partition outputs with delay compensation (CPU side)
        let mut output = vec![0.0f32; self.block_size];

        for partition_idx in 0..self.num_partitions {
            let partition_start = partition_idx * self.fft_size;
            let partition_output = &all_outputs[partition_start..partition_start + self.fft_size];

            // Each partition is delayed by partition_idx * partition_size samples
            let delay_samples = partition_idx * self.partition_size;

            // Add partition output to delay line
            for (i, &sample) in partition_output.iter().enumerate() {
                self.partition_delays[partition_idx][i] += sample;
            }

            // Output first block_size samples from this partition's delay line
            let samples_to_output = self.block_size.min(self.partition_delays[partition_idx].len());
            for i in 0..samples_to_output {
                if delay_samples == 0 || i >= delay_samples {
                    output[i] += self.partition_delays[partition_idx][i];
                }
            }

            // Shift delay line (move tail samples forward)
            self.partition_delays[partition_idx].rotate_left(self.block_size);

            // Zero out the end (samples we just shifted out)
            let tail_start = self.fft_size - self.block_size;
            for i in tail_start..self.fft_size {
                self.partition_delays[partition_idx][i] = 0.0;
            }
        }

        // Return output and empty overlap buffer (unused in partitioned convolution)
        Ok((output, vec![0.0; self.fft_size]))
    }

    /// Read complex buffer synchronously from GPU to CPU
    fn read_buffer_sync_complex(&self, buffer: &wgpu::Buffer, _size: usize) -> Result<Vec<GpuComplex>> {
        let buffer_slice = buffer.slice(..);

        // Map the buffer
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            sender.send(result).ok();
        });

        self.device.device.poll(wgpu::Maintain::Wait);

        pollster::block_on(async {
            receiver
                .receive()
                .await
                .context("Failed to map buffer")?
                .context("Buffer mapping failed")?;
            Ok::<(), anyhow::Error>(())
        })?;

        // Read data
        let data = buffer_slice.get_mapped_range();
        let result: Vec<GpuComplex> = bytemuck::cast_slice(&data).to_vec();

        drop(data);
        buffer.unmap();

        Ok(result)
    }

    /// Read buffer synchronously from GPU to CPU
    fn read_buffer_sync(&self, buffer: &wgpu::Buffer, _size: usize) -> Result<Vec<f32>> {
        let buffer_slice = buffer.slice(..);

        // Map the buffer
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            sender.send(result).ok();
        });

        self.device.device.poll(wgpu::Maintain::Wait);

        pollster::block_on(async {
            receiver
                .receive()
                .await
                .context("Failed to map buffer")?
                .context("Buffer mapping failed")?;
            Ok::<(), anyhow::Error>(())
        })?;

        // Read data
        let data = buffer_slice.get_mapped_range();
        let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();

        drop(data);
        buffer.unmap();

        Ok(result)
    }
}
