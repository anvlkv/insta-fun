use fundsp::prelude::*;
use insta_fun::prelude::*;

/// Modulation & Control examples:
/// - ADSR envelope applied to a tone
/// - LFO amplitude modulation
/// - Sample & Hold on noise
/// - Amplitude follower extracting envelope
///
/// Representative, single-parameter snapshots only (MAYA/KISS).
fn main() {
    const CHART_SAMPLES: usize = 2000;
    const ONE_SECOND_SAMPLES: usize = DEFAULT_SR as usize;

    // Chart config helper (title injected per snapshot name by macro unless explicitly overridden).
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

    // WAV config helper (1 second, 16-bit).
    let wav_cfg = || {
        SnapshotConfigBuilder::default()
            .num_samples(ONE_SECOND_SAMPLES)
            .output_mode(WavOutput::Wav16)
            .build()
            .unwrap()
    };

    /* =========================================================
       ADSR Envelope
       =========================================================
       Attack 10 ms, Decay 100 ms, Sustain 0.70, Release 200 ms.
       Driven by an impulse gate start; envelope multiplied with a 220 Hz sine.
    */
    assert_audio_unit_snapshot!(
        "mod_adsr_a10ms_d100ms_s0_70_r200ms",
        adsr_live(0.01, 0.10, 0.70, 0.20) * sine_hz::<f32>(220.0),
        InputSource::impulse(),
        chart_cfg("mod_adsr_a10ms_d100ms_s0_70_r200ms")
    );
    assert_audio_unit_snapshot!(
        "mod_adsr_a10ms_d100ms_s0_70_r200ms",
        adsr_live(0.01, 0.10, 0.70, 0.20) * sine_hz::<f32>(220.0),
        InputSource::impulse(),
        wav_cfg()
    );

    /* =========================================================
       LFO (native generator)
       =========================================================
       Native LFO generator using time-varying frequency.
       lfo(|t: f32| exp(-t)) — frequency decays exponentially over time.
       Representative control signal; no external input.
    */
    assert_audio_unit_snapshot!(
        "mod_lfo",
        lfo(|t: f32| exp(-t)),
        InputSource::None,
        chart_cfg("mod_lfo")
    );
    assert_audio_unit_snapshot!(
        "mod_lfo",
        lfo(|t: f32| exp(-t)),
        InputSource::None,
        wav_cfg()
    );

    /* =========================================================
       Sample & Hold
       =========================================================
       White noise sampled at 5 Hz with variability 0.2.
       Produces stepped random levels (good control source exemplar).
    */
    assert_audio_unit_snapshot!(
        "mod_sample_hold_5hz_var0_2",
        hold_hz(5.0, 0.2),
        InputSource::impulse(),
        chart_cfg("mod_sample_hold_5hz_var0_2")
    );
    assert_audio_unit_snapshot!(
        "mod_sample_hold_5hz_var0_2",
        hold_hz(5.0, 0.2),
        InputSource::impulse(),
        wav_cfg()
    );

    /* =========================================================
       Amplitude Follower
       =========================================================
       Source: 220 Hz sine with slow (2 Hz) 50% depth amplitude modulation.
       follow(0.10) tracks amplitude with ~100 ms halfway response time.
       Chart includes the input for comparison.
    */
    let modulated_source = sine_hz::<f32>(220.0) * (1.0 + 0.5 * sine_hz::<f32>(2.0));
    let follower = modulated_source.clone() >> follow(0.10);
    // Config with inputs shown (so we see original vs follower output).
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
    assert_audio_unit_snapshot!(
        "mod_follow_attack100ms",
        follower.clone(),
        InputSource::Unit(Box::new(modulated_source.clone())),
        chart_cfg_inputs("mod_follow_attack100ms")
    );
    assert_audio_unit_snapshot!(
        "mod_follow_attack100ms",
        follower,
        InputSource::Unit(Box::new(modulated_source)),
        wav_cfg()
    );
}
