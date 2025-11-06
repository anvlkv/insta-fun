/// Input provided to the audio unit
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
    pub fn sine(freq: f32, sample_rate: f32) -> Self {
        Self::Generator(Box::new(move |i, _| {
            let phase = 2.0 * std::f32::consts::PI * freq * i as f32 / sample_rate;
            phase.sin()
        }))
    }
}
