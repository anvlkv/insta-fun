use std::io::Cursor;

use fundsp::wave::Wave;

use crate::config::WavOutput;

pub(crate) fn generate_wav(
    output_data: &[Vec<f32>],
    output_mode: &WavOutput,
    sample_rate: f64,
    num_samples: usize,
) -> Vec<u8> {
    let num_channels = output_data.len();

    let mut buffer = Vec::<u8>::new();
    let mut cursor = Cursor::new(&mut buffer);

    let mut wave_data = Wave::with_capacity(num_channels, sample_rate, num_samples);

    wave_data.resize(num_channels * num_samples);

    for (ch, channel_data) in output_data.iter().enumerate() {
        for (i, &sample) in channel_data.iter().take(num_samples).enumerate() {
            wave_data.set(ch, i, sample);
        }
    }

    match output_mode {
        WavOutput::Wav16 => wave_data
            .write_wav16(&mut cursor)
            .expect("write .wav snapshot data"),
        WavOutput::Wav32 => wave_data
            .write_wav32(&mut cursor)
            .expect("write .wav snapshot data"),
    }

    buffer
}
