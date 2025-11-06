#![doc = include_str!("../README.md")]

mod chart;
pub mod config;
pub mod input;
pub mod macros;
pub mod snapshot;

pub mod prelude {
    pub use crate::assert_audio_unit_snapshot;
    pub use crate::config::*;
    pub use crate::input::*;
    pub use crate::snapshot::*;
}

#[cfg(test)]
mod tests;
