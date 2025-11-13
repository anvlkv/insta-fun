use fundsp::prelude::*;

use crate::assert_audio_unit_snapshot;
use crate::prelude::*;

#[test]
fn test_sine() {
    let config = SnapshotConfigBuilder::default().build().unwrap();
    let unit = sine_hz::<f32>(440.0);
    let svg = snapshot_audio_unit_with_input_and_options(unit, InputSource::None, config);

    insta::assert_binary_snapshot!("sine.svg", svg.into_bytes())
}

#[test]
fn test_custom_input() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(100)
        .build()
        .unwrap();
    let input = (0..100).map(|i| (i as f32 / 50.0).sin()).collect();

    let svg = snapshot_audio_unit_with_input_and_options(
        lowpass_hz(500.0, 0.7),
        InputSource::VecByChannel(vec![input]),
        config,
    );

    insta::assert_binary_snapshot!("custom_input.svg", svg.into_bytes())
}

#[test]
fn test_stereo() {
    let config = SnapshotConfigBuilder::default().build().unwrap();
    let unit = sine_hz::<f32>(440.0) | sine_hz::<f32>(880.0);

    let svg = snapshot_audio_unit_with_input_and_options(unit, InputSource::None, config);

    insta::assert_binary_snapshot!("stereo.svg", svg.into_bytes())
}

#[test]
fn test_lowpass_impulse() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(300)
        .build()
        .unwrap();
    let unit = lowpass_hz(1000.0, 1.0);

    let svg = snapshot_audio_unit_with_input_and_options(unit, InputSource::impulse(), config);

    insta::assert_binary_snapshot!("lowpass_impulse.svg", svg.into_bytes())
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

    let svg = snapshot_audio_unit_with_input_and_options(net, InputSource::None, config);

    insta::assert_binary_snapshot!("net.svg", svg.into_bytes())
}

#[test]
fn test_batch_prcessing() {
    let config = SnapshotConfig {
        processing_mode: Processing::Batch(64),
        ..Default::default()
    };

    let unit = sine_hz::<f32>(440.0);

    let svg = snapshot_audio_unit_with_options(unit, config);

    insta::assert_binary_snapshot!("process_64.svg", svg.into_bytes())
}

#[test]
fn test_vec_by_tick() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(100)
        .build()
        .unwrap();
    // Create input data organized by ticks (100 ticks, 1 channel each)
    let input_data: Vec<Vec<f32>> = (0..100).map(|i| vec![(i as f32 / 50.0).cos()]).collect();

    let svg = snapshot_audio_unit_with_input_and_options(
        lowpass_hz(800.0, 0.5),
        InputSource::VecByTick(input_data),
        config,
    );

    insta::assert_binary_snapshot!("vec_by_tick.svg", svg.into_bytes())
}

#[test]
fn test_flat_input() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(200)
        .build()
        .unwrap();
    // Flat input repeated for every tick
    let flat_input = vec![0.5];

    let svg = snapshot_audio_unit_with_input_and_options(
        highpass_hz(200.0, 0.7),
        InputSource::Flat(flat_input),
        config,
    );

    insta::assert_binary_snapshot!("flat_input.svg", svg.into_bytes())
}

#[test]
fn test_sine_input_source() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(200)
        .build()
        .unwrap();

    let svg = snapshot_audio_unit_with_input_and_options(
        bandpass_hz(1000.0, 500.0),
        InputSource::sine(100.0, 44100.0),
        config,
    );

    insta::assert_binary_snapshot!("sine_input_source.svg", svg.into_bytes())
}

#[test]
fn test_multi_channel_vec_by_channel_with_inputs() {
    let config = SnapshotConfigBuilder::default()
        .num_samples(150)
        .with_inputs(true)
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

    let svg = snapshot_audio_unit_with_input_and_options(
        unit,
        InputSource::VecByChannel(vec![left_channel, right_channel]),
        config,
    );

    insta::assert_binary_snapshot!(
        "multi_channel_vec_by_channel_with_inputs.svg",
        svg.into_bytes()
    )
}

