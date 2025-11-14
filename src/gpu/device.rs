//! GPU device initialization and management
use anyhow::{Context, Result};
use std::sync::Arc;

/// GPU device state (shared across threads)
#[derive(Clone)]
pub struct GpuDevice {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
}

/// GPU availability state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuState {
    /// GPU is available and initialized
    Available,
    /// GPU is not available (fallback to CPU)
    Unavailable,
}

/// GPU type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuType {
    /// Discrete GPU (dedicated graphics card) - Usually fast for compute
    Discrete,
    /// Integrated GPU (CPU graphics) - Usually slower, may not benefit from compute
    Integrated,
    /// Unknown GPU type
    Unknown,
}

impl GpuDevice {
    /// Initialize GPU device for compute shaders
    ///
    /// This attempts to initialize a GPU device with compute shader support.
    /// Returns an error if no suitable GPU is found.
    pub fn new() -> Result<Self> {
        pollster::block_on(async {
            // Create wgpu instance
            let instance = wgpu::Instance::default();

            // Request adapter (GPU)
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: None,
                    force_fallback_adapter: false,
                })
                .await
                .context("Failed to find a suitable GPU adapter")?;

            // Print GPU info
            let info = adapter.get_info();
            let gpu_type = Self::classify_gpu_type(&info);

            println!("🎮 GPU Device: {} ({:?})", info.name, info.backend);
            println!("   Driver: {} / {}", info.driver, info.driver_info);
            println!("   Type: {:?}", gpu_type);

            // Warn on integrated GPUs
            if gpu_type == GpuType::Integrated {
                println!("   ⚠️  Integrated GPU detected - may be slower than CPU synthesis");
                println!("   💡 Tip: GPU acceleration works best with discrete graphics cards");
            }

            // Request device and queue
            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: Some("Tunes GPU Synthesizer"),
                        required_features: wgpu::Features::empty(),
                        required_limits: wgpu::Limits::default(),
                        memory_hints: wgpu::MemoryHints::Performance,
                    },
                    None,
                )
                .await
                .context("Failed to create GPU device")?;

            Ok(Self {
                device: Arc::new(device),
                queue: Arc::new(queue),
            })
        })
    }

    /// Get GPU state (available/unavailable)
    pub fn state(&self) -> GpuState {
        GpuState::Available
    }

    /// Check if this device supports compute shaders
    pub fn supports_compute(&self) -> bool {
        // All devices we successfully initialize support compute
        true
    }

    /// Classify GPU type based on adapter info
    fn classify_gpu_type(info: &wgpu::AdapterInfo) -> GpuType {
        let name_lower = info.name.to_lowercase();

        // Check for integrated GPU keywords
        if name_lower.contains("intel")
            && (name_lower.contains("hd graphics")
                || name_lower.contains("uhd graphics")
                || name_lower.contains("iris")
                || name_lower.contains("integrated"))
        {
            return GpuType::Integrated;
        }

        if name_lower.contains("amd")
            && (name_lower.contains("radeon(tm) graphics")
                || name_lower.contains("vega") && !name_lower.contains("rx"))
        {
            return GpuType::Integrated;
        }

        // Check for discrete GPU keywords
        if name_lower.contains("nvidia")
            || name_lower.contains("geforce")
            || name_lower.contains("rtx")
            || name_lower.contains("gtx")
        {
            return GpuType::Discrete;
        }

        if name_lower.contains("amd")
            && (name_lower.contains("radeon rx")
                || name_lower.contains("radeon r9")
                || name_lower.contains("radeon r7"))
        {
            return GpuType::Discrete;
        }

        // Default to unknown
        GpuType::Unknown
    }
}

impl std::fmt::Debug for GpuDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GpuDevice")
            .field("state", &self.state())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_initialization() {
        match GpuDevice::new() {
            Ok(device) => {
                println!("GPU initialized successfully: {:?}", device);
                assert_eq!(device.state(), GpuState::Available);
                assert!(device.supports_compute());
            }
            Err(e) => {
                println!("GPU not available (expected on some systems): {}", e);
            }
        }
    }
}
