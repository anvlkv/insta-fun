use plotters::style::RGBColor;

use crate::{
    abnormal::AbnormalSample,
    config::SnapshotConfig,
    util::{INPUT_CHANNEL_COLORS, OUTPUT_CHANNEL_COLORS, parse_hex_color},
};

#[allow(dead_code)]
pub(crate) struct ChannelChartData {
    pub data: Vec<f32>,
    pub abnormalities: Vec<(usize, AbnormalSample)>,
    pub color: RGBColor,
    pub label: Option<String>,
    pub is_input: bool,
    pub idx: usize,
}

impl ChannelChartData {
    pub fn from_input_data(data: &[f32], idx: usize, config: &SnapshotConfig) -> Self {
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

    pub fn from_output_data(
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
