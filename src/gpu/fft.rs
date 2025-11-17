// ! GPU-accelerated FFT using compute shaders
//!
//! Implements Cooley-Tukey radix-2 FFT on GPU for spectral processing.
//! Provides forward and inverse FFT for sizes 256-4096 (powers of 2).

use super::device::GpuDevice;
use anyhow::{Context, Result};
use rustfft::num_complex::Complex;
use std::f32::consts::PI;
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// GPU FFT processor
///
/// Manages GPU resources and provides FFT operations.
/// Supports both forward and inverse FFT.
pub struct GpuFft {
    device: Arc<GpuDevice>,
    fft_size: usize,
    log2_size: u32,

    // GPU resources
    data_buffer: wgpu::Buffer,
    params_buffer: wgpu::Buffer,
    #[allow(dead_code)] // Used via bind group, never directly accessed
    twiddle_buffer: wgpu::Buffer,
    butterfly_params_buffer: wgpu::Buffer,

    // Pipelines
    bit_reversal_pipeline: wgpu::ComputePipeline,
    butterfly_pipeline: wgpu::ComputePipeline,
    normalize_pipeline: wgpu::ComputePipeline,

    // Bind groups
    main_bind_group: wgpu::BindGroup,
    butterfly_bind_groups: Vec<wgpu::BindGroup>, // One per stage
}

impl GpuFft {
    /// Create a new GPU FFT processor
    ///
    /// # Arguments
    /// * `device` - GPU device
    /// * `fft_size` - FFT size (must be power of 2: 256, 512, 1024, 2048, 4096)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::gpu::{GpuDevice, GpuFft};
    /// # use std::sync::Arc;
    /// let device = GpuDevice::new().unwrap();
    /// let fft = GpuFft::new(Arc::new(device), 2048).unwrap();
    /// ```
    pub fn new(device: Arc<GpuDevice>, fft_size: usize) -> Result<Self> {
        // Validate FFT size
        if !fft_size.is_power_of_two() {
            anyhow::bail!("FFT size must be power of 2, got {}", fft_size);
        }

        if !(8..=4096).contains(&fft_size) {
            anyhow::bail!("FFT size must be between 8 and 4096, got {}", fft_size);
        }

        let log2_size = (fft_size as f32).log2() as u32;

        // Load shader
        let shader = device
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("FFT Compute Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("fft.wgsl").into()),
            });

