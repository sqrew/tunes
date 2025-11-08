//! Spatial audio processing for 3D sound positioning
//!
//! This module provides spatial audio capabilities including:
//! - 3D positioning of sound sources
//! - Distance-based attenuation
//! - Azimuth-based stereo panning
//! - Listener position and orientation
//! - Doppler effect for moving sources

use std::f32::consts::PI;

/// 3D vector for positions and directions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Create a new 3D vector
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Zero vector (origin)
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Forward direction (positive Z)
    pub fn forward() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// Up direction (positive Y)
    pub fn up() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    /// Right direction (positive X)
    pub fn right() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    /// Calculate the length (magnitude) of the vector
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Calculate the squared length (avoids sqrt for performance)
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Normalize the vector to unit length
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self::new(self.x / len, self.y / len, self.z / len)
        } else {
            *self
        }
    }

    /// Dot product with another vector
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product with another vector
    pub fn cross(&self, other: &Vec3) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Subtract another vector
    pub fn sub(&self, other: &Vec3) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// Add another vector
    pub fn add(&self, other: &Vec3) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    /// Scale the vector by a scalar
    pub fn scale(&self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

/// Distance attenuation model for spatial audio
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttenuationModel {
    /// No attenuation (constant volume regardless of distance)
    None,
    /// Linear attenuation: volume = 1 - (distance / max_distance)
    Linear,
    /// Inverse distance: volume = ref_distance / (ref_distance + rolloff * (distance - ref_distance))
    Inverse,
    /// Inverse square (realistic): volume = (ref_distance / distance)^2
    InverseSquare,
    /// Exponential: volume = (distance / ref_distance)^(-rolloff)
    Exponential,
}

impl Default for AttenuationModel {
    fn default() -> Self {
        Self::InverseSquare
    }
}

/// Spatial position and velocity for a sound source
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialPosition {
    /// Position in 3D space
    pub position: Vec3,
    /// Velocity for Doppler effect (units per second)
    pub velocity: Vec3,
}

impl SpatialPosition {
    /// Create a new spatial position at a given location
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3::new(x, y, z),
            velocity: Vec3::zero(),
        }
    }

    /// Create a spatial position with velocity
    pub fn with_velocity(x: f32, y: f32, z: f32, vx: f32, vy: f32, vz: f32) -> Self {
        Self {
            position: Vec3::new(x, y, z),
            velocity: Vec3::new(vx, vy, vz),
        }
    }

    /// Set the position
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Vec3::new(x, y, z);
    }

    /// Set the velocity
    pub fn set_velocity(&mut self, vx: f32, vy: f32, vz: f32) {
        self.velocity = Vec3::new(vx, vy, vz);
    }
}

impl Default for SpatialPosition {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

/// Listener configuration for spatial audio
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ListenerConfig {
    /// Position in 3D space
    pub position: Vec3,
    /// Forward direction (where listener is facing)
    pub forward: Vec3,
    /// Up direction (top of listener's head)
    pub up: Vec3,
    /// Velocity for Doppler effect
    pub velocity: Vec3,
}

impl ListenerConfig {
    /// Create a new listener at the origin facing forward
    pub fn new() -> Self {
        Self {
            position: Vec3::zero(),
            forward: Vec3::forward(),
            up: Vec3::up(),
            velocity: Vec3::zero(),
        }
    }

    /// Set the listener position
    pub fn with_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.position = Vec3::new(x, y, z);
        self
    }

    /// Set the listener forward direction
    pub fn with_forward(mut self, x: f32, y: f32, z: f32) -> Self {
        self.forward = Vec3::new(x, y, z).normalize();
        self
    }

    /// Set the listener up direction
    pub fn with_up(mut self, x: f32, y: f32, z: f32) -> Self {
        self.up = Vec3::new(x, y, z).normalize();
        self
    }

    /// Set the listener velocity
    pub fn with_velocity(mut self, vx: f32, vy: f32, vz: f32) -> Self {
        self.velocity = Vec3::new(vx, vy, vz);
        self
    }

    /// Calculate the right vector (perpendicular to forward and up)
    pub fn right(&self) -> Vec3 {
        self.forward.cross(&self.up).normalize()
    }
}

impl Default for ListenerConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Parameters for spatial audio calculation
#[derive(Debug, Clone, Copy)]
pub struct SpatialParams {
    /// Distance attenuation model
    pub attenuation_model: AttenuationModel,
    /// Reference distance (distance at which volume is 1.0)
    pub ref_distance: f32,
    /// Maximum distance (beyond this, sound is silent)
    pub max_distance: f32,
    /// Rolloff factor (affects attenuation curve steepness)
    pub rolloff: f32,
    /// Speed of sound (for Doppler effect, in units per second)
    pub speed_of_sound: f32,
    /// Enable Doppler effect
    pub doppler_enabled: bool,
    /// Doppler factor (scales the effect, 1.0 = realistic)
    pub doppler_factor: f32,
}

