use plotters::backend::SVGBackend;
use plotters::coord::types::RangedCoordf64;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::*;

use crate::config::SvgChartConfig;
use crate::meta::{MetaValue, SnapshotMetadata};
use crate::util::{get_contrasting_color, parse_hex_color};

const MIN_RANGE_SPAN: f64 = 1e-6;

pub(crate) fn generate_meta_dashboard_svg(
    metadata: &SnapshotMetadata,
    config: &SvgChartConfig,
) -> String {
    let rows = metadata.fields.len().max(1);
    let width = config.svg_width.unwrap_or(1200) as u32;
    let height = (config.svg_height_per_channel * rows) as u32;

    let mut svg_buffer = String::new();
    {
        let root = SVGBackend::with_string(&mut svg_buffer, (width, height)).into_drawing_area();

        let bg = parse_hex_color(&config.background_color);
        root.fill(&bg).unwrap();

        let title_color = get_contrasting_color(&bg);
        let area = if let Some(title) = &config.chart_title {
            root.titled(title, TextStyle::from(("sans-serif", 20)).color(&title_color))
                .unwrap()
        } else {
            root
        };

        let areas = area.split_evenly((rows, 1));

        for (field, field_area) in metadata.fields.iter().zip(areas.into_iter()) {
            draw_meta_field(field, field_area, config, title_color);
        }

        area.present().unwrap();
    }

    if let Some(preserve_aspect_ratio) = config.preserve_aspect_ratio {
        svg_buffer.replace(
            format!(r#"<svg width=\"{width}\" height=\"{height}\" "#).as_str(),
            format!(
                r#"<svg width=\"100%\" height=\"100%\" preserveAspectRatio=\"{preserve_aspect_ratio}\" "#
            )
            .as_str(),
        )
    } else {
        svg_buffer
    }
}

fn draw_meta_field(
    field: &crate::meta::MetaField,
    area: DrawingArea<SVGBackend<'_>, plotters::coord::Shift>,
    config: &SvgChartConfig,
    text_color: RGBColor,
) {
    let line_color = parse_hex_color(
        config
            .output_colors
            .as_ref()
            .and_then(|colors| colors.first())
            .map(String::as_str)
            .unwrap_or("#4285F4"),
    );

    match &field.value {
        MetaValue::Scalar(value) => draw_scalar_chart(field, *value, area, line_color, config, text_color),
        MetaValue::Range { min, max } => {
            draw_range_chart(field, *min, *max, area, line_color, config, text_color)
        }
        MetaValue::Line(series) => draw_line_chart(field, series, area, line_color, config, text_color),
        MetaValue::Histogram(values) => {
            draw_histogram_chart(field, values, area, line_color, config, text_color)
        }
    }
}

fn draw_scalar_chart(
    field: &crate::meta::MetaField,
    value: f64,
    area: DrawingArea<SVGBackend<'_>, plotters::coord::Shift>,
    line_color: RGBColor,
    config: &SvgChartConfig,
    text_color: RGBColor,
) {
    let pad = (value.abs() * 0.15).max(0.5);
    let y_min = value.min(0.0) - pad;
    let y_max = value.max(0.0) + pad;

    let mut chart = ChartBuilder::on(&area)
        .margin(5)
        .x_label_area_size(35)
        .y_label_area_size(50)
        .caption(
            format!("{} = {:.6}", field.name, value),
            TextStyle::from(("sans-serif", 14)).color(&text_color),
        )
        .build_cartesian_2d(0f64..1f64, y_min..y_max)
        .unwrap();

    configure_mesh(&mut chart, config, text_color);

    chart
        .draw_series(std::iter::once(Rectangle::new(
            [(0.0, 0.0), (1.0, value)],
            line_color.filled(),
        )))
        .unwrap();
}

fn draw_range_chart(
    field: &crate::meta::MetaField,
    min: f64,
    max: f64,
    area: DrawingArea<SVGBackend<'_>, plotters::coord::Shift>,
    line_color: RGBColor,
    config: &SvgChartConfig,
    text_color: RGBColor,
) {
    let lo = min.min(max);
    let hi = max.max(min);
    let span = (hi - lo).max(MIN_RANGE_SPAN);
    let x_min = lo - span * 0.1;
    let x_max = hi + span * 0.1;

    let mut chart = ChartBuilder::on(&area)
        .margin(5)
        .x_label_area_size(35)
        .y_label_area_size(50)
        .caption(
            format!("{} [{:.6}, {:.6}]", field.name, lo, hi),
            TextStyle::from(("sans-serif", 14)).color(&text_color),
        )
        .build_cartesian_2d(x_min..x_max, 0f64..1f64)
        .unwrap();

    configure_mesh(&mut chart, config, text_color);

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(lo, 0.5), (hi, 0.5)],
            ShapeStyle {
                color: line_color.to_rgba(),
                filled: false,
                stroke_width: config.line_width as u32,
            },
        )))
        .unwrap();

    chart
        .draw_series(vec![
            Circle::new((lo, 0.5), 4, line_color.filled()),
            Circle::new((hi, 0.5), 4, line_color.filled()),
        ])
        .unwrap();
}

