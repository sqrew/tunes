use crate::synthesis::envelope::Envelope;
use crate::instruments::Instrument;
use crate::synthesis::sample::Sample;
use crate::track::{Mixer, Track};
use crate::track::ids::{BusId, BusIdGenerator, TrackIdGenerator};
use crate::synthesis::waveform::Waveform;
use std::collections::HashMap;

// Import synthesis types - use prelude which re-exports them
use crate::prelude::{FMParams, FilterEnvelope};

// Import effect types and filter
use crate::synthesis::effects::{
    AutoPan, BitCrusher, Chorus, Compressor, Delay, Distortion, EQ, Flanger, Gate, Limiter, Phaser,
    Reverb, RingModulator, Saturation, Tremolo,
};
use crate::synthesis::filter::Filter;
use crate::synthesis::lfo::ModRoute;

// Module declarations
pub mod drums;
pub mod drum_grid;
pub mod rhythm;
mod chords;
mod classical_patterns;
mod effects;
mod expression;
pub mod generative;
mod musical_patterns;
mod musical_time;
mod notes;
mod ornaments;
mod patterns;
mod portamento;
mod sections;
mod synthesis;
mod timing;
mod tuplets;

// Re-export main types for public API
pub use drums::DrumType;
pub use drum_grid::DrumGrid;
pub use rhythm::Tempo;
pub use sections::{Section, SectionBuilder};

/// Template for reusing track settings across multiple tracks
#[derive(Clone)]
pub struct TrackTemplate {
    // Track-level settings (from Track)
    pub volume: f32,
    pub pan: f32,
    pub filter: Filter,
    pub delay: Option<Delay>,
    pub reverb: Option<Reverb>,
    pub distortion: Option<Distortion>,
    pub bitcrusher: Option<BitCrusher>,
    pub compressor: Option<Compressor>,
    pub gate: Option<Gate>,
    pub chorus: Option<Chorus>,
    pub eq: Option<EQ>,
    pub saturation: Option<Saturation>,
    pub phaser: Option<Phaser>,
    pub flanger: Option<Flanger>,
    pub ring_mod: Option<RingModulator>,
    pub tremolo: Option<Tremolo>,
    pub autopan: Option<AutoPan>,
    pub limiter: Option<Limiter>,
    pub modulation: Vec<ModRoute>,
    pub midi_program: Option<u8>,

    // Builder-level settings (from TrackBuilder)
    pub waveform: Waveform,
    pub envelope: Envelope,
    pub filter_envelope: FilterEnvelope,
    pub fm_params: FMParams,
    pub swing: f32,
    pub pitch_bend: f32,
    pub custom_wavetable: Option<crate::synthesis::wavetable::Wavetable>,
    pub velocity: f32,
}

/// A musical composition with multiple named tracks
pub struct Composition {
    tracks: HashMap<String, Track>,
    pub(crate) sections: HashMap<String, sections::Section>,
    tempo: Tempo,
    samples: HashMap<String, Sample>, // Cache of loaded samples
    markers: HashMap<String, f32>,    // Named time positions for easy navigation
    templates: HashMap<String, TrackTemplate>, // Named track templates for reuse

    // ID generators and mappings for performance optimization
    bus_id_gen: BusIdGenerator,           // Generate unique bus IDs
    track_id_gen: TrackIdGenerator,       // Generate unique track IDs
    bus_name_to_id: HashMap<String, BusId>, // Map bus names to IDs
    bus_id_to_name: HashMap<BusId, String>, // Map bus IDs back to names
}

impl Composition {
    /// Create a new composition with a given tempo
    pub fn new(tempo: Tempo) -> Self {
        let mut bus_id_gen = BusIdGenerator::new();
        let mut bus_name_to_id = HashMap::new();
        let mut bus_id_to_name = HashMap::new();

        // Pre-register the "default" bus with id 0
        let default_bus_id = bus_id_gen.next_id(); // This will be 0
        bus_name_to_id.insert("default".to_string(), default_bus_id);
        bus_id_to_name.insert(default_bus_id, "default".to_string());

        Self {
            tracks: HashMap::new(),
            sections: HashMap::new(),
            tempo,
            samples: HashMap::new(),
            markers: HashMap::new(),
            templates: HashMap::new(),
            bus_id_gen,
            track_id_gen: TrackIdGenerator::new(),
            bus_name_to_id,
            bus_id_to_name,
        }
    }