impl Default for SpatialParams {
    fn default() -> Self {
        Self {
            attenuation_model: AttenuationModel::InverseSquare,
            ref_distance: 1.0,
            max_distance: 100.0,
            rolloff: 1.0,
            speed_of_sound: 343.0, // meters per second (realistic)
            doppler_enabled: true,
            doppler_factor: 1.0,
        }
    }
}

/// Result of spatial audio calculation
#[derive(Debug, Clone, Copy)]
pub struct SpatialResult {
    /// Volume attenuation (0.0 to 1.0)
    pub volume: f32,
    /// Stereo pan (-1.0 = left, 0.0 = center, 1.0 = right)
    pub pan: f32,
    /// Pitch multiplier for Doppler effect (1.0 = no change)
    pub pitch: f32,
}

impl Default for SpatialResult {
    fn default() -> Self {
        Self {
            volume: 1.0,
            pan: 0.0,
            pitch: 1.0,
        }
    }
}

/// Calculate distance attenuation based on the model and parameters
pub fn calculate_attenuation(
    distance: f32,
    model: AttenuationModel,
    ref_distance: f32,
    max_distance: f32,
    rolloff: f32,
) -> f32 {
    match model {
        AttenuationModel::None => 1.0,
        AttenuationModel::Linear => {
            if distance >= max_distance {
                0.0
            } else {
                1.0 - (distance / max_distance)
            }
        }
        AttenuationModel::Inverse => {
            let clamped_distance = distance.max(ref_distance);
            ref_distance / (ref_distance + rolloff * (clamped_distance - ref_distance))
        }
        AttenuationModel::InverseSquare => {
            if distance < ref_distance {
                1.0
            } else {
                (ref_distance / distance).powi(2)
            }
        }
        AttenuationModel::Exponential => {
            if distance < ref_distance {
                1.0
            } else {
                (distance / ref_distance).powf(-rolloff)
            }
        }
    }
}

/// Calculate azimuth angle (horizontal angle) from listener to source
/// Returns angle in radians, where 0 = forward, PI/2 = right, -PI/2 = left
pub fn calculate_azimuth(source_pos: &Vec3, listener: &ListenerConfig) -> f32 {
    // Vector from listener to source
    let to_source = source_pos.sub(&listener.position);

    // Project onto horizontal plane (ignore Y)
    let to_source_flat = Vec3::new(to_source.x, 0.0, to_source.z);
    let forward_flat = Vec3::new(listener.forward.x, 0.0, listener.forward.z);

    if to_source_flat.length() < 0.001 {
        return 0.0; // Source is at listener position
    }

    let to_source_norm = to_source_flat.normalize();
    let forward_norm = forward_flat.normalize();

    // Calculate angle using dot product and cross product
    let dot = forward_norm.dot(&to_source_norm);
    let cross = forward_norm.cross(&to_source_norm);

    // atan2 gives us the signed angle
    cross.y.atan2(dot)
}

/// Calculate stereo pan from azimuth angle
/// Maps azimuth to pan: -PI/2 (left) → -1.0, 0 (forward) → 0.0, PI/2 (right) → 1.0
pub fn azimuth_to_pan(azimuth: f32) -> f32 {
    // Normalize azimuth from [-PI, PI] to [-1, 1]
    // Clamp to [-PI/2, PI/2] for left-right panning
    let clamped = azimuth.clamp(-PI / 2.0, PI / 2.0);
    clamped / (PI / 2.0)
}

/// Calculate Doppler pitch shift
/// Returns pitch multiplier (1.0 = no shift, >1.0 = higher pitch, <1.0 = lower pitch)
pub fn calculate_doppler(
    source_pos: &Vec3,
    source_velocity: &Vec3,
    listener: &ListenerConfig,
    speed_of_sound: f32,
    doppler_factor: f32,
) -> f32 {
    // Vector from listener to source
    let to_source = source_pos.sub(&listener.position);
    let distance = to_source.length();

    if distance < 0.001 {
        return 1.0; // Too close, no Doppler
    }

    let direction = to_source.scale(1.0 / distance);

    // Velocity of source along the line from listener to source
    let source_radial_vel = source_velocity.dot(&direction);

    // Velocity of listener along the line from listener to source
    let listener_radial_vel = listener.velocity.dot(&direction);

    // Relative velocity (positive = moving apart, negative = moving together)
    let relative_velocity = source_radial_vel - listener_radial_vel;

    // Doppler formula: f_observed = f_source * (v_sound + v_listener) / (v_sound + v_source)
    // Simplified: pitch = (v_sound - v_relative) / v_sound
    let doppler_shift = (speed_of_sound - relative_velocity * doppler_factor) / speed_of_sound;

    // Clamp to reasonable range (0.5 to 2.0 = one octave down/up)
    doppler_shift.clamp(0.5, 2.0)
}

