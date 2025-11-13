use plotters::backend::SVGBackend;
use plotters::drawing::IntoDrawingArea;
use plotters::element::DashedPathElement;
use plotters::prelude::*;

use crate::config::SnapshotConfig;

const OUTPUT_CHANNEL_COLORS: &[&str] = &[
    "#4285F4", "#EA4335", "#FBBC04", "#34A853", "#FF6D00", "#AB47BC", "#00ACC1", "#7CB342",
    "#9C27B0", "#3F51B5", "#009688", "#8BC34A", "#FFEB3B", "#FF9800", "#795548", "#607D8B",
    "#E91E63", "#673AB7", "#2196F3", "#00BCD4", "#4CAF50", "#CDDC39", "#FFC107", "#FF5722",
    "#9E9E9E", "#03A9F4", "#8D6E63", "#78909C", "#880E4F", "#4A148C", "#0D47A1", "#004D40",
];

const INPUT_CHANNEL_COLORS: &[&str] = &[
    "#B39DDB", "#FFAB91", "#FFF59D", "#A5D6A7", "#FFCC80", "#CE93D8", "#80DEEA", "#C5E1A5",
    "#BA68C8", "#9FA8DA", "#80CBC4", "#DCE775", "#FFF176", "#FFB74D", "#BCAAA4", "#B0BEC5",
    "#F48FB1", "#B39DDB", "#90CAF9", "#80DEEA", "#A5D6A7", "#E6EE9C", "#FFD54F", "#FF8A65",
    "#BDBDBD", "#81D4FA", "#A1887F", "#90A4AE", "#C2185B", "#7B1FA2", "#1976D2", "#00796B",
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
/// Chart layout
///
/// Whether to plot channels on separate charts or combined charts.
pub enum Layout {
    /// Each channel plots on its own chart
    #[default]
    SeparateChannels,
    /// All input channels plot on one chart, all output channels plot on another chart
    ///
    /// Same as `Combined` when `config.with_inputs` is `false`
    CombinedPerChannelType,
    /// All channels plot on one chart
    Combined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AbnormalSample {
    Nan,
    NegInf,
    PosInf,
}

impl std::fmt::Display for AbnormalSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbnormalSample::Nan => write!(f, "NaN"),
            AbnormalSample::NegInf => write!(f, "-∞"),
            AbnormalSample::PosInf => write!(f, "∞"),
        }
    }
}

impl From<f32> for AbnormalSample {
    fn from(sample: f32) -> Self {
        if sample.is_nan() {
            AbnormalSample::Nan
        } else if sample.is_infinite() && sample.is_sign_negative() {
            AbnormalSample::NegInf
        } else if sample.is_infinite() && sample.is_sign_positive() {
            AbnormalSample::PosInf
        } else {
            unreachable!()
        }
    }
}

#[allow(dead_code)]
struct ChannelChartData {
    data: Vec<f32>,
    abnormalities: Vec<(usize, AbnormalSample)>,
    color: RGBColor,
    label: Option<String>,
    is_input: bool,
    idx: usize,
}

impl ChannelChartData {
    fn from_input_data(data: &[f32], idx: usize, config: &SnapshotConfig) -> Self {
        let color = config
            .input_colors
            .as_ref()
            .and_then(|colors| colors.get(idx))
            .map(|s| s.as_str())
            .unwrap_or_else(|| INPUT_CHANNEL_COLORS[idx % INPUT_CHANNEL_COLORS.len()]);
        let color = parse_hex_color(color);

        let label = if config.show_labels {
            config
                .input_titles
                .get(idx)
                .cloned()
                .or_else(|| Some(format!("Input Ch#{idx}")))
        } else {
            None
        };

        Self {
            data: data.to_vec(),
            abnormalities: Vec::new(),
            is_input: true,
            color,
            label,
            idx,
        }
    }

    fn from_output_data(
        data: &[f32],
        abnormalities: &[(usize, AbnormalSample)],
        idx: usize,
        config: &SnapshotConfig,
    ) -> Self {
        let color = config
            .output_colors
            .as_ref()
            .and_then(|colors| colors.get(idx))
            .map(|s| s.as_str())
            .unwrap_or_else(|| OUTPUT_CHANNEL_COLORS[idx % OUTPUT_CHANNEL_COLORS.len()]);
        let color = parse_hex_color(color);

        let label = if config.show_labels {
            config
                .output_titles
                .get(idx)
                .cloned()
                .or_else(|| Some(format!("Output Ch#{idx}")))
        } else {
            None
        };

        Self {
            data: data.to_vec(),
            abnormalities: abnormalities.to_vec(),
            is_input: false,
            color,
            label,
            idx,
        }
    }
}