        // Create buffers
        let data_buffer = device.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("FFT Data Buffer"),
            size: (fft_size * 2 * std::mem::size_of::<f32>()) as u64, // Complex: 2 floats per sample
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Pre-compute twiddle factors on CPU
        let twiddle_factors = Self::compute_twiddle_factors(fft_size);
        let twiddle_buffer = device
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("FFT Twiddle Factors"),
                contents: bytemuck::cast_slice(&twiddle_factors),
                usage: wgpu::BufferUsages::STORAGE,
            });

        // Parameters buffer
        let params_data = [
            fft_size as u32,
            log2_size,
            0u32, // inverse flag (will be set per-call)
            0u32, // padding
        ];

        let params_buffer = device
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("FFT Parameters"),
                contents: bytemuck::cast_slice(&params_data),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

        // Butterfly parameters buffer (stage index)
        let butterfly_params_buffer = device.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("FFT Butterfly Parameters"),
            size: 16, // 4 u32s (stage + padding)
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layouts
        let main_bind_group_layout =
            device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("FFT Main Bind Group Layout"),
                    entries: &[
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
                    ],
                });

        let butterfly_bind_group_layout =
            device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("FFT Butterfly Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        // Create main bind group
        let main_bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("FFT Main Bind Group"),
            layout: &main_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: data_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: twiddle_buffer.as_entire_binding(),
                },
            ],
        });

        // Create butterfly bind groups (one per stage)
        let mut butterfly_bind_groups = Vec::new();
        for _ in 0..log2_size {
            let bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("FFT Butterfly Bind Group"),
                layout: &butterfly_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: butterfly_params_buffer.as_entire_binding(),
                }],
            });
            butterfly_bind_groups.push(bind_group);
        }

        // Create pipeline layouts
        let main_pipeline_layout =
            device
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("FFT Main Pipeline Layout"),
                    bind_group_layouts: &[&main_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let butterfly_pipeline_layout =
            device
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("FFT Butterfly Pipeline Layout"),
                    bind_group_layouts: &[&main_bind_group_layout, &butterfly_bind_group_layout],
                    push_constant_ranges: &[],
                });

        // Create compute pipelines
        let bit_reversal_pipeline =
            device
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("FFT Bit Reversal Pipeline"),
                    layout: Some(&main_pipeline_layout),
                    module: &shader,
                    entry_point: Some("bit_reversal"),
                    compilation_options: Default::default(),
                    cache: None,
                });

        let butterfly_pipeline =
            device
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("FFT Butterfly Pipeline"),
                    layout: Some(&butterfly_pipeline_layout),
                    module: &shader,
                    entry_point: Some("fft_butterfly"),
                    compilation_options: Default::default(),
                    cache: None,
                });

        let normalize_pipeline =
            device
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("FFT Normalize Pipeline"),
                    layout: Some(&main_pipeline_layout),
                    module: &shader,
                    entry_point: Some("normalize"),
                    compilation_options: Default::default(),
                    cache: None,
                });

        Ok(Self {
            device,
            fft_size,
            log2_size,
            data_buffer,
            params_buffer,
            twiddle_buffer,
            butterfly_params_buffer,
            bit_reversal_pipeline,
            butterfly_pipeline,
            normalize_pipeline,
            main_bind_group,
            butterfly_bind_groups,
        })
    }

    /// Pre-compute twiddle factors for FFT
    ///
    /// Twiddle factor: W_N^k = e^(-2πik/N) = cos(2πk/N) - i*sin(2πk/N)
    /// Stored as [cos0, sin0, cos1, sin1, ...]
    fn compute_twiddle_factors(size: usize) -> Vec<f32> {
        let mut factors = Vec::with_capacity(size * 2);

        for k in 0..size {
            let angle = -2.0 * PI * k as f32 / size as f32;
            factors.push(angle.cos()); // Real part
            factors.push(angle.sin()); // Imaginary part
        }

        factors
    }

    /// Perform forward FFT
    ///
    /// # Arguments
    /// * `data` - Input/output complex data (modified in-place)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::gpu::{GpuDevice, GpuFft};
    /// # use rustfft::num_complex::Complex;
    /// # use std::sync::Arc;
    /// # let device = GpuDevice::new().unwrap();
    /// # let mut fft = GpuFft::new(Arc::new(device), 2048).unwrap();
    /// let mut data = vec![Complex::new(0.0, 0.0); 2048];
    /// // ... fill with input data ...
    /// fft.forward(&mut data).unwrap();
    /// // data now contains frequency domain representation
    /// ```
    pub fn forward(&mut self, data: &mut [Complex<f32>]) -> Result<()> {
        self.execute_fft(data, false)
    }

    /// Perform inverse FFT
    ///
    /// # Arguments
    /// * `data` - Input/output complex data (modified in-place)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::gpu::{GpuDevice, GpuFft};
    /// # use rustfft::num_complex::Complex;
    /// # use std::sync::Arc;
    /// # let device = GpuDevice::new().unwrap();
    /// # let mut fft = GpuFft::new(Arc::new(device), 2048).unwrap();
    /// let mut spectrum = vec![Complex::new(0.0, 0.0); 2048];
    /// // ... fill with frequency data ...
    /// fft.inverse(&mut spectrum).unwrap();
    /// // spectrum now contains time domain representation
    /// ```
    pub fn inverse(&mut self, data: &mut [Complex<f32>]) -> Result<()> {
        self.execute_fft(data, true)
    }

    /// Execute FFT (forward or inverse)
    fn execute_fft(&mut self, data: &mut [Complex<f32>], inverse: bool) -> Result<()> {
        if data.len() != self.fft_size {
            anyhow::bail!(
                "Data length {} doesn't match FFT size {}",
                data.len(),
                self.fft_size
            );
        }

        // Convert Complex<f32> to interleaved f32 [re0, im0, re1, im1, ...]
        let mut interleaved = Vec::with_capacity(self.fft_size * 2);
        for c in data.iter() {
            interleaved.push(c.re);
            interleaved.push(c.im);
        }

        // Upload data to GPU
        self.device
            .queue
            .write_buffer(&self.data_buffer, 0, bytemuck::cast_slice(&interleaved));

        // Update params buffer with inverse flag
        let params_data = [
            self.fft_size as u32,
            self.log2_size,
            if inverse { 1u32 } else { 0u32 },
            0u32,
        ];
        self.device
            .queue
            .write_buffer(&self.params_buffer, 0, bytemuck::cast_slice(&params_data));

        // Stage 1: Bit reversal (DIT algorithm - bit-reverse input on GPU)
        {
            let mut encoder =
                self.device
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("FFT Bit Reversal Encoder"),
                    });

            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("FFT Bit Reversal"),
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.bit_reversal_pipeline);
            pass.set_bind_group(0, &self.main_bind_group, &[]);

            let workgroups = (self.fft_size as u32).div_ceil(256);
            pass.dispatch_workgroups(workgroups, 1, 1);

            drop(pass);
            self.device.queue.submit(Some(encoder.finish()));
        }

        // Stage 2: Butterfly operations (one pass per FFT stage)
        for stage in 0..self.log2_size {
            // Update butterfly params
            let butterfly_params = [stage, 0u32, 0u32, 0u32];
            self.device.queue.write_buffer(
                &self.butterfly_params_buffer,
                0,
                bytemuck::cast_slice(&butterfly_params),
            );

            let mut encoder =
                self.device
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some(&format!("FFT Butterfly Stage {} Encoder", stage)),
                    });

            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some(&format!("FFT Butterfly Stage {}", stage)),
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.butterfly_pipeline);
            pass.set_bind_group(0, &self.main_bind_group, &[]);
            pass.set_bind_group(1, &self.butterfly_bind_groups[stage as usize], &[]);

            // Number of pairs to process at this stage
            let block_size = 1u32 << (stage + 1);
            let half_block = block_size >> 1;
            let num_pairs = (self.fft_size as u32 / block_size) * half_block;

            let workgroups = num_pairs.div_ceil(256);
            pass.dispatch_workgroups(workgroups, 1, 1);

            drop(pass);
            self.device.queue.submit(Some(encoder.finish()));
        }

        // Stage 3: Normalization (for inverse FFT)
        if inverse {
            let mut encoder =
                self.device
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("FFT Normalization Encoder"),
                    });

            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("FFT Normalization"),
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.normalize_pipeline);
            pass.set_bind_group(0, &self.main_bind_group, &[]);

            let workgroups = (self.fft_size as u32).div_ceil(256);
            pass.dispatch_workgroups(workgroups, 1, 1);

            drop(pass);
            self.device.queue.submit(Some(encoder.finish()));
        }

        // Read back results
        let buffer_size = (self.fft_size * 2 * std::mem::size_of::<f32>()) as u64;
        let staging_buffer = self.device.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("FFT Staging Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder =
            self.device
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("FFT Readback Encoder"),
                });
        encoder.copy_buffer_to_buffer(&self.data_buffer, 0, &staging_buffer, 0, buffer_size);
        self.device.queue.submit(Some(encoder.finish()));

        // Read back results from staging buffer
        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();

        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            sender.send(result).ok();
        });

        self.device.device.poll(wgpu::Maintain::Wait);
        pollster::block_on(receiver.receive())
            .context("Failed to receive buffer mapping result")?
            .context("Buffer mapping failed")?;

        {
            let data_view = buffer_slice.get_mapped_range();
            let result: &[f32] = bytemuck::cast_slice(&data_view);

            // Convert back to Complex<f32>
            for i in 0..self.fft_size {
                data[i] = Complex::new(result[i * 2], result[i * 2 + 1]);
            }
        }

        staging_buffer.unmap();

        Ok(())
    }

    /// Get FFT size
    pub fn size(&self) -> usize {
        self.fft_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustfft::FftPlanner;

    #[test]
    #[ignore] // Requires GPU
    fn test_fft_forward() {
        let device = GpuDevice::new().unwrap();
        let mut gpu_fft = GpuFft::new(Arc::new(device), 1024).unwrap();

        // Create test signal (sine wave)
        let mut data = vec![Complex::new(0.0, 0.0); 1024];
        for i in 0..1024 {
            let t = i as f32 / 1024.0;
            data[i] = Complex::new((2.0 * PI * 10.0 * t).sin(), 0.0);
        }

        // GPU FFT
        let mut gpu_result = data.clone();
        gpu_fft.forward(&mut gpu_result).unwrap();

        // CPU FFT for comparison
        let mut cpu_result = data.clone();
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(1024);
        fft.process(&mut cpu_result);

        // Compare results (allow small numerical differences)
        for i in 0..1024 {
            let diff_re = (gpu_result[i].re - cpu_result[i].re).abs();
            let diff_im = (gpu_result[i].im - cpu_result[i].im).abs();
            assert!(
                diff_re < 0.001,
                "Real part mismatch at {}: {} vs {}",
                i,
                gpu_result[i].re,
                cpu_result[i].re
            );
            assert!(
                diff_im < 0.001,
                "Imag part mismatch at {}: {} vs {}",
                i,
                gpu_result[i].im,
                cpu_result[i].im
            );
        }
    }

    #[test]
    #[ignore] // Requires GPU
    fn test_fft_inverse() {
        let device = GpuDevice::new().unwrap();
        let mut gpu_fft = GpuFft::new(Arc::new(device), 1024).unwrap();

        // Create test signal
        let original = vec![Complex::new(1.0, 0.0); 1024];
        let mut data = original.clone();

        // Forward then inverse should recover original
        gpu_fft.forward(&mut data).unwrap();
        gpu_fft.inverse(&mut data).unwrap();

        for i in 0..1024 {
            let diff_re = (data[i].re - original[i].re).abs();
            let diff_im = (data[i].im - original[i].im).abs();
            assert!(diff_re < 0.001, "Real part mismatch at {}", i);
            assert!(diff_im < 0.001, "Imag part mismatch at {}", i);
        }
    }
}
