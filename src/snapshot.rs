use fundsp::prelude::*;

use crate::chart::{AbnormalSample, generate_svg};
use crate::config::{Processing, SnapshotConfig};
use crate::input::InputSource;

/// Create an SVG snapshot of audio unit outputs
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::hacker::prelude::*;
///
/// let unit = sine_hz::<f32>(440.0);
/// let svg = snapshot_audio_unit(unit);
/// println!("{svg}");
/// ```
pub fn snapshot_audio_unit<N>(unit: N) -> String
where
    N: AudioUnit,
{
    snapshot_audio_unit_with_input_and_options(unit, InputSource::None, SnapshotConfig::default())
}

/// Create an SVG snapshot of audio unit outputs, with options
///
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::hacker::prelude::*;
///
/// let unit = sine_hz::<f32>(440.0);
/// let svg = snapshot_audio_unit_with_options(unit, SnapshotConfig::default());
/// println!("{svg}");
/// ```
pub fn snapshot_audio_unit_with_options<N>(unit: N, options: SnapshotConfig) -> String
where
    N: AudioUnit,
{
    snapshot_audio_unit_with_input_and_options(unit, InputSource::None, options)
}

/// Create an SVG snapshot of audio unit inputs and outputs
///
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::hacker::prelude::*;
///
/// let unit = sine_hz::<f32>(440.0);
/// let svg = snapshot_audio_unit_with_input(unit, InputSource::None);
/// println!("{svg}");
/// ```
pub fn snapshot_audio_unit_with_input<N>(unit: N, input_source: InputSource) -> String
where
    N: AudioUnit,
{
    snapshot_audio_unit_with_input_and_options(
        unit,
        input_source,
        SnapshotConfig {
            with_inputs: true,
            ..SnapshotConfig::default()
        },
    )
}

/// Create an SVG snapshot of audio unit inputs and outputs, with options
///
/// ## Example
///
/// ```
/// use insta_fun::prelude::*;
/// use fundsp::hacker::prelude::*;
///
/// let config = SnapshotConfig::default();
/// let unit = sine_hz::<f32>(440.0);
/// let svg = snapshot_audio_unit_with_input_and_options(unit, InputSource::None, config);
/// println!("{svg}");
/// ```
pub fn snapshot_audio_unit_with_input_and_options<N>(
    mut unit: N,
    mut input_source: InputSource,
    config: SnapshotConfig,
) -> String
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

    let mut abnormalities: Vec<Vec<(usize, AbnormalSample)>> = vec![vec![]; num_outputs];

    let mut checked_sample = |mut sample: f32, ch: usize, i: usize| {
        if sample.is_nan() || sample.is_infinite() {
            let abnormality = AbnormalSample::from(sample);

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
            input_frame[ch] = input_data[ch][i];
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

    generate_svg(&input_data, &output_data, &abnormalities, &config)
}
