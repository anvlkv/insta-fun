#![doc = include_str!("../README.md")]

use std::fmt::Write;

use fundsp::prelude::*;
use insta::assert_binary_snapshot;

const DEFAULT_HEIGHT: usize = 100;

#[derive(Debug, Clone, Copy)]
/// Configuration for snapshotting an audio node.
pub struct SnapshotConfig {
    /// Number of samples to generate.
    ///
    /// `Default` is 44100 - 1s
    pub num_samples: usize,
    /// Sample rate of the audio node.
    ///
    /// `Default` is 44100.0
    pub sample_rate: f64,
    /// Optional width of the SVG `viewBox`
    ///
    /// `None` means proportional to num_samples
    pub svg_width: Option<usize>,
    /// Height of **one** channel in the SVG `viewBox`
    ///
    /// `None` fallbacks to default - 100
    pub svg_height_per_channel: Option<usize>,
    /// Processing mode for snapshotting an audio node.
    pub processing_mode: Processing,
}

/// Processing mode for snapshotting an audio node.
#[derive(Debug, Clone, Copy, Default)]
pub enum Processing {
    #[default]
    /// Process one sample at a time.
    Tick,
    /// Process a batch of samples at a time.
    ///
    /// max batch size is 64
    Batch(u8),
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            num_samples: 44100,
            sample_rate: 44100.0,
            svg_width: None,
            svg_height_per_channel: Some(DEFAULT_HEIGHT),
            processing_mode: Processing::default(),
        }
    }
}

impl SnapshotConfig {
    pub fn with_samples(num_samples: usize) -> Self {
        Self {
            num_samples,
            ..Default::default()
        }
    }
}

/// Input provided to the audio node
pub enum InputSource {
    /// No input
    None,
    /// Input provided by a channel vec
    ///
    /// - First vec contains all **channels**
    /// - Second vec contains **samples** per channel
    VecByChannel(Vec<Vec<f32>>),
    /// Input provided by a tick vec
    ///
    /// - First vec contains all **ticks**
    /// - Second vec contains **samples** for all **channels** per tick
    VecByTick(Vec<Vec<f32>>),
    /// Input **repeated** on every tick
    ///
    /// - Vector contains **samples** for all **channels** for **one** tick
    Flat(Vec<f32>),
    /// Input provided by a generator function
    ///
    /// - First argument is the sample index
    /// - Second argument is the channel index
    Generator(Box<dyn Fn(usize, usize) -> f32>),
}

impl InputSource {
    pub fn impulse() -> Self {
        Self::Generator(Box::new(|i, _| if i == 0 { 1.0 } else { 0.0 }))
    }
    pub fn sine(freq: f32, sr: f32) -> Self {
        Self::Generator(Box::new(move |i, _| {
            let phase = 2.0 * std::f32::consts::PI * freq * i as f32 / sr;
            phase.sin()
        }))
    }
}

const OUTPUT_CHANNEL_COLORS: &[&str] = &[
    "#4285F4", "#EA4335", "#FBBC04", "#34A853", "#FF6D00", "#AB47BC", "#00ACC1", "#7CB342",
    "#9C27B0", "#3F51B5", "#009688", "#8BC34A", "#FFEB3B", "#FF9800", "#795548", "#607D8B",
    "#E91E63", "#673AB7", "#2196F3", "#00BCD4", "#4CAF50", "#CDDC39", "#FFC107", "#FF5722",
    "#9E9E9E", "#03A9F4", "#8D6E63", "#78909C", "#880E4F", "#4A148C", "#0D47A1", "#004D40",
];

const INPUT_CHANNEL_COLORS: &[&str] = &[
    "#B39DDB", "#FFAB91", "#FFF59D", "#A5D6A7", "#FFCC80", "#CE93D8", "#80DEEA", "#C5E1A5",
    "#BA68C8", "#9FA8DA", "#80CBC4", "#DCE775", "#FFF176", "#FFB74D", "#BCAAA4", "#B0BEC5",
    "#F48FB1", "#B39DDB", "#90CAF9", "#80DEEA", "#A5D6A7", "#E6EE9C", "#FFD54F", "#FF8A65",
    "#BDBDBD", "#81D4FA", "#A1887F", "#90A4AE", "#C2185B", "#7B1FA2", "#1976D2", "#00796B",
];

const PADDING: isize = 10;

