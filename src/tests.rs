use fundsp::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::assert_audio_unit_snapshot;
use crate::config::{SvgChartConfigBuilder, WavOutput};
use crate::prelude::*;

#[test]
fn test_sine() {
    let config = SnapshotConfigBuilder::default().build().unwrap();
    let unit = sine_hz::<f32>(440.0);
    let data = snapshot_audio_unit_with_input_and_options(unit, InputSource::None, config);

    insta::assert_binary_snapshot!("sine.svg", data)
}

#[test]
fn test_custom_input() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(100)
        .build()
        .unwrap();
    let input = (0..100).map(|i| (i as f32 / 50.0).sin()).collect();

    let data = snapshot_audio_unit_with_input_and_options(
        lowpass_hz(500.0, 0.7),
        InputSource::VecByChannel(vec![input]),
        config,
    );

    insta::assert_binary_snapshot!("custom_input.svg", data)
}

#[test]
fn test_stereo() {
    let config = SnapshotConfigBuilder::default().build().unwrap();
    let unit = sine_hz::<f32>(440.0) | sine_hz::<f32>(880.0);

    let data = snapshot_audio_unit_with_input_and_options(unit, InputSource::None, config);

    insta::assert_binary_snapshot!("stereo.svg", data)
}

#[test]
fn test_lowpass_impulse() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(300)
        .build()
        .unwrap();
    let unit = lowpass_hz(1000.0, 1.0);

    let data = snapshot_audio_unit_with_input_and_options(unit, InputSource::impulse(), config);

    insta::assert_binary_snapshot!("lowpass_impulse.svg", data)
}

#[test]
fn test_net() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(420)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0) >> lowpass_hz(500.0, 0.7);
    let mut net = Net::new(0, 1);
    let unit_id = net.push(Box::new(unit));
    net.pipe_input(unit_id);
    net.pipe_output(unit_id);

    let data = snapshot_audio_unit_with_input_and_options(net, InputSource::None, config);

    insta::assert_binary_snapshot!("net.svg", data)
}

#[test]
fn test_batch_prcessing() {
    let config = SnapshotConfig {
        processing_mode: Processing::Batch(64),
        ..Default::default()
    };

    let unit = sine_hz::<f32>(440.0);

    let data = snapshot_audio_unit_with_options(unit, config);

    insta::assert_binary_snapshot!("process_64.svg", data)
}

#[test]
fn test_vec_by_tick() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(100)
        .build()
        .unwrap();
    // Create input data organized by ticks (100 ticks, 1 channel each)
    let input_data: Vec<Vec<f32>> = (0..100).map(|i| vec![(i as f32 / 50.0).cos()]).collect();

    let data = snapshot_audio_unit_with_input_and_options(
        lowpass_hz(800.0, 0.5),
        InputSource::VecByTick(input_data),
        config,
    );

    insta::assert_binary_snapshot!("vec_by_tick.svg", data)
}

#[test]
fn test_flat_input() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(200)
        .build()
        .unwrap();
    // Flat input repeated for every tick
    let flat_input = vec![0.5];

    let data = snapshot_audio_unit_with_input_and_options(
        highpass_hz(200.0, 0.7),
        InputSource::Flat(flat_input),
        config,
    );

    insta::assert_binary_snapshot!("flat_input.svg", data)
}

#[test]
fn test_sine_input_source() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(200)
        .build()
        .unwrap();

    let data = snapshot_audio_unit_with_input_and_options(
        bandpass_hz(1000.0, 500.0),
        InputSource::sine(100.0, 44100.0),
        config,
    );

    insta::assert_binary_snapshot!("sine_input_source.svg", data)
}

