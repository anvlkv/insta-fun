use plotters::{coord::types::RangedCoordf64, prelude::*};

use crate::chart_data::ChannelChartData;

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

pub(crate) fn abnormal_smaples_series(
    charts_data: &[ChannelChartData],
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    y_min: f64,
    y_max: f64,
) {
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
                            TextStyle::from(("sans-serif", 10).into_font()).color(&RED),
                        )
                },
            ))
            .unwrap();
    }
}
