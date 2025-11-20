use derive_builder::Builder;
use fundsp::DEFAULT_SR;

use crate::warmup::WarmUp;

pub use crate::chart::Layout;

const DEFAULT_HEIGHT: usize = 500;

#[derive(Debug, Clone, Builder)]
/// Configuration for snapshotting an audio unit.
pub struct SnapshotConfig {
    // Audio configuration
    /// Sample rate of the audio unit.
    ///
    /// Default is 44100.0 [fundsp::DEFAULT_SR]
    #[builder(default = "fundsp::DEFAULT_SR")]
    pub sample_rate: f64,
    /// Number of samples to generate.
    ///
    /// Default is 1024
    #[builder(default = "1024")]
    pub num_samples: usize,
    /// Processing mode for snapshotting an audio unit.
    ///
    /// Default - `Tick`
    #[builder(default = "Processing::default()")]
    pub processing_mode: Processing,
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

    /// Snaphsot output mode
    ///
    /// Use configurable chart for visual snapshots
    ///
    /// Use Wav16 or Wav32 for audial snapshots
    #[builder(
        default = "SnapshotOutputMode::SvgChart(SvgChartConfig::default())",
        try_setter,
        setter(into)
    )]
    pub output_mode: SnapshotOutputMode,
}

#[derive(Debug, Clone, Builder)]
pub struct SvgChartConfig {
    // Chart configuration
    /// Chart layout
    ///
    /// Whether to plot channels on separate charts or combined charts.
    ///
    /// Default - `Layout::Separate`
    #[builder(default)]
    pub chart_layout: Layout,
    /// Whether to include inputs in snapshot
    ///
    /// Default - `false`
    #[builder(default)]
    pub with_inputs: bool,
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

    // Chart labels
    /// Show ax- labels
    ///
    /// Default - `true`
    #[builder(default = "true")]
    pub show_labels: bool,
    /// X axis labels format
    ///
    /// Whether to format X axis labels as time
    ///
    /// Default - `false`
    #[builder(default)]
    pub format_x_axis_labels_as_time: bool,
    /// Maximum number of labels along X axis
    ///
    /// Default - `Some(5)`
    #[builder(default = "Some(5)")]
    pub max_labels_x_axis: Option<usize>,
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

    // Lines
    /// Show grid lines on the chart
    ///
    /// Default - `false`
    #[builder(default)]
    pub show_grid: bool,
    /// Waveform line thickness
    ///
    /// Default - 2.0
    #[builder(default = "2.0")]
    pub line_width: f32,

    // Chart colors
    /// Chart background color (hex string)
    ///
    /// Default - "#000000" (black)
    #[builder(default = "\"#000000\".to_string()", setter(into))]
    pub background_color: String,
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
}

#[derive(Debug, Clone)]
pub enum WavOutput {
    Wav16,
    Wav32,
}

#[derive(Debug, Clone)]
pub enum SnapshotOutputMode {
    SvgChart(SvgChartConfig),
    Wav(WavOutput),
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

impl TryFrom<SvgChartConfigBuilder> for SnapshotOutputMode {
    type Error = SvgChartConfigBuilderError;

    fn try_from(value: SvgChartConfigBuilder) -> Result<Self, Self::Error> {
        let inner = value.build()?;
        Ok(SnapshotOutputMode::SvgChart(inner))
    }
}

impl From<WavOutput> for SnapshotOutputMode {
    fn from(value: WavOutput) -> Self {
        SnapshotOutputMode::Wav(value)
    }
}

impl From<SvgChartConfig> for SnapshotOutputMode {
    fn from(value: SvgChartConfig) -> Self {
        SnapshotOutputMode::SvgChart(value)
    }
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            num_samples: 1024,
            sample_rate: DEFAULT_SR,
            processing_mode: Processing::default(),
            warm_up: WarmUp::default(),
            allow_abnormal_samples: false,
            output_mode: SnapshotOutputMode::SvgChart(SvgChartConfig::default()),
        }
    }
}

impl Default for SvgChartConfig {
    fn default() -> Self {
        Self {
            svg_width: None,
            svg_height_per_channel: DEFAULT_HEIGHT,
            with_inputs: false,
            chart_title: None,
            output_titles: Vec::new(),
            input_titles: Vec::new(),
            show_grid: false,
            show_labels: true,
            max_labels_x_axis: Some(5),
            output_colors: None,
            input_colors: None,
            background_color: "#000000".to_string(),
            line_width: 2.0,
            chart_layout: Layout::default(),
            format_x_axis_labels_as_time: false,
        }
    }
}

impl SnapshotConfig {
    /// Intnded for internal use only
    ///
    /// Used by macros to determine snapshot filename
    pub fn file_name(&self, name: Option<&'_ str>) -> String {
        match &self.output_mode {
            SnapshotOutputMode::SvgChart(svg_chart_config) => match name {
                Some(name) => format!("{name}.svg"),
                None => match &svg_chart_config.chart_title {
                    Some(name) => format!("{name}.svg"),
                    None => ".svg".to_string(),
                },
            },
            SnapshotOutputMode::Wav(_) => match name {
                Some(name) => format!("{name}.wav"),
                None => ".wav".to_string(),
            },
        }
    }

