//! Error types for the tunes library
//!
//! This module provides a unified error handling system for all operations
//! that can fail in the tunes library.

use std::fmt;

/// Main error type for the tunes library
#[derive(Debug, Clone)]
pub enum TunesError {
    /// Sample not found in composition
    SampleNotFound(String),

    /// Section not found in composition
    SectionNotFound(String),

    /// Track not found in composition or section
    TrackNotFound(String),

    /// Template not found in composition
    TemplateNotFound(String),

    /// Marker not found in composition
    MarkerNotFound(String),

    /// Audio engine initialization or operation failed
    AudioEngineError(String),

    /// Invalid event type for operation
    InvalidEventType {
        expected: String,
        found: String,
        operation: String,
    },

    /// WAV file reading error
    WavReadError(String),

    /// WAV file writing error
    WavWriteError(String),

    /// MIDI file error
    MidiError(String),

    /// Invalid audio format
    InvalidAudioFormat(String),

    /// Effect not found or misconfigured
    EffectError(String),

    /// Invalid timing or tempo
    TimingError(String),

    /// Sequence operation error
    SequenceError(String),

    /// IO error
    IoError(String),

    /// Generic error for cases not covered above
    Other(String),
}

impl fmt::Display for TunesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TunesError::SampleNotFound(name) => {
                write!(
                    f,
                    "Sample '{}' not found. Load it first with comp.load_sample()",
                    name
                )
            }
            TunesError::SectionNotFound(name) => {
                write!(f, "Section '{}' not found", name)
            }
            TunesError::TrackNotFound(name) => {
                write!(f, "Track '{}' not found", name)
            }
            TunesError::TemplateNotFound(name) => {
                write!(
                    f,
                    "Template '{}' not found. Create it first with comp.track_template()",
                    name
                )
            }
            TunesError::MarkerNotFound(name) => {
                write!(f, "Marker '{}' not found", name)
            }
            TunesError::AudioEngineError(msg) => {
                write!(f, "Audio engine error: {}", msg)
            }
            TunesError::InvalidEventType {
                expected,
                found,
                operation,
            } => {
                write!(
                    f,
                    "Invalid event type for {}: expected {}, found {}",
                    operation, expected, found
                )
            }
            TunesError::WavReadError(msg) => {
                write!(f, "WAV read error: {}", msg)
            }
            TunesError::WavWriteError(msg) => {
                write!(f, "WAV write error: {}", msg)
            }
            TunesError::MidiError(msg) => {
                write!(f, "MIDI error: {}", msg)
            }
            TunesError::InvalidAudioFormat(msg) => {
                write!(f, "Invalid audio format: {}", msg)
            }
            TunesError::EffectError(msg) => {
                write!(f, "Effect error: {}", msg)
            }
            TunesError::TimingError(msg) => {
                write!(f, "Timing error: {}", msg)
            }
            TunesError::SequenceError(msg) => {
                write!(f, "Sequence error: {}", msg)
            }
            TunesError::IoError(msg) => {
                write!(f, "IO error: {}", msg)
            }
            TunesError::Other(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl std::error::Error for TunesError {}

// Conversion from IO errors
impl From<std::io::Error> for TunesError {
    fn from(err: std::io::Error) -> Self {
        TunesError::IoError(err.to_string())
    }
}

// Conversion from cpal errors
impl From<cpal::BuildStreamError> for TunesError {
    fn from(err: cpal::BuildStreamError) -> Self {
        TunesError::AudioEngineError(format!("Failed to build audio stream: {}", err))
    }
}

impl From<cpal::PlayStreamError> for TunesError {
    fn from(err: cpal::PlayStreamError) -> Self {
        TunesError::AudioEngineError(format!("Failed to play audio stream: {}", err))
    }
}

// Conversion from string errors (for convenience)
impl From<String> for TunesError {
    fn from(err: String) -> Self {
        TunesError::Other(err)
    }
}

impl From<&str> for TunesError {
    fn from(err: &str) -> Self {
        TunesError::Other(err.to_string())
    }
}

/// Result type alias for tunes operations
pub type Result<T> = std::result::Result<T, TunesError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = TunesError::SampleNotFound("kick.wav".to_string());
        assert_eq!(
            err.to_string(),
            "Sample 'kick.wav' not found. Load it first with comp.load_sample()"
        );
    }

    #[test]
    fn test_invalid_event_type_error() {
        let err = TunesError::InvalidEventType {
            expected: "NoteEvent".to_string(),
            found: "DrumEvent".to_string(),
            operation: "vibrato".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Invalid event type for vibrato: expected NoteEvent, found DrumEvent"
        );
    }

    #[test]
    fn test_from_string() {
        let err: TunesError = "Something went wrong".into();
        assert_eq!(err.to_string(), "Something went wrong");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let tunes_err: TunesError = io_err.into();
        assert!(matches!(tunes_err, TunesError::IoError(_)));
    }
}
