use std::fmt;

pub use insta_fun_meta_macros::insta_fun_meta;

#[derive(Debug, Clone, PartialEq)]
pub enum MetaValue {
    Scalar(f64),
    Range { min: f64, max: f64 },
    Line(Vec<f64>),
    Histogram(Vec<f64>),
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
