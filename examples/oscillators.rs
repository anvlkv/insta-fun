use fundsp::prelude::*;
use insta_fun::prelude::*;

/// Oscillators snapshot examples driven from main.
///
/// For each oscillator:
/// - SVG chart: 2000 samples
/// - WAV audio: 1 second at DEFAULT_SR (44100 Hz)
///
/// How to run:
///   cargo run --example oscillators
///
/// To update Insta snapshots, you may want:
///   INSTA_UPDATE=auto cargo run --example oscillators
fn main() {
    const CHART_SAMPLES: usize = 2000;
    const ONE_SECOND_SAMPLES: usize = fundsp::DEFAULT_SR as usize;

    // Helpers to build configs.
    let chart_cfg = |title: &str| {
        let chart = SvgChartConfigBuilder::default()
            .chart_title(title)
            .build()
            .unwrap();
        SnapshotConfigBuilder::default()
            .num_samples(CHART_SAMPLES)
            .output_mode(chart)
            .build()
            .unwrap()
    };
    let wav_cfg = |samples: usize| {
        SnapshotConfigBuilder::default()
            .output_mode(WavOutput::Wav16)
            .num_samples(samples)
            .build()
            .unwrap()
    };

    /* Core oscillators */

    // Sine 440 Hz
    assert_audio_unit_snapshot!(
        "osc_sine_440hz",
        sine_hz::<f32>(440.0),
        InputSource::None,
        chart_cfg("osc_sine_440hz")
    );
    assert_audio_unit_snapshot!(
        "osc_sine_440hz",
        sine_hz::<f32>(440.0),
        InputSource::None,
        wav_cfg(ONE_SECOND_SAMPLES)
    );

    // Saw 440 Hz
    assert_audio_unit_snapshot!(
        "osc_saw_440hz",
        saw_hz(440.0),
        InputSource::None,
        chart_cfg("osc_saw_440hz")
    );
    assert_audio_unit_snapshot!(
        "osc_saw_440hz",
        saw_hz(440.0),
        InputSource::None,
        wav_cfg(ONE_SECOND_SAMPLES)
    );

    // Square 440 Hz
    assert_audio_unit_snapshot!(
        "osc_square_440hz",
        square_hz(440.0),
        InputSource::None,
        chart_cfg("osc_square_440hz")
    );
    assert_audio_unit_snapshot!(
        "osc_square_440hz",
        square_hz(440.0),
        InputSource::None,
        wav_cfg(ONE_SECOND_SAMPLES)
    );

    // Triangle 440 Hz
    assert_audio_unit_snapshot!(
        "osc_triangle_440hz",
        triangle_hz(440.0),
        InputSource::None,
        chart_cfg("osc_triangle_440hz")
    );
    assert_audio_unit_snapshot!(
        "osc_triangle_440hz",
        triangle_hz(440.0),
        InputSource::None,
        wav_cfg(ONE_SECOND_SAMPLES)
    );

    /* Wavetable and band-limited variants */

    // Soft saw 440 Hz
    assert_audio_unit_snapshot!(
        "osc_soft_saw_440hz",
        soft_saw_hz(440.0),
        InputSource::None,
        chart_cfg("osc_soft_saw_440hz")
    );
    assert_audio_unit_snapshot!(
        "osc_soft_saw_440hz",
        soft_saw_hz(440.0),
        InputSource::None,
        wav_cfg(ONE_SECOND_SAMPLES)
    );

    // Organ 440 Hz
    assert_audio_unit_snapshot!(
        "osc_organ_440hz",
        organ_hz(440.0),
        InputSource::None,
        chart_cfg("osc_organ_440hz")
    );
    assert_audio_unit_snapshot!(
        "osc_organ_440hz",
        organ_hz(440.0),
        InputSource::None,
        wav_cfg(ONE_SECOND_SAMPLES)
    );

    // Hammond 440 Hz
    assert_audio_unit_snapshot!(
        "osc_hammond_440hz",
        hammond_hz(440.0),
        InputSource::None,
        chart_cfg("osc_hammond_440hz")
    );
    assert_audio_unit_snapshot!(
        "osc_hammond_440hz",
        hammond_hz(440.0),
        InputSource::None,
        wav_cfg(ONE_SECOND_SAMPLES)
    );

    /* Utility/LFO-style oscillator */

    // Ramp 1 Hz (0..1), not bandlimited
    assert_audio_unit_snapshot!(
        "osc_ramp_1hz",
        ramp_hz::<f32>(1.0),
        InputSource::None,
        chart_cfg("osc_ramp_1hz")
    );
    assert_audio_unit_snapshot!(
        "osc_ramp_1hz",
        ramp_hz::<f32>(1.0),
        InputSource::None,
        wav_cfg(ONE_SECOND_SAMPLES)
    );
}
