#![doc = include_str!("../README.md")]

mod abnormal;
mod chart;
mod chart_data;
mod util;

pub mod config;
pub mod input;
pub mod macros;
pub mod snapshot;
pub mod warmup;

pub mod prelude {
    pub use crate::assert_audio_unit_snapshot;
    pub use crate::chart::Layout;
    pub use crate::config::*;
    pub use crate::input::*;
    pub use crate::snapshot::*;
    pub use crate::warmup::*;
}

#[cfg(test)]
mod tests;
