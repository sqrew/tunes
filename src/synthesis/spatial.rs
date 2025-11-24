//! Spatial audio processing for 3D sound positioning
//!
//! This module provides spatial audio capabilities including:
//! - 3D positioning of sound sources
//! - Distance-based attenuation
//! - Azimuth-based stereo panning with elevation
//! - Listener position and orientation
//! - Doppler effect for moving sources
//! - Directional sound sources (sound cones)
//! - Occlusion support
//! - SIMD-accelerated batch processing (process 4-8 sounds at once)

use crate::synthesis::simd::{SimdLanes, SimdWidth, SIMD};
use std::f32::consts::PI;
use wide::{f32x4, f32x8};

/// Fast inverse square root (Quake III style)
/// Returns 1/sqrt(x) approximately 2x faster than 1.0/x.sqrt()
/// Accuracy: ~0.04% error with 2 Newton-Raphson iterations
#[inline]
fn fast_inv_sqrt(x: f32) -> f32 {
    // Modern Rust version of the famous Quake III fast inverse square root
    // Uses f32::from_bits for type punning (safe in Rust)
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1); // Magic constant
    let y = f32::from_bits(i);

    // Two Newton-Raphson iterations for excellent accuracy (~0.04% error)
    // Still ~1.8x faster than standard 1.0/sqrt(x) while being accurate enough for tests
    let y = y * (1.5 - 0.5 * x * y * y);
    y * (1.5 - 0.5 * x * y * y)
}

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
        let len_squared = self.length_squared();
        if len_squared > 1e-10 {
            // Fast inverse square root (Quake III style, ~2x faster than 1.0/sqrt(x))
            // Accurate enough for audio spatial calculations
            let inv_len = fast_inv_sqrt(len_squared);
            Self::new(self.x * inv_len, self.y * inv_len, self.z * inv_len)
        } else {
            *self
        }
    }

    /// Normalize using precise sqrt (for when accuracy is critical)
    pub fn normalize_precise(&self) -> Self {
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

/// Sound cone configuration for directional audio sources
///
/// A sound cone defines how a sound's volume changes based on the angle
/// between the source's forward direction and the direction to the listener.
///
/// # Example
/// ```
/// use tunes::synthesis::spatial::{SoundCone, Vec3};
///
/// // Create a narrow cone (like a megaphone or loudspeaker)
/// let cone = SoundCone::new(
///     Vec3::new(0.0, 0.0, 1.0), // Forward direction
///     30.0,  // Inner cone angle (30 degrees)
///     60.0,  // Outer cone angle (60 degrees)
///     0.3,   // Outer gain (30% volume outside cone)
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoundCone {
    /// Direction the sound source is pointing (normalized)
    pub direction: Vec3,
    /// Inner cone angle in degrees (full volume within this angle)
    pub inner_angle: f32,
    /// Outer cone angle in degrees (transitions from full to outer_gain)
    pub outer_angle: f32,
    /// Volume multiplier outside the outer cone (0.0 to 1.0)
    pub outer_gain: f32,
}

impl SoundCone {
    /// Create a new sound cone
    ///
    /// # Arguments
    /// * `direction` - Direction the source is pointing (will be normalized)
    /// * `inner_angle` - Inner cone angle in degrees (full volume)
    /// * `outer_angle` - Outer cone angle in degrees (transition region)
    /// * `outer_gain` - Volume multiplier outside cone (0.0 to 1.0)
    pub fn new(direction: Vec3, inner_angle: f32, outer_angle: f32, outer_gain: f32) -> Self {
        Self {
            direction: direction.normalize(),
            inner_angle: inner_angle.clamp(0.0, 360.0),
            outer_angle: outer_angle.clamp(0.0, 360.0),
            outer_gain: outer_gain.clamp(0.0, 1.0),
        }
    }

    /// Create a narrow cone (like a megaphone)
    /// Inner: 20°, Outer: 40°, Outer gain: 0.2
    pub fn narrow() -> Self {
        Self::new(Vec3::forward(), 20.0, 40.0, 0.2)
    }

    /// Create a medium cone (like a speaker)
    /// Inner: 45°, Outer: 90°, Outer gain: 0.3
    pub fn medium() -> Self {
        Self::new(Vec3::forward(), 45.0, 90.0, 0.3)
    }

    /// Create a wide cone (like a person talking)
    /// Inner: 90°, Outer: 150°, Outer gain: 0.5
    pub fn wide() -> Self {
        Self::new(Vec3::forward(), 90.0, 150.0, 0.5)
    }

    /// Create a 45° cone (tight directional)
    /// Inner: 20°, Outer: 45°, Outer gain: 0.2
    pub fn cone_45() -> Self {
        Self::new(Vec3::forward(), 20.0, 45.0, 0.2)
    }

    /// Create a 90° cone (quarter sphere)
    /// Inner: 45°, Outer: 90°, Outer gain: 0.3
    pub fn cone_90() -> Self {
        Self::new(Vec3::forward(), 45.0, 90.0, 0.3)
    }

    /// Create a 180° cone (hemisphere)
    /// Inner: 90°, Outer: 180°, Outer gain: 0.5
    pub fn cone_180() -> Self {
        Self::new(Vec3::forward(), 90.0, 180.0, 0.5)
    }

    /// Set the direction the cone is pointing
    pub fn with_direction(mut self, x: f32, y: f32, z: f32) -> Self {
        self.direction = Vec3::new(x, y, z).normalize();
        self
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
    /// Occlusion amount (0.0 = no occlusion, 1.0 = fully occluded)
    /// This can be used to apply low-pass filtering when sound is blocked
    pub occlusion: f32,
}

impl Default for SpatialResult {
    fn default() -> Self {
        Self {
            volume: 1.0,
            pan: 0.0,
            pitch: 1.0,
            occlusion: 0.0,
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

    // Fast atan2 gives us the signed angle (~3-4x faster than standard atan2)
    f32::fast_atan2(cross.y, dot)
}

/// Calculate stereo pan from azimuth angle
/// Maps azimuth to pan: -PI/2 (left) → -1.0, 0 (forward) → 0.0, PI/2 (right) → 1.0
pub fn azimuth_to_pan(azimuth: f32) -> f32 {
    // Normalize azimuth from [-PI, PI] to [-1, 1]
    // Clamp to [-PI/2, PI/2] for left-right panning
    let clamped = azimuth.clamp(-PI / 2.0, PI / 2.0);
    clamped / (PI / 2.0)
}

/// Calculate elevation angle (vertical angle) from listener to source
/// Returns angle in radians, where 0 = ear level, PI/2 = directly above, -PI/2 = directly below
pub fn calculate_elevation(source_pos: &Vec3, listener: &ListenerConfig) -> f32 {
    // Vector from listener to source
    let to_source = source_pos.sub(&listener.position);
    let distance_horizontal = (to_source.x * to_source.x + to_source.z * to_source.z).sqrt();

    if distance_horizontal < 0.001 {
        // Source is directly above or below
        if to_source.y > 0.0 {
            return PI / 2.0;
        } else if to_source.y < 0.0 {
            return -PI / 2.0;
        }
        return 0.0;
    }

    // Calculate elevation using atan2
    to_source.y.atan2(distance_horizontal)
}

/// Calculate elevation-based volume attenuation and pan adjustment
/// Returns (volume_multiplier, pan_adjustment)
///
/// Sounds above/below the listener are:
/// - Attenuated in volume (quieter when elevated)
/// - Pulled slightly toward center (mimics how ears perceive height)
pub fn calculate_elevation_effect(elevation: f32) -> (f32, f32) {
    let abs_elevation = elevation.abs();

    // Volume attenuation: reduce volume for sounds above/below
    // At 30 degrees: ~90% volume
    // At 60 degrees: ~70% volume
    // At 90 degrees (directly above/below): ~50% volume
    let elevation_factor = (abs_elevation / (PI / 2.0)).min(1.0);
    let volume_attenuation = 1.0 - (elevation_factor * 0.5);

    // Pan adjustment: pull toward center when elevated
    // This mimics how our ears perceive elevated sounds as more centered
    // At 30 degrees: ~10% pull to center
    // At 60 degrees: ~25% pull to center
    // At 90 degrees: ~40% pull to center
    let pan_reduction = elevation_factor * 0.4;

    (volume_attenuation, pan_reduction)
}

/// Calculate directional gain based on sound cone
/// Returns volume multiplier based on listener position relative to source direction
///
/// Cone angles are specified as the angle from the center axis (not total width).
/// For example, a 30-degree cone means 30 degrees from center, not 60 degrees total.
pub fn calculate_cone_gain(
    source_pos: &Vec3,
    listener_pos: &Vec3,
    cone: &SoundCone,
) -> f32 {
    // Vector from source to listener
    let to_listener = listener_pos.sub(source_pos);
    let distance = to_listener.length();

    if distance < 0.001 {
        return 1.0; // Listener at source position
    }

    let direction_to_listener = to_listener.scale(1.0 / distance);

    // Calculate angle between source direction and direction to listener
    let dot = cone.direction.dot(&direction_to_listener);
    let angle_rad = dot.clamp(-1.0, 1.0).acos();
    let angle_deg = angle_rad.to_degrees();

    // Cone angles are from center axis (no halving needed)
    if angle_deg <= cone.inner_angle {
        // Inside inner cone: full volume
        1.0
    } else if angle_deg >= cone.outer_angle {
        // Outside outer cone: reduced volume
        cone.outer_gain
    } else {
        // In transition zone: interpolate between full and outer gain
        let transition = (angle_deg - cone.inner_angle) / (cone.outer_angle - cone.inner_angle);
        1.0 + (cone.outer_gain - 1.0) * transition
    }
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
    calculate_spatial_with_cone(source, listener, params, None, 0.0)
}

/// Calculate complete spatial audio result with optional directional cone and occlusion
///
/// # Arguments
/// * `source` - Source position and velocity
/// * `listener` - Listener configuration
/// * `params` - Spatial audio parameters
/// * `cone` - Optional sound cone for directional sources
/// * `occlusion` - Occlusion amount (0.0 = none, 1.0 = fully occluded)
pub fn calculate_spatial_with_cone(
    source: &SpatialPosition,
    listener: &ListenerConfig,
    params: &SpatialParams,
    cone: Option<&SoundCone>,
    occlusion: f32,
) -> SpatialResult {
    // Calculate distance using length_squared first for culling (avoids sqrt)
    let to_source = source.position.sub(&listener.position);
    let distance_squared = to_source.length_squared();
    let max_distance_squared = params.max_distance * params.max_distance;

    // Early exit: if beyond max distance, return silent result immediately
    // This saves ~15-20 operations per culled sound (sqrt, trig, attenuation calc)
    if distance_squared >= max_distance_squared {
        return SpatialResult {
            volume: 0.0,
            pan: 0.0,
            pitch: 1.0,
            occlusion: occlusion.clamp(0.0, 1.0),
        };
    }

    // Now compute actual distance (sqrt) only for sounds within range
    let distance = distance_squared.sqrt();

    // Calculate distance attenuation (distance check already done above)
    let mut volume = calculate_attenuation(
        distance,
        params.attenuation_model,
        params.ref_distance,
        params.max_distance,
        params.rolloff,
    );

    // Calculate azimuth and base pan
    let azimuth = calculate_azimuth(&source.position, listener);
    let mut pan = azimuth_to_pan(azimuth);

    // Calculate elevation effects
    let elevation = calculate_elevation(&source.position, listener);
    let (elevation_volume, pan_reduction) = calculate_elevation_effect(elevation);

    // Apply elevation volume attenuation
    volume *= elevation_volume;

    // Apply elevation pan adjustment (pull toward center when elevated)
    pan *= 1.0 - pan_reduction;

    // Apply directional cone if present
    if let Some(sound_cone) = cone {
        let cone_gain = calculate_cone_gain(&source.position, &listener.position, sound_cone);
        volume *= cone_gain;
    }

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

    SpatialResult {
        volume,
        pan,
        pitch,
        occlusion: occlusion.clamp(0.0, 1.0),
    }
}

// ========== SIMD Batch Processing ==========

/// SIMD-accelerated batch distance calculation
/// Computes squared distances for multiple sources in parallel
///
/// # Arguments
/// * `sources` - Array of source positions
/// * `listener_pos` - Single listener position
/// * `distances_squared_out` - Output buffer for squared distances
#[inline]
fn batch_distance_squared_simd<V: SimdLanes>(
    sources_x: &[f32],
    sources_y: &[f32],
    sources_z: &[f32],
    listener_pos: &Vec3,
    distances_squared_out: &mut [f32],
) {
    let lanes = V::LANES;
    let chunks = sources_x.len() / lanes;

    let listener_x = V::splat(listener_pos.x);
    let listener_y = V::splat(listener_pos.y);
    let listener_z = V::splat(listener_pos.z);

    for chunk_idx in 0..chunks {
        let offset = chunk_idx * lanes;

        // Load source positions
        let src_x = V::from_array(&sources_x[offset..offset + lanes]);
        let src_y = V::from_array(&sources_y[offset..offset + lanes]);
        let src_z = V::from_array(&sources_z[offset..offset + lanes]);

        // Calculate deltas: to_source = source - listener
        let dx = src_x.sub(listener_x);
        let dy = src_y.sub(listener_y);
        let dz = src_z.sub(listener_z);

        // Calculate squared distance: dx² + dy² + dz²
        let dx_sq = dx.mul(dx);
        let dy_sq = dy.mul(dy);
        let dz_sq = dz.mul(dz);
        let dist_sq = dx_sq.add(dy_sq).add(dz_sq);

        dist_sq.write_to_slice(&mut distances_squared_out[offset..offset + lanes]);
    }

    // Handle remainder with scalar code
    let remainder_start = chunks * lanes;
    for i in remainder_start..sources_x.len() {
        let dx = sources_x[i] - listener_pos.x;
        let dy = sources_y[i] - listener_pos.y;
        let dz = sources_z[i] - listener_pos.z;
        distances_squared_out[i] = dx * dx + dy * dy + dz * dz;
    }
}

/// Batch calculate squared distances for multiple sources using optimal SIMD width
///
/// This is the public API for batch distance calculation that dispatches to the
/// appropriate SIMD implementation based on CPU capabilities.
///
/// # Example
/// ```ignore
/// let mut distances_sq = vec![0.0; sources.len()];
/// batch_distance_squared(&sources_x, &sources_y, &sources_z, &listener_pos, &mut distances_sq);
/// ```
pub fn batch_distance_squared(
    sources_x: &[f32],
    sources_y: &[f32],
    sources_z: &[f32],
    listener_pos: &Vec3,
    distances_squared_out: &mut [f32],
) {
    assert_eq!(sources_x.len(), sources_y.len());
    assert_eq!(sources_x.len(), sources_z.len());
    assert_eq!(sources_x.len(), distances_squared_out.len());

    match SIMD.simd_width() {
        SimdWidth::X8 => batch_distance_squared_simd::<f32x8>(
            sources_x,
            sources_y,
            sources_z,
            listener_pos,
            distances_squared_out,
        ),
        SimdWidth::X4 => batch_distance_squared_simd::<f32x4>(
            sources_x,
            sources_y,
            sources_z,
            listener_pos,
            distances_squared_out,
        ),
        SimdWidth::Scalar => {
            // Scalar fallback
            for i in 0..sources_x.len() {
                let dx = sources_x[i] - listener_pos.x;
                let dy = sources_y[i] - listener_pos.y;
                let dz = sources_z[i] - listener_pos.z;
                distances_squared_out[i] = dx * dx + dy * dy + dz * dz;
            }
        }
    }
}

/// SIMD-accelerated batch attenuation calculation using inverse square law
///
/// Processes multiple distances in parallel using SIMD
#[inline]
fn batch_attenuation_simd<V: SimdLanes>(
    distances: &[f32],
    ref_distance: f32,
    attenuations_out: &mut [f32],
) {
    let lanes = V::LANES;
    let chunks = distances.len() / lanes;

    let ref_dist_sq = V::splat(ref_distance * ref_distance);

    for chunk_idx in 0..chunks {
        let offset = chunk_idx * lanes;

        let dist_vec = V::from_array(&distances[offset..offset + lanes]);

        // Attenuation = (ref_distance / distance)²
        // = ref_distance² / distance²
        let dist_sq = dist_vec.mul(dist_vec);

        // For distances < ref_distance, clamp attenuation to 1.0
        let attenuation = ref_dist_sq.div(dist_sq).min(V::splat(1.0));

        attenuation.write_to_slice(&mut attenuations_out[offset..offset + lanes]);
    }

    // Handle remainder
    let remainder_start = chunks * lanes;
    for i in remainder_start..distances.len() {
        let dist = distances[i];
        if dist < ref_distance {
            attenuations_out[i] = 1.0;
        } else {
            attenuations_out[i] = (ref_distance / dist).powi(2);
        }
    }
}

/// Batch calculate inverse square attenuation for multiple distances
///
/// # Example
/// ```ignore
/// let mut attenuations = vec![0.0; distances.len()];
/// batch_attenuation_inverse_square(&distances, 1.0, &mut attenuations);
/// ```
pub fn batch_attenuation_inverse_square(
    distances: &[f32],
    ref_distance: f32,
    attenuations_out: &mut [f32],
) {
    assert_eq!(distances.len(), attenuations_out.len());

    match SIMD.simd_width() {
        SimdWidth::X8 => batch_attenuation_simd::<f32x8>(distances, ref_distance, attenuations_out),
        SimdWidth::X4 => batch_attenuation_simd::<f32x4>(distances, ref_distance, attenuations_out),
        SimdWidth::Scalar => {
            for i in 0..distances.len() {
                let dist = distances[i];
                if dist < ref_distance {
                    attenuations_out[i] = 1.0;
                } else {
                    attenuations_out[i] = (ref_distance / dist).powi(2);
                }
            }
        }
    }
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

    #[test]
    fn test_elevation_above() {
        let source = Vec3::new(0.0, 10.0, 0.0); // Directly above
        let listener = ListenerConfig::new();
        let elevation = calculate_elevation(&source, &listener);
        assert!((elevation - PI / 2.0).abs() < 0.01);
    }

    #[test]
    fn test_elevation_below() {
        let source = Vec3::new(0.0, -10.0, 0.0); // Directly below
        let listener = ListenerConfig::new();
        let elevation = calculate_elevation(&source, &listener);
        assert!((elevation + PI / 2.0).abs() < 0.01);
    }

    #[test]
    fn test_elevation_ear_level() {
        let source = Vec3::new(5.0, 0.0, 5.0); // At ear level
        let listener = ListenerConfig::new();
        let elevation = calculate_elevation(&source, &listener);
        assert!(elevation.abs() < 0.01);
    }

    #[test]
    fn test_elevation_effect_attenuation() {
        // Sound directly above should have reduced volume
        let (volume, _) = calculate_elevation_effect(PI / 2.0);
        assert!(volume < 1.0);
        assert!(volume >= 0.5);
    }

    #[test]
    fn test_elevation_effect_pan_reduction() {
        // Sound elevated should have pan pulled toward center
        let (_, pan_reduction) = calculate_elevation_effect(PI / 4.0);
        assert!(pan_reduction > 0.0);
        assert!(pan_reduction < 1.0);
    }

    #[test]
    fn test_sound_cone_inside() {
        let source_pos = Vec3::new(0.0, 0.0, 0.0);
        let listener_pos = Vec3::new(0.0, 0.0, 5.0); // In front
        let cone = SoundCone::new(Vec3::forward(), 60.0, 120.0, 0.3);

        let gain = calculate_cone_gain(&source_pos, &listener_pos, &cone);
        assert_eq!(gain, 1.0); // Full volume inside inner cone
    }

    #[test]
    fn test_sound_cone_outside() {
        let source_pos = Vec3::new(0.0, 0.0, 0.0);
        let listener_pos = Vec3::new(0.0, 0.0, -5.0); // Behind
        let cone = SoundCone::new(Vec3::forward(), 60.0, 120.0, 0.3);

        let gain = calculate_cone_gain(&source_pos, &listener_pos, &cone);
        assert_eq!(gain, 0.3); // Outer gain outside cone
    }

    #[test]
    fn test_sound_cone_transition() {
        let source_pos = Vec3::new(0.0, 0.0, 0.0);
        let listener_pos = Vec3::new(5.0, 0.0, 5.0); // 45 degrees to the right
        let cone = SoundCone::new(Vec3::forward(), 30.0, 90.0, 0.3);

        let gain = calculate_cone_gain(&source_pos, &listener_pos, &cone);
        assert!(gain > 0.3); // More than outer gain
        assert!(gain < 1.0); // Less than full volume
    }

    #[test]
    fn test_spatial_with_cone() {
        let source = SpatialPosition::new(0.0, 0.0, 5.0);
        let listener = ListenerConfig::new();
        let params = SpatialParams::default();
        let cone = SoundCone::narrow();

        let result = calculate_spatial_with_cone(&source, &listener, &params, Some(&cone), 0.0);

        assert!(result.volume > 0.0);
        assert_eq!(result.occlusion, 0.0);
    }

    #[test]
    fn test_occlusion() {
        let source = SpatialPosition::new(5.0, 0.0, 0.0);
        let listener = ListenerConfig::new();
        let params = SpatialParams::default();

        let result = calculate_spatial_with_cone(&source, &listener, &params, None, 0.7);

        assert_eq!(result.occlusion, 0.7);
    }

    #[test]
    fn test_occlusion_clamping() {
        let source = SpatialPosition::new(5.0, 0.0, 0.0);
        let listener = ListenerConfig::new();
        let params = SpatialParams::default();

        let result = calculate_spatial_with_cone(&source, &listener, &params, None, 1.5);

        assert_eq!(result.occlusion, 1.0); // Clamped to 1.0
    }

    // ========== SIMD Batch Processing Tests ==========

    #[test]
    fn test_batch_distance_squared() {
        let sources_x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let sources_y = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let sources_z = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let listener = Vec3::zero();

        let mut distances_sq = vec![0.0; 8];
        batch_distance_squared(&sources_x, &sources_y, &sources_z, &listener, &mut distances_sq);

        // Verify distances: d² = x²
        for i in 0..8 {
            let expected = (i + 1) as f32 * (i + 1) as f32;
            assert!((distances_sq[i] - expected).abs() < 0.001);
        }
    }

    #[test]
    fn test_batch_distance_squared_3d() {
        // 3-4-5 triangle in 3D (distance = 5)
        let sources_x = vec![3.0, 3.0, 3.0, 3.0];
        let sources_y = vec![4.0, 4.0, 4.0, 4.0];
        let sources_z = vec![0.0, 0.0, 0.0, 0.0];
        let listener = Vec3::zero();

        let mut distances_sq = vec![0.0; 4];
        batch_distance_squared(&sources_x, &sources_y, &sources_z, &listener, &mut distances_sq);

        for dist_sq in distances_sq.iter() {
            assert!((*dist_sq - 25.0).abs() < 0.001); // 3² + 4² = 25
        }
    }

    #[test]
    fn test_batch_attenuation() {
        let distances = vec![1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0];
        let ref_distance = 1.0;
        let mut attenuations = vec![0.0; 8];

        batch_attenuation_inverse_square(&distances, ref_distance, &mut attenuations);

        // Inverse square: (1/d)²
        assert!((attenuations[0] - 1.0).abs() < 0.001); // 1/1² = 1.0
        assert!((attenuations[1] - 0.25).abs() < 0.001); // 1/2² = 0.25
        assert!((attenuations[2] - 0.0625).abs() < 0.001); // 1/4² = 0.0625
        assert!((attenuations[3] - 0.015625).abs() < 0.001); // 1/8² = 0.015625
    }

    #[test]
    fn test_batch_attenuation_near_reference() {
        // Distances less than reference should clamp to 1.0
        let distances = vec![0.5, 0.75, 1.0, 1.5];
        let ref_distance = 1.0;
        let mut attenuations = vec![0.0; 4];

        batch_attenuation_inverse_square(&distances, ref_distance, &mut attenuations);

        assert_eq!(attenuations[0], 1.0); // < ref_dist, clamped
        assert_eq!(attenuations[1], 1.0); // < ref_dist, clamped
        assert_eq!(attenuations[2], 1.0); // == ref_dist
        assert!((attenuations[3] - 0.444).abs() < 0.01); // > ref_dist
    }
}
