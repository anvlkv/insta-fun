use fundsp::hacker32::*;
use insta_fun::prelude::*;

/// Advanced nonlinear & morphing filter snapshots (32-bit hacker32 API).
///
/// Each unit is captured twice:
/// - SVG chart: 2000 samples (impulse response visualization)
/// - WAV audio: 1 second at DEFAULT_SR (audible decay/resonance tail)
///
/// Categories:
/// 1. Dirty (state-shaped) biquads: dlowpass_hz, dhighpass_hz, dresonator_hz, dbell_hz
/// 2. Feedback-shaped biquads: flowpass_hz, fhighpass_hz, fresonator_hz, fbell_hz
/// 3. Morphing filter progression: morph_hz at discrete morph values
///
/// Input source: impulse (reveals impulse response & decay plainly).
///
/// Run:
///   cargo run --example filters_advanced
///
/// Update snapshots automatically:
///   INSTA_UPDATE=auto cargo run --example filters_advanced
///
/// Naming convention (uniform):
///   adv_<group>_<filter>_<freq>hz_q<QQ.Q0>[_gainG.G0]_shapeS.S0
/// Example: adv_dirty_lowpass_1000hz_q0_70_shape0_50
///
/// Notes:
/// - All literals are f32 for consistency with hacker32.
/// - We keep deterministic impulse snapshots (no warm-up). If a future need
///   arises to stabilize nonlinear internal state, introduce a warm-up phase
///   (builder field) before capturing — omitted here to avoid speculative complexity.
/// - shape(hardness) hardness range: 0.0..1.0.
/// - Morph snapshots use descriptive suffixes (lp, lp_mid, peak, hp_mid, hp).
fn main() {
    const CHART_SAMPLES: usize = 2000;
    const ONE_SECOND_SAMPLES: usize = DEFAULT_SR as usize;

    // Helpers to build configs (SVG chart & WAV16). Centralized to ensure
    // uniform sample count and future adjustability.
    fn cfg_chart(title: &str) -> SnapshotConfig {
        let chart = SvgChartConfigBuilder::default()
            // .chart_layout(Layout::SeparateChannels) // use default layout
            .chart_title(title)
            .build()
            .unwrap();
        SnapshotConfigBuilder::default()
            .num_samples(CHART_SAMPLES)
            .output_mode(chart)
            .build()
            .unwrap()
    }

    fn cfg_wav() -> SnapshotConfig {
        SnapshotConfigBuilder::default()
            .num_samples(ONE_SECOND_SAMPLES)
            .output_mode(WavOutput::Wav16)
            .build()
            .unwrap()
    }

    /* =========================================================
    1. Dirty (state-shaped) biquads
    ========================================================= */

    // Dirty Lowpass (1000 Hz, Q=0.70, hardness=0.50)
    assert_audio_unit_snapshot!(
        "adv_dirty_lowpass_1000hz_q0_70_shape0_50",
        dlowpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32),
        InputSource::impulse(),
        cfg_chart("adv_dirty_lowpass_1000hz_q0_70_shape0_50")
    );
    assert_audio_unit_snapshot!(
        "adv_dirty_lowpass_1000hz_q0_70_shape0_50",
        dlowpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32),
        InputSource::impulse(),
        cfg_wav()
    );

    // Dirty Highpass (1000 Hz, Q=0.70, hardness=0.50)
    assert_audio_unit_snapshot!(
        "adv_dirty_highpass_1000hz_q0_70_shape0_50",
        dhighpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32),
        InputSource::impulse(),
        cfg_chart("adv_dirty_highpass_1000hz_q0_70_shape0_50")
    );
    assert_audio_unit_snapshot!(
        "adv_dirty_highpass_1000hz_q0_70_shape0_50",
        dhighpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32),
        InputSource::impulse(),
        cfg_wav()
    );

    // Dirty Resonator (1000 Hz, Q=5.00, hardness=0.80)
    assert_audio_unit_snapshot!(
        "adv_dirty_resonator_1000hz_q5_00_shape0_80",
        dresonator_hz(SoftCrush(0.80f32), 1000.0f32, 5.00f32),
        InputSource::impulse(),
        cfg_chart("adv_dirty_resonator_1000hz_q5_00_shape0_80")
    );
    assert_audio_unit_snapshot!(
        "adv_dirty_resonator_1000hz_q5_00_shape0_80",
        dresonator_hz(SoftCrush(0.80f32), 1000.0f32, 5.00f32),
        InputSource::impulse(),
        cfg_wav()
    );

    // Dirty Bell (1000 Hz, Q=0.70, gain=1.50, hardness=0.50)
    assert_audio_unit_snapshot!(
        "adv_dirty_bell_1000hz_q0_70_gain1_50_shape0_50",
        dbell_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32, 1.50f32),
        InputSource::impulse(),
        cfg_chart("adv_dirty_bell_1000hz_q0_70_gain1_50_shape0_50")
    );
    assert_audio_unit_snapshot!(
        "adv_dirty_bell_1000hz_q0_70_gain1_50_shape0_50",
        dbell_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32, 1.50f32),
        InputSource::impulse(),
        cfg_wav()
    );

    /* =========================================================
    2. Feedback-shaped biquads
    ========================================================= */

    // Feedback Lowpass (1000 Hz, Q=0.70, hardness=0.50)
    assert_audio_unit_snapshot!(
        "adv_fb_lowpass_1000hz_q0_70_shape0_50",
        flowpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32),
        InputSource::impulse(),
        cfg_chart("adv_fb_lowpass_1000hz_q0_70_shape0_50")
    );
    assert_audio_unit_snapshot!(
        "adv_fb_lowpass_1000hz_q0_70_shape0_50",
        flowpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32),
        InputSource::impulse(),
        cfg_wav()
    );

    // Feedback Highpass (1000 Hz, Q=0.70, hardness=0.50)
    assert_audio_unit_snapshot!(
        "adv_fb_highpass_1000hz_q0_70_shape0_50",
        fhighpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32),
        InputSource::impulse(),
        cfg_chart("adv_fb_highpass_1000hz_q0_70_shape0_50")
    );
    assert_audio_unit_snapshot!(
        "adv_fb_highpass_1000hz_q0_70_shape0_50",
        fhighpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32),
        InputSource::impulse(),
        cfg_wav()
    );

    // Feedback Resonator (1000 Hz, Q=6.00, hardness=0.80)
    assert_audio_unit_snapshot!(
        "adv_fb_resonator_1000hz_q6_00_shape0_80",
        fresonator_hz(SoftCrush(0.80f32), 1000.0f32, 6.00f32),
        InputSource::impulse(),
        cfg_chart("adv_fb_resonator_1000hz_q6_00_shape0_80")
    );
    assert_audio_unit_snapshot!(
        "adv_fb_resonator_1000hz_q6_00_shape0_80",
        fresonator_hz(SoftCrush(0.80f32), 1000.0f32, 6.00f32),
        InputSource::impulse(),
        cfg_wav()
    );

    // Feedback Bell (1000 Hz, Q=0.70, gain=1.50, hardness=0.50)
    assert_audio_unit_snapshot!(
        "adv_fb_bell_1000hz_q0_70_gain1_50_shape0_50",
        fbell_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32, 1.50f32),
        InputSource::impulse(),
        cfg_chart("adv_fb_bell_1000hz_q0_70_gain1_50_shape0_50")
    );
    assert_audio_unit_snapshot!(
        "adv_fb_bell_1000hz_q0_70_gain1_50_shape0_50",
        fbell_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32, 1.50f32),
        InputSource::impulse(),
        cfg_wav()
    );

    /* =========================================================
       3. Morphing filter (discrete morph positions)
       =========================================================
       morph: -1.0 = lowpass, 0.0 = peak, 1.0 = highpass
       Intermediate (-0.5, 0.5) for transition snapshots.
    */

    // morph = -1.0 (lowpass)
    assert_audio_unit_snapshot!(
        "adv_morph_1000hz_q0_80_m_lp",
        morph_hz(1000.0f32, 0.80f32, -1.0f32),
        InputSource::impulse(),
        cfg_chart("adv_morph_1000hz_q0_80_m_lp")
    );
    assert_audio_unit_snapshot!(
        "adv_morph_1000hz_q0_80_m_lp",
        morph_hz(1000.0f32, 0.80f32, -1.0f32),
        InputSource::impulse(),
        cfg_wav()
    );

    // morph = -0.5 (toward peak)
    assert_audio_unit_snapshot!(
        "adv_morph_1000hz_q0_80_m_lp_mid",
        morph_hz(1000.0f32, 0.80f32, -0.5f32),
        InputSource::impulse(),
        cfg_chart("adv_morph_1000hz_q0_80_m_lp_mid")
    );
    assert_audio_unit_snapshot!(
        "adv_morph_1000hz_q0_80_m_lp_mid",
        morph_hz(1000.0f32, 0.80f32, -0.5f32),
        InputSource::impulse(),
        cfg_wav()
    );

    // morph = 0.0 (peak)
    assert_audio_unit_snapshot!(
        "adv_morph_1000hz_q0_80_m_peak",
        morph_hz(1000.0f32, 0.80f32, 0.0f32),
        InputSource::impulse(),
        cfg_chart("adv_morph_1000hz_q0_80_m_peak")
    );
    assert_audio_unit_snapshot!(
        "adv_morph_1000hz_q0_80_m_peak",
        morph_hz(1000.0f32, 0.80f32, 0.0f32),
        InputSource::impulse(),
        cfg_wav()
    );

    // morph = 0.5 (toward highpass)
    assert_audio_unit_snapshot!(
        "adv_morph_1000hz_q0_80_m_hp_mid",
        morph_hz(1000.0f32, 0.80f32, 0.5f32),
        InputSource::impulse(),
        cfg_chart("adv_morph_1000hz_q0_80_m_hp_mid")
    );
    assert_audio_unit_snapshot!(
        "adv_morph_1000hz_q0_80_m_hp_mid",
        morph_hz(1000.0f32, 0.80f32, 0.5f32),
        InputSource::impulse(),
        cfg_wav()
    );

    // morph = 1.0 (highpass)
    assert_audio_unit_snapshot!(
        "adv_morph_1000hz_q0_80_m_hp",
        morph_hz(1000.0f32, 0.80f32, 1.0f32),
        InputSource::impulse(),
        cfg_chart("adv_morph_1000hz_q0_80_m_hp")
    );
    assert_audio_unit_snapshot!(
        "adv_morph_1000hz_q0_80_m_hp",
        morph_hz(1000.0f32, 0.80f32, 1.0f32),
        InputSource::impulse(),
        cfg_wav()
    );
}
