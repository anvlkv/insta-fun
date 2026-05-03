use fundsp::prelude::*;

use crate::abnormal::AbnormalSample;
use crate::chart::generate_svg;
use crate::config::{Processing, SnapshotConfig, SnapshotOutputMode};
use crate::input::InputSource;
use crate::wav::generate_wav;

/// Describes a non-finite sample value captured during audio unit processing.
///
/// Abnormal samples are collected when [`SnapshotConfig::allow_abnormal_samples`] is `true`;
/// the offending sample is replaced with `0.0` and recorded in
/// [`AudioUnitSnapshotData::abnormalities`]. When `allow_abnormal_samples` is `false` (default)
/// encountering any abnormal sample panics immediately.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotAbnormalSample {
    /// The sample value was `f32::NAN`.
    Nan,
    /// The sample value was `f32::NEG_INFINITY`.
    NegInf,
    /// The sample value was `f32::INFINITY`.
    PosInf,
}

impl std::fmt::Display for SnapshotAbnormalSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnapshotAbnormalSample::Nan => write!(f, "NaN"),
            SnapshotAbnormalSample::NegInf => write!(f, "-∞"),
            SnapshotAbnormalSample::PosInf => write!(f, "∞"),
        }
    }
}

impl From<AbnormalSample> for SnapshotAbnormalSample {
    fn from(value: AbnormalSample) -> Self {
        match value {
            AbnormalSample::Nan => Self::Nan,
            AbnormalSample::NegInf => Self::NegInf,
            AbnormalSample::PosInf => Self::PosInf,
        }
    }
}

impl From<SnapshotAbnormalSample> for AbnormalSample {
    fn from(value: SnapshotAbnormalSample) -> Self {
        match value {
            SnapshotAbnormalSample::Nan => Self::Nan,
            SnapshotAbnormalSample::NegInf => Self::NegInf,
            SnapshotAbnormalSample::PosInf => Self::PosInf,
        }
    }
}

/// Raw sample buffers and metadata captured from one run of an audio unit.
///
/// Returned by the `snapshot_audio_unit_data*` family of functions and passed to the
/// closure argument of [`assert_audio_unit_data!`].
///
/// All sample buffers are indexed as `[channel][sample]`.
#[derive(Debug, Clone)]
pub struct AudioUnitSnapshotData {
    /// Input samples fed to the unit, one `Vec<f32>` per input channel.
    pub input_data: Vec<Vec<f32>>,
    /// Output samples produced by the unit, one `Vec<f32>` per output channel.
    pub output_data: Vec<Vec<f32>>,
    /// Abnormal (NaN / ±∞) samples per output channel.
    ///
    /// Each entry is `(sample_index, kind)`. Only populated when
    /// [`SnapshotConfig::allow_abnormal_samples`] is `true`; otherwise the processing
    /// function panics on the first abnormal sample.
    pub abnormalities: Vec<Vec<(usize, SnapshotAbnormalSample)>>,
    /// Sample rate used during processing, in Hz.
    pub sample_rate: f64,
    /// Number of output samples captured (excluding warmup).
    pub num_samples: usize,
    /// Index of the first captured sample relative to the start of the stream
    /// (i.e. the warmup length in samples).
    pub start_sample: usize,
}

/// Create a snapshot of audio unit outputs (default: SVG; configure `output_mode` for WAV)
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::prelude::*;
///
/// let unit = sine_hz::<f32>(440.0);
/// let svg = snapshot_audio_unit(unit);
/// println!("{}", svg.len());
/// ```
pub fn snapshot_audio_unit<N>(unit: N) -> Vec<u8>
where
    N: AudioUnit,
{
    snapshot_audio_unit_with_input_and_options(unit, InputSource::None, SnapshotConfig::default())
}

/// Capture raw sample data from an audio unit using default settings.
///
/// Equivalent to calling [`snapshot_audio_unit_data_with_input_and_options`] with
/// [`InputSource::None`] and a default [`SnapshotConfig`].
///
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::prelude::*;
///
/// let data = snapshot_audio_unit_data(sine_hz::<f32>(440.0));
/// assert_eq!(data.output_data.len(), 1);
/// assert_eq!(data.output_data[0].len(), data.num_samples);
/// ```
pub fn snapshot_audio_unit_data<N>(unit: N) -> AudioUnitSnapshotData
where
    N: AudioUnit,
{
    snapshot_audio_unit_data_with_input_and_options(unit, InputSource::None, SnapshotConfig::default())
}

