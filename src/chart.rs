use plotters::backend::SVGBackend;
use plotters::drawing::IntoDrawingArea;
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

pub(crate) fn generate_svg(
    input_data: &[Vec<f32>],
    output_data: &[Vec<f32>],
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

        // Process all channels
        let all_channels: Vec<(&Vec<f32>, bool)> = if config.with_inputs {
            input_data
                .iter()
                .map(|d| (d, true))
                .collect::<Vec<_>>()
                .into_iter()
                .chain(output_data.iter().map(|d| (d, false)))
                .collect()
        } else {
            output_data.iter().map(|d| (d, false)).collect()
        };

        // Split area for each channel
        let areas = current_area.split_evenly((num_channels, 1));

        for (idx, (area, (channel_data, is_input))) in
            areas.iter().zip(all_channels.iter()).enumerate()
        {
            if channel_data.is_empty() {
                continue;
            }

            let channel_data = *channel_data;

            // Get color for this channel
            let color_str = if *is_input {
                config
                    .input_colors
                    .as_ref()
                    .and_then(|colors| colors.get(idx))
                    .map(|s| s.as_str())
                    .unwrap_or_else(|| INPUT_CHANNEL_COLORS[idx % INPUT_CHANNEL_COLORS.len()])
            } else {
                let output_idx = if config.with_inputs {
                    idx - input_data.len()
                } else {
                    idx
                };
                config
                    .output_colors
                    .as_ref()
                    .and_then(|colors| colors.get(output_idx))
                    .map(|s| s.as_str())
                    .unwrap_or_else(|| {
                        OUTPUT_CHANNEL_COLORS[output_idx % OUTPUT_CHANNEL_COLORS.len()]
                    })
            };
            let color = parse_hex_color(color_str);

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
                .x_label_area_size(if config.show_labels { 35 } else { 0 })
                .y_label_area_size(if config.show_labels { 50 } else { 0 })
                .build_cartesian_2d(0f64..num_samples as f64, y_min..y_max)
                .unwrap();

            // Configure mesh (grid)
            if config.show_grid {
                chart
                    .configure_mesh()
                    .x_labels(5)
                    .y_labels(3)
                    .x_desc(if *is_input {
                        format!("Input Ch#{}", idx)
                    } else {
                        format!(
                            "Output Ch#{}",
                            if config.with_inputs {
                                idx - input_data.len()
                            } else {
                                idx
                            }
                        )
                    })
                    .label_style(("sans-serif", 10, &color))
                    .axis_style(color.mix(0.3))
                    .draw()
                    .unwrap();
            } else if config.show_labels {
                chart
                    .configure_mesh()
                    .disable_mesh()
                    .x_labels(5)
                    .y_labels(3)
                    .x_desc(if *is_input {
                        format!("Input Ch#{}", idx)
                    } else {
                        format!(
                            "Output Ch#{}",
                            if config.with_inputs {
                                idx - input_data.len()
                            } else {
                                idx
                            }
                        )
                    })
                    .label_style(("sans-serif", 10, &color))
                    .axis_style(color.mix(0.3))
                    .draw()
                    .unwrap();
            }

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
                        .map(|(i, &sample)| (i as f64, sample as f64))
                        .collect::<Vec<(f64, f64)>>(),
                    line_style,
                )))
                .unwrap();
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

#[cfg(test)]
mod tests {
    use crate::assert_audio_unit_snapshot;
    use crate::config::SnapshotConfigBuilder;
    use crate::prelude::InputSource;
    use fundsp::prelude::*;

    #[test]
    fn test_chart_with_title() {
        let config = SnapshotConfigBuilder::default()
            .chart_title(Some("Test Waveform 440Hz".to_string()))
            .build()
            .unwrap();
        let unit = sine_hz::<f32>(440.0);
        assert_audio_unit_snapshot!("chart_with_title", unit, InputSource::None, config);
    }

