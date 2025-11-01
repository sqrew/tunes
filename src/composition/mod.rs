use crate::drum_grid::DrumGrid;
use crate::envelope::Envelope;
use crate::instruments::Instrument;
use crate::rhythm::Tempo;
use crate::track::{Mixer, Track};
use crate::waveform::Waveform;
use std::collections::HashMap;

// Import synthesis types - use prelude which re-exports them
use crate::prelude::{FilterEnvelope, FMParams};

// Module declarations
mod notes;
mod musical_patterns;
mod ornaments;
mod portamento;
mod timing;
mod patterns;
mod effects;
mod expression;
mod tuplets;
mod classical_patterns;
mod musical_time;
mod sections;
mod synthesis;
pub mod generative;

// Re-export Section types for public API
pub use sections::{Section, SectionBuilder};

/// A musical composition with multiple named tracks
pub struct Composition {
    tracks: HashMap<String, Track>,
    pub(crate) sections: HashMap<String, sections::Section>,
    tempo: Tempo,
}

impl Composition {
    /// Create a new composition with a given tempo
    pub fn new(tempo: Tempo) -> Self {
        Self {
            tracks: HashMap::new(),
            sections: HashMap::new(),
            tempo,
        }
    }

    /// Get or create a track by name
    pub fn track(&mut self, name: &str) -> TrackBuilder<'_> {
        let tempo = self.tempo;
        TrackBuilder {
            composition: self,
            context: BuilderContext::Direct,
            track_name: name.to_string(),
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
        }
    }

    /// Create a track with an instrument preset
    pub fn instrument(&mut self, name: &str, instrument: &Instrument) -> TrackBuilder<'_> {
        let tempo = self.tempo;
        let mut builder = TrackBuilder {
            composition: self,
            context: BuilderContext::Direct,
            track_name: name.to_string(),
            cursor: 0.0,
            pattern_start: 0.0,
            waveform: instrument.waveform,
            envelope: instrument.envelope,
            filter_envelope: FilterEnvelope::default(), // Default (no filter envelope)
            fm_params: FMParams::default(), // Default (no FM)
            swing: 0.5, // No swing by default (straight timing)
            swing_counter: 0,
            pitch_bend: 0.0, // No pitch bend by default
            tempo,
        };

        // Apply instrument settings to the track
        let track = builder.get_track_mut();
        track.volume = instrument.volume;
        track.pan = instrument.pan;
        track.filter = instrument.filter;
        track.delay = instrument.delay.clone();
        track.reverb = instrument.reverb.clone();
        track.distortion = instrument.distortion;
        track.modulation = instrument.modulation.clone();

        builder
    }

    /// Convert this composition into a Mixer for playback
    pub fn into_mixer(self) -> Mixer {
        let mut mixer = Mixer::new();
        for (_name, track) in self.tracks {
            mixer.add_track(track);
        }
        mixer
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
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
    /// # use tunes::instruments::Instrument;
    /// # use tunes::drums::DrumType;
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
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
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
            if let Some(section) = self.sections.get(&section_name.to_string()) {
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
                        comp_track.delay = offset_track.delay;
                        comp_track.reverb = offset_track.reverb;
                        comp_track.distortion = offset_track.distortion;
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
        let section = self
            .sections
            .get_mut(section_name)
            .expect("Section should exist");

        section.tracks.entry(track_name.to_string()).or_default()
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
    pub(crate) cursor: f32,          // Current time position in the track
    pub(crate) pattern_start: f32,   // Start position of current pattern (for looping)
    pub(crate) waveform: Waveform,   // Current waveform for notes
    pub(crate) envelope: Envelope,   // Current envelope for notes
    pub(crate) filter_envelope: FilterEnvelope, // Current filter envelope for notes
    pub(crate) fm_params: FMParams,  // Current FM synthesis parameters
    pub(crate) swing: f32,           // Swing ratio (0.5 = straight, 0.67 = triplet swing, 0.75 = heavy)
    pub(crate) swing_counter: usize, // Counter to track even/odd notes for swing
    pub(crate) pitch_bend: f32,      // Pitch bend in semitones for subsequent notes (0.0 = no bend)
    pub(crate) tempo: Tempo,         // Tempo for musical time calculations
}

impl<'a> TrackBuilder<'a> {
    /// Get a mutable reference to the track being built
    ///
    /// This handles both direct composition tracks and section tracks
    pub(crate) fn get_track_mut(&mut self) -> &mut Track {
        match &self.context {
            BuilderContext::Direct => {
                self.composition
                    .tracks
                    .entry(self.track_name.clone())
                    .or_default()
            }
            BuilderContext::Section(section_name) => {
                self.composition
                    .get_or_create_section_track(section_name, &self.track_name)
            }
        }
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
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
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
            BuilderContext::Direct => {
                self.composition
                    .tracks
                    .entry(self.track_name.clone())
                    .or_default()
            }
            BuilderContext::Section(section_name) => {
                self.composition
                    .get_or_create_section_track(section_name, &self.track_name)
            }
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
    /// # use tunes::rhythm::Tempo;
    /// # use tunes::notes::*;
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
                panic!("and() can only be used in section context. Use comp.section(\"name\").track(...).and()...")
            }
        };

        SectionBuilder::new(self.composition, section_name, self.tempo)
    }
}