pub(crate) fn generate_svg(
    input_data: &[Vec<f32>],
    output_data: &[Vec<f32>],
    abnormalities: &[Vec<(usize, AbnormalSample)>],
    config: &SnapshotConfig,
) -> String {
    let height_per_channel = config.svg_height_per_channel;
    let num_channels = output_data.len()
        + if config.with_inputs {
            input_data.len()
        } else {
            0
        };
    let num_samples = output_data.first().map(|c| c.len()).unwrap_or(0);

    if num_samples == 0 || num_channels == 0 {
        return "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\"><text>Empty</text></svg>".to_string();
    }

    let svg_width = config.svg_width.unwrap_or(config.num_samples * 2) as u32;
    let total_height = (height_per_channel * num_channels) as u32;

    // Create SVG backend with buffer
    let mut svg_buffer = String::new();
    {
        let root =
            SVGBackend::with_string(&mut svg_buffer, (svg_width, total_height)).into_drawing_area();

        // Fill background
        let bg_color = parse_hex_color(&config.background_color);
        root.fill(&bg_color).unwrap();

        // Add optional title with contrasting color
        let current_area = if let Some(ref title) = config.chart_title {
            let title_color = get_contrasting_color(&bg_color);
            let text_style = TextStyle::from(("sans-serif", 20)).color(&title_color);
            root.titled(title, text_style).unwrap()
        } else {
            root
        };

        let input_charts: Vec<ChannelChartData> = if config.with_inputs {
            input_data
                .iter()
                .enumerate()
                .map(|(i, data)| ChannelChartData::from_input_data(data, i, config))
                .collect()
        } else {
            vec![]
        };

        let output_charts: Vec<ChannelChartData> = output_data
            .iter()
            .zip(abnormalities)
            .enumerate()
            .map(|(i, (data, abnormalities))| {
                ChannelChartData::from_output_data(data, abnormalities, i, config)
            })
            .collect();

        match config.chart_layout {
            Layout::SeparateChannels => {
                // Split area for each channel
                let areas = current_area.split_evenly((num_channels, 1));
                for (chart, area) in input_charts
                    .into_iter()
                    .chain(output_charts.into_iter())
                    .zip(areas)
                {
                    one_channel_chart(chart, config.line_width, config.show_grid, &area);
                }
            }
            Layout::CombinedPerChannelType => {
                let areas = current_area.split_evenly((if config.with_inputs { 2 } else { 1 }, 1));
                let output_axis_color = RGBColor(0, 0, 255);

                if config.with_inputs {
                    let input_axis_color = RGBColor(255, 0, 0);

                    multi_channel_chart(
                        input_charts,
                        config.line_width,
                        config.show_grid,
                        true,
                        input_axis_color,
                        &areas[0],
                    );
                    multi_channel_chart(
                        output_charts,
                        config.line_width,
                        config.show_grid,
                        true,
                        output_axis_color,
                        &areas[1],
                    );
                } else {
                    multi_channel_chart(
                        output_charts,
                        config.line_width,
                        config.show_grid,
                        true,
                        output_axis_color,
                        &areas[0],
                    );
                }
            }
            Layout::Combined => {
                let charts = output_charts.into_iter().chain(input_charts).collect();
                let output_axis_color = RGBColor(0, 0, 255);
                multi_channel_chart(
                    charts,
                    config.line_width,
                    config.show_grid,
                    false,
                    output_axis_color,
                    &current_area,
                );
            }
        }

        current_area.present().unwrap();
    }

    svg_buffer
}

// Helper function to parse hex color strings
fn parse_hex_color(hex: &str) -> RGBColor {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    RGBColor(r, g, b)
}