fn draw_line_chart(
    field: &crate::meta::MetaField,
    series: &[f64],
    area: DrawingArea<SVGBackend<'_>, plotters::coord::Shift>,
    line_color: RGBColor,
    config: &SvgChartConfig,
    text_color: RGBColor,
) {
    let (y_min, y_max) = padded_range(series);

    let mut chart = ChartBuilder::on(&area)
        .margin(5)
        .x_label_area_size(35)
        .y_label_area_size(50)
        .caption(
            field.name.clone(),
            TextStyle::from(("sans-serif", 14)).color(&text_color),
        )
        .build_cartesian_2d(0f64..series.len() as f64, y_min..y_max)
        .unwrap();

    configure_mesh(&mut chart, config, text_color);

    chart
        .draw_series(std::iter::once(PathElement::new(
            series
                .iter()
                .enumerate()
                .map(|(i, v)| (i as f64, *v))
                .collect::<Vec<_>>(),
            ShapeStyle {
                color: line_color.to_rgba(),
                filled: false,
                stroke_width: config.line_width as u32,
            },
        )))
        .unwrap();
}

fn draw_histogram_chart(
    field: &crate::meta::MetaField,
    values: &[f64],
    area: DrawingArea<SVGBackend<'_>, plotters::coord::Shift>,
    line_color: RGBColor,
    config: &SvgChartConfig,
    text_color: RGBColor,
) {
    let max = values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(0.0);

    let mut chart = ChartBuilder::on(&area)
        .margin(5)
        .x_label_area_size(35)
        .y_label_area_size(50)
        .caption(
            field.name.clone(),
            TextStyle::from(("sans-serif", 14)).color(&text_color),
        )
        .build_cartesian_2d(0f64..values.len() as f64, 0f64..(max * 1.1 + 1e-6))
        .unwrap();

    configure_mesh(&mut chart, config, text_color);

    chart
        .draw_series(values.iter().enumerate().map(|(idx, value)| {
            Rectangle::new(
                [(idx as f64, 0.0), ((idx + 1) as f64, *value)],
                line_color.mix(0.65).filled(),
            )
        }))
        .unwrap();
}

fn configure_mesh(
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    config: &SvgChartConfig,
    axis_color: RGBColor,
) {
    let mut mesh = chart.configure_mesh();
    mesh.axis_style(axis_color.mix(0.4));

    if !config.show_grid {
        mesh.disable_mesh();
    } else {
        mesh.light_line_style(axis_color.mix(0.15))
            .bold_line_style(axis_color.mix(0.3));
    }

    if !config.show_labels {
        mesh.disable_x_mesh();
        mesh.disable_y_mesh();
    }

    mesh.label_style(("sans-serif", 10, &axis_color));
    mesh.draw().unwrap();
}

fn padded_range(values: &[f64]) -> (f64, f64) {
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = (max - min).max(MIN_RANGE_SPAN);
    (min - span * 0.1, max + span * 0.1)
}
