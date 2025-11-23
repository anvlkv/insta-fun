use fundsp::hacker::AudioUnit;

/// Input provided to the audio unit
#[derive(Default)]
pub enum InputSource {
    /// No input
    #[default]
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
    /// Input provided by an audio unit
    ///
    /// Number of outputs of the audio unit must match
    /// the number of inputs to the test target
    ///
    /// * if you need to set sample rate on input unit do that upfront
    Unit(Box<dyn AudioUnit>),
}

impl From<Box<dyn AudioUnit>> for InputSource {
    fn from(unit: Box<dyn AudioUnit>) -> Self {
        InputSource::Unit(unit)
    }
}

impl From<Box<dyn Fn(usize, usize) -> f32>> for InputSource {
    fn from(generator_fn: Box<dyn Fn(usize, usize) -> f32>) -> Self {
        InputSource::Generator(generator_fn)
    }
}

impl From<Vec<f32>> for InputSource {
    fn from(data: Vec<f32>) -> Self {
        InputSource::Flat(data)
    }
}

impl From<Vec<Vec<f32>>> for InputSource {
    fn from(data: Vec<Vec<f32>>) -> Self {
        InputSource::VecByChannel(data)
    }
}

impl InputSource {
    pub fn impulse() -> Self {
        Self::Generator(Box::new(|i, _| if i == 0 { 1.0 } else { 0.0 }))
    }
    pub fn sine(freq: f32, sample_rate: f32) -> Self {
        Self::Generator(Box::new(move |i, _| {
            let phase = 2.0 * std::f32::consts::PI * freq * i as f32 / sample_rate;
            phase.sin()
        }))
    }

    pub fn make_data(&mut self, num_inputs: usize, num_samples: usize) -> Vec<Vec<f32>> {
        match self {
            InputSource::None => vec![vec![0.0; num_samples]; num_inputs],
            InputSource::VecByChannel(data) => {
                assert_eq!(
                    data.len(),
                    num_inputs,
                    "Input vec size mismatch. Expected {} channels, got {}",
                    num_inputs,
                    data.len()
                );
                assert!(
                    data.iter().all(|v| v.len() == num_samples),
                    "Input vec size mismatch. Expected {} samples per channel, got {}",
                    num_samples,
                    data.iter().map(|v| v.len()).max().unwrap_or(0)
                );
                data.to_vec()
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
                    num_samples,
                    "Input vec size mismatch. Expected {} samples, got {}",
                    num_samples,
                    data.len()
                );
                (0..num_inputs)
                    .map(|ch| (0..num_samples).map(|i| data[i][ch]).collect())
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
                    .map(|ch| (0..num_samples).map(|_| data[ch]).collect())
                    .collect()
            }
            InputSource::Generator(generator_fn) => (0..num_inputs)
                .map(|ch| (0..num_samples).map(|i| generator_fn(i, ch)).collect())
                .collect(),
            InputSource::Unit(unit) => {
                // 1. Tick the driving unit with an output frame sized to its own outputs().
                // 2. Collect its raw outputs.
                // 3. Map/truncate/pad those outputs to the required num_inputs for the target snapshot.
                let unit_outputs = unit.outputs();

                // Raw capture buffer sized to the driving unit's actual outputs.
                let mut raw = vec![vec![0.0; num_samples]; unit_outputs];
                (0..num_samples).for_each(|i| {
                    let mut outputs = vec![0.0; unit_outputs];
                    unit.tick(&[], &mut outputs);
                    for ch in 0..unit_outputs {
                        raw[ch][i] = outputs[ch];
                    }
                });

                // Map raw outputs to target input channels.
                // If fewer outputs than required, remaining channels stay silent (zeros).
                // If more outputs than required, excess channels are discarded.
                let mut data = vec![vec![0.0; num_samples]; num_inputs];
                for ch in 0..num_inputs {
                    if ch < unit_outputs {
                        data[ch] = raw[ch].clone();
                    }
                }
                data
            }
        }
    }
}
