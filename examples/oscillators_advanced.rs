use fundsp::hacker32::*;
use insta_fun::prelude::*;

fn main() {
    const CHART_SAMPLES: usize = 2000;
    const ONE_SECOND_SAMPLES: usize = DEFAULT_SR as usize;

    // Config helpers (centralized for consistency).
    fn cfg_chart(title: &str) -> SnapshotConfig {
        let chart = SvgChartConfigBuilder::default()
            .chart_title(title)
            .build()
            .unwrap();
        SnapshotConfigBuilder::default()
            .num_samples(CHART_SAMPLES)
            .allow_abnormal_samples(true)
            .output_mode(chart)
            .build()
            .unwrap()
    }

    fn cfg_wav() -> SnapshotConfig {
        SnapshotConfigBuilder::default()
            .num_samples(ONE_SECOND_SAMPLES)
            .allow_abnormal_samples(true)
            .output_mode(WavOutput::Wav16)
            .build()
            .unwrap()
    }

    /* =========================================================
    DSF Saw Roughness Sweep (roughness: 0.00, 0.50, 1.00 + safe 0.99)
    ========================================================= */
    assert_audio_unit_snapshot!(
        "adv_dsf_saw_440hz_rough0_00",
        constant(440.0f32) >> dsf_saw_r(0.00f32),
        InputSource::None,
        cfg_chart("adv_dsf_saw_440hz_rough0_00")
    );
    assert_audio_unit_snapshot!(
        "adv_dsf_saw_440hz_rough0_00",
        constant(440.0f32) >> dsf_saw_r(0.00f32),
        InputSource::None,
        cfg_wav()
    );

    assert_audio_unit_snapshot!(
        "adv_dsf_saw_440hz_rough0_50",
        constant(440.0f32) >> dsf_saw_r(0.50f32),
        InputSource::None,
        cfg_chart("adv_dsf_saw_440hz_rough0_50")
    );
    assert_audio_unit_snapshot!(
        "adv_dsf_saw_440hz_rough0_50",
        constant(440.0f32) >> dsf_saw_r(0.50f32),
        InputSource::None,
        cfg_wav()
    );

    assert_audio_unit_snapshot!(
        "adv_dsf_saw_440hz_rough1_00",
        constant(440.0f32) >> dsf_saw_r(1.00f32),
        InputSource::None,
        cfg_chart("adv_dsf_saw_440hz_rough1_00")
    );
    assert_audio_unit_snapshot!(
        "adv_dsf_saw_440hz_rough1_00",
        constant(440.0f32) >> dsf_saw_r(1.00f32),
        InputSource::None,
        cfg_wav()
    );

    // Safe finite waveform variant (roughness trimmed just below singularity).
    assert_audio_unit_snapshot!(
        "adv_dsf_saw_440hz_rough0_99",
        constant(440.0f32) >> dsf_saw_r(0.99f32),
        InputSource::None,
        cfg_chart("adv_dsf_saw_440hz_rough0_99")
    );
    assert_audio_unit_snapshot!(
        "adv_dsf_saw_440hz_rough0_99",
        constant(440.0f32) >> dsf_saw_r(0.99f32),
        InputSource::None,
        cfg_wav()
    );

    /* =========================================================
    DSF Square Roughness Sweep (roughness: 0.00, 0.50, 1.00 + safe 0.99)
    ========================================================= */
    assert_audio_unit_snapshot!(
        "adv_dsf_square_440hz_rough0_00",
        constant(440.0f32) >> dsf_square_r(0.00f32),
        InputSource::None,
        cfg_chart("adv_dsf_square_440hz_rough0_00")
    );
    assert_audio_unit_snapshot!(
        "adv_dsf_square_440hz_rough0_00",
        constant(440.0f32) >> dsf_square_r(0.00f32),
        InputSource::None,
        cfg_wav()
    );

    assert_audio_unit_snapshot!(
        "adv_dsf_square_440hz_rough0_50",
        constant(440.0f32) >> dsf_square_r(0.50f32),
        InputSource::None,
        cfg_chart("adv_dsf_square_440hz_rough0_50")
    );
    assert_audio_unit_snapshot!(
        "adv_dsf_square_440hz_rough0_50",
        constant(440.0f32) >> dsf_square_r(0.50f32),
        InputSource::None,
        cfg_wav()
    );

    assert_audio_unit_snapshot!(
        "adv_dsf_square_440hz_rough1_00",
        constant(440.0f32) >> dsf_square_r(1.00f32),
        InputSource::None,
        cfg_chart("adv_dsf_square_440hz_rough1_00")
    );
    assert_audio_unit_snapshot!(
        "adv_dsf_square_440hz_rough1_00",
        constant(440.0f32) >> dsf_square_r(1.00f32),
        InputSource::None,
        cfg_wav()
    );

    // Safe finite waveform variant (roughness trimmed just below singularity).
    assert_audio_unit_snapshot!(
        "adv_dsf_square_440hz_rough0_99",
        constant(440.0f32) >> dsf_square_r(0.99f32),
        InputSource::None,
        cfg_chart("adv_dsf_square_440hz_rough0_99")
    );
    assert_audio_unit_snapshot!(
        "adv_dsf_square_440hz_rough0_99",
        constant(440.0f32) >> dsf_square_r(0.99f32),
        InputSource::None,
        cfg_wav()
    );

    /* =========================================================
    Pulse Wave Width Sweep (width: 0.10, 0.25, 0.50, 0.75)
    ========================================================= */
    assert_audio_unit_snapshot!(
        "adv_pulse_440hz_width0_10",
        constant((440.0f32, 0.10f32)) >> pulse(),
        InputSource::None,
        cfg_chart("adv_pulse_440hz_width0_10")
    );
    assert_audio_unit_snapshot!(
        "adv_pulse_440hz_width0_10",
        constant((440.0f32, 0.10f32)) >> pulse(),
        InputSource::None,
        cfg_wav()
    );

    assert_audio_unit_snapshot!(
        "adv_pulse_440hz_width0_25",
        constant((440.0f32, 0.25f32)) >> pulse(),
        InputSource::None,
        cfg_chart("adv_pulse_440hz_width0_25")
    );
    assert_audio_unit_snapshot!(
        "adv_pulse_440hz_width0_25",
        constant((440.0f32, 0.25f32)) >> pulse(),
        InputSource::None,
        cfg_wav()
    );

    assert_audio_unit_snapshot!(
        "adv_pulse_440hz_width0_50",
        constant((440.0f32, 0.50f32)) >> pulse(),
        InputSource::None,
        cfg_chart("adv_pulse_440hz_width0_50")
    );
    assert_audio_unit_snapshot!(
        "adv_pulse_440hz_width0_50",
        constant((440.0f32, 0.50f32)) >> pulse(),
        InputSource::None,
        cfg_wav()
    );

    assert_audio_unit_snapshot!(
        "adv_pulse_440hz_width0_75",
        constant((440.0f32, 0.75f32)) >> pulse(),
        InputSource::None,
        cfg_chart("adv_pulse_440hz_width0_75")
    );
    assert_audio_unit_snapshot!(
        "adv_pulse_440hz_width0_75",
        constant((440.0f32, 0.75f32)) >> pulse(),
        InputSource::None,
        cfg_wav()
    );

    /* =========================================================
       Synthesized Threshold Pulses (duty: 0.10, 0.25, 0.75)
       =========================================================
       Approximate pulse: sine > threshold ? 1 : -1
       threshold = 1.0 - 2.0 * duty
    */
    fn synth_threshold_pulse(freq: f32, duty: f32) -> An<impl AudioNode> {
        let threshold = 1.0f32 - 2.0f32 * duty;
        sine_hz(freq)
            >> map(move |frame: &Frame<f32, U1>| {
                if frame[0] > threshold {
                    1.0f32
                } else {
                    -1.0f32
                }
            })
    }

    assert_audio_unit_snapshot!(
        "adv_synth_pulse_440hz_duty0_10",
        synth_threshold_pulse(440.0f32, 0.10f32),
        InputSource::None,
        cfg_chart("adv_synth_pulse_440hz_duty0_10")
    );
    assert_audio_unit_snapshot!(
        "adv_synth_pulse_440hz_duty0_10",
        synth_threshold_pulse(440.0f32, 0.10f32),
        InputSource::None,
        cfg_wav()
    );

    assert_audio_unit_snapshot!(
        "adv_synth_pulse_440hz_duty0_25",
        synth_threshold_pulse(440.0f32, 0.25f32),
        InputSource::None,
        cfg_chart("adv_synth_pulse_440hz_duty0_25")
    );
    assert_audio_unit_snapshot!(
        "adv_synth_pulse_440hz_duty0_25",
        synth_threshold_pulse(440.0f32, 0.25f32),
        InputSource::None,
        cfg_wav()
    );

    assert_audio_unit_snapshot!(
        "adv_synth_pulse_440hz_duty0_75",
        synth_threshold_pulse(440.0f32, 0.75f32),
        InputSource::None,
        cfg_chart("adv_synth_pulse_440hz_duty0_75")
    );
    assert_audio_unit_snapshot!(
        "adv_synth_pulse_440hz_duty0_75",
        synth_threshold_pulse(440.0f32, 0.75f32),
        InputSource::None,
        cfg_wav()
    );

    /* =========================================================
       Simple PWM (dynamic duty: ~0.10..0.90)
       =========================================================
       Carrier: 440 Hz sine
       LFO: 1 Hz sine
       duty(t) = 0.50 + 0.40 * lfo(t)  (lfo ∈ [-1,1] => duty ∈ [0.10,0.90])
       Output thresholded carrier.
    */
    let pwm = {
        let carrier = sine_hz(440.0f32);
        let lfo = sine_hz(1.0f32);
        (carrier | lfo)
            >> map(|f: &Frame<f32, U2>| {
                let carrier_val = f[0];
                let lfo_val = f[1];
                let duty = 0.50f32 + 0.40f32 * lfo_val;
                let threshold = 1.0f32 - 2.0f32 * duty;
                if carrier_val > threshold {
                    1.0f32
                } else {
                    -1.0f32
                }
            })
    };
    assert_audio_unit_snapshot!(
        "adv_pwm_sine_440hz_lfo1hz",
        pwm.clone(),
        InputSource::None,
        cfg_chart("adv_pwm_sine_440hz_lfo1hz")
    );
    assert_audio_unit_snapshot!(
        "adv_pwm_sine_440hz_lfo1hz",
        pwm,
        InputSource::None,
        cfg_wav()
    );
}