/// Create a snapshot of audio unit outputs with options (SVG or WAV via `output_mode`)
///
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::prelude::*;
///
/// let unit = sine_hz::<f32>(440.0);
/// let svg = snapshot_audio_unit_with_options(unit, SnapshotConfig::default());
/// println!("{}", svg.len());
/// ```
pub fn snapshot_audio_unit_with_options<N>(unit: N, options: SnapshotConfig) -> Vec<u8>
where
    N: AudioUnit,
{
    snapshot_audio_unit_with_input_and_options(unit, InputSource::None, options)
}

/// Capture raw sample data from an audio unit with custom [`SnapshotConfig`] settings.
///
/// Equivalent to calling [`snapshot_audio_unit_data_with_input_and_options`] with
/// [`InputSource::None`].
///
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::prelude::*;
///
/// let config = SnapshotConfigBuilder::default().num_samples(512).build().unwrap();
/// let data = snapshot_audio_unit_data_with_options(sine_hz::<f32>(440.0), config);
/// assert_eq!(data.num_samples, 512);
/// ```
pub fn snapshot_audio_unit_data_with_options<N>(unit: N, options: SnapshotConfig) -> AudioUnitSnapshotData
where
    N: AudioUnit,
{
    snapshot_audio_unit_data_with_input_and_options(unit, InputSource::None, options)
}

/// Create a snapshot of audio unit inputs and outputs (SVG by default; WAV if selected)
///
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::prelude::*;
///
/// let unit = sine_hz::<f32>(440.0);
/// let svg = snapshot_audio_unit_with_input(unit, InputSource::None);
/// println!("{}", svg.len());
/// ```
pub fn snapshot_audio_unit_with_input<N>(unit: N, input_source: InputSource) -> Vec<u8>
where
    N: AudioUnit,
{
    snapshot_audio_unit_with_input_and_options(
        unit,
        input_source,
        SnapshotConfig {
            ..SnapshotConfig::default()
        },
    )
}

/// Capture raw sample data from an audio unit driven by the given [`InputSource`].
///
/// Uses a default [`SnapshotConfig`]. For full control over both input and config
/// use [`snapshot_audio_unit_data_with_input_and_options`].
///
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::prelude::*;
///
/// let data = snapshot_audio_unit_data_with_input(lowpass_hz(1000.0, 0.7), InputSource::impulse());
/// assert_eq!(data.input_data.len(), 1);
/// assert_eq!(data.output_data.len(), 1);
/// ```
pub fn snapshot_audio_unit_data_with_input<N>(unit: N, input_source: InputSource) -> AudioUnitSnapshotData
where
    N: AudioUnit,
{
    snapshot_audio_unit_data_with_input_and_options(
        unit,
        input_source,
        SnapshotConfig {
            ..SnapshotConfig::default()
        },
    )
}

/// Create a snapshot (inputs & outputs) with options (choose SVG chart or WAV via `output_mode`)
///
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::prelude::*;
///
/// let config = SnapshotConfig::default();
/// let unit = sine_hz::<f32>(440.0);
/// let svg = snapshot_audio_unit_with_input_and_options(unit, InputSource::None, config);
/// println!("{}", svg.len());
/// ```
pub fn snapshot_audio_unit_with_input_and_options<N>(
    unit: N,
    input_source: InputSource,
    config: SnapshotConfig,
) -> Vec<u8>
where
    N: AudioUnit,
{
    let snapshot_data = capture_audio_unit_data(unit, input_source, &config);
    render_snapshot_output(&snapshot_data, &config.output_mode)
}

/// Capture raw sample data from an audio unit with a given [`InputSource`] and [`SnapshotConfig`].
///
/// This is the most flexible entry point for raw data capture. The returned
/// [`AudioUnitSnapshotData`] gives direct access to input/output buffers and any
/// recorded abnormal samples, without writing any SVG or WAV file.
///
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::prelude::*;
///
/// let config = SnapshotConfigBuilder::default().num_samples(64).build().unwrap();
/// let data = snapshot_audio_unit_data_with_input_and_options(
///     pass(),
///     InputSource::Generator(Box::new(|i, _| if i % 2 == 0 { 1.0 } else { -1.0 })),
///     config,
/// );
/// assert_eq!(data.output_data[0].len(), 64);
/// ```
pub fn snapshot_audio_unit_data_with_input_and_options<N>(
    unit: N,
    input_source: InputSource,
    config: SnapshotConfig,
) -> AudioUnitSnapshotData
where
    N: AudioUnit,
{
    capture_audio_unit_data(unit, input_source, &config)
}

