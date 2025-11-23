use fundsp::prelude::*;
use insta_fun::prelude::*;

/*
Utility, Shaping & Rate Examples
Representative snapshots only (MAYA/KISS):
- Clipping after gain boost
- Waveshaping (SoftCrush)
- Mapping (absolute value) transform
- Oversampling a simple oscillator
- Resampling with dynamic speed ramp (0.5x -> 1.5x)

Design notes:
- Each example demonstrates one concept clearly.
- Parameters chosen to produce visibly distinct charts and audible artifacts.
- Charts use explicit titles for clarity; WAV output captures 1 second of audio.
*/

fn main() {
    const CHART_SAMPLES: usize = 2000;
    const ONE_SECOND_SAMPLES: usize = DEFAULT_SR as usize;

    // Standard chart config helper.
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

    // Chart config with inputs (used where we want to compare original vs processed).
    let chart_cfg_inputs = |title: &str| {
        let chart = SvgChartConfigBuilder::default()
            .chart_title(title)
            .with_inputs(true)
            .build()
            .unwrap();
        SnapshotConfigBuilder::default()
            .num_samples(CHART_SAMPLES)
            .output_mode(chart)
            .build()
            .unwrap()
    };

    // WAV config helper (1 second, 16-bit).
    let wav_cfg = || {
        SnapshotConfigBuilder::default()
            .num_samples(ONE_SECOND_SAMPLES)
            .output_mode(WavOutput::Wav16)
            .build()
            .unwrap()
    };

    /* =========================================================
       1. Clip To Range
       =========================================================
       Process: Gain boost (x2) then clip to [-0.5, 0.5].
       Shows hard limiting of peaks (visible flat tops).
    */
    let clip_proc = mul(2.0) >> clip_to(-0.5, 0.5);
    assert_audio_unit_snapshot!(
        "util_clip_to_m0_5_p0_5_mul2_sine440",
        clip_proc.clone(),
        InputSource::Unit(Box::new(sine_hz::<f32>(440.0))),
        chart_cfg_inputs("util_clip_to_m0_5_p0_5_mul2_sine440")
    );
    assert_audio_unit_snapshot!(
        "util_clip_to_m0_5_p0_5_mul2_sine440",
        clip_proc,
        InputSource::Unit(Box::new(sine_hz::<f32>(440.0))),
        wav_cfg()
    );

    /* =========================================================
       2. Waveshaping (SoftCrush)
       =========================================================
       SoftCrush(0.5) introduces gentle nonlinear distortion.
       Input amplitude moderate to emphasize harmonic enrichment.
    */
    let softcrush_chain = sine_hz::<f32>(220.0) >> shape(SoftCrush(0.5));
    assert_audio_unit_snapshot!(
        "util_shape_softcrush0_5_sine220",
        softcrush_chain.clone(),
        InputSource::None,
        chart_cfg("util_shape_softcrush0_5_sine220")
    );
    assert_audio_unit_snapshot!(
        "util_shape_softcrush0_5_sine220",
        softcrush_chain,
        InputSource::None,
        wav_cfg()
    );

    /* =========================================================
       3. Map Absolute Value
       =========================================================
       Transform: |sine(110 Hz)|
       Demonstrates channel-wise function mapping & rectification.
       Chart includes input for comparison (half-wave folding).
    */
    // Map (absolute value) applied to externally supplied sine input.
    let abs_map = map(|frame: &Frame<f32, U1>| frame[0].abs());
    assert_audio_unit_snapshot!(
        "util_map_abs_sine110",
        abs_map.clone(),
        InputSource::Unit(Box::new(sine_hz::<f32>(110.0))),
        chart_cfg_inputs("util_map_abs_sine110")
    );
    assert_audio_unit_snapshot!(
        "util_map_abs_sine110",
        abs_map,
        InputSource::Unit(Box::new(sine_hz::<f32>(110.0))),
        wav_cfg()
    );

    /* =========================================================
       4. Oversample (2x)
       =========================================================
       Correct usage: Enclose a generator with no inputs inside oversample(...)
       and provide no external input (InputSource::None). This avoids channel
       mismatches that occur if you try to feed a signal into oversample as if
       it were a processor stage.
       Oversampling a simple 440 Hz sine. Audible result same pitch;
       illustrates structural use rather than a dramatic sonic change.
       (Generally used before nonlinear processing to reduce aliasing.)
    */
    let oversampled = oversample(sine_hz::<f32>(440.0)); // Generator enclosed; no external input.
    assert_audio_unit_snapshot!(
        "util_oversample2x_sine440",
        oversampled.clone(),
        InputSource::None,
        chart_cfg("util_oversample2x_sine440")
    );
    assert_audio_unit_snapshot!(
        "util_oversample2x_sine440",
        oversampled,
        InputSource::None,
        wav_cfg()
    );

    /* =========================================================
       5. Resample with Dynamic Speed Ramp
       =========================================================
       Goal: Demonstrate time-warp/pitch glide via a single control (speed) input.

       Correct pattern (to avoid output mismatch panics):
       - Enclose a generator with ZERO inputs: resample(sine_hz::<f32>(440.0))
       - Supply ONE external input channel providing playback speed: 0.5x .. 1.5x
       - Do NOT attempt to feed the audio source externally (the enclosed generator
         already produces audio frames; only speed is externally driven).

       Speed design:
       ramp 0..1 at 0.25 Hz -> lerp(0.5, 1.5, ramp) giving smooth glide 0.5x → 1.5x.

       Result: Pitch rises because effective phase increment scales with speed.

       Panics previously seen (`assertion failed: output.len() == self.outputs()`) can occur if:
       - The enclosed generator accidentally has inputs (violates U0 requirement).
       - Multiple channels are supplied as InputSource but resample expects exactly one (speed).
       - Audio is mistakenly passed in as the InputSource instead of just speed.

       This example follows the safe pattern.
    */
    // Single-channel speed control (0.5x → 1.5x glide).
    let speed_node = ramp_hz::<f32>(0.25) >> map(|f: &Frame<f32, U1>| lerp(0.5f32, 1.5f32, f[0]));
    // Enclosed generator with no inputs (sine 440 Hz).
    let resampler = resample(sine_hz::<f32>(440.0));
    // Provide ONLY the speed channel as input.
    assert_audio_unit_snapshot!(
        "util_resample_speed_ramp_0_5_to_1_5_sine440",
        resampler.clone(),
        InputSource::Unit(Box::new(speed_node.clone())),
        chart_cfg("util_resample_speed_ramp_0_5_to_1_5_sine440")
    );
    assert_audio_unit_snapshot!(
        "util_resample_speed_ramp_0_5_to_1_5_sine440",
        resampler,
        InputSource::Unit(Box::new(speed_node)),
        wav_cfg()
    );
}
