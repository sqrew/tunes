//! MIDI conversion utilities
//!
//! This module provides functions for converting between Tunes internal
//! representations and MIDI values (note numbers, velocities, ticks, etc.)

use crate::instruments::drums::DrumType;

/// MIDI ticks per quarter note (standard resolution)
pub const PPQ: u16 = 480;

/// Default MIDI velocity for notes without explicit velocity
pub const DEFAULT_VELOCITY: u8 = 80;

/// Convert frequency (Hz) to MIDI note number
///
/// Uses equal temperament tuning: MIDI note = 69 + 12 * log2(freq / 440)
/// Returns 0-127, clamped to valid MIDI range.
///
/// # Examples
/// ```
/// # use tunes::midi::frequency_to_midi_note;
/// assert_eq!(frequency_to_midi_note(440.0), 69); // A4
/// assert_eq!(frequency_to_midi_note(261.63), 60); // C4
/// ```
pub fn frequency_to_midi_note(freq: f32) -> u8 {
    if freq <= 0.0 {
        return 0;
    }

    // MIDI note number = 69 + 12 * log2(freq / 440)
    let note = 69.0 + 12.0 * (freq / 440.0).log2();
    note.round().clamp(0.0, 127.0) as u8
}

/// Convert MIDI note number to frequency (Hz)
///
/// Uses equal temperament tuning: freq = 440 * 2^((note - 69) / 12)
///
/// # Examples
/// ```
/// # use tunes::midi::midi_note_to_frequency;
/// assert_eq!(midi_note_to_frequency(69), 440.0); // A4
/// assert!((midi_note_to_frequency(60) - 261.63).abs() < 0.01); // C4
/// ```
pub fn midi_note_to_frequency(note: u8) -> f32 {
    // freq = 440 * 2^((note - 69) / 12)
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}

/// Convert time in seconds to MIDI ticks
///
/// # Arguments
/// * `time` - Time in seconds
/// * `tempo` - Tempo in BPM
/// * `ppq` - Pulses per quarter note (ticks per beat)
pub fn seconds_to_ticks(time: f32, tempo: f32, ppq: u16) -> u32 {
    // Beats = time * (bpm / 60)
    // Ticks = beats * ppq
    let beats = time * (tempo / 60.0);
    let ticks = beats * ppq as f32;
    ticks.round() as u32
}

/// Convert MIDI ticks to time in seconds
///
/// # Arguments
/// * `ticks` - MIDI ticks
/// * `tempo` - Tempo in BPM
/// * `ppq` - Pulses per quarter note (ticks per beat)
pub fn ticks_to_seconds(ticks: u32, tempo: f32, ppq: u16) -> f32 {
    // Beats = ticks / ppq
    // Time = beats / (bpm / 60)
    let beats = ticks as f32 / ppq as f32;
    beats / (tempo / 60.0)
}

/// Helper struct to track tempo changes and convert time to ticks accurately
///
/// This struct maintains a sorted list of tempo changes and calculates MIDI ticks
/// by integrating through tempo segments, ensuring accurate timing even with
/// multiple tempo changes.
pub struct TempoMap {
    changes: Vec<(f32, f32)>, // (time in seconds, bpm)
    ppq: u16,
}

impl TempoMap {
    /// Create a new TempoMap with an initial tempo
    pub fn new(initial_bpm: f32, ppq: u16) -> Self {
        Self {
            changes: vec![(0.0, initial_bpm)],
            ppq,
        }
    }

    /// Add a tempo change at a specific time
    pub fn add_change(&mut self, time: f32, bpm: f32) {
        self.changes.push((time, bpm));
    }

    /// Sort tempo changes by time (must be called after all changes are added)
    pub fn finalize(&mut self) {
        self.changes
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        // Remove duplicates at same time (keep last)
        self.changes.dedup_by(|a, b| (a.0 - b.0).abs() < 0.001);
    }

    /// Convert time in seconds to MIDI ticks, accounting for tempo changes
    ///
    /// This method integrates through tempo segments:
    /// - For each tempo segment, calculate ticks for time spent in that segment
    /// - Sum up ticks from all segments up to the target time
    ///
    /// # Example
    /// ```text
    /// Tempo changes: [(0.0, 120.0), (2.0, 60.0)]
    /// Converting 3.0 seconds:
    /// - First 2 seconds at 120 BPM = 1920 ticks
    /// - Next 1 second at 60 BPM = 480 ticks
    /// - Total = 2400 ticks
    /// ```
    pub fn seconds_to_ticks(&self, time: f32) -> u32 {
        if time <= 0.0 {
            return 0;
        }

        let mut accumulated_ticks = 0u32;
        let mut prev_time = 0.0;
        let mut prev_bpm = self.changes[0].1;

        for &(change_time, change_bpm) in &self.changes {
            if change_time >= time {
                // Target time is before this tempo change
                // Calculate ticks from prev_time to time at prev_bpm
                let duration = time - prev_time;
                accumulated_ticks += seconds_to_ticks(duration, prev_bpm, self.ppq);
                return accumulated_ticks;
            }

            // Calculate ticks from prev_time to change_time at prev_bpm
            let duration = change_time - prev_time;
            accumulated_ticks += seconds_to_ticks(duration, prev_bpm, self.ppq);

            prev_time = change_time;
            prev_bpm = change_bpm;
        }

        // Time is after all tempo changes - use last tempo
        let duration = time - prev_time;
        accumulated_ticks += seconds_to_ticks(duration, prev_bpm, self.ppq);
        accumulated_ticks
    }
}