#[test]
fn test_macro_variant_unit_only() {
    // Variant 1: Just the unit
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!(unit);
}

#[test]
fn test_macro_variant_name_and_unit() {
    // Variant 2: With name and unit
    let unit = saw_hz(220.0);
    assert_audio_unit_snapshot!("macro_with_name", unit);
}

#[test]
fn test_macro_variant_name_unit_input() {
    // Variant 3: With name, unit, and input source
    let unit = lowpass_hz(1000.0, 1.0);
    assert_audio_unit_snapshot!("macro_with_input", unit, InputSource::impulse());
}

#[test]
fn test_macro_variant_name_unit_input_config() {
    // Variant 4: With name, unit, input source, and config
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
    // Variant 5: With unit and config
    let unit = lowpass_hz(1000.0, 1.0);
    let config = SnapshotConfigBuilder::default()
        .num_samples(256)
        .build()
        .unwrap();
    assert_audio_unit_snapshot!(unit, config);
}

#[test]
fn test_chart_with_title() {
    let config = SnapshotConfigBuilder::default()
        .chart_title("Test Waveform 440Hz")
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_with_title", unit, InputSource::None, config);
}

#[test]
fn test_chart_with_grid() {
    let config = SnapshotConfigBuilder::default()
        .show_grid(true)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_with_grid", unit, InputSource::None, config);
}

#[test]
fn test_chart_without_labels() {
    let config = SnapshotConfigBuilder::default()
        .show_labels(false)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_without_labels", unit, InputSource::None, config);
}

#[test]
fn test_chart_with_custom_colors() {
    let config = SnapshotConfigBuilder::default()
        .with_inputs(true)
        .output_colors(vec!["#FF0000".to_string(), "#00FF00".to_string()])
        .input_colors(vec!["#0000FF".to_string(), "#FFFF00".to_string()])
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
    let config = SnapshotConfigBuilder::default()
        .background_color("#1E1E1E")
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_custom_background", unit, InputSource::None, config);
}

#[test]
fn test_chart_with_custom_line_width() {
    let config = SnapshotConfigBuilder::default()
        .line_width(4.0)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_custom_line_width", unit, InputSource::None, config);
}

#[test]
fn test_chart_with_custom_dimensions() {
    let config = SnapshotConfigBuilder::default()
        .svg_width(800)
        .svg_height_per_channel(150)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_custom_dimensions", unit, InputSource::None, config);
}

#[test]
fn test_chart_with_all_options() {
    let config = SnapshotConfigBuilder::default()
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
    let config = SnapshotConfigBuilder::default()
        .show_grid(true)
        .show_labels(false)
        .build()
        .unwrap();
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("chart_grid_no_labels", unit, InputSource::None, config);
}

#[test]
fn test_chart_multi_channel_with_custom_colors() {
    let config = SnapshotConfigBuilder::default()
        .with_inputs(true)
        .output_color("#FF1744")
        .output_color("#00E676")
        .input_color("#2979FF")
        .input_color("#FFEA00")
        .build()
        .unwrap();
    // Create stereo filter unit
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
    let config = SnapshotConfigBuilder::default()
        .with_inputs(true)
        .output_color("#FF1744")
        .output_color("#00E676")
        .output_title("lowpass_hz(1000.0, 0.7)")
        .output_title("highpass_hz(200.0, 0.7)")
        .build()
        .unwrap();

    // Create stereo filter unit
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
    let config = SnapshotConfigBuilder::default()
        .with_inputs(true)
        .input_color("#2979FF")
        .input_color("#FFEA00")
        .input_title("left")
        .input_title("right")
        .build()
        .unwrap();

    // Create stereo filter unit
    let unit = lowpass_hz(1000.0, 0.7) | highpass_hz(200.0, 0.7);
    assert_audio_unit_snapshot!(
        "chart_stereo_custom_inputs",
        unit,
        InputSource::Flat(vec![0.5, -0.5]),
        config
    );
}