/// Create an SVG snapshot of audio node outputs
/// ## Example
///
/// ```
/// use insta_fun::*;
/// use fundsp::hacker::prelude::*;
///
/// let node = sine_hz::<f32>(440.0);
/// snapshot_audio_node("sine_hz_4", node);
/// ```
pub fn snapshot_audio_node<N>(name: &str, node: N)
where
    N: AudioUnit,
{
    snapshot_audionode_with_input_and_options(
        name,
        node,
        InputSource::None,
        SnapshotConfig::default(),
    )
}

/// Create an SVG snapshot of audio node outputs, with options
///
/// ## Example
///
/// ```
/// use insta_fun::*;
/// use fundsp::hacker::prelude::*;
///
/// let node = sine_hz::<f32>(440.0);
/// snapshot_audio_node_with_options("sine_hz_3", node, SnapshotConfig::default());
/// ```
pub fn snapshot_audio_node_with_options<N>(name: &str, node: N, options: SnapshotConfig)
where
    N: AudioUnit,
{
    snapshot_audionode_with_input_and_options(name, node, InputSource::None, options)
}

/// Create an SVG snapshot of audio node inputs and outputs
///
/// ## Example
///
/// ```
/// use insta_fun::*;
/// use fundsp::hacker::prelude::*;
///
/// let node = sine_hz::<f32>(440.0);
/// snapshot_audio_node_with_input("sine_hz_2", node, InputSource::None);
/// ```
pub fn snapshot_audio_node_with_input<N>(name: &str, node: N, input_source: InputSource)
where
    N: AudioUnit,
{
    snapshot_audionode_with_input_and_options(name, node, input_source, SnapshotConfig::default())
}

/// Create an SVG snapshot of audio node inputs and outputs, with options
///
/// ## Example
///
/// ```
/// use insta_fun::*;
/// use fundsp::hacker::prelude::*;
///
/// let config = SnapshotConfig::default();
/// let node = sine_hz::<f32>(440.0);
/// snapshot_audionode_with_input_and_options("sine_hz_1", node, InputSource::None, config);
/// ```
pub fn snapshot_audionode_with_input_and_options<N>(
    name: &str,
    mut node: N,
    input_source: InputSource,
    config: SnapshotConfig,
) where
    N: AudioUnit,
{
    let num_inputs = N::inputs(&node);
    let num_outputs = N::outputs(&node);

    node.set_sample_rate(config.sample_rate);
    node.reset();
    node.allocate();

    let input_data = match input_source {
        InputSource::None => vec![vec![0.0; config.num_samples]; num_inputs],
        InputSource::VecByChannel(data) => {
            assert_eq!(
                data.len(),
                num_inputs,
                "Input vec size mismatch. Expected {} channels, got {}",
                num_inputs,
                data.len()
            );
            assert!(
                data.iter().all(|v| v.len() == config.num_samples),
                "Input vec size mismatch. Expected {} samples per channel, got {}",
                config.num_samples,
                data.iter().map(|v| v.len()).max().unwrap_or(0)
            );
            data
        }
        InputSource::VecByTick(data) => {
            assert!(
                data.iter().all(|v| v.len() == num_inputs),
                "Input vec size mismatch. Expected {} channels, got {}",
                num_inputs,
                data.iter().map(|v| v.len()).max().unwrap_or(0)
            );
            assert_eq!(
                data.len(),
                config.num_samples,
                "Input vec size mismatch. Expected {} samples, got {}",
                config.num_samples,
                data.len()
            );
            (0..num_inputs)
                .map(|ch| (0..config.num_samples).map(|i| data[i][ch]).collect())
                .collect()
        }
        InputSource::Flat(data) => {
            assert_eq!(
                data.len(),
                num_inputs,
                "Input vec size mismatch. Expected {} channels, got {}",
                num_inputs,
                data.len()
            );
            (0..num_inputs)
                .map(|ch| (0..config.num_samples).map(|_| data[ch]).collect())
                .collect()
        }
        InputSource::Generator(generator_fn) => (0..num_inputs)
            .map(|ch| {
                (0..config.num_samples)
                    .map(|i| generator_fn(i, ch))
                    .collect()
            })
            .collect(),
    };

    let mut output_data: Vec<Vec<f32>> = vec![vec![]; num_outputs];

    match config.processing_mode {
        Processing::Tick => {
            (0..config.num_samples).for_each(|i| {
                let mut input_frame = vec![0.0; num_inputs];
                for ch in 0..num_inputs {
                    input_frame[ch] = input_data[ch][i] as f32;
                }
                let mut output_frame = vec![0.0; num_outputs];
                node.tick(&input_frame, &mut output_frame);
                for ch in 0..num_outputs {
                    output_data[ch].push(output_frame[ch]);
                }
            });
        }
        Processing::Batch(batch_size) => {
            assert!(
                batch_size <= 64,
                "Batch size must be less than or equal to 64"
            );

            let samples_index = (0..config.num_samples).collect::<Vec<_>>();
            for chunk in samples_index.chunks(batch_size as usize) {
                let mut input_buff = BufferVec::new(num_inputs);
                for i in chunk {
                    for (ch, input_data) in input_data.iter().enumerate() {
                        let value: f32 = input_data[*i];
                        input_buff.set_f32(ch, *i, value);
                    }
                }
                let input_ref = input_buff.buffer_ref();
                let mut output_buf = BufferVec::new(num_outputs);
                let mut output_ref = output_buf.buffer_mut();

                node.process(chunk.len(), &input_ref, &mut output_ref);

                for (ch, data) in output_data.iter_mut().enumerate() {
                    data.extend_from_slice(output_buf.channel_f32(ch));
                }
            }
        }
    }

    let svg = generate_svg(&input_data, &output_data, &config);

    assert_binary_snapshot!(&format!("{name}.svg"), svg.as_bytes().to_vec());
}