/// Convert DrumType to General MIDI percussion note number
///
/// General MIDI defines percussion on channel 10 with specific note numbers.
/// See: https://en.wikipedia.org/wiki/General_MIDI#Percussion
pub fn drum_type_to_midi_note(drum_type: DrumType) -> u8 {
    match drum_type {
        // Kick drums
        DrumType::Kick => 36,    // Bass Drum 1
        DrumType::Kick808 => 35, // Acoustic Bass Drum
        DrumType::SubKick => 35, // Acoustic Bass Drum

        // Snare drums
        DrumType::Snare => 38,    // Acoustic Snare
        DrumType::Snare808 => 40, // Electric Snare

        // Hi-hats
        DrumType::HiHatClosed => 42,    // Closed Hi-Hat
        DrumType::HiHat808Closed => 42, // Closed Hi-Hat
        DrumType::HiHatOpen => 46,      // Open Hi-Hat
        DrumType::HiHat808Open => 46,   // Open Hi-Hat

        // Claps
        DrumType::Clap => 39,    // Hand Clap
        DrumType::Clap808 => 39, // Hand Clap

        // Toms
        DrumType::Tom => 47,     // Low-Mid Tom
        DrumType::TomHigh => 50, // High Tom
        DrumType::TomLow => 45,  // Low Tom

        // Percussion
        DrumType::Rimshot => 37, // Side Stick
        DrumType::Cowbell => 56, // Cowbell

        // Cymbals
        DrumType::Crash => 49,  // Crash Cymbal 1
        DrumType::Ride => 51,   // Ride Cymbal 1
        DrumType::China => 52,  // Chinese Cymbal
        DrumType::Splash => 55, // Splash Cymbal

        // Shakers/Percussion
        DrumType::Tambourine => 54, // Tambourine
        DrumType::Shaker => 70,     // Maracas

        // Special effects (map to toms as fallback)
        DrumType::BassDrop => 35, // Acoustic Bass Drum
        DrumType::Boom => 35,     // Acoustic Bass Drum

        // Simple percussion
        DrumType::Claves => 75,    // Claves
        DrumType::Triangle => 81,  // Open Triangle
        DrumType::SideStick => 37, // Side Stick
        DrumType::WoodBlock => 77, // Low Wood Block

        // 909 electronic drums
        DrumType::Kick909 => 36,  // Bass Drum 1
        DrumType::Snare909 => 40, // Electric Snare

        // Latin percussion
        DrumType::CongaHigh => 62, // Mute Hi Conga
        DrumType::CongaLow => 64,  // Low Conga
        DrumType::BongoHigh => 60, // Hi Bongo
        DrumType::BongoLow => 61,  // Low Bongo

        // Utility
        DrumType::RideBell => 53, // Ride Bell

        // Additional toms
        DrumType::FloorTomLow => 41,  // Low Floor Tom
        DrumType::FloorTomHigh => 43, // High Floor Tom

        // Additional hi-hat
        DrumType::HiHatPedal => 44, // Pedal Hi-Hat

        // Additional cymbals
        DrumType::Crash2 => 57, // Crash Cymbal 2

        // Special effects
        DrumType::Vibraslap => 58, // Vibraslap

        // Additional Latin percussion
        DrumType::TimbaleHigh => 65, // High Timbale
        DrumType::TimbaleLow => 66,  // Low Timbale
        DrumType::AgogoHigh => 67,   // High Agogo
        DrumType::AgogoLow => 68,    // Low Agogo

        // Additional shakers/scrapers
        DrumType::Cabasa => 69,     // Cabasa
        DrumType::GuiroShort => 73, // Short Guiro
        DrumType::GuiroLong => 74,  // Long Guiro

        // Additional wood percussion
        DrumType::WoodBlockHigh => 76, // Hi Wood Block

        // Orchestral percussion (no direct GM mapping, use approximations)
        DrumType::Timpani => 47, // Low-Mid Tom (closest approximation)
        DrumType::Gong => 52,    // Chinese Cymbal
        DrumType::Chimes => 84,  // Belltree (GM note 84)

        // World percussion (no direct GM mapping)
        DrumType::Djembe => 60,     // Hi Bongo (similar hand drum)
        DrumType::TablaBayan => 58, // Vibraslap (as placeholder)
        DrumType::TablaDayan => 77, // Low Wood Block (sharp attack)
        DrumType::Cajon => 38,      // Acoustic Snare (similar character)

        // Hand percussion
        DrumType::Fingersnap => 37,  // Side Stick (similar click)
        DrumType::Maracas => 70,     // Maracas (GM standard)
        DrumType::Castanet => 85,    // Castanets (GM note 85)
        DrumType::SleighBells => 83, // Jingle Bell (GM note 83)

        // Electronic / Effects (no GM equivalents, use generic)
        DrumType::LaserZap => 35,      // Bass Drum (placeholder)
        DrumType::ReverseCymbal => 49, // Crash Cymbal
        DrumType::WhiteNoiseHit => 39, // Hand Clap
        DrumType::StickClick => 37,    // Side Stick

        // Kick variations (all map to kick notes)
        DrumType::KickTight => 36,    // Bass Drum 1
        DrumType::KickDeep => 35,     // Acoustic Bass Drum
        DrumType::KickAcoustic => 36, // Bass Drum 1
        DrumType::KickClick => 36,    // Bass Drum 1

        // Snare variations (all map to snare notes)
        DrumType::SnareRim => 37,     // Side Stick (rim sound)
        DrumType::SnareTight => 38,   // Acoustic Snare
        DrumType::SnareLoose => 38,   // Acoustic Snare
        DrumType::SnarePiccolo => 40, // Electric Snare

        // Hi-hat variations
        DrumType::HiHatHalfOpen => 46, // Open Hi-Hat
        DrumType::HiHatSizzle => 46,   // Open Hi-Hat

        // Clap variations (all map to clap)
        DrumType::ClapDry => 39,   // Hand Clap
        DrumType::ClapRoom => 39,  // Hand Clap
        DrumType::ClapGroup => 39, // Hand Clap
        DrumType::ClapSnare => 39, // Hand Clap

        // Cymbal variations
        DrumType::CrashShort => 49, // Crash Cymbal 1
        DrumType::RideTip => 51,    // Ride Cymbal 1

        // Shaker variations
        DrumType::EggShaker => 70,  // Maracas
        DrumType::TubeShaker => 70, // Maracas

        // 808 Kit Completion
        DrumType::Tom808Low => 45,  // Low Tom
        DrumType::Tom808Mid => 47,  // Low-Mid Tom
        DrumType::Tom808High => 48, // Hi-Mid Tom
        DrumType::Cowbell808 => 56, // Cowbell
        DrumType::Clave808 => 75,   // Claves

        // 909 Kit Completion
        DrumType::HiHat909Closed => 42, // Closed Hi-Hat
        DrumType::HiHat909Open => 46,   // Open Hi-Hat
        DrumType::Clap909 => 39,        // Hand Clap
        DrumType::Cowbell909 => 56,     // Cowbell
        DrumType::Rim909 => 37,         // Side Stick

        // Transition Effects
        DrumType::ReverseSnare => 38, // Acoustic Snare
        DrumType::CymbalSwell => 55,  // Splash Cymbal
    }
}