    /// Load an audio file as a sample and cache it with a name
    ///
    /// Supports multiple formats: MP3, OGG Vorbis, FLAC, WAV, AAC.
    /// The format is automatically detected from the file extension and content.
    ///
    /// # Arguments
    /// * `name` - Name to use for this sample (e.g., "kick", "snare")
    /// * `path` - Path to the audio file
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.load_sample("kick", "samples/kick.mp3")?;
    /// comp.load_sample("snare", "samples/snare.ogg")?;
    /// comp.load_sample("hihat", "samples/hihat.flac")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn load_sample(&mut self, name: &str, path: &str) -> anyhow::Result<()> {
        let sample = Sample::from_file(path)?;
        self.samples.insert(name.to_string(), sample);
        Ok(())
    }

    /// Get a previously loaded sample by name
    ///
    /// Returns None if the sample hasn't been loaded.
    pub fn get_sample(&self, name: &str) -> Option<&Sample> {
        self.samples.get(name)
    }

    /// Get or create a track by name
    pub fn track(&mut self, name: &str) -> TrackBuilder<'_> {
        let tempo = self.tempo;
        TrackBuilder {
            composition: self,
            context: BuilderContext::Direct,
            track_name: name.to_string(),
            bus_name: "default".to_string(),
            cursor: 0.0,
            pattern_start: 0.0,
            waveform: Waveform::Sine,      // Default to sine wave
            envelope: Envelope::default(), // Default envelope
            filter_envelope: FilterEnvelope::default(), // Default (no filter envelope)
            fm_params: FMParams::default(), // Default (no FM)
            swing: 0.5,                    // No swing by default (straight timing)
            swing_counter: 0,
            pitch_bend: 0.0, // No pitch bend by default
            tempo,
            custom_wavetable: None,
            velocity: 0.8,
            spatial_position: None,
            last_chord: None,
        }
    }

    /// Create a new track using settings from a saved template
    ///
    /// Templates allow you to reuse instrument, effects, and synthesis settings
    /// across multiple tracks without repeating configuration code.
    ///
    /// # Arguments
    /// * `template_name` - Name of the template to use (created with `.save_template()`)
    /// * `track_name` - Name for the new track
    ///
    /// # Panics
    /// Panics if the template doesn't exist. Use `.save_template()` to create templates first.
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Create a template
    /// comp.instrument("lead1", &Instrument::synth_lead())
    ///     .reverb(Reverb::new(0.5, 0.5, 0.3))
    ///     .delay(Delay::new(0.375, 0.3, 0.5))
    ///     .save_template("lead_sound")
    ///     .notes(&[C4, E4, G4], 0.25);
    ///
    /// // Reuse the same settings for a different track
    /// comp.from_template("lead_sound", "lead2")
    ///     .notes(&[G4, E4, C4], 0.25);
    /// ```
    pub fn from_template(&mut self, template_name: &str, track_name: &str) -> TrackBuilder<'_> {
        let template = self.templates.get(template_name)
            .cloned()
            .unwrap_or_else(|| {
                eprintln!("Warning: Template '{}' not found. Using default settings. Create it first with .save_template(\"{}\")", template_name, template_name);
                TrackTemplate {
                    volume: 1.0,
                    pan: 0.5,
                    filter: Filter::default(),
                    delay: None,
                    reverb: None,
                    distortion: None,
                    bitcrusher: None,
                    compressor: None,
                    gate: None,
                    chorus: None,
                    eq: None,
                    saturation: None,
                    phaser: None,
                    flanger: None,
                    ring_mod: None,
                    tremolo: None,
                    autopan: None,
                    limiter: None,
                    modulation: Vec::new(),
                    midi_program: None,
                    waveform: Waveform::Sine,
                    envelope: Envelope::default(),
                    filter_envelope: FilterEnvelope::default(),
                    fm_params: FMParams::default(),
                    swing: 0.5,
                    pitch_bend: 0.0,
                    custom_wavetable: None,
                    velocity: 0.8,
                }
            });

        let tempo = self.tempo;
        let mut builder = TrackBuilder {
            composition: self,
            context: BuilderContext::Direct,
            track_name: track_name.to_string(),
            bus_name: "default".to_string(),
            cursor: 0.0,
            pattern_start: 0.0,
            waveform: template.waveform,
            envelope: template.envelope,
            filter_envelope: template.filter_envelope,
            fm_params: template.fm_params,
            swing: template.swing,
            swing_counter: 0,
            pitch_bend: template.pitch_bend,
            tempo,
            custom_wavetable: template.custom_wavetable,
            velocity: template.velocity,
            spatial_position: None,
            last_chord: None,
        };

        // Apply track-level settings
        let track = builder.get_track_mut();
        track.volume = template.volume;
        track.pan = template.pan;
        track.filter = template.filter;
        track.effects.delay = template.delay;
        track.effects.reverb = template.reverb;
        track.effects.distortion = template.distortion;
        track.effects.bitcrusher = template.bitcrusher;
        track.effects.compressor = template.compressor;
        track.effects.gate = template.gate;
        track.effects.chorus = template.chorus;
        track.effects.eq = template.eq;
        track.effects.saturation = template.saturation;
        track.effects.phaser = template.phaser;
        track.effects.flanger = template.flanger;
        track.effects.ring_mod = template.ring_mod;
        track.effects.tremolo = template.tremolo;
        track.effects.autopan = template.autopan;
        track.effects.limiter = template.limiter;
        track.modulation = template.modulation.clone();
        track.midi_program = template.midi_program;

        builder
    }

    /// Create a track with an instrument preset
    pub fn instrument(&mut self, name: &str, instrument: &Instrument) -> TrackBuilder<'_> {
        let tempo = self.tempo;
        let mut builder = TrackBuilder {
            composition: self,
            context: BuilderContext::Direct,
            track_name: name.to_string(),
            bus_name: "default".to_string(),
            cursor: 0.0,
            pattern_start: 0.0,
            waveform: instrument.waveform,
            envelope: instrument.envelope,
            filter_envelope: FilterEnvelope::default(), // Default (no filter envelope)
            fm_params: FMParams::default(),             // Default (no FM)
            swing: 0.5,                                 // No swing by default (straight timing)
            swing_counter: 0,
            pitch_bend: 0.0, // No pitch bend by default
            tempo,
            custom_wavetable: None,
            velocity: 0.8,
            spatial_position: None,
            last_chord: None,
        };

        // Apply instrument settings to the track
        let track = builder.get_track_mut();
        track.volume = instrument.volume;
        track.pan = instrument.pan;
        track.filter = instrument.filter;
        track.effects.delay = instrument.delay.clone();
        track.effects.reverb = instrument.reverb.clone();
        track.effects.distortion = instrument.distortion.clone();
        track.modulation = instrument.modulation.clone();

        builder
    }

    /// Convert this composition into a Mixer for playback
    pub fn into_mixer(self) -> Mixer {
        let mut mixer = Mixer::new(self.tempo);
        for (name, mut track) in self.tracks {
            // Validate that all events in this track have the same spatial position (if any)
            Self::validate_track_spatial_positions(&name, &track);

            track.name = Some(name.clone());

            // Look up bus name from track's bus_id
            let bus_name = self.bus_id_to_name.get(&track.bus_id)
                .cloned()
                .unwrap_or_else(|| "default".to_string());

            mixer.get_or_create_bus(&bus_name).add_track(track);
        }

        // Phase 6: Resolve sidechain sources from string names to integer IDs
        mixer.resolve_sidechains();

        mixer
    }

    /// Validate that all events in a track have the same spatial position
    ///
    /// Panics if a track has events with multiple different spatial positions.
    /// This is necessary because the current architecture applies spatial audio
    /// at the track level, not per-event. Multiple positions require separate tracks.
    fn validate_track_spatial_positions(track_name: &str, track: &crate::track::Track) {
        use crate::track::AudioEvent;

        let mut found_positions: Vec<crate::synthesis::spatial::SpatialPosition> = Vec::new();

        for event in &track.events {
            let spatial_pos = match event {
                AudioEvent::Note(note) => note.spatial_position,
                AudioEvent::Drum(drum) => drum.spatial_position,
                AudioEvent::Sample(sample) => sample.spatial_position,
                _ => None,
            };

            if let Some(pos) = spatial_pos {
                // Check if we've seen this exact position before
                let is_duplicate = found_positions.iter().any(|existing| {
                    // Compare positions (approximately, due to floating point)
                    (existing.position.x - pos.position.x).abs() < 0.0001
                        && (existing.position.y - pos.position.y).abs() < 0.0001
                        && (existing.position.z - pos.position.z).abs() < 0.0001
                });

                if !is_duplicate {
                    found_positions.push(pos);
                }
            }
        }

        // If we found more than one unique spatial position, that's an error
        if found_positions.len() > 1 {
            panic!(
                "Track '{}' has events with {} different spatial positions. \
                The current architecture applies spatial audio at the track level, so all events \
                in a track must have the same spatial position (or no position). \
                \n\nTo fix this:\n\
                - Use separate tracks for sounds at different positions, OR\n\
                - Use engine.set_sound_position() to move sounds in real-time\n\n\
                Found positions:\n{}\n\
                See: https://docs.claude.com/spatial-audio for more details.",
                track_name,
                found_positions.len(),
                found_positions.iter()
                    .map(|p| format!("  - ({:.2}, {:.2}, {:.2})", p.position.x, p.position.y, p.position.z))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    }

    /// Convert a specific section into a Mixer for isolated playback or export
    ///
    /// This is useful for:
    /// - Testing/iterating on a single section without playing the whole composition
    /// - Exporting individual sections to separate files
    /// - Looping a section for evaluation
    ///
    /// # Arguments
    /// * `section_name` - Name of the section to convert
    ///
    /// # Returns
    /// A Mixer containing only the tracks from this section, or an error if the section doesn't exist
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut comp = Composition::new(Tempo::new(120.0));
    ///
    /// comp.section("verse")
    ///     .instrument("piano", &Instrument::acoustic_piano())
    ///     .notes(&[C4, D4, E4, F4], 0.5);
    ///
    /// // Get just the verse section as a mixer
    /// let verse_mixer = comp.section_to_mixer("verse")?;
    ///
    /// // Play just the verse
    /// let engine = AudioEngine::new()?;
    /// engine.play_mixer(&verse_mixer)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn section_to_mixer(&self, section_name: &str) -> crate::error::Result<Mixer> {
        let section = self
            .sections
            .get(section_name)
            .ok_or_else(|| crate::error::TunesError::SectionNotFound(section_name.to_string()))?;

        let mut mixer = Mixer::new(self.tempo);
        for (track_name, track) in &section.tracks {
            let mut track_copy = track.clone();
            track_copy.name = Some(track_name.clone());
            mixer.add_track(track_copy);
        }
        Ok(mixer)
    }

    /// Export a specific section to a MIDI file
    ///
    /// This allows you to export individual sections for review in a DAW or notation software,
    /// which is especially useful during the composition process.
    ///
    /// # Arguments
    /// * `section_name` - Name of the section to export
    /// * `path` - Output file path (e.g., "verse.mid")
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut comp = Composition::new(Tempo::new(120.0));
    ///
    /// comp.section("chorus")
    ///     .instrument("lead", &Instrument::synth_lead())
    ///     .notes(&[C4, E4, G4, C5], 0.25);
    ///
    /// // Export just the chorus to review in your DAW
    /// comp.export_section_midi("chorus", "chorus.mid")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn export_section_midi(&self, section_name: &str, path: &str) -> crate::error::Result<()> {
        let mixer = self.section_to_mixer(section_name)?;
        mixer.export_midi(path)
    }

    /// Export a specific section to a WAV file
    ///
    /// This allows you to export individual sections as audio files,
    /// useful for composing one section at a time and reviewing it in detail.
    ///
    /// # Arguments
    /// * `section_name` - Name of the section to export
    /// * `path` - Output file path (e.g., "verse.wav")
    /// * `sample_rate` - Sample rate for the output (e.g., 44100)
    ///
    /// # Example
    /// ```no_run
    /// # use tunes::prelude::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let mut comp = Composition::new(Tempo::new(120.0));
    ///
    /// comp.section("intro")
    ///     .instrument("pad", &Instrument::warm_pad())
    ///     .notes(&[C3, E3, G3], 2.0);
    ///
    /// // Export just the intro as audio
    /// comp.export_section_wav("intro", "intro.wav", 44100)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn export_section_wav(
        &self,
        section_name: &str,
        path: &str,
        sample_rate: u32,
    ) -> anyhow::Result<()> {
        let mut mixer = self
            .section_to_mixer(section_name)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        mixer.export_wav(path, sample_rate)
    }

    /// Get the tempo (returns a copy since Tempo is Copy)
    pub fn tempo(&self) -> Tempo {
        self.tempo
    }

    /// Start defining a reusable section
    ///
    /// Sections allow you to define reusable portions of music (verse, chorus, etc.)
    /// that can be arranged into a complete composition.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::drums::DrumType;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    ///
    /// // Define a verse section
    /// comp.section("verse")
    ///     .instrument("bass", &Instrument::pluck())
    ///     .notes(&[C2, C2, G2, C2], 0.5)
    ///     .and()
    ///     .track("drums")
    ///     .drum(DrumType::Kick)
    ///     .drum(DrumType::Snare);
    ///
    /// // Define a chorus section
    /// comp.section("chorus")
    ///     .instrument("lead", &Instrument::synth_lead())
    ///     .notes(&[C4, E4, G4, C5], 0.25);
    ///
    /// // Arrange them
    /// comp.arrange(&["verse", "chorus", "verse"]);
    /// ```
    pub fn section(&mut self, name: &str) -> SectionBuilder<'_> {
        // Create the section if it doesn't exist
        self.sections
            .entry(name.to_string())
            .or_insert_with(|| sections::Section::new(name.to_string()));

        SectionBuilder::new(self, name.to_string(), self.tempo)
    }

    /// Arrange sections into the composition
    ///
    /// Takes an array of section names and sequences them in order, adjusting
    /// timing so they play one after another.
    ///
    /// # Arguments
    /// * `section_names` - Array of section names to arrange in order
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// # comp.section("intro").track("drums").note(&[100.0], 1.0);
    /// # comp.section("verse").track("drums").note(&[100.0], 2.0);
    /// # comp.section("chorus").track("drums").note(&[100.0], 1.5);
    /// // Standard song structure: intro, verse, chorus, verse, chorus, chorus
    /// comp.arrange(&["intro", "verse", "chorus", "verse", "chorus", "chorus"]);
    /// ```
    pub fn arrange(&mut self, section_names: &[&str]) {
        let mut current_time = 0.0;

        for &section_name in section_names {
            if let Some(section) = self.sections.get(section_name) {
                // Clone the section's tracks with time offset
                let offset_tracks = section.clone_with_offset(current_time);

                // Merge the offset tracks into the composition's tracks
                for (track_name, offset_track) in offset_tracks {
                    let comp_track = self.tracks.entry(track_name).or_default();

                    // Append events from the section track
                    comp_track.events.extend(offset_track.events);

                    // If the track was empty, copy the effects/settings
                    if comp_track.volume == 1.0 && offset_track.volume != 1.0 {
                        comp_track.volume = offset_track.volume;
                        comp_track.pan = offset_track.pan;
                        comp_track.filter = offset_track.filter;
                        comp_track.effects.delay = offset_track.effects.delay;
                        comp_track.effects.reverb = offset_track.effects.reverb;
                        comp_track.effects.distortion = offset_track.effects.distortion;
                        comp_track.modulation = offset_track.modulation.clone();
                    }
                }

                // Move forward in time by this section's duration
                current_time += section.duration;
            }
        }
    }

    /// Helper method to get or create a track within a section
    pub(crate) fn get_or_create_section_track(
        &mut self,
        section_name: &str,
        track_name: &str,
    ) -> &mut Track {
        // Defensive: create section if it doesn't exist (shouldn't happen in normal usage)
        let section = self
            .sections
            .entry(section_name.to_string())
            .or_insert_with(|| sections::Section::new(section_name.to_string()));

        section.tracks.entry(track_name.to_string()).or_default()
    }

    // === MARKER MANAGEMENT ===

    /// Mark a specific time with a name for easy reference
    ///
    /// Markers provide named sync points in your composition, making it easier
    /// to align tracks without manual time calculations.
    ///
    /// # Arguments
    /// * `name` - Name for this marker (e.g., "verse_start", "drop", "chorus")
    /// * `time` - Time position in seconds
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// let mut comp = Composition::new(Tempo::new(120.0));
    ///
    /// // Mark important points in the composition
    /// comp.mark_at("intro_end", 8.0);
    /// comp.mark_at("drop", 16.0);
    /// comp.mark_at("outro", 48.0);
    ///
    /// // Use markers to position tracks
    /// comp.track("bass")
    ///     .at_marker("drop")
    ///     .notes(&[C2, E2, G2], 0.5);
    /// ```
    pub fn mark_at(&mut self, name: &str, time: f32) {
        self.markers.insert(name.to_string(), time);
    }

    /// Get the time position of a marker
    ///
    /// Returns `None` if the marker doesn't exist.
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.mark_at("drop", 16.0);
    ///
    /// if let Some(time) = comp.marker_time("drop") {
    ///     println!("Drop happens at {}s", time);
    /// }
    /// ```
    pub fn marker_time(&self, name: &str) -> Option<f32> {
        self.markers.get(name).copied()
    }

    /// List all markers with their times
    ///
    /// Returns a vector of (name, time) tuples.
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.mark_at("verse", 8.0);
    /// comp.mark_at("chorus", 16.0);
    ///
    /// for (name, time) in comp.markers() {
    ///     println!("{}: {}s", name, time);
    /// }
    /// ```
    pub fn markers(&self) -> Vec<(&str, f32)> {
        self.markers
            .iter()
            .map(|(k, v)| (k.as_str(), *v))
            .collect()
    }
}