#[test]
fn test_multi_channel_vec_by_channel_with_inputs() {
    let chart = SvgChartConfigBuilder::default()
        .with_inputs(true)
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .num_samples(150)
        .output_mode(chart)
        .build()
        .unwrap();

    // Create stereo input data
    let left_channel: Vec<f32> = (0..150)
        .map(|i| (i as f32 / 75.0 * std::f32::consts::PI).sin())
        .collect();
    let right_channel: Vec<f32> = (0..150)
        .map(|i| (i as f32 / 75.0 * std::f32::consts::PI).cos())
        .collect();

    let unit = resonator_hz(440.0, 100.0) | resonator_hz(440.0, 100.0);

    let data = snapshot_audio_unit_with_input_and_options(
        unit,
        InputSource::VecByChannel(vec![left_channel, right_channel]),
        config,
    );

    insta::assert_binary_snapshot!("multi_channel_vec_by_channel_with_inputs.svg", data)
}

#[test]
fn test_macro_variant_unit_only() {
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!(unit);
}

#[test]
fn test_macro_variant_name_and_unit() {
    let unit = saw_hz(220.0);
    assert_audio_unit_snapshot!("macro_with_name", unit);
}

#[test]
fn test_macro_variant_name_unit_input() {
    let unit = lowpass_hz(1000.0, 1.0);
    assert_audio_unit_snapshot!("macro_with_input", unit, InputSource::impulse());
}

#[test]
fn test_macro_variant_name_unit_input_config() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(512)
        .build()
        .unwrap();
    let unit = highpass_hz(2000.0, 0.7);
    assert_audio_unit_snapshot!(
        "macro_with_config",
        unit,
        InputSource::sine(100.0, 44100.0),
        config
    );
}

#[test]
fn test_macro_variant_unit_and_config() {
    let unit = lowpass_hz(1000.0, 1.0);
    let config = SnapshotConfigBuilder::default()
        .num_samples(256)
        .build()
        .unwrap();
    assert_audio_unit_snapshot!(unit, config);
}

#[test]
fn test_chart_with_title() {
    let chart = SvgChartConfigBuilder::default()
        .chart_title("Test Waveform 440Hz")
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_with_title", unit, InputSource::None, config);
}

#[test]
fn test_chart_with_grid() {
    let chart = SvgChartConfigBuilder::default()
        .show_grid(true)
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_with_grid", unit, InputSource::None, config);
}

#[test]
fn test_chart_without_labels() {
    let chart = SvgChartConfigBuilder::default()
        .show_labels(false)
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_without_labels", unit, InputSource::None, config);
}

#[test]
fn test_chart_with_custom_colors() {
    let chart = SvgChartConfigBuilder::default()
        .with_inputs(true)
        .output_colors(vec!["#FF0000".to_string(), "#00FF00".to_string()])
        .input_colors(vec!["#0000FF".to_string(), "#FFFF00".to_string()])
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = lowpass_hz(1000.0, 0.7);
    assert_audio_unit_snapshot!(
        "chart_custom_colors",
        unit,
        InputSource::sine(200.0, 44100.0),
        config
    );
}

#[test]
fn test_chart_with_custom_background() {
    let chart = SvgChartConfigBuilder::default()
        .background_color("#1E1E1E")
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_custom_background", unit, InputSource::None, config);
}

#[test]
fn test_chart_with_custom_line_width() {
    let chart = SvgChartConfigBuilder::default()
        .line_width(4.0)
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_custom_line_width", unit, InputSource::None, config);
}

#[test]
fn test_chart_with_custom_dimensions() {
    let chart = SvgChartConfigBuilder::default()
        .svg_width(800)
        .svg_height_per_channel(150)
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_custom_dimensions", unit, InputSource::None, config);
}

#[test]
fn test_chart_with_all_options() {
    let chart = SvgChartConfigBuilder::default()
        .chart_title("Complete Waveform Test")
        .show_grid(true)
        .show_labels(true)
        .with_inputs(true)
        .output_color("#FF6B6B")
        .input_color("#95E77E")
        .background_color("#2C3E50")
        .line_width(3.0)
        .svg_width(1200)
        .svg_height_per_channel(120)
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!(
        "chart_all_options",
        unit,
        InputSource::sine(100.0, 44100.0),
        config
    );
}

#[test]
fn test_chart_grid_and_no_labels() {
    let chart = SvgChartConfigBuilder::default()
        .show_grid(true)
        .show_labels(false)
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_grid_no_labels", unit, InputSource::None, config);
}

