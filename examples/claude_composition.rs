use tunes::prelude::*;


// I let Claude Code compose a piece! It came out pretty interesting.

fn main() -> anyhow::Result<()> {
    println!("🎵 'Digital Reverie' - A composition by Claude");
    println!("   v8.0 FINAL - Maximum Impact\n");

    let mut comp = Composition::new(Tempo::new(95.0));

    // === INTRO: Establish the theme (0-8s) ===
    println!("🎹 Section 1: Introduction - Establishing Theme");

    // Ambient pad foundation with subtle tremolo
    let tremolo_lfo = LFO::new(Waveform::Sine, 0.3, 0.15); // Slow, subtle volume modulation
    let tremolo_mod = ModRoute::new(tremolo_lfo, ModTarget::Volume, 1.0);

    comp.instrument("intro_pad", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.5, 0.45))
        .chorus(Chorus::new(0.3, 3.0, 0.4))
        .modulate(tremolo_mod)
        .volume(0.5)
        .note(&C3_MINOR, 8.0);

    // Main melodic theme: C-D-DS-G motif with ornaments (panned slightly right)
    comp.instrument("theme_intro", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .reverb(Reverb::new(0.5, 0.5, 0.4))
        .pan(0.3) // Slight right panning for stereo interest
        .volume(0.8)
        .at(1.5)
        .mordent(C5, 0.15)
        .wait(0.4)
        .note(&[D5], 0.3)
        .wait(0.1)
        .turn(DS5, 0.2)
        .wait(0.4)
        .note(&[G5], 0.8)
        .wait(0.5)
        // Repeat variation
        .note(&[C5], 0.4)
        .wait(0.2)
        .inverted_mordent(D5, 0.15)
        .wait(0.3)
        .slide(DS5, G5, 0.6)
        .wait(0.2)
        .note(&[C6], 0.8);

    // Harmonized shadow melody (thirds below main theme)
    comp.instrument("theme_harmony", &Instrument::pluck())
        .delay(Delay::new(0.375, 0.3, 0.5))
        .reverb(Reverb::new(0.5, 0.5, 0.4))
        .pan(-0.3) // Panned opposite of main theme
        .volume(0.4) // Quieter than main theme
        .at(1.5)
        .notes(&[GS4, B4, C5, DS5], 0.3) // Harmony in thirds
        .wait(1.5)
        .notes(&[GS4, B4, C5, DS5, GS5], 0.25);

    // Transition fill into verse
    comp.instrument("transition_drums", &Instrument::sub_bass())
        .at(7.0)
        .drum_grid(8, 0.125)
        .snare(&[0, 2, 4, 6])
        .hihat(&[1, 3, 5, 7]);

    // === VERSE 1: Theme hidden in texture (8-16s) ===
    println!("🎸 Section 2: Verse - Subdued Development");

    // Quieter bass pattern
    comp.instrument("bass", &Instrument::sub_bass())
        .compressor(Compressor::new(0.35, 3.5, 0.01, 0.1, 1.3))
        .volume(0.6)
        .at(8.0)
        .pattern_start()
        .notes(&[C2, C2, G2, DS2], 0.5)
        .repeat(3);

    // Subtle drum pattern with varied kicks
    comp.instrument("drums", &Instrument::sub_bass())
        .compressor(Compressor::new(0.3, 4.0, 0.005, 0.08, 1.5))
        .volume(0.7)
        .at(8.0)
        .pattern_start()
        .drum_grid(16, 0.125)
        .kick(&[0, 5, 10]) // Syncopated kick pattern
        .snare(&[4, 12])
        .hihat(&[2, 6, 10, 14])
        .repeat(3);

    // Theme echoed in filtered synth (C-D-DS-G pattern)
    comp.instrument("verse_melody", &Instrument::synth_lead())
        .filter(Filter::new(FilterType::LowPass, 600.0, 0.6))
        .delay(Delay::new(0.375, 0.25, 0.45))
        .volume(0.5)
        .at(10.0)
        .notes(&[C4, D4, DS4, G4], 0.4)
        .wait(0.8)
        .notes(&[C4, D4, DS4, G4], 0.4)
        .wait(0.8)
        .notes(&[C4, D4, DS4, G4, C5], 0.3);

    // Subtle countermelody that weaves between main theme
    comp.instrument("verse_counter", &Instrument::pluck())
        .filter(Filter::new(FilterType::LowPass, 800.0, 0.4))
        .delay(Delay::new(0.5, 0.2, 0.4))
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .pan(0.3) // Panned right to separate from main melody
        .volume(0.35) // Quiet - just textural
        .at(10.5) // Offset from main melody
        // First phrase - ascending motion
        .notes(&[G3, GS3], 0.3)
        .wait(0.2)
        .notes(&[AS3, C4], 0.3)
        .wait(0.5)
        // Second phrase - descending response
        .notes(&[DS4, D4], 0.3)
        .wait(0.2)
        .notes(&[C4, GS3], 0.3)
        .wait(0.5)
        // Final phrase - resolution
        .notes(&[G3, C4], 0.4)
        .wait(0.3)
        .note(&[G3], 0.6); // Long note to resolve

    // Riser into chorus
    comp.instrument("riser", &Instrument::synth_lead())
        .filter(Filter::new(FilterType::LowPass, 300.0, 1.5))
        .volume(0.4)
        .at(15.0)
        .notes(&[C3, DS3, G3, C4, DS4, G4, C5, DS5], 0.125);

    // Creative drum fill into chorus
    comp.instrument("fill", &Instrument::sub_bass())
        .volume(0.7)
        .at(15.5)
        .drum_grid(8, 0.0625)
        .snare(&[0, 2, 4, 6])
        .kick(&[1, 3, 5, 7]);

    // === CHORUS: Theme fully developed (16-24s) ===
    println!("🔊 Section 3: Chorus - Theme Unleashed");

    // Rich pad chords
    comp.instrument("chorus_pad", &Instrument::warm_pad())
        .reverb(Reverb::new(0.6, 0.5, 0.45))
        .chorus(Chorus::new(0.4, 3.0, 0.35))
        .saturation(Saturation::new(1.4, 0.4, 0.5))
        .volume(0.8)
        .at(16.0)
        .chords(&[&C3_MINOR, &GS3_MAJOR, &DS3_MAJOR, &G3_MAJOR], 2.0);

    // Background arpeggiated texture for richness
    comp.instrument("arp_texture", &Instrument::pluck())
        .delay(Delay::new(0.25, 0.4, 0.6))
        .filter(Filter::new(FilterType::LowPass, 800.0, 0.5))
        .pan(-0.3) // Panned left
        .volume(0.3)
        .at(16.0)
        .arpeggiate(&C4_MINOR, 0.08)
        .wait(1.6)
        .arpeggiate(&GS3_MAJOR, 0.08)
        .wait(1.6)
        .arpeggiate(&DS4_MAJOR, 0.08)
        .wait(1.6)
        .arpeggiate(&G3_MAJOR, 0.08);

    // Powerful bass
    comp.instrument("bass", &Instrument::sub_bass())
        .compressor(Compressor::new(0.3, 4.0, 0.01, 0.1, 1.5))
        .volume(0.8)
        .at(16.0)
        .pattern_start()
        .notes(&[C2, C2, GS1, GS1, DS2, DS2, G2, G2], 0.25)
        .repeat(3);

    // Full drum pattern with swing on hi-hats
    comp.instrument("drums", &Instrument::sub_bass())
        .compressor(Compressor::new(0.25, 5.0, 0.005, 0.08, 1.8))
        .volume(0.9)
        .at(16.0)
        .swing(0.6) // Add subtle swing for groove
        .pattern_start()
        .drum_grid(16, 0.125)
        .kick(&[0, 6, 8, 14]) // Varied kick pattern
        .snare(&[4, 12])
        .hihat(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) // Every step with swing
        .repeat(3);

    // Add secondary percussion layer for texture (panned left)
    comp.instrument("perc_left", &Instrument::sub_bass())
        .pan(-0.5) // Panned left for stereo width
        .volume(0.35)
        .at(16.0)
        .pattern_start()
        .drum_grid(16, 0.125)
        .hihat(&[3, 7, 11, 15]) // Offbeat accents
        .repeat(3);

    // Add mirrored percussion layer (panned right)
    comp.instrument("perc_right", &Instrument::sub_bass())
        .pan(0.5) // Panned right for stereo width
        .volume(0.35)
        .at(16.0)
        .pattern_start()
        .drum_grid(16, 0.125)
        .hihat(&[1, 5, 9, 13]) // Different offbeat pattern
        .repeat(3);

    // CATCHY HOOK: Syncopated rhythmic theme with call-and-response (HARMONIZED!)
    comp.instrument("lead", &Instrument::synth_lead())
        .vibrato(5.5, 0.25)
        .delay(Delay::new(0.375, 0.3, 0.5))
        .saturation(Saturation::new(1.6, 0.5, 0.5))
        .volume(0.9)
        .at(16.5)
        // Call: Quick ascending motif
        .notes(&[C5, D5], 0.15)
        .wait(0.05)
        .note(&[DS5], 0.25)
        .wait(0.05)
        .note(&[G5], 0.5)
        .wait(0.3)
        // Response: Descending answer
        .notes(&[C6, AS5], 0.2)
        .wait(0.1)
        .notes(&[GS5, G5], 0.3)
        .wait(0.5)
        // Call 2: Rhythmic variation
        .note(&[DS5], 0.2)
        .wait(0.1)
        .notes(&[F5, G5], 0.15)
        .wait(0.05)
        .note(&[C6], 0.6)
        .wait(0.3)
        // Response 2: Final descent
        .notes(&[AS5, GS5], 0.2)
        .wait(0.1)
        .notes(&[G5, DS5], 0.3);

    // HARMONY LAYER: Thirds below main hook for thickness
    comp.instrument("lead_harmony", &Instrument::synth_lead())
        .vibrato(5.5, 0.25)
        .delay(Delay::new(0.375, 0.3, 0.5))
        .saturation(Saturation::new(1.6, 0.5, 0.5))
        .pan(-0.15) // Slightly left of main lead
        .volume(0.6) // Quieter than main
        .at(16.5)
        // Harmonized call
        .notes(&[GS4, B4], 0.15)
        .wait(0.05)
        .note(&[C5], 0.25)
        .wait(0.05)
        .note(&[DS5], 0.5)
        .wait(0.3)
        // Harmonized response
        .notes(&[GS5, F5], 0.2)
        .wait(0.1)
        .notes(&[E5, DS5], 0.3)
        .wait(0.5)
        // Harmonized call 2
        .note(&[C5], 0.2)
        .wait(0.1)
        .notes(&[CS5, DS5], 0.15) // CHROMATIC SPICE! CS5 instead of D5
        .wait(0.05)
        .note(&[GS5], 0.6)
        .wait(0.3)
        // Harmonized response 2
        .notes(&[F5, E5], 0.2)
        .wait(0.1)
        .notes(&[DS5, C5], 0.3);

    // === BREAKDOWN: DRAMATIC DECONSTRUCTION (24-28s) ===
    println!("💥 Section 4: Breakdown - Dramatic Deconstruction");

    // Strip down to just heavily processed theme fragments
    comp.instrument("glitch_theme", &Instrument::synth_lead())
        .bitcrusher(BitCrusher::new(3.0, 16.0, 0.9)) // HEAVY crushing
        .saturation(Saturation::new(2.5, 0.7, 0.7))
        .delay(Delay::new(0.333, 0.6, 0.8)) // Triplet delay for glitch
        .volume(0.6)
        .at(24.0)
        // Stuttering, fragmented theme
        .note(&[C4], 0.15)
        .wait(0.15)
        .note(&[D4], 0.1)
        .wait(0.2)
        .note(&[DS4], 0.15)
        .wait(0.3)
        .note(&[G4], 0.2)
        .wait(0.5)
        // Repeat with variation
        .note(&[C3], 0.1)
        .wait(0.1)
        .notes(&[D3, DS3], 0.08)
        .wait(0.2)
        .note(&[G3], 0.3)
        .wait(0.7)
        // Final glitchy burst
        .notes(&[C4, D4, DS4, G4], 0.05)
        .wait(0.1)
        .notes(&[G3, DS3, D3, C3], 0.05);

    // Minimal, sparse percussion - almost nothing
    comp.instrument("minimal_perc", &Instrument::sub_bass())
        .volume(0.4)
        .at(24.0)
        .pattern_start()
        .drum_grid(16, 0.125)
        .kick(&[0, 13]) // Only 2 kicks - super sparse
        .snare(&[7]) // Single snare
        .hihat(&[4, 11]) // Minimal hats
        .repeat(1);

    // Eerie, filtered pad with heavy modulation
    let breakdown_filter_lfo = LFO::new(Waveform::Triangle, 0.6, 0.8); // More intense
    let breakdown_filter_mod = ModRoute::new(breakdown_filter_lfo, ModTarget::FilterCutoff, 1.0);

    comp.instrument("breakdown_pad", &Instrument::warm_pad())
        .reverb(Reverb::new(0.9, 0.7, 0.6))
        .filter(Filter::new(FilterType::LowPass, 300.0, 1.2)) // Very dark
        .modulate(breakdown_filter_mod)
        .volume(0.3)
        .at(24.0)
        .note(&C3_MINOR, 4.0);

    // === BUILD-UP: Rising tension (28-32s) ===
    println!("⬆️  Section 5: Build-Up - Crescendo");

    // Theme fragments rising through octaves with filter sweep
    let filter_sweep = LFO::new(Waveform::Sine, 0.25, 0.7); // Slow sweep over 4 seconds
    let filter_mod = ModRoute::new(filter_sweep, ModTarget::FilterCutoff, 1.0);

    comp.instrument("buildup_theme", &Instrument::pluck())
        .filter(Filter::new(FilterType::LowPass, 300.0, 1.3))
        .modulate(filter_mod)
        .delay(Delay::new(0.25, 0.3, 0.6))
        .at(28.0)
        .volume(0.4)
        .notes(&[C3, D3, DS3, G3], 0.2)
        .wait(0.2)
        .volume(0.5)
        .notes(&[C4, D4, DS4, G4], 0.2)
        .wait(0.2)
        .volume(0.6)
        .notes(&[C5, D5, DS5, G5], 0.2)
        .wait(0.2)
        .volume(0.8)
        .notes(&[C6, D6, DS6, G6], 0.2);

    // Crescendo pad
    comp.instrument("buildup_pad", &Instrument::warm_pad())
        .reverb(Reverb::new(0.7, 0.6, 0.5))
        .at(28.0)
        .volume(0.2)
        .note(&C3_MINOR, 1.0)
        .volume(0.4)
        .note(&C3_MINOR, 1.0)
        .volume(0.6)
        .note(&C3_MINOR, 1.0)
        .volume(0.8)
        .note(&C3_MINOR, 1.0);

    // Accelerating drum roll
    comp.instrument("buildup_drums", &Instrument::sub_bass())
        .at(30.0)
        .drum_grid(32, 0.0625)
        .snare(&[0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30])
        .kick(&[0, 8, 16, 24]);

    // === DRAMATIC PAUSE: Silence before the drop (31.8-32.0s) ===
    // Reverse cymbal-like swell leading into silence
    comp.instrument("reverse_swell", &Instrument::synth_lead())
        .reverb(Reverb::new(0.9, 0.8, 0.8))
        .volume(0.4)
        .at(31.3)
        .slide(C7, C3, 0.5); // High to low sweep for reverse effect

    // (Natural gap at 31.8-32.0 - no instruments playing creates anticipation)

    // === FINALE: STAGGERED DROP for MAXIMUM IMPACT (32-42s) ===
    println!("🎆 Section 6: Finale - Theme Triumphant");

    // LAYER 1: Theme enters ALONE after silence (THE DROP!)
    comp.instrument("drop_theme_solo", &Instrument::synth_lead())
        .vibrato(6.0, 0.3)
        .delay(Delay::new(0.375, 0.3, 0.5))
        .volume(0.9)
        .at(32.0)
        .notes(&[C5, D5, DS5, G5], 0.4); // Just the core motif - POWERFUL entrance

    // LAYER 2: Pad enters 0.8s later
    comp.instrument("finale_pad", &Instrument::warm_pad())
        .reverb(Reverb::new(0.6, 0.5, 0.45))
        .chorus(Chorus::new(0.4, 3.0, 0.35))
        .saturation(Saturation::new(1.3, 0.4, 0.5))
        .volume(0.75)
        .at(32.8) // Delayed entry
        .chords(&[&C3_MINOR, &GS3_MAJOR, &DS3_MAJOR, &G3_MAJOR], 2.5);

    // Theme with full ornamental showcase AND SURPRISE harmonic twist
    comp.instrument("finale_theme", &Instrument::synth_lead())
        .vibrato(6.0, 0.3)
        .delay(Delay::new(0.375, 0.3, 0.5))
        .reverb(Reverb::new(0.5, 0.5, 0.4))
        .pan(0.2) // Slight right pan for stereo interest
        .volume(0.85)
        .at(32.5)
        // Opening: Trill into turn
        .trill(C5, D5, 8, 0.1)
        .wait(0.2)
        .turn(DS5, 0.2)
        .wait(0.2)
        // Main theme phrase
        .slide(DS5, G5, 0.6)
        .wait(0.2)
        .mordent(G5, 0.15)
        .wait(0.3)
        // **SURPRISE!** Unexpected high leap with inverted turn
        .note(&[C6], 0.3)
        .wait(0.1)
        .inverted_turn(DS6, 0.2) // High inverted turn - surprising!
        .wait(0.2)
        // Descending cascade with inverted mordents
        .inverted_mordent(C6, 0.15)
        .wait(0.1)
        .notes(&[AS5, GS5], 0.2)
        .wait(0.1)
        .inverted_mordent(G5, 0.15)
        .wait(0.1)
        .notes(&[F5, DS5, D5, C5], 0.2)
        .wait(0.4)
        // Final statement - triumphant theme
        .notes(&[C5, D5, DS5, G5, C6], 0.4);

    // LAYER 3: Bass enters next for low-end POWER (staggered drop!)
    comp.instrument("finale_bass", &Instrument::sub_bass())
        .compressor(Compressor::new(0.3, 4.0, 0.01, 0.1, 1.5))
        .volume(0.8) // Reduced from 0.9 for mix balance
        .at(33.2) // DELAYED - enters after theme and pad for impact!
        .notes(&[C2, C2, GS1, GS1, DS2, DS2, G2, G2], 0.5)
        .wait(0.5)
        .notes(&[C2, C2, GS1, GS1, DS2, DS2, G2, G2], 0.5);

    // LAYER 4: Drums enter LAST for maximum IMPACT (final staggered layer!)
    comp.instrument("finale_drums", &Instrument::sub_bass())
        .compressor(Compressor::new(0.25, 5.0, 0.005, 0.08, 1.8))
        .volume(0.85) // Reduced from 1.0 for mix balance
        .at(33.6) // DELAYED - full beat drops only after everything else builds!
        .swing(0.58) // Subtle swing for climax
        .pattern_start()
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 6, 10, 12, 14])
        .snare(&[4, 12])
        .hihat(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
        .repeat(4);

    // === OUTRO: Theme dissolves (42-52s) ===
    println!("🌅 Section 7: Outro - Peaceful Resolution");

    // Cascading theme one final time with stereo spread
    comp.instrument("outro_cascade_left", &Instrument::pluck())
        .reverb(Reverb::new(0.85, 0.6, 0.6))
        .delay(Delay::new(0.5, 0.4, 0.7))
        .pan(-0.4) // Panned left
        .volume(0.6)
        .at(42.0)
        .notes(&[C5, D5, DS5, G5], 0.3)
        .wait(1.0)
        .cascade(&[C4, D4, DS4, G4], 3.0, 0.2)
        .wait(1.5)
        .cascade(&[C3, D3, DS3, G3], 4.0, 0.3);

    // Mirrored cascade on the right
    comp.instrument("outro_cascade_right", &Instrument::pluck())
        .reverb(Reverb::new(0.85, 0.6, 0.6))
        .delay(Delay::new(0.5, 0.4, 0.7))
        .pan(0.4) // Panned right
        .volume(0.6)
        .at(42.3) // Slightly delayed for stereo movement
        .notes(&[G5, DS5, D5, C5], 0.3) // Theme inverted
        .wait(1.0)
        .cascade(&[G4, DS4, D4, C4], 3.0, 0.2)
        .wait(1.5)
        .cascade(&[G3, DS3, D3, C3], 4.0, 0.3);

    // Fading pad with theme embedded
    comp.instrument("outro_pad", &Instrument::warm_pad())
        .reverb(Reverb::new(0.9, 0.7, 0.7))
        .chorus(Chorus::new(0.2, 4.0, 0.6))
        .at(42.0)
        .volume(0.7)
        .note(&C3_MINOR, 3.0)
        .fade_to(0.3, 3.0)
        .note(&C3_MINOR, 4.0);

    // Final melodic statement - theme one last time before silence
    comp.instrument("outro_theme_finale", &Instrument::synth_lead())
        .reverb(Reverb::new(0.9, 0.7, 0.8))
        .delay(Delay::new(0.75, 0.5, 0.7)) // Long delay for ethereal tail
        .filter(Filter::new(FilterType::LowPass, 1200.0, 0.5))
        .volume(0.5)
        .at(46.0)
        // Simple, peaceful statement of the C-D-DS-G theme
        .notes(&[C4, D4, DS4, G4], 0.6)
        .wait(0.4)
        // With a gentle chromatic resolution: G->GS->G
        .note(&[GS4], 0.4)
        .note(&[G4], 0.8)
        .wait(0.2)
        // Final note - back to C (home)
        .note(&[C4], 2.0);

    // Final sustaining note
    comp.instrument("outro_final", &Instrument::warm_pad())
        .reverb(Reverb::new(0.95, 0.8, 0.8))
        .volume(0.5)
        .at(48.0)
        .note(&[C3], 4.0);

    println!("\n▶️  Playing 'Digital Reverie' v8.0 FINAL - Maximum Impact...");
    println!("    Duration: ~52 seconds\n");
    println!("    💎 Key Features:");
    println!("    • Recurring C-D-DS-G motif that develops throughout");
    println!("    • Syncopated call-and-response hook with harmonization");
    println!("    • Wide stereo field with panned elements");
    println!("    • Dramatic glitchy breakdown with heavy bitcrushing");
    println!("    • Staggered drop (theme→pad→bass→drums) for maximum impact");
    println!("    • Chromatic spice note (CS5) in harmony layer");
    println!("    • Verse countermelody for textural depth");
    println!("    • Final melodic statement with chromatic resolution");
    println!("    • Inverted ornaments and surprise high leap");
    println!("    • Arpeggiated textures and harmonized melodies");
    println!("    • Reverse cymbal swell effect");
    println!("    • Creative drum fills and transitions");
    println!("    • LFO modulation on filters and volume");
    println!("    • Euclidean-inspired polyrhythms");

    let mixer = comp.into_mixer();
    let engine = AudioEngine::new()?;
    engine.play_mixer(&mixer)?;

    println!("\n✨ Performance complete - Iteration 6 (v8.0 FINAL)!");
    println!("   'Digital Reverie' - A complete musical journey with maximum impact");
    println!("   Thank you for listening! 🎵");

    Ok(())
}
