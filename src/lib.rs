#![doc = include_str!("../README.md")]

extern crate self as insta_fun;

mod abnormal;
mod chart;
mod chart_data;
mod meta_dashboard;
mod util;
mod wav;

pub mod config;
#[cfg(feature = "dot")]
pub mod graph;

#[cfg(not(feature = "dot"))]
pub mod graph {
    use fundsp::net::Net;

    // Stub implementation when the `dot` feature is disabled.
    // Keeps APIs available so dependent code compiles, but returns a minimal DOT.
    pub fn snapshot_dsp_net_wiring(_net: Net) -> Vec<u8> {
        b"digraph { /* dot feature disabled */ }".to_vec()
    }
}
pub mod input;
pub mod macros;
pub mod meta;
pub mod snapshot;
pub mod warmup;

pub mod prelude {
    pub use crate::chart::Layout;
    pub use crate::config::*;
    pub use crate::graph::*;
    pub use crate::input::*;
    pub use crate::meta::*;
    pub use crate::snapshot::*;
    pub use crate::warmup::*;
    pub use crate::{
        assert_audio_unit_meta_data_snapshot, assert_audio_unit_snapshot, assert_dsp_net_snapshot,
    };
}

#[cfg(test)]
mod tests;
