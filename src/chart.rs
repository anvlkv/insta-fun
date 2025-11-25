use plotters::backend::SVGBackend;
use plotters::drawing::IntoDrawingArea;
use plotters::element::DashedPathElement;
use plotters::prelude::*;

use crate::abnormal::{AbnormalSample, abnormal_smaples_series};
use crate::chart_data::ChannelChartData;
use crate::config::SvgChartConfig;
use crate::util::{
    INPUT_CHANNEL_COLORS, OUTPUT_CHANNEL_COLORS, get_contrasting_color, num_x_labels,
    parse_hex_color, time_formatter,
};

/// Chart layout
///
/// Whether to plot channels on separate charts or combined charts.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

pub(crate) fn generate_svg(
    input_data: &[Vec<f32>],
    output_data: &[Vec<f32>],
    abnormalities: &[Vec<(usize, AbnormalSample)>],
    config: &SvgChartConfig,
    sample_rate: f64,
    num_samples: usize,
    start_sample: usize,
) -> String {
    let height_per_channel = config.svg_height_per_channel;
    let num_channels = output_data.len()
        + if config.with_inputs {
            input_data.len()
        } else {
            0
        };

    if num_samples == 0 || num_channels == 0 {
        return "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\"><text>Empty</text></svg>".to_string();
    }

    let svg_width = config.svg_width.unwrap_or(num_samples * 2) as u32;
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

        let output_axis_color = parse_hex_color(OUTPUT_CHANNEL_COLORS[0]);
        let input_axis_color = parse_hex_color(INPUT_CHANNEL_COLORS[0]);

        match config.chart_layout {
            Layout::SeparateChannels => {
                // Split area for each channel
                let areas = current_area.split_evenly((num_channels, 1));
                for (chart, area) in input_charts
                    .into_iter()
                    .chain(output_charts.into_iter())
                    .zip(areas)
                {
                    one_channel_chart(chart, config, start_sample, &area, sample_rate);
                }
            }
            Layout::CombinedPerChannelType => {
                if config.with_inputs {
                    let areas = current_area.split_evenly((2, 1));

                    multi_channel_chart(
                        input_charts,
                        config,
                        true,
                        start_sample,
                        input_axis_color,
                        &areas[0],
                        sample_rate,
                    );
                    multi_channel_chart(
                        output_charts,
                        config,
                        true,
                        start_sample,
                        output_axis_color,
                        &areas[1],
                        sample_rate,
                    );
                } else {
                    multi_channel_chart(
                        output_charts,
                        config,
                        true,
                        start_sample,
                        output_axis_color,
                        &current_area,
                        sample_rate,
                    );
                }
            }
            Layout::Combined => {
                let charts = output_charts.into_iter().chain(input_charts).collect();
                multi_channel_chart(
                    charts,
                    config,
                    false,
                    start_sample,
                    output_axis_color,
                    &current_area,
                    sample_rate,
                );
            }
        }

        current_area.present().unwrap();
    }

    if let Some(preserve_aspect_ratio) = config.preserve_aspect_ratio {
        svg_buffer.replace(
            "<svg ",
            format!(r#"<svg preserveAspectRatio="{preserve_aspect_ratio}" "#).as_str(),
        )
    } else {
        svg_buffer
    }
}

