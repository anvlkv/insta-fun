use derive_builder::Builder;
use fundsp::DEFAULT_SR;

use crate::warmup::WarmUp;

pub use crate::chart::Layout;

const DEFAULT_HEIGHT: usize = 500;

#[derive(Debug, Clone, Builder)]
/// Configuration for snapshotting an audio unit.
pub struct SnapshotConfig {
    /// Number of samples to generate.
    ///
    /// Default is 1024
    #[builder(default = "1024")]
    pub num_samples: usize,
    /// Sample rate of the audio unit.
    ///
    /// Default is 44100.0 [fundsp::DEFAULT_SR]
    #[builder(default = "fundsp::DEFAULT_SR")]
    pub sample_rate: f64,
    /// Optional width of the SVG `viewBox`
    ///
    /// `None` means proportional to num_samples
    #[builder(default, setter(strip_option))]
    pub svg_width: Option<usize>,
    /// Height of **one** channel in the SVG `viewBox`
    ///
    /// Default - 500
    #[builder(default = "DEFAULT_HEIGHT")]
    pub svg_height_per_channel: usize,
    /// Processing mode for snapshotting an audio unit.
    ///
    /// Default - `Tick`
    #[builder(default = "Processing::default()")]
    pub processing_mode: Processing,
    /// Whether to include inputs in snapshot
    ///
    /// Default - `false`
    #[builder(default)]
    pub with_inputs: bool,
    /// Optional chart title
    ///
    /// Default - `None`
    #[builder(default, setter(into, strip_option))]
    pub chart_title: Option<String>,
    /// Optional titles for output channels
    ///
    /// Default - empty `Vec`
    #[builder(default, setter(into, each(into, name = "output_title")))]
    pub output_titles: Vec<String>,
    /// Optional titles for input channels
    ///
    /// Default - empty `Vec`
    #[builder(default, setter(into, each(into, name = "input_title")))]
    pub input_titles: Vec<String>,
    /// Show grid lines on the chart
    ///
    /// Default - `false`
    #[builder(default)]
    pub show_grid: bool,
    /// Show ax- labels
    ///
    /// Default - `true`
    #[builder(default = "true")]
    pub show_labels: bool,
    /// Custom colors for output channels (hex strings)
    ///
    /// Default - `None` (uses default palette)
    #[builder(default, setter(into, strip_option, each(into, name = "output_color")))]
    pub output_colors: Option<Vec<String>>,
    /// Custom colors for input channels (hex strings)
    ///
    /// Default - `None` (uses default palette)
    #[builder(default, setter(into, strip_option, each(into, name = "input_color")))]
    pub input_colors: Option<Vec<String>>,
    /// Chart background color (hex string)
    ///
    /// Default - "#000000" (black)
    #[builder(default = "\"#000000\".to_string()", setter(into))]
    pub background_color: String,
    /// Waveform line thickness
    ///
    /// Default - 2.0
    #[builder(default = "2.0")]
    pub line_width: f32,
    /// Warm-up mode for snapshotting an audio unit.
    ///
    /// Default - `WarmUp::None`
    #[builder(default = "WarmUp::None")]
    pub warm_up: WarmUp,
    /// How to handle abnormal samples: `NaN`,`±Infinity`
    ///
    /// When set to `true` abnormal samples are allowed during processing,
    /// but skipped in actual output. Plotted with labeled dots.
    ///
    /// When set to `false` and encoutered abnormal samples,
    /// the snapshotting process will panic.
    #[builder(default = "false")]
    pub allow_abnormal_samples: bool,
    /// Chart layout
    ///
    /// Whether to plot channels on separate charts or combined charts.
    ///
    /// Default - `Layout::Separate`
    #[builder(default)]
    pub chart_layout: Layout,
}

/// Processing mode for snapshotting an audio unit.
#[derive(Debug, Clone, Copy, Default)]
pub enum Processing {
    #[default]
    /// Process one sample at a time.
    Tick,
    /// Process a batch of samples at a time.
    ///
    /// max batch size is 64 [fundsp::MAX_BUFFER_SIZE]
    Batch(u8),
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            num_samples: 1024,
            sample_rate: DEFAULT_SR,
            svg_width: None,
            svg_height_per_channel: DEFAULT_HEIGHT,
            processing_mode: Processing::default(),
            with_inputs: false,
            chart_title: None,
            output_titles: Vec::new(),
            input_titles: Vec::new(),
            show_grid: false,
            show_labels: true,
            output_colors: None,
            input_colors: None,
            background_color: "#000000".to_string(),
            line_width: 2.0,
            warm_up: WarmUp::default(),
            allow_abnormal_samples: false,
            chart_layout: Layout::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_builder() {
        SnapshotConfigBuilder::default()
            .build()
            .expect("defaul config builds");
    }
}