#[test]
fn test_chart_multi_channel_with_custom_colors() {
    let chart = SvgChartConfigBuilder::default()
        .with_inputs(true)
        .output_color("#FF1744")
        .output_color("#00E676")
        .input_color("#2979FF")
        .input_color("#FFEA00")
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = lowpass_hz(1000.0, 0.7) | highpass_hz(200.0, 0.7);
    assert_audio_unit_snapshot!(
        "chart_stereo_custom_colors",
        unit,
        InputSource::Flat(vec![0.5, -0.5]),
        config
    );
}

#[test]
fn chart_with_custom_output_channel_labels() {
    let chart = SvgChartConfigBuilder::default()
        .with_inputs(true)
        .output_color("#FF1744")
        .output_color("#00E676")
        .output_title("lowpass_hz(1000.0, 0.7)")
        .output_title("highpass_hz(200.0, 0.7)")
        .build()
        .unwrap();

    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = lowpass_hz(1000.0, 0.7) | highpass_hz(200.0, 0.7);
    assert_audio_unit_snapshot!(
        "chart_stereo_custom_labels",
        unit,
        InputSource::Flat(vec![0.5, -0.5]),
        config
    );
}

#[test]
fn chart_with_custom_input_channel_labels() {
    let chart = SvgChartConfigBuilder::default()
        .with_inputs(true)
        .input_color("#2979FF")
        .input_color("#FFEA00")
        .input_title("left")
        .input_title("right")
        .build()
        .unwrap();

    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = lowpass_hz(1000.0, 0.7) | highpass_hz(200.0, 0.7);
    assert_audio_unit_snapshot!(
        "chart_stereo_custom_inputs",
        unit,
        InputSource::Flat(vec![0.5, -0.5]),
        config
    );
}

#[test]
fn test_warmup_none() {
    let config = SnapshotConfigBuilder::default()
        .warm_up(WarmUp::None)
        .num_samples(256)
        .build()
        .unwrap();
    let unit = lowpass_hz(500.0, 0.8);
    assert_audio_unit_snapshot!("warmup_none", unit, InputSource::impulse(), config);
}

#[test]
fn test_warmup_samples() {
    let config = SnapshotConfigBuilder::default()
        .warm_up(WarmUp::Samples(20_000))
        .num_samples(2560)
        .build()
        .unwrap();
    let unit = lowpass_hz(500.0, 0.8);
    assert_audio_unit_snapshot!("warmup_samples", unit, InputSource::impulse(), config);
}

#[test]
fn test_warmup_seconds() {
    let config = SnapshotConfigBuilder::default()
        .warm_up(WarmUp::Seconds(0.01))
        .num_samples(600)
        .build()
        .unwrap();
    let unit = lowpass_hz(500.0, 0.8);
    assert_audio_unit_snapshot!("warmup_seconds_0_01", unit, InputSource::impulse(), config);
}

#[test]
fn test_warmup_samples_with_input() {
    let warm_input = Rc::new(RefCell::new(InputSource::impulse()));
    let config = SnapshotConfigBuilder::default()
        .warm_up(WarmUp::SamplesWithInput {
            samples: 128,
            input: warm_input.clone(),
        })
        .num_samples(256)
        .build()
        .unwrap();
    let unit = resonator_hz(440.0, 50.0);
    assert_audio_unit_snapshot!(
        "warmup_samples_with_input_impulse",
        unit,
        InputSource::impulse(),
        config
    );
}

#[test]
fn test_abnormal_allowed() {
    let config = SnapshotConfigBuilder::default()
        .allow_abnormal_samples(true)
        .num_samples(800)
        .build()
        .unwrap();
    let unit = pass();
    assert_audio_unit_snapshot!(
        "abnormal_allowed",
        unit,
        InputSource::Generator(Box::new(|sample, _| {
            if sample.is_multiple_of(3) && sample.is_multiple_of(5) {
                f32::NAN
            } else if sample.is_multiple_of(3) {
                f32::NEG_INFINITY
            } else if sample.is_multiple_of(5) {
                f32::INFINITY
            } else {
                sample as f32
            }
        })),
        config
    );
}