// Helper function to get a contrasting color for text
fn get_contrasting_color(bg: &RGBColor) -> RGBColor {
    // Calculate relative luminance using WCAG formula
    let r = bg.0 as f32 / 255.0;
    let g = bg.1 as f32 / 255.0;
    let b = bg.2 as f32 / 255.0;

    let r_linear = if r <= 0.03928 {
        r / 12.92
    } else {
        ((r + 0.055) / 1.055).powf(2.4)
    };
    let g_linear = if g <= 0.03928 {
        g / 12.92
    } else {
        ((g + 0.055) / 1.055).powf(2.4)
    };
    let b_linear = if b <= 0.03928 {
        b / 12.92
    } else {
        ((b + 0.055) / 1.055).powf(2.4)
    };

    let luminance = 0.2126 * r_linear + 0.7152 * g_linear + 0.0722 * b_linear;

    // Use white for dark backgrounds, dark gray for light backgrounds
    if luminance < 0.5 {
        RGBColor(255, 255, 255) // White
    } else {
        RGBColor(32, 32, 32) // Dark gray
    }
}

fn multi_channel_chart(
    charts_data: Vec<ChannelChartData>,
    line_width: f32,
    show_grid: bool,
    solid_input: bool,
    axis_color: RGBColor,
    area: &DrawingArea<SVGBackend<'_>, plotters::coord::Shift>,
) {
    let num_samples = charts_data
        .iter()
        .map(|chart| chart.data.len())
        .max()
        .unwrap_or_default();
    let min_val = charts_data
        .iter()
        .map(|chart| chart.data.iter().cloned().fold(f32::INFINITY, f32::min))
        .reduce(f32::min)
        .unwrap_or_default();
    let max_val = charts_data
        .iter()
        .map(|chart| chart.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max))
        .reduce(f32::max)
        .unwrap_or_default();
    let range = (max_val - min_val).max(f32::EPSILON);
    let y_min = (min_val - range * 0.1) as f64;
    let y_max = (max_val + range * 0.1) as f64;

    // Build chart
    let mut chart = ChartBuilder::on(area)
        .margin(5)
        .build_cartesian_2d(0f64..num_samples as f64, y_min..y_max)
        .unwrap();

    let mut mesh = chart.configure_mesh();

    mesh.axis_style(axis_color.mix(0.3));

    if !show_grid {
        mesh.disable_mesh();
    } else {
        mesh.light_line_style(axis_color.mix(0.1))
            .bold_line_style(axis_color.mix(0.2));
    }

    mesh.draw().unwrap();

    let ctx = chart
        .draw_series(
            charts_data
                .iter()
                .filter(|d| !d.is_input || solid_input)
                .map(|entry| {
                    let ChannelChartData {
                        data: channel_data,
                        color,
                        ..
                    } = entry;

                    let line_style = ShapeStyle {
                        color: color.to_rgba(),
                        filled: false,
                        stroke_width: line_width as u32,
                    };

                    PathElement::new(
                        channel_data
                            .iter()
                            .enumerate()
                            .map(|(i, &sample)| (i as f64, sample as f64))
                            .collect::<Vec<(f64, f64)>>(),
                        line_style,
                    )
                }),
        )
        .unwrap();

    for entry in charts_data
        .iter()
        .filter(|d| d.label.is_some() && (!d.is_input || solid_input))
    {
        ctx.label(entry.label.as_ref().unwrap())
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], entry.color));
    }

    if !solid_input && charts_data.iter().any(|d| d.is_input) {
        let ctx = chart
            .draw_series(charts_data.iter().filter(|d| d.is_input).map(|entry| {
                let ChannelChartData {
                    data: channel_data,
                    color,
                    ..
                } = entry;

                let line_style = ShapeStyle {
                    color: color.to_rgba(),
                    filled: false,
                    stroke_width: line_width as u32,
                };

                DashedPathElement::new(
                    channel_data
                        .iter()
                        .enumerate()
                        .map(|(i, &sample)| (i as f64, sample as f64))
                        .collect::<Vec<(f64, f64)>>(),
                    2,
                    3,
                    line_style,
                )
            }))
            .unwrap();

        for entry in charts_data
            .iter()
            .filter(|d| d.label.is_some() && d.is_input)
        {
            ctx.label(entry.label.as_ref().unwrap())
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], entry.color));
        }
    }

    if charts_data.iter().any(|d| !d.abnormalities.is_empty()) {
        chart
            .draw_series(PointSeries::of_element(
                charts_data
                    .iter()
                    .flat_map(|d| d.abnormalities.iter())
                    .map(|&(i, ab)| {
                        (
                            i as f64,
                            match ab {
                                AbnormalSample::Nan => 0.0,
                                AbnormalSample::NegInf => y_min.min(-1.0),
                                AbnormalSample::PosInf => y_max.max(1.0),
                            },
                        )
                    }),
                3,
                ShapeStyle::from(&RED).filled(),
                &|coord, size, style| {
                    EmptyElement::at(coord)
                        + Circle::new((0, 0), size, style)
                        + Text::new(
                            match coord.1 {
                                0.0 => AbnormalSample::Nan.to_string(),
                                y if y < 0.0 => AbnormalSample::NegInf.to_string(),
                                y if y > 0.0 => AbnormalSample::PosInf.to_string(),
                                _ => format!("{:.2}", coord.1),
                            },
                            match coord.1 {
                                0.0 => (0, -5),
                                y if y < 0.0 => (0, 15),
                                y if y > 0.0 => (0, -15),
                                _ => (0, 0),
                            },
                            ("sans-serif", 15),
                        )
                },
            ))
            .unwrap();
    }
}