fn capture_audio_unit_data<N>(
    mut unit: N,
    mut input_source: InputSource,
    config: &SnapshotConfig,
) -> AudioUnitSnapshotData
where
    N: AudioUnit,
{
    let num_inputs = N::inputs(&unit);
    let num_outputs = N::outputs(&unit);

    unit.set_sample_rate(config.sample_rate);
    unit.reset();
    unit.allocate();

    let input_data = input_source.make_data(num_inputs, config.num_samples);

    let mut output_data: Vec<Vec<f32>> = vec![vec![]; num_outputs];

    let warmup_samples = config
        .warm_up
        .warm_up_samples(config.sample_rate, num_inputs);

    let num_warmup_samples = warmup_samples
        .iter()
        .map(|ch| ch.len())
        .next()
        .unwrap_or_default();

    let mut abnormalities: Vec<Vec<(usize, SnapshotAbnormalSample)>> = vec![vec![]; num_outputs];

    let mut checked_sample = |mut sample: f32, ch: usize, i: usize| {
        if sample.is_nan() || sample.is_infinite() {
            let abnormality = SnapshotAbnormalSample::from(AbnormalSample::from(sample));

            if config.allow_abnormal_samples {
                abnormalities[ch].push((i, abnormality));
                sample = 0.0;
            } else {
                panic!("Output channel #[{ch}] at sample [{i}] produced [{abnormality}] sample");
            }
        }
        sample
    };

    (0..num_warmup_samples).for_each(|i| {
        let mut input_frame = vec![0.0; num_inputs];
        for ch in 0..num_inputs {
            input_frame[ch] = warmup_samples[ch][i];
        }
        let mut output_frame = vec![0.0; num_outputs];
        unit.tick(&input_frame, &mut output_frame);
        // do nothing, warmup samples
    });

    match config.processing_mode {
        Processing::Tick => {
            (0..config.num_samples).for_each(|i| {
                let mut input_frame = vec![0.0; num_inputs];
                for ch in 0..num_inputs {
                    input_frame[ch] = input_data[ch][i];
                }
                let mut output_frame = vec![0.0; num_outputs];
                unit.tick(&input_frame, &mut output_frame);
                for ch in 0..num_outputs {
                    let sample = checked_sample(output_frame[ch], ch, i);
                    output_data[ch].push(sample);
                }
            });
        }
        Processing::Batch(batch_size) => {
            assert!(
                batch_size <= MAX_BUFFER_SIZE as u8,
                "Batch size must be less than or equal to [{MAX_BUFFER_SIZE}]"
            );

            let samples_index = (0..config.num_samples).collect::<Vec<_>>();
            for chunk in samples_index.chunks(batch_size as usize) {
                let mut input_buff = BufferVec::new(num_inputs);
                for (frame_index, input_index) in chunk.iter().enumerate() {
                    for (ch, input) in input_data.iter().enumerate() {
                        let value: f32 = input[*input_index];
                        input_buff.set_f32(ch, frame_index, value);
                    }
                }
                let input_ref = input_buff.buffer_ref();
                let mut output_buf = BufferVec::new(num_outputs);
                let mut output_ref = output_buf.buffer_mut();

                unit.process(chunk.len(), &input_ref, &mut output_ref);

                for (ch, data) in output_data.iter_mut().enumerate() {
                    data.extend(
                        output_buf
                            .channel_f32(ch)
                            .iter()
                            .enumerate()
                            .map(|(i, &value)| checked_sample(value, ch, i + chunk[0])),
                    );
                }
            }
        }
    }

    AudioUnitSnapshotData {
        input_data,
        output_data,
        abnormalities,
        sample_rate: config.sample_rate,
        num_samples: config.num_samples,
        start_sample: config.warm_up.num_samples(config.sample_rate),
    }
}

fn render_snapshot_output(data: &AudioUnitSnapshotData, output_mode: &SnapshotOutputMode) -> Vec<u8> {
    match output_mode {
        SnapshotOutputMode::SvgChart(svg_chart_config) => {
            let abnormalities = data
                .abnormalities
                .iter()
                .map(|channel| {
                    channel
                        .iter()
                        .map(|(i, abnormality)| (*i, AbnormalSample::from(*abnormality)))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            generate_svg(
                &data.input_data,
                &data.output_data,
                &abnormalities,
                svg_chart_config,
                data.sample_rate,
                data.num_samples,
                data.start_sample,
            )
            .as_bytes()
            .to_vec()
        }
        SnapshotOutputMode::Wav(wav_output) => {
            generate_wav(&data.output_data, wav_output, data.sample_rate, data.num_samples)
        }
    }
}