#[test]
#[should_panic]
fn test_abnormal_disallowed_panic() {
    let config = SnapshotConfigBuilder::default()
        .allow_abnormal_samples(false)
        .num_samples(800)
        .build()
        .unwrap();
    let unit = pass();
    assert_audio_unit_snapshot!(
        "abnormal_disallowed_should_panic",
        unit,
        InputSource::Generator(Box::new(|sample, _| {
            if sample.is_multiple_of(3) && sample.is_multiple_of(5) {
                f32::NAN
            } else if sample.is_multiple_of(3) {
                f32::NEG_INFINITY
            } else if sample.is_multiple_of(5) {
                f32::INFINITY
            } else {
                sample as f32
            }
        })),
        config
    );
}

#[test]
fn test_chart_layout_combined() {
    let chart = SvgChartConfigBuilder::default()
        .chart_layout(Layout::Combined)
        .show_grid(true)
        .with_inputs(true)
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = lowpass_hz(1000.0, 0.7) | highpass_hz(200.0, 0.7);
    assert_audio_unit_snapshot!(
        "chart_layout_combined",
        unit,
        InputSource::Flat(vec![0.5, -0.5]),
        config
    );
}

#[test]
fn test_chart_layout_combined_per_type() {
    let chart = SvgChartConfigBuilder::default()
        .chart_layout(Layout::CombinedPerChannelType)
        .show_grid(true)
        .with_inputs(true)
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = lowpass_hz(1000.0, 0.7) | highpass_hz(200.0, 0.7);
    assert_audio_unit_snapshot!(
        "chart_layout_combined_per_type",
        unit,
        InputSource::Flat(vec![0.5, -0.5]),
        config
    );
}

#[test]
fn test_chart_layout_combined_per_type_no_inputs() {
    let chart = SvgChartConfigBuilder::default()
        .with_inputs(false)
        .chart_layout(Layout::CombinedPerChannelType)
        .show_grid(true)
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = lowpass_hz(800.0, 0.7) | highpass_hz(300.0, 0.7);
    assert_audio_unit_snapshot!(
        "chart_layout_combined_per_type_no_inputs",
        unit,
        InputSource::None,
        config
    );
}

#[test]
fn chart_x_axis_labels_as_time() {
    let chart = SvgChartConfigBuilder::default()
        .format_x_axis_labels_as_time(true)
        .show_labels(true)
        .max_labels_x_axis(None)
        .build()
        .unwrap();
    let config = SnapshotConfigBuilder::default()
        .num_samples(30000)
        .output_mode(chart)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);

    assert_audio_unit_snapshot!(
        "chart_x_axis_labels_as_time",
        unit,
        InputSource::None,
        config
    );
}

/* WAV output tests */

#[test]
fn test_wav16_basic() {
    let config = SnapshotConfigBuilder::default()
        .output_mode(WavOutput::Wav16)
        .num_samples(2048)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    let data = snapshot_audio_unit_with_input_and_options(unit, InputSource::None, config);
    insta::assert_binary_snapshot!("wav16.wav", data);
}

#[test]
fn test_wav32_basic() {
    let config = SnapshotConfigBuilder::default()
        .output_mode(WavOutput::Wav32)
        .num_samples(2048)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    let data = snapshot_audio_unit_with_input_and_options(unit, InputSource::None, config);
    insta::assert_binary_snapshot!("wav32.wav", data);
}

#[test]
fn test_wav16_with_warmup() {
    let config = SnapshotConfigBuilder::default()
        .warm_up(WarmUp::Samples(20_000))
        .output_mode(WavOutput::Wav16)
        .num_samples(2048)
        .build()
        .unwrap();
    let unit = lowpass_hz(1000.0, 0.8);
    let data = snapshot_audio_unit_with_input_and_options(unit, InputSource::impulse(), config);
    insta::assert_binary_snapshot!("wav16_warmup.wav", data);
}
