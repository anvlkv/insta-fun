use derive_builder::Builder;
use fundsp::DEFAULT_SR;

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
    #[builder(default = "None")]
    pub svg_width: Option<usize>,
    /// Height of **one** channel in the SVG `viewBox`
    ///
    /// `None` fallbacks to default - 100
    #[builder(default = "DEFAULT_HEIGHT")]
    pub svg_height_per_channel: usize,
    /// Processing mode for snapshotting an audio unit.
    ///
    /// Default is `Tick`
    #[builder(default = "Processing::default()")]
    pub processing_mode: Processing,
    /// Whether to include inputs in snapshot
    ///
    /// Default is `false`
    #[builder(default = "false")]
    pub with_inputs: bool,
    /// Optional chart title
    ///
    /// Default is `None`
    #[builder(default = "None")]
    pub chart_title: Option<String>,
    /// Show grid lines on the chart
    ///
    /// Default is `false`
    #[builder(default = "false")]
    pub show_grid: bool,
    /// Show axis labels
    ///
    /// Default is `true`
    #[builder(default = "true")]
    pub show_labels: bool,
    /// Custom colors for output channels (hex strings)
    ///
    /// Default is `None` (uses default palette)
    #[builder(default = "None")]
    pub output_colors: Option<Vec<String>>,
    /// Custom colors for input channels (hex strings)
    ///
    /// Default is `None` (uses default palette)
    #[builder(default = "None")]
    pub input_colors: Option<Vec<String>>,
    /// Chart background color (hex string)
    ///
    /// Default is "#000000" (black)
    #[builder(default = "\"#000000\".to_string()")]
    pub background_color: String,
    /// Waveform line thickness
    ///
    /// Default is 2.0
    #[builder(default = "2.0")]
    pub line_width: f32,
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
            show_grid: false,
            show_labels: true,
            output_colors: None,
            input_colors: None,
            background_color: "#000000".to_string(),
            line_width: 2.0,
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
