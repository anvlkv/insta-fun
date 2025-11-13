use std::{cell::RefCell, rc::Rc};

use crate::input::InputSource;

#[derive(Default, Clone)]
pub enum WarmUp {
    /// No warm-up
    #[default]
    None,
    /// Warm-up with a specific number of samples
    Samples(usize),
    /// Warm-up with a specific duration in seconds
    Seconds(f64),
    /// Warm-up with a specific number of samples and input source
    SamplesWithInput {
        samples: usize,
        input: Rc<RefCell<InputSource>>,
    },
}

impl std::fmt::Debug for WarmUp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Samples(arg0) => f.debug_tuple("Samples").field(arg0).finish(),
            Self::Seconds(arg0) => f.debug_tuple("Seconds").field(arg0).finish(),
            Self::SamplesWithInput { samples, .. } => f
                .debug_struct("SamplesWithInput")
                .field("Samples", samples)
                .finish(),
        }
    }
}

impl WarmUp {
    pub fn warm_up_samples(&self, sample_rate: f64, num_inputs: usize) -> Vec<Vec<f32>> {
        let none_input = Rc::new(RefCell::new(InputSource::None));
        let (num_samples, input) = match self {
            WarmUp::None => (0, &none_input),
            WarmUp::Samples(samples) => (*samples, &none_input),
            WarmUp::Seconds(seconds) => ((*seconds * sample_rate) as usize, &none_input),
            WarmUp::SamplesWithInput { samples, input } => (*samples, input),
        };

        input.borrow_mut().make_data(num_inputs, num_samples)
    }
}