/// Convert General MIDI percussion note number to DrumType
///
/// Maps standard MIDI percussion notes (channel 10) to tunes DrumType.
/// Returns None for unsupported MIDI percussion notes.
///
/// # Arguments
/// * `midi_note` - MIDI note number (typically 35-81 for GM percussion)
///
/// # Examples
/// ```
/// # use tunes::midi::midi_note_to_drum_type;
/// # use tunes::instruments::drums::DrumType;
/// assert_eq!(midi_note_to_drum_type(36), Some(DrumType::Kick));
/// assert_eq!(midi_note_to_drum_type(38), Some(DrumType::Snare));
/// assert_eq!(midi_note_to_drum_type(42), Some(DrumType::HiHatClosed));
/// ```
pub fn midi_note_to_drum_type(midi_note: u8) -> Option<DrumType> {
    match midi_note {
        // Kick drums
        35 => Some(DrumType::Kick808), // Acoustic Bass Drum
        36 => Some(DrumType::Kick),    // Bass Drum 1

        // Snare drums
        38 => Some(DrumType::Snare),    // Acoustic Snare
        40 => Some(DrumType::Snare808), // Electric Snare

        // Hi-hats
        42 => Some(DrumType::HiHatClosed), // Closed Hi-Hat
        46 => Some(DrumType::HiHatOpen),   // Open Hi-Hat

        // Claps and rimshots
        37 => Some(DrumType::Rimshot), // Side Stick
        39 => Some(DrumType::Clap),    // Hand Clap

        // Toms
        45 => Some(DrumType::TomLow),     // Low Tom
        47 => Some(DrumType::Tom),        // Low-Mid Tom
        48 => Some(DrumType::Tom808High), // Hi-Mid Tom (808 variant)
        50 => Some(DrumType::TomHigh),    // High Tom

        // Cymbals
        49 => Some(DrumType::Crash),  // Crash Cymbal 1
        51 => Some(DrumType::Ride),   // Ride Cymbal 1
        52 => Some(DrumType::China),  // Chinese Cymbal
        55 => Some(DrumType::Splash), // Splash Cymbal

        // Percussion
        54 => Some(DrumType::Tambourine), // Tambourine
        56 => Some(DrumType::Cowbell),    // Cowbell
        70 => Some(DrumType::Shaker),     // Maracas

        // Simple percussion
        75 => Some(DrumType::Claves),    // Claves
        77 => Some(DrumType::WoodBlock), // Low Wood Block
        81 => Some(DrumType::Triangle),  // Open Triangle

        // Latin percussion
        60 => Some(DrumType::BongoHigh), // Hi Bongo
        61 => Some(DrumType::BongoLow),  // Low Bongo
        62 => Some(DrumType::CongaHigh), // Mute Hi Conga
        64 => Some(DrumType::CongaLow),  // Low Conga

        // Ride bell
        53 => Some(DrumType::RideBell), // Ride Bell

        // Additional toms
        41 => Some(DrumType::FloorTomLow),  // Low Floor Tom
        43 => Some(DrumType::FloorTomHigh), // High Floor Tom

        // Additional hi-hat
        44 => Some(DrumType::HiHatPedal), // Pedal Hi-Hat

        // Additional cymbals
        57 => Some(DrumType::Crash2), // Crash Cymbal 2

        // Special effects
        58 => Some(DrumType::Vibraslap), // Vibraslap

        // Additional Latin percussion
        65 => Some(DrumType::TimbaleHigh), // High Timbale
        66 => Some(DrumType::TimbaleLow),  // Low Timbale
        67 => Some(DrumType::AgogoHigh),   // High Agogo
        68 => Some(DrumType::AgogoLow),    // Low Agogo

        // Additional shakers/scrapers
        69 => Some(DrumType::Cabasa),     // Cabasa
        73 => Some(DrumType::GuiroShort), // Short Guiro
        74 => Some(DrumType::GuiroLong),  // Long Guiro

        // Additional wood percussion
        76 => Some(DrumType::WoodBlockHigh), // Hi Wood Block

        // Additional GM percussion (orchestral/hand percussion)
        83 => Some(DrumType::SleighBells), // Jingle Bell
        84 => Some(DrumType::Chimes),      // Belltree
        85 => Some(DrumType::Castanet),    // Castanets

        // Unsupported MIDI percussion notes
        _ => None,
    }
}