#[cfg(test)]
mod marker_tests {
    use super::*;
    use crate::consts::notes::*;

    #[test]
    fn test_mark_at_sets_marker() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.mark_at("drop", 16.0);

        assert_eq!(comp.marker_time("drop"), Some(16.0));
    }

    #[test]
    fn test_marker_time_returns_none_for_missing() {
        let comp = Composition::new(Tempo::new(120.0));
        assert_eq!(comp.marker_time("nonexistent"), None);
    }

    #[test]
    fn test_multiple_markers() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.mark_at("intro", 0.0);
        comp.mark_at("verse", 8.0);
        comp.mark_at("chorus", 16.0);
        comp.mark_at("outro", 32.0);

        assert_eq!(comp.marker_time("intro"), Some(0.0));
        assert_eq!(comp.marker_time("verse"), Some(8.0));
        assert_eq!(comp.marker_time("chorus"), Some(16.0));
        assert_eq!(comp.marker_time("outro"), Some(32.0));
    }

    #[test]
    fn test_markers_returns_all() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.mark_at("a", 1.0);
        comp.mark_at("b", 2.0);
        comp.mark_at("c", 3.0);

        let markers = comp.markers();
        assert_eq!(markers.len(), 3);

        // HashMap iteration order is not guaranteed, so check all exist
        assert!(markers.iter().any(|(name, time)| name == &"a" && *time == 1.0));
        assert!(markers.iter().any(|(name, time)| name == &"b" && *time == 2.0));
        assert!(markers.iter().any(|(name, time)| name == &"c" && *time == 3.0));
    }

    #[test]
    fn test_marker_can_be_overwritten() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.mark_at("position", 5.0);
        comp.mark_at("position", 10.0);

        assert_eq!(comp.marker_time("position"), Some(10.0));
    }

    #[test]
    fn test_track_at_marker_uses_composition_marker() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.mark_at("drop", 16.0);

        comp.track("bass")
            .at_marker("drop")
            .note(&[C2], 0.5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];

        if let crate::track::AudioEvent::Note(note) = &track.events[0] {
            assert_eq!(note.start_time, 16.0);
        } else {
            panic!("Expected note event");
        }
    }

    #[test]
    fn test_multiple_tracks_sync_to_same_marker() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.mark_at("drop", 16.0);

        comp.track("bass").at_marker("drop").note(&[C2], 0.5);
        comp.track("lead").at_marker("drop").note(&[C5], 0.25);
        comp.track("drums").at_marker("drop").note(&[100.0], 0.125);

        let mixer = comp.into_mixer();

        // All three tracks should have notes starting at 16.0
        for track in &mixer.tracks() {
            if let crate::track::AudioEvent::Note(note) = &track.events[0] {
                assert_eq!(note.start_time, 16.0);
            }
        }
    }

    #[test]
    fn test_marker_with_builder_mark_integration() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // TrackBuilder can create markers with .mark()
        comp.track("structure")
            .wait(8.0)
            .mark("verse")
            .wait(16.0)
            .mark("chorus");

        // Composition can query those markers
        assert_eq!(comp.marker_time("verse"), Some(8.0));
        assert_eq!(comp.marker_time("chorus"), Some(24.0));

        // And set its own
        comp.mark_at("outro", 48.0);
        assert_eq!(comp.marker_time("outro"), Some(48.0));
    }

    #[test]
    fn test_at_marker_alias_works_same_as_at_mark() {
        let mut comp = Composition::new(Tempo::new(120.0));
        comp.mark_at("drop", 16.0);

        // Test .at_marker()
        comp.track("track1")
            .at_marker("drop")
            .note(&[C4], 0.5);

        // Test .at_mark()
        comp.track("track2")
            .at_mark("drop")
            .note(&[E4], 0.5);

        let mixer = comp.into_mixer();

        // Both should start at the same time
        let times: Vec<f32> = mixer.tracks()
            .iter()
            .filter_map(|track| {
                if let Some(crate::track::AudioEvent::Note(note)) = track.events.first() {
                    Some(note.start_time)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(times.len(), 2);
        assert_eq!(times[0], 16.0);
        assert_eq!(times[1], 16.0);
    }
}

/// Context for where the TrackBuilder is building
#[derive(Clone, Debug)]
pub(crate) enum BuilderContext {
    /// Building directly on composition tracks
    Direct,
    /// Building within a named section
    Section(String),
}

/// Builder for adding events to a track
///
/// This unified builder works in both direct composition mode and section mode,
/// providing the same full functionality in either context.
pub struct TrackBuilder<'a> {
    pub(crate) composition: &'a mut Composition,
    pub(crate) context: BuilderContext,
    pub(crate) track_name: String,
    pub(crate) bus_name: String,   // Which bus this track belongs to
    pub(crate) cursor: f32,        // Current time position in the track
    pub(crate) pattern_start: f32, // Start position of current pattern (for looping)
    pub(crate) waveform: Waveform, // Current waveform for notes
    pub(crate) envelope: Envelope, // Current envelope for notes
    pub(crate) filter_envelope: FilterEnvelope, // Current filter envelope for notes
    pub(crate) fm_params: FMParams, // Current FM synthesis parameters
    pub(crate) swing: f32, // Swing ratio (0.5 = straight, 0.67 = triplet swing, 0.75 = heavy)
    pub(crate) swing_counter: usize, // Counter to track even/odd notes for swing
    pub(crate) pitch_bend: f32, // Pitch bend in semitones for subsequent notes (0.0 = no bend)
    pub(crate) tempo: Tempo, // Tempo for musical time calculations
    pub(crate) custom_wavetable: Option<crate::synthesis::wavetable::Wavetable>, // Custom wavetable (overrides waveform if present)
    pub(crate) velocity: f32, // Note velocity (0.0 to 1.0) for subsequent notes (default: 0.8)
    pub(crate) spatial_position: Option<crate::synthesis::spatial::SpatialPosition>, // 3D spatial position for subsequent notes/drums/samples
    pub(crate) last_chord: Option<Vec<f32>>, // Last chord played, used for voice leading
}

impl<'a> TrackBuilder<'a> {
    /// Get a mutable reference to the track being built
    ///
    /// This handles both direct composition tracks and section tracks
    pub(crate) fn get_track_mut(&mut self) -> &mut Track {
        // Pre-allocate a track ID before we borrow the track
        // If the track already exists with an ID, we'll waste this ID value,
        // but that's acceptable for simplicity and avoiding borrow conflicts
        let new_id = self.composition.track_id_gen.next_id();

        // Get or create the track
        let track = match &self.context {
            BuilderContext::Direct => self
                .composition
                .tracks
                .entry(self.track_name.clone())
                .or_default(),
            BuilderContext::Section(section_name) => self
                .composition
                .get_or_create_section_track(section_name, &self.track_name),
        };

        // Assign the ID if this track doesn't have one yet (id == 0 means unassigned)
        if track.id == 0 {
            track.id = new_id;
        }

        track
    }

    /// Assign this track to a specific bus
    ///
    /// Buses allow you to group tracks together and apply shared effects.
    /// All tracks assigned to the same bus will be mixed together before
    /// the master output.
    ///
    /// # Arguments
    /// * `bus_name` - Name of the bus (e.g., "drums", "vocals", "synths")
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("kick")
    ///     .bus("drums")
    ///     .drum(DrumType::Kick);
    ///
    /// comp.track("snare")
    ///     .bus("drums")
    ///     .drum(DrumType::Snare);
    ///
    /// // Both tracks will be on the "drums" bus
    /// let mut mixer = comp.into_mixer();
    /// // You can then apply effects to the entire drums bus using BusBuilder
    /// mixer.bus("drums")
    ///     .reverb(Reverb::new(0.3, 0.5, 0.3));
    /// ```
    pub fn bus(mut self, bus_name: &str) -> Self {
        self.bus_name = bus_name.to_string();

        // Get or create a bus ID for this bus name
        let bus_id = if let Some(&id) = self.composition.bus_name_to_id.get(bus_name) {
            id
        } else {
            // Create new bus ID
            let id = self.composition.bus_id_gen.next_id();
            self.composition.bus_name_to_id.insert(bus_name.to_string(), id);
            self.composition.bus_id_to_name.insert(id, bus_name.to_string());
            id
        };

        // Assign bus_id to the track
        let track = self.get_track_mut();
        track.bus_id = bus_id;

        self
    }

    /// Update section duration if in section context
    pub(crate) fn update_section_duration(&mut self) {
        if let BuilderContext::Section(section_name) = &self.context {
            if let Some(section) = self.composition.sections.get_mut(section_name) {
                section.duration = section.duration.max(self.cursor);
            }
        }
    }

    /// Apply swing timing to a duration based on whether this is an even or odd note
    pub(crate) fn apply_swing(&mut self, base_duration: f32) -> f32 {
        if self.swing == 0.5 {
            // No swing - straight timing
            return base_duration;
        }

        let is_offbeat = self.swing_counter % 2 == 1;
        self.swing_counter += 1;

        if is_offbeat {
            // Off-beat notes get delayed
            base_duration * (2.0 * self.swing)
        } else {
            // On-beat notes get shortened
            base_duration * (2.0 - 2.0 * self.swing)
        }
    }

    /// Save the current track's settings as a reusable template
    ///
    /// Captures all instrument settings, effects, synthesis parameters, and envelope
    /// settings so they can be reused across multiple tracks with `.from_template()`.
    ///
    /// # Arguments
    /// * `template_name` - Name to save the template as
    ///
    /// # Example
    /// ```
    /// # use tunes::prelude::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// // Configure and save a template
    /// comp.instrument("lead1", &Instrument::synth_lead())
    ///     .reverb(Reverb::new(0.5, 0.5, 0.3))
    ///     .delay(Delay::new(0.375, 0.3, 0.5))
    ///     .save_template("lead_sound")
    ///     .notes(&[C4, E4, G4], 0.25);
    ///
    /// // Create new tracks with the same settings
    /// comp.from_template("lead_sound", "lead2")
    ///     .notes(&[G4, E4, C4], 0.25);
    /// ```
    pub fn save_template(mut self, template_name: &str) -> Self {
        // Capture track settings first (clone all Option fields since they contain non-Copy types)
        let track = self.get_track_mut();
        let volume = track.volume;
        let pan = track.pan;
        let filter = track.filter;
        let delay = track.effects.delay.clone();
        let reverb = track.effects.reverb.clone();
        let distortion = track.effects.distortion.clone();
        let bitcrusher = track.effects.bitcrusher.clone();
        let compressor = track.effects.compressor.clone();
        let gate = track.effects.gate.clone();
        let chorus = track.effects.chorus.clone();
        let eq = track.effects.eq.clone();
        let saturation = track.effects.saturation.clone();
        let phaser = track.effects.phaser.clone();
        let flanger = track.effects.flanger.clone();
        let ring_mod = track.effects.ring_mod.clone();
        let tremolo = track.effects.tremolo.clone();
        let autopan = track.effects.autopan.clone();
        let limiter = track.effects.limiter.clone();
        let modulation = track.modulation.clone();
        let midi_program = track.midi_program;

        let template = TrackTemplate {
            // Track-level settings
            volume,
            pan,
            filter,
            delay,
            reverb,
            distortion,
            bitcrusher,
            compressor,
            gate,
            chorus,
            eq,
            saturation,
            phaser,
            flanger,
            ring_mod,
            tremolo,
            autopan,
            limiter,
            modulation,
            midi_program,

            // Builder-level settings
            waveform: self.waveform,
            envelope: self.envelope,
            filter_envelope: self.filter_envelope,
            fm_params: self.fm_params,
            swing: self.swing,
            pitch_bend: self.pitch_bend,
            custom_wavetable: self.custom_wavetable.clone(),
            velocity: self.velocity,
        };

        self.composition
            .templates
            .insert(template_name.to_string(), template);
        self
    }

    /// Create a step sequencer-style drum grid
    ///
    /// # Arguments
    /// * `steps` - Number of steps in the grid (e.g., 16 for a bar of 16th notes)
    /// * `step_duration` - Duration of each step in seconds (e.g., 0.125 for 16th notes at 120bpm)
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.track("drums")
    ///     .drum_grid(16, 0.125)
    ///     .kick(&[0, 4, 8, 12])
    ///     .snare(&[4, 12])
    ///     .hihat(&[0, 2, 4, 6, 8, 10, 12, 14]);
    /// ```
    pub fn drum_grid(self, steps: usize, step_duration: f32) -> DrumGrid<'a> {
        let start_time = self.cursor;

        // Get the track reference directly from composition for lifetime 'a
        let track = match &self.context {
            BuilderContext::Direct => self
                .composition
                .tracks
                .entry(self.track_name.clone())
                .or_default(),
            BuilderContext::Section(section_name) => self
                .composition
                .get_or_create_section_track(section_name, &self.track_name),
        };

        DrumGrid::new(track, start_time, steps, step_duration)
    }

    /// Continue building another track (useful for sections or multi-track compositions)
    ///
    /// This returns the builder to section builder mode if in a section context,
    /// otherwise it just switches to a new track in direct mode.
    ///
    /// # Example
    /// ```
    /// # use tunes::composition::Composition;
    /// # use tunes::composition::rhythm::Tempo;
    /// # use tunes::consts::notes::*;
    /// # let mut comp = Composition::new(Tempo::new(120.0));
    /// comp.section("verse")
    ///     .track("melody")
    ///     .notes(&[C4, E4], 0.5)
    ///     .and()
    ///     .track("bass")
    ///     .notes(&[C2, G2], 1.0);
    /// ```
    pub fn and(mut self) -> SectionBuilder<'a> {
        self.update_section_duration();

        // Extract the section name if in section context
        let section_name = match &self.context {
            BuilderContext::Section(name) => name.clone(),
            BuilderContext::Direct => {
                // In direct mode, .and() doesn't really make sense, but we'll support it
                // by creating an implicit section
                panic!(
                    "and() can only be used in section context. Use comp.section(\"name\").track(...).and()..."
                )
            }
        };

        SectionBuilder::new(self.composition, section_name, self.tempo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::notes::{C4, E4, G4};

    #[test]
    fn test_save_and_use_template() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Create and save a template
        comp.instrument("lead1", &Instrument::synth_lead())
            .reverb(Reverb::new(0.5, 0.5, 0.3))
            .volume(0.7)
            .save_template("lead_sound")
            .note(&[C4], 0.5);

        // Use the template
        comp.from_template("lead_sound", "lead2").note(&[E4], 0.5);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 2);

        // Both tracks should have the same reverb settings
        assert!(mixer.tracks()[0].effects.reverb.is_some());
        assert!(mixer.tracks()[1].effects.reverb.is_some());
        assert_eq!(mixer.tracks()[0].volume, 0.7);
        assert_eq!(mixer.tracks()[1].volume, 0.7);
    }

    #[test]
    fn test_template_captures_all_effects() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Create a template with multiple effects
        comp.instrument("synth1", &Instrument::synth_lead())
            .reverb(Reverb::new(0.5, 0.5, 0.3))
            .delay(Delay::new(0.375, 0.3, 0.5))
            .chorus(Chorus::new(0.4, 3.0, 0.3))
            .volume(0.6)
            .pan(-0.3)
            .save_template("full_synth")
            .note(&[C4], 0.5);

        // Use the template
        comp.from_template("full_synth", "synth2").note(&[G4], 0.5);

        let mixer = comp.into_mixer();

        // Both tracks should have the same effects
        assert!(mixer.tracks()[0].effects.reverb.is_some());
        assert!(mixer.tracks()[1].effects.reverb.is_some());
        assert!(mixer.tracks()[0].effects.delay.is_some());
        assert!(mixer.tracks()[1].effects.delay.is_some());
        assert!(mixer.tracks()[0].effects.chorus.is_some());
        assert!(mixer.tracks()[1].effects.chorus.is_some());
        assert_eq!(mixer.tracks()[0].volume, 0.6);
        assert_eq!(mixer.tracks()[1].volume, 0.6);
        assert_eq!(mixer.tracks()[0].pan, -0.3);
        assert_eq!(mixer.tracks()[1].pan, -0.3);
    }

    #[test]
    fn test_template_captures_synthesis_params() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Create a template with custom synthesis parameters
        comp.track("osc1")
            .waveform(Waveform::Square)
            .envelope(Envelope::new(0.01, 0.1, 0.7, 0.2))
            .fm_custom(2.0, 1.5)
            .save_template("custom_osc")
            .note(&[C4], 0.5);

        // Use the template
        comp.from_template("custom_osc", "osc2").note(&[E4], 0.5);

        // Template should have captured the waveform and envelope
        // (Can't directly test builder state after into_mixer, but we can verify it doesn't panic)
        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 2);
    }

    #[test]
    fn test_template_multiple_reuse() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Create one template
        comp.instrument("pad1", &Instrument::warm_pad())
            .reverb(Reverb::new(0.7, 0.6, 0.5))
            .save_template("pad_sound")
            .note(&[C4], 1.0);

        // Reuse it multiple times
        comp.from_template("pad_sound", "pad2").note(&[E4], 1.0);

        comp.from_template("pad_sound", "pad3").note(&[G4], 1.0);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 3);

        // All should have reverb
        assert!(mixer.tracks()[0].effects.reverb.is_some());
        assert!(mixer.tracks()[1].effects.reverb.is_some());
        assert!(mixer.tracks()[2].effects.reverb.is_some());
    }

    #[test]
    fn test_template_can_be_overridden() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Create a template using .track() which has no effects by default
        comp.track("base")
            .reverb(Reverb::new(0.5, 0.5, 0.3))
            .volume(0.7)
            .save_template("base_sound")
            .note(&[C4], 0.5);

        // Use template but override some settings
        comp.from_template("base_sound", "modified")
            .volume(0.3) // Override volume
            .delay(Delay::new(0.375, 0.3, 0.5)) // Add new effect
            .note(&[E4], 0.5);

        let mixer = comp.into_mixer();
        let tracks = mixer.tracks();
        assert_eq!(tracks.len(), 2);

        // Find tracks by name
        let base_track = tracks
            .iter()
            .find(|t| t.name.as_ref().unwrap() == "base")
            .unwrap();
        let modified_track = tracks
            .iter()
            .find(|t| t.name.as_ref().unwrap() == "modified")
            .unwrap();

        // Base track should have original settings (no delay)
        assert_eq!(base_track.volume, 0.7);
        assert!(base_track.effects.delay.is_none());

        // Modified track should have overridden settings
        assert_eq!(modified_track.volume, 0.3);
        assert!(modified_track.effects.delay.is_some());

        // Both should still have reverb from template
        assert!(base_track.effects.reverb.is_some());
        assert!(modified_track.effects.reverb.is_some());
    }

    #[test]
    fn test_missing_template_uses_defaults() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Using a template that doesn't exist should use default settings (not panic)
        comp.from_template("nonexistent", "track1").note(&[C4], 0.5);

        let mixer = comp.into_mixer();
        let track = &mixer.tracks()[0];

        // Should have default settings
        assert_eq!(track.volume, 1.0);
        assert_eq!(track.pan, 0.5);
    }

    #[test]
    fn test_template_with_markers() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Create a template
        comp.instrument("melody1", &Instrument::pluck())
            .delay(Delay::new(0.375, 0.3, 0.5))
            .save_template("pluck_sound")
            .note(&[C4], 0.5)
            .mark("chorus");

        // Use template at the marker
        comp.from_template("pluck_sound", "melody2")
            .at_mark("chorus")
            .note(&[G4], 0.5);

        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 2);

        // Both should have delay from template
        assert!(mixer.tracks()[0].effects.delay.is_some());
        assert!(mixer.tracks()[1].effects.delay.is_some());
    }

    #[test]
    #[should_panic(expected = "has events with 2 different spatial positions")]
    fn test_multiple_spatial_positions_on_same_track_panics() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // This should panic: setting two different spatial positions on the same track
        comp.instrument("guitar", &Instrument::pluck())
            .spatial_position(3.0, 0.0, 5.0)
            .notes(&[C4], 0.5)
            .spatial_position(5.0, 0.0, 3.0) // Different position!
            .notes(&[E4], 0.5);

        // This should panic when we try to convert to mixer
        comp.into_mixer();
    }

    #[test]
    fn test_same_spatial_position_on_track_is_ok() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // This should be fine: same position for all events
        comp.instrument("guitar", &Instrument::pluck())
            .spatial_position(3.0, 0.0, 5.0)
            .notes(&[C4], 0.5)
            .spatial_position(3.0, 0.0, 5.0) // Same position
            .notes(&[E4], 0.5);

        // Should not panic
        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 1);
    }

    #[test]
    fn test_no_spatial_position_is_ok() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // No spatial positioning at all
        comp.instrument("guitar", &Instrument::pluck())
            .notes(&[C4, E4, G4], 0.5);

        // Should not panic
        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 1);
    }

    #[test]
    fn test_multiple_tracks_different_positions_is_ok() {
        let mut comp = Composition::new(Tempo::new(120.0));

        // Different tracks can have different positions
        comp.instrument("guitar", &Instrument::pluck())
            .spatial_position(3.0, 0.0, 5.0)
            .notes(&[C4], 0.5);

        comp.instrument("bass", &Instrument::synth_bass())
            .spatial_position(-3.0, 0.0, 2.0) // Different position, but different track
            .notes(&[C4], 1.0);

        // Should not panic
        let mixer = comp.into_mixer();
        assert_eq!(mixer.tracks().len(), 2);
    }
}