fn multi_channel_chart(
    charts_data: Vec<ChannelChartData>,
    config: &SvgChartConfig,
    solid_input: bool,
    start_from: usize,
    axis_color: RGBColor,
    area: &DrawingArea<SVGBackend<'_>, plotters::coord::Shift>,
    sample_rate: f64,
) {
    let num_samples = charts_data
        .iter()
        .map(|chart| chart.data.len())
        .max()
        .unwrap_or_default();
    let min_val = charts_data
        .iter()
        .flat_map(|c| c.data.iter())
        .cloned()
        .fold(f32::INFINITY, f32::min);
    let max_val = charts_data
        .iter()
        .flat_map(|c| c.data.iter())
        .cloned()
        .fold(f32::NEG_INFINITY, f32::max);

    let range = (max_val - min_val).max(f32::EPSILON);
    let y_min = (min_val - range * 0.1) as f64;
    let y_max = (max_val + range * 0.1) as f64;

    // Build chart
    let mut chart = ChartBuilder::on(area)
        .margin(5)
        .x_label_area_size(35)
        .y_label_area_size(50)
        .build_cartesian_2d(
            start_from as f64..(num_samples + start_from) as f64,
            y_min..y_max,
        )
        .unwrap();

    let mut mesh = chart.configure_mesh();

    mesh.axis_style(axis_color.mix(0.3));

    if !config.show_grid {
        mesh.disable_mesh();
    } else {
        mesh.light_line_style(axis_color.mix(0.1))
            .bold_line_style(axis_color.mix(0.2));
    }

    if config.show_labels {
        let x_labels = num_x_labels(num_samples, sample_rate);
        mesh.x_labels(
            config
                .max_labels_x_axis
                .map(|mx| x_labels.min(mx))
                .unwrap_or(x_labels),
        )
        .y_labels(3)
        .label_style(("sans-serif", 10, &axis_color));
    }

    let formatter = |v: &f64| time_formatter(*v as usize, sample_rate);
    if config.format_x_axis_labels_as_time {
        mesh.x_label_formatter(&formatter);
    }

    mesh.draw().unwrap();

    let mut has_legend = false;

    // Draw outputs (or inputs as solid when `solid_input` is true) one by one,
    // registering a legend entry per series.
    for entry in charts_data.iter().filter(|d| !d.is_input || solid_input) {
        let ChannelChartData {
            data: channel_data,
            color,
            label,
            ..
        } = entry;

        let line_style = ShapeStyle {
            color: color.to_rgba(),
            filled: false,
            stroke_width: config.line_width as u32,
        };

        let series = chart
            .draw_series(std::iter::once(PathElement::new(
                channel_data
                    .iter()
                    .enumerate()
                    .map(|(i, &sample)| ((i + start_from) as f64, sample as f64))
                    .collect::<Vec<(f64, f64)>>(),
                line_style,
            )))
            .unwrap();

        if let Some(label) = label {
            series
                .label(label)
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], entry.color));
            has_legend = true;
        }
    }

    // Dashed inputs when not solid
    if !solid_input && charts_data.iter().any(|d| d.is_input) {
        for entry in charts_data.iter().filter(|d| d.is_input) {
            let ChannelChartData {
                data: channel_data,
                color,
                label,
                ..
            } = entry;

            let line_style = ShapeStyle {
                color: color.to_rgba(),
                filled: false,
                stroke_width: config.line_width as u32,
            };

            let dashed = DashedPathElement::new(
                channel_data
                    .iter()
                    .enumerate()
                    .map(|(i, &sample)| ((i + start_from) as f64, sample as f64))
                    .collect::<Vec<(f64, f64)>>(),
                2,
                3,
                line_style,
            );

            let series = chart.draw_series(std::iter::once(dashed)).unwrap();

            if let Some(label) = label {
                series.label(label).legend(|(x, y)| {
                    DashedPathElement::new(vec![(x, y), (x + 20, y)], 2, 3, entry.color)
                });
                has_legend = true;
            }
        }
    }

    abnormal_smaples_series(&charts_data, &mut chart, y_min, y_max);

    if has_legend {
        let background = parse_hex_color(&config.background_color);
        let contrasting = get_contrasting_color(&background);

        chart
            .configure_series_labels()
            .border_style(contrasting)
            .background_style(background)
            .label_font(TextStyle::from(("sans-serif", 10)).color(&contrasting))
            .draw()
            .unwrap();
    }
}

fn one_channel_chart(
    chart_data: ChannelChartData,
    config: &SvgChartConfig,
    start_from: usize,
    area: &DrawingArea<SVGBackend<'_>, plotters::coord::Shift>,
    sample_rate: f64,
) {
    let ChannelChartData {
        data: channel_data,
        color,
        label,
        ..
    } = &chart_data;

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
        .build_cartesian_2d(
            start_from as f64..(num_samples + start_from) as f64,
            y_min..y_max,
        )
        .unwrap();

    let mut mesh = chart.configure_mesh();

    mesh.axis_style(color.mix(0.3));

    if !config.show_grid {
        mesh.disable_mesh();
    } else {
        mesh.light_line_style(color.mix(0.1))
            .bold_line_style(color.mix(0.2));
    }

    if let Some(label) = label {
        let x_labels = num_x_labels(num_samples, sample_rate);
        mesh.x_labels(
            config
                .max_labels_x_axis
                .map(|mx| x_labels.min(mx))
                .unwrap_or(x_labels),
        )
        .y_labels(3)
        .x_desc(label)
        .label_style(("sans-serif", 10, &color));
    }

    let formatter = |v: &f64| time_formatter(*v as usize, sample_rate);
    if config.format_x_axis_labels_as_time {
        mesh.x_label_formatter(&formatter);
    }

    mesh.draw().unwrap();

    // Draw waveform
    let line_style = ShapeStyle {
        color: color.to_rgba(),
        filled: false,
        stroke_width: config.line_width as u32,
    };

    chart
        .draw_series(std::iter::once(PathElement::new(
            channel_data
                .iter()
                .enumerate()
                .map(|(i, &sample)| ((i + start_from) as f64, sample as f64))
                .collect::<Vec<(f64, f64)>>(),
            line_style,
        )))
        .unwrap();

    abnormal_smaples_series(&[chart_data], &mut chart, y_min, y_max);
}
