use fundsp::prelude::*;
use insta_fun::prelude::*;

/// Filters snapshot examples driven from main.
///
/// For each filter:
/// - SVG chart: 2000 samples, impulse input (impulse response)
/// - WAV audio: 1 second at DEFAULT_SR (44100 Hz)
///   driven by 440 Hz sine via network (InputSource::None)
///
/// How to run:
///   cargo run --example filters
///
/// To update Insta snapshots:
///   INSTA_UPDATE=auto cargo run --example filters
fn main() {
    const CHART_SAMPLES: usize = 2000;
    const ONE_SECOND_SAMPLES: usize = fundsp::DEFAULT_SR as usize;

    // Helpers to build configs per snapshot
    let chart_cfg = || {
        SnapshotConfigBuilder::default()
            .num_samples(CHART_SAMPLES)
            .build()
            .unwrap()
        // output_mode defaults to SvgChart, macro will set chart title from name
    };
    let wav_cfg = || {
        SnapshotConfigBuilder::default()
            .output_mode(WavOutput::Wav16)
            .num_samples(ONE_SECOND_SAMPLES)
            .build()
            .unwrap()
    };

    // --- State Variable Filters and biquad-like filters ---

    // Lowpass 1 kHz, Q=0.707
    assert_audio_unit_snapshot!(
        "filter_svf_lowpass_1k_q0_707",
        lowpass_hz(1_000.0, 0.707),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_svf_lowpass_1k_q0_707",
        sine_hz::<f32>(440.0) >> lowpass_hz(1_000.0, 0.707),
        InputSource::None,
        wav_cfg()
    );

    // Highpass 1 kHz, Q=0.707
    assert_audio_unit_snapshot!(
        "filter_svf_highpass_1k_q0_707",
        highpass_hz(1_000.0, 0.707),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_svf_highpass_1k_q0_707",
        sine_hz::<f32>(440.0) >> highpass_hz(1_000.0, 0.707),
        InputSource::None,
        wav_cfg()
    );

    // Bandpass 1 kHz, Q=1.0
    assert_audio_unit_snapshot!(
        "filter_svf_bandpass_1k_q1_0",
        bandpass_hz(1_000.0, 1.0),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_svf_bandpass_1k_q1_0",
        sine_hz::<f32>(440.0) >> bandpass_hz(1_000.0, 1.0),
        InputSource::None,
        wav_cfg()
    );

    // Notch 1 kHz, Q=1.0
    assert_audio_unit_snapshot!(
        "filter_svf_notch_1k_q1_0",
        notch_hz(1_000.0, 1.0),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_svf_notch_1k_q1_0",
        sine_hz::<f32>(440.0) >> notch_hz(1_000.0, 1.0),
        InputSource::None,
        wav_cfg()
    );

    // Peak 1 kHz, Q=1.0
    assert_audio_unit_snapshot!(
        "filter_peak_1k_q1_0",
        peak_hz(1_000.0, 1.0),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_peak_1k_q1_0",
        sine_hz::<f32>(440.0) >> peak_hz(1_000.0, 1.0),
        InputSource::None,
        wav_cfg()
    );

    // Bell 1 kHz, Q=0.707, gain=+3.5 dB approx (1.5 linear)
    assert_audio_unit_snapshot!(
        "filter_bell_1k_q0_707_gain1_5",
        bell_hz(1_000.0, 0.707, 1.5),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_bell_1k_q0_707_gain1_5",
        sine_hz::<f32>(440.0) >> bell_hz(1_000.0, 0.707, 1.5),
        InputSource::None,
        wav_cfg()
    );

    // Low shelf 500 Hz, Q=0.707, gain=1.5
    assert_audio_unit_snapshot!(
        "filter_lowshelf_500hz_q0_707_gain1_5",
        lowshelf_hz(500.0, 0.707, 1.5),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_lowshelf_500hz_q0_707_gain1_5",
        sine_hz::<f32>(440.0) >> lowshelf_hz(500.0, 0.707, 1.5),
        InputSource::None,
        wav_cfg()
    );

    // High shelf 2 kHz, Q=0.707, gain=1.5
    assert_audio_unit_snapshot!(
        "filter_highshelf_2k_q0_707_gain1_5",
        highshelf_hz(2_000.0, 0.707, 1.5),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_highshelf_2k_q0_707_gain1_5",
        sine_hz::<f32>(440.0) >> highshelf_hz(2_000.0, 0.707, 1.5),
        InputSource::None,
        wav_cfg()
    );

    // --- Resonant/ladder and simple poles ---

    // Moog ladder 1 kHz, Q=0.5
    assert_audio_unit_snapshot!(
        "filter_moog_lowpass_1k_q0_5",
        moog_hz(1_000.0, 0.5),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_moog_lowpass_1k_q0_5",
        sine_hz::<f32>(440.0) >> moog_hz(1_000.0, 0.5),
        InputSource::None,
        wav_cfg()
    );

    // Lowrez 1 kHz, Q=1.0
    assert_audio_unit_snapshot!(
        "filter_lowrez_lowpass_1k_q1_0",
        lowrez_hz(1_000.0, 1.0),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_lowrez_lowpass_1k_q1_0",
        sine_hz::<f32>(440.0) >> lowrez_hz(1_000.0, 1.0),
        InputSource::None,
        wav_cfg()
    );

    // Bandrez 1 kHz, Q=3.0
    assert_audio_unit_snapshot!(
        "filter_bandrez_bandpass_1k_q3_0",
        bandrez_hz(1_000.0, 3.0),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_bandrez_bandpass_1k_q3_0",
        sine_hz::<f32>(440.0) >> bandrez_hz(1_000.0, 3.0),
        InputSource::None,
        wav_cfg()
    );

    // One-pole lowpass 800 Hz
    assert_audio_unit_snapshot!(
        "filter_lowpole_800hz",
        lowpole_hz(800.0),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_lowpole_800hz",
        sine_hz::<f32>(440.0) >> lowpole_hz(800.0),
        InputSource::None,
        wav_cfg()
    );

    // One-pole highpass 300 Hz
    assert_audio_unit_snapshot!(
        "filter_highpole_300hz",
        highpole_hz(300.0),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_highpole_300hz",
        sine_hz::<f32>(440.0) >> highpole_hz(300.0),
        InputSource::None,
        wav_cfg()
    );

    // --- Morphing SVF mode ---

    // Morph lowpass at 1 kHz, Q=0.8, morph=-1 (lowpass)
    assert_audio_unit_snapshot!(
        "filter_morph_lowpass_1k_q0_8",
        morph_hz(1_000.0, 0.8, -1.0),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_morph_lowpass_1k_q0_8",
        sine_hz::<f32>(440.0) >> morph_hz(1_000.0, 0.8, -1.0),
        InputSource::None,
        wav_cfg()
    );

    // --- Utility ---

    // DC blocker 10 Hz
    assert_audio_unit_snapshot!(
        "filter_dcblock_10hz",
        dcblock_hz(10.0),
        InputSource::impulse(),
        chart_cfg()
    );
    assert_audio_unit_snapshot!(
        "filter_dcblock_10hz",
        sine_hz::<f32>(440.0) >> dcblock_hz(10.0),
        InputSource::None,
        wav_cfg()
    );
}
