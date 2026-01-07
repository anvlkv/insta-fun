#![doc = include_str!("../README.md")]

mod abnormal;
mod chart;
mod chart_data;
mod util;
mod wav;

pub mod config;
pub mod graph;
pub mod input;
pub mod macros;
pub mod snapshot;
pub mod warmup;

pub mod prelude {
    pub use crate::chart::Layout;
    pub use crate::config::*;
    pub use crate::graph::*;
    pub use crate::input::*;
    pub use crate::snapshot::*;
    pub use crate::warmup::*;
    pub use crate::{assert_audio_unit_snapshot, assert_dsp_net_snapshot};
}

#[cfg(test)]
mod tests;
