use fundsp::prelude32::*;
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
    Physical & Chaotic Oscillators
    ========================================================= */
    // Pluck string 110 Hz, damping 0.50
    assert_audio_unit_snapshot!(
        "osc_pluck_110hz_damp0_50",
        pluck(110.0, 0.995, 0.50),
        InputSource::None,
        cfg_chart("osc_pluck_110hz_damp0_50")
    );
    assert_audio_unit_snapshot!(
        "osc_pluck_110hz_damp0_50",
        pluck(110.0, 0.995, 0.50),
        InputSource::None,
        cfg_wav()
    );

    // Lorenz attractor oscillator
    assert_audio_unit_snapshot!(
        "osc_lorenz",
        lorenz(),
        InputSource::None,
        cfg_chart("osc_lorenz")
    );
    assert_audio_unit_snapshot!("osc_lorenz", lorenz(), InputSource::None, cfg_wav());

    // Rossler attractor oscillator
    assert_audio_unit_snapshot!(
        "osc_rossler",
        rossler(),
        InputSource::None,
        cfg_chart("osc_rossler")
    );
    assert_audio_unit_snapshot!("osc_rossler", rossler(), InputSource::None, cfg_wav());
}
