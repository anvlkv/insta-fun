#![doc = include_str!("../README.md")]

use std::fmt::Write;

use fundsp::prelude::*;
use insta::assert_snapshot;

const DEFAULT_HEIGHT: usize = 100;

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Default)]
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

pub enum InputSource {
    None,
    Array(Vec<Vec<f64>>),
    Generator(Box<dyn Fn(usize, usize) -> f64>),
}

impl InputSource {
    pub fn impulse() -> Self {
        Self::Generator(Box::new(|i, _| if i == 0 { 1.0 } else { 0.0 }))
    }
    pub fn sine(freq: f64, sr: f64) -> Self {
        Self::Generator(Box::new(move |i, _| {
            let phase = 2.0 * std::f64::consts::PI * freq * i as f64 / sr;
            phase.sin()
        }))
    }
}

const CHANNEL_COLORS: &[&str] = &[
    "#4285F4", "#EA4335", "#FBBC04", "#34A853", "#FF6D00", "#AB47BC", "#00ACC1", "#7CB342",
];

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
        InputSource::Array(data) => {
            assert_eq!(data.len(), num_inputs, "input array size mismatch");
            data
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
                        let value: f64 = input_data[*i];
                        input_buff.set_f32(ch, *i, value as f32);
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

    let svg = generate_svg(&output_data, &config);

    assert_snapshot!(format!("{name}.svg"), svg);
}

fn generate_svg(output_data: &[Vec<f32>], config: &SnapshotConfig) -> String {
    let height_per_channel = config.svg_height_per_channel.unwrap_or(DEFAULT_HEIGHT);
    let num_channels = output_data.len();
    let num_samples = output_data.first().map(|c| c.len()).unwrap_or(0);
    if num_samples == 0 || num_channels == 0 {
        return "<svg xmlns=\"http://www.w3.org/2000/svg\"><text>Empty</text></svg>".to_string();
    }

    let svg_width = config.svg_width.unwrap_or(config.num_samples);
    let total_height = height_per_channel * num_channels;
    let mut svg = String::new();
    writeln!(
        &mut svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" preserveAspectRatio="xMinYMin">"#,
        svg_width, total_height
    ).unwrap();

    let y_scale = (height_per_channel as f32 / 2.0) * 0.9;
    let x_scale = config
        .svg_width
        .map(|width| width as f32 / config.num_samples as f32);

    for (ch, data) in output_data.iter().enumerate() {
        let color = CHANNEL_COLORS[ch % CHANNEL_COLORS.len()];
        let y_offset = ch * height_per_channel;
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
            r#"  <path d="{}" fill="none" stroke="{}" stroke-width="0.5"/>"#,
            path_data, color
        )
        .unwrap();
        writeln!(
            &mut svg,
            r#"  <text x="5" y="{}" font-family="monospace" font-size="12" fill="{}">Ch{}</text>"#,
            y_offset + 15,
            color,
            ch
        )
        .unwrap();
    }
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
        let input = (0..100).map(|i| (i as f64 / 50.0).sin()).collect();

        snapshot_audionode_with_input_and_options(
            "filter_sine",
            lowpass_hz(500.0, 0.7),
            InputSource::Array(vec![input]),
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
}