fn one_channel_chart(
    chart: ChannelChartData,
    line_width: f32,
    show_grid: bool,
    area: &DrawingArea<SVGBackend<'_>, plotters::coord::Shift>,
) {
    let ChannelChartData {
        data: channel_data,
        abnormalities: channel_abnormalities,
        color,
        label,
        ..
    } = chart;

    let num_samples = channel_data.len();

    // Calculate data range
    let min_val = channel_data.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_val = channel_data
        .iter()
        .cloned()
        .fold(f32::NEG_INFINITY, f32::max);
    let range = (max_val - min_val).max(f32::EPSILON);
    let y_min = (min_val - range * 0.1) as f64;
    let y_max = (max_val + range * 0.1) as f64;

    // Build chart
    let mut chart = ChartBuilder::on(area)
        .margin(5)
        .x_label_area_size(if label.is_some() { 35 } else { 0 })
        .y_label_area_size(if label.is_some() { 50 } else { 0 })
        .build_cartesian_2d(0f64..num_samples as f64, y_min..y_max)
        .unwrap();

    let mut mesh = chart.configure_mesh();

    mesh.axis_style(color.mix(0.3));

    if !show_grid {
        mesh.disable_mesh();
    } else {
        mesh.light_line_style(color.mix(0.1))
            .bold_line_style(color.mix(0.2));
    }

    if let Some(label) = label {
        mesh.x_labels(5)
            .y_labels(3)
            .x_desc(label)
            .label_style(("sans-serif", 10, &color));
    }

    mesh.draw().unwrap();

    // Draw waveform
    let line_style = ShapeStyle {
        color: color.to_rgba(),
        filled: false,
        stroke_width: line_width as u32,
    };

    chart
        .draw_series(std::iter::once(PathElement::new(
            channel_data
                .iter()
                .enumerate()
                .map(|(i, &sample)| (i as f64, sample as f64))
                .collect::<Vec<(f64, f64)>>(),
            line_style,
        )))
        .unwrap();

    if !channel_abnormalities.is_empty() {
        chart
            .draw_series(PointSeries::of_element(
                channel_abnormalities.iter().map(|&(i, ab)| {
                    (
                        i as f64,
                        match ab {
                            AbnormalSample::Nan => 0.0,
                            AbnormalSample::NegInf => y_min.min(-1.0),
                            AbnormalSample::PosInf => y_max.max(1.0),
                        },
                    )
                }),
                3,
                ShapeStyle::from(&RED).filled(),
                &|coord, size, style| {
                    EmptyElement::at(coord)
                        + Circle::new((0, 0), size, style)
                        + Text::new(
                            match coord.1 {
                                0.0 => AbnormalSample::Nan.to_string(),
                                y if y < 0.0 => AbnormalSample::NegInf.to_string(),
                                y if y > 0.0 => AbnormalSample::PosInf.to_string(),
                                _ => format!("{:.2}", coord.1),
                            },
                            match coord.1 {
                                0.0 => (0, -5),
                                y if y < 0.0 => (0, 15),
                                y if y > 0.0 => (0, -15),
                                _ => (0, 0),
                            },
                            ("sans-serif", 15),
                        )
                },
            ))
            .unwrap();
    }
}