    /// Intnded for internal use only
    ///
    /// Used by macros to set chart title if not already set
    pub fn maybe_title(&mut self, name: &str) {
        if matches!(
            self.output_mode,
            SnapshotOutputMode::SvgChart(SvgChartConfig {
                chart_title: None,
                ..
            })
        ) && let SnapshotOutputMode::SvgChart(ref mut svg_chart_config) = self.output_mode
        {
            svg_chart_config.chart_title = Some(name.to_string());
        }
    }
}

impl SnapshotConfigBuilder {
    /// Internal helper to ensure we have a mutable reference to an underlying `SvgChartConfig`
    /// Creating a default one if `output_mode` is `None` or replacing a `Wav` variant.
    fn legacy_svg_mut(&mut self) -> &mut SvgChartConfig {
        // If already a chart, return it.
        if let Some(SnapshotOutputMode::SvgChart(ref mut chart)) = self.output_mode {
            return chart;
        }
        // Otherwise replace (None or Wav) with default chart.
        self.output_mode = Some(SnapshotOutputMode::SvgChart(SvgChartConfig::default()));
        match self.output_mode {
            Some(SnapshotOutputMode::SvgChart(ref mut chart)) => chart,
            _ => unreachable!("Output mode was just set to SvgChart"),
        }
    }

    /// Set chart layout.
    pub fn chart_layout(&mut self, value: Layout) -> &mut Self {
        self.legacy_svg_mut().chart_layout = value;
        self
    }

    /// Include inputs in chart.
    pub fn with_inputs(&mut self, value: bool) -> &mut Self {
        self.legacy_svg_mut().with_inputs = value;
        self
    }

    /// Set fixed SVG width.
    pub fn svg_width(&mut self, value: usize) -> &mut Self {
        self.legacy_svg_mut().svg_width = Some(value);
        self
    }

    /// Set SVG height per channel.
    pub fn svg_height_per_channel(&mut self, value: usize) -> &mut Self {
        self.legacy_svg_mut().svg_height_per_channel = value;
        self
    }

    /// Toggle label visibility.
    pub fn show_labels(&mut self, value: bool) -> &mut Self {
        self.legacy_svg_mut().show_labels = value;
        self
    }

    /// Format X axis labels as time.
    pub fn format_x_axis_labels_as_time(&mut self, value: bool) -> &mut Self {
        self.legacy_svg_mut().format_x_axis_labels_as_time = value;
        self
    }

    /// Set maximum number of X axis labels.
    pub fn max_labels_x_axis(&mut self, value: Option<usize>) -> &mut Self {
        self.legacy_svg_mut().max_labels_x_axis = value;
        self
    }

    /// Set chart title.
    pub fn chart_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.legacy_svg_mut().chart_title = Some(value.into());
        self
    }

    /// Add an output channel title.
    pub fn output_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.legacy_svg_mut().output_titles.push(value.into());
        self
    }

    /// Add an input channel title.
    pub fn input_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.legacy_svg_mut().input_titles.push(value.into());
        self
    }

    /// Add output channels' titles.
    pub fn output_titles<S: Into<Vec<String>>>(&mut self, value: S) -> &mut Self {
        self.legacy_svg_mut().output_titles = value.into();
        self
    }

    /// Add input channels' titles.
    pub fn input_titles<S: Into<Vec<String>>>(&mut self, value: S) -> &mut Self {
        self.legacy_svg_mut().input_titles = value.into();
        self
    }

    /// Show grid lines.
    pub fn show_grid(&mut self, value: bool) -> &mut Self {
        self.legacy_svg_mut().show_grid = value;
        self
    }

    /// Set waveform line width.
    pub fn line_width(&mut self, value: f32) -> &mut Self {
        self.legacy_svg_mut().line_width = value;
        self
    }

    /// Set background color.
    pub fn background_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.legacy_svg_mut().background_color = value.into();
        self
    }

    /// Replace all output channel colors.
    pub fn output_colors(&mut self, colors: Vec<String>) -> &mut Self {
        self.legacy_svg_mut().output_colors = Some(colors);
        self
    }

    /// Append one output channel color.
    pub fn output_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let chart = self.legacy_svg_mut();
        chart
            .output_colors
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }

    /// Replace all input channel colors.
    pub fn input_colors(&mut self, colors: Vec<String>) -> &mut Self {
        self.legacy_svg_mut().input_colors = Some(colors);
        self
    }

    /// Append one input channel color.
    pub fn input_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let chart = self.legacy_svg_mut();
        chart
            .input_colors
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
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

    #[test]
    fn legacy_config_compat() {
        SnapshotConfigBuilder::default()
            .chart_title("Complete Waveform Test")
            .show_grid(true)
            .show_labels(true)
            .with_inputs(true)
            .output_color("#FF6B6B")
            .input_color("#95E77E")
            .background_color("#2C3E50")
            .line_width(3.0)
            .svg_width(1200)
            .svg_height_per_channel(120)
            .build()
            .expect("legacy config builds");
    }
}
