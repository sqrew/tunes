use tunes::prelude::*;

fn main() -> anyhow::Result<()> {
    let engine = AudioEngine::new()?;

    println!("🎵 Testing Sample Transformation Methods!");
    println!("=========================================\n");

    // Test 1: Normalize
    println!("1. Normalize - brings quiet samples to full volume");
    engine.play_sample("sfx_beep.wav")
        .normalize()
        .volume(0.7);
    std::thread::sleep(std::time::Duration::from_millis(600));

    // Test 2: Gain
    println!("2. Gain - amplify sample before effects");
    engine.play_sample("sfx_beep.wav")
        .gain(0.5)  // Halve the amplitude
        .volume(1.0);
    std::thread::sleep(std::time::Duration::from_millis(600));

    // Test 3: Reverse
    println!("3. Reverse - play backwards!");
    engine.play_sample("sfx_boop.wav")
        .reverse();
    std::thread::sleep(std::time::Duration::from_millis(600));

    // Test 4: Fade In
    println!("4. Fade In - smooth entrance");
    engine.play_sample("sfx_beep.wav")
        .fade_in(0.3);
    std::thread::sleep(std::time::Duration::from_millis(800));

    // Test 5: Fade Out
    println!("5. Fade Out - smooth exit");
    engine.play_sample("sfx_boop.wav")
        .fade_out(0.2);
    std::thread::sleep(std::time::Duration::from_millis(600));

    // Test 6: Time Stretch (slower, same pitch)
    println!("6. Time Stretch (2x slower, same pitch)");
    engine.play_sample("sfx_beep.wav")
        .time_stretch(2.0);
    std::thread::sleep(std::time::Duration::from_millis(1200));

    // Test 7: Time Stretch (faster, same pitch)
    println!("7. Time Stretch (0.5x faster, same pitch)");
    engine.play_sample("sfx_boop.wav")
        .time_stretch(0.5);
    std::thread::sleep(std::time::Duration::from_millis(400));

    // Test 8: Pitch Shift Up
    println!("8. Pitch Shift (+7 semitones)");
    engine.play_sample("sfx_beep.wav")
        .pitch_shift(7.0);
    std::thread::sleep(std::time::Duration::from_millis(600));

    // Test 9: Pitch Shift Down
    println!("9. Pitch Shift (-5 semitones)");
    engine.play_sample("sfx_boop.wav")
        .pitch_shift(-5.0);
    std::thread::sleep(std::time::Duration::from_millis(600));

    // Test 10: Chain multiple transformations!
    println!("10. Chain - normalize + reverse + fade in + pitch shift");
    engine.play_sample("sfx_beep.wav")
        .normalize()
        .reverse()
        .fade_in(0.1)
        .pitch_shift(-3.0);
    std::thread::sleep(std::time::Duration::from_millis(800));

    // Test 11: Transformations + Effects!
    println!("11. Transformations + Effects - time stretch + reverb + delay");
    engine.play_sample("sfx_boop.wav")
        .time_stretch(1.5)
        .pitch_shift(5.0)
        .reverb(Reverb::hall())
        .delay(Delay::new(0.2, 0.3, 0.4));
    std::thread::sleep(std::time::Duration::from_millis(1500));

    // Test 12: Everything combined!
    println!("12. ULTIMATE CHAIN - all features!");
    engine.play_sample("sfx_beep.wav")
        .normalize()              // Sample transform
        .reverse()                // Sample transform
        .fade_in(0.15)           // Sample transform
        .pitch_shift(7.0)        // Sample transform
        .volume(0.8)             // Track setting
        .pan(0.3)                // Track setting
        .reverb(Reverb::new(0.7, 0.4, 0.3))  // Effect
        .filter(Filter::low_pass(1500.0, 0.8)); // Effect
    std::thread::sleep(std::time::Duration::from_millis(1500));

    println!("\n✨ All sample transformation tests completed!");
    println!("The builder pattern makes it SO easy to transform samples on the fly!");

    Ok(())
}
