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
fn test_macros() {
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!("macros", unit);
}
