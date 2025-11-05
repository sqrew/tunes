//! Advanced synthesis techniques and modulation

pub mod waveform;
pub mod envelope;
pub mod lfo;
pub mod filter;
pub mod noise;
pub mod automation;
pub mod sample;
pub mod effects;
pub mod fm_synthesis;
pub mod granular;
pub mod wavetable;
pub mod filter_envelope;

// Re-export main types for convenience
pub use waveform::Waveform;
pub use lfo::{LFO, ModRoute, ModTarget};
pub use filter::{Filter, FilterType};
pub use noise::NoiseType;
pub use automation::{Automation, Interpolation};
pub use sample::Sample;
pub use effects::*;
pub use envelope::Envelope;
pub use fm_synthesis::FMParams;
pub use granular::GranularParams;
pub use wavetable::Wavetable;
pub use filter_envelope::FilterEnvelope;