/// Calculate complete spatial audio result
pub fn calculate_spatial(
    source: &SpatialPosition,
    listener: &ListenerConfig,
    params: &SpatialParams,
) -> SpatialResult {
    // Calculate distance
    let to_source = source.position.sub(&listener.position);
    let distance = to_source.length();

    // Calculate attenuation
    let volume = if distance >= params.max_distance {
        0.0
    } else {
        calculate_attenuation(
            distance,
            params.attenuation_model,
            params.ref_distance,
            params.max_distance,
            params.rolloff,
        )
    };

    // Calculate azimuth and pan
    let azimuth = calculate_azimuth(&source.position, listener);
    let pan = azimuth_to_pan(azimuth);

    // Calculate Doppler pitch shift
    let pitch = if params.doppler_enabled {
        calculate_doppler(
            &source.position,
            &source.velocity,
            listener,
            params.speed_of_sound,
            params.doppler_factor,
        )
    } else {
        1.0
    };

    SpatialResult { volume, pan, pitch }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_operations() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v1.add(&v2), Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(v1.sub(&v2), Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(v1.scale(2.0), Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_vec3_length() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert!((v.length() - 5.0).abs() < 0.001);
        assert_eq!(v.length_squared(), 25.0);
    }

    #[test]
    fn test_vec3_normalize() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let normalized = v.normalize();
        assert!((normalized.length() - 1.0).abs() < 0.001);
        assert!((normalized.x - 0.6).abs() < 0.001);
        assert!((normalized.y - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_attenuation_none() {
        let attenuation = calculate_attenuation(10.0, AttenuationModel::None, 1.0, 100.0, 1.0);
        assert_eq!(attenuation, 1.0);
    }

    #[test]
    fn test_attenuation_linear() {
        let attenuation = calculate_attenuation(50.0, AttenuationModel::Linear, 1.0, 100.0, 1.0);
        assert!((attenuation - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_attenuation_inverse_square() {
        let attenuation =
            calculate_attenuation(2.0, AttenuationModel::InverseSquare, 1.0, 100.0, 1.0);
        assert!((attenuation - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_azimuth_forward() {
        let listener = ListenerConfig::new();
        let source = Vec3::new(0.0, 0.0, 10.0); // In front
        let azimuth = calculate_azimuth(&source, &listener);
        assert!(azimuth.abs() < 0.01);
    }

    #[test]
    fn test_azimuth_right() {
        let listener = ListenerConfig::new();
        let source = Vec3::new(10.0, 0.0, 0.0); // To the right
        let azimuth = calculate_azimuth(&source, &listener);
        assert!((azimuth - PI / 2.0).abs() < 0.01);
    }

    #[test]
    fn test_azimuth_left() {
        let listener = ListenerConfig::new();
        let source = Vec3::new(-10.0, 0.0, 0.0); // To the left
        let azimuth = calculate_azimuth(&source, &listener);
        assert!((azimuth + PI / 2.0).abs() < 0.01);
    }

    #[test]
    fn test_azimuth_to_pan() {
        assert_eq!(azimuth_to_pan(0.0), 0.0); // Forward = center
        assert!((azimuth_to_pan(PI / 2.0) - 1.0).abs() < 0.001); // Right = 1.0
        assert!((azimuth_to_pan(-PI / 2.0) + 1.0).abs() < 0.001); // Left = -1.0
    }

    #[test]
    fn test_doppler_approaching() {
        let source_pos = Vec3::new(0.0, 0.0, 10.0);
        let source_velocity = Vec3::new(0.0, 0.0, -10.0); // Moving toward listener
        let listener = ListenerConfig::new();
        let pitch = calculate_doppler(&source_pos, &source_velocity, &listener, 343.0, 1.0);
        assert!(pitch > 1.0); // Higher pitch when approaching
    }

    #[test]
    fn test_doppler_receding() {
        let source_pos = Vec3::new(0.0, 0.0, 10.0);
        let source_velocity = Vec3::new(0.0, 0.0, 10.0); // Moving away
        let listener = ListenerConfig::new();
        let pitch = calculate_doppler(&source_pos, &source_velocity, &listener, 343.0, 1.0);
        assert!(pitch < 1.0); // Lower pitch when receding
    }

    #[test]
    fn test_spatial_calculation() {
        let source = SpatialPosition::new(10.0, 0.0, 0.0); // 10m to the right
        let listener = ListenerConfig::new();
        let params = SpatialParams::default();

        let result = calculate_spatial(&source, &listener, &params);

        assert!(result.volume < 1.0); // Attenuated due to distance
        assert!(result.pan > 0.0); // Panned right
        assert!((result.pitch - 1.0).abs() < 0.01); // No Doppler (no velocity)
    }

    #[test]
    fn test_spatial_at_listener() {
        let source = SpatialPosition::new(0.0, 0.0, 0.0); // At listener
        let listener = ListenerConfig::new();
        let params = SpatialParams::default();

        let result = calculate_spatial(&source, &listener, &params);

        assert_eq!(result.volume, 1.0); // Full volume at origin
        assert_eq!(result.pan, 0.0); // Centered
    }

    #[test]
    fn test_spatial_beyond_max_distance() {
        let source = SpatialPosition::new(200.0, 0.0, 0.0); // Beyond max distance
        let listener = ListenerConfig::new();
        let params = SpatialParams::default();

        let result = calculate_spatial(&source, &listener, &params);

        assert_eq!(result.volume, 0.0); // Silent beyond max distance
    }
}
