//! Drum synthesis and drum machine sounds
//!
//! This module provides a comprehensive collection of drum sounds including:
//! - Acoustic drum kits
//! - Electronic drum machines (808, 909)
//! - Cymbals and hi-hats
//! - Hand percussion and ethnic drums
//! - Auxiliary percussion
//! - Special effects

/// Fast deterministic noise generator for drum synthesis
/// Uses a hash-like function to generate pseudo-random values from a seed
/// This is much faster than thread_rng() and produces consistent, high-quality noise
/// Returns values in the range [-1.0, 1.0]
fn noise(seed: f32) -> f32 {
    // Classic GLSL-style hash function
    // fract(sin(x) * large_number) produces pseudo-random values in [0, 1]
    // Use abs() to ensure positive value before fract()
    let hash = ((seed * 12.9898).sin() * 43758.55).abs().fract();

    // Map from [0, 1] to [-1, 1]
    hash * 2.0 - 1.0
}

pub mod acoustic;
pub mod electronic;
pub mod cymbals;
pub mod hand_percussion;
pub mod auxiliary;
pub mod effects;

// Re-export all sample functions for internal use
pub(super) use acoustic::*;
pub(super) use electronic::*;
pub(super) use cymbals::*;
pub(super) use hand_percussion::*;
pub(super) use auxiliary::*;
pub(super) use effects::*;

/// Drum types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrumType {
    Kick,
    Kick808, // Long, pitched 808 kick
    SubKick, // Ultra-low sub kick
    Snare,
    Snare808, // 808 snare (dual triangle oscillators)
    HiHatClosed,
    HiHatOpen,
    HiHat808Closed, // 808 closed hi-hat (6 square oscillators)
    HiHat808Open,   // 808 open hi-hat (6 square oscillators)
    Clap,
    Clap808, // 808 clap (multiple noise bursts)
    Tom,     // Mid tom (original)
    TomHigh, // High tom
    TomLow,  // Low tom
    Rimshot,
    Cowbell,
    Crash,
    Ride,
    China,  // China cymbal
    Splash, // Splash cymbal
    Tambourine,
    Shaker,
    BassDrop, // Dramatic bass drop impact
    Boom,     // Deep cinematic boom
    // Simple percussion
    Claves,    // Sharp wooden click
    Triangle,  // Metallic ding
    SideStick, // Soft rim click
    WoodBlock, // Dry, pitched click
    // 909 electronic drums
    Kick909,  // Punchier electronic kick
    Snare909, // Brighter electronic snare
    // Latin percussion
    CongaHigh, // Bright, high-pitched hand drum
    CongaLow,  // Deep, resonant bass conga
    BongoHigh, // Sharp, articulate bongo
    BongoLow,  // Deeper bongo
    // Utility
    RideBell, // Metallic ping
    // Additional toms
    FloorTomLow,  // Deep floor tom
    FloorTomHigh, // Higher floor tom
    // Additional hi-hat
    HiHatPedal, // Pedal hi-hat chick
    // Additional cymbals
    Crash2, // Second crash cymbal
    // Special effects
    Vibraslap, // Rattling/buzzing percussion
    // Additional Latin percussion
    TimbaleHigh, // High timbale (metallic shell)
    TimbaleLow,  // Low timbale
    AgogoHigh,   // High agogo bell (Brazilian)
    AgogoLow,    // Low agogo bell
    // Additional shakers/scrapers
    Cabasa,     // Textured shaker/scraper
    GuiroShort, // Short scraping sound
    GuiroLong,  // Long scraping sound
    // Additional wood percussion
    WoodBlockHigh, // High-pitched wooden click
    // Orchestral percussion
    Timpani, // Tuned orchestral bass drum
    Gong,    // Deep metallic crash
    Chimes,  // Tubular bells/chimes
    // World percussion
    Djembe,     // West African hand drum
    TablaBayan, // Indian bass drum (left hand)
    TablaDayan, // Indian treble drum (right hand)
    Cajon,      // Box drum
    // Hand percussion
    Fingersnap,  // Fingersnap sound
    Maracas,     // Rattling shaker
    Castanet,    // Spanish wooden clapper
    SleighBells, // Jingle bells
    // Electronic / Effects
    LaserZap,      // Sci-fi laser sound
    ReverseCymbal, // Reverse crash buildup
    WhiteNoiseHit, // Noise burst/clap
    StickClick,    // Drumstick click
    // Kick variations
    KickTight,    // Short, punchy kick
    KickDeep,     // Extended low-end
    KickAcoustic, // Natural drum kit
    KickClick,    // Prominent beater attack
    // Snare variations
    SnareRim,     // Rim-focused
    SnareTight,   // Short, dry
    SnareLoose,   // Longer ring
    SnarePiccolo, // High-pitched, bright
    // Hi-hat variations
    HiHatHalfOpen, // Between closed and open
    HiHatSizzle,   // High-frequency content
    // Clap variations
    ClapDry,   // No reverb, tight
    ClapRoom,  // Natural room ambience
    ClapGroup, // Multiple claps layered
    ClapSnare, // Hybrid clap/snare
    // Cymbal variations
    CrashShort, // Quick crash, gated
    RideTip,    // Bell-less ride
    // Shaker variations
    EggShaker,  // Tight, short shake
    TubeShaker, // Longer, sustained
    // 808 Kit Completion
    Tom808Low,  // Deep 808 tom
    Tom808Mid,  // Mid 808 tom
    Tom808High, // High 808 tom
    Cowbell808, // Iconic 808 cowbell
    Clave808,   // Sharp 808 clave
    // 909 Kit Completion
    HiHat909Closed, // Bright 909 closed hat
    HiHat909Open,   // Sustained 909 open hat
    Clap909,        // Classic 909 clap
    Cowbell909,     // Sharp 909 cowbell
    Rim909,         // 909 rim shot
    // Transition Effects
    ReverseSnare, // Snare buildup effect
    CymbalSwell,  // Building cymbal wash
}