fn generate_svg(
    input_data: &[Vec<f32>],
    output_data: &[Vec<f32>],
    config: &SnapshotConfig,
) -> String {
    let height_per_channel = config.svg_height_per_channel.unwrap_or(DEFAULT_HEIGHT);
    let num_channels = output_data.len() + input_data.len();
    let num_samples = output_data.first().map(|c| c.len()).unwrap_or(0);
    if num_samples == 0 || num_channels == 0 {
        return "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\" preserveAspectRatio=\"none\"><text>Empty</text></svg>".to_string();
    }

    let svg_width = config.svg_width.unwrap_or(config.num_samples);
    let total_height = height_per_channel * num_channels;
    let y_scale = (height_per_channel as f32 / 2.0) * 0.9;
    let x_scale = config
        .svg_width
        .map(|width| width as f32 / config.num_samples as f32);
    let stroke_width = if let Some(scale) = x_scale {
        (2.0 / scale).clamp(0.5, 5.0)
    } else {
        2.0
    };

    let mut svg = String::new();
    let mut y_offset = 0;

    writeln!(
        &mut svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{start_x} {start_y} {width} {height}" preserveAspectRatio="none">
        <rect x="{start_x}" y="{start_y}" width="{background_width}" height="{background_height}" fill="black" />"#,
        start_x = -PADDING,
        start_y = -PADDING,
        width = svg_width as isize + PADDING,
        height = total_height as isize + PADDING,
        background_width = svg_width as isize + PADDING * 2,
        background_height = total_height as isize + PADDING * 2
    ).unwrap();

    let mut write_data = |all_channels_data: &[Vec<f32>], is_input: bool| {
        for (ch, data) in all_channels_data.iter().enumerate() {
            let color = if is_input {
                INPUT_CHANNEL_COLORS[ch % INPUT_CHANNEL_COLORS.len()]
            } else {
                OUTPUT_CHANNEL_COLORS[ch % OUTPUT_CHANNEL_COLORS.len()]
            };
            let y_center = y_offset + height_per_channel / 2;

            let min_val = data.iter().cloned().fold(f32::INFINITY, f32::min);
            let max_val = data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let range = (max_val - min_val).max(f32::EPSILON);

            let mut path_data = String::from("M ");
            for (i, &sample) in data.iter().enumerate() {
                let x = if let Some(scale) = x_scale {
                    scale * i as f32
                } else {
                    i as f32
                };
                let normalized = (sample.clamp(min_val, max_val) - min_val) / range * 2.0 - 1.0;
                let y = y_center as f32 - normalized * y_scale;
                if i == 0 {
                    write!(&mut path_data, "{:.6},{:.6} ", x, y).unwrap();
                } else {
                    write!(&mut path_data, "L {:.6},{:.6} ", x, y).unwrap();
                }
            }

            writeln!(
                &mut svg,
                r#"  <path d="{path_data}" fill="none" stroke="{color}" stroke-width="{stroke_width}"/>"#,
            )
            .unwrap();

            writeln!(
                &mut svg,
                r#"  <text x="5" y="{y}" font-family="monospace" font-size="12" fill="{color}">{label} Ch#{ch}</text>"#,
                y = y_offset + 15,
                color = color,
                label = if is_input {"Input"} else {"Output"},
                ch=ch
            )
            .unwrap();

            y_offset += height_per_channel
        }
    };

    write_data(input_data, true);
    write_data(output_data, false);

    svg.push_str("</svg>");
    svg
}

