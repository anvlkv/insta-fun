use fundsp::prelude::*;
use insta_fun::prelude::*;

fn main() {
    const CHART_SAMPLES: usize = 2000;
    const ONE_SECOND_SAMPLES: usize = fundsp::DEFAULT_SR as usize;
    const SR_F32: f32 = fundsp::DEFAULT_SR as f32;

    // Helpers to build configs per snapshot:
    // - chart_cfg(): default SvgChart with 2000 samples (title is set by macro from literal name)
    // - wav_cfg(): Wav16 with 1 second worth of samples
    let chart_cfg = || {
        SnapshotConfigBuilder::default()
            .num_samples(CHART_SAMPLES)
            .build()
            .unwrap()
        // output_mode defaults to SvgChart; macro will set chart title via name
    };
    let wav_cfg = || {
        SnapshotConfigBuilder::default()
            .output_mode(WavOutput::Wav16)
            .num_samples(ONE_SECOND_SAMPLES)
            .build()
            .unwrap()
    };

    // A reusable musical input for FX that need a drive signal.

    /* Time-based modulation FX */

    // Chorus (mono, 5 voices) - parameters based on docs examples
    // seed: 1; separation: 15ms; variation: 5ms; mod_frequency: 0.2 Hz
    assert_audio_unit_snapshot!(
        "fx_chorus",
        chorus(1, 0.015, 0.005, 0.2),
        InputSource::sine(440.0, SR_F32),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "fx_chorus",
        chorus(1, 0.015, 0.005, 0.2),
        InputSource::sine(440.0, SR_F32),
        wav_cfg()
    );

    // Flanger with sine-modulated delay between 5ms..10ms and strong feedback
    let min_d = 0.005f32;
    let max_d = 0.010f32;
    let center = (min_d + max_d) * 0.5;
    let half_range = (max_d - min_d) * 0.5;
    let flanger_node = flanger(0.9, min_d, max_d, move |t: f32| {
        center + half_range * sin_hz(0.1, t)
    });
    assert_audio_unit_snapshot!(
        "fx_flanger",
        flanger_node.clone(),
        InputSource::sine(440.0, SR_F32),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "fx_flanger",
        flanger_node,
        InputSource::sine(440.0, SR_F32),
        wav_cfg()
    );

    // Phaser with feedback 0.5 and simple 0..1 phase modulation
    assert_audio_unit_snapshot!(
        "fx_phaser",
        phaser(0.5, |t: f32| sin_hz(0.2, t) * 0.5 + 0.5),
        InputSource::sine(440.0, SR_F32),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "fx_phaser",
        phaser(0.5, |t: f32| sin_hz(0.2, t) * 0.5 + 0.5),
        InputSource::sine(440.0, SR_F32),
        wav_cfg()
    );

    /* Dynamics */

    // Limiter with a hot input (multiply by 2x before limiting) to demonstrate gain control
    let limiting_chain = mul(2.0) >> limiter(0.01, 0.1);
    assert_audio_unit_snapshot!(
        "dyn_limiter_mul2_in",
        limiting_chain.clone(),
        InputSource::sine(440.0, SR_F32),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "dyn_limiter_mul2_in",
        limiting_chain,
        InputSource::sine(440.0, SR_F32),
        wav_cfg()
    );

    /* Resonator (time-domain ringing) */

    // Constant-gain bandpass resonator at 440 Hz with ~50 Hz bandwidth
    assert_audio_unit_snapshot!(
        "fx_resonator_440hz_bw50",
        resonator_hz(440.0, 50.0),
        InputSource::sine(440.0, SR_F32),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "fx_resonator_440hz_bw50",
        resonator_hz(440.0, 50.0),
        InputSource::sine(440.0, SR_F32),
        wav_cfg()
    );

    /* Additional spatial/time effects (driven from generator chains) */

    // Stereo reverb (32-channel FDN): room 10m, time 2s, damping 0.5.
    // Generate stereo by stacking two mono sines, then feed into reverb.
    assert_audio_unit_snapshot!(
        "fx_reverb_stereo_room10m_time2s_damp0_5",
        (sine_hz::<f32>(220.0) | sine_hz::<f32>(220.0)) >> reverb_stereo(10.0, 2.0, 0.5),
        InputSource::None,
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "fx_reverb_stereo_room10m_time2s_damp0_5",
        (sine_hz::<f32>(220.0) | sine_hz::<f32>(220.0)) >> reverb_stereo(10.0, 2.0, 0.5),
        InputSource::None,
        wav_cfg()
    );

    // Simple delay of 250 ms on a 440 Hz sine.
    assert_audio_unit_snapshot!(
        "fx_delay_250ms_sine_440",
        sine_hz::<f32>(440.0) >> delay(0.25),
        InputSource::None,
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "fx_delay_250ms_sine_440",
        sine_hz::<f32>(440.0) >> delay(0.25),
        InputSource::None,
        wav_cfg()
    );

    /* Noise generators (no input) */

    // White noise
    assert_audio_unit_snapshot!("noise_white", white(), InputSource::None, chart_cfg());
    assert_audio_unit_snapshot!("noise_white", white(), InputSource::None, wav_cfg());

    // Pink noise (requires generic type parameter)
    assert_audio_unit_snapshot!("noise_pink", pink::<f32>(), InputSource::None, chart_cfg());
    assert_audio_unit_snapshot!("noise_pink", pink::<f32>(), InputSource::None, wav_cfg());

    // Brown noise (requires generic type parameter)
    assert_audio_unit_snapshot!(
        "noise_brown",
        brown::<f32>(),
        InputSource::None,
        chart_cfg()
    );
    assert_audio_unit_snapshot!("noise_brown", brown::<f32>(), InputSource::None, wav_cfg());

    // MLS noise
    assert_audio_unit_snapshot!("noise_mls", mls(), InputSource::None, chart_cfg());
    assert_audio_unit_snapshot!("noise_mls", mls(), InputSource::None, wav_cfg());
}
