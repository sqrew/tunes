//! GPU-accelerated synthesis methods for the Mixer.
//!
//! This module is conditionally compiled with the "gpu" feature.

#[cfg(feature = "gpu")]
use super::Mixer;
#[cfg(feature = "gpu")]
use crate::gpu::{GpuDevice, GpuSynthesizer};
#[cfg(feature = "gpu")]
use std::sync::Arc;

#[cfg(feature = "gpu")]
impl Mixer {
    /// Enable GPU-accelerated synthesis
    ///
    /// This enables GPU compute shaders for potentially 500-1000x faster synthesis
    /// **on discrete GPUs**. Performance depends heavily on GPU hardware:
    ///
    /// - **Discrete GPUs** (RTX 3060+, RX 6000+): 50-500x faster than CPU
    /// - **Integrated GPUs** (Intel HD/UHD): May be slower than CPU
    /// - **No GPU**: Automatic fallback to fast CPU synthesis
    ///
    /// **Note**: GPU acceleration works best with:
    /// - Large workloads (100+ unique sounds)
    /// - Complex synthesis (multi-oscillator FM, filters)
    /// - Discrete graphics cards
    ///
    /// **Important**: GPU synthesis requires caching to be enabled.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// let mut mixer = Composition::new(Tempo::new(120.0)).into_mixer();
    /// mixer.enable_cache();  // Required!
    /// mixer.enable_gpu();    // Try GPU acceleration
    /// ```
    pub fn enable_gpu(&mut self) -> &mut Self {
        self.enable_gpu_with_output(true)
    }

    /// Enable GPU with optional console output
    pub fn enable_gpu_with_output(&mut self, print_info: bool) -> &mut Self {
        // Try to initialize GPU
        match GpuDevice::new() {
            Ok(device) => match GpuSynthesizer::new(device) {
                Ok(synthesizer) => {
                    self.gpu_synthesizer = Some(Arc::new(synthesizer));
                    if print_info {
                        println!("✅ GPU synthesis enabled");
                    }
                }
                Err(e) => {
                    if print_info {
                        eprintln!("⚠️  Failed to create GPU synthesizer: {}", e);
                        eprintln!("   Falling back to CPU synthesis");
                    }
                }
            },
            Err(e) => {
                if print_info {
                    eprintln!("⚠️  GPU not available: {}", e);
                    eprintln!("   Using CPU synthesis");
                }
            }
        }

        self
    }

    /// Disable GPU synthesis (fall back to CPU)
    pub fn disable_gpu(&mut self) -> &mut Self {
        self.gpu_synthesizer = None;
        self
    }

    /// Check if GPU synthesis is enabled
    pub fn gpu_enabled(&self) -> bool {
        self.gpu_synthesizer.is_some()
    }

    /// Enable both cache and GPU acceleration in one call
    ///
    /// This is a convenience wrapper that enables both the sample cache and GPU
    /// compute shaders. Since GPU acceleration requires caching to be effective,
    /// this is the recommended way to enable maximum performance.
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums").note(&[C4], 0.5);
    ///
    /// let mut mixer = comp.into_mixer();
    /// mixer.enable_cache_and_gpu();  // Experimental GPU acceleration
    ///
    /// // Now export or play with GPU acceleration
    /// # let engine = AudioEngine::new()?;
    /// engine.export_wav(&mut mixer, "output.wav")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn enable_cache_and_gpu(&mut self) -> &mut Self {
        self.enable_cache();
        self.enable_gpu();
        self
    }
}