/// Convert volume (0.0-1.0) to MIDI velocity (0-127)
pub fn volume_to_velocity(volume: f32) -> u8 {
    (volume.clamp(0.0, 1.0) * 127.0).round() as u8
}

/// Convert MIDI velocity (0-127) to volume (0.0-1.0)
pub fn velocity_to_volume(velocity: u8) -> f32 {
    velocity as f32 / 127.0
}

/// Convert pitch bend in semitones to MIDI pitch bend value (14-bit)
///
/// MIDI pitch bend is a 14-bit value (0-16383) with center at 8192.
/// Standard pitch bend range is ±2 semitones.
///
/// # Arguments
/// * `semitones` - Pitch bend amount in semitones (positive = up, negative = down)
/// * `range` - Pitch bend range in semitones (default is 2.0 for ±2 semitones)
///
/// # Returns
/// 14-bit pitch bend value (0-16383), clamped to valid range
pub fn semitones_to_pitch_bend(semitones: f32, range: f32) -> u16 {
    // Center value is 8192 (no bend)
    // Each semitone within the range corresponds to ±8192/range units
    let bend_value = 8192.0 + (semitones / range) * 8192.0;
    bend_value.round().clamp(0.0, 16383.0) as u16
}

/// Convert MIDI pitch bend value (14-bit) to semitones
///
/// MIDI pitch bend is a 14-bit value (0-16383) with center at 8192.
/// Standard pitch bend range is ±2 semitones.
///
/// Note: The midly library returns pitch bend as a signed i16 value
/// relative to center (-8192 to +8191), not the raw 14-bit value.
///
/// # Arguments
/// * `bend_value` - Pitch bend value from midly (signed, relative to center)
/// * `range` - Pitch bend range in semitones (default is 2.0 for ±2 semitones)
///
/// # Returns
/// Pitch bend in semitones (positive = up, negative = down)
pub fn pitch_bend_to_semitones_from_signed(bend_value: i16, range: f32) -> f32 {
    // midly returns pitch bend as signed value relative to center
    // -8192 to +8191, where 0 = center (no bend)
    (bend_value as f32 / 8192.0) * range
}

/// Convert a modulation LFO value to MIDI CC value (0-127)
///
/// For unipolar modulation (volume): 0.0 -> 0, 1.0 -> 127
/// For bipolar modulation (pitch, pan): -1.0 -> 0, 0.0 -> 64, 1.0 -> 127
pub fn mod_value_to_cc(value: f32, bipolar: bool) -> u8 {
    if bipolar {
        // Bipolar: -1.0 to 1.0 -> 0 to 127
        ((value + 1.0) * 63.5).round().clamp(0.0, 127.0) as u8
    } else {
        // Unipolar: 0.0 to 1.0 -> 0 to 127
        (value * 127.0).round().clamp(0.0, 127.0) as u8
    }
}