impl DrumType {
    pub fn sample(&self, sample_index: usize, sample_rate: f32) -> f32 {
        match self {
            DrumType::Kick => kick_drum_sample(sample_index, sample_rate),
            DrumType::Kick808 => kick_808_sample(sample_index, sample_rate),
            DrumType::SubKick => sub_kick_sample(sample_index, sample_rate),
            DrumType::Snare => snare_drum_sample(sample_index, sample_rate),
            DrumType::Snare808 => snare_808_sample(sample_index, sample_rate),
            DrumType::HiHatClosed => hihat_sample(sample_index, sample_rate, true),
            DrumType::HiHatOpen => hihat_sample(sample_index, sample_rate, false),
            DrumType::HiHat808Closed => hihat_808_sample(sample_index, sample_rate, true),
            DrumType::HiHat808Open => hihat_808_sample(sample_index, sample_rate, false),
            DrumType::Clap => clap_sample(sample_index, sample_rate),
            DrumType::Clap808 => clap_808_sample(sample_index, sample_rate),
            DrumType::Tom => tom_sample(sample_index, sample_rate),
            DrumType::TomHigh => tom_high_sample(sample_index, sample_rate),
            DrumType::TomLow => tom_low_sample(sample_index, sample_rate),
            DrumType::Rimshot => rimshot_sample(sample_index, sample_rate),
            DrumType::Cowbell => cowbell_sample(sample_index, sample_rate),
            DrumType::Crash => crash_sample(sample_index, sample_rate),
            DrumType::Ride => ride_sample(sample_index, sample_rate),
            DrumType::China => china_sample(sample_index, sample_rate),
            DrumType::Splash => splash_sample(sample_index, sample_rate),
            DrumType::Tambourine => tambourine_sample(sample_index, sample_rate),
            DrumType::Shaker => shaker_sample(sample_index, sample_rate),
            DrumType::BassDrop => bass_drop_sample(sample_index, sample_rate),
            DrumType::Boom => boom_sample(sample_index, sample_rate),
            DrumType::Claves => claves_sample(sample_index, sample_rate),
            DrumType::Triangle => triangle_sample(sample_index, sample_rate),
            DrumType::SideStick => side_stick_sample(sample_index, sample_rate),
            DrumType::WoodBlock => wood_block_sample(sample_index, sample_rate),
            DrumType::Kick909 => kick_909_sample(sample_index, sample_rate),
            DrumType::Snare909 => snare_909_sample(sample_index, sample_rate),
            DrumType::CongaHigh => conga_high_sample(sample_index, sample_rate),
            DrumType::CongaLow => conga_low_sample(sample_index, sample_rate),
            DrumType::BongoHigh => bongo_high_sample(sample_index, sample_rate),
            DrumType::BongoLow => bongo_low_sample(sample_index, sample_rate),
            DrumType::RideBell => ride_bell_sample(sample_index, sample_rate),
            DrumType::FloorTomLow => floor_tom_low_sample(sample_index, sample_rate),
            DrumType::FloorTomHigh => floor_tom_high_sample(sample_index, sample_rate),
            DrumType::HiHatPedal => hihat_pedal_sample(sample_index, sample_rate),
            DrumType::Crash2 => crash2_sample(sample_index, sample_rate),
            DrumType::Vibraslap => vibraslap_sample(sample_index, sample_rate),
            DrumType::TimbaleHigh => timbale_high_sample(sample_index, sample_rate),
            DrumType::TimbaleLow => timbale_low_sample(sample_index, sample_rate),
            DrumType::AgogoHigh => agogo_high_sample(sample_index, sample_rate),
            DrumType::AgogoLow => agogo_low_sample(sample_index, sample_rate),
            DrumType::Cabasa => cabasa_sample(sample_index, sample_rate),
            DrumType::GuiroShort => guiro_short_sample(sample_index, sample_rate),
            DrumType::GuiroLong => guiro_long_sample(sample_index, sample_rate),
            DrumType::WoodBlockHigh => wood_block_high_sample(sample_index, sample_rate),
            DrumType::Timpani => timpani_sample(sample_index, sample_rate),
            DrumType::Gong => gong_sample(sample_index, sample_rate),
            DrumType::Chimes => chimes_sample(sample_index, sample_rate),
            DrumType::Djembe => djembe_sample(sample_index, sample_rate),
            DrumType::TablaBayan => tabla_bayan_sample(sample_index, sample_rate),
            DrumType::TablaDayan => tabla_dayan_sample(sample_index, sample_rate),
            DrumType::Cajon => cajon_sample(sample_index, sample_rate),
            DrumType::Fingersnap => fingersnap_sample(sample_index, sample_rate),
            DrumType::Maracas => maracas_sample(sample_index, sample_rate),
            DrumType::Castanet => castanet_sample(sample_index, sample_rate),
            DrumType::SleighBells => sleigh_bells_sample(sample_index, sample_rate),
            DrumType::LaserZap => laser_zap_sample(sample_index, sample_rate),
            DrumType::ReverseCymbal => reverse_cymbal_sample(sample_index, sample_rate),
            DrumType::WhiteNoiseHit => white_noise_hit_sample(sample_index, sample_rate),
            DrumType::StickClick => stick_click_sample(sample_index, sample_rate),
            DrumType::KickTight => kick_tight_sample(sample_index, sample_rate),
            DrumType::KickDeep => kick_deep_sample(sample_index, sample_rate),
            DrumType::KickAcoustic => kick_acoustic_sample(sample_index, sample_rate),
            DrumType::KickClick => kick_click_sample(sample_index, sample_rate),
            DrumType::SnareRim => snare_rim_sample(sample_index, sample_rate),
            DrumType::SnareTight => snare_tight_sample(sample_index, sample_rate),
            DrumType::SnareLoose => snare_loose_sample(sample_index, sample_rate),
            DrumType::SnarePiccolo => snare_piccolo_sample(sample_index, sample_rate),
            DrumType::HiHatHalfOpen => hihat_half_open_sample(sample_index, sample_rate),
            DrumType::HiHatSizzle => hihat_sizzle_sample(sample_index, sample_rate),
            DrumType::ClapDry => clap_dry_sample(sample_index, sample_rate),
            DrumType::ClapRoom => clap_room_sample(sample_index, sample_rate),
            DrumType::ClapGroup => clap_group_sample(sample_index, sample_rate),
            DrumType::ClapSnare => clap_snare_sample(sample_index, sample_rate),
            DrumType::CrashShort => crash_short_sample(sample_index, sample_rate),
            DrumType::RideTip => ride_tip_sample(sample_index, sample_rate),
            DrumType::EggShaker => egg_shaker_sample(sample_index, sample_rate),
            DrumType::TubeShaker => tube_shaker_sample(sample_index, sample_rate),
            DrumType::Tom808Low => tom_808_low_sample(sample_index, sample_rate),
            DrumType::Tom808Mid => tom_808_mid_sample(sample_index, sample_rate),
            DrumType::Tom808High => tom_808_high_sample(sample_index, sample_rate),
            DrumType::Cowbell808 => cowbell_808_sample(sample_index, sample_rate),
            DrumType::Clave808 => clave_808_sample(sample_index, sample_rate),
            DrumType::HiHat909Closed => hihat_909_closed_sample(sample_index, sample_rate),
            DrumType::HiHat909Open => hihat_909_open_sample(sample_index, sample_rate),
            DrumType::Clap909 => clap_909_sample(sample_index, sample_rate),
            DrumType::Cowbell909 => cowbell_909_sample(sample_index, sample_rate),
            DrumType::Rim909 => rim_909_sample(sample_index, sample_rate),
            DrumType::ReverseSnare => reverse_snare_sample(sample_index, sample_rate),
            DrumType::CymbalSwell => cymbal_swell_sample(sample_index, sample_rate),
        }
    }