    #[test]
    fn test_chart_with_grid() {
        let config = SnapshotConfigBuilder::default()
            .show_grid(true)
            .build()
            .unwrap();
        let unit = sine_hz::<f32>(440.0);
        assert_audio_unit_snapshot!("chart_with_grid", unit, InputSource::None, config);
    }

    #[test]
    fn test_chart_without_labels() {
        let config = SnapshotConfigBuilder::default()
            .show_labels(false)
            .build()
            .unwrap();
        let unit = sine_hz::<f32>(440.0);
        assert_audio_unit_snapshot!("chart_without_labels", unit, InputSource::None, config);
    }

    #[test]
    fn test_chart_with_custom_colors() {
        let config = SnapshotConfigBuilder::default()
            .with_inputs(true)
            .output_colors(Some(vec!["#FF0000".to_string(), "#00FF00".to_string()]))
            .input_colors(Some(vec!["#0000FF".to_string(), "#FFFF00".to_string()]))
            .build()
            .unwrap();
        let unit = lowpass_hz(1000.0, 0.7);
        assert_audio_unit_snapshot!(
            "chart_custom_colors",
            unit,
            InputSource::sine(200.0, 44100.0),
            config
        );
    }

    #[test]
    fn test_chart_with_custom_background() {
        let config = SnapshotConfigBuilder::default()
            .background_color("#1E1E1E".to_string())
            .build()
            .unwrap();
        let unit = sine_hz::<f32>(440.0);
        assert_audio_unit_snapshot!("chart_custom_background", unit, InputSource::None, config);
    }

    #[test]
    fn test_chart_with_custom_line_width() {
        let config = SnapshotConfigBuilder::default()
            .line_width(4.0)
            .build()
            .unwrap();
        let unit = sine_hz::<f32>(440.0);
        assert_audio_unit_snapshot!("chart_custom_line_width", unit, InputSource::None, config);
    }

    #[test]
    fn test_chart_with_custom_dimensions() {
        let config = SnapshotConfigBuilder::default()
            .svg_width(Some(800))
            .svg_height_per_channel(150)
            .build()
            .unwrap();
        let unit = sine_hz::<f32>(440.0);
        assert_audio_unit_snapshot!("chart_custom_dimensions", unit, InputSource::None, config);
    }

    #[test]
    fn test_chart_with_all_options() {
        let config = SnapshotConfigBuilder::default()
            .chart_title(Some("Complete Waveform Test".to_string()))
            .show_grid(true)
            .show_labels(true)
            .with_inputs(true)
            .output_colors(Some(vec!["#FF6B6B".to_string()]))
            .input_colors(Some(vec!["#95E77E".to_string()]))
            .background_color("#2C3E50".to_string())
            .line_width(3.0)
            .svg_width(Some(1200))
            .svg_height_per_channel(120)
            .build()
            .unwrap();
        let unit = sine_hz::<f32>(440.0);
        assert_audio_unit_snapshot!(
            "chart_all_options",
            unit,
            InputSource::sine(100.0, 44100.0),
            config
        );
    }

    #[test]
    fn test_chart_grid_and_no_labels() {
        let config = SnapshotConfigBuilder::default()
            .show_grid(true)
            .show_labels(false)
            .build()
            .unwrap();
        let unit = sine_hz::<f32>(440.0);
        assert_audio_unit_snapshot!("chart_grid_no_labels", unit, InputSource::None, config);
    }

    #[test]
    fn test_chart_multi_channel_with_custom_colors() {
        let config = SnapshotConfigBuilder::default()
            .with_inputs(true)
            .output_colors(Some(vec!["#FF1744".to_string(), "#00E676".to_string()]))
            .input_colors(Some(vec!["#2979FF".to_string(), "#FFEA00".to_string()]))
            .build()
            .unwrap();
        // Create stereo filter unit
        let unit = lowpass_hz(1000.0, 0.7) | highpass_hz(200.0, 0.7);
        assert_audio_unit_snapshot!(
            "chart_stereo_custom_colors",
            unit,
            InputSource::Flat(vec![0.5, -0.5]),
            config
        );
    }
}
