use std::fmt;

pub use insta_fun_meta_macros::insta_fun_meta;

#[derive(Debug, Clone, PartialEq)]
pub enum MetaValue {
    Scalar(f64),
    Range { min: f64, max: f64 },
    Line(Vec<f64>),
    Histogram(Vec<f64>),
    /// Statistical summary with min/mean/max and optional percentiles (p25, p50, p75)
    Statistics {
        min: f64,
        p25: Option<f64>,
        mean: f64,
        p50: Option<f64>,
        p75: Option<f64>,
        max: f64,
    },
    /// Frequency response: magnitude in dB at each frequency bin, optional phase in degrees
    FrequencyResponse {
        magnitude: Vec<f64>,
        phase: Option<Vec<f64>>,
    },
    /// Tabular metadata: field name -> value pairs
    Table(Vec<(String, String)>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaField {
    pub name: String,
    pub value: MetaValue,
}

impl MetaField {
    pub fn new(name: impl Into<String>, value: impl Into<MetaValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotMetadata {
    pub fields: Vec<MetaField>,
}

impl SnapshotMetadata {
    pub fn new(fields: Vec<MetaField>) -> Self {
        Self { fields }
    }

    pub fn validate(&self) -> Result<(), MetadataValidationError> {
        if self.fields.is_empty() {
            return Err(MetadataValidationError::EmptyMetadata);
        }

        for field in &self.fields {
            if field.name.trim().is_empty() {
                return Err(MetadataValidationError::EmptyFieldName);
            }

            match &field.value {
                MetaValue::Scalar(v) => {
                    if !v.is_finite() {
                        return Err(MetadataValidationError::NonFiniteValue {
                            field: field.name.clone(),
                        });
                    }
                }
                MetaValue::Range { min, max } => {
                    if !min.is_finite() || !max.is_finite() {
                        return Err(MetadataValidationError::NonFiniteValue {
                            field: field.name.clone(),
                        });
                    }
                }
                MetaValue::Line(values) | MetaValue::Histogram(values) => {
                    if values.is_empty() {
                        return Err(MetadataValidationError::EmptySeries {
                            field: field.name.clone(),
                        });
                    }
                    if values.iter().any(|v| !v.is_finite()) {
                        return Err(MetadataValidationError::NonFiniteValue {
                            field: field.name.clone(),
                        });
                    }
                }
                MetaValue::Statistics {
                    min,
                    mean,
                    max,
                    p25,
                    p50,
                    p75,
                } => {
                    if !min.is_finite() || !mean.is_finite() || !max.is_finite() {
                        return Err(MetadataValidationError::NonFiniteValue {
                            field: field.name.clone(),
                        });
                    }
                    if let Some(p) = p25
                        && (!p.is_finite() || *p < *min || *p > *max) {
                            return Err(MetadataValidationError::InvalidStatistics {
                                field: field.name.clone(),
                            });
                        }
                    if let Some(p) = p50
                        && (!p.is_finite() || *p < *min || *p > *max) {
                            return Err(MetadataValidationError::InvalidStatistics {
                                field: field.name.clone(),
                            });
                        }
                    if let Some(p) = p75
                        && (!p.is_finite() || *p < *min || *p > *max) {
                            return Err(MetadataValidationError::InvalidStatistics {
                                field: field.name.clone(),
                            });
                        }
                }
                MetaValue::FrequencyResponse { magnitude: _, phase } => {
                    if let Some(phases) = phase
                        && phases.iter().any(|v| !v.is_finite()) {
                            return Err(MetadataValidationError::NonFiniteValue {
                                field: field.name.clone(),
                            });
                        }
                }
                MetaValue::Table(pairs) => {
                    if pairs.is_empty() {
                        return Err(MetadataValidationError::EmptySeries {
                            field: field.name.clone(),
                        });
                    }
                    let mut seen_keys = std::collections::HashSet::new();
                    for (key, _) in pairs {
                        if key.trim().is_empty() {
                            return Err(MetadataValidationError::EmptyFieldName);
                        }
                        if !seen_keys.insert(key.clone()) {
                            return Err(MetadataValidationError::DuplicateTableKey {
                                field: field.name.clone(),
                                key: key.clone(),
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetadataValidationError {
    EmptyMetadata,
    EmptyFieldName,
    EmptySeries { field: String },
    NonFiniteValue { field: String },
    InvalidStatistics { field: String },
    DuplicateTableKey { field: String, key: String },
}

impl fmt::Display for MetadataValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetadataValidationError::EmptyMetadata => {
                write!(f, "metadata dashboard requires at least one field")
            }
            MetadataValidationError::EmptyFieldName => {
                write!(f, "metadata field name cannot be empty")
            }
            MetadataValidationError::EmptySeries { field } => {
                write!(f, "metadata field '{field}' has an empty series")
            }
            MetadataValidationError::NonFiniteValue { field } => {
                write!(f, "metadata field '{field}' contains non-finite values")
            }
            MetadataValidationError::InvalidStatistics { field } => {
                write!(f, "metadata field '{field}' has invalid percentiles (must be between min and max)")
            }
            MetadataValidationError::DuplicateTableKey { field, key } => {
                write!(f, "metadata field '{field}' has duplicate table key '{key}'")
            }
        }
    }
}

impl std::error::Error for MetadataValidationError {}

pub trait ToMetaNumber {
    fn to_meta_number(self) -> f64;
}

macro_rules! impl_to_meta_number {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ToMetaNumber for $ty {
                fn to_meta_number(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}

impl_to_meta_number!(
    f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

pub fn scalar<T>(value: T) -> MetaValue
where
    T: ToMetaNumber,
{
    MetaValue::Scalar(value.to_meta_number())
}

pub fn range<T, U>(min: T, max: U) -> MetaValue
where
    T: ToMetaNumber,
    U: ToMetaNumber,
{
    MetaValue::Range {
        min: min.to_meta_number(),
        max: max.to_meta_number(),
    }
}

pub fn line<I, T>(values: I) -> MetaValue
where
    I: IntoIterator<Item = T>,
    T: ToMetaNumber,
{
    MetaValue::Line(values.into_iter().map(ToMetaNumber::to_meta_number).collect())
}

pub fn histogram<I, T>(values: I) -> MetaValue
where
    I: IntoIterator<Item = T>,
    T: ToMetaNumber,
{
    MetaValue::Histogram(values.into_iter().map(ToMetaNumber::to_meta_number).collect())
}

pub fn statistics<T, U>(
    min: T,
    mean: U,
    max: T,
) -> MetaValue
where
    T: ToMetaNumber,
    U: ToMetaNumber,
{
    MetaValue::Statistics {
        min: min.to_meta_number(),
        mean: mean.to_meta_number(),
        max: max.to_meta_number(),
        p25: None,
        p50: None,
        p75: None,
    }
}

pub fn statistics_with_percentiles<T, U>(
    min: T,
    p25: Option<T>,
    p50: Option<T>,
    mean: U,
    p75: Option<T>,
    max: T,
) -> MetaValue
where
    T: ToMetaNumber,
    U: ToMetaNumber,
{
    MetaValue::Statistics {
        min: min.to_meta_number(),
        p25: p25.map(|v| v.to_meta_number()),
        p50: p50.map(|v| v.to_meta_number()),
        mean: mean.to_meta_number(),
        p75: p75.map(|v| v.to_meta_number()),
        max: max.to_meta_number(),
    }
}

/// Compute statistics from raw data, auto-calculating min, mean, max and percentiles (p25, p50, p75).
pub fn statistics_from_data<I, T>(data: I) -> MetaValue
where
    I: IntoIterator<Item = T>,
    T: ToMetaNumber,
{
    let values: Vec<f64> = data.into_iter().map(ToMetaNumber::to_meta_number).collect();
    if values.is_empty() {
        // Return default for empty data
        return MetaValue::Statistics {
            min: 0.0,
            p25: None,
            mean: 0.0,
            p50: None,
            p75: None,
            max: 0.0,
        };
    }

    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mean = values.iter().sum::<f64>() / values.len() as f64;

    // Compute percentiles
    let mut sorted = values.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let p25 = Some(sorted[sorted.len() / 4]);
    let p50 = Some(sorted[sorted.len() / 2]);
    let p75 = Some(sorted[(sorted.len() * 3) / 4]);

    MetaValue::Statistics {
        min,
        p25,
        mean,
        p50,
        p75,
        max,
    }
}

/// Compute statistics from raw data without percentiles (min, mean, max only).
pub fn statistics_from_data_simple<I, T>(data: I) -> MetaValue
where
    I: IntoIterator<Item = T>,
    T: ToMetaNumber,
{
    let values: Vec<f64> = data.into_iter().map(ToMetaNumber::to_meta_number).collect();
    if values.is_empty() {
        return MetaValue::Statistics {
            min: 0.0,
            p25: None,
            mean: 0.0,
            p50: None,
            p75: None,
            max: 0.0,
        };
    }

    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mean = values.iter().sum::<f64>() / values.len() as f64;

    MetaValue::Statistics {
        min,
        p25: None,
        mean,
        p50: None,
        p75: None,
        max,
    }
}

pub fn frequency_response<I, T>(magnitude: I) -> MetaValue
where
    I: IntoIterator<Item = T>,
    T: ToMetaNumber,
{
    MetaValue::FrequencyResponse {
        magnitude: magnitude.into_iter().map(ToMetaNumber::to_meta_number).collect(),
        phase: None,
    }
}

pub fn frequency_response_with_phase<I, T, J, U>(
    magnitude: I,
    phase: J,
) -> MetaValue
where
    I: IntoIterator<Item = T>,
    T: ToMetaNumber,
    J: IntoIterator<Item = U>,
    U: ToMetaNumber,
{
    MetaValue::FrequencyResponse {
        magnitude: magnitude.into_iter().map(ToMetaNumber::to_meta_number).collect(),
        phase: Some(phase.into_iter().map(ToMetaNumber::to_meta_number).collect()),
    }
}

pub fn table<I, K, V>(pairs: I) -> MetaValue
where
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: fmt::Display,
{
    MetaValue::Table(
        pairs
            .into_iter()
            .map(|(k, v)| (k.into(), v.to_string()))
            .collect(),
    )
}