    pub fn duration(&self) -> f32 {
        match self {
            DrumType::Kick => 0.15,
            DrumType::Kick808 => 0.5,
            DrumType::SubKick => 0.4,
            DrumType::Snare => 0.1,
            DrumType::Snare808 => 0.15,
            DrumType::HiHatClosed => 0.05,
            DrumType::HiHatOpen => 0.15,
            DrumType::HiHat808Closed => 0.04,
            DrumType::HiHat808Open => 0.12,
            DrumType::Clap => 0.08,
            DrumType::Clap808 => 0.1,
            DrumType::Tom => 0.3,
            DrumType::TomHigh => 0.25,
            DrumType::TomLow => 0.35,
            DrumType::Rimshot => 0.05,
            DrumType::Cowbell => 0.2,
            DrumType::Crash => 1.5,
            DrumType::Ride => 0.8,
            DrumType::China => 1.2,
            DrumType::Splash => 0.4,
            DrumType::Tambourine => 0.2,
            DrumType::Shaker => 0.15,
            DrumType::BassDrop => 0.8,
            DrumType::Boom => 1.0,
            DrumType::Claves => 0.02,
            DrumType::Triangle => 1.5,
            DrumType::SideStick => 0.04,
            DrumType::WoodBlock => 0.05,
            DrumType::Kick909 => 0.15,
            DrumType::Snare909 => 0.1,
            DrumType::CongaHigh => 0.2,
            DrumType::CongaLow => 0.3,
            DrumType::BongoHigh => 0.15,
            DrumType::BongoLow => 0.2,
            DrumType::RideBell => 0.6,
            DrumType::FloorTomLow => 0.4,
            DrumType::FloorTomHigh => 0.35,
            DrumType::HiHatPedal => 0.08,
            DrumType::Crash2 => 1.8,
            DrumType::Vibraslap => 0.15,
            DrumType::TimbaleHigh => 0.25,
            DrumType::TimbaleLow => 0.3,
            DrumType::AgogoHigh => 0.4,
            DrumType::AgogoLow => 0.5,
            DrumType::Cabasa => 0.25,
            DrumType::GuiroShort => 0.08,
            DrumType::GuiroLong => 0.2,
            DrumType::WoodBlockHigh => 0.06,
            DrumType::Timpani => 1.2,
            DrumType::Gong => 3.5,
            DrumType::Chimes => 2.0,
            DrumType::Djembe => 0.4,
            DrumType::TablaBayan => 0.5,
            DrumType::TablaDayan => 0.3,
            DrumType::Cajon => 0.25,
            DrumType::Fingersnap => 0.08,
            DrumType::Maracas => 0.12,
            DrumType::Castanet => 0.06,
            DrumType::SleighBells => 0.8,
            DrumType::LaserZap => 0.3,
            DrumType::ReverseCymbal => 1.5,
            DrumType::WhiteNoiseHit => 0.12,
            DrumType::StickClick => 0.03,
            DrumType::KickTight => 0.06,
            DrumType::KickDeep => 0.5,
            DrumType::KickAcoustic => 0.25,
            DrumType::KickClick => 0.12,
            DrumType::SnareRim => 0.08,
            DrumType::SnareTight => 0.07,
            DrumType::SnareLoose => 0.18,
            DrumType::SnarePiccolo => 0.08,
            DrumType::HiHatHalfOpen => 0.1,
            DrumType::HiHatSizzle => 0.2,
            DrumType::ClapDry => 0.05,
            DrumType::ClapRoom => 0.15,
            DrumType::ClapGroup => 0.12,
            DrumType::ClapSnare => 0.1,
            DrumType::CrashShort => 0.5,
            DrumType::RideTip => 0.6,
            DrumType::EggShaker => 0.08,
            DrumType::TubeShaker => 0.25,
            DrumType::Tom808Low => 0.4,
            DrumType::Tom808Mid => 0.35,
            DrumType::Tom808High => 0.3,
            DrumType::Cowbell808 => 0.3,
            DrumType::Clave808 => 0.025,
            DrumType::HiHat909Closed => 0.05,
            DrumType::HiHat909Open => 0.18,
            DrumType::Clap909 => 0.1,
            DrumType::Cowbell909 => 0.25,
            DrumType::Rim909 => 0.06,
            DrumType::ReverseSnare => 1.2,
            DrumType::CymbalSwell => 2.0,
        }
    }
}