/// Map General MIDI program number (0-127) to an Instrument preset
///
/// This provides automatic instrument selection when importing MIDI files.
/// The mapping follows the General MIDI standard and uses the best available
/// preset from the library's 160+ instruments.
///
/// # General MIDI Program Categories
/// - 0-7: Piano
/// - 8-15: Chromatic Percussion
/// - 16-23: Organ
/// - 24-31: Guitar
/// - 32-39: Bass
/// - 40-47: Strings
/// - 48-55: Ensemble
/// - 56-63: Brass
/// - 64-71: Reed
/// - 72-79: Pipe
/// - 80-87: Synth Lead
/// - 88-95: Synth Pad
/// - 96-103: Synth Effects
/// - 104-111: Ethnic
/// - 112-119: Percussive
/// - 120-127: Sound Effects
pub fn gm_program_to_instrument(program: u8) -> crate::instruments::Instrument {
    use crate::instruments::Instrument;

    match program {
        // Piano (0-7)
        0 => Instrument::acoustic_piano(),      // Acoustic Grand Piano
        1 => Instrument::acoustic_piano(),      // Bright Acoustic Piano
        2 => Instrument::electric_piano(),      // Electric Grand Piano
        3 => Instrument::honky_tonk_piano(),    // Honky-tonk Piano
        4 => Instrument::electric_piano(),      // Electric Piano 1 (Rhodes)
        5 => Instrument::stage_73(),            // Electric Piano 2 (Chorus)
        6 => Instrument::harpsichord(),         // Harpsichord
        7 => Instrument::clavinet(),            // Clavinet

        // Chromatic Percussion (8-15)
        8 => Instrument::celesta(),             // Celesta
        9 => Instrument::glockenspiel(),        // Glockenspiel
        10 => Instrument::music_box(),          // Music Box
        11 => Instrument::vibraphone(),         // Vibraphone
        12 => Instrument::marimba(),            // Marimba
        13 => Instrument::xylophone(),          // Xylophone
        14 => Instrument::tubular_bells(),      // Tubular Bells
        15 => Instrument::dulcimer(),           // Dulcimer

        // Organ (16-23)
        16 => Instrument::hammond_organ(),      // Drawbar Organ
        17 => Instrument::organ(),              // Percussive Organ
        18 => Instrument::church_organ(),       // Rock Organ
        19 => Instrument::church_organ(),       // Church Organ
        20 => Instrument::reed_organ(),         // Reed Organ
        21 => Instrument::accordion(),          // Accordion
        22 => Instrument::accordion(),          // Harmonica
        23 => Instrument::accordion(),          // Tango Accordion

        // Guitar (24-31)
        24 => Instrument::acoustic_guitar(),    // Acoustic Guitar (nylon)
        25 => Instrument::acoustic_guitar(),    // Acoustic Guitar (steel)
        26 => Instrument::electric_guitar_clean(), // Electric Guitar (jazz)
        27 => Instrument::electric_guitar_clean(), // Electric Guitar (clean)
        28 => Instrument::guitar_palm_muted(),  // Electric Guitar (muted)
        29 => Instrument::electric_guitar_distorted(), // Overdriven Guitar
        30 => Instrument::electric_guitar_distorted(), // Distortion Guitar
        31 => Instrument::guitar_harmonics(),   // Guitar Harmonics

        // Bass (32-39)
        32 => Instrument::upright_bass(),       // Acoustic Bass
        33 => Instrument::fingerstyle_bass(),   // Electric Bass (finger)
        34 => Instrument::slap_bass(),          // Electric Bass (pick)
        35 => Instrument::fretless_bass(),      // Fretless Bass
        36 => Instrument::slap_bass(),          // Slap Bass 1
        37 => Instrument::slap_bass(),          // Slap Bass 2
        38 => Instrument::synth_bass(),         // Synth Bass 1
        39 => Instrument::synth_bass(),         // Synth Bass 2

        // Strings (40-47)
        40 => Instrument::violin(),             // Violin
        41 => Instrument::viola(),              // Viola
        42 => Instrument::cello(),              // Cello
        43 => Instrument::double_bass(),        // Contrabass
        44 => Instrument::tremolo_strings(),    // Tremolo Strings
        45 => Instrument::pizzicato_strings(),  // Pizzicato Strings
        46 => Instrument::harp(),               // Orchestral Harp
        47 => Instrument::timpani(),            // Timpani

        // Ensemble (48-55)
        48 => Instrument::strings(),            // String Ensemble 1
        49 => Instrument::slow_strings(),       // String Ensemble 2
        50 => Instrument::strings(),            // Synth Strings 1
        51 => Instrument::strings(),            // Synth Strings 2
        52 => Instrument::choir_aahs(),         // Choir Aahs
        53 => Instrument::choir_oohs(),         // Voice Oohs
        54 => Instrument::synth_voice(),        // Synth Voice
        55 => Instrument::strings(),            // Orchestra Hit

        // Brass (56-63)
        56 => Instrument::solo_trumpet(),       // Trumpet
        57 => Instrument::trombone(),           // Trombone
        58 => Instrument::tuba(),               // Tuba
        59 => Instrument::muted_trumpet(),      // Muted Trumpet
        60 => Instrument::french_horn(),        // French Horn
        61 => Instrument::brass_section(),      // Brass Section
        62 => Instrument::prophet_brass(),      // Synth Brass 1
        63 => Instrument::analog_brass(),       // Synth Brass 2

        // Reed (64-71)
        64 => Instrument::soprano_sax(),        // Soprano Sax
        65 => Instrument::alto_sax(),           // Alto Sax
        66 => Instrument::tenor_sax(),          // Tenor Sax
        67 => Instrument::baritone_sax(),       // Baritone Sax
        68 => Instrument::oboe(),               // Oboe
        69 => Instrument::english_horn(),       // English Horn
        70 => Instrument::bassoon(),            // Bassoon
        71 => Instrument::clarinet(),           // Clarinet

        // Pipe (72-79)
        72 => Instrument::piccolo(),            // Piccolo
        73 => Instrument::flute(),              // Flute
        74 => Instrument::flute(),              // Recorder
        75 => Instrument::pan_flute(),          // Pan Flute
        76 => Instrument::didgeridoo(),         // Blown Bottle
        77 => Instrument::shakuhachi(),         // Shakuhachi
        78 => Instrument::shakuhachi(),         // Whistle
        79 => Instrument::uilleann_pipes(),     // Ocarina

        // Synth Lead (80-87)
        80 => Instrument::square_lead(),        // Lead 1 (square)
        81 => Instrument::saw_lead(),           // Lead 2 (sawtooth)
        82 => Instrument::synth_voice(),        // Lead 3 (calliope)
        83 => Instrument::chiptune(),           // Lead 4 (chiff)
        84 => Instrument::synth_lead(),         // Lead 5 (charang)
        85 => Instrument::synth_voice(),        // Lead 6 (voice)
        86 => Instrument::supersaw(),           // Lead 7 (fifths)
        87 => Instrument::saw_lead(),           // Lead 8 (bass + lead)

        // Synth Pad (88-95)
        88 => Instrument::warm_pad(),           // Pad 1 (new age)
        89 => Instrument::ambient_pad(),        // Pad 2 (warm)
        90 => Instrument::vocal_pad(),          // Pad 3 (polysynth)
        91 => Instrument::choir_oohs(),         // Pad 4 (choir)
        92 => Instrument::shimmer_pad(),        // Pad 5 (bowed)
        93 => Instrument::ambient_pad(),        // Pad 6 (metallic)
        94 => Instrument::warm_pad(),           // Pad 7 (halo)
        95 => Instrument::juno_pad(),           // Pad 8 (sweep)

        // Synth Effects (96-103)
        96 => Instrument::cosmic_rays(),        // FX 1 (rain)
        97 => Instrument::wind_chimes(),        // FX 2 (soundtrack)
        98 => Instrument::glass_harmonica(),    // FX 3 (crystal)
        99 => Instrument::ambient_pad(),        // FX 4 (atmosphere)
        100 => Instrument::shimmer_pad(),       // FX 5 (brightness)
        101 => Instrument::granular_pad(),      // FX 6 (goblins)
        102 => Instrument::cosmic_rays(),       // FX 7 (echoes)
        103 => Instrument::glitch(),            // FX 8 (sci-fi)

        // Ethnic (104-111)
        104 => Instrument::sitar(),             // Sitar
        105 => Instrument::banjo(),             // Banjo
        106 => Instrument::shamisen(),          // Shamisen
        107 => Instrument::koto(),              // Koto
        108 => Instrument::kalimba(),           // Kalimba
        109 => Instrument::bagpipes(),          // Bag pipe
        110 => Instrument::erhu(),              // Fiddle
        111 => Instrument::duduk(),             // Shanai

        // Percussive (112-119)
        112 => Instrument::steel_drums(),       // Tinkle Bell
        113 => Instrument::cowbell(),           // Agogo
        114 => Instrument::steel_drums(),       // Steel Drums
        115 => Instrument::taiko_drum(),        // Woodblock
        116 => Instrument::taiko(),             // Taiko Drum
        117 => Instrument::timpani(),           // Melodic Tom
        118 => Instrument::djembe(),            // Synth Drum
        119 => Instrument::metallic_perc(),     // Reverse Cymbal

        // Sound Effects (120-127)
        120 => Instrument::guitar_harmonics(),  // Guitar Fret Noise
        121 => Instrument::glitch(),            // Breath Noise
        122 => Instrument::wind_chimes(),       // Seashore
        123 => Instrument::cosmic_rays(),       // Bird Tweet
        124 => Instrument::glitch(),            // Telephone Ring
        125 => Instrument::impact(),            // Helicopter
        126 => Instrument::riser(),             // Applause
        127 => Instrument::laser(),             // Gunshot

        // Default fallback (should never reach here)
        _ => Instrument::acoustic_piano(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_to_midi_note() {
        assert_eq!(frequency_to_midi_note(440.0), 69); // A4
        assert_eq!(frequency_to_midi_note(261.63), 60); // C4 (approximate)
        assert_eq!(frequency_to_midi_note(523.25), 72); // C5 (approximate)

        // Edge cases
        assert_eq!(frequency_to_midi_note(0.0), 0);
        assert_eq!(frequency_to_midi_note(-100.0), 0);
        assert_eq!(frequency_to_midi_note(20000.0), 127); // Clamps to max
    }

    #[test]
    fn test_seconds_to_ticks() {
        // At 120 BPM, 1 beat = 0.5 seconds
        // At 480 PPQ, 1 beat = 480 ticks
        // So 0.5 seconds = 480 ticks
        assert_eq!(seconds_to_ticks(0.5, 120.0, 480), 480);

        // 1 second = 2 beats = 960 ticks
        assert_eq!(seconds_to_ticks(1.0, 120.0, 480), 960);

        // At 60 BPM, 1 beat = 1 second = 480 ticks
        assert_eq!(seconds_to_ticks(1.0, 60.0, 480), 480);
    }

    #[test]
    fn test_drum_type_to_midi_note() {
        // Test a few standard mappings
        assert_eq!(drum_type_to_midi_note(DrumType::Kick), 36);
        assert_eq!(drum_type_to_midi_note(DrumType::Snare), 38);
        assert_eq!(drum_type_to_midi_note(DrumType::HiHatClosed), 42);
        assert_eq!(drum_type_to_midi_note(DrumType::HiHatOpen), 46);
        assert_eq!(drum_type_to_midi_note(DrumType::Clap), 39);
    }

    #[test]
    fn test_volume_to_velocity() {
        assert_eq!(volume_to_velocity(0.0), 0);
        assert_eq!(volume_to_velocity(1.0), 127);
        assert_eq!(volume_to_velocity(0.5), 64); // Approximate
        assert_eq!(volume_to_velocity(1.5), 127); // Clamps
        assert_eq!(volume_to_velocity(-0.5), 0); // Clamps
    }

    #[test]
    fn test_semitones_to_pitch_bend() {
        // Center (no bend) should be 8192
        assert_eq!(semitones_to_pitch_bend(0.0, 2.0), 8192);

        // +2 semitones (max of standard range) should be 16383
        assert_eq!(semitones_to_pitch_bend(2.0, 2.0), 16383);

        // -2 semitones (min of standard range) should be 0
        assert_eq!(semitones_to_pitch_bend(-2.0, 2.0), 0);

        // +1 semitone (half of range) should be halfway between center and max
        assert_eq!(semitones_to_pitch_bend(1.0, 2.0), 12288);

        // -1 semitone should be halfway between center and min
        assert_eq!(semitones_to_pitch_bend(-1.0, 2.0), 4096);

        // Test clamping - values beyond range should clamp
        assert_eq!(semitones_to_pitch_bend(10.0, 2.0), 16383); // Clamps to max
        assert_eq!(semitones_to_pitch_bend(-10.0, 2.0), 0); // Clamps to min
    }

    #[test]
    fn test_semitones_to_pitch_bend_different_range() {
        // Test with 12 semitone range (full octave)
        assert_eq!(semitones_to_pitch_bend(0.0, 12.0), 8192);
        assert_eq!(semitones_to_pitch_bend(12.0, 12.0), 16383);
        assert_eq!(semitones_to_pitch_bend(-12.0, 12.0), 0);
        assert_eq!(semitones_to_pitch_bend(6.0, 12.0), 12288);
    }

    #[test]
    fn test_pitch_bend_fractional_semitones() {
        // Test fractional semitones (for microtonal bends)
        let bend_quarter_tone = semitones_to_pitch_bend(0.5, 2.0);
        // Should be between center (8192) and +1 semitone (12288)
        assert!(bend_quarter_tone > 8192 && bend_quarter_tone < 12288);
        assert_eq!(bend_quarter_tone, 10240); // Exactly halfway

        let bend_eighth_tone = semitones_to_pitch_bend(0.25, 2.0);
        // Should be between center and quarter tone
        assert!(bend_eighth_tone > 8192 && bend_eighth_tone < bend_quarter_tone);
    }

    #[test]
    fn test_mod_value_to_cc_unipolar() {
        // Unipolar modulation (volume): 0.0 -> 0, 1.0 -> 127
        assert_eq!(mod_value_to_cc(0.0, false), 0);
        assert_eq!(mod_value_to_cc(1.0, false), 127);
        assert_eq!(mod_value_to_cc(0.5, false), 64);

        // Test clamping
        assert_eq!(mod_value_to_cc(-0.5, false), 0);
        assert_eq!(mod_value_to_cc(1.5, false), 127);
    }

    #[test]
    fn test_mod_value_to_cc_bipolar() {
        // Bipolar modulation (pitch, pan): -1.0 -> 0, 0.0 -> 64, 1.0 -> 127
        assert_eq!(mod_value_to_cc(-1.0, true), 0);
        assert_eq!(mod_value_to_cc(0.0, true), 64);
        assert_eq!(mod_value_to_cc(1.0, true), 127);

        // Test intermediate values
        assert_eq!(mod_value_to_cc(0.5, true), 95); // Halfway between 64 and 127
        assert_eq!(mod_value_to_cc(-0.5, true), 32); // Halfway between 0 and 64

        // Test clamping
        assert_eq!(mod_value_to_cc(-2.0, true), 0);
        assert_eq!(mod_value_to_cc(2.0, true), 127);
    }

    #[test]
    fn test_midi_note_to_frequency() {
        // Test standard notes
        assert_eq!(midi_note_to_frequency(69), 440.0); // A4
        assert!((midi_note_to_frequency(60) - 261.63).abs() < 0.01); // C4
        assert!((midi_note_to_frequency(72) - 523.25).abs() < 0.01); // C5

        // Test octave relationship (doubling frequency)
        let c4 = midi_note_to_frequency(60);
        let c5 = midi_note_to_frequency(72);
        assert!((c5 / c4 - 2.0).abs() < 0.001); // C5 should be double C4
    }

    #[test]
    fn test_midi_note_frequency_roundtrip() {
        // Test that converting back and forth works
        for midi_note in 21..108 {
            // Test range of a standard 88-key piano
            let freq = midi_note_to_frequency(midi_note);
            let converted_back = frequency_to_midi_note(freq);
            assert_eq!(converted_back, midi_note);
        }
    }

    #[test]
    fn test_ticks_to_seconds() {
        // At 120 BPM, 1 beat = 0.5 seconds
        // At 480 PPQ, 1 beat = 480 ticks
        // So 480 ticks = 0.5 seconds
        assert_eq!(ticks_to_seconds(480, 120.0, 480), 0.5);

        // 960 ticks = 1 second
        assert_eq!(ticks_to_seconds(960, 120.0, 480), 1.0);

        // At 60 BPM, 1 beat = 1 second = 480 ticks
        assert_eq!(ticks_to_seconds(480, 60.0, 480), 1.0);
    }

    #[test]
    fn test_ticks_seconds_roundtrip() {
        // Test that converting back and forth works
        let tempo = 120.0;
        let ppq = 480;

        for seconds in [0.25, 0.5, 1.0, 2.0, 4.0] {
            let ticks = seconds_to_ticks(seconds, tempo, ppq);
            let converted_back = ticks_to_seconds(ticks, tempo, ppq);
            assert!((converted_back - seconds).abs() < 0.001);
        }
    }

    #[test]
    fn test_midi_note_to_drum_type() {
        // Test standard drum mappings
        assert_eq!(midi_note_to_drum_type(36), Some(DrumType::Kick));
        assert_eq!(midi_note_to_drum_type(38), Some(DrumType::Snare));
        assert_eq!(midi_note_to_drum_type(42), Some(DrumType::HiHatClosed));
        assert_eq!(midi_note_to_drum_type(46), Some(DrumType::HiHatOpen));
        assert_eq!(midi_note_to_drum_type(39), Some(DrumType::Clap));
        assert_eq!(midi_note_to_drum_type(49), Some(DrumType::Crash));

        // Test unsupported notes
        assert_eq!(midi_note_to_drum_type(0), None);
        assert_eq!(midi_note_to_drum_type(100), None);
    }

    #[test]
    fn test_drum_type_midi_note_roundtrip() {
        // Test that common drums can round-trip
        let drums = [
            DrumType::Kick,
            DrumType::Snare,
            DrumType::HiHatClosed,
            DrumType::HiHatOpen,
            DrumType::Clap,
            DrumType::Crash,
            DrumType::Ride,
        ];

        for drum in drums {
            let midi_note = drum_type_to_midi_note(drum);
            let converted_back = midi_note_to_drum_type(midi_note);
            assert!(converted_back.is_some());
        }
    }

    #[test]
    fn test_tempo_map_single_tempo() {
        // Test TempoMap with no tempo changes (single tempo throughout)
        let tempo_map = TempoMap::new(120.0, 480);

        // At 120 BPM: 1 beat = 0.5 seconds = 480 ticks
        assert_eq!(tempo_map.seconds_to_ticks(0.0), 0);
        assert_eq!(tempo_map.seconds_to_ticks(0.5), 480); // 1 beat
        assert_eq!(tempo_map.seconds_to_ticks(1.0), 960); // 2 beats
        assert_eq!(tempo_map.seconds_to_ticks(2.0), 1920); // 4 beats
    }

    #[test]
    fn test_tempo_map_multiple_tempos() {
        // Test TempoMap with tempo changes
        let mut tempo_map = TempoMap::new(120.0, 480);

        // Add tempo change at 2 seconds: switch to 60 BPM
        tempo_map.add_change(2.0, 60.0);
        tempo_map.finalize();

        // First 2 seconds at 120 BPM:
        // - 2 seconds = 4 beats = 1920 ticks
        assert_eq!(tempo_map.seconds_to_ticks(0.0), 0);
        assert_eq!(tempo_map.seconds_to_ticks(0.5), 480); // 1 beat at 120 BPM
        assert_eq!(tempo_map.seconds_to_ticks(1.0), 960); // 2 beats at 120 BPM
        assert_eq!(tempo_map.seconds_to_ticks(2.0), 1920); // 4 beats at 120 BPM

        // After 2 seconds at 60 BPM:
        // - Base: 1920 ticks (from first 2 seconds)
        // - 1 second at 60 BPM = 1 beat = 480 ticks
        // - Total at 3 seconds: 1920 + 480 = 2400 ticks
        assert_eq!(tempo_map.seconds_to_ticks(3.0), 2400);

        // 2 seconds at 60 BPM = 2 beats = 960 ticks
        // Total at 4 seconds: 1920 + 960 = 2880 ticks
        assert_eq!(tempo_map.seconds_to_ticks(4.0), 2880);
    }

    #[test]
    fn test_tempo_map_complex_scenario() {
        // Test with multiple tempo changes
        let mut tempo_map = TempoMap::new(120.0, 480);

        // Add multiple tempo changes
        tempo_map.add_change(1.0, 90.0); // Switch to 90 BPM at 1 second
        tempo_map.add_change(3.0, 180.0); // Switch to 180 BPM at 3 seconds
        tempo_map.finalize();

        // First 1 second at 120 BPM:
        // - 1 second = 2 beats = 960 ticks
        assert_eq!(tempo_map.seconds_to_ticks(1.0), 960);

        // Next 2 seconds (1-3s) at 90 BPM:
        // - 2 seconds at 90 BPM = 3 beats = 1440 ticks
        // - Total at 3 seconds: 960 + 1440 = 2400 ticks
        assert_eq!(tempo_map.seconds_to_ticks(3.0), 2400);

        // Next 1 second (3-4s) at 180 BPM:
        // - 1 second at 180 BPM = 3 beats = 1440 ticks
        // - Total at 4 seconds: 2400 + 1440 = 3840 ticks
        assert_eq!(tempo_map.seconds_to_ticks(4.0), 3840);
    }

    #[test]
    fn test_tempo_map_tempo_speedup() {
        // Test tempo doubling (should double tick rate)
        let mut tempo_map = TempoMap::new(60.0, 480);

        // At 60 BPM: 1 beat per second
        assert_eq!(tempo_map.seconds_to_ticks(1.0), 480);

        // Add tempo change to 120 BPM at 2 seconds
        tempo_map.add_change(2.0, 120.0);
        tempo_map.finalize();

        // First 2 seconds at 60 BPM: 2 beats = 960 ticks
        assert_eq!(tempo_map.seconds_to_ticks(2.0), 960);

        // Next 1 second at 120 BPM: 2 beats = 960 ticks
        // Total at 3 seconds: 960 + 960 = 1920 ticks
        assert_eq!(tempo_map.seconds_to_ticks(3.0), 1920);
    }

    #[test]
    fn test_tempo_map_edge_cases() {
        let tempo_map = TempoMap::new(120.0, 480);

        // Test zero and negative times
        assert_eq!(tempo_map.seconds_to_ticks(0.0), 0);
        assert_eq!(tempo_map.seconds_to_ticks(-1.0), 0);

        // Test very small times
        assert!(tempo_map.seconds_to_ticks(0.001) > 0);
    }
}