// Example tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine() {
        let config = SnapshotConfig::default();
        let node = sine_hz::<f32>(440.0);
        snapshot_audionode_with_input_and_options("sine_hz", node, InputSource::None, config);
    }

    #[test]
    fn test_custom_input() {
        let config = SnapshotConfig::with_samples(100);
        let input = (0..100).map(|i| (i as f32 / 50.0).sin()).collect();

        snapshot_audionode_with_input_and_options(
            "filter_sine",
            lowpass_hz(500.0, 0.7),
            InputSource::VecByChannel(vec![input]),
            config,
        );
    }

    #[test]
    fn test_stereo() {
        let config = SnapshotConfig::default();
        let node = sine_hz::<f32>(440.0) | sine_hz::<f32>(880.0);

        snapshot_audionode_with_input_and_options("stereo", node, InputSource::None, config);
    }

    #[test]
    fn test_lowpass_impulse() {
        let config = SnapshotConfig::with_samples(300);
        let node = lowpass_hz(1000.0, 1.0);

        snapshot_audionode_with_input_and_options(
            "lowpass_impulse",
            node,
            InputSource::impulse(),
            config,
        );
    }

    #[test]
    fn test_net() {
        let config = SnapshotConfig::with_samples(420);
        let node = sine_hz::<f32>(440.0) >> lowpass_hz(500.0, 0.7);
        let mut net = Net::new(0, 1);
        let node_id = net.push(Box::new(node));
        net.pipe_input(node_id);
        net.pipe_output(node_id);

        snapshot_audionode_with_input_and_options("net", net, InputSource::None, config);
    }

    #[test]
    fn test_batch_prcessing() {
        let config = SnapshotConfig {
            processing_mode: Processing::Batch(64),
            ..Default::default()
        };

        let node = sine_hz::<f32>(440.0);

        snapshot_audio_node_with_options("process_64", node, config);
    }

    #[test]
    fn test_vec_by_tick() {
        let config = SnapshotConfig::with_samples(100);
        // Create input data organized by ticks (100 ticks, 1 channel each)
        let input_data: Vec<Vec<f32>> = (0..100).map(|i| vec![(i as f32 / 50.0).cos()]).collect();

        snapshot_audionode_with_input_and_options(
            "vec_by_tick",
            lowpass_hz(800.0, 0.5),
            InputSource::VecByTick(input_data),
            config,
        );
    }

    #[test]
    fn test_flat_input() {
        let config = SnapshotConfig::with_samples(200);
        // Flat input repeated for every tick
        let flat_input = vec![0.5];

        snapshot_audionode_with_input_and_options(
            "flat_input",
            highpass_hz(200.0, 0.7),
            InputSource::Flat(flat_input),
            config,
        );
    }

    #[test]
    fn test_sine_input_source() {
        let config = SnapshotConfig::with_samples(200);

        snapshot_audionode_with_input_and_options(
            "sine_input_source",
            bandpass_hz(1000.0, 500.0),
            InputSource::sine(100.0, 44100.0),
            config,
        );
    }

    #[test]
    fn test_multi_channel_vec_by_channel() {
        let config = SnapshotConfig::with_samples(150);
        // Create stereo input data
        let left_channel: Vec<f32> = (0..150)
            .map(|i| (i as f32 / 75.0 * std::f32::consts::PI).sin())
            .collect();
        let right_channel: Vec<f32> = (0..150)
            .map(|i| (i as f32 / 75.0 * std::f32::consts::PI).cos())
            .collect();

        let node = resonator_hz(440.0, 100.0) | resonator_hz(440.0, 100.0);

        snapshot_audionode_with_input_and_options(
            "multi_channel_vec",
            node,
            InputSource::VecByChannel(vec![left_channel, right_channel]),
            config,
        );
    }
}
